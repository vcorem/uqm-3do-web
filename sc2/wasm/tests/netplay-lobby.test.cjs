// Room list (lobby) and named-room parts of the browser client, with a fake
// WebSocket and fake timers. The relay side is tested in tests/server and tests/worker.
const test = require('node:test');
const assert = require('node:assert/strict');
const netplay = require('../netplay-client.js');
const names = require('./room-names.cjs');

const token = '0123456789abcdef0123456789abcdef';
const other = 'fedcba9876543210fedcba9876543210';
const location = { href: 'https://game.example/hd/index.html?x=1#room=old' };

function fakes() {
    const sockets = [];
    const timers = new Map();
    let nextTimer = 1;
    class FakeSocket {
        constructor(url) {
            this.url = url; this.readyState = 0; this.bufferedAmount = 0; this.sent = []; this.closed = [];
            sockets.push(this);
        }
        send(data) { this.sent.push(data); }
        close(...args) { this.closed.push(args); this.readyState = 3; if (this.onclose) this.onclose(); }
        open() { this.readyState = 1; if (this.onopen) this.onopen(); }
        text(data) { this.onmessage({ data: typeof data === 'string' ? data : JSON.stringify(data) }); }
        rooms(rooms) { this.text({ type: 'rooms', rooms }); }
        drop() { this.readyState = 3; this.onclose(); }
    }
    return {
        sockets, timers, WebSocket: FakeSocket,
        setTimeout(fn, ms) { const id = nextTimer++; timers.set(id, { fn, ms }); return id; },
        clearTimeout(id) { timers.delete(id); },
        /** Runs the one pending timer with this delay. */
        fire(ms) {
            const due = [...timers].filter(([, timer]) => timer.ms === ms);
            assert.equal(due.length, 1, `one ${ms} ms timer is pending: ${JSON.stringify([...timers.values()].map(t => t.ms))}`);
            timers.delete(due[0][0]);
            due[0][1].fn();
        },
        delays() { return [...timers.values()].map(timer => timer.ms).sort((a, b) => a - b); },
    };
}

function watch(options = {}) {
    const f = fakes();
    const statuses = [];
    const snapshots = [];
    const watcher = netplay.watchLobby({ edition: 'hd', location, WebSocket: f.WebSocket,
        setTimeout: f.setTimeout, clearTimeout: f.clearTimeout,
        onStatus(state, message) { statuses.push({ state, message }); },
        onRooms(rooms) { snapshots.push(rooms); }, ...options });
    return { f, watcher, statuses, snapshots, get socket() { return f.sockets.at(-1); } };
}

const room = (id, name, extra = {}) => ({ id, name, engine: 'c', created: 1760000000000, ...extra });

test('room names are cleaned and checked like the relay checks them', () => {
    for (const [input, listed] of names.accepted) assert.equal(netplay.roomName(input), listed, JSON.stringify(input));
    for (const [label, input] of [...names.refused, ['null', null], ['undefined', undefined]]) {
        assert.throws(() => netplay.roomName(input), /^Error: Room names need 1 to 24 visible characters\.$/, label);
    }
    assert.equal(netplay.limits.roomNameChars, 24);
    assert.equal(netplay.limits.listedRooms, 50);
});

test('lobby and named-room URLs stay on this server and add only host listing parameters', () => {
    assert.equal(netplay.lobbyURL('hd', location), 'wss://game.example/netplay/lobby?edition=hd');
    assert.equal(netplay.lobbyURL(undefined, { href: 'http://localhost:10000/original/' }), 'ws://localhost:10000/netplay/lobby?edition=original');
    assert.throws(() => netplay.lobbyURL('3do', location), /edition/);
    assert.throws(() => netplay.lobbyURL('hd', { href: 'file:///tmp/game.html' }), /preview server/);
    assert.equal(netplay.relayURL(token, 'host', location, 'hd', { name: ' Red & Blue ', engine: 'rust' }),
        'wss://game.example/netplay/hd/' + token + '?role=host&name=Red%20%26%20Blue&engine=rust');
    assert.equal(netplay.relayURL(token, 'host', location, undefined, { name: 'חדר' }),
        'wss://game.example/netplay/' + token + '?role=host&name=%D7%97%D7%93%D7%A8');
    assert.equal(netplay.relayURL(token, 'host', location, 'hd', { engine: 'c' }), 'wss://game.example/netplay/hd/' + token + '?role=host',
        'engine without a name keeps the room private');
    assert.equal(netplay.relayURL(token, 'guest', location, 'hd', {}), 'wss://game.example/netplay/hd/' + token + '?role=guest');
    assert.throws(() => netplay.relayURL(token, 'guest', location, 'hd', { name: 'x' }), /Only the host/);
    assert.throws(() => netplay.relayURL(token, 'host', location, 'hd', { name: 'x', engine: 'go' }), /engine/);
    assert.throws(() => netplay.relayURL(token, 'host', location, 'hd', { name: '' }), /1 to 24/);
});

test('a named host room shows its name, pings while it waits, and stops pinging when paired', () => {
    const f = fakes();
    const statuses = [];
    const client = netplay.connect({ token, role: 'host', edition: 'hd', name: '  Arena  ', engine: 'c', location,
        WebSocket: f.WebSocket, setTimeout: f.setTimeout, clearTimeout: f.clearTimeout,
        onStatus(status) { statuses.push(status); } });
    const socket = f.sockets[0];
    assert.equal(socket.url, 'wss://game.example/netplay/hd/' + token + '?role=host&name=Arena&engine=c');
    assert.equal(client.name, 'Arena');
    socket.open();
    assert.match(statuses.at(-1).message, /Opening room “Arena”/);
    socket.text({ type: 'waiting' });
    assert.match(statuses.at(-1).message, /“Arena” is open/, 'the relay listed the room');
    assert.deepEqual(f.delays(), [30000]);
    f.fire(30000);
    assert.deepEqual(socket.sent, ['ping']);
    socket.text('pong');
    assert.equal(client.state, 'waiting', 'pong is not a protocol error');
    assert.match(statuses.at(-1).message, /“Arena” is open/);
    f.fire(30000);
    assert.deepEqual(socket.sent, ['ping', 'ping']);
    socket.text({ type: 'ready', role: 'host' });
    assert.equal(client.state, 'ready');
    assert.deepEqual(f.delays(), [], 'no keepalive once paired: game data flows');
    assert.equal(client.send(new Uint8Array([1])), true);
    client.close();
    assert.equal(f.timers.size, 0);
});

test('a named host whose ping gets no answer reports the lost connection', () => {
    const f = fakes();
    const statuses = [];
    const client = netplay.connect({ token, role: 'host', name: 'Quiet', location,
        WebSocket: f.WebSocket, setTimeout: f.setTimeout, clearTimeout: f.clearTimeout,
        onStatus(status) { statuses.push(status); } });
    const socket = f.sockets[0];
    socket.open();
    socket.text({ type: 'waiting' });
    f.fire(30000);
    assert.deepEqual(socket.sent, ['ping']);
    // Any message from the relay counts as an answer.
    socket.text({ type: 'waiting' });
    f.fire(30000);
    assert.deepEqual(socket.sent, ['ping', 'ping']);
    assert.equal(client.state, 'waiting');
    f.fire(30000);
    assert.equal(client.state, 'error');
    assert.equal(statuses.at(-1).message, 'Lost the connection to online play. Create a new room.');
    assert.deepEqual(socket.sent, ['ping', 'ping'], 'no ping after the lost answer');
    assert.equal(socket.closed.length, 1);
    assert.equal(f.timers.size, 0);
});

test('relay errors stand alone for a host and explain the failed join for a guest', () => {
    for (const [role, expected] of [
        ['host', 'The room closed because this page stopped answering. Create a new room.'],
        ['guest', 'Could not join this room. The room closed because this page stopped answering. Create a new room.'],
    ]) {
        const f = fakes();
        const statuses = [];
        const client = netplay.connect({ token, role, name: role === 'host' ? 'Arena' : undefined, location,
            WebSocket: f.WebSocket, setTimeout: f.setTimeout, clearTimeout: f.clearTimeout,
            onStatus(status) { statuses.push(status); } });
        f.sockets[0].open();
        f.sockets[0].text({ type: 'error', message: 'The room closed because this page stopped answering. Create a new room.' });
        assert.equal(client.state, 'error');
        assert.equal(statuses.at(-1).message, expected, role);
    }
});

test('private rooms and guests never ping, so relays without room lists keep working', () => {
    for (const options of [{ role: 'host' }, { role: 'guest' }]) {
        const f = fakes();
        const client = netplay.connect({ token, location, WebSocket: f.WebSocket,
            setTimeout: f.setTimeout, clearTimeout: f.clearTimeout, ...options });
        f.sockets[0].open();
        assert.deepEqual(f.delays(), []);
        assert.equal(client.name, null);
        assert.equal(f.sockets[0].url.includes('name='), false);
    }
});

test('the room list goes live on its first snapshot and passes only checked rooms', () => {
    const h = watch();
    assert.equal(h.socket.url, 'wss://game.example/netplay/lobby?edition=hd');
    assert.equal(h.watcher.state, 'connecting');
    assert.deepEqual(h.statuses, [{ state: 'connecting', message: 'Connecting to the room list…' }]);
    h.socket.open();
    assert.equal(h.watcher.state, 'connecting', 'open means the list is live, after a snapshot');
    h.socket.rooms([
        room(token, ' Alpha '),
        room(other, 'Beta', { engine: 'rust', created: 'soon' }),
        room(token.toUpperCase(), 'Upper-case id'),
        room('../' + token.slice(3), 'Bad id'),
        room(other, 'x'.repeat(25)),
        room(other, 'bell\u0007'),
        room(other, 42),
        room(other, 'Odd engine', { engine: 'zig' }),
        null, 'text',
    ]);
    assert.equal(h.watcher.state, 'open');
    assert.equal(h.statuses.at(-1).state, 'open');
    assert.deepEqual(h.snapshots, [[
        { id: token, name: 'Alpha', engine: 'c', created: 1760000000000 },
        { id: other, name: 'Beta', engine: 'rust', created: 0 },
        { id: other, name: 'Odd engine', engine: null, created: 1760000000000 },
    ]]);
    assert.deepEqual(h.watcher.rooms, h.snapshots[0]);
    h.snapshots[0].length = 0;
    assert.equal(h.watcher.rooms.length, 3, 'callers get a copy');
    h.socket.rooms(Array.from({ length: 60 }, () => room(token, 'Many')));
    assert.equal(h.snapshots.at(-1).length, 50);
    h.socket.rooms([]);
    assert.deepEqual(h.snapshots.at(-1), []);
    assert.equal(h.statuses.filter(status => status.state === 'open').length, 1);
});

test('a lost room list reconnects with backoff and resets it after a snapshot', () => {
    const h = watch();
    const delays = [];
    for (let n = 0; n < 6; n++) {
        h.socket.drop();
        assert.equal(h.watcher.state, 'error');
        assert.match(h.statuses.at(-1).message, /room list/);
        const [delay] = h.f.delays();
        delays.push(delay);
        h.f.fire(delay);
        assert.equal(h.watcher.state, 'connecting');
        assert.match(h.statuses.at(-1).message, /Reconnecting/);
    }
    assert.deepEqual(delays, [1000, 2000, 4000, 8000, 15000, 15000]);
    assert.equal(h.f.sockets.length, 7);
    h.socket.open();
    h.socket.rooms([room(token, 'Back')]);
    assert.equal(h.watcher.state, 'open');
    h.socket.drop();
    assert.deepEqual(h.f.delays(), [1000]);
    // Old sockets lose their handlers, so their late events change nothing.
    assert.equal(h.f.sockets[0].onmessage, null);
    h.f.fire(1000);
    assert.equal(h.f.sockets.length, 8);
});

test('the room list pings every 30 seconds and reconnects when nothing answers', () => {
    const h = watch();
    h.socket.open();
    h.socket.rooms([]);
    assert.deepEqual(h.f.delays(), [30000]);
    h.f.fire(30000);
    assert.deepEqual(h.socket.sent, ['ping']);
    h.socket.text('pong');
    h.f.fire(30000);
    assert.deepEqual(h.socket.sent, ['ping', 'ping']);
    const silent = h.socket;
    h.f.fire(30000);
    assert.equal(h.watcher.state, 'error');
    assert.match(h.statuses.at(-1).message, /stopped answering/);
    assert.equal(silent.closed.length, 1);
    assert.deepEqual(h.f.delays(), [1000]);
    h.f.fire(1000);
    assert.notEqual(h.socket, silent);
});

test('connection timeouts, relay errors and unexpected data reconnect instead of breaking the page', () => {
    const timedOut = watch();
    timedOut.socket.open();
    timedOut.f.fire(12000);
    assert.equal(timedOut.watcher.state, 'error');
    assert.match(timedOut.statuses.at(-1).message, /Could not reach/);
    for (const data of [JSON.stringify({ type: 'error', message: 'Connection limit reached.' }), 'not json',
        ' '.repeat(64 * 1024 + 1), JSON.stringify({ type: 'surprise' }), JSON.stringify({ type: 'rooms', rooms: 'x' }),
        new ArrayBuffer(2)]) {
        const h = watch();
        h.socket.open();
        h.socket.onmessage({ data });
        assert.equal(h.watcher.state, 'error', String(data).slice(0, 40));
        assert.equal(h.socket.closed.length, 1);
        assert.deepEqual(h.f.delays(), [1000]);
    }
    const denied = watch();
    denied.socket.text({ type: 'error', message: 'Connection limit reached.' });
    assert.match(denied.statuses.at(-1).message, /^Connection limit reached\. Trying again/);
});

test('close() stops the list for good, and a page outside a server never connects', () => {
    const h = watch();
    h.socket.open();
    h.socket.rooms([room(token, 'A')]);
    const socket = h.socket;
    h.watcher.close();
    h.watcher.close();
    assert.equal(h.watcher.state, 'closed');
    assert.deepEqual(h.statuses.filter(status => status.state === 'closed').length, 1);
    assert.equal(socket.closed.length, 1);
    assert.equal(h.f.timers.size, 0);
    assert.equal(h.f.sockets.length, 1, 'no reconnect after close');

    const offline = watch({ location: { href: 'file:///tmp/game.html' } });
    assert.equal(offline.f.sockets.length, 0);
    assert.equal(offline.watcher.state, 'closed');
    assert.match(offline.statuses.at(-1).message, /preview server/);
    assert.throws(() => netplay.watchLobby({ edition: 'both', location }), /edition/);

    const errors = console.error;
    const logged = [];
    console.error = (...args) => logged.push(args);
    try {
        const throwing = watch({ onRooms() { throw new Error('page bug'); }, onStatus() { throw new Error('page bug'); } });
        throwing.socket.open();
        throwing.socket.rooms([room(token, 'A')]);
        assert.equal(throwing.watcher.state, 'open', 'page callback errors do not stop the list');
    } finally { console.error = errors; }
    assert.equal(logged.length, 3);

    const leaving = watch({ onStatus(state) { if (state === 'error') leaving.watcher.close(); } });
    leaving.socket.drop();
    assert.equal(leaving.watcher.state, 'closed');
    assert.equal(leaving.f.timers.size, 0, 'closing from a status callback cancels the retry');
});
