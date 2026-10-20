// Cosmo Clash on the packaged site, in a real browser: the third card of the
// picker, the game at /cosmoclash/ with all its files from that folder, a short
// battle against the computer (the models, sounds and textures load), Online
// Melee between two pages through the site's own relay and cosmoclash room
// list (create a named room, find it in the list, join, fleet choice, battle,
// leave, back in the lobby), and the way back to the picker. Run with
// `npm run test:cosmoclash-site`.
//
// UQM_DIST=<site> tests a site from scripts/package-editions.py (the deploy
// job's .tools/release-web). Without UQM_DIST the test packages a site from
// COSMO_DIST (default cosmoclash/dist, built with --base /cosmoclash/) and
// small stand-in UQM pages, so it needs no UQM build. These sites are served by
// the Node relay. UQM_SITE=<split site> (deploy/split-data.py output, with
// UQM_SITE_MANIFEST, default deploy/data-parts.json) serves it with the
// Cloudflare Worker under `wrangler dev`. UQM_ORIGIN (for example
// https://urquanmasters.org) tests a deployed site; with UQM_DIST too, the page
// must run the Wasm of that package. On a deployed site the online part runs
// only with COSMO_SITE_ONLINE=1, because its room shows in the players' list.
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
function watch(target, who) {
  target.on('pageerror', error => report.errors.push(who + ': ' + error.message));
  target.on('request', request => report.requests.push(request.url()));
  target.on('response', response => { if (response.status() >= 400) report.failed.push({ who, url: response.url(), status: response.status() }); });
  target.on('websocket', socket => report.sockets.push(socket.url()));
}
const gameState = (target, value, timeout = 30000) => target.waitForFunction(v => window.__cosmo?.state === v, value, { timeout });
const screen = (target, value, timeout = 30000) => target.waitForFunction(v => window.__cosmo?.online?.screen === v, value, { timeout });
const online = target => target.evaluate(() => ({ ...window.__cosmo.online, net: window.__cosmo.netplay }));
const ROOM_PATH = /^\/netplay\/cosmoclash\/([0-9a-f]{32})$/;

/**
 * Online Melee with the game's own screens, between this page (the host) and a
 * second page in its own browser context (the guest), through the relay of the
 * site under test: the Node relay, the Worker, or the deployed site.
 */
async function onlineMelee(host) {
  const ROOM = 'Site check ⚔ קרב';
  const result = report.online = {};
  const firstSocket = report.sockets.length;
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  try {
    const guest = await context.newPage();
    watch(guest, 'guest');
    await guest.goto(origin + '/cosmoclash/');
    assert.ok(await host.evaluate(() => typeof window.__cosmo?.online === 'object'),
      'this Cosmo Clash build has the in-game Online Melee lobby (window.__cosmo.online)');

    // The host opens the lobby from the main menu: the list connects and is empty.
    await host.getByRole('menuitem', { name: 'Online Melee', exact: true }).click();
    await screen(host, 'lobby');
    await host.waitForFunction(() => ['open', 'unsupported', 'error'].includes(window.__cosmo.online.lobby), {}, { timeout: 30000 });
    assert.equal((await online(host)).lobby, 'open', 'the site relay has the cosmoclash room list');
    if (!live) {
      assert.deepEqual((await online(host)).rooms, [], 'a new relay lists no Cosmo Clash rooms');
      assert.equal((await online(host)).focus, 'create');
    }
    await host.screenshot({ path: path.join(out, 'online-lobby-host.png') });

    // Create room: the name screen, a name with spaces to collapse, the waiting screen.
    await host.getByRole('button', { name: 'Create room', exact: true }).click();
    await screen(host, 'name');
    await host.locator('#cosmo-online input').fill('  Site   check ⚔ קרב ');
    await host.keyboard.press('Enter');
    await screen(host, 'waiting');
    assert.equal((await online(host)).room, ROOM);
    await host.screenshot({ path: path.join(out, 'online-waiting-host.png') });

    // The guest: the room is in the list (newest first, with the focus); Enter joins.
    await guest.waitForFunction(() => window.__cosmo?.state === 'menu', {}, { timeout: 90000 });
    await guest.getByRole('menuitem', { name: 'Online Melee', exact: true }).click();
    await screen(guest, 'lobby');
    await guest.waitForFunction(name => window.__cosmo.online.rooms.includes(name), ROOM, { timeout: 30000 });
    const list = await online(guest);
    assert.equal(list.rooms[0], ROOM, 'the newest room is first');
    assert.match(list.focus, /^room:[0-9a-f]{32}$/);
    await guest.screenshot({ path: path.join(out, 'online-lobby-guest.png') });
    await guest.keyboard.press('Enter');
    await Promise.all([screen(host, 'fleet'), screen(guest, 'fleet')]);
    await guest.screenshot({ path: path.join(out, 'online-fleet-guest.png') });

    // The real sockets: one listed host room (name and engine=rust), the guest in the same room.
    const sockets = report.sockets.slice(firstSocket).map(url => new URL(url));
    const rooms = sockets.filter(url => ROOM_PATH.test(url.pathname));
    const hosts = rooms.filter(url => url.searchParams.get('role') === 'host');
    const guests = rooms.filter(url => url.searchParams.get('role') === 'guest');
    assert.equal(hosts.length, 1, JSON.stringify(sockets.map(String)));
    assert.equal(guests.length, 1, JSON.stringify(sockets.map(String)));
    assert.equal(hosts[0].searchParams.get('name'), ROOM);
    assert.equal(hosts[0].searchParams.get('engine'), 'rust');
    assert.equal(guests[0].pathname, hosts[0].pathname, 'the guest joins the listed room');
    assert.ok(sockets.filter(url => url.pathname === '/netplay/lobby').every(url => url.search === '?edition=cosmoclash'));
    assert.ok(sockets.filter(url => url.pathname === '/netplay/lobby').length >= 2, 'both pages watched the cosmoclash room list');
    result.room = hosts[0].pathname;

    // Fleets: one ship each, Ready; the countdown; a battle with checksums that both pages verify.
    await host.waitForTimeout(2600);
    for (const target of [host, guest]) {
      await target.keyboard.press('Enter');
      await target.waitForFunction(() => window.__cosmo.online.picks.length === 1);
      await target.keyboard.press('ArrowDown');
      await target.keyboard.press('Enter');
      await target.waitForFunction(() => window.__cosmo.netplay?.localReady === true, {}, { timeout: 30000 });
    }
    await Promise.all([screen(host, 'battle', 60000), screen(guest, 'battle', 60000)]);
    await Promise.all([host, guest].map(target => target.waitForFunction(() => window.__cosmo.netplay.verifiedChecksums >= 2, {}, { timeout: 60000 })));
    const battle = await Promise.all([host, guest].map(online));
    for (const state of battle) assert.ok(!['error', 'closed'].includes(state.net.phase), JSON.stringify(state.net));
    result.verifiedChecksums = battle.map(state => state.net.verifiedChecksums);
    await host.screenshot({ path: path.join(out, 'online-battle-host.png') });

    // The host leaves (Esc, then Leave): both pages are back in the lobby and say why.
    await host.keyboard.press('Escape');
    await host.waitForFunction(() => window.__cosmo.online.confirm);
    await host.getByRole('button', { name: 'Leave', exact: true }).click();
    await Promise.all([screen(host, 'lobby'), screen(guest, 'lobby')]);
    await host.waitForFunction(() => window.__cosmo.online.notice === 'You left the room.');
    await guest.waitForFunction(() => window.__cosmo.online.notice === 'Your opponent left the room.');
    await guest.waitForFunction(() => window.__cosmo.online.lobby === 'open');
    assert.ok(!(await online(guest)).rooms.includes(ROOM), 'the room left the list');
    await guest.screenshot({ path: path.join(out, 'online-lobby-opponent-left-guest.png') });

    // Esc goes back to the main menu on both pages.
    for (const target of [host, guest]) {
      await target.keyboard.press('Escape');
      await gameState(target, 'menu');
    }
    result.passed = true;
  } finally {
    await context.close();
  }
}

let browser;
try {
  browser = await chromium.launch({ headless: process.env.HEADED !== '1',
    executablePath: process.env.CHROME_BIN || (process.platform === 'darwin' ? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' : undefined),
    // Two game pages run at once (Online Melee): no throttling of the one behind.
    args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--autoplay-policy=no-user-gesture-required',
      '--disable-background-timer-throttling', '--disable-backgrounding-occluded-windows', '--disable-renderer-backgrounding'] });
  const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
  watch(page, 'host');

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
  assert.ok(report.requests.includes(origin + '/cosmoclash/cosmoclash.wasm'));

  // A short battle against the computer: the ships, sounds and textures load from
  // /cosmoclash/ (the game puts the base in front of the root paths of its code).
  await page.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click();
  await page.getByText('Easy', { exact: true }).click();
  await gameState(page, 'selection');
  await page.keyboard.press('Enter');
  await gameState(page, 'playing');
  for (const key of ['ArrowUp', '/', '.']) await page.keyboard.down(key);
  await page.waitForFunction(() => window.__cosmo.snapshot?.tick > 150, {}, { timeout: 30000 });
  for (const key of ['ArrowUp', '/', '.']) await page.keyboard.up(key);
  const solo = await page.evaluate(() => window.__cosmo.snapshot);
  assert.ok(solo.aiController, 'the computer flies the other ship');
  await page.waitForLoadState('networkidle');
  await page.screenshot({ path: path.join(out, 'game-battle.png') });
  const loaded = report.requests.map(url => new URL(url)).filter(url => url.origin === origin).map(url => url.pathname);
  for (const [kind, pattern] of [['ship models', /^\/cosmoclash\/models\/[^/]+\.glb$/], ['sounds', /^\/cosmoclash\/sounds\/[^/]+\.(?:ogg|wav)$/],
    ['textures', /^\/cosmoclash\/textures\/[^/]+\.(?:png|jpg)$/]]) {
    assert.ok(loaded.some(name => pattern.test(name)), 'the battle loads its ' + kind + ' from /cosmoclash/');
  }
  report.battleFiles = [...new Set(loaded)].sort();
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => window.__cosmo.paused);
  await page.getByRole('button', { name: 'Main menu', exact: true }).click();
  await gameState(page, 'menu');

  // Online Melee between two pages through this site's relay and cosmoclash room list.
  if (!live || process.env.COSMO_SITE_ONLINE === '1') await onlineMelee(page);
  else report.online = 'skipped on a deployed site (COSMO_SITE_ONLINE=1 runs it)';

  const sameOrigin = report.requests.filter(url => url.startsWith(origin + '/'));
  assert.deepEqual(sameOrigin.filter(url => !url.startsWith(origin + '/cosmoclash/')), [], 'the game loads its files from /cosmoclash/ only');
  // The game itself loads nothing from other origins. A deployed site can have edge
  // additions (analytics) that the zone adds; they are recorded, not refused.
  report.otherOrigins = report.requests.filter(url => /^https?:/.test(url) && !url.startsWith(origin + '/'));
  if (!live) assert.deepEqual(report.otherOrigins, [], 'the game loads nothing from other origins');
  // Online play uses this game's room list and rooms only.
  for (const url of report.sockets) {
    assert.match(new URL(url).pathname + new URL(url).search,
      /^\/netplay\/(?:lobby\?edition=cosmoclash|cosmoclash\/[0-9a-f]{32}\?(?:role=host&name=[^&]+&engine=rust|role=host|role=guest))$/, url);
  }

  // Back to the picker: with the game's own menu item when it has one, else with
  // the browser's Back. The choice is kept. /cosmoclash (no slash) opens the game too.
  const chooseGame = page.locator('#main-menu').getByRole('menuitem', { name: /choose game/i })
    .or(page.locator('#main-menu').getByRole('link', { name: /choose game/i }));
  if (await chooseGame.count()) {
    await Promise.all([page.waitForURL(url => url.origin === origin && ['/', '/index.html'].includes(url.pathname)), chooseGame.first().click()]);
    report.chooseGame = 'menu item';
  } else {
    await page.goBack();
    report.chooseGame = 'missing: the main menu has no Choose game item, the test used Back';
    console.warn('Cosmo Clash: ' + report.chooseGame);
  }
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
  console.log(`Cosmo Clash at ${origin} (${report.server}): picker card, keyboard choice, game at /cosmoclash/ with isolation, ` +
    `Wasm ${engine.slice(0, 12)}, a battle vs the computer, online: ${typeof report.online === 'object' ? 'room list, join, battle, leave' : report.online}, ` +
    `way back (${report.chooseGame}).`);
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
