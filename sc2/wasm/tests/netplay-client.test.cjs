const test = require('node:test');
const assert = require('node:assert/strict');
const netplay = require('../netplay-client.js');
const token = '0123456789abcdef0123456789abcdef';
const location = { href: 'https://game.example/uqm/index.html?ignored=1#room=old' };

function harness(role = 'host') {
    const statuses = [];
    const timers = new Map();
    let socket;
    class FakeSocket {
        constructor(url) { this.url = url; this.readyState = 0; this.bufferedAmount = 0; this.sent = []; this.closed = []; socket = this; }
        send(data) { this.sent.push(Uint8Array.from(data)); }
        close(...args) { this.closed.push(args); this.readyState = 3; if (this.onclose) this.onclose(); }
        open() { this.readyState = 1; this.onopen(); }
        control(data) { this.onmessage({ data: JSON.stringify(data) }); }
        binary(bytes) { this.onmessage({ data: Uint8Array.from(bytes).buffer }); }
    }
    const client = netplay.connect({ token, role, location, WebSocket: FakeSocket,
        setTimeout(fn) { timers.set(1, fn); return 1; }, clearTimeout(id) { timers.delete(id); },
        onStatus(status) { statuses.push(status); } });
    return { client, socket, statuses, timers, ready() { socket.open(); socket.control({ type: 'ready', role }); } };
}

test('invites use a 128-bit random token and a fragment, with no query', () => {
    const generated = netplay.createToken({ getRandomValues(bytes) { assert.equal(bytes.length, 16); bytes.fill(171); } });
    assert.equal(generated, 'ab'.repeat(16));
    assert.equal(netplay.inviteURL(token, location), 'https://game.example/uqm/index.html#room=' + token);
    assert.equal(netplay.parseRoom('#room=' + token, location), token);
    assert.equal(netplay.parseRoom(token.toUpperCase(), location), token);
    assert.equal(netplay.parseRoom(netplay.inviteURL(token, location), location), token);
});

test('only same-origin relay URLs are built, ignoring arbitrary query servers', () => {
    assert.equal(netplay.relayURL(token, 'host', { href: 'https://game.example/x?server=wss://evil.example/#other' }), 'wss://game.example/netplay/' + token + '?role=host');
    assert.equal(netplay.relayURL(token, 'guest', { href: 'http://localhost:10000/' }), 'ws://localhost:10000/netplay/' + token + '?role=guest');
    assert.throws(() => netplay.parseRoom('https://other.example/#room=' + token, location), /this game server/);
    assert.throws(() => netplay.parseRoom('?room=' + token, location), /valid room/);
    assert.throws(() => netplay.relayURL(token, 'admin', location), /role/);
    assert.throws(() => netplay.relayURL('../escape', 'host', location), /valid room/);
    assert.throws(() => netplay.relayURL(token, 'host', { href: 'file:///tmp/game.html' }), /preview server/);
});

test('room lifecycle does not send before readiness and queues binary for engine polling', () => {
    const h = harness();
    assert.equal(h.client.role, 0);
    assert.equal(h.client.state, 'waiting');
    assert.equal(h.socket.binaryType, 'arraybuffer');
    assert.equal(h.client.send(new Uint8Array([1])), false);
    h.socket.open();
    assert.equal(h.timers.size, 0);
    h.socket.control({ type: 'waiting' });
    h.socket.control({ type: 'ready', role: 'host' });
    h.socket.binary([1, 2, 3]);
    h.socket.binary([4]);
    assert.equal(h.client.state, 'ready');
    assert.equal(h.client.incomingBytes, 4);
    assert.deepEqual(h.client.incoming.map(bytes => Array.from(bytes)), [[1, 2, 3], [4]]);
    // This is the engine's polling contract; callbacks have not consumed data.
    const packet = h.client.incoming.shift();
    h.client.incomingBytes -= packet.byteLength;
    assert.equal(h.client.incomingBytes, 1);
    assert.equal(h.client.send(new Uint8Array([8, 9])), true);
    assert.equal(h.client.stats.sentBytes, 2);
    assert.equal(h.client.stats.receivedBytes, 4);
    assert.equal(h.client.stats.peakIncomingBytes, 4);
});

test('guest role is fixed, a peer departure closes and clears pending data', () => {
    const h = harness('guest');
    h.ready();
    assert.equal(h.client.role, 1);
    h.socket.binary([1]);
    h.socket.control({ type: 'peer-left' });
    assert.equal(h.client.state, 'closed');
    assert.equal(h.client.incoming.length, 0);
    assert.equal(h.client.incomingBytes, 0);
    assert.equal(h.client.send(new Uint8Array([1])), false);
    assert.match(h.statuses.at(-1).message, /opponent left/);
    h.socket.control({ type: 'ready', role: 'guest' });
    assert.equal(h.client.state, 'closed');
});

test('errors remain errors after socket close and explicit leave is idempotent', () => {
    const h = harness();
    h.socket.onerror();
    assert.equal(h.client.state, 'error');
    assert.match(h.statuses.at(-1).message, /online preview/);
    h.client.close();
    assert.equal(h.client.state, 'error');
    assert.equal(h.socket.closed.length, 1);
    const other = harness();
    other.ready();
    other.client.close();
    other.client.close();
    assert.equal(other.client.state, 'closed');
    assert.equal(other.socket.closed.length, 1);
});

test('connection timeout and protocol errors disconnect explicitly', () => {
    const timedOut = harness();
    timedOut.timers.get(1)();
    assert.equal(timedOut.client.state, 'error');
    const malformed = harness();
    malformed.socket.onmessage({ data: 'not json' });
    assert.equal(malformed.client.state, 'error');
    const wrongRole = harness();
    wrongRole.socket.control({ type: 'ready', role: 'guest' });
    assert.equal(wrongRole.client.role, 0);
    assert.equal(wrongRole.client.state, 'error');
    const denied = harness();
    denied.socket.control({ type: 'error', message: 'Room is full.' });
    assert.match(denied.statuses.at(-1).message, /Room is full/);
});

test('incoming frame and total queue limits stop the session instead of dropping bytes', () => {
    const oversized = harness();
    oversized.ready();
    oversized.socket.binary(new Uint8Array(netplay.limits.frameBytes + 1));
    assert.equal(oversized.client.state, 'error');
    const full = harness();
    full.ready();
    for (let i = 0; i < netplay.limits.queueBytes / netplay.limits.frameBytes; i++) full.socket.binary(new Uint8Array(netplay.limits.frameBytes));
    assert.equal(full.client.incomingBytes, netplay.limits.queueBytes);
    full.socket.binary([1]);
    assert.equal(full.client.state, 'error');
    assert.equal(full.client.incomingBytes, 0);
});

test('outgoing backpressure and send exceptions become explicit errors', () => {
    const full = harness();
    full.ready();
    full.socket.bufferedAmount = netplay.limits.queueBytes;
    assert.equal(full.client.send(new Uint8Array([1])), false);
    assert.equal(full.client.state, 'error');
    const broken = harness();
    broken.ready();
    broken.socket.send = () => { throw new Error('Connection lost'); };
    assert.equal(broken.client.send(new Uint8Array([1])), false);
    assert.equal(broken.client.state, 'error');
});

test('control text and frame-count limits are bounded as well as bytes', () => {
    const oversized = harness();
    oversized.socket.onmessage({ data: ' '.repeat(2049) });
    assert.equal(oversized.client.state, 'error');
    const frames = harness();
    frames.ready();
    for (let i = 0; i < netplay.limits.queueFrames; i++) frames.socket.binary([1]);
    assert.equal(frames.client.incoming.length, netplay.limits.queueFrames);
    frames.socket.binary([1]);
    assert.equal(frames.client.state, 'error');
});

// The page's real inline script runs against a minimal DOM and the real client.
// This tests launch gating without loading the game or requiring a browser.
function pageHarness(hash = '') {
    const fs = require('node:fs');
    const path = require('node:path');
    const vm = require('node:vm');
    const html = fs.readFileSync(path.join(__dirname, '..', 'shell.html'), 'utf8');
    const elements = new Map();
    const document = { activeElement: null, fullscreenElement: null,
        getElementById(id) {
            if (id === 'choose-edition' && !elements.has(id)) return null;
            assert.ok(elements.has(id), 'page element exists: ' + id); return elements.get(id);
        },
        addEventListener() {} };
    for (const match of html.matchAll(/<[^>]+\bid="([^"]+)"[^>]*>/g)) {
        const handlers = {};
        const element = { hidden: /\bhidden\b/.test(match[0]), disabled: /\bdisabled\b/.test(match[0]),
            value: '', textContent: '', className: '',
            addEventListener(type, fn) { handlers[type] = fn; },
            click() { if (!this.disabled && handlers.click) handlers.click(); },
            focus() { document.activeElement = this; }, select() {} };
        elements.set(match[1], element);
    }
    const sockets = [];
    const calls = [];
    class PageSocket {
        constructor(url) { this.url = url; this.readyState = 0; this.bufferedAmount = 0; sockets.push(this); }
        close() { this.readyState = 3; if (this.onclose) this.onclose(); }
        send() {}
        ready(role) { this.readyState = 1; this.onopen(); this.onmessage({ data: JSON.stringify({ type: 'ready', role }) }); }
    }
    const context = { document, console, URL, URLSearchParams, SharedArrayBuffer,
        location: { href: 'http://localhost:10000/index.html' + hash, hash },
        crypto: { getRandomValues(bytes) { bytes.fill(42); } },
        navigator: {}, WebSocket: PageSocket, crossOriginIsolated: true,
        setTimeout() { return 1; }, clearTimeout() {}, setInterval() { return 2; }, clearInterval() {},
        addEventListener() {} };
    context.window = context;
    vm.createContext(context);
    vm.runInContext(fs.readFileSync(path.join(__dirname, '..', 'netplay-client.js'), 'utf8'), context);
    const inline = html.match(/<script>\s*([\s\S]*?)<\/script>/)[1];
    vm.runInContext(inline, context);
    context.Module.detect3DOVideos = () => ({ complete: false, missing: ['intro.duk'], total: 1 });
    context.Module.callMain = args => calls.push(Array.from(args));
    return { context, elements, sockets, calls,
        runtimeReady() { context.Module.onRuntimeInitialized(); },
        click(id) { elements.get(id).click(); } };
}

test('page never auto-connects an invite or auto-launches on peer readiness', () => {
    const h = pageHarness('#room=' + token);
    assert.equal(h.elements.get('join-invite').value, token);
    assert.equal(h.sockets.length, 0, 'opening an invite requires a Join click');
    h.click('join-room');
    assert.equal(h.sockets.length, 1);
    assert.match(h.sockets[0].url, /role=guest$/);
    h.sockets[0].ready('guest');
    assert.equal(h.calls.length, 0);
    assert.equal(h.elements.get('launch-online').disabled, true, 'runtime must also finish loading');
    h.runtimeReady();
    assert.equal(h.elements.get('launch-online').disabled, false);
    assert.equal(h.calls.length, 0, 'connection and runtime ready still need a click');
    h.click('launch-online');
    assert.equal(h.calls.length, 1);
    assert.ok(h.calls[0].includes('--web-netplay=guest'));
    assert.match(h.elements.get('online-role').textContent, /top/);
    assert.equal(h.elements.get('online-entry').hidden, true);
    assert.equal(h.elements.get('launch-online').hidden, true);
    assert.equal(h.elements.get('leave-room').hidden, false);
});

test('page preserves local Launch game and closes any pending online room', () => {
    const h = pageHarness();
    h.runtimeReady();
    assert.equal(h.elements.get('start').textContent, 'Launch game');
    h.click('create-room');
    assert.match(h.elements.get('room-invite').value, /#room=[a-f0-9]{32}$/);
    h.click('start');
    assert.equal(h.calls.length, 1);
    assert.ok(!h.calls[0].some(arg => arg.startsWith('--web-netplay')));
    assert.equal(h.context.Module.webNet.state, 'closed');
    assert.equal(h.elements.get('overlay').hidden, true);
});

test('page shows assigned host fleet and engine verification diagnostics', () => {
    const h = pageHarness();
    h.runtimeReady();
    h.click('create-room');
    h.sockets[0].ready('host');
    h.click('launch-online');
    assert.ok(h.calls[0].includes('--web-netplay=host'));
    assert.match(h.elements.get('online-role').textContent, /bottom/);
    Object.assign(h.context.Module.webNet.stats, { engineState: 'in-battle', remotePlayer: 1,
        checksumsVerified: 22, lastVerifiedFrame: 120, checksumMismatches: 0 });
    h.context.renderNetStats();
    assert.match(h.elements.get('net-stats').textContent, /Checksums verified: 22/);
    assert.match(h.elements.get('net-stats').textContent, /Last verified frame: 120/);
    h.click('leave-room');
    assert.equal(h.context.Module.webNet.state, 'closed');
    assert.match(h.elements.get('online-status').textContent, /Reload this page/);
});
