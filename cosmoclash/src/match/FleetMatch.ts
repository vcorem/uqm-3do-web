/**
 * One fleet match for the three modes (Play vs Computer, Two Players on one
 * computer and Online Melee): two fleets of 1 to 3 different ships.
 *
 * The first ship of each fleet opens the match. After each battle the ship of
 * the loser is destroyed and the winner's ship stays (both ships reset for the
 * next battle). The loser then has:
 *   - two or more ships left: the loser chooses the next ship (choose()),
 *   - one ship left: it comes by itself ('auto'),
 *   - no ships left: the match is over.
 *
 * Pure and deterministic (no DOM, no clock, no randomness): the online peers
 * put state() in the hashed match state, so a different choice is found.
 */
export const SHIP_COUNT = 3;
export type AfterLoss = 'choose' | 'auto' | 'over';
export type Player = 0 | 1;

/** A fleet for the screens: the battle HUD chips, the countdown and the result. */
export interface FleetView {
  /** The ships in the order they flew: the destroyed ships, the ship in battle, then the ships left (fleet order). */
  ships: number[];
  /** The first `lost` ships are destroyed. */
  lost: number;
  /** The index in `ships` of the ship in battle; -1 while the player chooses and after the last loss. */
  current: number;
}

/** 1 to 3 different ships (0 MichiGichi, 1 Gondi, 2 Sholev). */
export function validFleet(fleet: unknown): fleet is number[] {
  return Array.isArray(fleet) && fleet.length >= 1 && fleet.length <= SHIP_COUNT
    && fleet.every(ship => Number.isInteger(ship) && ship >= 0 && ship < SHIP_COUNT)
    && new Set(fleet).size === fleet.length;
}

export class FleetMatch {
  readonly fleets: [number[], number[]];
  private readonly destroyed: [number[], number[]] = [[], []];
  private readonly active: [number | null, number | null];
  /** Battles that ended (a ship was destroyed). */
  private ended = 0;

  constructor(fleets: [number[], number[]]) {
    if (!validFleet(fleets[0]) || !validFleet(fleets[1])) throw new Error('A fleet has 1 to 3 different ships.');
    this.fleets = [[...fleets[0]], [...fleets[1]]];
    this.active = [this.fleets[0][0], this.fleets[1][0]];
  }

  /** The ship in battle, or null while the player chooses (or has no ships). */
  current(player: Player): number | null { return this.active[player]; }
  /** The destroyed ships, in the order they were destroyed. */
  lost(player: Player): number[] { return [...this.destroyed[player]]; }
  /** The ships that can still fly next: not destroyed and not in battle, in fleet order. */
  left(player: Player): number[] {
    return this.fleets[player].filter(ship => ship !== this.active[player] && !this.destroyed[player].includes(ship));
  }
  /** The number of ships that are not destroyed (the ship in battle too). */
  alive(player: Player): number { return this.fleets[player].length - this.destroyed[player].length; }
  get battles(): number { return this.ended; }
  /** The player who chooses the next ship now, or null. */
  get chooser(): Player | null {
    for (const player of [0, 1] as const) if (this.active[player] === null && this.alive(player) > 0) return player;
    return null;
  }
  get over(): boolean { return this.alive(0) === 0 || this.alive(1) === 0; }
  /** 1 or 2 (the player whose fleet is left) when the match is over, else null. */
  get winner(): 1 | 2 | null { return this.alive(1) === 0 ? 1 : this.alive(0) === 0 ? 2 : null; }

  /** The battle ended: the ship of this player is destroyed. Says what comes next. */
  lose(player: Player): AfterLoss {
    const ship = this.active[player];
    if (ship === null || this.chooser !== null || this.over) throw new Error('No ship of this player is in battle.');
    this.destroyed[player].push(ship);
    this.active[player] = null;
    this.ended++;
    const left = this.left(player);
    if (!left.length) return 'over';
    if (left.length === 1) { this.active[player] = left[0]; return 'auto'; }
    return 'choose';
  }
  /** The chooser's next ship (one of left()). */
  choose(player: Player, ship: number): void {
    if (this.chooser !== player || !this.left(player).includes(ship)) throw new Error('This ship cannot fly next.');
    this.active[player] = ship;
  }
  /** The ship that comes when the time to choose runs out: the first ship left in fleet order. */
  firstLeft(player: Player): number | null { return this.left(player)[0] ?? null; }

  view(player: Player): FleetView {
    const lost = this.destroyed[player];
    const current = this.active[player];
    const ships = [...lost, ...(current === null ? [] : [current]), ...this.left(player)];
    return { ships, lost: lost.length, current: current === null ? -1 : lost.length };
  }
  views(): [FleetView, FleetView] { return [this.view(0), this.view(1)]; }
  /** Plain data of the whole match, for the checksums of the online peers. */
  state(): { fleets: [number[], number[]]; destroyed: [number[], number[]]; active: [number | null, number | null]; battles: number } {
    return { fleets: [[...this.fleets[0]], [...this.fleets[1]]], destroyed: [[...this.destroyed[0]], [...this.destroyed[1]]], active: [...this.active] as [number | null, number | null], battles: this.ended };
  }
}
