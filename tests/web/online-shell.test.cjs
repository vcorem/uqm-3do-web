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
        getElementById: id => elements.get(id) || null, querySelector: () => ({ textContent: '' }),
        addEventListener() {} };
    for (const match of html.matchAll(/<[^>]+\bid="([^"]+)"[^>]*>/g)) {
        elements.set(match[1], { id: match[1], textContent: '', className: '', hidden: false,
            setAttribute() {}, addEventListener() {}, focus() { document.activeElement = this; } });
    }
    const calls = [];
    const menu = { progress: [], ready: 0, failed: [], exited: [] };
    let options = null;
    const context = { document, console, URL, URLSearchParams, SharedArrayBuffer,
        location: { href: `http://localhost:10000/${edition}/index.html`, hash: '' },
        crypto: { getRandomValues(bytes) { bytes.fill(42); } },
        navigator: {}, crossOriginIsolated: true, localStorage: {},
        setTimeout() { return 1; }, clearTimeout() {}, setInterval() { return 2; }, clearInterval() {},
        addEventListener() {},
        UQMHelp: { formatProgress: help.formatProgress, START_TEXT: help.START_TEXT, init() {} },
        UQMOnline: { init(given) {
            options = given;
            return { overlay: { hidden: false }, controller: {
                progress: p => menu.progress.push(p), ready: () => menu.ready++,
                fail: text => menu.failed.push(text), exited: text => menu.exited.push(text) } };
        } } };
    if (variant) context.UQM_VARIANT = variant;
    context.window = context;
    vm.createContext(context);
    vm.runInContext(fs.readFileSync(path.join(root, 'sc2/wasm/netplay-client.js'), 'utf8'), context);
    vm.runInContext(html.match(/<script>\s*([\s\S]*?)<\/script>/)[1], context);
    context.Module.detect3DOVideos = () => ({ complete: false, missing: ['intro.duk'], total: 1 });
    context.Module.callMain = args => calls.push(Array.from(args));
    return { context, Module: context.Module, options, menu, calls, elements, html,
        ready() { context.Module.onRuntimeInitialized(); } };
}

for (const edition of ['original', 'hd']) {
    test(`${edition} shell gives the menu its edition, engine, room client and game controls`, () => {
        const h = shell(edition);
        assert.equal(h.options.edition, edition);
        assert.equal(h.options.engine, 'c');
        assert.equal(h.options.netplay, h.context.UQMNetplay, 'the real room client');
        assert.equal(h.options.screen, h.elements.get('screen'));
        assert.equal(h.options.canvas, h.elements.get('canvas'));
        assert.equal(h.options.startSteps, help.START_TEXT, 'the How to start lines');
        assert.equal(shell(edition, { engine: 'rust', label: 'Rust engine (port)', saveDir: '/home/web_user/.uqm-x' }).options.engine, 'rust');
        assert.equal(h.options.game.started(), false);
        assert.equal(h.options.game.ready(), false);
        h.ready();
        assert.equal(h.menu.ready, 1);
        assert.equal(h.options.game.ready(), true);
    });

    test(`${edition} shell forwards download progress to the start screen`, () => {
        const h = shell(edition);
        h.Module.setStatus('Downloading data... (150000000/300000000)');
        // Objects made in the page's realm: compare their content.
        assert.equal(JSON.stringify(h.menu.progress.at(-1)), JSON.stringify({ label: 'Downloading game data: 150 / 300 MB (50%)', fraction: 0.5 }));
        h.Module.monitorRunDependencies(3);
        assert.equal(h.menu.progress.length, 1, 'byte progress wins over dependency counts');
        const plain = shell(edition);
        plain.Module.monitorRunDependencies(4);
        plain.Module.monitorRunDependencies(1);
        assert.equal(plain.menu.progress.at(-1).fraction, 0.75);
    });

    test(`${edition} shell starts a local game, or an online match with --web-netplay and the room`, () => {
        const local = shell(edition);
        assert.equal(local.options.game.start(null), false, 'not before the runtime is ready');
        local.ready();
        const base = Array.from(local.Module.arguments);
        assert.equal(local.options.game.start(null), true);
        assert.deepEqual(local.calls, [base]);
        assert.equal(local.Module.webNet, undefined);
        assert.equal(local.options.game.started(), true);
        assert.equal(local.context.document.activeElement, local.elements.get('canvas'), 'the game gets the keyboard');
        assert.equal(local.options.game.start('host', {}), false, 'one engine start per page');

        const online = shell(edition);
        online.ready();
        const room = online.context.UQMNetplay.connect({ token, role: 'guest', edition,
            WebSocket: class { constructor(url) { this.url = url; } close() {} }, setTimeout: () => 1, clearTimeout() {} });
        online.options.game.start('guest', room);
        assert.deepEqual(online.calls[0], [...Array.from(online.Module.arguments), '--web-netplay=guest']);
        assert.equal(online.Module.webNet, room);
        if (edition === 'hd') {
            assert.ok(online.calls[0].includes('--configdir=/home/web_user/.uqm-hd'));
            assert.ok(online.calls[0].includes('--contentdir=/content'));
        }
    });

    test(`${edition} shell hands later matches to the engine's main menu once`, () => {
        const h = shell(edition);
        h.ready();
        h.options.game.start(null);
        assert.equal(h.Module.takePendingNetplayRole(), -1);
        assert.equal(h.options.game.hook(), false);
        h.Module.supportsNetplayHook = true;
        h.Module.gameScreen = 'mainMenu';
        assert.equal(h.options.game.hook(), true);
        assert.equal(h.options.game.screen(), 'mainMenu');
        const room = { state: 'ready' };
        h.options.game.queue(1, room);
        assert.equal(h.Module.webNet, room, 'the room is set before the role is returned');
        assert.equal(h.Module.takePendingNetplayRole(), 1);
        assert.equal(h.Module.takePendingNetplayRole(), -1, 'the role is returned once');
        h.options.game.queue(0, room);
        assert.equal(h.Module.takePendingNetplayRole(), 0);
        assert.equal(h.calls.length, 1, 'no second engine start');
    });

    test(`${edition} shell reports load failures and the engine's exit to the menu`, () => {
        const h = shell(edition);
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
