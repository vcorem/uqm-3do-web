// Local play in a real browser: Play vs Computer (AI, morph, audio, mute),
// Two Players on one computer as a fleet match (the fleet choice of each
// player with Enter only, shield, slime, a "Player N: Choose Your Next Ship"
// after a loss, the last ship by itself, the game over with both fleets), the
// menus with the keyboard and the mouse, Esc pause, and the removed debug
// keys. Then Play vs Computer to the end: the battle message after a lost
// ship (no overlap with its second line), "Choose Your Next Ship" with two
// ships left, "Leave this game?", the last ship by itself ("Your last ship")
// and the game over screen. The game is served at /cosmoclash/. The local
// screens are saved at 1280x720, 1024x768 and 1920x1080 in
// test-results/local-ui/, the new and changed ones also in test-results/next-ship/.
// The waits for the computer count only the time in which the game runs
// (waitGame): a busy computer runs the game slower.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch, read, setKeys, botKeys, wasmHash, nextShipShots, shipSpace, textOutside, waitGame, settle, screenshot } from './helpers.mjs';

const output = path.resolve(root, process.env.COSMO_BROWSER_RESULTS_DIR || 'test-results/browser');
await fs.mkdir(output, { recursive: true });
const shotsDir = path.resolve(root, process.env.COSMO_LOCAL_UI_DIR || 'test-results/local-ui');
await fs.rm(shotsDir, { recursive: true, force: true });
await fs.mkdir(shotsDir, { recursive: true });
const SIZES = [[1280, 720], [1024, 768], [1920, 1080]];
const shots = [];
let counter = 0;
const next = await nextShipShots('solo');
/** Every visible text of the game is inside the game frame (no clipped or overflowing text). */
async function textInside() {
  assert.deepEqual(await textOutside(page), [], 'text outside the game frame');
}
/**
 * Saves the screen at the three sizes (after the checks) and returns to 960x720.
 * options.next: a new or changed screen of the fleet match, also in test-results/next-ship/.
 */
async function capture(name, check, options = {}) {
  const n = String(++counter).padStart(2, '0');
  for (const [width, height] of SIZES) {
    await page.setViewportSize({ width, height });
    await settle(page);
    await textInside();
    if (check) await check(width, height);
    const file = path.join(shotsDir, `${n}-${name}-${width}x${height}.png`);
    await screenshot(page, file);
    shots.push(path.relative(root, file));
    if (options.next) await next.add(file, name, `${width}x${height}`);
  }
  await page.setViewportSize({ width: 960, height: 720 });
}
/** The battle message and its second line do not overlap (a long ship name wraps to two lines). */
async function messageLayout() {
  const box = await page.evaluate(() => {
    const rect = selector => { const r = document.querySelector(selector).getBoundingClientRect(); return { top: r.top, bottom: r.bottom, left: r.left, right: r.right }; };
    return { message: rect('.match-message'), sub: rect('.match-message-sub'), frame: rect('#game-container'),
      lines: Math.round(document.querySelector('.match-message').getBoundingClientRect().height / parseFloat(getComputedStyle(document.querySelector('.match-message')).lineHeight)) };
  });
  assert.ok(box.message.bottom <= box.sub.top + 0.5, 'the second line starts under the message: ' + JSON.stringify(box));
  assert.ok(box.message.left >= box.frame.left && box.message.right <= box.frame.right, JSON.stringify(box));
  return box;
}
const expectedHash = await wasmHash();
const app = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createRustRelay({ distDir: false }), lobby: 'relay' });
const address = await app.listen(0);
let browser, page;
const result = { passed: false, wasmSha256: expectedHash };
const errors = [], failed = [];
const state = () => page.evaluate(() => window.__cosmo?.state);
const waitState = (value, timeout = 10000) => page.waitForFunction(v => window.__cosmo?.state === v, value, { timeout });
const active = () => page.evaluate(() => document.activeElement?.textContent?.trim() ?? '');
const fleetTitle = () => page.locator('.cc-local-fleet h1').textContent();
const chips = side => page.locator(`.match-side.${side} .match-ship`).allTextContents();
const NAMES = ['MichiGichi', 'Gondi', 'Sholev'];
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
  await capture('difficulty');
  await page.keyboard.press('Escape');
  await waitState('menu');
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Enter');
  await waitState('fleet');
  assert.equal(await fleetTitle(), 'Player 2 · Choose Your Fleet');
  // Two Players: Space is the fire key of Player 2, not a choice.
  await page.keyboard.press(' ');
  await page.waitForTimeout(200);
  assert.deepEqual((await read(page)).pick.picks, [], 'Space does not choose in Two Players');
  assert.equal(await active(), 'MichiGichiPlasma Blast · Transform between fast and weak / slow and strongEnter adds it · it opens the match');
  await capture('two-players-fleet-p2-empty', () => shipSpace(page), { next: true });
  await page.keyboard.press('Escape');
  await waitState('menu');
  assert.equal(await active(), 'Two Players · Same Computer', 'the menu keeps the last choice');
  result.keyboardMenus = true;

  // Play vs Computer with the mouse: Easy, then the ship with the keyboard.
  await page.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click();
  await page.getByText('Easy', { exact: true }).click();
  await waitState('selection');
  assert.equal(await page.locator('.selection-header').textContent(), 'Choose Your Ship');
  await capture('vs-computer-ship-choice');
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

  // Two players on one computer: a fleet match. Player 2 chooses first: Sholev opens, then MichiGichi
  // and Gondi (a full fleet puts the focus on Done). Space (the fire key of Player 2) never chooses.
  await page.getByRole('menuitem', { name: 'Two Players · Same Computer', exact: true }).click();
  await waitState('fleet');
  await page.keyboard.press('ArrowRight'); await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter');
  await page.keyboard.press('Enter'); await page.keyboard.press('Enter');
  assert.deepEqual((await read(page)).pick, { player: 1, picks: [2, 0, 1], carousel: 1, focus: 'done', notice: '' });
  assert.equal(await active(), 'Done · 3 ships');
  await page.keyboard.press(' ');
  await page.waitForTimeout(200);
  assert.equal((await read(page)).pick.player, 1, 'Space does not say Done');
  await capture('two-players-fleet-p2', () => shipSpace(page), { next: true });
  await page.keyboard.press('Enter');
  await page.waitForFunction(() => window.__cosmo.pick?.player === 0);
  assert.equal(await fleetTitle(), 'Player 1 · Choose Your Fleet');
  // Player 1: Gondi opens, then Sholev and MichiGichi.
  await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter'); await page.keyboard.press('Enter');
  assert.deepEqual((await read(page)).pick.picks, [1, 2]);
  await capture('two-players-fleet-p1', () => shipSpace(page), { next: true });
  await page.keyboard.press('Enter');
  assert.deepEqual((await read(page)).pick.picks, [1, 2, 0]);
  await page.keyboard.press('Enter');
  await waitState('playing');
  // Both fleets as battle chips, the opening ships in battle.
  assert.deepEqual([await chips('left'), await chips('right')], [['Gondi', 'Sholev', 'MichiGichi'], ['Sholev', 'MichiGichi', 'Gondi']]);
  assert.equal(await page.locator('.match-side.left .match-ship.current').textContent(), 'Gondi');
  await capture('two-players-battle-fleets', null, { next: true });
  await page.keyboard.down('ArrowUp'); await page.keyboard.down('/'); await page.keyboard.down('.'); await page.keyboard.down('w'); await page.keyboard.down(' '); await page.keyboard.down('Shift');
  await page.waitForFunction(() => window.__cosmo.snapshot.players[1].frostShieldActive, {}, { timeout: 10000 }); await page.keyboard.up('Shift');
  await page.waitForFunction(() => window.__cosmo.snapshot.players[1].frostShieldActive && window.__cosmo.snapshot.snots.length > 0 && window.__cosmo.snapshot.tick > 40, {}, { timeout: 15000 });
  const local = await page.evaluate(() => window.__cosmo.snapshot);
  assert.deepEqual(local.players.map(p => p.shipId), ['gondi', 'sholev']); assert.equal(local.aiController, null); assert.ok(local.bullets.some(b => b.kind === 'frost')); assert.ok(local.players.every(p => p.position.y > -70));
  await page.screenshot({ path: path.join(output, 'local-shield-slime.png') });
  for (const key of ['ArrowUp', '/', '.', 'w', ' ']) await page.keyboard.up(key);
  // Two Players paused: Space (fire of Player 2) does not resume; Esc does.
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => window.__cosmo.paused);
  await page.keyboard.press(' ');
  await page.waitForTimeout(200);
  assert.equal(await page.evaluate(() => window.__cosmo.paused), true, 'Space does not resume in Two Players');
  await capture('two-players-paused', null, { next: true });
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => !window.__cosmo.paused);

  // Both players fly with real keys until a fleet has no ships. After a loss with two ships left the
  // loser chooses the next ship ("Player N: Choose Your Next Ship", Enter only); the last ship comes
  // by itself. After the first choice only Player 1 flies, so that the match ends sooner.
  const held = new Set();
  const started = Date.now();
  const localMatch = { choices: [], lastShip: null };
  let frames = -1, movedAt = Date.now();
  for (;;) {
    const s = await read(page);
    if (s.state === 'gameOver') break;
    if (s.frames !== frames) { frames = s.frames; movedAt = Date.now(); }
    if (s.state === 'selection') {
      await setKeys(page, new Set(), held);
      const chooser = s.fleet.nextChooser;
      assert.ok(chooser === 0 || chooser === 1, JSON.stringify(s.fleet));
      assert.equal(await page.locator('.selection-header').textContent(), `Player ${chooser + 1}: Choose Your Next Ship`);
      assert.equal(s.fleet.sides[chooser].ships.length - s.fleet.sides[chooser].lost, 2, 'a choice only with two ships left');
      if (!localMatch.choices.length) {
        // Space (the fire key of Player 2) never chooses; Esc asks first; Space does not answer either.
        await page.keyboard.press(' ');
        await page.waitForTimeout(300);
        assert.equal(await state(), 'selection', 'Space does not choose the next ship in Two Players');
        await capture('two-players-next-ship', null, { next: true });
        await page.keyboard.press('Escape');
        await page.waitForFunction(() => document.querySelector('.cc-dialog h2')?.textContent === 'Leave this game?');
        assert.equal(await active(), 'Keep playing');
        await page.keyboard.press(' ');
        await page.waitForTimeout(200);
        assert.equal(await page.locator('.cc-dialog h2').count(), 1, 'Space does not answer in Two Players');
        await capture('two-players-leave-question', null, { next: true });
        await page.keyboard.press('Escape');
        await page.waitForFunction(() => !document.querySelector('.cc-dialog'));
        // The other ship of the two.
        await page.keyboard.press('ArrowRight');
      }
      const ship = NAMES.indexOf(await page.locator('.selection-ship-name').textContent());
      await page.keyboard.press('Enter');
      await waitState('playing');
      assert.equal((await read(page)).fleet.current[chooser], ship, 'the chosen ship flies next');
      localMatch.choices.push({ player: chooser, ship });
      continue;
    }
    if (s.state === 'roundOver' && !localMatch.lastShip) {
      // The battle text goes when the next screen comes: read it without waiting.
      const [text, sub] = await page.evaluate(() => ['.match-message', '.match-message-sub'].map(selector => document.querySelector(selector)?.textContent ?? ''));
      if (/^Player [12]'s last ship: /.test(sub)) {
        await setKeys(page, new Set(), held);
        localMatch.lastShip = { text, sub };
        await capture('two-players-last-ship', async () => { await messageLayout(); }, { next: true });
        // No choice: the next battle starts by itself.
        const after = await waitGame(page, () => window.__cosmo.state !== 'roundOver' && window.__cosmo.state, { label: 'the battle after the last ship message' });
        assert.ok(['playing', 'gameOver'].includes(after) && after !== 'selection', 'the last ship comes by itself: ' + after);
        continue;
      }
    }
    const keys = new Set();
    if (s.state === 'playing' && s.snapshot) {
      for (const key of botKeys(s.snapshot, 0)) keys.add(key);
      if (!localMatch.choices.length) for (const key of botKeys(s.snapshot, 1)) keys.add(P2[key]);
    }
    await setKeys(page, keys, held);
    if (Date.now() - movedAt > 90000) throw new Error('The game stopped during the Two Players match');
    if (Date.now() - started > 1200000) throw new Error('The Two Players fleet match did not end');
    await page.waitForTimeout(100);
  }
  await setKeys(page, new Set(), held);
  assert.ok(localMatch.choices.length >= 1, 'a loss with two ships left has a choice');
  assert.ok(localMatch.lastShip, 'the last ship came by itself');
  assert.match(await page.locator('.cc-dialog h2').textContent(), /^Player [12] wins$/);
  // The game over shows both fleets; the loser's three ships are destroyed.
  const winnerNumber = Number((await page.locator('.cc-dialog h2').textContent()).match(/\d/)[0]);
  assert.equal(await page.locator('.cc-dialog p', { hasText: `Player ${3 - winnerNumber} has no ships left.` }).count(), 1);
  assert.equal(await page.locator('.cc-dialog .cc-fleet-line').count(), 2);
  assert.equal(await page.locator('.cc-dialog .cc-fleet-line').nth(2 - winnerNumber).locator('.cc-chip.lost').count(), 3);
  assert.equal(await active(), 'Play again');
  // The fire key of Player 2 does not choose on the game over screen.
  await page.keyboard.press(' ');
  await page.waitForTimeout(700);
  assert.equal(await state(), 'gameOver', 'Space does not choose on the game over screen in Two Players');
  await page.screenshot({ path: path.join(output, 'local-game-over.png') });
  await capture('two-players-game-over', null, { next: true });
  result.localGameOver = { elapsedMs: Date.now() - started, ...localMatch };
  await page.keyboard.press('Enter');
  await waitState('fleet');
  assert.equal(await fleetTitle(), 'Player 2 · Choose Your Fleet');
  await page.keyboard.press('Escape');
  await waitState('menu');

  // The unshipped debug instant-kill keys must have no effect in the playable build.
  await page.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click(); await page.getByText('Hard', { exact: true }).click(); await page.keyboard.press('Enter');
  await waitState('playing'); await page.keyboard.press('1'); await page.keyboard.press('2');
  assert.ok(await page.evaluate(() => window.__cosmo.snapshot.players.every(p => p.alive))); result.noDebugKills = true;
  await page.keyboard.press('Escape'); await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter');
  await waitState('menu');

  // Play vs Computer to the end. The player does not fly: the Hard computer destroys the ships.
  // With two ships left the player chooses the next one; the last ship comes by itself.
  assert.equal(await active(), 'Play vs Computer');
  await page.keyboard.press('Enter');
  await waitState('difficulty');
  await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter');
  await waitState('selection');
  await page.keyboard.press('Enter');
  await waitState('playing');
  const gauntlet = Date.now();
  const vs = { handled: [], nextShip: false, lastShip: null, games: 1 };
  /** The next event of the game: the message after a battle, a ship choice or the game over. */
  const vsEvent = () => waitGame(page, handled => {
    const c = window.__cosmo;
    if (c.state === 'selection' || c.state === 'gameOver') return { kind: c.state, battle: c.fleet.battle };
    const centre = document.querySelector('.match-centre');
    if (c.state === 'roundOver' && centre && !centre.hidden && !handled.includes(c.fleet.battle)) {
      return { kind: 'message', battle: c.fleet.battle, text: document.querySelector('.match-message').textContent, sub: document.querySelector('.match-message-sub').textContent };
    }
    return null;
  }, { arg: vs.handled, label: 'the next event of the game against the computer' });
  for (;;) {
    const event = await vsEvent();
    if (event.kind === 'message') {
      vs.handled.push(event.battle);
      if (!result.battleMessage) {
        result.battleMessage = { text: event.text, sub: event.sub, boxes: [] };
        await capture('vs-computer-battle-message', async () => { result.battleMessage.boxes.push(await messageLayout()); }, { next: true });
      }
      if (/^Your last ship: /.test(event.sub)) {
        assert.match(event.text, /^Your \w+ was destroyed$/);
        vs.lastShip = { text: event.text, sub: event.sub };
        await capture('vs-computer-last-ship', async () => { await messageLayout(); }, { next: true });
        // No ship choice: the next battle starts by itself with the last ship.
        const after = await waitGame(page, b => (window.__cosmo.state !== 'roundOver' || window.__cosmo.fleet.battle !== b) && window.__cosmo.state, { arg: event.battle, label: 'the battle with the last ship' });
        assert.equal(after, 'playing', 'the last ship comes by itself');
        const ship = NAMES.indexOf(event.sub.replace('Your last ship: ', ''));
        assert.equal((await read(page)).snapshot.players[0].shipId, ['michiGichi', 'gondi', 'sholev'][ship]);
      }
      continue;
    }
    if (event.kind === 'selection') {
      assert.equal(await page.locator('.selection-header').textContent(), 'Choose Your Next Ship');
      assert.match(await page.locator('.selection-sub').textContent(), /^Your \w+ was destroyed\. The computer has [123] ships? left\.$/);
      assert.equal(await page.locator('.selection-remaining').textContent(), 'You have 2 ships left');
      assert.equal(vs.lastShip, null, 'no choice with one ship left');
      if (!vs.nextShip) {
        vs.nextShip = true;
        await capture('vs-computer-next-ship', null, { next: true });
        // Esc asks first; Keep playing (Esc) stays in this game.
        await page.keyboard.press('Escape');
        await page.waitForFunction(() => document.querySelector('.cc-dialog h2')?.textContent === 'Leave this game?');
        assert.equal(await active(), 'Keep playing');
        await capture('vs-computer-leave-question');
        await page.keyboard.press('Escape');
        await page.waitForFunction(() => !document.querySelector('.cc-dialog'));
        assert.equal(await state(), 'selection');
      }
      await page.keyboard.press('Enter');
      await waitState('playing');
      continue;
    }
    const over = await page.locator('.cc-dialog h2').textContent();
    assert.ok(['Defeat', 'Victory'].includes(over), over);
    if (vs.lastShip) {
      assert.equal(await page.locator('.cc-dialog .cc-fleet-line').count(), 2, 'the game over shows both fleets');
      result.vsComputerEnd = { title: over, elapsedMs: Date.now() - gauntlet, games: vs.games, lastShip: vs.lastShip };
      await capture('vs-computer-game-over', null, { next: true });
      break;
    }
    // The computer lost its ships before the player came to the last ship (the sun): play again.
    if (++vs.games > 3) throw new Error('Play vs Computer never came to the last ship');
    vs.handled.length = 0;
    await page.waitForTimeout(700);
    await page.keyboard.press('Enter');
    await waitState('selection');
    await page.keyboard.press('Enter');
    await waitState('playing');
  }
  await page.keyboard.press('Escape');
  await waitState('menu');
  await page.setViewportSize({ width: 390, height: 844 }); await page.screenshot({ path: path.join(output, 'mobile-menu.png') });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
  assert.deepEqual(errors, []); assert.deepEqual(failed, []);
  Object.assign(result, { passed: true, soloMorph: true, localTwoPlayer: true, slime: true, frostShield: true, allShipsRendered: true, errors, failed, screenshots: shots, nextShipScreenshots: next.files });
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
