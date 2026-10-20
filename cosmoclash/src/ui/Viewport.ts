const ASPECT = 4 / 3;
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
      if (!this.frame.requestFullscreen) return Promise.resolve(false);
      return Promise.resolve(this.frame.requestFullscreen({ navigationUI: 'hide' })).then(() => {
        const keyboard = (navigator as Navigator & { keyboard?: { lock?(keys: string[]): Promise<void> } }).keyboard;
        keyboard?.lock?.(['Escape']).catch(() => {});
        return true;
      }, () => false);
    } catch {
      return Promise.resolve(false);
    }
  }

  get width(): number {
    return this.container.clientWidth;
  }

  get height(): number {
    return this.container.clientHeight;
  }
}
