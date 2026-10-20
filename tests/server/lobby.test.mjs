// Room lists (lobbies) of the Node relay with real WebSocket clients. The same
// cases run against the Cloudflare relay in tests/worker/lobby.test.mjs.
import test from 'node:test';
import assert from 'node:assert/strict';
import { randomBytes } from 'node:crypto';
import { WebSocket } from 'ws';
import { createNetplayServer, parseRoomURL, addressKey } from '../../server/netplay.mjs';
import names from '../../sc2/wasm/tests/room-names.cjs';

const roomId = () => randomBytes(16).toString('hex');
const pause = ms => new Promise(resolve => setTimeout(resolve, ms));
const NAME_ERROR = /^Room names need 1 to 24 visible characters\.$/;
// encodeURIComponent cannot encode a lone surrogate; this is its UTF-8 form, which decodes to an error.
const encodeName = name => { try { return encodeURIComponent(name); } catch { return '%ED%A0%80'; } };

function roomPath(id, role, { edition, name, engine } = {}) {
  let url = `/netplay/${edition && edition !== 'original' ? edition + '/' : ''}${id}?role=${role}`;
  if (name !== undefined) url += '&name=' + encodeName(name);
  if (engine !== undefined) url += '&engine=' + engine;
  return url;
}

async function fixture(t, options = {}) {
  const app = createNetplayServer({ distDir: false, ...options });
  const address = await app.listen(0);
  const origin = `http://127.0.0.1:${address.port}`;
  t.after(() => app.close());
  function socket(url, clientOptions = {}, protocol) {
    const target = `ws://127.0.0.1:${address.port}${url}`;
    const ws = protocol ? new WebSocket(target, protocol, { origin, ...clientOptions })
      : new WebSocket(target, { origin, ...clientOptions });
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
      },
      /** The next snapshot whose room list passes the check. */
      rooms(check = () => true, timeout) {
        return peer.next(value => value?.type === 'rooms' && check(value.rooms), timeout).then(value => value.rooms);
      },
    };
    return peer;
  }
  const host = (id, listing, clientOptions) => socket(roomPath(id, 'host', listing), clientOptions);
  const guest = (id, edition, clientOptions) => socket(roomPath(id, 'guest', { edition }), clientOptions);
  const lobby = (edition = 'original', clientOptions) => socket(`/netplay/lobby?edition=${edition}`, clientOptions);
  return { app, origin, socket, host, guest, lobby };
}

const ids = rooms => rooms.map(room => room.id);

test('the room list shows named rooms of its own edition only, newest first, with exact fields', async t => {
  const { host, lobby, app } = await fixture(t);
  const original = lobby('original');
  const hd = lobby('hd');
  assert.deepEqual(await original.next(), { type: 'rooms', rooms: [] });
  assert.deepEqual(await hd.next(), { type: 'rooms', rooms: [] });
  const [a, b, c] = [roomId(), roomId(), roomId()];
  const before = Date.now();
  const first = host(a, { name: '  Alpha  ', engine: 'c' });
  assert.deepEqual(await first.next(), { type: 'waiting' });
  const [listed] = await original.rooms(rooms => rooms.length === 1);
  assert.deepEqual(Object.keys(listed).sort(), ['created', 'engine', 'id', 'name']);
  assert.equal(listed.id, a);
  assert.equal(listed.name, 'Alpha', 'the name is trimmed');
  assert.equal(listed.engine, 'c');
  assert.ok(listed.created >= before && listed.created <= Date.now());
  const second = host(b, { name: 'Beta' });
  await second.next();
  const both = await original.rooms(rooms => rooms.length === 2);
  assert.deepEqual(ids(both), [b, a], 'newest first');
  assert.equal(both[0].engine, null, 'a room without engine lists null');
  // The same token in HD is an independent HD room in the HD list.
  const third = host(c, { edition: 'hd', name: 'גמא', engine: 'rust' });
  await third.next();
  const sameToken = host(a, { edition: 'hd', name: 'HD Alpha' });
  await sameToken.next();
  const hdRooms = await hd.rooms(rooms => rooms.length === 2);
  assert.deepEqual(hdRooms.map(({ id, name, engine }) => ({ id, name, engine })), [
    { id: a, name: 'HD Alpha', engine: null },
    { id: c, name: 'גמא', engine: 'rust' },
  ]);
  await pause(600);
  for (const snapshot of original.all) assert.ok(!snapshot.rooms.some(room => room.name.startsWith('HD') || room.id === c));
  for (const snapshot of hd.all) assert.ok(!snapshot.rooms.some(room => room.id === b));
  assert.equal(app.stats.listedRooms, 4);
  assert.equal(app.stats.lobbyConnections, 2);
  assert.equal(app.stats.connections, 4, 'lobby sockets are not room connections');
});

test('Cosmo Clash has its own room list: its rooms are listed there only, with the rust engine tag', async t => {
  const { host, guest, lobby, socket, app } = await fixture(t);
  const watchers = { original: lobby('original'), hd: lobby('hd'), cosmoclash: lobby('cosmoclash') };
  for (const watcher of Object.values(watchers)) assert.deepEqual(await watcher.next(), { type: 'rooms', rooms: [] });
  const id = roomId();
  const before = Date.now();
  const cosmo = host(id, { edition: 'cosmoclash', name: '  Cosmo   duel ', engine: 'rust' });
  assert.deepEqual(await cosmo.next(), { type: 'waiting' });
  const [listed] = await watchers.cosmoclash.rooms(rooms => rooms.length === 1);
  assert.deepEqual(Object.keys(listed).sort(), ['created', 'engine', 'id', 'name']);
  assert.deepEqual({ id: listed.id, name: listed.name, engine: listed.engine }, { id, name: 'Cosmo duel', engine: 'rust' });
  assert.ok(listed.created >= before && listed.created <= Date.now());
  // The same token in the UQM editions makes independent rooms in their own lists.
  const original = host(id, { name: 'UQM duel', engine: 'c' });
  const hd = host(id, { edition: 'hd', name: 'HD duel' });
  for (const peer of [original, hd]) assert.deepEqual(await peer.next(), { type: 'waiting' });
  await watchers.original.rooms(rooms => rooms.length === 1);
  await watchers.hd.rooms(rooms => rooms.length === 1);
  // A Cosmo Clash room takes only the rust tag; without a tag it lists null.
  for (const url of [`/netplay/cosmoclash/${roomId()}?role=host&name=a&engine=c`,
    `/netplay/cosmoclash/${roomId()}?role=host&engine=rust`, `/netplay/cosmoclash/${roomId()}?role=guest&engine=rust`]) {
    const refused = socket(url);
    assert.match((await refused.next()).message, /Invalid room URL/, url);
  }
  const untagged = roomId();
  const plain = host(untagged, { edition: 'cosmoclash', name: 'No tag' });
  await plain.next();
  assert.equal((await watchers.cosmoclash.rooms(rooms => rooms.length === 2)).find(room => room.id === untagged).engine, null);
  // A guest who picks the Cosmo Clash room joins it, not the UQM room with the same token.
  const joining = guest(id, 'cosmoclash');
  assert.deepEqual(await joining.next(), { type: 'waiting' });
  assert.deepEqual(await joining.next(), { type: 'ready', role: 'guest' });
  assert.deepEqual(await cosmo.next(), { type: 'ready', role: 'host' });
  assert.deepEqual(ids(await watchers.cosmoclash.rooms(rooms => rooms.length === 1)), [untagged]);
  cosmo.ws.send(Buffer.from('cosmo'));
  assert.deepEqual(await joining.next(Buffer.isBuffer), Buffer.from('cosmo'));
  assert.equal(original.messages.length + hd.messages.length, 0, 'the UQM hosts with the same token got nothing');
  await pause(600);
  for (const snapshot of watchers.original.all) assert.ok(snapshot.rooms.every(room => room.name === 'UQM duel'));
  for (const snapshot of watchers.hd.all) assert.ok(snapshot.rooms.every(room => room.name === 'HD duel'));
  for (const snapshot of watchers.cosmoclash.all) assert.ok(snapshot.rooms.every(room => ['Cosmo duel', 'No tag'].includes(room.name)));
  assert.equal(app.stats.listedRooms, 3);
  watchers.cosmoclash.ws.send('ping');
  assert.equal(await watchers.cosmoclash.next(value => value === 'pong'), 'pong');
});

test('the room list bounds count each edition apart, also for Cosmo Clash', async t => {
  const { host, lobby } = await fixture(t, { maxListedRooms: 1 });
  const watcher = lobby('cosmoclash');
  await watcher.rooms();
  const original = host(roomId(), { name: 'UQM' });
  assert.deepEqual(await original.next(), { type: 'waiting' });
  const first = host(roomId(), { edition: 'cosmoclash', name: 'First', engine: 'rust' });
  assert.deepEqual(await first.next(), { type: 'waiting' });
  const second = host(roomId(), { edition: 'cosmoclash', name: 'Second', engine: 'rust' });
  assert.match((await second.next()).message, /room list is full/);
  assert.deepEqual((await watcher.rooms(rooms => rooms.length === 1)).map(room => room.name), ['First']);
});

test('private rooms are never listed, and invites to listed rooms keep working', async t => {
  const { host, guest, lobby } = await fixture(t);
  const watcher = lobby();
  await watcher.rooms();
  const hidden = roomId();
  const privateHost = host(hidden);
  await privateHost.next();
  const named = roomId();
  const namedHost = host(named, { name: 'Open' });
  await namedHost.next();
  assert.deepEqual(ids(await watcher.rooms(rooms => rooms.length > 0)), [named]);
  // A guest joins the private room with its invite token.
  const privateGuest = guest(hidden);
  assert.deepEqual(await privateGuest.next(), { type: 'waiting' });
  assert.deepEqual(await privateHost.next(), { type: 'ready', role: 'host' });
  await pause(600);
  for (const snapshot of watcher.all) assert.ok(!ids(snapshot.rooms).includes(hidden));
});

test('a listed room leaves the list when a guest joins, when its host leaves, and when it fails', async t => {
  const { host, guest, lobby } = await fixture(t);
  const watcher = lobby();
  await watcher.rooms();
  const paired = roomId();
  const pairedHost = host(paired, { name: 'Pairs' });
  await pairedHost.next();
  await watcher.rooms(rooms => ids(rooms).includes(paired));
  const joining = guest(paired);
  assert.deepEqual(await joining.next(), { type: 'waiting' });
  assert.deepEqual(await joining.next(), { type: 'ready', role: 'guest' });
  assert.deepEqual(await pairedHost.next(), { type: 'ready', role: 'host' });
  await watcher.rooms(rooms => !ids(rooms).includes(paired));
  // The paired room still relays game data.
  pairedHost.ws.send(Buffer.from([7, 7]));
  assert.deepEqual(await joining.next(), Buffer.from([7, 7]));
  // A second guest cannot take the paired room.
  const late = guest(paired);
  assert.match((await late.next()).message, /two players/);

  const left = roomId();
  const leaving = host(left, { name: 'Leaves' });
  await leaving.next();
  await watcher.rooms(rooms => ids(rooms).includes(left));
  leaving.ws.close();
  await watcher.rooms(rooms => !ids(rooms).includes(left));

  const failed = roomId();
  const early = host(failed, { name: 'Too soon' });
  await early.next();
  await watcher.rooms(rooms => ids(rooms).includes(failed));
  early.ws.send(Buffer.from('before a guest'));
  assert.match((await early.next()).message, /Wait for the other player/);
  await watcher.rooms(rooms => !ids(rooms).includes(failed));
});

test('room names are validated, and name and engine are host-only parameters', async t => {
  const { socket, host, lobby } = await fixture(t, { maxListedPerIp: 10 });
  const id = roomId();
  for (const [url, message] of [
    ...names.refused.map(([, name]) => [roomPath(id, 'host', { name }), NAME_ERROR]),
    [`/netplay/${id}?role=host&name=%ZZ`, NAME_ERROR],
    [`/netplay/${id}?role=host&name=${'%41'.repeat(97)}`, NAME_ERROR],
    [`/netplay/${id}?role=host&name=a&name=b`, /Invalid room URL/],
    [`/netplay/${id}?role=guest&name=a`, /Invalid room URL/],
    [`/netplay/${id}?role=guest&engine=c`, /Invalid room URL/],
    [`/netplay/${id}?role=host&engine=c`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&engine=go`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&engine=C`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&engine=c&engine=c`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&lobby=1`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&`, /Invalid room URL/],
    [`/netplay/${id}?name=a`, /Invalid room URL/],
    [`/netplay/hd/${id}?role=host&name=a&target=localhost`, /Invalid room URL/],
  ]) {
    const invalid = socket(url);
    const reply = await invalid.next();
    assert.equal(reply.type, 'error', url);
    assert.match(reply.message, message, url);
    assert.equal((await invalid.closure).code, 1008, url);
  }
  // Rejected names do not retire the token.
  const watcher = lobby();
  await watcher.rooms();
  const accepted = host(id, { name: 'א'.repeat(24), engine: 'rust' });
  assert.deepEqual(await accepted.next(), { type: 'waiting' });
  assert.equal((await watcher.rooms(rooms => rooms.length === 1))[0].name, 'א'.repeat(24));
  for (const [raw, name] of [['Red+Blue', 'Red Blue'], ['1%2B1', '1+1'], ['%F0%9F%9A%80%20Rocket', '🚀 Rocket'],
    ['a=b', 'a=b'], ['Caf%65%CC%81%20%20Bar', 'Café Bar']]) {
    const named = socket(`/netplay/${roomId()}?role=host&name=${raw}`);
    assert.deepEqual(await named.next(), { type: 'waiting' }, raw);
    assert.ok((await watcher.rooms(rooms => rooms.some(room => room.name === name))), raw);
  }
});

test('the room URL parser lists the cleaned name and refuses the same names as the client', () => {
  const id = roomId();
  for (const [input, listed] of names.accepted) {
    assert.equal(parseRoomURL(roomPath(id, 'host', { name: input })).name, listed, JSON.stringify(input));
  }
  for (const [label, input] of names.refused) {
    assert.match(parseRoomURL(roomPath(id, 'host', { name: input })).error, NAME_ERROR, label);
  }
});

test('per-address bounds count an IPv6 client by its /64 network and IPv4 addresses one by one', () => {
  for (const [ip, key] of [
    ['203.0.113.7', '203.0.113.7'],
    ['::ffff:203.0.113.7', '203.0.113.7'],
    ['::FFFF:cb00:7107', '203.0.113.7'],
    ['2001:db8::1', '2001:db8:0:0::/64'],
    ['2001:db8:0:0:ffff:ffff:ffff:ffff', '2001:db8:0:0::/64'],
    ['2001:0DB8:0000:0000:0000:0000:0000:0016', '2001:db8:0:0::/64'],
    ['2001:db8:0:1::1', '2001:db8:0:1::/64'],
    ['2001:db8::1.2.3.4', '2001:db8:0:0::/64'],
    ['fe80::1%en0', 'fe80:0:0:0::/64'],
    ['::1', '0:0:0:0::/64'],
    [undefined, 'unknown'],
    ['', 'unknown'],
    ['1::2::3', '1::2::3'],
    ['1:2:3:4:5:6:7:8:9', '1:2:3:4:5:6:7:8:9'],
  ]) assert.equal(addressKey(ip), key, String(ip));
  assert.ok(addressKey('x'.repeat(100)).length <= 64);
});

test('the room URL parser accepts parameters in any order but each one only once', () => {
  const id = roomId();
  assert.deepEqual(parseRoomURL(`/netplay/${id}?engine=c&name=A&role=host`),
    { edition: null, token: id, role: 'host', name: 'A', engine: 'c' });
  assert.deepEqual(parseRoomURL(`/netplay/hd/${id}?role=guest`),
    { edition: 'hd', token: id, role: 'guest', name: null, engine: null });
  assert.deepEqual(parseRoomURL(`/netplay/cosmoclash/${id}?role=host&name=Cosmo&engine=rust`),
    { edition: 'cosmoclash', token: id, role: 'host', name: 'Cosmo', engine: 'rust' });
  assert.ok(parseRoomURL(`/netplay/cosmoclash/${id}?role=host&name=Cosmo&engine=c`).error);
  for (const url of [`/netplay/${id}?role=host&role=host`, `/netplay/${id}?Role=host`, `/netplay/${id}?role=host&name`,
    `/netplay/${id}?role=host#x`, `/netplay/${id}`, `/netplay/lobby?edition=hd`]) {
    assert.ok(parseRoomURL(url).error, url);
  }
});

test('listed rooms wait longer than private rooms, then expire and leave the list', async t => {
  const { host, lobby, app } = await fixture(t, { waitingMs: 300, listedWaitingMs: 1500 });
  const watcher = lobby();
  await watcher.rooms();
  const privateHost = host(roomId());
  await privateHost.next();
  const named = roomId();
  const namedHost = host(named, { name: 'Patient' });
  await namedHost.next();
  await watcher.rooms(rooms => ids(rooms).includes(named));
  assert.match((await privateHost.next()).message, /expired/);
  await privateHost.closure;
  assert.equal(namedHost.closed, null, 'the listed room still waits');
  assert.match((await namedHost.next(value => value.type === 'error', 4000)).message, /expired/);
  await namedHost.closure;
  await watcher.rooms(rooms => !ids(rooms).includes(named));
  assert.equal(app.stats.listedRooms, 0);
});

test('the room list is bounded in total and per address, and a refused name does not retire its token', async t => {
  const full = await fixture(t, { maxListedRooms: 2 });
  const first = full.host(roomId(), { name: 'One' });
  const second = full.host(roomId(), { name: 'Two' });
  await Promise.all([first.next(), second.next()]);
  const refusedId = roomId();
  const refused = full.host(refusedId, { name: 'Three' });
  assert.match((await refused.next()).message, /room list is full/);
  assert.equal((await refused.closure).code, 1008);
  // Private rooms are not part of the list bound.
  const privateHost = full.host(roomId());
  assert.deepEqual(await privateHost.next(), { type: 'waiting' });
  first.ws.close();
  await first.closure;
  await pause(50);
  const retry = full.host(refusedId, { name: 'Three' });
  assert.deepEqual(await retry.next(), { type: 'waiting' });

  const perIp = await fixture(t, { maxListedPerIp: 1 });
  const mineId = roomId();
  const mine = perIp.host(mineId, { name: 'Mine' });
  await mine.next();
  const another = perIp.host(roomId(), { name: 'Another' });
  assert.match((await another.next()).message, /most open rooms/);
  // Once the first room pairs it leaves the list, and the address may list another.
  const opponent = perIp.guest(mineId);
  await opponent.next(value => value.type === 'ready');
  const next = perIp.host(roomId(), { name: 'Next' });
  assert.deepEqual(await next.next(), { type: 'waiting' });
});

test('lobby sockets check the origin, the URL and subprotocols, and have their own connection bounds', async t => {
  const local = await fixture(t, { maxLobbyPerIp: 2, maxPerIp: 2 });
  for (const origin of ['https://evil.example', 'null', undefined, `${local.origin}/`]) {
    const denied = local.lobby('original', { origin });
    await assert.rejects(denied.opened, /403/, String(origin));
  }
  for (const url of ['/netplay/lobby', '/netplay/lobby?edition=3do', '/netplay/lobby?edition=HD',
    '/netplay/lobby?edition=hd&x=1', '/netplay/lobby?edition=original&edition=hd', '/netplay/lobby?',
    '/netplay/lobby/?edition=hd', '/netplay/lobby/hd', '/netplay/hd/lobby?edition=hd',
    '/netplay/lobby?edition=cosmo', '/netplay/lobby?edition=CosmoClash', '/netplay/cosmoclash/lobby?edition=cosmoclash']) {
    const invalid = local.socket(url);
    assert.equal((await invalid.next()).type, 'error', url);
    assert.equal((await invalid.closure).code, 1008, url);
  }
  const subprotocol = local.socket('/netplay/lobby?edition=hd', {}, 'binary');
  await assert.rejects(subprotocol.opened, /400/);
  const plain = await fetch(`${local.origin}/netplay/lobby?edition=hd`, { headers: { Origin: local.origin } });
  assert.equal(plain.status, 404, 'plain HTTP is not a lobby');
  const a = local.lobby();
  const b = local.lobby('hd');
  await Promise.all([a.rooms(), b.rooms()]);
  const third = local.lobby();
  await assert.rejects(third.opened, /503/);
  // Lobby sockets leave the room connections of the address free.
  const hosts = [local.host(roomId()), local.host(roomId())];
  for (const peer of hosts) assert.deepEqual(await peer.next(), { type: 'waiting' });
  a.ws.close();
  await a.closure;
  await pause(50);
  const again = local.lobby();
  assert.deepEqual(await again.next(), { type: 'rooms', rooms: [] });

  const total = await fixture(t, { maxLobbyConnections: 1 });
  const only = total.lobby();
  await only.rooms();
  await assert.rejects(total.lobby('hd').opened, /503/);
});

test('lobby sockets accept only keepalive pings, within the control quota', async t => {
  const { lobby } = await fixture(t);
  const watcher = lobby();
  await watcher.rooms();
  watcher.ws.send('ping');
  assert.equal(await watcher.next(), 'pong');
  for (const payload of ['hello', Buffer.from([1])]) {
    const talker = lobby();
    await talker.rooms();
    talker.ws.send(payload);
    assert.match((await talker.next()).message, /accepts no messages/);
    assert.equal((await talker.closure).code, 1008);
  }
  const flood = lobby();
  await flood.rooms();
  for (let n = 0; n < 40; n++) flood.ws.send('ping');
  assert.match((await flood.next(value => value?.type === 'error')).message, /Control frame rate exceeded/);
  await flood.closure;
  assert.equal(watcher.closed, null);
});

test('a waiting host can keep its room alive with ping texts, and the room still pairs', async t => {
  const { host, guest } = await fixture(t);
  for (const listing of [{ name: 'Keepalive' }, undefined]) {
    const id = roomId();
    const waiting = host(id, listing);
    await waiting.next();
    waiting.ws.send('ping');
    assert.equal(await waiting.next(), 'pong');
    const joining = guest(id);
    await joining.next(value => value.type === 'ready');
    await waiting.next(value => value.type === 'ready');
    waiting.ws.send(Buffer.from('game'));
    assert.deepEqual(await joining.next(Buffer.isBuffer), Buffer.from('game'));
  }
});

test('snapshots are throttled to about two per second and always end with the latest list', async t => {
  const { host, lobby } = await fixture(t, { maxListedPerIp: 20 });
  const watcher = lobby();
  await watcher.rooms();
  const start = watcher.all.length;
  const hosts = Array.from({ length: 12 }, (_, n) => host(roomId(), { name: 'Room ' + n }));
  await Promise.all(hosts.map(peer => peer.next()));
  const latest = await watcher.rooms(rooms => rooms.length === 12);
  assert.deepEqual(latest.map(room => room.name).sort(), hosts.map((_, n) => 'Room ' + n).sort());
  for (let n = 1; n < latest.length; n++) assert.ok(latest[n - 1].created >= latest[n].created, 'newest first');
  await pause(700);
  const snapshots = watcher.all.slice(start);
  const times = watcher.times.slice(start);
  assert.ok(snapshots.length < 12, `${snapshots.length} snapshots for 12 changes`);
  for (let n = 1; n < times.length; n++) assert.ok(times[n] - times[n - 1] >= 450, `snapshot gap ${times[n] - times[n - 1]} ms`);
  assert.equal(snapshots.at(-1).rooms.length, 12);
});

test('a listed host that stops answering heartbeats leaves the list', async t => {
  const { socket, lobby, app } = await fixture(t, { heartbeatMs: 250 });
  const watcher = lobby();
  await watcher.rooms();
  const id = roomId();
  const silent = socket(roomPath(id, 'host', { name: 'Silent' }), { autoPong: false });
  await silent.next();
  await watcher.rooms(rooms => ids(rooms).includes(id));
  await watcher.rooms(rooms => !ids(rooms).includes(id), 4000);
  await silent.closure;
  assert.equal(app.stats.listedRooms, 0);
  assert.equal(watcher.closed, null, 'the watcher answers its heartbeats');
});

test('a listed host whose page stops sending pings leaves the list, also when the browser still answers heartbeats', async t => {
  const { host, guest, lobby, app } = await fixture(t, { heartbeatMs: 1000, listedSilenceMs: 2500 });
  const watcher = lobby();
  await watcher.rooms();
  // A frozen page: the browser's network code still answers ping frames, but no script runs.
  const frozenId = roomId();
  const frozen = host(frozenId, { name: 'Frozen' });
  await frozen.next();
  const pingingId = roomId();
  const pinging = host(pingingId, { name: 'Awake' });
  await pinging.next();
  const pings = setInterval(() => pinging.ws.send('ping'), 300);
  t.after(() => clearInterval(pings));
  await watcher.rooms(rooms => ids(rooms).includes(frozenId) && ids(rooms).includes(pingingId));
  const left = await watcher.rooms(rooms => !ids(rooms).includes(frozenId), 10000);
  assert.ok(ids(left).includes(pingingId), 'a host that pings stays listed');
  assert.equal((await frozen.next(value => value?.type === 'error')).message,
    'The room closed because this page stopped answering. Create a new room.');
  await frozen.closure;
  await pause(3000);
  assert.equal(pinging.closed, null);
  assert.ok(ids(watcher.all.at(-1).rooms).includes(pingingId), 'a host that pings stays listed past the limit');
  const joining = guest(pingingId);
  assert.deepEqual(await joining.next(value => value.type === 'ready'), { type: 'ready', role: 'guest' });
  assert.equal(app.stats.listedRooms, 0);
});

test('a guest who picks a silent host from the list is refused, and the room closes', async t => {
  // A slow heartbeat: only the guest's join finds the silent host.
  const { host, guest, lobby } = await fixture(t, { heartbeatMs: 120000, listedSilenceMs: 400 });
  const watcher = lobby();
  await watcher.rooms();
  const id = roomId();
  const silent = host(id, { name: 'Asleep' });
  await silent.next();
  await watcher.rooms(rooms => ids(rooms).includes(id));
  await pause(600);
  assert.ok(ids(watcher.all.at(-1).rooms).includes(id), 'still listed before anyone looks');
  const joining = guest(id);
  assert.match((await joining.next()).message, /does not exist or has expired/);
  assert.match((await silent.next(value => value?.type === 'error')).message, /stopped answering/);
  await watcher.rooms(rooms => !ids(rooms).includes(id));
});

test('one address can join a limited number of listed rooms in a window; private rooms are not counted', async t => {
  const { host, guest, lobby } = await fixture(t, { maxListedJoins: 2, listedJoinWindowMs: 1500, maxListedPerIp: 10 });
  const watcher = lobby();
  await watcher.rooms();
  const listed = [roomId(), roomId(), roomId()];
  const hosts = listed.map((id, n) => host(id, { name: 'Open ' + n }));
  await Promise.all(hosts.map(peer => peer.next()));
  await watcher.rooms(rooms => listed.every(id => ids(rooms).includes(id)));
  for (const id of listed.slice(0, 2)) {
    const joining = guest(id);
    assert.deepEqual(await joining.next(value => value.type === 'ready'), { type: 'ready', role: 'guest' });
    joining.ws.close();
  }
  const third = guest(listed[2]);
  assert.equal((await third.next()).message, 'You joined many open rooms in a short time. Wait a few minutes, then try again.');
  assert.equal((await third.closure).code, 1008);
  // The refused join did not end the room: it still waits on the list.
  await pause(600);
  assert.equal(hosts[2].closed, null);
  assert.deepEqual(ids(watcher.all.at(-1).rooms), [listed[2]]);
  const privateId = roomId();
  const privateHost = host(privateId);
  await privateHost.next();
  const privateGuest = guest(privateId);
  assert.deepEqual(await privateGuest.next(value => value.type === 'ready'), { type: 'ready', role: 'guest' });
  // After the window the address may join again.
  await pause(1600);
  const later = guest(listed[2]);
  assert.deepEqual(await later.next(value => value.type === 'ready'), { type: 'ready', role: 'guest' });
});
