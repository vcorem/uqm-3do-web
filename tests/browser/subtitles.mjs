// Actual voiced dialogue: a normal save, real keys, file-open diagnostics,
// OpenAL's scheduled PCM, and pixels from the rendered canvas. No game state
// or subtitle drawing functions are patched. See fixtures/README.md.
import assert from 'node:assert/strict';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { chromium } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';

const distDir = path.resolve(process.env.UQM_DIST || 'dist');
const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/subtitles');
const fixture = await readFile(new URL('./fixtures/before-starbase.uqm', import.meta.url));
const config = 'subtitles = BOOLEAN:false\n';
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const results = [];
await mkdir(output, { recursive: true });
const buildHashes = Object.fromEntries(await Promise.all(
  ['index.html', 'uqm-debug.wasm', 'uqm-debug.js', 'uqm-debug.worker.js', 'uqm-debug.data'].map(async file =>
    [file, createHash('sha256').update(await readFile(path.join(distDir, file))).digest('hex')])));
await writeFile(path.join(output, 'build-hashes.json'), JSON.stringify({
  ...buildHashes, fixture: createHash('sha256').update(fixture).digest('hex'),
}, null, 2) + '\n');

async function speechState(page) {
  return page.evaluate(() => {
    const ctx = typeof AL === 'object' ? AL.currentCtx : null;
    const sources = Object.values(ctx?.sources || {}).filter(Boolean).sort((a, b) => a.id - b.id);
    // Fresh UQM OpenAL initialization creates five effects sources, music,
    // then speech. Fail closed if the backend/layout changes.
    const speech = sources.length === 7 ? sources[6] : null;
    let peak = 0, samples = 0, energy = 0;
    for (const source of speech?.audioQueue || []) {
      if (source._startTime > ctx.audioCtx.currentTime + 0.11) continue;
      if (source._startTime + source._duration < ctx.audioCtx.currentTime) continue;
      const data = source.buffer?.getChannelData(0);
      if (data) for (const value of data) {
        peak = Math.max(peak, Math.abs(value)); energy += value * value; samples++;
      }
    }
    return {
      context: ctx?.audioCtx.state, time: ctx?.audioCtx.currentTime,
      contextGain: ctx?.gain.gain.value, sourceCount: sources.length,
      speechId: speech?.id, state: speech?.state, gain: speech?.gain.gain.value,
      queued: speech?.audioQueue.length, peak, rms: samples ? Math.sqrt(energy / samples) : 0,
      samples, voiceFiles: Object.keys(FS.readFiles || {}).filter(p => /\/commander\/commander-003\.ogg$/.test(p)),
    };
  });
}

async function subtitlePixels(page, png) {
  return page.evaluate(async base64 => {
    const bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
    const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
    const canvas = document.createElement('canvas');
    canvas.width = bitmap.width; canvas.height = bitmap.height;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(bitmap, 0, 0); bitmap.close();
    // Hayes's subtitles occupy this region of the original 320x240 canvas.
    // Its portrait background has no white pixels; facial animations and
    // running lights are outside it. The negative control verifies that too.
    const sx = canvas.width / 320, sy = canvas.height / 240;
    const pixels = ctx.getImageData(Math.round(100 * sx), Math.round(10 * sy),
      Math.round(142 * sx), Math.round(60 * sy)).data;
    let white = 0, hash = 2166136261;
    for (let i = 0; i < pixels.length; i += 4) {
      const value = Math.min(pixels[i], pixels[i + 1], pixels[i + 2]) >= 225 ? 1 : 0;
      white += value; hash = Math.imul(hash ^ value, 16777619) >>> 0;
    }
    return { width: canvas.width, height: canvas.height, whitePixels: white,
      nativeWhitePixels: white / sx / sy, glyphMaskHash: hash };
  }, png.toString('base64'));
}

async function scenario(origin, name, enableOverride) {
  const entry = { name, passed: false, errors: [], frames: [] };
  results.push(entry);
  let browser, page;
  let held = [];
  try {
    browser = await chromium.launch({
      headless: process.env.HEADED !== '1',
      ...(process.env.CHROME_BIN ? { executablePath: process.env.CHROME_BIN } : {}),
      args: ['--autoplay-policy=no-user-gesture-required', '--disable-renderer-backgrounding',
        '--disable-backgrounding-occluded-windows', '--disable-background-timer-throttling'],
    });
    entry.browser = browser.version();
    page = await browser.newPage({ viewport: { width: 1100, height: 1100 } });
    page.on('pageerror', error => entry.errors.push(error.message));
    page.on('crash', () => entry.errors.push('page crashed'));
    await page.goto(origin);
    await page.waitForFunction(() => runtimeReady, null, { timeout: 90000 });
    await page.evaluate(({ save, config, enableOverride }) => {
      FS.mkdirTree('/home/web_user/.uqm/save');
      FS.writeFile('/home/web_user/.uqm/save/uqmsave.00', new Uint8Array(save));
      FS.writeFile('/home/web_user/.uqm/uqm.cfg', config);
      // This is an existing Emscripten read-open diagnostic, not a replacement
      // filesystem or hook that alters file contents/decoder behavior.
      Module.logReadFiles = true;
      if (!enableOverride) Module.arguments = Module.arguments.filter(arg => arg !== '--subtitles');
    }, { save: Array.from(fixture), config, enableOverride });
    await page.getByRole('button', { name: 'Launch game', exact: true }).click();
    await page.waitForFunction(() => Module.gameScreen === 'mainMenu', null, { timeout: 60000 });
    entry.startup = await page.evaluate(() => ({
      arguments: Module.arguments,
      // Record successful game access BEFORE the diagnostic reads its content.
      configOpened: Object.keys(FS.readFiles || {}).filter(p => p.endsWith('/uqm.cfg')),
      configContent: FS.readFile('/home/web_user/.uqm/uqm.cfg', { encoding: 'utf8' }),
    }));
    assert.ok(entry.startup.configOpened.includes('/home/web_user/.uqm/uqm.cfg'));
    assert.equal(entry.startup.configContent, config);
    assert.equal(entry.startup.arguments.includes('--subtitles'), enableOverride);
    assert.ok(!entry.startup.arguments.includes('--nosubtitles'));
    await page.locator('#canvas').focus();
    await page.keyboard.press('ArrowDown', { delay: 150 });
    await page.keyboard.press('Enter', { delay: 150 });
    // First load prepares the real orbital assets before displaying save slots.
    await sleep(12000);
    await page.locator('#canvas').screenshot({ path: path.join(output, `${name}-load-slots.png`) });
    await page.keyboard.press('Enter', { delay: 150 });
    // A loaded ship retains momentum. Repeated Escape reaches the solar menu
    // as soon as flight resumes; Escape is harmless while that menu is open.
    for (let i = 0; i < 10; i++) {
      await sleep(300); await page.keyboard.press('Escape', { delay: 80 });
    }
    await page.locator('#canvas').screenshot({ path: path.join(output, `${name}-before-flight.png`) });
    await page.keyboard.press('ArrowUp', { delay: 100 }); // StarMap -> Navigate
    await page.keyboard.press('Enter', { delay: 100 });
    await sleep(60);
    const segments = [
      { keys: ['ArrowRight', 'ArrowUp'], ms: 1950 },
      { keys: ['ArrowUp'], ms: 2000 },
    ];
    let reachedHayes = false;
    for (const segment of segments) {
      for (const key of held) if (!segment.keys.includes(key)) await page.keyboard.up(key);
      for (const key of segment.keys) if (!held.includes(key)) await page.keyboard.down(key);
      held = segment.keys;
      const until = Date.now() + segment.ms;
      while (Date.now() < until) {
        if ((await speechState(page)).voiceFiles.length) { reachedHayes = true; break; }
        await sleep(40);
      }
      if (reachedHayes) break;
    }
    for (const key of held) await page.keyboard.up(key);
    held = [];
    // The collision can start the encounter at the end of the last segment.
    // Let real portrait/music/voice loading finish with no further movement.
    // Reaching the file alone is insufficient: PCM and glyph checks follow.
    const dialogueDeadline = Date.now() + 20000;
    while (!reachedHayes && Date.now() < dialogueDeadline) {
      reachedHayes = (await speechState(page)).voiceFiles.length > 0;
      if (!reachedHayes) await sleep(100);
    }
    assert.ok(reachedHayes, 'Real keyboard navigation did not reach Hayes; inspect failure screenshot.');
    const until = Date.now() + 8500;
    while (Date.now() < until) {
      const file = `${name}-speech-${String(entry.frames.length).padStart(2, '0')}.png`;
      const png = await page.locator('#canvas').screenshot({ path: path.join(output, file) });
      entry.frames.push({ file, pixels: await subtitlePixels(page, png), audio: await speechState(page) });
      await sleep(350);
    }
    const voiced = entry.frames.filter(({ audio: a }) => a.voiceFiles.length && a.sourceCount === 7 &&
      a.state === 4114 && a.context === 'running' && a.peak > 0.001 && a.rms > 0 && a.gain > 0 && a.contextGain > 0);
    assert.ok(voiced.length >= 8, 'Expected multiple actual speech PCM samples, not only a loaded filename.');
    assert.ok(voiced.at(-1).audio.time - voiced[0].audio.time > 4, 'Speech clock did not advance.');
    const visible = voiced.filter(frame => frame.pixels.nativeWhitePixels > 30);
    const masks = new Set(visible.map(frame => frame.pixels.glyphMaskHash));
    entry.actualVoiceSamples = voiced.length;
    entry.subtitleSamples = visible.length;
    entry.uniqueGlyphMasks = masks.size;
    if (enableOverride) {
      assert.ok(visible.length >= 8, 'Spoken dialogue was not visibly subtitled.');
      assert.ok(masks.size >= 2, 'Expected changing subtitle pages during speech.');
    } else {
      assert.ok(voiced.every(frame => frame.pixels.whitePixels === 0),
        'Legacy disabled preference unexpectedly produced visible subtitle pixels.');
    }
    const logs = await page.locator('#log').textContent();
    assert.match(logs, /Switching to stream .*commander-003\.ogg/);
    assert.doesNotMatch(logs, /Aborted\(|Assertion failed|memory access out of bounds|Uncaught RuntimeError/);
    assert.deepEqual(entry.errors, []);
    entry.passed = true;
    console.log(`${name}: ${voiced.length} speech samples; ${visible.length} subtitled; ${masks.size} text pages`);
  } catch (error) {
    entry.error = error.stack || String(error);
    if (page && !page.isClosed()) {
      for (const key of held) await page.keyboard.up(key).catch(() => {});
      await page.locator('#canvas').screenshot({ path: path.join(output, `${name}-failure.png`) }).catch(() => {});
    }
    throw error;
  } finally {
    if (page && !page.isClosed()) await writeFile(path.join(output, `${name}.log`),
      await page.locator('#log').textContent().catch(() => 'page unavailable'));
    await browser?.close();
    await writeFile(path.join(output, 'results.json'), JSON.stringify(results, null, 2) + '\n');
  }
}

const app = createNetplayServer({ distDir });
try {
  const address = await app.listen(0);
  const origin = `http://127.0.0.1:${address.port}`;
  await scenario(origin, 'enabled-over-legacy-false', true);
  await scenario(origin, 'legacy-false-without-override', false);
  console.log(`Actual voiced subtitle regression passed. Evidence: ${output}`);
} finally {
  await app.close();
}
