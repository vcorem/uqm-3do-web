import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { SelectionUI } from './SelectionUI';
import { fleetLine } from './fleetChips';

describe('SelectionUI', () => {
  let ui: SelectionUI;

  beforeEach(() => {
    ui = new SelectionUI(document.body);
  });

  afterEach(() => {
    ui.destroy();
  });

  it('creates root element in DOM', () => {
    expect(document.getElementById('selection-ui')).not.toBeNull();
  });

  it('displays player number in header', () => {
    ui.show(2, { id: 'gondi', name: 'Gondi', weapon: 'Galactic Flame', special: 'Space Slime', modelPath: '/models/gondi.glb', modelScale: 1.8, modelRotation: [0, 270, 90], isMichiGichi: false });
    const header = document.querySelector('.selection-header');
    expect(header?.textContent).toContain('Player 2');
  });

  it('displays character info', () => {
    ui.show(1, { id: 'michiGichi', name: 'MichiGichi', weapon: 'Plasma Blast', special: 'Transform between fast and weak / slow and strong', modelPath: '/models/michiGichi.glb', modelScale: 2.0, modelRotation: [-90, 180, 0], isMichiGichi: true });
    const name = document.querySelector('.selection-ship-name');
    const weapon = document.querySelector('.selection-weapon');
    const special = document.querySelector('.selection-special');
    expect(name?.textContent).toContain('MichiGichi');
    expect(weapon?.textContent).toContain('Plasma Blast');
    expect(special?.textContent).toContain('Transform');
  });

  it('hide removes visibility', () => {
    ui.show(1, { id: 'gondi', name: 'Gondi', weapon: 'Galactic Flame', special: 'Space Slime', modelPath: '/models/gondi.glb', modelScale: 1.8, modelRotation: [0, 270, 90], isMichiGichi: false });
    ui.hide();
    const root = document.getElementById('selection-ui');
    expect(root?.style.display).toBe('none');
  });

  it('the next ship: the ships that the player has left', () => {
    ui.showRemaining(2);
    expect(document.querySelector('.selection-remaining')?.textContent).toBe('You have 2 ships left');
    ui.showRemaining(1);
    expect(document.querySelector('.selection-remaining')?.textContent).toBe('You have 1 ship left');
  });

  it('Choose Your Next Ship: the Esc hint and both fleets as chips over the button', () => {
    ui.destroy();
    ui = new SelectionUI(document.body, { prev() {}, next() {}, choose() {} });
    const hint = () => document.querySelector('.selection-controls')?.textContent;
    expect(hint()).toBe('← → Browse  |  Enter to Select  |  Esc Menu');
    ui.setHint('← → Browse  |  Enter to Select  |  Esc Leave');
    expect(hint()).toBe('← → Browse  |  Enter to Select  |  Esc Leave');
    ui.setFleets([fleetLine('Player 1', 0, { ships: [1, 2, 0], lost: 1, current: -1 }), fleetLine('Player 2', 1, { ships: [2, 0], lost: 0, current: 0 })]);
    const bottom = document.querySelector('.selection-bottom')!;
    expect(bottom.firstElementChild?.className).toBe('selection-fleets');
    expect([...bottom.querySelectorAll('.cc-fleet-line')].map(line => line.textContent)).toEqual(['Player 1:GondiSholevMichiGichi', 'Player 2:SholevMichiGichi']);
    expect(bottom.querySelectorAll('.cc-chip.lost')).toHaveLength(1);
    expect(bottom.querySelector('.selection-choose')?.textContent).toBe('Choose this ship');
    ui.setFleets([]);
    expect(document.querySelector('.selection-fleets')).toBeNull();
    ui.setHint('');
    expect(hint()).toBe('← → Browse  |  Enter to Select  |  Esc Menu');
  });

  it('destroy removes element from DOM', () => {
    ui.destroy();
    expect(document.getElementById('selection-ui')).toBeNull();
  });
});
