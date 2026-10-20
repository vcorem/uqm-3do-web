/**
 * The fleet choice of one player on this computer (Two Players · Same
 * Computer), over the 3D ship choice: the same look as the online fleet
 * screen (src/netplay/online.css). FleetPicker holds the state; the page
 * sends the keys to it and calls render(). Every item is a real button for
 * the mouse; the keyboard focus is the DOM focus.
 */
import '@/netplay/online.css';
import './cosmo-ui.css';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { SHIP_COUNT } from '@/match/FleetMatch';
import type { FleetPicker, PickItem, PickResult } from '@/match/FleetPicker';
import { el, button } from './dom';
import { announce } from './live';

const KEYS = ['Player 1 keys: arrows to fly · / fire · . special', 'Player 2 keys: W A D to fly · Space fire · Shift special'];
const plural = (n: number, word: string) => `${n} ${word}${n === 1 ? '' : 's'}`;

export class FleetPickView {
  readonly root: HTMLDivElement;
  private key = '';
  private binds = new Map<string, HTMLElement>();

  constructor(parent: HTMLElement, private readonly player: 0 | 1, private readonly picker: FleetPicker,
    private readonly handlers: { changed(): void; result(result: PickResult): void }) {
    this.root = el('div', { class: 'cc-screen cc-over cc-fleet cc-local-fleet' + (player ? ' cc-p2-side' : ''), data: { screen: 'local-fleet' } });
    parent.appendChild(this.root);
    this.root.addEventListener('click', event => {
      const target = event.target as HTMLElement;
      const browse = target.closest<HTMLElement>('[data-browse]');
      if (browse) { this.picker.browse(Number(browse.dataset.browse) as 1 | -1); this.handlers.changed(); return; }
      const item = target.closest<HTMLElement>('[data-item]')?.dataset.item as PickItem | undefined;
      if (!item) return;
      const result = this.picker.activate(item);
      this.handlers.changed();
      if (result) this.handlers.result(result);
    });
    this.root.addEventListener('mouseover', event => {
      const item = (event.target as HTMLElement).closest<HTMLElement>('[data-item]')?.dataset.item as PickItem | undefined;
      if (item && item !== this.picker.focus) { this.picker.focus = item; this.render(); }
    });
    // Tab moves the DOM focus: the picker's focus follows it, so Enter acts on the item that shows the focus.
    this.root.addEventListener('focusin', event => {
      const item = (event.target as HTMLElement).closest<HTMLElement>('[data-item]')?.dataset.item as PickItem | undefined;
      if (item && item !== this.picker.focus) { this.picker.focus = item; this.handlers.changed(); }
    });
    // The mouse arrows never take the focus from the keyboard.
    this.root.addEventListener('mousedown', event => { if ((event.target as HTMLElement).closest('[data-browse]')) event.preventDefault(); });
  }

  render(): void {
    const p = this.picker;
    const key = [p.picks.join(','), !!p.notice].join('|');
    if (key !== this.key) { this.key = key; this.build(); }
    const ship = CHARACTER_REGISTRY[p.carousel];
    const at = p.picks.indexOf(p.carousel);
    const texts: Record<string, string> = {
      'ship-name': ship.name,
      'ship-detail': `${ship.weapon} · ${ship.special}`,
      'ship-state': at === 0 ? 'Opens the match · Enter takes it out' : at > 0 ? 'In your fleet · Enter takes it out'
        : p.picks.length >= SHIP_COUNT ? 'Your fleet is full' : p.picks.length ? 'Enter adds it to your fleet' : 'Enter adds it · it opens the match',
      self: p.picks.length ? `Pick up to ${SHIP_COUNT} ships, or choose Done with ${plural(p.picks.length, 'ship')}.` : 'Pick 1 to 3 ships. Your first pick opens the match.',
      notice: p.notice,
    };
    for (const [name, value] of Object.entries(texts)) {
      const node = this.binds.get(name);
      if (node && node.textContent !== value) node.textContent = value;
    }
    for (const node of this.root.querySelectorAll<HTMLElement>('[data-item]')) {
      const focused = node.dataset.item === p.focus;
      node.classList.toggle('is-focus', focused);
      if (focused && document.activeElement !== node) node.focus({ preventScroll: true });
    }
    announce(`Player ${this.player + 1}: choose your fleet. ${ship.name}. ${plural(p.picks.length, 'ship')} picked.` + (p.notice ? ' ' + p.notice : ''));
  }

  private build(): void {
    const p = this.picker;
    const picks = p.picks;
    const slots = el('div', { class: 'cc-slots', aria: { label: 'Your fleet: the first ship opens the match' } });
    for (let i = 0; i < SHIP_COUNT; i++) {
      const ship = picks[i];
      slots.append(el('div', { class: 'cc-slot' + (ship !== undefined ? ' filled' : '') + (i === 0 ? ' first' : '') },
        el('span', { class: 'cc-slot-n', text: i === 0 ? '1' : ship !== undefined ? '✓' : '+', aria: { hidden: 'true' } }),
        el('span', { class: 'cc-slot-ship', text: ship !== undefined ? CHARACTER_REGISTRY[ship].name : i === picks.length ? (i === 0 ? 'Opener' : 'Add') : '—' })));
    }
    const colour = this.player ? 'cc-p2' : 'cc-p1';
    this.root.replaceChildren(
      el('div', { class: 'cc-fleet-head' },
        el('h1', { class: 'cc-title cc-title-small' }, el('span', { class: colour, text: `Player ${this.player + 1}` }), ' · Choose Your Fleet'),
        el('p', { class: 'cc-sub', text: 'Two Players · Same Computer · ' + KEYS[this.player] }),
        el('button', { type: 'button', class: 'cc-btn cc-ship', item: 'ship' },
          el('span', { class: 'cc-ship-name', bind: 'ship-name' }),
          el('span', { class: 'cc-ship-detail', bind: 'ship-detail' }),
          el('span', { class: 'cc-ship-state', bind: 'ship-state' }))),
      el('button', { type: 'button', class: 'cc-arrow left', data: { browse: '-1' }, tabIndex: -1, aria: { label: 'Previous ship' }, text: '‹' }),
      el('button', { type: 'button', class: 'cc-arrow right', data: { browse: '1' }, tabIndex: -1, aria: { label: 'Next ship' }, text: '›' }),
      el('div', { class: 'cc-fleet-bottom' },
        el('div', { class: 'cc-tray cc-panel' },
          slots,
          el('p', { class: 'cc-muted cc-rule', text: 'Ship 1 opens the match. After each loss you choose your next ship from the ships left.' }),
          el('p', { class: 'cc-self', bind: 'self' })),
        el('p', { class: 'cc-notice' + (p.notice ? ' error' : ''), bind: 'notice', role: p.notice ? 'alert' : undefined }),
        el('div', { class: 'cc-buttons' },
          button('done', picks.length ? `Done · ${plural(picks.length, 'ship')}` : 'Done', { aria: picks.length ? {} : { disabled: 'true' } }),
          button('undo', 'Undo', { aria: picks.length ? {} : { disabled: 'true' } }),
          button('back', 'Back')),
        el('p', { class: 'cc-hint', text: '← → ships · Enter add or remove · ↓ Done · Backspace undo · Esc menu' })));
    this.binds.clear();
    for (const node of this.root.querySelectorAll<HTMLElement>('[data-bind]')) this.binds.set(node.dataset.bind!, node);
  }

  destroy(): void { this.root.remove(); }
}
