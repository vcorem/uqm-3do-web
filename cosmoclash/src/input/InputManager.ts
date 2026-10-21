export interface PlayerInput {
  thrust: boolean;
  rotateLeft: boolean;
  rotateRight: boolean;
  firePrimary: boolean;
  fireSecondary: boolean;
}

interface PlayerKeyBindings {
  thrust: string[];
  rotateLeft: string[];
  rotateRight: string[];
  firePrimary: string[];
  fireSecondary: string[];
}

/** Preserve text editing and browser shortcuts while game listeners are active. */
export function shouldIgnoreGameKeyDown(event: KeyboardEvent): boolean {
  if (event.ctrlKey || event.metaKey || event.altKey || event.isComposing) return true;
  // A shadow root retargets event.target to its host; the composed path retains
  // the actual input element for an open shadow root.
  const target = event.composedPath().find(node => node instanceof Element) ?? event.target;
  if (!(target instanceof Element)) return false;
  if (target.closest('input, textarea, select')) return true;
  for (let element: Element | null = target; element; element = element.parentElement) {
    const editable = element.getAttribute('contenteditable')?.toLowerCase();
    if (editable === 'false') return false;
    if (editable === '' || editable === 'true' || editable === 'plaintext-only') return true;
  }
  return false;
}

const DEFAULT_BINDINGS: PlayerKeyBindings[] = [
  // Player 1: Arrow keys + / + .
  {
    thrust: ['ArrowUp'],
    rotateLeft: ['ArrowLeft'],
    rotateRight: ['ArrowRight'],
    firePrimary: ['/'],
    fireSecondary: ['.'],
  },
  // Player 2: WASD + Space + Shift
  {
    thrust: ['w', 'W'],
    rotateLeft: ['a', 'A'],
    rotateRight: ['d', 'D'],
    firePrimary: [' '],
    fireSecondary: ['Shift'],
  },
];

/**
 * Shift is Player 2's special. While it is held, Player 1's '/' and '.' report the
 * shifted symbols ('?' and '>' on US keyboards), and Player 1 could not fire.
 * They count as the keys under them, on press and on release.
 */
const UNSHIFTED: Record<string, string> = { '?': '/', '>': '.' };

export class InputManager {
  private pressed = new Set<string>();
  private bindings: PlayerKeyBindings[];
  private allGameKeys: Set<string>;
  private onKeyDown: (e: KeyboardEvent) => void;
  private onKeyUp: (e: KeyboardEvent) => void;
  private onBlur = () => this.pressed.clear();

  constructor(bindings: PlayerKeyBindings[] = DEFAULT_BINDINGS) {
    this.bindings = bindings;

    // Build set of all game keys for preventDefault
    this.allGameKeys = new Set<string>();
    for (const b of bindings) {
      for (const keys of Object.values(b)) {
        for (const k of keys) this.allGameKeys.add(k);
      }
    }

    this.onKeyDown = (e: KeyboardEvent) => {
      if (shouldIgnoreGameKeyDown(e)) return;
      const key = UNSHIFTED[e.key] ?? e.key;
      if (this.allGameKeys.has(key)) {
        e.preventDefault();
      }
      this.pressed.add(key);
    };

    this.onKeyUp = (e: KeyboardEvent) => {
      this.pressed.delete(e.key);
      this.pressed.delete(UNSHIFTED[e.key] ?? e.key);
      // Shift modifier changes key case: releasing 'd' while Shift is held
      // reports 'D', leaving lowercase 'd' stuck. Delete both cases.
      if (e.key.length === 1) {
        this.pressed.delete(e.key.toLowerCase());
        this.pressed.delete(e.key.toUpperCase());
      }
    };

    window.addEventListener('keydown', this.onKeyDown);
    window.addEventListener('keyup', this.onKeyUp);
    window.addEventListener('blur', this.onBlur);
  }

  player(index: number): PlayerInput {
    const b = this.bindings[index];
    return {
      thrust: b.thrust.some(k => this.pressed.has(k)),
      rotateLeft: b.rotateLeft.some(k => this.pressed.has(k)),
      rotateRight: b.rotateRight.some(k => this.pressed.has(k)),
      firePrimary: b.firePrimary.some(k => this.pressed.has(k)),
      fireSecondary: b.fireSecondary.some(k => this.pressed.has(k)),
    };
  }

  destroy(): void {
    window.removeEventListener('keydown', this.onKeyDown);
    window.removeEventListener('keyup', this.onKeyUp);
    window.removeEventListener('blur', this.onBlur);
    this.pressed.clear();
  }
}
