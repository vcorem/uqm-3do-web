import { CosmoNetplay, createRoomId, invitationURL, parseRoom, type NetplayOptions, type NetplayState } from './CosmoNetplay';
import './netplay.css';
export interface NetplayPanelOptions extends NetplayOptions {
  container?: HTMLElement;
  relayOrigin?: string;
  prepare?: () => Promise<void>;
  onLeave?: () => void;
}
const names = ['MichiGichi', 'Gondi', 'Sholev'];
/** Owns only the room overlay. Parent owns rendering, input and the Rust simulation. */
export class NetplayPanel {
  readonly element: HTMLDivElement;
  session: CosmoNetplay | null = null;
  private readonly toolbar: HTMLDivElement;
  private readonly options: NetplayPanelOptions;
  private preparing = false;
  private destroyed = false;
  constructor(options: NetplayPanelOptions) {
    this.options = options;
    this.element = document.createElement('div'); this.element.className = 'cosmo-netplay'; this.element.hidden = true;
    this.element.innerHTML = `<section class="cosmo-room" aria-label="Online fleet match">
      <h1>Online fleet match</h1><p class="cosmo-net-status" role="status" aria-live="polite"></p>
      <div class="cosmo-connect"><button type="button" data-action="create">Create room</button>
      <label>Invitation or room code <input data-room placeholder="Paste your friend's invitation" autocomplete="off" spellcheck="false"></label>
      <button type="button" data-action="join">Join room</button></div>
      <div class="cosmo-invitation" hidden><label>Share this invitation <input data-invitation readonly></label><button type="button" data-action="copy">Copy invitation</button></div>
      <div class="cosmo-lobby" hidden><fieldset><legend>Your fleet — select one to three ships</legend>
      ${names.map((name, id) => `<label><input type="checkbox" data-ship="${id}" checked> ${name}</label>`).join('')}
      </fieldset><p data-fleet-order></p><p data-peer></p><button type="button" data-action="ready">Ready</button></div>
      <div class="cosmo-ended" hidden><button type="button" data-action="rematch">Rematch</button></div>
      <p class="cosmo-help">Both players use the arrow keys to fly, / to fire and . for the special ability. Keep the game tab open during a match.</p>
      <button type="button" data-action="leave">Back to menu</button>
    </section>`;
    (options.container ?? document.body).appendChild(this.element);
    this.toolbar = document.createElement('div'); this.toolbar.className = 'cosmo-online-toolbar'; this.toolbar.hidden = true;
    this.toolbar.innerHTML = '<span>Online fleet match</span><button type="button">Leave match</button>';
    this.toolbar.querySelector('button')!.addEventListener('click', () => { this.session?.close(); this.session = null; this.hide(); this.options.onLeave?.(); });
    (options.container ?? document.body).appendChild(this.toolbar);
    this.element.addEventListener('click', this.onClick);
    this.element.addEventListener('change', this.onFleetChange);
  }
  open(room = new URL(location.href).searchParams.get('cosmoRoom') ?? ''): void {
    this.element.hidden = false;
    if (this.session && !['closed', 'error'].includes(this.session.state.phase)) { this.render(this.session.state); return; }
    this.session = null;
    this.query<HTMLInputElement>('[data-room]').value = room;
    this.query('.cosmo-connect').hidden = false; this.query('.cosmo-invitation').hidden = true;
    this.query('.cosmo-lobby').hidden = true; this.query('.cosmo-ended').hidden = true;
    this.status('Create a private room or paste a friend’s invitation.');
  }
  show(): void { this.open(); }
  hide(): void { this.element.hidden = true; }
  destroy(): void {
    this.destroyed = true; this.session?.close(); this.element.removeEventListener('click', this.onClick);
    this.element.removeEventListener('change', this.onFleetChange); this.element.remove(); this.toolbar.remove();
  }
  private query<T extends HTMLElement = HTMLElement>(selector: string): T { return this.element.querySelector<T>(selector)!; }
  private status(message: string): void { this.query('.cosmo-net-status').textContent = message; }
  private onClick = (event: Event): void => {
    const action = (event.target as HTMLElement).closest<HTMLButtonElement>('button[data-action]')?.dataset.action;
    if (action === 'create') this.connect(createRoomId(), 'host');
    if (action === 'join') {
      const room = parseRoom(this.query<HTMLInputElement>('[data-room]').value);
      if (room) this.connect(room, 'guest'); else this.status('Paste a valid invitation link or 32-character room code.');
    }
    if (action === 'copy') {
      const input = this.query<HTMLInputElement>('[data-invitation]');
      void navigator.clipboard?.writeText(input.value).then(() => this.status('Invitation copied. Send it to your friend.')).catch(() => { input.select(); this.status('Select and copy the invitation above.'); });
      if (!navigator.clipboard) { input.select(); this.status('Select and copy the invitation above.'); }
    }
    if (action === 'ready') void this.ready();
    if (action === 'rematch') this.session?.rematch();
    if (action === 'leave') { this.session?.close(); this.session = null; this.hide(); this.options.onLeave?.(); }
  };
  private onFleetChange = (event: Event): void => {
    const input = event.target as HTMLInputElement;
    if (!input.matches('[data-ship]') || !this.session) return;
    const id = Number(input.dataset.ship); const fleet = this.session.state.localFleet;
    const selected = input.checked ? [...fleet.filter(n => n !== id), id] : fleet.filter(n => n !== id);
    if (!selected.length) { input.checked = true; this.status('Keep at least one ship in your fleet.'); return; }
    this.session.chooseFleet(selected);
  };
  private connect(room: string, role: 'host' | 'guest'): void {
    this.session?.close();
    const session = new CosmoNetplay({...this.options,
      onState: state => { if (this.destroyed || this.session !== session) return; this.render(state); this.options.onState?.(state); },
      onStart: start => { this.hide(); this.options.onStart(start); },
      onEnd: winner => { this.element.hidden = false; this.options.onEnd?.(winner); },
    });
    this.session = session;
    this.query<HTMLInputElement>('[data-invitation]').value = invitationURL(room);
    session.connect({origin: this.options.relayOrigin ?? location.origin, room, role});
  }
  private async ready(): Promise<void> {
    const session = this.session;
    if (!session || this.preparing) return;
    this.preparing = true; this.query<HTMLButtonElement>('[data-action="ready"]').disabled = true;
    this.status('Preparing the game…');
    let error = '';
    try {
      await this.options.prepare?.();
      if (!this.destroyed && this.session === session) session.ready();
    } catch { error = 'The game assets could not load. Please try again.'; }
    finally { this.preparing = false; if (!this.destroyed && this.session === session) { this.render(session.state); if (error) this.status(error); } }
  }
  private render(state: NetplayState): void {
    this.status(state.message);
    this.toolbar.hidden = !['playing', 'finishing'].includes(state.phase);
    this.toolbar.querySelector('span')!.textContent = state.phase === 'finishing' ? 'Verifying match result…' : `Online · ${state.role === 'host' ? 'Host' : 'Guest'} · ${state.verifiedChecksums} state checks`;
    const done = ['closed', 'error'].includes(state.phase);
    if (done) this.element.hidden = false;
    this.query('.cosmo-connect').hidden = !done;
    this.query('.cosmo-invitation').hidden = done || state.role !== 'host';
    this.query('.cosmo-lobby').hidden = state.phase !== 'lobby';
    this.query('.cosmo-ended').hidden = state.phase !== 'ended';
    for (const checkbox of this.element.querySelectorAll<HTMLInputElement>('[data-ship]')) {
      checkbox.checked = state.localFleet.includes(Number(checkbox.dataset.ship)); checkbox.disabled = state.localReady || this.preparing;
    }
    this.query('[data-fleet-order]').textContent = `Your order: ${state.localFleet.map(id => names[id]).join(' → ')}`;
    this.query('[data-peer]').textContent = state.peerFleet.length ? `Friend’s fleet: ${state.peerFleet.map(id => names[id]).join(' → ')}${state.peerReady ? ' · Ready' : ''}` : 'Waiting for your friend’s fleet…';
    const ready = this.query<HTMLButtonElement>('[data-action="ready"]');
    ready.disabled = state.localReady || !state.peerFleet.length || this.preparing;
    ready.textContent = state.localReady ? 'Ready — waiting for your friend' : 'Ready';
    const rematch = this.query<HTMLButtonElement>('[data-action="rematch"]');
    rematch.disabled = state.localRematch; rematch.textContent = state.localRematch ? 'Rematch requested' : state.peerRematch ? 'Accept rematch' : 'Rematch';
  }
}
