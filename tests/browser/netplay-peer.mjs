// One player of a real Online Melee match on a deployed site, for play
// between two different computers and networks (for example this Mac and a
// GitHub runner). Each side runs this script with the same room token; the
// host makes the room and the guest retries the join until the room exists.
// The moves and checks are those of tests/browser/netplay.mjs.
import assert from 'node:assert/strict';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium } from 'playwright';

const origin = process.env.UQM_ORIGIN || 'https://urquanmasters.org';
const edition = process.env.UQM_EDITION || 'original';
const role = process.env.UQM_ROLE;
const token = process.env.UQM_ROOM;
const joinTimeout = Number(process.env.UQM_JOIN_TIMEOUT_MS || 15 * 60 * 1000);
const output = path.resolve(process.env.UQM_TEST_OUTPUT || `test-results/peer-${edition}-${role}`);
if (!['original', 'hd'].includes(edition)) throw new Error('UQM_EDITION must be original or hd');
if (!['host', 'guest'].includes(role)) throw new Error('UQM_ROLE must be host or guest');
if (!/^[0-9a-f]{32}$/.test(token || '')) throw new Error('UQM_ROOM must be a 32-character hex room token');
const meleeConfig = (edition === 'hd' ? '/home/web_user/.uqm-hd' : '/home/web_user/.uqm') + '/melee.cfg';
await mkdir(output, { recursive: true });
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const log = message => console.log(`[${new Date().toISOString()}] ${edition} ${role}: ${message}`);

function seedFleet(file) {
  const bytes = new Uint8Array(140);
  for (let side = 0; side < 2; side++) {
    const offset = side * 70;
    bytes[offset] = 17; // HUMAN_CONTROL | STANDARD_RATING
    bytes.fill(255, offset + 1, offset + 15);
    bytes[offset + 1] = bytes[offset + 2] = side === 1 ? 13 : 5;
    bytes.set(new TextEncoder().encode(side ? 'Test host' : 'Test guest'), offset + 15);
  }
  FS.writeFile(file, bytes);
}

const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
const page = await (await browser.newContext({ viewport: { width: 1100, height: 1100 } })).newPage();
page.setDefaultTimeout(30000);
const failures = [];
page.on('pageerror', error => failures.push(error.message));
page.on('crash', () => failures.push('page crashed'));
const result = { edition, role, origin, passed: false, started: new Date().toISOString() };
const readState = () => page.evaluate(() => ({ ...Module.webNet.stats, connection: Module.webNet.state }));
const waitState = (state, timeout = 120000) => page.waitForFunction(
  state => Module.webNet.stats.engineState === state, state, { polling: 100, timeout });
async function enterUntil(state) {
  const deadline = Date.now() + 120000;
  while (Date.now() < deadline) {
    const current = await readState();
    if (current.engineState === state) return;
    assert.equal(current.connection, 'ready', JSON.stringify(current));
    await page.keyboard.press('Enter', { delay: 150 });
    await sleep(600);
  }
  throw new Error(`Did not reach ${state}: ${JSON.stringify(await readState())}`);
}
async function checkSynchronization(minimum) {
  await page.waitForFunction(minimum => (Module.webNet.stats.checksumsVerified || 0) >= minimum,
    minimum, { polling: 100, timeout: 120000 });
  const state = await readState();
  assert.equal(state.checksumMismatches || 0, 0, JSON.stringify(state));
  assert.equal(state.connection, 'ready');
  assert.ok(state.receivedFrames > 0 && state.sentFrames > 0);
  return state;
}

try {
  await page.goto(origin + (edition === 'hd' ? '/hd/index.html' : '/original/index.html'));
  await page.waitForFunction(() => runtimeReady, null, { polling: 250, timeout: 300000 });
  log('game loaded');
  await page.evaluate(seedFleet, meleeConfig);
  await page.evaluate(() => new Promise((resolve, reject) => FS.syncfs(false, error => error ? reject(error) : resolve())));
  // openRoom is the page's own room function behind Create room / Join room.
  const deadline = Date.now() + joinTimeout;
  for (let attempt = 1; ; attempt++) {
    await page.evaluate(([role, token]) => openRoom(role, token), [role, token]);
    // The client shows 'waiting' when its socket opens, before the relay answers. A host is
    // in its room after the relay's 'waiting'; a guest is in only when the room is 'ready'.
    const goal = role === 'host' ? 'waiting' : 'ready';
    const state = await page.waitForFunction(goal => {
      const net = Module.webNet;
      const state = net && net.state;
      if (state === 'error' || state === 'closed' || state === 'ready') return state;
      return goal === 'waiting' && state === 'waiting' &&
        document.getElementById('online-status').textContent === 'Waiting for your opponent to join…' ? state : false;
    }, goal, { polling: 100, timeout: 30000 }).then(handle => handle.jsonValue()).catch(() => 'timeout');
    if (state === goal || state === 'ready') { log(`in room (${state}) after ${attempt} attempt(s)`); break; }
    if (role === 'host' || Date.now() > deadline) throw new Error(`Could not enter the room: ${state}`);
    await sleep(2000);
  }
  await page.waitForFunction(() => Module.webNet.state === 'ready', null, { polling: 100, timeout: 300000 });
  log('opponent connected');
  await page.locator('#launch-online').click();
  await waitState('inSetup');
  log('fleet setup');
  await page.locator('#canvas').click();
  await enterUntil('selectShip');
  await enterUntil('inBattle');
  await checkSynchronization(60);
  log('in battle, checksums verified');
  const held = role === 'host' ? ['ArrowUp'] : ['ArrowUp', 'ControlRight', 'ArrowLeft'];
  for (const key of held) await page.keyboard.down(key);
  await sleep(3000);
  for (const key of held) await page.keyboard.up(key);
  for (let ship = 0; ship < 2; ship++) {
    await checkSynchronization((await readState()).checksumsVerified + 60);
    // The host flies Shofixti; its Glory Device ends each ship without waiting for an AI outcome.
    if (role === 'host') for (let n = 0; n < 3; n++) { await page.keyboard.press('ShiftRight', { delay: 300 }); await sleep(300); }
    if (ship === 0) {
      await waitState('selectShip');
      log('first ship destroyed');
      await enterUntil('inBattle');
    }
  }
  await sleep(5000);
  await enterUntil('inSetup');
  result.statsAfterMatch = await checkSynchronization(130);
  await page.screenshot({ path: path.join(output, 'match-complete.png') });
  log(`full fleet match completed: ${JSON.stringify(result.statsAfterMatch)}`);
  // The host leaves; the guest must see the room close.
  if (role === 'host') {
    await sleep(3000);
    await page.evaluate(() => Module.webNet.close());
  } else {
    await page.waitForFunction(() => Module.webNet.state === 'closed', null, { polling: 100, timeout: 60000 });
    log('host left and the room closed');
  }
  const logs = await page.locator('#log').textContent();
  assert.doesNotMatch(logs, /Aborted\(|Assertion failed|memory access out of bounds|Uncaught RuntimeError/);
  assert.deepEqual(failures, []);
  result.passed = true;
} catch (error) {
  result.error = error.stack || String(error);
  result.state = await readState().catch(() => null);
  await page.screenshot({ path: path.join(output, 'failure.png') }).catch(() => {});
  await writeFile(path.join(output, 'failure.log'), await page.locator('#log').textContent().catch(() => ''));
  process.exitCode = 1;
} finally {
  result.finished = new Date().toISOString();
  await writeFile(path.join(output, 'result.json'), JSON.stringify(result, null, 2) + '\n');
  console.log('PEER_RESULT ' + JSON.stringify(result));
  await browser.close();
}
