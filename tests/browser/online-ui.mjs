// The start screen and the online lobby inside the game frame, in both
// editions: the real page shells and page scripts with a test engine
// (fixtures/fake-engine.js), through a test lobby (tests/helpers/online-lobby.mjs)
// in front of the real Node relay. Keyboard-only and mouse flows run at a
// full-screen 1280x960 and 1024x768; every screen is saved for review.
//   node tests/browser/online-ui.mjs   (UQM_EDITIONS=original,hd  UQM_TEST_OUTPUT=dir)
import assert from 'node:assert/strict';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';
import { createLobbyStub } from '../helpers/online-lobby.mjs';
import { buildHarness, shimPath } from './online-harness.mjs';

const output = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/online-ui');
const pages = path.join(output, 'pages');
await mkdir(output, { recursive: true });
// HD needs the HD content (.tools/hd-source); without UQM_EDITIONS it is skipped when missing.
const requested = (process.env.UQM_EDITIONS || 'original,hd').split(',');
const editions = await buildHarness(pages, requested);
if (process.env.UQM_EDITIONS) assert.deepEqual(editions, requested, 'Every requested edition must build');
else assert.ok(editions.includes('original'), 'The Original harness must build');
const viewports = (process.env.UQM_VIEWPORTS || '1280x960,1024x768').split(',').map(text => {
  const [width, height] = text.split('x').map(Number);
  return { width, height };
});
const stub = createLobbyStub(createNetplayServer({ distDir: pages }));
const { port } = await stub.listen(0);
const origin = `http://127.0.0.1:${port}`;
const browser = await chromium.launch({ headless: process.env.HEADED !== '1' });
const results = [];
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const token = n => n.toString(16).padStart(2, '0').repeat(16);

async function player(edition, viewport, query = '', hash = '') {
  const context = await browser.newContext({ viewport });
  await context.addInitScript({ path: shimPath });
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', error => errors.push(error.stack || error.message));
  page.on('console', message => { if (message.type() === 'error' && !/WebSocket|Failed to load resource/.test(message.text())) errors.push(message.text()); });
  await page.goto(`${origin}/${edition}/index.html?fakeLoadMs=1500${query}${hash}`);
  await page.waitForFunction(() => window.online && online.state && document.querySelector('.uqm-online-hit'), null, { timeout: 15000 });
  return { context, page, errors, edition, viewport };
}
const ui = p => p.page.evaluate(() => {
  const s = online.state;
  return { screen: s.screen, focus: s.focus, items: s.items, lobby: s.lobby.status, rooms: s.lobby.rooms.length,
    notice: s.notice && s.notice.text, message: s.message && s.message.text, nameError: s.nameError, fullscreen: !!document.fullscreenElement,
    overlay: !online.overlay.hidden, corner: !online.corner.hidden, gameScreen: Module.gameScreen,
    args: window.fakeArgs || null, log: window.fakeEngineLog || [], status: document.getElementById('status').textContent };
});
async function until(p, check, label, timeout = 15000) {
  const deadline = Date.now() + timeout;
  let last;
  while (Date.now() < deadline) {
    last = await ui(p);
    if (check(last)) return last;
    await sleep(100);
  }
  throw new Error(`${p.edition}: ${label} did not happen: ${JSON.stringify(last)}`);
}
const screenIs = (p, screen) => until(p, s => s.screen === screen, 'screen ' + screen);
async function shot(p, name) {
  const file = path.join(output, `${p.edition}-${p.viewport.width}x${p.viewport.height}-${name}.png`);
  await sleep(250);
  const state = await ui(p);
  if (state.fullscreen) await p.page.screenshot({ path: file });
  else await p.page.locator('#screen').screenshot({ path: file });
  results.push({ edition: p.edition, viewport: p.viewport, name, file, screen: state.screen });
}
async function noErrors(...players) {
  for (const p of players) assert.deepEqual(p.errors, [], `${p.edition}: page errors`);
}
async function press(p, ...keys) {
  for (const key of keys) { await p.page.keyboard.press(key); await sleep(60); }
}

async function keyboardFlow(edition, viewport) {
  const host = await player(edition, viewport);
  let s = await ui(host);
  assert.equal(s.screen, 'start');
  assert.equal(s.focus, 'play');
  await shot(host, 'start-loading');
  await press(host, 'ArrowDown');
  await shot(host, 'start-online');
  await press(host, 'Enter');
  s = await until(host, s => s.screen === 'lobby' && s.lobby === 'open' && s.fullscreen, 'lobby open in full screen');
  await shot(host, 'lobby-empty');
  const now = Date.now();
  const fake = [["Zelnick's room", 'c', 5000], ['Ur-Quan Kzer-Za', 'rust', 125000], ['Spathi fan club 2026!!', 'c', 400000], ['חדר של גיא', 'c', 30000]];
  fake.forEach(([name, engine, age], i) => stub.addFakeRoom(edition, { id: token(200 + i), name, engine, created: now - age }));
  s = await until(host, s => s.rooms === 4, 'four rooms listed');
  assert.equal(s.focus, 'room:' + token(200), 'The newest room gets the focus before the player moves');
  await press(host, 'ArrowDown', 'ArrowDown');
  await shot(host, 'lobby-rooms');
  for (let i = 0; i < 14; i++) stub.addFakeRoom(edition, { id: token(210 + i), name: 'Room ' + (i + 1), engine: i % 3 ? 'c' : 'rust', created: now - i * 61000 });
  await until(host, s => s.rooms === 18, 'eighteen rooms listed');
  await press(host, 'End', 'ArrowUp', 'ArrowUp', 'ArrowUp');
  await shot(host, 'lobby-many');
  stub.clear();
  s = await until(host, s => s.rooms === 0, 'rooms removed');
  // Create a room: Right goes to the buttons, Enter opens the name dialog.
  await press(host, 'ArrowRight');
  assert.equal((await ui(host)).focus, 'create');
  await press(host, 'Enter');
  await screenIs(host, 'name');
  await shot(host, 'name');
  await host.page.keyboard.press(process.platform === 'darwin' ? 'Meta+A' : 'Control+A');
  await host.page.keyboard.press('Backspace');
  await press(host, 'Enter');
  s = await until(host, s => s.screen === 'name' && /Type a name/.test(s.nameError), 'name error');
  await shot(host, 'name-error');
  await host.page.keyboard.type("Zelnick's room");
  await press(host, 'Enter');
  await screenIs(host, 'waiting');
  await until(host, () => stub.listings[edition].size === 1, 'room listed');
  const listed = [...stub.listings[edition].values()][0];
  assert.equal(listed.name, "Zelnick's room");
  assert.equal(listed.engine, 'c');
  await shot(host, 'waiting');

  const guest = await player(edition, viewport);
  await press(guest, 'ArrowDown', 'Enter');
  s = await until(guest, s => s.screen === 'lobby' && s.rooms === 1, 'guest sees the room');
  assert.equal(s.focus, 'room:' + listed.id);
  await shot(guest, 'guest-lobby');
  await press(guest, 'Enter');
  await Promise.all([screenIs(host, 'found'), screenIs(guest, 'found')]);
  await Promise.all([shot(host, 'found-host'), shot(guest, 'found-guest')]);
  assert.equal(stub.listings[edition].size, 0, 'A paired room leaves the list');
  // The countdown starts the match: the first session uses --web-netplay.
  s = await until(host, s => s.gameScreen === 'melee', 'host match starts', 20000);
  assert.ok(s.args.includes('--web-netplay=host'), JSON.stringify(s.args));
  assert.equal(s.overlay, false);
  assert.equal(s.corner, false);
  s = await until(guest, s => s.gameScreen === 'melee', 'guest match starts', 20000);
  assert.ok(s.args.includes('--web-netplay=guest'));
  assert.equal(s.fullscreen, true);
  await shot(host, 'match');
  // The guest leaves; the host's engine returns to its main menu and the lobby comes back.
  await guest.context.close();
  s = await until(host, s => s.screen === 'lobby', 'lobby after the session', 20000);
  assert.equal(s.fullscreen, true, 'Still in full screen');
  assert.ok(s.notice, 'The reason is shown');
  await shot(host, 'after-session');
  // Back to the game's main menu: the corner button and the O key.
  await press(host, 'Escape');
  s = await until(host, s => s.screen === 'hidden' && s.corner, 'corner button at the main menu');
  await shot(host, 'main-menu');
  await press(host, 'o');
  await screenIs(host, 'lobby');
  // A second session from the main menu uses the engine hook.
  await press(host, 'ArrowRight', 'Enter');
  await screenIs(host, 'name');
  s = await ui(host);
  await press(host, 'Enter');
  await screenIs(host, 'waiting');
  const second = await player(edition, viewport);
  await press(second, 'ArrowDown', 'Enter');
  await until(second, s => s.screen === 'lobby' && s.rooms === 1, 'second guest sees the room');
  await press(second, 'Enter');
  s = await until(host, s => s.log.includes('hook:0'), 'host starts through the main-menu hook', 20000);
  assert.equal(s.overlay, false, 'nothing covers the match');
  await until(second, s => s.gameScreen === 'melee', 'second guest match starts', 20000);
  await second.context.close();
  await until(host, s => s.screen === 'lobby', 'lobby after the second session', 20000);
  await noErrors(host);
  await host.context.close();
}

async function mouseFlow(edition, viewport) {
  const host = await player(edition, viewport);
  const guest = await player(edition, viewport);
  const click = (p, id) => p.page.locator(`.uqm-online-hit[data-id="${id}"]`).click();
  await click(host, 'online');
  await until(host, s => s.screen === 'lobby' && s.lobby === 'open', 'mouse lobby');
  await click(host, 'create');
  await screenIs(host, 'name');
  await host.page.locator('.uqm-online-hit[data-id="name"]').fill('Mouse room');
  await click(host, 'open');
  await screenIs(host, 'waiting');
  await click(guest, 'online');
  const room = await until(guest, s => s.rooms === 1, 'mouse guest sees the room');
  await guest.page.locator(`.uqm-online-hit[data-id="${room.items[0]}"]`).hover();
  await shot(guest, 'mouse-hover');
  await click(guest, room.items[0]);
  await Promise.all([until(host, s => s.gameScreen === 'melee', 'mouse host match', 20000), until(guest, s => s.gameScreen === 'melee', 'mouse guest match', 20000)]);
  await noErrors(host, guest);
  await host.context.close();
  await guest.context.close();
}

async function otherScreens(edition, viewport) {
  // An invite link: one click joins (and goes full screen).
  const host = await player(edition, viewport);
  await host.page.locator('.uqm-online-hit[data-id="online"]').click();
  await screenIs(host, 'lobby');
  await press(host, 'ArrowRight', 'Enter', 'Enter');
  await screenIs(host, 'waiting');
  const id = [...stub.listings[edition].keys()][0];
  await host.page.locator('.uqm-online-hit[data-id="copy"]').click();
  await until(host, s => /copied|Copy this link/.test(s.notice || ''), 'invite copied');
  await shot(host, 'waiting-copied');
  const invited = await player(edition, viewport, '', '#room=' + id);
  await screenIs(invited, 'invite');
  await shot(invited, 'invite');
  await invited.page.locator('.uqm-online-hit[data-id="join"]').click();
  await until(invited, s => s.screen === 'found' || s.screen === 'hidden', 'invite joins');
  await until(invited, s => s.fullscreen, 'the invite click puts the frame in full screen');
  await noErrors(host, invited);
  await host.context.close();
  await invited.context.close();

  // An engine without the main-menu hook: the lobby offers a reload instead.
  const old = await player(edition, viewport, '&fakeHook=0');
  await until(old, s => s.status.includes('Ready') || s.screen === 'start', 'start');
  await old.page.waitForFunction(() => runtimeReady);
  await press(old, 'Enter');
  await until(old, s => s.gameScreen === 'mainMenu' && s.corner, 'local game main menu');
  await press(old, 'o');
  await screenIs(old, 'restart');
  await shot(old, 'restart');
  await press(old, 'Escape');
  await screenIs(old, 'hidden');
  await noErrors(old);
  await old.context.close();

  // The room list is down: the lobby says so and keeps trying.
  stub.setAvailable(false);
  const down = await player(edition, viewport);
  await press(down, 'ArrowDown', 'Enter');
  await until(down, s => s.lobby === 'error', 'lobby error');
  await shot(down, 'lobby-error');
  stub.setAvailable(true);
  await until(down, s => s.lobby === 'open', 'lobby recovers', 30000);
  await down.page.evaluate(() => fail('Loading failed. Reload to try again. Could not download the game script.'));
  await screenIs(down, 'message');
  await shot(down, 'failed');
  assert.match((await ui(down)).status, /^Unable to start/);
  await down.context.close();
}

try {
  for (const edition of editions) {
    for (const viewport of viewports) {
      await keyboardFlow(edition, viewport);
      console.log(`${edition} ${viewport.width}x${viewport.height}: keyboard flow passed`);
    }
    await mouseFlow(edition, viewports[0]);
    console.log(`${edition}: mouse flow passed`);
    await otherScreens(edition, viewports[0]);
    console.log(`${edition}: invite, reload, lobby error and failure screens passed`);
  }
} finally {
  await writeFile(path.join(output, 'results.json'), JSON.stringify(results, null, 2) + '\n');
  await browser.close();
  await stub.close();
}
console.log(`Online UI flows passed. Screens: ${output}`);
