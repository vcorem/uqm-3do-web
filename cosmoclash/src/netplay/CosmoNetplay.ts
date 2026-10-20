/** Two-peer deterministic input lockstep. The relay transports opaque bytes only.
 * Simulation is driven by advance(), never by message timing or wall-clock deltas. */
export const PROTOCOL_VERSION = 1;
export const INPUT_DELAY = 3;
const MAX_PACKET = 2048;
const MAX_BUFFERED = 65536;
const MAX_TICK = 86400000;
export type Role = 'host' | 'guest';
export type Phase = 'idle' | 'connecting' | 'waiting' | 'handshake' | 'lobby' | 'starting' | 'playing' | 'finishing' | 'ended' | 'closed' | 'error';
export interface MatchStart { round: number; seed: number; fleets: [number[], number[]]; localPlayer: 0 | 1 }
export interface Frame { tick: number; inputs: [number, number] }
export interface NetplayState {
  phase: Phase; message: string; role: Role; room: string; round: number; tick: number;
  localFleet: number[]; peerFleet: number[]; localReady: boolean; peerReady: boolean;
  localRematch: boolean; peerRematch: boolean; verifiedChecksums: number;
}
export interface SocketLike {
  binaryType: string; readyState: number; bufferedAmount: number;
  onopen: ((event: any) => void) | null; onmessage: ((event: any) => void) | null;
  onclose: ((event: any) => void) | null; onerror: ((event: any) => void) | null;
  send(data: Uint8Array): void; close(code?: number, reason?: string): void;
}
export interface NetplayOptions {
  buildId: string;
  socketFactory?: (url: string) => SocketLike;
  randomSeed?: () => number;
  timeoutMs?: number;
  onState?: (state: NetplayState) => void;
  onStart: (start: MatchStart) => void;
  onFrame: (frame: Frame) => void;
  onEnd?: (winner: 0 | 1 | 2) => void;
}
type Packet = Record<string, unknown>;
const integer = (value: unknown, max: number): value is number => Number.isSafeInteger(value) && (value as number) >= 0 && (value as number) <= max;
const validFleet = (value: unknown): value is number[] => Array.isArray(value) && value.length >= 1 && value.length <= 3 && value.every(n => integer(n, 2)) && new Set(value).size === value.length;
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
const shape = (p: Packet, keys: string[]) => Object.keys(p).length === keys.length && keys.every(k => Object.prototype.hasOwnProperty.call(p, k));

export function createRoomId(): string {
  return Array.from(crypto.getRandomValues(new Uint8Array(16)), byte => byte.toString(16).padStart(2, '0')).join('');
}
export function parseRoom(value: string): string | null {
  const text = value.trim();
  if (/^[0-9a-f]{32}$/.test(text)) return text;
  try { const url = new URL(text); const room = url.searchParams.get('cosmoRoom'); return /^[0-9a-f]{32}$/.test(room ?? '') ? room : null; }
  catch { return null; }
}
export function invitationURL(room: string, base = location.href): string {
  if (!/^[0-9a-f]{32}$/.test(room)) throw new Error('Invalid room code.');
  const url = new URL(base); url.searchParams.set('cosmoRoom', room); url.hash = ''; return url.href;
}
export class CosmoNetplay {
  private socket: SocketLike | null = null;
  private phase: Phase = 'idle';
  private message = 'Create a room or join a friend.';
  private role: Role = 'host';
  private room = '';
  private round = 0;
  private tick = 0;
  private sendTick = INPUT_DELAY;
  private receiveTick = INPUT_DELAY;
  private localFleet = [0, 1, 2];
  private peerFleet: number[] = [];
  private localReady = false;
  private peerReady = false;
  private localRematch = false;
  private peerRematch = false;
  private hello = false;
  private pendingStart: MatchStart | null = null;
  private localInputs = new Map<number, number>();
  private peerInputs = new Map<number, number>();
  private localChecksums = new Map<number, string>();
  private peerChecksums = new Map<number, string>();
  private verifiedChecksums = 0;
  private lastLocalChecksum = -1;
  private lastPeerChecksum = -1;
  private localFinish: {tick: number; winner: 0 | 1 | 2} | null = null;
  private peerFinish: {tick: number; winner: 0 | 1 | 2} | null = null;
  private timer: ReturnType<typeof setInterval> | null = null;
  private lastPeerAt = Date.now();
  private rateStart = 0;
  private rateCount = 0;
  constructor(private readonly options: NetplayOptions) {
    if (!/^[a-zA-Z0-9._:-]{1,128}$/.test(options.buildId)) throw new Error('A stable build identity is required.');
  }
  get state(): NetplayState {
    return {phase: this.phase, message: this.message, role: this.role, room: this.room, round: this.round, tick: this.tick,
      localFleet: [...this.localFleet], peerFleet: [...this.peerFleet], localReady: this.localReady, peerReady: this.peerReady,
      localRematch: this.localRematch, peerRematch: this.peerRematch, verifiedChecksums: this.verifiedChecksums};
  }
  private emit(phase = this.phase, message = this.message): void {
    this.phase = phase; this.message = message; this.options.onState?.(this.state);
  }
  connect({origin, room, role}: {origin: string; room: string; role: Role}): void {
    if (this.phase !== 'idle') throw new Error('Create a new session for a new room.');
    const url = new URL(origin);
    if (!['http:', 'https:', 'ws:', 'wss:'].includes(url.protocol) || url.username || url.password || !/^[0-9a-f]{32}$/.test(room) || !['host', 'guest'].includes(role)) throw new Error('Invalid relay address or room.');
    url.protocol = ['https:', 'wss:'].includes(url.protocol) ? 'wss:' : 'ws:';
    url.pathname = `/netplay/cosmoclash/${room}`; url.search = `?role=${role}`; url.hash = '';
    this.role = role; this.room = room; this.lastPeerAt = Date.now();
    this.emit('connecting', 'Connecting to the relay…');
    try { this.socket = (this.options.socketFactory ?? (address => new WebSocket(address)))(url.href); }
    catch { this.fail('Could not connect to the relay.'); return; }
    this.socket.binaryType = 'arraybuffer';
    this.socket.onmessage = event => this.receive(event.data);
    this.socket.onerror = () => this.fail('The relay connection failed. Check the server address and try a new room.');
    this.socket.onclose = () => { if (!['closed', 'error'].includes(this.phase)) this.stop('closed', 'Connection closed. Create a new room to play again.'); };
    this.timer = setInterval(() => {
      const waiting = ['connecting', 'waiting', 'lobby', 'ended'].includes(this.phase);
      const timeout = this.options.timeoutMs ?? (waiting ? 300000 : 20000);
      if (Date.now() - this.lastPeerAt > timeout) this.fail('The other player stopped responding. Create a new room.');
    }, 1000);
  }
  chooseFleet(fleet: number[]): void {
    if (!validFleet(fleet)) throw new Error('Choose one to three different ships.');
    if (!['idle', 'connecting', 'waiting', 'handshake', 'lobby'].includes(this.phase) || this.localReady) return;
    this.localFleet = [...fleet];
    if (this.phase === 'lobby') this.send({type: 'fleet', round: this.round, ships: this.localFleet});
    this.emit();
  }
  ready(): void {
    if (this.phase !== 'lobby' || this.localReady || !this.peerFleet.length) return;
    this.localReady = true; this.send({type: 'ready', round: this.round}); this.emit(); this.tryStart();
  }
  /** Call once per fixed 1/60-second simulation opportunity. Never synthesize remote input. */
  advance(input: number): boolean {
    if (!integer(input, 31)) throw new Error('Invalid input mask.');
    if (this.phase !== 'playing') return false;
    if (this.peerFinish && this.tick > this.peerFinish.tick) { this.fail('The players reported different match results.'); return false; }
    if (this.sendTick <= this.tick + INPUT_DELAY) {
      this.localInputs.set(this.sendTick, input);
      this.send({type: 'input', round: this.round, tick: this.sendTick++, input});
    }
    if (this.phase !== 'playing' || !this.localInputs.has(this.tick) || !this.peerInputs.has(this.tick)) return false;
    const local = this.localInputs.get(this.tick)!; const remote = this.peerInputs.get(this.tick)!;
    this.localInputs.delete(this.tick); this.peerInputs.delete(this.tick);
    const frame = {tick: this.tick++, inputs: this.role === 'host' ? [local, remote] : [remote, local]} as Frame;
    try { this.options.onFrame(frame); } catch { this.fail('The game could not advance safely.'); }
    return true;
  }
  reportChecksum(tick: number, hash: string | number): void {
    if (!['playing', 'finishing'].includes(this.phase)) return;
    const value = String(hash);
    if (!integer(tick, MAX_TICK) || tick !== this.tick - 1 || tick <= this.lastLocalChecksum || !/^[a-zA-Z0-9._:-]{1,128}$/.test(value)) throw new Error('Invalid state checksum.');
    if (this.localChecksums.size >= 120) { this.fail('The other player is not verifying game state.'); return; }
    this.lastLocalChecksum = tick; this.localChecksums.set(tick, value);
    this.send({type: 'hash', round: this.round, tick, hash: value}); this.compareHash(tick);
  }
  finish(winner: 0 | 1 | 2): void {
    if (this.phase !== 'playing' || !integer(winner, 2)) return;
    if (this.tick === 0 || this.lastLocalChecksum !== this.tick - 1) throw new Error('Report the final state checksum before finishing.');
    this.localFinish = {tick: this.tick - 1, winner};
    this.send({type: 'finish', round: this.round, ...this.localFinish});
    if (this.phase !== 'playing') return;
    this.emit('finishing', 'Verifying the match result…'); this.tryFinish();
  }
  rematch(): void {
    if (this.phase !== 'ended' || this.localRematch) return;
    this.localRematch = true; this.send({type: 'rematch', round: this.round}); this.emit(); this.tryRematch();
  }
  close(): void { this.stop('closed', 'You left the room.'); }
  private stop(phase: 'closed' | 'error', message: string): void {
    if (['closed', 'error'].includes(this.phase)) return;
    if (this.timer) clearInterval(this.timer); this.timer = null;
    const socket = this.socket; this.socket = null;
    if (socket) { socket.onmessage = null; socket.onclose = null; socket.onerror = null; socket.close(1000, 'Session ended'); }
    this.localInputs.clear(); this.peerInputs.clear(); this.localChecksums.clear(); this.peerChecksums.clear();
    this.emit(phase, message);
  }
  private fail(message: string): void { this.stop('error', message); }
  private send(packet: Packet): void {
    if (!this.socket || this.socket.readyState !== 1) { this.fail('The connection is no longer available.'); return; }
    const bytes = new TextEncoder().encode(JSON.stringify(packet));
    if (bytes.length > MAX_PACKET || this.socket.bufferedAmount + bytes.length > MAX_BUFFERED) { this.fail('The connection is too slow to continue safely.'); return; }
    try { this.socket.send(bytes); } catch { this.fail('Game data could not be sent.'); }
  }
  private receive(data: unknown): void {
    if (['closed', 'error'].includes(this.phase)) return;
    try {
      if (typeof data === 'string') { this.receiveRelay(data); this.lastPeerAt = Date.now(); return; }
      const bytes = data instanceof ArrayBuffer ? new Uint8Array(data) : ArrayBuffer.isView(data) ? new Uint8Array(data.buffer, data.byteOffset, data.byteLength) : null;
      if (!bytes || !bytes.length || bytes.length > MAX_PACKET) throw new Error();
      const now = Date.now();
      if (now - this.rateStart >= 1000) { this.rateStart = now; this.rateCount = 0; }
      if (++this.rateCount > 250) throw new Error();
      const packet: unknown = JSON.parse(new TextDecoder('utf-8', {fatal: true}).decode(bytes));
      if (!packet || typeof packet !== 'object' || Array.isArray(packet)) throw new Error();
      this.receivePacket(packet as Packet); this.lastPeerAt = now;
    } catch { this.fail('Invalid or incompatible game data received.'); }
  }
  private receiveRelay(text: string): void {
    if (text.length > MAX_PACKET) throw new Error();
    const p = JSON.parse(text);
    if (p.type === 'waiting' && shape(p, ['type']) && this.phase === 'connecting') this.emit('waiting', this.role === 'host' ? 'Room created. Share the invitation with your friend.' : 'Waiting for the host…');
    else if (p.type === 'ready' && shape(p, ['type', 'role']) && p.role === this.role && this.phase === 'waiting') {
      this.emit('handshake', 'Checking game versions…');
      this.send({type: 'hello', version: PROTOCOL_VERSION, build: this.options.buildId, role: this.role, delay: INPUT_DELAY});
    } else if (['peer-left', 'error'].includes(p.type) && typeof p.message === 'string' && p.message.length <= 512) this.stop(p.type === 'error' ? 'error' : 'closed', p.message);
    else throw new Error();
  }
  private receivePacket(p: Packet): void {
    if (p.type === 'hello') {
      if (this.phase !== 'handshake' || this.hello || !shape(p, ['type', 'version', 'build', 'role', 'delay'])) throw new Error();
      if (p.version !== PROTOCOL_VERSION || p.build !== this.options.buildId || p.delay !== INPUT_DELAY || p.role !== (this.role === 'host' ? 'guest' : 'host')) { this.fail('The game versions differ. Both players should reload the same release.'); return; }
      this.hello = true; this.phase = 'lobby';
      this.send({type: 'fleet', round: this.round, ships: this.localFleet});
      if (this.phase === 'lobby') this.emit('lobby', 'Choose your fleet, then select Ready.'); return;
    }
    if (!this.hello || p.round !== this.round) throw new Error();
    switch (p.type) {
      case 'fleet':
        if (this.phase !== 'lobby' || this.peerReady || !shape(p, ['type', 'round', 'ships']) || !validFleet(p.ships)) throw new Error();
        this.peerFleet = [...p.ships]; this.emit(); break;
      case 'ready':
        if (this.phase !== 'lobby' || this.peerReady || !this.peerFleet.length || !shape(p, ['type', 'round'])) throw new Error();
        this.peerReady = true; this.emit(); this.tryStart(); break;
      case 'start': {
        const fleets = [this.peerFleet, this.localFleet];
        if (this.role !== 'guest' || this.phase !== 'lobby' || !this.localReady || !this.peerReady || !shape(p, ['type', 'round', 'seed', 'fleets']) || !integer(p.seed, 0xffffffff) || !same(p.fleets, fleets)) throw new Error();
        this.pendingStart = {round: this.round, seed: p.seed, fleets: [this.peerFleet.slice(), this.localFleet.slice()], localPlayer: 1};
        this.emit('starting', 'Starting the match…'); this.send({type: 'ack', round: this.round}); break;
      }
      case 'ack':
        if (this.role !== 'host' || this.phase !== 'starting' || !shape(p, ['type', 'round'])) throw new Error();
        this.send({type: 'go', round: this.round}); if (this.phase === 'starting') this.start(); break;
      case 'go':
        if (this.role !== 'guest' || this.phase !== 'starting' || !shape(p, ['type', 'round'])) throw new Error();
        this.start(); break;
      case 'input':
        if (!['playing', 'finishing', 'ended'].includes(this.phase) || !shape(p, ['type', 'round', 'tick', 'input']) || !integer(p.tick, MAX_TICK) || p.tick !== this.receiveTick || p.tick > this.tick + 2 * INPUT_DELAY + 1 || !integer(p.input, 31)) throw new Error();
        this.receiveTick++; this.peerInputs.set(p.tick, p.input); break;
      case 'hash':
        if (!['playing', 'finishing', 'ended'].includes(this.phase) || !shape(p, ['type', 'round', 'tick', 'hash']) || !integer(p.tick, MAX_TICK) || p.tick <= this.lastPeerChecksum || p.tick > this.tick + INPUT_DELAY || typeof p.hash !== 'string' || !/^[a-zA-Z0-9._:-]{1,128}$/.test(p.hash) || this.peerChecksums.size >= 120) throw new Error();
        this.lastPeerChecksum = p.tick; this.peerChecksums.set(p.tick, p.hash); this.compareHash(p.tick); break;
      case 'finish':
        if (!['playing', 'finishing'].includes(this.phase) || this.peerFinish || !shape(p, ['type', 'round', 'tick', 'winner']) || !integer(p.tick, MAX_TICK) || p.tick < this.tick - 1 || p.tick > this.tick + INPUT_DELAY || p.tick !== this.lastPeerChecksum || !integer(p.winner, 2)) throw new Error();
        this.peerFinish = {tick: p.tick, winner: p.winner as 0 | 1 | 2}; this.tryFinish(); break;
      case 'rematch':
        if (this.phase !== 'ended' || this.peerRematch || !shape(p, ['type', 'round'])) throw new Error();
        this.peerRematch = true; this.emit(); this.tryRematch(); break;
      default: throw new Error();
    }
  }
  private tryStart(): void {
    if (this.phase !== 'lobby' || !this.localReady || !this.peerReady || this.role !== 'host') return;
    const seed = (this.options.randomSeed ?? (() => crypto.getRandomValues(new Uint32Array(1))[0]))();
    if (!integer(seed, 0xffffffff)) { this.fail('Could not create a match seed.'); return; }
    this.pendingStart = {round: this.round, seed, fleets: [this.localFleet.slice(), this.peerFleet.slice()], localPlayer: 0};
    this.emit('starting', 'Starting the match…'); this.send({type: 'start', round: this.round, seed, fleets: this.pendingStart.fleets});
  }
  private start(): void {
    if (!this.pendingStart) throw new Error();
    for (let tick = 0; tick < INPUT_DELAY; tick++) { this.localInputs.set(tick, 0); this.peerInputs.set(tick, 0); }
    this.emit('playing', 'Online match — arrows to fly, / to fire, . for special.');
    if (this.phase === 'playing') this.options.onStart(this.pendingStart); this.pendingStart = null;
  }
  private compareHash(tick: number): void {
    const local = this.localChecksums.get(tick), peer = this.peerChecksums.get(tick);
    if (local === undefined || peer === undefined) return;
    if (local !== peer) { this.fail(`The game states differ at tick ${tick}. The match was stopped.`); return; }
    this.localChecksums.delete(tick); this.peerChecksums.delete(tick); this.verifiedChecksums++; this.emit();
  }
  private tryFinish(): void {
    if (!this.localFinish || !this.peerFinish) return;
    if (!same(this.localFinish, this.peerFinish)) { this.fail('The players reported different match results.'); return; }
    this.emit('ended', this.localFinish.winner === 0 ? 'The match is a draw.' : `${this.localFinish.winner === 1 ? 'Host' : 'Guest'} wins the fleet match.`);
    this.options.onEnd?.(this.localFinish.winner);
  }
  private tryRematch(): void {
    if (!this.localRematch || !this.peerRematch) return;
    this.round++; this.tick = 0; this.sendTick = INPUT_DELAY; this.receiveTick = INPUT_DELAY;
    this.localReady = false; this.peerReady = false; this.localRematch = false; this.peerRematch = false;
    this.localFinish = null; this.peerFinish = null; this.pendingStart = null;
    this.localInputs.clear(); this.peerInputs.clear(); this.localChecksums.clear(); this.peerChecksums.clear();
    this.lastLocalChecksum = -1; this.lastPeerChecksum = -1; this.verifiedChecksums = 0;
    this.phase = 'lobby';
    this.send({type: 'fleet', round: this.round, ships: this.localFleet});
    if (this.phase === 'lobby') this.emit('lobby', 'Rematch agreed. Choose your fleet and select Ready.');
  }
}
