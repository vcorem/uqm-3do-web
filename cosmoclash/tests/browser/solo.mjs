// Local play in a real browser: Play vs Computer (AI, morph, audio, mute),
// Two Players on one computer (shield, slime, a real battle to the game
// over screen), the menus with the keyboard and the mouse, Esc pause, and
// the removed debug keys. The game is served at /cosmoclash/.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch, read, setKeys, botKeys, wasmHash } from './helpers.mjs';

const output = path.resolve(root, process.env.COSMO_BROWSER_RESULTS_DIR || 'test-results/browser');
await fs.mkdir(output, { recursive: true });
const expectedHash = await wasmHash();
const app = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createRustRelay({ distDir: false }), lobby: 'relay' });
const address = await app.listen(0);
let browser, page;
const result = { passed: false, wasmSha256: expectedHash };
const errors = [], failed = [];
const state = () => page.evaluate(() => window.__cosmo?.state);
const waitState = (value, timeout = 10000) => page.waitForFunction(v => window.__cosmo?.state === v, value, { timeout });
const active = () => page.evaluate(() => document.activeElement?.textContent?.trim() ?? '');
const P2 = { ArrowLeft: 'a', ArrowRight: 'd', ArrowUp: 'w', '/': ' ', '.': 'Shift' };
try {
  browser = await launch();
  page = await browser.newPage({ viewport: { width: 960, height: 720 } });
  // Observe the actual master output without replacing audio generation or game code.
  await page.addInitScript(() => {
    const connect = AudioNode.prototype.connect; const probes = [];
    window.__audioProbe = { read() { let peak = 0; for (const analyser of probes) { const pcm = new Float32Array(analyser.fftSize); analyser.getFloatTimeDomainData(pcm); for (const x of pcm) peak = Math.max(peak, Math.abs(x)); } return peak; } };
    AudioNode.prototype.connect = function (destination, ...args) { const out = connect.call(this, destination, ...args); if (destination === this.context.destination) { const analyser = this.context.createAnalyser(); analyser.fftSize = 2048; connect.call(this, analyser); probes.push(analyser); } return out; };
  });
  page.on('pageerror', e => errors.push(e.message));
  page.on('response', r => { if (r.status() >= 400 && !r.url().endsWith('favicon.ico')) failed.push({ url: r.url(), status: r.status() }); });
  await page.goto(`http://127.0.0.1:${address.port}/cosmoclash/`);
  await page.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 60000 });
  assert.equal(await page.evaluate(() => window.__cosmo.engine), expectedHash);
  await page.keyboard.press('Enter');
  await waitState('menu');
  // The logo's skip key does not also start a game.
  await page.waitForTimeout(300);
  assert.equal(await state(), 'menu');
  await page.screenshot({ path: path.join(output, 'menu.png') });

  // Keyboard: the menu, a held Enter makes one choice, Esc goes back.
  assert.equal(await active(), 'Play vs Computer');
  await page.keyboard.down('Enter'); await page.keyboard.down('Enter'); await page.keyboard.up('Enter');
  await waitState('difficulty');
  await page.waitForTimeout(200);
  assert.equal(await state(), 'difficulty', 'a held Enter must not also choose a difficulty');
  assert.equal(await active(), 'Medium');
  await page.keyboard.press('Escape');
  await waitState('menu');
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Enter');
  await waitState('selection');
  assert.match(await page.locator('.selection-header').textContent(), /Player 2/);
  await page.keyboard.press('Escape');
  await waitState('menu');
  assert.equal(await active(), 'Two Players · Same Computer', 'the menu keeps the last choice');
  result.keyboardMenus = true;

  // Play vs Computer with the mouse: Easy, then the ship with the keyboard.
  await page.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click();
  await page.getByText('Easy', { exact: true }).click();
  await waitState('selection');
  assert.equal(await page.locator('.selection-header').textContent(), 'Choose Your Ship');
  await page.keyboard.press('Enter');
  await waitState('playing');
  await page.keyboard.down('ArrowUp'); await page.keyboard.down('/'); await page.keyboard.down('.');
  await page.waitForFunction(() => window.__cosmo.snapshot.players[0].morphState !== 0, {}, { timeout: 10000 }); await page.keyboard.up('.');
  await page.waitForFunction(() => window.__cosmo.snapshot.players[0].morphState === 2 && window.__cosmo.snapshot.tick > 70, {}, { timeout: 20000 });
  await page.keyboard.up('ArrowUp'); await page.keyboard.up('/');
  const solo = await read(page);
  assert.equal(solo.snapshot.players[0].shipId, 'michiGichi'); assert.ok(Math.abs(solo.snapshot.players[0].position.y + 70) > 10); assert.ok(solo.snapshot.aiController, 'Rust AI must be active');
  assert.equal(await page.locator('.match-side.left .match-tag').textContent(), 'P1 · You');
  assert.equal(await page.locator('.match-side.right .match-tag').textContent(), 'P2 · Computer');
  await page.waitForFunction(() => window.__audioProbe.read() > 0.0001, {}, { timeout: 10000 });
  result.audioPeak = await page.evaluate(() => window.__audioProbe.read()); assert.ok(result.audioPeak > 0);
  await page.keyboard.press('m'); await page.waitForTimeout(300); assert.equal(await page.evaluate(() => window.__audioProbe.read()), 0); await page.keyboard.press('m');
  await page.waitForFunction(() => window.__audioProbe.read() > 0.0001, {}, { timeout: 5000 }); result.muteRoundTrip = true;
  await page.screenshot({ path: path.join(output, 'solo-morph.png') });

  // Esc pauses the local battle; Esc again goes on; the simulation waits meanwhile.
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => window.__cosmo.paused);
  const pausedAt = (await read(page)).snapshot.tick;
  await page.waitForTimeout(500);
  assert.equal((await read(page)).snapshot.tick, pausedAt, 'the battle waits while paused');
  assert.equal(await active(), 'Resume');
  await page.screenshot({ path: path.join(output, 'solo-paused.png') });
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => !window.__cosmo.paused);
  await page.waitForFunction(t => window.__cosmo.snapshot.tick > t, pausedAt);
  // The Menu button asks the same question; Main menu leaves.
  await page.getByRole('button', { name: 'Menu (Esc)', exact: true }).click();
  await page.getByRole('button', { name: 'Main menu', exact: true }).click();
  await waitState('menu');
  result.pause = true;

  // Two players on one computer.
  await page.getByRole('menuitem', { name: 'Two Players · Same Computer', exact: true }).click();
  await page.keyboard.press('ArrowRight'); await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter');
  assert.match(await page.locator('.selection-header').textContent(), /Player 1/);
  await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter');
  await waitState('playing');
  await page.keyboard.down('ArrowUp'); await page.keyboard.down('/'); await page.keyboard.down('.'); await page.keyboard.down('w'); await page.keyboard.down(' '); await page.keyboard.down('Shift');
  await page.waitForFunction(() => window.__cosmo.snapshot.players[1].frostShieldActive, {}, { timeout: 10000 }); await page.keyboard.up('Shift');
  await page.waitForFunction(() => window.__cosmo.snapshot.players[1].frostShieldActive && window.__cosmo.snapshot.snots.length > 0 && window.__cosmo.snapshot.tick > 40, {}, { timeout: 15000 });
  const local = await page.evaluate(() => window.__cosmo.snapshot);
  assert.deepEqual(local.players.map(p => p.shipId), ['gondi', 'sholev']); assert.equal(local.aiController, null); assert.ok(local.bullets.some(b => b.kind === 'frost')); assert.ok(local.players.every(p => p.position.y > -70));
  await page.screenshot({ path: path.join(output, 'local-shield-slime.png') });
  for (const key of ['ArrowUp', '/', '.', 'w', ' ']) await page.keyboard.up(key);

  // Both players fly with real keys until one ship is destroyed: the game over screen.
  const held = new Set();
  const started = Date.now();
  while ((await state()) !== 'gameOver') {
    const s = await read(page);
    const keys = new Set();
    if (s.state === 'playing' && s.snapshot) {
      for (const key of botKeys(s.snapshot, 0)) keys.add(key);
      for (const key of botKeys(s.snapshot, 1)) keys.add(P2[key]);
    }
    await setKeys(page, keys, held);
    if (Date.now() - started > 240000) throw new Error('The local battle did not end');
    await page.waitForTimeout(100);
  }
  await setKeys(page, new Set(), held);
  assert.match(await page.locator('.cc-dialog h2').textContent(), /^Player [12] wins$/);
  assert.equal(await active(), 'Play again');
  await page.screenshot({ path: path.join(output, 'local-game-over.png') });
  result.localGameOver = { elapsedMs: Date.now() - started };
  await page.keyboard.press('Enter');
  await waitState('selection');
  await page.keyboard.press('Escape');
  await waitState('menu');

  // The unshipped debug instant-kill keys must have no effect in the playable build.
  await page.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click(); await page.getByText('Hard', { exact: true }).click(); await page.keyboard.press('Enter');
  await waitState('playing'); await page.keyboard.press('1'); await page.keyboard.press('2');
  assert.ok(await page.evaluate(() => window.__cosmo.snapshot.players.every(p => p.alive))); result.noDebugKills = true;
  await page.keyboard.press('Escape'); await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter');
  await waitState('menu');
  await page.setViewportSize({ width: 390, height: 844 }); await page.screenshot({ path: path.join(output, 'mobile-menu.png') });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
  assert.deepEqual(errors, []); assert.deepEqual(failed, []);
  Object.assign(result, { passed: true, soloMorph: true, localTwoPlayer: true, slime: true, frostShield: true, allShipsRendered: true, errors, failed });
  console.log(JSON.stringify(result));
} catch (error) {
  result.error = String(error); result.errors = errors; result.failed = failed;
  result.state = await page?.evaluate(() => ({ state: window.__cosmo?.state, snapshot: window.__cosmo?.snapshot })).catch(() => null);
  await page?.screenshot({ path: path.join(output, 'solo-failure.png') }).catch(() => {});
  throw error;
} finally {
  await fs.writeFile(path.join(output, 'solo-results.json'), JSON.stringify(result, null, 2));
  await browser?.close(); await app.close();
}
