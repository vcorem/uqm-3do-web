// Static files of the Cloudflare site (deploy/worker.js) under `wrangler dev`:
// Cosmo Clash at /cosmoclash/ next to the UQM editions, with the content types,
// the cache rules and the cross-origin isolation headers. Run with `npm run test:worker`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { startWorker, useDataManifest } from './wrangler-dev.mjs';

useDataManifest();

const ROOM = '0123456789abcdef0123456789abcdef';
const COSMO_PAGE = '<!doctype html><script type="module" src="/cosmoclash/assets/index-AbCd_123.js"></script>';
const WASM = Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]);
const FILES = {
  'original/index.html': 'original page',
  'cosmoclash/index.html': COSMO_PAGE,
  'cosmoclash/assets/index-AbCd_123.js': 'export {};',
  'cosmoclash/assets/index-Xy9-Zw78.css': 'body{}',
  'cosmoclash/assets/notes.js': 'export {};',
  'cosmoclash/cosmoclash.wasm': WASM,
  'cosmoclash/models/gondi.glb': 'glTF',
  'cosmoclash/sounds/Shoot-3.ogg': 'OggS',
  'cosmoclash/sounds/bigang.wav': 'RIFF',
  'cosmoclash/textures/heart.png': 'png',
  'cosmoclash/textures/main-bk.jpg': 'jpg',
};
const ISOLATION = {
  'cross-origin-opener-policy': 'same-origin',
  'cross-origin-embedder-policy': 'require-corp',
  'cross-origin-resource-policy': 'same-origin',
  'x-content-type-options': 'nosniff',
};
const IMMUTABLE = 'public, max-age=31536000, immutable';

let shared;
const cleanups = [];
test.after(() => Promise.all(cleanups.map(cleanup => cleanup())));
async function site() {
  shared ??= startWorker({ after: cleanup => cleanups.push(cleanup) }, {}, FILES);
  return (await shared).origin;
}
function isolated(response, label) {
  for (const [name, value] of Object.entries(ISOLATION)) assert.equal(response.headers.get(name), value, `${label}: ${name}`);
}

test('/cosmoclash redirects to /cosmoclash/ and keeps an invitation', async () => {
  const origin = await site();
  for (const [from, to] of [['/cosmoclash', '/cosmoclash/'], [`/cosmoclash?cosmoRoom=${ROOM}`, `/cosmoclash/?cosmoRoom=${ROOM}`]]) {
    const response = await fetch(origin + from, { redirect: 'manual' });
    assert.equal(response.status, 301, from);
    assert.equal(response.headers.get('location'), origin + to, from);
    isolated(response, from);
  }
});

test('the Cosmo Clash page is HTML that the browser checks again on each visit, with the isolation headers', async () => {
  const origin = await site();
  for (const url of ['/cosmoclash/', `/cosmoclash/?cosmoRoom=${ROOM}`, '/cosmoclash/index.html']) {
    const response = await fetch(origin + url);
    assert.equal(response.status, 200, url);
    assert.equal(await response.text(), COSMO_PAGE, url);
    assert.equal(response.headers.get('content-type'), 'text/html; charset=utf-8', url);
    assert.equal(response.headers.get('cache-control'), 'no-cache', url);
    isolated(response, url);
  }
});

test('hashed Vite files are immutable; files without a hash are checked again', async () => {
  const origin = await site();
  for (const [url, type, cache] of [
    ['/cosmoclash/assets/index-AbCd_123.js', 'text/javascript; charset=utf-8', IMMUTABLE],
    ['/cosmoclash/assets/index-Xy9-Zw78.css', 'text/css; charset=utf-8', IMMUTABLE],
    ['/cosmoclash/assets/notes.js', 'text/javascript; charset=utf-8', 'no-cache'],
    ['/cosmoclash/cosmoclash.wasm', 'application/wasm', 'no-cache'],
    ['/cosmoclash/models/gondi.glb', 'model/gltf-binary', 'no-cache'],
    ['/cosmoclash/sounds/Shoot-3.ogg', 'audio/ogg', 'no-cache'],
    ['/cosmoclash/sounds/bigang.wav', 'audio/wav', 'no-cache'],
    ['/cosmoclash/textures/heart.png', 'image/png', 'no-cache'],
    ['/cosmoclash/textures/main-bk.jpg', 'image/jpeg', 'no-cache'],
  ]) {
    const response = await fetch(origin + url);
    assert.equal(response.status, 200, url);
    assert.equal(response.headers.get('content-type'), type, url);
    assert.equal(response.headers.get('cache-control'), cache, url);
    isolated(response, url);
    await response.arrayBuffer();
  }
  const wasm = await fetch(origin + '/cosmoclash/cosmoclash.wasm');
  assert.deepEqual(Buffer.from(await wasm.arrayBuffer()), WASM);
});

test('the Wasm has an ETag: a repeat visit gets 304 with the same headers; HEAD has no body', async () => {
  const origin = await site();
  const first = await fetch(origin + '/cosmoclash/cosmoclash.wasm');
  await first.arrayBuffer();
  const etag = first.headers.get('etag');
  assert.ok(etag, 'the asset store sends an ETag');
  const again = await fetch(origin + '/cosmoclash/cosmoclash.wasm', { headers: { 'If-None-Match': etag } });
  assert.equal(again.status, 304);
  assert.equal(again.headers.get('cache-control'), 'no-cache');
  isolated(again, '304');
  const head = await fetch(origin + '/cosmoclash/cosmoclash.wasm', { method: 'HEAD' });
  assert.equal(head.status, 200);
  assert.equal(head.headers.get('content-type'), 'application/wasm');
  assert.equal((await head.arrayBuffer()).byteLength, 0);
});

test('missing Cosmo Clash files are 404, other methods 405, and the UQM pages keep their rules', async () => {
  const origin = await site();
  for (const url of ['/cosmoclash/missing.js', '/cosmoclash/assets/', '/cosmoclash/assets', '/cosmoclashx/', '/cosmoclash/index.htm']) {
    const response = await fetch(origin + url, { redirect: 'manual' });
    assert.equal(response.status, 404, url);
    isolated(response, url);
    await response.arrayBuffer();
  }
  const post = await fetch(origin + '/cosmoclash/', { method: 'POST' });
  assert.equal(post.status, 405);
  assert.equal(post.headers.get('allow'), 'GET, HEAD');
  for (const [url, body] of [['/', '<canvas>game</canvas>'], ['/original/', 'original page']]) {
    const response = await fetch(origin + url);
    assert.equal(response.status, 200, url);
    assert.equal(await response.text(), body, url);
    assert.equal(response.headers.get('content-type'), 'text/html; charset=utf-8', url);
    assert.notEqual(response.headers.get('cache-control'), IMMUTABLE, url);
    isolated(response, url);
  }
});
