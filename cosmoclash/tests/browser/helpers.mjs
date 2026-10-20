// Shared parts of the Cosmo Clash browser tests: real Chrome processes, the
// built game at /cosmoclash/ (scripts/cosmo-server.mjs) and a real relay.
import { chromium } from 'playwright';
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
    fullscreen: window.__cosmo?.fullscreen, paused: window.__cosmo?.paused, live: document.getElementById('cosmo-status')?.textContent ?? '' }));
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
