// @vitest-environment node
import { describe, expect, it } from 'vitest';
import { FleetMatch, battleSeed, validFleet } from './FleetMatch';
import { FleetPicker } from './FleetPicker';

describe('fleet match (all three modes)', () => {
  it('the first ship of each fleet opens; a loser with two ships left chooses, the last ship comes by itself, then the match is over', () => {
    const m = new FleetMatch([[1, 2, 0], [0]]);
    expect([m.current(0), m.current(1)]).toEqual([1, 0]);
    expect(m.views()).toEqual([{ ships: [1, 2, 0], lost: 0, current: 0 }, { ships: [0], lost: 0, current: 0 }]);
    expect(m.lose(0)).toBe('choose');
    expect(m.chooser).toBe(0);
    expect(m.current(0)).toBe(null);
    expect(m.left(0)).toEqual([2, 0]);
    expect(m.firstLeft(0)).toBe(2);
    // While the player chooses: the destroyed ship, then the ships left; no ship in battle.
    expect(m.view(0)).toEqual({ ships: [1, 2, 0], lost: 1, current: -1 });
    // The order of the others does not matter: the player takes the last ship of the fleet.
    m.choose(0, 0);
    expect(m.chooser).toBe(null);
    expect(m.view(0)).toEqual({ ships: [1, 0, 2], lost: 1, current: 1 });
    expect(m.lose(0)).toBe('auto');
    expect(m.current(0)).toBe(2);
    expect(m.view(0)).toEqual({ ships: [1, 0, 2], lost: 2, current: 2 });
    expect(m.lose(1)).toBe('over');
    expect(m).toMatchObject({ over: true, winner: 1, battles: 3, chooser: null });
    expect(m.view(1)).toEqual({ ships: [0], lost: 1, current: -1 });
    expect(m.lost(0)).toEqual([1, 0]);
  });
  it('refuses a ship that cannot fly next, a second loss while a player chooses, and a loss after the end', () => {
    const m = new FleetMatch([[0, 1, 2], [2, 1]]);
    expect(() => m.choose(0, 1)).toThrow();
    m.lose(0);
    expect(() => m.choose(0, 0)).toThrow();
    expect(() => m.choose(1, 1)).toThrow();
    expect(() => m.lose(1)).toThrow();
    m.choose(0, 2);
    expect(m.lose(1)).toBe('auto');
    expect(m.lose(1)).toBe('over');
    expect(() => m.lose(0)).toThrow();
    expect(m.winner).toBe(1);
  });
  it('one ship each: one battle; state() is plain data that changes with each choice', () => {
    const one = new FleetMatch([[2], [1]]);
    expect(one.lose(1)).toBe('over');
    expect(one.winner).toBe(1);
    const a = new FleetMatch([[0, 1, 2], [0, 1, 2]]), b = new FleetMatch([[0, 1, 2], [0, 1, 2]]);
    a.lose(1); b.lose(1);
    expect(JSON.stringify(a.state())).toBe(JSON.stringify(b.state()));
    a.choose(1, 1); b.choose(1, 2);
    expect(JSON.stringify(a.state())).not.toBe(JSON.stringify(b.state()));
    expect(a.state()).toEqual({ fleets: [[0, 1, 2], [0, 1, 2]], destroyed: [[], [0]], active: [0, 1], battles: 1 });
  });
  it('determinism: the same losses and choices give the same states on two peers; each battle has its own seed', () => {
    const run = (choices: number[]) => {
      const m = new FleetMatch([[0, 1, 2], [2, 0, 1]]);
      const states: string[] = [];
      for (const [loser, choice] of [[1, choices[0]], [0, choices[1]], [1, -1], [0, -1]] as const) {
        const next = m.lose(loser);
        if (next === 'choose') m.choose(loser, choice);
        states.push(JSON.stringify(m.state()));
      }
      return states;
    };
    expect(run([1, 2])).toEqual(run([1, 2]));
    expect(run([1, 2])[0]).not.toBe(run([0, 2])[0]);
    expect(battleSeed(123456789, 0)).toBe(123456789);
    expect(battleSeed(0xffffffff, 0)).toBe(0xffffffff);
    const seeds = [0, 1, 2, 3, 4].map(n => battleSeed(123456789, n));
    expect(new Set(seeds).size).toBe(5);
    expect(seeds.every(seed => Number.isInteger(seed) && seed >= 0 && seed <= 0xffffffff)).toBe(true);
    expect(battleSeed(123456789, 3)).toBe((123456789 + Math.imul(3, 0x9e3779b9)) >>> 0);
  });
  it('fleets have 1 to 3 different ships', () => {
    expect([[0], [2, 1], [0, 1, 2]].every(validFleet)).toBe(true);
    for (const bad of [[], [0, 0], [0, 1, 2, 0], [3], [-1], [1.5], 'x', null]) expect(validFleet(bad)).toBe(false);
    expect(() => new FleetMatch([[0], []])).toThrow();
  });
});

describe('local fleet choice', () => {
  it('Enter adds ships (the first opens), takes a ship out, Backspace undoes, Done needs a ship; Space does not choose', () => {
    const p = new FleetPicker(['Enter']);
    expect(p.key(' ')).toEqual({ handled: false, result: null });
    expect(p.picks).toEqual([]);
    expect(p.key('ArrowDown').handled).toBe(true);
    expect(p.focus).toBe('done');
    expect(p.key('Enter').result).toBe(null);
    expect(p.notice).toBe('Pick at least one ship first.');
    expect(p.focus).toBe('ship');
    p.key('ArrowRight');
    expect(p.carousel).toBe(1);
    p.key('Enter');
    expect(p.picks).toEqual([1]);
    expect(p.carousel).toBe(2);
    p.key('Enter');
    p.key('Enter');
    expect(p.picks).toEqual([1, 2, 0]);
    expect(p.focus).toBe('done');
    p.key('ArrowUp');
    p.key('Enter');
    expect(p.picks).toEqual([1, 2]);
    p.key('Backspace');
    expect(p).toMatchObject({ picks: [1], carousel: 2, focus: 'ship' });
    p.key('ArrowDown'); p.key('ArrowRight');
    expect(p.focus).toBe('undo');
    p.key('ArrowRight'); p.key('ArrowRight');
    expect(p.focus).toBe('back');
    p.key('ArrowLeft'); p.key('ArrowLeft');
    expect(p.key('Enter').result).toBe('done');
    expect(p.key('Escape').result).toBe('back');
  });
});
