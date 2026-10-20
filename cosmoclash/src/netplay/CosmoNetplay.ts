/** Two-peer deterministic input lockstep. The relay transports opaque bytes only.
 * Simulation is driven by advance(), never by message timing or wall-clock deltas.
 *
 * Rooms (the relay contract, cosmoclash/docs/online.md):
 *   /netplay/cosmoclash/<token>?role=host&name=<name>&engine=rust  listed host (in the room list)
 *   /netplay/cosmoclash/<token>?role=host                          private host (invite link only)
 *   /netplay/cosmoclash/<token>?role=guest                         guest
 * Relay control messages are JSON text; game data is binary frames only. A
 * listed host sends the text "ping" every 30 s and the relay answers "pong".
 *
 * Protocol 2: while the players choose, a fleet packet has 0 to 3 ships (the
 * picks so far, so that the other player sees the progress); Ready needs 1 to 3.
 * A ready player can take Ready back ("Change fleet"): the host decides, because
 * only the host starts the match (see unready()). While the players choose and
 * on the result screen, each peer sends a small "beat" packet every 20 s, so a
 * silent opponent is found in 2 minutes and a slow choice is not a lost one.
 *
 * Protocol 3: the first ship of a fleet opens the match; after a loss with two
 * or more ships left the loser chooses the next ship. Both peers stop the
 * lockstep at the same tick (the game calls awaitNext() from that frame), only
 * the loser's "next" packet decides the ship, the other peer answers
 * "next-ack", and each peer goes on with the next battle when it has both:
 * the waiting peer when "next" comes, the loser when "next-ack" comes. The
 * inputs of both peers stay in one ordered stream, so the next battle starts
 * at the same tick with the same ship on both peers. The choice has beats too. */
import { roomName } from './roomName';
import { BASE } from '@/base';
export const PROTOCOL_VERSION = 3;
export const INPUT_DELAY = 3;
const MAX_PACKET = 2048;
const MAX_BUFFERED = 65536;
const MAX_TICK = 86400000;
const CONNECT_MS = 12000;
export const KEEPALIVE_MS = 30000;
/** Fleet choice and result: a beat packet this often, and the limit without any packet. */
export const BEAT_MS = 20000;
export const QUIET_PEER_MS = 120000;
/** The relay's text when the other socket closed: the opponent left. Other peer-left texts say why the room ended. */
const OTHER_LEFT = 'The other player disconnected.';
// Longer than the longest relay wait (10 min for a listed room): the relay closes rooms itself.
const ROOM_WAIT_MS = 11 * 60000;
const ROOM_TOKEN = /^[0-9a-f]{32}$/;
export type Role = 'host' | 'guest';
export type Phase = 'idle' | 'connecting' | 'waiting' | 'handshake' | 'lobby' | 'starting' | 'playing' | 'choosing' | 'finishing' | 'ended' | 'closed' | 'error';
export interface MatchStart { round: number; seed: number; fleets: [number[], number[]]; localPlayer: 0 | 1 }
export interface Frame { tick: number; inputs: [number, number] }
/** The next battle after a choice: battle number, the player who chose (0 host, 1 guest) and the ship. */
export interface NextBattle { battle: number; player: 0 | 1; ship: number }
/** A choice of the next ship: the ships that the loser can choose from, and the chosen one when known. */
export interface NextChoice { battle: number; player: 0 | 1; ships: number[]; chosen: number | null }
export interface NetplayState {
  phase: Phase; message: string; role: Role; room: string; round: number; tick: number;
  localFleet: number[]; peerFleet: number[]; localReady: boolean; peerReady: boolean;
  localRematch: boolean; peerRematch: boolean; verifiedChecksums: number;
  /** The listed name of this room (a listed host), or '' for a private room or a guest. */
  name: string;
  /** The other player's first fleet packet came (its picks are in peerFleet). */
  peerFleetKnown: boolean;
  /** A guest asked to take Ready back and waits for the host's answer. */
  changing: boolean;
  /** Protocol 3: the choice of the next ship while the lockstep waits ('choosing'). */
  next: NextChoice | null;
}
export interface SocketLike {
  binaryType: string; readyState: number; bufferedAmount: number;
  onopen: ((event: any) => void) | null; onmessage: ((event: any) => void) | null;
  onclose: ((event: any) => void) | null; onerror: ((event: any) => void) | null;
  send(data: Uint8Array | string): void; close(code?: number, reason?: string): void;
}
export interface NetplayOptions {
  buildId: string;
  socketFactory?: (url: string) => SocketLike;
  randomSeed?: () => number;
  timeoutMs?: number;
  keepaliveMs?: number;
  beatMs?: number;
  /** The fleet before the player picks (protocol 2: none). */
  initialFleet?: number[];
  onState?: (state: NetplayState) => void;
  onStart: (start: MatchStart) => void;
  onFrame: (frame: Frame) => void;
  onEnd?: (winner: 0 | 1 | 2) => void;
  /** The next ship is agreed: build the next battle before the next frame. */
  onNext?: (next: NextBattle) => void;
}
type Packet = Record<string, unknown>;
const integer = (value: unknown, max: number): value is number => Number.isSafeInteger(value) && (value as number) >= 0 && (value as number) <= max;
const validPicks = (value: unknown): value is number[] => Array.isArray(value) && value.length <= 3 && value.every(n => integer(n, 2)) && new Set(value).size === value.length;
const validFleet = (value: unknown): value is number[] => validPicks(value) && value.length >= 1;
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
/** <page base>?cosmoRoom=<token>, for example https://urquanmasters.org/cosmoclash/?cosmoRoom=… */
export function invitationURL(room: string, base = new URL(BASE, location.href).href): string {
  if (!ROOM_TOKEN.test(room)) throw new Error('Invalid room code.');
  const url = new URL(base); url.search = ''; url.searchParams.set('cosmoRoom', room); url.hash = ''; return url.href;
}
/** The room token of an invite link in the page address, or null. */
export function readInvite(href: string): string | null {
  try { const room = new URL(href).searchParams.get('cosmoRoom'); return room && ROOM_TOKEN.test(room) ? room : null; }
  catch { return null; }
}
/** ws(s)://<host>/netplay/cosmoclash/<token>?role=…[&name=…&engine=rust] */
export function roomURL(origin: string, room: string, role: Role, name?: string | null): string {
  const url = new URL(origin);
  if (!['http:', 'https:', 'ws:', 'wss:'].includes(url.protocol) || url.username || url.password || !ROOM_TOKEN.test(room) || !['host', 'guest'].includes(role)) throw new Error('Invalid relay address or room.');
  if (name != null && role !== 'host') throw new Error('Only the host names a room.');
  url.protocol = ['https:', 'wss:'].includes(url.protocol) ? 'wss:' : 'ws:';
  url.pathname = `/netplay/cosmoclash/${room}`;
  url.search = `?role=${role}` + (name != null ? `&name=${encodeURIComponent(roomName(name))}&engine=rust` : '');
  url.hash = '';
  return url.href;
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
  private localFleet: number[];
  private peerFleet: number[] = [];
  private peerFleetKnown = false;
  private changing = false;
  private lastSentAt = Date.now();
  private name = '';
  private keepTimer: ReturnType<typeof setTimeout> | null = null;
  private answered = true;
  private waitingSince = Date.now();
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
  /** Protocol 3: the choice of the next ship, a "next" packet that came before this peer reached the choice, the last battle chosen. */
  private choice: NextChoice | null = null;
  private earlyNext: { battle: number; ship: number } | null = null;
  private lastBattle = 0;
  constructor(private readonly options: NetplayOptions) {
    if (!/^[a-zA-Z0-9._:-]{1,128}$/.test(options.buildId)) throw new Error('A stable build identity is required.');
    const initial = options.initialFleet ?? [];
    if (!validPicks(initial)) throw new Error('Invalid initial fleet.');
    this.localFleet = [...initial];
  }
  get state(): NetplayState {
    return {phase: this.phase, message: this.message, role: this.role, room: this.room, round: this.round, tick: this.tick,
      localFleet: [...this.localFleet], peerFleet: [...this.peerFleet], localReady: this.localReady, peerReady: this.peerReady,
      localRematch: this.localRematch, peerRematch: this.peerRematch, verifiedChecksums: this.verifiedChecksums,
      name: this.name, peerFleetKnown: this.peerFleetKnown, changing: this.changing,
      next: this.choice ? {...this.choice, ships: [...this.choice.ships]} : null};
  }
  /** This peer's player: the host is player 0 (left), the guest player 1. */
  private get local(): 0 | 1 { return this.role === 'host' ? 0 : 1; }
  private emit(phase = this.phase, message = this.message): void {
    this.phase = phase; this.message = message; this.options.onState?.(this.state);
  }
  /** name: a host with a name opens a listed room (in the room list); without one, a private room. */
  connect({origin, room, role, name}: {origin: string; room: string; role: Role; name?: string | null}): void {
    if (this.phase !== 'idle') throw new Error('Create a new session for a new room.');
    const address = roomURL(origin, room, role, name);
    this.role = role; this.room = room; this.name = name != null ? roomName(name) : ''; this.lastPeerAt = Date.now();
    this.emit('connecting', 'Connecting to online play…');
    try { this.socket = (this.options.socketFactory ?? (url => new WebSocket(url) as unknown as SocketLike))(address); }
    catch { this.fail('Could not connect to online play on this server.'); return; }
    this.socket.binaryType = 'arraybuffer';
    this.socket.onopen = () => { if (this.name && this.phase === 'connecting') this.keepalive(); };
    this.socket.onmessage = event => this.receive(event.data);
    this.socket.onerror = () => this.fail('Could not connect to online play on this server.');
    this.socket.onclose = () => { if (!['closed', 'error'].includes(this.phase)) this.stop('closed', 'The connection closed.'); };
    const started = Date.now();
    this.timer = setInterval(() => {
      const now = Date.now();
      if (this.phase === 'connecting') { if (now - started > (this.options.timeoutMs ?? CONNECT_MS)) this.fail('Could not connect to online play on this server.'); return; }
      // The relay closes a room that waits too long, with its own message.
      if (this.phase === 'waiting') { if (now - this.waitingSince > (this.options.timeoutMs ?? ROOM_WAIT_MS)) this.fail('Nobody joined before the room expired.'); return; }
      // No game data while the players choose or read the result: a beat says that this peer is still here.
      const patient = ['lobby', 'choosing', 'ended'].includes(this.phase);
      if (patient && now - this.lastSentAt >= (this.options.beatMs ?? BEAT_MS)) this.send({type: 'beat', round: this.round});
      if (['closed', 'error'].includes(this.phase)) return;
      const timeout = this.options.timeoutMs ?? (patient ? QUIET_PEER_MS : 20000);
      if (now - this.lastPeerAt > timeout) this.fail('Your opponent stopped responding.');
    }, Math.min(1000, this.options.beatMs ?? BEAT_MS));
  }
  /** A listed host pings while its socket is open; a ping without an answer means a lost connection. */
  private keepalive(): void {
    this.keepTimer = setTimeout(() => {
      this.keepTimer = null;
      const socket = this.socket;
      if (!socket || ['closed', 'error'].includes(this.phase)) return;
      if (!this.answered) { this.fail('Lost the connection to online play.'); return; }
      this.answered = false;
      try { socket.send('ping'); } catch { this.fail('Lost the connection to online play.'); return; }
      this.keepalive();
    }, this.options.keepaliveMs ?? KEEPALIVE_MS);
  }
  /** The picks so far (0 to 3 different ships; the first opens the match, the loser chooses the next ones). */
  chooseFleet(fleet: number[]): void {
    if (!validPicks(fleet)) throw new Error('Choose up to three different ships.');
    if (!['idle', 'connecting', 'waiting', 'handshake', 'lobby'].includes(this.phase) || this.localReady) return;
    this.localFleet = [...fleet];
    if (this.phase === 'lobby') this.send({type: 'fleet', round: this.round, ships: this.localFleet});
    this.emit();
  }
  ready(): void {
    if (this.phase !== 'lobby' || this.localReady || !validFleet(this.localFleet)) return;
    this.localReady = true; this.send({type: 'ready', round: this.round}); this.emit(); this.tryStart();
  }
  /**
   * Take Ready back ("Change fleet"), before the match starts. Only the host
   * starts the match, so the host decides: a host is not ready at once; a guest
   * asks and is not ready when the host answers "unready-ok". When the host has
   * already sent "start", the start wins and the host ignores the request.
   */
  unready(): void {
    if (this.phase !== 'lobby' || !this.localReady || this.changing) return;
    if (this.role === 'host') { this.localReady = false; this.send({type: 'unready', round: this.round}); this.emit(); return; }
    this.changing = true; this.send({type: 'unready', round: this.round}); this.emit();
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
  /**
   * Protocol 3: the battle ended and this player has two or more ships left:
   * the lockstep waits for the choice of the next ship. Call it from the frame
   * at which both peers know it (onFrame), after its checksum. player: the
   * loser (0 host, 1 guest); ships: the ships that the loser can choose.
   */
  awaitNext({battle, player, ships}: {battle: number; player: 0 | 1; ships: number[]}): void {
    if (this.phase !== 'playing' || this.choice) throw new Error('The next ship can be chosen only during a match.');
    if (!integer(battle, MAX_TICK) || battle <= this.lastBattle || (player !== 0 && player !== 1) || !validPicks(ships) || ships.length < 2) throw new Error('Invalid choice of the next ship.');
    this.choice = {battle, player, ships: [...ships], chosen: null};
    this.emit('choosing', player === this.local ? 'Choose your next ship.' : 'Your opponent is choosing the next ship.');
    const early = this.earlyNext;
    this.earlyNext = null;
    // emit() may have stopped the session (the page left the room).
    if (early && (this.phase as Phase) === 'choosing') {
      try { this.takeNext(early); } catch { this.fail('Invalid or incompatible game data received.'); }
    }
  }
  /** Protocol 3: the loser's choice (or the first ship left when the time ran out). */
  chooseNext(ship: number): void {
    const choice = this.choice;
    if (this.phase !== 'choosing' || !choice || choice.player !== this.local || choice.chosen !== null) return;
    if (!choice.ships.includes(ship)) throw new Error('This ship cannot fly next.');
    choice.chosen = ship;
    this.send({type: 'next', round: this.round, battle: choice.battle, ship});
    if (this.phase === 'choosing') this.emit();
  }
  /** The opponent's choice: answer next-ack and go on. */
  private takeNext({battle, ship}: {battle: number; ship: number}): void {
    const choice = this.choice;
    if (!choice || choice.player === this.local || choice.chosen !== null || battle !== choice.battle || !choice.ships.includes(ship)) throw new Error();
    choice.chosen = ship;
    this.send({type: 'next-ack', round: this.round, battle});
    if (this.phase === 'choosing') this.resume();
  }
  private resume(): void {
    const choice = this.choice!;
    const next: NextBattle = {battle: choice.battle, player: choice.player, ship: choice.chosen!};
    this.choice = null; this.lastBattle = choice.battle;
    this.emit('playing', 'The next battle starts.');
    if (this.phase === 'playing') this.options.onNext?.(next);
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
    if (this.keepTimer) clearTimeout(this.keepTimer); this.keepTimer = null;
    const socket = this.socket; this.socket = null;
    if (socket) { socket.onopen = null; socket.onmessage = null; socket.onclose = null; socket.onerror = null; try { socket.close(1000, 'Session ended'); } catch { /* Already closed. */ } }
    this.localInputs.clear(); this.peerInputs.clear(); this.localChecksums.clear(); this.peerChecksums.clear();
    this.choice = null; this.earlyNext = null;
    this.emit(phase, message);
  }
  private fail(message: string): void { this.stop('error', message); }
  private send(packet: Packet): void {
    if (!this.socket || this.socket.readyState !== 1) { this.fail('The connection is no longer available.'); return; }
    const bytes = new TextEncoder().encode(JSON.stringify(packet));
    if (bytes.length > MAX_PACKET || this.socket.bufferedAmount + bytes.length > MAX_BUFFERED) { this.fail('The connection is too slow to continue safely.'); return; }
    try { this.socket.send(bytes); this.lastSentAt = Date.now(); } catch { this.fail('Game data could not be sent.'); }
  }
  private receive(data: unknown): void {
    if (['closed', 'error'].includes(this.phase)) return;
    this.answered = true;
    // The relay's answer to the keepalive; it says nothing about the other player.
    if (data === 'pong') return;
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
  /** Relay control messages: waiting, ready, error and peer-left (server/netplay.mjs, deploy/netplay.js). */
  private receiveRelay(text: string): void {
    if (text.length > MAX_PACKET) throw new Error();
    const p = JSON.parse(text);
    if (!p || typeof p !== 'object' || Array.isArray(p)) throw new Error();
    if (p.type === 'waiting' && this.phase === 'connecting') {
      this.waitingSince = Date.now();
      this.emit('waiting', this.role === 'guest' ? 'Joining the room…' : this.name ? `Room “${this.name}” is open. Waiting for an opponent…` : 'Waiting for your opponent to join…');
    } else if (p.type === 'waiting' && this.phase === 'waiting') {
      // A repeated notice changes nothing.
    } else if (p.type === 'ready' && p.role === this.role && this.phase === 'waiting') {
      this.emit('handshake', 'Opponent found. Checking game versions…');
      this.send({type: 'hello', version: PROTOCOL_VERSION, build: this.options.buildId, role: this.role, delay: INPUT_DELAY});
    } else if (p.type === 'ready' && this.phase === 'waiting') {
      this.fail('The room assigned an unexpected player role.');
    } else if (['peer-left', 'error'].includes(p.type) && typeof p.message === 'string' && p.message.length <= 512) {
      // peer-left also comes to both players when the relay ends the room (it stops, the room is too old or idle).
      if (p.type === 'error') this.stop('error', p.message);
      else this.stop('closed', !p.message.trim() || p.message === OTHER_LEFT ? 'Your opponent left the room.' : `The room closed. ${p.message}`);
    } else if (p.type === 'peer-left' || p.type === 'closed') {
      this.stop('closed', 'Your opponent left the room.');
    } else throw new Error();
  }
  private receivePacket(p: Packet): void {
    if (p.type === 'hello') {
      if (this.phase !== 'handshake' || this.hello || !shape(p, ['type', 'version', 'build', 'role', 'delay'])) throw new Error();
      if (p.version !== PROTOCOL_VERSION || p.build !== this.options.buildId || p.delay !== INPUT_DELAY || p.role !== (this.role === 'host' ? 'guest' : 'host')) { this.fail('The game versions differ. Both players should reload the same release.'); return; }
      this.hello = true; this.phase = 'lobby';
      this.send({type: 'fleet', round: this.round, ships: this.localFleet});
      if (this.phase === 'lobby') this.emit('lobby', 'Choose your fleet, then select Ready.'); return;
    }
    if (p.type === 'beat') {
      // The other peer may still send a beat of the last round just after a rematch.
      if (!this.hello || !shape(p, ['type', 'round']) || !integer(p.round, MAX_TICK) || (p.round !== this.round && p.round !== this.round - 1)) throw new Error();
      return;
    }
    if (!this.hello || p.round !== this.round) throw new Error();
    switch (p.type) {
      case 'fleet':
        if (this.phase !== 'lobby' || this.peerReady || !shape(p, ['type', 'round', 'ships']) || !validPicks(p.ships)) throw new Error();
        this.peerFleet = [...p.ships]; this.peerFleetKnown = true; this.emit(); break;
      case 'ready':
        if (this.phase !== 'lobby' || this.peerReady || !validFleet(this.peerFleet) || !shape(p, ['type', 'round'])) throw new Error();
        this.peerReady = true; this.emit(); this.tryStart(); break;
      case 'unready':
        if (!shape(p, ['type', 'round'])) throw new Error();
        // The host sent "start" before this request came: the start wins.
        if (this.role === 'host' && this.phase === 'starting') break;
        if (this.phase !== 'lobby' || !this.peerReady) throw new Error();
        this.peerReady = false;
        if (this.role === 'host') this.send({type: 'unready-ok', round: this.round});
        if (this.phase === 'lobby') this.emit();
        break;
      case 'unready-ok':
        if (this.role !== 'guest' || this.phase !== 'lobby' || !this.changing || !shape(p, ['type', 'round'])) throw new Error();
        this.changing = false; this.localReady = false; this.emit(); break;
      case 'start': {
        const fleets = [this.peerFleet, this.localFleet];
        if (this.role !== 'guest' || this.phase !== 'lobby' || !this.localReady || !this.peerReady || !shape(p, ['type', 'round', 'seed', 'fleets']) || !integer(p.seed, 0xffffffff) || !same(p.fleets, fleets)) throw new Error();
        // A "Change fleet" request crossed this start: the match starts with the fleet that was ready.
        this.changing = false;
        this.pendingStart = {round: this.round, seed: p.seed, fleets: [this.peerFleet.slice(), this.localFleet.slice()], localPlayer: 1};
        this.emit('starting', 'Starting the match…'); this.send({type: 'ack', round: this.round}); break;
      }
      case 'ack':
        if (this.role !== 'host' || this.phase !== 'starting' || !shape(p, ['type', 'round'])) throw new Error();
        this.send({type: 'go', round: this.round}); if (this.phase === 'starting') this.start(); break;
      case 'go':
        if (this.role !== 'guest' || this.phase !== 'starting' || !shape(p, ['type', 'round'])) throw new Error();
        this.start(); break;
      case 'next': {
        if (!shape(p, ['type', 'round', 'battle', 'ship']) || !integer(p.battle, MAX_TICK) || !integer(p.ship, 2) || p.battle <= this.lastBattle) throw new Error();
        const next = {battle: p.battle, ship: p.ship};
        // The loser chose before this peer reached the end of the battle: keep it for awaitNext().
        if (this.phase === 'playing' && !this.choice) { if (this.earlyNext) throw new Error(); this.earlyNext = next; break; }
        if (this.phase !== 'choosing') throw new Error();
        this.takeNext(next); break;
      }
      case 'next-ack': {
        const choice = this.choice;
        if (this.phase !== 'choosing' || !choice || choice.player !== this.local || choice.chosen === null || !shape(p, ['type', 'round', 'battle']) || p.battle !== choice.battle) throw new Error();
        this.resume(); break;
      }
      case 'input':
        if (!['playing', 'choosing', 'finishing', 'ended'].includes(this.phase) || !shape(p, ['type', 'round', 'tick', 'input']) || !integer(p.tick, MAX_TICK) || p.tick !== this.receiveTick || p.tick > this.tick + 2 * INPUT_DELAY + 1 || !integer(p.input, 31)) throw new Error();
        this.receiveTick++; this.peerInputs.set(p.tick, p.input); break;
      case 'hash':
        if (!['playing', 'choosing', 'finishing', 'ended'].includes(this.phase) || !shape(p, ['type', 'round', 'tick', 'hash']) || !integer(p.tick, MAX_TICK) || p.tick <= this.lastPeerChecksum || p.tick > this.tick + INPUT_DELAY || typeof p.hash !== 'string' || !/^[a-zA-Z0-9._:-]{1,128}$/.test(p.hash) || this.peerChecksums.size >= 120) throw new Error();
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
    this.emit('playing', 'Online match: arrows to fly, / to fire, . for the special ability.');
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
    this.emit('ended', this.localFinish.winner === 0 ? 'The match is a draw.' : this.localFinish.winner === (this.role === 'host' ? 1 : 2) ? 'You win the fleet match.' : 'Your opponent wins the fleet match.');
    this.options.onEnd?.(this.localFinish.winner);
  }
  private tryRematch(): void {
    if (!this.localRematch || !this.peerRematch) return;
    this.round++; this.tick = 0; this.sendTick = INPUT_DELAY; this.receiveTick = INPUT_DELAY;
    this.localReady = false; this.peerReady = false; this.localRematch = false; this.peerRematch = false; this.peerFleetKnown = false; this.changing = false;
    this.localFinish = null; this.peerFinish = null; this.pendingStart = null;
    this.choice = null; this.earlyNext = null; this.lastBattle = 0;
    this.localInputs.clear(); this.peerInputs.clear(); this.localChecksums.clear(); this.peerChecksums.clear();
    this.lastLocalChecksum = -1; this.lastPeerChecksum = -1; this.verifiedChecksums = 0;
    this.phase = 'lobby';
    this.send({type: 'fleet', round: this.round, ships: this.localFleet});
    if (this.phase === 'lobby') this.emit('lobby', 'Rematch agreed. Choose your fleet, then select Ready.');
  }
}
