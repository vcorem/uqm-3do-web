// Private two-player Melee relay on Cloudflare: the Durable Object port of
// server/netplay.mjs. One NetplayRoom object per room ID holds both players'
// WebSockets with the hibernation API. One NetplayGate object holds the
// connection leases for the per-IP and global bounds.
import { DurableObject } from 'cloudflare:workers';

const ROOM_URL = /^\/netplay\/([0-9a-f]{32})\?role=(host|guest)$/;
const DEFAULT_ORIGINS = 'https://urquanmasters.org';

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
  const match = ROOM_URL.exec(url.pathname + url.search);
  if (!match) return rejectedSocket('Invalid room URL or role.');
  // Cloudflare sets CF-Connecting-IP at the edge; a client cannot choose it in production.
  const headers = new Headers(request.headers);
  headers.set('X-Netplay-IP', request.headers.get('CF-Connecting-IP') || 'unknown');
  const room = env.NETPLAY_ROOMS.get(env.NETPLAY_ROOMS.idFromName(match[1]));
  return room.fetch(new Request(request.url, { method: 'GET', headers }));
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
    ctx.blockConcurrencyWhile(async () => {
      this.room = (await ctx.storage.get('room')) ?? null;
      this.lastData = this.room?.lastData ?? 0;
      this.savedLastData = this.lastData;
    });
  }

  socket(role) {
    return this.ctx.getWebSockets(role)[0] ?? null;
  }

  async fetch(request) {
    const url = new URL(request.url);
    const match = ROOM_URL.exec(url.pathname + url.search);
    if (!match) return rejectedSocket('Invalid room URL or role.');
    const [, id, role] = match;
    const ip = request.headers.get('X-Netplay-IP') || 'unknown';
    const room = this.room;
    if (role === 'host') {
      if (room || this.pending.host) return rejectedSocket('Room already exists or has expired. Create a new room.');
    } else {
      if (!room || room.status === 'ended' || !this.socket('host')) {
        if (room && room.status !== 'ended') await this.endRoom(null, 'The other player disconnected.');
        return rejectedSocket('Room does not exist or has expired.');
      }
      if (room.status === 'paired' || this.pending.guest || this.socket('guest')) {
        return rejectedSocket('Room already has two players.');
      }
    }
    // Reserve the role before the gate call, because other requests can run during it.
    this.pending[role] = true;
    const conn = crypto.randomUUID();
    let lease;
    try {
      lease = await this.gate.acquire({ conn, ip, room: id, newRoom: role === 'host' });
    } catch {
      lease = 'busy';
    }
    this.pending[role] = false;
    if (lease === 'busy') return jsonResponse(503, 'Connection limit reached.');
    if (lease === 'rooms') return rejectedSocket('Room limit reached.');
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
    } else {
      this.room = { ...this.room, status: 'paired', lastData: now };
    }
    this.savedLastData = now;
    await this.ctx.storage.put('room', this.room);
    await this.schedule();
    this.send(server, { type: 'waiting' });
    if (role === 'guest') {
      this.send(this.socket('host'), { type: 'ready', role: 'host' });
      this.send(server, { type: 'ready', role: 'guest' });
    }
    return new Response(null, { status: 101, webSocket: client });
  }

  /** Called by the gate to drop leases of connections that no longer exist. */
  isLive(conn) {
    return this.ctx.getWebSockets('c:' + conn).length > 0;
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

  async endRoom(departing, message = 'The other player disconnected.') {
    const sockets = this.ctx.getWebSockets();
    const wasActive = this.room && this.room.status !== 'ended';
    if (wasActive) {
      this.room = { ...this.room, status: 'ended', endedAt: Date.now(), lastData: this.lastData };
    }
    this.queues.clear();
    for (const ws of sockets) {
      if (ws !== departing && wasActive) this.send(ws, { type: 'peer-left', message });
      this.close(ws, 1001, 'Room closed');
    }
    await Promise.all(sockets.map(ws => this.release(ws.deserializeAttachment()?.conn)));
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
    if (!peer) return this.endRoom(ws);
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
    await this.release(ws.deserializeAttachment()?.conn);
    this.close(ws, 1000, 'Closed');
    await this.endRoom(ws);
  }

  async webSocketError(ws) {
    await this.webSocketClose(ws);
  }

  // One alarm covers every room deadline: waiting expiry, total lifetime,
  // idle time, and deletion of the retired marker.
  async schedule() {
    const room = this.room;
    if (!room) return;
    const { waitingMs, lifetimeMs, idleMs, retainMs } = this.config;
    let at;
    if (room.status === 'waiting') at = Math.min(room.created + waitingMs, room.created + lifetimeMs);
    else if (room.status === 'paired') at = Math.min(room.created + lifetimeMs, Math.max(room.lastData, this.lastData) + idleMs);
    else at = room.endedAt + retainMs;
    await this.ctx.storage.setAlarm(at);
  }

  async alarm() {
    const room = this.room;
    if (!room) return;
    const now = Date.now();
    const { waitingMs, lifetimeMs, idleMs, retainMs } = this.config;
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

export class NetplayGate extends DurableObject {
  constructor(ctx, env) {
    super(ctx, env);
    this.config = settings(env);
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

  /** Returns 'ok', 'busy' (connection bounds) or 'rooms' (active room bound). */
  async acquire({ conn, ip, room, newRoom }) {
    let denial = this.denial(ip, newRoom);
    if (denial) {
      await this.prune(ip, this.counts(ip).perIp < this.config.maxPerIp);
      denial = this.denial(ip, newRoom);
      if (denial) return denial;
    }
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
