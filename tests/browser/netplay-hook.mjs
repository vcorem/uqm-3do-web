// Online Melee started from the game's own main menu, without --web-netplay.
// Both pages launch the game normally. The page opens a room and then gives
// the engine its role through Module.takePendingNetplayRole(). The engine
// plays the match, returns to its main menu after the disconnect, and starts
// a second session in the same pages. Real keyboard input, real relay, and
// the engine's own checksums; no game state is patched.
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
const meleeConfig = `/home/web_user/${edition === 'hd' ? '.uqm-hd' : '.uqm'}/melee.cfg`;
const roles = ['host', 'guest'];
await mkdir(output, { recursive: true });
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const report = { edition, distDir, peerDist, browsers: [], sessions: [], passed: false };

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
  return page.evaluate(() => ({ ...Module.webNet.stats, connection: Module.webNet.state,
    screen: Module.gameScreen }));
}
async function waitState(pages, state, timeout = 45000) {
  await Promise.all(pages.map(page => page.waitForFunction(
    state => Module.webNet && Module.webNet.stats.engineState === state, state, { polling: 100, timeout })));
}
async function key(pages, name, delay = 150) {
  await Promise.all(pages.map(page => page.keyboard.press(name, { delay })));
}
async function enterUntil(pages, state) {
  const deadline = Date.now() + 30000;
  while (Date.now() < deadline) {
    const states = await Promise.all(pages.map(readState));
    if (states.every(s => s.engineState === state)) return;
    assert.ok(states.every(s => s.connection === 'ready'), JSON.stringify(states));
    await key(pages, 'Enter');
    await sleep(600);
  }
  throw new Error(`Did not reach ${state}: ${JSON.stringify(await Promise.all(pages.map(readState)))}`);
}
async function checkSynchronization(pages, minimum) {
  await Promise.all(pages.map(page => page.waitForFunction(
    minimum => (Module.webNet.stats.checksumsVerified || 0) >= minimum,
    minimum, { polling: 100, timeout: 30000 })));
  const states = await Promise.all(pages.map(readState));
  for (const state of states) {
    assert.equal(state.checksumMismatches || 0, 0, JSON.stringify(state));
    assert.equal(state.connection, 'ready');
    assert.ok(state.receivedFrames > 0 && state.sentFrames > 0);
  }
  return states;
}
async function canvasShot(page, name) {
  await page.locator('#canvas').screenshot({ path: path.join(output, name) });
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

// The engine polls the hook every main-menu frame. Until a role is set it
// returns -1; a set role is returned once. Module.holdMainMenu keeps the main
// menu from timing out into the intro, as a page does while its lobby is open.
function installHook() {
  Module.holdMainMenu = true;
  window.uqmTestPendingRole = -1;
  window.uqmTestRolesTaken = 0;
  Module.takePendingNetplayRole = () => {
    const role = window.uqmTestPendingRole;
    window.uqmTestPendingRole = -1;
    if (role !== -1) window.uqmTestRolesTaken++;
    return role;
  };
}

// openRoom() is the launcher's own room function. It ignores calls after the
// launch in the current page, so the launch flag is lowered for the call only.
function openTestRoom({ role, token }) {
  const wasStarted = started;
  started = false;
  try { openRoom(role, token); } finally { started = wasStarted; }
  return Module.webNet.state;
}

// The relay knows a room only after the host's connection; a guest that is
// too early receives an error and joins again.
async function joinTestRoom(page, token) {
  for (let attempt = 0; attempt < 20; attempt++) {
    await page.evaluate(openTestRoom, { role: 'guest', token });
    const handle = await page.waitForFunction(() => Module.webNet.state !== 'waiting' && Module.webNet.state,
      null, { polling: 100, timeout: 20000 });
    const state = await handle.jsonValue();
    if (state === 'ready') return attempt;
    await sleep(500);
  }
  throw new Error('The guest could not join the room');
}

async function backToMainMenu(page, label) {
  // The engine shows why the room closed; one key press acknowledges it.
  for (let attempt = 0; attempt < 3; attempt++) {
    if (await page.evaluate(() => Module.gameScreen === 'mainMenu')) return;
    await page.keyboard.press('Enter', { delay: 150 });
    try {
      await page.waitForFunction(() => Module.gameScreen === 'mainMenu', null, { polling: 100, timeout: 10000 });
      return;
    } catch (error) { if (attempt === 2) throw new Error(`${label} did not return to the main menu`); }
  }
}

async function playSession(pages, number) {
  const entry = { number, passed: false };
  report.sessions.push(entry);
  const token = randomBytes(16).toString('hex');
  entry.token = token;
  for (const page of pages) {
    assert.equal(await page.evaluate(() => Module.gameScreen), 'mainMenu');
    assert.equal(await page.evaluate(() => Module.supportsNetplayHook), true);
  }
  // Nothing starts without a role, even with an open room.
  entry.hostState = await pages[0].evaluate(openTestRoom, { role: 'host', token });
  await sleep(500);
  entry.guestJoinRetries = await joinTestRoom(pages[1], token);
  await pages[0].waitForFunction(() => Module.webNet.state === 'ready', null, { polling: 100, timeout: 20000 });
  if (number === 1) {
    // An open room alone keeps the main menu up: the menu's inactivity
    // timeout (20 s without menu music) must not start the intro.
    await Promise.all(pages.map(page => page.evaluate(() => { Module.holdMainMenu = false; })));
    await sleep(25000);
    for (const page of pages) assert.equal(await page.evaluate(() => Module.gameScreen), 'mainMenu');
    await Promise.all(pages.map(page => page.evaluate(() => { Module.holdMainMenu = true; })));
    entry.roomHeldMainMenu = true;
  }
  for (const page of pages) {
    assert.equal(await page.evaluate(() => Module.gameScreen), 'mainMenu');
    assert.equal(await page.evaluate(() => Module.webNet.stats.engineState), undefined);
  }
  await Promise.all(pages.map((page, i) => page.evaluate(role => { window.uqmTestPendingRole = role; }, i)));
  await Promise.all(pages.map(page => page.waitForFunction(number => window.uqmTestRolesTaken === number
    && window.uqmTestPendingRole === -1, number, { polling: 100, timeout: 10000 })));
  await waitState(pages, 'inSetup');
  for (const page of pages) assert.equal(await page.evaluate(() => Module.gameScreen), 'melee');
  console.log(`${edition} session ${number}: both players in fleet setup from the main menu`);
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
  for (let ship = 0; ship < 2; ship++) {
    const before = await Promise.all(pages.map(readState));
    await checkSynchronization(pages, Math.max(...before.map(s => s.checksumsVerified || 0)) + 60);
    for (let n = 0; n < 3; n++) {
      await key([pages[0]], 'ShiftRight', 300);
      await sleep(300);
    }
    if (ship === 0) {
      await waitState(pages, 'selectShip');
      console.log(`${edition} session ${number}: first ship destroyed; choosing next ship`);
      await enterUntil(pages, 'inBattle');
    }
  }
  // The last ship's destruction shows the victory screen; acknowledging it
  // returns both players to the fleet setup in the same room.
  await sleep(5000);
  await enterUntil(pages, 'inSetup');
  entry.statsAfterMatch = await checkSynchronization(pages, 130);
  entry.completedFleetMatch = true;
  await canvasShot(pages[0], `session${number}-match-complete.png`);
  console.log(`${edition} session ${number}: full fleet match completed without checksum mismatch`);

  // The guest leaves the room in the page, as a lobby's Leave button does.
  // Both engines show why the room closed and return to their main menus.
  await pages[1].evaluate(() => Module.webNet.close());
  await pages[0].waitForFunction(() => Module.webNet.state === 'closed', null, { polling: 100, timeout: 20000 });
  await sleep(1500);
  await Promise.all(pages.map((page, i) => canvasShot(page, `session${number}-closed-${roles[i]}.png`)));
  await Promise.all(pages.map((page, i) => backToMainMenu(page, roles[i])));
  await Promise.all(pages.map(page => page.waitForFunction(() => Module.supportsNetplayHook === true,
    null, { polling: 100, timeout: 10000 })));
  await sleep(1500);
  await Promise.all(pages.map((page, i) => canvasShot(page, `session${number}-main-menu-${roles[i]}.png`)));
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
    null, { polling: 100, timeout: 240000 })));
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
  await Promise.all(pages.map(page => page.locator('#start').click()));
  await Promise.all(pages.map(page => page.locator('#canvas').focus()));
  await Promise.all(pages.map(page => page.waitForFunction(
    () => Module.gameScreen === 'mainMenu' && Module.supportsNetplayHook === true,
    null, { polling: 100, timeout: 180000 })));
  await sleep(1000);
  await Promise.all(pages.map((page, i) => canvasShot(page, `main-menu-${roles[i]}.png`)));
  console.log(`${edition}: both engines in the main menu with the netplay hook`);
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
