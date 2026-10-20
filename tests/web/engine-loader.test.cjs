const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const loader = require('../../web/engine-loader/engine-loader.js');
const help = require('../../web/game-help/game-help.js');

const ORIGIN = 'https://game.test';

function content(length, seed = 7) {
    const bytes = new Uint8Array(length);
    for (let i = 0; i < length; i++) bytes[i] = (i * 31 + seed) & 255;
    return bytes;
}

/** A body that arrives in chunks, optionally failing part of the way through. */
function chunked(bytes, size, failAfter) {
    let offset = 0;
    return new ReadableStream({
        pull(controller) {
            if (failAfter !== undefined && offset >= failAfter) return controller.error(new TypeError('network connection lost'));
            if (offset >= bytes.length) return controller.close();
            controller.enqueue(bytes.slice(offset, offset + size));
            offset += size;
        },
    });
}

/** The Worker: HEAD answers the ETag and size, GET streams the bytes. */
function server(bytes, { etag = '"v1"', headStatus = 200, headThrows = false, noETag = false, getETag, failAfter } = {}) {
    const requests = [];
    async function fetch(url, init = {}) {
        const method = init.method || 'GET';
        requests.push({ url: String(url), method, cache: init.cache });
        if (method === 'HEAD') {
            if (headThrows) throw new TypeError('Failed to fetch');
            const headers = { 'Content-Length': String(bytes.length) };
            if (!noETag) headers.ETag = etag;
            return new Response(null, { status: headStatus, headers });
        }
        return new Response(chunked(bytes, 4096, failAfter), { headers: {
            'Content-Length': String(bytes.length), ETag: getETag || etag } });
    }
    return { fetch, requests, gets: () => requests.filter(request => request.method === 'GET') };
}

/** Cache Storage: put consumes the body like a browser, and may run out of quota part of the way. */
class FakeCaches {
    constructor({ quotaBytes, openError } = {}) {
        this.stores = new Map();
        this.quotaBytes = quotaBytes;
        this.openError = openError;
    }
    async open(name) {
        if (this.openError) throw this.openError;
        if (!this.stores.has(name)) this.stores.set(name, new FakeCache(this));
        return this.stores.get(name);
    }
    urls(name) { return [...(this.stores.get(name)?.entries.keys() || [])]; }
}
class FakeCache {
    constructor(owner) { this.owner = owner; this.entries = new Map(); }
    async match(key) {
        const entry = this.entries.get(String(key));
        if (!entry) return undefined;
        return new Response(entry.broken ? chunked(entry.bytes, 5000, 5000) : chunked(entry.bytes, 5000), { headers: entry.headers });
    }
    async put(key, response) {
        const reader = response.body.getReader();
        const parts = [];
        let total = 0;
        for (;;) {
            const { done, value } = await reader.read();
            if (done) break;
            total += value.byteLength;
            if (this.owner.quotaBytes !== undefined && total > this.owner.quotaBytes) {
                await reader.cancel();
                throw new DOMException('The quota has been exceeded.', 'QuotaExceededError');
            }
            parts.push(value);
        }
        const bytes = new Uint8Array(total);
        let offset = 0;
        for (const part of parts) { bytes.set(part, offset); offset += part.byteLength; }
        this.entries.set(String(key), { bytes, headers: Object.fromEntries(response.headers) });
    }
    async delete(key) { return this.entries.delete(typeof key === 'string' ? key : key.url); }
    async keys() { return [...this.entries.keys()].map(url => new Request(url)); }
}

function storage({ quota = 10e9, usage = 0 } = {}) {
    const calls = { persist: 0 };
    return { calls, estimate: async () => ({ quota, usage }), persisted: async () => false,
        persist: async () => { calls.persist++; return true; } };
}

/** The page: its Module, status text, progress bar and the template emcc filled. */
function page(folder = 'original', scripts = [[['async', ''], ['type', 'text/javascript'], ['src', 'uqm-debug.js']]]) {
    const appended = [];
    const statuses = [];
    const logs = [];
    const module = {
        setStatus(text) {
            statuses.push(text);
            // The shells' own handler.
            const download = help.formatProgress(text);
            if (download) { this.byteProgress = true; progress.max = download.total; progress.value = download.done; }
            status.textContent = download ? download.label : text;
        },
        printErr: message => logs.push(message),
        onAbort: reason => logs.push('abort: ' + reason),
    };
    const status = { textContent: 'Loading…' };
    const progress = { max: 1, value: 0 };
    const inert = scripts.map(attributes => ({
        attributes: attributes.map(([name, value]) => ({ name, value })),
        hasAttribute: name => attributes.some(([attribute]) => attribute === name),
        textContent: 'inline();',
    }));
    const template = { content: { querySelectorAll: selector => (selector === 'script' ? inert : []) } };
    const document = {
        baseURI: `${ORIGIN}/${folder}/index.html`,
        body: { appendChild(element) {
            // The data must be ready before the engine script runs.
            element.preloadedReady = typeof module.getPreloadedPackage === 'function';
            appended.push(element);
        } },
        createElement: tag => ({ tag, attributes: {}, text: undefined, setAttribute(name, value) { this.attributes[name] = value; } }),
    };
    return { module, status, progress, template, document, appended, statuses, logs };
}

async function run(view, net, caches, store = storage(), extra = {}) {
    const result = await loader.start({ module: view.module, edition: extra.edition || 'original', template: view.template,
        status: view.status, progress: view.progress, document: view.document, fetch: net.fetch, caches, storage: store, ...extra });
    return result;
}

const DATA = `${ORIGIN}/original/uqm-debug.data`;
const KEY = etag => `${ORIGIN}/.uqm-game-data/original/${encodeURIComponent(etag)}`;

test('first visit downloads with progress, stores the copy and hands one buffer to Emscripten', async () => {
    const bytes = content(100000);
    const net = server(bytes);
    const caches = new FakeCaches();
    const store = storage();
    const view = page();
    const result = await run(view, net, caches, store);
    assert.equal(result.source, 'download');
    assert.equal(result.stored, true);
    assert.equal(result.etag, '"v1"');
    assert.equal(result.size, bytes.length);
    assert.deepEqual(net.requests.map(request => [request.method, request.url, request.cache]),
        [['HEAD', DATA, 'no-store'], ['GET', DATA, 'no-store']]);
    assert.deepEqual(caches.urls('uqm-game-data-original'), [KEY('"v1"')]);
    // Download progress uses Emscripten's wording, which the page formats with UQMHelp.formatProgress.
    const downloads = view.statuses.filter(text => help.formatProgress(text));
    assert.ok(downloads.length >= 2, view.statuses.join('\n'));
    assert.equal(downloads.at(-1), `Downloading data... (${bytes.length}/${bytes.length})`);
    assert.equal(view.module.byteProgress, true);
    assert.equal(view.progress.max, bytes.length);
    assert.equal(view.progress.value, bytes.length);
    assert.equal(view.status.textContent, 'Starting the game…');
    assert.equal(store.calls.persist, 1, 'persistent storage is requested once the copy is stored');
    // The engine script starts only now, with the data ready.
    assert.equal(view.appended.length, 1);
    assert.deepEqual(view.appended[0].attributes, { async: '', type: 'text/javascript', src: 'uqm-debug.js' });
    assert.equal(view.appended[0].text, undefined, 'an external script gets no inline text');
    assert.equal(view.appended[0].preloadedReady, true);
    const buffer = view.module.getPreloadedPackage('uqm-debug.data', bytes.length);
    assert.ok(buffer instanceof ArrayBuffer);
    assert.deepEqual(new Uint8Array(buffer), bytes);
    assert.equal(view.module.getPreloadedPackage('uqm-debug.data', bytes.length), null, 'the loader keeps no second reference');
    assert.equal(view.module.gameData.source, 'download');
    assert.ok(view.logs.some(line => /downloaded 0 MB and saved it/.test(line)), view.logs.join('\n'));
});

test('a return visit reads the stored copy after one HEAD, and the Rust page shares it', async () => {
    const bytes = content(150000, 3);
    const net = server(bytes);
    const caches = new FakeCaches();
    await run(page(), net, caches);
    for (const folder of ['original', 'original-rust']) {
        net.requests.length = 0;
        const view = page(folder);
        const result = await run(view, net, caches);
        assert.equal(result.source, 'cache', folder);
        assert.deepEqual(net.requests.map(request => request.method), ['HEAD'], folder + ': no data body is transferred');
        assert.equal(net.requests[0].url, `${ORIGIN}/${folder}/uqm-debug.data`);
        assert.deepEqual(new Uint8Array(view.module.getPreloadedPackage('uqm-debug.data', bytes.length)), bytes);
        assert.match(view.status.textContent, /Starting the game/);
        assert.ok(view.statuses.every(text => !help.formatProgress(text)), 'a stored copy is not reported as a download');
        assert.ok(result.timings.read >= result.timings.match);
    }
    assert.deepEqual(caches.urls('uqm-game-data-original'), [KEY('"v1"')], 'one stored copy for both pages');
});

test('reading the stored copy shows its own progress text on the page bar', async () => {
    const bytes = content(2_000_000, 5);
    const net = server(bytes);
    const caches = new FakeCaches();
    await run(page(), net, caches);
    const view = page();
    const seen = [];
    Object.defineProperty(view.status, 'textContent', { get() { return seen.at(-1); }, set(value) { seen.push(value); } });
    await run(view, net, caches);
    assert.ok(seen.some(text => text === 'Loading game data saved in this browser: 2 / 2 MB (100%)'), seen.join('\n'));
    assert.equal(view.progress.max, bytes.length);
    assert.equal(view.progress.value, bytes.length);
    assert.equal(loader.storedLabel(154409693, 298763132), 'Loading game data saved in this browser: 154 / 299 MB (51%)');
});

test('a new ETag replaces the old copy of that edition and leaves the other edition alone', async () => {
    const caches = new FakeCaches();
    const hd = content(20000, 9);
    await run(page('hd'), server(hd, { etag: '"hd1"' }), caches, storage(), { edition: 'hd' });
    await run(page(), server(content(50000, 1), { etag: '"v1"' }), caches);
    const fresh = content(60000, 2);
    const net = server(fresh, { etag: '"v2"' });
    const view = page();
    const result = await run(view, net, caches);
    assert.equal(result.source, 'download');
    assert.equal(result.removed, 1);
    assert.deepEqual(caches.urls('uqm-game-data-original'), [KEY('"v2"')]);
    assert.deepEqual(caches.urls('uqm-game-data-hd'), [`${ORIGIN}/.uqm-game-data/hd/${encodeURIComponent('"hd1"')}`]);
    assert.deepEqual(new Uint8Array(view.module.getPreloadedPackage('uqm-debug.data', fresh.length)), fresh);
});

test('a weak HEAD validator matches the strong ETag of the stored copy', async () => {
    const bytes = content(30000);
    const caches = new FakeCaches();
    await run(page(), server(bytes, { etag: '"abc"' }), caches);
    const net = server(bytes, { etag: 'W/"abc"' });
    assert.equal((await run(page(), net, caches)).source, 'cache');
    assert.equal(loader.normalizeETag('W/"abc"'), '"abc"');
    assert.equal(loader.normalizeETag('"c662a266"'), '"c662a266"');
    for (const value of [null, '', '  ', '"has space"', 'W/']) assert.equal(loader.normalizeETag(value), null, String(value));
});

test('a deploy between the HEAD and the download stores the bytes under the version received', async () => {
    const bytes = content(40000);
    const caches = new FakeCaches();
    const result = await run(page(), server(bytes, { etag: '"old"', getETag: '"new"' }), caches);
    assert.equal(result.etag, '"new"');
    assert.deepEqual(caches.urls('uqm-game-data-original'), [KEY('"new"')]);
});

/** Today's path: no package from the loader, the engine script starts and downloads the file itself. */
function assertNormalPath(view, result, reason) {
    assert.equal(result.source, 'network');
    assert.match(result.reason, reason);
    assert.equal(view.module.getPreloadedPackage, undefined);
    assert.equal(view.appended.length, 1, 'the engine still starts');
    assert.equal(view.appended[0].attributes.src, 'uqm-debug.js');
    assert.ok(view.logs.some(line => line.startsWith('Game data: downloading it the usual way')), view.logs.join('\n'));
}

test('without Cache Storage, a HEAD answer, an ETag or a size the engine downloads the data as before', async () => {
    const bytes = content(1000);
    const cases = [
        ['no Cache API', server(bytes), null, /Cache Storage is unavailable/],
        ['HEAD fails', server(bytes, { headThrows: true }), new FakeCaches(), /version check failed/],
        ['HEAD refused', server(bytes, { headStatus: 405 }), new FakeCaches(), /HTTP 405/],
        ['no ETag', server(bytes, { noETag: true }), new FakeCaches(), /no ETag/],
        ['private mode', server(bytes), new FakeCaches({ openError: new DOMException('denied', 'SecurityError') }), /Cache Storage is unavailable: denied/],
    ];
    for (const [name, net, caches, reason] of cases) {
        const view = page();
        const result = await run(view, net, caches);
        assertNormalPath(view, result, reason);
        assert.deepEqual(net.gets(), [], name + ': the loader downloads nothing');
    }
});

test('too little browser storage leaves the download to the engine', async () => {
    const bytes = content(1000);
    const net = server(bytes);
    const view = page();
    const result = await run(view, net, new FakeCaches(), storage({ quota: 100e6, usage: 90e6 }));
    assertNormalPath(view, result, /not enough browser storage/);
    assert.deepEqual(net.gets(), []);
});

test('a quota error part of the way stores nothing and the engine starts the normal way', async () => {
    const bytes = content(100000);
    const net = server(bytes);
    const caches = new FakeCaches({ quotaBytes: 30000 });
    const store = storage();
    const view = page();
    const result = await run(view, net, caches, store);
    assertNormalPath(view, result, /could not be stored: The quota has been exceeded/);
    assert.deepEqual(caches.urls('uqm-game-data-original'), []);
    assert.equal(store.calls.persist, 0);
});

test('a broken download stores nothing and the engine starts the normal way', async () => {
    const bytes = content(100000);
    const caches = new FakeCaches();
    const view = page();
    const result = await run(view, server(bytes, { failAfter: 40000 }), caches);
    assertNormalPath(view, result, /could not be stored: network connection lost/);
    assert.deepEqual(caches.urls('uqm-game-data-original'), []);
});

test('a stored copy of the wrong size is downloaded again; an unreadable one is removed', async () => {
    const bytes = content(20000);
    const caches = new FakeCaches();
    await run(page(), server(bytes), caches);
    const cache = await caches.open('uqm-game-data-original');
    cache.entries.get(KEY('"v1"')).headers['x-uqm-size'] = '19999';
    const net = server(bytes);
    const again = await run(page(), net, caches);
    assert.equal(again.source, 'download');
    assert.equal(net.gets().length, 1);
    cache.entries.get(KEY('"v1"')).broken = true;
    const view = page();
    assertNormalPath(view, await run(view, server(bytes), caches), /could not be read: network connection lost/);
    assert.deepEqual(caches.urls('uqm-game-data-original'), [], 'the next visit stores a fresh copy');
});

test('the package goes only to the matching file and size, and is released either way', async () => {
    const bytes = content(5000);
    const caches = new FakeCaches();
    await run(page(), server(bytes), caches);
    const view = page();
    await run(view, server(bytes), caches);
    assert.equal(view.module.getPreloadedPackage('uqm-debug.data', bytes.length + 1), null);
    assert.equal(view.module.getPreloadedPackage('uqm-debug.data', bytes.length), null, 'released after a mismatch');
    assert.equal(view.module.gameData.source, 'network');
    const other = page();
    await run(other, server(bytes), caches);
    assert.equal(other.module.getPreloadedPackage('other.data', bytes.length), null);
});

test('a body that arrives as one whole chunk (WebKit) is kept as is, small chunks fill one exact buffer', async () => {
    const whole = content(70000);
    const one = new ReadableStream({ start(controller) { controller.enqueue(whole); controller.close(); } });
    const seen = [];
    const buffer = await loader.readBody({ body: one }, whole.length, (done, total) => seen.push([done, total]));
    assert.equal(buffer, whole.buffer, 'no second copy of a whole-body chunk');
    assert.deepEqual(seen, [[70000, 70000]]);
    const parts = await loader.readBody({ body: chunked(whole, 3000) }, whole.length, () => {});
    assert.notEqual(parts, whole.buffer);
    assert.equal(parts.byteLength, whole.length);
    assert.deepEqual(new Uint8Array(parts), whole);
    await assert.rejects(loader.readBody({ body: chunked(whole, 3000) }, whole.length - 1, () => {}), /larger than/);
    await assert.rejects(loader.readBody({ body: chunked(whole, 3000) }, whole.length + 1, () => {}), /70000 of 70001 bytes/);
});

test('several emcc scripts keep their order; inline text is copied', async () => {
    const view = page('original', [[['src', 'a.js']], [['type', 'text/javascript']]]);
    await run(view, server(content(10)), null);
    assert.equal(view.appended.length, 2);
    assert.deepEqual(view.appended.map(script => [script.attributes, script.async, script.text]),
        [[{ src: 'a.js' }, false, undefined], [{ type: 'text/javascript' }, false, 'inline();']]);
});

test('a page without its engine template reports the failure instead of waiting', async () => {
    const view = page('original', []);
    const result = await run(view, server(content(10)), null);
    assert.equal(result.source, 'error');
    assert.ok(view.logs.includes('abort: The page has no engine script.'), view.logs.join('\n'));
});

// The shells: emcc's {{{ SCRIPT }}} is inert until the loader starts it.
for (const [base, edition] of [['sc2/wasm', 'original'], ['hd/wasm', 'hd']]) {
    test(`${edition} shell keeps the engine script in a template and starts it through the loader`, () => {
        const html = fs.readFileSync(base + '/shell.html', 'utf8');
        assert.equal(html.split('{{{ SCRIPT }}}').length, 2, 'one placeholder');
        assert.ok(html.includes('<template id="engine-script">{{{ SCRIPT }}}</template>'));
        const tail = html.slice(html.indexOf('<template id="engine-script">'));
        assert.ok(tail.includes('<script src="engine-loader.js"></script>'));
        assert.ok(html.lastIndexOf('</script>', html.indexOf('<template id="engine-script">')) > html.indexOf('var Module'),
            'the template comes after the page script that defines Module');
        const call = tail.match(/<script>\s*([\s\S]*?)<\/script>/)[1];
        // Run the call with the loader present, then without it.
        const template = { content: { querySelectorAll: () => [{ getAttribute: name => (name === 'src' ? 'uqm-debug.js' : null) }] } };
        const elements = { 'engine-script': template, status: { id: 'status' }, progress: { id: 'progress' } };
        const appended = [];
        const document = { getElementById: id => elements[id] || null, createElement: () => ({}), body: { appendChild: node => appended.push(node) } };
        const calls = [];
        const context = { document, Module: { name: 'module' }, UQMEngineLoader: { start: options => calls.push(options) } };
        context.window = context;
        vm.runInNewContext(call, context);
        assert.equal(calls.length, 1);
        assert.equal(calls[0].module, context.Module);
        assert.equal(calls[0].edition, edition);
        assert.equal(calls[0].template, template);
        assert.equal(calls[0].status, elements.status);
        assert.equal(calls[0].progress, elements.progress);
        assert.deepEqual(appended, []);
        const bare = { document, Module: {} };
        bare.window = bare;
        vm.runInNewContext(call, bare);
        assert.deepEqual(appended, [{ src: 'uqm-debug.js' }], 'without the loader the engine starts as before');
    });
}

test('every build and the package publish the loader next to the page', () => {
    for (const script of ['scripts/build-web.sh', 'scripts/build-hd-web.sh']) {
        assert.ok(fs.readFileSync(script, 'utf8').includes('cp "$root/web/engine-loader/engine-loader.js" "$output/engine-loader.js"'), script);
    }
    assert.ok(fs.readFileSync('scripts/rust-engine/link.py', 'utf8').includes("root / 'web/engine-loader/engine-loader.js', output / 'engine-loader.js'"));
    assert.ok(fs.readFileSync('scripts/package-editions.py', 'utf8').includes("'engine-loader.js'"));
});
