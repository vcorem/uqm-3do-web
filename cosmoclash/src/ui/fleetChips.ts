import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import type { FleetView } from '@/match/FleetMatch';
import { el } from './dom';

const EMPTY: FleetView = { ships: [], lost: 0, current: -1 };
/**
 * A fleet as chips (countdown, the choice of the next ship, the results): the
 * destroyed ships struck through, the ship in battle bright, then the ships
 * left. A ship id below 0 is a ship that is not known yet ('?').
 */
export function fleetChips(side: FleetView | undefined): HTMLElement[] {
  const view = side ?? EMPTY;
  return view.ships.map((ship, i) => el('span', {
    class: 'cc-chip' + (i < view.lost ? ' lost' : i === view.current ? ' current' : ''),
    text: CHARACTER_REGISTRY[ship]?.name ?? '?', title: i < view.lost ? 'Destroyed' : i === view.current ? 'In battle' : 'Ready to fly',
  }));
}
/** "Title: chips" in the colour of the player (0 cyan, 1 orange). */
export function fleetLine(title: string, player: 0 | 1, side: FleetView | undefined, className = 'cc-fleet-line'): HTMLElement {
  return el('p', { class: className + (player ? ' cc-line-p2' : '') },
    el('b', { class: player ? 'cc-p2' : 'cc-p1', text: title + ':' }), ...fleetChips(side));
}
