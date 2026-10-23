// Cloudflare Worker for urquanmasters.org: static game files with the
// cross-origin isolation headers, the large data files joined from parts, and
// the Online Melee relay and its room lists (deploy/netplay.js). The site has
// four games: UQM Original, HD and Melee Remastered (C and Rust engines each; Melee
// is the HD engine with its Melee-only page) and Cosmo Clash at /cosmoclash/.
import dataFiles from './data-parts.json';
import { handleNetplay } from './netplay.js';

export { NetplayRoom, NetplayGate, NetplayLobby } from './netplay.js';

const CANONICAL_HOST = 'urquanmasters.org';
const ISOLATION = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
  'Cross-Origin-Resource-Policy': 'same-origin',
  'X-Content-Type-Options': 'nosniff',
};
const INDEX = { '/': '/index.html' };
for (const folder of ['original', 'hd', 'original-rust', 'hd-rust', 'melee', 'melee-rust', 'cosmoclash']) INDEX['/' + folder + '/'] = '/' + folder + '/index.html';
const PART = /\.data\.p\d{3}$/;
// Content types by file extension. The asset store has its own table; this one
// makes the types the same as those of the local servers (server/netplay.mjs).
const TYPES = {
  html: 'text/html; charset=utf-8', js: 'text/javascript; charset=utf-8', mjs: 'text/javascript; charset=utf-8',
  css: 'text/css; charset=utf-8', json: 'application/json', wasm: 'application/wasm',
  data: 'application/octet-stream', txt: 'text/plain; charset=utf-8', glb: 'model/gltf-binary',
  ogg: 'audio/ogg', wav: 'audio/wav', png: 'image/png', jpg: 'image/jpeg', jpeg: 'image/jpeg',
  webp: 'image/webp', svg: 'image/svg+xml', ico: 'image/x-icon',
};
// Cosmo Clash is a Vite build (base /cosmoclash/). Vite names the files in
// assets/ after their content hash, so a browser can keep them. All other files
// (the page, cosmoclash.wasm, the models, sounds and textures) keep their names
// from build to build: the browser asks again each time (ETag, 304). Both
// online players must run the same Wasm.
const COSMO = '/cosmoclash/';
const COSMO_HASHED = /^\/cosmoclash\/assets\/[^/]+-[A-Za-z0-9_-]{8,}\.[a-z0-9]+$/;
const IMMUTABLE = 'public, max-age=31536000, immutable';

export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    // The game needs a secure context, and the bare domain is the only origin.
    if (url.hostname.endsWith(CANONICAL_HOST) && (url.protocol !== 'https:' || url.hostname !== CANONICAL_HOST)) {
      url.protocol = 'https:';
      url.hostname = CANONICAL_HOST;
      return Response.redirect(url.toString(), 301);
    }
    // The relay answers with the room object's own response: a 101 upgrade
    // cannot be copied into a new Response without losing its WebSocket.
    if (url.pathname.startsWith('/netplay/')) return handleNetplay(request, env);
    if (request.method !== 'GET' && request.method !== 'HEAD') {
      return isolated(new Response('Method not allowed', { status: 405, headers: { Allow: 'GET, HEAD' } }));
    }
    if (PART.test(url.pathname)) return isolated(new Response('Not found', { status: 404 }));
    // The game page's relative URLs (cosmoclash.wasm) need the folder form; an invitation query stays.
    if (url.pathname === COSMO.slice(0, -1)) {
      url.pathname = COSMO;
      return isolated(Response.redirect(url.toString(), 301));
    }
    const entry = dataFiles[url.pathname];
    if (entry) return isolated(joinParts(request, env, entry));
    if (INDEX[url.pathname]) url.pathname = INDEX[url.pathname];
    return isolated(typed(url.pathname, await env.ASSETS.fetch(new Request(url, request))));
  },
};

/** Sets the content type, and the cache rule of Cosmo Clash files, on a found file. */
function typed(pathname, response) {
  if (response.status !== 200 && response.status !== 304) return response;
  const result = new Response(response.body, response);
  const type = TYPES[pathname.slice(pathname.lastIndexOf('.') + 1)];
  if (type) result.headers.set('Content-Type', type);
  if (pathname.startsWith(COSMO)) result.headers.set('Cache-Control', COSMO_HASHED.test(pathname) ? IMMUTABLE : 'no-cache');
  return result;
}

function joinParts(request, env, entry) {
  const etag = '"' + entry.sha256 + '"';
  const headers = new Headers({
    'Content-Type': 'application/octet-stream',
    'Content-Length': String(entry.size),
    'Cache-Control': 'no-cache',
    ETag: etag,
  });
  if (request.headers.get('If-None-Match') === etag) return new Response(null, { status: 304, headers });
  if (request.method === 'HEAD') return new Response(null, { headers });
  // The runtime pipes asset bodies natively; this keeps CPU time flat for large files.
  const { readable, writable } = new FixedLengthStream(entry.size);
  (async () => {
    try {
      for (const part of entry.parts) {
        const response = await env.ASSETS.fetch(new URL(part, request.url));
        if (!response.ok || !response.body) throw new Error('Missing data part ' + part + ': ' + response.status);
        await response.body.pipeTo(writable, { preventClose: true });
      }
      await writable.close();
    } catch (error) {
      await writable.abort(error);
    }
  })();
  return new Response(readable, { headers });
}

function isolated(response) {
  const result = new Response(response.body, response);
  for (const [name, value] of Object.entries(ISOLATION)) result.headers.set(name, value);
  return result;
}
