// Test-only lobby in front of a room relay. It implements the lobby contract
// (docs/online-ui.md): a live room list at /netplay/lobby?edition=original|hd,
// and public rooms created by hosts that add name= and engine= to the room URL.
// Room traffic is forwarded unchanged (without the lobby parameters) to the
// relay behind it, so the real relay pairs the players.
import http from 'node:http';
import net from 'node:net';
import { WebSocketServer, WebSocket } from 'ws';

const ROOM = /^\/netplay\/(?:(hd)\/)?([0-9a-f]{32})\?(.*)$/;
const CONTROL = /[\u0000-\u001f\u007f-\u009f]/;

export function createLobbyStub(backend) {
  const listings = { original: new Map(), hd: new Map() };
  const watchers = new Set();
  const sockets = new Set();
  const wss = new WebSocketServer({ noServer: true });
  let proxy, port, timer = null, available = true;

  function snapshot(edition) {
    return [...listings[edition].values()].sort((a, b) => b.created - a.created).slice(0, 50)
      .map(({ id, name, engine, created }) => ({ id, name, engine, created }));
  }
  function send(ws) {
    if (ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'rooms', rooms: snapshot(ws.edition) }));
  }
  function changed() {
    if (timer) return;
    timer = setTimeout(() => { timer = null; for (const ws of watchers) send(ws); }, 300);
  }
  function deny(socket, status, message) {
    const body = JSON.stringify({ type: 'error', message });
    socket.end(`HTTP/1.1 ${status}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
  }
  function forward(req, socket, head, url) {
    const upstream = net.connect(port, '127.0.0.1', () => {
      const headers = req.rawHeaders.reduce((text, value, i) => text + value + (i % 2 ? '\r\n' : ': '), '');
      upstream.write(`${req.method} ${url} HTTP/${req.httpVersion}\r\n${headers}\r\n`);
      if (head.length) upstream.write(head);
      socket.pipe(upstream).pipe(socket);
    });
    upstream.on('error', () => socket.destroy());
    socket.on('error', () => upstream.destroy());
    socket.on('close', () => upstream.destroy());
    return upstream;
  }

  return {
    listings,
    get watchers() { return watchers.size; },
    // Rooms without a host, for screenshots of a busy lobby.
    addFakeRoom(edition, room) {
      listings[edition].set(room.id, { id: room.id, name: room.name, engine: room.engine ?? null,
        created: room.created ?? Date.now(), fake: true });
      changed();
    },
    removeRoom(edition, id) { listings[edition].delete(id); changed(); },
    clear() { listings.original.clear(); listings.hd.clear(); changed(); },
    // false: lobby connections are refused, like a relay without a lobby.
    setAvailable(value) {
      available = value;
      if (!value) for (const ws of watchers) ws.close(1011, 'Lobby off');
    },
    async listen(listenPort = 0) {
      port = (await backend.listen(0)).port;
      proxy = http.createServer((req, res) => {
        const upstream = http.request({ host: '127.0.0.1', port, path: req.url, method: req.method, headers: req.headers }, response => {
          res.writeHead(response.statusCode, response.headers);
          response.pipe(res);
        });
        upstream.on('error', () => { if (!res.headersSent) res.writeHead(502); res.end(); });
        res.on('close', () => upstream.destroy());
        req.pipe(upstream);
      });
      proxy.on('connection', socket => { sockets.add(socket); socket.on('close', () => sockets.delete(socket)); });
      proxy.on('upgrade', (req, socket, head) => {
        const ownOrigin = `http://${req.headers.host}`;
        if (req.headers.origin !== ownOrigin) return deny(socket, '403 Forbidden', 'Origin is not allowed.');
        const url = new URL(req.url, ownOrigin);
        if (url.pathname === '/netplay/lobby') {
          const edition = url.searchParams.get('edition');
          if (!available) return deny(socket, '503 Service Unavailable', 'Lobby unavailable.');
          if (edition !== 'original' && edition !== 'hd') return deny(socket, '400 Bad Request', 'Unknown edition.');
          wss.handleUpgrade(req, socket, head, ws => {
            ws.edition = edition;
            watchers.add(ws);
            ws.on('close', () => watchers.delete(ws));
            send(ws);
          });
          return;
        }
        const match = ROOM.exec(req.url);
        if (!match) return forward(req, socket, head, req.url);
        const [, hd, token, query] = match;
        const edition = hd ? 'hd' : 'original';
        const params = new URLSearchParams(query);
        const role = params.get('role');
        const target = `/netplay/${hd ? 'hd/' : ''}${token}?role=${role}`;
        if (role === 'guest') {
          if (listings[edition].delete(token)) changed();
          return forward(req, socket, head, target);
        }
        const name = params.has('name') ? params.get('name').trim() : null;
        if (name !== null && (!name.length || Array.from(name).length > 24 || CONTROL.test(name))) {
          wss.handleUpgrade(req, socket, head, ws => {
            ws.send(JSON.stringify({ type: 'error', message: 'Room names need 1 to 24 characters.' }));
            ws.close(1008, 'Bad room name');
          });
          return;
        }
        const upstream = forward(req, socket, head, target);
        if (name) {
          const engine = ['c', 'rust'].includes(params.get('engine')) ? params.get('engine') : null;
          listings[edition].set(token, { id: token, name, engine, created: Date.now() });
          changed();
          const remove = () => { if (listings[edition].get(token)?.fake !== true && listings[edition].delete(token)) changed(); };
          socket.on('close', remove);
          upstream.on('close', remove);
        }
      });
      await new Promise((resolve, reject) => { proxy.once('error', reject); proxy.listen(listenPort, '127.0.0.1', resolve); });
      return proxy.address();
    },
    async close() {
      clearTimeout(timer);
      for (const ws of watchers) ws.terminate();
      for (const socket of sockets) socket.destroy();
      if (proxy?.listening) await new Promise(resolve => proxy.close(resolve));
      await backend.close();
    },
  };
}
