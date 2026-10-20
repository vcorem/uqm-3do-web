// Online Melee started from the game's own main menu, without --web-netplay.
// Both pages launch the game normally. The test acts as the page of engine
// contract (A): it opens a room with the shared room client
// (UQMNetplay.connect), sets Module.webNet to it and then gives the engine its
// role once through Module.takePendingNetplayRole(). The engine plays the
// match, returns to its main menu after the disconnect, and starts a second
// session in the same pages. Real keyboard input, real relay, and the
// engine's own checksums; no game state is patched.
//
// Session 1 follows the online lobby's order without any hold: the room is
// paired while the main menu stays idle for more than 20 s (the Original menu
// times out into the title and the credits then), and Module.webNet is set
// only when the role is queued. Session 2 holds the main menu as a lobby page
// does: first with Module.holdMainMenu, then with a ready room in
// Module.webNet; the menu must not time out.
//
// The test uses only the contract and the shell's start path (the launcher's
// Start button or the in-frame start screen's Play), so it works with both
// page shells.
import assert from 'node:assert/strict';
import { mkdir, writeFile, readFile } from 'node:fs/promises';
import { createHash, randomBytes } from 'node:crypto';
import path from 'node:path';
import { chromium, firefox, webkit } from 'playwright';
import { createGameBackend } from '../helpers/game-backend.mjs';

const browserEngines = { chromium, firefox, webkit };
const selectedEngines = [
  process.env.UQM_BROWSER || 'chromium',
  process.env.UQM_PEER_BROWSER || process.env.UQM_BROWSER || 'chromium',
];
for (const [index, engine] of selectedEngines.entries()) {
  if (!Object.hasOwn(browserEngines, engine)) {
    throw new Error(`${index === 0 ? 'UQM_BROWSER' : 'UQM_PEER_BROWSER'} must be chromium, firefox, or webkit; received ${JSON.stringify(engine)}`);
  }
}
function browserLaunchOptions(engine) {
  const options = { headless: process.env.HEADED !== '1' };
  if (engine === 'chromium') {
    if (process.env.CHROME_BIN) options.executablePath = process.env.CHROME_BIN;
    options.args = ['--autoplay-policy=no-user-gesture-required', '--disable-renderer-backgrounding',
      '--disable-backgrounding-occluded-windows', '--disable-background-timer-throttling'];
  }
  return options;
}

const distDir = path.resolve(process.env.UQM_DIST || 'dist');
const peerDist = process.env.UQM_PEER_DIST ? path.resolve(process.env.UQM_PEER_DIST) : null;
const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/netplay-hook');
const edition = process.env.UQM_EDITION || 'original';
assert.ok(['original', 'hd'].includes(edition), 'UQM_EDITION must be original or hd');
const sessions = Number(process.env.UQM_HOOK_SESSIONS || 2);
assert.ok(Number.isInteger(sessions) && sessions >= 1, 'UQM_HOOK_SESSIONS must be a positive integer');
// More than the 20 s main menu time-out of the Original edition without menu music.
const idleMs = Number(process.env.UQM_HOOK_IDLE_MS || 25000);
const meleeConfig = `/home/web_user/${edition === 'hd' ? '.uqm-hd' : '.uqm'}/melee.cfg`;
const roles = ['host', 'guest'];
await mkdir(output, { recursive: true });
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const report = { edition, distDir, peerDist, idleMs, browsers: [], sessions: [], passed: false };

async function hashes(directory) {
  return Object.fromEntries(await Promise.all(['index.html', 'uqm-debug.wasm', 'netplay-client.js'].map(async file =>
    [file, createHash('sha256').update(await readFile(path.join(directory, file))).digest('hex')])));
}
report.buildHashes = await hashes(distDir);
if (peerDist) {
  report.peerBuildHashes = await hashes(peerDist);
  assert.notEqual(report.peerBuildHashes['uqm-debug.wasm'], report.buildHashes['uqm-debug.wasm'],
    'Differential testing requires independent C and Rust engine builds');
}

async function readState(page) {
  return page.evaluate(() => {
    const net = Module.webNet;
    return { ...(net ? net.stats : {}), connection: net ? net.state : 'none', screen: Module.gameScreen };
  });
}
function readStates(pages) {
  return Promise.all(pages.map(readState));
}
async function waitState(pages, state, timeout = 60000) {
  await Promise.all(pages.map(page => page.waitForFunction(
    state => Module.webNet && Module.webNet.stats.engineState === state, state, { polling: 100, timeout })));
}
async function key(pages, name, delay = 150) {
  await Promise.all(pages.map(page => page.keyboard.press(name, { delay })));
}
// Enter on each page that is not yet in the state. A press can be lost in a
// screen change (the engine flushes its input then), so the test presses
// again instead of counting presses.
async function enterUntil(pages, state, timeout = 60000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const states = await readStates(pages);
    if (states.every(s => s.engineState === state)) return;
    assert.ok(states.every(s => s.connection === 'ready'), JSON.stringify(states));
    await Promise.all(pages.map((page, i) =>
      states[i].engineState === state ? null : page.keyboard.press('Enter', { delay: 150 })));
    await sleep(600);
  }
  throw new Error(`Did not reach ${state}: ${JSON.stringify(await readStates(pages))}`);
}
// The battle's frame counters show whether the simulation still runs. The
// wait ends with an error only when nothing moves for stallMs (or after
// maxMs), so a slow machine does not fail a match that is still progressing.
function progress(states) {
  return states.map(s => `${s.sentFrames}/${s.receivedFrames}/${s.checksumsVerified || 0}/${s.engineState}`).join(' ');
}
async function untilDone(pages, done, label, { stallMs = 20000, maxMs = 180000, step } = {}) {
  const start = Date.now();
  let last = '';
  let lastChange = Date.now();
  while (Date.now() - start < maxMs) {
    const states = await readStates(pages);
    if (done(states)) return states;
    assert.ok(states.every(s => s.connection === 'ready'), JSON.stringify(states));
    const now = progress(states);
    if (now !== last) { last = now; lastChange = Date.now(); }
    if (Date.now() - lastChange > stallMs) throw new Error(`${label}: no progress for ${stallMs} ms: ${JSON.stringify(states)}`);
    if (step) await step(states);
    else await sleep(250);
  }
  throw new Error(`${label}: not done after ${maxMs} ms: ${JSON.stringify(await readStates(pages))}`);
}
async function checkSynchronization(pages, minimum) {
  await untilDone(pages, states => states.every(s => (s.checksumsVerified || 0) >= minimum),
    `${minimum} verified checksums`);
  const states = await readStates(pages);
  for (const state of states) {
    assert.equal(state.checksumMismatches || 0, 0, JSON.stringify(state));
    assert.equal(state.connection, 'ready');
    assert.ok(state.receivedFrames > 0 && state.sentFrames > 0);
  }
  return states;
}
function canvasBytes(page) {
  return page.locator('#canvas').screenshot();
}
async function canvasShot(page, name) {
  await writeFile(path.join(output, name), await canvasBytes(page));
}

// The same saved Melee configuration as tests/browser/netplay.mjs: two
// Earthlings and two Shofixti, so a fleet can be finished with keyboard input.
function seedFleet({ file, guestShip }) {
  const bytes = new Uint8Array(140);
  for (let side = 0; side < 2; side++) {
    const offset = side * 70;
    bytes[offset] = 17; // HUMAN_CONTROL | STANDARD_RATING
    bytes.fill(255, offset + 1, offset + 15);
    bytes[offset + 1] = bytes[offset + 2] = side === 1 ? 13 : guestShip;
    bytes.set(new TextEncoder().encode(side ? 'Test host' : 'Test guest'), offset + 15);
  }
  FS.mkdirTree(file.slice(0, file.lastIndexOf('/')));
  FS.writeFile(file, bytes);
  return Array.from(bytes);
}

// Contract (A) from the page's side. The engine calls the function in every
// main-menu frame; until a role is queued it returns -1, and a queued role
// is returned once. The page lowers its menu hold when the role is taken.
function installHook() {
  window.uqmTestPendingRole = -1;
  window.uqmTestRolesTaken = 0;
  window.uqmTestPolls = 0;
  Module.takePendingNetplayRole = () => {
    window.uqmTestPolls++;
    const role = window.uqmTestPendingRole;
    window.uqmTestPendingRole = -1;
    if (role !== -1) {
      window.uqmTestRolesTaken++;
      Module.holdMainMenu = false;
    }
    return role;
  };
}

// The shell's own start path: #start is the launcher's Start button, or the
// Play item of the in-frame start screen. Without it, Enter on the start
// screen's focused Play.
async function launchGame(page) {
  if (await page.locator('#start').count()) {
    await page.locator('#start').click();
    return page.evaluate(() => typeof online !== 'undefined' && online ? 'start screen Play (#start)' : 'launcher Start button');
  }
  await page.waitForFunction(() => typeof online !== 'undefined' && online
    && online.state.screen === 'start' && online.state.focus === 'play', null, { polling: 100, timeout: 30000 });
  await page.keyboard.press('Enter');
  return 'start screen Play';
}

// A room of the shared client. As the lobby does, the page keeps it to
// itself until the role is queued.
function openTestRoom({ role, token, edition }) {
  if (window.uqmTestRoom) window.uqmTestRoom.close();
  window.uqmTestRoom = UQMNetplay.connect({ token, role, edition });
  return window.uqmTestRoom.state;
}

// The relay knows a room only after the host's connection; a guest that is
// too early receives an error and joins again.
async function joinTestRoom(page, token) {
  for (let attempt = 0; attempt < 20; attempt++) {
    await page.evaluate(openTestRoom, { role: 'guest', token, edition });
    const handle = await page.waitForFunction(() => window.uqmTestRoom.state !== 'waiting' && window.uqmTestRoom.state,
      null, { polling: 100, timeout: 20000 });
    if (await handle.jsonValue() === 'ready') return attempt;
    await sleep(500);
  }
  throw new Error('The guest could not join the room');
}

// A popup (why the room closed, or the main menu's mouse message) closes
// with one key, and the engine returns to its main menu. Press again only
// while the popup stays: a key that arrives in the main menu would select a
// menu item.
async function backToMainMenu(page, label, popup) {
  for (let attempt = 0; attempt < 3; attempt++) {
    if (await page.evaluate(() => Module.gameScreen === 'mainMenu')) return attempt;
    await page.keyboard.press('Enter', { delay: 150 });
    const start = Date.now();
    let bytes = await canvasBytes(page);
    while (bytes.equals(popup) && Date.now() - start < 8000) {
      await sleep(200);
      bytes = await canvasBytes(page);
    }
    if (!bytes.equals(popup)) {
      await page.waitForFunction(() => Module.gameScreen === 'mainMenu', null, { polling: 100, timeout: 120000 });
      return attempt + 1;
    }
  }
  throw new Error(`${label} did not leave the popup`);
}

// One key leaves the battle result; press again only if the picture stays.
async function acknowledge(page, before) {
  for (let attempt = 0; attempt < 3; attempt++) {
    await page.keyboard.press('Enter', { delay: 150 });
    const start = Date.now();
    while (Date.now() - start < 6000) {
      await sleep(200);
      if (!(await canvasBytes(page)).equals(before)) return attempt + 1;
    }
  }
  throw new Error('The battle result did not close');
}

// After the room closes, wait until the picture changes (the engine's popup),
// so that the screenshot shows what the player sees.
async function shotAfterChange(page, before, name, timeout = 15000) {
  const start = Date.now();
  let bytes = await canvasBytes(page);
  while (bytes.equals(before) && Date.now() - start < timeout) {
    await sleep(200);
    bytes = await canvasBytes(page);
  }
  // The popup's frame and text can take a few frames.
  await sleep(500);
  bytes = await canvasBytes(page);
  await writeFile(path.join(output, name), bytes);
  return { changed: !bytes.equals(before), afterMs: Date.now() - start, bytes };
}

// The engine reports 'mainMenu' only while it polls the hook. Its blocking
// sub-screens report 'transition': the Setup menu (both editions) and the
// "mouse not supported" popup (Original). The page offers the lobby only in
// 'mainMenu'.
async function checkMenuSubScreens(page, label) {
  const screen = () => page.evaluate(() => Module.gameScreen);
  const press = async name => { await page.keyboard.press(name, { delay: 120 }); await sleep(250); };
  // In the main menu: the next key only after two engine frames (and some
  // time for the page's event loop) have seen this key up. Two presses that
  // reach the engine in the same frame count as one held key.
  const menuKey = async name => {
    await page.keyboard.press(name, { delay: 120 });
    const polls = await page.evaluate(() => window.uqmTestPolls);
    await Promise.all([sleep(300),
      page.waitForFunction(polls => window.uqmTestPolls >= polls + 2, polls, { polling: 20, timeout: 10000 })]);
  };
  const result = {};
  // New Game, Load Game, Super Melee, Setup: three steps down.
  for (let i = 0; i < 3; i++) await menuKey('ArrowDown');
  await press('Enter');
  await page.waitForFunction(() => Module.gameScreen === 'transition', null, { polling: 100, timeout: 10000 });
  await sleep(2500);
  result.setup = await screen();
  assert.equal(result.setup, 'transition', `${label}: the Setup menu must not report the main menu`);
  await canvasShot(page, `setup-menu-${label}.png`);
  for (let attempt = 0; attempt < 6 && await screen() !== 'mainMenu'; attempt++) {
    if (await screen() === 'melee') throw new Error(`${label}: Super Melee opened instead of Setup (a menu key was lost)`);
    await press('Escape');
    await page.waitForFunction(() => Module.gameScreen === 'mainMenu', null, { polling: 100, timeout: 4000 }).catch(() => {});
  }
  result.afterSetup = await screen();
  assert.equal(result.afterSetup, 'mainMenu', `${label}: back from Setup`);
  if (edition === 'original') {
    // The engine reads the mouse button state once per frame: hold it down,
    // and click again if a slow frame missed it.
    for (let attempt = 0; attempt < 4 && await screen() === 'mainMenu'; attempt++) {
      await page.locator('#canvas').click({ delay: 800 });
      await page.waitForFunction(() => Module.gameScreen === 'transition', null, { polling: 100, timeout: 4000 }).catch(() => {});
    }
    result.popup = await screen();
    assert.equal(result.popup, 'transition', `${label}: the mouse popup must not report the main menu`);
    await sleep(500);
    const popup = await canvasBytes(page);
    await writeFile(path.join(output, `mouse-popup-${label}.png`), popup);
    await backToMainMenu(page, label, popup);
    result.afterPopup = await screen();
    assert.equal(result.afterPopup, 'mainMenu', `${label}: back from the popup`);
  }
  return result;
}

// No input, as while the player looks at the lobby or waits for an opponent.
async function idle(pages, entry) {
  const screens = [];
  const screen = () => Promise.all(pages.map(page => page.evaluate(() => Module.gameScreen)));
  if (entry.mode === 'hold') {
    // The lobby is open: the page holds the main menu.
    await Promise.all(pages.map(page => page.evaluate(() => { Module.holdMainMenu = true; })));
    await sleep(idleMs / 2);
    screens.push(await screen());
    // A ready room in Module.webNet holds it as well.
    await Promise.all(pages.map(page => page.evaluate(() => {
      Module.holdMainMenu = false;
      Module.webNet = window.uqmTestRoom;
    })));
    await sleep(idleMs / 2);
    screens.push(await screen());
    for (const list of screens) assert.deepEqual(list, ['mainMenu', 'mainMenu'], 'The held main menu timed out');
    await Promise.all(pages.map(page => page.evaluate(() => { Module.holdMainMenu = true; })));
  } else {
    await sleep(idleMs);
    screens.push(await screen());
  }
  entry.screensAfterIdle = screens;
  await Promise.all(pages.map((page, i) => canvasShot(page, `session${entry.number}-idle-${roles[i]}.png`)));
  return screens[screens.length - 1];
}

async function playSession(pages, number) {
  const entry = { number, mode: number === 1 ? 'no hold' : 'hold', passed: false };
  report.sessions.push(entry);
  const token = randomBytes(16).toString('hex');
  entry.token = token;
  for (const page of pages) {
    assert.equal(await page.evaluate(() => Module.gameScreen), 'mainMenu');
    assert.equal(await page.evaluate(() => Module.supportsNetplayHook), true);
    assert.equal(await page.evaluate(() => Module.webNet ? Module.webNet.state : 'none'), number === 1 ? 'none' : 'closed');
  }
  // The lobby pairs the players while the main menu is idle.
  entry.hostState = await pages[0].evaluate(openTestRoom, { role: 'host', token, edition });
  await sleep(500);
  entry.guestJoinRetries = await joinTestRoom(pages[1], token);
  await pages[0].waitForFunction(() => window.uqmTestRoom.state === 'ready', null, { polling: 100, timeout: 20000 });
  const screens = await idle(pages, entry);
  if (edition === 'original' && entry.mode === 'no hold') {
    // Without menu music the main menu times out after 20 s into the title
    // and the credits. The engine must take the role there as well.
    entry.attractSequence = screens.every(screen => screen === 'transition');
    if (!entry.attractSequence) console.log(`${edition} session ${number}: the main menu did not time out (${screens}); the credits path is not used`);
  }
  for (const page of pages) assert.equal(await page.evaluate(() => Module.webNet && Module.webNet.stats.engineState), undefined);

  // Opponent found: the page sets Module.webNet and queues the role.
  const queuedAt = Date.now();
  await Promise.all(pages.map((page, i) => page.evaluate(role => {
    Module.webNet = window.uqmTestRoom;
    window.uqmTestPendingRole = role;
  }, i)));
  await Promise.all(pages.map(page => page.waitForFunction(number => window.uqmTestRolesTaken === number
    && window.uqmTestPendingRole === -1, number, { polling: 50, timeout: entry.mode === 'hold' ? 10000 : 30000 })));
  entry.roleTakenAfterMs = Date.now() - queuedAt;
  console.log(`${edition} session ${number} (${entry.mode}): ${screens.join('/')} after ${idleMs} ms without input; roles taken after ${entry.roleTakenAfterMs} ms`);
  for (const page of pages) assert.equal(await page.evaluate(() => Module.holdMainMenu), false);
  await waitState(pages, 'inSetup');
  for (const page of pages) assert.equal(await page.evaluate(() => Module.gameScreen), 'melee');
  console.log(`${edition} session ${number}: both players in fleet setup`);
  await Promise.all(pages.map(page => page.locator('#canvas').click()));
  await sleep(1500);
  await Promise.all(pages.map((page, i) => canvasShot(page, `session${number}-setup-${roles[i]}.png`)));

  if (number === 1) {
    // Several status messages in a row on the host: confirm, cancel, confirm.
    await key([pages[0]], 'Enter');
    await sleep(1200);
    await canvasShot(pages[0], 'status-1-waiting-host.png');
    await key([pages[0]], 'Escape');
    await sleep(1200);
    await canvasShot(pages[0], 'status-2-cancelled-host.png');
    await key([pages[0]], 'Enter');
    await sleep(1200);
    await canvasShot(pages[0], 'status-3-waiting-again-host.png');
    await canvasShot(pages[1], 'status-3-guest.png');
  }

  await enterUntil(pages, 'selectShip');
  await enterUntil(pages, 'inBattle');
  await checkSynchronization(pages, 60);
  await Promise.all([pages[0].keyboard.down('ArrowUp'), pages[1].keyboard.down('ArrowUp'),
    pages[1].keyboard.down('ControlRight'), pages[1].keyboard.down('ArrowLeft')]);
  await sleep(3000);
  await Promise.all([pages[0].keyboard.up('ArrowUp'), pages[1].keyboard.up('ArrowUp'),
    pages[1].keyboard.up('ControlRight'), pages[1].keyboard.up('ArrowLeft')]);
  entry.destructPresses = [];
  for (let ship = 0; ship < 2; ship++) {
    const before = await readStates(pages);
    await checkSynchronization(pages, Math.max(...before.map(s => s.checksumsVerified || 0)) + 60);
    // The host's Shofixti fires its Glory Device after three presses of the
    // special key. Press it until the battle ends, while the battle runs.
    let presses = 0;
    await untilDone(pages, states => states.some(s => s.engineState !== 'inBattle'), `ship ${ship + 1} destruct`, {
      step: async () => { presses++; await key([pages[0]], 'ShiftRight', 300); await sleep(300); },
    });
    entry.destructPresses.push(presses);
    if (ship === 0) {
      await untilDone(pages, states => states.every(s => s.engineState === 'selectShip'), 'next ship picker', { stallMs: 60000 });
      console.log(`${edition} session ${number}: first ship destroyed; choosing next ship`);
      await enterUntil(pages, 'inBattle');
    }
  }
  // The last ship's destruction shows the battle result. The room is back
  // in fleet setup at once, but the result stays until the player presses a
  // key; then the fleet setup screen returns.
  await sleep(5000);
  await enterUntil(pages, 'inSetup');
  entry.statsAfterMatch = await checkSynchronization(pages, 130);
  entry.completedFleetMatch = true;
  await sleep(1000);
  const result = await Promise.all(pages.map(canvasBytes));
  await Promise.all(pages.map((page, i) => writeFile(path.join(output, `session${number}-result-${roles[i]}.png`), result[i])));
  entry.resultAcknowledged = await Promise.all(pages.map((page, i) => acknowledge(page, result[i])));
  await sleep(1500);
  const complete = await Promise.all(pages.map(canvasBytes));
  await Promise.all(pages.map((page, i) => writeFile(path.join(output, `session${number}-match-complete-${roles[i]}.png`), complete[i])));
  console.log(`${edition} session ${number}: full fleet match completed without checksum mismatch`);

  // The guest leaves the room in the page, as a lobby's Leave button does.
  // Both engines show why the room closed and return to their main menus.
  await pages[1].evaluate(() => Module.webNet.close());
  await pages[0].waitForFunction(() => Module.webNet.state === 'closed', null, { polling: 100, timeout: 20000 });
  const closed = await Promise.all(pages.map((page, i) =>
    shotAfterChange(page, complete[i], `session${number}-closed-${roles[i]}.png`)));
  entry.closed = closed.map(({ changed, afterMs }) => ({ changed, afterMs }));
  console.log(`${edition} session ${number}: room closed; screen changed ${JSON.stringify(entry.closed)}`);
  assert.ok(entry.closed.every(shot => shot.changed), 'The engines did not show that the room closed');
  entry.popupKeys = await Promise.all(pages.map((page, i) => backToMainMenu(page, roles[i], closed[i].bytes)));
  await Promise.all(pages.map(page => page.waitForFunction(() => Module.supportsNetplayHook === true,
    null, { polling: 100, timeout: 10000 })));
  await sleep(1500);
  await Promise.all(pages.map((page, i) => canvasShot(page, `session${number}-main-menu-${roles[i]}.png`)));
  for (const page of pages) assert.equal(await page.evaluate(() => Module.gameScreen), 'mainMenu');
  entry.returnedToMainMenu = true;
  entry.passed = true;
  console.log(`${edition} session ${number}: both engines returned to the main menu`);
}

const app = createGameBackend({ distDir, latencyPatternMs: [0] }, peerDist);
const browsers = [];
const pages = [];
const failures = [];
try {
  const address = await app.listen(0);
  const origin = `http://127.0.0.1:${address.port}`;
  for (let i = 0; i < 2; i++) {
    const engine = selectedEngines[i];
    const browser = await browserEngines[engine].launch(browserLaunchOptions(engine));
    browsers.push(browser);
    report.browsers.push({ role: roles[i], engine, version: browser.version() });
    const context = await browser.newContext({ viewport: { width: 1100, height: 1100 } });
    if (i === 1 && peerDist) {
      await context.addCookies([{ name: 'uqm_test_peer', value: '1', url: origin }]);
    }
    const page = await context.newPage();
    page.setDefaultTimeout(30000);
    page.on('pageerror', error => failures.push(`${i}: ${error.stack || error.message}`));
    page.on('crash', () => failures.push(`${i}: page crashed`));
    pages.push(page);
  }
  await Promise.all(pages.map(page => page.goto(origin)));
  await Promise.all(pages.map(page => page.waitForFunction(() => runtimeReady,
    null, { polling: 100, timeout: 300000 })));
  report.wasmHashes = await Promise.all(pages.map(page => page.evaluate(async () => {
    const bytes = await (await fetch('uqm-debug.wasm')).arrayBuffer();
    const digest = await crypto.subtle.digest('SHA-256', bytes);
    return Array.from(new Uint8Array(digest), b => b.toString(16).padStart(2, '0')).join('');
  })));
  assert.equal(report.wasmHashes[0], report.buildHashes['uqm-debug.wasm']);
  assert.equal(report.wasmHashes[1], (report.peerBuildHashes || report.buildHashes)['uqm-debug.wasm']);
  const fleet = { file: meleeConfig, guestShip: 5 };
  const expectedFleet = await pages[0].evaluate(seedFleet, fleet);
  await pages[1].evaluate(seedFleet, fleet);
  await Promise.all(pages.map(page => page.evaluate(() => new Promise((resolve, reject) =>
    FS.syncfs(false, error => error ? reject(error) : resolve())))));
  await Promise.all(pages.map(page => page.evaluate(installHook)));
  // A normal launch: the game, not an online session.
  report.launch = await Promise.all(pages.map(launchGame));
  await Promise.all(pages.map(page => page.waitForFunction(() => started, null, { polling: 100, timeout: 30000 })));
  await Promise.all(pages.map(page => page.locator('#canvas').focus()));
  await Promise.all(pages.map(page => page.waitForFunction(
    () => Module.gameScreen === 'mainMenu' && Module.supportsNetplayHook === true,
    null, { polling: 100, timeout: 180000 })));
  await sleep(1000);
  await Promise.all(pages.map((page, i) => canvasShot(page, `main-menu-${roles[i]}.png`)));
  console.log(`${edition}: both engines in the main menu with the netplay hook (${report.launch.join(', ')})`);
  report.menuSubScreens = await Promise.all(pages.map((page, i) => checkMenuSubScreens(page, roles[i])));
  console.log(`${edition}: sub-screens of the main menu report ${JSON.stringify(report.menuSubScreens)}`);
  for (let number = 1; number <= sessions; number++) await playSession(pages, number);
  for (const page of pages) {
    assert.deepEqual(await page.evaluate(file => Array.from(FS.readFile(file)), meleeConfig), expectedFleet);
    const logs = await page.locator('#log').textContent();
    assert.doesNotMatch(logs, /Aborted\(|Assertion failed|memory access out of bounds|Uncaught RuntimeError/);
  }
  assert.deepEqual(failures, []);
  report.passed = true;
  console.log(`All ${sessions} ${edition} online sessions from the main menu passed. See ${path.join(output, 'results.json')}.`);
} catch (error) {
  report.error = error.stack || String(error);
  report.browserErrors = failures;
  await Promise.all(pages.map(async (page, i) => {
    if (page.isClosed()) return;
    report[`state${i}`] = await Promise.race([readState(page).catch(e => e.message), sleep(2000).then(() => 'timeout')]);
    const log = await Promise.race([page.locator('#log').textContent().catch(() => ''), sleep(2000).then(() => '')]);
    await writeFile(path.join(output, `failure-${i}.log`), log || 'page unavailable');
    await page.screenshot({ path: path.join(output, `failure-${i}.png`), timeout: 5000 }).catch(() => {});
  }));
  throw error;
} finally {
  await writeFile(path.join(output, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  await Promise.allSettled(browsers.map(browser => browser.close()));
  await app.close();
}
