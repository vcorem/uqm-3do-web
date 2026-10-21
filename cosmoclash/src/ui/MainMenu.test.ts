import { afterEach, describe, expect, it } from 'vitest';
import { MainMenu, pickerURL, type MenuChoice } from './MainMenu';
import { DifficultyUI } from './DifficultyUI';
import { attachLiveRegion, announced } from './live';

afterEach(() => { document.body.innerHTML = ''; });
describe('main menu', () => {
  it('Enter chooses the focused item (not always Play vs Computer)', () => {
    attachLiveRegion(document.body);
    const chosen: MenuChoice[] = [];
    const menu = new MainMenu(document.body, choice => chosen.push(choice));
    expect(document.activeElement?.textContent).toBe('Play vs Computer');
    expect(menu.key('ArrowDown')).toBe(true);
    expect(menu.key('Enter')).toBe(true);
    expect(chosen).toEqual(['local']);
    menu.key('ArrowDown'); menu.key(' ');
    expect(chosen).toEqual(['local', 'online']);
    menu.key('ArrowDown');
    expect(menu.focused).toBe('ai');
    expect(document.querySelector('.menu-btn.is-focus')?.textContent).toBe('Play vs Computer');
    expect(announced()).toBe('Play vs Computer');
    menu.key('ArrowUp');
    expect(menu.focused).toBe('online');
    expect(menu.key('x')).toBe(false);
  });
  it('a click chooses that item; a focus on start can be given', () => {
    const chosen: MenuChoice[] = [];
    const menu = new MainMenu(document.body, choice => chosen.push(choice), 'online');
    expect(menu.focused).toBe('online');
    [...document.querySelectorAll<HTMLButtonElement>('.menu-btn')][1].click();
    expect(chosen).toEqual(['local']);
    expect(document.querySelectorAll('[role=menuitem]')).toHaveLength(3);
  });
  it('a corner button turns full screen on and off; the menu keeps the keyboard focus', () => {
    let active = false, toggles = 0;
    const menu = new MainMenu(document.body, () => {}, 'online', { active: () => active, toggle: () => { toggles++; active = !active; document.dispatchEvent(new Event('fullscreenchange')); } });
    const corner = document.querySelector<HTMLButtonElement>('.menu-fullscreen')!;
    expect(corner.textContent).toBe('Full screen (F)');
    expect(corner.closest('[role=menu]')).toBe(null);
    expect(document.querySelector('.menu-keys')?.textContent).toBe('↑ ↓ choose · Enter start · F full screen · K keys');
    corner.click();
    expect(toggles).toBe(1);
    expect(corner.textContent).toBe('Exit full screen (F)');
    expect(document.activeElement?.textContent).toBe('Online Melee');
    menu.destroy();
    expect(document.querySelector('.menu-fullscreen')).toBe(null);
  });
  it('difficulty: arrows, Enter, Space and a Back button', () => {
    const levels: number[] = [];
    let back = 0;
    const ui = new DifficultyUI(document.body, level => levels.push(level), () => back++);
    expect(ui.handleKey('ArrowUp')).toBe(true);
    expect(ui.handleKey('Enter')).toBe(true);
    expect(ui.handleKey('ArrowDown')).toBe(true);
    expect(ui.handleKey(' ')).toBe(true);
    expect(ui.handleKey('q')).toBe(false);
    expect(levels).toEqual([1, 2]);
    document.querySelector<HTMLButtonElement>('.difficulty-back')!.click();
    expect(back).toBe(1);
  });
});

describe('Choose game', () => {
  it('opens the picker at the parent of the game path (the item shows only in a /cosmoclash/ build)', () => {
    expect(pickerURL('/cosmoclash/', 'https://urquanmasters.org/cosmoclash/?cosmoRoom=x')).toBe('https://urquanmasters.org/');
    expect(pickerURL('/cosmoclash/', 'http://127.0.0.1:9999/cosmoclash/')).toBe('http://127.0.0.1:9999/');
  });
});

