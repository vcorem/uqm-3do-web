const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const help = require('../../web/game-help/game-help.js');

test('download progress is shown in MB and percent from the Emscripten byte counts', () => {
    assert.deepEqual(help.formatProgress('Downloading data... (154409693/298763132)'),
        { done: 154409693, total: 298763132, label: 'Downloading game data: 154 / 299 MB (51%)' });
    assert.equal(help.formatProgress('Downloading data... (640188199/640188199)').label, 'Downloading game data: 640 / 640 MB (100%)');
    for (const text of ['Running...', '', null, 'Downloading data... (5/0)', 'Downloading data... (9/5)']) {
        assert.equal(help.formatProgress(text), null, String(text));
    }
});

function page() {
    const elements = new Map();
    const make = (tag, id) => {
        const handlers = {};
        const element = { tagName: tag.toUpperCase(), id, hidden: false, children: [], attributes: {}, innerHTML: '', textContent: '',
            parentNode: null, focused: false,
            setAttribute(name, value) { this.attributes[name] = value; },
            appendChild(child) { child.parentNode = this; this.children.push(child); if (child.id) elements.set(child.id, child); return child; },
            insertBefore(child, before) { child.parentNode = this; this.children.splice(this.children.indexOf(before), 0, child); if (child.id) elements.set(child.id, child); return child; },
            addEventListener(type, fn) { handlers[type] = fn; },
            click() { handlers.click?.(); },
            focus() { this.focused = true; } };
        if (id) elements.set(id, element);
        return element;
    };
    const head = make('head'), bar = make('div', 'bar');
    const screen = make('div', 'screen'), canvas = make('canvas', 'canvas'), panel = make('aside', 'keys-panel');
    bar.appendChild(make('button', 'fullscreen'));
    const created = [];
    const document = {
        head,
        createElement(tag) { const element = make(tag); created.push(element); return element; },
        getElementById(id) {
            if (!elements.has(id)) {
                // Buttons written through innerHTML.
                const owner = created.find(element => element.innerHTML.includes('id="' + id + '"'));
                if (owner) return make('button', id);
            }
            return elements.get(id) || null;
        },
    };
    const listeners = {};
    let started = false;
    const api = help.init({ document, isStarted: () => started,
        eventTarget: { addEventListener(type, fn, capture) { assert.equal(capture, true); listeners[type] = fn; } } });
    const key = (key, extra = {}) => {
        const event = { key, target: canvas, prevented: false, stopped: false,
            preventDefault() { this.prevented = true; }, stopPropagation() { this.stopped = true; }, ...extra };
        listeners.keydown(event);
        return event;
    };
    return { api, elements, panel, canvas, key, start() { started = true; } };
}

test('the keys panel, Keys button and ? key show every key group', () => {
    const { elements, panel, key, canvas, start } = page();
    for (const [title] of help.KEYS) assert.ok(panel.innerHTML.includes('<h3>' + title + '</h3>'), title);
    const pop = elements.get('keys-pop'), button = elements.get('keys-button');
    assert.equal(pop.hidden, true);
    assert.equal(button.parentNode.children[0], button, 'Keys button comes before Full screen');
    button.click();
    assert.equal(pop.hidden, false);
    assert.equal(button.attributes['aria-expanded'], 'true');
    start();
    const event = key('?');
    assert.ok(event.prevented && event.stopped, 'the game does not receive ?');
    assert.equal(pop.hidden, true);
    assert.ok(canvas.focused, 'the keyboard returns to the game');
    assert.equal(key('?', { target: { tagName: 'INPUT' } }).prevented, false, 'typing ? into a field still works');
    assert.equal(key('?', { ctrlKey: true }).prevented, false);
});

test('F1 pauses the game without the browser help page, only after launch', () => {
    const { key, start } = page();
    assert.equal(key('F1').prevented, false);
    start();
    const event = key('F1');
    assert.equal(event.prevented, true);
    assert.equal(event.stopped, false, 'the game still receives F1');
});

test('the How to start lines for the start screen are plain text that fits the game fonts', () => {
    assert.equal(help.START_TEXT.length, 4);
    for (const line of help.START_TEXT) {
        assert.ok(!/[<>&]/.test(line), line);
        assert.ok(/^[\x20-\x7e]+$/.test(line), 'only characters the game fonts have: ' + line);
        assert.ok(line.length <= 60, line);
    }
    assert.match(help.START_TEXT.join(' '), /New Game.*Navigate.*starbase at Earth/);
});

test('both edition pages load the help script and have the panel the script fills', () => {
    for (const base of ['sc2/wasm', 'hd/wasm']) {
        const html = fs.readFileSync(base + '/shell.html', 'utf8');
        assert.ok(html.includes('<script src="game-help.js"></script>'), base);
        assert.ok(html.includes('id="keys-panel"'), base);
        assert.ok(html.includes('UQMHelp.formatProgress(text)'), base);
        assert.ok(!html.includes('class="controls"'), base);
    }
    for (const script of ['scripts/build-web.sh', 'scripts/build-hd-web.sh']) {
        assert.ok(fs.readFileSync(script, 'utf8').includes('web/game-help/game-help.js'), script);
    }
    assert.ok(fs.readFileSync('scripts/package-editions.py', 'utf8').includes("'game-help.js'"));
});
