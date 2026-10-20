// @vitest-environment node
import { afterEach, describe, expect, it } from 'vitest';
// These adapters run the real relay implementations, not mocked transports.
// @ts-ignore JavaScript relay contract is exercised at runtime.
import { createNetplayServer } from '../../../server/netplay.mjs';
// @ts-ignore JavaScript subprocess adapter has no TypeScript declarations.
import { createRustRelay } from '../../../tests/helpers/rust-relay.mjs';
// @ts-ignore ws is provided by the repository's root dependencies.
import { WebSocket } from 'ws';
import { CosmoNetplay, createRoomId, invitationURL, parseRoom, type Frame, type SocketLike } from './CosmoNetplay';
const cleanup: (() => Promise<void> | void)[] = [];
afterEach(async () => { for (const fn of cleanup.splice(0).reverse()) await fn(); });
const sleep = (ms = 2) => new Promise(resolve => setTimeout(resolve, ms));
async function until(predicate: () => boolean, detail: () => unknown = () => '', ms = 6000) {
  const end = Date.now() + ms;
  while (!predicate()) { if (Date.now() >= end) throw new Error(`Timed out: ${JSON.stringify(detail())}`); await sleep(); }
}
async function pair(kind = 'node', jitter = false, guestBuild = 'test-rust-v1') {
  const app = (kind === 'rust' ? createRustRelay : createNetplayServer)({distDir: false, latencyPatternMs: jitter ? [20, 0, 9, 2] : [0]});
  const address = await app.listen(0); cleanup.push(() => app.close());
  const origin = `http://127.0.0.1:${address.port}`; const room = createRoomId();
  const sockets: any[] = []; const starts: any[][] = [[], []]; const frames: Frame[][] = [[], []];
  let peers: CosmoNetplay[] = [];
  let endAt = -1; let corruptHash = false;
  peers = [0, 1].map(index => new CosmoNetplay({
    buildId: index ? guestBuild : 'test-rust-v1', randomSeed: () => 123456789,
    socketFactory: url => { const ws = new WebSocket(url, {origin}); sockets[index] = ws; return ws as SocketLike; },
    onStart: value => { starts[index].push(value); frames[index] = []; },
    onFrame: frame => {
      frames[index].push(frame);
      if (frame.tick % 30 === 0 || frame.tick === endAt) peers[index].reportChecksum(frame.tick, (corruptHash && index ? 'bad' : 'ok') + frame.tick);
      if (frame.tick === endAt) peers[index].finish(0);
    },
  }));
  cleanup.push(() => { peers.forEach(peer => peer.close()); });
  peers[0].connect({origin, room, role: 'host'});
  await until(() => peers[0].state.phase === 'waiting');
  peers[1].connect({origin, room, role: 'guest'});
  const states = () => peers.map(peer => peer.state);
  await until(() => peers.every(peer => peer.state.phase === 'lobby' && peer.state.peerFleet.length > 0) || peers.some(peer => peer.state.phase === 'error'), states);
  return {peers, sockets, states, starts, frames, origin, app, room,
    endAt(tick: number) { endAt = tick; }, corruptHash() { corruptHash = true; },
    async start() {
      peers[0].chooseFleet([2, 0]); peers[1].chooseFleet([1]);
      // Fleet and ready packets share one ordered WebSocket stream.
      peers.forEach(peer => peer.ready());
      await until(() => peers.every(peer => peer.state.phase === 'playing') || peers.some(peer => peer.state.phase === 'error'), states);
      expect(peers.map(peer => peer.state.phase)).toEqual(['playing', 'playing']);
    },
    async pump(tick: number) {
      const deadline = Date.now() + 10000;
      while (peers.some(peer => peer.state.tick < tick && peer.state.phase === 'playing')) {
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
  p.peers.forEach(peer => peer.ready());
  await until(() => p.peers[0].state.phase === 'error', p.states);
  await until(() => p.peers[1].state.phase === 'closed', p.states);
  expect(p.starts).toEqual([[], []]);
});
