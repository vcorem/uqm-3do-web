/* Game data loader shared by every engine page (Original and HD, C and Rust).

   The Emscripten data file (Original 299 MB, HD 640 MB) is too large for the
   browsers' HTTP caches, so every visit used to download it again. The page
   keeps the engine script in an inert <template>. This loader asks the server
   for the data file's ETag with a HEAD request, keeps one copy per edition and
   ETag in Cache Storage, and gives that copy to Emscripten through
   Module.getPreloadedPackage before it starts the engine script. The C and Rust
   pages of an edition package the same bytes, so they share one stored copy.

   Memory: a download streams into Cache Storage and is then read back into one
   ArrayBuffer of the exact size. Emscripten's file system keeps views of that
   buffer (canOwn), and the loader drops its own reference when it hands it over.

   Whenever Cache Storage, an ETag, the HEAD request or the quota is unavailable,
   the engine starts exactly as before and downloads the file itself. */
(function (root, factory) {
    'use strict';
    const api = factory();
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMEngineLoader = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function () {
    'use strict';
    const DATA_FILE = 'uqm-debug.data';
    const CACHE_PREFIX = 'uqm-game-data-';
    const KEY_PATH = '/.uqm-game-data/';
    // Room for the cache's own records next to the stored copy.
    const QUOTA_MARGIN = 64 * 1024 * 1024;
    const PROGRESS_INTERVAL_MS = 100;
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

    function storedLabel(done, total) {
        return 'Loading game data saved in this browser: ' + megabytes(done) + ' / ' + megabytes(total) +
            ' MB (' + Math.floor(done * 100 / total) + '%)';
    }

    function environment(options) {
        const scope = typeof globalThis !== 'undefined' ? globalThis : {};
        let caches = options.caches;
        if (caches === undefined) {
            // Some private modes throw on access instead of leaving the API out.
            try { caches = scope.caches || null; } catch (_) { caches = null; }
        }
        const document = options.document || scope.document;
        return {
            fetch: options.fetch || (typeof scope.fetch === 'function' ? scope.fetch.bind(scope) : null),
            caches,
            storage: options.storage !== undefined ? options.storage
                : (scope.navigator && scope.navigator.storage) || null,
            document,
            base: options.base || (document && document.baseURI) || (scope.location && scope.location.href),
            TransformStream: options.TransformStream || scope.TransformStream,
            Response: options.Response || scope.Response,
        };
    }

    /** Feeds the page's progress bar and status text. Downloads use Emscripten's own
        wording, so the page formats them exactly like its own download (UQMHelp.formatProgress). */
    function reporter(options, module) {
        let last = -Infinity;
        function text(message) {
            if (options.status) options.status.textContent = message;
            else if (typeof module.setStatus === 'function') module.setStatus(message);
        }
        return {
            progress(phase, done, total) {
                const time = now();
                if (done < total && time - last < PROGRESS_INTERVAL_MS) return;
                last = time;
                if (phase === 'download' && typeof module.setStatus === 'function') {
                    module.setStatus('Downloading data... (' + done + '/' + total + ')');
                    return;
                }
                // Keep the dependency counter from taking over the byte progress bar.
                module.byteProgress = true;
                if (options.progress) {
                    options.progress.max = total;
                    options.progress.value = done;
                }
                text(phase === 'download' ? 'Downloading game data: ' + megabytes(done) + ' / ' + megabytes(total) + ' MB'
                    : storedLabel(done, total));
            },
            text,
        };
    }

    /** Reads a response body into one ArrayBuffer of exactly `expected` bytes. */
    async function readBody(response, expected, onProgress) {
        if (!response.body || typeof response.body.getReader !== 'function') {
            const buffer = await response.arrayBuffer();
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
                const { done, value } = await reader.read();
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
            },
        }));
        const key = cacheKey(edition, etag, url);
        await cache.put(key, new env.Response(counted, { headers: {
            'Content-Type': 'application/octet-stream',
            'X-UQM-ETag': etag,
            'X-UQM-Size': String(total),
            'X-UQM-Stored': new Date().toISOString(),
        } }));
        return { key, etag, size: total };
    }

    async function removeOtherVersions(cache, keep) {
        const keys = await cache.keys();
        await Promise.all(keys.filter(request => request.url !== keep).map(request => cache.delete(request)));
        return keys.length - (keys.some(request => request.url === keep) ? 1 : 0);
    }

    async function hasRoom(storage, bytes) {
        if (!storage || typeof storage.estimate !== 'function') return true;
        try {
            const estimate = await storage.estimate();
            if (!(estimate && estimate.quota > 0)) return true;
            return estimate.quota - (estimate.usage || 0) >= bytes + QUOTA_MARGIN;
        } catch (_) {
            return true;
        }
    }

    function requestPersistence(storage, say) {
        if (!storage || typeof storage.persist !== 'function') return;
        Promise.resolve(typeof storage.persisted === 'function' ? storage.persisted() : false)
            .then(persisted => persisted || storage.persist())
            .then(granted => say(granted ? 'this browser keeps it until site data is cleared'
                : 'the browser may remove it when disk space runs low'))
            .catch(() => {});
    }

    /** Resolves to { source: 'cache' | 'download', buffer, ... } or { source: 'network', reason }. */
    async function load(options, env, report, say) {
        const edition = options.edition;
        const url = new URL(options.dataFile || DATA_FILE, env.base).href;
        const result = { source: 'network', edition, url, stored: false, timings: {} };
        const begin = now();
        const mark = name => { result.timings[name] = Math.round(now() - begin); };
        const network = reason => Object.assign(result, { source: 'network', stored: false, reason });
        if (!/^[a-z0-9-]{1,32}$/.test(String(edition || ''))) return network('the page named no edition');
        if (!env.fetch || !env.caches || typeof env.caches.open !== 'function' || !env.TransformStream || !env.Response) {
            return network('Cache Storage is unavailable in this browser');
        }
        let head;
        try {
            head = await env.fetch(url, { method: 'HEAD', cache: 'no-store', credentials: 'same-origin' });
        } catch (error) {
            return network('the version check failed: ' + (error && error.message || error));
        }
        mark('head');
        if (!head.ok) return network('the version check answered HTTP ' + head.status);
        const etag = normalizeETag(head.headers.get('ETag'));
        const size = byteCount(head.headers.get('Content-Length'));
        if (!etag) return network('the server sent no ETag');
        if (!size) return network('the server sent no size');
        Object.assign(result, { etag, size });
        let cache;
        try {
            cache = await env.caches.open(cacheName(edition));
        } catch (error) {
            return network('Cache Storage is unavailable: ' + (error && error.message || error));
        }
        const key = cacheKey(edition, etag, url);
        let stored = await cache.match(key);
        mark('match');
        if (stored && stored.headers.get('X-UQM-Size') !== String(size)) {
            say('the stored copy has the wrong size; downloading it again');
            await cache.delete(key);
            stored = null;
        }
        if (stored) {
            result.source = 'cache';
            // Older versions normally left already; never let them linger.
            removeOtherVersions(cache, key).catch(() => {});
        } else {
            // Free the old version first, so an update never needs room for two copies.
            result.removed = await removeOtherVersions(cache, key);
            if (!await hasRoom(env.storage, size)) return network('there is not enough browser storage for ' + megabytes(size) + ' MB');
            let saved;
            try {
                saved = await downloadToCache(env, url, cache, edition, { etag, size }, (done, total) => report.progress('download', done, total));
            } catch (error) {
                return network('the download could not be stored: ' + (error && error.message || error));
            }
            mark('download');
            Object.assign(result, { source: 'download', etag: saved.etag, size: saved.size, stored: true });
            stored = await cache.match(saved.key);
            if (!stored) return network('the stored copy disappeared');
            requestPersistence(env.storage, say);
        }
        try {
            result.buffer = await readBody(stored, result.size, (done, total) => report.progress('stored', done, total));
        } catch (error) {
            await cache.delete(cacheKey(edition, result.etag, url)).catch(() => {});
            return network('the stored copy could not be read: ' + (error && error.message || error));
        }
        mark('read');
        return result;
    }

    /** Starts the scripts that emcc wrote into the page's inert template. */
    function startEngine(template, document) {
        const inert = template && template.content ? Array.from(template.content.querySelectorAll('script')) : [];
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

    /** options: { module, edition, template, status?, progress?, dataFile?, and test seams }. */
    function start(options) {
        const module = options.module;
        const env = environment(options);
        const say = message => {
            try { (module.printErr || module.print || console.log)('Game data: ' + message); } catch (_) { /* Logging is optional. */ }
        };
        const report = reporter(options, module);
        const begin = now();
        return load(options, env, report, say).catch(error => ({
            source: 'network', edition: options.edition, reason: String(error && error.message || error),
        })).then(result => {
            let buffer = result.buffer || null;
            delete result.buffer;
            result.ms = Math.round(now() - begin);
            module.gameData = result;
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
                report.text('Starting the game…');
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
            const message = String(error && error.message || error);
            say(message);
            if (typeof module.onAbort === 'function') module.onAbort(message);
            return { source: 'error', reason: message };
        });
    }

    return { start, startEngine, readBody, normalizeETag, byteCount, cacheName, cacheKey, storedLabel, DATA_FILE };
});
