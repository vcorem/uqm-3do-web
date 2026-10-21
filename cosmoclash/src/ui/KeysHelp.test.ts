import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { KeysHelp, KEYS_STORAGE, SIDE_MIN, isKeysKey, keysContent, keysLayout, type KeysHelpOptions } from './KeysHelp';
import { attachLiveRegion, announced } from './live';

let frame: HTMLDivElement, container: HTMLDivElement;
beforeEach(() => {
  frame = document.createElement('div');
  frame.id = 'cosmo-frame';
  container = document.createElement('div');
  container.id = 'game-container';
  frame.appendChild(container);
  document.body.appendChild(frame);
  attachLiveRegion(container);
});
afterEach(() => { document.body.innerHTML = ''; vi.restoreAllMocks(); });

const WIDE: [number, number, number] = [1280, 720, 960];
const NARROW: [number, number, number] = [1024, 768, 1024];
function memory(initial: Record<string, string> = {}) {
  const data = { ...initial };
  return { data, get: (key: string) => data[key] ?? null, set: (key: string, value: string) => { data[key] = value; } };
}
function help(size: [number, number, number], options: Partial<KeysHelpOptions> = {}) {
  let current = size;
  const keys = new KeysHelp({ frame, container, storage: memory(), measure: () => current, ...options });
  return { keys, resize(next: [number, number, number]) { current = next; keys.layout(); } };
}
const press = (keys: KeysHelp, key: string, init: KeyboardEventInit = {}) => keys.key(new KeyboardEvent('keydown', { key, ...init }));
const titles = (node: Element) => [...node.querySelectorAll('h3')].map(h => h.textContent);

describe('keys beside the game: the margin decision', () => {
  it('wide windows and full screen have a panel in each margin; 4:3 windows and phones have the pop-up', () => {
    expect(keysLayout(1280, 720, 960)).toEqual({ side: true, margin: 160, font: 13.3, gap: 8 });
    expect(keysLayout(1920, 1080, 1440)).toEqual({ side: true, margin: 240, font: 18, gap: 12 });
    expect(keysLayout(1366, 768, 1024).side).toBe(true);
    expect(keysLayout(1024, 768, 1024)).toMatchObject({ side: false, margin: 0 });
    expect(keysLayout(960, 720, 960).side).toBe(false);
    // A phone upright: the game is as wide as the screen (bars above and under it).
    expect(keysLayout(390, 844, 390).side).toBe(false);
    // Just enough room beside the game, or not.
    expect(keysLayout(960 + 2 * SIDE_MIN, 720, 960).side).toBe(true);
    expect(keysLayout(960 + 2 * SIDE_MIN - 2, 720, 960).side).toBe(false);
    // A window too low for the panels.
    expect(keysLayout(700, 280, 373).side).toBe(false);
  });
});

describe('keys content per mode', () => {
  it('Two Players shows both players and says that only Enter selects (Space is the fire key of Player 2)', () => {
    const content = keysContent('local');
    expect(content.title).toBe('Two Players');
    expect(content.flying.map(s => s.title)).toEqual(['Player 1', 'Player 2']);
    expect(content.flying[0].rows).toEqual([{ keys: ['↑'], label: 'thrust' }, { keys: ['←', '→'], label: 'turn' }, { keys: ['/'], label: 'fire' }, { keys: ['.'], label: 'special' }]);
    expect(content.flying[1].rows).toEqual([{ keys: ['W'], label: 'thrust' }, { keys: ['A', 'D'], label: 'turn' }, { keys: ['Space'], label: 'fire' }, { keys: ['Shift'], label: 'special' }]);
    const menus = content.menus[0];
    expect(menus.rows.find(row => row.label === 'select')?.keys).toEqual(['Enter']);
    expect(menus.note).toMatch(/only Enter selects/);
    expect(menus.rows.some(row => row.keys.includes('Backspace'))).toBe(true);
    expect(content.menus[1].rows.find(row => row.keys[0] === 'Esc')?.label).toBe('pause');
  });
  it('vs Computer and Online show the Player 1 keys only, and say so', () => {
    const ai = keysContent('ai');
    expect(ai.flying.map(s => s.title)).toEqual(['Your ship']);
    expect(ai.intro).toMatch(/Player 1 keys/);
    expect(ai.menus[0].rows.find(row => row.label === 'select')?.keys).toEqual(['Enter', 'Space']);
    // No fleet choice against the computer: nothing to undo.
    expect(ai.menus[0].rows.some(row => row.keys.includes('Backspace'))).toBe(false);
    const online = keysContent('online');
    expect(online.flying.map(s => s.title)).toEqual(['Your ship']);
    expect(online.intro).toBe('Both players fly with the Player 1 keys, each on their own computer.');
    expect(online.menus[1].rows[0]).toEqual({ keys: ['Esc'], label: 'leave the match' });
    expect(online.menus[1].note).toMatch(/does not pause/);
  });
  it('the main menu shows both players; every mode has the game keys and the ships of the game data', () => {
    expect(keysContent('menu').flying.map(s => s.title)).toEqual(['Player 1', 'Player 2']);
    for (const mode of ['menu', 'ai', 'local', 'online'] as const) {
      const content = keysContent(mode);
      expect(content.menus[1].rows.map(row => row.keys[0])).toEqual(['Esc', 'M', 'F', 'K']);
      expect(content.ships).toEqual(CHARACTER_REGISTRY.map(def => ({ name: def.name, weapon: def.weapon, special: def.special })));
    }
    expect(keysContent('ai').ships.map(ship => ship.name)).toEqual(['MichiGichi', 'Gondi', 'Sholev']);
  });
});

describe('the keys panels and the pop-up', () => {
  it('beside the game: shown at first, K hides and shows them and the choice is remembered', () => {
    const storage = memory();
    const { keys } = help(WIDE, { storage });
    const [left, right] = keys.panels;
    expect(keys.sideMode).toBe(true);
    expect(keys.visible).toBe(true);
    expect(left.tagName).toBe('ASIDE');
    expect(left.getAttribute('aria-label')).toBe('Keys: flying and ships');
    expect(right.getAttribute('aria-label')).toBe('Keys: menus and game');
    expect(left.hidden || right.hidden).toBe(false);
    // The panels are in the frame, not in the game; the button is on top of the right margin.
    expect(container.contains(left) || container.contains(right) || container.contains(keys.button)).toBe(false);
    expect(keys.button.parentElement).toBe(right.parentElement);
    expect(keys.button.textContent).toBe('Keys (K)');
    expect(keys.button.getAttribute('aria-expanded')).toBe('true');
    expect(keys.button.getAttribute('aria-controls')).toBe('cosmo-keys-flying cosmo-keys-menus');
    expect(frame.style.getPropertyValue('--ck-margin')).toBe('160px');

    expect(press(keys, 'k')).toBe(true);
    expect(keys.visible).toBe(false);
    expect(left.hidden && right.hidden).toBe(true);
    expect(keys.button.getAttribute('aria-expanded')).toBe('false');
    expect(storage.data[KEYS_STORAGE]).toBe('hidden');
    expect(announced()).toBe('Keys hidden. Press K to show them.');
    // A held K repeats: no second toggle, and nobody else gets the key.
    expect(press(keys, 'k', { repeat: true })).toBe(true);
    expect(keys.visible).toBe(false);
    keys.button.click();
    expect(keys.visible).toBe(true);
    expect(storage.data[KEYS_STORAGE]).toBe('shown');
    expect(announced()).toBe('Keys shown beside the game.');
    // Esc is a game key while the side panels show.
    expect(press(keys, 'Escape')).toBe(false);
  });
  it('a hidden choice stays hidden on the next visit; a blocked localStorage shows the panels and does not throw', () => {
    const { keys } = help(WIDE, { storage: memory({ [KEYS_STORAGE]: 'hidden' }) });
    expect(keys.visible).toBe(false);
    expect(keys.button.getAttribute('aria-expanded')).toBe('false');
    keys.destroy();
    vi.spyOn(Storage.prototype, 'getItem').mockImplementation(() => { throw new Error('denied'); });
    vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => { throw new Error('denied'); });
    const blocked = new KeysHelp({ frame, container, measure: () => WIDE });
    expect(blocked.visible).toBe(true);
    expect(() => blocked.toggle()).not.toThrow();
    expect(blocked.visible).toBe(false);
  });
  it('without room: a dialog over the game that never opens by itself; K, Esc and Close close it', () => {
    const { keys, resize } = help(NARROW);
    expect(keys.sideMode).toBe(false);
    expect(keys.visible).toBe(false);
    expect(keys.pop.hidden).toBe(true);
    expect(keys.pop.getAttribute('role')).toBe('dialog');
    expect(keys.pop.getAttribute('aria-label')).toBe('Keys');
    expect(container.contains(keys.pop)).toBe(true);
    // The button is in the corner of the game, the margins are hidden.
    expect(keys.button.parentElement).toBe(container);
    expect(keys.panels.every(panel => panel.parentElement!.hidden)).toBe(true);
    expect(keys.button.getAttribute('aria-controls')).toBe('cosmo-keys-pop');

    expect(press(keys, 'K', { shiftKey: true })).toBe(true);
    expect(keys.pop.hidden).toBe(false);
    expect(keys.button.getAttribute('aria-expanded')).toBe('true');
    expect(announced()).toBe('Keys shown over the game. Press K or Esc to close them.');
    // Esc closes the pop-up only: the page does not also pause or leave.
    expect(press(keys, 'Escape')).toBe(true);
    expect(keys.pop.hidden).toBe(true);
    expect(announced()).toBe('Keys closed.');
    expect(press(keys, 'Escape')).toBe(false);
    press(keys, 'k');
    keys.pop.querySelector<HTMLButtonElement>('.ck-close')!.click();
    expect(keys.visible).toBe(false);
    press(keys, 'k');
    press(keys, 'k');
    expect(keys.visible).toBe(false);

    // The pop-up does not open again by itself after a size change, and the side panels come back.
    press(keys, 'k');
    resize(WIDE);
    expect(keys.pop.hidden).toBe(true);
    expect(keys.sideMode).toBe(true);
    expect(keys.visible).toBe(true);
    expect(keys.button.parentElement).toBe(keys.panels[1].parentElement);
    resize(NARROW);
    expect(keys.visible).toBe(false);
    expect(keys.button.parentElement).toBe(container);
  });
  it('a K typed in a text field is a letter; Ctrl, Cmd and Alt with K are browser keys', () => {
    const { keys } = help(WIDE);
    const input = document.createElement('input');
    container.appendChild(input);
    let used: boolean | null = null;
    input.addEventListener('keydown', event => { used = keys.key(event); });
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'k', bubbles: true, composed: true }));
    expect(used).toBe(false);
    expect(keys.visible).toBe(true);
    for (const modifier of ['ctrlKey', 'metaKey', 'altKey']) expect(press(keys, 'k', { [modifier]: true })).toBe(false);
    expect(keys.visible).toBe(true);
  });
  it('K is the letter K, or the K key of a non-Latin layout (Hebrew ל)', () => {
    expect(isKeysKey({ key: 'k', code: 'KeyK' })).toBe(true);
    expect(isKeysKey({ key: 'K', code: 'KeyK' })).toBe(true);
    expect(isKeysKey({ key: 'ל', code: 'KeyK' })).toBe(true);
    // Dvorak: the key at the K position types t.
    expect(isKeysKey({ key: 't', code: 'KeyK' })).toBe(false);
    expect(isKeysKey({ key: '?', code: 'Slash' })).toBe(false);
    expect(isKeysKey({ key: 'f', code: 'KeyF' })).toBe(false);
  });
  it('the content follows the mode in the panels and in the pop-up', () => {
    const { keys } = help(WIDE);
    expect(titles(keys.panels[0])).toEqual(['Player 1', 'Player 2', 'Ships']);
    expect(titles(keys.panels[1])).toEqual(['Menus', 'Game']);
    expect(keys.panels[0].querySelectorAll('.ck-ships li')).toHaveLength(3);
    keys.setMode('ai');
    expect(keys.currentMode).toBe('ai');
    expect(titles(keys.panels[0])).toEqual(['Your ship', 'Ships']);
    expect(keys.panels[0].querySelector('.ck-mode')?.textContent).toBe('vs Computer You fly with the Player 1 keys.');
    keys.setMode('online');
    expect(keys.pop.querySelector('h2')?.textContent).toBe('Keys · Online Melee');
    expect(titles(keys.pop)).toEqual(['Your ship', 'Menus', 'Game', 'Ships']);
    expect(keys.pop.textContent).toContain('Ramming Energy');
    keys.destroy();
    expect(frame.querySelector('aside, .ck-pop, .ck-button')).toBe(null);
  });
});
