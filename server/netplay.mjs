import http from 'node:http';
import { createReadStream } from 'node:fs';
import { realpath, stat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { WebSocketServer, WebSocket } from 'ws';

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8',
  '.wasm': 'application/wasm', '.data': 'application/octet-stream', '.json': 'application/json',
  '.css': 'text/css; charset=utf-8', '.glb': 'model/gltf-binary', '.ogg': 'audio/ogg', '.wav': 'audio/wav', '.png': 'image/png', '.jpg': 'image/jpeg', '.svg': 'image/svg+xml' };
const isolatedHeaders = { 'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp', 'Cross-Origin-Resource-Policy': 'same-origin',
  'X-Content-Type-Options': 'nosniff', 'Cache-Control': 'no-store' };

function integer(value, fallback, min, max, name) {
  const result = value ?? fallback;
  if (!Number.isSafeInteger(result) || result < min || result > max) throw new Error(`Invalid ${name}`);
  return result;
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
  let shuttingDown = false;
  const wss = new WebSocketServer({ noServer: true, maxPayload, perMessageDeflate: false, autoPong: false,
    maxFragments: 128, maxBufferedChunks: 128, closeTimeout: 1000 });

  function json(ws, value) {
    if (ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify(value), error => {
      if (error) ws.terminate();
    });
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
  function receiveControl(ws, data, isPing) {
    if (ws.readyState !== WebSocket.OPEN) return;
    const now = performance.now();
    if (now - ws.controlRateStart >= 1000) {
      ws.controlRateStart = now; ws.controlRateBytes = 0; ws.controlRateMessages = 0;
    }
    // Control frames bypass the normal message handler. Bound both pings and
    // unsolicited pongs, and reply only after checking outbound backpressure.
    ws.controlRateBytes += data.length + 2;
    ws.controlRateMessages++;
    if (ws.controlRateMessages > maxControlFrames || ws.controlRateBytes > 4096) {
      return reject(ws, 'Control frame rate exceeded.');
    }
    if (isPing) {
      if (ws.bufferedAmount + data.length + 2 > maxBufferedBytes) return reject(ws, 'Peer is too slow.');
      ws.pong(data, false, error => { if (error) ws.terminate(); });
    } else {
      // Browsers automatically answer our heartbeat pings with pong frames.
      ws.alive = true;
    }
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
  server.maxConnections = maxRooms * 2 + 64;
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
    const ip = req.socket.remoteAddress;
    if (shuttingDown || wss.clients.size >= maxRooms * 2 || (ipCounts.get(ip) ?? 0) >= maxPerIp) {
      return deny('503 Service Unavailable', 'Connection limit reached.');
    }
    // Browsers use the standard WebSocket protocol; arbitrary proxy subprotocols are unsupported.
    if (req.headers['sec-websocket-protocol']) return deny('400 Bad Request', 'Subprotocols are unsupported.');
    wss.handleUpgrade(req, socket, head, ws => {
      ipCounts.set(ip, (ipCounts.get(ip) ?? 0) + 1);
      Object.assign(ws, { alive: true, queue: [], queuedBytes: 0, deliveryTimer: null,
        sequence: 0, rateStart: performance.now(), rateBytes: 0, rateMessages: 0,
        controlRateStart: performance.now(), controlRateBytes: 0, controlRateMessages: 0 });
      ws.on('ping', data => receiveControl(ws, data, true));
      ws.on('pong', data => receiveControl(ws, data, false));
      ws.on('error', () => endRoom(ws.room, ws));
      ws.on('close', () => {
        const count = (ipCounts.get(ip) ?? 1) - 1;
        if (count) ipCounts.set(ip, count); else ipCounts.delete(ip);
        clearQueue(ws);
        endRoom(ws.room, ws);
      });
      ws.on('message', (data, isBinary) => receive(ws, data, isBinary));
      // Different games and editions cannot stay in sync. Namespace active and
      // retired rooms by edition, even when their random token is identical.
      const match = /^\/netplay\/(?:(hd|cosmoclash)\/)?([0-9a-f]{32})\?role=(host|guest)$/.exec(req.url);
      if (!match) return reject(ws, 'Invalid room URL or role.');
      const [, edition, token, role] = match;
      const id = (edition ? edition + ':' : '') + token;
      let room = rooms.get(id);
      if (role === 'host') {
        if (usedRoomIds.has(id)) return reject(ws, 'Room already exists or has expired. Create a new room.');
        if (rooms.size >= maxRooms || usedRoomIds.size >= maxRoomIds) return reject(ws, 'Room limit reached.');
        usedRoomIds.add(id);
        room = { id, host: ws, guest: null, ended: false, lastData: performance.now() };
        rooms.set(id, room);
        room.waitingTimer = setTimeout(() => {
          json(ws, { type: 'error', message: 'Nobody joined before the room expired.' });
          endRoom(room, ws);
        }, waitingMs);
        room.waitingTimer.unref();
        room.lifetimeTimer = setTimeout(() => endRoom(room, null, 'Room time limit reached.'), lifetimeMs);
        room.lifetimeTimer.unref();
      } else {
        if (!room || room.ended || room.host.readyState !== WebSocket.OPEN) return reject(ws, 'Room does not exist or has expired.');
        if (room.guest) return reject(ws, 'Room already has two players.');
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
      }
    });
  });
  const heartbeat = setInterval(() => {
    const now = performance.now();
    for (const room of rooms.values()) {
      if (room.guest && now - room.lastData > idleMs) endRoom(room, null, 'Room was idle for too long.');
    }
    for (const ws of wss.clients) {
      if (ws.readyState !== WebSocket.OPEN) continue;
      if (!ws.alive) { endRoom(ws.room, ws); ws.terminate(); continue; }
      ws.alive = false;
      ws.ping(undefined, undefined, error => { if (error) ws.terminate(); });
    }
  }, heartbeatMs);
  heartbeat.unref();

  return {
    server,
    get stats() { return { rooms: rooms.size, connections: wss.clients.size, usedRoomIds: usedRoomIds.size }; },
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
      for (const ws of wss.clients) ws.terminate();
      await new Promise(resolve => wss.close(resolve));
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
  console.log(`UQM game and private Melee relay: http://${host.includes(':') ? `[${host}]` : host}:${address.port}/`);
  let closing = false;
  for (const signal of ['SIGINT', 'SIGTERM']) process.on(signal, async () => {
    if (closing) return;
    closing = true;
    await app.close();
  });
}
