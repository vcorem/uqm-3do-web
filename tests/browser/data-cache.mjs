// The game data stays in the browser between visits. The packaged site runs in
// the real Cloudflare Worker (deploy/worker.js under `wrangler dev`, with the
// data files split by deploy/split-data.py), which answers the data file with
// its SHA-256 ETag. For each edition: the first visit downloads the file once
// and stores it, a return visit transfers no data body (only the HEAD) and still
// reaches the main menu, and the Rust page of the same edition reuses the copy.
//
//   UQM_DIST=dist node tests/browser/data-cache.mjs
//   UQM_BROWSER=firefox|webkit, UQM_DATA_EDITIONS=original,hd, UQM_RETURN_VISITS=3,
//   UQM_SITE=<split site> UQM_SITE_MANIFEST=<its data-parts.json> to reuse a split
import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { access, copyFile, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
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

const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-data-cache-'));
const report = { browserType, editions: {}, errors: [], passed: false };
let worker, context;
let workerLog = '';
try {
  // The Worker reads the part manifest beside itself; copy it with its relay module.
  const workerDir = path.join(temp, 'worker');
  await mkdir(workerDir);
  for (const file of ['worker.js', 'netplay.js']) await copyFile(path.join(root, 'deploy', file), path.join(workerDir, file));
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
  const deployed = await readFile(path.join(root, 'deploy/wrangler.jsonc'), 'utf8');
  const config = {
    name: 'uqm-web-data-cache-test',
    main: path.join(workerDir, 'worker.js'),
    compatibility_date: /"compatibility_date":\s*"([^"]+)"/.exec(deployed)[1],
    assets: { directory: site, binding: 'ASSETS', run_worker_first: true, html_handling: 'none', not_found_handling: 'none' },
    durable_objects: { bindings: [
      { name: 'NETPLAY_ROOMS', class_name: 'NetplayRoom' }, { name: 'NETPLAY_GATE', class_name: 'NetplayGate' }] },
    migrations: [{ tag: 'netplay-v1', new_sqlite_classes: ['NetplayRoom', 'NetplayGate'] }],
    vars: { NETPLAY_ORIGINS: origin },
  };
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
  context = await types[browserType].launchPersistentContext(profile, {
    headless: process.env.HEADED !== '1', viewport: { width: 1440, height: 1100 },
    ...(browserType === 'chromium' ? { args: ['--autoplay-policy=no-user-gesture-required'] } : {}),
  });
  report.browser = context.browser()?.version() ?? browserType;

  /** Opens one engine page as a fresh visit and records what it transferred. */
  async function visit(folder, label, { noCacheAPI = false } = {}) {
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
    // Every status text the page shows while it loads.
    await page.addInitScript(() => {
      window.__statusTexts = [];
      new MutationObserver(() => {
        const text = document.getElementById('status')?.textContent;
        if (text && window.__statusTexts.at(-1) !== text) window.__statusTexts.push(text);
      }).observe(document, { subtree: true, childList: true, characterData: true });
    });
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
    async function waitFor(target, timeout) {
      await Promise.race([page.waitForFunction(target => {
        const button = document.getElementById('start');
        const failed = button?.textContent === 'Unable to start' || (typeof Module === 'object' && Module.exitStatus !== undefined);
        return failed || (target === 'runtime' ? typeof runtimeReady !== 'undefined' && runtimeReady
          : typeof Module === 'object' && Module.gameScreen === target);
      }, target, { timeout, polling: 50 }), fatal]).catch(async error => {
        await page.screenshot({ path: path.join(out, `${label}-stalled.png`) }).catch(() => {});
        const state = await page.evaluate(() => ({ screen: Module.gameScreen, status: document.getElementById('status')?.textContent,
          log: document.getElementById('log')?.textContent.split('\n').slice(-25) })).catch(() => null);
        throw new Error(`${folder}: waiting for ${target}: ${error.message}\n${JSON.stringify(state, null, 1)}`);
      });
      const state = await page.evaluate(() => ({ failed: document.getElementById('start')?.textContent === 'Unable to start',
        status: document.getElementById('status')?.textContent, screen: Module.gameScreen }));
      assert.ok(!state.failed, folder + ': ' + JSON.stringify(state) + '\n' + await page.locator('#log').textContent());
    }
    const navigation = Date.now();
    await page.goto(`${origin}/${folder}/index.html`, { timeout: 120000 });
    const gotoMs = Date.now() - navigation;
    // A picture of the loading text: the download on a first visit, the stored copy on a return visit.
    const loadingLabel = noCacheAPI ? null : label.endsWith('-first') ? /^Downloading game data: [1-9]/ : /^Loading game data saved in this browser: [1-9]/;
    if (loadingLabel) {
      await page.waitForFunction(pattern => new RegExp(pattern).test(document.getElementById('status')?.textContent || ''),
        loadingLabel.source, { polling: 20, timeout: 120000 }).then(() => page.locator('#screen').screenshot({ path: path.join(out, `${label}-loading.png`) }))
        .catch(() => {});
    }
    await waitFor('runtime', 300000);
    const loaded = await page.evaluate(() => ({
      readyMs: Math.round(performance.now()), data: Module.gameData, status: document.getElementById('status').textContent,
      statusTexts: window.__statusTexts,
      // Resource Timing: the data file's requests and the body bytes each one carried.
      dataEntries: performance.getEntriesByType('resource').filter(entry => entry.name.endsWith('/uqm-debug.data'))
        .map(entry => ({ type: entry.initiatorType, body: entry.encodedBodySize, transfer: entry.transferSize })),
    }));
    const receivedAtReady = received;
    await page.screenshot({ path: path.join(out, `${label}-ready.png`) });
    const clicked = Date.now();
    await page.locator('#start').click();
    await waitFor('mainMenu', 300000);
    const launchToMenuMs = Date.now() - clicked;
    await sleep(1500);
    await page.locator('#canvas').screenshot({ path: path.join(out, `${label}-menu.png`) });
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
    return { folder, gotoMs, readyMs: loaded.readyMs, launchToMenuMs, methods, dataEntries: loaded.dataEntries,
      receivedAtReady, files, backingStore, caches, gameData: loaded.data, readyStatus: loaded.status, statusTexts: loaded.statusTexts };
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
    const downloadLabel = `Downloading game data: ${MB(size)} / ${MB(size)} MB (100%)`;
    assert.ok(first.statusTexts.includes(downloadLabel), JSON.stringify(first.statusTexts));
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
      assert.match(again.readyStatus, /ready/i);
      // The page says where the data comes from, and never calls the stored copy a download.
      assert.ok(again.statusTexts.includes(`Loading game data saved in this browser: ${MB(size)} / ${MB(size)} MB (100%)`), JSON.stringify(again.statusTexts));
      assert.ok(again.statusTexts.includes('Starting the game…'), JSON.stringify(again.statusTexts));
      assert.ok(!again.statusTexts.some(text => text.startsWith('Downloading')), JSON.stringify(again.statusTexts));
    }
    // Without Cache Storage the engine downloads the file itself, as before the loader.
    const fallback = results.fallback = await visit(edition, edition + '-fallback', { noCacheAPI: true });
    assert.equal(fallback.gameData.source, 'network');
    assert.match(fallback.gameData.reason, /Cache Storage is unavailable/);
    assert.deepEqual(fallback.methods, ['GET']);
    for (const result of [first, results.second, results.rust, fallback]) {
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
      `return visit ${second.readyMs} ms to Ready (median of ${returnReady.length}: ${results.returnReadyMs.median} ms; engine script started at ${second.gameData.engineStartedAt} ms; HEAD ${second.gameData.timings.head} ms, stored copy read by ` +
      `${second.gameData.timings.read} ms)` + (second.receivedAtReady !== null ? `, ${Math.round(second.receivedAtReady / 1024)} KB received` : '') +
      `, then ${second.launchToMenuMs} ms from Launch to the main menu; Rust page ${results.rust.readyMs} ms to Ready; ` +
      `without Cache Storage ${fallback.readyMs} ms to Ready.` +
      (second.backingStore !== null ? ` ArrayBuffers ${MB(second.backingStore)} MB (cache) vs ${MB(fallback.backingStore)} MB (no cache).` : ''));
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
