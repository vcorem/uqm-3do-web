const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const online = require('../../web/online/online-ui.js');

const TOKEN_A = 'a'.repeat(32), TOKEN_B = 'b'.repeat(32), TOKEN_C = 'c'.repeat(32);

function harness({ started = false, ready = false, screen, hook = false, lobby = true, netplay = true } = {}) {
    let clock = 1000, nextId = 1;
    const timers = [];
    const calls = [];
    const clients = [];
    const watches = [];
    const game = {
        isStarted: started, isReady: ready, current: screen, hasHook: hook, starts: [], queued: [],
        started() { return this.isStarted; }, ready() { return this.isReady; }, screen() { return this.current; },
        hook() { return this.hasHook; },
        start(role, client) { this.starts.push([role, client]); this.isStarted = true; },
        queue(role, client) { this.queued.push([role, client]); },
    };
    const fakeNetplay = {
        createToken: () => TOKEN_C,
        inviteURL: token => 'https://game.example/original/index.html#room=' + token,
        connect(options) {
            const client = { state: 'waiting', options, closed: false,
                close() { this.closed = true; this.state = 'closed'; options.onStatus({ state: 'closed', message: 'You left the room.' }); },
                status(state, message) { this.state = state; options.onStatus({ state, message, role: options.role }); } };
            clients.push(client);
            return client;
        },
    };
    if (lobby) fakeNetplay.watchLobby = options => {
        const watch = { options, closed: false, close() { this.closed = true; } };
        watches.push(watch);
        return watch;
    };
    const store = new Map();
    const env = {
        edition: 'original', engine: 'rust', now: () => clock, random: () => 0.5,
        setTimeout(fn, ms) { const id = nextId++; timers.push({ id, fn, at: clock + ms }); return id; },
        clearTimeout(id) { const timer = timers.find(t => t.id === id); if (timer) timer.cancelled = true; },
        storage: { get: key => store.get(key) ?? null, set: (key, value) => store.set(key, value) },
        netplay: netplay ? fakeNetplay : undefined, game,
        fullscreen: () => calls.push('fullscreen'), reload: online => calls.push('reload:' + online),
        copy: text => { calls.push('copy:' + text); return Promise.resolve(); },
        clearInvite: () => calls.push('clear-invite'), focusGame: () => calls.push('focus-game'),
        changed: () => {},
    };
    const ui = online.createController(env);
    return {
        ui, state: ui.state, game, calls, clients, watches, store,
        advance(ms) {
            const end = clock + ms;
            for (;;) {
                const due = timers.filter(t => !t.cancelled && !t.done && t.at <= end).sort((a, b) => a.at - b.at)[0];
                if (!due) break;
                clock = Math.max(clock, due.at);
                due.done = true;
                due.fn();
            }
            clock = end;
        },
        rooms(list) { watches.at(-1).options.onRooms(list); },
    };
}
const room = (id, name = 'Room', created = 1000, engine = 'c') => ({ id, name, engine, created });

test('helpers: room names, waiting times, invites and room-client messages', () => {
    assert.deepEqual(online.cleanName('  Zelnick  '), { ok: true, name: 'Zelnick' });
    assert.equal(online.cleanName('   ').ok, false);
    assert.equal(online.cleanName('x'.repeat(24)).ok, true);
    assert.equal(online.cleanName('x'.repeat(25)).ok, false);
    assert.equal(online.cleanName('a\u0007b').ok, false);
    assert.equal(online.cleanName('חדר של גיא').ok, true, 'any script is allowed');
    assert.equal(online.cleanName('🚀'.repeat(24)).ok, true, 'characters, not UTF-16 units, are counted');
    assert.equal(online.defaultName(() => 0.5), 'Captain 55');
    assert.equal(online.waitingTime(1000, 30000), 'new');
    assert.equal(online.waitingTime(0, 125000), 'new', 'no time from the server');
    assert.equal(online.waitingTime(1000, 1000 + 125000), '2 min');
    assert.equal(online.waitingTime(1000, 1000 + 3 * 3600000), '3 h');
    assert.equal(online.clock(65000), '1:05');
    assert.equal(online.readInvite('#room=' + 'A'.repeat(32)), 'a'.repeat(32));
    assert.equal(online.readInvite('#room=nope'), null);
    assert.equal(online.readInvite(''), null);
    assert.equal(online.cleanMessage('Your opponent left the room. Reload to start a new match.'), 'Your opponent left the room.');
    assert.equal(online.cleanMessage('Could not connect to online play on this server. Please try the online preview link.'), 'Could not connect to online play on this server.');
});

test('start screen: arrows move, Enter chooses, the first choice goes full screen', () => {
    const h = harness();
    assert.equal(h.state.screen, 'start');
    assert.equal(h.state.focus, 'play');
    assert.deepEqual(h.state.items, ['play', 'online']);
    h.ui.key('ArrowDown');
    assert.equal(h.state.focus, 'online');
    h.ui.key('ArrowDown');
    assert.equal(h.state.focus, 'play', 'the menu wraps');
    h.ui.key('ArrowUp');
    h.ui.key('Enter');
    assert.equal(h.state.screen, 'lobby');
    assert.deepEqual(h.calls, ['fullscreen']);
    assert.equal(h.watches.length, 1);
    assert.equal(h.watches[0].options.edition, 'original');
    h.ui.key('Escape');
    assert.equal(h.state.screen, 'start', 'Esc in the lobby returns to the start screen before the game runs');
    assert.equal(h.state.focus, 'online');
    assert.equal(h.watches[0].closed, true, 'the room list stops when the lobby is not shown');
    h.ui.key('Enter');
    assert.deepEqual(h.calls, ['fullscreen'], 'full screen is asked for once');
});

test('Play works while the game downloads and starts when it is ready', () => {
    const h = harness();
    h.ui.activate('play');
    assert.equal(h.state.pendingPlay, true);
    assert.equal(h.state.screen, 'start');
    assert.deepEqual(h.game.starts, []);
    h.ui.key('Escape');
    assert.equal(h.state.pendingPlay, false, 'Esc cancels the pending start');
    h.ui.activate('play');
    h.ui.progress({ label: 'Downloading game data: 10 / 299 MB (3%)', fraction: 0.03 });
    h.ui.ready();
    assert.deepEqual(h.game.starts, [[null, undefined]]);
    assert.equal(h.state.screen, 'hidden');
    assert.ok(h.calls.includes('focus-game'));
});

test('lobby: live rooms, own room hidden, focus follows the list', () => {
    const h = harness();
    h.ui.activate('online');
    assert.equal(h.state.lobby.status, 'connecting');
    assert.equal(h.state.focus, 'create');
    h.rooms([room(TOKEN_A, 'Alpha'), room('not-a-token', 'Bad'), room(TOKEN_B, 'Beta', 900, 'rust')]);
    assert.equal(h.state.lobby.status, 'open');
    assert.deepEqual(h.state.items, ['room:' + TOKEN_A, 'room:' + TOKEN_B, 'create', 'back']);
    assert.equal(h.state.focus, 'room:' + TOKEN_A, 'the newest room is offered before the player moves');
    h.ui.key('ArrowDown');
    assert.equal(h.state.focus, 'room:' + TOKEN_B);
    h.rooms([room(TOKEN_C, 'Gamma'), room(TOKEN_A, 'Alpha'), room(TOKEN_B, 'Beta')]);
    assert.equal(h.state.focus, 'room:' + TOKEN_B, 'a moved focus stays on its room');
    h.rooms([room(TOKEN_C, 'Gamma'), room(TOKEN_A, 'Alpha')]);
    assert.equal(h.state.focus, 'room:' + TOKEN_A, 'a closed room passes the focus to its neighbour');
    h.ui.key('ArrowRight');
    assert.equal(h.state.focus, 'create');
    h.ui.key('ArrowLeft');
    assert.equal(h.state.focus, 'room:' + TOKEN_A, 'Left returns to the last room');
    h.ui.key('End');
    assert.equal(h.state.focus, 'back');
    h.rooms(Array.from({ length: 60 }, (_, i) => room(i.toString(16).padStart(32, '0'), 'R' + i)));
    assert.equal(h.state.lobby.rooms.length, 50, 'at most 50 rooms');
});

test('create a room: remembered name, checks, host connection with name and engine', () => {
    const h = harness();
    h.ui.activate('online');
    h.ui.activate('create');
    assert.equal(h.state.screen, 'name');
    assert.equal(h.state.name, 'Captain 55');
    assert.equal(h.watches[0].closed, false, 'the list stays live behind the dialog');
    h.ui.setName('   ');
    h.ui.activate('open');
    assert.match(h.state.nameError, /Type a name/);
    assert.equal(h.clients.length, 0);
    h.ui.setName('x'.repeat(25));
    h.ui.key('Enter');
    assert.match(h.state.nameError, /24/);
    h.ui.setName("  Zelnick's room ");
    h.ui.activate('open');
    assert.equal(h.state.screen, 'waiting');
    assert.equal(h.watches[0].closed, true);
    const options = h.clients[0].options;
    assert.deepEqual([options.token, options.role, options.name, options.engine, options.edition],
        [TOKEN_C, 'host', "Zelnick's room", 'rust', 'original']);
    assert.equal(h.store.get(online.NAME_KEY), "Zelnick's room");
    h.ui.key('Escape');
    assert.equal(h.clients[0].closed, true, 'Esc closes the room');
    assert.equal(h.state.screen, 'lobby');
    assert.equal(h.state.notice.text, 'Your room is closed.');
    h.ui.activate('create');
    assert.equal(h.state.name, "Zelnick's room", 'the name is remembered');
    h.ui.key('Escape');
    assert.equal(h.state.screen, 'lobby');
});

test('opponent found: countdown, then the first session starts the engine with the room', () => {
    const h = harness({ ready: true });
    h.ui.activate('online');
    h.ui.activate('create');
    h.ui.activate('open');
    const host = h.clients[0];
    host.status('waiting', 'Waiting for your opponent to join…');
    assert.equal(h.state.screen, 'waiting');
    host.status('ready', 'Opponent connected. Each player can now launch online Melee.');
    assert.equal(h.state.screen, 'found');
    h.advance(2900);
    assert.equal(h.state.screen, 'found');
    assert.deepEqual(h.game.starts, []);
    h.advance(200);
    assert.deepEqual(h.game.starts, [['host', host]]);
    assert.equal(h.state.screen, 'hidden');
    assert.ok(h.state.session);
    // The match: the engine reports melee, the opponent leaves, the engine returns to its menu.
    h.ui.gameScreen('melee');
    host.status('closed', 'Your opponent left the room. Reload to start a new match.');
    assert.equal(h.state.screen, 'hidden', 'nothing covers the engine while it shows its own message');
    h.game.hasHook = true;
    h.game.current = 'mainMenu';
    h.ui.gameScreen('mainMenu');
    assert.equal(h.state.screen, 'lobby');
    assert.equal(h.state.notice.text, 'Your opponent left the room.');
    assert.equal(h.state.session, null);
    // The next session uses the main-menu hook.
    h.ui.activate('create');
    h.ui.activate('open');
    h.clients[1].status('ready', 'Opponent connected.');
    h.advance(3000);
    assert.deepEqual(h.game.queued, [[0, h.clients[1]]]);
    assert.equal(h.game.starts.length, 1);
    assert.equal(h.state.screen, 'hidden');
});

test('a guest joins from the list; the hook gives the guest role 1', () => {
    const h = harness({ ready: true, screen: 'mainMenu', hook: true });
    h.ui.fromMenu();
    assert.equal(h.state.screen, 'start', 'the menu button works only while the game is shown');
    h.ui.activate('play');
    assert.equal(h.state.screen, 'hidden');
    h.ui.fromMenu();
    assert.equal(h.state.screen, 'lobby');
    h.rooms([room(TOKEN_A, 'Alpha')]);
    h.ui.key('Enter');
    assert.equal(h.state.screen, 'joining');
    assert.deepEqual([h.clients[0].options.role, h.clients[0].options.token, h.clients[0].options.name], ['guest', TOKEN_A, undefined]);
    h.clients[0].status('ready', 'Opponent connected.');
    h.advance(3000);
    assert.deepEqual(h.game.queued, [[1, h.clients[0]]]);
    h.ui.key('Escape');
    assert.equal(h.state.screen, 'hidden', 'menu keys do nothing during a match');
    h.ui.gameScreen('melee');
    h.ui.gameScreen('mainMenu');
    assert.equal(h.state.screen, 'lobby');
    assert.equal(h.state.notice.text, 'The online match is over.');
    h.ui.key('Escape');
    assert.equal(h.state.screen, 'hidden', 'Esc in the lobby returns to the game');
});

test('found while the game still downloads: the match starts when it is ready', () => {
    const h = harness();
    h.ui.activate('online');
    h.rooms([room(TOKEN_A, 'Alpha')]);
    h.ui.activate('room:' + TOKEN_A);
    h.clients[0].status('ready', 'Opponent connected.');
    h.advance(3000);
    assert.equal(h.state.screen, 'found');
    assert.equal(h.state.waitingForEngine, true);
    h.game.isReady = true;
    h.ui.ready();
    assert.deepEqual(h.game.starts, [['guest', h.clients[0]]]);
    assert.equal(h.state.screen, 'hidden');
});

test('join errors and the room list going away are shown in the lobby', () => {
    const h = harness();
    h.ui.activate('online');
    h.rooms([room(TOKEN_A, 'Alpha')]);
    h.ui.activate('room:' + TOKEN_A);
    h.clients[0].status('error', 'Could not join this room. Room does not exist or has expired.');
    assert.equal(h.state.screen, 'lobby');
    assert.deepEqual([h.state.notice.text, h.state.notice.kind], ['Could not join this room. Room does not exist or has expired.', 'error']);
    assert.equal(h.watches.length, 2, 'the lobby list restarts after the join attempt');
    h.watches[0].options.onStatus('closed', 'An old list.');
    assert.equal(h.state.lobby.status, 'connecting', 'a closed old list is ignored');
    h.watches[1].options.onStatus('closed', 'The lobby closed.');
    assert.equal(h.state.lobby.status, 'error');
    assert.equal(h.watches[1].closed, true);
    h.advance(1900);
    assert.equal(h.watches.length, 2);
    h.advance(100);
    assert.equal(h.watches.length, 3, 'the lobby reconnects by itself');
    h.watches[2].options.onStatus('open', '');
    assert.equal(h.state.lobby.status, 'open');
});

test('without the engine hook, a second online match needs a reload', () => {
    const h = harness({ ready: true });
    h.ui.activate('online');
    h.rooms([room(TOKEN_A, 'Alpha')]);
    h.ui.activate('room:' + TOKEN_A);
    h.clients[0].status('ready', 'Opponent connected.');
    h.advance(3000);
    h.ui.gameScreen('melee');
    h.clients[0].status('closed', 'Your opponent left the room.');
    h.ui.gameScreen('mainMenu');
    assert.equal(h.state.screen, 'hidden', 'the page waits a moment for the engine to mark the hook');
    h.advance(2100);
    assert.equal(h.state.screen, 'restart');
    assert.match(h.state.message.detail, /play online again/);
    h.ui.activate('reload');
    assert.deepEqual(h.calls.slice(-1), ['reload:true']);
    h.ui.activate('back');
    assert.equal(h.state.screen, 'hidden');
    // From the main menu of a game started for local play.
    h.ui.fromMenu();
    assert.equal(h.state.screen, 'restart');
    assert.match(h.state.message.detail, /Reload to open Online Melee/);
});

test('invite links ask once, then join; copy link; no lobby support; failure and exit', async () => {
    const h = harness();
    h.ui.invite(TOKEN_B);
    assert.equal(h.state.screen, 'invite');
    assert.equal(h.state.focus, 'join');
    h.ui.key('Enter');
    assert.deepEqual(h.calls, ['fullscreen', 'clear-invite']);
    assert.deepEqual([h.clients[0].options.role, h.clients[0].options.token], ['guest', TOKEN_B]);
    assert.equal(h.state.screen, 'joining');

    const d = harness();
    d.ui.invite(TOKEN_B);
    d.ui.key('Escape');
    assert.equal(d.state.screen, 'start');

    const c = harness();
    c.ui.activate('online');
    c.ui.activate('create');
    c.ui.activate('open');
    c.ui.activate('copy');
    await Promise.resolve(); await Promise.resolve();
    assert.ok(c.calls.includes('copy:https://game.example/original/index.html#room=' + TOKEN_C));
    assert.match(c.state.notice.text, /copied/);

    const old = harness({ lobby: false });
    old.ui.activate('online');
    assert.equal(old.state.lobby.status, 'unsupported');
    assert.deepEqual(old.state.items, ['create', 'back']);

    const none = harness({ netplay: false });
    none.ui.activate('online');
    assert.equal(none.state.screen, 'message');
    assert.deepEqual(none.state.items, []);

    const f = harness();
    f.ui.fail('This server needs cross-origin isolation.');
    assert.equal(f.state.screen, 'message');
    assert.equal(f.state.message.say, 'Unable to start. This server needs cross-origin isolation.');
    f.ui.key('Enter');
    assert.deepEqual(f.calls.slice(-1), ['reload:false']);
    const e = harness({ started: true });
    e.ui.exited('Reload this page to play again.');
    assert.equal(e.state.message.say, 'Game closed. Reload this page to play again.');
});

test('both page shells use the in-frame UI, not the old Online Melee section', () => {
    for (const base of ['sc2/wasm', 'hd/wasm']) {
        const html = fs.readFileSync(base + '/shell.html', 'utf8');
        assert.ok(html.includes('<script src="online-ui.js"></script>'), base);
        assert.ok(html.includes('UQMOnline.init('), base);
        assert.ok(html.includes('takePendingNetplayRole'), base);
        assert.ok(html.includes("'--web-netplay=' + role"), base);
        for (const old of ['id="create-room"', 'id="launch-online"', 'id="join-invite"', 'class="online"', 'id="overlay"']) {
            assert.ok(!html.includes(old), base + ' still has ' + old);
        }
        assert.ok(html.includes('{{{ SCRIPT }}}'), base);
    }
    for (const script of ['scripts/build-web.sh', 'scripts/build-hd-web.sh', 'scripts/rust-engine/link.py']) {
        const text = fs.readFileSync(script, 'utf8');
        assert.ok(text.includes('web/online'), script + ' copies the online UI');
    }
    assert.ok(fs.readFileSync('scripts/package-editions.py', 'utf8').includes("'online-ui.js'"));
});
