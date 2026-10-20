import test from 'node:test';
import assert from 'node:assert/strict';
import { randomBytes } from 'node:crypto';
import { mkdtemp, writeFile, mkdir, symlink, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import http from 'node:http';
import { WebSocket } from 'ws';
import { createNetplayServer } from '../../server/netplay.mjs';

const roomId = () => randomBytes(16).toString('hex');
const pause = ms => new Promise(resolve => setTimeout(resolve, ms));

async function fixture(t, options = {}) {
  const app = createNetplayServer({ distDir: false, ...options });
  const address = await app.listen(0);
  const origin = `http://127.0.0.1:${address.port}`;
  t.after(() => app.close());
  function client(id, role, clientOptions = {}, urlOverride) {
    const ws = new WebSocket(`ws://127.0.0.1:${address.port}${urlOverride ?? `/netplay/${id}?role=${role}`}`,
      { origin, ...clientOptions });
    const messages = [];
    const waiters = [];
    let closed = null;
    const opened = new Promise((resolve, reject) => { ws.once('open', resolve); ws.once('error', reject); });
    // Attach immediately: rejection may happen before the caller requests next().
    opened.catch(() => {});
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
      next(predicate = () => true, timeout = 6000) {
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
  async function pair(id = roomId(), edition = 'original') {
    const key = edition === 'hd' ? `hd/${id}` : id;
    const host = client(key, 'host');
    assert.deepEqual(await host.next(), { type: 'waiting' });
    const guest = client(key, 'guest');
    assert.deepEqual(await guest.next(), { type: 'waiting' });
    assert.deepEqual(await host.next(), { type: 'ready', role: 'host' });
    assert.deepEqual(await guest.next(), { type: 'ready', role: 'guest' });
    return { id, host, guest };
  }
  return { app, origin, client, pair };
}

test('two real WebSocket clients receive ready, relay exact ordered bytes both ways, and isolate rooms', async t => {
  const { pair } = await fixture(t);
  const a = await pair();
  const b = await pair();
  const expected = Array.from({ length: 100 }, (_, n) => Buffer.from([n, 0, 255, n ^ 7]));
  for (const bytes of expected) { a.host.ws.send(bytes); a.guest.ws.send(Buffer.from(bytes).reverse()); }
  for (const bytes of expected) {
    assert.deepEqual(await a.guest.next(), bytes);
    assert.deepEqual(await a.host.next(), Buffer.from(bytes).reverse());
  }
  await pause(20);
  assert.deepEqual(a.host.messages, []);
  assert.deepEqual(a.guest.messages, []);
  assert.deepEqual(b.host.messages, []);
  assert.deepEqual(b.guest.messages, []);
});

test('deterministic latency and jitter retain byte order', async t => {
  const { pair } = await fixture(t, { latencyPatternMs: [45, 0, 20, 5] });
  const { host, guest } = await pair();
  const start = performance.now();
  for (let n = 0; n < 12; n++) host.ws.send(Buffer.from([n]));
  for (let n = 0; n < 12; n++) assert.deepEqual(await guest.next(), Buffer.from([n]));
  assert.ok(performance.now() - start >= 35);
});

test('strict URL validation rejects malformed room tokens, roles, query parameters and paths', async t => {
  const { client } = await fixture(t);
  for (const url of ['/netplay/short?role=host', `/netplay/${'A'.repeat(32)}?role=host`,
    `/netplay/${roomId()}?role=owner`, `/netplay/${roomId()}?role=host&role=guest`,
    `/netplay/${roomId()}?role=host&target=localhost`, `/netplay/${roomId()}`, '/proxy/anything',
    '/netplay/hd/short?role=host', `/netplay/hd/${'A'.repeat(32)}?role=host`,
    `/netplay/hd/${roomId()}?role=owner`, `/netplay/hd/${roomId()}?role=host&role=guest`,
    `/netplay/hd/${roomId()}?role=host&target=localhost`, `/netplay/hd/${roomId()}`,
    `/netplay/hd/hd/${roomId()}?role=host`, `/netplay/3do/${roomId()}?role=host`,
    `/netplay/original/${roomId()}?role=host`, `/netplay/HD/${roomId()}?role=host`,
    `/netplay/%68d/${roomId()}?role=host`, `/netplay/hd%2f${roomId()}?role=host`,
    `/netplay/hd//${roomId()}?role=host`, `/netplay/hd/${roomId()}/?role=host`]) {
    const invalid = client(null, null, {}, url);
    assert.equal((await invalid.next()).type, 'error', url);
    assert.equal((await invalid.closure).code, 1008, url);
  }
});

test('HD rooms are separate from Original rooms with the same token', async t => {
  const { client } = await fixture(t);
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

test('Original and HD rooms with the same token cannot pair, exchange data, or disconnect each other', async t => {
  const { client } = await fixture(t);
  for (const firstEdition of ['original', 'hd']) {
    const id = roomId();
    const keys = firstEdition === 'hd' ? [`hd/${id}`, id] : [id, `hd/${id}`];
    const firstHost = client(keys[0], 'host');
    assert.deepEqual(await firstHost.next(), { type: 'waiting' });
    const wrongEdition = client(keys[1], 'guest');
    assert.match((await wrongEdition.next()).message, /does not exist/);
    assert.equal((await wrongEdition.closure).code, 1008);
    assert.deepEqual(firstHost.messages, []);

    const secondHost = client(keys[1], 'host');
    assert.deepEqual(await secondHost.next(), { type: 'waiting' });
    const hosts = [firstHost, secondHost];
    const guests = [];
    for (const [index, key] of keys.entries()) {
      const guest = client(key, 'guest');
      guests.push(guest);
      assert.deepEqual(await guest.next(), { type: 'waiting' });
      assert.deepEqual(await hosts[index].next(), { type: 'ready', role: 'host' });
      assert.deepEqual(await guest.next(), { type: 'ready', role: 'guest' });
      for (const role of ['host', 'guest']) {
        const duplicate = client(key, role);
        assert.equal((await duplicate.next()).type, 'error');
        assert.equal((await duplicate.closure).code, 1008);
      }
    }
    for (let n = 0; n < 32; n++) {
      for (const index of [0, 1]) {
        hosts[index].ws.send(Buffer.from([index, n, 255, 0]));
        guests[index].ws.send(Buffer.from([index, n, 0, 255]));
      }
    }
    for (let n = 0; n < 32; n++) {
      for (const index of [0, 1]) {
        assert.deepEqual(await guests[index].next(), Buffer.from([index, n, 255, 0]));
        assert.deepEqual(await hosts[index].next(), Buffer.from([index, n, 0, 255]));
      }
    }
    firstHost.ws.close();
    assert.equal((await guests[0].next()).type, 'peer-left');
    await Promise.all([firstHost.closure, guests[0].closure]);
    for (const role of ['host', 'guest']) {
      const retired = client(keys[0], role);
      assert.equal((await retired.next()).type, 'error');
      await retired.closure;
    }
    // Closing and rejecting reuse of one edition must leave the other live.
    secondHost.ws.send(Buffer.from('still playing'));
    guests[1].ws.send(Buffer.from('still replying'));
    assert.deepEqual(await guests[1].next(), Buffer.from('still playing'));
    assert.deepEqual(await secondHost.next(), Buffer.from('still replying'));
    assert.deepEqual(secondHost.messages, []);
    assert.deepEqual(guests[1].messages, []);
    assert.equal(secondHost.closed, null);
    assert.equal(guests[1].closed, null);
    guests[1].ws.close();
    assert.equal((await secondHost.next()).type, 'peer-left');
    await Promise.all([secondHost.closure, guests[1].closure]);
  }
});

test('retiring an Original or HD token does not retire the same token in the other edition', async t => {
  const { client, pair } = await fixture(t);
  for (const retiredEdition of ['original', 'hd']) {
    const id = roomId();
    const key = retiredEdition === 'hd' ? `hd/${id}` : id;
    const expired = client(key, 'host');
    assert.deepEqual(await expired.next(), { type: 'waiting' });
    expired.ws.send(Buffer.from('before pairing'));
    assert.match((await expired.next()).message, /Wait for the other player/);
    assert.equal((await expired.closure).code, 1008);
    const retry = client(key, 'host');
    assert.match((await retry.next()).message, /expired/);
    await retry.closure;
    const other = await pair(id, retiredEdition === 'hd' ? 'original' : 'hd');
    other.host.ws.send(Buffer.from('independent token lifetime'));
    assert.deepEqual(await other.guest.next(), Buffer.from('independent token lifetime'));
    other.host.ws.close();
    await Promise.all([other.host.closure, other.guest.closure]);
  }
});

test('room and retired-token capacity is shared across Original and HD editions', async t => {
  const { client } = await fixture(t, { maxRooms: 1, maxRoomIds: 1 });
  const id = roomId();
  const host = client(id, 'host');
  assert.deepEqual(await host.next(), { type: 'waiting' });
  const hd = client(`hd/${id}`, 'host');
  assert.match((await hd.next()).message, /Room limit reached/);
  await hd.closure;
  host.ws.close();
  await host.closure;
  const retiredCapacity = client(`hd/${id}`, 'host');
  assert.match((await retiredCapacity.next()).message, /Room limit reached/);
  await retiredCapacity.closure;
});

test('guests cannot create rooms; duplicate roles and third players cannot displace either player', async t => {
  const { client, pair } = await fixture(t);
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
  const { client, app } = await fixture(t);
  const id = roomId();
  const host = client(id, 'host');
  await host.next();
  host.ws.send(Buffer.from('too soon'));
  assert.match((await host.next()).message, /Wait for the other player/);
  await host.closure;
  assert.equal(app.stats.rooms, 0);
  for (const role of ['host', 'guest']) {
    const retry = client(id, role);
    assert.equal((await retry.next()).type, 'error');
    await retry.closure;
  }
});

test('text and zero-length messages close both sides instead of entering the binary game stream', async t => {
  const { pair } = await fixture(t);
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

test('disconnect notifies and closes the survivor, releases active rooms, and forbids token reuse', async t => {
  const { pair, client, app } = await fixture(t);
  for (const departingRole of ['host', 'guest']) {
    const peers = await pair();
    const departing = peers[departingRole];
    const survivor = peers[departingRole === 'host' ? 'guest' : 'host'];
    departing.ws.close();
    assert.equal((await survivor.next()).type, 'peer-left');
    await Promise.all([departing.closure, survivor.closure]);
    assert.equal(app.stats.rooms, 0);
    const reuse = client(peers.id, 'host');
    assert.match((await reuse.next()).message, /expired/);
    await reuse.closure;
  }
});

test('same-origin is required by default and explicit origin allowlists are exact', async t => {
  const local = await fixture(t);
  for (const origin of ['https://evil.example', 'null', undefined, `${local.origin}/`]) {
    const denied = local.client(roomId(), 'host', { origin });
    await assert.rejects(denied.opened, /403/);
  }
  const configured = await fixture(t, { allowedOrigins: ['https://game.example'] });
  const denied = configured.client(roomId(), 'host');
  await assert.rejects(denied.opened, /403/);
  const accepted = configured.client(roomId(), 'host', { origin: 'https://game.example' });
  assert.equal((await accepted.next()).type, 'waiting');
});

test('oversized payload is rejected by WebSocket parser and peer gets no partial bytes', async t => {
  const { pair } = await fixture(t);
  const { host, guest } = await pair();
  host.ws.send(Buffer.alloc(65537, 17));
  assert.equal((await host.closure).code, 1009);
  assert.equal((await guest.next()).type, 'peer-left');
  await guest.closure;
  assert.ok(!guest.messages.some(Buffer.isBuffer));
});


test('maximum-size binary frames are forwarded intact', async t => {
  const { pair } = await fixture(t);
  const { host, guest } = await pair();
  const bytes = randomBytes(65536);
  host.ws.send(bytes);
  assert.deepEqual(await guest.next(), bytes);
});

test('excessive message rates close the sending connection and its room', async t => {
  const { pair } = await fixture(t);
  const { host, guest } = await pair();
  for (let n = 0; n < 1001; n++) host.ws.send(Buffer.from([n & 255]));
  assert.match((await host.next()).message, /rate exceeded/);
  assert.equal((await guest.next(value => value.type === 'peer-left')).type, 'peer-left');
  await Promise.all([host.closure, guest.closure]);
});

test('ordinary ping responses and browser heartbeat pongs remain functional', async t => {
  const { pair } = await fixture(t, { heartbeatMs: 250 });
  const { host, guest } = await pair();
  const payload = Buffer.from('diagnostic ping');
  const reply = new Promise(resolve => host.ws.once('pong', resolve));
  host.ws.ping(payload);
  assert.deepEqual(await reply, payload);
  // Observe the server's actual heartbeat traffic instead of assuming that
  // several timer ticks occurred during a fixed sleep on a busy build host.
  const heartbeats = socket => new Promise((resolve, reject) => {
    let observed = 0;
    const finish = error => {
      clearTimeout(timer);
      socket.off('ping', onPing);
      socket.off('close', onClose);
      if (error) reject(error); else resolve();
    };
    const onPing = () => { if (++observed === 3) finish(); };
    const onClose = () => finish(new Error('Peer closed before three heartbeat pings'));
    const timer = setTimeout(() => finish(new Error(`Only ${observed} heartbeat pings observed`)), 5000);
    socket.on('ping', onPing);
    socket.once('close', onClose);
  });
  await Promise.all([heartbeats(host.ws), heartbeats(guest.ws)]);
  host.ws.send(Buffer.from('heartbeat survived'));
  assert.deepEqual(await guest.next(), Buffer.from('heartbeat survived'));
  assert.equal(host.closed, null);
  assert.equal(guest.closed, null);
});

test('ping and unsolicited pong floods close the room under the control-frame quota', async t => {
  const { pair } = await fixture(t);
  for (const control of ['ping', 'pong']) {
    const { host, guest } = await pair();
    for (let n = 0; n < 33; n++) host.ws[control](Buffer.alloc(125, n));
    assert.match((await host.next()).message, /Control frame rate exceeded/);
    assert.equal((await guest.next()).type, 'peer-left');
    assert.equal((await host.closure).code, 1008);
    await guest.closure;
  }
});

test('a ping flood from a paused reader cannot grow the server write queue', async t => {
  const maxBufferedBytes = 65536;
  const { app, pair } = await fixture(t, { maxBufferedBytes });
  let peakWriteQueue = 0;
  app.server.on('connection', socket => {
    const write = socket.write;
    socket.write = function (...args) {
      const result = write.apply(this, args);
      peakWriteQueue = Math.max(peakWriteQueue, this.writableLength);
      return result;
    };
  });
  const { host, guest } = await pair();
  host.ws._socket.pause();
  const payload = Buffer.alloc(125, 1);
  for (let n = 0; n < 40000; n++) host.ws.ping(payload);
  assert.equal((await guest.next()).type, 'peer-left');
  assert.equal(app.stats.rooms, 0);
  // A final error/close notification may add a small fixed amount to the cap.
  assert.ok(peakWriteQueue <= maxBufferedBytes + 512, `write queue grew to ${peakWriteQueue}`);
  host.ws._socket.resume();
  await Promise.all([host.closure, guest.closure]);
});

test('arbitrary WebSocket proxy subprotocols are rejected', async t => {
  const { origin } = await fixture(t);
  const ws = new WebSocket(`${origin.replace('http:', 'ws:')}/netplay/${roomId()}?role=host`,
    'binary', { origin });
  await assert.rejects(new Promise((resolve, reject) => {
    ws.once('open', resolve); ws.once('error', reject);
  }), /400/);
});

test('queued slow-peer data has a strict memory bound and disconnects both peers', async t => {
  const { pair } = await fixture(t, { maxBufferedBytes: 65536, latencyPatternMs: [500] });
  const { host, guest } = await pair();
  for (let n = 0; n < 3; n++) host.ws.send(Buffer.alloc(32768, n));
  assert.match((await host.next()).message, /too slow/);
  assert.equal((await guest.next()).type, 'peer-left');
  await Promise.all([host.closure, guest.closure]);
  assert.ok(!guest.messages.some(Buffer.isBuffer));
});

test('room creation and connection resource limits stay bounded', async t => {
  const { client, app } = await fixture(t, { maxRooms: 1, maxRoomIds: 1 });
  const host = client(roomId(), 'host');
  await host.next();
  const extra = client(roomId(), 'host');
  assert.match((await extra.next()).message, /limit/);
  await extra.closure;
  host.ws.close();
  await host.closure;
  const retiredCapacity = client(roomId(), 'host');
  assert.match((await retiredCapacity.next()).message, /limit/);
  await retiredCapacity.closure;
  assert.equal(app.stats.rooms, 0);
  assert.equal(app.stats.usedRoomIds, 1);
});

test('waiting rooms expire and silent clients fail the heartbeat', async t => {
  const waiting = await fixture(t, { waitingMs: 30 });
  const host = waiting.client(roomId(), 'host');
  await host.next();
  assert.match((await host.next()).message, /expired/);
  await host.closure;
  assert.equal(waiting.app.stats.rooms, 0);
  const heartbeat = await fixture(t, { heartbeatMs: 20 });
  const silent = heartbeat.client(roomId(), 'host', { autoPong: false });
  await silent.next();
  await silent.closure;
  assert.equal(heartbeat.app.stats.rooms, 0);
});

test('absolute room lifetime and idle timeout notify both peers', async t => {
  for (const [options, message] of [
    [{ lifetimeMs: 2500 }, 'Room time limit reached.'],
    [{ idleMs: 500, heartbeatMs: 100 }, 'Room was idle for too long.']
  ]) {
    // Leave enough time to pair even while real-browser builds/tests run.
    // Check the reason so an unrelated heartbeat failure cannot pass as expiry.
    const { pair, app } = await fixture(t, options);
    const { host, guest } = await pair();
    assert.deepEqual(await host.next(), { type: 'peer-left', message });
    assert.deepEqual(await guest.next(), { type: 'peer-left', message });
    await Promise.all([host.closure, guest.closure]);
    assert.equal(app.stats.rooms, 0);
  }
});

function request(origin, requestPath, method = 'GET') {
  return new Promise((resolve, reject) => {
    const req = http.request(origin, { path: requestPath, method }, res => {
      const chunks = [];
      res.on('data', chunk => chunks.push(chunk));
      res.on('end', () => resolve({ status: res.statusCode, headers: res.headers, body: Buffer.concat(chunks) }));
    });
    req.on('error', reject);
    req.end();
  });
}

test('static serving isolates WASM, preserves files, supports HEAD, and blocks traversal/listing/symlinks', async t => {
  const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-relay-'));
  t.after(() => rm(temp, { recursive: true, force: true }));
  const dist = path.join(temp, 'dist');
  await mkdir(dist);
  await mkdir(path.join(dist, 'folder'));
  await writeFile(path.join(dist, 'index.html'), '<canvas>game</canvas>');
  await writeFile(path.join(dist, 'game.wasm'), Buffer.from([0, 97, 115, 109]));
  for (const extension of ['glb', 'ogg', 'wav']) await writeFile(path.join(dist, `asset.${extension}`), 'fixture');
  await writeFile(path.join(temp, 'secret.txt'), 'outside');
  await writeFile(path.join(dist, '.secret'), 'hidden');
  await symlink(path.join(temp, 'secret.txt'), path.join(dist, 'linked.txt'));
  const { origin } = await fixture(t, { distDir: dist });
  const page = await request(origin, '/');
  assert.equal(page.status, 200);
  assert.equal(page.body.toString(), '<canvas>game</canvas>');
  assert.equal(page.headers['cross-origin-opener-policy'], 'same-origin');
  assert.equal(page.headers['cross-origin-embedder-policy'], 'require-corp');
  const wasm = await request(origin, '/game.wasm');
  assert.equal(wasm.headers['content-type'], 'application/wasm');
  assert.deepEqual(wasm.body, Buffer.from([0, 97, 115, 109]));
  for (const [extension, contentType] of [['glb', 'model/gltf-binary'], ['ogg', 'audio/ogg'], ['wav', 'audio/wav']]) {
    assert.equal((await request(origin, `/asset.${extension}`)).headers['content-type'], contentType);
  }
  const head = await request(origin, '/game.wasm', 'HEAD');
  assert.equal(head.status, 200);
  assert.equal(head.headers['content-length'], '4');
  assert.equal(head.body.length, 0);
  for (const url of ['/folder', '/folder/', '/../secret.txt', '/%2e%2e/secret.txt',
    '/..%2fsecret.txt', '/linked.txt', '/.secret', '/bad%00name', '/bad%zz']) {
    assert.equal((await request(origin, url)).status, 404, url);
  }
  assert.equal((await request(origin, '/', 'POST')).status, 405);
});


test('Cosmo Clash rooms isolate active and retired identities from Original and HD', async t => {
  const {client} = await fixture(t);
  const id = roomId();
  const keys = [id, `hd/${id}`, `cosmoclash/${id}`];
  const hosts = [], guests = [];
  for (const key of keys) { const host = client(key, 'host'); assert.equal((await host.next()).type, 'waiting'); hosts.push(host); }
  for (let i = 0; i < keys.length; i++) {
    const guest = client(keys[i], 'guest'); guests.push(guest);
    assert.equal((await guest.next()).type, 'waiting');
    assert.equal((await hosts[i].next()).type, 'ready'); assert.equal((await guest.next()).type, 'ready');
  }
  for (let i = 0; i < keys.length; i++) hosts[i].ws.send(Buffer.from(`edition ${i}`));
  for (let i = 0; i < keys.length; i++) assert.deepEqual(await guests[i].next(), Buffer.from(`edition ${i}`));
  hosts[2].ws.close(); assert.equal((await guests[2].next()).type, 'peer-left');
  await Promise.all([hosts[2].closure, guests[2].closure]);
  const retired = client(keys[2], 'host'); assert.match((await retired.next()).message, /expired/); await retired.closure;
  for (let i = 0; i < 2; i++) {
    hosts[i].ws.send(Buffer.from(`still connected ${i}`)); assert.deepEqual(await guests[i].next(), Buffer.from(`still connected ${i}`));
    assert.equal(hosts[i].closed, null); assert.equal(guests[i].closed, null);
  }
});

test('Cosmo Clash room URLs stay strict and share room and retired-ID limits', async t => {
  const {client} = await fixture(t);
  for (const url of [
    '/netplay/cosmoclash/short?role=host', `/netplay/cosmoclash/${'A'.repeat(32)}?role=host`,
    `/netplay/CosmoClash/${roomId()}?role=host`, `/netplay/cosmoclash/hd/${roomId()}?role=host`,
    `/netplay/cosmoclash/${roomId()}?role=host&role=guest`, `/netplay/cosmoclash/${roomId()}/?role=host`,
    `/netplay/cosmoclash%2f${roomId()}?role=host`, `/netplay/cosmoclash/${roomId()}?role=owner`,
  ]) { const bad = client(null, null, {}, url); assert.equal((await bad.next()).type, 'error'); await bad.closure; }
  const bounded = await fixture(t, {maxRooms: 1, maxRoomIds: 1});
  const host = bounded.client(`cosmoclash/${roomId()}`, 'host'); assert.equal((await host.next()).type, 'waiting');
  const full = bounded.client(roomId(), 'host'); assert.match((await full.next()).message, /limit/); await full.closure;
  host.ws.close(); await host.closure;
  const retiredFull = bounded.client(`hd/${roomId()}`, 'host'); assert.match((await retiredFull.next()).message, /limit/); await retiredFull.closure;
});
