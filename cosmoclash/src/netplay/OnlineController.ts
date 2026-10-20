/**
 * Online Melee screens of Cosmo Clash: the state machine (no DOM; unit tested).
 * OnlineView draws the state; main.ts runs the 3D scenes and the battle.
 * The behaviour follows the UQM online lobby (web/online/online-ui.js,
 * docs/online-ui.md): room list, named rooms, waiting, invite links,
 * one choice per key press, Enter never cancels, Esc goes back.
 *
 * Screens: hidden (main menu or local play), lobby, name, waiting (host),
 * joining (guest), invite (?cosmoRoom= link), fleet (3D ship choice),
 * countdown, battle, next (the loser chooses the next ship in the 3D ship
 * choice, the winner waits; NEXT_MS for the choice), result. A confirm dialog
 * can cover fleet, countdown, battle, next and result ("Leave the match?").
 */
import { checkRoomName, defaultRoomName } from './roomName';
import type { LobbyRoom, LobbyStatus, LobbyWatcher } from './CosmoLobby';
import type { MatchStart, NetplayState, NextBattle, Role } from './CosmoNetplay';
import type { FleetView } from '@/match/FleetMatch';

export const NAME_KEY = 'cosmoclash.online.roomName.v1';
export const COUNTDOWN_MS = 3000;
export const SHIP_COUNT = 3;
/** The relays keep a listed room 10 minutes and a private room 2 minutes. */
export const LISTED_ROOM_MINUTES = 10;
export const PRIVATE_ROOM_MINUTES = 2;
/** An info notice shows this long (as in the UQM lobby). */
export const NOTICE_MS = 10000;
const BANNER_MS = 2500;
/** A new fleet or result screen ignores Enter and Space this long (a key pressed for the last screen). */
export const INPUT_LOCK_MS = 600;
/** Open room waits this long for the room list to answer (named room or private room). */
export const LIST_WAIT_MS = 10000;
/** The loser chooses the next ship in this time; then the first ship left in fleet order comes. */
export const NEXT_MS = 20000;

export type Screen = 'hidden' | 'lobby' | 'name' | 'waiting' | 'joining' | 'invite' | 'fleet' | 'countdown' | 'battle' | 'next' | 'result';
export interface Notice { text: string; kind: 'info' | 'error' | 'link' }
export interface RoomInfo { token: string; role: Role; name: string; listed: boolean; opened: number; closing?: boolean }
export interface FleetState {
  /** The ship that the 3D carousel shows (0 MichiGichi, 1 Gondi, 2 Sholev). */
  carousel: number;
  /** This player's picks in battle order (1 to 3 for Ready). */
  picks: number[];
  preparing: boolean;
  banner: string;
  bannerUntil: number;
}
export interface MatchResult { winner: 0 | 1 | 2; localPlayer: 0 | 1; sides: [FleetView, FleetView] }
/** The choice of the next ship after a lost battle (the 'next' screen). */
export interface NextState {
  battle: number;
  /** The loser, who chooses (0 host, 1 guest). */
  player: 0 | 1;
  /** This player chooses (else the opponent chooses and this player waits). */
  mine: boolean;
  /** The ships that can fly next, in fleet order: the first one comes when the time runs out. */
  ships: number[];
  /** The ship that the 3D ship choice shows. */
  carousel: number;
  /** env.now() when the time to choose runs out. */
  deadline: number;
  chosen: number | null;
  /** env.now() at the choice (the time to choose stops there). */
  chosenAt?: number;
  /** The time ran out and the game chose. */
  auto: boolean;
}
export interface OnlineState {
  screen: Screen;
  focus: string | null;
  items: string[];
  confirm: boolean;
  confirmFocus: 'stay' | 'leave-confirm';
  lobby: { status: 'idle' | LobbyStatus; rooms: LobbyRoom[]; error: string; retries: number };
  notice: Notice | null;
  name: string;
  nameError: string;
  room: RoomInfo | null;
  net: NetplayState | null;
  fleet: FleetState;
  start: MatchStart | null;
  countdownEnd: number;
  /** The battle of the countdown: 0 is the first one, later battles come after a choice. */
  battle: number;
  /** Both fleets for the countdown and the next screen (destroyed ships first). */
  sides: [FleetView, FleetView] | null;
  next: NextState | null;
  result: MatchResult | null;
  invite: string | null;
  /** Lobby: the player moved the focus (new rooms do not take it then). */
  touched: boolean;
  /** Name: Open room waits for the room list (a listed room needs a server with room lists). */
  opening: boolean;
  /** Enter and Space do nothing before this time (env.now()). */
  lockUntil: number;
}

/** What the controller needs from a room session (CosmoNetplay). */
export interface SessionLike {
  readonly state: NetplayState;
  connect(options: { origin: string; room: string; role: Role; name?: string | null }): void;
  chooseFleet(fleet: number[]): void;
  ready(): void;
  /** Take Ready back (Change fleet). */
  unready(): void;
  /** Protocol 3: this player's next ship (the loser's choice). */
  chooseNext(ship: number): void;
  rematch(): void;
  close(): void;
}
export interface SessionHandlers {
  onState(state: NetplayState): void;
  onStart(start: MatchStart): void;
  onEnd(winner: 0 | 1 | 2): void;
  onNext(next: NextBattle): void;
}
export interface OnlineEnv {
  now(): number;
  setTimeout(fn: () => void, ms: number): unknown;
  clearTimeout(id: unknown): void;
  random?(): number;
  storage?: { get(key: string): string | null; set(key: string, value: string): void };
  origin: string;
  createToken(): string;
  watchLobby(handlers: { onRooms(rooms: LobbyRoom[]): void; onStatus(status: LobbyStatus, message: string): void }): LobbyWatcher;
  createSession(handlers: SessionHandlers): SessionLike;
  inviteURL(token: string): string;
  copy?(text: string): Promise<void>;
  /**
   * Called at each choice made with a user gesture (not Esc). The page asks for
   * full screen when the game is not in full screen and the player did not turn it off.
   */
  fullscreen?(): Promise<boolean> | boolean;
  /** Loads the battle assets before this player says Ready. */
  prepare?(): Promise<void>;
  sound?(name: 'found' | 'error'): void;
  clearInvite?(): void;
  game: {
    /** Back to the main menu (the online screens hide). */
    menu(): void;
    /** Both players are ready: build the first battle (it starts after the countdown). */
    start(start: MatchStart): void;
    /** The session ended: remove the battle and the ship choice. */
    stop(): void;
    /** Both fleets now (the countdown, the choice of the next ship, the result). */
    fleets(): [FleetView, FleetView];
    /** The next ship is agreed: build the next battle (it starts after the countdown). */
    next(next: NextBattle): void;
  };
  changed?(state: OnlineState): void;
}

const ESCAPE: Partial<Record<Screen, string>> = { lobby: 'back', name: 'cancel', waiting: 'cancel', joining: 'cancel', invite: 'dismiss' };
const IN_MATCH: Screen[] = ['fleet', 'countdown', 'battle', 'next', 'result'];
/** An open "Leave the match?" stays open while the match goes from one of these screens to another. */
const KEEP_CONFIRM: Screen[] = ['countdown', 'battle', 'next'];
/** Text of the room client that tells the old page what to do next. The screens show their own next steps. */
export function cleanMessage(message: unknown): string {
  return String(message ?? '').replace(/\s*(Reload|Please try|Create a new room)[^.]*\.\s*$/i, '').trim();
}
export function waitingTime(created: number, now: number): string {
  const seconds = Math.max(0, Math.floor((now - created) / 1000));
  if (!(created > 0) || seconds < 60) return 'new';
  const minutes = Math.floor(seconds / 60);
  return minutes >= 60 ? Math.floor(minutes / 60) + ' h' : minutes + ' min';
}
export function clock(ms: number): string {
  const seconds = Math.max(0, Math.floor(ms / 1000));
  return Math.floor(seconds / 60) + ':' + String(seconds % 60).padStart(2, '0');
}
const TOKEN = /^[0-9a-f]{32}$/;

export class OnlineController {
  readonly state: OnlineState;
  private session: SessionLike | null = null;
  private watcher: LobbyWatcher | null = null;
  private retryTimer: unknown = null;
  private countdownTimer: unknown = null;
  private clockTimer: unknown = null;
  private noticeTimer: unknown = null;
  private listTimer: unknown = null;
  private nextTimer: unknown = null;
  private lastRoom: string | null = null;

  constructor(private readonly env: OnlineEnv) {
    this.state = {
      screen: 'hidden', focus: null, items: [], confirm: false, confirmFocus: 'stay',
      lobby: { status: 'idle', rooms: [], error: '', retries: 0 },
      notice: null, name: '', nameError: '', room: null, net: null,
      fleet: { carousel: 0, picks: [], preparing: false, banner: '', bannerUntil: 0 },
      start: null, countdownEnd: 0, battle: 0, sides: null, next: null, result: null, invite: null, touched: false, opening: false, lockUntil: 0,
    };
  }

  /** The room session, for the battle loop (advance, checksums, finish). */
  get current(): SessionLike | null { return this.session; }
  get visible(): boolean { return this.state.screen !== 'hidden'; }
  /** The battle runs (the lockstep advances) only on the battle screen. */
  get running(): boolean { return this.state.screen === 'battle'; }

  // ------------------------------------------------------------ items
  private itemsFor(): string[] {
    const s = this.state;
    switch (s.screen) {
      case 'lobby':
        if (s.lobby.status === 'unsupported') return ['create', 'back'];
        return [...s.lobby.rooms.map(room => 'room:' + room.id), ...(s.lobby.status === 'error' ? ['retry'] : []), 'create', 'back'];
      case 'name': return ['name', 'open', 'cancel'];
      case 'waiting': return ['copy', 'cancel'];
      case 'joining': return ['cancel'];
      case 'invite': return ['join', 'dismiss'];
      case 'fleet': return this.localReady() ? ['ship', 'change', 'leave'] : ['ship', 'ready', 'undo', 'leave'];
      case 'countdown': case 'battle': return ['leave'];
      // The loser: the ship card (Enter chooses), Leave. The winner waits: Leave only (Esc or a click).
      case 'next': return s.next?.mine ? ['ship', 'leave'] : ['leave'];
      case 'result': return ['rematch', 'lobby'];
      default: return [];
    }
  }
  private changed(): void {
    const s = this.state;
    s.items = this.itemsFor();
    if (!s.focus || !s.items.includes(s.focus)) s.focus = s.items[0] ?? null;
    this.env.changed?.(s);
  }
  private show(screen: Screen, focus?: string): void {
    const s = this.state;
    const wasLobby = s.screen === 'lobby' || s.screen === 'name';
    // An open "Leave the match?" stays open when the countdown becomes the battle, and around the choice of the next ship.
    if (screen !== s.screen) { s.touched = false; if (!(KEEP_CONFIRM.includes(s.screen) && KEEP_CONFIRM.includes(screen))) s.confirm = false; }
    if (screen !== s.screen && (screen === 'fleet' || screen === 'result' || screen === 'next')) s.lockUntil = this.env.now() + INPUT_LOCK_MS;
    if (screen !== 'name') this.stopOpening();
    s.screen = screen;
    if (focus !== undefined) s.focus = focus;
    if (screen === 'lobby' || screen === 'name') this.watch(); else if (wasLobby) this.unwatch();
    this.tickClock();
    this.changed();
  }
  private notice(text: string | null, kind: Notice['kind'] = 'info'): void {
    const notice = text ? { text, kind } : null;
    this.state.notice = notice;
    if (text && kind === 'error') this.env.sound?.('error');
    if (this.noticeTimer) { this.env.clearTimeout(this.noticeTimer); this.noticeTimer = null; }
    // An info notice (why a match ended, 'You left the room.') gives way to the screen's
    // own text after a while, as in the UQM lobby; errors and a link to copy stay.
    if (notice && kind === 'info') {
      this.noticeTimer = this.env.setTimeout(() => {
        this.noticeTimer = null;
        if (this.state.notice === notice) { this.state.notice = null; this.changed(); }
      }, NOTICE_MS);
    }
  }
  /** Waiting clocks and room ages change once a second while those screens show. */
  private tickClock(): void {
    const needs = ['lobby', 'waiting', 'joining', 'fleet'].includes(this.state.screen);
    if (needs && !this.clockTimer) {
      this.clockTimer = this.env.setTimeout(() => { this.clockTimer = null; this.tickClock(); this.changed(); }, 1000);
    }
  }
  /** A choice with a user gesture: the page may ask for full screen again (env.fullscreen decides). */
  private gesture(): void {
    if (!this.env.fullscreen) return;
    try { Promise.resolve(this.env.fullscreen()).catch(() => {}); } catch { /* No full screen here. */ }
  }

  // ------------------------------------------------------------ room list
  private watch(): void {
    const lobby = this.state.lobby;
    if (this.watcher || this.retryTimer || lobby.status === 'unsupported') return;
    if (lobby.status !== 'open') lobby.status = 'connecting';
    let handle: LobbyWatcher | null = null;
    const mine = () => !handle || this.watcher === handle;
    try {
      handle = this.env.watchLobby({
        onRooms: rooms => {
          if (!mine()) return;
          lobby.status = 'open'; lobby.retries = 0; this.setRooms(rooms);
          // Open room waited for this answer: the room is in the list.
          if (this.state.opening) this.openNamed();
        },
        onStatus: (status, message) => {
          if (!mine()) return;
          if (status === 'open') { lobby.status = 'open'; lobby.error = ''; lobby.retries = 0; this.changed(); return; }
          if (status === 'connecting') { if (lobby.status !== 'error') { lobby.status = 'connecting'; this.changed(); } return; }
          // No room list on this server (the Rust relay, older servers): private rooms with invite links.
          if (status === 'unsupported') {
            lobby.status = 'unsupported'; lobby.rooms = []; this.watcher = null; this.changed();
            if (this.state.opening) this.openNamed();
            return;
          }
          if (status === 'error') {
            // The watcher reconnects by itself (1, 2, 4, 8, 15 s); Retry reconnects at once.
            // The rooms of the lost list may be gone: they are not shown or joined.
            lobby.status = 'error'; lobby.retries++; lobby.rooms = [];
            lobby.error = cleanMessage(message).replace(/\s*(Trying again|Reconnecting)…$/, '') || 'The room list is not available.';
            this.changed();
            return;
          }
          if (status === 'closed') { this.watcher = null; this.changed(); }
        },
      });
      this.watcher = handle;
    } catch (error) {
      lobby.status = 'error';
      lobby.error = cleanMessage((error as Error)?.message) || 'The room list is not available.';
    }
  }
  private unwatch(): void {
    if (this.retryTimer) { this.env.clearTimeout(this.retryTimer); this.retryTimer = null; }
    const handle = this.watcher;
    this.watcher = null;
    try { handle?.close(); } catch { /* Already closed. */ }
    if (this.state.lobby.status !== 'unsupported') this.state.lobby.status = 'idle';
    // The next visit shows the rooms of a new list only: an old room may be gone.
    this.state.lobby.rooms = [];
  }
  private setRooms(rooms: LobbyRoom[]): void {
    const s = this.state;
    const mine = s.room?.role === 'host' ? s.room.token : null;
    const before = s.focus, index = s.items.indexOf(before ?? '');
    s.lobby.rooms = rooms.filter(room => room && TOKEN.test(String(room.id)) && room.id !== mine).slice(0, 50);
    s.items = this.itemsFor();
    if (s.screen === 'lobby' && !s.touched && (s.focus === 'create' || s.focus === 'retry') && s.lobby.rooms.length) {
      // The first rooms came before the player moved: offer the newest.
      s.focus = 'room:' + s.lobby.rooms[0].id;
    } else if (s.screen === 'lobby' && before?.startsWith('room:') && !s.items.includes(before)) {
      // The focused room went away: stay at the same place in the list.
      const list = s.items.filter(item => item.startsWith('room:'));
      s.focus = list[Math.min(index, list.length - 1)] ?? 'create';
    }
    this.changed();
  }

  // ------------------------------------------------------------ rooms
  private localReady(): boolean { return !!this.state.net?.localReady; }
  private closeRoom(): void {
    const session = this.session;
    this.session = null;
    if (this.countdownTimer) { this.env.clearTimeout(this.countdownTimer); this.countdownTimer = null; }
    this.stopNextTimer();
    this.state.next = null;
    if (this.state.room) this.state.room.closing = true;
    this.state.room = null;
    this.state.net = null;
    this.state.start = null;
    if (session && !['closed', 'error'].includes(session.state.phase)) {
      try { session.close(); } catch { /* Already closed. */ }
    }
  }
  /** name: a listed room for a host; '' opens a private room. */
  private connect(role: Role, token: string, name: string): void {
    this.closeRoom();
    const room: RoomInfo = { token, role, name, listed: role === 'host' && !!name, opened: this.env.now() };
    const s = this.state;
    s.room = room;
    s.result = null;
    s.fleet = { carousel: 0, picks: [], preparing: false, banner: '', bannerUntil: 0 };
    let session: SessionLike;
    try {
      session = this.env.createSession({
        onState: net => this.roomState(session, room, net),
        onStart: start => this.started(session, start),
        onEnd: winner => this.ended(session, winner),
        onNext: next => this.nexted(session, next),
      });
      this.session = session;
      // Enter must not close a room just opened: Copy invite link has the focus first.
      this.show(role === 'host' ? 'waiting' : 'joining', role === 'host' ? 'copy' : 'cancel');
      session.connect({ origin: this.env.origin, room: token, role, name: room.listed ? name : null });
    } catch (error) {
      if (s.room !== room) return;
      this.closeRoom();
      const message = cleanMessage((error as Error)?.message) || 'Could not open the room.';
      if (room.listed && /Room names/.test(message)) { s.nameError = message; this.show('name', 'name'); return; }
      this.notice(message, 'error');
      this.show('lobby', 'create');
    }
  }
  private roomState(session: SessionLike, room: RoomInfo, net: NetplayState): void {
    const s = this.state;
    if (this.session !== session || s.room !== room || room.closing) return;
    s.net = net;
    if (net.phase === 'closed' || net.phase === 'error') {
      const inMatch = IN_MATCH.includes(s.screen);
      const message = cleanMessage(net.message);
      this.session = null;
      this.closeRoom();
      if (inMatch) this.env.game.stop();
      // The relay refused the listed name: back to the name dialog with the reason.
      if (room.listed && !inMatch && /Room names/.test(message)) { s.nameError = message; this.show('name', 'name'); return; }
      const fallback = room.role === 'host' ? 'The room closed.' : 'Could not join this room.';
      const text = !inMatch && room.role === 'guest' && net.phase === 'error' && message && !/opponent/i.test(message)
        ? 'Could not join this room. ' + message : message || fallback;
      this.notice(text, net.phase === 'error' ? 'error' : 'info');
      this.show('lobby', 'create');
      return;
    }
    if (net.phase === 'lobby') {
      if (s.screen !== 'fleet') this.enterFleet(net);
      else this.changed();
      return;
    }
    if (net.phase === 'choosing') {
      if (s.screen !== 'next') this.enterNext(net);
      else this.changed();
      return;
    }
    // 'ended' comes just before onEnd(winner), which shows the result.
    this.changed();
  }
  private enterFleet(net: NetplayState): void {
    const s = this.state;
    const rematch = net.round > 0;
    s.fleet.picks = [...net.localFleet];
    s.fleet.preparing = false;
    s.fleet.carousel = this.nextFree(s.fleet.picks, s.fleet.carousel, 0);
    s.fleet.banner = rematch ? 'Rematch!' : 'Opponent found!';
    s.fleet.bannerUntil = this.env.now() + BANNER_MS;
    s.result = null;
    s.start = null;
    if (!rematch) this.env.sound?.('found');
    this.notice(null);
    this.env.game.stop();
    // A rematch keeps the last fleet: Enter says Ready at once.
    this.show('fleet', rematch && s.fleet.picks.length ? 'ready' : 'ship');
  }
  private started(session: SessionLike, start: MatchStart): void {
    if (this.session !== session) return;
    const s = this.state;
    s.start = start;
    s.result = null;
    s.next = null;
    this.env.game.start(start);
    s.battle = 0;
    s.sides = this.env.game.fleets();
    s.countdownEnd = this.env.now() + COUNTDOWN_MS;
    this.show('countdown', 'leave');
    this.scheduleCountdown();
  }
  private scheduleCountdown(): void {
    if (this.countdownTimer) this.env.clearTimeout(this.countdownTimer);
    const left = this.state.countdownEnd - this.env.now();
    this.countdownTimer = this.env.setTimeout(() => {
      this.countdownTimer = null;
      if (this.state.screen !== 'countdown') return;
      if (this.env.now() < this.state.countdownEnd) { this.changed(); this.scheduleCountdown(); return; }
      this.show('battle', 'leave');
    }, Math.max(0, Math.min(left, 250)));
  }
  private ended(session: SessionLike, winner: 0 | 1 | 2): void {
    if (this.session !== session) return;
    const s = this.state;
    const start = s.start;
    const local = (start?.localPlayer ?? (s.room?.role === 'guest' ? 1 : 0)) as 0 | 1;
    this.stopNextTimer();
    s.next = null;
    s.result = { winner, localPlayer: local, sides: this.env.game.fleets() };
    this.show('result', 'rematch');
  }

  // ------------------------------------------------------------ the next ship (protocol 3)
  /** The lockstep waits for the loser's choice: the 3D ship choice for the loser, a waiting screen for the winner. */
  private enterNext(net: NetplayState): void {
    const s = this.state;
    const pending = net.next;
    if (!pending) { this.changed(); return; }
    const local = s.start?.localPlayer ?? (s.room?.role === 'guest' ? 1 : 0);
    const mine = pending.player === local;
    s.next = { battle: pending.battle, player: pending.player, mine, ships: [...pending.ships], carousel: pending.ships[0],
      deadline: this.env.now() + NEXT_MS, chosen: null, auto: false };
    s.sides = this.env.game.fleets();
    this.notice(null);
    this.show('next', mine ? 'ship' : 'leave');
    this.scheduleNext();
  }
  private stopNextTimer(): void {
    if (this.nextTimer) { this.env.clearTimeout(this.nextTimer); this.nextTimer = null; }
  }
  /** The countdown of the choice changes at each full second; at the end the loser's game chooses. */
  private scheduleNext(): void {
    this.stopNextTimer();
    const next = this.state.next;
    if (!next || this.state.screen !== 'next') return;
    const left = next.deadline - this.env.now();
    if (left <= 0) { this.timeUp(); return; }
    this.nextTimer = this.env.setTimeout(() => {
      this.nextTimer = null;
      if (this.state.screen !== 'next' || this.state.next !== next) return;
      if (this.env.now() >= next.deadline) { this.timeUp(); return; }
      this.changed();
      this.scheduleNext();
    }, left % 1000 || 1000);
  }
  /** The time ran out: the loser's game takes the first ship left in fleet order. The winner never chooses. */
  private timeUp(): void {
    const next = this.state.next;
    if (next?.mine && next.chosen === null) { this.chooseNext(next.ships[0], true); return; }
    this.changed();
  }
  private chooseNext(ship: number, auto = false): void {
    const s = this.state;
    const next = s.next;
    if (s.screen !== 'next' || !next?.mine || next.chosen !== null || !next.ships.includes(ship)) return;
    next.chosen = ship;
    next.chosenAt = this.env.now();
    next.carousel = ship;
    next.auto = auto;
    this.stopNextTimer();
    try { this.session?.chooseNext(ship); } catch { /* The session checks the ship. */ }
    this.changed();
  }
  /** Both peers have the next ship: build the next battle and count down. */
  private nexted(session: SessionLike, next: NextBattle): void {
    if (this.session !== session) return;
    const s = this.state;
    this.stopNextTimer();
    this.env.game.next(next);
    s.next = null;
    s.battle = next.battle;
    s.sides = this.env.game.fleets();
    s.countdownEnd = this.env.now() + COUNTDOWN_MS;
    this.show('countdown', 'leave');
    this.scheduleCountdown();
  }
  /** Seconds left to choose the next ship (20 to 0). */
  nextLeft(): number { return Math.ceil(this.nextMsLeft() / 1000); }
  /** Milliseconds left to choose the next ship; at the choice the time stops. */
  nextMsLeft(): number {
    const next = this.state.next;
    if (!next) return 0;
    if (next.chosen !== null) return Math.max(0, next.deadline - (next.chosenAt ?? next.deadline));
    return Math.max(0, next.deadline - this.env.now());
  }
  /** The first ship from index (in the direction) that is not picked; index when all are. */
  private nextFree(picks: number[], index: number, direction: 1 | -1 | 0): number {
    const step = direction || 1;
    for (let n = direction ? 1 : 0; n <= SHIP_COUNT; n++) {
      const i = ((index + step * n) % SHIP_COUNT + SHIP_COUNT) % SHIP_COUNT;
      if (!picks.includes(i)) return i;
    }
    return index;
  }

  // ------------------------------------------------------------ fleet
  private setPicks(picks: number[]): void {
    const s = this.state;
    s.fleet.picks = picks;
    try { this.session?.chooseFleet(picks); } catch { /* The session checks the fleet. */ }
  }
  /** Left/Right in the 3D ship choice. */
  browse(delta: 1 | -1): void {
    const s = this.state;
    if (s.screen === 'next') {
      const next = s.next;
      if (!next?.mine || next.chosen !== null || s.confirm) return;
      const index = Math.max(0, next.ships.indexOf(next.carousel));
      next.carousel = next.ships[(index + delta + next.ships.length) % next.ships.length];
      s.focus = 'ship';
      this.changed();
      return;
    }
    if (s.screen !== 'fleet' || s.confirm) return;
    s.fleet.carousel = ((s.fleet.carousel + delta) % SHIP_COUNT + SHIP_COUNT) % SHIP_COUNT;
    s.focus = 'ship';
    this.changed();
  }
  /** Enter on the ship: add it to the fleet, or take it out when it is in the fleet. */
  private toggleShip(): void {
    const s = this.state;
    if (this.localReady()) { this.notice('You are ready. Choose Change fleet to pick other ships.'); this.changed(); return; }
    const ship = s.fleet.carousel;
    const picks = s.fleet.picks;
    if (picks.includes(ship)) {
      this.setPicks(picks.filter(id => id !== ship));
      this.notice(null);
      this.changed();
      return;
    }
    this.setPicks([...picks, ship]);
    this.notice(null);
    if (s.fleet.picks.length >= SHIP_COUNT) { s.focus = 'ready'; this.changed(); return; }
    s.fleet.carousel = this.nextFree(s.fleet.picks, ship, 1);
    this.changed();
  }
  undo(): void {
    const s = this.state;
    if (s.screen !== 'fleet' || this.localReady() || !s.fleet.picks.length) return;
    const last = s.fleet.picks[s.fleet.picks.length - 1];
    this.setPicks(s.fleet.picks.slice(0, -1));
    s.fleet.carousel = last;
    s.focus = 'ship';
    this.notice(null);
    this.changed();
  }
  /** Change fleet: take Ready back while the opponent is not ready yet. */
  private unready(): void {
    const net = this.state.net;
    if (!this.session || !net?.localReady || net.peerReady || net.changing) return;
    this.notice(null);
    this.session.unready();
    this.changed();
  }
  private ready(): void {
    const s = this.state;
    const session = this.session;
    if (!session || this.localReady() || s.fleet.preparing) return;
    if (!s.fleet.picks.length) { this.notice('Pick at least one ship first.', 'error'); s.focus = 'ship'; this.changed(); return; }
    if (session.state.phase !== 'lobby') return;
    s.fleet.preparing = true;
    this.changed();
    Promise.resolve(this.env.prepare?.()).then(() => {
      if (this.session !== session) return;
      s.fleet.preparing = false;
      session.ready();
      s.focus = 'ship';
      this.changed();
    }, () => {
      if (this.session !== session) return;
      s.fleet.preparing = false;
      this.notice('The game files could not load. Try Ready again.', 'error');
      this.changed();
    });
  }

  // ------------------------------------------------------------ choices
  /** how.escape: chosen with Esc, which gives no user activation for full screen. */
  activate(id: string | null, how: { escape?: boolean } = {}): void {
    const s = this.state;
    if (!id) return;
    if (s.confirm) {
      if (id === 'stay') { s.confirm = false; this.changed(); return; }
      if (id === 'leave-confirm') { s.confirm = false; this.leave(); return; }
      return;
    }
    if (!s.items.includes(id)) return;
    if (!how.escape) this.gesture();
    if (id.startsWith('room:')) {
      const room = s.lobby.rooms.find(item => 'room:' + item.id === id);
      if (room) { this.notice(null); this.connect('guest', room.id, room.name); }
      return;
    }
    switch (id) {
      case 'create':
        this.notice(null);
        // No room list on this server: a private room at once (a name would show nowhere).
        if (s.lobby.status === 'unsupported') { this.connect('host', this.env.createToken(), ''); return; }
        s.name = this.env.storage?.get(NAME_KEY) || defaultRoomName(this.env.random);
        s.nameError = '';
        this.show('name', 'name');
        return;
      case 'retry':
        this.unwatch();
        this.state.lobby.error = '';
        this.watch();
        this.changed();
        return;
      case 'back':
        this.notice(null);
        this.unwatch();
        this.show('hidden');
        this.env.game.menu();
        return;
      case 'name': case 'open': {
        const checked = checkRoomName(s.name);
        if (!checked.ok) { s.nameError = checked.error ?? 'This name cannot be used.'; s.focus = 'name'; this.changed(); return; }
        if (s.opening) return;
        // The name as the list shows it; remembered only once it is accepted.
        s.name = checked.name;
        s.nameError = '';
        this.env.storage?.set(NAME_KEY, checked.name);
        this.openNamed();
        return;
      }
      case 'cancel': {
        if (s.screen === 'name') { s.nameError = ''; this.show('lobby', 'create'); return; }
        const wasHost = s.room?.role === 'host';
        this.closeRoom();
        this.notice(wasHost ? 'Your room is closed.' : null);
        this.show('lobby', wasHost ? 'create' : undefined);
        return;
      }
      case 'copy': {
        const room = s.room;
        if (!room) return;
        let url: string;
        try { url = this.env.inviteURL(room.token); } catch { return; }
        const copy = this.env.copy ? this.env.copy(url) : Promise.reject(new Error('No clipboard'));
        Promise.resolve(copy).then(
          () => { if (s.room === room) { this.notice('Invite link copied. Send it to your opponent.'); this.changed(); } },
          () => { if (s.room === room) { this.notice(url, 'link'); this.changed(); } });
        return;
      }
      case 'join': {
        const token = s.invite;
        s.invite = null;
        this.env.clearInvite?.();
        if (token) this.connect('guest', token, '');
        return;
      }
      case 'dismiss':
        s.invite = null;
        this.env.clearInvite?.();
        this.show('hidden');
        this.env.game.menu();
        return;
      case 'ship':
        if (s.screen === 'next') { if (s.next) this.chooseNext(s.next.carousel); return; }
        this.toggleShip();
        return;
      case 'ready': this.ready(); return;
      case 'change': this.unready(); return;
      case 'undo': this.undo(); return;
      case 'leave': s.confirm = true; s.confirmFocus = 'stay'; this.changed(); return;
      case 'rematch':
        if (s.net?.localRematch) return;
        this.session?.rematch();
        this.changed();
        return;
      case 'lobby': this.leave(); return;
    }
  }
  /** Leave the room from the ship choice, the battle or the result. */
  leave(): void {
    const s = this.state;
    const hadMatch = IN_MATCH.includes(s.screen);
    this.closeRoom();
    if (hadMatch) this.env.game.stop();
    this.notice('You left the room.');
    this.show('lobby', 'create');
  }
  focus(id: string): void {
    const s = this.state;
    if (s.confirm) { if ((id === 'stay' || id === 'leave-confirm') && s.confirmFocus !== id) { s.confirmFocus = id; this.changed(); } return; }
    if (s.items.includes(id) && s.focus !== id) { s.focus = id; s.touched = true; this.changed(); }
  }
  setName(value: string): void {
    this.state.name = Array.from(String(value)).slice(0, 64).join('');
    this.state.nameError = '';
    // A new name: Open room again (the wait for the room list stops).
    this.stopOpening();
    this.changed();
  }
  /**
   * Open room with the accepted name in state.name. A named room is in the room
   * list, so it needs a server with room lists: it opens when the list answered
   * ('open'); without room lists ('unsupported') the room is private. While the
   * list connects (or reconnects after an error) Open room waits LIST_WAIT_MS.
   */
  private openNamed(): void {
    const s = this.state;
    if (s.screen !== 'name') { this.stopOpening(); return; }
    const status = s.lobby.status;
    if (status === 'open' || status === 'unsupported') {
      this.stopOpening();
      const listed = status === 'open';
      this.connect('host', this.env.createToken(), listed ? s.name : '');
      // s.room stays only when the private room opened (a refused room goes back to the lobby).
      if (!listed && s.room) {
        this.notice('This server has no room list: your room is private.');
        this.changed();
      }
      return;
    }
    s.opening = true;
    if (!this.listTimer) {
      this.listTimer = this.env.setTimeout(() => {
        this.listTimer = null;
        if (!s.opening) return;
        s.opening = false;
        if (s.screen === 'name') { s.nameError = 'The room list does not answer. Try again in a moment.'; s.focus = 'open'; }
        this.changed();
      }, LIST_WAIT_MS);
    }
    this.changed();
  }
  private stopOpening(): void {
    this.state.opening = false;
    if (this.listTimer) { this.env.clearTimeout(this.listTimer); this.listTimer = null; }
  }
  private move(delta: number): void {
    const s = this.state;
    const items = s.items;
    if (!items.length) return;
    s.touched = true;
    const index = Math.max(0, items.indexOf(s.focus ?? ''));
    s.focus = items[(index + delta + items.length * 8) % items.length];
    this.changed();
  }
  /** Lobby: Left/Right jump between the room list and the buttons. */
  private side(direction: 1 | -1): void {
    const s = this.state;
    const rooms = s.items.filter(item => item.startsWith('room:'));
    const buttons = s.items.filter(item => !item.startsWith('room:'));
    s.touched = true;
    if (s.focus?.startsWith('room:')) {
      if (direction > 0) { this.lastRoom = s.focus; s.focus = buttons.includes('create') ? 'create' : buttons[0]; }
      else return;
    } else if (direction < 0 && rooms.length && s.focus === buttons[0]) {
      s.focus = rooms.includes(this.lastRoom ?? '') ? this.lastRoom : rooms[0];
    } else {
      const index = buttons.indexOf(s.focus ?? '');
      const next = buttons[index + direction];
      if (!next) return;
      s.focus = next;
    }
    this.changed();
  }
  /**
   * One key press (repeats are filtered by the caller). Returns true when the
   * key was a menu key here, so that the page does not use it.
   */
  key(name: string): boolean {
    const s = this.state;
    if (s.screen === 'hidden') return false;
    // A key pressed for the last screen (a double press, a held fire key) does not choose on a new one.
    if ((name === 'Enter' || name === ' ') && this.env.now() < s.lockUntil && !s.confirm) return true;
    if (s.confirm) {
      if (name === 'ArrowLeft' || name === 'ArrowRight' || name === 'ArrowUp' || name === 'ArrowDown' || name === 'Tab') {
        s.confirmFocus = s.confirmFocus === 'stay' ? 'leave-confirm' : 'stay'; this.changed(); return true;
      }
      if (name === 'Enter' || name === ' ') { this.activate(s.confirmFocus); return true; }
      if (name === 'Escape') { this.activate('stay'); return true; }
      return s.screen !== 'battle' && s.screen !== 'countdown';
    }
    // In the battle only Esc is a menu key: arrows, / and . fly the ship.
    if (s.screen === 'battle' || s.screen === 'countdown') {
      if (name === 'Escape') { this.activate('leave', { escape: true }); return true; }
      return false;
    }
    if (s.screen === 'fleet') return this.fleetKey(name);
    if (s.screen === 'next') return this.nextKey(name);
    switch (name) {
      case 'ArrowUp': this.move(-1); return true;
      case 'ArrowDown': this.move(1); return true;
      case 'ArrowLeft': if (s.screen === 'lobby') this.side(-1); else this.move(-1); return true;
      case 'ArrowRight': if (s.screen === 'lobby') this.side(1); else this.move(1); return true;
      case 'Home': if (s.items.length) { s.focus = s.items[0]; s.touched = true; this.changed(); } return true;
      case 'End': if (s.items.length) { s.focus = s.items[s.items.length - 1]; s.touched = true; this.changed(); } return true;
      case 'PageUp': this.move(-5); return true;
      case 'PageDown': this.move(5); return true;
      case 'Enter': case ' ':
        // Joining has only Cancel: a reflex Enter must not undo the join (Esc or a click cancels).
        if (s.screen === 'joining') return true;
        this.activate(s.focus);
        return true;
      case 'Escape': {
        const action = ESCAPE[s.screen];
        if (s.screen === 'result') { this.activate('lobby', { escape: true }); return true; }
        if (action) { this.activate(action, { escape: true }); return true; }
        return false;
      }
      default: return false;
    }
  }
  private fleetKey(name: string): boolean {
    const s = this.state;
    const row = s.items.filter(item => item !== 'ship');
    const onShip = s.focus === 'ship';
    switch (name) {
      case 'ArrowLeft': case 'ArrowRight': {
        const delta = name === 'ArrowLeft' ? -1 : 1;
        if (onShip) { this.browse(delta); return true; }
        const index = row.indexOf(s.focus ?? '');
        const next = row[index + delta];
        if (next) { s.focus = next; this.changed(); }
        return true;
      }
      case 'ArrowDown':
        if (onShip) { s.focus = row[0]; this.changed(); }
        return true;
      case 'ArrowUp':
        if (!onShip) { s.focus = 'ship'; this.changed(); }
        return true;
      case 'Enter': case ' ': this.activate(s.focus); return true;
      case 'Backspace': case 'Delete': this.undo(); return true;
      case 'Escape': this.activate('leave', { escape: true }); return true;
      default: return false;
    }
  }
  /** The next ship: the loser browses and chooses; the winner waits (only Esc, as in the battle). */
  private nextKey(name: string): boolean {
    const s = this.state;
    if (name === 'Escape') { this.activate('leave', { escape: true }); return true; }
    if (!s.next?.mine) return false;
    switch (name) {
      case 'ArrowLeft': case 'ArrowRight':
        if (s.focus === 'ship') this.browse(name === 'ArrowLeft' ? -1 : 1);
        return true;
      case 'ArrowDown': if (s.focus !== 'leave') { s.focus = 'leave'; this.changed(); } return true;
      case 'ArrowUp': if (s.focus !== 'ship') { s.focus = 'ship'; this.changed(); } return true;
      case 'Enter': case ' ': this.activate(s.focus); return true;
      default: return false;
    }
  }

  // ------------------------------------------------------------ page events
  openLobby(): void {
    this.notice(null);
    // The rooms come with the first list; the newest room takes the focus then (setRooms).
    this.show('lobby', 'create');
  }
  /** The page was opened with an invite link (?cosmoRoom=<token>). */
  invite(token: string): void {
    if (!TOKEN.test(token)) return;
    this.state.invite = token;
    this.show('invite', 'join');
  }
  close(): void { this.stopOpening(); this.closeRoom(); this.unwatch(); }
  /** Hides the online screens without a callback (the page shows another screen). */
  reset(): void {
    this.close();
    this.state.confirm = false;
    this.notice(null);
    if (this.state.screen !== 'hidden') this.show('hidden');
  }
  /** The invite link of the open room. */
  inviteLink(): string {
    const room = this.state.room;
    if (!room) return '';
    try { return this.env.inviteURL(room.token); } catch { return ''; }
  }
  /** Seconds left in the countdown (3, 2, 1), 0 at the start. */
  countdownLeft(): number { return Math.max(0, Math.ceil((this.state.countdownEnd - this.env.now()) / 1000)); }
  bannerVisible(): boolean { return !!this.state.fleet.banner && this.env.now() < this.state.fleet.bannerUntil; }
}
