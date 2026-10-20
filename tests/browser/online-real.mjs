// The start screen and lobby on a real built page (real engine): loading
// progress in the frame, Play to the game's main menu in full screen, the
// corner button and the O key over the real main menu, and back.
//   UQM_DIST=.tools/original-dist UQM_EDITION=original node tests/browser/online-real.mjs
import assert from 'node:assert/strict';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';
import { createLobbyStub } from '../helpers/online-lobby.mjs';
import { shimPath } from './online-harness.mjs';

const dist = path.resolve(process.env.UQM_DIST || '.tools/original-dist');
const edition = process.env.UQM_EDITION || 'original';
const label = process.env.UQM_LABEL || path.basename(dist);
const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/online-real');
const viewport = { width: 1280, height: 960 };
await mkdir(output, { recursive: true });
const stub = createLobbyStub(createNetplayServer({ distDir: dist }));
const { port } = await stub.listen(0);
const browser = await chromium.launch({ headless: process.env.HEADED !== '1', args: ['--autoplay-policy=no-user-gesture-required'] });
const result = { dist, edition, steps: [], passed: false };
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
try {
  const context = await browser.newContext({ viewport });
  await context.addInitScript({ path: shimPath });
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', error => errors.push(error.stack || error.message));
  const state = () => page.evaluate(() => ({ screen: online.state.screen, focus: online.state.focus, gameScreen: Module.gameScreen,
    fullscreen: !!document.fullscreenElement, corner: !online.corner.hidden, overlay: !online.overlay.hidden,
    loading: online.state.loading, ready: runtimeReady, status: document.getElementById('status').textContent }));
  const shot = async name => {
    await sleep(300);
    const s = await state();
    const file = path.join(output, `${label}-${name}.png`);
    if (s.fullscreen) await page.screenshot({ path: file }); else await page.locator('#screen').screenshot({ path: file });
    result.steps.push({ name, file, state: s });
    console.log(`${label}: ${name} ${JSON.stringify({ screen: s.screen, gameScreen: s.gameScreen, fullscreen: s.fullscreen })}`);
  };
  await page.goto(`http://127.0.0.1:${port}/`);
  await page.waitForFunction(() => window.online && online.state.loading && online.state.loading.fraction > 0.05, null, { timeout: 120000 });
  await shot('start-loading');
  // Play during the download: the game starts by itself when it is ready.
  await page.keyboard.press('Enter');
  await page.waitForFunction(() => online.state.pendingPlay || runtimeReady, null, { timeout: 10000 });
  await shot('start-pending');
  await page.waitForFunction(() => runtimeReady && Module.gameScreen === 'mainMenu', null, { timeout: 240000, polling: 250 });
  let s = await state();
  assert.equal(s.fullscreen, true, 'Play put the frame in full screen');
  assert.equal(s.overlay, false);
  await page.waitForFunction(() => !online.corner.hidden, null, { timeout: 10000 });
  await shot('main-menu');
  assert.equal(await page.evaluate(() => document.activeElement === canvas), true, 'the game has the keyboard');
  await page.keyboard.press('o');
  await page.waitForFunction(() => ['lobby', 'restart'].includes(online.state.screen), null, { timeout: 10000 });
  s = await state();
  result.hook = await page.evaluate(() => Module.supportsNetplayHook === true);
  await shot(s.screen === 'lobby' ? 'lobby-over-game' : 'restart-over-game');
  await page.keyboard.press('Escape');
  await page.waitForFunction(() => online.state.screen === 'hidden', null, { timeout: 10000 });
  await sleep(500);
  s = await state();
  assert.equal(s.gameScreen, 'mainMenu', 'the O key and Esc did not reach the game menu');
  assert.equal(await page.evaluate(() => document.activeElement === canvas), true);
  await shot('back-to-menu');
  assert.deepEqual(errors, []);
  result.passed = true;
} finally {
  await writeFile(path.join(output, `${label}-results.json`), JSON.stringify(result, null, 2) + '\n');
  await browser.close();
  await stub.close();
}
console.log(`${label}: real page start screen, main menu button and lobby passed`);
