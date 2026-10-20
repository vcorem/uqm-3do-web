// Test-only launcher: exercise unchanged game clients against either relay and
// optionally serve the guest's assets from an independent C-reference build.
import http from 'node:http';
import net from 'node:net';
import { createNetplayServer } from '../../server/netplay.mjs';
import { createRustRelay } from './rust-relay.mjs';

export function createGameBackend(options, peerDist) {
  const kind = process.env.UQM_RELAY || 'node';
  if (!['node', 'rust'].includes(kind)) throw new Error('UQM_RELAY must be node or rust');
  const app = kind === 'rust' ? createRustRelay(options) : createNetplayServer(options);
  if (!peerDist) return app;

  // Both clients must see the same origin for invitations, COOP and COEP. A
  // test cookie chooses the independent guest static build; every WebSocket
  // upgrade still reaches one relay. Streams avoid buffering the 299 MB bundle.
  const oracle = createNetplayServer({ distDir: peerDist });
  const sockets = new Set();
  let proxy, primaryPort, oraclePort;
  return {
    async listen(port = 0) {
      try {
        primaryPort = (await app.listen(0)).port;
        oraclePort = (await oracle.listen(0)).port;
        proxy = http.createServer((req, res) => {
          const peer = /(?:^|;\s*)uqm_test_peer=1(?:;|$)/.test(req.headers.cookie || '');
          const upstream = http.request({ host: '127.0.0.1', port: peer ? oraclePort : primaryPort,
            path: req.url, method: req.method, headers: req.headers }, response => {
            res.writeHead(response.statusCode, response.headers);
            response.on('error', error => res.destroy(error));
            response.on('aborted', () => res.destroy());
            response.pipe(res);
          });
          upstream.on('error', () => { if (!res.headersSent) res.writeHead(502); res.end(); });
          res.on('close', () => upstream.destroy());
          req.pipe(upstream);
        });
        proxy.on('connection', socket => { sockets.add(socket); socket.on('close', () => sockets.delete(socket)); });
        proxy.on('upgrade', (req, socket, head) => {
          const upstream = net.connect(primaryPort, '127.0.0.1', () => {
            const headers = req.rawHeaders.reduce((text, value, i) =>
              text + value + (i % 2 ? '\r\n' : ': '), '');
            upstream.write(`${req.method} ${req.url} HTTP/${req.httpVersion}\r\n${headers}\r\n`);
            if (head.length) upstream.write(head);
            socket.pipe(upstream).pipe(socket);
          });
          upstream.on('error', () => socket.destroy());
          socket.on('error', () => upstream.destroy());
          socket.on('close', () => upstream.destroy());
        });
        await new Promise((resolve, reject) => {
          proxy.once('error', reject); proxy.listen(port, '127.0.0.1', resolve);
        });
        return proxy.address();
      } catch (error) {
        await this.close();
        throw error;
      }
    },
    async close() {
      for (const socket of sockets) socket.destroy();
      if (proxy?.listening) await new Promise(resolve => proxy.close(resolve));
      await Promise.all([oracle.close(), app.close()]);
    },
  };
}
