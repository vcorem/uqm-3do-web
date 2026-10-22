const test = require('node:test');
const assert = require('node:assert/strict');
const presentation = require('../../hd/wasm/melee-presentation.js');

function harness(initial = null, blocked = false) {
    const entries = new Map([[presentation.STORAGE_KEY, initial]]);
    const getStorage = () => {
        if (blocked) throw new Error('storage disabled');
        return { getItem: key => entries.get(key), setItem: (key, value) => entries.set(key, value) };
    };
    const ids = ['screen', 'presentation-panel', 'presentation-toggle', 'presentation-close', 'melee-style',
        'melee-soundtrack', 'melee-music-volume', 'melee-volume-value', 'presentation-status'];
    const elements = new Map(ids.map(id => [id, {
        hidden: id === 'presentation-panel', listeners: {}, attributes: {}, value: '', textContent: '',
        addEventListener(name, fn) { this.listeners[name] = fn; },
        setAttribute(name, value) { this.attributes[name] = value; }, focus() { this.focused = true; },
        send(name, value) { this.listeners[name]({ target: { value }, stopPropagation() {}, preventDefault() {} }); }
    }]));
    const native = [];
    const module = { _UQM_SetMeleeRemastered: value => native.push(value) };
    const canvas = { focus() { this.focused = true; } };
    let started = false;
    const ui = presentation.init({ document: { getElementById: id => elements.get(id) }, module,
        getStorage, isStarted: () => started, canvas });
    return { ui, entries, elements, native, canvas, module, start() { started = true; } };
}

test('corrupt or blocked storage starts with a playable remastered presentation', () => {
    for (const value of [null, '{bad json', '[]', '123']) {
        const h = harness(value);
        assert.deepEqual(h.ui.settings, { style: 'remastered', soundtrack: 'remastered', musicVolume: 72 });
        assert.deepEqual(h.native, [], 'never call native exports before the runtime is ready');
        h.ui.runtimeReady();
        assert.deepEqual(h.native, [1]);
    }
    const blocked = harness(null, true);
    blocked.ui.runtimeReady();
    blocked.elements.get('melee-style').send('change', 'classic');
    assert.deepEqual(blocked.native, [1, 0]);
    assert.match(blocked.elements.get('presentation-status').textContent, /this tab only/);
});

test('appearance switches live and persists, without changing soundtrack or battle inputs', () => {
    const h = harness();
    h.ui.runtimeReady();
    h.start();
    h.elements.get('melee-style').send('change', 'classic');
    assert.deepEqual(h.native, [1, 0]);
    assert.equal(h.elements.get('screen').attributes['data-presentation'], 'classic');
    assert.equal(JSON.parse(h.entries.get(presentation.STORAGE_KEY)).style, 'classic');
    assert.equal(h.ui.settings.soundtrack, 'remastered');
    h.elements.get('presentation-toggle').send('click');
    assert.equal(h.elements.get('presentation-panel').hidden, false);
    h.elements.get('presentation-close').send('click');
    assert.equal(h.elements.get('presentation-panel').hidden, true);
    assert.equal(h.canvas.focused, true, 'return keyboard control to the game');
});

test('native soundtrack and bounded volume are applied once at launch without disturbing netplay', () => {
    const base = ['--contentdir=/content', '--addon=other', '--web-netplay=guest', '--musicvol=12', '--addon=remastered'];
    assert.deepEqual(presentation.launchArguments(base, { soundtrack: 'classic', musicVolume: 199 }),
        ['--contentdir=/content', '--addon=other', '--web-netplay=guest', '--musicvol=100']);
    assert.equal(base.length, 5, 'input arguments are never mutated');
    const h = harness();
    const launch = h.ui.prepareLaunch(base);
    assert.deepEqual(launch, ['--contentdir=/content', '--addon=other', '--web-netplay=guest', '--addon=remastered', '--musicvol=72']);
    h.start();
    h.elements.get('melee-soundtrack').send('change', 'classic');
    h.elements.get('melee-music-volume').send('input', '0');
    assert.match(h.elements.get('presentation-status').textContent, /next launch/);
    assert.equal(h.ui.settings.musicVolume, 0);
    assert.equal(launch.at(-1), '--musicvol=72', 'never reload or change an active match behind the player');
});

test('an older engine without presentation exports still starts and explains the fallback', () => {
    const h = harness();
    delete h.module._UQM_SetMeleeRemastered;
    assert.doesNotThrow(() => h.ui.runtimeReady());
    assert.match(h.elements.get('presentation-status').textContent, /does not include/);
    assert.deepEqual(h.ui.prepareLaunch(['--windowed']), ['--windowed', '--addon=remastered', '--musicvol=72']);
});
