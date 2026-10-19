// The Cloudflare relay (deploy/netplay.js) under `wrangler dev`, with real
// WebSocket clients. Mirrors tests/server/netplay.test.mjs where the Workers
// runtime allows it. Run with `npm run test:worker`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { randomBytes } from 'node:crypto';
import { access, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { WebSocket } from 'ws';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const WRANGLER = process.env.WRANGLER ?? 'wrangler@4.143.1';
const roomId = () => randomBytes(16).toString('hex');
const pause = ms => new Promise(resolve => setTimeout(resolve, ms));

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

// worker.js imports the data-part manifest that the deploy job writes. Tests need only an empty one.
const manifest = path.join(root, 'deploy/data-parts.json');
let createdManifest = false;
test.before(async () => {
  try { await access(manifest); } catch { await writeFile(manifest, '{}\n'); createdManifest = true; }
});
test.after(async () => { if (createdManifest) await rm(manifest, { force: true }); });

/** Starts one `wrangler dev` with the given relay vars. */
async function startWorker(t, vars = {}) {
  const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-worker-'));
  const port = await freePort();
  const inspectorPort = await freePort();
  const origin = `http://127.0.0.1:${port}`;
  await writeFile(path.join(temp, 'index.html'), '<canvas>game</canvas>');
  // The real deploy config must declare the same objects, or this test proves nothing about it.
  const deployed = await readFile(path.join(root, 'deploy/wrangler.jsonc'), 'utf8');
  const config = {
    name: 'uqm-web-test',
    main: path.join(root, 'deploy/worker.js'),
    compatibility_date: /"compatibility_date":\s*"([^"]+)"/.exec(deployed)[1],
    assets: { directory: temp, binding: 'ASSETS', run_worker_first: true,
      html_handling: 'none', not_found_handling: 'none' },
    durable_objects: { bindings: [
      { name: 'NETPLAY_ROOMS', class_name: 'NetplayRoom' },
      { name: 'NETPLAY_GATE', class_name: 'NetplayGate' }] },
    migrations: [{ tag: 'netplay-v1', new_sqlite_classes: ['NetplayRoom', 'NetplayGate'] }],
    vars: { NETPLAY_ORIGINS: origin, ...vars },
  };
  for (const name of ['NetplayRoom', 'NetplayGate', 'NETPLAY_ROOMS', 'NETPLAY_GATE']) assert.ok(deployed.includes(name), name);
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
  t.after(async () => {
    const signal = name => { try { process.kill(-child.pid, name); } catch { /* Already gone. */ } };
    signal('SIGTERM');
    await Promise.race([exited, pause(5000)]);
    signal('SIGKILL');
    child.stdout.destroy();
    child.stderr.destroy();
    await rm(temp, { recursive: true, force: true });
  });
  const deadline = Date.now() + 90000;
  let lastCheck = 'no answer';
  for (let attempt = 0; ; attempt++) {
    if (child.exitCode !== null) throw new Error(`wrangler dev exited:\n${log}`);
    try {
      const response = await fetch(origin + '/', { signal: AbortSignal.timeout(2000) });
      if (response.ok) break;
      lastCheck = 'HTTP ' + response.status;
    } catch (error) { lastCheck = String(error.cause?.code ?? error.name); /* Not listening yet. */ }
    if (Date.now() > deadline) throw new Error(`wrangler dev did not start in 90 s (last check: ${lastCheck}):\n${log}`);
    await pause(100);
  }
  const wsBase = origin.replace('http:', 'ws:');

  function client(id, role, clientOptions = {}, urlOverride) {
    const ws = new WebSocket(`${wsBase}${urlOverride ?? `/netplay/${id}?role=${role}`}`, { origin, ...clientOptions });
    const messages = [];
    const waiters = [];
    let closed = null;
    const opened = new Promise((resolve, reject) => { ws.once('open', resolve); ws.once('error', reject); });
    opened.catch(() => {});
    ws.on('error', () => {});
    ws.on('message', (data, binary) => {
      const value = binary ? Buffer.from(data) : JSON.parse(data.toString());
      const index = waiters.findIndex(waiter => waiter.predicate(value));
      if (index < 0) messages.push(value);
      else { const [waiter] = waiters.splice(index, 1); clearTimeout(waiter.timer); waiter.resolve(value); }
    });
    const closure = new Promise(resolve => ws.on('close', (code, reason) => {
      closed = { code, reason: reason.toString() }; resolve(closed);
    }));
    return { ws, messages, opened, closure, get closed() { return closed; },
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
      }
    };
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
  return { origin, client, pair, get log() { return log; } };
}

// One default instance serves the tests that need no special bounds.
let shared;
const sharedCleanups = [];
test.after(() => Promise.all(sharedCleanups.map(cleanup => cleanup())));
function defaultWorker() {
  shared ??= startWorker({ after: cleanup => sharedCleanups.push(cleanup) });
  return shared;
}

test('two real WebSocket clients receive ready, relay exact ordered bytes both ways, and isolate rooms', async t => {
  const { pair } = await defaultWorker(t);
  const a = await pair();
  const b = await pair();
  const expected = Array.from({ length: 100 }, (_, n) => Buffer.from([n, 0, 255, n ^ 7]));
  for (const bytes of expected) { a.host.ws.send(bytes); a.guest.ws.send(Buffer.from(bytes).reverse()); }
  for (const bytes of expected) {
    assert.deepEqual(await a.guest.next(), bytes);
    assert.deepEqual(await a.host.next(), Buffer.from(bytes).reverse());
  }
  await pause(50);
  for (const peer of [a.host, a.guest, b.host, b.guest]) assert.deepEqual(peer.messages, []);
});

test('strict URL validation rejects malformed room tokens, roles and query parameters', async t => {
  const { client } = await defaultWorker(t);
  for (const url of ['/netplay/short?role=host', `/netplay/${'A'.repeat(32)}?role=host`,
    `/netplay/${roomId()}?role=owner`, `/netplay/${roomId()}?role=host&role=guest`,
    `/netplay/${roomId()}?role=host&target=localhost`, `/netplay/${roomId()}`]) {
    const invalid = client(null, null, {}, url);
    assert.equal((await invalid.next()).type, 'error', url);
    assert.equal((await invalid.closure).code, 1008, url);
  }
});

test('plain HTTP requests to the relay do not reach a room', async t => {
  const { origin } = await defaultWorker(t);
  const response = await fetch(`${origin}/netplay/${roomId()}?role=host`, { headers: { Origin: origin } });
  assert.equal(response.status, 426);
});

test('guests cannot create rooms; duplicate roles and third players cannot displace either player', async t => {
  const { client, pair } = await defaultWorker(t);
  const missing = client(roomId(), 'guest');
  assert.match((await missing.next()).message, /does not exist/);
  await missing.closure;
  const { id, host, guest } = await pair();
  for (const role of ['host', 'guest']) {
    const extra = client(id, role);
    assert.equal((await extra.next()).type, 'error');
    await extra.closure;
  }
  host.ws.send(Buffer.from('still connected'));
  assert.deepEqual(await guest.next(), Buffer.from('still connected'));
});

test('data sent before pairing is rejected, never buffered, and retires the room', async t => {
  const { client } = await defaultWorker(t);
  const id = roomId();
  const host = client(id, 'host');
  await host.next();
  host.ws.send(Buffer.from('too soon'));
  assert.match((await host.next()).message, /Wait for the other player/);
  await host.closure;
  for (const role of ['host', 'guest']) {
    const retry = client(id, role);
    assert.equal((await retry.next()).type, 'error');
    await retry.closure;
  }
});

test('text and zero-length messages close both sides instead of entering the binary game stream', async t => {
  const { pair } = await defaultWorker(t);
  for (const payload of ['{"type":"ready"}', Buffer.alloc(0)]) {
    const { host, guest } = await pair();
    host.ws.send(payload);
    assert.equal((await host.next()).type, 'error');
    assert.equal((await guest.next()).type, 'peer-left');
    assert.equal((await host.closure).code, 1008);
    await guest.closure;
    assert.ok(!guest.messages.some(Buffer.isBuffer));
  }
});

test('disconnect notifies and closes the survivor and forbids token reuse', async t => {
  const { pair, client } = await defaultWorker(t);
  for (const departingRole of ['host', 'guest']) {
    const peers = await pair();
    const departing = peers[departingRole];
    const survivor = peers[departingRole === 'host' ? 'guest' : 'host'];
    departing.ws.close();
    assert.equal((await survivor.next()).type, 'peer-left');
    await Promise.all([departing.closure, survivor.closure]);
    const reuse = client(peers.id, 'host');
    assert.match((await reuse.next()).message, /expired/);
    await reuse.closure;
  }
});

test('the origin must match the configured site exactly', async t => {
  const { client, origin } = await defaultWorker(t);
  for (const bad of ['https://evil.example', 'null', undefined, `${origin}/`]) {
    const denied = client(roomId(), 'host', { origin: bad });
    await assert.rejects(denied.opened, /403/, String(bad));
  }
});

test('arbitrary WebSocket proxy subprotocols are rejected', async t => {
  const { origin } = await defaultWorker(t);
  const ws = new WebSocket(`${origin.replace('http:', 'ws:')}/netplay/${roomId()}?role=host`, 'binary', { origin });
  await assert.rejects(new Promise((resolve, reject) => { ws.once('open', resolve); ws.once('error', reject); }), /400/);
});

test('oversized payload closes with 1009 and the peer gets no partial bytes', async t => {
  const { pair } = await defaultWorker(t);
  const { host, guest } = await pair();
  host.ws.send(Buffer.alloc(65537, 17));
  assert.equal((await host.closure).code, 1009);
  assert.equal((await guest.next()).type, 'peer-left');
  await guest.closure;
  assert.ok(!guest.messages.some(Buffer.isBuffer));
});

test('maximum-size binary frames are forwarded intact', async t => {
  const { pair } = await defaultWorker(t);
  const { host, guest } = await pair();
  const bytes = randomBytes(65536);
  host.ws.send(bytes);
  assert.deepEqual(await guest.next(), bytes);
});

test('excessive message rates close the sending connection and its room', async t => {
  const { pair } = await defaultWorker(t);
  const { host, guest } = await pair();
  for (let n = 0; n < 1001; n++) host.ws.send(Buffer.from([n & 255]));
  assert.match((await host.next(value => value.type === 'error')).message, /rate exceeded/);
  assert.equal((await guest.next(value => value.type === 'peer-left')).type, 'peer-left');
  await Promise.all([host.closure, guest.closure]);
  console.log(guest.closed, host.closed, (await defaultWorker(t)).log);
});

test('client pings are answered by the runtime and do not end the room', async t => {
  const { pair } = await defaultWorker(t);
  const { host, guest } = await pair();
  const payload = Buffer.from('diagnostic ping');
  const reply = new Promise(resolve => host.ws.once('pong', resolve));
  host.ws.ping(payload);
  assert.deepEqual(await reply, payload);
  host.ws.send(Buffer.from('after ping'));
  assert.deepEqual(await guest.next(), Buffer.from('after ping'));
});

test('deterministic latency and jitter retain byte order; queued slow-peer data is bounded', async t => {
  const { pair } = await startWorker(t, { NETPLAY_LATENCY_MS: '200,0,20,5', NETPLAY_MAX_BUFFERED_BYTES: '65536' });
  const ordered = await pair();
  const start = performance.now();
  for (let n = 0; n < 12; n++) ordered.host.ws.send(Buffer.from([n]));
  for (let n = 0; n < 12; n++) assert.deepEqual(await ordered.guest.next(), Buffer.from([n]));
  assert.ok(performance.now() - start >= 150);
  const slow = await pair();
  for (let n = 0; n < 3; n++) slow.host.ws.send(Buffer.alloc(32768, n));
  assert.match((await slow.host.next()).message, /too slow/);
  assert.equal((await slow.guest.next()).type, 'peer-left');
  await Promise.all([slow.host.closure, slow.guest.closure]);
  assert.ok(!slow.guest.messages.some(Buffer.isBuffer));
});

test('room and per-IP connection limits use CF-Connecting-IP and recover when rooms end', async t => {
  const { client } = await startWorker(t, { NETPLAY_MAX_ROOMS: '2', NETPLAY_MAX_PER_IP: '2' });
  const ipA = { headers: { 'CF-Connecting-IP': '203.0.113.10' } };
  const ipB = { headers: { 'CF-Connecting-IP': '203.0.113.20' } };
  const first = client(roomId(), 'host', ipA);
  assert.equal((await first.next()).type, 'waiting');
  const second = client(roomId(), 'host', ipA);
  assert.equal((await second.next()).type, 'waiting');
  // A third connection from the same address is refused before the upgrade.
  const third = client(roomId(), 'host', ipA);
  await assert.rejects(third.opened, /503/);
  // Another address may connect, but the global room bound (2) is reached.
  const other = client(roomId(), 'host', ipB);
  assert.match((await other.next()).message, /Room limit reached/);
  await other.closure;
  // Ending a room releases its lease.
  first.ws.close();
  await first.closure;
  await pause(300);
  const again = client(roomId(), 'host', ipB);
  assert.equal((await again.next()).type, 'waiting');
});

test('waiting rooms expire', async t => {
  const { client } = await startWorker(t, { NETPLAY_WAITING_MS: '1500' });
  const host = client(roomId(), 'host');
  await host.next();
  assert.match((await host.next()).message, /expired/);
  await host.closure;
});

test('absolute room lifetime and idle timeout notify both peers', async t => {
  for (const [vars, message] of [
    [{ NETPLAY_LIFETIME_MS: '3000' }, 'Room time limit reached.'],
    [{ NETPLAY_IDLE_MS: '1500' }, 'Room was idle for too long.'],
  ]) {
    const { pair } = await startWorker(t, vars);
    const { host, guest } = await pair();
    assert.deepEqual(await host.next(), { type: 'peer-left', message });
    assert.deepEqual(await guest.next(), { type: 'peer-left', message });
    await Promise.all([host.closure, guest.closure]);
  }
});
