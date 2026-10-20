import {afterEach, expect, it, vi} from 'vitest';
import {TextDecoder, TextEncoder} from 'node:util';
import {NetplayPanel} from './NetplayPanel';
import {INPUT_DELAY, PROTOCOL_VERSION, type SocketLike} from './CosmoNetplay';
Object.assign(globalThis, {TextDecoder, TextEncoder});
const panels: NetplayPanel[] = [];
afterEach(() => { panels.splice(0).forEach(panel => panel.destroy()); document.body.innerHTML = ''; });
function fixture(prepare?: () => Promise<void>) {
  const sent: any[] = [];
  const socket: SocketLike = {readyState: 1, binaryType: '', bufferedAmount: 0, onopen: null, onclose: null, onerror: null, onmessage: null,
    close: vi.fn(), send(bytes) { sent.push(JSON.parse(new TextDecoder().decode(bytes))); }};
  const onLeave = vi.fn();
  const panel = new NetplayPanel({buildId: 'panel-test', onStart: vi.fn(), onFrame: vi.fn(), prepare, onLeave,
    relayOrigin: 'http://localhost:9999', socketFactory: () => socket}); panels.push(panel);
  const relay = (p: unknown) => socket.onmessage?.({data: JSON.stringify(p)});
  const peer = (p: unknown) => socket.onmessage?.({data: new TextEncoder().encode(JSON.stringify(p))});
  const click = (action: string) => panel.element.querySelector<HTMLButtonElement>(`[data-action="${action}"]`)!.click();
  panel.open(); click('create'); relay({type: 'waiting'}); relay({type: 'ready', role: 'host'});
  peer({type: 'hello', version: PROTOCOL_VERSION, build: 'panel-test', role: 'guest', delay: INPUT_DELAY});
  peer({type: 'fleet', round: 0, ships: [2, 1]});
  return {panel, sent, socket, click, peer, onLeave};
}
it('does not tell the peer it is ready until game assets have actually loaded', async () => {
  let resolve!: () => void;
  const p = fixture(() => new Promise<void>(done => { resolve = done; }));
  p.click('ready'); expect(p.sent.some(packet => packet.type === 'ready')).toBe(false);
  expect(p.panel.element.querySelector<HTMLButtonElement>('[data-action="ready"]')!.disabled).toBe(true);
  resolve(); await Promise.resolve(); await Promise.resolve();
  expect(p.sent.filter(packet => packet.type === 'ready')).toHaveLength(1);
});
it('a failed preparation stays visible, permits retry and never readies the peer', async () => {
  const p = fixture(async () => { throw new Error('model missing'); }); p.click('ready');
  await Promise.resolve(); await Promise.resolve();
  expect(p.panel.element.textContent).toContain('assets could not load');
  expect(p.sent.some(packet => packet.type === 'ready')).toBe(false);
  expect(p.panel.element.querySelector<HTMLButtonElement>('[data-action="ready"]')!.disabled).toBe(false);
});
it('leaving while assets load cannot reopen the room or send a late ready', async () => {
  let resolve!: () => void;
  const p = fixture(() => new Promise<void>(done => { resolve = done; }));
  p.click('ready'); p.click('leave'); resolve(); await Promise.resolve(); await Promise.resolve();
  expect(p.panel.element.hidden).toBe(true); expect(p.onLeave).toHaveBeenCalledOnce();
  expect(p.sent.some(packet => packet.type === 'ready')).toBe(false);
});
it('fleet removal keeps one valid ship and communicates the visible selected order', () => {
  const p = fixture();
  for (const id of [0, 1, 2]) {
    const checkbox = p.panel.element.querySelector<HTMLInputElement>(`[data-ship="${id}"]`)!;
    checkbox.checked = false; checkbox.dispatchEvent(new Event('change', {bubbles: true}));
  }
  expect(p.panel.session!.state.localFleet).toEqual([2]);
  const fleets = p.sent.filter(packet => packet.type === 'fleet');
  expect(fleets[fleets.length - 1].ships).toEqual([2]);
  expect(p.panel.element.textContent).toContain('at least one ship');
});
