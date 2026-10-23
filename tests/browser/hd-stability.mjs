// Real standalone HD runtime: no response rewriting or simulation state patches.
// UQM_REMASTERED=1 adds the new palette and read-only native presentation diagnostics.
// UQM_IDLE_MS defaults to 300000. Set 0 only for a navigation smoke check.
import assert from 'node:assert/strict';
import { createReadStream } from 'node:fs';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium, firefox, webkit } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';
import { recordMeleeScreens, assertMeleeScreens, assertMeleeChoices, launchLocalMelee, leaveLocalFleet, leaveLocalBattle } from './melee-navigation.mjs';
import { useMeleePage } from './melee-page.mjs';

const dist = path.resolve(process.env.UQM_DIST || '.tools/hd-dist');
const out = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/hd-stability');
const browserType = process.env.UQM_BROWSER || 'chromium';
const remastered = process.env.UQM_REMASTERED === '1';
const meleeOnly = process.env.UQM_MELEE_ONLY === '1';
const browsers = { chromium, firefox, webkit };
const idleMs = Number(process.env.UQM_IDLE_MS ?? 300000);
assert.ok(browsers[browserType], 'Unsupported UQM_BROWSER');
assert.ok(Number.isSafeInteger(idleMs) && idleMs >= 0 && idleMs <= 3600000,
  'UQM_IDLE_MS must be an integer from 0 to 3600000');
assert.match(await readFile(path.join(dist, 'index.html'), 'utf8'), /edition: 'hd'/,
  'UQM_DIST must contain the standalone HD build, not the combined edition chooser');
await mkdir(out, { recursive: true });
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const report = { browserType, dist, idleMs, remastered, meleeOnly, startedAt: new Date().toISOString(),
  stage: 'starting', passed: false, errors: [], console: [], battleFrames: [], buildHashes: {} };
const server = createNetplayServer({ distDir: dist });
let browser, page, monitor, monitoring = false;
// The Melee run reloads its page near the end: the health checks wait for the new page.
let reloading = false;
let rejectFatal;
const fatal = new Promise((_, reject) => { rejectFatal = reject; });
fatal.catch(() => {});
function fail(error) {
  const message = error.stack || String(error);
  if (!report.errors.includes(message)) report.errors.push(message);
  rejectFatal(error instanceof Error ? error : new Error(message));
}
const guarded = operation => Promise.race([operation, fatal]);
const deadline = setTimeout(() => fail(new Error('HD stability test exceeded its overall deadline')),
  idleMs + 480000);
const fatalText = /Aborted\(|Assertion failed|Invalid array length|Call args mismatch|UQM_PROXY_MISMATCH|ERROR: AddressSanitizer|runtime error:|stack overflow/i;
async function checkHealth() {
  const state = await guarded(page.evaluate(() => ({
    aborted: typeof ABORT !== 'undefined' && ABORT,
    exitStatus: typeof Module === 'object' ? Module.exitStatus : undefined,
    failed: /^Unable to start/.test(document.getElementById('status')?.textContent || ''),
    status: document.getElementById('status')?.textContent || '',
    log: document.getElementById('log')?.textContent || '',
  })));
  if (state.aborted || state.exitStatus !== undefined || state.failed ||
      state.status.includes('Game closed') || fatalText.test(state.log)) {
    throw new Error('HD runtime failed: ' + JSON.stringify(state));
  }
}
async function waitForScreen(screen, timeout = 120000) {
  await guarded(page.waitForFunction(expected => typeof Module === 'object' &&
    Module.gameScreen === expected, screen, { timeout }));
  await checkHealth();
}
async function key(value) {
  await guarded(page.keyboard.press(value, { delay: 250 }));
  // UQM polls input; release long enough to distinguish adjacent equal presses.
  await guarded(sleep(750));
}
async function readPresentation() {
  if (!remastered) return null;
  return guarded(page.evaluate(() => ({
    enabled: Module._UQM_GetMeleeRemastered(),
    images: Module._UQM_GetMeleeRemasteredImageCount(),
    draws: Module._UQM_GetMeleeRemasteredDrawCount(),
    backgrounds: Module._UQM_GetMeleeRemasteredBackgroundCount(),
    gameScreen: Module.gameScreen,
  })));
}
async function capture() {
  const nativeBefore = await readPresentation();
  const png = await guarded(page.locator('#canvas').screenshot());
  const pixels = await guarded(page.evaluate(async encoded => {
    const bytes = Uint8Array.from(atob(encoded), c => c.charCodeAt(0));
    const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
    const copy = document.createElement('canvas');
    copy.width = bitmap.width; copy.height = bitmap.height;
    const context = copy.getContext('2d');
    context.drawImage(bitmap, 0, 0); bitmap.close();
    const { data, width, height } = context.getImageData(0, 0, copy.width, copy.height);
    let title = 0, titleLight = 0, titleCount = 0, gray = 0, navy = 0, panelCount = 0;
    let dark = 0, playCount = 0, blue = 0, pickCount = 0;
    const playfield = [];
    for (let y = 0; y < height; y += 8) for (let x = 0; x < width; x += 8) {
      const i = (y * width + x) * 4;
      const r = data[i], g = data[i + 1], b = data[i + 2];
      const nx = x / width, ny = y / height;
      if (nx > 0.15 && nx < 0.8 && ny > 0.065 && ny < 0.15) {
        titleCount++;
        if (r > 80 && b > 80 && g < Math.min(r, b) / 2) title++;
        if (r > 150 && g > 165 && b > 175 && Math.max(r,g,b) - Math.min(r,g,b) < 70) titleLight++;
      }
      if (nx > 0.76 && nx < 0.96 && ny > 0.08 && ny < 0.94) {
        panelCount++;
        if (r > 35 && r < 220 && Math.abs(r - g) < 8 && Math.abs(r - b) < 8) gray++;
        if (r >= 10 && r <= 48 && g >= r + 8 && g <= 78 && b >= g + 5 && b <= 98) navy++;
      }
      if (nx > 0.02 && nx < 0.74 && ny > 0.06 && ny < 0.94) {
        playCount++;
        if (Math.max(r, g, b) < 32) dark++;
        playfield.push(r, g, b);
      }
      if (nx > 0.23 && nx < 0.68 && ny > 0.57 && ny < 0.74) {
        pickCount++;
        if (b > 65 && b > r * 2 && b > g * 2) blue++;
      }
    }
    return { width, height, titleMagenta: title / titleCount, titleLight: titleLight / titleCount,
      panelNavy: navy / panelCount, panelGray: gray / panelCount, playfieldDark: dark / playCount,
      pickerBlue: blue / pickCount, playfield };
  }, png.toString('base64')));
  return { png, pixels, nativeBefore, nativeAfter: await readPresentation() };
}
function metrics(frame) {
  const { playfield, ...result } = frame.pixels;
  return { ...result, ...(remastered ? { nativeBefore: frame.nativeBefore, nativeAfter: frame.nativeAfter } : {}) };
}
const isFleet = frame => remastered
  ? frame.pixels.titleLight > 0.04 && frame.pixels.panelNavy > 0.08 &&
    frame.nativeAfter.enabled === 1 && frame.nativeAfter.gameScreen === 'melee' &&
    frame.nativeBefore.backgrounds === frame.nativeAfter.backgrounds
  : frame.pixels.titleMagenta > 0.04;
const isBattle = frame => remastered
  ? frame.pixels.panelNavy > 0.22 && frame.pixels.playfieldDark > 0.65 &&
    frame.pixels.titleLight < 0.02 && frame.nativeAfter.enabled === 1 &&
    frame.nativeAfter.gameScreen === 'melee' && frame.nativeAfter.images >= 48 &&
    frame.nativeAfter.backgrounds > frame.nativeBefore.backgrounds &&
    frame.nativeAfter.draws > frame.nativeBefore.draws
  : frame.pixels.panelGray > 0.22 &&
    frame.pixels.playfieldDark > 0.65 && frame.pixels.titleMagenta < 0.02;
const isPicker = frame => !isFleet(frame) && !isBattle(frame) && frame.pixels.pickerBlue > 0.3;
async function waitForCanvas(predicate, file, timeout = 90000) {
  const until = Date.now() + timeout;
  let frame;
  do {
    await checkHealth();
    frame = await capture();
    if (predicate(frame)) {
      await writeFile(path.join(out, file), frame.png);
      return frame;
    }
    await guarded(sleep(300));
  } while (Date.now() < until);
  await writeFile(path.join(out, file.replace('.png', '-unexpected.png')), frame.png);
  assert.fail('Expected canvas state was not rendered: ' + file + ' ' + JSON.stringify(metrics(frame)));
}
function changedPixels(a, b) {
  assert.equal(a.length, b.length, 'Canvas dimensions changed during combat');
  let changed = 0;
  for (let i = 0; i < a.length; i += 3) {
    if (Math.max(Math.abs(a[i] - b[i]), Math.abs(a[i + 1] - b[i + 1]),
      Math.abs(a[i + 2] - b[i + 2])) > 16) changed++;
  }
  return changed;
}
try {
  for (const file of ['index.html', 'uqm-debug.js', 'uqm-debug.wasm', 'uqm-debug.worker.js', 'uqm-debug.data']) {
    const hash = createHash('sha256');
    for await (const chunk of createReadStream(path.join(dist, file))) hash.update(chunk);
    report.buildHashes[file] = hash.digest('hex');
  }
  const address = await server.listen(0, '127.0.0.1');
  browser = await browsers[browserType].launch({ headless: process.env.HEADED !== '1',
    ...(browserType === 'chromium' && process.env.CHROME_BIN ? { executablePath: process.env.CHROME_BIN } : {}),
    ...(browserType === 'chromium' ? { args: ['--autoplay-policy=no-user-gesture-required'] } : {}) });
  report.browser = browser.version();
  const context = await browser.newContext({ viewport: { width: 1440, height: 1200 } });
  if (meleeOnly) await useMeleePage(context);
  page = await context.newPage();
  page.setDefaultTimeout(30000);
  page.on('pageerror', fail);
  page.on('crash', () => fail(new Error('Browser page crashed')));
  page.on('console', message => {
    const text = message.text();
    report.console.push({ type: message.type(), text });
    if (report.console.length > 240) report.console.shift();
    if (fatalText.test(text)) fail(new Error(text));
  });
  report.stage = 'loading';
  await guarded(page.goto(`http://127.0.0.1:${address.port}/`));
  monitoring = true;
  monitor = (async () => {
    while (monitoring) {
      // A check that overlaps the test's own page.reload() loses its page: that is no failure.
      try { if (!reloading) await checkHealth(); }
      catch (error) { if (!reloading) throw error; }
      await sleep(300);
    }
  })().catch(error => { if (monitoring) fail(error); });
  await guarded(page.waitForFunction(() => typeof runtimeReady !== 'undefined' && runtimeReady,
    null, { timeout: 240000 }));
  // Only the Melee Remastered page shows Picture & sound; the HD page keeps its own art.
  if (await page.locator('#presentation-toggle').isVisible()) {
    // Use the real preference UI, just as a player would. Fresh tabs otherwise
    // default to Remastered, even when this run is checking Classic HD.
    await guarded(page.getByRole('button', { name: 'Picture & sound', exact: true }).click());
    await guarded(page.locator('#melee-style').selectOption(remastered ? 'remastered' : 'classic'));
    await guarded(page.getByRole('button', { name: 'Close picture and sound', exact: true }).click());
  }
  if (remastered) {
    const ready = await readPresentation();
    assert.equal(ready.enabled, 1, 'Remastered mode must use the actual native renderer');
    assert.equal(ready.backgrounds, 0, 'No battle has begun during loading');
    report.presentationAtStart = ready;
  }
  if (meleeOnly) {
    await recordMeleeScreens(page);
    await assertMeleeChoices(page);
    await launchLocalMelee(page);
  } else {
    await guarded(page.getByRole('button', { name: 'Play', exact: true }).click());
    report.stage = 'main-menu';
    await waitForScreen('mainMenu');
  }
  report.canvas = await guarded(page.locator('#canvas').evaluate(canvas => ({
    width: canvas.width, height: canvas.height,
  })));
  assert.deepEqual(report.canvas, { width: 1280, height: 960 });
  await guarded(sleep(1500));
  await guarded(page.locator('#canvas').focus());
  if (!meleeOnly) {
    await key('ArrowDown');
    await key('ArrowDown');
    await key('Enter');
  }
  report.stage = 'fleet-setup';
  await waitForScreen('melee');
  report.fleet = metrics(await waitForCanvas(isFleet, 'fleet-before-idle.png'));
  report.stage = 'idle';
  report.idleStartedAt = new Date().toISOString();
  console.log(`${browserType}: actual HD fleet setup ready; idle check ${idleMs} ms.`);
  await guarded(sleep(idleMs));
  await checkHealth();
  report.idleCompletedAt = new Date().toISOString();
  const afterIdle = await capture();
  assert.ok(isFleet(afterIdle), 'The idle session left fleet setup unexpectedly');
  await writeFile(path.join(out, 'fleet-after-idle.png'), afterIdle.png);
  report.stage = 'ship-selection';
  await key('Enter');
  report.picker = metrics(await waitForCanvas(isPicker, 'ship-selection.png'));
  await key('Enter');
  report.stage = 'battle';
  let previous = await waitForCanvas(isBattle, 'battle-start.png');
  report.battleFrames.push({ sample: 0, elapsedMs: 0, ...metrics(previous) });
  await guarded(page.keyboard.down('ArrowUp'));
  await guarded(page.keyboard.down('ControlRight'));
  const battleStart = Date.now();
  try {
    // Sample only the playfield. HUD animation or a blinking selection cursor
    // cannot satisfy the motion check. A destroyed ship may return to selection.
    // Screenshots can take longer under load; record real elapsed time and
    // number images by sample rather than claiming exact capture seconds.
    for (let second = 1; second <= 15; second++) {
      await guarded(sleep(Math.max(0, battleStart + second * 1000 - Date.now())));
      await checkHealth();
      const frame = await capture();
      if (isBattle(frame)) {
        const changed = changedPixels(previous.pixels.playfield, frame.pixels.playfield);
        report.battleFrames.push({ sample: second, elapsedMs: Date.now() - battleStart,
          changedPixels: changed, ...metrics(frame) });
        previous = frame;
        if (second === 1 || second % 5 === 0) {
          await writeFile(path.join(out, `battle-${String(second).padStart(2, '0')}.png`), frame.png);
        }
      }
    }
  } finally {
    await page.keyboard.up('ControlRight').catch(() => {});
    await page.keyboard.up('ArrowUp').catch(() => {});
  }
  report.battleElapsedMs = Date.now() - battleStart;
  await writeFile(path.join(out, 'after-controls.png'), (await capture()).png);
  assert.ok(report.battleFrames.filter(frame => frame.changedPixels >= 4).length >= 2,
    'Did not observe multiple changing frames in an actual battle playfield');
  if (remastered) {
    assert.ok(report.battleFrames.some(frame => frame.elapsedMs >= 14000),
      'The remastered renderer must still be drawing a real battle near the end of the 15-second control interval');
    report.presentationAtEnd = await readPresentation();
    assert.ok(report.presentationAtEnd.backgrounds > report.battleFrames[0].nativeAfter.backgrounds,
      'Native battle background drawing must continue during the control interval');
  }
  if (meleeOnly) {
    report.stage = 'battle-exit';
    await leaveLocalBattle(page, path.join(out, 'quit-confirmation.png'));
    await page.screenshot({ path: path.join(out, 'two-modes-after-battle.png') });
    report.returnHomeIdleMs = 25000;
    await guarded(sleep(report.returnHomeIdleMs));
    await assertMeleeChoices(page, true);
    report.screensBeforeReload = await assertMeleeScreens(page);
    assert.ok(report.screensBeforeReload.includes('meleeMenu'));
    report.stage = 'local-replay';
    await launchLocalMelee(page);
    report.replayFleet = metrics(await waitForCanvas(isFleet, 'fleet-repeat.png'));
    await leaveLocalFleet(page);
    await page.getByRole('button', { name: 'Network Melee', exact: true }).click();
    await page.waitForFunction(() => online.state.screen === 'lobby');
    await page.keyboard.press('Escape');
    await assertMeleeChoices(page, true);
    report.screensBeforeReload = await assertMeleeScreens(page);
    report.stage = 'reload';
    reloading = true;
    await page.reload();
    await page.waitForFunction(() => runtimeReady, null, { timeout: 240000 });
    reloading = false;
    await recordMeleeScreens(page);
    await assertMeleeChoices(page);
    await launchLocalMelee(page);
    report.reloadedFleet = metrics(await waitForCanvas(isFleet, 'fleet-after-reload.png'));
    await leaveLocalFleet(page);
    report.screensAfterReload = await assertMeleeScreens(page);
    report.localReplayAndReload = true;
  }
  await checkHealth();
  assert.deepEqual(report.errors, []);
  report.stage = 'complete';
  report.passed = true;
  console.log(`${browserType}: HD survived ${idleMs} ms idle and at least 15 seconds of battle controls with real playfield motion.`);
} catch (error) {
  report.failure = error.stack || String(error);
  if (page && !page.isClosed()) {
    await page.screenshot({ path: path.join(out, 'failure.png'), fullPage: true, timeout: 5000 }).catch(() => {});
  }
  throw error;
} finally {
  monitoring = false;
  clearTimeout(deadline);
  if (page && !page.isClosed()) {
    report.log = await Promise.race([
      page.locator('#log').textContent().catch(() => ''), sleep(2000).then(() => 'Log collection timed out'),
    ]);
  }
  report.finishedAt = new Date().toISOString();
  await writeFile(path.join(out, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  await browser?.close();
  await monitor;
  await server.close();
}
