/**
 * The Cosmo Clash room list: a live list of the named rooms that wait for a
 * guest. Same behaviour as UQMNetplay.watchLobby (sc2/wasm/netplay-client.js),
 * with the edition 'cosmoclash':
 *
 *   /netplay/lobby?edition=cosmoclash -> {"type":"rooms","rooms":[{id,name,engine,created}]}
 *
 * The server sends the list at connect and on every change. The client sends
 * the text "ping" every 30 s and the server answers "pong". A relay without
 * room lists refuses the lobby URL before it sends a list; then the watcher
 * reports 'unsupported' and does not try again (the page offers private rooms).
 */
import { roomName } from './roomName';

export const LOBBY_EDITION = 'cosmoclash';
export const MAX_LISTED_ROOMS = 50;
const MAX_LOBBY_TEXT = 64 * 1024;
const CONNECT_MS = 12000;
export const KEEPALIVE_MS = 30000;
const RETRY_MAX_MS = 15000;
const LISTED_TOKEN = /^[a-f0-9]{32}$/;

export type LobbyStatus = 'connecting' | 'open' | 'error' | 'closed' | 'unsupported';
export interface LobbyRoom { id: string; name: string; engine: 'c' | 'rust' | null; created: number }
export interface LobbySocket {
  binaryType: string; readyState: number;
  onopen: ((event: unknown) => void) | null; onmessage: ((event: { data: unknown }) => void) | null;
  onclose: ((event: unknown) => void) | null; onerror: ((event: unknown) => void) | null;
  send(data: string): void; close(code?: number, reason?: string): void;
}
export interface LobbyOptions {
  /** The page address; the lobby is on the same host. Default: location.href. */
  origin?: string;
  socketFactory?: (url: string) => LobbySocket;
  setTimeout?: (fn: () => void, ms: number) => unknown;
  clearTimeout?: (id: unknown) => void;
  onRooms?: (rooms: LobbyRoom[]) => void;
  onStatus?: (status: LobbyStatus, message: string) => void;
}
export interface LobbyWatcher { readonly state: LobbyStatus; readonly rooms: LobbyRoom[]; close(): void }

/** ws(s)://<host>/netplay/lobby?edition=cosmoclash for the page at origin. */
export function lobbyURL(origin: string): string {
  const url = new URL(origin);
  if (url.protocol !== 'http:' && url.protocol !== 'https:') throw new Error('Open the game through its web server to play online.');
  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
  url.username = url.password = '';
  url.pathname = '/netplay/lobby';
  url.search = '?edition=' + LOBBY_EDITION;
  url.hash = '';
  return url.href;
}

/** Checked copies of the relay's room entries; anything malformed is left out. */
export function lobbyRooms(list: unknown[]): LobbyRoom[] {
  const rooms: LobbyRoom[] = [];
  for (const room of list.slice(0, MAX_LISTED_ROOMS) as Record<string, unknown>[]) {
    if (!room || typeof room !== 'object' || typeof room.id !== 'string' || !LISTED_TOKEN.test(room.id) || typeof room.name !== 'string') continue;
    let name: string;
    try { name = roomName(room.name); } catch { continue; }
    rooms.push({ id: room.id, name,
      engine: room.engine === 'c' || room.engine === 'rust' ? room.engine : null,
      created: Number.isSafeInteger(room.created) ? room.created as number : 0 });
  }
  return rooms;
}

export function watchLobby(options: LobbyOptions = {}): LobbyWatcher {
  const factory = options.socketFactory ?? ((url: string) => new WebSocket(url) as unknown as LobbySocket);
  const schedule = options.setTimeout ?? ((fn: () => void, ms: number) => setTimeout(fn, ms));
  const cancel = options.clearTimeout ?? ((id: unknown) => clearTimeout(id as ReturnType<typeof setTimeout>));
  let socket: LobbySocket | null = null;
  let timer: unknown;
  let keepTimer: unknown;
  let attempts = 0;
  let closed = false;
  let answered = false;
  // A room list came on this connection (a refusal before one means: no lists here).
  let listed = false;
  let state: LobbyStatus = 'connecting';
  let rooms: LobbyRoom[] = [];
  const watcher: LobbyWatcher = {
    get state() { return state; },
    get rooms() { return rooms.slice(); },
    close() {
      if (closed) return;
      closed = true;
      stopTimers();
      const current = socket;
      socket = null;
      closeSocket(current);
      status('closed', 'Left the room list.');
    },
  };
  function status(next: LobbyStatus, message: string): void {
    state = next;
    try { options.onStatus?.(next, message); } catch (error) { console.error('Room list status could not be shown:', error); }
  }
  function stopTimers(): void {
    if (timer !== undefined) cancel(timer);
    if (keepTimer !== undefined) cancel(keepTimer);
    timer = keepTimer = undefined;
  }
  function closeSocket(current: LobbySocket | null): void {
    if (!current) return;
    current.onopen = current.onmessage = current.onerror = current.onclose = null;
    if (current.readyState < 2) {
      try { current.close(1000, 'Room list closed'); } catch { /* Already disconnected. */ }
    }
  }
  // Lost or refused connections retry after 1, 2, 4, 8, then every 15 seconds.
  function drop(current: LobbySocket, message: string): void {
    if (closed || current !== socket) return;
    socket = null;
    stopTimers();
    closeSocket(current);
    const delay = Math.min(RETRY_MAX_MS, 1000 * Math.pow(2, Math.min(attempts, 4)));
    attempts++;
    status('error', message);
    if (!closed) timer = schedule(() => { timer = undefined; open(); }, delay);
  }
  // The Rust relay says 'Invalid room URL or role.'; Node and the Worker say
  // 'Invalid lobby URL.' for a lobby that they cannot serve. No retry.
  function unsupported(current: LobbySocket, message: string): void {
    if (closed || current !== socket) return;
    socket = null;
    closed = true;
    stopTimers();
    closeSocket(current);
    status('unsupported', message);
  }
  function keepalive(current: LobbySocket): void {
    keepTimer = schedule(() => {
      keepTimer = undefined;
      if (current !== socket) return;
      if (!answered) { drop(current, 'The room list stopped answering. Reconnecting…'); return; }
      answered = false;
      try { current.send('ping'); } catch { drop(current, 'Lost the connection to the room list. Reconnecting…'); return; }
      keepalive(current);
    }, KEEPALIVE_MS);
  }
  function receive(current: LobbySocket, data: unknown): void {
    answered = true;
    if (data === 'pong') return;
    let message: { type?: unknown; message?: unknown; rooms?: unknown } | null = null;
    if (typeof data === 'string' && data.length <= MAX_LOBBY_TEXT) {
      try { message = JSON.parse(data); } catch { message = null; }
    }
    if (message && typeof message === 'object' && message.type === 'error') {
      const detail = typeof message.message === 'string' ? message.message.slice(0, 240) : 'The room list is unavailable.';
      if (!listed && /^Invalid (room|lobby) URL/.test(detail)) { unsupported(current, detail); return; }
      drop(current, detail + ' Trying again…');
      return;
    }
    if (!message || typeof message !== 'object' || message.type !== 'rooms' || !Array.isArray(message.rooms)) {
      drop(current, 'The room list sent an unexpected message. Reconnecting…');
      return;
    }
    listed = true;
    if (state !== 'open') {
      if (timer !== undefined) cancel(timer);
      timer = undefined;
      attempts = 0;
      status('open', 'Showing the open rooms.');
      keepalive(current);
    }
    rooms = lobbyRooms(message.rooms);
    try { options.onRooms?.(rooms.slice()); } catch (error) { console.error('Room list could not be shown:', error); }
  }
  function open(): void {
    if (closed) return;
    let url: string;
    try { url = lobbyURL(options.origin ?? location.href); }
    catch (error) { closed = true; status('closed', (error as Error).message); return; }
    status('connecting', attempts ? 'Reconnecting to the room list…' : 'Connecting to the room list…');
    let current: LobbySocket;
    try { current = factory(url); }
    catch { closed = true; status('closed', 'Online play is unavailable in this browser.'); return; }
    socket = current;
    answered = false;
    listed = false;
    current.binaryType = 'arraybuffer';
    timer = schedule(() => { timer = undefined; drop(current, 'Could not reach the room list. Trying again…'); }, CONNECT_MS);
    current.onmessage = event => { if (!closed && current === socket) receive(current, event.data); };
    current.onerror = () => drop(current, 'Could not reach the room list. Trying again…');
    current.onclose = () => drop(current, 'Lost the connection to the room list. Reconnecting…');
  }
  open();
  return watcher;
}
