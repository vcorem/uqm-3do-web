// @vitest-environment node
// The room list and listed rooms through real relays: the site's Node relay
// with the lobby adapter of npm start (or the relay's own cosmoclash lobby),
// and relays without room lists (the Rust relay; the Node relay without the
// adapter), which must give 'unsupported'.
import { afterEach, describe, expect, it } from 'vitest';
// @ts-ignore JavaScript helpers, used read-only.
import { createNetplayServer } from '../../../server/netplay.mjs';
// @ts-ignore JavaScript subprocess adapter.
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
// @ts-ignore JavaScript local server of npm start.
import { createCosmoServer } from '../../scripts/cosmo-server.mjs';
// @ts-ignore JavaScript lobby adapter.
import { SILENT_HOST } from '../../scripts/cosmo-lobby.mjs';
// @ts-ignore ws is a root dependency.
import { WebSocket } from 'ws';
import { watchLobby, type LobbyRoom, type LobbySocket, type LobbyStatus } from './CosmoLobby';
import { CosmoNetplay, createRoomId, type SocketLike } from './CosmoNetplay';

const cleanup: (() => unknown)[] = [];
afterEach(async () => { for (const fn of cleanup.splice(0).reverse()) await fn(); });
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
async function until(predicate: () => boolean, detail: () => unknown = () => '', ms = 8000) {
  const end = Date.now() + ms;
  while (!predicate()) { if (Date.now() > end) throw new Error('Timed out: ' + JSON.stringify(detail())); await sleep(5); }
}
async function site(relay: { listen(port: number, host?: string): Promise<{ port: number }>; close(): Promise<void> }, lobby = 'auto', lobbyOptions = {}) {
  const app = createCosmoServer({ distDir: '/nonexistent', relay, lobby, lobbyOptions });
  const { port } = await app.listen(0);
  cleanup.push(() => app.close());
  const origin = `http://127.0.0.1:${port}/cosmoclash/`;
  const socket = (url: string) => new WebSocket(url, { origin: `http://127.0.0.1:${port}` });
  return { app, origin, socket };
}
function watch(s: Awaited<ReturnType<typeof site>>) {
  const statuses: [LobbyStatus, string][] = []; let rooms: LobbyRoom[] = [];
  const watcher = watchLobby({ origin: s.origin, socketFactory: url => s.socket(url) as unknown as LobbySocket,
    onStatus: (status, message) => statuses.push([status, message]), onRooms: list => { rooms = list; } });
  cleanup.push(() => watcher.close());
  return { watcher, statuses, get rooms() { return rooms; } };
}
function peer(s: Awaited<ReturnType<typeof site>>, options: { keepaliveMs?: number } = {}) {
  const session = new CosmoNetplay({ buildId: 'relay-test', keepaliveMs: options.keepaliveMs, onStart: () => {}, onFrame: () => {},
    socketFactory: url => s.socket(url) as unknown as SocketLike });
  cleanup.push(() => session.close());
  return session;
}

describe('room list and listed rooms through the Node relay', () => {
  it('a named room is listed (name, engine rust), a guest joins it from the list, and it leaves the list', async () => {
    const s = await site(createNetplayServer({ distDir: false }));
    expect(['adapter', 'relay']).toContain(s.app.lobby);
    const list = watch(s);
    await until(() => list.watcher.state === 'open', () => list.statuses);
    expect(list.rooms).toEqual([]);
    const host = peer(s, { keepaliveMs: 60 });
    host.connect({ origin: s.origin, room: createRoomId(), role: 'host', name: '  Friday   duel ⚔ ' });
    await until(() => list.rooms.length === 1, () => [list.rooms, host.state]);
    expect(list.rooms[0]).toMatchObject({ name: 'Friday duel ⚔', engine: 'rust', id: host.state.room });
    expect(Date.now() - list.rooms[0].created).toBeLessThan(5000);
    // The keepalive is answered: the host still waits after several pings.
    await sleep(300);
    expect(host.state.phase).toBe('waiting');
    const guest = peer(s);
    guest.connect({ origin: s.origin, room: list.rooms[0].id, role: 'guest' });
    await until(() => [host, guest].every(p => p.state.phase === 'lobby' && p.state.peerFleetKnown), () => [host.state, guest.state]);
    await until(() => list.rooms.length === 0, () => list.rooms);
    // Pings go on in the match; the relay answers them.
    await sleep(200);
    expect(host.state.phase).toBe('lobby');
    guest.close();
    await until(() => host.state.phase === 'closed', () => host.state);
    expect(host.state.message).toBe('Your opponent left the room.');
  }, 20000);
  it('a private room is not listed; its invite token still joins it', async () => {
    const s = await site(createNetplayServer({ distDir: false }));
    const list = watch(s);
    await until(() => list.watcher.state === 'open');
    const host = peer(s); const room = createRoomId();
    host.connect({ origin: s.origin, room, role: 'host' });
    await until(() => host.state.phase === 'waiting');
    await sleep(700);
    expect(list.rooms).toEqual([]);
    const guest = peer(s); guest.connect({ origin: s.origin, room, role: 'guest' });
    await until(() => guest.state.phase === 'lobby');
  });
  it('closes a listed room whose host stops pinging', async () => {
    const relay = createNetplayServer({ distDir: false });
    const s = await site(relay, 'adapter', { silenceMs: 250, checkMs: 50, snapshotMs: 50 });
    const list = watch(s);
    const host = peer(s, { keepaliveMs: 60000 });
    host.connect({ origin: s.origin, room: createRoomId(), role: 'host', name: 'Silent' });
    await until(() => list.rooms.length === 1);
    await until(() => host.state.phase === 'error', () => host.state);
    expect(host.state.message).toBe(SILENT_HOST);
    await until(() => list.rooms.length === 0);
  });
  it('refuses a name that is not allowed with the relay message', async () => {
    const s = await site(createNetplayServer({ distDir: false }), 'adapter');
    const messages: string[] = [];
    const ws = s.socket(s.origin.replace('http', 'ws').replace('/cosmoclash/', `/netplay/cosmoclash/${createRoomId()}?role=host&name=%ZZ&engine=rust`));
    ws.on('message', (data: Buffer) => messages.push(data.toString()));
    await until(() => messages.length > 0);
    expect(JSON.parse(messages[0])).toEqual({ type: 'error', message: 'Room names need 1 to 24 visible characters.' });
  });
});

describe('relays without a cosmoclash room list', () => {
  it('the Node relay without the adapter answers "Invalid lobby URL." before any list: unsupported', async () => {
    const s = await site(createNetplayServer({ distDir: false }), 'relay');
    const list = watch(s);
    await until(() => list.watcher.state !== 'connecting', () => list.statuses);
    // After the platform stream the relay has its own cosmoclash lobby: then it is open.
    if (list.watcher.state === 'open') return;
    expect(list.watcher.state).toBe('unsupported');
    expect(list.statuses.at(-1)).toEqual(['unsupported', 'Invalid lobby URL.']);
  });
  it('the Rust relay answers "Invalid room URL or role.": unsupported, and private rooms work', async () => {
    const s = await site(createRustRelay({ distDir: false }), 'relay');
    const list = watch(s);
    await until(() => list.watcher.state !== 'connecting', () => list.statuses);
    expect(list.statuses.at(-1)).toEqual(['unsupported', 'Invalid room URL or role.']);
    await sleep(1200);
    expect(list.statuses.filter(([status]) => status === 'connecting')).toHaveLength(1);
    const room = createRoomId();
    const host = peer(s, { keepaliveMs: 30 }); host.connect({ origin: s.origin, room, role: 'host' });
    await until(() => host.state.phase === 'waiting');
    // A private host does not ping: the Rust relay would refuse the text.
    await sleep(150);
    expect(host.state.phase).toBe('waiting');
    const guest = peer(s); guest.connect({ origin: s.origin, room, role: 'guest' });
    await until(() => [host, guest].every(p => p.state.phase === 'lobby'), () => [host.state, guest.state]);
  }, 20000);
});
