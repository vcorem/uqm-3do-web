// End-to-end tests use the real packaged game, two independent browsers, real
// keyboard input, and the engine's checksums. No battle state is patched.
import assert from 'node:assert/strict';
import { mkdir, writeFile, readFile, rm } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { chromium, firefox, webkit } from 'playwright';
import { createGameBackend } from '../helpers/game-backend.mjs';
import { createLobbyStub } from '../helpers/online-lobby.mjs';
import { shimPath } from './online-harness.mjs';

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
const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/browser');
// The HD engine has independent settings, but uses the same saved-fleet format
// and ship indexes. Every peer in a test plays the selected edition.
const edition = process.env.UQM_EDITION || 'original';
assert.ok(['original', 'hd'].includes(edition), 'UQM_EDITION must be original or hd');
const meleeConfig = `/home/web_user/${edition === 'hd' ? '.uqm-hd' : '.uqm'}/melee.cfg`;
await mkdir(output, { recursive: true });
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
async function boundedDiagnostic(operation, label) {
  let timer;
  try {
    return await Promise.race([operation, new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(`${label} timed out`)), 2000);
    })]);
  } finally { clearTimeout(timer); }
}
const results = [];
const buildHashes = Object.fromEntries(await Promise.all(
  ['index.html', 'uqm-debug.wasm', 'uqm-debug.js', 'uqm-debug.worker.js', 'uqm-debug.data', 'netplay-client.js'].map(async file =>
    [file, createHash('sha256').update(await readFile(path.join(distDir, file))).digest('hex')])));
await writeFile(path.join(output, 'build-hashes.json'), JSON.stringify(buildHashes, null, 2) + '\n');
let peerBuildHashes;
if (peerDist) {
  peerBuildHashes = Object.fromEntries(await Promise.all(Object.keys(buildHashes).map(async file =>
    [file, createHash('sha256').update(await readFile(path.join(peerDist, file))).digest('hex')])));
  assert.notEqual(peerBuildHashes['uqm-debug.wasm'], buildHashes['uqm-debug.wasm'],
    'Differential testing requires independent C and Rust engine builds');
  await writeFile(path.join(output, 'peer-build-hashes.json'), JSON.stringify(peerBuildHashes, null, 2) + '\n');
}

async function readState(page) {
  return page.evaluate(() => ({ ...Module.webNet.stats, connection: Module.webNet.state }));
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

// A normal saved Melee configuration: two Earthlings at top, two Shofixti
// at bottom. Shofixti's original special lets us finish a fleet deterministically
// using keyboard input, without cheating or waiting for an AI combat outcome.
function seedFleet({ file, guestShip }) {
  const bytes = new Uint8Array(140);
  for (let side = 0; side < 2; side++) {
    const offset = side * 70;
    bytes[offset] = 17; // HUMAN_CONTROL | STANDARD_RATING
    bytes.fill(255, offset + 1, offset + 15);
    bytes[offset + 1] = bytes[offset + 2] = side === 1 ? 13 : guestShip;
    bytes.set(new TextEncoder().encode(side ? 'Test host' : 'Test guest'), offset + 15);
  }
  FS.writeFile(file, bytes);
  return Array.from(bytes);
}

async function scenario(name, latencyPatternMs, disconnectAt, smokeShip = null) {
  // Players meet in the lobby (a test lobby in front of the relay), as on the site.
  const app = createLobbyStub(createGameBackend({ distDir, latencyPatternMs }, peerDist));
  const address = await app.listen(0);
  const origin = `http://127.0.0.1:${address.port}`;
  const browsers = [];
  const pages = [];
  const fetchedWasm = [[], []];
  const roomSockets = [[], []];
  const failures = [];
  const entry = {
    name, edition, latencyPatternMs, passed: false, relay: process.env.UQM_RELAY || 'node',
    distDir, peerDist,
    browsers: selectedEngines.map((engine, index) => ({
      role: index === 0 ? 'host' : 'guest', engine, version: null,
    })),
  };
  results.push(entry);
  try {
    for (let i = 0; i < 2; i++) {
      // Separate browser processes also model the independent event loops of
      // two computers; neither test client's page is a background tab.
      const engine = selectedEngines[i];
      const browser = await browserEngines[engine].launch(browserLaunchOptions(engine));
      browsers.push(browser);
      entry.browsers[i].version = browser.version();
      const context = await browser.newContext({ viewport: { width: 1100, height: 1100 } });
      await context.addInitScript({ path: shimPath });
      // HD's large media response can evict Wasm from the browser inspector's
      // response cache. Hash the exact server response before forwarding those
      // unchanged bytes to the browser; this also works outside Chromium.
      await context.route('**/uqm-debug.wasm', async route => {
        const verified = (async () => {
          const response = await route.fetch();
          const bytes = await response.body();
          const hash = createHash('sha256').update(bytes).digest('hex');
          await route.fulfill({ response, body: bytes });
          return hash;
        })().catch(error => {
          failures.push(`${i}: Wasm response could not be verified: ${error.message}`);
          return null;
        });
        fetchedWasm[i].push(verified);
        await verified;
      });
      if (i === 1 && peerDist) {
        await context.addCookies([{ name: 'uqm_test_peer', value: '1', url: origin }]);
      }
      const page = await context.newPage();
      page.setDefaultTimeout(30000);
      page.on('websocket', socket => { if (!/\/netplay\/lobby/.test(socket.url())) roomSockets[i].push(socket.url()); });
      page.on('pageerror', error => failures.push(`${i}: ${error.stack || error.message}`));
      page.on('console', message => {
        if (['error', 'warning'].includes(message.type())) {
          entry.browserConsole ||= [];
          if (entry.browserConsole.length < 100) entry.browserConsole.push({ player: i,
            type: message.type(), text: message.text() });
        }
      });
      page.on('crash', () => failures.push(`${i}: page crashed`));
      pages.push(page);
    }
    await Promise.all(pages.map(page => page.goto(origin)));
    await Promise.all(pages.map(page => page.waitForFunction(() => runtimeReady,
      null, { polling: 100, timeout: 90000 })));
    entry.fetchedWasmHashes = await Promise.all(fetchedWasm.map(responses => Promise.all(responses)));
    for (let i = 0; i < 2; i++) {
      assert.ok(entry.fetchedWasmHashes[i].length > 0, `Player ${i} did not fetch the expected engine`);
      const expected = i === 1 && peerDist ? peerBuildHashes : buildHashes;
      for (const hash of entry.fetchedWasmHashes[i]) assert.equal(hash, expected['uqm-debug.wasm']);
    }
    const fleet = { file: meleeConfig, guestShip: smokeShip ?? 5 };
    const expectedFleet = await pages[0].evaluate(seedFleet, fleet);
    await pages[1].evaluate(seedFleet, fleet);
    // Persist the baseline before launching so reload can detect accidental
    // replacement of local fleet/control settings by the online session.
    await Promise.all(pages.map(page => page.evaluate(() => new Promise((resolve, reject) =>
      FS.syncfs(false, error => error ? reject(error) : resolve())))));
    // Keyboard only, in the game frame: the host opens a named room from the lobby,
    // the guest picks it from the live list, and the match starts by itself.
    const press = async (page, ...keys) => { for (const name of keys) { await page.keyboard.press(name); await sleep(80); } };
    const screen = (page, name, timeout = 30000) => page.waitForFunction(name => online.state.screen === name, name, { polling: 100, timeout });
    await press(pages[0], 'ArrowDown', 'Enter');
    await screen(pages[0], 'lobby');
    await pages[0].waitForFunction(() => online.state.focus === 'create', null, { polling: 100 });
    await press(pages[0], 'Enter');
    await screen(pages[0], 'name');
    await pages[0].keyboard.press(process.platform === 'darwin' ? 'Meta+A' : 'Control+A');
    await pages[0].keyboard.type(`Test ${name}`);
    await press(pages[0], 'Enter');
    await pages[0].waitForFunction(() => online.state.room && /Waiting for your opponent/.test(online.state.room.message), null, { polling: 100 });
    await press(pages[1], 'ArrowDown', 'Enter');
    await pages[1].waitForFunction(name => online.state.screen === 'lobby' && online.state.lobby.rooms.some(room => room.name === name) &&
      online.state.focus === 'room:' + online.state.lobby.rooms.find(room => room.name === name).id, `Test ${name}`, { polling: 100 });
    entry.lobbyRoom = await pages[1].evaluate(() => online.state.lobby.rooms[0]);
    // The page knows its engine from the packager's UQM_VARIANT; an unpackaged build says C.
    assert.ok(['c', 'rust'].includes(entry.lobbyRoom.engine), JSON.stringify(entry.lobbyRoom));
    await press(pages[1], 'Enter');
    await Promise.all(pages.map(page => page.waitForFunction(() => online.state.session, null, { polling: 100, timeout: 90000 })));
    await waitState(pages, 'inSetup');
    entry.roomSocketURLs = roomSockets;
    for (const [i, urls] of roomSockets.entries()) {
      assert.equal(urls.length, 1, `Player ${i} must use one room WebSocket`);
      const url = new URL(urls[0]);
      const expectedPath = edition === 'hd' ? /^\/netplay\/hd\/[a-f0-9]{32}$/ : /^\/netplay\/[a-f0-9]{32}$/;
      assert.match(url.pathname, expectedPath, `Player ${i} connected to the wrong edition's relay path`);
      assert.equal(url.searchParams.get('role'), i === 0 ? 'host' : 'guest');
    }
    console.log(`${name}: both players in fleet setup`);
    await Promise.all(pages.map(page => page.locator('#canvas').click()));
    if (disconnectAt !== 'setup') {
      await enterUntil(pages, 'selectShip');
      await enterUntil(pages, 'inBattle');
      await checkSynchronization(pages, 60);
      if (smokeShip !== null) {
        entry.shipIndex = smokeShip;
        const before = await Promise.all(pages.map(readState));
        await Promise.all([pages[0].keyboard.down('ArrowUp'), pages[1].keyboard.down('ArrowUp'),
          pages[1].keyboard.down('ArrowLeft'), pages[1].keyboard.down('ControlRight')]);
        await sleep(2500);
        await pages[1].keyboard.up('ControlRight');
        await pages[1].keyboard.down('ShiftRight');
        await sleep(2500);
        await Promise.all([pages[0].keyboard.up('ArrowUp'), pages[1].keyboard.up('ArrowUp'),
          pages[1].keyboard.up('ArrowLeft'), pages[1].keyboard.up('ShiftRight')]);
        entry.statsAfterAbilities = await checkSynchronization(pages,
          Math.max(...before.map(state => state.checksumsVerified || 0)) + 90);
        await pages[0].screenshot({ path: path.join(output, `${name}-abilities.png`) });
        assert.deepEqual(failures, []);
        entry.passed = true;
        console.log(`${name}: movement, weapon and special input passed without checksum mismatch`);
        return;
      }
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
          await pages[0].screenshot({ path: path.join(output, `${name}-next-ship.png`) });
          console.log(`${name}: first ship destroyed; choosing next ship`);
          await enterUntil(pages, 'inBattle');
        }
      }
      // The last ship's destruction shows the original victory/defeat screen.
      // Acknowledging it returns both players to the fleet setup in the same room.
      await sleep(5000);
      await enterUntil(pages, 'inSetup');
      entry.completedFleetMatch = true;
      entry.statsAfterMatch = await checkSynchronization(pages, 130);
      await pages[0].screenshot({ path: path.join(output, `${name}-match-complete.png`) });
      console.log(`${name}: full fleet match completed without checksum mismatch`);
      await enterUntil(pages, 'selectShip');
      if (disconnectAt === 'battle') {
        await enterUntil(pages, 'inBattle');
        const beforeDisconnect = await Promise.all(pages.map(readState));
        await checkSynchronization(pages, Math.max(...beforeDisconnect.map(s => s.checksumsVerified || 0)) + 60);
      }
    }
    // Closing a real browser, rather than merely updating a mocked connection,
    // exercises relay departure and the engine's wait/cleanup paths together.
    await browsers[1].close();
    await pages[0].waitForFunction(() => Module.webNet.state === 'closed', null, { polling: 100 });
    await sleep(1500);
    await pages[0].screenshot({ path: path.join(output, `${name}-disconnect.png`) });
    for (let attempt = 0; attempt < 3; attempt++) {
      await key([pages[0]], 'Enter');
      try {
        await pages[0].waitForFunction(() => Module.gameScreen === 'mainMenu', null, { polling: 100, timeout: 10000 });
        break;
      } catch (error) { if (attempt === 2) throw error; }
    }
    entry.returnedToMainMenu = true;
    await pages[0].screenshot({ path: path.join(output, `${name}-returned-menu.png`) });
    assert.deepEqual(await pages[0].evaluate(file => Array.from(FS.readFile(file)), meleeConfig), expectedFleet);
    const logs = await pages[0].locator('#log').textContent();
    assert.doesNotMatch(logs, /Aborted\(|Assertion failed|memory access out of bounds|Uncaught RuntimeError/);
    assert.deepEqual(failures, []);
    await pages[0].reload();
    await pages[0].waitForFunction(() => runtimeReady, null, { polling: 100, timeout: 90000 });
    assert.deepEqual(await pages[0].evaluate(file => Array.from(FS.readFile(file)), meleeConfig), expectedFleet);
    entry.localFleetPreservedAfterReload = true;
    entry.passed = true;
    console.log(`${name}: ${disconnectAt} disconnect and persisted local settings passed`);
  } catch (error) {
    entry.error = error.stack || String(error);
    entry.browserErrors = failures;
    // Preserve the real failure before optional screenshots or a stalled
    // compositor can delay diagnostics and browser shutdown.
    await writeFile(path.join(output, 'results.json'), JSON.stringify(results, null, 2) + '\n');
    entry.diagnosticErrors = [];
    await Promise.all(pages.map(async (page, i) => {
      const screenshot = path.join(output, `${name}-failure-${i}.png`);
      await rm(screenshot, { force: true });
      if (page.isClosed()) return;
      const record = diagnostic => {
        entry.diagnosticErrors.push(`${i}: ${diagnostic.message}`);
        return null;
      };
      const [state, log] = await Promise.all([
        boundedDiagnostic(readState(page), 'Engine state collection').catch(record),
        boundedDiagnostic(page.locator('#log').textContent(), 'Engine log collection').catch(record),
      ]);
      entry[`state${i}`] = state;
      await writeFile(path.join(output, `${name}-failure-${i}.log`), log ?? 'page unavailable');
      await page.screenshot({ path: screenshot, timeout: 5000 }).catch(record);
    }));
    throw error;
  } finally {
    // A failed browser close must not hide the test result already collected.
    await writeFile(path.join(output, 'results.json'), JSON.stringify(results, null, 2) + '\n');
    await Promise.allSettled(browsers.map(browser => browser.close()));
    await app.close();
  }
}

if (process.env.UQM_SHIP_SMOKE) {
  const ships = process.env.UQM_SHIP_SMOKE === 'all'
    ? Array.from({ length: 25 }, (_, index) => index)
    : process.env.UQM_SHIP_SMOKE.split(',').map(Number);
  assert.ok(ships.length && ships.every(ship => Number.isInteger(ship) && ship >= 0 && ship < 25),
    'UQM_SHIP_SMOKE must be all or comma-separated Melee indexes 0..24');
  for (const ship of [...new Set(ships)]) await scenario(`ship-${ship}`, [0], 'battle', ship);
} else {
  await scenario('normal', [0], 'selection');
  await scenario('jitter', [80, 120, 30, 60], 'battle');
  await scenario('setup', [0], 'setup');
}
console.log(`All real-browser multiplayer scenarios passed. See ${path.join(output, 'results.json')}.`);
