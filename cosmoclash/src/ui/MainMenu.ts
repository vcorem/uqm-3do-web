import './main-menu.css';
import { assetURL } from '@/base';
import { el, button } from './dom';
import { announce } from './live';

export type MenuChoice = 'ai' | 'local' | 'online';
const ITEMS: { id: MenuChoice; label: string }[] = [
  { id: 'ai', label: 'Play vs Computer' },
  { id: 'local', label: 'Two Players · Same Computer' },
  { id: 'online', label: 'Online Melee' },
];

/** Full screen on and off: the F key, and this corner button for the mouse. */
export interface FullscreenToggle { active(): boolean; toggle(): void }

/** The main menu over the title art. Arrows choose, Enter or a click starts the focused item. */
export class MainMenu {
  private root: HTMLDivElement;
  private focus: MenuChoice;
  private buttons = new Map<MenuChoice, HTMLButtonElement>();
  private corner: HTMLButtonElement | null = null;
  private onFullscreen = () => this.showFullscreen();

  constructor(parent: HTMLElement, private readonly onChoose: (choice: MenuChoice) => void, focus: MenuChoice = 'ai', private readonly fullscreen?: FullscreenToggle) {
    this.focus = focus;
    this.root = el('div', { id: 'main-menu', role: 'menu', aria: { label: 'Cosmo Clash main menu' } });
    this.root.style.backgroundImage = `url(${assetURL('textures/main-bk.jpg')})`;
    const list = el('div', { class: 'menu-list' });
    for (const item of ITEMS) {
      const node = button(item.id, item.label, { class: 'menu-btn', role: 'menuitem' });
      node.addEventListener('click', () => this.choose(item.id));
      node.addEventListener('mouseenter', () => this.setFocus(item.id, false));
      node.addEventListener('focus', () => this.setFocus(item.id, false));
      this.buttons.set(item.id, node);
      list.appendChild(node);
    }
    this.root.append(list,
      el('p', { class: 'menu-controls', text: 'Fly: ↑ thrust · ← → turn · / fire · . special  ·  M mute' }),
      el('p', { class: 'menu-keys', text: fullscreen ? '↑ ↓ choose · Enter start · F full screen' : '↑ ↓ choose · Enter start' }),
      el('div', { class: 'menu-credits', text: 'Made by Ori Hanegby, Guy Corem, Orr Kowarsky' }));
    parent.appendChild(this.root);
    if (fullscreen) {
      // Outside the menu list: the arrows do not stop on it, and the pointer does not take the keyboard focus.
      this.corner = el('button', { type: 'button', class: 'menu-fullscreen' });
      this.corner.addEventListener('mousedown', event => event.preventDefault());
      this.corner.addEventListener('click', () => { fullscreen.toggle(); this.buttons.get(this.focus)?.focus({ preventScroll: true }); });
      parent.appendChild(this.corner);
      document.addEventListener('fullscreenchange', this.onFullscreen);
      this.showFullscreen();
    }
    this.setFocus(focus, true);
    announce('Cosmo Clash main menu. ' + ITEMS.find(item => item.id === focus)!.label + '.');
  }

  /** One key press. Returns true for a menu key. */
  key(name: string): boolean {
    const index = ITEMS.findIndex(item => item.id === this.focus);
    if (name === 'ArrowUp' || name === 'ArrowDown') {
      const next = ITEMS[(index + (name === 'ArrowUp' ? -1 : 1) + ITEMS.length) % ITEMS.length].id;
      this.setFocus(next, true);
      announce(ITEMS.find(item => item.id === next)!.label);
      return true;
    }
    if (name === 'Home' || name === 'End') { this.setFocus(ITEMS[name === 'Home' ? 0 : ITEMS.length - 1].id, true); return true; }
    if (name === 'Enter' || name === ' ') { this.choose(this.focus); return true; }
    return false;
  }

  get focused(): MenuChoice { return this.focus; }

  private setFocus(id: MenuChoice, moveDom: boolean): void {
    this.focus = id;
    for (const [key, node] of this.buttons) node.classList.toggle('is-focus', key === id);
    if (moveDom) this.buttons.get(id)?.focus({ preventScroll: true });
  }

  private choose(id: MenuChoice): void {
    this.setFocus(id, false);
    this.onChoose(id);
  }

  private showFullscreen(): void {
    if (this.corner) this.corner.textContent = this.fullscreen?.active() ? 'Exit full screen (F)' : 'Full screen (F)';
  }

  destroy(): void {
    this.root.remove();
    this.corner?.remove();
    document.removeEventListener('fullscreenchange', this.onFullscreen);
  }
}
