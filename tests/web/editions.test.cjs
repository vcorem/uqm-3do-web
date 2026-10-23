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

function harness({ saved, savedEngine, storageFailure, href = 'https://game.example/', navigationFailure, page = html } = {}) {
    const entries = new Map(saved === undefined ? [] : [[editions.STORAGE_KEY, saved]]);
    if (savedEngine !== undefined) entries.set(editions.ENGINE_KEY, savedEngine);
    const navigations = [], requests = [], writes = [];
    const pageHandlers = {};
    const elements = new Map();
    for (const match of page.matchAll(/<[^>]+\bid="([^"]+)"[^>]*>/g)) {
        const handlers = {};
        elements.set(match[1], {
            value: match[0].match(/\bvalue="([^"]*)"/)?.[1] || '',
            checked: /\bchecked\b/.test(match[0]), disabled: /\bdisabled\b/.test(match[0]),
            hidden: /\bhidden\b/.test(match[0]), textContent: '',
            getAttribute(name) { return match[0].match(new RegExp('\\b' + name + '="([^"]*)"'))?.[1] ?? null; },
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
        chooseEngine(engine) { const radio = elements.get('engine-' + engine); radio.checked = true; radio.trigger('change'); },
        submit() { elements.get('edition-form').trigger('submit'); },
        pageShow(persisted = true) { pageHandlers.pageshow?.({ persisted }); } };
}

test('chooser loads only its own interface assets, never an edition bundle', () => {
    const scripts = [...html.matchAll(/<script\b[^>]*\bsrc="([^"]+)"/g)].map(match => match[1]);
    const styles = [...html.matchAll(/<link\b[^>]*\bhref="([^"]+)"/g)].map(match => match[1]);
    const images = [...html.matchAll(/<img\b[^>]*\bsrc="([^"]+)"/g)].map(match => match[1]);
    assert.deepEqual(scripts, ['editions.js']);
    assert.deepEqual(styles, ['editions.css']);
    // Two pictures next to this page: the Melee Remastered card art (scripts/make-melee-card.py)
    // and the Cosmo Clash card art, a small copy of its title screen.
    assert.deepEqual(images, ['melee-card.jpg', 'cosmoclash-card.jpg']);
    for (const image of images) assert.ok(fs.statSync(path.join(root, image)).size < 120000, image + ': the card art stays small');
    assert.doesNotMatch(html, /<(?:iframe|object|embed|video|audio)\b|\b(?:preload|prefetch)\b/i);
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
    assert.match(first.elements.get('selection-note').textContent, /1280 × 960.*Online Melee with HD players.*separate saves/);
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

test('the engine switch opens the Rust builds and remembers the choice', () => {
    assert.equal(editions.destination('https://game.example/', 'original', 'rust'), 'https://game.example/original-rust/index.html');
    assert.equal(editions.destination('https://game.example/', 'hd', 'rust'), 'https://game.example/hd-rust/index.html');
    assert.equal(editions.destination('https://game.example/', 'hd', 'other'), 'https://game.example/hd/index.html');
    assert.equal(editions.destination('https://game.example/', 'hd'), 'https://game.example/hd/index.html');
    const h = harness();
    assert.equal(h.elements.get('engine-choice').hidden, false);
    assert.equal(h.elements.get('engine-c').checked, true);
    h.chooseEngine('rust');
    h.choose('hd');
    assert.match(h.elements.get('launch-label').textContent, /Play Redrawn HD · Rust/);
    assert.match(h.elements.get('selection-note').textContent, /own saves/);
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/hd-rust/index.html']);
    assert.equal(h.entries.get(editions.ENGINE_KEY), 'rust');
    const again = harness({ saved: 'original', savedEngine: 'rust' });
    assert.equal(again.elements.get('engine-rust').checked, true);
    again.submit();
    assert.deepEqual(again.navigations, ['https://game.example/original-rust/index.html']);
    assert.deepEqual(again.requests, []);
});

test('an invitation keeps Original but lets the guest choose the Rust engine', () => {
    const h = harness({ savedEngine: 'rust', href: 'https://game.example/#room=' + token });
    assert.equal(h.elements.get('edition-hd').disabled, true);
    assert.equal(h.elements.get('engine-rust').disabled, false);
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/original-rust/index.html#room=' + token]);
});

test('a package without Rust builds hides the switch and never opens a Rust page', () => {
    const page = html.replace('data-engines="c,rust"', 'data-engines="c"');
    assert.notEqual(page, html);
    const h = harness({ page, saved: 'hd', savedEngine: 'rust' });
    assert.equal(h.elements.get('engine-choice').hidden, true);
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/hd/index.html']);
});

// The package without Cosmo Clash, as scripts/package-editions.py writes it.
const withoutCosmo = html.replace(/\s*<!-- cosmoclash(?::[^>]*)? -->[\s\S]*?<!-- \/cosmoclash -->/g, '')
    .replace('data-games="uqm,cosmoclash"', 'data-games="uqm"');

test('the Cosmo Clash card opens /cosmoclash/, hides the engine switch and is remembered', () => {
    assert.equal(editions.destination('https://game.example/', 'cosmoclash'), 'https://game.example/cosmoclash/');
    assert.equal(editions.destination('https://game.example/', 'cosmoclash', 'rust'), 'https://game.example/cosmoclash/');
    const h = harness({ savedEngine: 'rust' });
    assert.equal(h.elements.get('edition-cosmoclash').checked, false, 'Original stays the default');
    h.choose('cosmoclash');
    assert.equal(h.elements.get('edition-cosmoclash').checked, true);
    assert.equal(h.elements.get('edition-original').checked, false);
    assert.equal(h.elements.get('launch-label').textContent, 'Play Cosmo Clash');
    assert.match(h.elements.get('selection-note').textContent, /computer.*one keyboard.*online/);
    assert.equal(h.elements.get('engine-choice').hidden, true, 'Cosmo Clash has one engine');
    assert.equal(h.entries.get(editions.STORAGE_KEY), 'cosmoclash');
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/cosmoclash/']);
    assert.deepEqual(h.requests, []);
    // Back from the game: the card is still chosen; a UQM edition shows the engine switch again.
    h.pageShow();
    assert.equal(h.elements.get('edition-cosmoclash').disabled, false);
    h.choose('hd');
    assert.equal(h.elements.get('engine-choice').hidden, false);
    assert.match(h.elements.get('launch-label').textContent, /Play Redrawn HD · Rust/);
    const revisit = harness({ saved: 'cosmoclash' });
    assert.equal(revisit.elements.get('edition-cosmoclash').checked, true);
    assert.equal(revisit.elements.get('engine-choice').hidden, true);
    revisit.submit();
    assert.deepEqual(revisit.navigations, ['https://game.example/cosmoclash/']);
});

test('an Original invitation keeps the Cosmo Clash card closed', () => {
    const h = harness({ saved: 'cosmoclash', href: 'https://game.example/#room=' + token });
    assert.equal(h.elements.get('edition-original').checked, true);
    assert.equal(h.elements.get('edition-cosmoclash').disabled, true);
    assert.equal(h.elements.get('engine-choice').hidden, false);
    h.choose('cosmoclash');
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/original/index.html#room=' + token]);
    assert.equal(h.entries.get(editions.STORAGE_KEY), 'cosmoclash', 'the saved choice stays');
    h.pageShow();
    assert.equal(h.elements.get('edition-cosmoclash').disabled, true);
});

test('the Melee Remastered card opens the HD engine with the Melee page, keeps the engine switch and is remembered', () => {
    assert.equal(editions.destination('https://game.example/', 'melee'), 'https://game.example/melee/index.html');
    assert.equal(editions.destination('https://game.example/', 'melee', 'rust'), 'https://game.example/melee-rust/index.html');
    const h = harness();
    h.choose('melee');
    assert.equal(h.elements.get('edition-melee').checked, true);
    assert.equal(h.elements.get('edition-hd').checked, false);
    assert.equal(h.elements.get('launch-label').textContent, 'Play Melee Remastered');
    assert.match(h.elements.get('selection-note').textContent, /Network Melee.*shared with Redrawn HD/);
    assert.equal(h.elements.get('engine-choice').hidden, false, 'Melee has the C and Rust engines of HD');
    assert.equal(h.entries.get(editions.STORAGE_KEY), 'melee');
    h.chooseEngine('rust');
    assert.equal(h.elements.get('launch-label').textContent, 'Play Melee Remastered · Rust');
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/melee-rust/index.html']);
    const revisit = harness({ saved: 'melee', savedEngine: 'c' });
    assert.equal(revisit.elements.get('edition-melee').checked, true);
    revisit.submit();
    assert.deepEqual(revisit.navigations, ['https://game.example/melee/index.html']);
    // An Original invitation keeps this card closed too.
    const invited = harness({ saved: 'melee', href: 'https://game.example/#room=' + token });
    assert.equal(invited.elements.get('edition-melee').disabled, true);
    invited.submit();
    assert.deepEqual(invited.navigations, ['https://game.example/original/index.html#room=' + token]);
    // The card order: Original, HD, Melee Remastered, Cosmo Clash.
    assert.deepEqual([...html.matchAll(/id="edition-([a-z]+)" type="radio"/g)].map(match => match[1]),
        ['original', 'hd', 'melee', 'cosmoclash']);
});

test('a package without Cosmo Clash has three cards and opens Original for a saved Cosmo Clash choice', () => {
    assert.notEqual(withoutCosmo, html);
    assert.doesNotMatch(withoutCosmo, /cosmoclash|Cosmo Clash/i);
    assert.deepEqual([...withoutCosmo.matchAll(/<img\b[^>]*\bsrc="([^"]+)"/g)].map(match => match[1]), ['melee-card.jpg']);
    const h = harness({ page: withoutCosmo, saved: 'cosmoclash' });
    assert.equal(h.elements.has('edition-cosmoclash'), false);
    assert.equal(h.elements.get('edition-original').checked, true);
    assert.equal(h.elements.get('launch-label').textContent, 'Play Original 3DO');
    h.submit();
    assert.deepEqual(h.navigations, ['https://game.example/original/index.html']);
});
