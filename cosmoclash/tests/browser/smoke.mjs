// A short look at a real battle: Play vs Computer at /cosmoclash/.
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch } from './helpers.mjs';
const app = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createRustRelay({ distDir: false }), lobby: 'relay' });
const address = await app.listen(0); let browser;
try {
  browser = await launch();
  const page = await browser.newPage({ viewport: { width: 1280, height: 960 } }); const errors = []; const failed = [];
  page.on('pageerror', e => errors.push(e.message)); page.on('response', r => { if (r.status() >= 400) failed.push({ url: r.url(), status: r.status() }); });
  await page.goto(`http://127.0.0.1:${address.port}/cosmoclash/`);
  await page.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 60000 });
  await page.keyboard.press(' ');
  await page.waitForFunction(() => window.__cosmo?.state === 'menu', {}, { timeout: 60000 });
  await page.getByRole('menuitem', { name: 'Play vs Computer', exact: true }).click();
  await page.getByText('Easy', { exact: true }).click();
  await page.keyboard.press('Enter');
  await page.waitForFunction(() => window.__cosmo?.state === 'playing', {}, { timeout: 10000 });
  await page.keyboard.down('ArrowUp'); await page.keyboard.down('/'); await page.waitForTimeout(3500); await page.keyboard.up('ArrowUp'); await page.keyboard.up('/');
  await fs.mkdir(path.join(root, 'test-results/browser'), { recursive: true });
  await page.screenshot({ path: path.join(root, 'test-results/browser/solo.png') });
  console.log(JSON.stringify({ status: await page.evaluate(() => ({ state: window.__cosmo.state, engine: window.__cosmo.engine, frames: window.__cosmo.frames, players: window.__cosmo.snapshot?.players.map(p => ({ ship: p.shipId, position: p.position, health: p.health, energy: p.energy })), bullets: window.__cosmo.snapshot?.bullets.length })), errors, failed }, null, 2));
  if (errors.length || failed.some(r => !r.url.endsWith('favicon.ico'))) throw new Error('Browser smoke failed');
} finally { await browser?.close(); await app.close(); }
