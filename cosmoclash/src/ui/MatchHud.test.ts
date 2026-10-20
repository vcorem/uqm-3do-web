import { afterEach, describe, expect, it } from 'vitest';
import { MatchHud } from './MatchHud';

afterEach(() => { document.body.innerHTML = ''; });
describe('battle text', () => {
  it('the battle message and its second line are one column (a long ship name pushes the second line down)', () => {
    const hud = new MatchHud(document.body, { exitLabel: 'Menu (Esc)', onExit() {} });
    const centre = document.querySelector<HTMLElement>('.match-centre')!;
    expect(centre.hidden).toBe(true);
    hud.message('Your MichiGichi was destroyed', 'Choose your next ship.');
    expect(centre.hidden).toBe(false);
    expect([...centre.children].map(node => [node.className, node.textContent])).toEqual([
      ['match-message', 'Your MichiGichi was destroyed'], ['match-message-sub', 'Choose your next ship.']]);
    hud.message('');
    expect(centre.hidden).toBe(true);
  });
});
