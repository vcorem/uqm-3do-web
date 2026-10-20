import './match-hud.css';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { el } from './dom';

export interface SideInfo {
  /** "P1", "P2" */
  player: string;
  /** "YOU", "OPPONENT", "COMPUTER", "PLAYER 2" */
  tag: string;
  /** The ships in the order they flew (a ship id below 0 is not known yet): the first `lost` are destroyed. */
  fleet: number[];
  lost: number;
  /** The index of the ship in battle, -1 while the player chooses the next ship. */
  current: number;
}

/**
 * Battle text over the HUD bars: who is who (colour and side), the fleets, a
 * message after each battle, and a small Menu or Leave button for the mouse.
 */
export class MatchHud {
  private root: HTMLDivElement;
  private sides: HTMLDivElement[];
  private centre: HTMLDivElement;
  private messageEl: HTMLDivElement;
  private subEl: HTMLDivElement;
  private infoEl: HTMLDivElement;
  readonly exit: HTMLButtonElement;
  private last = '';

  constructor(parent: HTMLElement, options: { exitLabel: string; onExit(): void; info?: string }) {
    this.root = el('div', { class: 'match-hud' });
    this.sides = [el('div', { class: 'match-side left cc-p1' }), el('div', { class: 'match-side right cc-p2' })];
    this.messageEl = el('div', { class: 'match-message', aria: { hidden: 'true' } });
    this.subEl = el('div', { class: 'match-message-sub', aria: { hidden: 'true' } });
    // One column: a message on two lines (a long ship name) pushes the second line down.
    this.centre = el('div', { class: 'match-centre', hidden: true }, this.messageEl, this.subEl);
    this.infoEl = el('div', { class: 'match-info', text: options.info ?? '', hidden: !options.info });
    this.exit = el('button', { type: 'button', class: 'match-exit', text: options.exitLabel });
    this.exit.addEventListener('click', () => options.onExit());
    this.exit.addEventListener('mousedown', event => event.preventDefault());
    this.root.append(...this.sides, this.centre, this.infoEl, this.exit);
    parent.appendChild(this.root);
  }

  setSides(sides: [SideInfo, SideInfo]): void {
    const key = JSON.stringify(sides);
    if (key === this.last) return;
    this.last = key;
    sides.forEach((side, i) => {
      const node = this.sides[i];
      node.replaceChildren(
        el('div', { class: 'match-tag' }, el('b', { text: side.player }), ' · ' + side.tag),
        el('div', { class: 'match-fleet' }, ...side.fleet.map((ship, n) => el('span', {
          class: 'match-ship' + (n < side.lost ? ' lost' : n === side.current ? ' current' : ''),
          text: CHARACTER_REGISTRY[ship]?.name ?? '?',
          title: n < side.lost ? 'Destroyed' : n === side.current ? 'In battle' : 'Waiting',
        }))));
    });
  }

  /** A large message in the middle (after a battle); '' hides it. */
  message(text: string, sub = ''): void {
    this.messageEl.textContent = text;
    this.subEl.textContent = sub;
    this.messageEl.hidden = !text;
    this.subEl.hidden = !sub;
    this.centre.hidden = !text && !sub;
  }

  setInfo(text: string): void { this.infoEl.textContent = text; this.infoEl.hidden = !text; }

  destroy(): void { this.root.remove(); }
}
