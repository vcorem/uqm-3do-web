// Real packaged engines, separate IDBFS databases, no gameplay or draw mocks.
import assert from 'node:assert/strict';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium, firefox, webkit } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';

const dist = path.resolve(process.env.UQM_DIST || '.tools/editions-dist');
const out = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/editions');
const browserType = process.env.UQM_BROWSER || 'chromium';
const types = { chromium, firefox, webkit };
assert.ok(types[browserType], 'Unsupported UQM_BROWSER');
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const fixture = await readFile(new URL('./fixtures/before-starbase.uqm', import.meta.url));
await mkdir(out, { recursive: true });
const server = createNetplayServer({ distDir: dist });
const address = await server.listen(0, '127.0.0.1');
const origin = `http://127.0.0.1:${address.port}`;
const report = { browserType, errors: [], requests: [], passed: false };
let browser, page;
let rejectFatal;
const fatal = new Promise((_, reject) => { rejectFatal = reject; });
// A page error may arrive between awaited operations; retain it for the next
// wait without producing an unrelated unhandled rejection.
fatal.catch(() => {});
async function waitForGame(target, timeout) {
  await Promise.race([page.waitForFunction(target => {
    const module = typeof Module === 'object' ? Module : {};
    const button = document.getElementById('start');
    const failed = button?.textContent === 'Unable to start' || module.exitStatus !== undefined ||
      (button?.hidden && document.getElementById('status')?.textContent.includes('Game closed'));
    return failed || (target === 'runtime'
      ? typeof runtimeReady !== 'undefined' && runtimeReady
      : module.gameScreen === target);
  }, target, { timeout }), fatal]);
  const state = await page.evaluate(() => ({
    runtimeReady: typeof runtimeReady !== 'undefined' && runtimeReady,
    screen: typeof Module === 'object' ? Module.gameScreen : undefined,
    status: document.getElementById('status')?.textContent,
    failed: document.getElementById('start')?.textContent === 'Unable to start',
    exited: typeof Module === 'object' && Module.exitStatus !== undefined,
  }));
  assert.ok(!state.failed && !state.exited && !state.status?.includes('Game closed'),
    JSON.stringify(state) + '\n' + await page.locator('#log').textContent());
  assert.ok(target === 'runtime' ? state.runtimeReady : state.screen === target, JSON.stringify(state));
}
async function chooseEditionWithKeyboard() {
  assert.equal(await page.evaluate(() => document.activeElement === canvas), true,
    'Launching must focus the game canvas');
  await page.keyboard.press('Shift+Tab');
  await page.waitForFunction(() => document.activeElement?.id === 'choose-edition');
  await page.keyboard.press('Enter');
}
async function waitForRenderedCanvas(file, timeout = 60000) {
  const deadline = Date.now() + timeout;
  let pixels;
  do {
    const png = await page.locator('#canvas').screenshot();
    pixels = await page.evaluate(async encoded => {
      const bytes = Uint8Array.from(atob(encoded), c => c.charCodeAt(0));
      const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
      const surface = document.createElement('canvas');
      surface.width = bitmap.width; surface.height = bitmap.height;
      const ctx = surface.getContext('2d'); ctx.drawImage(bitmap, 0, 0); bitmap.close();
      const data = ctx.getImageData(0, 0, surface.width, surface.height).data;
      const colors = new Set(); let sampled = 0, lit = 0;
      for (let i = 0; i < data.length; i += 64) {
        sampled++;
        if (Math.max(data[i], data[i + 1], data[i + 2]) > 32) lit++;
        colors.add((data[i] >> 4) * 256 + (data[i + 1] >> 4) * 16 + (data[i + 2] >> 4));
      }
      return { litFraction: lit / sampled, colors: colors.size };
    }, png.toString('base64'));
    if (pixels.litFraction > 0.05 && pixels.colors >= 24) {
      await writeFile(path.join(out, file), png);
      return pixels;
    }
    await Promise.race([sleep(300), fatal]);
  } while (Date.now() < deadline);
  assert.fail('Melee entered but canvas remained blank: ' + JSON.stringify(pixels));
}
async function sampleAudio() {
  return page.evaluate(() => {
    const ctx = typeof AL === 'object' ? AL.currentCtx : null;
    const time = ctx?.audioCtx.currentTime;
    let peak = 0, energy = 0, samples = 0, playingSources = 0;
    for (const source of Object.values(ctx?.sources || {})) {
      if (source.state !== 4114 || !(source.gain.gain.value > 0)) continue;
      playingSources++;
      for (const item of source.audioQueue || []) {
        if (item._startTime > time + 0.11 || item._startTime + item._duration < time) continue;
        const data = item.buffer?.getChannelData(0);
        if (data) for (const value of data) {
          peak = Math.max(peak, Math.abs(value)); energy += value * value; samples++;
        }
      }
    }
    return { state: ctx?.audioCtx.state, time, gain: ctx?.gain.gain.value,
      playingSources, peak, samples, rms: samples ? Math.sqrt(energy / samples) : 0 };
  });
}
try {
  browser = await types[browserType].launch({ headless: process.env.HEADED !== '1',
    ...(browserType === 'chromium' && process.env.CHROME_BIN ? { executablePath: process.env.CHROME_BIN } : {}),
    ...(browserType === 'chromium' ? { args: ['--autoplay-policy=no-user-gesture-required'] } : {}) });
  report.browser = browser.version();
  page = await browser.newPage({ viewport: { width: 1440, height: 1200 } });
  page.on('pageerror', error => { report.errors.push(error.message); rejectFatal(error); });
  page.on('crash', () => { report.errors.push('Browser page crashed'); rejectFatal(new Error('Browser page crashed')); });
  page.on('request', request => report.requests.push(new URL(request.url()).pathname));
  await page.goto(origin);
  await sleep(250);
  assert.ok(report.requests.every(p => !/\.(wasm|data)$|uqm-debug|netplay-client/.test(p)), 'Picker loaded an engine before a choice');
  await page.screenshot({ path: path.join(out, 'chooser.png'), fullPage: true });
  await page.locator('#launch-edition').click();
  await waitForGame('runtime', 180000);
  assert.equal(new URL(page.url()).pathname, '/original/index.html');
  await page.evaluate(async bytes => {
    FS.mkdirTree('/home/web_user/.uqm/save');
    FS.writeFile('/home/web_user/.uqm/save/uqmsave.00', new Uint8Array(bytes));
    await new Promise((resolve, reject) => FS.syncfs(false, e => e ? reject(e) : resolve()));
  }, Array.from(fixture));
  assert.ok(!report.requests.some(p => p.startsWith('/hd/')), 'Original selection downloaded HD');
  await page.getByRole('button', { name: 'Launch game', exact: true }).click();
  await waitForGame('mainMenu', 120000);
  await page.locator('#canvas').screenshot({ path: path.join(out, 'original-menu.png') });
  await chooseEditionWithKeyboard();
  // The chooser restores preferences in a deferred script after navigation.
  await page.waitForFunction(() => typeof UQMEditions === 'object');
  await page.locator('#edition-hd').check();
  await page.locator('#launch-edition').click();
  await waitForGame('runtime', 240000);
  assert.equal(new URL(page.url()).pathname, '/hd/index.html');
  report.hd = await page.evaluate(() => ({
    config: FS.readFile('/home/web_user/.uqm-hd/uqm.cfg', { encoding: 'utf8' }),
    originalMounted: FS.analyzePath('/home/web_user/.uqm').exists,
    args: Module.arguments,
    archives: FS.readdir('/content/addons'),
  }));
  assert.equal(report.hd.originalMounted, false);
  assert.match(report.hd.config, /^resolutionfactor = INT32:2$/m);
  assert.match(report.hd.config, /^subtitles = BOOLEAN:true$/m);
  assert.ok(report.hd.archives.includes('mm-0.8.5-hd-content.uqm'));
  await page.getByRole('button', { name: 'Launch HD game', exact: true }).click();
  await waitForGame('mainMenu', 90000);
  await sleep(1500);
  report.hd.render = await page.evaluate(() => ({ width: canvas.width, height: canvas.height,
    audioState: AL.currentCtx?.audioCtx.state, audioTime: AL.currentCtx?.audioCtx.currentTime }));
  assert.equal(report.hd.render.width, 1280);
  assert.equal(report.hd.render.height, 960);
  assert.equal(report.hd.render.audioState, 'running');
  report.hd.audioSamples = [];
  for (let i = 0; i < 2; i++) {
    await sleep(300);
    report.hd.audioSamples.push(await sampleAudio());
  }
  for (const sample of report.hd.audioSamples) {
    assert.equal(sample.state, 'running');
    assert.ok(sample.gain > 0 && sample.playingSources > 0 && sample.peak > 0.001 && sample.rms > 0,
      'HD menu must play nonzero PCM through an audible OpenAL source: ' + JSON.stringify(sample));
  }
  assert.ok(report.hd.audioSamples[1].time - report.hd.audioSamples[0].time > 0.1,
    'HD audio clock did not advance');
  await page.locator('#canvas').screenshot({ path: path.join(out, 'hd-menu.png') });
  await page.locator('#canvas').focus();
  // Keep each press below UQM's 500ms auto-repeat delay, and allow a full
  // released interval so Firefox cannot coalesce consecutive Down inputs.
  await page.keyboard.press('ArrowDown', { delay: 250 });
  await sleep(750);
  await page.keyboard.press('ArrowDown', { delay: 250 });
  await sleep(750);
  await page.locator('#canvas').screenshot({ path: path.join(out, 'hd-melee-selected.png') });
  await page.keyboard.press('Enter', { delay: 250 });
  await waitForGame('melee', 60000);
  await sleep(2500);
  report.hd.meleeCanvas = await waitForRenderedCanvas('hd-melee.png');
  // Persist a distinct HD marker through the actual IDBFS mount. Campaign
  // gameplay saving is checked separately; this proves database isolation.
  await page.evaluate(async () => {
    FS.writeFile('/home/web_user/.uqm-hd/edition-check.txt', 'hd-only');
    await new Promise((resolve, reject) => FS.syncfs(false, e => e ? reject(e) : resolve()));
  });
  await page.reload();
  await waitForGame('runtime', 240000);
  assert.equal(await page.evaluate(() => FS.readFile('/home/web_user/.uqm-hd/edition-check.txt', { encoding: 'utf8' })), 'hd-only');
  await page.getByRole('button', { name: 'Launch HD game', exact: true }).click();
  await waitForGame('mainMenu', 90000);
  await chooseEditionWithKeyboard();
  // The chooser restores preferences in a deferred script after navigation.
  await page.waitForFunction(() => typeof UQMEditions === 'object');
  assert.equal(await page.locator('#edition-hd').isChecked(), true);
  await page.locator('#edition-original').check();
  await page.locator('#launch-edition').click();
  await waitForGame('runtime', 180000);
  const saved = await page.evaluate(() => ({
    bytes: Array.from(FS.readFile('/home/web_user/.uqm/save/uqmsave.00')),
    hdMounted: FS.analyzePath('/home/web_user/.uqm-hd').exists,
  }));
  assert.deepEqual(saved.bytes, Array.from(fixture));
  assert.equal(saved.hdMounted, false);
  // A package with the Rust builds offers an engine switch. The Rust pages must boot,
  // say which engine runs, and keep their saves apart from the C saves.
  const offersRust = (await readFile(path.join(dist, 'index.html'), 'utf8')).includes('data-engines="c,rust"');
  if (offersRust) {
    report.rust = {};
    for (const [edition, folder, saveDir, button] of [
      ['original', 'original-rust', '/home/web_user/.uqm-rust', 'Launch game'],
      ['hd', 'hd-rust', '/home/web_user/.uqm-hd-rust', 'Launch HD game']]) {
      await page.goto(origin);
      await page.waitForFunction(() => typeof UQMEditions === 'object');
      await page.locator('#engine-rust').check();
      await page.locator('#edition-' + edition).check();
      assert.match(await page.locator('#launch-label').textContent(), /· Rust$/);
      await page.locator('#launch-edition').click();
      await waitForGame('runtime', 240000);
      assert.equal(new URL(page.url()).pathname, '/' + folder + '/index.html');
      const state = await page.evaluate(([saveDir, edition]) => ({
        savePath: Module.uqmSavePath, args: Module.arguments, label: document.querySelector('.edition').textContent,
        rustMounted: FS.analyzePath(saveDir).exists,
        cSaveVisible: FS.analyzePath(edition === 'hd' ? '/home/web_user/.uqm-hd/edition-check.txt' : '/home/web_user/.uqm/save/uqmsave.00').exists,
      }), [saveDir, edition]);
      report.rust[folder] = state;
      assert.equal(state.savePath, saveDir);
      assert.ok(state.args.includes('--configdir=' + saveDir), JSON.stringify(state.args));
      assert.match(state.label, /Rust engine/);
      assert.equal(state.rustMounted, true);
      assert.equal(state.cSaveVisible, false, 'A Rust page must not see the C saves');
      const language = await (await page.request.get(origin + '/' + folder + '/engine-language.json')).json();
      assert.equal(language.core, 'rust');
      await page.evaluate(async ([saveDir, folder]) => {
        FS.writeFile(saveDir + '/engine-check.txt', folder);
        await new Promise((resolve, reject) => FS.syncfs(false, e => e ? reject(e) : resolve()));
      }, [saveDir, folder]);
      await page.reload();
      await waitForGame('runtime', 240000);
      assert.equal(await page.evaluate(saveDir => FS.readFile(saveDir + '/engine-check.txt', { encoding: 'utf8' }), saveDir), folder);
      await page.getByRole('button', { name: button, exact: true }).click();
      await waitForGame('mainMenu', 120000);
      if (edition === 'hd') assert.equal(await page.evaluate(() => canvas.width), 1280);
      await page.locator('#canvas').screenshot({ path: path.join(out, folder + '-menu.png') });
    }
    // The C saves are unchanged after both Rust engines ran.
    await page.goto(origin + '/original/index.html');
    await waitForGame('runtime', 180000);
    const after = await page.evaluate(() => ({ bytes: Array.from(FS.readFile('/home/web_user/.uqm/save/uqmsave.00')),
      rustCheck: FS.analyzePath('/home/web_user/.uqm/engine-check.txt').exists,
      databases: indexedDB.databases ? null : 'unsupported' }));
    assert.deepEqual(after.bytes, Array.from(fixture));
    assert.equal(after.rustCheck, false);
    report.rust.databases = await page.evaluate(async () => indexedDB.databases ? (await indexedDB.databases()).map(d => d.name).sort() : null);
  }
  assert.deepEqual(report.errors, []);
  report.passed = true;
  console.log(`${browserType}: both real engines boot; HD is 1280x960; audio and Melee work; saves isolated and persisted.` +
    (offersRust ? ' Rust Original and HD boot from the engine switch with their own saves.' : ''));
} catch (error) {
  report.failure = error.stack;
  if (page && !page.isClosed()) {
    await page.screenshot({ path: path.join(out, 'failure.png'), fullPage: true }).catch(() => {});
    report.log = await page.locator('#log').textContent().catch(() => '');
  }
  throw error;
} finally {
  await writeFile(path.join(out, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  await browser?.close();
  await server.close();
}
