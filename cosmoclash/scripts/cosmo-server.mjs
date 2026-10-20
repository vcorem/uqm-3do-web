// Local web server for Cosmo Clash: the built game at /cosmoclash/ (as on
// urquanmasters.org/cosmoclash/) and online play at /netplay/ on the same
// host. The relay is the site's Node relay (server/netplay.mjs, read-only use)
// or any relay with the same contract (tests use the Rust relay too).
//
// lobby: 'auto'    use the relay's own cosmoclash room list when it has one,
//                  else put the lobby adapter (cosmo-lobby.mjs) in front;
//        'adapter' always the adapter;
//        'relay'   never the adapter (a relay without room lists then makes
//                  the game offer private rooms with invite links).
import http from 'node:http';
import net from 'node:net';
import { createReadStream } from 'node:fs';
import { realpath, stat } from 'node:fs/promises';
import path from 'node:path';
import { WebSocket } from 'ws';
import { createCosmoLobby } from './cosmo-lobby.mjs';

const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.css': 'text/css; charset=utf-8',
  '.wasm': 'application/wasm', '.json': 'application/json', '.glb': 'model/gltf-binary', '.ogg': 'audio/ogg', '.wav': 'audio/wav',
  '.png': 'image/png', '.jpg': 'image/jpeg', '.svg': 'image/svg+xml', '.ico': 'image/x-icon', '.txt': 'text/plain; charset=utf-8' };
// The same isolation headers as the site (deploy/worker.js).
const HEADERS = { 'Cross-Origin-Opener-Policy': 'same-origin', 'Cross-Origin-Embedder-Policy': 'require-corp',
  'Cross-Origin-Resource-Policy': 'same-origin', 'X-Content-Type-Options': 'nosniff', 'Cache-Control': 'no-store' };

/** True when the relay answers the cosmoclash lobby URL with a room list. */
export function relayHasLobby(port, host = '127.0.0.1', timeoutMs = 3000) {
  return new Promise(resolve => {
    const ws = new WebSocket(`ws://${host}:${port}/netplay/lobby?edition=cosmoclash`, { origin: `http://${host}:${port}` });
    const done = value => { clearTimeout(timer); ws.removeAllListeners(); ws.on('error', () => {}); ws.terminate(); resolve(value); };
    const timer = setTimeout(() => done(false), timeoutMs);
    ws.on('message', data => { try { done(JSON.parse(data.toString()).type === 'rooms'); } catch { done(false); } });
    ws.on('error', () => done(false));
    ws.on('close', () => done(false));
    ws.on('unexpected-response', () => done(false));
  });
}

export function createCosmoServer({ distDir, relay, lobby = 'auto', base = '/cosmoclash/', lobbyOptions = {} }) {
  if (!base.startsWith('/') || !base.endsWith('/')) throw new Error('The base path needs a slash at both ends.');
  let adapter = null;
  let relayPort = 0;
  const sockets = new Set();
  async function serveFile(req, res) {
    for (const [key, value] of Object.entries(HEADERS)) res.setHeader(key, value);
    if (!['GET', 'HEAD'].includes(req.method)) { res.writeHead(405, { Allow: 'GET, HEAD' }); return res.end(); }
    const url = new URL(req.url, 'http://local');
    if (url.pathname === '/' || url.pathname === base.slice(0, -1)) {
      res.writeHead(url.pathname === '/' ? 302 : 301, { Location: base + url.search });
      return res.end();
    }
    try {
      if (!url.pathname.startsWith(base)) throw new Error('Outside the game');
      const decoded = decodeURIComponent(url.pathname.slice(base.length - 1));
      if (decoded.includes('\\') || decoded.includes('\0') || decoded.split('/').some(part => part.startsWith('.'))) throw new Error('Invalid path');
      const root = await realpath(distDir);
      let file = await realpath(path.join(root, decoded));
      if (file !== root && !file.startsWith(root + path.sep)) throw new Error('Outside the game');
      let info = await stat(file);
      if (info.isDirectory()) { file = path.join(file, 'index.html'); info = await stat(file); }
      if (!info.isFile()) throw new Error('Not a file');
      res.writeHead(200, { 'Content-Type': MIME[path.extname(file)] ?? 'application/octet-stream', 'Content-Length': info.size });
      if (req.method === 'HEAD') return res.end();
      createReadStream(file).on('error', () => res.destroy()).pipe(res);
    } catch {
      res.writeHead(404, { 'Content-Type': 'text/plain; charset=utf-8' });
      res.end('Not found');
    }
  }
  const server = http.createServer((req, res) => { void serveFile(req, res); });
  server.on('connection', socket => { sockets.add(socket); socket.on('close', () => sockets.delete(socket)); });
  server.on('upgrade', (req, socket, head) => {
    if (!req.url.startsWith('/netplay/')) { socket.destroy(); return; }
    const ownOrigin = `http://${req.headers.host}`;
    if (req.headers.origin !== ownOrigin) {
      const body = JSON.stringify({ type: 'error', message: 'Origin is not allowed.' });
      socket.end(`HTTP/1.1 403 Forbidden\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
      return;
    }
    if (adapter) { adapter.upgrade(req, socket, head, socket.remoteAddress); return; }
    // The relay itself: raw TCP, so that it sees the browser's own request.
    const upstream = net.connect(relayPort, '127.0.0.1', () => {
      const headers = req.rawHeaders.reduce((text, value, i) => text + value + (i % 2 ? '\r\n' : ': '), '');
      upstream.write(`${req.method} ${req.url} HTTP/${req.httpVersion}\r\n${headers}\r\n`);
      if (head.length) upstream.write(head);
      socket.pipe(upstream).pipe(socket);
    });
    upstream.on('error', () => socket.destroy());
    socket.on('error', () => upstream.destroy());
    socket.on('close', () => upstream.destroy());
  });
  return {
    server,
    get adapter() { return adapter; },
    /** 'relay' when the relay has the cosmoclash room list, 'adapter' when this server adds it, 'none'. */
    get lobby() { return adapter ? 'adapter' : lobby === 'relay' ? 'none' : 'relay'; },
    async listen(port = 10000, host = '127.0.0.1') {
      relayPort = (await relay.listen(0, '127.0.0.1')).port;
      const useAdapter = lobby === 'adapter' || (lobby === 'auto' && !(await relayHasLobby(relayPort)));
      if (useAdapter) adapter = createCosmoLobby({ backendPort: relayPort, ...lobbyOptions });
      await new Promise((resolve, reject) => { server.once('error', reject); server.listen(port, host, () => { server.off('error', reject); resolve(); }); });
      return server.address();
    },
    async close() {
      adapter?.close();
      for (const socket of sockets) socket.destroy();
      if (server.listening) await new Promise(resolve => server.close(resolve));
      await relay.close();
    },
  };
}
