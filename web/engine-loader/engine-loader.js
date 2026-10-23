/* Game data loader shared by every engine page (Original and HD, C and Rust).

   The Emscripten data file (Original 299 MB, HD 663 MB) is too large for the
   browsers' HTTP caches, so every visit used to download it again. The page
   keeps the engine script in an inert <template>. This loader asks the server
   for the data file's ETag with a HEAD request, keeps one copy per edition and
   ETag in Cache Storage, and gives that copy to Emscripten through
   Module.getPreloadedPackage before it starts the engine script. The C and Rust
   pages of an edition package the same bytes, so they share one stored copy.

   Memory: a download streams into Cache Storage and is then read back into one
   ArrayBuffer of the exact size. Emscripten's file system keeps views of that
   buffer (canOwn), and the loader drops its own reference when it hands it over.

   Whenever Cache Storage, an ETag, the HEAD request or durable storage with
   enough quota is unavailable, or a step does not answer in time, the engine
   starts exactly as before and downloads the file itself.

   While the data downloads or is read, the engine's Wasm is compiled (Module.instantiateWasm),
   and on a first visit the engine script is fetched into the HTTP cache.

   Progress goes to options.onProgress({ phase, label, done, total, fraction }):
   phase 'download' (first visit), 'stored' (reading the stored copy) or
   'starting' (label only). Without onProgress it goes to Module.setStatus.
   Module.gameData describes the load while it runs and after it ends. */
(function (root, factory) {
    'use strict';
    const api = factory(root);
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMEngineLoader = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function (scope) {
    'use strict';
    const DATA_FILE = 'uqm-debug.data';
    const CACHE_PREFIX = 'uqm-game-data-';
    const KEY_PATH = '/.uqm-game-data/';
    // Room for the cache's own records next to the stored copy.
    const QUOTA_MARGIN = 64 * 1024 * 1024;
    // Private windows report a small quota (Chrome incognito 4-5 GiB on a 24 GB Mac, Safari
    // 1000 MiB) and keep Cache Storage in memory. Normal profiles of Chrome, Firefox and Safari
    // report 10 GiB or more, except on small disks.
    const TEMPORARY_QUOTA = 6 * 1024 * 1024 * 1024;
    const LIMITS = {
        // The HEAD request and each Cache Storage call before the engine starts.
        check: 10000,
        // Reading the stored copy: the longest wait for its next part.
        read: 60000,
        // Storing a download: the longest wait after its last byte arrived.
        store: 60000,
    };
    const PROGRESS_INTERVAL_MS = 100;
    const STEP_LIMIT = 16;
    const now = () => (typeof performance !== 'undefined' && performance.now ? performance.now() : Date.now());

    /** One version of the file. A weak validator still names one version. */
    function normalizeETag(value) {
        const tag = String(value == null ? '' : value).trim().replace(/^W\//, '');
        return /^"?[\x21\x23-\x7e]{1,256}"?$/.test(tag) ? tag : null;
    }

    function byteCount(value) {
        const text = String(value == null ? '' : value).trim();
        if (!/^\d{1,15}$/.test(text)) return 0;
        const count = Number(text);
        return Number.isSafeInteger(count) && count > 0 ? count : 0;
    }

    function cacheName(edition) {
        return CACHE_PREFIX + edition;
    }

    /** Cache Storage is per origin: the C and Rust pages of an edition use the same key. */
    function cacheKey(edition, etag, base) {
        return new URL(KEY_PATH + edition + '/' + encodeURIComponent(etag), base).href;
    }

    function megabytes(bytes) {
        return Math.round(bytes / 1e6);
    }

    function percent(done, total) {
        return Math.floor(done * 100 / total);
    }

    function storedLabel(done, total) {
        return 'Loading game data saved in this browser: ' + megabytes(done) + ' / ' + megabytes(total) +
            ' MB (' + percent(done, total) + '%)';
    }

    function message(error) {
        return String(error && error.message || error);
    }

    /** Rejects when `promise` has not settled after `ms`; the error names the step that stalled. */
    function deadline(promise, ms, what) {
        let timer;
        const late = new Promise((_, reject) => {
            timer = setTimeout(() => {
                const error = new Error(what + ' did not answer within ' + Math.round(ms / 1000) + ' s');
                error.timeout = true;
                reject(error);
            }, ms);
        });
        return Promise.race([Promise.resolve(promise), late]).finally(() => clearTimeout(timer));
    }

    function environment(options) {
        let caches = options.caches;
        if (caches === undefined) {
            // Some private modes throw on access instead of leaving the API out.
            try { caches = scope.caches || null; } catch (_) { caches = null; }
        }
        const document = options.document || scope.document;
        const help = scope.UQMHelp;
        return {
            fetch: options.fetch || (typeof scope.fetch === 'function' ? scope.fetch.bind(scope) : null),
            caches,
            storage: options.storage !== undefined ? options.storage
                : (scope.navigator && scope.navigator.storage) || null,
            document,
            base: options.base || (document && document.baseURI) || (scope.location && scope.location.href),
            TransformStream: options.TransformStream || scope.TransformStream,
            Response: options.Response || scope.Response,
            AbortController: options.AbortController || scope.AbortController || null,
            WebAssembly: options.WebAssembly !== undefined ? options.WebAssembly : scope.WebAssembly || null,
            formatProgress: options.formatProgress !== undefined ? options.formatProgress
                : (help && typeof help.formatProgress === 'function' ? help.formatProgress : null),
            limits: Object.assign({}, LIMITS, options.limits),
        };
    }

    /** Feeds the page's loading display and records the steps in Module.gameData. */
    function reporter(options, module, env, result) {
        let last = -Infinity;
        let lastPhase = null;
        function downloadLabel(done, total) {
            // The page's own wording for a download (UQMHelp.formatProgress), so both paths read alike.
            const formatted = env.formatProgress && env.formatProgress('(' + done + '/' + total + ')');
            return formatted && formatted.label ? formatted.label
                : 'Downloading game data: ' + megabytes(done) + ' / ' + megabytes(total) + ' MB (' + percent(done, total) + '%)';
        }
        function emit(event) {
            result.loading = event;
            if ((event.phase !== lastPhase || event.done === event.total) && result.steps.length < STEP_LIMIT) {
                result.steps.push({ phase: event.phase, label: event.label, at: Math.round(now()) });
            }
            lastPhase = event.phase;
            if (typeof options.onProgress === 'function') {
                try { options.onProgress(event); } catch (_) { /* The display is optional; loading goes on. */ }
                return;
            }
            if (typeof module.setStatus !== 'function') return;
            // Without a page callback: Emscripten's wording for a download, which the page formats itself.
            module.setStatus(event.phase === 'download' ? 'Downloading data... (' + event.done + '/' + event.total + ')' : event.label);
        }
        return {
            progress(phase, done, total) {
                const time = now();
                if (phase === lastPhase && done < total && time - last < PROGRESS_INTERVAL_MS) return;
                last = time;
                // Keep the dependency counter from taking over the byte progress bar.
                module.byteProgress = true;
                emit({ phase, done, total, fraction: done / total,
                    label: phase === 'download' ? downloadLabel(done, total) : storedLabel(done, total) });
            },
            text(phase, label) {
                emit({ phase, label });
            },
        };
    }

    /** Reads a response body into one ArrayBuffer of exactly `expected` bytes. */
    async function readBody(response, expected, onProgress, stallMs) {
        const wait = promise => (stallMs ? deadline(promise, stallMs, 'Reading the stored copy') : promise);
        if (!response.body || typeof response.body.getReader !== 'function') {
            const buffer = await wait(response.arrayBuffer());
            if (buffer.byteLength !== expected) throw new Error('the stored copy has ' + buffer.byteLength + ' of ' + expected + ' bytes');
            onProgress(expected, expected);
            return buffer;
        }
        const reader = response.body.getReader();
        let bytes = null;
        let whole = null;
        let offset = 0;
        try {
            for (;;) {
                const { done, value } = await wait(reader.read());
                if (done) break;
                if (offset + value.byteLength > expected) throw new Error('the stored copy is larger than ' + expected + ' bytes');
                if (!offset && value.byteLength === expected && value.byteOffset === 0 && value.buffer.byteLength === expected) {
                    // WebKit gives the whole body as one chunk: keep its buffer instead of copying it.
                    whole = value.buffer;
                } else {
                    // Other browsers give small chunks: copy them into one buffer of the exact size.
                    if (!bytes) bytes = new Uint8Array(expected);
                    bytes.set(value, offset);
                }
                offset += value.byteLength;
                onProgress(offset, expected);
            }
        } catch (error) {
            reader.cancel().catch(() => {});
            throw error;
        }
        if (offset !== expected) throw new Error('the stored copy has ' + offset + ' of ' + expected + ' bytes');
        return whole || bytes.buffer;
    }

    /** Streams the file from the server straight into Cache Storage, counting bytes for the progress bar. */
    async function downloadToCache(env, url, cache, edition, announced, onProgress) {
        const response = await env.fetch(url, { cache: 'no-store', credentials: 'same-origin' });
        if (!response.ok || !response.body) throw new Error('the download answered HTTP ' + response.status);
        // A deploy between the HEAD and this request: store the bytes under the version actually received.
        const etag = normalizeETag(response.headers.get('ETag')) || announced.etag;
        const total = byteCount(response.headers.get('Content-Length')) || announced.size;
        let received = 0;
        let bodyEnded;
        const ended = new Promise(resolve => { bodyEnded = resolve; });
        const counted = response.body.pipeThrough(new env.TransformStream({
            transform(chunk, controller) {
                received += chunk.byteLength;
                if (received > total) throw new Error('the download is larger than ' + total + ' bytes');
                onProgress(received, total);
                controller.enqueue(chunk);
            },
            flush() {
                // An error here makes the put fail, so a short copy is never stored.
                if (received !== total) throw new Error('the download ended after ' + received + ' of ' + total + ' bytes');
                bodyEnded();
            },
        }));
        const key = cacheKey(edition, etag, url);
        const put = cache.put(key, new env.Response(counted, { headers: {
            'Content-Type': 'application/octet-stream',
            'X-UQM-ETag': etag,
            'X-UQM-Size': String(total),
            'X-UQM-Stored': new Date().toISOString(),
        } }));
        // A slow network waits as long as Emscripten's own download would. A storage backend
        // that does not finish once every byte has arrived gives way to the normal path.
        await Promise.race([put, ended.then(() => deadline(put, env.limits.store, 'Storing the game data'))]);
        return { key, etag, size: total };
    }

    async function removeOtherVersions(cache, keep) {
        const keys = await cache.keys();
        await Promise.all(keys.filter(request => request.url !== keep).map(request => cache.delete(request)));
        return keys.length - (keys.some(request => request.url === keep) ? 1 : 0);
    }

    /** null when the copy may be stored, else the reason it may not. */
    async function storageProblem(storage, bytes, limit) {
        if (!storage || typeof storage.estimate !== 'function') return null;
        let estimate;
        let persisted = false;
        try {
            estimate = await deadline(storage.estimate(), limit, 'The storage estimate');
            if (typeof storage.persisted === 'function') persisted = await deadline(storage.persisted(), limit, 'The storage check');
        } catch (_) {
            return null;
        }
        if (!(estimate && estimate.quota > 0)) return null;
        if (!persisted && estimate.quota < TEMPORARY_QUOTA) {
            return 'browser storage looks temporary here (a private window keeps it in memory)';
        }
        if (estimate.quota - (estimate.usage || 0) < bytes + QUOTA_MARGIN) {
            return 'there is not enough browser storage for ' + megabytes(bytes) + ' MB';
        }
        return null;
    }

    function requestPersistence(storage, say) {
        if (!storage || typeof storage.persist !== 'function') return;
        Promise.resolve(typeof storage.persisted === 'function' ? storage.persisted() : false)
            .then(persisted => persisted || storage.persist())
            .then(granted => say(granted ? 'this browser keeps it until site data is cleared'
                : 'the browser may remove it when disk space runs low'))
            .catch(() => {});
    }

    function engineScripts(template) {
        return template && template.content ? Array.from(template.content.querySelectorAll('script')) : [];
    }

    /** The engine script, and its Wasm beside it (emcc names them uqm-debug.js and uqm-debug.wasm). */
    function engineFiles(env, template) {
        for (const script of engineScripts(template)) {
            const src = typeof script.getAttribute === 'function' ? script.getAttribute('src') : null;
            if (!src) continue;
            try {
                const js = new URL(src, env.base);
                if (!/\.js$/.test(js.pathname)) continue;
                const wasm = new URL(js.href);
                wasm.pathname = wasm.pathname.replace(/\.js$/, '.wasm');
                return { js: js.href, wasm: wasm.href };
            } catch (_) { /* Not a URL: the engine fetches its own files. */ }
        }
        return null;
    }

    /** Emscripten asks for its script only after the data. On a first visit, fetch it during the
        download, so the engine gets it from the HTTP cache (a 304 answer) later. */
    function warmScript(env, url) {
        return Promise.resolve().then(() => env.fetch(url, { credentials: 'same-origin' })).then(async response => {
            if (!response.ok || !response.body) return 0;
            // Read to the end, so the browser keeps the whole response; nothing is kept here.
            const reader = response.body.getReader();
            let bytes = 0;
            for (;;) {
                const { done, value } = await reader.read();
                if (done) return bytes;
                bytes += value.byteLength;
            }
        }).catch(() => 0);
    }

    /** Emscripten fetches and compiles its Wasm only after the data. Compile it now, while the data
        downloads or is read, and give the compiled module to Emscripten (Module.instantiateWasm).
        Safari's HTTP cache keeps no file this large, so a warm-up fetch would not help there. */
    function compileEarly(env, module, url, result, say) {
        const wasm = env.WebAssembly;
        if (!url || !wasm || typeof wasm.compileStreaming !== 'function' || typeof wasm.instantiateStreaming !== 'function' ||
            module.instantiateWasm) return;
        const fetchWasm = () => env.fetch(url, { credentials: 'same-origin' });
        const begin = now();
        const state = result.wasm = { url, state: 'compiling' };
        const compiled = wasm.compileStreaming(Promise.resolve().then(fetchWasm));
        compiled.then(() => Object.assign(state, { state: 'compiled', compileMs: Math.round(now() - begin) }),
            error => Object.assign(state, { state: 'failed', reason: message(error) }));
        module.instantiateWasm = function (imports, receiveInstance) {
            compiled.then(compiledModule => wasm.instantiate(compiledModule, imports).then(instance => ({ instance, module: compiledModule })))
                .catch(error => {
                    // Emscripten's own way: fetch and compile it again.
                    say('the Wasm compiled during loading could not start (' + message(error) + '); loading it again');
                    state.state = 'reloaded';
                    return wasm.instantiateStreaming(fetchWasm(), imports);
                })
                .then(output => {
                    state.used = true;
                    receiveInstance(output.instance, output.module);
                }, error => {
                    const text = 'The game engine could not start: ' + message(error);
                    say(text);
                    if (typeof module.onAbort === 'function') module.onAbort(text);
                });
            // Emscripten waits for receiveInstance.
            return {};
        };
    }

    /** Fills `result` (Module.gameData) and `held.buffer`; source 'cache', 'download' or 'network'. */
    async function load(options, env, report, say, result, held) {
        const edition = options.edition;
        const module = options.module;
        const url = new URL(options.dataFile || DATA_FILE, env.base).href;
        const limit = env.limits.check;
        result.url = url;
        const begin = now();
        const mark = name => { result.timings[name] = Math.round(now() - begin); };
        const network = reason => Object.assign(result, { source: 'network', stored: false, reason });
        if (!/^[a-z0-9-]{1,32}$/.test(String(edition || ''))) return network('the page named no edition');
        if (!env.fetch || !env.caches || typeof env.caches.open !== 'function' || !env.TransformStream || !env.Response) {
            return network('Cache Storage is unavailable in this browser');
        }
        let head;
        const abort = env.AbortController ? new env.AbortController() : null;
        try {
            head = await deadline(env.fetch(url, { method: 'HEAD', cache: 'no-store', credentials: 'same-origin',
                signal: abort ? abort.signal : undefined }), limit, 'The version check');
        } catch (error) {
            if (abort) abort.abort();
            return network('the version check failed: ' + message(error));
        }
        mark('head');
        if (!head.ok) return network('the version check answered HTTP ' + head.status);
        const etag = normalizeETag(head.headers.get('ETag'));
        const size = byteCount(head.headers.get('Content-Length'));
        if (!etag) return network('the server sent no ETag');
        if (!size) return network('the server sent no size');
        Object.assign(result, { etag, size });
        const key = cacheKey(edition, etag, url);
        let cache;
        let stored;
        try {
            cache = await deadline(env.caches.open(cacheName(edition)), limit, 'Cache Storage');
            stored = await deadline(cache.match(key), limit, 'Cache Storage');
        } catch (error) {
            return network('Cache Storage is unavailable: ' + message(error));
        }
        mark('match');
        if (stored && stored.headers.get('X-UQM-Size') !== String(size)) {
            say('the stored copy has the wrong size; downloading it again');
            stored = null;
        }
        const files = engineFiles(env, options.template);
        if (stored) {
            result.source = 'cache';
            compileEarly(env, module, files && files.wasm, result, say);
            // Older versions normally left already; never let them linger.
            removeOtherVersions(cache, key).catch(() => {});
        } else {
            try {
                // Free the old version first (and a copy of the wrong size), so an update never needs room for two copies.
                await deadline(cache.delete(key), limit, 'Cache Storage');
                result.removed = await deadline(removeOtherVersions(cache, key), limit, 'Cache Storage');
            } catch (error) {
                return network('Cache Storage is unavailable: ' + message(error));
            }
            const problem = await storageProblem(env.storage, size, limit);
            if (problem) return network(problem);
            if (files) warmScript(env, files.js);
            compileEarly(env, module, files && files.wasm, result, say);
            let saved;
            try {
                saved = await downloadToCache(env, url, cache, edition, { etag, size }, (done, total) => report.progress('download', done, total));
                mark('download');
                stored = await deadline(cache.match(saved.key), limit, 'Cache Storage');
            } catch (error) {
                return network('the download could not be stored: ' + message(error));
            }
            Object.assign(result, { source: 'download', etag: saved.etag, size: saved.size, stored: true });
            if (!stored) return network('the stored copy disappeared');
            requestPersistence(env.storage, say);
        }
        try {
            held.buffer = await readBody(stored, result.size, (done, total) => report.progress('stored', done, total), env.limits.read);
        } catch (error) {
            // A copy that reads wrong is removed; a slow one is kept for the next visit.
            if (!error.timeout) cache.delete(cacheKey(edition, result.etag, url)).catch(() => {});
            return network('the stored copy could not be read: ' + message(error));
        }
        mark('read');
        return result;
    }

    /** Starts the scripts that emcc wrote into the page's inert template. */
    function startEngine(template, document) {
        const inert = engineScripts(template);
        if (!inert.length) throw new Error('The page has no engine script.');
        for (const source of inert) {
            const script = document.createElement('script');
            // Attributes, not properties: the template's inert document has no base URL.
            for (const attribute of Array.from(source.attributes)) script.setAttribute(attribute.name, attribute.value);
            if (!source.hasAttribute('src')) script.text = source.textContent;
            // A dynamic script runs as soon as it arrives; several must keep emcc's order.
            if (inert.length > 1) script.async = false;
            document.body.appendChild(script);
        }
    }

    /** options: { module, edition, template, onProgress?, dataFile?, limits?, and test seams }. */
    function start(options) {
        const module = options.module;
        const env = environment(options);
        const say = text => {
            try { (module.printErr || module.print || console.log)('Game data: ' + text); } catch (_) { /* Logging is optional. */ }
        };
        // Module.gameData: what the page and the tests can read while the data loads and afterwards.
        const result = { source: 'pending', edition: options.edition, stored: false, timings: {}, steps: [], loading: null };
        module.gameData = result;
        const held = { buffer: null };
        const report = reporter(options, module, env, result);
        const begin = now();
        return load(options, env, report, say, result, held).catch(error => Object.assign(result, {
            source: 'network', stored: false, reason: message(error),
        })).then(() => {
            let buffer = held.buffer;
            held.buffer = null;
            result.ms = Math.round(now() - begin);
            if (buffer) {
                const expected = result.url;
                module.getPreloadedPackage = function (name, packageSize) {
                    const data = buffer;
                    // Hand over the one copy; this loader keeps no reference to it.
                    buffer = null;
                    if (!data) return null;
                    let requested = null;
                    try { requested = new URL(name, env.base).href; } catch (_) { /* Not a URL: no match. */ }
                    if (requested !== expected || packageSize !== data.byteLength) {
                        say('the engine expects ' + name + ' with ' + packageSize + ' bytes; downloading it the usual way');
                        result.source = 'network';
                        return null;
                    }
                    return data;
                };
                say(result.source === 'cache'
                    ? 'loaded ' + megabytes(result.size) + ' MB saved in this browser (' + result.etag + ') in ' + result.ms + ' ms'
                    : 'downloaded ' + megabytes(result.size) + ' MB and saved it in this browser for the next visit');
                report.text('starting', 'Starting the game…');
            } else {
                say('downloading it the usual way: ' + result.reason);
            }
            // Page-clock times, for the start-up measurements in tests/browser/data-cache.mjs.
            result.loaderStartedAt = Math.round(begin);
            result.engineStartedAt = Math.round(now());
            startEngine(options.template, env.document);
            return result;
        }).catch(error => {
            // Only a page without its engine template gets here; say so instead of waiting forever.
            const text = message(error);
            say(text);
            if (typeof module.onAbort === 'function') module.onAbort(text);
            return Object.assign(result, { source: 'error', reason: text });
        });
    }

    async function storedCaches(caches) {
        return (await caches.keys()).filter(name => name.indexOf(CACHE_PREFIX) === 0);
    }

    /** The stored copies in this browser: [{ edition, etag }]. Reads no game data. */
    async function storedCopies(options = {}) {
        const caches = environment(options).caches;
        if (!caches) return [];
        const copies = [];
        for (const name of await storedCaches(caches)) {
            const cache = await caches.open(name);
            for (const request of await cache.keys()) {
                const etag = decodeURIComponent(new URL(request.url).pathname.split('/').pop());
                copies.push({ edition: name.slice(CACHE_PREFIX.length), etag });
            }
        }
        return copies;
    }

    /** Frees the space of the stored game data (one edition, or every edition) and keeps the
        saved games. The next visit downloads the data again. Resolves to the editions removed. */
    async function removeStoredData(edition, options = {}) {
        const caches = environment(options).caches;
        if (!caches) return 0;
        const names = edition ? [cacheName(edition)] : await storedCaches(caches);
        let removed = 0;
        for (const name of names) if (await caches.delete(name)) removed++;
        return removed;
    }

    return { start, startEngine, readBody, normalizeETag, byteCount, cacheName, cacheKey, storedLabel,
        storedCopies, removeStoredData, DATA_FILE, LIMITS, TEMPORARY_QUOTA };
});
