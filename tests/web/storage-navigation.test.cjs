const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');

// variant: the UQM_VARIANT that package-editions.py writes into a page (Rust or Melee Remastered).
function harness(edition, withLink = true, persistGranted = true, variant = null) {
    const base = edition === 'hd' ? 'hd/wasm' : 'sc2/wasm';
    const html = fs.readFileSync(base + '/shell.html', 'utf8');
    const elements = new Map();
    for (const match of html.matchAll(/<[^>]+\bid="([^"]+)"[^>]*>/g)) {
        elements.set(match[1], element());
    }
    if (withLink) elements.set('choose-edition', element());
    else elements.delete('choose-edition');
    const pending = [];
    const files = new Map();
    const context = {
        document: { getElementById: id => elements.get(id) || null, addEventListener() {},
            querySelector: () => element(), createTextNode: text => ({ text }),
            documentElement: { classList: { add() {} } } },
        console: { log() {}, warn() {} }, URLSearchParams, SharedArrayBuffer,
        location: { hash: '' }, navigator: { storage: { persist: () => Promise.resolve(persistGranted) } },
        crossOriginIsolated: true, addEventListener() {}, addRunDependency() {}, removeRunDependency() {},
        UQM_VARIANT: variant || undefined,
        setInterval() {}, setTimeout() {},
        ENV: {}, IDBFS: {}, FS: {
            mkdirTree() {}, mount() {}, unmount() {},
            syncfs(populate, callback) { if (populate) callback(null); else pending.push(callback); },
            analyzePath: path => ({ exists: files.has(path) }),
            readFile(path) { if (!files.has(path)) throw new Error('missing'); return files.get(path); },
            writeFile: (path, content) => files.set(path, content),
        },
    };
    context.window = context;
    vm.createContext(context);
    // The page's own scripts, up to the engine template (the HD page has one in its head too).
    for (const match of html.split('<template id="engine-script">')[0].matchAll(/<script>\s*([\s\S]*?)<\/script>/g)) {
        vm.runInContext(match[1], context);
    }
    vm.runInContext(fs.readFileSync(base + '/pre.js', 'utf8'), context);
    context.Module.preRun.forEach(callback => callback());
    assert.equal(context.ENV.SDL_EMSCRIPTEN_KEYBOARD_ELEMENT, '#canvas');
    return { context, elements, pending,
        write() { context.wasm_syncfs(); },
        finish(error = null) { assert.ok(pending.length); pending.shift()(error); },
        click() { return elements.get('choose-edition').click(); } };
}
function element() {
    const handlers = {}, attrs = {};
    return { textContent: '', className: '', hidden: false, disabled: false, attrs,
        addEventListener(type, handler) { handlers[type] = handler; },
        setAttribute(name, value) { attrs[name] = value; },
        focus() {}, click() { let prevented = false; handlers.click?.({ preventDefault() { prevented = true; } }); return prevented; } };
}
for (const edition of ['original', 'hd']) {
    test(`${edition}: edition switch waits for all queued browser writes`, () => {
        const h = harness(edition);
        const link = h.elements.get('choose-edition');
        assert.equal(h.context.Module.storageState, 'ready');
        assert.equal(h.click(), false);
        h.write();
        h.write();
        assert.equal(h.pending.length, 1, 'writes are serialized');
        assert.equal(h.context.Module.storageState, 'saving');
        assert.equal(link.attrs['aria-disabled'], 'true');
        assert.equal(link.textContent, 'Saving…');
        assert.equal(h.click(), true, 'mouse and keyboard-generated click cannot navigate');
        h.finish();
        assert.equal(h.pending.length, 1, 'the queued flush must also finish');
        assert.equal(h.click(), true);
        h.finish();
        assert.equal(h.context.Module.storageState, 'saved');
        assert.equal(link.attrs['aria-disabled'], 'false');
        assert.equal(link.textContent, 'Choose edition');
        assert.equal(h.click(), false);
    });
    test(`${edition}: persistence warning cannot unlock an unfinished save`, async () => {
        const h = harness(edition, true, false);
        h.write();
        await new Promise(resolve => setImmediate(resolve));
        assert.equal(h.context.Module.storageState, 'saving');
        assert.equal(h.elements.get('choose-edition').attrs['aria-disabled'], 'true');
        assert.equal(h.click(), true);
        h.finish();
        assert.equal(h.click(), false);
    });
}
test('standalone Original has no edition link and still saves normally', () => {
    const h = harness('original', false);
    h.write(); h.finish();
    assert.equal(h.context.Module.storageState, 'saved');
    assert.ok(!fs.readFileSync('sc2/wasm/shell.html', 'utf8').includes('<a id="choose-edition"'));
});

const MELEE = { engine: 'c', saveDir: '/home/web_user/.uqm-melee', meleeOnly: true };

test('the HD page is Redrawn HD without the Melee flag', () => {
    const h = harness('hd');
    assert.equal(h.context.Module.meleeOnly, false);
    assert.ok(h.context.Module.arguments.includes('--configdir=/home/web_user/.uqm-hd'));
    assert.ok(h.context.Module.arguments.includes('--intro=3do'));
});

test('Melee Remastered: its own fleets folder, and "Choose game" waits for every queued write', () => {
    const h = harness('hd', true, true, MELEE);
    assert.equal(h.context.Module.meleeOnly, true);
    assert.equal(h.context.Module.uqmSavePath, MELEE.saveDir);
    assert.ok(h.context.Module.arguments.includes('--configdir=' + MELEE.saveDir));
    assert.ok(!h.context.Module.arguments.some(arg => arg.startsWith('--intro')), 'the Melee profile has no intro');
    const link = h.elements.get('choose-edition');
    assert.equal(h.context.Module.storageState, 'ready');
    h.write(); h.write();
    assert.equal(h.pending.length, 1);
    assert.equal(h.context.Module.storageState, 'saving');
    assert.equal(link.textContent, 'Saving…');
    assert.equal(h.click(), true, 'no navigation while a write is open');
    h.finish();
    assert.equal(h.pending.length, 1, 'the second flush is required before reporting a saved fleet');
    assert.equal(h.context.Module.storageState, 'saving');
    h.finish();
    assert.equal(h.context.Module.storageState, 'saved');
    assert.equal(link.textContent, 'Choose game');
    assert.equal(h.click(), false);
});

test('Melee Remastered persistence warning preserves an unfinished fleet/preferences save', async () => {
    const h = harness('hd', false, false, MELEE);
    h.write();
    await new Promise(resolve => setImmediate(resolve));
    assert.equal(h.context.Module.storageState, 'saving');
    assert.equal(h.pending.length, 1);
    h.finish();
    assert.equal(h.context.Module.storageState, 'saved');
    assert.equal(h.elements.has('choose-edition'), false);
});
