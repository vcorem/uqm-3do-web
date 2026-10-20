// @vitest-environment node
import { afterEach, describe, expect, it, vi } from 'vitest';
// These adapters run the real relay implementations, not mocked transports.
// @ts-ignore JavaScript relay contract is exercised at runtime.
import { createNetplayServer } from '../../../server/netplay.mjs';
// @ts-ignore JavaScript subprocess adapter has no TypeScript declarations.
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
// @ts-ignore ws is provided by the repository's root dependencies.
import { WebSocket } from 'ws';
import { CosmoNetplay, createRoomId, invitationURL, parseRoom, readInvite, roomURL, PROTOCOL_VERSION, INPUT_DELAY, BEAT_MS, QUIET_PEER_MS, NEXT_MS, NEXT_WAIT_MS, NEXT_ACK_MS, type Frame, type NetplayOptions, type NextBattle, type SocketLike } from './CosmoNetplay';
const cleanup: (() => Promise<void> | void)[] = [];
afterEach(async () => { for (const fn of cleanup.splice(0).reverse()) await fn(); });
const sleep = (ms = 2) => new Promise(resolve => setTimeout(resolve, ms));
async function until(predicate: () => boolean, detail: () => unknown = () => '', ms = 6000) {
  const end = Date.now() + ms;
  while (!predicate()) { if (Date.now() >= end) throw new Error(`Timed out: ${JSON.stringify(detail())}`); await sleep(); }
}
async function pair(kind = 'node', jitter = false, guestBuild = 'test-rust-v1', extra: Partial<NetplayOptions> = {}) {
  const app = (kind === 'rust' ? createRustRelay : createNetplayServer)({distDir: false, latencyPatternMs: jitter ? [20, 0, 9, 2] : [0]});
  const address = await app.listen(0); cleanup.push(() => app.close());
  const origin = `http://127.0.0.1:${address.port}`; const room = createRoomId();
  const sockets: any[] = []; const starts: any[][] = [[], []]; const frames: Frame[][] = [[], []];
  let peers: CosmoNetplay[] = [];
  let endAt = -1; let corruptHash = false;
  // Protocol 3: the battle that ends at this tick needs a choice of the next ship.
  let pause: { tick: number; battle: number; player: 0 | 1; ships: number[] } | null = null;
  const nexts: (NextBattle & { tick: number })[][] = [[], []];
  peers = [0, 1].map(index => new CosmoNetplay({
    buildId: index ? guestBuild : 'test-rust-v1', randomSeed: () => 123456789, ...extra,
    socketFactory: url => { const ws = new WebSocket(url, {origin}); sockets[index] = ws; return ws as SocketLike; },
    onStart: value => { starts[index].push(value); frames[index] = []; },
    onNext: next => nexts[index].push({ ...next, tick: peers[index].state.tick }),
    onFrame: frame => {
      frames[index].push(frame);
      const pausing = pause?.tick === frame.tick;
      if (frame.tick % 30 === 0 || frame.tick === endAt || pausing) peers[index].reportChecksum(frame.tick, (corruptHash && index ? 'bad' : 'ok') + frame.tick);
      if (frame.tick === endAt) peers[index].finish(0);
      if (pausing) peers[index].awaitNext(pause!);
    },
  }));
  cleanup.push(() => { peers.forEach(peer => peer.close()); });
  peers[0].connect({origin, room, role: 'host'});
  await until(() => peers[0].state.phase === 'waiting');
  peers[1].connect({origin, room, role: 'guest'});
  const states = () => peers.map(peer => peer.state);
  await until(() => peers.every(peer => peer.state.phase === 'lobby' && peer.state.peerFleetKnown) || peers.some(peer => peer.state.phase === 'error'), states);
  return {peers, sockets, states, starts, frames, origin, app, room, nexts,
    endAt(tick: number) { endAt = tick; }, corruptHash() { corruptHash = true; },
    pauseAt(tick: number, player: 0 | 1, ships: number[], battle = 1) { pause = { tick, battle, player, ships }; },
    async start() {
      peers[0].chooseFleet([2, 0]); peers[1].chooseFleet([1]);
      // Fleet and ready packets share one ordered WebSocket stream.
      peers.forEach(peer => peer.ready());
      await until(() => peers.every(peer => peer.state.phase === 'playing') || peers.some(peer => peer.state.phase === 'error'), states);
      expect(peers.map(peer => peer.state.phase)).toEqual(['playing', 'playing']);
    },
    async pump(tick: number) {
      const deadline = Date.now() + 10000;
      while (peers.some(peer => peer.state.tick < tick && ['playing', 'choosing'].includes(peer.state.phase)) && !peers.every(peer => peer.state.phase === 'choosing')) {
        if (Date.now() > deadline) throw new Error(`Stalled: ${JSON.stringify(states())}`);
        peers.forEach((peer, index) => peer.advance((peer.state.tick * (index + 3)) & 31));
        if (peers.some(peer => peer.state.phase === 'error')) throw new Error(JSON.stringify(states()));
        await sleep(6);
      }
    },
  };
}
describe.each(['node', 'rust'])('real %s relay', kind => {
  it.each([false, true])('agrees seed/fleets, locksteps inputs, checksums, finish and rematch (jitter=%s)', async jitter => {
    const p = await pair(kind, jitter); p.endAt(299); await p.start();
    expect(p.starts[0][0]).toEqual({round: 0, seed: 123456789, fleets: [[2, 0], [1]], localPlayer: 0});
    expect(p.starts[1][0]).toEqual({...p.starts[0][0], localPlayer: 1});
    await p.pump(300); await until(() => p.peers.every(peer => peer.state.phase === 'ended'), p.states);
    expect(p.frames[0]).toEqual(p.frames[1]); expect(p.frames[0]).toHaveLength(300);
    expect(p.frames[0].slice(0, 3).map(frame => frame.inputs)).toEqual([[0, 0], [0, 0], [0, 0]]);
    expect(p.peers.map(peer => peer.state.verifiedChecksums)).toEqual([11, 11]);
    p.peers.forEach(peer => peer.rematch());
    await until(() => p.peers.every(peer => peer.state.phase === 'lobby' && peer.state.round === 1), p.states);
    p.endAt(89); await p.start(); await p.pump(90);
    await until(() => p.peers.every(peer => peer.state.phase === 'ended'), p.states);
    expect(p.frames[0]).toEqual(p.frames[1]); expect(p.starts[0]).toHaveLength(2);
    p.peers[0].close(); await until(() => p.peers[1].state.phase === 'closed', p.states);
  }, 20000);
  it('rejects different builds before any simulation begins', async () => {
    const p = await pair(kind, false, 'different-wasm');
    await until(() => p.peers.some(peer => peer.state.phase === 'error'), p.states);
    expect(p.starts).toEqual([[], []]); expect(p.peers.map(peer => peer.state.message).join(' ')).toMatch(/versions differ/);
  });
  it.each(['lobby', 'playing'])('closes safely when the peer leaves during %s', async phase => {
    const p = await pair(kind); if (phase === 'playing') { await p.start(); await p.pump(30); }
    const before = p.peers[1].state.tick; p.peers[0].close();
    await until(() => p.peers[1].state.phase === 'closed', p.states);
    expect(p.peers[1].advance(31)).toBe(false); expect(p.peers[1].state.tick).toBe(before);
  });
});
it.each([
  {type: 'input', round: 0, tick: 4, input: 1}, // first remote tick must be 3
  {type: 'input', round: 8, tick: 3, input: 1},
  {type: 'input', round: 0, tick: 3, input: 32},
  {type: 'input', round: 0, tick: 3, input: 1, hidden: true},
  {type: 'hash', round: 0, tick: 999, hash: 'future'},
  {type: 'finish', round: 0, tick: 1e9, winner: 0},
  {type: 'start', round: 0, seed: 1, fleets: [[0], [0]]},
])('fails closed on malformed, stale or out-of-order game packets: %j', async packet => {
  const p = await pair(); await p.start(); p.sockets[0].send(Buffer.from(JSON.stringify(packet)));
  await until(() => p.peers[1].state.phase === 'error', p.states); expect(p.peers[1].state.tick).toBe(0);
});
it.each([Buffer.from([255]), Buffer.from('null'), Buffer.from('[]'), Buffer.alloc(2049, 32)])('rejects non-JSON, invalid UTF-8 and oversized protocol messages', async bytes => {
  const p = await pair(); p.sockets[0].send(bytes); await until(() => p.peers[1].state.phase === 'error', p.states);
});
it('fails closed on a lobby packet flood before unbounded work accumulates', async () => {
  const p = await pair(); const packet = Buffer.from(JSON.stringify({type: 'fleet', round: 0, ships: [0]}));
  for (let n = 0; n < 251; n++) p.sockets[0].send(packet);
  await until(() => p.peers[1].state.phase === 'error', p.states);
});
it('stops both players on a real state checksum mismatch', async () => {
  const p = await pair(); await p.start(); p.corruptHash(); p.peers.forEach(peer => peer.advance(0));
  await until(() => p.peers.some(peer => peer.state.phase === 'error'), p.states);
  expect(p.peers.map(peer => peer.state.message).join(' ')).toMatch(/states differ/);
});
it('stalls rather than guessing remote input, with a bounded send window', async () => {
  const p = await pair(); await p.start();
  for (let n = 0; n < 10000; n++) p.peers[0].advance(31);
  expect(p.peers[0].state.tick).toBe(3); expect(p.peers[1].state.tick).toBe(0);
  await p.pump(60); expect(p.frames[0].slice(0, 60)).toEqual(p.frames[1].slice(0, 60));
});
it('outgoing backpressure is bounded and closes the room', async () => {
  const p = await pair(); await p.start();
  Object.defineProperty(p.sockets[0], 'bufferedAmount', {get: () => 65536});
  expect(p.peers[0].advance(1)).toBe(false); expect(p.peers[0].state.phase).toBe('error');
  await until(() => p.peers[1].state.phase === 'closed', p.states);
});
it('room invitations cannot change the relay host or the namespace', () => {
  const room = createRoomId(); expect(room).toMatch(/^[0-9a-f]{32}$/);
  expect(parseRoom(room)).toBe(room); expect(parseRoom(`https://another.example/?cosmoRoom=${room}`)).toBe(room);
  expect(parseRoom('javascript:alert(1)')).toBe(null); expect(parseRoom('A'.repeat(32))).toBe(null);
  expect(new URL(invitationURL(room, 'https://game.example/cosmo/?x=1#old')).searchParams.get('cosmoRoom')).toBe(room);
  // The contract: /cosmoclash/?cosmoRoom=<32 lowercase hex>.
  expect(invitationURL(room, 'https://urquanmasters.org/cosmoclash/?other=1#x')).toBe(`https://urquanmasters.org/cosmoclash/?cosmoRoom=${room}`);
  expect(readInvite(`https://urquanmasters.org/cosmoclash/?cosmoRoom=${room}`)).toBe(room);
  expect(readInvite('https://urquanmasters.org/cosmoclash/?cosmoRoom=' + 'A'.repeat(32))).toBe(null);
  expect(readInvite('https://urquanmasters.org/cosmoclash/')).toBe(null);
});
it('room URLs: a listed host (name and engine=rust), a private host and a guest', () => {
  const room = 'ab'.repeat(16);
  expect(roomURL('https://urquanmasters.org/cosmoclash/', room, 'host', '  Friday   duel ⚔ ')).toBe(`wss://urquanmasters.org/netplay/cosmoclash/${room}?role=host&name=Friday%20duel%20%E2%9A%94&engine=rust`);
  expect(roomURL('http://127.0.0.1:10000/cosmoclash/', room, 'host')).toBe(`ws://127.0.0.1:10000/netplay/cosmoclash/${room}?role=host`);
  expect(roomURL('http://127.0.0.1:10000/cosmoclash/', room, 'guest')).toBe(`ws://127.0.0.1:10000/netplay/cosmoclash/${room}?role=guest`);
  expect(() => roomURL('http://h/', room, 'guest', 'Name')).toThrow(/Only the host/);
  expect(() => roomURL('http://h/', room, 'host', 'x'.repeat(25))).toThrow(/Room names need 1 to 24 visible characters/);
  expect(() => roomURL('http://h/', 'A'.repeat(32), 'host')).toThrow(/Invalid/);
  expect(() => roomURL('http://user:pw@h/', room, 'host')).toThrow(/Invalid/);
  expect(PROTOCOL_VERSION).toBe(3);
});

/** One peer against a scripted relay socket (no network): control messages and keepalive. */
function scripted(options: { name?: string; keepaliveMs?: number; role?: 'host' | 'guest' } = {}) {
  const sent: (string | Record<string, unknown>)[] = []; const states: string[] = []; const nexts: NextBattle[] = [];
  const socket: SocketLike & { closed: boolean; url: string } = { url: '', closed: false, readyState: 1, binaryType: '', bufferedAmount: 0,
    onopen: null, onclose: null, onerror: null, onmessage: null, close() { this.closed = true; },
    send(data) { sent.push(typeof data === 'string' ? data : JSON.parse(new TextDecoder().decode(data))); } };
  const peer = new CosmoNetplay({ buildId: 'scripted', keepaliveMs: options.keepaliveMs, onStart: () => {}, onFrame: () => {},
    onNext: next => nexts.push(next),
    onState: state => states.push(state.phase), socketFactory: url => { socket.url = url; return socket; } });
  peer.connect({ origin: 'http://127.0.0.1:1/cosmoclash/', room: 'cd'.repeat(16), role: options.role ?? 'host', name: options.name });
  socket.onopen?.({});
  return { peer, socket, sent, states, nexts,
    relay(value: unknown) { socket.onmessage?.({ data: typeof value === 'string' ? value : JSON.stringify(value) }); },
    game(value: unknown) { socket.onmessage?.({ data: new TextEncoder().encode(JSON.stringify(value)) }); } };
}
it('a listed host pings while its socket is open, ignores pong and fails without an answer', async () => {
  const p = scripted({ name: 'Ping room', keepaliveMs: 20 });
  expect(p.socket.url).toContain('?role=host&name=Ping%20room&engine=rust');
  expect(p.peer.state.name).toBe('Ping room');
  p.relay({ type: 'waiting' });
  expect(p.peer.state.phase).toBe('waiting');
  expect(p.peer.state.message).toBe('Room “Ping room” is open. Waiting for an opponent…');
  await sleep(30); expect(p.sent).toEqual(['ping']);
  p.relay('pong'); await sleep(25); expect(p.sent).toEqual(['ping', 'ping']);
  expect(p.peer.state.phase).toBe('waiting');
  await sleep(30);
  expect(p.peer.state.phase).toBe('error'); expect(p.peer.state.message).toBe('Lost the connection to online play.');
  expect(p.socket.closed).toBe(true);
});
it('a private host and a guest never ping (relays without room lists refuse text)', async () => {
  for (const role of ['host', 'guest'] as const) {
    const p = scripted({ role, keepaliveMs: 10 });
    p.relay({ type: 'waiting' }); await sleep(40);
    expect(p.sent).toEqual([]); expect(p.peer.state.phase).toBe('waiting'); p.peer.close();
  }
});
it('relay control messages: waiting, ready, error and peer-left', () => {
  const p = scripted();
  p.relay({ type: 'waiting' }); p.relay({ type: 'waiting' });
  expect(p.peer.state.message).toBe('Waiting for your opponent to join…');
  p.relay({ type: 'ready', role: 'host' });
  expect(p.peer.state.phase).toBe('handshake');
  expect(p.sent).toEqual([{ type: 'hello', version: 3, build: 'scripted', role: 'host', delay: INPUT_DELAY }]);
  p.relay({ type: 'peer-left', message: 'The other player disconnected.' });
  expect(p.peer.state).toMatchObject({ phase: 'closed', message: 'Your opponent left the room.' });
  const e = scripted(); e.relay({ type: 'error', message: 'Room already exists or has expired. Create a new room.' });
  expect(e.peer.state).toMatchObject({ phase: 'error', message: 'Room already exists or has expired. Create a new room.' });
  const r = scripted(); r.relay({ type: 'waiting' }); r.relay({ type: 'ready', role: 'guest' });
  expect(r.peer.state).toMatchObject({ phase: 'error', message: 'The room assigned an unexpected player role.' });
  const u = scripted(); u.relay({ type: 'surprise' });
  expect(u.peer.state.phase).toBe('error');
  const n = scripted(); n.relay('not json');
  expect(n.peer.state.phase).toBe('error');
});
it('protocol 2: the picks so far go to the other player, and Ready needs one ship', () => {
  const p = scripted();
  p.relay({ type: 'waiting' }); p.relay({ type: 'ready', role: 'host' });
  p.game({ type: 'hello', version: 3, build: 'scripted', role: 'guest', delay: INPUT_DELAY });
  expect(p.peer.state.phase).toBe('lobby');
  expect(p.sent.at(-1)).toEqual({ type: 'fleet', round: 0, ships: [] });
  p.peer.ready(); expect(p.peer.state.localReady).toBe(false);
  p.peer.chooseFleet([2]); expect(p.sent.at(-1)).toEqual({ type: 'fleet', round: 0, ships: [2] });
  p.peer.chooseFleet([2, 0]);
  expect(() => p.peer.chooseFleet([1, 1])).toThrow(); expect(() => p.peer.chooseFleet([0, 1, 2, 0])).toThrow();
  p.game({ type: 'fleet', round: 0, ships: [] });
  expect(p.peer.state).toMatchObject({ peerFleet: [], peerFleetKnown: true });
  p.game({ type: 'fleet', round: 0, ships: [1] });
  expect(p.peer.state.peerFleet).toEqual([1]);
  p.peer.ready(); expect(p.sent.at(-1)).toEqual({ type: 'ready', round: 0 });
  p.peer.chooseFleet([0]); expect(p.peer.state.localFleet).toEqual([2, 0]);
  // A Ready from a player without ships is invalid.
  const q = scripted(); q.relay({ type: 'waiting' }); q.relay({ type: 'ready', role: 'host' });
  q.game({ type: 'hello', version: 3, build: 'scripted', role: 'guest', delay: INPUT_DELAY });
  q.game({ type: 'fleet', round: 0, ships: [] }); q.game({ type: 'ready', round: 0 });
  expect(q.peer.state.phase).toBe('error');
  // Protocol 2 clients (fleets in a fixed order) cannot play protocol 3 clients.
  const o = scripted(); o.relay({ type: 'waiting' }); o.relay({ type: 'ready', role: 'host' });
  o.game({ type: 'hello', version: 2, build: 'scripted', role: 'guest', delay: INPUT_DELAY });
  expect(o.peer.state.message).toMatch(/versions differ/);
  expect(o.peer.state.phase).toBe('error');
});

it('requires a final checksum and rejects conflicting fleet match winners', async () => {
  const p = await pair(); await p.start();
  p.peers.forEach(peer => peer.advance(0));
  p.peers.forEach(peer => peer.advance(0));
  expect(() => p.peers[0].finish(0)).toThrow(/final state checksum/);
  p.peers.forEach(peer => peer.reportChecksum(1, 'final'));
  p.peers[0].finish(0); p.peers[1].finish(1);
  await until(() => p.peers.some(peer => peer.state.phase === 'error'), p.states);
  expect(p.peers.map(peer => peer.state.message).join(' ')).toMatch(/different match results/);
});
it('a failed final-result send remains terminal and cannot become a rematch lobby', async () => {
  const p = await pair(); await p.start(); p.peers.forEach(peer => peer.advance(0));
  Object.defineProperty(p.sockets[0], 'bufferedAmount', {get: () => 65536});
  p.peers[0].finish(1); expect(p.peers[0].state.phase).toBe('error');
  p.peers[0].rematch(); expect(p.peers[0].state.phase).toBe('error');
});
it('a failed GO send never starts either simulation', async () => {
  const p = await pair(); const original = p.sockets[0].send.bind(p.sockets[0]);
  p.sockets[0].send = (bytes: Uint8Array) => {
    if (JSON.parse(new TextDecoder().decode(bytes)).type === 'go') throw new Error('send failed');
    original(bytes);
  };
  p.peers.forEach(peer => { peer.chooseFleet([0]); peer.ready(); });
  await until(() => p.peers[0].state.phase === 'error', p.states);
  await until(() => p.peers[1].state.phase === 'closed', p.states);
  expect(p.starts).toEqual([[], []]);
});

describe.each(['node', 'rust'])('beats and Change fleet through the real %s relay', kind => {
  it('a slow fleet choice and a long look at the result stay open with beats; without them the room ends', async () => {
    const p = await pair(kind, false, 'test-rust-v1', { beatMs: 40, timeoutMs: 500 });
    await sleep(1200);
    expect(p.peers.map(peer => peer.state.phase)).toEqual(['lobby', 'lobby']);
    p.endAt(59); await p.start(); await p.pump(60);
    await until(() => p.peers.every(peer => peer.state.phase === 'ended'), p.states);
    await sleep(1200);
    expect(p.peers.map(peer => peer.state.phase)).toEqual(['ended', 'ended']);
    p.peers.forEach(peer => peer.rematch());
    await until(() => p.peers.every(peer => peer.state.phase === 'lobby' && peer.state.round === 1), p.states);
    const q = await pair(kind, false, 'test-rust-v1', { beatMs: 600000, timeoutMs: 500 });
    await until(() => q.peers.some(peer => peer.state.phase === 'error'), q.states, 4000);
    expect(q.peers.map(peer => peer.state.message)).toContain('Your opponent stopped responding.');
  }, 20000);
  it('the host and the guest take Ready back; then the match starts with the new fleets', async () => {
    const p = await pair(kind);
    p.peers[0].chooseFleet([0]); p.peers[0].ready();
    await until(() => p.peers[1].state.peerReady, p.states);
    p.peers[0].unready();
    expect(p.peers[0].state.localReady).toBe(false);
    await until(() => !p.peers[1].state.peerReady, p.states);
    p.peers[1].chooseFleet([1]); p.peers[1].ready();
    await until(() => p.peers[0].state.peerReady, p.states);
    p.peers[1].unready();
    expect(p.peers[1].state).toMatchObject({ localReady: true, changing: true });
    await until(() => !p.peers[1].state.localReady && !p.peers[1].state.changing && !p.peers[0].state.peerReady, p.states);
    p.peers[1].chooseFleet([2, 1]);
    await until(() => p.peers[0].state.peerFleet.join() === '2,1', p.states);
    p.peers.forEach(peer => peer.ready());
    await until(() => p.peers.every(peer => peer.state.phase === 'playing') || p.peers.some(peer => peer.state.phase === 'error'), p.states);
    expect(p.starts[0][0].fleets).toEqual([[0], [2, 1]]);
    expect(p.starts[1][0].fleets).toEqual([[0], [2, 1]]);
  });
});

/** A host or a guest in the fleet choice against a scripted relay; the other player picked one ship. */
function inLobby(role: 'host' | 'guest') {
  const p = scripted({ role });
  p.relay({ type: 'waiting' }); p.relay({ type: 'ready', role });
  p.game({ type: 'hello', version: 3, build: 'scripted', role: role === 'host' ? 'guest' : 'host', delay: INPUT_DELAY });
  p.game({ type: 'fleet', round: 0, ships: [1] });
  return p;
}
const sentTypes = (p: ReturnType<typeof scripted>, type: string) => p.sent.filter(item => typeof item === 'object' && item.type === type);
it('Change fleet: a host takes Ready back at once and answers a guest; a start that crossed the request wins', () => {
  const h = inLobby('host');
  h.peer.chooseFleet([0]); h.peer.ready();
  h.peer.unready();
  expect(h.peer.state).toMatchObject({ localReady: false, changing: false });
  expect(h.sent.at(-1)).toEqual({ type: 'unready', round: 0 });
  h.peer.chooseFleet([2]); expect(h.peer.state.localFleet).toEqual([2]);
  h.game({ type: 'ready', round: 0 }); expect(h.peer.state.peerReady).toBe(true);
  h.game({ type: 'unready', round: 0 });
  expect(h.peer.state.peerReady).toBe(false);
  expect(h.sent.at(-1)).toEqual({ type: 'unready-ok', round: 0 });
  h.game({ type: 'fleet', round: 0, ships: [0, 1] }); expect(h.peer.state.peerFleet).toEqual([0, 1]);
  h.game({ type: 'ready', round: 0 }); h.peer.ready();
  expect(h.peer.state.phase).toBe('starting');
  h.game({ type: 'unready', round: 0 });
  expect(h.peer.state.phase).toBe('starting');
  // Not ready, nothing to take back: refused.
  const bad = inLobby('host'); bad.game({ type: 'unready', round: 0 }); expect(bad.peer.state.phase).toBe('error');
  const late = inLobby('host'); late.game({ type: 'unready-ok', round: 0 }); expect(late.peer.state.phase).toBe('error');
});
it('Change fleet: a guest asks, keeps its fleet until the host answers, and starts when the start crossed the request', () => {
  const g = inLobby('guest');
  g.peer.chooseFleet([2]); g.peer.ready();
  g.peer.unready();
  expect(g.peer.state).toMatchObject({ localReady: true, changing: true });
  g.peer.chooseFleet([0]); expect(g.peer.state.localFleet).toEqual([2]);
  g.peer.unready(); expect(sentTypes(g, 'unready')).toHaveLength(1);
  g.game({ type: 'unready-ok', round: 0 });
  expect(g.peer.state).toMatchObject({ localReady: false, changing: false });
  // The host's own Change fleet.
  g.game({ type: 'ready', round: 0 }); expect(g.peer.state.peerReady).toBe(true);
  g.game({ type: 'unready', round: 0 }); expect(g.peer.state.peerReady).toBe(false);
  const c = inLobby('guest');
  c.peer.chooseFleet([2]); c.peer.ready(); c.game({ type: 'ready', round: 0 }); c.peer.unready();
  c.game({ type: 'start', round: 0, seed: 5, fleets: [[1], [2]] });
  expect(c.peer.state).toMatchObject({ phase: 'starting', changing: false, localReady: true });
  const d = inLobby('guest'); d.game({ type: 'unready-ok', round: 0 }); expect(d.peer.state.phase).toBe('error');
});
it('peer-left: the other player left, or the relay ended the room and says why', () => {
  const a = scripted(); a.relay({ type: 'waiting' });
  a.relay({ type: 'peer-left', message: 'The other player disconnected.' });
  expect(a.peer.state).toMatchObject({ phase: 'closed', message: 'Your opponent left the room.' });
  for (const reason of ['Server is stopping.', 'Room time limit reached.', 'Room was idle for too long.']) {
    const p = scripted(); p.relay({ type: 'waiting' }); p.relay({ type: 'peer-left', message: reason });
    expect(p.peer.state).toMatchObject({ phase: 'closed', message: `The room closed. ${reason}` });
  }
  const n = scripted(); n.relay({ type: 'peer-left' });
  expect(n.peer.state).toMatchObject({ phase: 'closed', message: 'Your opponent left the room.' });
});
it('a relay that stops tells both players why (not that the other player left)', async () => {
  const p = await pair();
  await p.app.close();
  await until(() => p.peers.every(peer => peer.state.phase === 'closed' || peer.state.phase === 'error'), p.states);
  expect(p.peers.map(peer => peer.state.message)).toEqual(['The room closed. Server is stopping.', 'The room closed. Server is stopping.']);
});
it('beats: one every 20 s in the fleet choice; the beats of the opponent keep a 10-minute choice open; 2 minutes of silence end it', () => {
  vi.useFakeTimers();
  try {
    const p = inLobby('host');
    expect(p.peer.state.phase).toBe('lobby');
    for (let n = 0; n < 30; n++) { vi.advanceTimersByTime(BEAT_MS); p.game({ type: 'beat', round: 0 }); }
    expect(p.peer.state.phase).toBe('lobby');
    expect(sentTypes(p, 'beat').length).toBeGreaterThanOrEqual(29);
    expect(sentTypes(p, 'beat')[0]).toEqual({ type: 'beat', round: 0 });
    vi.advanceTimersByTime(QUIET_PEER_MS - 5000);
    expect(p.peer.state.phase).toBe('lobby');
    vi.advanceTimersByTime(7000);
    expect(p.peer.state).toMatchObject({ phase: 'error', message: 'Your opponent stopped responding.' });
  } finally { vi.useRealTimers(); }
});
it('beats: the last round is accepted just after a rematch; other rounds, extra fields and early beats are refused', () => {
  const ok = inLobby('host'); ok.game({ type: 'beat', round: 0 }); expect(ok.peer.state.phase).toBe('lobby');
  for (const packet of [{ type: 'beat', round: 1 }, { type: 'beat', round: 0, x: 1 }, { type: 'beat' }]) {
    const p = inLobby('host'); p.game(packet); expect(p.peer.state.phase).toBe('error');
  }
  const early = scripted(); early.relay({ type: 'waiting' }); early.relay({ type: 'ready', role: 'host' });
  early.game({ type: 'beat', round: 0 }); expect(early.peer.state.phase).toBe('error');
});

describe.each(['node', 'rust'])('protocol 3: the next ship through the real %s relay', kind => {
  it('the loser chooses; both peers stop at the same tick, go on at the next tick with the same ship, and keep the same frames and checksums', async () => {
    const p = await pair(kind, kind === 'rust');
    p.pauseAt(59, 1, [0, 2]); p.endAt(149); await p.start();
    await p.pump(150);
    await until(() => p.peers.every(peer => peer.state.phase === 'choosing'), p.states);
    expect(p.peers.map(peer => peer.state.tick)).toEqual([60, 60]);
    expect(p.peers.map(peer => peer.state.next)).toEqual([0, 1].map(() => ({ battle: 1, player: 1, ships: [0, 2], chosen: null })));
    expect(p.peers.map(peer => peer.state.message)).toEqual(['Your opponent is choosing the next ship.', 'Choose your next ship.']);
    // The lockstep waits: no frame, and the winner cannot choose for the loser.
    for (let n = 0; n < 20; n++) p.peers.forEach(peer => expect(peer.advance(31)).toBe(false));
    p.peers[0].chooseNext(0);
    expect(p.peers[0].state.next?.chosen).toBe(null);
    expect(() => p.peers[1].chooseNext(1)).toThrow();
    await sleep(30);
    expect(p.peers.map(peer => peer.state.phase)).toEqual(['choosing', 'choosing']);
    p.peers[1].chooseNext(2);
    expect(p.peers[1].state.next?.chosen).toBe(2);
    await until(() => p.peers.every(peer => peer.state.phase === 'playing'), p.states);
    expect(p.nexts).toEqual([[{ battle: 1, player: 1, ship: 2, tick: 60 }], [{ battle: 1, player: 1, ship: 2, tick: 60 }]]);
    expect(p.peers.map(peer => peer.state.next)).toEqual([null, null]);
    await p.pump(150); await until(() => p.peers.every(peer => peer.state.phase === 'ended'), p.states);
    expect(p.frames[0]).toEqual(p.frames[1]); expect(p.frames[0]).toHaveLength(150);
    expect(p.peers.map(peer => peer.state.verifiedChecksums)).toEqual([7, 7]);
  }, 20000);
});

/** A host or a guest in a running battle against a scripted relay. Its fleet is [0, 2, 1]; the other player has [1]. */
function playing(role: 'host' | 'guest') {
  const p = inLobby(role);
  p.peer.chooseFleet([0, 2, 1]); p.peer.ready();
  p.game({ type: 'ready', round: 0 });
  if (role === 'host') p.game({ type: 'ack', round: 0 });
  else { p.game({ type: 'start', round: 0, seed: 5, fleets: [[1], [0, 2, 1]] }); p.game({ type: 'go', round: 0 }); }
  expect(p.peer.state.phase).toBe('playing');
  return p;
}
const local = (role: 'host' | 'guest') => (role === 'host' ? 0 : 1) as 0 | 1;
it.each(['host', 'guest'] as const)('protocol 3 (%s): the loser sends next and goes on with next-ack; the winner answers next-ack and goes on', role => {
  const loser = playing(role);
  loser.peer.awaitNext({ battle: 1, player: local(role), ships: [2, 1] });
  expect(loser.peer.state).toMatchObject({ phase: 'choosing', next: { battle: 1, player: local(role), ships: [2, 1], chosen: null } });
  expect(loser.peer.advance(0)).toBe(false);
  expect(() => loser.peer.chooseNext(0)).toThrow();
  loser.peer.chooseNext(1);
  expect(loser.sent.at(-1)).toEqual({ type: 'next', round: 0, battle: 1, ship: 1 });
  loser.peer.chooseNext(2);
  expect(sentTypes(loser, 'next')).toHaveLength(1);
  expect(loser.peer.state.phase).toBe('choosing');
  loser.game({ type: 'next-ack', round: 0, battle: 1 });
  expect(loser.peer.state).toMatchObject({ phase: 'playing', next: null });
  expect(loser.nexts).toEqual([{ battle: 1, player: local(role), ship: 1 }]);
  // The winner waits for the loser's packet and never chooses.
  const winner = playing(role);
  const other = (1 - local(role)) as 0 | 1;
  winner.peer.awaitNext({ battle: 1, player: other, ships: [0, 2] });
  winner.peer.chooseNext(0);
  expect(sentTypes(winner, 'next')).toEqual([]);
  winner.game({ type: 'next', round: 0, battle: 1, ship: 2 });
  expect(winner.sent.at(-1)).toEqual({ type: 'next-ack', round: 0, battle: 1 });
  expect(winner.peer.state).toMatchObject({ phase: 'playing', next: null });
  expect(winner.nexts).toEqual([{ battle: 1, player: other, ship: 2 }]);
  // The next choice needs a later battle.
  expect(() => winner.peer.awaitNext({ battle: 1, player: other, ships: [0, 1] })).toThrow();
  winner.peer.awaitNext({ battle: 3, player: other, ships: [0, 1] });
  expect(winner.peer.state.phase).toBe('choosing');
});
it('protocol 3: a next packet that came before this peer reached the choice is kept for it', () => {
  const p = playing('host');
  p.game({ type: 'next', round: 0, battle: 1, ship: 0 });
  expect(p.peer.state.phase).toBe('playing');
  expect(p.nexts).toEqual([]);
  p.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
  expect(p.sent.at(-1)).toEqual({ type: 'next-ack', round: 0, battle: 1 });
  expect(p.peer.state.phase).toBe('playing');
  expect(p.nexts).toEqual([{ battle: 1, player: 1, ship: 0 }]);
  // An early choice that does not fit the choice stops the match.
  const bad = playing('host');
  bad.game({ type: 'next', round: 0, battle: 1, ship: 1 });
  bad.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
  expect(bad.peer.state).toMatchObject({ phase: 'error', message: 'Invalid or incompatible game data received.' });
  const twice = playing('host');
  twice.game({ type: 'next', round: 0, battle: 1, ship: 0 }); twice.game({ type: 'next', round: 0, battle: 1, ship: 0 });
  expect(twice.peer.state.phase).toBe('error');
});
it.each([
  ['the winner sends next', 'loser-local', { type: 'next', round: 0, battle: 1, ship: 2 }],
  ['a ship that is not left', 'peer', { type: 'next', round: 0, battle: 1, ship: 1 }],
  ['another battle', 'peer', { type: 'next', round: 0, battle: 2, ship: 2 }],
  ['an extra field', 'peer', { type: 'next', round: 0, battle: 1, ship: 2, x: 1 }],
  ['another round', 'peer', { type: 'next', round: 1, battle: 1, ship: 2 }],
  ['next-ack without a choice', 'loser-local', { type: 'next-ack', round: 0, battle: 1 }],
  ['next-ack to the waiting peer', 'peer', { type: 'next-ack', round: 0, battle: 1 }],
  ['next-ack of another battle', 'chosen', { type: 'next-ack', round: 0, battle: 2 }],
  ['next after the end of the match', 'ended', { type: 'next', round: 0, battle: 1, ship: 2 }],
] as const)('protocol 3 refuses %s', (_name, setup, packet) => {
  const p = playing('host');
  if (setup === 'ended') {
    p.peer.advance(0); p.game({ type: 'input', round: 0, tick: 3, input: 0 });
    for (let n = 0; n < 4; n++) p.peer.advance(0);
    p.peer.reportChecksum(p.peer.state.tick - 1, 'x'); p.game({ type: 'hash', round: 0, tick: p.peer.state.tick - 1, hash: 'x' });
    p.game({ type: 'finish', round: 0, tick: p.peer.state.tick - 1, winner: 1 }); p.peer.finish(1);
    expect(p.peer.state.phase).toBe('ended');
  } else p.peer.awaitNext({ battle: 1, player: setup === 'peer' ? 1 : 0, ships: [0, 2] });
  if (setup === 'chosen') p.peer.chooseNext(2);
  p.game(packet);
  expect(p.peer.state.phase).toBe('error');
});
it('protocol 3: awaitNext needs a match and a real choice (two or more ships)', () => {
  const p = playing('guest');
  expect(() => p.peer.awaitNext({ battle: 1, player: 0, ships: [2] })).toThrow();
  expect(() => p.peer.awaitNext({ battle: 0, player: 0, ships: [0, 2] })).toThrow();
  expect(() => p.peer.awaitNext({ battle: 1, player: 0, ships: [2, 2] })).toThrow();
  const lobby = inLobby('host');
  expect(() => lobby.peer.awaitNext({ battle: 1, player: 0, ships: [0, 2] })).toThrow();
  expect(p.peer.state.phase).toBe('playing');
});
it('protocol 3: the choice has beats; the waiting peer stops when no next comes in NEXT_WAIT_MS (a full 20 s choice never ends the room); inputs and checksums of the last battle are accepted', () => {
  vi.useFakeTimers();
  try {
    expect(NEXT_WAIT_MS).toBe(NEXT_MS + 20000);
    const p = playing('host');
    p.peer.advance(0); p.peer.advance(0);
    p.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
    p.game({ type: 'input', round: 0, tick: 3, input: 1 }); p.game({ type: 'input', round: 0, tick: 4, input: 1 });
    p.game({ type: 'hash', round: 0, tick: 0, hash: 'h' });
    expect(p.peer.state.phase).toBe('choosing');
    vi.advanceTimersByTime(25000);
    expect(p.peer.state.phase).toBe('choosing');
    expect(sentTypes(p, 'beat')).toEqual([{ type: 'beat', round: 0 }]);
    // The loser's game always sends next within NEXT_MS: its beats do not keep the wait open
    // (a connection that dropped without a close ends in 40 s, not after 2 minutes of silence).
    p.game({ type: 'beat', round: 0 });
    vi.advanceTimersByTime(NEXT_WAIT_MS - 25000 - 1500);
    expect(p.peer.state.phase).toBe('choosing');
    // The limits are checked once a second.
    vi.advanceTimersByTime(3000);
    expect(p.peer.state).toMatchObject({ phase: 'error', message: 'Your opponent stopped responding.', next: null });
    // A next just before the limit: the next battle.
    const q = playing('host');
    q.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
    vi.advanceTimersByTime(NEXT_WAIT_MS - 1500);
    q.game({ type: 'next', round: 0, battle: 1, ship: 2 });
    expect(q.peer.state.phase).toBe('playing');
    expect(q.nexts).toEqual([{ battle: 1, player: 1, ship: 2 }]);
  } finally { vi.useRealTimers(); }
});
it('protocol 3: the loser has the 2-minute limit while it chooses, then NEXT_ACK_MS for next-ack', () => {
  vi.useFakeTimers();
  try {
    const p = playing('guest');
    p.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
    // Its own game decides when it chooses; the opponent's beats keep the room open meanwhile.
    for (let n = 0; n < 3; n++) { vi.advanceTimersByTime(BEAT_MS); p.game({ type: 'beat', round: 0 }); }
    expect(p.peer.state.phase).toBe('choosing');
    p.peer.chooseNext(2);
    vi.advanceTimersByTime(NEXT_ACK_MS - 1500);
    expect(p.peer.state.phase).toBe('choosing');
    vi.advanceTimersByTime(3000);
    expect(p.peer.state).toMatchObject({ phase: 'error', message: 'Your opponent stopped responding.', next: null });
    // A next-ack in time: the next battle, with the battle limit of silence again.
    const q = playing('guest');
    q.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
    vi.advanceTimersByTime(NEXT_MS);
    q.peer.chooseNext(0);
    vi.advanceTimersByTime(NEXT_ACK_MS - 1500);
    q.game({ type: 'next-ack', round: 0, battle: 1 });
    expect(q.peer.state.phase).toBe('playing');
    expect(q.nexts).toEqual([{ battle: 1, player: 1, ship: 0 }]);
    vi.advanceTimersByTime(10000);
    expect(q.peer.state.phase).toBe('playing');
  } finally { vi.useRealTimers(); }
});
it('protocol 3: the opponent leaves during the choice: the room closes with the reason', () => {
  const p = playing('guest');
  p.peer.awaitNext({ battle: 1, player: 1, ships: [0, 2] });
  p.relay({ type: 'peer-left', message: 'The other player disconnected.' });
  expect(p.peer.state).toMatchObject({ phase: 'closed', message: 'Your opponent left the room.', next: null });
});
