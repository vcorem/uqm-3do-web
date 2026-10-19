// End-to-end tests use the real packaged game, two independent browsers, real
// keyboard input, and the engine's checksums. No battle state is patched.
import assert from 'node:assert/strict';
import { mkdir, writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { chromium, firefox, webkit } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';

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
// UQM_ORIGIN plays on a deployed game page (for example
// https://urquanmasters.org/original/) through that site's own relay.
// No local relay starts, so server-side delay scenarios are skipped.
const remoteOrigin = process.env.UQM_ORIGIN || null;
const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/browser');
await mkdir(output, { recursive: true });
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const results = [];
if (!remoteOrigin) {
  const buildHashes = Object.fromEntries(await Promise.all(
    ['index.html', 'uqm-debug.wasm', 'uqm-debug.js', 'uqm-debug.worker.js', 'uqm-debug.data', 'netplay-client.js'].map(async file =>
      [file, createHash('sha256').update(await readFile(path.join(distDir, file))).digest('hex')])));
  await writeFile(path.join(output, 'build-hashes.json'), JSON.stringify(buildHashes, null, 2) + '\n');
}

async function readState(page) {
  return page.evaluate(() => ({ ...Module.webNet.stats, connection: Module.webNet.state }));
}
async function waitState(pages, state, timeout = 45000) {
  await Promise.all(pages.map(page => page.waitForFunction(
    state => Module.webNet.stats.engineState === state, state, { polling: 100, timeout })));
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
function seedFleet() {
  const bytes = new Uint8Array(140);
  for (let side = 0; side < 2; side++) {
    const offset = side * 70;
    bytes[offset] = 17; // HUMAN_CONTROL | STANDARD_RATING
    bytes.fill(255, offset + 1, offset + 15);
    bytes[offset + 1] = bytes[offset + 2] = side === 1 ? 13 : 5;
    bytes.set(new TextEncoder().encode(side ? 'Test host' : 'Test guest'), offset + 15);
  }
  FS.writeFile('/home/web_user/.uqm/melee.cfg', bytes);
  return Array.from(bytes);
}

async function scenario(name, latencyPatternMs, disconnectAt) {
  const app = remoteOrigin ? null : createNetplayServer({ distDir, latencyPatternMs });
  const origin = remoteOrigin ?? `http://127.0.0.1:${(await app.listen(0)).port}`;
  const browsers = [];
  const pages = [];
  const failures = [];
  const entry = {
    name, latencyPatternMs, passed: false,
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
      const page = await context.newPage();
      page.setDefaultTimeout(30000);
      page.on('pageerror', error => failures.push(`${i}: ${error.message}`));
      page.on('crash', () => failures.push(`${i}: page crashed`));
      pages.push(page);
    }
    await Promise.all(pages.map(page => page.goto(origin)));
    await Promise.all(pages.map(page => page.waitForFunction(() => runtimeReady,
      null, { polling: 100, timeout: 90000 })));
    const expectedFleet = await pages[0].evaluate(seedFleet);
    await pages[1].evaluate(seedFleet);
    // Persist the baseline before launching so reload can detect accidental
    // replacement of local fleet/control settings by the online session.
    await Promise.all(pages.map(page => page.evaluate(() => new Promise((resolve, reject) =>
      FS.syncfs(false, error => error ? reject(error) : resolve())))));
    await pages[0].locator('#create-room').click();
    await pages[0].waitForFunction(() => document.getElementById('online-status').textContent === 'Waiting for your opponent to join…', null, { polling: 100 });
    const invite = await pages[0].locator('#room-invite').inputValue();
    await pages[1].locator('#join-invite').fill(invite);
    await pages[1].locator('#join-room').click();
    await Promise.all(pages.map(page => page.locator('#launch-online').click()));
    await waitState(pages, 'inSetup');
    console.log(`${name}: both players in fleet setup`);
    await Promise.all(pages.map(page => page.locator('#canvas').click()));
    if (disconnectAt !== 'setup') {
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
    // exercises relay departure and the C wait/cleanup paths together.
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
    assert.deepEqual(await pages[0].evaluate(() => Array.from(FS.readFile('/home/web_user/.uqm/melee.cfg'))), expectedFleet);
    const logs = await pages[0].locator('#log').textContent();
    assert.doesNotMatch(logs, /Aborted\(|Assertion failed|memory access out of bounds|Uncaught RuntimeError/);
    assert.deepEqual(failures, []);
    await pages[0].reload();
    await pages[0].waitForFunction(() => runtimeReady, null, { polling: 100, timeout: 90000 });
    assert.deepEqual(await pages[0].evaluate(() => Array.from(FS.readFile('/home/web_user/.uqm/melee.cfg'))), expectedFleet);
    entry.localFleetPreservedAfterReload = true;
    entry.passed = true;
    console.log(`${name}: ${disconnectAt} disconnect and persisted local settings passed`);
  } catch (error) {
    entry.error = error.stack || String(error);
    for (let i = 0; i < pages.length; i++) {
      if (!pages[i].isClosed()) {
        await pages[i].screenshot({ path: path.join(output, `${name}-failure-${i}.png`) }).catch(() => {});
        await writeFile(path.join(output, `${name}-failure-${i}.log`),
          await pages[i].locator('#log').textContent().catch(() => 'page unavailable'));
        entry[`state${i}`] = await readState(pages[i]).catch(() => null);
      }
    }
    throw error;
  } finally {
    await Promise.allSettled(browsers.map(browser => browser.close()));
    await app?.close();
    await writeFile(path.join(output, 'results.json'), JSON.stringify(results, null, 2) + '\n');
  }
}

await scenario('normal', [0], 'selection');
if (!remoteOrigin) await scenario('jitter', [80, 120, 30, 60], 'battle');
await scenario('setup', [0], 'setup');
console.log(`All real-browser multiplayer scenarios passed. See ${path.join(output, 'results.json')}.`);
