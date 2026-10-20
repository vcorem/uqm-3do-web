/**
 * Draws the Online Melee screens of OnlineController inside the game
 * container, in the look of the Cosmo Clash menus (src/ui/cosmo-ui.css).
 * Each focus item is a real button (data-item), so mouse, touch, Tab and
 * screen readers work; the keyboard focus of the controller is the DOM focus.
 */
import './online.css';
import '@/ui/cosmo-ui.css';
import { assetURL } from '@/base';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { el, button } from '@/ui/dom';
import { announce } from '@/ui/live';
import {
  OnlineController, clock, waitingTime, LISTED_ROOM_MINUTES, PRIVATE_ROOM_MINUTES, SHIP_COUNT, NEXT_MS, type OnlineState,
} from './OnlineController';
import type { FleetView } from '@/match/FleetMatch';
import { fleetLine } from '@/ui/fleetChips';

const shipName = (id: number) => CHARACTER_REGISTRY[id]?.name ?? '?';
const plural = (n: number, word: string) => `${n} ${word}${n === 1 ? '' : 's'}`;
/** The ship destroyed last (the last of the destroyed ships). */
const lastLost = (side: FleetView | undefined) => side && side.lost ? side.ships[side.lost - 1] : -1;

export class OnlineView {
  readonly root: HTMLDivElement;
  private key = '';
  private input: HTMLInputElement | null = null;
  /** A new name field: its text is selected once, so that typing replaces the default name. */
  private selectName = false;
  private binds = new Map<string, HTMLElement>();

  constructor(private readonly container: HTMLElement, private readonly controller: OnlineController) {
    this.root = el('div', { class: 'cc-online', id: 'cosmo-online' });
    this.root.style.setProperty('--cc-art', `url(${assetURL('textures/main-bk.jpg')})`);
    this.root.hidden = true;
    container.appendChild(this.root);
    this.root.addEventListener('click', event => {
      const target = event.target as HTMLElement;
      const browse = target.closest<HTMLElement>('[data-browse]');
      if (browse) { this.controller.browse(Number(browse.dataset.browse) as 1 | -1); return; }
      const item = target.closest<HTMLElement>('[data-item]');
      if (item?.dataset.item && item !== this.input) this.controller.activate(item.dataset.item);
    });
    this.root.addEventListener('mouseover', event => {
      // The pointer moves the focus, but never out of the text field while the player types.
      if (this.input && document.activeElement === this.input) return;
      const item = (event.target as HTMLElement).closest<HTMLElement>('[data-item]');
      if (item?.dataset.item && item !== this.input) this.controller.focus(item.dataset.item);
    });
    this.root.addEventListener('focusin', event => {
      const item = (event.target as HTMLElement).closest<HTMLElement>('[data-item]');
      if (item?.dataset.item) this.controller.focus(item.dataset.item);
    });
    // The mouse arrows never take the focus from the keyboard.
    this.root.addEventListener('mousedown', event => { if ((event.target as HTMLElement).closest('[data-browse]')) event.preventDefault(); });
  }

  render(): void {
    const s = this.controller.state;
    const visible = s.screen !== 'hidden';
    this.root.hidden = !visible;
    if (!visible) { this.key = ''; this.root.replaceChildren(); this.input = null; return; }
    const key = this.structure(s);
    if (key !== this.key) {
      // The name field stays while the name screen changes (an error, Open room waits): the caret stays too.
      const keepInput = s.screen === 'name' && this.key.startsWith('name|') ? this.input : null;
      this.key = key;
      this.binds.clear();
      const screen = this.build(s, keepInput);
      const nodes: Node[] = [screen];
      if (s.confirm) nodes.push(this.dialog(s));
      this.root.replaceChildren(...nodes);
      for (const node of this.root.querySelectorAll<HTMLElement>('[data-bind]')) this.binds.set(node.dataset.bind!, node);
    }
    for (const [name, value] of Object.entries(this.texts(s))) {
      const node = this.binds.get(name);
      if (node && node.textContent !== value) node.textContent = value;
    }
    if (this.input && s.screen === 'name' && this.input.value !== s.name) this.input.value = s.name;
    this.applyFocus(s);
    announce(this.announcement(s));
  }

  // ------------------------------------------------------------ structure
  /** The parts that change the DOM; texts change in place (texts()). */
  private structure(s: OnlineState): string {
    const net = s.net;
    switch (s.screen) {
      case 'lobby': return ['lobby', s.lobby.status, s.lobby.rooms.map(room => room.id).join(','), s.notice?.kind ?? '', s.items.join(',')].join('|');
      case 'name': return ['name', s.nameError ? 'error' : '', s.opening].join('|');
      case 'waiting': return ['waiting', s.room?.listed, s.notice?.kind ?? ''].join('|');
      case 'joining': return 'joining|' + !!s.room?.name;
      case 'invite': return 'invite';
      case 'fleet': return ['fleet', s.fleet.picks.join(','), net?.localReady, net?.localReady && net.peerReady, net?.changing, s.fleet.preparing, this.controller.bannerVisible(), s.notice?.kind ?? '', s.confirm].join('|');
      case 'countdown': return ['countdown', s.confirm, s.battle].join('|');
      // The timer turns red for the last 5 seconds.
      case 'next': return ['next', s.next?.battle, s.next?.mine, s.next?.chosen, s.next?.auto, this.controller.nextMsLeft() <= 5000, s.notice?.kind ?? '', s.confirm].join('|');
      case 'battle': return ['battle', s.confirm].join('|');
      case 'result': return ['result', s.result?.winner, net?.localRematch, net?.peerRematch, s.confirm].join('|');
      default: return s.screen;
    }
  }
  private build(s: OnlineState, keepInput: HTMLInputElement | null): HTMLElement {
    switch (s.screen) {
      case 'lobby': return this.lobby(s);
      case 'name': return this.name(s, keepInput);
      case 'waiting': return this.waiting(s);
      case 'joining': return this.joining(s);
      case 'invite': return this.invite();
      case 'fleet': return this.fleet(s);
      case 'countdown': return this.countdown(s);
      case 'next': return s.next?.mine ? this.nextChoice(s) : this.nextWait(s);
      case 'battle': return el('div', { class: 'cc-screen cc-over cc-battle' }, el('p', { class: 'cc-battle-status', bind: 'battle-status' }));
      case 'result': return this.result(s);
      default: return el('div');
    }
  }
  private screen(name: string, art: boolean, ...children: (Node | null | false)[]): HTMLDivElement {
    return el('div', { class: `cc-screen cc-${name}` + (art ? ' cc-art' : ' cc-over'), data: { screen: name } }, ...children);
  }
  private notice(s: OnlineState): HTMLElement {
    return el('p', { class: 'cc-notice' + (s.notice ? ' ' + s.notice.kind : ''), bind: 'notice', role: s.notice?.kind === 'error' ? 'alert' : undefined });
  }

  private lobby(s: OnlineState): HTMLElement {
    const status = s.lobby.status;
    let body: HTMLElement;
    if (s.lobby.rooms.length && status !== 'unsupported') {
      body = el('ul', { class: 'cc-rooms', aria: { label: 'Open rooms' } }, ...s.lobby.rooms.map(room => el('li', {},
        el('button', { type: 'button', class: 'cc-btn cc-room', item: 'room:' + room.id },
          el('span', { class: 'cc-room-name', dir: 'auto', text: room.name }),
          el('span', { class: 'cc-room-age', bind: 'age:' + room.id }),
          el('span', { class: 'cc-room-join', text: 'Join ›' })))));
    } else if (status === 'unsupported') {
      body = el('div', { class: 'cc-empty' },
        el('p', { class: 'cc-empty-title', text: 'No room list on this server' }),
        el('p', { text: 'Create a private room, then send its invite link to your opponent.' }));
    } else if (status === 'error') {
      body = el('div', { class: 'cc-empty' },
        el('p', { class: 'cc-empty-title', text: 'The room list is not available' }),
        el('p', { bind: 'lobby-error' }),
        el('p', { class: 'cc-muted', text: 'The game tries again by itself. You can also try now.' }));
    } else if (status === 'open') {
      body = el('div', { class: 'cc-empty' },
        el('p', { class: 'cc-empty-title', text: 'No open rooms yet' }),
        el('p', { text: 'Create a room: it shows here for other players, and the match starts when someone joins.' }));
    } else {
      body = el('div', { class: 'cc-empty' }, el('p', { class: 'cc-empty-title cc-pulse', text: 'Connecting to the room list…' }));
    }
    const buttons = el('div', { class: 'cc-buttons' },
      s.items.includes('retry') ? button('retry', 'Try again') : null,
      button('create', status === 'unsupported' ? 'Create private room' : 'Create room'),
      button('back', 'Back'));
    return this.screen('lobby', true,
      el('h1', { class: 'cc-title', text: 'Online Melee' }),
      el('p', { class: 'cc-sub', bind: 'lobby-status' }),
      this.notice(s),
      el('div', { class: 'cc-panel cc-list-panel' }, body),
      buttons,
      el('p', { class: 'cc-hint', text: s.lobby.rooms.length ? '↑ ↓ choose · Enter join · → buttons · Esc back' : '← → choose · Enter select · Esc back' }));
  }

  private name(s: OnlineState, keep: HTMLInputElement | null): HTMLElement {
    const input = keep ?? this.makeInput();
    if (!keep) this.selectName = true;
    this.input = input;
    input.setAttribute('aria-invalid', String(!!s.nameError));
    return this.screen('name', true,
      el('h1', { class: 'cc-title', text: 'Create Room' }),
      el('div', { class: 'cc-panel cc-name-panel' },
        el('label', { class: 'cc-label', text: 'Room name' }, input),
        el('p', { class: 'cc-muted' + (s.opening ? ' cc-pulse' : ''), text: s.opening ? 'Checking the room list…' : 'Other players see this name in the room list.' }),
        el('p', { class: 'cc-error', bind: 'name-error', role: 'alert' })),
      el('div', { class: 'cc-buttons' }, button('open', s.opening ? 'Opening…' : 'Open room', { aria: s.opening ? { disabled: 'true' } : {} }), button('cancel', 'Cancel')),
      el('p', { class: 'cc-hint', text: 'Enter open room · Esc cancel' }));
  }
  private makeInput(): HTMLInputElement {
    const input = el('input', { class: 'cc-input', item: 'name', dir: 'auto' }) as HTMLInputElement;
    input.type = 'text';
    input.autocomplete = 'off';
    input.spellcheck = false;
    input.enterKeyHint = 'go';
    input.addEventListener('input', () => this.controller.setName(input.value));
    input.addEventListener('keydown', event => {
      if (event.isComposing) return;
      if ((event.key === 'Enter' || event.key === 'Escape') && event.repeat) { event.preventDefault(); return; }
      if (event.key === 'Enter') { event.preventDefault(); this.controller.activate('open'); }
      else if (event.key === 'Escape') { event.preventDefault(); this.controller.key('Escape'); }
      else if (event.key === 'ArrowDown' || (event.key === 'Tab' && !event.shiftKey)) { event.preventDefault(); this.controller.focus('open'); }
    });
    return input;
  }

  private waiting(s: OnlineState): HTMLElement {
    const listed = !!s.room?.listed;
    return this.screen('waiting', true,
      el('h1', { class: 'cc-title', text: listed ? 'Room Open' : 'Private Room Open' }),
      listed ? el('p', { class: 'cc-room-title', dir: 'auto', text: `“${s.room?.name ?? ''}”` }) : null,
      el('div', { class: 'cc-panel cc-wait-panel' },
        el('p', { class: 'cc-big cc-pulse', bind: 'wait-status' }),
        el('p', { class: 'cc-muted', bind: 'wait-clock' }),
        el('p', { text: listed ? 'Your room is in the list. The match starts when someone joins.'
          : 'Send the invite link to your opponent. The match starts when they join.' }),
        listed ? null : el('p', { class: 'cc-link', bind: 'invite-link' })),
      this.notice(s),
      el('div', { class: 'cc-buttons' }, button('copy', 'Copy invite link'), button('cancel', 'Cancel')),
      el('p', { class: 'cc-hint', text: 'Esc closes the room' }));
  }

  private joining(s: OnlineState): HTMLElement {
    return this.screen('joining', true,
      el('h1', { class: 'cc-title', text: 'Joining' }),
      s.room?.name ? el('p', { class: 'cc-room-title', dir: 'auto', text: `“${s.room.name}”` }) : null,
      el('div', { class: 'cc-panel cc-wait-panel' }, el('p', { class: 'cc-big cc-pulse', bind: 'join-status' })),
      el('div', { class: 'cc-buttons' }, button('cancel', 'Cancel')),
      el('p', { class: 'cc-hint', text: 'Esc cancels' }));
  }

  private invite(): HTMLElement {
    return this.screen('invite', true,
      el('h1', { class: 'cc-title', text: 'Online Melee' }),
      el('div', { class: 'cc-panel cc-wait-panel' },
        el('p', { class: 'cc-big', text: 'You have an invite to an online fleet battle.' }),
        el('p', { class: 'cc-muted', text: 'Join the room of the player who sent you this link.' })),
      el('div', { class: 'cc-buttons' }, button('join', 'Join'), button('dismiss', 'Not now')),
      el('p', { class: 'cc-hint', text: 'Enter join · Esc not now' }));
  }

  private you(s: OnlineState): HTMLElement {
    const local = s.room?.role === 'guest' ? 1 : 0;
    return el('span', { class: local ? 'cc-p2' : 'cc-p1', text: local ? 'Player 2 · orange · right' : 'Player 1 · cyan · left' });
  }
  private local(s: OnlineState): 0 | 1 { return (s.start?.localPlayer ?? (s.room?.role === 'guest' ? 1 : 0)) as 0 | 1; }
  /** "You: chips" and "Opponent: chips" in the player colours. */
  private sides(s: OnlineState): HTMLElement[] {
    const local = this.local(s);
    return [fleetLine('You', local, s.sides?.[local]), fleetLine('Opponent', (1 - local) as 0 | 1, s.sides?.[1 - local])];
  }
  private fleet(s: OnlineState): HTMLElement {
    const ready = !!s.net?.localReady;
    const picks = s.fleet.picks;
    // Ship 1 opens the match; the order of the others does not matter (the loser chooses the next ship).
    const slots = el('div', { class: 'cc-slots', aria: { label: 'Your fleet: the first ship opens the match' } });
    for (let i = 0; i < SHIP_COUNT; i++) {
      const ship = picks[i];
      slots.append(el('div', { class: 'cc-slot' + (ship !== undefined ? ' filled' : '') + (i === 0 ? ' first' : '') },
        el('span', { class: 'cc-slot-n', text: i === 0 ? '1' : ship !== undefined ? '✓' : '+', aria: { hidden: 'true' } }),
        el('span', { class: 'cc-slot-ship', text: ship !== undefined ? shipName(ship) : i === picks.length && !ready ? (i === 0 ? 'Opener' : 'Add') : '—' })));
    }
    // Change fleet takes Ready back while the opponent still chooses (when both are ready the match starts).
    const locked = !!s.net?.changing || !!s.net?.peerReady;
    const buttons = ready ? el('div', { class: 'cc-buttons' },
      button('change', s.net?.changing ? 'Changing…' : 'Change fleet', { aria: locked ? { disabled: 'true' } : {} }),
      button('leave', 'Leave room'))
      : el('div', { class: 'cc-buttons' },
        button('ready', picks.length ? `Ready · ${plural(picks.length, 'ship')}` : 'Ready', { aria: picks.length && !s.fleet.preparing ? {} : { disabled: 'true' } }),
        button('undo', 'Undo', { aria: picks.length ? {} : { disabled: 'true' } }),
        button('leave', 'Leave room'));
    const screen = this.screen('fleet', false,
      el('div', { class: 'cc-fleet-head' },
        el('h1', { class: 'cc-title cc-title-small', text: 'Choose Your Fleet' }),
        el('p', { class: 'cc-sub' }, s.room?.name ? el('span', { dir: 'auto', text: `“${s.room.name}” · ` }) : '', 'You are ', this.you(s)),
        el('button', { type: 'button', class: 'cc-btn cc-ship' + (ready ? ' locked' : ''), item: 'ship' },
          el('span', { class: 'cc-ship-name', bind: 'ship-name' }),
          el('span', { class: 'cc-ship-detail', bind: 'ship-detail' }),
          el('span', { class: 'cc-ship-state', bind: 'ship-state' }))),
      this.controller.bannerVisible() ? el('div', { class: 'cc-banner', bind: 'banner' }) : null,
      el('button', { type: 'button', class: 'cc-arrow left', data: { browse: '-1' }, tabIndex: -1, aria: { label: 'Previous ship' }, text: '‹' }),
      el('button', { type: 'button', class: 'cc-arrow right', data: { browse: '1' }, tabIndex: -1, aria: { label: 'Next ship' }, text: '›' }),
      el('div', { class: 'cc-fleet-bottom' },
        el('div', { class: 'cc-tray cc-panel' },
          slots,
          el('p', { class: 'cc-muted cc-rule', text: 'Ship 1 opens the match. After each loss you choose your next ship from the ships left.' }),
          el('p', { class: 'cc-self', bind: 'self' }),
          el('p', { class: 'cc-opponent', bind: 'opponent' })),
        this.notice(s),
        buttons,
        el('p', { class: 'cc-hint', text: ready ? '← → look at the ships · ↓ Change fleet · Esc leave' : '← → ships · Enter add or remove · ↓ Ready · Backspace undo · Esc leave' })));
    if (s.room?.role === 'guest') screen.classList.add('cc-p2-side');
    return screen;
  }

  private countdown(s: OnlineState): HTMLElement {
    const local = this.local(s);
    const mine = s.sides?.[local], theirs = s.sides?.[1 - local];
    const now = (side: FleetView | undefined) => side && side.current >= 0 ? shipName(side.ships[side.current]) : '?';
    return this.screen('countdown', false,
      el('div', { class: 'cc-count', bind: 'count', aria: { hidden: 'true' } }),
      el('div', { class: 'cc-panel cc-count-panel' },
        el('p', { class: 'cc-big' }, s.battle ? 'Next battle · You are ' : 'You are ', this.you(s)),
        ...this.sides(s),
        el('p', { class: 'cc-muted', text: s.battle ? `Your ${now(mine)} against the enemy ${now(theirs)}.`
          : 'The first ship opens. After a loss, the loser chooses the next ship.' })),
      el('p', { class: 'cc-hint cc-hint-over', text: 'Fly: ↑ thrust · ← → turn · / fire · . special · Esc leave' }));
  }

  /** The time to choose: the seconds left and a bar that runs down (stopped when the ship is chosen). */
  private timer(s: OnlineState): HTMLElement {
    const left = this.controller.nextMsLeft();
    const bar = el('i', { class: 'cc-timer-run' + (s.next?.chosen !== null ? ' stopped' : '') });
    bar.style.animationDuration = NEXT_MS + 'ms';
    bar.style.animationDelay = -(NEXT_MS - left) + 'ms';
    return el('div', { class: 'cc-timer' + (left <= 5000 ? ' low' : ''), role: 'timer', aria: { label: 'Time to choose the next ship' } },
      el('span', { class: 'cc-timer-n', bind: 'timer' }),
      el('span', { class: 'cc-timer-bar' }, bar));
  }
  /** The loser chooses the next ship in the 3D ship choice (destroyed ships and ships not in the fleet are not shown). */
  private nextChoice(s: OnlineState): HTMLElement {
    const local = this.local(s);
    const chosen = s.next?.chosen !== null;
    const screen = this.screen('next', false,
      el('div', { class: 'cc-fleet-head' },
        el('h1', { class: 'cc-title cc-title-small', text: 'Choose Your Next Ship' }),
        el('p', { class: 'cc-sub' }, `Your ${shipName(lastLost(s.sides?.[local]))} was destroyed · You are `, this.you(s)),
        el('button', { type: 'button', class: 'cc-btn cc-ship' + (chosen ? ' locked' : ''), item: 'ship' },
          el('span', { class: 'cc-ship-name', bind: 'ship-name' }),
          el('span', { class: 'cc-ship-detail', bind: 'ship-detail' }),
          el('span', { class: 'cc-ship-state', bind: 'ship-state' }))),
      chosen ? null : el('button', { type: 'button', class: 'cc-arrow left', data: { browse: '-1' }, tabIndex: -1, aria: { label: 'Previous ship' }, text: '‹' }),
      chosen ? null : el('button', { type: 'button', class: 'cc-arrow right', data: { browse: '1' }, tabIndex: -1, aria: { label: 'Next ship' }, text: '›' }),
      el('div', { class: 'cc-fleet-bottom' },
        el('div', { class: 'cc-tray cc-panel cc-next-tray' },
          this.timer(s),
          el('p', { class: 'cc-self', bind: 'self' }),
          ...this.sides(s)),
        this.notice(s),
        el('div', { class: 'cc-buttons' }, button('leave', 'Leave match')),
        el('p', { class: 'cc-hint', text: chosen ? 'Esc leave' : '← → ships · Enter choose · Esc leave' })));
    if (local) screen.classList.add('cc-p2-side');
    return screen;
  }
  /** The winner waits for the loser's choice, with the same time. The winner's ship stays. */
  private nextWait(s: OnlineState): HTMLElement {
    const local = this.local(s);
    const mine = s.sides?.[local];
    const screen = this.screen('next', false,
      el('div', { class: 'cc-veil' }),
      el('h1', { class: 'cc-title cc-title-small', text: `Enemy ${shipName(lastLost(s.sides?.[1 - local]))} destroyed!` }),
      el('div', { class: 'cc-panel cc-next-panel' },
        el('p', { class: 'cc-big', text: 'Your opponent is choosing the next ship' }),
        this.timer(s),
        el('p', { class: 'cc-self', bind: 'self' }),
        ...this.sides(s),
        el('p', { class: 'cc-muted', text: mine && mine.current >= 0 ? `Your ${shipName(mine.ships[mine.current])} stays for the next battle.` : '' })),
      this.notice(s),
      // As in the battle: no keyboard focus here (Enter does nothing), Esc asks; the button is for the mouse.
      el('div', { class: 'cc-buttons' }, button('leave', 'Leave (Esc)')));
    screen.classList.add('cc-next-wait');
    return screen;
  }

  private result(s: OnlineState): HTMLElement {
    const result = s.result;
    const local = result?.localPlayer ?? 0;
    const winner = result?.winner ?? 0;
    const won = winner === local + 1, draw = winner === 0;
    const net = s.net;
    const label = net?.localRematch ? 'Rematch requested' : net?.peerRematch ? 'Accept rematch' : 'Rematch';
    const line = (title: string, player: 0 | 1) => fleetLine(title, player, result?.sides[player], 'cc-result-fleet');
    return this.screen('result', false,
      el('div', { class: 'cc-veil' }),
      el('h1', { class: 'cc-title cc-result-title ' + (draw ? 'draw' : won ? 'won' : 'lost'), text: draw ? 'Draw' : won ? 'Victory' : 'Defeat' }),
      el('p', { class: 'cc-big', text: draw ? 'Both fleets are gone.' : won ? 'You won the fleet match.' : 'Your opponent won the fleet match.' }),
      el('div', { class: 'cc-panel cc-result-panel' }, line('You', local as 0 | 1), line('Opponent', (1 - local) as 0 | 1),
        el('p', { class: 'cc-rematch', bind: 'rematch' })),
      this.notice(s),
      el('div', { class: 'cc-buttons' }, button('rematch', label, { aria: net?.localRematch ? { disabled: 'true' } : {} }), button('lobby', 'Back to lobby')),
      el('p', { class: 'cc-hint', text: 'Enter choose · Esc back to lobby' }));
  }

  private dialog(s: OnlineState): HTMLElement {
    const room = s.screen === 'fleet';
    const choosing = s.screen === 'next' && !!s.next && s.next.chosen === null;
    return el('div', { class: 'cc-dialog-veil' },
      el('div', { class: 'cc-dialog cc-panel', role: 'alertdialog', aria: { label: room ? 'Leave the room?' : 'Leave the match?' } },
        el('h2', { text: room ? 'Leave the room?' : 'Leave the match?' }),
        el('p', { text: room ? 'Your opponent goes back to the lobby.' : 'The match ends for both players. Your opponent goes back to the lobby.' }),
        s.screen === 'battle' ? el('p', { class: 'cc-muted', text: 'The battle goes on while this question is open.' }) : null,
        choosing ? el('p', { class: 'cc-muted', text: 'The time to choose goes on while this question is open.' }) : null,
        el('div', { class: 'cc-buttons' }, button('stay', 'Stay'), button('leave-confirm', 'Leave'))));
  }

  // ------------------------------------------------------------ texts
  private texts(s: OnlineState): Record<string, string> {
    const now = Date.now();
    const texts: Record<string, string> = { notice: s.notice?.text ?? '' };
    const net = s.net;
    switch (s.screen) {
      case 'lobby': {
        const n = s.lobby.rooms.length;
        texts['lobby-status'] = {
          idle: 'Connecting to the room list…', connecting: 'Connecting to the room list…',
          open: n ? `${plural(n, 'open room')}. Pick one to play, or create your own.` : 'Play an opponent on the internet.',
          error: 'Lost the room list. Trying again…', closed: 'The room list is closed.',
          unsupported: 'Private rooms with an invite link.',
        }[s.lobby.status];
        texts['lobby-error'] = s.lobby.error;
        for (const room of s.lobby.rooms) texts['age:' + room.id] = waitingTime(room.created, now) === 'new' ? 'new' : 'waiting ' + waitingTime(room.created, now);
        break;
      }
      case 'name': texts['name-error'] = s.nameError; break;
      case 'waiting': {
        const phase = net?.phase;
        // The screen is drawn once a second: one dot more each second.
        const dots = '.'.repeat(1 + Math.floor(now / 1000) % 3);
        texts['wait-status'] = !phase || phase === 'connecting' ? 'Opening the room…'
          : phase === 'handshake' ? 'Opponent found! Checking game versions…' : 'Waiting for an opponent' + dots;
        const minutes = s.room?.listed ? LISTED_ROOM_MINUTES : PRIVATE_ROOM_MINUTES;
        texts['wait-clock'] = `Open for ${clock(now - (s.room?.opened ?? now))} · closes after ${minutes} min without an opponent`;
        texts['invite-link'] = this.controller.inviteLink();
        break;
      }
      case 'joining': {
        const phase = net?.phase;
        texts['join-status'] = phase === 'handshake' ? 'Opponent found! Checking game versions…' : 'Connecting to the room…';
        break;
      }
      case 'fleet': {
        const ship = CHARACTER_REGISTRY[s.fleet.carousel];
        const picks = s.fleet.picks;
        const at = picks.indexOf(s.fleet.carousel);
        const ready = !!net?.localReady;
        texts['ship-name'] = ship.name;
        texts['ship-detail'] = `${ship.weapon} · ${ship.special}`;
        texts['ship-state'] = ready ? (at === 0 ? 'Opens the match' : at > 0 ? 'In your fleet' : 'Not in your fleet')
          : at === 0 ? 'Opens the match · Enter takes it out' : at > 0 ? 'In your fleet · Enter takes it out'
            : picks.length >= SHIP_COUNT ? 'Your fleet is full' : picks.length ? 'Enter adds it to your fleet' : 'Enter adds it · it opens the match';
        texts.self = ready ? (net?.changing ? 'Taking Ready back…' : net?.peerReady ? 'Both ready! Starting…' : 'You are ready. Waiting for your opponent…')
          : s.fleet.preparing ? 'Getting ready…'
            : picks.length ? `Pick up to ${SHIP_COUNT} ships, or say Ready with ${plural(picks.length, 'ship')}.` : 'Pick 1 to 3 ships. Your first pick opens the match.';
        const peer = net?.peerFleet.length ?? 0;
        texts.opponent = !net?.peerFleetKnown ? 'Opponent: connecting…'
          : net.peerReady ? `Opponent: ready with ${plural(peer, 'ship')}` : `Opponent: choosing · ${peer} of ${SHIP_COUNT} picked`;
        texts.banner = s.fleet.banner;
        break;
      }
      case 'countdown': {
        const left = this.controller.countdownLeft();
        texts.count = left > 0 ? String(left) : 'Go!';
        break;
      }
      case 'battle': texts['battle-status'] = net?.phase === 'finishing' ? 'Checking the result with your opponent…' : ''; break;
      case 'next': {
        const next = s.next;
        if (!next) break;
        const left = this.controller.nextLeft();
        texts.timer = String(left);
        const first = shipName(next.ships[0]);
        if (next.mine) {
          const ship = CHARACTER_REGISTRY[next.chosen ?? next.carousel];
          texts['ship-name'] = ship.name;
          texts['ship-detail'] = `${ship.weapon} · ${ship.special}`;
          texts['ship-state'] = next.chosen !== null ? (next.auto ? `Time is up: ${ship.name} launches` : `${ship.name} launches`)
            : next.carousel === next.ships[0] ? 'Enter launches it · it comes by itself when the time runs out' : 'Enter launches it';
          texts.self = next.chosen !== null ? 'Starting the next battle…'
            : `Choose in ${plural(left, 'second')}, or ${first} comes by itself.`;
        } else {
          texts.self = left > 0 ? `Their choice: ${next.ships.map(shipName).join(' or ')} · ${plural(left, 'second')} left` : 'Waiting for your opponent…';
        }
        break;
      }
      case 'result':
        texts.rematch = net?.localRematch ? 'Rematch requested. Waiting for your opponent…' : net?.peerRematch ? 'Your opponent wants a rematch!' : 'A rematch needs both players.';
        break;
    }
    return texts;
  }

  private announcement(s: OnlineState): string {
    const notice = s.notice && s.notice.kind !== 'link' ? ' ' + s.notice.text : '';
    if (s.confirm) return s.screen === 'fleet' ? 'Leave the room? Stay or Leave.' : 'Leave the match? Stay or Leave.';
    switch (s.screen) {
      case 'lobby': {
        const n = s.lobby.rooms.length;
        const list = s.lobby.status === 'open' ? (n ? ` ${plural(n, 'open room')}.` : ' No open rooms.') : s.lobby.status === 'unsupported' ? ' No room list on this server.' : s.lobby.status === 'error' ? ' The room list is not available.' : '';
        return 'Online Melee lobby.' + list + notice;
      }
      case 'name': return 'Create room. Room name.' + (s.opening ? ' Checking the room list.' : '') + (s.nameError ? ' ' + s.nameError : '');
      case 'waiting': return (s.net?.phase === 'handshake' ? 'Opponent found.' : s.room?.listed ? `Room ${s.room.name} is open. Waiting for an opponent.` : 'Private room open. Send the invite link.') + notice;
      case 'joining': return 'Joining the room.';
      case 'invite': return 'You have an invite to an online fleet battle. Join or Not now.';
      case 'fleet': {
        const net = s.net;
        const head = this.controller.bannerVisible() ? s.fleet.banner + ' ' : '';
        if (net?.localReady) return head + (net.peerReady ? 'Both ready. Starting.' : 'You are ready. Waiting for your opponent.');
        return head + `Choose your fleet. ${CHARACTER_REGISTRY[s.fleet.carousel].name}. ${plural(s.fleet.picks.length, 'ship')} picked.` + notice;
      }
      case 'countdown': return `${s.battle ? 'The next battle starts.' : 'The battle starts.'} You are ${s.start?.localPlayer ? 'Player 2, orange, right' : 'Player 1, cyan, left'}.`;
      case 'next': {
        const next = s.next;
        if (!next) return '';
        if (!next.mine) return this.controller.nextLeft() > 0 ? `Your opponent is choosing the next ship. ${NEXT_MS / 1000} seconds.` + notice : 'Waiting for your opponent.' + notice;
        if (next.chosen !== null) return (next.auto ? 'Time is up. ' : '') + `${shipName(next.chosen)} launches.`;
        return `Choose your next ship in ${NEXT_MS / 1000} seconds. ${shipName(next.carousel)}.` + notice;
      }
      case 'battle': return 'Battle.';
      case 'result': {
        const r = s.result;
        const word = !r || r.winner === 0 ? 'Draw.' : r.winner === r.localPlayer + 1 ? 'Victory.' : 'Defeat.';
        return word + (s.net?.peerRematch && !s.net.localRematch ? ' Your opponent wants a rematch.' : '');
      }
      default: return '';
    }
  }

  // ------------------------------------------------------------ focus
  private applyFocus(s: OnlineState): void {
    // In the battle the keys fly the ship: no button holds the focus there (nor while the opponent chooses the next ship).
    const none = !s.confirm && (s.screen === 'battle' || s.screen === 'countdown' || (s.screen === 'next' && !s.next?.mine));
    const focus = none ? null : s.confirm ? s.confirmFocus : s.focus;
    let target: HTMLElement | null = null;
    for (const node of this.root.querySelectorAll<HTMLElement>('[data-item]')) {
      node.classList.toggle('is-focus', node.dataset.item === focus);
      if (node.dataset.item === focus) target = node;
    }
    if (none) {
      const active = document.activeElement as HTMLElement | null;
      if (active && this.root.contains(active)) active.blur();
      return;
    }
    if (target && document.activeElement !== target) target.focus({ preventScroll: true });
    if (this.selectName && this.input && target === this.input && document.activeElement === this.input) { this.selectName = false; this.input.select(); }
    if (target?.closest('.cc-rooms')) target.scrollIntoView?.({ block: 'nearest' });
  }
}
