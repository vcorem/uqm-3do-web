const ASPECT = 4 / 3;
/** Under this frame height (CSS px) the fleet screens leave out their secondary lines (see cosmo-ui.css). */
export const SHORT_HEIGHT = 420;
/** Design height of the menu screens: their sizes scale with var(--u) = height / 720. */
const DESIGN_HEIGHT = 720;

/**
 * The game frame (the element that goes full screen) and in it the 4:3
 * letterboxed game container. All game UI and the canvas go in the container.
 */
export class Viewport {
  readonly frame: HTMLDivElement;
  readonly container: HTMLDivElement;
  private listeners: (() => void)[] = [];
  /** The player turned full screen off (F or the menu button): menu choices do not turn it on again. */
  private windowed = false;
  private request: Promise<boolean> | null = null;

  constructor() {
    // Black background for letterbox bars
    document.body.style.margin = '0';
    document.body.style.overflow = 'hidden';
    document.body.style.background = '#000';

    this.frame = document.createElement('div');
    this.frame.id = 'cosmo-frame';
    this.frame.style.cssText = 'position:fixed;inset:0;overflow:hidden;background:#000';
    document.body.appendChild(this.frame);

    this.container = document.createElement('div');
    this.container.id = 'game-container';
    this.container.style.position = 'absolute';
    this.container.style.overflow = 'hidden';
    this.frame.appendChild(this.container);

    this.resize();
    window.addEventListener('resize', () => this.resize());
    document.addEventListener('fullscreenchange', () => this.resize());
  }

  /** Called after each size change (the renderer and cameras follow). */
  onResize(listener: () => void): void { this.listeners.push(listener); }

  private resize(): void {
    const w = this.frame.clientWidth || window.innerWidth;
    const h = this.frame.clientHeight || window.innerHeight;

    let cw: number, ch: number;
    if (w / h > ASPECT) {
      // Window is wider than 4:3 → pillarbox (bars on sides)
      ch = h;
      cw = Math.floor(h * ASPECT);
    } else {
      // Window is taller than 4:3 → letterbox (bars on top/bottom)
      cw = w;
      ch = Math.floor(w / ASPECT);
    }

    this.container.style.width = `${cw}px`;
    this.container.style.height = `${ch}px`;
    this.container.style.left = `${Math.floor((w - cw) / 2)}px`;
    this.container.style.top = `${Math.floor((h - ch) / 2)}px`;
    this.container.style.setProperty('--u', String(Math.max(0.3, ch / DESIGN_HEIGHT)));
    // A low frame (a phone held upright): the fleet screens leave out their secondary lines.
    this.container.classList.toggle('cc-short', ch < SHORT_HEIGHT);
    for (const listener of this.listeners) listener();
  }

  /**
   * Full screen for the game frame. Chromium also keeps Esc as a game key
   * (navigator.keyboard.lock); other browsers leave full screen with Esc.
   * Resolves false when the browser refused (no user activation) or cannot.
   */
  requestFullscreen(): Promise<boolean> {
    try {
      if (document.fullscreenElement) return Promise.resolve(true);
      if (this.request) return this.request;
      if (!this.frame.requestFullscreen) return Promise.resolve(false);
      const request = Promise.resolve(this.frame.requestFullscreen({ navigationUI: 'hide' })).then(() => {
        const keyboard = (navigator as Navigator & { keyboard?: { lock?(keys: string[]): Promise<void> } }).keyboard;
        keyboard?.lock?.(['Escape']).catch(() => {});
        return true;
      }, () => false).finally(() => { if (this.request === request) this.request = null; });
      this.request = request;
      return request;
    } catch {
      return Promise.resolve(false);
    }
  }

  /**
   * A menu choice made with a user gesture: full screen, unless the game is in
   * full screen already or the player turned it off. So full screen comes back
   * at the next choice when Esc (Firefox, Safari) or the browser left it.
   */
  fullscreenGesture(): Promise<boolean> {
    if (document.fullscreenElement) return Promise.resolve(true);
    if (this.windowed) return Promise.resolve(false);
    return this.requestFullscreen();
  }

  /** F or the menu button: full screen on or off; "off" stays until the player turns it on. */
  toggleFullscreen(): Promise<boolean> {
    if (document.fullscreenElement) {
      this.windowed = true;
      try { Promise.resolve(document.exitFullscreen?.()).catch(() => {}); } catch { /* Not in full screen. */ }
      return Promise.resolve(false);
    }
    this.windowed = false;
    return this.requestFullscreen();
  }

  get fullscreen(): boolean { return !!document.fullscreenElement; }

  get width(): number {
    return this.container.clientWidth;
  }

  get height(): number {
    return this.container.clientHeight;
  }
}
