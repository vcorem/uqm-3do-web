// Two-player Melee relay on Cloudflare: the Durable Object port of
// server/netplay.mjs. One NetplayRoom object per room ID holds both players'
// WebSockets with the hibernation API. One NetplayGate object holds the
// connection leases for the per-IP and global bounds. One NetplayLobby object
// per edition (original, hd, cosmoclash) holds the list of named rooms that
// wait for a guest, and the WebSockets of the players who look at that list.
import { DurableObject } from 'cloudflare:workers';

// /netplay/<token> is an Original room, /netplay/hd/<token> an HD room and
// /netplay/cosmoclash/<token> a Cosmo Clash room. These games cannot share a
// stream, so the edition is part of the room name.
const ROOM_PATH = /^\/netplay\/(?:(hd|cosmoclash)\/)?([0-9a-f]{32})$/;
const QUERY_PART = /^(role|name|engine)=([^&]*)$/;
const LOBBY_URL = /^\/netplay\/lobby\?edition=(original|hd|cosmoclash)$/;
// The engine tags that a listed room of each edition can show. Cosmo Clash has
// one engine, its Rust gameplay engine.
const ENGINES = { original: ['c', 'rust'], hd: ['c', 'rust'], cosmoclash: ['rust'] };
const roomName = (edition, token) => (edition ? edition + ':' : '') + token;
const lobbyName = edition => 'lobby:' + (edition ?? 'original');
/** The edition of a room name from roomName(): null for Original. */
const roomEdition = id => (id.includes(':') ? id.slice(0, id.indexOf(':')) : null);
const DEFAULT_ORIGINS = 'https://urquanmasters.org';
const INVALID_ROOM = 'Invalid room URL or role.';
const INVALID_NAME = 'Room names need 1 to 24 visible characters.';
// The same name rule is in server/netplay.mjs and sc2/wasm/netplay-client.js.
// Refused: control and format characters (bidi overrides, zero-width marks),
// line separators, surrogates, private-use and unassigned code points
// (noncharacters too), the blank fillers U+115F, U+1160, U+2800, U+3164 and
// U+FFA0, and more than 3 stacked nonspacing marks (Hebrew with niqqud needs 3).
const NAME_FORBIDDEN = /[\p{Cc}\p{Cf}\p{Cs}\p{Co}\p{Cn}\p{Zl}\p{Zp}\u115f\u1160\u2800\u3164\uffa0]/u;
// Counted in the decomposed form, where a precomposed letter shows its marks.
const NAME_STACKED = /[\p{Mn}\p{Me}]{4}/u;
// At least one letter, digit, punctuation mark or symbol.
const NAME_VISIBLE = /[\p{L}\p{N}\p{P}\p{S}]/u;
const NAME_CHARS = 24;
const MAX_LISTED = 50;
const SNAPSHOT_MS = 500;
const LISTING_DENIED = {
  full: 'The room list is full. Try again in a few minutes.',
  ip: 'Your network already has the most open rooms allowed. Start or close one first.',
  unavailable: 'The room list is unavailable. Try again.',
};
const SILENT_HOST = 'The room closed because this page stopped answering. Create a new room.';
const TOO_MANY_JOINS = 'You joined many open rooms in a short time. Wait a few minutes, then try again.';

/**
 * The listed form of a room name (NFC, inner spaces collapsed to one, trimmed),
 * or null when it does not have 1 to 24 allowed characters.
 */
function checkedName(value) {
  const text = value.normalize('NFC');
  // Before collapsing spaces: line breaks and tabs are control characters.
  if (NAME_FORBIDDEN.test(text) || NAME_STACKED.test(text.normalize('NFD'))) return null;
  const name = text.replace(/\s+/g, ' ').trim();
  const length = [...name].length;
  return length >= 1 && length <= NAME_CHARS && NAME_VISIBLE.test(name) ? name : null;
}

/**
 * The key for the per-address bounds. One IPv6 client controls a whole /64
 * network, so IPv6 addresses count by their first 64 bits. IPv4 addresses, also
 * in IPv4-mapped IPv6 form, count one by one. The same function is in server/netplay.mjs.
 */
export function addressKey(ip) {
  const text = String(ip ?? '').trim().toLowerCase().replace(/%.*$/, '');
  if (!text.includes(':')) return text.slice(0, 64) || 'unknown';
  let hex = text;
  // An IPv4 address in the last 32 bits becomes two hex words.
  const v4 = /:(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/.exec(hex);
  if (v4) {
    const bytes = v4.slice(1).map(Number);
    if (bytes.some(byte => byte > 255)) return text.slice(0, 64);
    hex = hex.slice(0, v4.index + 1) + ((bytes[0] << 8) | bytes[1]).toString(16) + ':' +
      ((bytes[2] << 8) | bytes[3]).toString(16);
  }
  const halves = hex.split('::');
  if (halves.length > 2) return text.slice(0, 64);
  const head = halves[0] ? halves[0].split(':') : [];
  const tail = halves.length === 2 && halves[1] ? halves[1].split(':') : [];
  const missing = 8 - head.length - tail.length;
  if (halves.length === 1 ? missing !== 0 : missing < 1) return text.slice(0, 64);
  const parts = [...head, ...Array(halves.length === 2 ? missing : 0).fill('0'), ...tail];
  if (parts.some(part => !/^[0-9a-f]{1,4}$/.test(part))) return text.slice(0, 64);
  const words = parts.map(part => parseInt(part, 16));
  if (words.slice(0, 5).every(word => word === 0) && words[5] === 0xffff) {
    return [words[6] >> 8, words[6] & 255, words[7] >> 8, words[7] & 255].join('.');
  }
  return words.slice(0, 4).map(word => word.toString(16)).join(':') + '::/64';
}

/**
 * Strict room URL: role=host|guest, and for a host only an optional name
 * (which lists the room in its edition's lobby) and engine=c|rust (Cosmo
 * Clash: engine=rust, also the tag of a named room without one). Each
 * parameter at most once; nothing else.
 */
export function parseRoomURL(pathname, search) {
  const path = ROOM_PATH.exec(pathname);
  if (!path || !search.startsWith('?')) return { error: INVALID_ROOM };
  const params = new Map();
  for (const part of search.slice(1).split('&')) {
    const match = QUERY_PART.exec(part);
    if (!match || params.has(match[1])) return { error: INVALID_ROOM };
    params.set(match[1], match[2]);
  }
  const role = params.get('role');
  const rawName = params.get('name');
  const engine = params.get('engine') ?? null;
  if (role !== 'host' && role !== 'guest') return { error: INVALID_ROOM };
  if (role === 'guest' && params.size > 1) return { error: INVALID_ROOM };
  const edition = path[1] ?? null;
  if (engine !== null && (rawName === undefined || !ENGINES[edition ?? 'original'].includes(engine))) {
    return { error: INVALID_ROOM };
  }
  let name = null;
  if (rawName !== undefined) {
    // 24 characters of up to 4 UTF-8 bytes, each byte as %XX.
    try { name = rawName.length <= 288 ? checkedName(decodeURIComponent(rawName.replace(/\+/g, ' '))) : null; }
    catch { name = null; }
    if (name === null) return { error: INVALID_NAME };
  }
  // An edition with one engine (Cosmo Clash: rust) lists every named room with
  // that tag, also when the host sends none, so that the list never shows null.
  const engines = ENGINES[edition ?? 'original'];
  const tag = engine ?? (name !== null && engines.length === 1 ? engines[0] : null);
  return { edition, token: path[2], role, name, engine: tag };
}

function integer(raw, fallback, min, max) {
  const value = raw == null || raw === '' ? fallback : Number(raw);
  if (!Number.isSafeInteger(value) || value < min || value > max) throw new Error('Invalid netplay setting');
  return value;
}

// Bounds come from wrangler vars so that tests can shorten timers. Production uses the defaults.
function settings(env) {
  const latency = String(env.NETPLAY_LATENCY_MS ?? '0').split(',').map(Number);
  if (!latency.length || latency.length > 100 || latency.some(n => !Number.isSafeInteger(n) || n < 0 || n > 2000)) {
    throw new Error('Invalid NETPLAY_LATENCY_MS');
  }
  const maxPayload = 65536;
  const maxRooms = integer(env.NETPLAY_MAX_ROOMS, 128, 1, 1024);
  return {
    maxPayload,
    maxBufferedBytes: integer(env.NETPLAY_MAX_BUFFERED_BYTES, 262144, maxPayload, 1048576),
    maxRooms,
    maxConnections: maxRooms * 2,
    maxPerIp: integer(env.NETPLAY_MAX_PER_IP, 16, 2, 2048),
    waitingMs: integer(env.NETPLAY_WAITING_MS, 120000, 10, 3600000),
    lifetimeMs: integer(env.NETPLAY_LIFETIME_MS, 14400000, 10, 86400000),
    idleMs: integer(env.NETPLAY_IDLE_MS, 900000, 10, 86400000),
    // An ended room keeps its retired marker this long, then deletes its storage.
    retainMs: integer(env.NETPLAY_RETAIN_MS, 604800000, 10, 2592000000),
    latencyPatternMs: latency.some(n => n > 0) ? latency : null,
    origins: new Set(String(env.NETPLAY_ORIGINS ?? DEFAULT_ORIGINS).split(/[\s,]+/).filter(Boolean)),
    // Lobby: named rooms wait longer than private ones, because nobody has to copy an invite.
    listedWaitingMs: integer(env.NETPLAY_LISTED_WAITING_MS, 600000, 10, 3600000),
    maxListed: integer(env.NETPLAY_MAX_LISTED, MAX_LISTED, 1, MAX_LISTED),
    maxListedPerIp: integer(env.NETPLAY_MAX_LISTED_PER_IP, 3, 1, MAX_LISTED),
    maxLobby: integer(env.NETPLAY_MAX_LOBBY, 1000, 1, 10000),
    maxLobbyPerIp: integer(env.NETPLAY_MAX_LOBBY_PER_IP, 10, 1, 1000),
    lobbyCheckMs: integer(env.NETPLAY_LOBBY_CHECK_MS, 20000, 100, 600000),
    // A refused listing checks the listed rooms at most this often (the alarm still runs).
    lobbySweepMs: integer(env.NETPLAY_LOBBY_SWEEP_MS, 5000, 0, 600000),
    // A listed host pings every 30 s. Without a ping for this long it is gone
    // (a sleeping computer or a lost network); background tabs ping at least once a minute.
    listedSilenceMs: integer(env.NETPLAY_LISTED_SILENCE_MS, 150000, 100, 3600000),
    // Joins of listed rooms per address: list tokens are public, so one script
    // could otherwise join and leave every new room.
    maxListedJoins: integer(env.NETPLAY_MAX_LISTED_JOINS, 10, 1, 10000),
    listedJoinWindowMs: integer(env.NETPLAY_LISTED_JOIN_WINDOW_MS, 600000, 100, 86400000),
  };
}

function jsonResponse(status, message) {
  return new Response(JSON.stringify({ type: 'error', message }), {
    status, headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' },
  });
}

// Accept, send the reason and close at once. The client shows the reason to the player.
function rejectedSocket(message, code = 1008) {
  const { 0: client, 1: server } = new WebSocketPair();
  server.accept();
  server.send(JSON.stringify({ type: 'error', message }));
  server.close(code, message.slice(0, 100));
  return new Response(null, { status: 101, webSocket: client });
}

/** Worker entry for /netplay/*. Checks run before a room object is involved. */
export async function handleNetplay(request, env) {
  const config = settings(env);
  if (request.method !== 'GET' || request.headers.get('Upgrade')?.toLowerCase() !== 'websocket') {
    return jsonResponse(426, 'Online Melee needs a WebSocket connection.');
  }
  const origin = request.headers.get('Origin');
  if (!origin || !config.origins.has(origin)) return jsonResponse(403, 'Origin is not allowed.');
  // Browsers use the standard WebSocket protocol; arbitrary proxy subprotocols are unsupported.
  if (request.headers.has('Sec-WebSocket-Protocol')) return jsonResponse(400, 'Subprotocols are unsupported.');
  const url = new URL(request.url);
  // Cloudflare sets CF-Connecting-IP at the edge; a client cannot choose it in production.
  // Every per-address bound (gate leases, listed rooms, lobby sockets) uses this key.
  const headers = new Headers(request.headers);
  headers.set('X-Netplay-IP', addressKey(request.headers.get('CF-Connecting-IP') || 'unknown'));
  const forwarded = new Request(request.url, { method: 'GET', headers });
  if (url.pathname === '/netplay/lobby') {
    const lobby = LOBBY_URL.exec(url.pathname + url.search);
    if (!lobby) return rejectedSocket('Invalid lobby URL.');
    const edition = lobby[1] === 'original' ? null : lobby[1];
    return env.NETPLAY_LOBBY.get(env.NETPLAY_LOBBY.idFromName(lobbyName(edition))).fetch(forwarded);
  }
  const parsed = parseRoomURL(url.pathname, url.search);
  if (parsed.error) return rejectedSocket(parsed.error);
  const room = env.NETPLAY_ROOMS.get(env.NETPLAY_ROOMS.idFromName(roomName(parsed.edition, parsed.token)));
  return room.fetch(forwarded);
}

export class NetplayRoom extends DurableObject {
  constructor(ctx, env) {
    super(ctx, env);
    this.config = settings(env);
    this.gate = env.NETPLAY_GATE.get(env.NETPLAY_GATE.idFromName('global'));
    this.pending = { host: false, guest: false };
    // Memory is lost when the object hibernates. Counters then restart, which is
    // safe: hibernation happens only after a quiet period longer than a rate window.
    this.rates = new Map();
    this.queues = new Map();
    this.savedLastData = 0;
    // Browsers cannot send ping frames. A named room's host sends the text "ping"
    // while it waits; the runtime answers without waking the object.
    ctx.setWebSocketAutoResponse(new WebSocketRequestResponsePair('ping', 'pong'));
    ctx.blockConcurrencyWhile(async () => {
      this.room = (await ctx.storage.get('room')) ?? null;
      this.lastData = this.room?.lastData ?? 0;
      this.savedLastData = this.lastData;
    });
  }

  socket(role) {
    return this.ctx.getWebSockets(role)[0] ?? null;
  }

  /** The lobby of this room's edition: a Cosmo Clash room is listed only in the Cosmo Clash lobby. */
  lobby() {
    return this.env.NETPLAY_LOBBY.get(this.env.NETPLAY_LOBBY.idFromName(lobbyName(roomEdition(this.room.id))));
  }

  /** Takes a named room off its lobby list. Errors are left to the lobby's liveness check. */
  async unlist() {
    try { await this.lobby().remove(this.room.id.slice(-32)); } catch { /* Checked later. */ }
  }

  async fetch(request) {
    const url = new URL(request.url);
    const parsed = parseRoomURL(url.pathname, url.search);
    if (parsed.error) return rejectedSocket(parsed.error);
    const { edition, token, role, name, engine } = parsed;
    const id = roomName(edition, token);
    const ip = request.headers.get('X-Netplay-IP') || 'unknown';
    const room = this.room;
    if (role === 'host') {
      // A host socket without a room is a refused listing that is still closing.
      if (room || this.pending.host || this.socket('host')) {
        return rejectedSocket('Room already exists or has expired. Create a new room.');
      }
    } else {
      if (!room || room.status === 'ended' || !this.socket('host')) {
        if (room && room.status !== 'ended') await this.endRoom(null, 'The other player disconnected.');
        return rejectedSocket('Room does not exist or has expired.');
      }
      if (room.status === 'paired' || this.pending.guest || this.socket('guest')) {
        return rejectedSocket('Room already has two players.');
      }
      // A guest who picks a listed room must find a host that still answers.
      if (room.listed && this.silent()) {
        await this.closeSilent(true);
        return rejectedSocket('Room does not exist or has expired.');
      }
    }
    // Reserve the role before the gate call, because other requests can run during it.
    this.pending[role] = true;
    const conn = crypto.randomUUID();
    let lease;
    try {
      lease = await this.gate.acquire({ conn, ip, room: id, newRoom: role === 'host',
        listedJoin: role === 'guest' && Boolean(room.listed) });
    } catch {
      lease = 'busy';
    }
    this.pending[role] = false;
    if (lease === 'busy') return jsonResponse(503, 'Connection limit reached.');
    if (lease === 'rooms') return rejectedSocket('Room limit reached.');
    if (lease === 'joins') return rejectedSocket(TOO_MANY_JOINS);
    // The room can change while the gate call runs.
    if (role === 'guest' && (this.room?.status !== 'waiting' || !this.socket('host'))) {
      await this.release(conn);
      return rejectedSocket('Room does not exist or has expired.');
    }

    const { 0: client, 1: server } = new WebSocketPair();
    this.ctx.acceptWebSocket(server, [role, 'c:' + conn]);
    server.serializeAttachment({ role, conn });
    const now = Date.now();
    this.lastData = now;
    if (role === 'host') {
      this.room = { id, status: 'waiting', created: now, lastData: now };
      if (name) this.room.listed = { name, engine };
    } else {
      this.room = { ...this.room, status: 'paired', lastData: now };
    }
    this.savedLastData = now;
    await this.ctx.storage.put('room', this.room);
    await this.schedule();
    // The room is listed only after its host socket exists, so a guest who
    // picks it from the list always finds the host.
    if (role === 'host' && name) {
      let listing;
      try {
        listing = await this.lobby().add({ id: token, room: id, name, engine, created: now, ip });
      } catch {
        listing = 'unavailable';
      }
      if (listing !== 'ok') {
        const message = LISTING_DENIED[listing] ?? LISTING_DENIED.unavailable;
        if (this.room.status === 'waiting' && !this.socket('guest')) {
          // Like the room limit, a refused listing leaves no room, so the token stays usable.
          // Its close event must not end a room that a retry with the same token opens.
          server.serializeAttachment({ role, conn, refused: true });
          this.send(server, { type: 'error', message });
          this.close(server, 1008, message);
          this.room = null;
          await this.release(conn);
          await this.ctx.storage.deleteAlarm();
          await this.ctx.storage.deleteAll();
        } else {
          await this.reject(server, message);
        }
        return new Response(null, { status: 101, webSocket: client });
      }
      // The host can leave, or a guest with the invite can join, during the lobby call.
      if (this.room.status !== 'waiting') await this.unlist();
    }
    this.send(server, { type: 'waiting' });
    if (role === 'guest') {
      this.send(this.socket('host'), { type: 'ready', role: 'host' });
      this.send(server, { type: 'ready', role: 'guest' });
      if (this.room.listed) await this.unlist();
    }
    return new Response(null, { status: 101, webSocket: client });
  }

  /** Called by the gate to drop leases of connections that no longer exist. */
  isLive(conn) {
    return this.ctx.getWebSockets('c:' + conn).length > 0;
  }

  /**
   * Called by the lobby's liveness check: is this still a named room that waits
   * for a guest, with a host that answers? A silent host's room is closed here.
   */
  async isListed() {
    if (this.room?.status !== 'waiting' || !this.room.listed || !this.socket('host')) return false;
    if (!this.silent()) return true;
    // The lobby drops the entry because of the answer; it is not called back.
    await this.closeSilent(false);
    return false;
  }

  /**
   * True when the waiting listed host sent no "ping" for listedSilenceMs. The
   * runtime answers the pings and records the time; the object does not wake up.
   */
  silent() {
    const host = this.socket('host');
    if (!host) return true;
    let answered = null;
    try { answered = this.ctx.getWebSocketAutoResponseTimestamp(host); } catch { /* Not available: use the creation time. */ }
    const last = Math.max(answered ? answered.getTime() : 0, this.room.created);
    return Date.now() - last > this.config.listedSilenceMs;
  }

  async closeSilent(tellLobby) {
    const host = this.socket('host');
    this.send(host, { type: 'error', message: SILENT_HOST });
    await this.endRoom(host, undefined, tellLobby);
  }

  send(ws, value) {
    try { ws?.send(JSON.stringify(value)); } catch { /* Already closed. */ }
  }

  close(ws, code, reason) {
    try { ws.close(code, reason.slice(0, 100)); } catch { /* Already closed. */ }
  }

  async release(conn) {
    try { await this.gate.release(conn); } catch { /* The lease expires or is checked later. */ }
  }

  async endRoom(departing, message = 'The other player disconnected.', tellLobby = true) {
    const sockets = this.ctx.getWebSockets();
    const wasActive = this.room && this.room.status !== 'ended';
    const wasListed = tellLobby && wasActive && this.room.status === 'waiting' && this.room.listed;
    if (wasActive) {
      this.room = { ...this.room, status: 'ended', endedAt: Date.now(), lastData: this.lastData };
    }
    this.queues.clear();
    for (const ws of sockets) {
      if (ws !== departing && wasActive) this.send(ws, { type: 'peer-left', message });
      this.close(ws, 1001, 'Room closed');
    }
    await Promise.all([...sockets.map(ws => this.release(ws.deserializeAttachment()?.conn)),
      wasListed && this.unlist()]);
    if (wasActive) {
      await this.ctx.storage.put('room', this.room);
      await this.schedule();
    }
  }

  async reject(ws, message, code = 1008) {
    this.send(ws, { type: 'error', message });
    this.close(ws, code, message);
    await this.endRoom(ws);
  }

  async webSocketMessage(ws, data) {
    const room = this.room;
    if (!room || room.status === 'ended') return;
    if (room.status !== 'paired') return this.reject(ws, 'Wait for the other player before sending game data.');
    if (typeof data === 'string' || data.byteLength === 0) {
      return this.reject(ws, 'Only nonempty binary game data is accepted.');
    }
    if (data.byteLength > this.config.maxPayload) {
      // Same result as the Node parser limit: close code 1009 and no partial data to the peer.
      this.close(ws, 1009, 'Message too big');
      return this.endRoom(ws);
    }
    const { role } = ws.deserializeAttachment();
    const now = Date.now();
    let rate = this.rates.get(role);
    if (!rate || now - rate.start >= 1000) this.rates.set(role, rate = { start: now, bytes: 0, messages: 0 });
    rate.bytes += data.byteLength;
    rate.messages++;
    if (rate.bytes > 1048576 || rate.messages > 1000) return this.reject(ws, 'Game data rate exceeded.');
    const peer = this.socket(role === 'host' ? 'guest' : 'host');
    // The room is paired, so a missing peer has left: its close handler still waits for the
    // gate. The sender is the survivor and hears why (peer-left), as on the queued path below.
    if (!peer) return this.endRoom(null);
    this.lastData = now;
    // Persist the activity time at most once a minute; the idle limit is 15 minutes.
    if (now - this.savedLastData > 60000) {
      this.savedLastData = now;
      this.room = { ...this.room, lastData: now };
      this.ctx.storage.put('room', this.room);
    }
    if (!this.config.latencyPatternMs) {
      try { peer.send(data); } catch { return this.endRoom(peer); }
      return;
    }
    // Test-only deterministic delay. Jitter can delay later chunks but never reorder them.
    let queue = this.queues.get(role);
    if (!queue) this.queues.set(role, queue = { items: [], bytes: 0, sequence: 0, timer: null });
    if (queue.bytes + data.byteLength > this.config.maxBufferedBytes || queue.items.length >= 1024) {
      return this.reject(ws, 'Peer is too slow.');
    }
    const pattern = this.config.latencyPatternMs;
    const due = Math.max(now + pattern[queue.sequence++ % pattern.length], queue.items.at(-1)?.due ?? 0);
    queue.items.push({ data, due });
    queue.bytes += data.byteLength;
    if (!queue.timer) this.deliver(role, queue);
  }

  deliver(role, queue) {
    queue.timer = null;
    if (this.queues.get(role) !== queue) return;
    const peer = this.socket(role === 'host' ? 'guest' : 'host');
    while (queue.items.length && queue.items[0].due <= Date.now()) {
      const next = queue.items.shift();
      queue.bytes -= next.data.byteLength;
      if (!peer) return void this.endRoom(null);
      try { peer.send(next.data); } catch { return void this.endRoom(peer); }
    }
    if (queue.items.length) {
      queue.timer = setTimeout(() => this.deliver(role, queue), Math.max(1, queue.items[0].due - Date.now()));
    }
  }

  async webSocketClose(ws) {
    const attachment = ws.deserializeAttachment();
    await this.release(attachment?.conn);
    this.close(ws, 1000, 'Closed');
    if (!attachment?.refused) await this.endRoom(ws);
  }

  async webSocketError(ws) {
    await this.webSocketClose(ws);
  }

  /** A named (listed) room waits 10 minutes for a guest; a private room 2 minutes. */
  waitingLimit(room) {
    return room.listed ? this.config.listedWaitingMs : this.config.waitingMs;
  }

  // One alarm covers every room deadline: waiting expiry, total lifetime,
  // idle time, and deletion of the retired marker.
  async schedule() {
    const room = this.room;
    if (!room) return;
    const { lifetimeMs, idleMs, retainMs } = this.config;
    let at;
    if (room.status === 'waiting') at = Math.min(room.created + this.waitingLimit(room), room.created + lifetimeMs);
    else if (room.status === 'paired') at = Math.min(room.created + lifetimeMs, Math.max(room.lastData, this.lastData) + idleMs);
    else at = room.endedAt + retainMs;
    await this.ctx.storage.setAlarm(at);
  }

  async alarm() {
    const room = this.room;
    if (!room) return;
    const now = Date.now();
    const { lifetimeMs, idleMs, retainMs } = this.config;
    const waitingMs = this.waitingLimit(room);
    if (room.status === 'ended') {
      if (now >= room.endedAt + retainMs) {
        this.room = null;
        await this.ctx.storage.deleteAll();
        return;
      }
    } else if (now >= room.created + lifetimeMs) {
      return this.endRoom(null, 'Room time limit reached.');
    } else if (room.status === 'waiting' && now >= room.created + waitingMs) {
      const host = this.socket('host');
      this.send(host, { type: 'error', message: 'Nobody joined before the room expired.' });
      return this.endRoom(host);
    } else if (room.status === 'paired' && now >= Math.max(room.lastData, this.lastData) + idleMs) {
      return this.endRoom(null, 'Room was idle for too long.');
    }
    await this.schedule();
  }
}

// A lease lasts at most the room lifetime plus a margin. Leases of connections
// that ended without a close event (for example after a deploy restarts the
// room objects) are found by asking the room before a limit refuses a client.
const LEASE_MARGIN_MS = 300000;

// The gate remembers recent listed-room joins for at most this many addresses.
const MAX_JOIN_ADDRESSES = 10000;

export class NetplayGate extends DurableObject {
  constructor(ctx, env) {
    super(ctx, env);
    this.config = settings(env);
    // Join times of listed rooms per address, least recently used first. Memory
    // only: the object stays in memory while an address keeps joining.
    this.joins = new Map();
    ctx.blockConcurrencyWhile(async () => {
      this.leases = await ctx.storage.list({ prefix: 'c:' });
    });
  }

  counts(ip) {
    let total = 0, perIp = 0, rooms = 0;
    for (const lease of this.leases.values()) {
      total++;
      if (lease.ip === ip) perIp++;
      if (lease.newRoom) rooms++;
    }
    return { total, perIp, rooms };
  }

  denial(ip, newRoom) {
    const { total, perIp, rooms } = this.counts(ip);
    if (total >= this.config.maxConnections || perIp >= this.config.maxPerIp) return 'busy';
    if (newRoom && rooms >= this.config.maxRooms) return 'rooms';
    return null;
  }

  async prune(ip, everything) {
    const now = Date.now();
    const checks = [];
    for (const [key, lease] of this.leases) {
      if (lease.expires <= now) { checks.push(Promise.resolve([key, false])); continue; }
      if (!everything && lease.ip !== ip) continue;
      const room = this.env.NETPLAY_ROOMS.get(this.env.NETPLAY_ROOMS.idFromName(lease.room));
      checks.push(room.isLive(key.slice(2)).then(live => [key, live], () => [key, true]));
    }
    const dead = (await Promise.all(checks)).filter(([, live]) => !live).map(([key]) => key);
    for (const key of dead) this.leases.delete(key);
    if (dead.length) await this.ctx.storage.delete(dead);
  }

  /** Counts a join of a listed room; false when the address has joined too many recently. */
  countJoin(ip) {
    const now = Date.now();
    const times = (this.joins.get(ip) ?? []).filter(time => now - time < this.config.listedJoinWindowMs);
    this.joins.delete(ip);
    const allowed = times.length < this.config.maxListedJoins;
    if (allowed) times.push(now);
    this.joins.set(ip, times);
    while (this.joins.size > MAX_JOIN_ADDRESSES) this.joins.delete(this.joins.keys().next().value);
    return allowed;
  }

  /**
   * Returns 'ok', 'busy' (connection bounds), 'rooms' (active room bound) or
   * 'joins' (too many listed-room joins from this address in the window).
   */
  async acquire({ conn, ip, room, newRoom, listedJoin = false }) {
    let denial = this.denial(ip, newRoom);
    if (denial) {
      await this.prune(ip, this.counts(ip).perIp < this.config.maxPerIp);
      denial = this.denial(ip, newRoom);
      if (denial) return denial;
    }
    if (listedJoin && !this.countJoin(ip)) return 'joins';
    const lease = { ip, room, newRoom, expires: Date.now() + this.config.lifetimeMs + LEASE_MARGIN_MS };
    this.leases.set('c:' + conn, lease);
    await this.ctx.storage.put('c:' + conn, lease);
    return 'ok';
  }

  async release(conn) {
    if (!conn || !this.leases.delete('c:' + conn)) return;
    await this.ctx.storage.delete('c:' + conn);
  }
}

/**
 * The live room list of one edition. Room objects add a named room when its
 * host is connected and remove it when it pairs or ends. A liveness alarm asks
 * each listed room again, for rooms that ended without telling the lobby (for
 * example after a deploy restarts the room objects). Lobby sockets only
 * receive; they hibernate between snapshots and use no connection leases.
 */
export class NetplayLobby extends DurableObject {
  constructor(ctx, env) {
    super(ctx, env);
    this.config = settings(env);
    this.timer = null;
    this.lastSent = 0;
    this.lastCheck = 0;
    this.text = null;
    // Keepalive for clients behind proxies that close quiet connections.
    ctx.setWebSocketAutoResponse(new WebSocketRequestResponsePair('ping', 'pong'));
    ctx.blockConcurrencyWhile(async () => {
      this.rooms = await ctx.storage.list({ prefix: 'r:' });
    });
  }

  async fetch(request) {
    const url = new URL(request.url);
    if (!LOBBY_URL.test(url.pathname + url.search)) return rejectedSocket('Invalid lobby URL.');
    const tag = 'ip:' + (request.headers.get('X-Netplay-IP') || 'unknown').slice(0, 64);
    if (this.ctx.getWebSockets().length >= this.config.maxLobby ||
        this.ctx.getWebSockets(tag).length >= this.config.maxLobbyPerIp) {
      return jsonResponse(503, 'Connection limit reached.');
    }
    const { 0: client, 1: server } = new WebSocketPair();
    this.ctx.acceptWebSocket(server, [tag]);
    server.send(this.snapshot());
    return new Response(null, { status: 101, webSocket: client });
  }

  webSocketMessage(ws) {
    const message = 'The room list accepts no messages.';
    try {
      ws.send(JSON.stringify({ type: 'error', message }));
      ws.close(1008, message);
    } catch { /* Already closed. */ }
  }

  webSocketClose(ws) {
    try { ws.close(1000, 'Closed'); } catch { /* Already closed. */ }
  }

  webSocketError(ws) {
    this.webSocketClose(ws);
  }

  /** Full snapshot, newest first. Cached until the list changes. */
  snapshot() {
    this.text ??= JSON.stringify({ type: 'rooms', rooms: [...this.rooms.values()]
      .sort((a, b) => b.created - a.created || (a.id < b.id ? -1 : 1))
      .slice(0, MAX_LISTED)
      .map(({ id, name, engine, created }) => ({ id, name, engine, created })) });
    return this.text;
  }

  // At most one snapshot per SNAPSHOT_MS to all sockets; the last change always goes out.
  changed() {
    this.text = null;
    if (this.timer) return;
    const wait = this.lastSent + SNAPSHOT_MS - Date.now();
    if (wait > 0) this.timer = setTimeout(() => this.broadcast(), wait);
    else this.broadcast();
  }

  broadcast() {
    this.timer = null;
    this.lastSent = Date.now();
    const text = this.snapshot();
    for (const ws of this.ctx.getWebSockets()) {
      try { ws.send(text); } catch { /* Closing. */ }
    }
  }

  denial(ip) {
    if (this.rooms.size >= this.config.maxListed) return 'full';
    let mine = 0;
    for (const entry of this.rooms.values()) if (entry.ip === ip) mine++;
    return mine >= this.config.maxListedPerIp ? 'ip' : null;
  }

  /** Called by a room object. Returns 'ok', 'full' (list bound) or 'ip' (rooms per address). */
  async add({ id, room, name, engine, created, ip }) {
    const key = 'r:' + id;
    if (this.rooms.has(key)) return 'ok';
    if (this.denial(ip)) {
      // Each check asks every listed room, so repeated refusals use the last result.
      if (Date.now() - this.lastCheck >= this.config.lobbySweepMs) await this.check();
      const denial = this.denial(ip);
      if (denial) return denial;
    }
    const entry = { id, room, name, engine: engine ?? null, created, ip };
    this.rooms.set(key, entry);
    this.changed();
    await this.ctx.storage.put(key, entry);
    if ((await this.ctx.storage.getAlarm()) === null) {
      await this.ctx.storage.setAlarm(Date.now() + this.config.lobbyCheckMs);
    }
    return 'ok';
  }

  /** Called by a room object when its named room pairs or ends. */
  async remove(id) {
    const key = 'r:' + id;
    if (!this.rooms.delete(key)) return;
    this.changed();
    await this.ctx.storage.delete(key);
  }

  /** Drops entries whose room no longer waits for a guest, or that outlived the waiting limit. */
  async check() {
    const now = Date.now();
    this.lastCheck = now;
    const { listedWaitingMs, lobbyCheckMs } = this.config;
    const results = await Promise.all([...this.rooms].map(async ([key, entry]) => {
      if (now >= entry.created + listedWaitingMs + lobbyCheckMs) return [key, false];
      try {
        const room = this.env.NETPLAY_ROOMS.get(this.env.NETPLAY_ROOMS.idFromName(entry.room));
        return [key, await room.isListed()];
      } catch {
        return [key, true];
      }
    }));
    // An entry can be removed by its room while the checks run.
    const dead = results.filter(([, live]) => !live).map(([key]) => key).filter(key => this.rooms.delete(key));
    if (!dead.length) return;
    this.changed();
    await this.ctx.storage.delete(dead);
  }

  async alarm() {
    await this.check();
    if (this.rooms.size) await this.ctx.storage.setAlarm(Date.now() + this.config.lobbyCheckMs);
  }
}
