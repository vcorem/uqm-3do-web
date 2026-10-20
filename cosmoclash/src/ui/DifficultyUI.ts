// src/ui/DifficultyUI.ts
// Difficulty selection overlay — appears after main menu, before character selection.
// Same HTML/CSS overlay pattern as MainMenu and SelectionUI.

import './difficulty.css';
import { announce } from './live';

const LABELS = ['Easy', 'Medium', 'Hard'] as const;

export class DifficultyUI {
  private root: HTMLDivElement;
  private options: HTMLButtonElement[] = [];
  private selectedIndex = 1; // Default to Medium
  private onSelect: (level: 1 | 2 | 3) => void;

  constructor(parent: HTMLElement, onSelect: (level: 1 | 2 | 3) => void, onBack?: () => void) {
    this.onSelect = onSelect;

    this.root = document.createElement('div');
    this.root.id = 'difficulty-screen';
    this.root.setAttribute('role', 'menu');
    this.root.setAttribute('aria-label', 'Select difficulty');

    const title = document.createElement('div');
    title.className = 'difficulty-title';
    title.textContent = 'Select Difficulty';
    this.root.appendChild(title);

    const list = document.createElement('div');
    list.className = 'difficulty-list';

    for (let i = 0; i < LABELS.length; i++) {
      const opt = document.createElement('button');
      opt.type = 'button';
      opt.className = 'difficulty-option';
      opt.setAttribute('role', 'menuitemradio');
      opt.textContent = LABELS[i];
      opt.addEventListener('click', () => {
        this.selectedIndex = i;
        this.updateHighlight(false);
        this.confirm();
      });
      opt.addEventListener('mouseenter', () => { this.selectedIndex = i; this.updateHighlight(false); });
      list.appendChild(opt);
      this.options.push(opt);
    }

    this.root.appendChild(list);

    const hint = document.createElement('div');
    hint.className = 'difficulty-hint';
    hint.textContent = '↑ ↓ choose · Enter start · Esc back';
    this.root.appendChild(hint);

    if (onBack) {
      const back = document.createElement('button');
      back.type = 'button';
      back.className = 'difficulty-back';
      back.textContent = 'Back';
      back.addEventListener('click', onBack);
      this.root.appendChild(back);
    }

    parent.appendChild(this.root);
    this.updateHighlight(true);
    announce('Select difficulty. ' + LABELS[this.selectedIndex] + '.');
  }

  /** One key press; true for a key of this screen. */
  handleKey(key: string): boolean {
    if (key === 'ArrowUp') {
      this.selectedIndex = Math.max(0, this.selectedIndex - 1);
      this.updateHighlight(true);
    } else if (key === 'ArrowDown') {
      this.selectedIndex = Math.min(2, this.selectedIndex + 1);
      this.updateHighlight(true);
    } else if (key === 'Enter' || key === ' ') {
      this.confirm();
    } else {
      return false;
    }
    if (key !== 'Enter' && key !== ' ') announce(LABELS[this.selectedIndex]);
    return true;
  }

  private confirm(): void {
    this.onSelect((this.selectedIndex + 1) as 1 | 2 | 3);
  }

  private updateHighlight(moveDom: boolean): void {
    for (let i = 0; i < this.options.length; i++) {
      const selected = i === this.selectedIndex;
      this.options[i].classList.toggle('selected', selected);
      this.options[i].setAttribute('aria-checked', String(selected));
    }
    if (moveDom) this.options[this.selectedIndex]?.focus({ preventScroll: true });
  }

  destroy(): void {
    this.root.remove();
  }
}
