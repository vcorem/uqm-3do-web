// @vitest-environment node
import { describe, expect, it } from 'vitest';
import { OnlineController, NAME_KEY, COUNTDOWN_MS, cleanMessage, clock, waitingTime, type SessionHandlers, type SessionLike } from './OnlineController';
import type { LobbyRoom, LobbyStatus, LobbyWatcher } from './CosmoLobby';
import type { MatchStart, NetplayState, Role } from './CosmoNetplay';

const token = (c: string) => c.repeat(32);
function netState(over: Partial<NetplayState> = {}): NetplayState {
  return { phase: 'connecting', message: '', role: 'host', room: token('a'), round: 0, tick: 0, localFleet: [], peerFleet: [],
    localReady: false, peerReady: false, localRematch: false, peerRematch: false, verifiedChecksums: 0, name: '', peerFleetKnown: false, ...over };
}
class FakeSession implements SessionLike {
  state: NetplayState = netState();
  connected: { room: string; role: Role; name?: string | null } | null = null;
  readied = false; rematched = false; closed = false; fleets: number[][] = [];
  constructor(readonly handlers: SessionHandlers, private readonly refuse = '') {}
  connect(options: { origin: string; room: string; role: Role; name?: string | null }): void {
    if (this.refuse) throw new Error(this.refuse);
    this.connected = options;
    this.set({ phase: 'connecting', role: options.role, room: options.room, name: options.name ?? '' });
  }
  set(over: Partial<NetplayState>): void { this.state = { ...this.state, ...over }; this.handlers.onState(this.state); }
  chooseFleet(fleet: number[]): void { this.fleets.push(fleet); this.state = { ...this.state, localFleet: [...fleet] }; }
  ready(): void { this.readied = true; this.set({ localReady: true }); }
  rematch(): void { this.rematched = true; this.set({ localRematch: true }); }
  close(): void { this.closed = true; this.set({ phase: 'closed', message: 'You left the room.' }); }
  lobby(): void { this.set({ phase: 'lobby', peerFleetKnown: true }); }
}
function harness(options: { refuse?: string; storage?: Record<string, string> } = {}) {
  let now = 1000;
  const timers = new Map<number, { at: number; fn: () => void }>();
  let next = 1;
  const store: Record<string, string> = { ...options.storage };
  const sessions: FakeSession[] = [];
  const watchers: { handlers: { onRooms(rooms: LobbyRoom[]): void; onStatus(status: LobbyStatus, message: string): void }; closed: boolean }[] = [];
  const calls: string[] = [];
  let lost: [number, number] = [0, 0];
  const controller = new OnlineController({
    now: () => now, random: () => 0.5,
    setTimeout: (fn, ms) => { const id = next++; timers.set(id, { at: now + ms, fn }); return id; },
    clearTimeout: id => { timers.delete(id as number); },
    storage: { get: key => store[key] ?? null, set: (key, value) => { store[key] = value; } },
    origin: 'https://urquanmasters.org/cosmoclash/',
    createToken: () => token(String(sessions.length + 1)),
    watchLobby: handlers => {
      const entry = { handlers, closed: false };
      watchers.push(entry);
      handlers.onStatus('connecting', 'Connecting to the room list…');
      return { get state() { return 'connecting' as LobbyStatus; }, rooms: [], close() { entry.closed = true; handlers.onStatus('closed', 'Left the room list.'); } } as LobbyWatcher;
    },
    createSession: handlers => { const session = new FakeSession(handlers, options.refuse); sessions.push(session); return session; },
    inviteURL: room => 'https://urquanmasters.org/cosmoclash/?cosmoRoom=' + room,
    copy: async () => { calls.push('copy'); },
    fullscreen: () => { calls.push('fullscreen'); return true; },
    prepare: async () => { calls.push('prepare'); },
    sound: name => calls.push('sound:' + name),
    clearInvite: () => calls.push('clearInvite'),
    game: {
      menu: () => calls.push('menu'), start: () => calls.push('start'), stop: () => calls.push('stop'), lost: () => lost,
    },
  });
  return {
    controller, s: controller.state, sessions, watchers, calls, store,
    get session() { return sessions[sessions.length - 1]; },
    get watcher() { return watchers[watchers.length - 1]; },
    rooms(rooms: LobbyRoom[]) { this.watcher.handlers.onStatus('open', 'Showing the open rooms.'); this.watcher.handlers.onRooms(rooms); },
    keys(...names: string[]) { for (const name of names) controller.key(name); },
    advance(ms: number) {
      const end = now + ms;
      for (;;) {
        const due = [...timers.entries()].filter(([, t]) => t.at <= end).sort((a, b) => a[1].at - b[1].at)[0];
        if (!due) break;
        timers.delete(due[0]); now = due[1].at; due[1].fn();
      }
      now = end;
    },
    setLost(value: [number, number]) { lost = value; },
  };
}
const room = (c: string, name: string, created = 0): LobbyRoom => ({ id: token(c), name, engine: 'rust', created });
const flush = () => new Promise(resolve => setTimeout(resolve, 0));
/** A host and an opponent in the fleet screen. */
function inFleet(role: Role = 'host') {
  const h = harness();
  h.controller.openLobby();
  h.rooms(role === 'guest' ? [room('b', 'Their room')] : []);
  if (role === 'guest') h.keys('Enter'); else h.keys('Enter', 'Enter');
  h.session.set({ phase: 'waiting' });
  h.session.set({ phase: 'handshake' });
  h.session.lobby();
  return h;
}

describe('lobby', () => {
  it('opens with the list; the newest room takes the focus when it comes before the player moves', () => {
    const h = harness();
    h.controller.openLobby();
    expect(h.s).toMatchObject({ screen: 'lobby', focus: 'create', items: ['create', 'back'] });
    expect(h.s.lobby.status).toBe('connecting');
    h.rooms([room('b', 'Newest', 2), room('a', 'Older', 1)]);
    expect(h.s.items).toEqual(['room:' + token('b'), 'room:' + token('a'), 'create', 'back']);
    expect(h.s.focus).toBe('room:' + token('b'));
    h.keys('ArrowDown');
    expect(h.s.focus).toBe('room:' + token('a'));
    // The focused room went away: the focus stays at the same place in the list.
    h.rooms([room('b', 'Newest', 2)]);
    expect(h.s.focus).toBe('room:' + token('b'));
    h.rooms([room('c', 'Brand new', 3), room('b', 'Newest', 2)]);
    expect(h.s.focus).toBe('room:' + token('b'));
  });
  it('Left and Right jump between the list and the buttons; Esc goes back to the main menu', () => {
    const h = harness();
    h.controller.openLobby();
    h.rooms([room('b', 'B'), room('a', 'A')]);
    h.keys('ArrowDown', 'ArrowRight');
    expect(h.s.focus).toBe('create');
    h.keys('ArrowRight');
    expect(h.s.focus).toBe('back');
    h.keys('ArrowLeft', 'ArrowLeft');
    expect(h.s.focus).toBe('room:' + token('a'));
    h.keys('Escape');
    expect(h.s.screen).toBe('hidden');
    expect(h.calls).toContain('menu');
    expect(h.watcher.closed).toBe(true);
  });
  it('a lost list shows Try again, which reconnects at once', () => {
    const h = harness();
    h.controller.openLobby();
    h.watcher.handlers.onStatus('error', 'Lost the connection to the room list. Reconnecting…');
    expect(h.s.lobby).toMatchObject({ status: 'error', error: 'Lost the connection to the room list.' });
    expect(h.s.items).toEqual(['retry', 'create', 'back']);
    expect(h.s.focus).toBe('create');
    h.keys('ArrowLeft', 'Enter');
    expect(h.watchers).toHaveLength(2);
    expect(h.watchers[0].closed).toBe(true);
    expect(h.s.lobby.status).toBe('connecting');
  });
  it('a server without room lists offers a private room with an invite link at once', async () => {
    const h = harness();
    h.controller.openLobby();
    h.watcher.handlers.onStatus('unsupported', 'Invalid room URL or role.');
    expect(h.s).toMatchObject({ items: ['create', 'back'] });
    h.keys('Enter');
    expect(h.s.screen).toBe('waiting');
    expect(h.s.room).toMatchObject({ role: 'host', listed: false, name: '' });
    expect(h.session.connected).toMatchObject({ role: 'host', name: null });
    expect(h.s.focus).toBe('copy');
    h.keys('Enter');
    await flush();
    expect(h.calls).toContain('copy');
    expect(h.s.notice?.text).toBe('Invite link copied. Send it to your opponent.');
    // The watcher does not come back for this server.
    h.keys('Escape');
    expect(h.watchers).toHaveLength(1);
  });
  it('waiting time and clock texts', () => {
    expect(waitingTime(0, 99999)).toBe('new');
    expect(waitingTime(1000, 1000 + 59000)).toBe('new');
    expect(waitingTime(1000, 1000 + 125000)).toBe('2 min');
    expect(waitingTime(1000, 1000 + 2 * 3600000)).toBe('2 h');
    expect(clock(83000)).toBe('1:23');
    expect(cleanMessage('Lost the connection to online play. Create a new room.')).toBe('Lost the connection to online play.');
  });
});

describe('name, waiting and joining', () => {
  it('Create room gives a default name, refuses bad names with the reason, and remembers an accepted name', () => {
    const h = harness();
    h.controller.openLobby();
    h.keys('Enter');
    expect(h.s).toMatchObject({ screen: 'name', focus: 'name', name: 'Pilot 55' });
    h.controller.setName('   ');
    h.controller.activate('open');
    expect(h.s).toMatchObject({ screen: 'name', nameError: 'Type a name for your room.', focus: 'name' });
    h.controller.setName('x'.repeat(25));
    h.controller.activate('open');
    expect(h.s.nameError).toBe('Use at most 24 characters.');
    h.controller.setName('  Friday   duel  ');
    h.controller.activate('open');
    expect(h.s).toMatchObject({ screen: 'waiting', focus: 'copy' });
    expect(h.session.connected).toMatchObject({ role: 'host', name: 'Friday duel' });
    expect(h.store[NAME_KEY]).toBe('Friday duel');
    expect(h.calls.filter(call => call === 'fullscreen')).toHaveLength(1);
    // Cancel closes the room; the next Create room offers the remembered name.
    h.keys('ArrowRight', 'Enter');
    expect(h.session.closed).toBe(true);
    expect(h.s).toMatchObject({ screen: 'lobby', focus: 'create', notice: { text: 'Your room is closed.' } });
    h.keys('Enter');
    expect(h.s.name).toBe('Friday duel');
    h.keys('Escape');
    expect(h.s).toMatchObject({ screen: 'lobby', focus: 'create' });
  });
  it('a name that the relay refuses goes back to the name dialog with its reason', () => {
    const h = harness();
    h.controller.openLobby();
    h.keys('Enter', 'Enter');
    h.session.set({ phase: 'error', message: 'Room names need 1 to 24 visible characters.' });
    expect(h.s).toMatchObject({ screen: 'name', nameError: 'Room names need 1 to 24 visible characters.' });
    const refused = harness({ refuse: 'Room names need 1 to 24 visible characters.' });
    refused.controller.openLobby();
    refused.keys('Enter', 'Enter');
    expect(refused.s).toMatchObject({ screen: 'name', nameError: 'Room names need 1 to 24 visible characters.' });
  });
  it('a full room list or an expired room returns to the lobby with the relay message', () => {
    const h = harness();
    h.controller.openLobby();
    h.keys('Enter', 'Enter');
    h.session.set({ phase: 'error', message: 'The room list is full. Try again in a few minutes.' });
    expect(h.s).toMatchObject({ screen: 'lobby', notice: { text: 'The room list is full. Try again in a few minutes.', kind: 'error' } });
  });
  it('a guest joins from the list; Enter does nothing while joining; a refused join says why', () => {
    const h = harness();
    h.controller.openLobby();
    h.rooms([room('b', 'Their room')]);
    h.keys('Enter');
    expect(h.s).toMatchObject({ screen: 'joining', focus: 'cancel', room: { role: 'guest', name: 'Their room', token: token('b') } });
    expect(h.session.connected).toMatchObject({ role: 'guest', room: token('b'), name: null });
    h.keys('Enter');
    expect(h.s.screen).toBe('joining');
    h.session.set({ phase: 'error', message: 'Room does not exist or has expired.' });
    expect(h.s).toMatchObject({ screen: 'lobby', notice: { text: 'Could not join this room. Room does not exist or has expired.' } });
    // The room is still in the list: Left goes to it, Enter joins again, Esc cancels.
    h.keys('ArrowLeft', 'Enter');
    expect(h.s.screen).toBe('joining');
    h.keys('Escape');
    expect(h.s.screen).toBe('lobby');
    expect(h.sessions[1].closed).toBe(true);
  });
  it('an invite link offers Join and Not now', () => {
    const h = harness();
    h.controller.invite(token('c'));
    expect(h.s).toMatchObject({ screen: 'invite', focus: 'join', items: ['join', 'dismiss'] });
    h.keys('Enter');
    expect(h.s.screen).toBe('joining');
    expect(h.session.connected).toMatchObject({ role: 'guest', room: token('c') });
    expect(h.calls).toContain('clearInvite');
    const n = harness();
    n.controller.invite(token('d'));
    n.keys('Escape');
    expect(n.s.screen).toBe('hidden');
    expect(n.calls).toEqual(['clearInvite', 'menu']);
    n.controller.invite('not a token');
    expect(n.s.screen).toBe('hidden');
  });
});

describe('fleet choice, battle and result', () => {
  it('opponent found: the fleet screen; Enter adds ships in order, the focus goes to Ready after three', () => {
    const h = inFleet();
    expect(h.s).toMatchObject({ screen: 'fleet', focus: 'ship', fleet: { carousel: 0, picks: [] } });
    expect(h.calls).toContain('sound:found');
    expect(h.controller.bannerVisible()).toBe(true);
    h.keys('ArrowRight');
    expect(h.s.fleet.carousel).toBe(1);
    h.keys('Enter');
    expect(h.s.fleet).toMatchObject({ picks: [1], carousel: 2 });
    expect(h.session.fleets.at(-1)).toEqual([1]);
    h.keys('Enter', 'Enter');
    expect(h.s.fleet.picks).toEqual([1, 2, 0]);
    expect(h.s.focus).toBe('ready');
    // Enter on a ship that is in the fleet takes it out.
    h.keys('ArrowUp');
    h.keys('Enter');
    expect(h.s.fleet.picks).toEqual([1, 2]);
    h.keys('Backspace');
    expect(h.s.fleet).toMatchObject({ picks: [1], carousel: 2 });
  });
  it('finishing with fewer ships: Down, Ready; Ready with no ships says why', async () => {
    const h = inFleet();
    h.keys('ArrowDown', 'Enter');
    expect(h.s.notice?.text).toBe('Pick at least one ship first.');
    expect(h.s.focus).toBe('ship');
    expect(h.session.readied).toBe(false);
    h.keys('Enter', 'ArrowDown');
    expect(h.s.focus).toBe('ready');
    h.keys('ArrowRight');
    expect(h.s.focus).toBe('undo');
    h.keys('ArrowLeft', 'Enter');
    await flush();
    expect(h.calls).toContain('prepare');
    expect(h.session.readied).toBe(true);
    expect(h.s.items).toEqual(['ship', 'leave']);
    h.keys('Enter');
    expect(h.s.notice?.text).toBe('You are ready. Waiting for your opponent.');
    expect(h.s.fleet.picks).toEqual([0]);
  });
  it('Esc in the fleet screen asks first; Stay is the default', () => {
    const h = inFleet();
    h.keys('Escape');
    expect(h.s).toMatchObject({ confirm: true, confirmFocus: 'stay' });
    h.keys('Enter');
    expect(h.s.confirm).toBe(false);
    expect(h.session.closed).toBe(false);
    h.keys('Escape', 'ArrowRight', 'Enter');
    expect(h.session.closed).toBe(true);
    expect(h.s).toMatchObject({ screen: 'lobby', notice: { text: 'You left the room.' } });
    expect(h.calls).toContain('stop');
  });
  it('both ready: a countdown, then the battle; in the battle only Esc is a menu key', () => {
    const h = inFleet();
    const start: MatchStart = { round: 0, seed: 7, fleets: [[0], [1, 2]], localPlayer: 0 };
    h.session.set({ phase: 'playing' });
    h.session.handlers.onStart(start);
    expect(h.calls).toContain('start');
    expect(h.s).toMatchObject({ screen: 'countdown', start });
    expect(h.controller.running).toBe(false);
    expect(h.controller.countdownLeft()).toBe(3);
    h.advance(1100);
    expect(h.controller.countdownLeft()).toBe(2);
    h.keys('Escape');
    expect(h.s.confirm).toBe(true);
    h.advance(COUNTDOWN_MS);
    // The question stays open when the battle starts.
    expect(h.s).toMatchObject({ screen: 'battle', confirm: true });
    expect(h.controller.running).toBe(true);
    h.keys('Escape');
    expect(h.s.confirm).toBe(false);
    expect(h.controller.key('ArrowUp')).toBe(false);
    expect(h.controller.key('/')).toBe(false);
    expect(h.controller.key('Enter')).toBe(false);
    expect(h.controller.key('Escape')).toBe(true);
    h.keys('ArrowLeft', 'Enter');
    expect(h.session.closed).toBe(true);
    expect(h.s.screen).toBe('lobby');
  });
  it('the result: Victory data, rematch needs both, then the fleet screen with the last fleet; Back to lobby', () => {
    const h = inFleet('guest');
    h.keys('Enter');
    h.session.set({ phase: 'playing' });
    h.session.handlers.onStart({ round: 0, seed: 7, fleets: [[2], [0]], localPlayer: 1 });
    h.advance(COUNTDOWN_MS);
    h.setLost([1, 0]);
    h.session.set({ phase: 'ended' });
    h.session.handlers.onEnd(2);
    expect(h.s).toMatchObject({ screen: 'result', focus: 'rematch', result: { winner: 2, localPlayer: 1, fleets: [[2], [0]], lost: [1, 0] } });
    h.session.set({ peerRematch: true });
    h.keys('Enter');
    expect(h.session.rematched).toBe(true);
    h.session.set({ phase: 'lobby', round: 1, localFleet: [0], localRematch: false, peerRematch: false });
    expect(h.s).toMatchObject({ screen: 'fleet', focus: 'ready', fleet: { picks: [0], banner: 'Rematch!' } });
    const b = inFleet();
    b.keys('Enter');
    b.session.set({ phase: 'playing' });
    b.session.handlers.onStart({ round: 0, seed: 1, fleets: [[0], [1]], localPlayer: 0 });
    b.session.set({ phase: 'ended' });
    b.session.handlers.onEnd(1);
    b.keys('Escape');
    expect(b.s).toMatchObject({ screen: 'lobby', notice: { text: 'You left the room.' } });
    expect(b.session.closed).toBe(true);
  });
  it('the opponent leaves or the connection is lost: back to the lobby with the reason', () => {
    for (const [phase, message, kind] of [['closed', 'Your opponent left the room.', 'info'], ['error', 'The game states differ at tick 90. The match was stopped.', 'error']] as const) {
      const h = inFleet();
      h.session.set({ phase: 'playing' });
      h.session.handlers.onStart({ round: 0, seed: 7, fleets: [[0], [1]], localPlayer: 0 });
      h.advance(COUNTDOWN_MS);
      h.session.set({ phase, message });
      expect(h.s).toMatchObject({ screen: 'lobby', focus: 'create', notice: { text: message, kind } });
      expect(h.calls.filter(call => call === 'stop').length).toBeGreaterThan(1);
      expect(h.controller.current).toBe(null);
    }
  });
  it('full screen is asked at the first choice, not for Esc, and again after a refusal', async () => {
    let answer = false;
    const calls: string[] = [];
    const h = harness();
    (h.controller as unknown as { env: { fullscreen(): boolean } }).env.fullscreen = () => { calls.push('ask'); return answer; };
    h.controller.invite(token('e'));
    h.keys('Escape');
    expect(calls).toEqual([]);
    h.controller.openLobby();
    h.keys('Enter');
    await flush();
    expect(calls).toEqual(['ask']);
    answer = true;
    h.keys('Escape', 'Enter');
    await flush();
    h.keys('Escape', 'Enter');
    expect(calls).toEqual(['ask', 'ask']);
  });
});
