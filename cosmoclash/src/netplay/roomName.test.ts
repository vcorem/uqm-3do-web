// @vitest-environment node
import { describe, expect, it } from 'vitest';
import { createRequire } from 'node:module';
import { checkRoomName, defaultRoomName, isRtl, roomName, ROOM_NAME_ERROR } from './roomName';
// The shared cases of the UQM room client and both relays (read-only use).
const names = createRequire(import.meta.url)('../../../sc2/wasm/tests/room-names.cjs') as { accepted: [string, string][]; refused: [string, string][] };
// @ts-ignore The relay is JavaScript.
import { checkedName } from '../../../server/netplay.mjs';

describe('room names follow the UQM rule exactly', () => {
  it.each(names.accepted)('accepts %j as the listed name', (input, listed) => {
    expect(roomName(input)).toBe(listed);
    expect(checkRoomName(input)).toEqual({ ok: true, name: listed });
    expect(checkedName(input)).toBe(listed);
  });
  it.each(names.refused)('refuses %s', (_label, input) => {
    expect(() => roomName(input)).toThrow(ROOM_NAME_ERROR);
    const checked = checkRoomName(input);
    expect(checked.ok).toBe(false);
    expect(checked.error).toMatch(/^(Type a name|Use at most 24|Use at least one|Use only visible)/);
    expect(checkedName(input)).toBe(null);
  });
  it('gives the reason for the player', () => {
    expect(checkRoomName('   ').error).toBe('Type a name for your room.');
    expect(checkRoomName('x'.repeat(25)).error).toBe('Use at most 24 characters.');
    expect(checkRoomName('a​b').error).toBe('Use only visible letters, numbers and symbols.');
    expect(checkRoomName('́́').error).toBe('Use at least one letter or number.');
    expect(ROOM_NAME_ERROR).toBe('Room names need 1 to 24 visible characters.');
  });
  it('has a default name and knows right-to-left names', () => {
    expect(defaultRoomName(() => 0)).toBe('Pilot 10');
    expect(defaultRoomName(() => 0.999)).toBe('Pilot 99');
    expect(roomName(defaultRoomName())).toMatch(/^Pilot \d\d$/);
    expect(isRtl('קרב')).toBe(true);
    expect(isRtl('  1 קרב')).toBe(true);
    expect(isRtl('Duel')).toBe(false);
  });
});
