import { afterEach, describe, expect, it } from 'vitest';
import { OnlineController, type SessionHandlers, type SessionLike } from './OnlineController';
import { OnlineView } from './OnlineView';
import { attachLiveRegion, announced } from '@/ui/live';
import type { LobbyRoom, LobbyStatus, LobbyWatcher } from './CosmoLobby';
import type { NetplayState } from './CosmoNetplay';

afterEach(() => { document.body.innerHTML = ''; });
function setup() {
  const container = document.createElement('div');
  document.body.appendChild(container);
  attachLiveRegion(container);
  let lobby: { onRooms(rooms: LobbyRoom[]): void; onStatus(status: LobbyStatus, message: string): void } | null = null;
  let session: (SessionLike & { set(over: Partial<NetplayState>): void; handlers: SessionHandlers }) | null = null;
  let view!: OnlineView;
  const controller = new OnlineController({
    now: () => Date.now(), setTimeout: () => 0, clearTimeout: () => {},
    origin: 'http://localhost/cosmoclash/', createToken: () => 'a'.repeat(32),
    watchLobby: handlers => { lobby = handlers; return { state: 'connecting', rooms: [], close() {} } as unknown as LobbyWatcher; },
    createSession: handlers => {
      let state: NetplayState = { phase: 'connecting', message: '', role: 'host', room: 'a'.repeat(32), round: 0, tick: 0, localFleet: [], peerFleet: [],
        localReady: false, peerReady: false, localRematch: false, peerRematch: false, verifiedChecksums: 0, name: '', peerFleetKnown: false };
      session = { handlers, get state() { return state; }, connect() {}, chooseFleet(fleet) { state = { ...state, localFleet: fleet }; }, ready() {}, rematch() {}, close() {},
        set(over) { state = { ...state, ...over }; handlers.onState(state); } };
      return session;
    },
    inviteURL: token => 'http://localhost/cosmoclash/?cosmoRoom=' + token,
    game: { menu() {}, start() {}, stop() {}, lost: () => [0, 0] },
    changed: () => view?.render(),
  });
  view = new OnlineView(container, controller);
  return { container, controller, view, get lobby() { return lobby!; }, get session() { return session!; } };
}
const items = (root: HTMLElement) => [...root.querySelectorAll<HTMLElement>('[data-item]')].map(node => node.dataset.item);

describe('online screens in the page', () => {
  it('the lobby lists rooms as buttons with a visible focus, says the screen in the live region, and a click joins', () => {
    const t = setup();
    t.controller.openLobby();
    expect(t.view.root.hidden).toBe(false);
    expect(t.view.root.querySelector('h1')?.textContent).toBe('Online Melee');
    expect(t.view.root.textContent).toContain('Connecting to the room list…');
    t.lobby.onStatus('open', '');
    t.lobby.onRooms([{ id: 'b'.repeat(32), name: 'קרב ⚔', engine: 'rust', created: Date.now() - 180000 }]);
    expect(items(t.view.root)).toEqual(['room:' + 'b'.repeat(32), 'create', 'back']);
    const roomButton = t.view.root.querySelector<HTMLElement>('.cc-room')!;
    expect(roomButton.classList.contains('is-focus')).toBe(true);
    expect(document.activeElement).toBe(roomButton);
    expect(roomButton.querySelector('.cc-room-name')?.getAttribute('dir')).toBe('auto');
    expect(roomButton.textContent).toContain('waiting 3 min');
    expect(announced()).toBe('Online Melee lobby. 1 open room.');
    expect(document.getElementById('cosmo-status')?.getAttribute('role')).toBe('status');
    roomButton.click();
    expect(t.controller.state.screen).toBe('joining');
    expect(t.view.root.textContent).toContain('“קרב ⚔”');
  });
  it('the name dialog: typing, the reason for a refused name, Enter and Esc in the field', () => {
    const t = setup();
    t.controller.openLobby();
    t.lobby.onStatus('open', '');
    t.controller.activate('create');
    const input = t.view.root.querySelector<HTMLInputElement>('input')!;
    expect(document.activeElement).toBe(input);
    input.value = '   ';
    input.dispatchEvent(new Event('input'));
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
    expect(t.view.root.querySelector('.cc-error')?.textContent).toBe('Type a name for your room.');
    expect(t.view.root.querySelector('input')).toBe(input);
    input.value = 'Duel';
    input.dispatchEvent(new Event('input'));
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true, repeat: true }));
    expect(t.controller.state.screen).toBe('name');
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
    expect(t.controller.state.screen).toBe('waiting');
    expect(t.view.root.textContent).toContain('Room Open');
    expect(t.view.root.textContent).toContain('closes after 10 min');
    expect(document.activeElement?.textContent).toBe('Copy invite link');
  });
  it('the fleet screen shows the order, the opponent and Ready; the leave question has Stay first', () => {
    const t = setup();
    t.controller.openLobby();
    t.lobby.onStatus('open', '');
    t.controller.activate('create');
    t.controller.activate('open');
    t.session.set({ phase: 'lobby', peerFleetKnown: true });
    expect(t.view.root.querySelector('h1')?.textContent).toBe('Choose Your Fleet');
    expect(t.view.root.textContent).toContain('You are Player 1 · cyan · left');
    t.controller.key('Enter');
    t.controller.key('Enter');
    const slots = [...t.view.root.querySelectorAll('.cc-slot')].map(node => node.textContent);
    expect(slots).toEqual(['1MichiGichi', '2Gondi', '3Next']);
    expect(t.view.root.querySelector('[data-item="ready"]')?.textContent).toBe('Ready · 2 ships');
    t.session.set({ peerFleet: [1] });
    expect(t.view.root.querySelector('.cc-opponent')?.textContent).toBe('Opponent: choosing · 1 of 3 picked');
    t.session.set({ peerReady: true });
    expect(t.view.root.querySelector('.cc-opponent')?.textContent).toBe('Opponent: ready with 1 ship');
    t.view.root.querySelector<HTMLElement>('[data-browse="1"]')!.click();
    expect(t.controller.state.fleet.carousel).toBe(0);
    t.controller.key('Escape');
    const dialog = t.view.root.querySelector('[role=alertdialog]')!;
    expect(dialog.textContent).toContain('Leave the room?');
    expect(document.activeElement?.textContent).toBe('Stay');
    expect(announced()).toBe('Leave the room? Stay or Leave.');
  });
  it('a private room shows its invite link; the result shows Victory and the rematch state', () => {
    const t = setup();
    t.controller.openLobby();
    t.lobby.onStatus('unsupported', 'Invalid room URL or role.');
    expect(t.view.root.textContent).toContain('No room list on this server');
    expect(t.view.root.querySelector('[data-item="create"]')?.textContent).toBe('Create private room');
    t.controller.activate('create');
    expect(t.view.root.textContent).toContain('Private Room Open');
    expect(t.view.root.querySelector('.cc-link')?.textContent).toMatch(/\/cosmoclash\/\?cosmoRoom=a{32}$/);
    t.session.set({ phase: 'lobby', peerFleetKnown: true });
    t.controller.key('Enter');
    t.session.set({ phase: 'playing' });
    t.session.handlers.onStart({ round: 0, seed: 1, fleets: [[0], [1]], localPlayer: 0 });
    t.session.set({ phase: 'ended' });
    t.session.handlers.onEnd(1);
    expect(t.view.root.querySelector('h1')?.textContent).toBe('Victory');
    t.session.set({ peerRematch: true });
    expect(t.view.root.querySelector('[data-item="rematch"]')?.textContent).toBe('Accept rematch');
    expect(t.view.root.textContent).toContain('Your opponent wants a rematch!');
  });
});
