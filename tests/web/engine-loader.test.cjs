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

/** The Worker: HEAD answers the ETag and size, GET streams the bytes; other files are the engine's. */
function server(bytes, { etag = '"v1"', headStatus = 200, headThrows = false, headHangs = false, noETag = false, getETag, failAfter } = {}) {
    const requests = [];
    const others = [];
    async function fetch(url, init = {}) {
        const method = init.method || 'GET';
        if (!String(url).endsWith('/uqm-debug.data')) {
            // The engine script and its Wasm: the loader reads them to the end and keeps nothing.
            const file = { url: String(url), method, credentials: init.credentials, read: false };
            others.push(file);
            const body = content(10000);
            return new Response(new ReadableStream({ pull(controller) {
                if (file.offset >= body.length) { file.read = true; return controller.close(); }
                controller.enqueue(body.slice(file.offset || 0, (file.offset || 0) + 4000));
                file.offset = (file.offset || 0) + 4000;
            } }));
        }
        requests.push({ url: String(url), method, cache: init.cache, signal: init.signal });
        if (method === 'HEAD') {
            if (headThrows) throw new TypeError('Failed to fetch');
            if (headHangs) return new Promise(() => {});
            const headers = { 'Content-Length': String(bytes.length) };
            if (!noETag) headers.ETag = etag;
            return new Response(null, { status: headStatus, headers });
        }
        return new Response(chunked(bytes, 4096, failAfter), { headers: {
            'Content-Length': String(bytes.length), ETag: getETag || etag } });
    }
    return { fetch, requests, others, gets: () => requests.filter(request => request.method === 'GET') };
}

/** Cache Storage: put consumes the body like a browser, and may run out of quota part of the way. */
class FakeCaches {
    constructor({ quotaBytes, openError, hang = {} } = {}) {
        this.stores = new Map();
        this.quotaBytes = quotaBytes;
        this.openError = openError;
        // Calls that never answer: open, match, read (the stored body) or put (after the body).
        this.hang = hang;
    }
    async open(name) {
        if (this.openError) throw this.openError;
        if (this.hang.open) return new Promise(() => {});
        if (!this.stores.has(name)) this.stores.set(name, new FakeCache(this));
        return this.stores.get(name);
    }
    async keys() { return [...this.stores.keys()]; }
    async delete(name) { return this.stores.delete(name); }
    urls(name) { return [...(this.stores.get(name)?.entries.keys() || [])]; }
}
class FakeCache {
    constructor(owner) { this.owner = owner; this.entries = new Map(); }
    async match(key) {
        if (this.owner.hang.match) return new Promise(() => {});
        const entry = this.entries.get(String(key));
        if (!entry) return undefined;
        const body = this.owner.hang.read ? new ReadableStream({ pull: () => new Promise(() => {}) })
            : entry.broken ? chunked(entry.bytes, 5000, 5000) : chunked(entry.bytes, 5000);
        return new Response(body, { headers: entry.headers });
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
        if (this.owner.hang.put) return new Promise(() => {});
        const bytes = new Uint8Array(total);
        let offset = 0;
        for (const part of parts) { bytes.set(part, offset); offset += part.byteLength; }
        this.entries.set(String(key), { bytes, headers: Object.fromEntries(response.headers) });
    }
    async delete(key) { return this.entries.delete(typeof key === 'string' ? key : key.url); }
    async keys() { return [...this.entries.keys()].map(url => new Request(url)); }
}

function storage({ quota = 10e9, usage = 0, persisted = false } = {}) {
    const calls = { persist: 0 };
    return { calls, estimate: async () => ({ quota, usage }), persisted: async () => persisted,
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
        getAttribute: name => (attributes.find(([attribute]) => attribute === name) || [])[1] ?? null,
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
    const events = [];
    return { module, status, progress, template, document, appended, statuses, logs, events };
}

/** WebAssembly: compileStreaming waits for its response; failures on request. */
function fakeWasm({ compileFails = false, instantiateFails = false, streamingFails = false } = {}) {
    const calls = [];
    return { calls,
        async compileStreaming(source) {
            const response = await source;
            calls.push(['compile', response.ok]);
            if (compileFails) throw new TypeError('not a Wasm file');
            return { compiled: true };
        },
        async instantiate(compiled, imports) {
            calls.push(['instantiate', compiled, imports]);
            if (instantiateFails) throw new Error('import mismatch');
            return { instance: true };
        },
        async instantiateStreaming(source, imports) {
            await source;
            calls.push(['instantiateStreaming', imports]);
            if (streamingFails) throw new Error('offline');
            return { instance: { streamed: true }, module: { streamed: true } };
        } };
}

/** Runs the loader as a shell does. extra.onProgress: null leaves the progress to Module.setStatus. */
async function run(view, net, caches, store = storage(), extra = {}) {
    const result = await loader.start({ module: view.module, edition: 'original', template: view.template,
        onProgress: event => view.events.push(event), formatProgress: help.formatProgress, WebAssembly: fakeWasm(),
        document: view.document, fetch: net.fetch, caches, storage: store, ...extra });
    return result;
}

/** What Emscripten does with Module.instantiateWasm: call it and wait for receiveInstance. */
function instantiate(module, imports = { env: {} }) {
    return new Promise((resolve, reject) => {
        const returned = module.instantiateWasm(imports, (instance, compiled) => resolve({ instance, compiled, returned }));
        if (!returned || typeof returned !== 'object') reject(new Error('instantiateWasm must return an object'));
        setTimeout(() => reject(new Error('receiveInstance was not called')), 1000);
    });
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
    // Download progress uses the page's own wording (UQMHelp.formatProgress), then the read-back, then the start.
    const downloads = view.events.filter(event => event.phase === 'download');
    assert.ok(downloads.length >= 2, JSON.stringify(view.events));
    assert.deepEqual(downloads.at(-1), { phase: 'download', done: bytes.length, total: bytes.length, fraction: 1,
        label: help.formatProgress(`(${bytes.length}/${bytes.length})`).label });
    assert.equal(view.events.filter(event => event.phase === 'stored').at(-1).label, 'Loading game data saved in this browser: 0 / 0 MB (100%)');
    assert.deepEqual(view.events.at(-1), { phase: 'starting', label: 'Starting the game…' });
    assert.equal(view.module.byteProgress, true);
    assert.deepEqual(view.statuses, [], 'with a display callback the loader leaves Module.setStatus alone');
    assert.equal(store.calls.persist, 1, 'persistent storage is requested once the copy is stored');
    // During the download the engine script went into the HTTP cache and the Wasm was fetched for compiling.
    assert.deepEqual(net.others.map(file => [file.method, file.url, file.credentials]).sort(), [
        ['GET', `${ORIGIN}/original/uqm-debug.js`, 'same-origin'], ['GET', `${ORIGIN}/original/uqm-debug.wasm`, 'same-origin']]);
    assert.equal(net.others.find(file => file.url.endsWith('.js')).read, true, 'the script is read to the end');
    // The engine script starts only now, with the data ready.
    assert.equal(view.appended.length, 1);
    assert.deepEqual(view.appended[0].attributes, { async: '', type: 'text/javascript', src: 'uqm-debug.js' });
    assert.equal(view.appended[0].text, undefined, 'an external script gets no inline text');
    assert.equal(view.appended[0].preloadedReady, true);
    const buffer = view.module.getPreloadedPackage('uqm-debug.data', bytes.length);
    assert.ok(buffer instanceof ArrayBuffer);
    assert.deepEqual(new Uint8Array(buffer), bytes);
    assert.equal(view.module.getPreloadedPackage('uqm-debug.data', bytes.length), null, 'the loader keeps no second reference');
    assert.equal(view.module.gameData, result);
    assert.equal(view.module.gameData.source, 'download');
    // The steps: where each phase began and ended.
    assert.deepEqual(result.steps.map(step => step.phase), ['download', 'download', 'stored', 'stored', 'starting']);
    assert.ok(view.logs.some(line => /downloaded 0 MB and saved it/.test(line)), view.logs.join('\n'));
});

test('Module.gameData shows the load while it runs and never holds the data', async () => {
    const bytes = content(300000);
    const caches = new FakeCaches();
    await run(page(), server(bytes), caches);
    const view = page();
    const net = server(bytes);
    const seen = [];
    const running = run(view, net, caches, storage(), { onProgress: event => seen.push([event.phase, view.module.gameData.loading === event]) });
    assert.equal(view.module.gameData.source, 'pending', 'readable before the HEAD answers');
    const result = await running;
    assert.ok(seen.length >= 2 && seen.every(([, same]) => same), JSON.stringify(seen));
    assert.ok(JSON.stringify(result).length < 4000, 'no bytes in the description');
    assert.deepEqual(result.steps.map(step => step.phase), ['stored', 'stored', 'starting']);
    assert.equal(result.steps[1].label, 'Loading game data saved in this browser: 0 / 0 MB (100%)');
});

test('without a display callback, progress goes to Module.setStatus as the page formats it', async () => {
    const bytes = content(100000);
    const caches = new FakeCaches();
    const view = page();
    await run(view, server(bytes), caches, storage(), { onProgress: null });
    const downloads = view.statuses.filter(text => help.formatProgress(text));
    assert.ok(downloads.length >= 2, view.statuses.join('\n'));
    assert.equal(downloads.at(-1), `Downloading data... (${bytes.length}/${bytes.length})`);
    assert.ok(view.statuses.includes('Loading game data saved in this browser: 0 / 0 MB (100%)'), view.statuses.join('\n'));
    assert.equal(view.progress.max, bytes.length, 'the page drew its own bar for the download');
    assert.equal(view.status.textContent, 'Starting the game…');
    // A display that throws does not stop the game from loading.
    const broken = page();
    const result = await run(broken, server(bytes), caches, storage(), { onProgress: () => { throw new Error('display gone'); } });
    assert.equal(result.source, 'cache');
    assert.equal(broken.appended.length, 1);
});

test('a return visit reads the stored copy after one HEAD, and the Rust page shares it', async () => {
    const bytes = content(150000, 3);
    const net = server(bytes);
    const caches = new FakeCaches();
    await run(page(), net, caches);
    for (const folder of ['original', 'original-rust']) {
        net.requests.length = 0;
        net.others.length = 0;
        const view = page(folder);
        const result = await run(view, net, caches);
        assert.equal(result.source, 'cache', folder);
        assert.deepEqual(net.requests.map(request => request.method), ['HEAD'], folder + ': no data body is transferred');
        assert.equal(net.requests[0].url, `${ORIGIN}/${folder}/uqm-debug.data`);
        assert.deepEqual(net.others.map(file => file.url), [`${ORIGIN}/${folder}/uqm-debug.wasm`], 'only the Wasm, to compile it early');
        assert.deepEqual(new Uint8Array(view.module.getPreloadedPackage('uqm-debug.data', bytes.length)), bytes);
        assert.equal(view.events.at(-1).label, 'Starting the game…');
        assert.ok(view.events.every(event => event.phase !== 'download'), 'a stored copy is not reported as a download');
        assert.ok(result.timings.read >= result.timings.match);
    }
    assert.deepEqual(caches.urls('uqm-game-data-original'), [KEY('"v1"')], 'one stored copy for both pages');
});

test('reading the stored copy reports its own progress text and fraction', async () => {
    const bytes = content(2_000_000, 5);
    const net = server(bytes);
    const caches = new FakeCaches();
    await run(page(), net, caches);
    const view = page();
    await run(view, net, caches);
    const stored = view.events.filter(event => event.phase === 'stored');
    assert.ok(stored.length >= 2, JSON.stringify(stored));
    assert.deepEqual(stored.at(-1), { phase: 'stored', done: bytes.length, total: bytes.length, fraction: 1,
        label: 'Loading game data saved in this browser: 2 / 2 MB (100%)' });
    assert.ok(stored.every((event, i) => !i || event.done > stored[i - 1].done));
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

test('stored copies can be listed and removed per edition, and saved games stay', async () => {
    const caches = new FakeCaches();
    await run(page('hd'), server(content(2000), { etag: '"hd1"' }), caches, storage(), { edition: 'hd' });
    await run(page(), server(content(3000)), caches);
    (await caches.open('uqm-saves-other')).entries.set(`${ORIGIN}/save`, { bytes: new Uint8Array(1), headers: {} });
    assert.deepEqual(await loader.storedCopies({ caches }), [{ edition: 'hd', etag: '"hd1"' }, { edition: 'original', etag: '"v1"' }]);
    assert.equal(await loader.removeStoredData('hd', { caches }), 1);
    assert.deepEqual(await caches.keys(), ['uqm-game-data-original', 'uqm-saves-other']);
    assert.equal(await loader.removeStoredData(undefined, { caches }), 1);
    assert.deepEqual(await caches.keys(), ['uqm-saves-other'], 'only the game data caches are removed');
    assert.equal(await loader.removeStoredData('hd', { caches: null }), 0);
    assert.deepEqual(await loader.storedCopies({ caches: null }), []);
});

test('the Wasm compiles while the data loads and Emscripten gets the compiled module', async () => {
    const bytes = content(50000);
    const caches = new FakeCaches();
    for (const visit of ['first', 'return']) {
        const view = page();
        const wasm = fakeWasm();
        const result = await run(view, server(bytes), caches, storage(), { WebAssembly: wasm });
        assert.equal(result.source, visit === 'first' ? 'download' : 'cache');
        assert.equal(typeof view.module.instantiateWasm, 'function', visit);
        const imports = { env: { tag: visit } };
        const got = await instantiate(view.module, imports);
        assert.deepEqual(got, { instance: { instance: true }, compiled: { compiled: true }, returned: {} });
        assert.deepEqual(wasm.calls, [['compile', true], ['instantiate', { compiled: true }, imports]]);
        assert.equal(result.wasm.state, 'compiled');
        assert.equal(result.wasm.used, true);
        assert.equal(result.wasm.url, `${ORIGIN}/original/uqm-debug.wasm`);
    }
});

test('a Wasm that fails to compile early is loaded the usual way; a page hook is left alone', async () => {
    const bytes = content(5000);
    const caches = new FakeCaches();
    for (const failure of [{ compileFails: true }, { instantiateFails: true }]) {
        const view = page();
        const wasm = fakeWasm(failure);
        const result = await run(view, server(bytes), caches, storage(), { WebAssembly: wasm });
        const got = await instantiate(view.module);
        assert.deepEqual(got.instance, { streamed: true });
        assert.deepEqual(got.compiled, { streamed: true });
        assert.equal(result.wasm.state, 'reloaded');
        assert.ok(view.logs.some(line => /could not start \((not a Wasm file|import mismatch)\); loading it again/.test(line)), view.logs.join('\n'));
    }
    // Neither way works: the page shows that the game could not start.
    const view = page();
    await run(view, server(bytes), caches, storage(), { WebAssembly: fakeWasm({ compileFails: true, streamingFails: true }) });
    view.module.instantiateWasm({}, () => assert.fail('no instance'));
    await new Promise(resolve => setTimeout(resolve, 20));
    assert.ok(view.logs.includes('abort: The game engine could not start: offline'), view.logs.join('\n'));
    // The page's own hook, and browsers without streaming compilation, keep Emscripten's way.
    const own = page();
    const hook = () => ({});
    own.module.instantiateWasm = hook;
    await run(own, server(bytes), caches);
    assert.equal(own.module.instantiateWasm, hook);
    const old = page();
    await run(old, server(bytes), caches, storage(), { WebAssembly: { instantiate() {} } });
    assert.equal(old.module.instantiateWasm, undefined);
});

/** Today's path: no package from the loader, the engine script starts and downloads the file itself. */
function assertNormalPath(view, result, reason, { compiledEarly = false } = {}) {
    assert.equal(result.source, 'network');
    assert.match(result.reason, reason);
    assert.equal(view.module.getPreloadedPackage, undefined);
    // A failure after the data was found or the download began keeps the Wasm compiled meanwhile.
    assert.equal(typeof view.module.instantiateWasm, compiledEarly ? 'function' : 'undefined');
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
    const result = await run(view, net, new FakeCaches(), storage({ quota: 20e9, usage: 19.95e9 }));
    assertNormalPath(view, result, /not enough browser storage/);
    assert.deepEqual(net.gets(), []);
    assert.deepEqual(net.others, [], 'no engine files are fetched early either');
});

test('storage that looks temporary (a private window) is not used; granted persistence is', async () => {
    const bytes = content(1000);
    // Chrome incognito reports 5 GiB, Safari private browsing 1000 MiB; both keep Cache Storage in memory.
    for (const quota of [5 * 2 ** 30, 1000 * 2 ** 20]) {
        const caches = new FakeCaches();
        const net = server(bytes);
        const view = page();
        assertNormalPath(view, await run(view, net, caches, storage({ quota })), /storage looks temporary here/);
        assert.deepEqual(net.gets(), []);
        assert.deepEqual(caches.urls('uqm-game-data-original'), []);
    }
    const caches = new FakeCaches();
    assert.equal((await run(page(), server(bytes), caches, storage({ quota: 5 * 2 ** 30, persisted: true }))).source, 'download');
    // A normal profile reports 10 GiB or more.
    assert.equal((await run(page(), server(bytes, { etag: '"v2"' }), caches, storage({ quota: 10 * 2 ** 30 }))).source, 'download');
    assert.ok(loader.TEMPORARY_QUOTA > 5 * 2 ** 30 && loader.TEMPORARY_QUOTA < 10 * 2 ** 30);
});

test('a version check or Cache Storage call that never answers gives way to the normal path in time', async () => {
    const bytes = content(20000);
    const limits = { check: 40, read: 40, store: 40 };
    const hung = server(bytes, { headHangs: true });
    let view = page();
    const begin = Date.now();
    assertNormalPath(view, await run(view, hung, new FakeCaches(), storage(), { limits, AbortController }),
        /version check failed: The version check did not answer within 0 s/);
    assert.ok(Date.now() - begin < 2000);
    assert.equal(hung.requests[0].signal.aborted, true, 'the stalled HEAD is cancelled');
    for (const [hang, reason] of [['open', /Cache Storage is unavailable: Cache Storage did not answer/],
        ['match', /Cache Storage is unavailable: Cache Storage did not answer/]]) {
        view = page();
        assertNormalPath(view, await run(view, server(bytes), new FakeCaches({ hang: { [hang]: true } }), storage(), { limits }), reason);
    }
    // Every byte arrived but the store never finishes.
    view = page();
    assertNormalPath(view, await run(view, server(bytes), new FakeCaches({ hang: { put: true } }), storage(), { limits }),
        /could not be stored: Storing the game data did not answer/, { compiledEarly: true });
    // The stored copy stops arriving part of the way: the copy is kept for the next visit.
    const caches = new FakeCaches();
    await run(page(), server(bytes), caches);
    caches.hang.read = true;
    view = page();
    assertNormalPath(view, await run(view, server(bytes), caches, storage(), { limits }),
        /could not be read: Reading the stored copy did not answer/, { compiledEarly: true });
    assert.deepEqual(caches.urls('uqm-game-data-original'), [KEY('"v1"')]);
    assert.deepEqual(loader.LIMITS, { check: 10000, read: 60000, store: 60000 });
});

test('a quota error part of the way stores nothing and the engine starts the normal way', async () => {
    const bytes = content(100000);
    const net = server(bytes);
    const caches = new FakeCaches({ quotaBytes: 30000 });
    const store = storage();
    const view = page();
    const result = await run(view, net, caches, store);
    assertNormalPath(view, result, /could not be stored: The quota has been exceeded/, { compiledEarly: true });
    assert.deepEqual(caches.urls('uqm-game-data-original'), []);
    assert.equal(store.calls.persist, 0);
});

test('a broken download stores nothing and the engine starts the normal way', async () => {
    const bytes = content(100000);
    const caches = new FakeCaches();
    const view = page();
    const result = await run(view, server(bytes, { failAfter: 40000 }), caches);
    assertNormalPath(view, result, /could not be stored: network connection lost/, { compiledEarly: true });
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
    assertNormalPath(view, await run(view, server(bytes), caches), /could not be read: network connection lost/, { compiledEarly: true });
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
        assert.ok(tail.includes('<script id="engine-loader" src="engine-loader.js"></script>'));
        assert.ok(html.lastIndexOf('</script>', html.indexOf('<template id="engine-script">')) > html.indexOf('var Module'),
            'the template comes after the page script that defines Module');
        const call = tail.match(/<script>\s*([\s\S]*?)<\/script>/)[1];
        // Run the call with the loader present, then without it.
        const template = { content: { querySelectorAll: () => [{ getAttribute: name => (name === 'src' ? 'uqm-debug.js' : null) }] } };
        const elements = { 'engine-script': template, status: { textContent: '' }, progress: { max: 1, value: 0 } };
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
        assert.equal(calls[0].status, undefined, 'the loader writes into no page element itself');
        assert.deepEqual(appended, []);
        // This page: the status line and progress bar.
        const show = calls[0].onProgress;
        show({ phase: 'stored', done: 25, total: 100, fraction: 0.25, label: 'Loading game data saved in this browser: 0 / 0 MB (25%)' });
        assert.deepEqual(elements.progress, { max: 100, value: 25 });
        assert.equal(elements.status.textContent, 'Loading game data saved in this browser: 0 / 0 MB (25%)');
        show({ phase: 'starting', label: 'Starting the game…' });
        assert.deepEqual(elements.progress, { max: 100, value: 25 }, 'a text step keeps the bar');
        assert.equal(elements.status.textContent, 'Starting the game…');
        // A page with the game menu (web/online) shows the progress there.
        const menu = [];
        context.online = { controller: { progress: loading => menu.push(loading) } };
        show({ phase: 'download', done: 50, total: 100, fraction: 0.5, label: 'Downloading game data: 0 / 0 MB (50%)' });
        show({ phase: 'starting', label: 'Starting the game…' });
        // The page's objects come from another realm: compare their contents.
        assert.deepEqual(JSON.parse(JSON.stringify(menu)), [{ label: 'Downloading game data: 0 / 0 MB (50%)', fraction: 0.5 }, { label: 'Starting the game…' }]);
        assert.equal(elements.status.textContent, 'Starting the game…');
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
