import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { InputManager } from './InputManager';

function press(key: string) {
  window.dispatchEvent(new KeyboardEvent('keydown', { key }));
}

function release(key: string) {
  window.dispatchEvent(new KeyboardEvent('keyup', { key }));
}

describe('InputManager', () => {
  let input: InputManager;

  beforeEach(() => {
    input = new InputManager();
  });

  afterEach(() => {
    input.destroy();
    document.body.replaceChildren();
  });

  describe('text editing and browser shortcuts', () => {
    const gameKeys = ['a', 'd', 'w', '/', '.', ' ', 'ArrowUp', 'ArrowLeft', 'ArrowRight', 'Shift'];
    const idle = () => {
      for (const player of [0, 1]) expect(Object.values(input.player(player))).not.toContain(true);
    };

    for (const tag of ['input', 'textarea', 'select']) {
      it(`preserves editing keys in a focused ${tag}`, () => {
        const editor = document.createElement(tag);
        document.body.appendChild(editor); editor.focus();
        for (const key of gameKeys) {
          const event = new KeyboardEvent('keydown', { key, bubbles: true, cancelable: true });
          editor.dispatchEvent(event);
          expect(event.defaultPrevented, key).toBe(false);
          idle();
        }
      });
    }

    it('preserves editing keys in nested contenteditable elements', () => {
      const editor = document.createElement('div'); editor.setAttribute('contenteditable', 'true');
      const child = document.createElement('span'); editor.appendChild(child); document.body.appendChild(editor);
      for (const key of gameKeys) {
        const event = new KeyboardEvent('keydown', { key, bubbles: true, cancelable: true });
        child.dispatchEvent(event);
        expect(event.defaultPrevented, key).toBe(false); idle();
      }
    });

    it('uses the real input target inside an open shadow root', () => {
      const host = document.createElement('div'); document.body.appendChild(host);
      const editor = document.createElement('input'); host.attachShadow({ mode: 'open' }).appendChild(editor);
      const event = new KeyboardEvent('keydown', { key: '/', bubbles: true, cancelable: true, composed: true });
      editor.dispatchEvent(event);
      expect(event.target).toBe(host);
      expect(event.defaultPrevented).toBe(false); idle();
    });

    it('still releases previously held keys when keyup occurs inside an editor', () => {
      press('d'); press('/');
      const editor = document.createElement('input'); document.body.appendChild(editor); editor.focus();
      editor.dispatchEvent(new KeyboardEvent('keyup', { key: 'D', bubbles: true }));
      editor.dispatchEvent(new KeyboardEvent('keyup', { key: '/', bubbles: true }));
      idle();
    });

    for (const modifier of ['ctrlKey', 'metaKey', 'altKey']) {
      it(`preserves ${modifier} browser shortcuts outside an editor`, () => {
        const event = new KeyboardEvent('keydown', { key: 'a', [modifier]: true, cancelable: true });
        window.dispatchEvent(event);
        expect(event.defaultPrevented).toBe(false); idle();
      });
    }

    it('keeps Shift and ordinary game controls active outside editors', () => {
      for (const key of ['Shift', 'D']) {
        const event = new KeyboardEvent('keydown', { key, shiftKey: true, cancelable: true });
        window.dispatchEvent(event);
        expect(event.defaultPrevented).toBe(true);
      }
      expect(input.player(1).fireSecondary).toBe(true);
      expect(input.player(1).rotateRight).toBe(true);
    });
  });

  describe('Player 1 (Arrow keys + /)', () => {
    it('reports no input initially', () => {
      const p1 = input.player(0);
      expect(p1.thrust).toBe(false);
      expect(p1.rotateLeft).toBe(false);
      expect(p1.rotateRight).toBe(false);
      expect(p1.firePrimary).toBe(false);
    });

    it('detects ArrowUp as thrust', () => {
      press('ArrowUp');
      expect(input.player(0).thrust).toBe(true);
      release('ArrowUp');
      expect(input.player(0).thrust).toBe(false);
    });

    it('detects ArrowLeft as rotateLeft', () => {
      press('ArrowLeft');
      expect(input.player(0).rotateLeft).toBe(true);
    });

    it('detects ArrowRight as rotateRight', () => {
      press('ArrowRight');
      expect(input.player(0).rotateRight).toBe(true);
    });

    it('detects / as firePrimary', () => {
      press('/');
      expect(input.player(0).firePrimary).toBe(true);
      release('/');
      expect(input.player(0).firePrimary).toBe(false);
    });

    it('detects P1 secondary fire on period key', () => {
      window.dispatchEvent(new KeyboardEvent('keydown', { key: '.' }));
      const pi = input.player(0);
      expect(pi.fireSecondary).toBe(true);
      window.dispatchEvent(new KeyboardEvent('keyup', { key: '.' }));
    });

    it('does NOT respond to P2 keys', () => {
      press('w');
      press('a');
      press('d');
      press(' ');
      const p1 = input.player(0);
      expect(p1.thrust).toBe(false);
      expect(p1.rotateLeft).toBe(false);
      expect(p1.rotateRight).toBe(false);
      expect(p1.firePrimary).toBe(false);
    });
  });

  describe('Player 2 (WASD + Space)', () => {
    it('detects w as thrust (case insensitive)', () => {
      press('w');
      expect(input.player(1).thrust).toBe(true);
      release('w');
      press('W');
      expect(input.player(1).thrust).toBe(true);
    });

    it('detects a as rotateLeft (case insensitive)', () => {
      press('a');
      expect(input.player(1).rotateLeft).toBe(true);
      release('a');
      press('A');
      expect(input.player(1).rotateLeft).toBe(true);
    });

    it('detects d as rotateRight (case insensitive)', () => {
      press('d');
      expect(input.player(1).rotateRight).toBe(true);
      release('d');
      press('D');
      expect(input.player(1).rotateRight).toBe(true);
    });

    it('detects Space as firePrimary', () => {
      press(' ');
      expect(input.player(1).firePrimary).toBe(true);
      release(' ');
      expect(input.player(1).firePrimary).toBe(false);
    });

    it('detects P2 secondary fire on Shift key', () => {
      window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Shift' }));
      const pi = input.player(1);
      expect(pi.fireSecondary).toBe(true);
      window.dispatchEvent(new KeyboardEvent('keyup', { key: 'Shift' }));
    });

    it('does NOT respond to P1 keys', () => {
      press('ArrowUp');
      press('ArrowLeft');
      press('ArrowRight');
      press('/');
      const p2 = input.player(1);
      expect(p2.thrust).toBe(false);
      expect(p2.rotateLeft).toBe(false);
      expect(p2.rotateRight).toBe(false);
      expect(p2.firePrimary).toBe(false);
    });
  });

  describe('Shift + letter key case handling', () => {
    it('clears lowercase key when released with Shift held (uppercase keyup)', () => {
      // Press 'd' (lowercase, no Shift)
      press('d');
      expect(input.player(1).rotateRight).toBe(true);

      // Release D while Shift changes it to uppercase 'D'
      release('D');
      expect(input.player(1).rotateRight).toBe(false);
    });

    it('clears uppercase key when released without Shift (lowercase keyup)', () => {
      // Press 'D' (uppercase, Shift was held)
      press('D');
      expect(input.player(1).rotateRight).toBe(true);

      // Release d after Shift released (lowercase keyup)
      release('d');
      expect(input.player(1).rotateRight).toBe(false);
    });
  });

  describe('destroy', () => {
    it('stops tracking keys after destroy', () => {
      press('ArrowUp');
      input.destroy();
      press('ArrowLeft');
      expect(input.player(0).thrust).toBe(false);
      expect(input.player(0).rotateLeft).toBe(false);
    });
  });
});

it('releases held controls when the browser loses focus', () => {
  const manager = new InputManager();
  window.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowUp' }));
  expect(manager.player(0).thrust).toBe(true);
  window.dispatchEvent(new Event('blur'));
  expect(manager.player(0).thrust).toBe(false);
  manager.destroy();
});

describe('Player 1 fires while Player 2 holds Shift', () => {
  it("'?' and '>' count as '/' and '.' on press and on release, whatever the order", () => {
    const im = new InputManager();
    try {
      press('Shift');
      press('?');
      expect(im.player(0).firePrimary).toBe(true);
      expect(im.player(1).fireSecondary).toBe(true);
      release('Shift');
      release('/'); // Shift came up first: the release reports '/'.
      expect(im.player(0).firePrimary).toBe(false);
      press('/'); press('Shift');
      release('?'); // Shift went down meanwhile: the release reports '?'.
      expect(im.player(0).firePrimary).toBe(false);
      press('>');
      expect(im.player(0).fireSecondary).toBe(true);
      release('.');
      expect(im.player(0).fireSecondary).toBe(false);
    } finally { im.destroy(); }
  });
});

