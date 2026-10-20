import { afterEach, describe, expect, it, vi } from 'vitest';
import { Panel, PANEL_LOCK_MS } from './Panel';

afterEach(() => { document.body.innerHTML = ''; vi.useRealTimers(); });
describe('panels and the battle message', () => {
  it('Enter and Space choose; Two Players uses Enter only (Space is the fire key of Player 2)', () => {
    const chosen: string[] = [];
    const both = new Panel(document.body, { title: 'Paused', buttons: [{ id: 'resume', label: 'Resume' }, { id: 'menu', label: 'Main menu' }], onChoose: id => chosen.push(id) });
    expect(both.key(' ')).toBe(true);
    expect(both.key('ArrowRight')).toBe(true);
    expect(both.key('Enter')).toBe(true);
    expect(chosen).toEqual(['resume', 'menu']);
    both.destroy();
    const local = new Panel(document.body, { title: 'Paused', buttons: [{ id: 'resume', label: 'Resume' }], chooseKeys: ['Enter'], onChoose: id => chosen.push(id) });
    expect(local.key(' ')).toBe(false);
    expect(chosen).toEqual(['resume', 'menu']);
    local.key('Enter');
    expect(chosen).toEqual(['resume', 'menu', 'resume']);
  });
  it('a panel that opens by itself ignores the choose keys for a moment; Esc and the mouse work at once', () => {
    vi.useFakeTimers({ toFake: ['performance'] });
    const chosen: string[] = [];
    const panel = new Panel(document.body, { title: 'Player 1 wins', buttons: [{ id: 'again', label: 'Play again' }, { id: 'menu', label: 'Main menu' }],
      escape: 'menu', lockMs: PANEL_LOCK_MS, onChoose: id => chosen.push(id) });
    expect(panel.key('Enter')).toBe(true);
    expect(chosen).toEqual([]);
    vi.advanceTimersByTime(PANEL_LOCK_MS);
    panel.key('Enter');
    expect(chosen).toEqual(['again']);
    const other = new Panel(document.body, { title: 'Defeat', buttons: [{ id: 'again', label: 'Play again' }, { id: 'menu', label: 'Main menu' }],
      escape: 'menu', lockMs: PANEL_LOCK_MS, onChoose: id => chosen.push(id) });
    other.key('Escape');
    document.querySelectorAll<HTMLButtonElement>('.cc-dialog [data-item="again"]')[1].click();
    expect(chosen).toEqual(['again', 'menu', 'again']);
  });
});
