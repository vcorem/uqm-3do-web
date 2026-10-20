// scripts/serve.py names each file's version (SHA-256 ETag, as the Cloudflare Worker
// does), so the engine pages keep their game data between local visits too.
const test = require('node:test');
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const net = require('node:net');
const os = require('node:os');
const path = require('node:path');
const { spawn } = require('node:child_process');

function freePort() {
    return new Promise((resolve, reject) => {
        const server = net.createServer();
        server.once('error', reject);
        server.listen(0, '127.0.0.1', () => {
            const { port } = server.address();
            server.close(() => resolve(port));
        });
    });
}

const sha = bytes => '"' + crypto.createHash('sha256').update(bytes).digest('hex') + '"';

test('the preview server sends SHA-256 ETags and answers a matching If-None-Match with 304', async t => {
    const site = fs.mkdtempSync(path.join(os.tmpdir(), 'uqm-serve-'));
    t.after(() => fs.rmSync(site, { recursive: true, force: true }));
    fs.writeFileSync(path.join(site, 'index.html'), '<title>The Ur-Quan Masters</title>');
    fs.mkdirSync(path.join(site, 'hd'));
    const data = crypto.randomBytes(3 * 1024 * 1024 + 7);
    const file = path.join(site, 'hd', 'uqm-debug.data');
    fs.writeFileSync(file, data);
    const port = await freePort();
    const child = spawn('python3', [path.join(__dirname, '../../scripts/serve.py'), '--port', String(port), '--directory', site],
        { stdio: ['ignore', 'pipe', 'pipe'] });
    t.after(() => child.kill());
    let output = '';
    child.stdout.on('data', chunk => { output += chunk; });
    child.stderr.on('data', chunk => { output += chunk; });
    const origin = `http://127.0.0.1:${port}`;
    for (let tries = 0; ; tries++) {
        try { await fetch(origin + '/index.html'); break; } catch (error) {
            if (tries > 100 || child.exitCode !== null) throw new Error('serve.py did not start: ' + output);
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    }
    const url = origin + '/hd/uqm-debug.data';
    const head = await fetch(url, { method: 'HEAD' });
    assert.equal(head.status, 200);
    assert.equal(head.headers.get('ETag'), sha(data));
    assert.equal(head.headers.get('Content-Length'), String(data.length));
    assert.equal(head.headers.get('Cache-Control'), 'no-cache');
    assert.equal(head.headers.get('Cross-Origin-Embedder-Policy'), 'require-corp');
    const same = await fetch(url, { headers: { 'If-None-Match': sha(data) } });
    assert.equal(same.status, 304);
    assert.equal(same.headers.get('ETag'), sha(data));
    assert.equal((await same.arrayBuffer()).byteLength, 0);
    const other = await fetch(url, { headers: { 'If-None-Match': '"older", "other"' } });
    assert.equal(other.status, 200);
    assert.deepEqual(Buffer.from(await other.arrayBuffer()), data);
    // A rebuild changes the version.
    const rebuilt = Buffer.concat([data, Buffer.from('x')]);
    fs.writeFileSync(file, rebuilt);
    fs.utimesSync(file, new Date(), new Date(Date.now() + 5000));
    assert.equal((await fetch(url, { method: 'HEAD' })).headers.get('ETag'), sha(rebuilt));
    // Missing files and folders carry no ETag.
    const missing = await fetch(origin + '/hd/missing.data');
    assert.equal(missing.status, 404);
    assert.equal(missing.headers.get('ETag'), null);
    assert.equal((await fetch(origin + '/hd', { redirect: 'manual' })).headers.get('ETag'), null);
});
