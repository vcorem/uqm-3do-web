/**
 * Room names, ported exactly from the UQM room client (UQMNetplay.roomName in
 * sc2/wasm/netplay-client.js) and the relays (server/netplay.mjs,
 * deploy/netplay.js): NFC, inner spaces collapsed to one, trimmed, 1 to 24
 * code points. Refused: control and format characters, line separators,
 * surrogates, private-use and unassigned code points, the blank fillers
 * U+115F, U+1160, U+2800, U+3164 and U+FFA0, and more than 3 stacked marks.
 * A name needs at least one letter, number, punctuation mark or symbol.
 */
export const ROOM_NAME_CHARS = 24;
export const ROOM_NAME_ERROR = `Room names need 1 to ${ROOM_NAME_CHARS} visible characters.`;
const NAME_FORBIDDEN = /[\p{Cc}\p{Cf}\p{Cs}\p{Co}\p{Cn}\p{Zl}\p{Zp}ᅟᅠ⠀ㅤﾠ]/u;
// Counted in the decomposed form, where a precomposed letter shows its marks.
const NAME_STACKED = /[\p{Mn}\p{Me}]{4}/u;
const NAME_VISIBLE = /[\p{L}\p{N}\p{P}\p{S}]/u;

/** The name as the room list shows it. Throws ROOM_NAME_ERROR, as the relay does. */
export function roomName(input: unknown): string {
  const text = String(input ?? '').normalize('NFC');
  // Before collapsing spaces: line breaks and tabs are control characters.
  const refused = NAME_FORBIDDEN.test(text) || NAME_STACKED.test(text.normalize('NFD'));
  const name = refused ? '' : text.replace(/\s+/g, ' ').trim();
  const length = Array.from(name).length;
  if (!length || length > ROOM_NAME_CHARS || !NAME_VISIBLE.test(name)) throw new Error(ROOM_NAME_ERROR);
  return name;
}

export interface CheckedName { ok: boolean; name: string; error?: string }

/** The same rule with the reason for the player (the name dialog). */
export function checkRoomName(value: unknown): CheckedName {
  const text = String(value ?? '').normalize('NFC');
  if (NAME_FORBIDDEN.test(text) || NAME_STACKED.test(text.normalize('NFD'))) {
    return { ok: false, name: text.trim(), error: 'Use only visible letters, numbers and symbols.' };
  }
  const name = text.replace(/\s+/g, ' ').trim();
  const length = Array.from(name).length;
  if (!length) return { ok: false, name, error: 'Type a name for your room.' };
  if (length > ROOM_NAME_CHARS) return { ok: false, name, error: `Use at most ${ROOM_NAME_CHARS} characters.` };
  if (!NAME_VISIBLE.test(name)) return { ok: false, name, error: 'Use at least one letter or number.' };
  return { ok: true, name };
}

/** A default room name, for example "Pilot 42". */
export function defaultRoomName(random: () => number = Math.random): string {
  return 'Pilot ' + (10 + Math.floor(random() * 90));
}

/** Text that starts with a right-to-left letter (Hebrew, Arabic, ...). */
export function isRtl(text: string): boolean {
  return /^[^\p{L}]*[\p{Script=Hebrew}\p{Script=Arabic}\p{Script=Syriac}\p{Script=Thaana}\p{Script=Nko}]/u.test(text);
}
