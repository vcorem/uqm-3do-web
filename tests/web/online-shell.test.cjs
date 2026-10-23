// The page shells' real inline script with the real room client, against a
// minimal DOM and a stand-in for the in-frame menu (web/online/online-ui.js):
// what the shell gives the menu, and how the menu starts the engine.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const root = path.join(__dirname, '..', '..');
const token = 'ab'.repeat(16);
const help = require('../../web/game-help/game-help.js');

function shell(edition = 'original', variant = null) {
    const html = fs.readFileSync(path.join(root, edition === 'hd' ? 'hd/wasm/shell.html' : 'sc2/wasm/shell.html'), 'utf8');
    const elements = new Map();
    const document = { activeElement: null, fullscreenElement: null, head: { appendChild() {} },
        getElementById: id => elements.get(id) || null, querySelector: () => ({ textContent: '', appendChild() {} }),
        createTextNode: text => ({ text }), documentElement: { classList: { add() {} } }, addEventListener() {} };
    for (const match of html.matchAll(/<[^>]+\bid="([^"]+)"[^>]*>/g)) {
        elements.set(match[1], { id: match[1], textContent: '', className: '', hidden: false,
            setAttribute() {}, addEventListener() {}, focus() { document.activeElement = this; } });
    }
    const calls = [];
    const menu = { progress: [], ready: 0, failed: [], exited: [], taken: 0 };
    let options = null;
    const context = { document, console, URL, URLSearchParams, SharedArrayBuffer,
        location: { href: `http://localhost:10000/${edition}/index.html`, hash: '' },
        crypto: { getRandomValues(bytes) { bytes.fill(42); } },
        navigator: {}, crossOriginIsolated: true, localStorage: {},
        setTimeout() { return 1; }, clearTimeout() {}, setInterval() { return 2; }, clearInterval() {},
        addEventListener() {},
        UQMHelp: { formatProgress: help.formatProgress, START_TEXT: help.START_TEXT,
            MELEE_START_TEXT: help.MELEE_START_TEXT, init() {} },
        UQMOnline: { init(given) {
            options = given;
            return { overlay: { hidden: false }, controller: {
                progress: p => menu.progress.push(p), ready: () => menu.ready++,
                fail: text => menu.failed.push(text), exited: text => menu.exited.push(text),
                roleTaken: () => menu.taken++ } };
        } } };
    if (variant) context.UQM_VARIANT = variant;
    context.window = context;
    vm.createContext(context);
    vm.runInContext(fs.readFileSync(path.join(root, 'sc2/wasm/netplay-client.js'), 'utf8'), context);
    // The page's own scripts, up to the engine template (the HD page has one in its head too).
    for (const match of html.split('<template id="engine-script">')[0].matchAll(/<script>\s*([\s\S]*?)<\/script>/g)) {
        vm.runInContext(match[1], context);
    }
    context.Module.detect3DOVideos = () => ({ complete: false, missing: ['intro.duk'], total: 1 });
    context.Module.callMain = args => calls.push(Array.from(args));
    return { context, Module: context.Module, options, menu, calls, elements, html,
        ready() { context.Module.onRuntimeInitialized(); } };
}

// The HD page is also the Melee Remastered page: package-editions.py marks it with this variant.
const MELEE = { engine: 'c', saveDir: '/home/web_user/.uqm-melee', meleeOnly: true };
for (const [page, edition, variant] of [['original', 'original', null], ['hd', 'hd', null], ['melee', 'hd', MELEE]]) {
    const melee = page === 'melee';
    test(`${page} shell gives the menu its edition, engine, room client and game controls`, () => {
        const h = shell(edition, variant);
        assert.equal(h.options.edition, edition);
        assert.equal(h.options.engine, 'c');
        assert.equal(h.options.netplay, h.context.UQMNetplay, 'the real room client');
        assert.equal(h.options.screen, h.elements.get('screen'));
        assert.equal(h.options.canvas, h.elements.get('canvas'));
        assert.equal(h.options.startSteps, melee ? help.MELEE_START_TEXT : help.START_TEXT, 'page-specific How to start lines');
        assert.equal(!!h.options.meleeOnly, melee);
        assert.equal(!!h.Module.meleeOnly, melee);
        assert.equal(shell(edition, { ...variant, engine: 'rust', label: 'Rust engine (port)', saveDir: '/home/web_user/.uqm-x' }).options.engine, 'rust');
        assert.equal(h.options.game.started(), false);
        assert.equal(h.options.game.ready(), false);
        h.ready();
        assert.equal(h.menu.ready, 1);
        assert.equal(h.options.game.ready(), true);
    });

    test(`${page} shell forwards download progress to the start screen`, () => {
        const h = shell(edition, variant);
        h.Module.setStatus('Downloading data... (150000000/300000000)');
        // Objects made in the page's realm: compare their content.
        assert.equal(JSON.stringify(h.menu.progress.at(-1)), JSON.stringify({ label: 'Downloading game data: 150 / 300 MB (50%)', fraction: 0.5 }));
        h.Module.monitorRunDependencies(3);
        assert.equal(h.menu.progress.length, 1, 'byte progress wins over dependency counts');
        const plain = shell(edition, variant);
        plain.Module.monitorRunDependencies(4);
        plain.Module.monitorRunDependencies(1);
        assert.equal(plain.menu.progress.at(-1).fraction, 0.75);
    });

    test(`${page} shell starts a local game, or an online match with --web-netplay and the room`, () => {
        const local = shell(edition, variant);
        assert.equal(local.options.game.start(null), false, 'not before the runtime is ready');
        local.ready();
        const base = Array.from(local.Module.arguments);
        assert.equal(local.options.game.start(null), true);
        assert.deepEqual(local.calls, [base]);
        assert.equal(local.Module.webNet, undefined);
        assert.equal(local.options.game.started(), true);
        assert.equal(local.context.document.activeElement, local.elements.get('canvas'), 'the game gets the keyboard');
        assert.equal(local.options.game.start('host', {}), false, 'one engine start per page');

        const online = shell(edition, variant);
        online.ready();
        const room = online.context.UQMNetplay.connect({ token, role: 'guest', edition,
            WebSocket: class { constructor(url) { this.url = url; } close() {} }, setTimeout: () => 1, clearTimeout() {} });
        online.options.game.start('guest', room);
        assert.deepEqual(online.calls[0], [...Array.from(online.Module.arguments), '--web-netplay=guest']);
        assert.equal(online.Module.webNet, room);
        if (edition === 'hd') {
            assert.ok(online.calls[0].includes('--configdir=/home/web_user/' + (melee ? '.uqm-melee' : '.uqm-hd')));
            assert.ok(online.calls[0].includes('--contentdir=/content'));
        }
    });

    test(`${page} shell hands later matches to the engine's main menu once`, () => {
        const h = shell(edition, variant);
        h.ready();
        h.options.game.start(null);
        assert.equal(h.Module.takePendingNetplayRole(), -1);
        assert.equal(h.options.game.hook(), false);
        h.Module.supportsNetplayHook = true;
        const menuScreen = melee ? 'meleeMenu' : 'mainMenu';
        h.Module.gameScreen = menuScreen;
        assert.equal(h.options.game.hook(), true);
        assert.equal(h.options.game.screen(), menuScreen);
        const room = { state: 'ready' };
        h.options.game.queue(1, room);
        assert.equal(h.Module.webNet, room, 'the room is set before the role is returned');
        assert.equal(h.menu.taken, 0);
        assert.equal(h.Module.takePendingNetplayRole(), 1);
        assert.equal(h.menu.taken, 1, 'the menu learns that the engine took the role');
        assert.equal(h.Module.takePendingNetplayRole(), -1, 'the role is returned once');
        assert.equal(h.menu.taken, 1);
        h.options.game.queue(0, room);
        assert.equal(h.Module.takePendingNetplayRole(), 0);
        assert.equal(h.calls.length, 1, 'no second engine start');
        // A role not taken in time is withdrawn; the room stays as it was.
        h.options.game.queue(0, room);
        h.options.game.queue(-1, null);
        assert.equal(h.Module.takePendingNetplayRole(), -1);
        assert.equal(h.Module.webNet, room);
    });

    test(`${page} shell holds the main menu for the lobby and reports every game screen`, () => {
        const h = shell(edition, variant);
        h.options.game.hold(true);
        assert.equal(h.Module.holdMainMenu, true);
        h.options.game.hold(false);
        assert.equal(h.Module.holdMainMenu, false);
        const seen = [];
        h.options.game.watchScreen(name => seen.push(name));
        // The engine sets the screen in quick steps; each one is reported.
        const menuScreen = melee ? 'meleeMenu' : 'mainMenu';
        for (const name of ['transition', 'melee', menuScreen]) h.Module.gameScreen = name;
        assert.deepEqual(seen, ['transition', 'melee', menuScreen]);
        assert.equal(h.Module.gameScreen, menuScreen);
        assert.equal(h.options.game.screen(), menuScreen);
        // A failing report never reaches the engine.
        h.options.game.watchScreen(() => { throw new Error('menu broke'); });
        h.Module.gameScreen = 'melee';
        assert.equal(h.Module.gameScreen, 'melee');
        assert.match(h.elements.get('log').textContent, /menu broke/);
    });

    test(`${page} shell reports load failures and the engine's exit to the menu`, () => {
        const h = shell(edition, variant);
        h.Module.onAbort('out of memory');
        assert.match(h.menu.failed[0], /could not start: out of memory/);
        h.Module.onExit(0);
        assert.deepEqual(h.menu.exited, ['Reload this page to play again.']);
        assert.ok(!/id="(overlay|create-room|launch-online|join-invite|start)"/.test(h.html), 'no old start overlay or Online Melee section');
    });
}

test('without the menu script the shell still starts the game when it is ready', () => {
    const html = fs.readFileSync(path.join(root, 'sc2/wasm/shell.html'), 'utf8');
    assert.ok(html.includes('if (window.UQMOnline) online = UQMOnline.init('));
    assert.ok(html.includes('else launchGame(null);'));
});

test('Melee Remastered local requests are one-shot, survive initial startup and never restart the engine', () => {
    const h = shell('hd', MELEE);
    assert.equal(h.Module.takePendingLocalMelee(), false);
    h.ready();
    h.options.game.start(null);
    assert.equal(h.Module.takePendingLocalMelee(), true);
    assert.equal(h.Module.takePendingLocalMelee(), false);
    h.Module.supportsNetplayHook = true;
    h.Module.gameScreen = 'meleeMenu';
    h.options.game.local();
    assert.equal(h.Module.takePendingLocalMelee(), true);
    assert.equal(h.Module.takePendingLocalMelee(), false);
    assert.equal(h.calls.length, 1);
    const network = shell('hd', MELEE);
    network.ready();
    network.options.game.start('host', { state: 'ready' });
    assert.equal(network.Module.takePendingLocalMelee(), false, 'an online launch cannot also queue local play');
});

test('the HD page keeps its campaign flow and its own Melee art; the Melee page keeps the new art', () => {
    const hd = shell('hd');
    const styles = [];
    hd.Module._UQM_SetMeleeRemastered = value => styles.push(value);
    hd.ready();
    assert.deepEqual(styles, [0], 'Redrawn HD draws its own art in Super-Melee');
    hd.options.game.start(null);
    assert.equal(hd.Module.takePendingLocalMelee(), false, 'the campaign page queues no Melee-only choice');
    hd.Module.gameScreen = 'meleeMenu';
    assert.equal(hd.options.game.local(), false);
    assert.equal(hd.options.presentation, null);
    const melee = shell('hd', MELEE);
    melee.Module._UQM_SetMeleeRemastered = value => styles.push(value);
    melee.ready();
    assert.deepEqual(styles, [0], 'the Melee page leaves the art to its Picture & sound setting');
});
