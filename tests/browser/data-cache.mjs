// The game data stays in the browser between visits. The packaged site runs in
// the real Cloudflare Worker (deploy/worker.js under `wrangler dev`, configured from
// deploy/wrangler.jsonc, with the data files split by deploy/split-data.py), which
// answers the data file with its SHA-256 ETag. For each edition: the first visit
// downloads the file once and stores it, a return visit transfers no data body (only
// the HEAD) and still reaches the main menu, and the Rust page of the same edition
// reuses the copy. The checks read Module-level signals (Module.gameData, runtimeReady,
// Module.gameScreen), so they hold for the shells with and without the game menu.
//
//   UQM_DIST=dist node tests/browser/data-cache.mjs
//   UQM_BROWSER=firefox|webkit, UQM_DATA_EDITIONS=original,hd, UQM_RETURN_VISITS=3,
//   UQM_SITE=<split site> UQM_SITE_MANIFEST=<its data-parts.json> to reuse a split,
//   UQM_DATA_CACHE_LEAN=1 for the deploy gate: first visit, return visit and Rust page
//   only, without launching the engine and without the no-Cache-API and private visits.
import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { access, copyFile, mkdir, mkdtemp, readdir, readFile, rm, writeFile } from 'node:fs/promises';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium, firefox, webkit } from 'playwright';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const dist = path.resolve(process.env.UQM_DIST || 'dist');
const out = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/data-cache');
const browserType = process.env.UQM_BROWSER || 'chromium';
const editions = (process.env.UQM_DATA_EDITIONS || 'original,hd').split(',');
const WRANGLER = process.env.WRANGLER ?? 'wrangler@4.143.1';
const lean = process.env.UQM_DATA_CACHE_LEAN === '1';
const headless = process.env.HEADED !== '1';
const types = { chromium, firefox, webkit };
assert.ok(types[browserType], 'Unsupported UQM_BROWSER');
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const MB = bytes => Math.round(bytes / 1e6);
await mkdir(out, { recursive: true });

function freePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const { port } = server.address();
      server.close(() => resolve(port));
    });
  });
}

/** deploy/wrangler.jsonc: JSON with comments and trailing commas. */
function parseJSONC(source) {
  let json = '';
  for (let i = 0; i < source.length; i++) {
    const char = source[i];
    if (char === '"') {
      const end = /"(?:[^"\\]|\\.)*"/y;
      end.lastIndex = i;
      const string = end.exec(source)[0];
      json += string;
      i += string.length - 1;
    } else if (char === '/' && source[i + 1] === '/') {
      while (i < source.length && source[i] !== '\n') i++;
      json += '\n';
    } else if (char === '/' && source[i + 1] === '*') {
      i = source.indexOf('*/', i + 2) + 1;
    } else {
      json += char;
    }
  }
  return JSON.parse(json.replace(/,(\s*[}\]])/g, '$1'));
}

const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-data-cache-'));
const report = { browserType, editions: {}, errors: [], passed: false };
let worker, context;
let workerLog = '';
try {
  // The Worker reads the part manifest beside itself; copy it with its relay module.
  const workerDir = path.join(temp, 'worker');
  await mkdir(workerDir);
  for (const file of (await readdir(path.join(root, 'deploy'))).filter(name => /\.m?js$/.test(name))) {
    await copyFile(path.join(root, 'deploy', file), path.join(workerDir, file));
  }
  // UQM_SITE and UQM_SITE_MANIFEST name a split site to reuse (the deploy job's .tools/site and
  // deploy/data-parts.json); when they do not exist yet, the split is written there and kept.
  const site = process.env.UQM_SITE ? path.resolve(process.env.UQM_SITE) : path.join(temp, 'site');
  const siteManifest = process.env.UQM_SITE_MANIFEST ? path.resolve(process.env.UQM_SITE_MANIFEST) : path.join(temp, 'data-parts.json');
  const manifest = path.join(workerDir, 'data-parts.json');
  if (!await access(siteManifest).then(() => access(site)).then(() => true, () => false)) {
    await mkdir(path.dirname(site), { recursive: true });
    await mkdir(path.dirname(siteManifest), { recursive: true });
    const split = spawnSync('python3', [path.join(root, 'deploy/split-data.py'), '--site', dist, '--output', site, '--manifest', siteManifest],
      { stdio: ['ignore', 'pipe', 'inherit'], encoding: 'utf8' });
    assert.equal(split.status, 0, 'split-data.py failed');
    process.stdout.write(split.stdout);
  }
  await copyFile(siteManifest, manifest);
  const parts = JSON.parse(await readFile(manifest, 'utf8'));
  report.manifest = parts;
  const port = await freePort();
  const origin = `http://127.0.0.1:${port}`;
  // The deploy config with its bindings and migrations; only the site, the origin and the name differ.
  const config = parseJSONC(await readFile(path.join(root, 'deploy/wrangler.jsonc'), 'utf8'));
  for (const deployOnly of ['routes', 'workers_dev', 'preview_urls', 'observability']) delete config[deployOnly];
  Object.assign(config, { name: 'uqm-web-data-cache-test', main: path.join(workerDir, path.basename(config.main)),
    assets: { ...config.assets, directory: site }, vars: { ...config.vars, NETPLAY_ORIGINS: origin } });
  assert.ok(config.durable_objects?.bindings?.length && config.migrations?.length, 'the deploy config declares the relay objects');
  const configPath = path.join(temp, 'wrangler.json');
  await writeFile(configPath, JSON.stringify(config));
  worker = spawn('npx', ['--yes', WRANGLER, 'dev', '--config', configPath, '--ip', '127.0.0.1', '--port', String(port),
    '--inspector-port', String(await freePort()), '--persist-to', path.join(temp, 'state'),
    '--show-interactive-dev-session=false', '--log-level', 'warn'],
  { cwd: temp, env: { ...process.env, WRANGLER_SEND_METRICS: 'false', CI: '1' }, stdio: ['ignore', 'pipe', 'pipe'], detached: true });
  worker.stdout.on('data', chunk => { workerLog += chunk; });
  worker.stderr.on('data', chunk => { workerLog += chunk; });
  const deadline = Date.now() + 180000;
  for (;;) {
    if (worker.exitCode !== null) throw new Error('wrangler dev exited:\n' + workerLog);
    try { if ((await fetch(origin + '/', { signal: AbortSignal.timeout(2000) })).ok) break; } catch { /* Not listening yet. */ }
    if (Date.now() > deadline) throw new Error('wrangler dev did not start:\n' + workerLog);
    await sleep(200);
  }
  for (const edition of editions) {
    const head = await fetch(`${origin}/${edition}/uqm-debug.data`, { method: 'HEAD' });
    assert.equal(head.headers.get('ETag'), '"' + parts[`/${edition}/uqm-debug.data`].sha256 + '"');
  }

  const profile = path.join(temp, 'profile');
  const launchOptions = { headless, ...(browserType === 'chromium' ? { args: ['--autoplay-policy=no-user-gesture-required'] } : {}) };
  context = await types[browserType].launchPersistentContext(profile, { ...launchOptions, viewport: { width: 1440, height: 1100 } });
  report.browser = context.browser()?.version() ?? browserType;

  // Recorded inside the page, so the harness's own waits never count as start-up time.
  function recordInPage() {
    const times = window.__uqmTimes = {};
    const shown = window.__uqmShown = [];
    let ready = false;
    // The shells declare `var runtimeReady` and set it to true when the engine is ready.
    Object.defineProperty(window, 'runtimeReady', { configurable: true, enumerable: true, get: () => ready,
      set: value => { ready = value; if (value && times.ready === undefined) { times.ready = performance.now(); record(); } } });
    // What the page shows while it loads: the game menu's loading label, else the status line.
    function record() {
      let text = '';
      try {
        const menu = window.online && window.online.controller;
        text = (menu && menu.state && menu.state.loading && menu.state.loading.label) || document.getElementById('status')?.textContent || '';
      } catch { /* The page is still parsing. */ }
      if (text && shown.at(-1) !== text) shown.push(text);
    }
    const timer = setInterval(() => { record(); if (ready) clearInterval(timer); }, 10);
  }

  /** Opens one engine page as a fresh visit and records what it transferred. */
  async function visit(folder, label, { noCacheAPI = false, launch = !lean } = {}) {
    const page = await context.newPage();
    const methods = [];
    let rejectFatal;
    const fatal = new Promise((_, reject) => { rejectFatal = reject; });
    fatal.catch(() => {});
    page.on('pageerror', error => { report.errors.push(folder + ': ' + error.message); rejectFatal(error); });
    page.on('crash', () => rejectFatal(new Error('Browser page crashed')));
    page.on('request', request => {
      if (new URL(request.url()).pathname.endsWith('/uqm-debug.data')) methods.push(request.method());
    });
    await page.addInitScript(recordInPage);
    // A browser without Cache Storage (as in some private windows): today's download path.
    if (noCacheAPI) await page.addInitScript(() => Object.defineProperty(window, 'caches', { get: () => undefined, configurable: true }));
    // Chromium: every byte the page's own requests received, headers included.
    let received = null;
    let cdp = null;
    if (browserType === 'chromium') {
      cdp = await context.newCDPSession(page);
      await cdp.send('Network.enable');
      received = 0;
      cdp.on('Network.loadingFinished', event => { received += event.encodedDataLength; });
    }
    // A picture of the loading text, when the harness happens to see it: the download on a
    // first visit, the stored copy on a return visit (WebKit reads it in one step).
    const loadingLabel = noCacheAPI ? null : label.endsWith('-first') ? /^Downloading game data: [1-9]/ : /^Loading game data saved in this browser: [1-9]/;
    let pictured = false;
    async function waitFor(target, timeout) {
      const end = Date.now() + timeout;
      for (;;) {
        const state = await Promise.race([page.evaluate(target => {
          const module = typeof Module === 'object' ? Module : {};
          const status = document.getElementById('status')?.textContent || '';
          return {
            done: target === 'runtime' ? typeof runtimeReady !== 'undefined' && runtimeReady : module.gameScreen === target,
            failed: document.getElementById('start')?.textContent === 'Unable to start' || /^Unable to start/.test(status) ||
              module.exitStatus !== undefined,
            loading: module.gameData?.loading?.label || '', status, screen: module.gameScreen,
          };
        }, target).catch(error => ({ error: error.message })), fatal]).catch(error => ({ error: error.message }));
        if (state.failed || state.error || Date.now() > end) {
          await page.screenshot({ path: path.join(out, `${label}-stalled.png`) }).catch(() => {});
          const log = await page.evaluate(() => document.getElementById('log')?.textContent.split('\n').slice(-25)).catch(() => null);
          throw new Error(`${folder}: waiting for ${target}: ${state.error || (state.failed ? 'the page failed' : 'timed out')}\n` +
            JSON.stringify({ ...state, log }, null, 1));
        }
        if (state.done) return state;
        if (!pictured && loadingLabel?.test(state.loading)) {
          pictured = true;
          await page.screenshot({ path: path.join(out, `${label}-loading.png`) }).catch(() => {});
        }
        await sleep(50);
      }
    }
    await page.goto(`${origin}/${folder}/index.html`, { timeout: 120000 });
    await waitFor('runtime', 300000);
    const loaded = await page.evaluate(() => ({
      readyMs: Math.round(window.__uqmTimes.ready), data: Module.gameData, shown: window.__uqmShown,
      status: document.getElementById('status')?.textContent,
      // Resource Timing: the requests for the data file and the engine files, and the bytes each one carried.
      entries: performance.getEntriesByType('resource').filter(entry => /\/uqm-debug\.(data|js|wasm)$/.test(entry.name))
        .map(entry => ({ file: entry.name.split('.').pop(), type: entry.initiatorType, body: entry.encodedBodySize, transfer: entry.transferSize })),
    }));
    const receivedAtReady = received;
    // Pictures are evidence, not checks: a starved page may take long to paint one.
    const picture = (target, name) => target.screenshot({ path: path.join(out, `${label}-${name}.png`), timeout: 120000 }).catch(() => {});
    await picture(page, 'ready');
    let launchToMenuMs = null;
    if (launch) {
      // The page's own start control: "Play" in the game menu, or the Launch button.
      const clicked = Date.now();
      // A busy machine can hold the page's main thread for a while after Ready.
      await page.getByRole('button', { name: /^(Play|Launch( HD)? game)$/ }).first().click({ timeout: 180000 });
      await waitFor('mainMenu', 300000);
      launchToMenuMs = Date.now() - clicked;
      await sleep(1500);
      await picture(page.locator('#canvas'), 'menu');
    }
    // Emscripten's files are views of the one buffer the loader handed over.
    const files = await page.evaluate(() => {
      const found = [];
      (function walk(dir) {
        for (const name of FS.readdir(dir)) {
          if (name === '.' || name === '..' || found.length >= 3) continue;
          const full = dir + '/' + name;
          const node = FS.lookupPath(full).node;
          if (FS.isDir(node.mode)) walk(full); else if (node.contents?.buffer) found.push(node);
        }
      })('/content');
      return { count: found.length, buffers: found.map(node => node.contents.buffer.byteLength),
        shared: found.every(node => node.contents.buffer === found[0].contents.buffer),
        wasmHeap: (typeof wasmMemory === 'object' ? wasmMemory : HEAP8).buffer.byteLength };
    });
    // V8's ArrayBuffer backing stores in this page: the data buffer, the Wasm heap and small buffers.
    let backingStore = null;
    if (cdp) {
      await cdp.send('HeapProfiler.collectGarbage').catch(() => {});
      backingStore = (await cdp.send('Runtime.getHeapUsage')).backingStorageSize ?? null;
    }
    const caches = noCacheAPI ? null : await page.evaluate(async () => {
      const result = {};
      for (const name of await caches.keys()) result[name] = (await (await caches.open(name)).keys()).map(request => request.url);
      return result;
    });
    await page.close();
    return { folder, readyMs: loaded.readyMs, launchToMenuMs, methods, entries: loaded.entries,
      dataEntries: loaded.entries.filter(entry => entry.file === 'data'), receivedAtReady, files, backingStore, caches,
      gameData: loaded.data, readyStatus: loaded.status, shown: loaded.shown,
      steps: loaded.data.steps.map(step => step.label) };
  }

  /** A private window: the loader leaves the data to the engine and stores nothing. */
  async function privateVisit(folder) {
    const browser = await types[browserType].launch(launchOptions);
    try {
      const page = await (await browser.newContext()).newPage();
      await page.goto(`${origin}/${folder}/index.html`, { timeout: 120000 });
      await page.waitForFunction(() => typeof Module === 'object' && Module.gameData && Module.gameData.source !== 'pending', null,
        { timeout: 120000, polling: 50 });
      return await page.evaluate(async () => {
        const stored = {};
        for (const name of await caches.keys()) stored[name] = (await (await caches.open(name)).keys()).map(request => request.url);
        return { source: Module.gameData.source, reason: Module.gameData.reason, quota: (await navigator.storage.estimate()).quota, stored };
      });
    } finally {
      await browser.close();
    }
  }

  for (const edition of editions) {
    const size = parts[`/${edition}/uqm-debug.data`].size;
    const etag = '"' + parts[`/${edition}/uqm-debug.data`].sha256 + '"';
    const cacheName = 'uqm-game-data-' + edition;
    const key = `${origin}/.uqm-game-data/${edition}/${encodeURIComponent(etag)}`;
    // An older stored version of this edition must make room for the current one.
    const seed = await context.newPage();
    await seed.goto(origin + '/');
    await seed.evaluate(async ([cacheName, url]) => {
      const cache = await caches.open(cacheName);
      await cache.put(url, new Response(new Uint8Array(1000), { headers: { 'X-UQM-Size': '1000' } }));
    }, [cacheName, `${origin}/.uqm-game-data/${edition}/${encodeURIComponent('"older-version"')}`]);
    await seed.close();
    const results = report.editions[edition] = {};
    const first = results.first = await visit(edition, edition + '-first');
    assert.equal(first.gameData.source, 'download', JSON.stringify(first.gameData));
    assert.equal(first.gameData.stored, true);
    assert.equal(first.gameData.etag, etag);
    assert.deepEqual(first.methods, ['HEAD', 'GET'], 'the first visit downloads the file once');
    assert.deepEqual(first.caches[cacheName], [key], 'the older version was removed');
    assert.ok(first.steps.includes(`Downloading game data: ${MB(size)} / ${MB(size)} MB (100%)`), JSON.stringify(first.steps));
    assert.equal(first.steps.at(-1), 'Starting the game…');
    // During the download the loader fetched the engine script into the HTTP cache (the engine's
    // own request then gets a 304 answer of a few hundred bytes) and compiled the Wasm, so the
    // engine fetched no Wasm of its own.
    const scripts = first.entries.filter(entry => entry.file === 'js');
    assert.ok(scripts.length >= 2, JSON.stringify(first.entries));
    if (scripts.every(entry => entry.transfer > 0)) {
      assert.equal(scripts.filter(entry => entry.transfer > 64 * 1024).length, 1, 'the script crossed the network once: ' + JSON.stringify(scripts));
    }
    assert.equal(first.entries.filter(entry => entry.file === 'wasm').length, 1, JSON.stringify(first.entries));
    // More return visits give a steadier start-up time on a busy machine.
    const returns = Math.max(1, Number(process.env.UQM_RETURN_VISITS) || 1);
    const visits = [['second', edition], ['rust', edition + '-rust']];
    for (let n = 2; n <= returns; n++) visits.push(['second-' + n, edition]);
    for (const [name, folder] of visits) {
      const again = results[name] = await visit(folder, folder + '-' + name);
      assert.equal(again.gameData.source, 'cache', folder + ': ' + JSON.stringify(again.gameData));
      assert.deepEqual(again.methods, ['HEAD'], folder + ': only the HEAD reaches the server');
      assert.equal(again.dataEntries.length, 1, JSON.stringify(again.dataEntries));
      assert.equal(again.dataEntries[0].body, 0, folder + ': the HEAD carries no body');
      if (again.receivedAtReady !== null) {
        assert.ok(again.receivedAtReady < 32e6, folder + ': the visit received ' + again.receivedAtReady + ' bytes');
      }
      assert.deepEqual(again.caches[cacheName], [key]);
      assert.equal(again.entries.filter(entry => entry.file === 'wasm').length, 1, JSON.stringify(again.entries));
      // The loader's steps, and what the page showed: never a download for the stored copy.
      assert.deepEqual(again.steps.slice(-2), [`Loading game data saved in this browser: ${MB(size)} / ${MB(size)} MB (100%)`,
        'Starting the game…'], JSON.stringify(again.steps));
      assert.ok(again.steps.every(step => step.startsWith('Loading game data saved in this browser') || step === 'Starting the game…'),
        JSON.stringify(again.steps));
      assert.ok(again.shown.some(text => text.startsWith('Loading game data saved in this browser') || text === 'Starting the game…'),
        JSON.stringify(again.shown));
      assert.ok(!again.shown.some(text => text.startsWith('Downloading')), JSON.stringify(again.shown));
    }
    const checked = [first, results.second, results.rust];
    if (!lean) {
      // Without Cache Storage the engine downloads the file itself, as before the loader.
      const fallback = results.fallback = await visit(edition, edition + '-fallback', { noCacheAPI: true });
      assert.equal(fallback.gameData.source, 'network');
      assert.match(fallback.gameData.reason, /Cache Storage is unavailable/);
      assert.deepEqual(fallback.methods, ['GET']);
      checked.push(fallback);
      // A private window keeps Cache Storage in memory: nothing is stored there (Firefox's
      // Playwright contexts are not private windows).
      if (browserType !== 'firefox' && edition === editions[0]) {
        const privately = results.private = await privateVisit(edition);
        assert.equal(privately.source, 'network', JSON.stringify(privately));
        assert.match(privately.reason, /storage looks temporary/);
        assert.deepEqual(privately.stored[cacheName] ?? [], [], 'nothing stored: ' + JSON.stringify(privately));
      }
    }
    for (const result of checked) {
      // Emscripten started from the Wasm the loader compiled while the data loaded (not without Cache Storage).
      if (result.gameData.source !== 'network') assert.equal(result.gameData.wasm?.used, true, JSON.stringify(result.gameData.wasm));
      else assert.equal(result.gameData.wasm, undefined);
      assert.ok(result.files.count >= 2 && result.files.shared, JSON.stringify(result.files));
      assert.ok(result.files.buffers.every(bytes => bytes === size), 'one buffer of exactly the data size: ' + JSON.stringify(result.files));
      if (result.backingStore !== null) {
        // One data buffer plus the Wasm heap: never a second copy of the data.
        assert.ok(result.backingStore < size + result.files.wasmHeap + 64e6,
          `${result.folder}: ${MB(result.backingStore)} MB of ArrayBuffers for ${MB(size)} MB of data and a ${MB(result.files.wasmHeap)} MB heap`);
      }
    }
    const second = results.second;
    const returnReady = Object.entries(results).filter(([name]) => name.startsWith('second')).map(([, r]) => r.readyMs).sort((a, b) => a - b);
    results.returnReadyMs = { all: returnReady, median: returnReady[Math.floor(returnReady.length / 2)], min: returnReady[0] };
    console.log(`${edition} (${MB(size)} MB): first visit ${first.readyMs} ms to Ready (download over loopback); ` +
      `return visit ${second.readyMs} ms to Ready (median of ${returnReady.length}: ${results.returnReadyMs.median} ms; ` +
      `HEAD ${second.gameData.timings.head} ms, stored copy read by ${second.gameData.timings.read} ms, Wasm compiled in ` +
      `${second.gameData.wasm?.compileMs} ms, engine script started at ${second.gameData.engineStartedAt} ms)` + (second.receivedAtReady !== null ? `, ${Math.round(second.receivedAtReady / 1024)} KB received` : '') +
      (second.launchToMenuMs !== null ? `, then ${second.launchToMenuMs} ms from Launch to the main menu` : '') +
      `; Rust page ${results.rust.readyMs} ms to Ready` + (results.fallback ? `; without Cache Storage ${results.fallback.readyMs} ms to Ready` : '') + '.' +
      (second.backingStore !== null && results.fallback ? ` ArrayBuffers ${MB(second.backingStore)} MB (cache) vs ${MB(results.fallback.backingStore)} MB (no cache).` : '') +
      (results.private ? ` Private window: ${results.private.reason}.` : ''));
  }
  assert.deepEqual(report.errors, []);
  report.passed = true;
} catch (error) {
  report.failure = error.stack;
  report.workerLog = workerLog.slice(-20000);
  throw error;
} finally {
  await writeFile(path.join(out, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  await context?.close().catch(() => {});
  if (worker) {
    const signal = name => { try { process.kill(-worker.pid, name); } catch { /* Already gone. */ } };
    signal('SIGTERM');
    await Promise.race([new Promise(resolve => worker.once('exit', resolve)), sleep(5000)]);
    signal('SIGKILL');
    worker.stdout.destroy();
    worker.stderr.destroy();
  }
  await rm(temp, { recursive: true, force: true }).catch(() => {});
}
