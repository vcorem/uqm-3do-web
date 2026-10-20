// Shared parts of the Cosmo Clash browser tests: real Chrome processes, the
// built game at /cosmoclash/ (scripts/cosmo-server.mjs) and a real relay.
import { chromium } from 'playwright';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

export const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
export const executablePath = process.env.CHROME_BIN || (process.platform === 'darwin' ? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' : undefined);
export async function wasmHash() {
  return createHash('sha256').update(await fs.readFile(path.join(root, 'dist/cosmoclash.wasm'))).digest('hex');
}
export function launch() {
  return chromium.launch({ executablePath, headless: true, args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader',
    '--disable-background-timer-throttling', '--disable-backgrounding-occluded-windows', '--disable-renderer-backgrounding'] });
}
export async function poll(fn, timeout = 20000, label = 'condition') {
  const end = Date.now() + timeout;
  let last;
  while (Date.now() < end) { last = await fn(); if (last) return last; await new Promise(r => setTimeout(r, 100)); }
  throw new Error(`Timed out after ${timeout} ms waiting for ${label}`);
}
export async function read(page) {
  return page.evaluate(() => ({ state: window.__cosmo?.state, mode: window.__cosmo?.mode, net: window.__cosmo?.netplay, snapshot: window.__cosmo?.snapshot,
    frames: window.__cosmo?.frames, fleet: window.__cosmo?.fleet, engine: window.__cosmo?.engine, online: window.__cosmo?.online,
    fullscreen: window.__cosmo?.fullscreen, paused: window.__cosmo?.paused, hash: window.__cosmo?.hash, pick: window.__cosmo?.pick,
    live: document.getElementById('cosmo-status')?.textContent ?? '' }));
}
export const screen = (page, name, timeout = 20000) => page.waitForFunction(value => window.__cosmo?.online?.screen === value, name, { timeout });
export async function setKeys(page, keys, held) {
  for (const key of [...held]) if (!keys.has(key)) { await page.keyboard.up(key); held.delete(key); }
  for (const key of keys) if (!held.has(key)) { await page.keyboard.down(key); held.add(key); }
}
/** An aiming player: real keys only (the arrows, / and .). */
export function botKeys(snapshot, player) {
  const self = snapshot.players[player], enemy = snapshot.players[1 - player];
  const dx = enemy.position.x - self.position.x, dy = enemy.position.y - self.position.y;
  const desired = (Math.atan2(-dx, dy) * 180 / Math.PI + 360) % 360;
  const diff = ((desired - self.angle + 540) % 360) - 180;
  const keys = new Set(['/']);
  if (diff > 4) keys.add('ArrowRight'); else if (diff < -4) keys.add('ArrowLeft');
  if (Math.abs(diff) < 45) keys.add('ArrowUp');
  if (self.shipId === 'sholev' && !self.frostShieldActive && self.energy > 0.5) keys.add('.');
  return keys;
}
/** Collects page errors, failed requests and WebSocket paths of a page. */
export function watchPage(page, index, sink) {
  page.on('pageerror', error => sink.errors.push({ index, message: error.message }));
  page.on('response', response => { if (response.status() >= 400 && !response.url().endsWith('favicon.ico')) sink.requests.push({ url: response.url(), status: response.status() }); });
  page.on('websocket', socket => sink.sockets.push(new URL(socket.url()).pathname + new URL(socket.url()).search));
}
/** Records navigator.keyboard.lock calls (the game asks Chromium to keep Esc). */
export const recordKeyboardLock = () => {
  window.__keyboardLock = [];
  const keyboard = navigator.keyboard;
  if (keyboard && keyboard.lock) {
    const lock = keyboard.lock.bind(keyboard);
    keyboard.lock = keys => { window.__keyboardLock.push(keys); return lock(keys).catch(() => {}); };
  }
};

// ---------------------------------------------------------------- the next ship (screenshots and checks)
export const SIZES = [[1280, 720], [1024, 768], [1920, 1080]];
/**
 * The screens of the choice of the next ship (and the screens it changed) go to
 * test-results/next-ship/ from all three browser tests. Each test removes only
 * its own files there (its prefix).
 */
export async function nextShipShots(prefix) {
  const dir = path.resolve(root, process.env.COSMO_NEXT_SHIP_DIR || 'test-results/next-ship');
  await fs.mkdir(dir, { recursive: true });
  for (const name of await fs.readdir(dir)) if (name.startsWith(prefix + '-')) await fs.rm(path.join(dir, name), { force: true });
  let counter = 0;
  const files = [];
  /** Saves the page at the three sizes (check runs at each size first) and goes back to the first size. */
  async function save(page, name, { sizes = SIZES, check, back = [1280, 720] } = {}) {
    const n = String(++counter).padStart(2, '0');
    for (const [width, height] of sizes) {
      await page.setViewportSize({ width, height });
      await page.waitForTimeout(150);
      assert.deepEqual(await textOutside(page), [], `text outside the game frame (${name} ${width}x${height})`);
      if (check) await check(width, height);
      const file = path.join(dir, `${prefix}-${n}-${name}-${width}x${height}.png`);
      await page.screenshot({ path: file });
      files.push(path.relative(root, file));
    }
    await page.setViewportSize({ width: back[0], height: back[1] });
  }
  /** Copies a screenshot that a test saved anyway (name without the size). */
  async function add(file, name, size) {
    const target = path.join(dir, `${prefix}-${String(++counter).padStart(2, '0')}-${name}-${size}.png`);
    await fs.copyFile(file, target);
    files.push(path.relative(root, target));
  }
  return { dir, files, save, add };
}
/** The visible texts of the game that are outside the game frame (clipped or overflowing text). */
export function textOutside(page) {
  return page.evaluate(() => {
    const frame = document.getElementById('game-container').getBoundingClientRect();
    const bad = [];
    for (const node of document.querySelectorAll('#game-container *')) {
      if (!node.childNodes.length || ![...node.childNodes].some(child => child.nodeType === 3 && child.textContent.trim())) continue;
      const box = node.getBoundingClientRect();
      if (!box.width || !box.height || getComputedStyle(node).visibility === 'hidden') continue;
      if (box.left < frame.left - 1 || box.right > frame.right + 1 || box.top < frame.top - 1 || box.bottom > frame.bottom + 1) bad.push(node.textContent.trim().slice(0, 60));
    }
    return bad;
  });
}
/**
 * Over the 3D ship choice the ship sits between the ship card and the panel
 * under it: the space between them is at least this part of the frame height.
 */
export async function shipSpace(page, top = '.cc-ship', bottom = '.cc-fleet-bottom') {
  const space = await page.evaluate(([a, b]) => {
    const frame = document.getElementById('game-container').getBoundingClientRect();
    const card = document.querySelector(a)?.getBoundingClientRect(), panel = document.querySelector(b)?.getBoundingClientRect();
    return card && panel ? (panel.top - card.bottom) / frame.height : -1;
  }, [top, bottom]);
  assert.ok(space >= 0.22, `the ship has room between the card and the panel: ${space.toFixed(3)} of the frame height`);
  return space;
}
/**
 * Waits for a game condition. A slow, busy computer runs the game slower than
 * real time, so the limit counts only the time in which the game stops (no new
 * frames for stallMs while the condition is not met), with a total limit.
 */
export async function waitGame(page, condition, { label = 'condition', arg, stallMs = 90000, maxMs = 1200000, every = 250 } = {}) {
  const start = Date.now();
  let frames = -1, movedAt = Date.now();
  for (;;) {
    const result = await page.evaluate(condition, arg);
    if (result) return result;
    const now = await page.evaluate(() => window.__cosmo?.frames ?? 0);
    if (now !== frames) { frames = now; movedAt = Date.now(); }
    if (Date.now() - movedAt > stallMs) throw new Error(`The game stopped (no frame for ${stallMs} ms) while waiting for ${label}`);
    if (Date.now() - start > maxMs) throw new Error(`Timed out after ${maxMs} ms waiting for ${label}`);
    await new Promise(r => setTimeout(r, every));
  }
}
