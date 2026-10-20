// @vitest-environment node
import { describe, expect, it } from 'vitest';
import { lobbyRooms, lobbyURL, watchLobby, KEEPALIVE_MS, type LobbySocket, type LobbyStatus } from './CosmoLobby';

/** A fake clock and fake sockets: the watcher's timers and messages under test control. */
function harness() {
  let now = 0;
  const timers = new Map<number, { at: number; fn: () => void }>();
  let next = 1;
  const sockets: (LobbySocket & { url: string; sent: string[]; closed: boolean })[] = [];
  const statuses: [LobbyStatus, string][] = [];
  const lists: unknown[][] = [];
  const watcher = watchLobby({
    origin: 'https://urquanmasters.org/cosmoclash/?cosmoRoom=x',
    setTimeout: (fn, ms) => { const id = next++; timers.set(id, { at: now + ms, fn }); return id; },
    clearTimeout: id => { timers.delete(id as number); },
    socketFactory: url => {
      const socket = { url, sent: [] as string[], closed: false, binaryType: '', readyState: 1,
        onopen: null, onmessage: null, onclose: null, onerror: null,
        send(data: string) { this.sent.push(data); }, close() { this.closed = true; this.readyState = 3; } };
      sockets.push(socket);
      return socket;
    },
    onStatus: (status, message) => statuses.push([status, message]),
    onRooms: rooms => lists.push(rooms),
  });
  return {
    watcher, sockets, statuses, lists,
    get socket() { return sockets[sockets.length - 1]; },
    message(data: unknown) { this.socket.onmessage?.({ data: typeof data === 'string' ? data : JSON.stringify(data) }); },
    advance(ms: number) {
      const end = now + ms;
      for (;;) {
        const due = [...timers.entries()].filter(([, t]) => t.at <= end).sort((a, b) => a[1].at - b[1].at)[0];
        if (!due) break;
        timers.delete(due[0]); now = due[1].at; due[1].fn();
      }
      now = end;
    },
    pending: () => timers.size,
  };
}
const room = (id: string, name: string, created = 5) => ({ id: id.repeat(32).slice(0, 32), name, engine: 'rust', created });

describe('the Cosmo Clash room list (UQMNetplay.watchLobby behaviour)', () => {
  it('opens the cosmoclash lobby on the same host', () => {
    expect(lobbyURL('https://urquanmasters.org/cosmoclash/?cosmoRoom=1#x')).toBe('wss://urquanmasters.org/netplay/lobby?edition=cosmoclash');
    expect(lobbyURL('http://127.0.0.1:10000/cosmoclash/')).toBe('ws://127.0.0.1:10000/netplay/lobby?edition=cosmoclash');
    expect(() => lobbyURL('file:///game/index.html')).toThrow(/web server/);
    const h = harness();
    expect(h.socket.url).toBe('wss://urquanmasters.org/netplay/lobby?edition=cosmoclash');
    expect(h.statuses[0]).toEqual(['connecting', 'Connecting to the room list…']);
  });
  it('gives each snapshot, newest first as the server sends them, and leaves out malformed entries', () => {
    const h = harness();
    h.message({ type: 'rooms', rooms: [] });
    expect(h.watcher.state).toBe('open');
    h.message({ type: 'rooms', rooms: [room('b', 'Second', 9), room('a', '  Red   Blue ', 3), { id: 'A'.repeat(32), name: 'Upper' }, { id: 'c'.repeat(32), name: 'x​y' }, { id: 'd'.repeat(32) }, null, room('e', 'C engine')] });
    expect(h.lists).toHaveLength(2);
    expect(h.lists[1]).toEqual([
      { id: 'b'.repeat(32), name: 'Second', engine: 'rust', created: 9 },
      { id: 'a'.repeat(32), name: 'Red Blue', engine: 'rust', created: 3 },
      { id: 'e'.repeat(32), name: 'C engine', engine: 'rust', created: 5 },
    ]);
    expect(lobbyRooms(Array.from({ length: 60 }, (_, i) => room(String(i % 10), 'R' + i)))).toHaveLength(50);
    expect(lobbyRooms([{ id: 'f'.repeat(32), name: 'n', engine: 'other', created: 1.5 }])).toEqual([{ id: 'f'.repeat(32), name: 'n', engine: null, created: 0 }]);
  });
  it('pings every 30 s, ignores pong, and reconnects when a ping gets no answer', () => {
    const h = harness();
    h.message({ type: 'rooms', rooms: [] });
    h.advance(KEEPALIVE_MS);
    expect(h.socket.sent).toEqual(['ping']);
    h.message('pong');
    h.advance(KEEPALIVE_MS);
    expect(h.socket.sent).toEqual(['ping', 'ping']);
    expect(h.lists).toHaveLength(1);
    // No answer to the second ping: the list is lost and the watcher tries again.
    h.advance(KEEPALIVE_MS);
    expect(h.statuses.at(-1)).toEqual(['error', 'The room list stopped answering. Reconnecting…']);
    expect(h.sockets[0].closed).toBe(true);
    h.advance(1000);
    expect(h.sockets).toHaveLength(2);
    expect(h.statuses.at(-1)).toEqual(['connecting', 'Reconnecting to the room list…']);
  });
  it('retries lost connections after 1, 2, 4, 8, then every 15 seconds, and resets after a list', () => {
    const h = harness();
    const waits: number[] = [];
    for (let attempt = 0; attempt < 6; attempt++) {
      h.socket.onclose?.({});
      const before = h.sockets.length;
      let waited = 0;
      while (h.sockets.length === before) { h.advance(500); waited += 500; }
      waits.push(waited);
    }
    expect(waits).toEqual([1000, 2000, 4000, 8000, 15000, 15000]);
    h.message({ type: 'rooms', rooms: [] });
    h.socket.onerror?.({});
    expect(h.statuses.at(-1)).toEqual(['error', 'Could not reach the room list. Trying again…']);
    h.advance(1000);
    expect(h.sockets).toHaveLength(8);
  });
  it.each(['Invalid room URL or role.', 'Invalid lobby URL.'])('a relay without room lists (%s) is unsupported, with no retry', message => {
    const h = harness();
    h.message({ type: 'error', message });
    expect(h.watcher.state).toBe('unsupported');
    expect(h.statuses.at(-1)).toEqual(['unsupported', message]);
    h.advance(60000);
    expect(h.sockets).toHaveLength(1);
    expect(h.pending()).toBe(0);
  });
  it('other errors and the same error after a list are retried', () => {
    const h = harness();
    h.message({ type: 'error', message: 'Connection limit reached.' });
    expect(h.statuses.at(-1)).toEqual(['error', 'Connection limit reached. Trying again…']);
    h.advance(1000);
    h.message({ type: 'rooms', rooms: [] });
    h.message({ type: 'error', message: 'Invalid lobby URL.' });
    expect(h.watcher.state).toBe('error');
    h.message('{not json');
    h.advance(1000);
    expect(h.sockets.length).toBeGreaterThan(2);
  });
  it('an unexpected message or no answer at all drops the connection', () => {
    const h = harness();
    h.message({ type: 'other' });
    expect(h.statuses.at(-1)).toEqual(['error', 'The room list sent an unexpected message. Reconnecting…']);
    h.advance(1000);
    h.advance(12000);
    expect(h.statuses.at(-1)).toEqual(['error', 'Could not reach the room list. Trying again…']);
  });
  it('close() stops every timer and the socket', () => {
    const h = harness();
    h.message({ type: 'rooms', rooms: [] });
    h.watcher.close();
    expect(h.socket.closed).toBe(true);
    expect(h.statuses.at(-1)).toEqual(['closed', 'Left the room list.']);
    expect(h.pending()).toBe(0);
    h.message({ type: 'rooms', rooms: [room('a', 'Late')] });
    expect(h.lists).toHaveLength(1);
  });
});
