// The Cosmo Clash room list in front of a room relay, for local play and tests.
// It follows the lobby contract of the site (docs/cosmoclash-online.md):
//
//   /netplay/lobby?edition=cosmoclash
//     server -> {"type":"rooms","rooms":[{id,name,engine,created}]} at connect and on every change
//     client "ping" every 30 s -> server "pong"
//   /netplay/cosmoclash/<token>?role=host&name=<name>&engine=rust   a listed room
//   /netplay/cosmoclash/<token>?role=host                          a private room
//   /netplay/cosmoclash/<token>?role=guest                         join
//
// A listed host pings every 30 s while its socket is open; this lobby answers
// "pong" and closes a listed room whose host is silent for more than 150 s.
// Room traffic goes unchanged (without name and engine) to the relay behind
// it, so the real relay pairs the players. Other paths (the UQM editions and
// their lobbies) go to the relay as raw TCP. When the relay itself has the
// cosmoclash lobby (the platform stream), scripts/cosmo-server.mjs uses the
// relay directly and this file is not used.
import net from 'node:net';
import { WebSocketServer, WebSocket } from 'ws';
// Read-only use of the relay's own URL and name rules.
import { parseRoomURL } from '../../server/netplay.mjs';

export const SILENT_HOST = 'The room closed because this page stopped answering. Create a new room.';
const LISTING_DENIED = {
  full: 'The room list is full. Try again in a few minutes.',
  ip: 'Your network already has the most open rooms allowed. Start or close one first.',
};
const LOBBY_PATH = '/netplay/lobby';
const COSMO_LOBBY = '/netplay/lobby?edition=cosmoclash';
const COSMO_ROOM = /^\/netplay\/cosmoclash\/[0-9a-f]{32}(\?|$)/;
const MAX_LISTED = 50;

export function createCosmoLobby(options) {
  const backendHost = options.backendHost ?? '127.0.0.1';
  const backendPort = options.backendPort;
  const silenceMs = options.silenceMs ?? 150000;
  const checkMs = options.checkMs ?? 5000;
  const snapshotMs = options.snapshotMs ?? 500;
  const maxListed = Math.min(MAX_LISTED, options.maxListed ?? MAX_LISTED);
  const maxListedPerIp = options.maxListedPerIp ?? 3;
  /** token -> { id, name, engine, created, host, upstream, lastPing, ip, fake } (oldest first) */
  const listed = new Map();
  const watchers = new Set();
  const sockets = new Set();
  const wss = new WebSocketServer({ noServer: true, perMessageDeflate: false, maxPayload: 65536 });
  let timer = null, lastSent = 0;

  function snapshot() {
    const rooms = [...listed.values()].reverse().slice(0, MAX_LISTED)
      .map(({ id, name, engine, created }) => ({ id, name, engine, created }));
    return JSON.stringify({ type: 'rooms', rooms });
  }
  function broadcast() {
    timer = null;
    lastSent = Date.now();
    const text = snapshot();
    for (const ws of watchers) if (ws.readyState === WebSocket.OPEN) ws.send(text);
  }
  // At most one snapshot per snapshotMs; the last change always goes out.
  function changed() {
    if (timer) return;
    const wait = lastSent + snapshotMs - Date.now();
    if (wait <= 0) broadcast(); else timer = setTimeout(broadcast, wait);
  }
  function unlist(token) { if (listed.delete(token)) changed(); }
  function sendJSON(ws, value) { if (ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify(value)); }
  function refuse(req, socket, head, message) {
    wss.handleUpgrade(req, socket, head, ws => { sendJSON(ws, { type: 'error', message }); ws.close(1008, message.slice(0, 100)); });
  }
  /** Raw TCP to the relay, request line and headers unchanged except the target. */
  function forward(req, socket, head, target = req.url) {
    const upstream = net.connect(backendPort, backendHost, () => {
      const headers = req.rawHeaders.reduce((text, value, i) => text + value + (i % 2 ? '\r\n' : ': '), '');
      upstream.write(`${req.method} ${target} HTTP/${req.httpVersion}\r\n${headers}\r\n`);
      if (head.length) upstream.write(head);
      socket.pipe(upstream).pipe(socket);
    });
    upstream.on('error', () => socket.destroy());
    socket.on('error', () => upstream.destroy());
    socket.on('close', () => upstream.destroy());
  }
  function acceptLobby(req, socket, head) {
    wss.handleUpgrade(req, socket, head, ws => {
      watchers.add(ws);
      sockets.add(ws);
      ws.on('close', () => { watchers.delete(ws); sockets.delete(ws); });
      ws.on('error', () => ws.terminate());
      ws.on('message', (data, binary) => {
        if (!binary && data.toString() === 'ping') { ws.send('pong'); return; }
        sendJSON(ws, { type: 'error', message: 'The room list accepts no messages.' });
        ws.close(1008, 'No messages');
      });
      ws.send(snapshot());
    });
  }
  /** A WebSocket-level proxy, so that this lobby sees the host's pings and the end of the room. */
  function acceptRoom(req, socket, head, parsed, ip) {
    const { token, role, name, engine } = parsed;
    if (role === 'host' && name) {
      if (listed.size >= maxListed) return refuse(req, socket, head, LISTING_DENIED.full);
      let mine = 0;
      for (const room of listed.values()) if (room.ip === ip) mine++;
      if (mine >= maxListedPerIp) return refuse(req, socket, head, LISTING_DENIED.ip);
    }
    const entry = role === 'guest' ? listed.get(token) : null;
    if (entry && !entry.fake && Date.now() - entry.lastPing > silenceMs) {
      closeSilent(entry);
      return refuse(req, socket, head, 'Room does not exist or has expired.');
    }
    wss.handleUpgrade(req, socket, head, client => {
      sockets.add(client);
      const upstream = new WebSocket(`ws://${backendHost}:${backendPort}/netplay/cosmoclash/${token}?role=${role}`,
        { origin: `http://${backendHost}:${backendPort}`, perMessageDeflate: false });
      sockets.add(upstream);
      const queue = [];
      let room = null;
      if (role === 'host' && name) {
        room = { id: token, name, engine: engine ?? null, created: Date.now(), host: client, upstream, lastPing: Date.now(), ip, fake: false };
      }
      // Listed once the relay has the host (its "waiting"), so that a guest who picks it finds the host.
      upstream.on('message', (data, binary) => {
        if (client.readyState !== WebSocket.OPEN) return;
        if (!binary && room && !listed.has(token) && !room.paired) {
          try { if (JSON.parse(data.toString()).type === 'waiting') { listed.set(token, room); changed(); } } catch { /* Not JSON. */ }
        }
        if (!binary && room) {
          try { if (JSON.parse(data.toString()).type === 'ready') { room.paired = true; unlist(token); } } catch { /* Not JSON. */ }
        }
        client.send(data, { binary });
      });
      upstream.on('open', () => { for (const [data, binary] of queue.splice(0)) upstream.send(data, { binary }); });
      const end = (code, reason) => {
        if (room) unlist(token);
        for (const ws of [client, upstream]) {
          if (ws.readyState === WebSocket.OPEN) ws.close(code >= 1000 && code < 5000 && code !== 1005 && code !== 1006 ? code : 1000, reason);
          else if (ws.readyState === WebSocket.CONNECTING) ws.terminate();
        }
      };
      upstream.on('close', (code, reason) => end(code, reason.toString()));
      upstream.on('error', () => end(1011, 'Relay unavailable'));
      client.on('close', (code, reason) => end(code, reason.toString()));
      client.on('error', () => end(1011, 'Client error'));
      client.on('message', (data, binary) => {
        if (room && !binary && data.length === 4 && data.toString() === 'ping') {
          room.lastPing = Date.now();
          if (client.readyState === WebSocket.OPEN) client.send('pong');
          return;
        }
        if (upstream.readyState === WebSocket.OPEN) upstream.send(data, { binary });
        else if (upstream.readyState === WebSocket.CONNECTING) queue.push([data, binary]);
      });
      if (role === 'guest' && entry) unlist(token);
    });
  }
  function closeSilent(room) {
    unlist(room.id);
    if (room.fake) return;
    sendJSON(room.host, { type: 'error', message: SILENT_HOST });
    room.host.close(1008, 'Silent host');
    room.upstream.close(1000, 'Silent host');
  }
  const check = setInterval(() => {
    for (const room of [...listed.values()]) if (!room.fake && Date.now() - room.lastPing > silenceMs) closeSilent(room);
  }, checkMs);
  check.unref();

  return {
    listed,
    get watchers() { return watchers.size; },
    /** Handles a WebSocket upgrade; false when the path is not a Cosmo Clash path. */
    upgrade(req, socket, head, ip = '127.0.0.1') {
      const query = req.url.indexOf('?');
      const pathname = query < 0 ? req.url : req.url.slice(0, query);
      if (req.url === COSMO_LOBBY) { acceptLobby(req, socket, head); return true; }
      if (pathname === LOBBY_PATH && /[?&]edition=cosmoclash(&|$)/.test(req.url)) { refuse(req, socket, head, 'Invalid lobby URL.'); return true; }
      if (!COSMO_ROOM.test(req.url)) { forward(req, socket, head); return true; }
      const parsed = parseRoomURL(req.url);
      if (parsed.error) { refuse(req, socket, head, parsed.error); return true; }
      acceptRoom(req, socket, head, parsed, ip);
      return true;
    },
    /** A listed room without a host, for screenshots of a busy lobby. */
    addFakeRoom(room) {
      listed.set(room.id, { id: room.id, name: room.name, engine: room.engine ?? 'rust', created: room.created ?? Date.now(), fake: true, lastPing: Date.now() });
      changed();
    },
    removeRoom(id) { unlist(id); },
    clear() { listed.clear(); changed(); },
    close() {
      clearInterval(check);
      clearTimeout(timer);
      for (const ws of sockets) ws.terminate();
      wss.close();
    },
  };
}
