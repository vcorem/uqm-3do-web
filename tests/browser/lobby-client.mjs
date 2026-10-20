// The room-list client API (sc2/wasm/netplay-client.js) in real browsers against
// the Node relay: one player watches the HD room list, a second player creates a
// named room, the first picks it from the list, and the two exchange game data.
// No game build is needed. Screenshots and a report go to UQM_TEST_OUTPUT.
import assert from 'node:assert/strict';
import { copyFile, mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { chromium, firefox, webkit } from 'playwright';
import { createNetplayServer } from '../../server/netplay.mjs';

const out = path.resolve(process.env.UQM_TEST_OUTPUT || 'test-results/lobby-client');
const types = { chromium, firefox, webkit };
const browsers = (process.env.UQM_BROWSERS || 'chromium,firefox,webkit').split(',');
for (const name of browsers) assert.ok(types[name], 'Unsupported browser ' + name);
await mkdir(out, { recursive: true });

// A test page, not the game: it only shows what the client API reports.
const site = await mkdtemp(path.join(os.tmpdir(), 'uqm-lobby-client-'));
await mkdir(path.join(site, 'hd'));
await copyFile(new URL('../../sc2/wasm/netplay-client.js', import.meta.url), path.join(site, 'hd/netplay-client.js'));
await writeFile(path.join(site, 'hd/index.html'), `<!doctype html>
<meta charset="utf-8"><title>Room list client test</title>
<style>
  body { font: 16px system-ui, sans-serif; background: #050816; color: #e8f0ff; margin: 24px; }
  h1 { font-size: 18px; } #status { color: #9fd3ff; } ul { list-style: none; padding: 0; }
  li { border: 1px solid #3a5a9a; border-radius: 6px; padding: 10px 14px; margin: 8px 0; cursor: pointer; max-width: 420px; }
  li small { color: #9aa8c8; margin-inline-start: 8px; } #room { color: #ffe08a; }
</style>
<h1>HD room list (client API test page)</h1>
<p id="status">Not started</p>
<ul id="rooms"></ul>
<p id="room"></p>
<script src="netplay-client.js"></script>`);

const server = createNetplayServer({ distDir: site });
const address = await server.listen(0, '127.0.0.1');
const origin = `http://127.0.0.1:${address.port}`;
const report = { browsers: {}, passed: false };

async function run(name) {
  const result = { errors: [] };
  report.browsers[name] = result;
  const browser = await types[name].launch();
  try {
    const [guestPage, hostPage] = await Promise.all([1, 2].map(async () => {
      const page = await (await browser.newContext({ viewport: { width: 640, height: 420 } })).newPage();
      page.on('pageerror', error => result.errors.push(String(error)));
      await page.goto(origin + '/hd/index.html');
      return page;
    }));
    // The guest watches the HD list and joins a room by clicking it.
    await guestPage.evaluate(() => {
      const list = document.getElementById('rooms');
      window.statuses = [];
      window.watcher = UQMNetplay.watchLobby({ edition: 'hd',
        onStatus(state, message) { window.statuses.push(state); document.getElementById('status').textContent = state + ': ' + message; },
        onRooms(rooms) {
          list.replaceChildren(...rooms.map(room => {
            const item = document.createElement('li');
            item.textContent = room.name;
            const engine = document.createElement('small');
            engine.textContent = room.engine === 'rust' ? 'Rust engine' : room.engine === 'c' ? 'C engine' : '';
            item.append(engine);
            item.addEventListener('click', () => {
              window.room = UQMNetplay.connect({ token: room.id, role: 'guest', edition: 'hd',
                onStatus(status) { document.getElementById('room').textContent = status.state + ': ' + status.message; } });
            });
            return item;
          }));
        } });
    });
    await guestPage.waitForFunction(() => window.watcher.state === 'open');
    // The host also watches the Original list, which must stay empty.
    const roomName = 'Friday duel ⚔ קרב';
    await hostPage.evaluate(name => {
      window.originalRooms = [];
      window.original = UQMNetplay.watchLobby({ edition: 'original', onRooms(rooms) { window.originalRooms.push(rooms.length); } });
      try { UQMNetplay.connect({ token: UQMNetplay.createToken(), role: 'host', edition: 'hd', name: 'x'.repeat(25) }); }
      catch (error) { window.nameError = error.message; }
      // A blank filler (U+3164) is refused in every browser, like on the relay.
      try { UQMNetplay.roomName(String.fromCodePoint(0x3164)); } catch (error) { window.fillerError = error.message; }
      // The listed name has one space between words.
      window.room = UQMNetplay.connect({ token: UQMNetplay.createToken(), role: 'host', edition: 'hd', name: '  ' + name.replace(' ', '   ') + '  ',
        engine: 'rust', onStatus(status) { document.getElementById('room').textContent = status.state + ': ' + status.message; } });
    }, roomName);
    assert.equal(await hostPage.evaluate(() => window.nameError), 'Room names need 1 to 24 visible characters.');
    assert.equal(await hostPage.evaluate(() => window.fillerError), 'Room names need 1 to 24 visible characters.');
    const item = guestPage.locator('#rooms li', { hasText: roomName });
    await item.waitFor({ timeout: 10000 });
    assert.equal(await item.locator('small').textContent(), 'Rust engine');
    assert.equal(await item.evaluate(li => li.firstChild.nodeValue), roomName, 'the relay lists the cleaned name');
    await guestPage.screenshot({ path: path.join(out, `${name}-1-room-listed.png`) });
    await hostPage.screenshot({ path: path.join(out, `${name}-1-host-waiting.png`) });
    assert.match(await hostPage.locator('#room').textContent(), /is open/);
    await item.click();
    await Promise.all([guestPage, hostPage].map(page => page.waitForFunction(() => window.room?.state === 'ready')));
    await guestPage.waitForFunction(() => document.querySelectorAll('#rooms li').length === 0, null, { timeout: 5000 });
    await hostPage.evaluate(() => window.room.send(new Uint8Array([1, 2, 3])));
    await guestPage.waitForFunction(() => window.room.incoming.length === 1);
    assert.deepEqual(await guestPage.evaluate(() => Array.from(window.room.incoming[0])), [1, 2, 3]);
    await guestPage.screenshot({ path: path.join(out, `${name}-2-joined.png`) });
    assert.deepEqual([...new Set(await hostPage.evaluate(() => window.originalRooms))], [0], 'the Original list stays empty');
    // Leaving the list closes its socket for good.
    await guestPage.evaluate(() => window.watcher.close());
    assert.equal(await guestPage.evaluate(() => window.statuses.at(-1)), 'closed');
    assert.deepEqual(result.errors, []);
    result.passed = true;
  } finally {
    await browser.close();
  }
}

try {
  for (const name of browsers) await run(name);
  report.passed = true;
} finally {
  await writeFile(path.join(out, 'report.json'), JSON.stringify(report, null, 2) + '\n');
  await server.close();
  await rm(site, { recursive: true, force: true });
}
console.log(JSON.stringify(report));
