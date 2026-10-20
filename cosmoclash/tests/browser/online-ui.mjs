// Online Melee inside the game, with two real browsers through a real relay
// and the cosmoclash room list (the relay's own, or scripts/cosmo-lobby.mjs).
// The host creates a named room with the keyboard; the guest sees it in the
// list and joins with the keyboard; both choose fleets in the 3D ship choice;
// Change fleet (host and guest); a real fleet match with verified checksums
// (protocol 3), in which one player flies and the other does not, so that the
// guest loses first and chooses the next ship with the keyboard while the
// host waits, then the host loses and its time runs out (the first ship left
// comes; the last seconds at three sizes), then the guest's last ship comes by
// itself (the battle message on both pages, no choice); rematch; the opponent
// leaves -> lobby; Open room
// before the room list answers; full screen again after the browser left it,
// and F. Then a relay without room lists (the Rust relay): a private room, its
// invite link and the Join screen, with the mouse. Every screen is saved at
// 1280x720, 1024x768 and 1920x1080 in test-results/online-ui/; the new and
// changed screens of the next-ship choice also in test-results/next-ship/.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createNetplayServer } from '../../../server/netplay.mjs';
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch, poll, read, screen, setKeys, botKeys, watchPage, recordKeyboardLock, wasmHash, nextShipShots, shipSpace, textOutside, settle, screenshot } from './helpers.mjs';

const output = path.resolve(root, process.env.COSMO_ONLINE_UI_DIR || 'test-results/online-ui');
await fs.rm(output, { recursive: true, force: true });
await fs.mkdir(output, { recursive: true });
const SIZES = [[1280, 720], [1024, 768], [1920, 1080]];
const shots = [];
let counter = 0;
const next = await nextShipShots('online');
/**
 * Saves the screen at the three sizes (or the first only) and returns to 1280x720.
 * options.next: a new or changed screen of the next-ship choice, also in
 * test-results/next-ship/, with no text outside the game frame (and options.check).
 */
async function capture(page, name, who, sizes = SIZES, options = {}) {
  const n = String(++counter).padStart(2, '0');
  for (const [width, height] of sizes) {
    if (width !== 1280 || height !== 720) { await page.setViewportSize({ width, height }); await settle(page, 120); }
    else await settle(page, 80);
    if (options.next) assert.deepEqual(await textOutside(page), [], `text outside the game frame (${name} ${width}x${height})`);
    if (options.check) await options.check(width, height);
    const file = path.join(output, `${n}-${name}-${who}-${width}x${height}.png`);
    await screenshot(page, file);
    shots.push(path.relative(root, file));
    if (options.next) await next.add(file, `${name}-${who}`, `${width}x${height}`);
  }
  await page.setViewportSize({ width: 1280, height: 720 });
}
const SHIP_IDS = ['michiGichi', 'gondi', 'sholev'];
const NAMES = ['MichiGichi', 'Gondi', 'Sholev'];
/**
 * The last seconds of a choice: the screenshots at three sizes take longer than
 * that, so the page clock stops meanwhile (Date.now and the CSS animations, such
 * as the bar of the time) and then goes on. The game and its 3D scene still draw.
 */
const stopClock = page => page.evaluate(() => {
  const now = Date.now(), real = Date.now;
  const paused = document.getAnimations().filter(animation => animation.playState === 'running');
  paused.forEach(animation => animation.pause());
  Date.now = () => now;
  window.__startClock = () => { Date.now = real; paused.forEach(animation => animation.play()); delete window.__startClock; };
});
const startClock = page => page.evaluate(() => window.__startClock?.());
/** The time of the choice is red, with 1 to 4 seconds left. */
async function lowTimer(page) {
  assert.equal(await page.locator('.cc-timer.low').count(), 1, 'the timer is red');
  assert.match(await page.locator('.cc-timer-n').textContent(), /^[1-4]$/);
}
/**
 * Protocol 3: both pages show the choice of the next ship (pages[ci] chooses).
 * how: 'keyboard' (screens of both sides, Esc asks, the other ship with the
 * keys), 'timeout' (no key: the time runs out, the first ship left comes) or
 * 'quick' (Enter). Returns the choice after both pages are in the next battle.
 */
async function nextShip(pages, ci, how) {
  const chooser = pages[ci], waiter = pages[1 - ci];
  const who = ci ? 'guest' : 'host', other = ci ? 'host' : 'guest';
  const states = await Promise.all(pages.map(read));
  const choice = states[ci].online.next;
  assert.equal(choice.mine, true);
  assert.equal(states[1 - ci].online.next.mine, false);
  assert.deepEqual(states[1 - ci].online.next.ships, choice.ships, 'both pages know the ships left');
  assert.equal(choice.ships.length, 2);
  // Both peers stopped at the same tick: the same state and the same checksum.
  assert.equal(states[0].net.tick, states[1].net.tick);
  assert.equal(states[0].hash, states[1].hash, 'both peers have the same state during the choice');
  assert.deepEqual(states.map(s => s.net.phase), ['choosing', 'choosing']);
  let ship;
  if (how === 'keyboard') {
    await capture(chooser, 'next-ship-choose', who, SIZES, { next: true, check: () => shipSpace(chooser) });
    assert.match(await chooser.locator('#cosmo-online h1').textContent(), /^Choose Your Next Ship$/);
    assert.match(await chooser.locator('.cc-timer-n').textContent(), /^(1\d|20)$/);
    await capture(waiter, 'next-ship-wait', other, SIZES, { next: true });
    assert.match(await waiter.locator('#cosmo-online').textContent(), /Your opponent is choosing the next ship/);
    // The waiting player never chooses: Enter does nothing there.
    await waiter.keyboard.press('Enter');
    await waiter.waitForTimeout(200);
    assert.deepEqual([(await read(waiter)).online.confirm, (await read(waiter)).online.screen], [false, 'next']);
    // Esc asks first; Stay (the default) keeps the match, the time goes on.
    await chooser.waitForTimeout(700);
    await chooser.keyboard.press('Escape');
    await chooser.waitForFunction(() => window.__cosmo.online.confirm);
    assert.equal(await activeText(chooser), 'Stay');
    await capture(chooser, 'next-ship-leave-question', who, SIZES, { next: true });
    await chooser.keyboard.press('Enter');
    assert.equal((await read(chooser)).online.confirm, false);
    // The other ship (not the first ship left), with the keyboard.
    await chooser.keyboard.press('ArrowRight');
    ship = (await read(chooser)).online.next.carousel;
    assert.notEqual(ship, choice.ships[0]);
    assert.equal(await chooser.locator('.cc-ship-name').textContent(), ['MichiGichi', 'Gondi', 'Sholev'][ship]);
    await chooser.keyboard.press('Enter');
  } else if (how === 'timeout') {
    ship = choice.ships[0];
    // The last seconds: the timer is red on both sides (both clocks stop for the screenshots).
    await Promise.all(pages.map(p => p.waitForFunction(() => window.__cosmo.online.next?.left <= 4, {}, { timeout: 30000 })));
    await Promise.all(pages.map(stopClock));
    try {
      await Promise.all([
        capture(chooser, 'next-ship-last-seconds', who, SIZES, { next: true, check: () => lowTimer(chooser) }),
        capture(waiter, 'next-ship-wait-last-seconds', other, SIZES, { next: true, check: () => lowTimer(waiter) })]);
      assert.equal((await read(chooser)).online.next.chosen, null, 'the screenshots show the last seconds');
    } finally { await Promise.all(pages.map(startClock)); }
    await chooser.waitForFunction(() => window.__cosmo.online.screen !== 'next' || window.__cosmo.online.next?.chosen !== null, {}, { timeout: 15000 });
  } else {
    await chooser.waitForTimeout(700);
    await chooser.keyboard.press('Enter');
    ship = choice.carousel;
  }
  await Promise.all(pages.map(p => screen(p, 'countdown', 15000)));
  const after = await Promise.all(pages.map(read));
  for (const s of after) {
    assert.equal(s.fleet.current[choice.player], ship, 'both peers fly the chosen ship next');
    assert.equal(s.snapshot.players[choice.player].shipId, SHIP_IDS[ship]);
    assert.equal(s.fleet.battle, choice.battle);
  }
  if (how === 'keyboard') await capture(chooser, 'next-ship-countdown', who, SIZES, { next: true });
  await Promise.all(pages.map(p => screen(p, 'battle', 15000)));
  if (how === 'keyboard') { await pages[0].waitForTimeout(600); await capture(pages[0], 'next-ship-battle', 'host', SIZES, { next: true }); }
  return { battle: choice.battle, player: choice.player, ship, how, ships: choice.ships };
}

const expectedHash = await wasmHash();
const sink = { errors: [], requests: [], sockets: [] };
const result = { passed: false, wasmSha256: expectedHash, checks: {} };
const browsers = [];
const servers = [];
async function newPage(browser, index, options = {}) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 }, ...options });
  const page = await context.newPage();
  await page.addInitScript(recordKeyboardLock);
  watchPage(page, index, sink);
  return page;
}
async function boot(page, url) {
  await page.goto(url);
  await page.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 90000 });
  assert.equal((await read(page)).engine, expectedHash);
}
const activeText = page => page.evaluate(() => document.activeElement?.textContent?.trim() ?? '');
const focusClass = page => page.evaluate(() => document.querySelector('#cosmo-online .is-focus')?.textContent?.trim() ?? '');

try {
  // ---------------------------------------------------------------- listed rooms (Node relay)
  const app = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createNetplayServer({ distDir: false }) });
  servers.push(app);
  const { port } = await app.listen(0);
  const url = `http://127.0.0.1:${port}/cosmoclash/`;
  result.lobby = app.lobby;
  browsers.push(await launch(), await launch());
  const [A, B] = await Promise.all([newPage(browsers[0], 0), newPage(browsers[1], 1)]);
  // The guest's room list and room sockets wait for these gates, to show "connecting" and "joining".
  let openLobby, openRoom;
  let lobbyGate = new Promise(resolve => { openLobby = resolve; });
  const roomGate = new Promise(resolve => { openRoom = resolve; });
  await B.routeWebSocket(/\/netplay\/lobby/, async ws => { await lobbyGate; ws.connectToServer(); });
  // The guest's room sockets go through the test: first behind the gate; later whileHeld() keeps the packets
  // of both directions (as a slow network does), so that both lockstep peers stop within a few ticks
  // and a short screen (the message after a battle shows 1.5 s) stays for the screenshots.
  const relay = { holding: false, queue: [] };
  const forward = send => { if (relay.holding) relay.queue.push(send); else send(); };
  await B.routeWebSocket(/\/netplay\/cosmoclash\//, async ws => {
    await roomGate;
    const server = ws.connectToServer();
    ws.onMessage(message => forward(() => server.send(message)));
    server.onMessage(message => forward(() => ws.send(message)));
  });
  /** Runs fn while the guest's room packets wait (both peers stopped), then sends them in order. */
  async function whileHeld(fn) {
    relay.holding = true;
    try {
      await poll(async () => {
        const before = await Promise.all([A, B].map(read));
        await new Promise(r => setTimeout(r, 300));
        const after = await Promise.all([A, B].map(read));
        return after.every((s, i) => s.net.tick === before[i].net.tick);
      }, 10000, 'both peers stop');
      return await fn();
    } finally {
      relay.holding = false;
      for (const send of relay.queue.splice(0)) { try { send(); } catch { /* The socket closed. */ } }
    }
  }
  /**
   * The message after a battle on both pages (stopped by whileHeld()): what comes next
   * (a choice or the last ship) on the loser's page and on the winner's, at three sizes.
   */
  async function battleMessage(kind, battle) {
    return whileHeld(async () => {
      const states = await Promise.all([A, B].map(read));
      assert.deepEqual(states.map(s => s.state), ['roundOver', 'roundOver'], 'both pages show the message after the battle');
      const texts = await Promise.all([A, B].map(p => p.evaluate(() => ['.match-message', '.match-message-sub'].map(selector => document.querySelector(selector)?.textContent ?? ''))));
      const loser = texts[0][0].startsWith('Your ') ? 0 : 1;
      const lost = NAMES[states[0].fleet.sides[loser].ships[states[0].fleet.sides[loser].lost - 1]];
      const last = NAMES[states[0].fleet.current[loser]];
      assert.deepEqual(texts[loser], [`Your ${lost} was destroyed`, kind === 'choice' ? 'Choose your next ship.' : `Your last ship: ${last}`]);
      assert.deepEqual(texts[1 - loser], [`Enemy ${lost} destroyed!`, kind === 'choice' ? 'Your opponent chooses the next ship.' : `Their last ship: ${last}`]);
      const name = kind === 'choice' ? 'battle-over-choice' : 'battle-over-last-ship';
      await Promise.all([capture(A, name, loser ? 'host-won' : 'host-lost', SIZES, { next: true }), capture(B, name, loser ? 'guest-lost' : 'guest-won', SIZES, { next: true })]);
      return { kind, battle, loser, lost, last: kind === 'last' ? last : null, texts };
    });
  }
  await Promise.all([boot(A, url), boot(B, url)]);
  await A.waitForTimeout(1200);
  await capture(A, 'logo', 'host', [[1280, 720]]);
  await A.keyboard.press(' '); await B.keyboard.press(' ');
  await Promise.all([A, B].map(p => p.waitForFunction(() => window.__cosmo?.state === 'menu')));
  assert.equal(await activeText(A), 'Play vs Computer');
  await capture(A, 'menu', 'host');

  // Main menu: Enter chooses the focused item; a held Enter makes one choice.
  await A.keyboard.press('ArrowDown'); await A.keyboard.press('ArrowDown');
  assert.equal(await activeText(A), 'Online Melee');
  await A.keyboard.down('Enter'); await A.keyboard.down('Enter'); await A.keyboard.down('Enter'); await A.keyboard.up('Enter');
  await screen(A, 'lobby');
  await A.waitForTimeout(300);
  assert.equal((await read(A)).online.screen, 'lobby', 'a held Enter must not also choose Create room');
  // The first menu choice asks for full screen on the game frame and keeps Esc as a game key.
  const full = await read(A);
  assert.equal(full.fullscreen, 'cosmo-frame');
  result.checks.fullscreen = full.fullscreen;
  result.checks.keyboardLock = await A.evaluate(() => window.__keyboardLock);
  assert.deepEqual(result.checks.keyboardLock?.[0], ['Escape']);
  await A.waitForFunction(() => window.__cosmo.online.lobby === 'open');
  assert.match((await read(A)).live, /^Online Melee lobby\. No open rooms\./);
  assert.equal(await focusClass(A), 'Create room');
  await capture(A, 'lobby-empty', 'host');
  // The browser leaves full screen (Esc in Firefox and Safari): the next choice asks again.
  await A.evaluate(() => document.exitFullscreen());
  await A.waitForFunction(() => window.__cosmo.fullscreen === null);

  // Name dialog: a default name (selected: typing replaces it), a refused name with the reason, then the room.
  await A.keyboard.press('Enter');
  await screen(A, 'name');
  await A.waitForFunction(() => window.__cosmo.fullscreen === 'cosmo-frame');
  result.checks.fullscreenAgain = true;
  const defaultName = await A.locator('#cosmo-online input').inputValue();
  assert.match(defaultName, /^Pilot \d\d$/);
  await capture(A, 'name', 'host');
  await A.keyboard.type('x'.repeat(25));
  assert.equal(await A.locator('#cosmo-online input').inputValue(), 'x'.repeat(25), 'typing replaces the default name');
  await A.keyboard.press('Enter');
  await A.waitForFunction(() => document.querySelector('#cosmo-online .cc-error')?.textContent === 'Use at most 24 characters.');
  assert.equal((await read(A)).online.screen, 'name');
  await capture(A, 'name-refused', 'host');
  const ROOM = 'Friday duel ⚔ קרב';
  await A.keyboard.press('ControlOrMeta+A'); await A.keyboard.type('  Friday   duel ⚔ קרב '); await A.keyboard.press('Enter');
  await screen(A, 'waiting');
  assert.equal((await read(A)).online.room, ROOM);
  assert.equal(await activeText(A), 'Copy invite link', 'Enter never closes a room just opened');
  await capture(A, 'waiting', 'host');

  // Older rooms in the list, to show the waiting times (the adapter can list rooms without a host).
  const fakes = [];
  if (app.adapter) {
    app.adapter.addFakeRoom({ id: 'f1'.repeat(16), name: 'Old timers', created: Date.now() - 12 * 60000 });
    app.adapter.addFakeRoom({ id: 'f2'.repeat(16), name: 'Squadron 7', created: Date.now() - 3 * 60000 });
    fakes.push('f1'.repeat(16), 'f2'.repeat(16));
  }
  // The guest opens the lobby with the keyboard; the list waits behind the gate first.
  await B.keyboard.press('ArrowDown'); await B.keyboard.press('ArrowDown'); await B.keyboard.press('Enter');
  await screen(B, 'lobby');
  assert.equal((await read(B)).online.lobby, 'connecting');
  await capture(B, 'lobby-connecting', 'guest');
  openLobby();
  await B.waitForFunction(n => window.__cosmo.online.rooms.length === n, 1 + fakes.length);
  const lobbyB = (await read(B)).online;
  assert.equal(lobbyB.rooms[0], ROOM, 'the newest room is first');
  assert.match(lobbyB.focus, /^room:/);
  assert.equal(await focusClass(B), `${ROOM}newJoin ›`);
  await capture(B, 'lobby-rooms', 'guest');

  // The guest joins with Enter; the room socket waits behind the gate first.
  await B.keyboard.press('Enter');
  await screen(B, 'joining');
  await capture(B, 'joining', 'guest');
  openRoom();
  await Promise.all([screen(A, 'fleet'), screen(B, 'fleet')]);
  assert.match((await read(B)).live, /^Opponent found! Choose your fleet\./);
  await capture(A, 'fleet-opponent-found', 'host', [[1280, 720]]);
  await capture(B, 'fleet-opponent-found', 'guest', [[1280, 720]]);
  for (const fake of fakes) app.adapter.removeRoom(fake);

  // Fleets: the host picks Gondi and Sholev and says Ready with two ships.
  await A.waitForTimeout(2600);
  await A.keyboard.press('ArrowRight'); await A.keyboard.press('Enter'); await A.keyboard.press('Enter');
  assert.deepEqual((await read(A)).online.picks, [1, 2]);
  assert.equal(await A.locator('.cc-rule').textContent(), 'Ship 1 opens the match. After each loss you choose your next ship from the ships left.');
  assert.deepEqual(await A.locator('.cc-slot').allTextContents(), ['1Gondi', '✓Sholev', '+Add']);
  await capture(A, 'fleet-two-picks', 'host', SIZES, { next: true, check: () => shipSpace(A) });
  await A.keyboard.press('ArrowDown');
  assert.equal(await activeText(A), 'Ready · 2 ships');
  await A.keyboard.press('Enter');
  await A.waitForFunction(() => window.__cosmo.netplay?.localReady);
  await B.waitForFunction(() => document.querySelector('.cc-opponent')?.textContent === 'Opponent: ready with 2 ships');
  await capture(A, 'fleet-ready-waiting', 'host');
  // Change fleet (host): Ready is taken back at once, and the guest sees it.
  await A.keyboard.press('ArrowDown');
  assert.equal(await activeText(A), 'Change fleet');
  await A.keyboard.press('Enter');
  await A.waitForFunction(() => window.__cosmo.netplay && !window.__cosmo.netplay.localReady);
  await B.waitForFunction(() => document.querySelector('.cc-opponent')?.textContent === 'Opponent: choosing · 2 of 3 picked');
  // Esc on the fleet screen asks first; Stay (the default) keeps the room.
  await B.keyboard.press('Escape');
  await B.waitForFunction(() => window.__cosmo.online.confirm);
  assert.equal(await activeText(B), 'Stay');
  await capture(B, 'fleet-leave-question', 'guest');
  await B.keyboard.press('Enter');
  assert.equal((await read(B)).online.confirm, false);
  // The guest fills the fleet and says Ready, takes Ready back (the host answers),
  // takes two ships back out and finishes with one.
  await B.keyboard.press('Enter'); await B.keyboard.press('Enter'); await B.keyboard.press('Enter');
  assert.deepEqual((await read(B)).online.picks, [0, 1, 2]);
  assert.equal(await activeText(B), 'Ready · 3 ships');
  await capture(B, 'fleet-full', 'guest');
  await B.keyboard.press('Enter');
  await B.waitForFunction(() => window.__cosmo.netplay?.localReady);
  await A.waitForFunction(() => document.querySelector('.cc-opponent')?.textContent === 'Opponent: ready with 3 ships');
  await B.keyboard.press('ArrowDown');
  assert.equal(await activeText(B), 'Change fleet');
  await capture(B, 'fleet-ready-change', 'guest');
  await B.keyboard.press('Enter');
  await B.waitForFunction(() => window.__cosmo.netplay && !window.__cosmo.netplay.localReady && !window.__cosmo.netplay.changing);
  await A.waitForFunction(() => document.querySelector('.cc-opponent')?.textContent === 'Opponent: choosing · 3 of 3 picked');
  result.checks.changeFleet = true;
  await B.keyboard.press('Backspace'); await B.keyboard.press('Backspace');
  assert.deepEqual((await read(B)).online.picks, [0]);
  await capture(B, 'fleet-one-pick', 'guest', SIZES, { next: true });
  // The host adds MichiGichi (the focus is on the ship after Change fleet; a full fleet puts it on Ready) and says Ready.
  await A.keyboard.press('Enter');
  assert.deepEqual((await read(A)).online.picks, [1, 2, 0]);
  assert.equal(await activeText(A), 'Ready · 3 ships');
  await A.keyboard.press('Enter');
  await B.waitForFunction(() => document.querySelector('.cc-opponent')?.textContent === 'Opponent: ready with 3 ships');
  // The guest takes its two ships back (three ships each: the first loss always has a choice).
  await B.keyboard.press('Enter'); await B.keyboard.press('Enter');
  assert.deepEqual((await read(B)).online.picks, [0, 1, 2]);
  assert.equal(await activeText(B), 'Ready · 3 ships');
  await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'countdown'), screen(B, 'countdown')]);
  await capture(A, 'countdown', 'host', SIZES, { next: true });
  await capture(B, 'countdown', 'guest', [[1280, 720]]);
  await Promise.all([screen(A, 'battle'), screen(B, 'battle')]);
  const starts = await Promise.all([A, B].map(read));
  assert.deepEqual(starts.map(s => s.net.localFleet), [[1, 2, 0], [0, 1, 2]]);
  await A.waitForTimeout(800);
  await capture(A, 'battle', 'host', SIZES, { next: true });
  await capture(B, 'battle', 'guest', [[1280, 720]]);
  // Esc asks first; Stay (the default) keeps the match.
  await B.keyboard.press('Escape');
  await B.waitForFunction(() => window.__cosmo.online.confirm);
  assert.equal(await activeText(B), 'Stay');
  await capture(B, 'battle-leave-question', 'guest');
  await B.keyboard.press('Enter');
  assert.equal((await read(B)).online.confirm, false);

  // A real fleet match with real keys, until it ends. One player flies and the other does not, so the
  // losers come in this order: the guest (two ships left: it chooses with the keyboard), the host (its
  // time runs out), the guest (its last ship comes by itself, no choice) and the guest (the end).
  const LOSERS = [1, 0, 1, 1];
  const idle = battle => LOSERS[battle] ?? 1;
  const held = [new Set(), new Set()];
  const started = Date.now();
  let maxChecks = 0, maxShots = 0;
  const choices = [], messages = [];
  const how = ci => ci === 1 && !choices.some(c => c.how === 'keyboard') ? 'keyboard' : !choices.some(c => c.how === 'timeout') ? 'timeout' : 'quick';
  for (;;) {
    const states = await Promise.all([A, B].map(read));
    for (const s of states) {
      assert.ok(!['error', 'closed'].includes(s.net?.phase), JSON.stringify(s.net));
      maxChecks = Math.max(maxChecks, s.net.verifiedChecksums);
      maxShots = Math.max(maxShots, s.snapshot?.bullets.length ?? 0);
    }
    if (states.every(s => s.online.screen === 'result')) break;
    const ci = states.findIndex(s => s.online.screen === 'next' && s.online.next?.mine);
    if (ci >= 0 && states[1 - ci].online.screen === 'next') {
      await Promise.all([A, B].map((p, i) => setKeys(p, new Set(), held[i])));
      choices.push(await nextShip([A, B], ci, how(ci)));
      continue;
    }
    // The message after a battle: the first one (a choice comes) and the first last ship.
    const over = states.find(s => s.state === 'roundOver');
    if (over && !messages.some(m => m.battle === over.fleet.battle)) {
      const sides = over.fleet.sides;
      const kind = sides.some(side => side.ships.length === side.lost) ? 'end' : over.fleet.chooser !== null ? 'choice' : 'last';
      if ((kind === 'choice' && !messages.length) || (kind === 'last' && !messages.some(m => m.kind === 'last'))) {
        await Promise.all([A, B].map((p, i) => setKeys(p, new Set(), held[i])));
        const message = await battleMessage(kind, over.fleet.battle);
        messages.push(message);
        if (kind === 'last') {
          // No choice with one ship left: the next battle starts by itself with that ship on both pages.
          const next = await poll(async () => {
            const now = await Promise.all([A, B].map(read));
            assert.ok(now.every(s => s.online.screen !== 'next'), 'no choice of the next ship with one ship left');
            return now.every(s => s.fleet.battle === message.battle + 1 && s.state === 'playing') ? now : null;
          }, 30000, 'the battle with the last ship');
          for (const s of next) {
            assert.equal(NAMES[s.fleet.current[message.loser]], message.last);
            assert.equal(s.snapshot.players[message.loser].shipId, SHIP_IDS[NAMES.indexOf(message.last)]);
            assert.equal(s.online.screen, 'battle');
          }
          message.nextBattle = true;
        }
        continue;
      }
      messages.push({ kind, battle: over.fleet.battle });
    }
    if (Date.now() - started > (process.env.CI ? 600000 : 480000)) throw new Error('The fleet match did not finish: ' + JSON.stringify(states.map(s => s.net)));
    await Promise.all([A, B].map((p, i) => setKeys(p, states[i].net.phase === 'playing' && states[i].snapshot && i !== idle(states[i].fleet.battle) ? botKeys(states[i].snapshot, i) : new Set(), held[i])));
    await new Promise(r => setTimeout(r, 120));
  }
  await Promise.all([A, B].map((p, i) => setKeys(p, new Set(), held[i])));
  const ended = await Promise.all([A, B].map(read));
  assert.ok(ended.every(s => s.net.phase === 'ended' && s.net.verifiedChecksums >= 5), 'both peers verify the state many times');
  assert.ok(maxShots > 0, 'weapons fire');
  assert.ok(choices.length >= 1, 'the first loss of a fleet of three always has a choice');
  assert.ok(choices.some(c => c.how === 'keyboard' && c.player === 1), 'the guest chose the next ship with the keyboard: ' + JSON.stringify(choices));
  assert.ok(messages.some(m => m.kind === 'choice' && m.texts), 'the message before a choice on both pages: ' + JSON.stringify(messages));
  assert.ok(messages.some(m => m.kind === 'last' && m.nextBattle), 'the last ship came by itself on both pages: ' + JSON.stringify(messages));
  result.checks.match = { ticks: ended.map(s => s.net.tick), verifiedChecksums: ended.map(s => s.net.verifiedChecksums), battles: ended.map(s => s.fleet.battle), choices: [...choices], messages, elapsedMs: Date.now() - started };
  assert.deepEqual(ended[0].fleet.sides, ended[1].fleet.sides, 'both pages end with the same fleets');
  const titles = await Promise.all([A, B].map(p => p.locator('#cosmo-online h1').textContent()));
  assert.deepEqual([...titles].sort(), ['Defeat', 'Victory']);
  await capture(A, 'result', 'host', SIZES, { next: true });
  await capture(B, 'result', 'guest', [[1280, 720]]);

  // Rematch: both must accept; the fleet screen keeps the last fleet.
  assert.equal(await activeText(A), 'Rematch');
  await A.keyboard.press('Enter');
  await A.waitForFunction(() => window.__cosmo.netplay.localRematch);
  await B.waitForFunction(() => document.querySelector('[data-item="rematch"]')?.textContent === 'Accept rematch');
  await capture(A, 'result-rematch-requested', 'host');
  await capture(B, 'result-rematch-offer', 'guest');
  await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'fleet'), screen(B, 'fleet')]);
  assert.equal((await read(A)).net.round, 1);
  assert.equal(await activeText(A), 'Ready · 3 ships');
  await capture(A, 'fleet-rematch', 'host', [[1280, 720]]);
  // A new fleet screen ignores Enter for 600 ms (a double press of Accept rematch).
  await A.waitForTimeout(700);
  await A.keyboard.press('Enter'); await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'battle'), screen(B, 'battle')]);
  await poll(async () => (await Promise.all([A, B].map(read))).every(s => s.net.verifiedChecksums >= 3), 30000, 'rematch checksums');
  result.checks.rematch = true;
  // The rematch: the time to choose runs out (when the first match had no second choice), then the next battle.
  if (!choices.some(c => c.how === 'timeout')) {
    const again = Date.now();
    for (;;) {
      const states = await Promise.all([A, B].map(read));
      for (const s of states) assert.ok(!['error', 'closed', 'ended'].includes(s.net?.phase), JSON.stringify(s.net));
      const ci = states.findIndex(s => s.online.screen === 'next' && s.online.next?.mine);
      if (ci >= 0 && states[1 - ci].online.screen === 'next') {
        await Promise.all([A, B].map((p, i) => setKeys(p, new Set(), held[i])));
        choices.push(await nextShip([A, B], ci, 'timeout'));
        break;
      }
      if (Date.now() - again > (process.env.CI ? 480000 : 300000)) throw new Error('No choice of the next ship in the rematch');
      await Promise.all([A, B].map((p, i) => setKeys(p, states[i].net.phase === 'playing' && states[i].snapshot ? botKeys(states[i].snapshot, i) : new Set(), held[i])));
      await new Promise(r => setTimeout(r, 120));
    }
    const checks = (await Promise.all([A, B].map(read))).map(s => s.net.verifiedChecksums);
    await poll(async () => (await Promise.all([A, B].map(read))).every((s, i) => s.net.verifiedChecksums >= checks[i] + 3), 30000, 'checksums after the choice');
  }
  const timeout = choices.find(c => c.how === 'timeout');
  assert.ok(timeout && timeout.ship === timeout.ships[0], 'when the time runs out the first ship left in fleet order comes');
  result.checks.nextShip = choices;
  await Promise.all([A, B].map((p, i) => setKeys(p, new Set(), held[i])));

  // The guest leaves with Esc and Leave: both back in the lobby with a message.
  await B.keyboard.press('Escape'); await B.keyboard.press('ArrowRight'); await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'lobby'), screen(B, 'lobby')]);
  assert.equal((await read(B)).online.notice, 'You left the room.');
  await A.waitForFunction(() => window.__cosmo.online.notice === 'Your opponent left the room.');
  await capture(B, 'lobby-you-left', 'guest', [[1280, 720]]);
  await capture(A, 'lobby-opponent-left', 'host');

  // Open room before the room list answers: the game waits for the list, then the room is in the list.
  lobbyGate = new Promise(resolve => { openLobby = resolve; });
  await B.keyboard.press('Escape');
  await B.waitForFunction(() => window.__cosmo.state === 'menu');
  await B.keyboard.press('Enter');
  await screen(B, 'lobby');
  assert.equal((await read(B)).online.lobby, 'connecting');
  await B.keyboard.press('Enter');
  await screen(B, 'name');
  const nameB = await B.locator('#cosmo-online input').inputValue();
  await B.keyboard.press('Enter');
  await B.waitForFunction(() => window.__cosmo.online.opening);
  assert.equal((await read(B)).online.screen, 'name');
  await capture(B, 'name-checking-list', 'guest');
  openLobby();
  await screen(B, 'waiting');
  assert.equal((await read(B)).online.room, nameB);
  await A.waitForFunction(name => window.__cosmo.online.rooms.includes(name), nameB);
  await B.keyboard.press('Escape');
  await screen(B, 'lobby');
  await A.waitForFunction(name => !window.__cosmo.online.rooms.includes(name), nameB);
  result.checks.openRoomWaitsForList = true;

  // A lost room list: Try again reconnects.
  if (app.adapter) {
    app.adapter.setAvailable(false);
    await A.waitForFunction(() => window.__cosmo.online.lobby === 'error');
    await capture(A, 'lobby-lost', 'host');
    app.adapter.setAvailable(true);
    assert.deepEqual((await read(A)).online.items.slice(0, 1), ['retry']);
    await A.keyboard.press('ArrowLeft');
    assert.equal(await activeText(A), 'Try again');
    await A.keyboard.press('Enter');
    await A.waitForFunction(() => window.__cosmo.online.lobby === 'open');
    result.checks.lostList = true;
  }
  // The name is remembered (and selected); Esc goes back step by step to the main menu.
  await A.keyboard.press('Enter');
  await screen(A, 'name');
  assert.equal(await A.locator('#cosmo-online input').inputValue(), ROOM);
  assert.deepEqual(await A.evaluate(() => { const input = document.querySelector('#cosmo-online input'); return [input.selectionStart, input.selectionEnd]; }), [0, ROOM.length]);
  await A.keyboard.press('Escape'); await screen(A, 'lobby');
  await A.keyboard.press('Escape');
  await A.waitForFunction(() => window.__cosmo.state === 'menu');
  assert.equal(await activeText(A), 'Online Melee');
  // F turns full screen off; then menu choices do not turn it on again; F or the corner button does.
  await A.waitForFunction(() => window.__cosmo.fullscreen === 'cosmo-frame');
  await A.keyboard.press('f');
  await A.waitForFunction(() => window.__cosmo.fullscreen === null);
  // The label follows the fullscreenchange event (one frame after the change).
  await A.waitForFunction(() => document.querySelector('.menu-fullscreen')?.textContent === 'Full screen (F)');
  await capture(A, 'menu-windowed', 'host', [[1280, 720]]);
  await A.keyboard.press('Enter');
  await screen(A, 'lobby');
  await A.waitForTimeout(300);
  assert.equal((await read(A)).fullscreen, null, 'a choice does not undo F');
  await A.keyboard.press('Escape');
  await A.waitForFunction(() => window.__cosmo.state === 'menu');
  await A.locator('.menu-fullscreen').click();
  await A.waitForFunction(() => window.__cosmo.fullscreen === 'cosmo-frame');
  await A.waitForFunction(() => document.querySelector('.menu-fullscreen')?.textContent === 'Exit full screen (F)');
  assert.equal(await activeText(A), 'Online Melee', 'the corner button leaves the focus in the menu');
  result.checks.fullscreenKey = true;
  const hostSockets = sink.sockets.filter(s => /role=host&name=/.test(s));
  assert.ok(hostSockets.length >= 1 && hostSockets.every(s => s.endsWith('&engine=rust')), JSON.stringify(sink.sockets));
  assert.ok(sink.sockets.includes('/netplay/lobby?edition=cosmoclash'));
  result.checks.listedRooms = true;
  await Promise.all([A, B].map(p => p.context().close()));

  // ---------------------------------------------------------------- private room and invite link (Rust relay, mouse)
  const rust = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createRustRelay({ distDir: false }), lobby: 'relay' });
  servers.push(rust);
  const rustPort = (await rust.listen(0)).port;
  const rustURL = `http://127.0.0.1:${rustPort}/cosmoclash/`;
  const C = await newPage(browsers[0], 2, { permissions: ['clipboard-read', 'clipboard-write'] });
  await boot(C, rustURL);
  await C.keyboard.press(' ');
  await C.waitForFunction(() => window.__cosmo?.state === 'menu');
  await C.getByRole('menuitem', { name: 'Online Melee' }).click();
  await C.waitForFunction(() => window.__cosmo.online.lobby === 'unsupported');
  await capture(C, 'lobby-unsupported', 'host');
  await C.getByRole('button', { name: 'Create private room' }).click();
  await screen(C, 'waiting');
  const link = await C.locator('.cc-link').textContent();
  assert.match(link, new RegExp(`^http://127\\.0\\.0\\.1:${rustPort}/cosmoclash/\\?cosmoRoom=[0-9a-f]{32}$`));
  await capture(C, 'waiting-private', 'host');
  await C.getByRole('button', { name: 'Copy invite link' }).click();
  await C.waitForFunction(() => /Invite link copied|cosmoRoom=/.test(window.__cosmo.online.notice));
  result.checks.copied = (await read(C)).online.notice;
  await capture(C, 'waiting-private-copied', 'host', [[1280, 720]]);
  const D = await newPage(browsers[1], 3);
  await boot(D, link);
  await D.keyboard.press(' ');
  await screen(D, 'invite');
  await capture(D, 'invite', 'guest');
  await D.getByRole('button', { name: 'Join', exact: true }).click();
  await Promise.all([screen(C, 'fleet'), screen(D, 'fleet')]);
  assert.equal(new URL(D.url()).searchParams.has('cosmoRoom'), false, 'the invite leaves the address after Join');
  // Mouse: the arrow browses, a click on the ship card adds it, Ready.
  await D.waitForTimeout(2600);
  await D.getByRole('button', { name: 'Next ship' }).click();
  await D.locator('[data-item="ship"]').click();
  assert.deepEqual((await read(D)).online.picks, [1]);
  await D.locator('[data-item="ready"]').click();
  await C.keyboard.press('Enter'); await C.keyboard.press('ArrowDown'); await C.keyboard.press('Enter');
  await Promise.all([screen(C, 'battle'), screen(D, 'battle')]);
  await D.getByRole('button', { name: 'Leave (Esc)' }).click();
  await D.waitForFunction(() => window.__cosmo.online.confirm);
  await D.getByRole('button', { name: 'Leave', exact: true }).click();
  await Promise.all([screen(C, 'lobby'), screen(D, 'lobby')]);
  await C.waitForFunction(() => window.__cosmo.online.notice === 'Your opponent left the room.');
  await capture(C, 'lobby-unsupported-opponent-left', 'host', [[1280, 720]]);
  result.checks.privateRoom = true;

  assert.deepEqual(sink.errors, []);
  assert.deepEqual(sink.requests, []);
  result.passed = true;
} catch (error) {
  result.error = String(error && error.stack || error);
  throw error;
} finally {
  Object.assign(result, { errors: sink.errors, requests: sink.requests, sockets: sink.sockets, screenshots: shots, nextShipScreenshots: next.files });
  await fs.writeFile(path.join(output, 'results.json'), JSON.stringify(result, null, 2));
  await Promise.all(browsers.map(b => b.close().catch(() => {})));
  for (const server of servers) await server.close().catch(() => {});
  console.log(JSON.stringify({ passed: result.passed, lobby: result.lobby, checks: result.checks, screenshots: shots.length, error: result.error }, null, 1));
}
