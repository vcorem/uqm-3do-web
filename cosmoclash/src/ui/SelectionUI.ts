import { CharacterDef } from '@/game/CharacterRegistry';
import './selection.css';
import { announce } from './live';

export interface SelectionControls { prev(): void; next(): void; choose(): void }
const DEFAULT_HINT = '← → Browse  |  Enter to Select  |  Esc Menu';

export class SelectionUI {
  private root: HTMLDivElement;
  private parent: HTMLElement;
  private header: HTMLDivElement;
  private subEl: HTMLDivElement;
  private nameEl: HTMLDivElement;
  private weaponLabel: HTMLSpanElement;
  private weaponValue: Text;
  private specialLabel: HTMLSpanElement;
  private specialValue: Text;
  private remainingEl: HTMLDivElement;
  private arrows: HTMLButtonElement[] = [];
  private chooseButton: HTMLButtonElement | null = null;
  private bottom: HTMLDivElement | null = null;
  private hintEl: HTMLDivElement;
  private fleetsEl: HTMLDivElement | null = null;
  private headerText = '';

  constructor(parent: HTMLElement, controls?: SelectionControls) {
    this.root = document.createElement('div');
    this.root.id = 'selection-ui';
    this.root.style.display = 'none';

    this.header = document.createElement('div');
    this.header.className = 'selection-header';

    this.subEl = document.createElement('div');
    this.subEl.className = 'selection-sub';
    this.subEl.style.display = 'none';

    const panel = document.createElement('div');
    panel.className = 'selection-info-panel';

    this.nameEl = document.createElement('div');
    this.nameEl.className = 'selection-ship-name';

    const weaponEl = document.createElement('div');
    weaponEl.className = 'selection-row selection-weapon';
    this.weaponLabel = document.createElement('span');
    this.weaponLabel.className = 'label';
    this.weaponLabel.textContent = 'Weapon: ';
    this.weaponValue = document.createTextNode('');
    weaponEl.appendChild(this.weaponLabel);
    weaponEl.appendChild(this.weaponValue);

    const specialEl = document.createElement('div');
    specialEl.className = 'selection-row selection-special';
    this.specialLabel = document.createElement('span');
    this.specialLabel.className = 'label';
    this.specialLabel.textContent = 'Special: ';
    this.specialValue = document.createTextNode('');
    specialEl.appendChild(this.specialLabel);
    specialEl.appendChild(this.specialValue);

    const controlsHint = document.createElement('div');
    controlsHint.className = 'selection-controls';
    controlsHint.textContent = DEFAULT_HINT;
    this.hintEl = controlsHint;

    this.remainingEl = document.createElement('div');
    this.remainingEl.className = 'selection-remaining';
    this.remainingEl.style.display = 'none';

    panel.appendChild(this.nameEl);
    panel.appendChild(weaponEl);
    panel.appendChild(specialEl);
    panel.appendChild(this.remainingEl);
    if (controls) {
      // Under the ship: the button for the mouse and the keys.
      this.bottom = document.createElement('div');
      this.bottom.className = 'selection-bottom';
      this.chooseButton = document.createElement('button');
      this.chooseButton.type = 'button';
      this.chooseButton.className = 'selection-choose';
      this.chooseButton.textContent = 'Choose this ship';
      this.chooseButton.addEventListener('click', () => controls.choose());
      this.chooseButton.addEventListener('mousedown', event => event.preventDefault());
      this.bottom.append(this.chooseButton, controlsHint);
    } else {
      panel.appendChild(controlsHint);
    }

    this.root.appendChild(this.header);
    this.root.appendChild(this.subEl);
    this.root.appendChild(panel);
    this.parent = parent;
    parent.appendChild(this.root);
    if (controls) {
      // Mouse: the arrows at the sides browse the ships.
      for (const [side, label, action] of [['left', 'Previous ship', controls.prev], ['right', 'Next ship', controls.next]] as const) {
        const arrow = document.createElement('button');
        arrow.type = 'button';
        arrow.className = `selection-arrow ${side}`;
        arrow.setAttribute('aria-label', label);
        arrow.tabIndex = -1;
        arrow.textContent = side === 'left' ? '‹' : '›';
        arrow.addEventListener('click', () => action());
        parent.appendChild(arrow);
        this.arrows.push(arrow);
      }
      if (this.bottom) parent.appendChild(this.bottom);
    }
  }

  show(playerNum: number, character: CharacterDef): void {
    const header = this.headerText || `Player ${playerNum} — Choose Your Ship`;
    const changed = this.header.textContent !== header;
    this.header.textContent = header;
    const before = this.nameEl.textContent;
    this.nameEl.textContent = character.name;
    this.weaponValue.textContent = character.weapon;
    this.specialValue.textContent = character.special;
    this.root.style.display = '';
    if (this.bottom) this.bottom.style.display = '';
    if (changed || before !== character.name) announce(`${changed ? header + '. ' : ''}${character.name}. Weapon: ${character.weapon}. Special: ${character.special}.`);
  }

  /** A header in place of "Player N — Choose Your Ship" ('' for the default). */
  setHeader(text: string): void { this.headerText = text; }

  /** The key hint ('' for the default: Esc goes to the menu). */
  setHint(text: string): void { this.hintEl.textContent = text || DEFAULT_HINT; }

  /** The fleets of the match as chips, over the button under the ship (an empty list: none). */
  setFleets(lines: HTMLElement[]): void {
    if (!this.bottom) return;
    if (!lines.length) { this.fleetsEl?.remove(); this.fleetsEl = null; return; }
    if (!this.fleetsEl) {
      this.fleetsEl = document.createElement('div');
      this.fleetsEl.className = 'selection-fleets';
      this.bottom.prepend(this.fleetsEl);
    }
    this.fleetsEl.replaceChildren(...lines);
  }

  /** A second line under the header, for example the opponent of this battle. */
  setSubtitle(text: string): void {
    this.subEl.textContent = text;
    this.subEl.style.display = text ? '' : 'none';
  }

  showRemaining(count: number): void {
    if (count > 0) {
      this.remainingEl.textContent = `You have ${count} ship${count === 1 ? '' : 's'} left`;
      this.remainingEl.style.display = '';
    } else {
      this.remainingEl.style.display = 'none';
    }
  }

  hide(): void {
    this.root.style.display = 'none';
    if (this.bottom) this.bottom.style.display = 'none';
  }

  destroy(): void {
    this.root.remove();
    this.bottom?.remove();
    for (const arrow of this.arrows) arrow.remove();
  }
}
