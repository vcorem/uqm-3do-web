// npm start: play Cosmo Clash locally at http://127.0.0.1:10000/cosmoclash/
// with online play (the site's Node relay and the cosmoclash room list).
//   node scripts/serve.mjs [--port 10000] [--host 127.0.0.1] [--relay node|rust]
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createCosmoServer } from './cosmo-server.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const dist = path.join(root, 'dist');
const args = process.argv.slice(2);
let port = 10000, host = '127.0.0.1', kind = 'node';
for (let i = 0; i < args.length; i++) {
  const flag = args[i], value = args[i + 1];
  if (flag === '--help') { console.log('Usage: npm start -- [--port 10000] [--host 127.0.0.1] [--relay node|rust]'); process.exit(0); }
  if (!['--port', '--host', '--relay'].includes(flag) || value === undefined) { console.error(`Unknown or incomplete option: ${flag}`); process.exit(2); }
  i++;
  if (flag === '--port') port = Number(value);
  if (flag === '--host') host = value;
  if (flag === '--relay') kind = value;
}
if (!Number.isSafeInteger(port) || port < 0 || port > 65535 || !['node', 'rust'].includes(kind)) { console.error('Invalid --port or --relay.'); process.exit(2); }
if (!fs.existsSync(path.join(dist, 'index.html'))) { console.error('Build Cosmo Clash first: npm run build --prefix cosmoclash'); process.exit(1); }

let relay;
if (kind === 'rust') {
  const { createRustRelay } = await import('../../tests/helpers/rust-relay.mjs');
  relay = createRustRelay({ distDir: false });
} else {
  const { createNetplayServer } = await import('../../server/netplay.mjs');
  // Listed rooms wait 10 minutes. Behind the lobby adapter the relay sees
  // every room as private, so its private rooms wait as long here.
  relay = createNetplayServer({ distDir: false, waitingMs: 600000 });
}
const app = createCosmoServer({ distDir: dist, relay });
const address = await app.listen(port, host);
const shown = host.includes(':') ? `[${host}]` : host;
console.log(`Cosmo Clash: http://${shown}:${address.port}/cosmoclash/`);
console.log(app.lobby === 'adapter'
  ? `Online play: ${kind} relay, with the room list of scripts/cosmo-lobby.mjs (this relay has no cosmoclash room list).`
  : `Online play: ${kind} relay with its own cosmoclash room list.`);
let closing = false;
for (const signal of ['SIGINT', 'SIGTERM']) process.on(signal, async () => {
  if (closing) return;
  closing = true;
  await app.close();
  process.exit(0);
});
