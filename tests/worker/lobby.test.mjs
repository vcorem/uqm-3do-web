// Room lists (lobbies) of the Cloudflare relay under `wrangler dev`, with real
// WebSocket clients. The same cases run against the Node relay in
// tests/server/lobby.test.mjs. Run with `npm run test:worker`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { pause, roomId, startWorker, useDataManifest } from './wrangler-dev.mjs';
import names from '../../sc2/wasm/tests/room-names.cjs';

useDataManifest();

const NAME_ERROR = /^Room names need 1 to 24 visible characters\.$/;
// encodeURIComponent cannot encode a lone surrogate; this is its UTF-8 form, which decodes to an error.
const encodeName = name => { try { return encodeURIComponent(name); } catch { return '%ED%A0%80'; } };
const ids = rooms => rooms.map(room => room.id);
// The relay counts rooms and lobby sockets per CF-Connecting-IP. Each test uses its own addresses.
let nextAddress = 1;
const address = () => ({ headers: { 'CF-Connecting-IP': `198.51.100.${nextAddress++}` } });

function roomPath(id, role, { edition, name, engine } = {}) {
  let url = `/netplay/${edition && edition !== 'original' ? edition + '/' : ''}${id}?role=${role}`;
  if (name !== undefined) url += '&name=' + encodeName(name);
  if (engine !== undefined) url += '&engine=' + engine;
  return url;
}

function helpers(worker) {
  return {
    ...worker,
    host: (id, listing, clientOptions = address()) => worker.socket(roomPath(id, 'host', listing), clientOptions),
    guest: (id, edition, clientOptions = address()) => worker.socket(roomPath(id, 'guest', { edition }), clientOptions),
    lobby: (edition = 'original', clientOptions = address()) => worker.socket(`/netplay/lobby?edition=${edition}`, clientOptions),
  };
}

// One instance with the production bounds serves the tests that need nothing special.
let shared;
const sharedCleanups = [];
test.after(() => Promise.all(sharedCleanups.map(cleanup => cleanup())));
async function defaultWorker() {
  shared ??= startWorker({ after: cleanup => sharedCleanups.push(cleanup) });
  return helpers(await shared);
}

test('the room list shows named rooms of its own edition only, newest first, with exact fields', async () => {
  const { host, lobby } = await defaultWorker();
  const original = lobby('original');
  const hd = lobby('hd');
  const [a, b, c] = [roomId(), roomId(), roomId()];
  await Promise.all([original.rooms(), hd.rooms()]);
  const before = Date.now();
  const first = host(a, { name: '  Alpha  ', engine: 'c' });
  assert.deepEqual(await first.next(), { type: 'waiting' });
  const listed = (await original.rooms(rooms => ids(rooms).includes(a))).find(room => room.id === a);
  assert.deepEqual(Object.keys(listed).sort(), ['created', 'engine', 'id', 'name']);
  assert.equal(listed.name, 'Alpha', 'the name is trimmed');
  assert.equal(listed.engine, 'c');
  assert.ok(listed.created >= before - 1000 && listed.created <= Date.now() + 1000);
  const second = host(b, { name: 'Beta' });
  await second.next();
  const both = await original.rooms(rooms => ids(rooms).includes(b));
  assert.deepEqual(ids(both).filter(id => id === a || id === b), [b, a], 'newest first');
  assert.equal(both.find(room => room.id === b).engine, null);
  const third = host(c, { edition: 'hd', name: 'גמא', engine: 'rust' });
  await third.next();
  // The same token in HD is an independent HD room in the HD list.
  const sameToken = host(a, { edition: 'hd', name: 'HD Alpha' });
  await sameToken.next();
  const hdRooms = await hd.rooms(rooms => ids(rooms).includes(a) && ids(rooms).includes(c));
  assert.deepEqual(hdRooms.filter(room => room.id === a || room.id === c).map(({ id, name, engine }) => ({ id, name, engine })), [
    { id: a, name: 'HD Alpha', engine: null },
    { id: c, name: 'גמא', engine: 'rust' },
  ]);
  await pause(700);
  for (const snapshot of original.all) assert.ok(!snapshot.rooms.some(room => room.id === c || room.name === 'HD Alpha'));
  for (const snapshot of hd.all) assert.ok(!ids(snapshot.rooms).includes(b));
  for (const peer of [first, second, third, sameToken]) peer.ws.close();
});

test('Cosmo Clash has its own room list: its rooms are listed there only, with the rust engine tag', async () => {
  const { host, guest, lobby, socket } = await defaultWorker();
  const watchers = { original: lobby('original'), hd: lobby('hd'), cosmoclash: lobby('cosmoclash') };
  await Promise.all(Object.values(watchers).map(watcher => watcher.rooms()));
  const id = roomId();
  const before = Date.now();
  const cosmo = host(id, { edition: 'cosmoclash', name: '  Cosmo   duel ', engine: 'rust' });
  assert.deepEqual(await cosmo.next(), { type: 'waiting' });
  const listed = (await watchers.cosmoclash.rooms(rooms => ids(rooms).includes(id))).find(room => room.id === id);
  assert.deepEqual(Object.keys(listed).sort(), ['created', 'engine', 'id', 'name']);
  assert.deepEqual({ name: listed.name, engine: listed.engine }, { name: 'Cosmo duel', engine: 'rust' });
  assert.ok(listed.created >= before - 1000 && listed.created <= Date.now() + 1000);
  // The same token in the UQM editions makes independent rooms in their own lists.
  const original = host(id, { name: 'UQM duel', engine: 'c' });
  const hd = host(id, { edition: 'hd', name: 'HD duel' });
  for (const peer of [original, hd]) assert.deepEqual(await peer.next(), { type: 'waiting' });
  await watchers.original.rooms(rooms => ids(rooms).includes(id));
  await watchers.hd.rooms(rooms => ids(rooms).includes(id));
  // A Cosmo Clash room takes only the rust tag; a named room without a tag is listed as rust too.
  for (const url of [`/netplay/cosmoclash/${roomId()}?role=host&name=a&engine=c`,
    `/netplay/cosmoclash/${roomId()}?role=host&engine=rust`, `/netplay/cosmoclash/${roomId()}?role=guest&engine=rust`]) {
    const refused = socket(url, address());
    assert.match((await refused.next()).message, /Invalid room URL/, url);
  }
  const untagged = roomId();
  const plain = host(untagged, { edition: 'cosmoclash', name: 'No tag' });
  assert.deepEqual(await plain.next(), { type: 'waiting' });
  assert.equal((await watchers.cosmoclash.rooms(rooms => ids(rooms).includes(untagged))).find(room => room.id === untagged).engine, 'rust');
  // A guest who picks the Cosmo Clash room joins it, not the UQM rooms with the same token.
  const joining = guest(id, 'cosmoclash');
  assert.deepEqual(await joining.next(), { type: 'waiting' });
  assert.deepEqual(await joining.next(), { type: 'ready', role: 'guest' });
  assert.deepEqual(await cosmo.next(), { type: 'ready', role: 'host' });
  await watchers.cosmoclash.rooms(rooms => !ids(rooms).includes(id));
  cosmo.ws.send(Buffer.from('cosmo'));
  assert.deepEqual(await joining.next(Buffer.isBuffer), Buffer.from('cosmo'));
  await pause(700);
  assert.equal(original.messages.length + hd.messages.length, 0, 'the UQM hosts with the same token got nothing');
  for (const snapshot of watchers.original.all) assert.ok(!snapshot.rooms.some(room => room.name === 'Cosmo duel' || room.name === 'HD duel'));
  for (const snapshot of watchers.hd.all) assert.ok(!snapshot.rooms.some(room => room.name === 'Cosmo duel' || room.name === 'UQM duel'));
  for (const snapshot of watchers.cosmoclash.all) assert.ok(!snapshot.rooms.some(room => room.name.endsWith('duel') && room.name !== 'Cosmo duel'));
  for (const snapshot of watchers.cosmoclash.all) assert.ok(snapshot.rooms.every(room => room.engine === 'rust'));
  watchers.cosmoclash.ws.send('ping');
  assert.equal(await watchers.cosmoclash.next(value => value === 'pong'), 'pong');
  for (const peer of [cosmo, plain, joining, original, hd, ...Object.values(watchers)]) peer.ws.close();
});

test('private rooms are never listed, and invites to listed rooms keep working', async () => {
  const { host, guest, lobby } = await defaultWorker();
  const watcher = lobby();
  await watcher.rooms();
  const hidden = roomId();
  const privateHost = host(hidden);
  await privateHost.next();
  const named = roomId();
  const namedHost = host(named, { name: 'Open' });
  await namedHost.next();
  await watcher.rooms(rooms => ids(rooms).includes(named));
  const privateGuest = guest(hidden);
  assert.deepEqual(await privateGuest.next(), { type: 'waiting' });
  assert.deepEqual(await privateHost.next(), { type: 'ready', role: 'host' });
  await pause(700);
  for (const snapshot of watcher.all) assert.ok(!ids(snapshot.rooms).includes(hidden));
  namedHost.ws.close();
});

test('a listed room leaves the list when a guest joins, when its host leaves, and when it fails', async () => {
  const { host, guest, lobby } = await defaultWorker();
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
  pairedHost.ws.send(Buffer.from([7, 7]));
  assert.deepEqual(await joining.next(Buffer.isBuffer), Buffer.from([7, 7]));
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

test('room names are validated, and name and engine are host-only parameters', async () => {
  const { socket, host, lobby } = await defaultWorker();
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
    [`/netplay/${id}?role=host&name=a&engine=c&engine=c`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&lobby=1`, /Invalid room URL/],
    [`/netplay/${id}?role=host&name=a&`, /Invalid room URL/],
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
  assert.equal((await watcher.rooms(rooms => ids(rooms).includes(id))).find(room => room.id === id).name, 'א'.repeat(24));
  accepted.ws.close();
  for (const [raw, name] of [['Red+Blue', 'Red Blue'], ['1%2B1', '1+1'], ['%F0%9F%9A%80%20Rocket', '🚀 Rocket'],
    ['Caf%65%CC%81%20%20Bar', 'Café Bar']]) {
    const named = socket(`/netplay/${roomId()}?role=host&name=${raw}`, address());
    assert.deepEqual(await named.next(), { type: 'waiting' }, raw);
    await watcher.rooms(rooms => rooms.some(room => room.name === name));
    named.ws.close();
  }
});

test('lobby sockets check the origin, the URL and subprotocols; plain HTTP is refused', async () => {
  const { socket, lobby, origin } = await defaultWorker();
  for (const bad of ['https://evil.example', 'null', undefined, `${origin}/`]) {
    const denied = lobby('original', { origin: bad });
    await assert.rejects(denied.opened, /403/, String(bad));
  }
  for (const url of ['/netplay/lobby', '/netplay/lobby?edition=3do', '/netplay/lobby?edition=HD',
    '/netplay/lobby?edition=hd&x=1', '/netplay/lobby?edition=original&edition=hd', '/netplay/lobby?',
    '/netplay/lobby/?edition=hd', '/netplay/lobby/hd', '/netplay/hd/lobby?edition=hd',
    '/netplay/lobby?edition=cosmo', '/netplay/lobby?edition=CosmoClash', '/netplay/cosmoclash/lobby?edition=cosmoclash']) {
    const invalid = socket(url);
    assert.equal((await invalid.next()).type, 'error', url);
    assert.equal((await invalid.closure).code, 1008, url);
  }
  const subprotocol = socket('/netplay/lobby?edition=hd', {}, 'binary');
  await assert.rejects(subprotocol.opened, /400/);
  const plain = await fetch(`${origin}/netplay/lobby?edition=hd`, { headers: { Origin: origin } });
  assert.equal(plain.status, 426);
});

test('lobby sockets accept only keepalive pings', async () => {
  const { lobby } = await defaultWorker();
  const watcher = lobby();
  await watcher.rooms();
  watcher.ws.send('ping');
  assert.equal(await watcher.next(value => value === 'pong'), 'pong');
  for (const payload of ['hello', Buffer.from([1])]) {
    const talker = lobby();
    await talker.rooms();
    talker.ws.send(payload);
    assert.match((await talker.next(value => value?.type === 'error')).message, /accepts no messages/);
    assert.equal((await talker.closure).code, 1008);
  }
  assert.equal(watcher.closed, null);
});

test('a waiting host can keep its room alive with ping texts, and the room still pairs', async () => {
  const { host, guest } = await defaultWorker();
  for (const listing of [{ name: 'Keepalive' }, undefined]) {
    const id = roomId();
    const waiting = host(id, listing);
    await waiting.next();
    waiting.ws.send('ping');
    assert.equal(await waiting.next(value => value === 'pong'), 'pong');
    const joining = guest(id);
    await joining.next(value => value.type === 'ready');
    await waiting.next(value => value.type === 'ready');
    waiting.ws.send(Buffer.from('game'));
    assert.deepEqual(await joining.next(Buffer.isBuffer), Buffer.from('game'));
  }
});

test('snapshots are throttled to about two per second and always end with the latest list', async () => {
  const { host, lobby } = await defaultWorker();
  const watcher = lobby('hd');
  await watcher.rooms();
  await pause(600);
  const start = watcher.all.length;
  const created = Array.from({ length: 8 }, () => roomId());
  const hosts = created.map((id, n) => host(id, { edition: 'hd', name: 'Room ' + n }));
  await Promise.all(hosts.map(peer => peer.next()));
  const latest = await watcher.rooms(rooms => created.every(id => ids(rooms).includes(id)));
  for (let n = 1; n < latest.length; n++) assert.ok(latest[n - 1].created >= latest[n].created, 'newest first');
  await pause(700);
  const times = watcher.times.slice(start);
  assert.ok(times.length < created.length, `${times.length} snapshots for ${created.length} changes`);
  for (let n = 1; n < times.length; n++) assert.ok(times[n] - times[n - 1] >= 400, `snapshot gap ${times[n] - times[n - 1]} ms`);
  assert.ok(created.every(id => ids(watcher.all.at(-1).rooms).includes(id)));
  for (const peer of hosts) peer.ws.close();
});

test('listed rooms wait longer than private rooms, then expire and leave the list', async t => {
  const { host, lobby } = helpers(await startWorker(t, { NETPLAY_WAITING_MS: '1500', NETPLAY_LISTED_WAITING_MS: '5000' }));
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
  assert.match((await namedHost.next(value => value.type === 'error', 10000)).message, /expired/);
  await namedHost.closure;
  await watcher.rooms(rooms => !ids(rooms).includes(named));
});

test('the room list and lobby sockets are bounded per address and in total; lobby sockets use no room leases', async t => {
  const { host, guest, lobby } = helpers(await startWorker(t, { NETPLAY_MAX_LISTED: '2', NETPLAY_MAX_LISTED_PER_IP: '1',
    NETPLAY_MAX_LOBBY: '4', NETPLAY_MAX_LOBBY_PER_IP: '2', NETPLAY_MAX_PER_IP: '2' }));
  const [ipA, ipB, ipC, ipD] = [address(), address(), address(), address()];
  // Lobby sockets: 2 per address, 4 per edition.
  const a1 = lobby('original', ipA);
  const a2 = lobby('original', ipA);
  await Promise.all([a1.rooms(), a2.rooms()]);
  await assert.rejects(lobby('original', ipA).opened, /503/);
  const hdA = lobby('hd', ipA);
  await hdA.rooms();
  const b1 = lobby('original', ipB);
  const b2 = lobby('original', ipB);
  await Promise.all([b1.rooms(), b2.rooms()]);
  await assert.rejects(lobby('original', ipC).opened, /503/);
  b2.ws.close();
  await b2.closure;
  await pause(300);
  const c1 = lobby('original', ipC);
  await c1.rooms();
  // Listed rooms: 1 per address, 2 per edition.
  const mineId = roomId();
  const mine = host(mineId, { name: 'Mine' }, ipA);
  assert.deepEqual(await mine.next(), { type: 'waiting' });
  const second = host(roomId(), { name: 'Another' }, ipA);
  assert.match((await second.next()).message, /most open rooms/);
  // The runtime completes this close handshake slowly; the error message comes first.
  const closures = [second.closure];
  await pause(300);
  // Address A holds three lobby sockets and still gets its two room connections.
  const privateA = host(roomId(), undefined, ipA);
  assert.deepEqual(await privateA.next(), { type: 'waiting' });
  const theirs = host(roomId(), { name: 'Theirs' }, ipB);
  assert.deepEqual(await theirs.next(), { type: 'waiting' });
  const refusedId = roomId();
  const refused = host(refusedId, { name: 'Third' }, ipC);
  assert.match((await refused.next()).message, /room list is full/);
  closures.push(refused.closure);
  // Private rooms are not part of the list bound.
  const privateHost = host(roomId(), undefined, ipC);
  assert.deepEqual(await privateHost.next(), { type: 'waiting' });
  // Once the first room pairs it leaves the list, and the refused token can still be used.
  const opponent = guest(mineId, undefined, ipD);
  await opponent.next(value => value.type === 'ready');
  await c1.rooms(rooms => !ids(rooms).includes(mineId));
  const retry = host(refusedId, { name: 'Third' }, ipD);
  assert.deepEqual(await retry.next(), { type: 'waiting' });
  assert.equal((await c1.rooms(rooms => ids(rooms).includes(refusedId))).length, 2);
  for (const closure of closures) assert.equal((await closure).code, 1008);
  // The Cosmo Clash list is another lobby object with its own bounds: the full Original list does not refuse it.
  const ipE = address();
  const cosmoWatcher = lobby('cosmoclash', ipE);
  assert.deepEqual(await cosmoWatcher.rooms(), []);
  const cosmoId = roomId();
  const cosmo = host(cosmoId, { edition: 'cosmoclash', name: 'Cosmo', engine: 'rust' }, ipE);
  assert.deepEqual(await cosmo.next(), { type: 'waiting' });
  assert.deepEqual(ids(await cosmoWatcher.rooms(rooms => rooms.length > 0)), [cosmoId]);
  const cosmoSecond = host(roomId(), { edition: 'cosmoclash', name: 'Cosmo 2', engine: 'rust' }, ipE);
  assert.match((await cosmoSecond.next()).message, /most open rooms/, 'one listed room per address in this list too');
});

const SILENT_HOST = 'The room closed because this page stopped answering. Create a new room.';
const TOO_MANY_JOINS = 'You joined many open rooms in a short time. Wait a few minutes, then try again.';
const ip = value => ({ headers: { 'CF-Connecting-IP': value } });

test('the liveness check closes a listed room whose host stops sending pings; a host that pings stays', async t => {
  const { host, guest, lobby } = helpers(await startWorker(t, { NETPLAY_LISTED_SILENCE_MS: '2500', NETPLAY_LOBBY_CHECK_MS: '500' }));
  const watcher = lobby();
  await watcher.rooms();
  // A sleeping computer or a frozen page: the socket stays open and nothing arrives.
  const frozenId = roomId();
  const frozen = host(frozenId, { name: 'Frozen' });
  await frozen.next();
  frozen.ws.pause();
  const pingingId = roomId();
  const pinging = host(pingingId, { name: 'Awake' });
  await pinging.next();
  const pings = setInterval(() => pinging.ws.send('ping'), 300);
  t.after(() => clearInterval(pings));
  await watcher.rooms(rooms => ids(rooms).includes(frozenId) && ids(rooms).includes(pingingId));
  const left = await watcher.rooms(rooms => !ids(rooms).includes(frozenId), 15000);
  assert.ok(ids(left).includes(pingingId), 'a host that pings stays listed');
  frozen.ws.resume();
  assert.equal((await frozen.next(value => value?.type === 'error', 15000)).message, SILENT_HOST);
  await pause(3000);
  assert.equal(pinging.closed, null);
  assert.ok(ids(watcher.all.at(-1).rooms).includes(pingingId), 'a host that pings stays listed past the limit');
  const joining = guest(pingingId);
  assert.deepEqual(await joining.next(value => value.type === 'ready'), { type: 'ready', role: 'guest' });
  // The frozen room's token is retired like any other ended room.
  assert.match((await guest(frozenId).next()).message, /does not exist or has expired/);
});

test('a guest who picks a silent host from the list is refused before pairing, and the room closes', async t => {
  // The liveness check is far away: only the guest's join finds the silent host.
  const { host, guest, lobby } = helpers(await startWorker(t, { NETPLAY_LISTED_SILENCE_MS: '1000', NETPLAY_LOBBY_CHECK_MS: '600000' }));
  const watcher = lobby();
  await watcher.rooms();
  const id = roomId();
  const silent = host(id, { name: 'Asleep' });
  await silent.next();
  await watcher.rooms(rooms => ids(rooms).includes(id));
  await pause(1500);
  assert.ok(ids(watcher.all.at(-1).rooms).includes(id), 'still listed before anyone looks');
  const joining = guest(id);
  const reply = await joining.next();
  assert.equal(reply.type, 'error');
  assert.match(reply.message, /does not exist or has expired/);
  assert.ok(!joining.all.some(value => value?.type === 'ready'), 'the guest never pairs');
  assert.equal((await silent.next(value => value?.type === 'error')).message, SILENT_HOST);
  await watcher.rooms(rooms => !ids(rooms).includes(id));
});

test('per-address bounds count an IPv6 client by its /64 network, and listed-room joins per address are limited', async t => {
  const { host, guest, lobby } = helpers(await startWorker(t, { NETPLAY_MAX_LISTED_PER_IP: '2', NETPLAY_MAX_LOBBY_PER_IP: '2',
    NETPLAY_MAX_LISTED_JOINS: '2', NETPLAY_LISTED_JOIN_WINDOW_MS: '4000' }));
  // Lobby sockets: 2 per /64 in each edition; an IPv4-mapped address is the same IPv4 address.
  const sockets = [lobby('original', ip('2001:db8:9:9::1')), lobby('original', ip('2001:db8:9:9:ffff::2'))];
  await Promise.all(sockets.map(socket => socket.rooms()));
  await assert.rejects(lobby('original', ip('2001:db8:9:9:abcd::3')).opened, /503/);
  const other64 = lobby('original', ip('2001:db8:9:a::1'));
  const watcher = other64;
  await other64.rooms();
  const v4 = [lobby('original', ip('198.51.100.250')), lobby('original', ip('::ffff:198.51.100.250'))];
  await Promise.all(v4.map(socket => socket.rooms()));
  await assert.rejects(lobby('original', ip('198.51.100.250')).opened, /503/);

  // Listed rooms: 2 per /64.
  const mine = [roomId(), roomId()];
  const mineHosts = [host(mine[0], { name: 'Six A' }, ip('2001:db8:1:2::1')), host(mine[1], { name: 'Six B' }, ip('2001:db8:1:2:ffff::2'))];
  for (const peer of mineHosts) assert.deepEqual(await peer.next(), { type: 'waiting' });
  const third = host(roomId(), { name: 'Six C' }, ip('2001:db8:1:2:abcd::3'));
  assert.match((await third.next()).message, /most open rooms/);
  const nextNetwork = host(roomId(), { name: 'Next /64' }, ip('2001:db8:1:3::1'));
  assert.deepEqual(await nextNetwork.next(), { type: 'waiting' });

  // Joins of listed rooms: 2 per /64 in the window. Private rooms are not counted.
  const open = [roomId(), roomId(), roomId()];
  const openHosts = open.map((id, n) => host(id, { name: 'Open ' + n }));
  for (const peer of openHosts) await peer.next();
  await watcher.rooms(rooms => open.every(id => ids(rooms).includes(id)));
  for (const [n, address] of ['2001:db8:5:5::1', '2001:db8:5:5:1::2'].entries()) {
    assert.deepEqual(await guest(open[n], undefined, ip(address)).next(value => value.type === 'ready'), { type: 'ready', role: 'guest' });
  }
  const refused = guest(open[2], undefined, ip('2001:db8:5:5:2::3'));
  assert.equal((await refused.next()).message, TOO_MANY_JOINS);
  assert.equal((await refused.closure).code, 1008);
  await pause(700);
  assert.equal(openHosts[2].closed, null, 'a refused join does not end the room');
  assert.ok(ids(watcher.all.at(-1).rooms).includes(open[2]));
  const privateId = roomId();
  const privateHost = host(privateId);
  await privateHost.next();
  assert.deepEqual(await guest(privateId, undefined, ip('2001:db8:5:5::1')).next(value => value.type === 'ready'),
    { type: 'ready', role: 'guest' });
  // After the window the network may join again.
  await pause(4000);
  assert.deepEqual(await guest(open[2], undefined, ip('2001:db8:5:5::1')).next(value => value.type === 'ready'),
    { type: 'ready', role: 'guest' });
});
