/**
 * One polite live region (role=status) for the whole game: the screen and its
 * messages, for screen readers. The same text is not said twice in a row.
 */
let region: HTMLElement | null = null;
let last = '';
export function attachLiveRegion(parent: HTMLElement): HTMLElement {
  if (region?.isConnected) return region;
  region = document.createElement('div');
  region.id = 'cosmo-status';
  region.className = 'cc-live';
  region.setAttribute('role', 'status');
  region.setAttribute('aria-live', 'polite');
  parent.appendChild(region);
  return region;
}
export function announce(text: string): void {
  const value = text.trim();
  if (!region || value === last) return;
  last = value;
  region.textContent = value;
}
/** The text that the live region says now (tests read it). */
export function announced(): string { return last; }
