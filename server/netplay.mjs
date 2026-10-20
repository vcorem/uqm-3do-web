import http from 'node:http';
import { createReadStream } from 'node:fs';
import { realpath, stat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { WebSocketServer, WebSocket } from 'ws';

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8',
  '.wasm': 'application/wasm', '.data': 'application/octet-stream', '.json': 'application/json',
  '.css': 'text/css; charset=utf-8', '.png': 'image/png', '.jpg': 'image/jpeg', '.svg': 'image/svg+xml' };
const isolatedHeaders = { 'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp', 'Cross-Origin-Resource-Policy': 'same-origin',
  'X-Content-Type-Options': 'nosniff', 'Cache-Control': 'no-store' };

function integer(value, fallback, min, max, name) {
  const result = value ?? fallback;
  if (!Number.isSafeInteger(result) || result < min || result > max) throw new Error(`Invalid ${name}`);
  return result;
}

// Same URL contract as the Cloudflare relay (deploy/netplay.js).
const ROOM_PATH = /^\/netplay\/(?:(hd)\/)?([0-9a-f]{32})$/;
const QUERY_PART = /^(role|name|engine)=([^&]*)$/;
const LOBBY_URL = /^\/netplay\/lobby\?edition=(original|hd)$/;
const INVALID_ROOM = 'Invalid room URL or role.';
const INVALID_NAME = 'Room names need 1 to 24 visible characters.';
// The same name rule is in deploy/netplay.js and sc2/wasm/netplay-client.js.
// Refused: control and format characters (bidi overrides, zero-width marks),
// line separators, surrogates, private-use and unassigned code points
// (noncharacters too), the blank fillers U+115F, U+1160, U+2800, U+3164 and
// U+FFA0, and more than 3 stacked nonspacing marks (Hebrew with niqqud needs 3).
const NAME_FORBIDDEN = /[\p{Cc}\p{Cf}\p{Cs}\p{Co}\p{Cn}\p{Zl}\p{Zp}\u115f\u1160\u2800\u3164\uffa0]/u;
// Counted in the decomposed form, where a precomposed letter shows its marks.
const NAME_STACKED = /[\p{Mn}\p{Me}]{4}/u;
// At least one letter, digit, punctuation mark or symbol.
const NAME_VISIBLE = /[\p{L}\p{N}\p{P}\p{S}]/u;
const NAME_CHARS = 24;
const MAX_LISTED = 50;
const SNAPSHOT_MS = 500;
const LISTING_DENIED = {
  full: 'The room list is full. Try again in a few minutes.',
  ip: 'Your network already has the most open rooms allowed. Start or close one first.',
};
const SILENT_HOST = 'The room closed because this page stopped answering. Create a new room.';
const TOO_MANY_JOINS = 'You joined many open rooms in a short time. Wait a few minutes, then try again.';
// Recent listed-room joins are kept for at most this many addresses.
const MAX_JOIN_ADDRESSES = 10000;

/**
 * The listed form of a room name (NFC, inner spaces collapsed to one, trimmed),
 * or null when it does not have 1 to 24 allowed characters.
 */
export function checkedName(value) {
  const text = value.normalize('NFC');
  // Before collapsing spaces: line breaks and tabs are control characters.
  if (NAME_FORBIDDEN.test(text) || NAME_STACKED.test(text.normalize('NFD'))) return null;
  const name = text.replace(/\s+/g, ' ').trim();
  const length = [...name].length;
  return length >= 1 && length <= NAME_CHARS && NAME_VISIBLE.test(name) ? name : null;
}

/**
 * The key for the per-address bounds. One IPv6 client controls a whole /64
 * network, so IPv6 addresses count by their first 64 bits. IPv4 addresses, also
 * in IPv4-mapped IPv6 form, count one by one. The same function is in deploy/netplay.js.
 */
export function addressKey(ip) {
  const text = String(ip ?? '').trim().toLowerCase().replace(/%.*$/, '');
  if (!text.includes(':')) return text.slice(0, 64) || 'unknown';
  let hex = text;
  // An IPv4 address in the last 32 bits becomes two hex words.
  const v4 = /:(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/.exec(hex);
  if (v4) {
    const bytes = v4.slice(1).map(Number);
    if (bytes.some(byte => byte > 255)) return text.slice(0, 64);
    hex = hex.slice(0, v4.index + 1) + ((bytes[0] << 8) | bytes[1]).toString(16) + ':' +
      ((bytes[2] << 8) | bytes[3]).toString(16);
  }
  const halves = hex.split('::');
  if (halves.length > 2) return text.slice(0, 64);
  const head = halves[0] ? halves[0].split(':') : [];
  const tail = halves.length === 2 && halves[1] ? halves[1].split(':') : [];
  const missing = 8 - head.length - tail.length;
  if (halves.length === 1 ? missing !== 0 : missing < 1) return text.slice(0, 64);
  const parts = [...head, ...Array(halves.length === 2 ? missing : 0).fill('0'), ...tail];
  if (parts.some(part => !/^[0-9a-f]{1,4}$/.test(part))) return text.slice(0, 64);
  const words = parts.map(part => parseInt(part, 16));
  if (words.slice(0, 5).every(word => word === 0) && words[5] === 0xffff) {
    return [words[6] >> 8, words[6] & 255, words[7] >> 8, words[7] & 255].join('.');
  }
  return words.slice(0, 4).map(word => word.toString(16)).join(':') + '::/64';
}

/**
 * Strict room URL: role=host|guest, and for a host only an optional name
 * (which lists the room in its edition's lobby) and engine=c|rust. Each
 * parameter at most once; nothing else. The raw request target is checked, so
 * encoded path characters never match.
 */
export function parseRoomURL(target) {
  const query = target.indexOf('?');
  const path = ROOM_PATH.exec(query < 0 ? target : target.slice(0, query));
  if (!path || query < 0) return { error: INVALID_ROOM };
  const params = new Map();
  for (const part of target.slice(query + 1).split('&')) {
    const match = QUERY_PART.exec(part);
    if (!match || params.has(match[1])) return { error: INVALID_ROOM };
    params.set(match[1], match[2]);
  }
  const role = params.get('role');
  const rawName = params.get('name');
  const engine = params.get('engine') ?? null;
  if (role !== 'host' && role !== 'guest') return { error: INVALID_ROOM };
  if (role === 'guest' && params.size > 1) return { error: INVALID_ROOM };
  if (engine !== null && (rawName === undefined || (engine !== 'c' && engine !== 'rust'))) return { error: INVALID_ROOM };
  let name = null;
  if (rawName !== undefined) {
    // 24 characters of up to 4 UTF-8 bytes, each byte as %XX.
    try { name = rawName.length <= 288 ? checkedName(decodeURIComponent(rawName.replace(/\+/g, ' '))) : null; }
    catch { name = null; }
    if (name === null) return { error: INVALID_NAME };
  }
  return { edition: path[1] ?? null, token: path[2], role, name, engine };
}

/** A bounded, two-peer relay. Room IDs are single-use capabilities for this process. */
export function createNetplayServer(options = {}) {
  const maxPayload = integer(options.maxPayload, 65536, 2048, 65536, 'maxPayload');
  const maxBufferedBytes = integer(options.maxBufferedBytes, 262144, maxPayload, 1048576, 'maxBufferedBytes');
  const maxRooms = integer(options.maxRooms, 128, 1, 1024, 'maxRooms');
  const maxRoomIds = integer(options.maxRoomIds, 10000, maxRooms, 1000000, 'maxRoomIds');
  const maxPerIp = integer(options.maxPerIp, 16, 2, 2048, 'maxPerIp');
  const waitingMs = integer(options.waitingMs, 120000, 10, 3600000, 'waitingMs');
  const lifetimeMs = integer(options.lifetimeMs, 14400000, 10, 86400000, 'lifetimeMs');
  const heartbeatMs = integer(options.heartbeatMs, 15000, 10, 120000, 'heartbeatMs');
  const idleMs = integer(options.idleMs, 900000, 10, 86400000, 'idleMs');
  // Lobby: named rooms wait longer than private ones, because nobody has to copy an invite.
  const listedWaitingMs = integer(options.listedWaitingMs, 600000, 10, 3600000, 'listedWaitingMs');
  const maxListedRooms = integer(options.maxListedRooms, MAX_LISTED, 1, MAX_LISTED, 'maxListedRooms');
  const maxListedPerIp = integer(options.maxListedPerIp, 3, 1, MAX_LISTED, 'maxListedPerIp');
  const maxLobbyConnections = integer(options.maxLobbyConnections, 256, 1, 10000, 'maxLobbyConnections');
  const maxLobbyPerIp = integer(options.maxLobbyPerIp, 10, 1, 1000, 'maxLobbyPerIp');
  // A listed host pings every 30 s. Without a ping for this long it is gone
  // (a sleeping computer or a lost network); background tabs ping at least once a minute.
  const listedSilenceMs = integer(options.listedSilenceMs, 150000, 100, 3600000, 'listedSilenceMs');
  // Joins of listed rooms per address: list tokens are public, so one script
  // could otherwise join and leave every new room.
  const maxListedJoins = integer(options.maxListedJoins, 10, 1, 10000, 'maxListedJoins');
  const listedJoinWindowMs = integer(options.listedJoinWindowMs, 600000, 100, 86400000, 'listedJoinWindowMs');
  // Accommodate deliberately accelerated heartbeat intervals in integration tests.
  const maxControlFrames = Math.max(32, Math.ceil(1000 / heartbeatMs) + 4);
  // Integration tests may inject deterministic network delay. Never controlled by a client URL.
  const latencyPatternMs = options.latencyPatternMs ?? [0];
  if (!Array.isArray(latencyPatternMs) || !latencyPatternMs.length || latencyPatternMs.length > 100 ||
      latencyPatternMs.some(n => !Number.isSafeInteger(n) || n < 0 || n > 2000)) {
    throw new Error('Invalid latencyPatternMs');
  }
  const allowedOrigins = options.allowedOrigins == null ? null : new Set(options.allowedOrigins);
  if (allowedOrigins && [...allowedOrigins].some(origin => {
    try { const url = new URL(origin); return !['http:', 'https:'].includes(url.protocol) || url.origin !== origin; }
    catch { return true; }
  })) throw new Error('Allowed origins must be exact http(s) origins without trailing slashes');
  const distRoot = options.distDir === false ? null : path.resolve(options.distDir ?? path.join(projectRoot, 'dist'));
  const rooms = new Map();
  const usedRoomIds = new Set();
  const ipCounts = new Map();
  const lobbyIpCounts = new Map();
  // Join times of listed rooms per address, least recently used first.
  const listedJoins = new Map();
  // One room list per edition: named rooms that wait for a guest, oldest first, and the watching sockets.
  const lobbies = { original: newLobby(), hd: newLobby() };
  let shuttingDown = false;
  const wss = new WebSocketServer({ noServer: true, maxPayload, perMessageDeflate: false, autoPong: false,
    maxFragments: 128, maxBufferedChunks: 128, closeTimeout: 1000 });
  // Lobby sockets only receive. They have their own bounds and never count as room connections.
  const lobbyWss = new WebSocketServer({ noServer: true, maxPayload: 1024, perMessageDeflate: false, autoPong: false,
    maxFragments: 4, maxBufferedChunks: 4, closeTimeout: 1000 });

  function newLobby() {
    return { rooms: new Map(), sockets: new Set(), timer: null, lastSent: 0, text: null };
  }
  function json(ws, value) {
    if (ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify(value), error => {
      if (error) ws.terminate();
    });
  }
  /** Full snapshot, newest first. Cached until the list changes. */
  function snapshot(lobby) {
    lobby.text ??= JSON.stringify({ type: 'rooms', rooms: [...lobby.rooms.values()].reverse().slice(0, MAX_LISTED)
      .map(room => ({ id: room.token, name: room.listed.name, engine: room.listed.engine, created: room.listed.created })) });
    return lobby.text;
  }
  function sendSnapshot(ws, text) {
    if (ws.readyState !== WebSocket.OPEN) return;
    // A reader that does not keep up with the list is dropped; it reconnects for a fresh snapshot.
    if (ws.bufferedAmount + text.length > maxBufferedBytes) return ws.terminate();
    ws.send(text, error => { if (error) ws.terminate(); });
  }
  function broadcast(lobby) {
    lobby.timer = null;
    lobby.lastSent = performance.now();
    const text = snapshot(lobby);
    for (const ws of lobby.sockets) sendSnapshot(ws, text);
  }
  // At most one snapshot per SNAPSHOT_MS to all sockets; the last change always goes out.
  function lobbyChanged(lobby) {
    lobby.text = null;
    if (lobby.timer) return;
    const wait = lobby.lastSent + SNAPSHOT_MS - performance.now();
    if (wait <= 0) return broadcast(lobby);
    lobby.timer = setTimeout(() => broadcast(lobby), wait);
    lobby.timer.unref();
  }
  function unlist(room) {
    if (room.listed && room.lobby.rooms.delete(room.id)) lobbyChanged(room.lobby);
  }
  function clearQueue(ws) {
    clearTimeout(ws.deliveryTimer);
    ws.deliveryTimer = null;
    ws.queue = [];
    ws.queuedBytes = 0;
  }
  function endRoom(room, departing, message = 'The other player disconnected.') {
    if (!room || room.ended) return;
    room.ended = true;
    rooms.delete(room.id);
    unlist(room);
    clearTimeout(room.waitingTimer);
    clearTimeout(room.lifetimeTimer);
    for (const ws of [room.host, room.guest]) {
      if (!ws) continue;
      clearQueue(ws);
      if (ws !== departing) json(ws, { type: 'peer-left', message });
      if (ws.readyState === WebSocket.OPEN) ws.close(1001, 'Room closed');
    }
  }
  function reject(ws, message, code = 1008) {
    json(ws, { type: 'error', message });
    ws.close(code, message.slice(0, 100));
    endRoom(ws.room, ws);
  }
  // Control frames and keepalive texts bypass the normal message rules. Bound
  // them together, and reply only after checking outbound backpressure.
  function controlAllowed(ws, bytes) {
    const now = performance.now();
    if (now - ws.controlRateStart >= 1000) {
      ws.controlRateStart = now; ws.controlRateBytes = 0; ws.controlRateMessages = 0;
    }
    ws.controlRateBytes += bytes;
    ws.controlRateMessages++;
    return ws.controlRateMessages <= maxControlFrames && ws.controlRateBytes <= 4096;
  }
  function receiveControl(ws, data, isPing) {
    if (ws.readyState !== WebSocket.OPEN) return;
    // Count the two-byte frame header too.
    if (!controlAllowed(ws, data.length + 2)) return reject(ws, 'Control frame rate exceeded.');
    if (isPing) {
      if (ws.bufferedAmount + data.length + 2 > maxBufferedBytes) return reject(ws, 'Peer is too slow.');
      ws.pong(data, false, error => { if (error) ws.terminate(); });
    } else {
      // Browsers automatically answer our heartbeat pings with pong frames.
      ws.alive = true;
    }
  }
  // Browsers cannot send ping frames. A waiting client sends the text "ping"
  // instead; the Cloudflare relay answers it in its runtime in the same way.
  function isKeepalive(data, isBinary) {
    return !isBinary && data.length === 4 && data.toString() === 'ping';
  }
  function keepalive(ws) {
    if (!controlAllowed(ws, 6)) return reject(ws, 'Control frame rate exceeded.');
    if (ws.bufferedAmount + 6 > maxBufferedBytes) return reject(ws, 'Peer is too slow.');
    ws.lastKeepalive = performance.now();
    ws.send('pong', error => { if (error) ws.terminate(); });
  }
  // Protocol pings do not show a frozen page: the browser answers them. A listed
  // host's page must send its own "ping" texts, as on the Cloudflare relay.
  function silent(room) {
    return performance.now() - room.host.lastKeepalive > listedSilenceMs;
  }
  function closeSilent(room) {
    json(room.host, { type: 'error', message: SILENT_HOST });
    endRoom(room, room.host);
  }
  /** Counts a join of a listed room; false when the address has joined too many recently. */
  function countJoin(ip) {
    const now = performance.now();
    const times = (listedJoins.get(ip) ?? []).filter(time => now - time < listedJoinWindowMs);
    listedJoins.delete(ip);
    const allowed = times.length < maxListedJoins;
    if (allowed) times.push(now);
    listedJoins.set(ip, times);
    while (listedJoins.size > MAX_JOIN_ADDRESSES) listedJoins.delete(listedJoins.keys().next().value);
    return allowed;
  }
  function deliver(ws) {
    ws.deliveryTimer = null;
    const room = ws.room;
    if (!room || room.ended) return clearQueue(ws);
    const peer = ws.role === 'host' ? room.guest : room.host;
    while (ws.queue.length && ws.queue[0].due <= performance.now()) {
      const next = ws.queue.shift();
      ws.queuedBytes -= next.data.length;
      if (!peer || peer.readyState !== WebSocket.OPEN) return endRoom(room, peer);
      if (peer.bufferedAmount + next.data.length > maxBufferedBytes) return reject(ws, 'Peer is too slow.');
      peer.send(next.data, { binary: true, compress: false }, error => {
        if (error) endRoom(room, peer);
      });
    }
    if (ws.queue.length) {
      ws.deliveryTimer = setTimeout(() => deliver(ws), Math.max(1, ws.queue[0].due - performance.now()));
      ws.deliveryTimer.unref();
    }
  }
  function receive(ws, data, isBinary) {
    const room = ws.room;
    if (!room || room.ended) return;
    if (isKeepalive(data, isBinary)) return keepalive(ws);
    if (!room.guest) return reject(ws, 'Wait for the other player before sending game data.');
    if (!isBinary || data.length === 0) return reject(ws, 'Only nonempty binary game data is accepted.');
    const now = performance.now();
    if (now - ws.rateStart >= 1000) { ws.rateStart = now; ws.rateBytes = 0; ws.rateMessages = 0; }
    ws.rateBytes += data.length;
    ws.rateMessages++;
    if (ws.rateBytes > 1048576 || ws.rateMessages > 1000) return reject(ws, 'Game data rate exceeded.');
    const peer = ws.role === 'host' ? room.guest : room.host;
    if (peer.readyState !== WebSocket.OPEN) return endRoom(room, peer);
    if (peer.bufferedAmount + ws.queuedBytes + data.length > maxBufferedBytes || ws.queue.length >= 1024) {
      return reject(ws, 'Peer is too slow.');
    }
    room.lastData = now;
    const delay = latencyPatternMs[ws.sequence++ % latencyPatternMs.length];
    // Ordered transport: jitter may delay later chunks, but must never reorder them.
    const due = Math.max(now + delay, ws.queue.at(-1)?.due ?? 0);
    ws.queue.push({ data, due });
    ws.queuedBytes += data.length;
    if (!ws.deliveryTimer) deliver(ws);
  }
  function watchControlFrames(ws) {
    Object.assign(ws, { alive: true, controlRateStart: performance.now(), controlRateBytes: 0, controlRateMessages: 0 });
    ws.on('ping', data => receiveControl(ws, data, true));
    ws.on('pong', data => receiveControl(ws, data, false));
  }
  function acceptLobby(ws, req, ip) {
    lobbyIpCounts.set(ip, (lobbyIpCounts.get(ip) ?? 0) + 1);
    watchControlFrames(ws);
    ws.on('error', () => ws.terminate());
    ws.on('close', () => {
      const count = (lobbyIpCounts.get(ip) ?? 1) - 1;
      if (count) lobbyIpCounts.set(ip, count); else lobbyIpCounts.delete(ip);
      ws.lobby?.sockets.delete(ws);
    });
    ws.on('message', (data, isBinary) => {
      if (isKeepalive(data, isBinary)) return keepalive(ws);
      reject(ws, 'The room list accepts no messages.');
    });
    const match = LOBBY_URL.exec(req.url);
    if (!match) return reject(ws, 'Invalid lobby URL.');
    ws.lobby = lobbies[match[1]];
    ws.lobby.sockets.add(ws);
    sendSnapshot(ws, snapshot(ws.lobby));
  }
  function acceptRoom(ws, req, ip) {
    ipCounts.set(ip, (ipCounts.get(ip) ?? 0) + 1);
    watchControlFrames(ws);
    Object.assign(ws, { queue: [], queuedBytes: 0, deliveryTimer: null, lastKeepalive: performance.now(),
      sequence: 0, rateStart: performance.now(), rateBytes: 0, rateMessages: 0 });
    ws.on('error', () => endRoom(ws.room, ws));
    ws.on('close', () => {
      const count = (ipCounts.get(ip) ?? 1) - 1;
      if (count) ipCounts.set(ip, count); else ipCounts.delete(ip);
      clearQueue(ws);
      endRoom(ws.room, ws);
    });
    ws.on('message', (data, isBinary) => receive(ws, data, isBinary));
    // HD and Original engines cannot stay in sync. Namespace active and
    // retired rooms by edition, even when their random token is identical.
    const parsed = parseRoomURL(req.url);
    if (parsed.error) return reject(ws, parsed.error);
    const { edition, token, role, name, engine } = parsed;
    const id = (edition ? edition + ':' : '') + token;
    const lobby = lobbies[edition ?? 'original'];
    let room = rooms.get(id);
    if (role === 'host') {
      if (usedRoomIds.has(id)) return reject(ws, 'Room already exists or has expired. Create a new room.');
      if (rooms.size >= maxRooms || usedRoomIds.size >= maxRoomIds) return reject(ws, 'Room limit reached.');
      if (name) {
        if (lobby.rooms.size >= maxListedRooms) return reject(ws, LISTING_DENIED.full);
        let mine = 0;
        for (const listed of lobby.rooms.values()) if (listed.ip === ip) mine++;
        if (mine >= maxListedPerIp) return reject(ws, LISTING_DENIED.ip);
      }
      usedRoomIds.add(id);
      room = { id, token, ip, lobby, listed: name ? { name, engine, created: Date.now() } : null,
        host: ws, guest: null, ended: false, lastData: performance.now() };
      rooms.set(id, room);
      room.waitingTimer = setTimeout(() => {
        json(ws, { type: 'error', message: 'Nobody joined before the room expired.' });
        endRoom(room, ws);
      }, room.listed ? listedWaitingMs : waitingMs);
      room.waitingTimer.unref();
      room.lifetimeTimer = setTimeout(() => endRoom(room, null, 'Room time limit reached.'), lifetimeMs);
      room.lifetimeTimer.unref();
    } else {
      if (!room || room.ended || room.host.readyState !== WebSocket.OPEN) return reject(ws, 'Room does not exist or has expired.');
      if (room.guest) return reject(ws, 'Room already has two players.');
      if (room.listed) {
        // A guest who picks a listed room must find a host that still answers.
        if (silent(room)) {
          closeSilent(room);
          return reject(ws, 'Room does not exist or has expired.');
        }
        if (!countJoin(ip)) return reject(ws, TOO_MANY_JOINS);
      }
      room.guest = ws;
      clearTimeout(room.waitingTimer);
    }
    ws.room = room;
    ws.role = role;
    json(ws, { type: 'waiting' });
    if (room.guest) {
      room.lastData = performance.now();
      json(room.host, { type: 'ready', role: 'host' });
      json(room.guest, { type: 'ready', role: 'guest' });
      unlist(room);
    } else if (room.listed) {
      // Listed after the host socket exists, so a guest who picks it always finds the host.
      lobby.rooms.set(id, room);
      lobbyChanged(lobby);
    }
  }

  async function serve(req, res) {
    Object.entries(isolatedHeaders).forEach(([key, value]) => res.setHeader(key, value));
    if (!['GET', 'HEAD'].includes(req.method)) { res.writeHead(405, { Allow: 'GET, HEAD' }); return res.end(); }
    if (!distRoot) { res.writeHead(404); return res.end(); }
    try {
      const rawPath = req.url.split('?')[0];
      const decoded = decodeURIComponent(rawPath);
      if (!decoded.startsWith('/') || decoded.includes('\\') || decoded.includes('\0') ||
          decoded.split('/').some(segment => segment.startsWith('.'))) throw new Error('Invalid path');
      const root = await realpath(distRoot);
      const file = await realpath(path.join(root, decoded === '/' ? 'index.html' : decoded));
      if (!file.startsWith(root + path.sep)) throw new Error('Outside document root');
      const info = await stat(file);
      if (!info.isFile()) throw new Error('Not a file');
      res.writeHead(200, { 'Content-Type': MIME[path.extname(file)] ?? 'application/octet-stream',
        'Content-Length': info.size });
      if (req.method === 'HEAD') return res.end();
      createReadStream(file).on('error', () => res.destroy()).pipe(res);
    } catch { res.writeHead(404); res.end('Not found'); }
  }
  const server = http.createServer({ maxHeaderSize: 8192, requestTimeout: 15000,
    headersTimeout: 10000, keepAliveTimeout: 5000 }, (req, res) => { void serve(req, res); });
  server.maxConnections = maxRooms * 2 + maxLobbyConnections + 64;
  server.on('clientError', (_error, socket) => socket.destroy());
  server.on('upgrade', (req, socket, head) => {
    function deny(status, message) {
      const body = JSON.stringify({ type: 'error', message });
      socket.end(`HTTP/1.1 ${status}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
    }
    let origin;
    try { origin = new URL(req.headers.origin); } catch { return deny('403 Forbidden', 'Origin required.'); }
    const ownOrigin = `${req.socket.encrypted ? 'https' : 'http'}://${req.headers.host}`;
    if (origin.origin !== req.headers.origin || !(allowedOrigins ? allowedOrigins.has(origin.origin) : origin.origin === ownOrigin)) {
      return deny('403 Forbidden', 'Origin is not allowed.');
    }
    // Every per-address bound (room and lobby sockets, listed rooms, joins) uses this key.
    const ip = addressKey(req.socket.remoteAddress);
    const toLobby = req.url.split('?')[0] === '/netplay/lobby';
    const full = toLobby
      ? lobbyWss.clients.size >= maxLobbyConnections || (lobbyIpCounts.get(ip) ?? 0) >= maxLobbyPerIp
      : wss.clients.size >= maxRooms * 2 || (ipCounts.get(ip) ?? 0) >= maxPerIp;
    if (shuttingDown || full) return deny('503 Service Unavailable', 'Connection limit reached.');
    // Browsers use the standard WebSocket protocol; arbitrary proxy subprotocols are unsupported.
    if (req.headers['sec-websocket-protocol']) return deny('400 Bad Request', 'Subprotocols are unsupported.');
    if (toLobby) lobbyWss.handleUpgrade(req, socket, head, ws => acceptLobby(ws, req, ip));
    else wss.handleUpgrade(req, socket, head, ws => acceptRoom(ws, req, ip));
  });
  const heartbeat = setInterval(() => {
    const now = performance.now();
    for (const room of rooms.values()) {
      if (room.guest && now - room.lastData > idleMs) endRoom(room, null, 'Room was idle for too long.');
    }
    // Liveness check of the room lists: every listed room still waits with a connected host that pings.
    for (const lobby of Object.values(lobbies)) {
      for (const room of [...lobby.rooms.values()]) {
        if (room.ended || room.guest || room.host.readyState !== WebSocket.OPEN) unlist(room);
        else if (silent(room)) closeSilent(room);
      }
    }
    for (const [ip, times] of listedJoins) {
      if (now - times.at(-1) >= listedJoinWindowMs) listedJoins.delete(ip);
    }
    for (const ws of [...wss.clients, ...lobbyWss.clients]) {
      if (ws.readyState !== WebSocket.OPEN) continue;
      if (!ws.alive) { endRoom(ws.room, ws); ws.terminate(); continue; }
      ws.alive = false;
      ws.ping(undefined, undefined, error => { if (error) ws.terminate(); });
    }
  }, heartbeatMs);
  heartbeat.unref();

  return {
    server,
    get stats() {
      return { rooms: rooms.size, connections: wss.clients.size, usedRoomIds: usedRoomIds.size,
        lobbyConnections: lobbyWss.clients.size, listedRooms: lobbies.original.rooms.size + lobbies.hd.rooms.size };
    },
    async listen(port = 9999, host = '127.0.0.1') {
      await new Promise((resolve, reject) => {
        server.once('error', reject);
        server.listen(port, host, () => { server.off('error', reject); resolve(); });
      });
      return server.address();
    },
    async close() {
      shuttingDown = true;
      clearInterval(heartbeat);
      for (const room of rooms.values()) endRoom(room, null, 'Server is stopping.');
      for (const lobby of Object.values(lobbies)) clearTimeout(lobby.timer);
      for (const ws of [...wss.clients, ...lobbyWss.clients]) ws.terminate();
      await Promise.all([new Promise(resolve => wss.close(resolve)), new Promise(resolve => lobbyWss.close(resolve))]);
      if (server.listening) {
        server.closeAllConnections();
        await new Promise(resolve => server.close(resolve));
      }
    }
  };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  const config = {};
  let host = '127.0.0.1';
  let port = 9999;
  for (let i = 0; i < args.length; i++) {
    const flag = args[i];
    if (flag === '--help') {
      console.log('Usage: npm start -- [--host 127.0.0.1] [--port 9999] [--origin https://game.example] [--dist DIR]');
      process.exit(0);
    }
    if (!['--host', '--port', '--origin', '--dist'].includes(flag) || !args[i + 1]) throw new Error(`Unknown or incomplete option: ${flag}`);
    const value = args[++i];
    if (flag === '--host') host = value;
    if (flag === '--port') port = integer(Number(value), 9999, 0, 65535, 'port');
    if (flag === '--origin') (config.allowedOrigins ??= []).push(value);
    if (flag === '--dist') config.distDir = value;
  }
  const app = createNetplayServer(config);
  const address = await app.listen(port, host);
  console.log(`UQM game, Melee relay and room lists: http://${host.includes(':') ? `[${host}]` : host}:${address.port}/`);
  let closing = false;
  for (const signal of ['SIGINT', 'SIGTERM']) process.on(signal, async () => {
    if (closing) return;
    closing = true;
    await app.close();
  });
}
