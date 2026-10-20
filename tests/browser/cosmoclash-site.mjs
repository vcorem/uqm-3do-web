// Cosmo Clash on the packaged site, in a real browser: the third card of the
// picker, the game at /cosmoclash/ with all its files from that folder, and
// the way back. Run with `npm run test:cosmoclash-site`.
//
// UQM_DIST=<site> tests a site from scripts/package-editions.py (the deploy
// job's .tools/release-web). Without UQM_DIST the test packages a site from
// COSMO_DIST (default cosmoclash/dist, built with --base /cosmoclash/) and
// small stand-in UQM pages, so it needs no UQM build. These sites are served by
// the Node relay. UQM_SITE=<split site> (deploy/split-data.py output, with
// UQM_SITE_MANIFEST, default deploy/data-parts.json) serves it with the
// Cloudflare Worker under `wrangler dev`. UQM_ORIGIN (for example
// https://urquanmasters.org) tests a deployed site; with UQM_DIST too, the page
// must run the Wasm of that package.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';
import { startSiteWorker } from '../helpers/site-worker.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const out = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/cosmoclash-site');
await mkdir(out, { recursive: true });

async function standInSite() {
  const temp = await mkdtemp(path.join(os.tmpdir(), 'uqm-cosmo-site-'));
  const files = ['index.html', 'uqm-debug.js', 'uqm-debug.wasm', 'uqm-debug.data', 'uqm-debug.worker.js', 'COPYING.txt',
    'toolchain-fixes.json', 'game-help.js', 'engine-loader.js', 'online-ui.js', 'online-art.json', 'online-art.png',
    'online-stars.png', 'online-title.png', 'netplay-client.js', 'content-licenses.txt'];
  for (const edition of ['original', 'hd']) {
    await mkdir(path.join(temp, edition));
    for (const name of files) {
      await writeFile(path.join(temp, edition, name), name === 'index.html'
        ? `<!doctype html><head><title>UQM ${edition}</title></head><header>Stand-in ${edition} page</header>` : name);
    }
  }
  const cosmo = path.resolve(process.env.COSMO_DIST || path.join(root, 'cosmoclash/dist'));
  execFileSync('python3', [path.join(root, 'scripts/package-editions.py'), '--original', path.join(temp, 'original'),
    '--hd', path.join(temp, 'hd'), '--cosmoclash', cosmo, '--output', path.join(temp, 'site')], { stdio: 'inherit' });
  return { site: path.join(temp, 'site'), cleanup: () => rm(temp, { recursive: true, force: true }) };
}

const live = process.env.UQM_ORIGIN ? new URL(process.env.UQM_ORIGIN).origin : null;
const workerSite = !live && process.env.UQM_SITE ? path.resolve(process.env.UQM_SITE) : null;
const packaged = process.env.UQM_DIST ? { site: path.resolve(process.env.UQM_DIST), cleanup: async () => {} }
  : live || workerSite ? null : await standInSite();
const site = packaged?.site ?? workerSite;
const wasmHash = site && createHash('sha256').update(await readFile(path.join(site, 'cosmoclash/cosmoclash.wasm'))).digest('hex');
const worker = workerSite && await startSiteWorker({ site: workerSite,
  manifest: process.env.UQM_SITE_MANIFEST ? path.resolve(process.env.UQM_SITE_MANIFEST) : undefined });
const server = live || worker ? null : createNetplayServer({ distDir: site });
const origin = live ?? worker?.origin ?? `http://127.0.0.1:${(await server.listen(0, '127.0.0.1')).port}`;
const report = { origin, server: live ? 'deployed' : worker ? 'worker' : 'node', site, wasmHash,
  requests: [], failed: [], errors: [], sockets: [], passed: false };
let browser;
try {
  browser = await chromium.launch({ headless: process.env.HEADED !== '1',
    executablePath: process.env.CHROME_BIN || (process.platform === 'darwin' ? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' : undefined),
    args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--autoplay-policy=no-user-gesture-required'] });
  const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('request', request => report.requests.push(request.url()));
  page.on('response', response => { if (response.status() >= 400) report.failed.push({ url: response.url(), status: response.status() }); });
  page.on('websocket', socket => report.sockets.push(socket.url()));

  // The picker: three cards; nothing of the game loads before a choice, except the card art.
  await page.goto(origin + '/');
  await page.waitForFunction(() => typeof UQMEditions === 'object');
  const card = page.locator('label[for="edition-cosmoclash"]');
  await card.waitFor();
  assert.match(await card.textContent(), /Cosmo\s*Clash[\s\S]*by Ori Hanegby, Guy Corem and Orr Kowarsky/);
  assert.match(await card.textContent(), /Against the computer[\s\S]*Two players on one keyboard[\s\S]*Online/);
  assert.ok(await page.locator('.card-art img').evaluate(image => image.complete && image.naturalWidth > 0), 'the card art loads');
  assert.equal(await page.locator('input[name="edition"]').count(), 3);
  assert.deepEqual(report.requests.filter(url => new URL(url).pathname.startsWith('/cosmoclash/')), [],
    'the picker loads no Cosmo Clash file before a choice');
  assert.ok(report.requests.every(url => !/\.(wasm|data)$/.test(new URL(url).pathname)), 'the picker loads no engine');
  await page.screenshot({ path: path.join(out, 'picker.png'), fullPage: true });

  // Choose the card with the keyboard, as a player can: Original -> HD -> Cosmo Clash.
  await page.locator('#edition-original').focus();
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  assert.equal(await page.locator('#edition-cosmoclash').isChecked(), true);
  assert.equal(await page.locator('#launch-label').textContent(), 'Play Cosmo Clash');
  assert.equal(await page.locator('#engine-choice').isHidden(), true, 'no engine switch for Cosmo Clash');
  await page.screenshot({ path: path.join(out, 'picker-cosmoclash.png'), fullPage: true });
  report.requests.length = 0;
  await Promise.all([page.waitForURL(origin + '/cosmoclash/'), page.locator('#launch-edition').click()]);

  // The game: menu reached, the packaged Wasm runs, and every file came from /cosmoclash/.
  await page.waitForFunction(() => window.__cosmo?.state === 'menu', {}, { timeout: 90000 });
  const engine = await page.evaluate(() => window.__cosmo.engine);
  if (wasmHash) assert.equal(engine, wasmHash, 'the page runs the packaged Wasm');
  else assert.match(engine, /^[0-9a-f]{64}$/);
  assert.equal(await page.evaluate(() => crossOriginIsolated), true, 'the site isolation headers do not stop the game');
  await page.waitForLoadState('networkidle');
  await page.screenshot({ path: path.join(out, 'game-menu.png') });
  const outside = report.requests.filter(url => !url.startsWith(origin + '/cosmoclash/') && !url.startsWith('blob:') && !url.startsWith('data:'));
  assert.deepEqual(outside, [], 'the game loads files only from /cosmoclash/');
  assert.ok(report.requests.includes(origin + '/cosmoclash/cosmoclash.wasm'));
  // Online play uses this game's room list and rooms only.
  for (const url of report.sockets) {
    assert.match(new URL(url).pathname + new URL(url).search, /^\/netplay\/(?:lobby\?edition=cosmoclash|cosmoclash\/[0-9a-f]{32}\?)/, url);
  }

  // Back to the picker: the choice is kept. /cosmoclash (no slash) opens the game too.
  await page.goBack();
  await page.waitForFunction(() => document.getElementById('edition-cosmoclash')?.checked === true);
  assert.equal(await page.locator('#launch-edition').isDisabled(), false);
  // Only the redirect: opening the game with this invitation would ask the relay for a room.
  const redirect = await page.request.get(origin + '/cosmoclash?cosmoRoom=0123456789abcdef0123456789abcdef', { maxRedirects: 0 });
  assert.equal(redirect.status(), 301);
  assert.equal(new URL(redirect.headers().location, origin).href, origin + '/cosmoclash/?cosmoRoom=0123456789abcdef0123456789abcdef');
  await page.goto(origin + '/');
  await page.locator('#edition-hd').check();
  const offersRust = await page.locator('#edition-form').getAttribute('data-engines') === 'c,rust';
  assert.equal(await page.locator('#engine-choice').isVisible(), offersRust, 'the engine switch is back for HD');

  assert.deepEqual(report.failed, []);
  assert.deepEqual(report.errors, []);
  report.passed = true;
  console.log(`Cosmo Clash at ${origin} (${report.server}): picker card, keyboard choice, game at /cosmoclash/ with isolation, Wasm ${engine.slice(0, 12)}, way back.`);
} catch (error) {
  report.failure = error.stack;
  throw error;
} finally {
  await writeFile(path.join(out, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  await browser?.close();
  await server?.close();
  await worker?.close();
  await packaged?.cleanup();
}
