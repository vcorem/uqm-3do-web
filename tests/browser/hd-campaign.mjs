// Real HD engine and normal save: keyboard Load Game, Sol flight, Hyperspace.
// Only the fixture is seeded; no simulation state or rendering code is patched.
import assert from 'node:assert/strict';
import { createReadStream } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { chromium } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';
import { measureSceneImage, hasAmbientStars } from './scene-pixels.mjs';

const dist = path.resolve(process.env.UQM_DIST || '.tools/hd-dist');
const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/hd-campaign');
const fixture = await readFile(new URL('./fixtures/hd-new-game.uqm', import.meta.url));
const savePath = '/home/web_user/.uqm-hd/save/uqmsave.00';
const report = { passed: false, errors: [], console: [], steps: [], buildHashes: {},
  fixture: { bytes: fixture.length, sha256: createHash('sha256').update(fixture).digest('hex') } };
const fatalPattern = /Aborted\(|Assertion failed|memory access out of bounds|function signature mismatch|indirect call signature mismatch|Uncaught RuntimeError/;
let browser, page, server, rejectFatal;
const held = new Set();
const fatal = new Promise((_, reject) => { rejectFatal = reject; });
fatal.catch(() => {});
const guarded = operation => Promise.race([operation, fatal]);
const sleep = ms => guarded(new Promise(resolve => setTimeout(resolve, ms)));
function mark(step) {
  report.steps.push({ step, time: new Date().toISOString() });
  console.log(step);
}
async function checkHealthy() {
  const state = await guarded(page.evaluate(() => ({
    failed: document.getElementById('start')?.textContent === 'Unable to start',
    exitStatus: Module.exitStatus, status: document.getElementById('status')?.textContent,
    log: document.getElementById('log')?.textContent || '',
  })));
  assert.ok(!state.failed && state.exitStatus === undefined, JSON.stringify(state));
  assert.doesNotMatch(state.log, fatalPattern);
  assert.deepEqual(report.errors, []);
  return state.log;
}
async function waitFor(target, timeout) {
  await guarded(page.waitForFunction(target => {
    const module = typeof Module === 'object' ? Module : {};
    return document.getElementById('start')?.textContent === 'Unable to start' ||
      module.exitStatus !== undefined || (target === 'runtime'
        ? typeof runtimeReady !== 'undefined' && runtimeReady : module.gameScreen === target);
  }, target, { timeout }));
  await checkHealthy();
}
async function capture(name) {
  await checkHealthy();
  const png = await guarded(page.locator('#canvas').screenshot({ path: path.join(output, name + '.png') }));
  return guarded(page.evaluate(measureSceneImage, png.toString('base64')));
}
async function hold(key, ms) {
  await guarded(page.keyboard.down(key)); held.add(key);
  try { await sleep(ms); } finally { await page.keyboard.up(key); held.delete(key); }
}

await mkdir(output, { recursive: true });
try {
  assert.equal(fixture.length, 3628, 'Expected the documented untouched HD new-game fixture');
  for (const file of ['index.html', 'uqm-debug.js', 'uqm-debug.worker.js', 'uqm-debug.wasm', 'uqm-debug.data']) {
    const hash = createHash('sha256');
    for await (const chunk of createReadStream(path.join(dist, file))) hash.update(chunk);
    report.buildHashes[file] = hash.digest('hex');
  }
  server = createNetplayServer({ distDir: dist });
  const address = await server.listen(0, '127.0.0.1');
  browser = await chromium.launch({ headless: process.env.HEADED !== '1',
    ...(process.env.CHROME_BIN ? { executablePath: process.env.CHROME_BIN } : {}),
    args: ['--autoplay-policy=no-user-gesture-required', '--disable-renderer-backgrounding',
      '--disable-backgrounding-occluded-windows', '--disable-background-timer-throttling'] });
  report.browser = browser.version();
  page = await browser.newPage({ viewport: { width: 1440, height: 1200 } });
  page.on('pageerror', error => { report.errors.push(error.stack || error.message); rejectFatal(error); });
  page.on('crash', () => { const error = new Error('Browser page crashed'); report.errors.push(error.message); rejectFatal(error); });
  page.on('console', message => {
    if (['warning', 'error'].includes(message.type()) && report.console.length < 100)
      report.console.push({ type: message.type(), text: message.text(), location: message.location() });
    if (fatalPattern.test(message.text())) rejectFatal(new Error(message.text()));
  });
  await guarded(page.goto(`http://127.0.0.1:${address.port}`));
  await waitFor('runtime', 240000);
  report.config = await guarded(page.evaluate(({ bytes, savePath }) => {
    FS.mkdirTree('/home/web_user/.uqm-hd/save');
    FS.writeFile(savePath, new Uint8Array(bytes));
    // Existing read-open diagnostics prove the game accesses the seeded file.
    Module.logReadFiles = true;
    return { uqm: FS.readFile('/home/web_user/.uqm-hd/uqm.cfg', { encoding: 'utf8' }),
      megamod: FS.readFile('/home/web_user/.uqm-hd/megamod.cfg', { encoding: 'utf8' }) };
  }, { bytes: Array.from(fixture), savePath }));
  assert.match(report.config.uqm, /^resolutionfactor = INT32:2$/m);
  assert.match(report.config.megamod, /^difficulty = INT32:0$/m);
  assert.match(report.config.megamod, /^extended = BOOLEAN:false$/m);
  await guarded(page.getByRole('button', { name: 'Launch HD game', exact: true }).click());
  await waitFor('mainMenu', 90000);
  report.canvas = await guarded(page.evaluate(() => ({ width: canvas.width, height: canvas.height })));
  assert.deepEqual(report.canvas, { width: 1280, height: 960 });
  await guarded(page.locator('#canvas').focus());
  await guarded(page.keyboard.press('ArrowDown', { delay: 250 })); await sleep(750);
  await guarded(page.keyboard.press('Enter', { delay: 250 })); await sleep(5000);
  report.loadSlots = await capture('load-slots');
  report.fixtureOpened = await guarded(page.evaluate(savePath => Object.hasOwn(FS.readFiles || {}, savePath), savePath));
  assert.ok(report.fixtureOpened, 'Game did not read the actual save through Load Game');
  mark('Real Load Game opened the HD fixture');
  await guarded(page.keyboard.press('Enter', { delay: 250 })); await sleep(4000);
  report.sol = await capture('sol-loaded');
  assert.ok(report.sol.litFraction > 0.01 && report.sol.colors > 24, 'Loaded Sol is blank');
  const loadedLog = await checkHealthy();
  assert.match(loadedLog, /Loading -> Slot: 0[\s\S]*Name: HDtest[\s\S]*Difficulty: Normal/,
    'Engine did not parse the documented fixture as a normal campaign');
  assert.doesNotMatch(loadedLog, /hyperspace\.ani/, 'Hyperspace was already loaded before flight');
  await hold('ArrowUp', 6000);
  report.flight = await capture('sol-flight');
  mark('Ship thrust north for six seconds, now coasting out of Sol');
  const deadline = Date.now() + 180000;
  let reached = false;
  while (Date.now() < deadline) {
    if (/hyperspace\.ani/.test(await checkHealthy())) { reached = true; break; }
    await sleep(500);
  }
  assert.ok(reached, 'Normal keyboard flight never reached Hyperspace');
  await sleep(5000);
  report.hyperspace = await capture('hyperspace');
  assert.ok(report.hyperspace.litFraction > 0.01 && report.hyperspace.colors > 24, 'Hyperspace scene is blank');
  assert.notEqual(report.hyperspace.sceneHash, report.sol.sceneHash, 'Scene never changed from Sol');
  assert.ok(hasAmbientStars(report.hyperspace),
    `Hyperspace ambient stars are missing outside the ship/HUD: ${JSON.stringify(report.hyperspace.ambientStars)}`);
  mark('Hyperspace transition rendered without a Wasm trap');
  await hold('ArrowUp', 5000);
  report.hyperspaceMoving = await capture('hyperspace-moving');
  assert.notEqual(report.hyperspaceMoving.sceneHash, report.hyperspace.sceneHash, 'Hyperspace scene did not advance while flying');
  assert.ok(hasAmbientStars(report.hyperspaceMoving),
    `Flying Hyperspace lost its ambient stars: ${JSON.stringify(report.hyperspaceMoving.ambientStars)}`);
  await checkHealthy();
  report.passed = true;
  mark('HD campaign load, Sol exit and continued Hyperspace flight passed');
} catch (error) {
  report.failure = error.stack || String(error);
  if (page && !page.isClosed()) await page.screenshot({ path: path.join(output, 'failure.png'), fullPage: true }).catch(() => {});
  throw error;
} finally {
  if (page && !page.isClosed()) {
    for (const key of held) await page.keyboard.up(key).catch(() => {});
    report.log = await page.locator('#log').textContent().catch(() => 'page unavailable');
  }
  await writeFile(path.join(output, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  await browser?.close();
  await server?.close();
}
