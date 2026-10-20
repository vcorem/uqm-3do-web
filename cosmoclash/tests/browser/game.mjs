// Online fleet matches through the Rust relay, which has no room list: the
// host opens a private room from the game's lobby, the guest opens its
// invite link and joins, both choose fleets with the keyboard, and the
// browsers play with real keys. The full run is a complete three-ship fleet
// match with a rematch; the jitter run adds relay latency and jitter. Both
// end with a disconnect back to the lobby.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
import { root, launch, poll, read, screen, setKeys, botKeys, watchPage, wasmHash } from './helpers.mjs';

const output = path.resolve(root, process.env.COSMO_BROWSER_RESULTS_DIR || 'test-results/browser');
await fs.mkdir(output, { recursive: true });
const expectedHash = await wasmHash();
const scenarios = [];
async function onlineScenario(name, latency, fullMatch) {
  const app = createCosmoServer({ distDir: path.join(root, 'dist'), relay: createRustRelay({ distDir: false, latencyPatternMs: latency }), lobby: 'relay' });
  const address = await app.listen(0);
  const url = `http://127.0.0.1:${address.port}/cosmoclash/`;
  let browsers = [], pages = [];
  const sink = { errors: [], requests: [], sockets: [] };
  const held = [new Set(), new Set()];
  const result = { name, passed: false, latency, fullMatch, wasmSha256: expectedHash };
  try {
    browsers = await Promise.all([launch(), launch()]);
    pages = await Promise.all(browsers.map(b => b.newPage({ viewport: { width: 960, height: 720 } })));
    pages.forEach((page, index) => watchPage(page, index, sink));
    const [host, guest] = pages;
    await host.goto(url);
    await host.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 60000 });
    assert.equal((await read(host)).engine, expectedHash);
    await host.keyboard.press(' ');
    await host.waitForFunction(() => window.__cosmo?.state === 'menu');
    // Online Melee; this relay has no room list, so Create makes a private room with an invite link.
    await host.keyboard.press('ArrowDown'); await host.keyboard.press('ArrowDown'); await host.keyboard.press('Enter');
    await host.waitForFunction(() => window.__cosmo.online.lobby === 'unsupported');
    await host.keyboard.press('Enter');
    await screen(host, 'waiting');
    const invitation = await poll(async () => (await host.locator('.cc-link').textContent()) || null);
    assert.match(invitation, /\/cosmoclash\/\?cosmoRoom=[0-9a-f]{32}$/);
    await guest.goto(invitation);
    await guest.waitForFunction(() => window.__cosmo?.state === 'logo', {}, { timeout: 60000 });
    assert.equal((await read(guest)).engine, expectedHash);
    await guest.keyboard.press(' ');
    await screen(guest, 'invite');
    await guest.keyboard.press('Enter');
    await Promise.all(pages.map(p => screen(p, 'fleet', 15000)));
    // A new fleet screen ignores Enter for 600 ms (a key pressed for the last screen).
    await host.waitForTimeout(700);
    // Gondi first for contact and weapon fire. The full run keeps all three ships
    // (Gondi, Sholev, MichiGichi); the jitter run uses Gondi alone.
    for (const page of pages) {
      await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter');
      if (fullMatch) { await page.keyboard.press('Enter'); await page.keyboard.press('Enter'); }
      else await page.keyboard.press('ArrowDown');
      assert.deepEqual((await read(page)).online.picks, fullMatch ? [1, 2, 0] : [1]);
      await page.keyboard.press('Enter');
    }
    await Promise.all(pages.map(p => screen(p, 'battle', 20000)));
    const initial = await Promise.all(pages.map(read));
    const initialPositions = initial.map(s => s.snapshot.players.map(p => ({ ...p.position })));
    const started = Date.now(); let progressAt = started; const seenShips = new Set(); let states; let shots = 0, motion = false, maxChecks = 0, maxBattle = 0;
    while (Date.now() - started < (fullMatch ? (process.env.CI ? 480000 : 300000) : 30000)) {
      states = await Promise.all(pages.map(read));
      for (const [index, s] of states.entries()) {
        assert.ok(!['error', 'closed'].includes(s.net.phase), JSON.stringify(s.net)); assert.ok(s.snapshot);
        shots = Math.max(shots, s.snapshot.bullets.length); maxChecks = Math.max(maxChecks, s.net.verifiedChecksums); maxBattle = Math.max(maxBattle, s.fleet.battle);
        s.snapshot.players.forEach(p => seenShips.add(p.shipId));
        motion ||= s.snapshot.players.some((p, i) => Math.hypot(p.position.x - initialPositions[index][i].x, p.position.y - initialPositions[index][i].y) > 10);
      }
      if (Date.now() - progressAt > 10000) { progressAt = Date.now(); console.log(JSON.stringify({ scenario: name, elapsedMs: progressAt - started, ticks: states.map(s => s.net.tick), battles: states.map(s => s.fleet.battle), checksums: states.map(s => s.net.verifiedChecksums) })); }
      if (states.every(s => s.net.phase === 'ended')) break;
      await Promise.all(pages.map((p, i) => setKeys(p, states[i].net.phase === 'playing' ? botKeys(states[i].snapshot, i) : new Set(), held[i])));
      if (!fullMatch && maxChecks >= 8) break;
      await new Promise(r => setTimeout(r, 150));
    }
    states = await Promise.all(pages.map(read)); assert.ok(motion, 'Actual ships must move'); assert.ok(shots > 0, 'Actual weapons must spawn'); assert.ok(states.every(s => s.net.verifiedChecksums >= 5), 'Both engines must verify state repeatedly');
    await Promise.all(pages.map((p, i) => p.screenshot({ path: path.join(output, `${name}-${i}.png`) })));
    Object.assign(result, { elapsedMs: Date.now() - started, seenShips: [...seenShips].sort(), frames: states.map(s => s.net.tick), verifiedChecksums: states.map(s => s.net.verifiedChecksums), maxShots: shots, motion, battleTransitions: maxBattle });
    if (fullMatch) {
      assert.ok(states.every(s => s.net.phase === 'ended'), 'Full fleet match must finish naturally'); assert.ok(maxBattle >= 2, 'Losing fleet must advance through all3ships'); assert.deepEqual([...seenShips].sort(), ['gondi', 'michiGichi', 'sholev']);
      await Promise.all(pages.map((p, i) => setKeys(p, new Set(), held[i])));
      await Promise.all(pages.map(p => screen(p, 'result')));
      // Rematch: the host asks, the guest accepts; the last fleets stay and Enter says Ready
      // (the result and fleet screens ignore Enter for their first 600 ms).
      await host.waitForTimeout(700);
      await host.keyboard.press('Enter');
      await guest.waitForFunction(() => document.querySelector('[data-item="rematch"]')?.textContent === 'Accept rematch');
      await guest.keyboard.press('Enter');
      await Promise.all(pages.map(p => p.waitForFunction(() => window.__cosmo?.online?.screen === 'fleet' && window.__cosmo.netplay.round === 1, {}, { timeout: 15000 })));
      await host.waitForTimeout(700);
      for (const page of pages) await page.keyboard.press('Enter');
      await Promise.all(pages.map(p => p.waitForFunction(() => window.__cosmo?.netplay?.phase === 'playing' && window.__cosmo.netplay.round === 1, {}, { timeout: 15000 })));
      await poll(async () => (await Promise.all(pages.map(read))).every(s => s.net.verifiedChecksums >= 3), 20000, 'rematch checksums'); result.rematch = true;
    }
    await Promise.all(pages.map((p, i) => setKeys(p, new Set(), held[i])));
    // The guest leaves (Esc, then Leave); the host is back in the lobby with the reason.
    await guest.keyboard.press('Escape'); await guest.keyboard.press('ArrowRight'); await guest.keyboard.press('Enter');
    await host.waitForFunction(() => window.__cosmo?.online?.screen === 'lobby' && window.__cosmo.online.notice === 'Your opponent left the room.', {}, { timeout: 10000 });
    await host.keyboard.press('Escape');
    await guest.keyboard.press('Escape');
    await Promise.all(pages.map(p => p.waitForFunction(() => window.__cosmo?.state === 'menu', {}, { timeout: 10000 })));
    result.disconnect = true;
    const rooms = sink.sockets.filter(s => s.startsWith('/netplay/cosmoclash/'));
    assert.deepEqual(sink.errors, []); assert.deepEqual(sink.requests, []);
    assert.equal(rooms.length, 2, JSON.stringify(sink.sockets));
    assert.ok(rooms.every(s => /^\/netplay\/cosmoclash\/[0-9a-f]{32}\?role=(host|guest)$/.test(s)));
    result.socketPaths = sink.sockets; result.passed = true;
  } catch (error) {
    result.error = String(error); result.errors = sink.errors; result.requests = sink.requests;
    result.state = await Promise.all(pages.map(p => read(p).catch(() => null)));
    await fs.writeFile(path.join(output, `${name}-failure.json`), JSON.stringify(result, null, 2));
    throw error;
  } finally {
    scenarios.push(result); await fs.writeFile(path.join(output, 'results.json'), JSON.stringify({ passed: scenarios.every(s => s.passed), scenarios }, null, 2));
    await Promise.all(browsers.map(b => b.close())); await app.close();
  }
  console.log(JSON.stringify(result));
}
await onlineScenario('normal', [0], !process.env.COSMO_QUICK);
if (!process.env.COSMO_QUICK) await onlineScenario('jitter', [0, 30, 5, 70, 15], false);
