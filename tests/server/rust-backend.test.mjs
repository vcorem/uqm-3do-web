import test from 'node:test';
import assert from 'node:assert/strict';
import { randomBytes } from 'node:crypto';
import { mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import http from 'node:http';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { WebSocket } from 'ws';
import { createGameBackend } from '../helpers/game-backend.mjs';
import { createRustRelay } from '../helpers/rust-relay.mjs';

function rustBackend(options, peerDist) {
  const previous = process.env.UQM_RELAY;
  process.env.UQM_RELAY = 'rust';
  try { return createGameBackend(options, peerDist); }
  finally {
    if (previous === undefined) delete process.env.UQM_RELAY;
    else process.env.UQM_RELAY = previous;
  }
}

function request(origin, pathname, cookie, method = 'GET') {
  return new Promise((resolve, reject) => {
    const req = http.request(origin, { path: pathname, method, agent: false,
      headers: cookie ? { Cookie: cookie } : {} }, response => {
      const chunks = [];
      response.on('data', chunk => chunks.push(chunk));
      response.on('error', reject);
      response.on('end', () => resolve({ status: response.statusCode,
        headers: response.headers, body: Buffer.concat(chunks) }));
    });
    req.on('error', reject);
    req.setTimeout(5000, () => req.destroy(new Error('HTTP timeout')));
    req.end();
  });
}

function client(t, origin, id, role, cookie, edition = 'original') {
  const prefix = edition === 'hd' ? 'hd/' : '';
  const socket = new WebSocket(`${origin.replace('http:', 'ws:')}/netplay/${prefix}${id}?role=${role}`, {
    origin, headers: cookie ? { Cookie: cookie } : {},
  });
  t.after(() => socket.terminate());
  const messages = [];
  const waiters = [];
  socket.on('error', error => {
    for (const waiter of waiters.splice(0)) { clearTimeout(waiter.timer); waiter.reject(error); }
  });
  socket.on('message', (bytes, binary) => {
    const value = binary ? Buffer.from(bytes) : JSON.parse(bytes.toString());
    const waiter = waiters.shift();
    if (waiter) { clearTimeout(waiter.timer); waiter.resolve(value); }
    else messages.push(value);
  });
  const closed = new Promise(resolve => socket.once('close', resolve));
  return { socket, closed, next() {
    if (messages.length) return Promise.resolve(messages.shift());
    return new Promise((resolve, reject) => {
      const waiter = { resolve, reject, timer: setTimeout(() => {
        const index = waiters.indexOf(waiter);
        if (index >= 0) waiters.splice(index, 1);
        reject(new Error(`No ${role} message from shared relay`));
      }, 5000) };
      waiters.push(waiter);
    });
  } };
}

async function withTimeout(promise, milliseconds, message) {
  let timer;
  try {
    return await Promise.race([promise, new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(message)), milliseconds);
    })]);
  } finally { clearTimeout(timer); }
}

test('differential backend serves distinct engines and routes both editions and cookies through one Rust relay', async t => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'uqm-differential-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  const primary = path.join(root, 'rust');
  const oracle = path.join(root, 'c');
  await Promise.all([mkdir(primary), mkdir(oracle)]);
  const rustBytes = randomBytes(262144);
  const cBytes = randomBytes(262144);
  await Promise.all([
    writeFile(path.join(primary, 'index.html'), '<title>Rust fixture</title>'),
    writeFile(path.join(oracle, 'index.html'), '<title>C oracle fixture</title>'),
    writeFile(path.join(primary, 'game.wasm'), rustBytes),
    writeFile(path.join(oracle, 'game.wasm'), cBytes),
  ]);
  const backend = rustBackend({ distDir: primary }, oracle);
  t.after(() => backend.close());
  const address = await backend.listen(0);
  const origin = `http://127.0.0.1:${address.port}`;

  for (const [cookie, expected, title] of [
    [undefined, rustBytes, '<title>Rust fixture</title>'],
    ['uqm_test_peer=1', cBytes, '<title>C oracle fixture</title>'],
    ['unrelated=x; uqm_test_peer=1; another=y', cBytes, '<title>C oracle fixture</title>'],
    ['uqm_test_peer=10', rustBytes, '<title>Rust fixture</title>'],
    ['xuqm_test_peer=1', rustBytes, '<title>Rust fixture</title>'],
  ]) {
    const index = await request(origin, '/', cookie);
    assert.equal(index.body.toString(), title, cookie);
    const asset = await request(origin, '/game.wasm?build=fixture', cookie);
    assert.equal(asset.status, 200);
    assert.equal(asset.headers['content-type'], 'application/wasm');
    assert.equal(asset.headers['cross-origin-opener-policy'], 'same-origin');
    assert.equal(asset.headers['cross-origin-embedder-policy'], 'require-corp');
    assert.equal(asset.headers['cross-origin-resource-policy'], 'same-origin');
    assert.equal(asset.headers['content-length'], String(expected.length));
    assert.deepEqual(asset.body, expected, cookie);
    const head = await request(origin, '/game.wasm', cookie, 'HEAD');
    assert.equal(head.status, 200);
    assert.equal(head.headers['content-length'], String(expected.length));
    assert.equal(head.body.length, 0);
  }

  const id = randomBytes(16).toString('hex');
  const peers = [];
  for (const edition of ['original', 'hd']) {
    const host = client(t, origin, id, 'host', undefined, edition);
    peers.push(host);
    assert.deepEqual(await host.next(), { type: 'waiting' });
    const guest = client(t, origin, id, 'guest', 'uqm_test_peer=1', edition);
    peers.push(guest);
    assert.deepEqual(await guest.next(), { type: 'waiting' });
    assert.deepEqual(await host.next(), { type: 'ready', role: 'host' });
    assert.deepEqual(await guest.next(), { type: 'ready', role: 'guest' });
    // If guest upgrades followed its static cookie, the oracle would reject
    // the room. Identical tokens also prove edition isolation through the proxy.
    const packet = randomBytes(65536);
    host.socket.send(packet);
    guest.socket.send(Buffer.from([0, 255, 128, 1]));
    assert.deepEqual(await guest.next(), packet);
    assert.deepEqual(await host.next(), Buffer.from([0, 255, 128, 1]));
  }

  await withTimeout(backend.close(), 5000, 'Differential backend did not stop');
  await withTimeout(Promise.all(peers.map(peer => peer.closed)), 1000, 'Proxy left WebSocket clients connected');
  await assert.rejects(new Promise((resolve, reject) => {
    const socket = net.connect(address.port, '127.0.0.1');
    socket.once('connect', () => { socket.destroy(); resolve(); });
    socket.once('error', reject);
  }), error => error.code === 'ECONNREFUSED');
});

test('missing Rust executable rejects startup and cleanup completes without waiting for an exit event', async () => {
  const previous = process.env.UQM_RUST_RELAY_BIN;
  process.env.UQM_RUST_RELAY_BIN = path.join(os.tmpdir(), `uqm-missing-${randomBytes(16).toString('hex')}`);
  let backend;
  try { backend = createRustRelay({ distDir: false }); }
  finally {
    if (previous === undefined) delete process.env.UQM_RUST_RELAY_BIN;
    else process.env.UQM_RUST_RELAY_BIN = previous;
  }
  await assert.rejects(backend.listen(0), error => error.code === 'ENOENT');
  await withTimeout(backend.close(), 500, 'Failed spawn left cleanup waiting for exit');
});

test('invalid Rust startup options preserve the startup error and leave an idempotent close', async () => {
  const backend = createRustRelay({ maxRooms: 0, distDir: false });
  await assert.rejects(backend.listen(0), /Invalid maxRooms/);
  await withTimeout(backend.close(), 500, 'Invalid configuration left a child process');
  await backend.close();
});

test('a busy differential proxy port rolls back its already-started listeners and Rust process', async t => {
  const occupied = net.createServer();
  await new Promise((resolve, reject) => {
    occupied.once('error', reject);
    occupied.listen(0, '127.0.0.1', resolve);
  });
  t.after(() => new Promise(resolve => occupied.close(resolve)));
  // Only inspect newly created server/process handles, without depending on
  // unrelated test-runner pipes or Node's own persistent background handles.
  const baseline = new Set(process._getActiveHandles());
  const backend = rustBackend({ distDir: false }, os.tmpdir());
  t.after(() => backend.close());
  await assert.rejects(withTimeout(backend.listen(occupied.address().port), 5000,
    'Proxy startup rollback did not complete'), error => error.code === 'EADDRINUSE');
  await new Promise(resolve => setImmediate(resolve));
  const leaked = process._getActiveHandles().filter(handle => !baseline.has(handle) &&
    ((handle instanceof net.Server && handle.listening) ||
     (handle.constructor.name === 'ChildProcess' && handle.exitCode === null && handle.signalCode === null)));
  assert.deepEqual(leaked.map(handle => handle.constructor.name), [], 'Failed proxy startup leaked a listener or Rust child');
});
