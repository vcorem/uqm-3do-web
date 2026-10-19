const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const editions = require('../../web/editions/editions.js');
const root = path.join(__dirname, '../../web/editions');
const html = fs.readFileSync(path.join(root, 'index.html'), 'utf8');
const script = fs.readFileSync(path.join(root, 'editions.js'), 'utf8');
const token = '0123456789abcdef0123456789abcdef';

function harness({ saved, storageFailure, href = 'https://game.example/', navigationFailure } = {}) {
    const entries = new Map(saved === undefined ? [] : [[editions.STORAGE_KEY, saved]]);
    const navigations = [], requests = [], writes = [];
    const pageHandlers = {};
    const elements = new Map();
    for (const match of html.matchAll(/<[^>]+\bid="([^"]+)"[^>]*>/g)) {
        const handlers = {};
        elements.set(match[1], {
            value: match[0].match(/\bvalue="([^"]*)"/)?.[1] || '',
            checked: /\bchecked\b/.test(match[0]), disabled: /\bdisabled\b/.test(match[0]),
            hidden: /\bhidden\b/.test(match[0]), textContent: '',
            addEventListener(type, handler) { handlers[type] = handler; },
            trigger(type) { handlers[type]?.call(this, { preventDefault() {} }); },
        });
    }
    const context = {
        URL, URLSearchParams,
        addEventListener(type, handler) { pageHandlers[type] = handler; },
        document: { getElementById(id) { assert.ok(elements.has(id), id); return elements.get(id); },
            createElement() { throw new Error('The chooser must not inject engines or media.'); } },
        location: { href, assign(url) { if (navigationFailure) throw new Error(navigationFailure); navigations.push(url); } },
        fetch(url) { requests.push(url); throw new Error('The chooser must not fetch a game.'); },
        Worker: function (url) { requests.push(url); throw new Error('The chooser must not start game workers.'); },
        WebSocket: function (url) { requests.push(url); throw new Error('An invite must not connect automatically.'); },
    };
    Object.defineProperty(context, 'localStorage', { get() {
        if (storageFailure === 'getter') throw new Error('SecurityError');
        return {
            getItem(key) { if (storageFailure === 'read') throw new Error('SecurityError'); return entries.get(key) ?? null; },
            setItem(key, value) { if (storageFailure === 'write') throw new Error('QuotaExceededError'); entries.set(key, value); writes.push([key, value]); },
        };
    } });
    vm.runInNewContext(script, context, { filename: 'editions.js' });
    return { elements, entries, navigations, requests, writes,
        choose(edition) { const radio = elements.get('edition-' + edition); radio.checked = true; radio.trigger('change'); },
        submit() { elements.get('edition-form').trigger('submit'); },
        pageShow(persisted = true) { pageHandlers.pageshow?.({ persisted }); } };
}

test('chooser loads only its own interface assets, never an edition bundle', () => {
    const scripts = [...html.matchAll(/<script\b[^>]*\bsrc="([^"]+)"/g)].map(match => match[1]);
    const styles = [...html.matchAll(/<link\b[^>]*\bhref="([^"]+)"/g)].map(match => match[1]);
    assert.deepEqual(scripts, ['editions.js']);
    assert.deepEqual(styles, ['editions.css']);
    assert.doesNotMatch(html, /<(?:iframe|object|embed|img)\b|\b(?:preload|prefetch)\b/i);
    assert.doesNotMatch(fs.readFileSync(path.join(root, 'editions.css'), 'utf8'), /\burl\s*\(/i);
    assert.ok(fs.existsSync(path.join(root, scripts[0])));
    assert.ok(fs.existsSync(path.join(root, styles[0])));
    const h = harness();
    assert.deepEqual(h.requests, []);
    assert.deepEqual(h.navigations, []);
    h.choose('hd');
    assert.deepEqual(h.requests, []);
    assert.deepEqual(h.navigations, []);
    h.submit();
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/hd/index.html']);
});

test('Original is the default; valid persisted choice restores and changes persist', () => {
    const first = harness();
    assert.equal(first.elements.get('edition-original').checked, true);
    assert.equal(first.elements.get('launch-label').textContent, 'Play Original 3DO');
    assert.equal(first.writes.length, 0, 'merely opening the chooser does not write preferences');
    first.choose('hd');
    assert.equal(first.entries.get(editions.STORAGE_KEY), 'hd');
    assert.equal(first.elements.get('edition-original').checked, false);
    assert.match(first.elements.get('selection-note').textContent, /1280 × 960.*Local play.*separate saves/);
    const revisit = harness({ saved: first.entries.get(editions.STORAGE_KEY) });
    assert.equal(revisit.elements.get('edition-hd').checked, true);
    revisit.choose('original');
    revisit.submit();
    assert.equal(revisit.entries.get(editions.STORAGE_KEY), 'original');
    assert.deepEqual(revisit.navigations, ['https://game.example/original/index.html']);
});

test('unknown preferences and untrusted selection values cannot change destination origin', () => {
    for (const saved of [null, '', 'HD', 'other', 'https://evil.example/', '__proto__']) {
        const h = harness({ saved });
        assert.equal(h.elements.get('edition-original').checked, true);
        h.submit();
        assert.deepEqual(h.navigations, ['https://game.example/original/index.html']);
    }
    const h = harness();
    h.elements.get('edition-hd').value = 'https://evil.example/';
    h.choose('hd');
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/original/index.html']);
});

for (const storageFailure of ['getter', 'read', 'write']) {
    test(`storage ${storageFailure} failure still allows HD launch`, () => {
        const h = harness({ storageFailure });
        assert.equal(h.elements.get('edition-original').checked, true);
        h.choose('hd');
        h.submit();
        assert.deepEqual(h.navigations, ['https://game.example/hd/index.html']);
        if (storageFailure !== 'read') assert.match(h.elements.get('preference-note').textContent, /cannot remember/);
    });
}

test('Original invitations preserve query and fragment, await clicks, and retain saved HD preference', () => {
    const href = 'https://game.example/?source=friend&redirect=https%3A%2F%2Fevil.example#room=' + token + '&extra=1';
    const h = harness({ saved: 'hd', href });
    assert.equal(h.elements.get('edition-original').checked, true);
    assert.equal(h.elements.get('edition-hd').disabled, true);
    assert.equal(h.elements.get('invite-note').hidden, false);
    assert.deepEqual(h.requests, []);
    assert.deepEqual(h.navigations, []);
    h.choose('hd'); // Even a programmatic change cannot move a room into HD.
    h.submit();
    const expected = new URL(href); expected.pathname = '/original/index.html';
    assert.deepEqual(h.navigations, [expected.href]);
    assert.equal(h.entries.get(editions.STORAGE_KEY), 'hd');
    assert.equal(h.writes.length, 0);
});

test('malformed room fragments reach the Original client for validation; unrelated fragments do not force it', () => {
    for (const fragment of ['#room=', '#room=invalid', '#room=one&room=two']) {
        const h = harness({ saved: 'hd', href: 'https://game.example/' + fragment });
        h.submit();
        assert.equal(h.navigations[0], 'https://game.example/original/index.html' + fragment);
    }
    const h = harness({ saved: 'hd', href: 'https://game.example/?room=not-an-invite#controls' });
    h.submit();
    assert.equal(h.navigations[0], 'https://game.example/hd/index.html?room=not-an-invite#controls');
});

test('failed navigation restores controls and explains how to open the game', () => {
    const h = harness({ href: 'file:///tmp/index.html' });
    h.submit();
    assert.deepEqual(h.navigations, []);
    assert.equal(h.elements.get('launch-edition').disabled, false);
    assert.equal(h.elements.get('edition-hd').disabled, false);
    assert.match(h.elements.get('selection-note').textContent, /included game server/);
});


test('Back/Forward cache restores a usable chooser and permits another selection', () => {
    const h = harness({ saved: 'hd' });
    h.submit();
    assert.equal(h.elements.get('launch-edition').disabled, true);
    assert.equal(h.elements.get('edition-original').disabled, true);
    assert.equal(h.elements.get('edition-hd').disabled, true);
    h.pageShow(false); // Initial pageshow must not unlock an in-flight navigation.
    assert.equal(h.elements.get('launch-edition').disabled, true);
    h.pageShow();
    assert.equal(h.elements.get('launch-edition').disabled, false);
    assert.equal(h.elements.get('edition-original').disabled, false);
    assert.equal(h.elements.get('edition-hd').disabled, false);
    assert.equal(h.elements.get('edition-hd').checked, true);
    h.choose('original');
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/hd/index.html',
        'https://game.example/original/index.html']);
    assert.equal(h.entries.get(editions.STORAGE_KEY), 'original');
});

test('restored invitation remains Original-only without overwriting the saved HD choice', () => {
    const h = harness({ saved: 'hd', href: 'https://game.example/#room=' + token });
    h.submit();
    h.pageShow();
    assert.equal(h.elements.get('launch-edition').disabled, false);
    assert.equal(h.elements.get('edition-original').disabled, false);
    assert.equal(h.elements.get('edition-hd').disabled, true);
    assert.equal(h.elements.get('edition-original').checked, true);
    assert.equal(h.elements.get('invite-note').hidden, false);
    h.choose('hd'); // Programmatic selection still cannot redirect an invitation.
    h.submit();
    assert.deepEqual(h.navigations, Array(2).fill('https://game.example/original/index.html#room=' + token));
    assert.equal(h.entries.get(editions.STORAGE_KEY), 'hd');
    assert.equal(h.writes.length, 0);
});
