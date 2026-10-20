// The Cloudflare relay (deploy/netplay.js) under `wrangler dev`, with real
// WebSocket clients. Mirrors tests/server/netplay.test.mjs where the Workers
// runtime allows it. Run with `npm run test:worker`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { randomBytes } from 'node:crypto';
import { WebSocket } from 'ws';
import { pause, roomId, startWorker, useDataManifest } from './wrangler-dev.mjs';

useDataManifest();

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

test('HD rooms are separate from Original rooms with the same token', async t => {
  const { client } = await defaultWorker(t);
  const id = roomId();
  const hd = role => `/netplay/hd/${id}?role=${role}`;
  const host = client(id, 'host', {}, hd('host'));
  assert.deepEqual(await host.next(), { type: 'waiting' });
  // An Original guest with the same token finds no room: the editions cannot play each other.
  const original = client(id, 'guest');
  assert.equal((await original.next()).type, 'error');
  await original.closure;
  const guest = client(id, 'guest', {}, hd('guest'));
  assert.deepEqual(await guest.next(), { type: 'waiting' });
  assert.deepEqual(await host.next(), { type: 'ready', role: 'host' });
  assert.deepEqual(await guest.next(), { type: 'ready', role: 'guest' });
  host.ws.send(Buffer.from([1, 2, 3]));
  assert.deepEqual(await guest.next(), Buffer.from([1, 2, 3]));
  for (const url of [`/netplay/3do/${roomId()}?role=host`, `/netplay/hd/hd/${roomId()}?role=host`, `/netplay/hd/${roomId()}`]) {
    const invalid = client(null, null, {}, url);
    assert.equal((await invalid.next()).type, 'error', url);
  }
});

test('Cosmo Clash rooms are separate from Original and HD rooms with the same token', async t => {
  const { client } = await defaultWorker(t);
  const id = roomId();
  const keys = [id, `hd/${id}`, `cosmoclash/${id}`];
  const hosts = [];
  for (const key of keys) {
    const host = client(null, null, {}, `/netplay/${key}?role=host`);
    assert.deepEqual(await host.next(), { type: 'waiting' }, key);
    hosts.push(host);
  }
  const guests = [];
  for (const [index, key] of keys.entries()) {
    const guest = client(null, null, {}, `/netplay/${key}?role=guest`);
    assert.deepEqual(await guest.next(), { type: 'waiting' }, key);
    assert.deepEqual(await guest.next(), { type: 'ready', role: 'guest' }, key);
    assert.deepEqual(await hosts[index].next(), { type: 'ready', role: 'host' }, key);
    guests.push(guest);
  }
  for (const [index, host] of hosts.entries()) host.ws.send(Buffer.from(`game ${index}`));
  for (const [index, guest] of guests.entries()) assert.deepEqual(await guest.next(), Buffer.from(`game ${index}`));
  // Closing the Cosmo Clash room retires its token there only.
  hosts[2].ws.close();
  assert.equal((await guests[2].next()).type, 'peer-left');
  const retired = client(null, null, {}, `/netplay/cosmoclash/${id}?role=host`);
  assert.match((await retired.next()).message, /expired/);
  for (const [index, host] of hosts.slice(0, 2).entries()) {
    host.ws.send(Buffer.from(`still ${index}`));
    assert.deepEqual(await guests[index].next(), Buffer.from(`still ${index}`));
  }
  for (const url of [`/netplay/CosmoClash/${roomId()}?role=host`, `/netplay/cosmoclash/hd/${roomId()}?role=host`,
    `/netplay/cosmoclash/${roomId()}/?role=host`, `/netplay/cosmoclash%2f${roomId()}?role=host`, `/netplay/cosmoclash/${roomId()}`]) {
    const invalid = client(null, null, {}, url);
    assert.equal((await invalid.next()).type, 'error', url);
  }
  for (const peer of [...hosts, ...guests]) peer.ws.close();
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
  // 17 maximum-size frames pass 1 MiB in a few milliseconds. 1,001 tiny frames can
  // spread over two one-second windows on a slow CI runner and never trip the limit.
  for (let n = 0; n < 17; n++) host.ws.send(randomBytes(65536));
  assert.match((await host.next(value => value.type === 'error')).message, /rate exceeded/);
  assert.equal((await guest.next(value => value.type === 'peer-left')).type, 'peer-left');
  await Promise.all([host.closure, guest.closure]);
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
