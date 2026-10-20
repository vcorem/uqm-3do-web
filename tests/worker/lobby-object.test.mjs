// The NetplayLobby Durable Object of deploy/netplay.js with a fake state and
// fake room objects. It covers what `wrangler dev` cannot show: the liveness
// check for rooms that ended without telling the lobby, the list bounds and the
// reload after hibernation. tests/worker/lobby.test.mjs covers the real runtime.
import test from 'node:test';
import assert from 'node:assert/strict';
import nodeModule from 'node:module';

// module.registerHooks needs Node.js 22.15 or newer; older versions skip this file.
const supported = typeof nodeModule.registerHooks === 'function';
if (supported) nodeModule.registerHooks({
  resolve(specifier, context, nextResolve) {
    if (specifier !== 'cloudflare:workers') return nextResolve(specifier, context);
    return { shortCircuit: true, url: 'data:text/javascript,' + encodeURIComponent(
      'export class DurableObject { constructor(ctx, env) { this.ctx = ctx; this.env = env; } }') };
  },
});
globalThis.WebSocketRequestResponsePair ??= class { constructor(request, response) { Object.assign(this, { request, response }); } };
const { NetplayLobby, parseRoomURL, addressKey } = supported ? await import('../../deploy/netplay.js') : {};
const node = await import('../../server/netplay.mjs');
const { default: names } = await import('../../sc2/wasm/tests/room-names.cjs');
const check = supported ? test : test.skip;

const pause = ms => new Promise(resolve => setTimeout(resolve, ms));
const token = n => n.toString(16).padStart(32, '0');

function state() {
  const storage = new Map();
  let alarm = null;
  const sockets = [];
  const ctx = {
    autoResponse: null,
    setWebSocketAutoResponse(pair) { ctx.autoResponse = pair; },
    blockConcurrencyWhile(fn) { ctx.ready = fn(); return ctx.ready; },
    getWebSockets(tag) { return sockets.filter(ws => !tag || ws.tags.includes(tag)); },
    storage: {
      async list({ prefix }) { return new Map([...storage].filter(([key]) => key.startsWith(prefix)).map(([k, v]) => [k, structuredClone(v)])); },
      async put(key, value) { storage.set(key, structuredClone(value)); },
      async delete(keys) { for (const key of [].concat(keys)) storage.delete(key); },
      async getAlarm() { return alarm; },
      async setAlarm(at) { alarm = at; },
    },
  };
  return { ctx, storage, sockets, get alarm() { return alarm; } };
}

/** A lobby with fake room objects; live[name] says what each room answers (or 'throw'). */
async function lobby(vars = {}, shared = state()) {
  const live = {};
  const asked = [];
  const env = { ...vars, NETPLAY_ROOMS: { idFromName: name => name, get: name => ({
    async isListed() {
      asked.push(name);
      if (live[name] === 'throw') throw new Error('Room object unavailable');
      return live[name] ?? true;
    } }) } };
  const object = new NetplayLobby(shared.ctx, env);
  await shared.ctx.ready;
  const watcher = { tags: ['ip:watcher'], sent: [], send(text) { this.sent.push(JSON.parse(text)); } };
  shared.sockets.push(watcher);
  return { object, live, asked, watcher, ...shared, shared };
}

const entry = (n, extra = {}) => ({ id: token(n), room: token(n), name: 'Room ' + n, engine: 'c',
  created: Date.now() + n, ip: '203.0.113.' + n, ...extra });

check('listed rooms are stored, snapshotted newest first without addresses, and removed', async () => {
  const l = await lobby();
  assert.equal(l.ctx.autoResponse.request, 'ping');
  assert.equal(l.ctx.autoResponse.response, 'pong');
  const one = entry(1);
  const two = entry(2, { engine: undefined, room: 'hd:' + token(2) });
  assert.equal(await l.object.add(one), 'ok');
  assert.equal(await l.object.add(two), 'ok');
  assert.equal(await l.object.add(one), 'ok', 'adding twice is harmless');
  assert.deepEqual(JSON.parse(l.object.snapshot()), { type: 'rooms', rooms: [
    { id: token(2), name: 'Room 2', engine: null, created: two.created },
    { id: token(1), name: 'Room 1', engine: 'c', created: one.created },
  ] });
  assert.equal(l.storage.size, 2);
  assert.ok(l.shared.alarm > Date.now(), 'a liveness check is scheduled');
  await l.object.remove(token(1));
  await l.object.remove(token(1));
  assert.deepEqual(JSON.parse(l.object.snapshot()).rooms.map(room => room.id), [token(2)]);
  assert.deepEqual([...l.storage.keys()], ['r:' + token(2)]);
});

check('the list bounds count all rooms and rooms per address, after dropping rooms that ended', async () => {
  const l = await lobby({ NETPLAY_MAX_LISTED: '2', NETPLAY_MAX_LISTED_PER_IP: '1', NETPLAY_LOBBY_SWEEP_MS: '0' });
  assert.equal(await l.object.add(entry(1, { ip: 'a' })), 'ok');
  assert.equal(await l.object.add(entry(2, { ip: 'a' })), 'ip');
  assert.equal(await l.object.add(entry(3, { ip: 'b' })), 'ok');
  assert.equal(await l.object.add(entry(4, { ip: 'c' })), 'full');
  assert.deepEqual(l.asked.sort(), [token(1), token(1), token(3)].sort(), 'a refusal asks the rooms first');
  // Room 1 ended without telling the lobby: the next refusal check finds it.
  l.live[token(1)] = false;
  assert.equal(await l.object.add(entry(4, { ip: 'c' })), 'ok');
  assert.deepEqual(JSON.parse(l.object.snapshot()).rooms.map(room => room.id).sort(), [token(3), token(4)].sort());
});

check('the liveness alarm drops rooms that stopped waiting or outlived the waiting limit, and keeps the rest', async () => {
  const l = await lobby({ NETPLAY_LISTED_WAITING_MS: '60000', NETPLAY_LOBBY_CHECK_MS: '1000' });
  await l.object.add(entry(1));
  await l.object.add(entry(2));
  await l.object.add(entry(3));
  await l.object.add(entry(4, { created: Date.now() - 62000 }));
  l.live[token(2)] = false;
  l.live[token(3)] = 'throw';
  await pause(1100);
  l.watcher.sent.length = 0;
  await l.object.alarm();
  assert.equal(l.watcher.sent.length, 1, 'one snapshot for the change');
  assert.ok(!l.asked.includes(token(4)), 'an expired entry is dropped without asking');
  assert.deepEqual(JSON.parse(l.object.snapshot()).rooms.map(room => room.id).sort(), [token(1), token(3)].sort(),
    'a room object that cannot answer keeps its entry until the next check');
  assert.deepEqual([...l.storage.keys()].sort(), ['r:' + token(1), 'r:' + token(3)].sort());
  assert.ok(l.shared.alarm > Date.now(), 'checks continue while rooms are listed');
  l.live[token(1)] = false;
  l.live[token(3)] = false;
  const before = l.shared.alarm;
  await pause(550);
  await l.object.alarm();
  assert.equal(JSON.parse(l.object.snapshot()).rooms.length, 0);
  assert.equal(l.shared.alarm, before, 'no new check is scheduled for an empty list');
});

check('snapshots to sockets are throttled and the last change always goes out', async () => {
  const l = await lobby({ NETPLAY_MAX_LISTED_PER_IP: '50' });
  for (let n = 1; n <= 6; n++) await l.object.add(entry(n));
  assert.equal(l.watcher.sent.length, 1, 'the first change goes out at once');
  await pause(600);
  assert.equal(l.watcher.sent.length, 2, 'later changes in the same half second go out together');
  assert.equal(l.watcher.sent[1].rooms.length, 6);
  await pause(500);
  await l.object.remove(token(6));
  assert.equal(l.watcher.sent.length, 3);
  assert.equal(l.watcher.sent[2].rooms.length, 5);
});

check('a lobby that wakes from hibernation reloads its list from storage', async () => {
  const shared = state();
  const first = await lobby({}, shared);
  await first.object.add(entry(1));
  await first.object.add(entry(2, { ip: 'other' }));
  const woken = await lobby({}, shared);
  assert.deepEqual(JSON.parse(woken.object.snapshot()).rooms.map(room => room.id), [token(2), token(1)]);
  assert.equal(await woken.object.add(entry(3)), 'ok');
});

check('repeated refusals of a full list ask the listed rooms at most once per sweep interval', async () => {
  const l = await lobby({ NETPLAY_MAX_LISTED: '2', NETPLAY_LOBBY_SWEEP_MS: '300' });
  assert.equal(await l.object.add(entry(1)), 'ok');
  assert.equal(await l.object.add(entry(2)), 'ok');
  for (let n = 0; n < 20; n++) assert.equal(await l.object.add(entry(100 + n)), 'full');
  assert.equal(l.asked.length, 2, 'one sweep of the 2 listed rooms for 20 refusals');
  await pause(350);
  l.live[token(1)] = false;
  assert.equal(await l.object.add(entry(200)), 'ok', 'the next sweep finds the room that ended');
  assert.equal(l.asked.length, 4);
  // The alarm's check also counts as a sweep.
  await pause(350);
  await l.object.alarm();
  const asked = l.asked.length;
  assert.equal(await l.object.add(entry(300)), 'full');
  assert.equal(l.asked.length, asked);
});

check('the Cloudflare and Node relays parse room names and address keys the same way', () => {
  const id = token(7);
  for (const [input, listed] of names.accepted) {
    const query = '?role=host&name=' + encodeURIComponent(input);
    assert.equal(parseRoomURL('/netplay/' + id, query).name, listed, JSON.stringify(input));
    assert.equal(node.parseRoomURL('/netplay/' + id + query).name, listed, JSON.stringify(input));
  }
  for (const [label, input] of names.refused) {
    let encoded;
    try { encoded = encodeURIComponent(input); } catch { encoded = '%ED%A0%80'; }
    const query = '?role=host&name=' + encoded;
    assert.equal(parseRoomURL('/netplay/' + id, query).error, 'Room names need 1 to 24 visible characters.', label);
    assert.equal(node.parseRoomURL('/netplay/' + id + query).error, 'Room names need 1 to 24 visible characters.', label);
  }
  for (const value of ['203.0.113.7', '::ffff:203.0.113.7', '2001:db8::1', '2001:db8:0:0:ffff::16', '2001:db8:0:1::1',
    'fe80::1%en0', '::', '::1', '2001:db8::1.2.3.4', '1::2::3', 'unknown', '', 'x'.repeat(100), 'A:B:C:D:E:F:0:1']) {
    assert.equal(addressKey(value), node.addressKey(value), value);
  }
  assert.equal(addressKey('2001:db8:0:0:ffff::16'), addressKey('2001:db8::1'));
  assert.notEqual(addressKey('2001:db8:0:1::1'), addressKey('2001:db8::1'));
});
