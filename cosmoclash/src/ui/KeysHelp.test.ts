import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { KeysHelp, KEYS_STORAGE, SIDE_MIN, SIDE_WIDE, isKeysKey, keysContent, keysLayout, type KeysHelpOptions } from './KeysHelp';
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

type Size = [number, number, number, number?];
const WIDE: Size = [1280, 720, 960];
const NARROW: Size = [1024, 768, 1024];
/** A MacBook Pro 14" in full screen: 101 px beside the game. */
const MACBOOK: Size = [1512, 982, 1309];
/** A phone upright: the game as wide as the screen, a bar above and under it. */
const PHONE: Size = [390, 844, 390, 292];
function memory(initial: Record<string, string> = {}) {
  const data = { ...initial };
  return { data, get: (key: string) => data[key] ?? null, set: (key: string, value: string) => { data[key] = value; } };
}
function help(size: Size, options: Partial<KeysHelpOptions> = {}) {
  let current = size;
  const keys = new KeysHelp({ frame, container, storage: memory(), measure: () => current, ...options });
  return { keys, resize(next: Size) { current = next; keys.layout(); } };
}
const press = (keys: KeysHelp, key: string, init: KeyboardEventInit = {}) => keys.key(new KeyboardEvent('keydown', { key, ...init }));
const titles = (node: Element) => [...node.querySelectorAll('h3')].map(h => h.textContent);

describe('keys beside the game: the margin decision', () => {
  it('wide windows and full screen have a panel in each margin; 4:3 windows and phones have the pop-up', () => {
    expect(keysLayout(1280, 720, 960)).toEqual({ side: true, compact: false, below: false, margin: 160, font: 13.3, gap: 8 });
    expect(keysLayout(1920, 1080, 1440)).toEqual({ side: true, compact: false, below: false, margin: 240, font: 18, gap: 12 });
    expect(keysLayout(1366, 768, 1024)).toMatchObject({ side: true, compact: false });
    expect(keysLayout(1024, 768, 1024)).toMatchObject({ side: false, below: false, margin: 0 });
    expect(keysLayout(960, 720, 960).side).toBe(false);
    // A phone upright: the game is as wide as the screen (bars above and under it): the button goes under the game.
    expect(keysLayout(390, 844, 390, 292)).toMatchObject({ side: false, below: true });
    expect(keysLayout(390, 844, 390)).toMatchObject({ side: false, below: true });
    // A phone on its side, with less than SIDE_MIN beside the game: the button in the corner of the game.
    expect(keysLayout(667, 375, 500)).toMatchObject({ side: false, below: false });
    // Just enough room beside the game, or not.
    expect(keysLayout(960 + 2 * SIDE_MIN, 720, 960).side).toBe(true);
    expect(keysLayout(960 + 2 * SIDE_MIN - 2, 720, 960).side).toBe(false);
    // A window too low for the panels.
    expect(keysLayout(700, 280, 373).side).toBe(false);
  });
  it('16:10 screens and MacBooks in full screen have one-column side panels, never under 12 px', () => {
    // [frame width, frame height, game width] in full screen, and the margin beside the game.
    const screens: [number, number, number, number][] = [
      [1280, 800, 1066, 107], [1440, 900, 1200, 120], [1470, 956, 1274, 98], [1512, 982, 1309, 101],
      [1600, 1000, 1333, 133], [1680, 1050, 1400, 140], [1710, 1112, 1482, 114], [1728, 1117, 1489, 119],
    ];
    for (const [width, height, game, margin] of screens) {
      const layout = keysLayout(width, height, game);
      expect(layout, `${width}x${height}`).toMatchObject({ side: true, compact: true, below: false, margin, gap: 6 });
      expect(layout.font).toBeGreaterThanOrEqual(12);
      expect(layout.font).toBeLessThanOrEqual(16);
    }
    expect(keysLayout(1512, 982, 1309).font).toBe(12.6);
    // The SIDE_MIN edge of the old two-column panels (1680x1050) has 15 px text now.
    expect(keysLayout(1680, 1050, 1400).font).toBe(15);
    // From SIDE_WIDE on: two columns again.
    expect(keysLayout(960 + 2 * SIDE_WIDE, 720, 960)).toMatchObject({ side: true, compact: false });
    expect(keysLayout(960 + 2 * SIDE_WIDE - 2, 720, 960)).toMatchObject({ side: true, compact: true });
    // One column is long: a narrow margin on a low window (a phone on its side) gets the pop-up.
    expect(keysLayout(740, 360, 480)).toMatchObject({ side: false, margin: 130 });
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
    // In the frame (centred over the game), so it can be as high as the frame.
    expect(keys.pop.parentElement).toBe(frame);
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
  it('a phone upright: the button is in the bar under the game, the pop-up has the size unit of the game', () => {
    const { keys, resize } = help(PHONE);
    expect(keys.sideMode).toBe(false);
    expect(keys.button.parentElement).toBe(frame);
    expect(container.contains(keys.button)).toBe(false);
    expect(frame.style.getPropertyValue('--ck-game-w')).toBe('390px');
    expect(frame.style.getPropertyValue('--ck-game-h')).toBe('292px');
    expect(keys.pop.style.getPropertyValue('--u')).toBe(String(292 / 720));
    keys.toggle();
    expect(keys.visible).toBe(true);
    // Turned on its side (no room beside the game): the button goes into the corner of the game; the pop-up stays open.
    container.style.setProperty('--u', '0.52');
    resize([667, 375, 500, 375]);
    expect(keys.button.parentElement).toBe(container);
    expect(keys.visible).toBe(true);
    expect(keys.pop.style.getPropertyValue('--u')).toBe('0.52');
    expect(keys.button.style.getPropertyValue('--u')).toBe('0.52');
  });
  it('a MacBook in full screen: the side panels have one column; wider margins have two', () => {
    const { keys, resize } = help(MACBOOK);
    expect(keys.sideMode).toBe(true);
    expect(keys.compact).toBe(true);
    expect(keys.visible).toBe(true);
    expect(keys.panels.every(panel => panel.parentElement!.classList.contains('ck-compact'))).toBe(true);
    expect(frame.style.getPropertyValue('--ck-margin')).toBe('101px');
    expect(frame.style.getPropertyValue('--ck-font')).toBe('12.6px');
    expect(keys.button.parentElement).toBe(keys.panels[1].parentElement);
    resize(WIDE);
    expect(keys.compact).toBe(false);
    expect(keys.panels.some(panel => panel.parentElement!.classList.contains('ck-compact'))).toBe(false);
    resize(NARROW);
    expect(keys.compact).toBe(false);
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
    expect(press(keys, 'k', { isComposing: true })).toBe(false);
    expect(keys.visible).toBe(true);
  });
  it('Esc closes the pop-up also from the room name field (it does not cancel the name too)', () => {
    const { keys } = help(NARROW);
    const input = document.createElement('input');
    container.appendChild(input);
    const used: boolean[] = [];
    input.addEventListener('keydown', event => { used.push(keys.key(event)); });
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'k', bubbles: true }));
    expect(keys.visible).toBe(false);
    keys.toggle();
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true, isComposing: true }));
    expect(keys.visible).toBe(true);
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
    expect(keys.visible).toBe(false);
    input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
    expect(used).toEqual([false, false, true, false]);
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
    // The labels of the ship choice card: Weapon and Special.
    expect([...keys.panels[0].querySelectorAll('.ck-ships li')].map(li => [...li.children].map(node => node.textContent)))
      .toEqual(CHARACTER_REGISTRY.map(def => [def.name, 'Weapon ' + def.weapon, 'Special ' + def.special]));
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
