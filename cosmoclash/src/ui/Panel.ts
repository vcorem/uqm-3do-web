import './cosmo-ui.css';
import { el, button } from './dom';
import { announce } from './live';

export interface PanelOptions {
  title: string;
  lines?: string[];
  /** More content under the lines (the fleets of a game over). */
  content?: Node[];
  buttons: { id: string; label: string }[];
  focus?: string;
  /** The button that Esc chooses. */
  escape?: string;
  /** A dialog over the game (dimmed) instead of a full screen. */
  dialog?: boolean;
  titleClass?: string;
  hint?: string;
  /**
   * The keys that choose the focused button (default Enter and Space). Two
   * Players on one computer: Enter only, because Space is the fire key of Player 2.
   */
  chooseKeys?: string[];
  /**
   * The choose keys do nothing this long after the panel opens: for a panel
   * that opens by itself (game over), where a key may still be pressed for the battle.
   */
  lockMs?: number;
  onChoose(id: string): void;
}
export const PANEL_LOCK_MS = 600;

/** A small menu screen in the game's look: pause, game over. Arrows, Enter, Esc and the mouse. */
export class Panel {
  private root: HTMLDivElement;
  private buttons: HTMLButtonElement[] = [];
  private focus: string;
  private readonly openedAt = performance.now();

  constructor(parent: HTMLElement, private readonly options: PanelOptions) {
    this.focus = options.focus ?? options.buttons[0].id;
    const body = el('div', { class: 'cc-dialog cc-panel', role: options.dialog ? 'alertdialog' : 'dialog', aria: { label: options.title } },
      el('h2', { class: options.titleClass ?? '', text: options.title }),
      ...(options.lines ?? []).map(line => el('p', { text: line })),
      ...(options.content ?? []),
      el('div', { class: 'cc-buttons' }, ...options.buttons.map(item => {
        const node = button(item.id, item.label);
        node.addEventListener('click', () => this.options.onChoose(item.id));
        node.addEventListener('mouseenter', () => this.setFocus(item.id, false));
        node.addEventListener('focus', () => this.setFocus(item.id, false));
        this.buttons.push(node);
        return node;
      })),
      options.hint ? el('p', { class: 'cc-hint', text: options.hint }) : null);
    this.root = el('div', { class: 'cc-dialog-veil' + (options.dialog ? '' : ' cc-solid') }, body);
    parent.appendChild(this.root);
    this.setFocus(this.focus, true);
    announce([options.title, ...(options.lines ?? [])].join('. ') + '.');
  }
  key(name: string): boolean {
    const ids = this.options.buttons.map(item => item.id);
    const index = ids.indexOf(this.focus);
    if (['ArrowLeft', 'ArrowUp', 'ArrowRight', 'ArrowDown'].includes(name)) {
      const delta = name === 'ArrowLeft' || name === 'ArrowUp' ? -1 : 1;
      this.setFocus(ids[(index + delta + ids.length) % ids.length], true);
      return true;
    }
    if ((this.options.chooseKeys ?? ['Enter', ' ']).includes(name)) {
      if (performance.now() - this.openedAt >= (this.options.lockMs ?? 0)) this.options.onChoose(this.focus);
      return true;
    }
    if (name === 'Escape' && this.options.escape) { this.options.onChoose(this.options.escape); return true; }
    return name === 'Escape';
  }
  private setFocus(id: string, moveDom: boolean): void {
    this.focus = id;
    for (const node of this.buttons) node.classList.toggle('is-focus', node.dataset.item === id);
    if (moveDom) this.buttons.find(node => node.dataset.item === id)?.focus({ preventScroll: true });
  }
  destroy(): void { this.root.remove(); }
}
