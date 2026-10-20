// Online Melee inside the game, with two real browsers through a real relay
// and the cosmoclash room list (the relay's own, or scripts/cosmo-lobby.mjs).
// The host creates a named room with the keyboard; the guest sees it in the
// list and joins with the keyboard; both choose fleets in the 3D ship choice;
// a real battle with verified checksums; rematch; the opponent leaves -> lobby.
// Then a relay without room lists (the Rust relay): a private room, its invite
// link and the Join screen, with the mouse. Every screen is saved at
// 1280x720, 1024x768 and 1920x1080 in test-results/online-ui/.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createNetplayServer } from '../../../server/netplay.mjs';
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch, poll, read, screen, setKeys, botKeys, watchPage, recordKeyboardLock, wasmHash } from './helpers.mjs';

const output = path.resolve(root, process.env.COSMO_ONLINE_UI_DIR || 'test-results/online-ui');
await fs.rm(output, { recursive: true, force: true });
await fs.mkdir(output, { recursive: true });
const SIZES = [[1280, 720], [1024, 768], [1920, 1080]];
const shots = [];
let counter = 0;
/** Saves the screen at the three sizes (or the first only) and returns to 1280x720. */
async function capture(page, name, who, sizes = SIZES) {
  const n = String(++counter).padStart(2, '0');
  for (const [width, height] of sizes) {
    if (width !== 1280 || height !== 720) { await page.setViewportSize({ width, height }); await page.waitForTimeout(120); }
    else await page.waitForTimeout(80);
    const file = path.join(output, `${n}-${name}-${who}-${width}x${height}.png`);
    await page.screenshot({ path: file });
    shots.push(path.relative(root, file));
  }
  await page.setViewportSize({ width: 1280, height: 720 });
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
  const lobbyGate = new Promise(resolve => { openLobby = resolve; });
  const roomGate = new Promise(resolve => { openRoom = resolve; });
  await B.routeWebSocket(/\/netplay\/lobby/, async ws => { await lobbyGate; ws.connectToServer(); });
  await B.routeWebSocket(/\/netplay\/cosmoclash\//, async ws => { await roomGate; ws.connectToServer(); });
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

  // Name dialog: a default name, a refused name with the reason, then the room.
  await A.keyboard.press('Enter');
  await screen(A, 'name');
  const defaultName = await A.locator('#cosmo-online input').inputValue();
  assert.match(defaultName, /^Pilot \d\d$/);
  await capture(A, 'name', 'host');
  await A.keyboard.press('ControlOrMeta+A'); await A.keyboard.type('x'.repeat(25)); await A.keyboard.press('Enter');
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
  await capture(A, 'fleet-two-picks', 'host');
  await A.keyboard.press('ArrowDown');
  assert.equal(await activeText(A), 'Ready · 2 ships');
  await A.keyboard.press('Enter');
  await A.waitForFunction(() => window.__cosmo.netplay?.localReady);
  await capture(A, 'fleet-ready-waiting', 'host');
  // The guest fills the fleet, takes two ships back out and finishes with one.
  await B.keyboard.press('Enter'); await B.keyboard.press('Enter'); await B.keyboard.press('Enter');
  assert.deepEqual((await read(B)).online.picks, [0, 1, 2]);
  assert.equal(await activeText(B), 'Ready · 3 ships');
  await B.waitForFunction(() => document.querySelector('.cc-opponent')?.textContent === 'Opponent: ready with 2 ships');
  await capture(B, 'fleet-full', 'guest');
  await B.keyboard.press('Backspace'); await B.keyboard.press('Backspace');
  assert.deepEqual((await read(B)).online.picks, [0]);
  await capture(B, 'fleet-one-pick', 'guest');
  await B.keyboard.press('ArrowDown'); await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'countdown'), screen(B, 'countdown')]);
  await capture(A, 'countdown', 'host');
  await capture(B, 'countdown', 'guest', [[1280, 720]]);
  await Promise.all([screen(A, 'battle'), screen(B, 'battle')]);
  const starts = await Promise.all([A, B].map(read));
  assert.deepEqual(starts.map(s => s.net.localFleet), [[1, 2], [0]]);
  await A.waitForTimeout(800);
  await capture(A, 'battle', 'host');
  await capture(B, 'battle', 'guest', [[1280, 720]]);
  // Esc asks first; Stay (the default) keeps the match.
  await B.keyboard.press('Escape');
  await B.waitForFunction(() => window.__cosmo.online.confirm);
  assert.equal(await activeText(B), 'Stay');
  await capture(B, 'battle-leave-question', 'guest');
  await B.keyboard.press('Enter');
  assert.equal((await read(B)).online.confirm, false);

  // A real battle with real keys, until the fleet match ends.
  const held = [new Set(), new Set()];
  const started = Date.now();
  let roundShot = false, maxChecks = 0, maxShots = 0;
  for (;;) {
    const states = await Promise.all([A, B].map(read));
    for (const s of states) {
      assert.ok(!['error', 'closed'].includes(s.net?.phase), JSON.stringify(s.net));
      maxChecks = Math.max(maxChecks, s.net.verifiedChecksums);
      maxShots = Math.max(maxShots, s.snapshot?.bullets.length ?? 0);
    }
    if (states.every(s => s.online.screen === 'result')) break;
    if (!roundShot && states[0].state === 'roundOver' && states[0].fleet.indices.some(n => n === 0)) {
      roundShot = true;
      await capture(A, 'battle-round-over', 'host', [[1280, 720]]);
    }
    if (Date.now() - started > (process.env.CI ? 480000 : 300000)) throw new Error('The fleet match did not finish: ' + JSON.stringify(states.map(s => s.net)));
    await Promise.all([A, B].map((p, i) => setKeys(p, states[i].net.phase === 'playing' && states[i].snapshot ? botKeys(states[i].snapshot, i) : new Set(), held[i])));
    await new Promise(r => setTimeout(r, 120));
  }
  await Promise.all([A, B].map((p, i) => setKeys(p, new Set(), held[i])));
  const ended = await Promise.all([A, B].map(read));
  assert.ok(ended.every(s => s.net.phase === 'ended' && s.net.verifiedChecksums >= 5), 'both peers verify the state many times');
  assert.ok(maxShots > 0, 'weapons fire');
  result.checks.match = { ticks: ended.map(s => s.net.tick), verifiedChecksums: ended.map(s => s.net.verifiedChecksums), battles: ended.map(s => s.fleet.battle), elapsedMs: Date.now() - started };
  const titles = await Promise.all([A, B].map(p => p.locator('#cosmo-online h1').textContent()));
  assert.deepEqual([...titles].sort(), ['Defeat', 'Victory']);
  await capture(A, 'result', 'host');
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
  assert.equal(await activeText(A), 'Ready · 2 ships');
  await capture(A, 'fleet-rematch', 'host', [[1280, 720]]);
  await A.keyboard.press('Enter'); await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'battle'), screen(B, 'battle')]);
  await poll(async () => (await Promise.all([A, B].map(read))).every(s => s.net.verifiedChecksums >= 3), 30000, 'rematch checksums');
  result.checks.rematch = true;

  // The guest leaves with Esc and Leave: both back in the lobby with a message.
  await B.keyboard.press('Escape'); await B.keyboard.press('ArrowRight'); await B.keyboard.press('Enter');
  await Promise.all([screen(A, 'lobby'), screen(B, 'lobby')]);
  assert.equal((await read(B)).online.notice, 'You left the room.');
  await A.waitForFunction(() => window.__cosmo.online.notice === 'Your opponent left the room.');
  await capture(B, 'lobby-you-left', 'guest', [[1280, 720]]);
  await capture(A, 'lobby-opponent-left', 'host');

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
  // The name is remembered; Esc goes back step by step to the main menu.
  await A.keyboard.press('Enter');
  await screen(A, 'name');
  assert.equal(await A.locator('#cosmo-online input').inputValue(), ROOM);
  await A.keyboard.press('Escape'); await screen(A, 'lobby');
  await A.keyboard.press('Escape');
  await A.waitForFunction(() => window.__cosmo.state === 'menu');
  assert.equal(await activeText(A), 'Online Melee');
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
  Object.assign(result, { errors: sink.errors, requests: sink.requests, sockets: sink.sockets, screenshots: shots });
  await fs.writeFile(path.join(output, 'results.json'), JSON.stringify(result, null, 2));
  await Promise.all(browsers.map(b => b.close().catch(() => {})));
  for (const server of servers) await server.close().catch(() => {});
  console.log(JSON.stringify({ passed: result.passed, lobby: result.lobby, checks: result.checks, screenshots: shots.length, error: result.error }, null, 1));
}
