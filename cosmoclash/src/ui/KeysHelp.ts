import './keys-help.css';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { shouldIgnoreGameKeyDown } from '@/input/InputManager';
import { el } from './dom';
import { announce } from './live';

/**
 * The keys of the game outside the game screen, as on the UQM pages
 * (web/game-help/game-help.js): when the window is wider than the 4:3 game
 * (also in full screen), a panel in each dark margin beside it: flying and
 * the ships on the left, the menus and the game keys on the right under the
 * Keys (K) button (narrow margins, as 16:10 screens and MacBooks in full
 * screen, have one column: each key above its label). Without room, the same
 * help is a pop-up over the game, and the button is under the game (a phone
 * upright) or in its corner.
 *
 * K or the button shows or hides it in every screen. Nothing pauses (the
 * online lockstep goes on) and the keyboard focus stays with the game. The
 * UQM pages use ?, but here ? is Shift + /: two fire keys of a Two Players
 * battle. K is not a key of any player or menu.
 */
export type KeysMode = 'menu' | 'ai' | 'local' | 'online';
/** The remembered choice for the side panels: 'shown' (also the default) or 'hidden'. */
export const KEYS_STORAGE = 'cosmoclash.keys.v1';
/**
 * The side panels need this many pixels in each margin beside the game, and a
 * frame this high. 96 px also takes the 16:10 screens and the MacBooks in full
 * screen (1280x800: 107, 1440x900: 120, 1470x956: 98, 1512x982: 101).
 */
export const SIDE_MIN = 96;
export const SIDE_MIN_HEIGHT = 300;
/** Margins under this width have one-column panels (each key above its label), which need a frame this high. */
export const SIDE_WIDE = 150;
export const COMPACT_MIN_HEIGHT = 600;
/** Without side panels, a bar this high under the game (a phone upright) takes the button. */
export const BAR_MIN = 48;

export interface KeysLayout {
  /** Panels in the side margins (else the pop-up). */
  side: boolean;
  /** Narrow margins: one column in each panel, each key above its label. */
  compact: boolean;
  /** No side panels, but a bar under the game: the button goes there, not over the game. */
  below: boolean;
  /** The width of each margin beside the game. */
  margin: number;
  /** The text size of the side panels and their distance to the edges (px). */
  font: number;
  gap: number;
}
const clamp = (value: number, low: number, high: number): number => Math.min(high, Math.max(low, value));
/** The height of the 4:3 game in the frame, as Viewport makes it. */
export const gameHeightIn = (frameWidth: number, frameHeight: number): number =>
  frameWidth / frameHeight > 4 / 3 ? frameHeight : Math.floor(frameWidth * 3 / 4);
/** The side panels or the pop-up, from the size of the frame and the size of the 4:3 game in it. */
export function keysLayout(frameWidth: number, frameHeight: number, gameWidth: number, gameHeight = gameHeightIn(frameWidth, frameHeight)): KeysLayout {
  const margin = Math.max(0, Math.floor((frameWidth - gameWidth) / 2));
  const bar = Math.max(0, Math.floor((frameHeight - gameHeight) / 2));
  const compact = margin < SIDE_WIDE;
  const side = margin >= SIDE_MIN && frameHeight >= (compact ? COMPACT_MIN_HEIGHT : SIDE_MIN_HEIGHT);
  // Two columns need about 12 font sizes of width, one column about 8 (and its rows about 70 of height). Never under 12 px.
  const font = Math.round(clamp(compact ? Math.min(margin / 8, frameHeight / 70) : Math.min(margin / 12, frameHeight / 48), 12, compact ? 16 : 18) * 10) / 10;
  const gap = compact ? 6 : Math.round(clamp(margin * 0.05, 8, 16));
  return { side, compact: side && compact, below: !side && bar >= BAR_MIN, margin, font, gap };
}

// ---------------------------------------------------------------- content
export interface KeyRow { keys: string[]; label: string }
export interface KeySection { title: string; tone?: 'p1' | 'p2'; note?: string; rows: KeyRow[] }
export interface ShipKeys { name: string; weapon: string; special: string }
export interface KeysContent {
  /** The mode and who flies with which keys. */
  title: string;
  intro: string;
  /** Left: the players' keys and the ships. Right: the menus and the game keys. */
  flying: KeySection[];
  ships: ShipKeys[];
  menus: KeySection[];
}
const P1_ROWS: KeyRow[] = [{ keys: ['↑'], label: 'thrust' }, { keys: ['←', '→'], label: 'turn' }, { keys: ['/'], label: 'fire' }, { keys: ['.'], label: 'special' }];
const P2_ROWS: KeyRow[] = [{ keys: ['W'], label: 'thrust' }, { keys: ['A', 'D'], label: 'turn' }, { keys: ['Space'], label: 'fire' }, { keys: ['Shift'], label: 'special' }];
const TITLES: Record<KeysMode, string> = { menu: 'Keys', ai: 'vs Computer', local: 'Two Players', online: 'Online Melee' };
const INTROS: Record<KeysMode, string> = {
  menu: 'Player 2 flies only in Two Players.',
  ai: 'You fly with the Player 1 keys.',
  local: 'Player 1 and Player 2 share one keyboard.',
  online: 'Both players fly with the Player 1 keys, each on their own computer.',
};

/** What matters in this mode: Two Players (and the main menu) show both players. */
export function keysContent(mode: KeysMode): KeysContent {
  const both = mode === 'local' || mode === 'menu';
  const flying: KeySection[] = both
    ? [{ title: 'Player 1', tone: 'p1', rows: P1_ROWS }, { title: 'Player 2', tone: 'p2', rows: P2_ROWS }]
    : [{ title: 'Your ship', tone: 'p1', rows: P1_ROWS }];
  const menus: KeySection[] = [
    {
      title: 'Menus',
      // Two Players: Space is the fire key of Player 2, so the menus choose with Enter only.
      note: mode === 'local' ? 'Space is the fire key of Player 2: only Enter selects.' : undefined,
      rows: [
        { keys: ['↑ ↓', '← →'], label: 'move' },
        { keys: mode === 'local' ? ['Enter'] : ['Enter', 'Space'], label: 'select' },
        { keys: ['Esc'], label: 'back' },
        // Against the computer both fleets have all three ships: no fleet choice, nothing to undo.
        ...(mode === 'ai' ? [] : [{ keys: ['Backspace'], label: 'undo a pick' }]),
      ],
    },
    {
      title: 'Game',
      note: mode === 'online' ? 'An online battle does not pause.' : undefined,
      rows: [
        { keys: ['Esc'], label: mode === 'online' ? 'leave the match' : mode === 'menu' ? 'pause (online: leave)' : 'pause' },
        { keys: ['M'], label: 'sound on/off' },
        { keys: ['F'], label: 'full screen (menus)' },
        { keys: ['K'], label: 'these keys' },
      ],
    },
  ];
  const ships = CHARACTER_REGISTRY.map(def => ({ name: def.name, weapon: def.weapon, special: def.special }));
  return { title: TITLES[mode], intro: INTROS[mode], flying, ships, menus };
}

function sectionNode(section: KeySection): HTMLElement {
  return el('section', { class: 'ck-section' + (section.tone ? ' ck-' + section.tone : '') },
    el('h3', { text: section.title }),
    el('dl', { class: 'ck-rows' }, ...section.rows.flatMap(row => [
      el('dt', {}, ...row.keys.map(key => el('kbd', { text: key }))),
      el('dd', { text: row.label }),
    ])),
    section.note ? el('p', { class: 'ck-note', text: section.note }) : null);
}
function shipsNode(ships: ShipKeys[]): HTMLElement {
  return el('section', { class: 'ck-section ck-ships' },
    el('h3', { text: 'Ships' }),
    el('ul', {}, ...ships.map(ship => el('li', {},
      el('b', { text: ship.name }),
      el('span', {}, el('i', { text: 'Weapon ' }), ship.weapon),
      el('span', {}, el('i', { text: 'Special ' }), ship.special)))));
}
/** The mode and who flies with which keys, then the players' keys (the pop-up has the mode in its title). */
function flyingNodes(content: KeysContent, title: boolean): HTMLElement[] {
  return [el('p', { class: 'ck-mode' }, title ? el('strong', { text: content.title }) : null, title ? ' ' : null, content.intro),
    ...content.flying.map(sectionNode)];
}
const menuNodes = (content: KeysContent): HTMLElement[] => content.menus.map(sectionNode);

// ---------------------------------------------------------------- the panels
interface KeysStore { get(key: string): string | null; set(key: string, value: string): void }
const localStore: KeysStore = {
  get(key) { try { return localStorage.getItem(key); } catch { return null; } },
  set(key, value) { try { localStorage.setItem(key, value); } catch { /* Private mode: this visit only. */ } },
};
export interface KeysHelpOptions {
  /** The full-window frame (the full screen element) and the 4:3 game container in it. */
  frame: HTMLElement;
  container: HTMLElement;
  storage?: KeysStore;
  mode?: KeysMode;
  /** The frame width and height and the game width, and the game height (tests; else measured). */
  measure?(): [number, number, number, number?];
}
/** The key event parts that KeysHelp reads (a KeyboardEvent in the page). */
export type KeysEvent = Pick<KeyboardEvent, 'key' | 'code' | 'repeat' | 'ctrlKey' | 'metaKey' | 'altKey' | 'isComposing' | 'target' | 'composedPath'>;

/** K: is this the K key (also a K typed with Shift, and the K position of a non-Latin layout)? */
export function isKeysKey(event: Pick<KeyboardEvent, 'key' | 'code'>): boolean {
  if (event.key === 'k' || event.key === 'K') return true;
  return event.code === 'KeyK' && !/^[a-z]$/i.test(event.key);
}

export class KeysHelp {
  readonly button: HTMLButtonElement;
  /** The side panels: left (flying, ships) and right (menus, game). */
  readonly panels: [HTMLElement, HTMLElement];
  readonly pop: HTMLDivElement;
  private readonly margins: [HTMLDivElement, HTMLDivElement];
  private readonly popBody: HTMLDivElement;
  private readonly popTitle: HTMLHeadingElement;
  private readonly storage: KeysStore;
  private mode: KeysMode;
  /** The player's choice for the side panels (remembered). */
  private shown: boolean;
  private popOpen = false;
  private side = false;
  private oneColumn = false;
  /** Where the button is: on top of the right margin, in the bar under the game, or in the corner of the game. */
  private place: 'side' | 'below' | 'corner' | null = null;

  constructor(private readonly options: KeysHelpOptions) {
    this.storage = options.storage ?? localStore;
    this.mode = options.mode ?? 'menu';
    this.shown = this.storage.get(KEYS_STORAGE) !== 'hidden';

    this.button = el('button', { type: 'button', id: 'cosmo-keys-button', class: 'ck-button', text: 'Keys (K)', title: 'Show or hide the keys (K)', aria: { expanded: 'false' } });
    // The pointer does not take the keyboard focus from the game.
    this.button.addEventListener('mousedown', event => event.preventDefault());
    this.button.addEventListener('click', () => this.toggle());

    this.panels = [
      el('aside', { id: 'cosmo-keys-flying', class: 'ck-panel', aria: { label: 'Keys: flying and ships' } }),
      el('aside', { id: 'cosmo-keys-menus', class: 'ck-panel', aria: { label: 'Keys: menus and game' } }),
    ];
    this.margins = [el('div', { class: 'ck-margin ck-left', hidden: true }, this.panels[0]), el('div', { class: 'ck-margin ck-right', hidden: true }, this.panels[1])];
    options.frame.append(...this.margins);

    const close = el('button', { type: 'button', class: 'cc-btn cc-small ck-close', text: 'Close (K)' });
    close.addEventListener('mousedown', event => event.preventDefault());
    close.addEventListener('click', () => this.close());
    this.popBody = el('div', { class: 'ck-pop-body' });
    // Not modal: the game goes on under it and keeps the keyboard. In the frame, centred over the
    // game: as wide as the game at most, and as high as the frame (a phone upright has bars above and under the game).
    this.pop = el('div', { id: 'cosmo-keys-pop', class: 'ck-pop', role: 'dialog', hidden: true, aria: { label: 'Keys', modal: 'false' } },
      el('header', {}, this.popTitle = el('h2', { text: 'Keys' }), close), this.popBody);
    options.frame.append(this.pop);

    this.render();
    this.layout();
  }

  /** The help shows now (the side panels, or the pop-up). */
  get visible(): boolean { return this.side ? this.shown : this.popOpen; }
  /** There is room for the side panels. */
  get sideMode(): boolean { return this.side; }
  /** The side panels have one column (narrow margins). */
  get compact(): boolean { return this.oneColumn; }
  get currentMode(): KeysMode { return this.mode; }

  /** The game mode for the content: the main menu shows both players. */
  setMode(mode: KeysMode): void {
    if (mode === this.mode) return;
    this.mode = mode;
    this.render();
  }

  /** After a size change of the frame (Viewport.onResize): side panels or the pop-up. */
  layout(): void {
    const { frame, container } = this.options;
    const [width, height, game, measured] = this.options.measure?.() ?? [frame.clientWidth || window.innerWidth, frame.clientHeight || window.innerHeight, container.offsetWidth, container.offsetHeight];
    const gameHeight = measured || gameHeightIn(width, height);
    const layout = keysLayout(width, height, game, gameHeight);
    frame.style.setProperty('--ck-margin', `${layout.margin}px`);
    frame.style.setProperty('--ck-font', `${layout.font}px`);
    frame.style.setProperty('--ck-gap', `${layout.gap}px`);
    frame.style.setProperty('--ck-game-w', `${game}px`);
    frame.style.setProperty('--ck-game-h', `${gameHeight}px`);
    // The pop-up and the button under the game are outside #game-container: the same --u (the size unit of the menus).
    const unit = container.style.getPropertyValue('--u') || String(Math.max(0.3, gameHeight / 720));
    for (const node of [this.pop, this.button]) node.style.setProperty('--u', unit);
    this.oneColumn = layout.compact;
    for (const margin of this.margins) margin.classList.toggle('ck-compact', layout.compact);
    // The pop-up never opens by itself, also not after a size change.
    if (layout.side !== this.side) this.popOpen = false;
    this.side = layout.side;
    const place = layout.side ? 'side' : layout.below ? 'below' : 'corner';
    if (place !== this.place || !this.button.isConnected) {
      this.place = place;
      const focused = document.activeElement === this.button;
      // Beside the game: on top of the right margin. A phone upright: in the bar under the game. Else the bottom right corner of the game.
      if (place === 'side') this.margins[1].prepend(this.button);
      else if (place === 'below') frame.append(this.button);
      else container.append(this.button);
      if (focused) this.button.focus({ preventScroll: true });
    }
    this.sync();
  }

  /** K or the button: the side panels (the choice is remembered) or the pop-up. */
  toggle(): void {
    if (this.side) {
      this.shown = !this.shown;
      this.storage.set(KEYS_STORAGE, this.shown ? 'shown' : 'hidden');
      announce(this.shown ? 'Keys shown beside the game.' : 'Keys hidden. Press K to show them.');
    } else {
      this.popOpen = !this.popOpen;
      announce(this.popOpen ? 'Keys shown over the game. Press K or Esc to close them.' : 'Keys closed.');
    }
    this.sync();
  }

  /** Closes the pop-up (Close, Esc). */
  close(): void {
    if (!this.popOpen) return;
    this.popOpen = false;
    this.sync();
    announce('Keys closed.');
  }

  /**
   * A key press. Returns true when the help used it (the page then gives it to
   * nobody else): K (a held-key repeat does nothing), and Esc while the pop-up
   * is open (it closes the pop-up only, so it does not also pause, leave or
   * cancel the room name). A K typed in a text field (the room name) is a letter.
   */
  key(event: KeysEvent): boolean {
    const plain = !event.ctrlKey && !event.metaKey && !event.altKey && !event.isComposing;
    if (event.key === 'Escape' && this.popOpen && !this.side && plain) {
      if (!event.repeat) this.close();
      return true;
    }
    if (shouldIgnoreGameKeyDown(event as KeyboardEvent)) return false;
    if (isKeysKey(event)) {
      if (!event.repeat) this.toggle();
      return true;
    }
    return false;
  }

  private sync(): void {
    for (const margin of this.margins) margin.hidden = !this.side;
    for (const panel of this.panels) panel.hidden = !(this.side && this.shown);
    this.pop.hidden = this.side || !this.popOpen;
    this.button.setAttribute('aria-expanded', String(this.visible));
    this.button.setAttribute('aria-controls', this.side ? this.panels.map(panel => panel.id).join(' ') : this.pop.id);
    this.button.classList.toggle('is-on', this.visible);
  }

  private render(): void {
    const content = keysContent(this.mode);
    // Beside the game: flying and the ships on the left, the menus and the game keys on the right.
    this.panels[0].replaceChildren(...flyingNodes(content, true), shipsNode(content.ships));
    this.panels[1].replaceChildren(...menuNodes(content));
    // Over the game: flying | menus and game, the ships in a row under them.
    this.popTitle.textContent = this.mode === 'menu' ? 'Keys' : `Keys · ${content.title}`;
    this.popBody.replaceChildren(el('div', { class: 'ck-part' }, ...flyingNodes(content, false)), el('div', { class: 'ck-part' }, ...menuNodes(content)), shipsNode(content.ships));
  }

  destroy(): void {
    for (const margin of this.margins) margin.remove();
    this.pop.remove();
    this.button.remove();
  }
}
