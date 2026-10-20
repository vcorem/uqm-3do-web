import { afterEach, describe, expect, it } from 'vitest';
import { FleetPicker, type PickResult } from '@/match/FleetPicker';
import { FleetPickView } from './FleetPickView';
import { attachLiveRegion, announced } from './live';

afterEach(() => { document.body.innerHTML = ''; });
describe('the fleet choice of Two Players', () => {
  it('the slots (the first opens), Done needs a ship, the mouse adds a ship and says Done; the focus is the DOM focus', () => {
    attachLiveRegion(document.body);
    const picker = new FleetPicker(['Enter']);
    const results: PickResult[] = [];
    const view = new FleetPickView(document.body, 1, picker, { changed: () => view.render(), result: result => results.push(result) });
    view.render();
    const root = view.root;
    expect(root.querySelector('h1')?.textContent).toBe('Player 2 · Choose Your Fleet');
    expect(root.textContent).toContain('Player 2 keys: W A D to fly · Space fire · Shift special');
    expect([...root.querySelectorAll('.cc-slot')].map(node => node.textContent)).toEqual(['1Opener', '+—', '+—']);
    expect(root.querySelector('[data-item="done"]')?.getAttribute('aria-disabled')).toBe('true');
    expect(document.activeElement).toBe(root.querySelector('[data-item="ship"]'));
    expect(announced()).toBe('Player 2: choose your fleet. MichiGichi. 0 ships picked.');
    root.querySelector<HTMLElement>('[data-item="done"]')!.click();
    expect(results).toEqual([]);
    expect(root.querySelector('.cc-notice')?.textContent).toBe('Pick at least one ship first.');
    root.querySelector<HTMLElement>('[data-browse="1"]')!.click();
    root.querySelector<HTMLElement>('[data-item="ship"]')!.click();
    expect(picker.picks).toEqual([1]);
    expect([...root.querySelectorAll('.cc-slot')].map(node => node.textContent)).toEqual(['1Gondi', '+Add', '+—']);
    expect(root.querySelector('.cc-ship-state')?.textContent).toBe('Enter adds it to your fleet');
    expect(root.querySelector('[data-item="done"]')?.textContent).toBe('Done · 1 ship');
    root.querySelector<HTMLElement>('[data-item="done"]')!.click();
    expect(results).toEqual(['done']);
    root.querySelector<HTMLElement>('[data-item="back"]')!.click();
    expect(results).toEqual(['done', 'back']);
    view.destroy();
    expect(document.querySelector('[data-screen="local-fleet"]')).toBeNull();
  });
  it('Tab moves the focus: one item shows it, and Enter acts on that item', () => {
    attachLiveRegion(document.body);
    const picker = new FleetPicker(['Enter']);
    const results: PickResult[] = [];
    const view = new FleetPickView(document.body, 0, picker, { changed: () => view.render(), result: result => results.push(result) });
    view.render();
    picker.key('Enter');
    view.render();
    expect(picker.picks).toEqual([0]);
    const root = view.root;
    // The browser's Tab moves the DOM focus from the ship card to Done.
    const done = root.querySelector<HTMLElement>('[data-item="done"]')!;
    done.focus();
    expect(picker.focus).toBe('done');
    expect([...root.querySelectorAll('.is-focus')]).toEqual([done]);
    expect(document.activeElement).toBe(done);
    expect(picker.key('Enter')).toEqual({ handled: true, result: 'done' });
    expect(picker.picks).toEqual([0]);
    // Shift+Tab back to the card: Enter adds the ship there.
    root.querySelector<HTMLElement>('[data-item="ship"]')!.focus();
    expect(picker.focus).toBe('ship');
    expect(root.querySelector('.is-focus')?.getAttribute('data-item')).toBe('ship');
    picker.key('Enter');
    expect(picker.picks).toEqual([0, 1]);
    expect(results).toEqual([]);
  });
});
