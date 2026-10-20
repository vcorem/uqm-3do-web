/**
 * The fleet choice of one player on this computer (Two Players · Same
 * Computer): the 3D ship choice, Enter adds the ship to the fleet or takes it
 * out, the first ship picked opens the match. No DOM (FleetPickView draws it).
 *
 * Focus items: the ship card, then the buttons Done, Undo and Back.
 * Keys: Left/Right browse the ships (on the card) or move along the buttons,
 * Down goes to the buttons, Up back to the card, Backspace takes the last
 * ship out, Esc goes back. The choose keys are given by the page: Enter only
 * in Two Players, because Space is the fire key of Player 2.
 */
import { SHIP_COUNT } from './FleetMatch';

export type PickItem = 'ship' | 'done' | 'undo' | 'back';
/** What the page does after a key or a click: the fleet is done, or back to the menu. */
export type PickResult = 'done' | 'back' | null;
const BUTTONS: PickItem[] = ['done', 'undo', 'back'];

export class FleetPicker {
  carousel = 0;
  picks: number[] = [];
  focus: PickItem = 'ship';
  /** A short text for the player ('Pick at least one ship first.'), '' for none. */
  notice = '';

  constructor(private readonly chooseKeys: readonly string[] = ['Enter']) {}

  readonly items: readonly PickItem[] = ['ship', ...BUTTONS];

  browse(delta: 1 | -1): void {
    this.carousel = (this.carousel + delta + SHIP_COUNT) % SHIP_COUNT;
    this.focus = 'ship';
    this.notice = '';
  }
  /** Enter on the ship card: add the ship, or take it out when it is in the fleet. */
  toggle(): void {
    const ship = this.carousel;
    this.notice = '';
    if (this.picks.includes(ship)) { this.picks = this.picks.filter(id => id !== ship); return; }
    if (this.picks.length >= SHIP_COUNT) return;
    this.picks = [...this.picks, ship];
    if (this.picks.length >= SHIP_COUNT) { this.focus = 'done'; return; }
    // The next ship that is not in the fleet.
    for (let n = 1; n <= SHIP_COUNT; n++) {
      const next = (ship + n) % SHIP_COUNT;
      if (!this.picks.includes(next)) { this.carousel = next; break; }
    }
  }
  undo(): void {
    if (!this.picks.length) return;
    this.carousel = this.picks[this.picks.length - 1];
    this.picks = this.picks.slice(0, -1);
    this.focus = 'ship';
    this.notice = '';
  }
  activate(item: PickItem): PickResult {
    this.focus = item;
    switch (item) {
      case 'ship': this.toggle(); return null;
      case 'undo': this.undo(); return null;
      case 'back': return 'back';
      case 'done':
        if (!this.picks.length) { this.notice = 'Pick at least one ship first.'; this.focus = 'ship'; return null; }
        return 'done';
    }
  }
  /** One key press. handled: the key was a menu key here. */
  key(name: string): { handled: boolean; result: PickResult } {
    const onShip = this.focus === 'ship';
    const none = { handled: true, result: null };
    if (this.chooseKeys.includes(name)) return { handled: true, result: this.activate(this.focus) };
    switch (name) {
      case 'ArrowLeft': case 'ArrowRight': {
        const delta = name === 'ArrowLeft' ? -1 : 1;
        if (onShip) { this.browse(delta); return none; }
        const next = BUTTONS[BUTTONS.indexOf(this.focus) + delta];
        if (next) this.focus = next;
        return none;
      }
      case 'ArrowDown': if (onShip) this.focus = 'done'; return none;
      case 'ArrowUp': this.focus = 'ship'; return none;
      case 'Backspace': case 'Delete': this.undo(); return none;
      case 'Escape': return { handled: true, result: 'back' };
      default: return { handled: false, result: null };
    }
  }
}
