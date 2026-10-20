// Shared room-name cases for the browser client (netplay-lobby.test.cjs), the
// Node relay (tests/server/lobby.test.mjs) and the Cloudflare relay
// (tests/worker/lobby.test.mjs, tests/worker/lobby-object.test.mjs).
// Code points are written as numbers, so that no invisible character is in this file.
'use strict';
const u = (...points) => String.fromCodePoint(...points);

/** [input, the name as the room list shows it] */
const accepted = [
    ['Alpha', 'Alpha'],
    ['  Alpha  ', 'Alpha'],
    ['Red   Blue', 'Red Blue'],
    ['a' + u(0xa0, 0x3000) + 'b', 'a b'],
    ['א'.repeat(24), 'א'.repeat(24)],
    ['גמא', 'גמא'],
    // Hebrew with niqqud: a letter with 3 marks.
    ['ש' + u(0x5bc, 0x5c1, 0x5b8) + 'לום', 'ש' + u(0x5b8, 0x5bc, 0x5c1) + 'לום'],
    // NFC: e + combining acute is the single letter é.
    ['Caf' + 'e' + u(0x301), 'Caf' + u(0xe9)],
    ['a' + u(0x301, 0x302, 0x303), u(0xe1, 0x302, 0x303)],
    // Burmese and Tibetan use several marks in a row.
    [u(0x1001, 0x103b, 0x103d, 0x1031, 0x1038), u(0x1001, 0x103b, 0x103d, 0x1031, 0x1038)],
    [u(0xf56, 0xf66, 0xf92, 0xfb2, 0xf74, 0xf56, 0xf66), u(0xf56, 0xf66, 0xf92, 0xfb2, 0xf74, 0xf56, 0xf66)],
    [u(0x1f680).repeat(24), u(0x1f680).repeat(24)],
    [u(0x1f680) + ' Rocket', u(0x1f680) + ' Rocket'],
    // A keycap emoji: # + variation selector + enclosing keycap.
    ['#' + u(0xfe0f, 0x20e3), '#' + u(0xfe0f, 0x20e3)],
    ['...', '...'],
    ['1+1=2', '1+1=2'],
];

/** [label, input] */
const refused = [
    ['empty', ''],
    ['spaces', '    '],
    ['no-break spaces', u(0xa0, 0xa0)],
    ['25 characters', 'x'.repeat(25)],
    ['bell', 'bell' + u(0x7)],
    ['line break', 'new\nline'],
    ['tab', 'tab\there'],
    ['line separator', 'a' + u(0x2028) + 'b'],
    ['bidi override', 'evil' + u(0x202e) + 'exe'],
    ['zero-width space', 'zero' + u(0x200b) + 'width'],
    ['zero-width joiner', 'a' + u(0x200d) + 'b'],
    ['tag character', 'flag' + u(0xe0067)],
    ['Hangul filler', u(0x3164)],
    ['Hangul filler in a name', 'x' + u(0x3164) + 'y'],
    ['halfwidth Hangul filler', u(0xffa0)],
    ['Hangul choseong filler', u(0x115f)],
    ['Hangul jungseong filler', u(0x1160)],
    ['braille blank', u(0x2800)],
    ['private use', u(0xe000)],
    ['noncharacter FFFF', u(0xffff)],
    ['noncharacter FDD0', 'a' + u(0xfdd0)],
    ['unassigned', 'a' + u(0x378)],
    ['lone surrogate', 'a' + String.fromCharCode(0xd800)],
    ['marks only', u(0x301, 0x301)],
    ['4 stacked marks', 'a' + u(0x301, 0x302, 0x303, 0x304)],
    ['stacked marks', 'a' + u(0x301).repeat(23)],
];

module.exports = { accepted, refused };
