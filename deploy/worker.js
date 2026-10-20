// Cloudflare Worker for urquanmasters.org: static game files with the
// cross-origin isolation headers, the large data files joined from parts, and
// the private Online Melee relay (deploy/netplay.js).
import dataFiles from './data-parts.json';
import { handleNetplay } from './netplay.js';

export { NetplayRoom, NetplayGate } from './netplay.js';

const CANONICAL_HOST = 'urquanmasters.org';
const ISOLATION = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
  'Cross-Origin-Resource-Policy': 'same-origin',
  'X-Content-Type-Options': 'nosniff',
};
const INDEX = { '/': '/index.html' };
for (const folder of ['original', 'hd', 'original-rust', 'hd-rust']) INDEX['/' + folder + '/'] = '/' + folder + '/index.html';
const PART = /\.data\.p\d{3}$/;

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
    const entry = dataFiles[url.pathname];
    if (entry) return isolated(joinParts(request, env, entry));
    if (INDEX[url.pathname]) url.pathname = INDEX[url.pathname];
    return isolated(await env.ASSETS.fetch(new Request(url, request)));
  },
};

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
