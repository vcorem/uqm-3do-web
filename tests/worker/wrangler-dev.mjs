// Starts the Cloudflare relay (deploy/worker.js, deploy/netplay.js) under
// `wrangler dev` for the tests in this folder, with real WebSocket clients.
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { randomBytes } from 'node:crypto';
import { access, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { WebSocket } from 'ws';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const WRANGLER = process.env.WRANGLER ?? 'wrangler@4.143.1';
export const roomId = () => randomBytes(16).toString('hex');
export const pause = ms => new Promise(resolve => setTimeout(resolve, ms));

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

/** One wrangler dev on new ports, ready to answer: { origin, log }. */
async function launch(temp, site, deployed, vars, children) {
  const port = await freePort();
  const inspectorPort = await freePort();
  const origin = `http://127.0.0.1:${port}`;
  const config = {
    name: 'uqm-web-test',
    main: path.join(root, 'deploy/worker.js'),
    compatibility_date: deployed.compatibility_date,
    assets: { directory: site, binding: 'ASSETS', run_worker_first: true,
      html_handling: 'none', not_found_handling: 'none' },
    durable_objects: deployed.durable_objects,
    migrations: deployed.migrations,
    vars: { NETPLAY_ORIGINS: origin, ...vars },
  };
  const configPath = path.join(temp, 'wrangler.json');
  await writeFile(configPath, JSON.stringify(config));
  const child = spawn('npx', ['--yes', WRANGLER, 'dev', '--config', configPath, '--ip', '127.0.0.1',
    '--port', String(port), '--inspector-port', String(inspectorPort), '--persist-to', path.join(temp, 'state'),
    '--show-interactive-dev-session=false', '--log-level', process.env.WRANGLER_TEST_LOG_LEVEL ?? 'warn'],
  // Own process group: npx does not always pass SIGTERM on to wrangler and workerd,
  // and their open pipes would keep this test file running.
  { cwd: temp, env: { ...process.env, WRANGLER_SEND_METRICS: 'false', CI: '1' }, stdio: ['ignore', 'pipe', 'pipe'], detached: true });
  let log = '';
  child.stdout.on('data', chunk => { log += chunk; });
  child.stderr.on('data', chunk => { log += chunk; });
  const exited = new Promise(resolve => child.once('exit', resolve));
  children.push({ child, exited });
  const deadline = Date.now() + 90000;
  let lastCheck = 'no answer';
  for (;;) {
    if (child.exitCode !== null) throw new Error(`wrangler dev exited:\n${log}`);
    try {
      const response = await fetch(origin + '/', { signal: AbortSignal.timeout(2000) });
      if (response.ok) return { origin, get log() { return log; } };
      lastCheck = 'HTTP ' + response.status;
    } catch (error) { lastCheck = String(error.cause?.code ?? error.name); /* Not listening yet. */ }
    if (Date.now() > deadline) throw new Error(`wrangler dev did not start in 90 s (last check: ${lastCheck}):\n${log}`);
    await pause(100);
  }
}

/** worker.js imports the data-part manifest that the deploy job writes. Tests need only an empty one. */
export function useDataManifest() {
  const manifest = path.join(root, 'deploy/data-parts.json');
  let createdManifest = false;
  test.before(async () => {
    try { await access(manifest); } catch { await writeFile(manifest, '{}\n'); createdManifest = true; }
  });
  test.after(async () => { if (createdManifest) await rm(manifest, { force: true }); });
}

/** Starts one `wrangler dev` with the given relay vars and extra static files ({ 'path/name': content }). */
export async function startWorker(t, vars = {}, files = {}) {
  const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-worker-'));
  // Static files get their own folder: wrangler dev watches it, and its cache and
  // --persist-to state in the same folder make it reload without end on Linux.
  const site = path.join(temp, 'site');
  await mkdir(site);
  await writeFile(path.join(site, 'index.html'), '<canvas>game</canvas>');
  for (const [name, content] of Object.entries(files)) {
    await mkdir(path.dirname(path.join(site, name)), { recursive: true });
    await writeFile(path.join(site, name), content);
  }
  // The objects and migrations come from the real deploy config, so this test
  // also proves that the deployed config declares every object the code uses.
  const deployed = JSON.parse((await readFile(path.join(root, 'deploy/wrangler.jsonc'), 'utf8'))
    .replace(/^\s*\/\/.*$/gm, ''));
  const bindings = deployed.durable_objects.bindings.map(binding => binding.name + '=' + binding.class_name).sort();
  assert.deepEqual(bindings, ['NETPLAY_GATE=NetplayGate', 'NETPLAY_LOBBY=NetplayLobby', 'NETPLAY_ROOMS=NetplayRoom']);
  assert.deepEqual(deployed.migrations.flatMap(migration => migration.new_sqlite_classes).sort(),
    ['NetplayGate', 'NetplayLobby', 'NetplayRoom']);
  const children = [];
  t.after(async () => {
    for (const { child, exited } of children) {
      const signal = name => { try { process.kill(-child.pid, name); } catch { /* Already gone. */ } };
      signal('SIGTERM');
      await Promise.race([exited, pause(5000)]);
      signal('SIGKILL');
      child.stdout.destroy();
      child.stderr.destroy();
    }
    await rm(temp, { recursive: true, force: true });
  });
  // freePort() closes its port before wrangler binds it, so another process can take it
  // in between: then wrangler dev stops with "Address already in use", and it starts again
  // on new ports.
  let started;
  for (let start = 1; ; start++) {
    try {
      started = await launch(temp, site, deployed, vars, children);
      break;
    } catch (error) {
      if (start >= 3 || !/Address already in use/.test(error.message)) throw error;
    }
  }
  const { origin } = started;
  const wsBase = origin.replace('http:', 'ws:');


  function socket(url, clientOptions = {}, protocol) {
    const ws = protocol ? new WebSocket(wsBase + url, protocol, { origin, ...clientOptions })
      : new WebSocket(wsBase + url, { origin, ...clientOptions });
    const messages = [];
    const all = [];
    const times = [];
    const waiters = [];
    let closed = null;
    const opened = new Promise((resolve, reject) => { ws.once('open', resolve); ws.once('error', reject); });
    opened.catch(() => {});
    ws.on('error', () => {});
    ws.on('message', (data, binary) => {
      let value = binary ? Buffer.from(data) : data.toString();
      if (!binary) try { value = JSON.parse(value); } catch { /* Keepalive text. */ }
      all.push(value);
      times.push(performance.now());
      const index = waiters.findIndex(waiter => waiter.predicate(value));
      if (index < 0) messages.push(value);
      else { const [waiter] = waiters.splice(index, 1); clearTimeout(waiter.timer); waiter.resolve(value); }
    });
    const closure = new Promise(resolve => ws.on('close', (code, reason) => {
      closed = { code, reason: reason.toString() }; resolve(closed);
    }));
    const peer = { ws, messages, all, times, opened, closure, get closed() { return closed; },
      next(predicate = () => true, timeout = 8000) {
        const index = messages.findIndex(predicate);
        if (index >= 0) return Promise.resolve(messages.splice(index, 1)[0]);
        return new Promise((resolve, reject) => {
          const waiter = { predicate, resolve, timer: setTimeout(() => {
            const i = waiters.indexOf(waiter);
            if (i >= 0) waiters.splice(i, 1);
            reject(new Error(`Message timeout; remaining=${JSON.stringify(messages)}; closed=${JSON.stringify(closed)}`));
          }, timeout) };
          waiters.push(waiter);
        });
      },
      /** The next room-list snapshot whose rooms pass the check. */
      rooms(check = () => true, timeout) {
        return peer.next(value => value?.type === 'rooms' && check(value.rooms), timeout).then(value => value.rooms);
      },
    };
    return peer;
  }
  function client(id, role, clientOptions = {}, urlOverride) {
    return socket(urlOverride ?? `/netplay/${id}?role=${role}`, clientOptions);
  }
  async function pair(id = roomId(), options = {}) {
    const host = client(id, 'host', options);
    assert.deepEqual(await host.next(), { type: 'waiting' });
    const guest = client(id, 'guest', options);
    assert.deepEqual(await guest.next(), { type: 'waiting' });
    assert.deepEqual(await host.next(), { type: 'ready', role: 'host' });
    assert.deepEqual(await guest.next(), { type: 'ready', role: 'guest' });
    return { id, host, guest };
  }
  return { origin, socket, client, pair, get log() { return started.log; } };
}
