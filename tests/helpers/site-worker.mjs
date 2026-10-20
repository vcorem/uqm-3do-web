// The Cloudflare Worker (deploy/worker.js) under `wrangler dev` for a browser
// test: a split site (deploy/split-data.py --output) and its part manifest,
// with the bindings and migrations of deploy/wrangler.jsonc.
import { spawn } from 'node:child_process';
import { copyFile, mkdir, mkdtemp, readdir, readFile, rm, writeFile } from 'node:fs/promises';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const WRANGLER = process.env.WRANGLER ?? 'wrangler@4.143.1';
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

function freePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const { port } = server.address();
      server.close(() => resolve(port));
    });
  });
}

/** Starts the Worker for site; returns { origin, close(), log }. */
export async function startSiteWorker({ site, manifest = path.join(root, 'deploy/data-parts.json') }) {
  const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-site-worker-'));
  // The Worker imports the part manifest beside itself: copy it with the Worker modules.
  const workerDir = path.join(temp, 'worker');
  await mkdir(workerDir);
  for (const file of (await readdir(path.join(root, 'deploy'))).filter(name => /\.m?js$/.test(name))) {
    await copyFile(path.join(root, 'deploy', file), path.join(workerDir, file));
  }
  await copyFile(manifest, path.join(workerDir, 'data-parts.json'));
  const port = await freePort();
  const origin = `http://127.0.0.1:${port}`;
  const deployed = JSON.parse((await readFile(path.join(root, 'deploy/wrangler.jsonc'), 'utf8')).replace(/^\s*\/\/.*$/gm, ''));
  const config = {
    name: 'uqm-web-site-test', main: path.join(workerDir, 'worker.js'), compatibility_date: deployed.compatibility_date,
    assets: { ...deployed.assets, directory: path.resolve(site) },
    durable_objects: deployed.durable_objects, migrations: deployed.migrations, vars: { NETPLAY_ORIGINS: origin },
  };
  const configPath = path.join(temp, 'wrangler.json');
  await writeFile(configPath, JSON.stringify(config));
  const child = spawn('npx', ['--yes', WRANGLER, 'dev', '--config', configPath, '--ip', '127.0.0.1', '--port', String(port),
    '--inspector-port', String(await freePort()), '--persist-to', path.join(temp, 'state'),
    '--show-interactive-dev-session=false', '--log-level', 'warn'],
  // Own process group: npx does not always pass SIGTERM on to wrangler and workerd.
  { cwd: temp, env: { ...process.env, WRANGLER_SEND_METRICS: 'false', CI: '1' }, stdio: ['ignore', 'pipe', 'pipe'], detached: true });
  let log = '';
  child.stdout.on('data', chunk => { log += chunk; });
  child.stderr.on('data', chunk => { log += chunk; });
  async function close() {
    const signal = name => { try { process.kill(-child.pid, name); } catch { /* Already gone. */ } };
    signal('SIGTERM');
    await Promise.race([new Promise(resolve => child.once('exit', resolve)), sleep(5000)]);
    signal('SIGKILL');
    child.stdout.destroy();
    child.stderr.destroy();
    await rm(temp, { recursive: true, force: true }).catch(() => {});
  }
  const deadline = Date.now() + 120000;
  for (;;) {
    if (child.exitCode !== null) { await close(); throw new Error('wrangler dev exited:\n' + log); }
    try { if ((await fetch(origin + '/', { signal: AbortSignal.timeout(2000) })).ok) break; } catch { /* Not listening yet. */ }
    if (Date.now() > deadline) { await close(); throw new Error('wrangler dev did not start:\n' + log); }
    await sleep(200);
  }
  return { origin, close, get log() { return log; } };
}
