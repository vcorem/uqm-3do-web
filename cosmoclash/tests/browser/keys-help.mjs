// The keys outside the game screen (src/ui/KeysHelp.ts) in real browsers with
// real keys and a real relay: on wide windows a panel in each dark margin
// beside the 4:3 game (shown at first, the choice remembered after a reload),
// on 4:3 windows a pop-up over the game that only K or the Keys (K) button
// opens; K or Esc closes it, and that Esc does not also pause. In every
// screen K toggles without pausing: the main menu, a battle against the
// computer, a Two Players battle (also K with Shift, the special key of
// Player 2, held), the online lobby, the room name (a K typed there is a
// letter), the fleet choice and an online battle (the lockstep goes on).
// The screenshots are at 1280x720 and 1920x1080 (side panels) and 1024x768
// (the pop-up), also in full screen (document.fullscreenElement is
// #cosmo-frame), in test-results/keys-help/. Each one measures that the side
// panels and the button never overlap #game-container, that their text is not
// cut, and that the game frame has the same size with and without them.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createNetplayServer } from '../../../server/netplay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch, poll, read, screen, watchPage, wasmHash, textOutside, settle, screenshot } from './helpers.mjs';

const output = path.resolve(root, process.env.COSMO_KEYS_HELP_DIR || 'test-results/keys-help');
await fs.rm(output, { recursive: true, force: true });
await fs.mkdir(output, { recursive: true });
const SIZES = [[1280, 720], [1920, 1080], [1024, 768]];
const shots = [];
const result = { passed: false, checks: {}, layouts: [], shots };
const sink = { errors: [], requests: [], sockets: [] };
let counter = 0;
const expectedHash = await wasmHash();

/** The layout of the frame, the game and the keys (rectangles in CSS pixels). */
const measure = page => page.evaluate(() => {
  const rect = node => { if (!node || node.hidden || !node.getClientRects().length) return null; const r = node.getBoundingClientRect(); return { left: r.left, top: r.top, right: r.right, bottom: r.bottom, width: r.width, height: r.height }; };
  const panels = ['cosmo-keys-flying', 'cosmo-keys-menus'].map(id => document.getElementById(id));
  // Text that is cut or goes out of its panel (a panel may scroll down, never sideways).
  const cut = [];
  for (const panel of panels) {
    if (!rect(panel)) continue;
    const box = panel.getBoundingClientRect();
    if (panel.scrollWidth > panel.clientWidth + 1) cut.push(panel.id + ' scrolls sideways');
    for (const node of panel.querySelectorAll('*')) {
      const r = node.getBoundingClientRect();
      if (r.width && (r.left < box.left - 0.5 || r.right > box.right + 0.5)) cut.push(node.textContent.trim().slice(0, 40));
    }
  }
  const fonts = panels.filter(rect).flatMap(panel => [...panel.querySelectorAll('dd, span, p, h3, kbd')].map(node => parseFloat(getComputedStyle(node).fontSize)));
  return {
    viewport: { width: innerWidth, height: innerHeight },
    fullscreen: document.fullscreenElement?.id ?? null,
    game: rect(document.getElementById('game-container')),
    panels: panels.map(rect), button: rect(document.getElementById('cosmo-keys-button')),
    buttonInGame: document.getElementById('cosmo-keys-button')?.parentElement?.id === 'game-container',
    pop: rect(document.getElementById('cosmo-keys-pop')),
    scrolls: panels.filter(rect).map(panel => panel.scrollHeight > panel.clientHeight + 1 ? getComputedStyle(panel).overflowY : 'fits'),
    minFont: fonts.length ? Math.min(...fonts) : null,
    cut,
    keys: window.__cosmo.keys,
    expanded: document.getElementById('cosmo-keys-button')?.getAttribute('aria-expanded'),
  };
});
const overlaps = (a, b) => a.left < b.right - 0.5 && b.left < a.right - 0.5 && a.top < b.bottom - 0.5 && b.top < a.bottom - 0.5;
const inside = (a, b) => a.left >= b.left - 0.5 && a.right <= b.right + 0.5 && a.top >= b.top - 0.5 && a.bottom <= b.bottom + 0.5;
const frameBox = view => ({ left: 0, top: 0, right: view.width, bottom: view.height });

/** One press of K, and the page answered it (the help shows or hides). */
async function pressK(page, key = 'k') {
  const before = await page.evaluate(() => window.__cosmo.keys.visible);
  await page.keyboard.press(key);
  await page.waitForFunction(v => window.__cosmo.keys.visible !== v, before);
  await settle(page, 60);
  return !before;
}
/**
 * The side panels beside the game: shown, never over the game, inside the
 * window, not cut, readable; hiding them does not move or resize the game.
 */
async function checkSide(page, label) {
  const shown = await measure(page);
  assert.equal(shown.keys.side, true, `${label}: side panels`);
  assert.equal(shown.keys.visible, true, `${label}: the side panels show`);
  assert.equal(shown.expanded, 'true');
  assert.equal(shown.buttonInGame, false, `${label}: the button is beside the game`);
  for (const box of [...shown.panels, shown.button]) {
    assert.ok(box, `${label}: a panel or the button is not shown`);
    assert.ok(!overlaps(box, shown.game), `${label}: over the game ${JSON.stringify({ box, game: shown.game })}`);
    assert.ok(inside(box, frameBox(shown.viewport)), `${label}: out of the window ${JSON.stringify(box)}`);
  }
  assert.deepEqual(shown.cut, [], `${label}: cut text`);
  assert.ok(shown.minFont >= 11, `${label}: text size ${shown.minFont}`);
  assert.ok(shown.scrolls.every(value => value === 'fits' || value === 'auto'), `${label}: ${shown.scrolls}`);
  assert.equal(shown.pop, null, `${label}: no pop-up beside the panels`);
  // K hides them: the game frame keeps its place and size; K shows them again.
  assert.equal(await pressK(page), false);
  const hidden = await measure(page);
  assert.deepEqual(hidden.panels, [null, null]);
  assert.equal(hidden.expanded, 'false');
  assert.deepEqual(hidden.game, shown.game, `${label}: the game moved or changed its size`);
  assert.ok(hidden.button && !overlaps(hidden.button, hidden.game), `${label}: the button stays beside the game`);
  assert.equal(await pressK(page), true);
  assert.deepEqual((await measure(page)).game, shown.game);
  result.layouts.push({ label, game: shown.game, panels: shown.panels, button: shown.button, scrolls: shown.scrolls, minFont: shown.minFont, fullscreen: shown.fullscreen });
  return shown;
}
/** Without room beside the game: no panel; K opens the pop-up over the game (all its text inside the game frame). */
async function openPop(page, label) {
  const closed = await measure(page);
  assert.equal(closed.keys.side, false, `${label}: no room beside the game`);
  assert.equal(closed.keys.visible, false, `${label}: the pop-up does not open by itself`);
  assert.deepEqual([closed.panels, closed.pop], [[null, null], null]);
  assert.ok(closed.buttonInGame && inside(closed.button, closed.game), `${label}: the button is in the corner of the game`);
  assert.equal(await pressK(page), true);
  const open = await measure(page);
  assert.ok(open.pop && inside(open.pop, open.game), `${label}: the pop-up is over the game ${JSON.stringify(open.pop)}`);
  assert.equal(open.expanded, 'true');
  assert.deepEqual(await textOutside(page), [], `${label}: text outside the game frame`);
  assert.deepEqual(open.game, closed.game);
  result.layouts.push({ label, game: open.game, pop: open.pop, fullscreen: open.fullscreen });
}
/** Esc closes the pop-up only (it does not pause, resume or ask to leave). */
async function escapePop(page, label) {
  const before = await read(page), dialogs = await page.locator('.cc-dialog').count();
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => !window.__cosmo.keys.visible);
  await settle(page, 60);
  const after = await read(page);
  assert.equal(after.state, before.state, `${label}: Esc closed the keys only`);
  assert.equal(after.paused, before.paused, `${label}: Esc did not pause or resume`);
  assert.equal(after.online?.confirm ?? false, false, `${label}: Esc did not ask to leave`);
  assert.equal(await page.locator('.cc-dialog').count(), dialogs, `${label}: no dialog opened or closed`);
}
/**
 * The screen at 1280x720 and 1920x1080 (side panels) and 1024x768 (the
 * pop-up, opened with K for the screenshot and closed with Esc); back to 1280x720.
 */
async function capture(page, name, options = {}) {
  const n = String(++counter).padStart(2, '0');
  for (const [width, height] of options.sizes ?? SIZES) {
    await page.setViewportSize({ width, height });
    await settle(page, 150);
    const label = `${name} ${width}x${height}`;
    if (options.fullscreen) assert.equal((await read(page)).fullscreen, 'cosmo-frame', `${label}: full screen`);
    if (width / height > 1.5) await checkSide(page, label); else await openPop(page, label);
    if (options.check) await options.check(width, height);
    const file = path.join(output, `${n}-${name}-${width}x${height}.png`);
    await screenshot(page, file);
    shots.push(path.relative(root, file));
    if (width / height <= 1.5) await escapePop(page, label);
  }
  await page.setViewportSize({ width: 1280, height: 720 });
  await settle(page, 100);
}
const waitState = (page, value, timeout = 20000) => page.waitForFunction(v => window.__cosmo?.state === v, value, { timeout });
/** The game goes on: more simulation ticks (local) or lockstep ticks (online) after a while. */
async function goesOn(page, online = false) {
  const tick = async () => online ? (await read(page)).net.tick : (await read(page)).snapshot.tick;
  const start = await tick();
  await poll(async () => (await tick()) > start + 10, 15000, 'the game goes on');
}

const servers = [], browsers = [];
try {
  const app = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createNetplayServer({ distDir: false }) });
  servers.push(app);
  const { port } = await app.listen(0);
  const url = `http://127.0.0.1:${port}/cosmoclash/`;
  browsers.push(await launch());
  const context = await browsers[0].newContext({ viewport: { width: 1280, height: 720 } });
  const A = await context.newPage();
  watchPage(A, 0, sink);
  async function boot(visible) {
    await A.goto(url);
    await A.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 90000 });
    assert.equal((await read(A)).engine, expectedHash);
    assert.equal((await A.evaluate(() => window.__cosmo.keys)).visible, visible);
  }
  // The side panels show from the start, also beside the logo; K does not skip the logo.
  await boot(true);
  assert.equal(await pressK(A), false);
  assert.equal(await pressK(A), true);
  assert.equal((await read(A)).state, 'logo', 'K does not skip the logo');
  await A.keyboard.press('Enter');
  await waitState(A, 'menu');

  // ------------------------------------------------------------ the main menu (not in full screen)
  assert.equal(await A.locator('.menu-keys').textContent(), '↑ ↓ choose · Enter start · F full screen · K keys');
  assert.deepEqual(await A.evaluate(() => window.__cosmo.keys), { side: true, visible: true, mode: 'menu' });
  assert.equal(await A.locator('#cosmo-keys-flying').getAttribute('aria-label'), 'Keys: flying and ships');
  assert.equal(await A.locator('#cosmo-keys-button').getAttribute('aria-controls'), 'cosmo-keys-flying cosmo-keys-menus');
  assert.deepEqual(await A.locator('#cosmo-keys-flying h3').allTextContents(), ['Player 1', 'Player 2', 'Ships']);
  // A held K toggles once; the live region says it.
  await A.keyboard.down('k'); await A.keyboard.down('k'); await A.keyboard.down('k'); await A.keyboard.up('k');
  await A.waitForTimeout(200);
  assert.equal((await A.evaluate(() => window.__cosmo.keys)).visible, false, 'a held K toggles once');
  assert.equal((await read(A)).live, 'Keys hidden. Press K to show them.');
  // The choice is remembered: hidden after a reload; the button shows them again.
  await boot(false);
  await A.keyboard.press('Enter');
  await waitState(A, 'menu');
  await A.locator('#cosmo-keys-button').click();
  assert.equal((await A.evaluate(() => window.__cosmo.keys)).visible, true);
  assert.equal(await A.evaluate(() => document.activeElement?.textContent), 'Play vs Computer', 'the button does not take the focus of the menu');
  assert.equal((await read(A)).live, 'Keys shown beside the game.');
  // The pop-up also opens with the button and closes with Close.
  await A.setViewportSize({ width: 1024, height: 768 }); await settle(A);
  await A.locator('#cosmo-keys-button').click();
  await A.waitForFunction(() => window.__cosmo.keys.visible);
  assert.equal(await A.locator('#cosmo-keys-pop').getAttribute('role'), 'dialog');
  await A.getByRole('button', { name: 'Close (K)', exact: true }).click();
  await A.waitForFunction(() => !window.__cosmo.keys.visible);
  await A.setViewportSize({ width: 1280, height: 720 }); await settle(A);
  await capture(A, 'menu');
  result.checks.menu = true;

  // ------------------------------------------------------------ vs Computer, in full screen
  await A.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click();
  await A.getByText('Easy', { exact: true }).click();
  await waitState(A, 'selection');
  assert.equal((await read(A)).fullscreen, 'cosmo-frame', 'a menu choice puts the game frame in full screen');
  assert.equal((await A.evaluate(() => window.__cosmo.keys)).mode, 'ai');
  assert.deepEqual(await A.locator('#cosmo-keys-flying h3').allTextContents(), ['Your ship', 'Ships']);
  assert.match(await A.locator('#cosmo-keys-flying .ck-mode').textContent(), /vs Computer You fly with the Player 1 keys\./);
  await A.keyboard.press('Enter');
  await waitState(A, 'playing');
  // K in the battle: nothing pauses, the computer flies on.
  await pressK(A); await goesOn(A); await pressK(A);
  assert.equal((await read(A)).paused, false);
  await capture(A, 'vs-computer-battle-fullscreen', { fullscreen: true, check: () => goesOn(A) });
  // Paused: the hint mentions K.
  await A.keyboard.press('Escape');
  await A.waitForFunction(() => window.__cosmo.paused);
  assert.equal(await A.locator('.cc-dialog .cc-hint').textContent(), 'Enter choose · Esc resume · K keys');
  await capture(A, 'vs-computer-paused-fullscreen', { fullscreen: true, sizes: [[1280, 720], [1024, 768]] });
  assert.equal((await read(A)).paused, true, 'Esc closed the pop-up and the battle stays paused');
  await A.getByRole('button', { name: 'Main menu', exact: true }).click();
  await waitState(A, 'menu');
  await capture(A, 'menu-fullscreen', { fullscreen: true });
  result.checks.vsComputer = true;

  // ------------------------------------------------------------ Two Players (F: not in full screen)
  await A.keyboard.press('f');
  await A.waitForFunction(() => window.__cosmo.fullscreen === null);
  await A.keyboard.press('ArrowDown');
  await A.keyboard.press('Enter');
  await waitState(A, 'fleet');
  assert.equal((await A.evaluate(() => window.__cosmo.keys)).mode, 'local');
  assert.match(await A.locator('#cosmo-keys-menus').textContent(), /only Enter selects/);
  // K in the fleet choice changes nothing there.
  const pick = (await read(A)).pick;
  await pressK(A); await pressK(A);
  assert.deepEqual((await read(A)).pick, pick);
  await A.keyboard.press('Enter'); await A.keyboard.press('ArrowDown'); await A.keyboard.press('Enter');
  await A.waitForFunction(() => window.__cosmo.pick?.player === 0);
  await A.keyboard.press('ArrowRight'); await A.keyboard.press('Enter'); await A.keyboard.press('ArrowDown'); await A.keyboard.press('Enter');
  await waitState(A, 'playing');
  // K with Shift held (the special key of Player 2) toggles too; the battle goes on.
  await A.keyboard.down('Shift');
  assert.equal(await pressK(A, 'K'), false);
  await A.keyboard.up('Shift');
  await goesOn(A);
  await pressK(A);
  await capture(A, 'two-players-battle', { check: () => goesOn(A) });
  await A.keyboard.press('Escape');
  await A.waitForFunction(() => window.__cosmo.paused);
  await A.getByRole('button', { name: 'Main menu', exact: true }).click();
  await waitState(A, 'menu');
  result.checks.twoPlayers = true;

  // ------------------------------------------------------------ Online Melee: lobby, room name, fleet, battle
  browsers.push(await launch());
  const B = await browsers[1].newPage({ viewport: { width: 1280, height: 720 } });
  watchPage(B, 1, sink);
  await B.goto(url);
  await B.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 90000 });
  await B.keyboard.press('Enter');
  await waitState(B, 'menu');
  await A.getByRole('menuitem', { name: 'Online Melee', exact: true }).click();
  await screen(A, 'lobby');
  await A.waitForFunction(() => window.__cosmo.online.lobby === 'open');
  assert.equal((await A.evaluate(() => window.__cosmo.keys)).mode, 'online');
  assert.match(await A.locator('#cosmo-keys-flying .ck-mode').textContent(), /Both players fly with the Player 1 keys, each on their own computer\./);
  await capture(A, 'online-lobby');
  // The room name: a K typed there is a letter (the keys stay as they are).
  await A.getByRole('button', { name: 'Create room', exact: true }).click();
  await screen(A, 'name');
  await A.locator('#cosmo-online input').fill('');
  await A.keyboard.type('Kk duel');
  assert.equal(await A.locator('#cosmo-online input').inputValue(), 'Kk duel');
  assert.equal((await A.evaluate(() => window.__cosmo.keys)).visible, true, 'K typed in the room name is a letter');
  // Over the game (no room beside it): the button opens the pop-up, the field keeps the focus, Esc closes the pop-up only.
  await A.setViewportSize({ width: 1024, height: 768 }); await settle(A);
  await A.locator('#cosmo-keys-button').click();
  await A.waitForFunction(() => window.__cosmo.keys.visible);
  assert.equal(await A.evaluate(() => document.activeElement?.tagName), 'INPUT');
  await A.keyboard.press('Escape');
  await A.waitForFunction(() => !window.__cosmo.keys.visible);
  await A.waitForTimeout(200);
  assert.equal((await read(A)).online.screen, 'name', 'Esc closed the keys, not the room name');
  assert.equal(await A.locator('#cosmo-online input').inputValue(), 'Kk duel');
  await A.setViewportSize({ width: 1280, height: 720 }); await settle(A);
  await A.keyboard.press('Enter');
  await screen(A, 'waiting');
  await B.getByRole('menuitem', { name: 'Online Melee', exact: true }).click();
  await screen(B, 'lobby');
  await B.waitForFunction(() => window.__cosmo.online.rooms.includes('Kk duel'), {}, { timeout: 30000 });
  await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'fleet'), screen(B, 'fleet')]);
  await A.waitForTimeout(800);
  const fleet = (await read(A)).online;
  await pressK(A); await pressK(A);
  assert.deepEqual([(await read(A)).online.picks, (await read(A)).online.focus], [fleet.picks, fleet.focus], 'K changes nothing in the fleet choice');
  await capture(A, 'online-fleet');
  for (const page of [A, B]) {
    await page.keyboard.press('Enter');
    await page.waitForFunction(() => window.__cosmo.online.picks.length === 1);
    await page.keyboard.press('ArrowDown');
    await page.keyboard.press('Enter');
    await page.waitForFunction(() => window.__cosmo.netplay?.localReady === true, {}, { timeout: 30000 });
  }
  await Promise.all([screen(A, 'battle', 60000), screen(B, 'battle', 60000)]);
  // K in an online battle: the lockstep goes on on both pages, and the checksums agree.
  const checksums = (await read(A)).net.verifiedChecksums;
  await pressK(A); await Promise.all([goesOn(A, true), goesOn(B, true)]);
  await pressK(A); await Promise.all([goesOn(A, true), goesOn(B, true)]);
  await A.waitForFunction(n => window.__cosmo.netplay.verifiedChecksums > n, checksums, { timeout: 30000 });
  assert.deepEqual(await Promise.all([A, B].map(async page => (await read(page)).online.screen)), ['battle', 'battle']);
  await capture(A, 'online-battle', { sizes: [[1280, 720], [1024, 768]], check: () => goesOn(A, true) });
  for (const state of await Promise.all([A, B].map(read))) assert.ok(!['error', 'closed'].includes(state.net.phase), JSON.stringify(state.net));
  result.checks.online = true;

  assert.deepEqual(sink.errors, [], 'page errors');
  assert.deepEqual(sink.requests, [], 'failed requests');
  result.passed = true;
} finally {
  for (const browser of browsers) await browser.close().catch(() => {});
  for (const server of servers) await server.close?.().catch?.(() => {});
  await fs.writeFile(path.join(output, 'results.json'), JSON.stringify({ ...result, errors: sink.errors, requests: sink.requests }, null, 2));
}
console.log(JSON.stringify({ passed: result.passed, checks: result.checks, shots: shots.length }));
process.exit(result.passed ? 0 : 1);
