/*
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

// Rust translation of src/libs/input/sdl/vcontrol.c.
extern "C" {
    pub type _SDL_Joystick;
    pub type SDL_SysWMmsg;
    fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SDL_NumJoysticks() -> ::core::ffi::c_int;
    fn SDL_JoystickOpen(device_index: ::core::ffi::c_int) -> *mut SDL_Joystick;
    fn SDL_JoystickName(joystick: *mut SDL_Joystick) -> *const ::core::ffi::c_char;
    fn SDL_JoystickNumAxes(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
    fn SDL_JoystickNumHats(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
    fn SDL_JoystickNumButtons(joystick: *mut SDL_Joystick) -> ::core::ffi::c_int;
    fn SDL_JoystickClose(joystick: *mut SDL_Joystick);
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn VControl_code2name(code: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn VControl_name2code(code: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Sint32 = int32_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_Scancode(pub ::core::ffi::c_uint);
impl SDL_Scancode {
    pub const SDL_SCANCODE_UNKNOWN: Self = Self(0);
    pub const SDL_SCANCODE_A: Self = Self(4);
    pub const SDL_SCANCODE_B: Self = Self(5);
    pub const SDL_SCANCODE_C: Self = Self(6);
    pub const SDL_SCANCODE_D: Self = Self(7);
    pub const SDL_SCANCODE_E: Self = Self(8);
    pub const SDL_SCANCODE_F: Self = Self(9);
    pub const SDL_SCANCODE_G: Self = Self(10);
    pub const SDL_SCANCODE_H: Self = Self(11);
    pub const SDL_SCANCODE_I: Self = Self(12);
    pub const SDL_SCANCODE_J: Self = Self(13);
    pub const SDL_SCANCODE_K: Self = Self(14);
    pub const SDL_SCANCODE_L: Self = Self(15);
    pub const SDL_SCANCODE_M: Self = Self(16);
    pub const SDL_SCANCODE_N: Self = Self(17);
    pub const SDL_SCANCODE_O: Self = Self(18);
    pub const SDL_SCANCODE_P: Self = Self(19);
    pub const SDL_SCANCODE_Q: Self = Self(20);
    pub const SDL_SCANCODE_R: Self = Self(21);
    pub const SDL_SCANCODE_S: Self = Self(22);
    pub const SDL_SCANCODE_T: Self = Self(23);
    pub const SDL_SCANCODE_U: Self = Self(24);
    pub const SDL_SCANCODE_V: Self = Self(25);
    pub const SDL_SCANCODE_W: Self = Self(26);
    pub const SDL_SCANCODE_X: Self = Self(27);
    pub const SDL_SCANCODE_Y: Self = Self(28);
    pub const SDL_SCANCODE_Z: Self = Self(29);
    pub const SDL_SCANCODE_1: Self = Self(30);
    pub const SDL_SCANCODE_2: Self = Self(31);
    pub const SDL_SCANCODE_3: Self = Self(32);
    pub const SDL_SCANCODE_4: Self = Self(33);
    pub const SDL_SCANCODE_5: Self = Self(34);
    pub const SDL_SCANCODE_6: Self = Self(35);
    pub const SDL_SCANCODE_7: Self = Self(36);
    pub const SDL_SCANCODE_8: Self = Self(37);
    pub const SDL_SCANCODE_9: Self = Self(38);
    pub const SDL_SCANCODE_0: Self = Self(39);
    pub const SDL_SCANCODE_RETURN: Self = Self(40);
    pub const SDL_SCANCODE_ESCAPE: Self = Self(41);
    pub const SDL_SCANCODE_BACKSPACE: Self = Self(42);
    pub const SDL_SCANCODE_TAB: Self = Self(43);
    pub const SDL_SCANCODE_SPACE: Self = Self(44);
    pub const SDL_SCANCODE_MINUS: Self = Self(45);
    pub const SDL_SCANCODE_EQUALS: Self = Self(46);
    pub const SDL_SCANCODE_LEFTBRACKET: Self = Self(47);
    pub const SDL_SCANCODE_RIGHTBRACKET: Self = Self(48);
    pub const SDL_SCANCODE_BACKSLASH: Self = Self(49);
    pub const SDL_SCANCODE_NONUSHASH: Self = Self(50);
    pub const SDL_SCANCODE_SEMICOLON: Self = Self(51);
    pub const SDL_SCANCODE_APOSTROPHE: Self = Self(52);
    pub const SDL_SCANCODE_GRAVE: Self = Self(53);
    pub const SDL_SCANCODE_COMMA: Self = Self(54);
    pub const SDL_SCANCODE_PERIOD: Self = Self(55);
    pub const SDL_SCANCODE_SLASH: Self = Self(56);
    pub const SDL_SCANCODE_CAPSLOCK: Self = Self(57);
    pub const SDL_SCANCODE_F1: Self = Self(58);
    pub const SDL_SCANCODE_F2: Self = Self(59);
    pub const SDL_SCANCODE_F3: Self = Self(60);
    pub const SDL_SCANCODE_F4: Self = Self(61);
    pub const SDL_SCANCODE_F5: Self = Self(62);
    pub const SDL_SCANCODE_F6: Self = Self(63);
    pub const SDL_SCANCODE_F7: Self = Self(64);
    pub const SDL_SCANCODE_F8: Self = Self(65);
    pub const SDL_SCANCODE_F9: Self = Self(66);
    pub const SDL_SCANCODE_F10: Self = Self(67);
    pub const SDL_SCANCODE_F11: Self = Self(68);
    pub const SDL_SCANCODE_F12: Self = Self(69);
    pub const SDL_SCANCODE_PRINTSCREEN: Self = Self(70);
    pub const SDL_SCANCODE_SCROLLLOCK: Self = Self(71);
    pub const SDL_SCANCODE_PAUSE: Self = Self(72);
    pub const SDL_SCANCODE_INSERT: Self = Self(73);
    pub const SDL_SCANCODE_HOME: Self = Self(74);
    pub const SDL_SCANCODE_PAGEUP: Self = Self(75);
    pub const SDL_SCANCODE_DELETE: Self = Self(76);
    pub const SDL_SCANCODE_END: Self = Self(77);
    pub const SDL_SCANCODE_PAGEDOWN: Self = Self(78);
    pub const SDL_SCANCODE_RIGHT: Self = Self(79);
    pub const SDL_SCANCODE_LEFT: Self = Self(80);
    pub const SDL_SCANCODE_DOWN: Self = Self(81);
    pub const SDL_SCANCODE_UP: Self = Self(82);
    pub const SDL_SCANCODE_NUMLOCKCLEAR: Self = Self(83);
    pub const SDL_SCANCODE_KP_DIVIDE: Self = Self(84);
    pub const SDL_SCANCODE_KP_MULTIPLY: Self = Self(85);
    pub const SDL_SCANCODE_KP_MINUS: Self = Self(86);
    pub const SDL_SCANCODE_KP_PLUS: Self = Self(87);
    pub const SDL_SCANCODE_KP_ENTER: Self = Self(88);
    pub const SDL_SCANCODE_KP_1: Self = Self(89);
    pub const SDL_SCANCODE_KP_2: Self = Self(90);
    pub const SDL_SCANCODE_KP_3: Self = Self(91);
    pub const SDL_SCANCODE_KP_4: Self = Self(92);
    pub const SDL_SCANCODE_KP_5: Self = Self(93);
    pub const SDL_SCANCODE_KP_6: Self = Self(94);
    pub const SDL_SCANCODE_KP_7: Self = Self(95);
    pub const SDL_SCANCODE_KP_8: Self = Self(96);
    pub const SDL_SCANCODE_KP_9: Self = Self(97);
    pub const SDL_SCANCODE_KP_0: Self = Self(98);
    pub const SDL_SCANCODE_KP_PERIOD: Self = Self(99);
    pub const SDL_SCANCODE_NONUSBACKSLASH: Self = Self(100);
    pub const SDL_SCANCODE_APPLICATION: Self = Self(101);
    pub const SDL_SCANCODE_POWER: Self = Self(102);
    pub const SDL_SCANCODE_KP_EQUALS: Self = Self(103);
    pub const SDL_SCANCODE_F13: Self = Self(104);
    pub const SDL_SCANCODE_F14: Self = Self(105);
    pub const SDL_SCANCODE_F15: Self = Self(106);
    pub const SDL_SCANCODE_F16: Self = Self(107);
    pub const SDL_SCANCODE_F17: Self = Self(108);
    pub const SDL_SCANCODE_F18: Self = Self(109);
    pub const SDL_SCANCODE_F19: Self = Self(110);
    pub const SDL_SCANCODE_F20: Self = Self(111);
    pub const SDL_SCANCODE_F21: Self = Self(112);
    pub const SDL_SCANCODE_F22: Self = Self(113);
    pub const SDL_SCANCODE_F23: Self = Self(114);
    pub const SDL_SCANCODE_F24: Self = Self(115);
    pub const SDL_SCANCODE_EXECUTE: Self = Self(116);
    pub const SDL_SCANCODE_HELP: Self = Self(117);
    pub const SDL_SCANCODE_MENU: Self = Self(118);
    pub const SDL_SCANCODE_SELECT: Self = Self(119);
    pub const SDL_SCANCODE_STOP: Self = Self(120);
    pub const SDL_SCANCODE_AGAIN: Self = Self(121);
    pub const SDL_SCANCODE_UNDO: Self = Self(122);
    pub const SDL_SCANCODE_CUT: Self = Self(123);
    pub const SDL_SCANCODE_COPY: Self = Self(124);
    pub const SDL_SCANCODE_PASTE: Self = Self(125);
    pub const SDL_SCANCODE_FIND: Self = Self(126);
    pub const SDL_SCANCODE_MUTE: Self = Self(127);
    pub const SDL_SCANCODE_VOLUMEUP: Self = Self(128);
    pub const SDL_SCANCODE_VOLUMEDOWN: Self = Self(129);
    pub const SDL_SCANCODE_KP_COMMA: Self = Self(133);
    pub const SDL_SCANCODE_KP_EQUALSAS400: Self = Self(134);
    pub const SDL_SCANCODE_INTERNATIONAL1: Self = Self(135);
    pub const SDL_SCANCODE_INTERNATIONAL2: Self = Self(136);
    pub const SDL_SCANCODE_INTERNATIONAL3: Self = Self(137);
    pub const SDL_SCANCODE_INTERNATIONAL4: Self = Self(138);
    pub const SDL_SCANCODE_INTERNATIONAL5: Self = Self(139);
    pub const SDL_SCANCODE_INTERNATIONAL6: Self = Self(140);
    pub const SDL_SCANCODE_INTERNATIONAL7: Self = Self(141);
    pub const SDL_SCANCODE_INTERNATIONAL8: Self = Self(142);
    pub const SDL_SCANCODE_INTERNATIONAL9: Self = Self(143);
    pub const SDL_SCANCODE_LANG1: Self = Self(144);
    pub const SDL_SCANCODE_LANG2: Self = Self(145);
    pub const SDL_SCANCODE_LANG3: Self = Self(146);
    pub const SDL_SCANCODE_LANG4: Self = Self(147);
    pub const SDL_SCANCODE_LANG5: Self = Self(148);
    pub const SDL_SCANCODE_LANG6: Self = Self(149);
    pub const SDL_SCANCODE_LANG7: Self = Self(150);
    pub const SDL_SCANCODE_LANG8: Self = Self(151);
    pub const SDL_SCANCODE_LANG9: Self = Self(152);
    pub const SDL_SCANCODE_ALTERASE: Self = Self(153);
    pub const SDL_SCANCODE_SYSREQ: Self = Self(154);
    pub const SDL_SCANCODE_CANCEL: Self = Self(155);
    pub const SDL_SCANCODE_CLEAR: Self = Self(156);
    pub const SDL_SCANCODE_PRIOR: Self = Self(157);
    pub const SDL_SCANCODE_RETURN2: Self = Self(158);
    pub const SDL_SCANCODE_SEPARATOR: Self = Self(159);
    pub const SDL_SCANCODE_OUT: Self = Self(160);
    pub const SDL_SCANCODE_OPER: Self = Self(161);
    pub const SDL_SCANCODE_CLEARAGAIN: Self = Self(162);
    pub const SDL_SCANCODE_CRSEL: Self = Self(163);
    pub const SDL_SCANCODE_EXSEL: Self = Self(164);
    pub const SDL_SCANCODE_KP_00: Self = Self(176);
    pub const SDL_SCANCODE_KP_000: Self = Self(177);
    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: Self = Self(178);
    pub const SDL_SCANCODE_DECIMALSEPARATOR: Self = Self(179);
    pub const SDL_SCANCODE_CURRENCYUNIT: Self = Self(180);
    pub const SDL_SCANCODE_CURRENCYSUBUNIT: Self = Self(181);
    pub const SDL_SCANCODE_KP_LEFTPAREN: Self = Self(182);
    pub const SDL_SCANCODE_KP_RIGHTPAREN: Self = Self(183);
    pub const SDL_SCANCODE_KP_LEFTBRACE: Self = Self(184);
    pub const SDL_SCANCODE_KP_RIGHTBRACE: Self = Self(185);
    pub const SDL_SCANCODE_KP_TAB: Self = Self(186);
    pub const SDL_SCANCODE_KP_BACKSPACE: Self = Self(187);
    pub const SDL_SCANCODE_KP_A: Self = Self(188);
    pub const SDL_SCANCODE_KP_B: Self = Self(189);
    pub const SDL_SCANCODE_KP_C: Self = Self(190);
    pub const SDL_SCANCODE_KP_D: Self = Self(191);
    pub const SDL_SCANCODE_KP_E: Self = Self(192);
    pub const SDL_SCANCODE_KP_F: Self = Self(193);
    pub const SDL_SCANCODE_KP_XOR: Self = Self(194);
    pub const SDL_SCANCODE_KP_POWER: Self = Self(195);
    pub const SDL_SCANCODE_KP_PERCENT: Self = Self(196);
    pub const SDL_SCANCODE_KP_LESS: Self = Self(197);
    pub const SDL_SCANCODE_KP_GREATER: Self = Self(198);
    pub const SDL_SCANCODE_KP_AMPERSAND: Self = Self(199);
    pub const SDL_SCANCODE_KP_DBLAMPERSAND: Self = Self(200);
    pub const SDL_SCANCODE_KP_VERTICALBAR: Self = Self(201);
    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: Self = Self(202);
    pub const SDL_SCANCODE_KP_COLON: Self = Self(203);
    pub const SDL_SCANCODE_KP_HASH: Self = Self(204);
    pub const SDL_SCANCODE_KP_SPACE: Self = Self(205);
    pub const SDL_SCANCODE_KP_AT: Self = Self(206);
    pub const SDL_SCANCODE_KP_EXCLAM: Self = Self(207);
    pub const SDL_SCANCODE_KP_MEMSTORE: Self = Self(208);
    pub const SDL_SCANCODE_KP_MEMRECALL: Self = Self(209);
    pub const SDL_SCANCODE_KP_MEMCLEAR: Self = Self(210);
    pub const SDL_SCANCODE_KP_MEMADD: Self = Self(211);
    pub const SDL_SCANCODE_KP_MEMSUBTRACT: Self = Self(212);
    pub const SDL_SCANCODE_KP_MEMMULTIPLY: Self = Self(213);
    pub const SDL_SCANCODE_KP_MEMDIVIDE: Self = Self(214);
    pub const SDL_SCANCODE_KP_PLUSMINUS: Self = Self(215);
    pub const SDL_SCANCODE_KP_CLEAR: Self = Self(216);
    pub const SDL_SCANCODE_KP_CLEARENTRY: Self = Self(217);
    pub const SDL_SCANCODE_KP_BINARY: Self = Self(218);
    pub const SDL_SCANCODE_KP_OCTAL: Self = Self(219);
    pub const SDL_SCANCODE_KP_DECIMAL: Self = Self(220);
    pub const SDL_SCANCODE_KP_HEXADECIMAL: Self = Self(221);
    pub const SDL_SCANCODE_LCTRL: Self = Self(224);
    pub const SDL_SCANCODE_LSHIFT: Self = Self(225);
    pub const SDL_SCANCODE_LALT: Self = Self(226);
    pub const SDL_SCANCODE_LGUI: Self = Self(227);
    pub const SDL_SCANCODE_RCTRL: Self = Self(228);
    pub const SDL_SCANCODE_RSHIFT: Self = Self(229);
    pub const SDL_SCANCODE_RALT: Self = Self(230);
    pub const SDL_SCANCODE_RGUI: Self = Self(231);
    pub const SDL_SCANCODE_MODE: Self = Self(257);
    pub const SDL_SCANCODE_AUDIONEXT: Self = Self(258);
    pub const SDL_SCANCODE_AUDIOPREV: Self = Self(259);
    pub const SDL_SCANCODE_AUDIOSTOP: Self = Self(260);
    pub const SDL_SCANCODE_AUDIOPLAY: Self = Self(261);
    pub const SDL_SCANCODE_AUDIOMUTE: Self = Self(262);
    pub const SDL_SCANCODE_MEDIASELECT: Self = Self(263);
    pub const SDL_SCANCODE_WWW: Self = Self(264);
    pub const SDL_SCANCODE_MAIL: Self = Self(265);
    pub const SDL_SCANCODE_CALCULATOR: Self = Self(266);
    pub const SDL_SCANCODE_COMPUTER: Self = Self(267);
    pub const SDL_SCANCODE_AC_SEARCH: Self = Self(268);
    pub const SDL_SCANCODE_AC_HOME: Self = Self(269);
    pub const SDL_SCANCODE_AC_BACK: Self = Self(270);
    pub const SDL_SCANCODE_AC_FORWARD: Self = Self(271);
    pub const SDL_SCANCODE_AC_STOP: Self = Self(272);
    pub const SDL_SCANCODE_AC_REFRESH: Self = Self(273);
    pub const SDL_SCANCODE_AC_BOOKMARKS: Self = Self(274);
    pub const SDL_SCANCODE_BRIGHTNESSDOWN: Self = Self(275);
    pub const SDL_SCANCODE_BRIGHTNESSUP: Self = Self(276);
    pub const SDL_SCANCODE_DISPLAYSWITCH: Self = Self(277);
    pub const SDL_SCANCODE_KBDILLUMTOGGLE: Self = Self(278);
    pub const SDL_SCANCODE_KBDILLUMDOWN: Self = Self(279);
    pub const SDL_SCANCODE_KBDILLUMUP: Self = Self(280);
    pub const SDL_SCANCODE_EJECT: Self = Self(281);
    pub const SDL_SCANCODE_SLEEP: Self = Self(282);
    pub const SDL_SCANCODE_APP1: Self = Self(283);
    pub const SDL_SCANCODE_APP2: Self = Self(284);
    pub const SDL_SCANCODE_AUDIOREWIND: Self = Self(285);
    pub const SDL_SCANCODE_AUDIOFASTFORWARD: Self = Self(286);
    pub const SDL_SCANCODE_SOFTLEFT: Self = Self(287);
    pub const SDL_SCANCODE_SOFTRIGHT: Self = Self(288);
    pub const SDL_SCANCODE_CALL: Self = Self(289);
    pub const SDL_SCANCODE_ENDCALL: Self = Self(290);
    pub const SDL_NUM_SCANCODES: Self = Self(512);
}
pub type SDL_Keycode = Sint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SDLK_UNKNOWN: Self = Self(0);
    pub const SDLK_RETURN: Self = Self(13);
    pub const SDLK_ESCAPE: Self = Self(27);
    pub const SDLK_BACKSPACE: Self = Self(8);
    pub const SDLK_TAB: Self = Self(9);
    pub const SDLK_SPACE: Self = Self(32);
    pub const SDLK_EXCLAIM: Self = Self(33);
    pub const SDLK_QUOTEDBL: Self = Self(34);
    pub const SDLK_HASH: Self = Self(35);
    pub const SDLK_PERCENT: Self = Self(37);
    pub const SDLK_DOLLAR: Self = Self(36);
    pub const SDLK_AMPERSAND: Self = Self(38);
    pub const SDLK_QUOTE: Self = Self(39);
    pub const SDLK_LEFTPAREN: Self = Self(40);
    pub const SDLK_RIGHTPAREN: Self = Self(41);
    pub const SDLK_ASTERISK: Self = Self(42);
    pub const SDLK_PLUS: Self = Self(43);
    pub const SDLK_COMMA: Self = Self(44);
    pub const SDLK_MINUS: Self = Self(45);
    pub const SDLK_PERIOD: Self = Self(46);
    pub const SDLK_SLASH: Self = Self(47);
    pub const SDLK_0: Self = Self(48);
    pub const SDLK_1: Self = Self(49);
    pub const SDLK_2: Self = Self(50);
    pub const SDLK_3: Self = Self(51);
    pub const SDLK_4: Self = Self(52);
    pub const SDLK_5: Self = Self(53);
    pub const SDLK_6: Self = Self(54);
    pub const SDLK_7: Self = Self(55);
    pub const SDLK_8: Self = Self(56);
    pub const SDLK_9: Self = Self(57);
    pub const SDLK_COLON: Self = Self(58);
    pub const SDLK_SEMICOLON: Self = Self(59);
    pub const SDLK_LESS: Self = Self(60);
    pub const SDLK_EQUALS: Self = Self(61);
    pub const SDLK_GREATER: Self = Self(62);
    pub const SDLK_QUESTION: Self = Self(63);
    pub const SDLK_AT: Self = Self(64);
    pub const SDLK_LEFTBRACKET: Self = Self(91);
    pub const SDLK_BACKSLASH: Self = Self(92);
    pub const SDLK_RIGHTBRACKET: Self = Self(93);
    pub const SDLK_CARET: Self = Self(94);
    pub const SDLK_UNDERSCORE: Self = Self(95);
    pub const SDLK_BACKQUOTE: Self = Self(96);
    pub const SDLK_a: Self = Self(97);
    pub const SDLK_b: Self = Self(98);
    pub const SDLK_c: Self = Self(99);
    pub const SDLK_d: Self = Self(100);
    pub const SDLK_e: Self = Self(101);
    pub const SDLK_f: Self = Self(102);
    pub const SDLK_g: Self = Self(103);
    pub const SDLK_h: Self = Self(104);
    pub const SDLK_i: Self = Self(105);
    pub const SDLK_j: Self = Self(106);
    pub const SDLK_k: Self = Self(107);
    pub const SDLK_l: Self = Self(108);
    pub const SDLK_m: Self = Self(109);
    pub const SDLK_n: Self = Self(110);
    pub const SDLK_o: Self = Self(111);
    pub const SDLK_p: Self = Self(112);
    pub const SDLK_q: Self = Self(113);
    pub const SDLK_r: Self = Self(114);
    pub const SDLK_s: Self = Self(115);
    pub const SDLK_t: Self = Self(116);
    pub const SDLK_u: Self = Self(117);
    pub const SDLK_v: Self = Self(118);
    pub const SDLK_w: Self = Self(119);
    pub const SDLK_x: Self = Self(120);
    pub const SDLK_y: Self = Self(121);
    pub const SDLK_z: Self = Self(122);
    pub const SDLK_CAPSLOCK: Self = Self(1073741881);
    pub const SDLK_F1: Self = Self(1073741882);
    pub const SDLK_F2: Self = Self(1073741883);
    pub const SDLK_F3: Self = Self(1073741884);
    pub const SDLK_F4: Self = Self(1073741885);
    pub const SDLK_F5: Self = Self(1073741886);
    pub const SDLK_F6: Self = Self(1073741887);
    pub const SDLK_F7: Self = Self(1073741888);
    pub const SDLK_F8: Self = Self(1073741889);
    pub const SDLK_F9: Self = Self(1073741890);
    pub const SDLK_F10: Self = Self(1073741891);
    pub const SDLK_F11: Self = Self(1073741892);
    pub const SDLK_F12: Self = Self(1073741893);
    pub const SDLK_PRINTSCREEN: Self = Self(1073741894);
    pub const SDLK_SCROLLLOCK: Self = Self(1073741895);
    pub const SDLK_PAUSE: Self = Self(1073741896);
    pub const SDLK_INSERT: Self = Self(1073741897);
    pub const SDLK_HOME: Self = Self(1073741898);
    pub const SDLK_PAGEUP: Self = Self(1073741899);
    pub const SDLK_DELETE: Self = Self(127);
    pub const SDLK_END: Self = Self(1073741901);
    pub const SDLK_PAGEDOWN: Self = Self(1073741902);
    pub const SDLK_RIGHT: Self = Self(1073741903);
    pub const SDLK_LEFT: Self = Self(1073741904);
    pub const SDLK_DOWN: Self = Self(1073741905);
    pub const SDLK_UP: Self = Self(1073741906);
    pub const SDLK_NUMLOCKCLEAR: Self = Self(1073741907);
    pub const SDLK_KP_DIVIDE: Self = Self(1073741908);
    pub const SDLK_KP_MULTIPLY: Self = Self(1073741909);
    pub const SDLK_KP_MINUS: Self = Self(1073741910);
    pub const SDLK_KP_PLUS: Self = Self(1073741911);
    pub const SDLK_KP_ENTER: Self = Self(1073741912);
    pub const SDLK_KP_1: Self = Self(1073741913);
    pub const SDLK_KP_2: Self = Self(1073741914);
    pub const SDLK_KP_3: Self = Self(1073741915);
    pub const SDLK_KP_4: Self = Self(1073741916);
    pub const SDLK_KP_5: Self = Self(1073741917);
    pub const SDLK_KP_6: Self = Self(1073741918);
    pub const SDLK_KP_7: Self = Self(1073741919);
    pub const SDLK_KP_8: Self = Self(1073741920);
    pub const SDLK_KP_9: Self = Self(1073741921);
    pub const SDLK_KP_0: Self = Self(1073741922);
    pub const SDLK_KP_PERIOD: Self = Self(1073741923);
    pub const SDLK_APPLICATION: Self = Self(1073741925);
    pub const SDLK_POWER: Self = Self(1073741926);
    pub const SDLK_KP_EQUALS: Self = Self(1073741927);
    pub const SDLK_F13: Self = Self(1073741928);
    pub const SDLK_F14: Self = Self(1073741929);
    pub const SDLK_F15: Self = Self(1073741930);
    pub const SDLK_F16: Self = Self(1073741931);
    pub const SDLK_F17: Self = Self(1073741932);
    pub const SDLK_F18: Self = Self(1073741933);
    pub const SDLK_F19: Self = Self(1073741934);
    pub const SDLK_F20: Self = Self(1073741935);
    pub const SDLK_F21: Self = Self(1073741936);
    pub const SDLK_F22: Self = Self(1073741937);
    pub const SDLK_F23: Self = Self(1073741938);
    pub const SDLK_F24: Self = Self(1073741939);
    pub const SDLK_EXECUTE: Self = Self(1073741940);
    pub const SDLK_HELP: Self = Self(1073741941);
    pub const SDLK_MENU: Self = Self(1073741942);
    pub const SDLK_SELECT: Self = Self(1073741943);
    pub const SDLK_STOP: Self = Self(1073741944);
    pub const SDLK_AGAIN: Self = Self(1073741945);
    pub const SDLK_UNDO: Self = Self(1073741946);
    pub const SDLK_CUT: Self = Self(1073741947);
    pub const SDLK_COPY: Self = Self(1073741948);
    pub const SDLK_PASTE: Self = Self(1073741949);
    pub const SDLK_FIND: Self = Self(1073741950);
    pub const SDLK_MUTE: Self = Self(1073741951);
    pub const SDLK_VOLUMEUP: Self = Self(1073741952);
    pub const SDLK_VOLUMEDOWN: Self = Self(1073741953);
    pub const SDLK_KP_COMMA: Self = Self(1073741957);
    pub const SDLK_KP_EQUALSAS400: Self = Self(1073741958);
    pub const SDLK_ALTERASE: Self = Self(1073741977);
    pub const SDLK_SYSREQ: Self = Self(1073741978);
    pub const SDLK_CANCEL: Self = Self(1073741979);
    pub const SDLK_CLEAR: Self = Self(1073741980);
    pub const SDLK_PRIOR: Self = Self(1073741981);
    pub const SDLK_RETURN2: Self = Self(1073741982);
    pub const SDLK_SEPARATOR: Self = Self(1073741983);
    pub const SDLK_OUT: Self = Self(1073741984);
    pub const SDLK_OPER: Self = Self(1073741985);
    pub const SDLK_CLEARAGAIN: Self = Self(1073741986);
    pub const SDLK_CRSEL: Self = Self(1073741987);
    pub const SDLK_EXSEL: Self = Self(1073741988);
    pub const SDLK_KP_00: Self = Self(1073742000);
    pub const SDLK_KP_000: Self = Self(1073742001);
    pub const SDLK_THOUSANDSSEPARATOR: Self = Self(1073742002);
    pub const SDLK_DECIMALSEPARATOR: Self = Self(1073742003);
    pub const SDLK_CURRENCYUNIT: Self = Self(1073742004);
    pub const SDLK_CURRENCYSUBUNIT: Self = Self(1073742005);
    pub const SDLK_KP_LEFTPAREN: Self = Self(1073742006);
    pub const SDLK_KP_RIGHTPAREN: Self = Self(1073742007);
    pub const SDLK_KP_LEFTBRACE: Self = Self(1073742008);
    pub const SDLK_KP_RIGHTBRACE: Self = Self(1073742009);
    pub const SDLK_KP_TAB: Self = Self(1073742010);
    pub const SDLK_KP_BACKSPACE: Self = Self(1073742011);
    pub const SDLK_KP_A: Self = Self(1073742012);
    pub const SDLK_KP_B: Self = Self(1073742013);
    pub const SDLK_KP_C: Self = Self(1073742014);
    pub const SDLK_KP_D: Self = Self(1073742015);
    pub const SDLK_KP_E: Self = Self(1073742016);
    pub const SDLK_KP_F: Self = Self(1073742017);
    pub const SDLK_KP_XOR: Self = Self(1073742018);
    pub const SDLK_KP_POWER: Self = Self(1073742019);
    pub const SDLK_KP_PERCENT: Self = Self(1073742020);
    pub const SDLK_KP_LESS: Self = Self(1073742021);
    pub const SDLK_KP_GREATER: Self = Self(1073742022);
    pub const SDLK_KP_AMPERSAND: Self = Self(1073742023);
    pub const SDLK_KP_DBLAMPERSAND: Self = Self(1073742024);
    pub const SDLK_KP_VERTICALBAR: Self = Self(1073742025);
    pub const SDLK_KP_DBLVERTICALBAR: Self = Self(1073742026);
    pub const SDLK_KP_COLON: Self = Self(1073742027);
    pub const SDLK_KP_HASH: Self = Self(1073742028);
    pub const SDLK_KP_SPACE: Self = Self(1073742029);
    pub const SDLK_KP_AT: Self = Self(1073742030);
    pub const SDLK_KP_EXCLAM: Self = Self(1073742031);
    pub const SDLK_KP_MEMSTORE: Self = Self(1073742032);
    pub const SDLK_KP_MEMRECALL: Self = Self(1073742033);
    pub const SDLK_KP_MEMCLEAR: Self = Self(1073742034);
    pub const SDLK_KP_MEMADD: Self = Self(1073742035);
    pub const SDLK_KP_MEMSUBTRACT: Self = Self(1073742036);
    pub const SDLK_KP_MEMMULTIPLY: Self = Self(1073742037);
    pub const SDLK_KP_MEMDIVIDE: Self = Self(1073742038);
    pub const SDLK_KP_PLUSMINUS: Self = Self(1073742039);
    pub const SDLK_KP_CLEAR: Self = Self(1073742040);
    pub const SDLK_KP_CLEARENTRY: Self = Self(1073742041);
    pub const SDLK_KP_BINARY: Self = Self(1073742042);
    pub const SDLK_KP_OCTAL: Self = Self(1073742043);
    pub const SDLK_KP_DECIMAL: Self = Self(1073742044);
    pub const SDLK_KP_HEXADECIMAL: Self = Self(1073742045);
    pub const SDLK_LCTRL: Self = Self(1073742048);
    pub const SDLK_LSHIFT: Self = Self(1073742049);
    pub const SDLK_LALT: Self = Self(1073742050);
    pub const SDLK_LGUI: Self = Self(1073742051);
    pub const SDLK_RCTRL: Self = Self(1073742052);
    pub const SDLK_RSHIFT: Self = Self(1073742053);
    pub const SDLK_RALT: Self = Self(1073742054);
    pub const SDLK_RGUI: Self = Self(1073742055);
    pub const SDLK_MODE: Self = Self(1073742081);
    pub const SDLK_AUDIONEXT: Self = Self(1073742082);
    pub const SDLK_AUDIOPREV: Self = Self(1073742083);
    pub const SDLK_AUDIOSTOP: Self = Self(1073742084);
    pub const SDLK_AUDIOPLAY: Self = Self(1073742085);
    pub const SDLK_AUDIOMUTE: Self = Self(1073742086);
    pub const SDLK_MEDIASELECT: Self = Self(1073742087);
    pub const SDLK_WWW: Self = Self(1073742088);
    pub const SDLK_MAIL: Self = Self(1073742089);
    pub const SDLK_CALCULATOR: Self = Self(1073742090);
    pub const SDLK_COMPUTER: Self = Self(1073742091);
    pub const SDLK_AC_SEARCH: Self = Self(1073742092);
    pub const SDLK_AC_HOME: Self = Self(1073742093);
    pub const SDLK_AC_BACK: Self = Self(1073742094);
    pub const SDLK_AC_FORWARD: Self = Self(1073742095);
    pub const SDLK_AC_STOP: Self = Self(1073742096);
    pub const SDLK_AC_REFRESH: Self = Self(1073742097);
    pub const SDLK_AC_BOOKMARKS: Self = Self(1073742098);
    pub const SDLK_BRIGHTNESSDOWN: Self = Self(1073742099);
    pub const SDLK_BRIGHTNESSUP: Self = Self(1073742100);
    pub const SDLK_DISPLAYSWITCH: Self = Self(1073742101);
    pub const SDLK_KBDILLUMTOGGLE: Self = Self(1073742102);
    pub const SDLK_KBDILLUMDOWN: Self = Self(1073742103);
    pub const SDLK_KBDILLUMUP: Self = Self(1073742104);
    pub const SDLK_EJECT: Self = Self(1073742105);
    pub const SDLK_SLEEP: Self = Self(1073742106);
    pub const SDLK_APP1: Self = Self(1073742107);
    pub const SDLK_APP2: Self = Self(1073742108);
    pub const SDLK_AUDIOREWIND: Self = Self(1073742109);
    pub const SDLK_AUDIOFASTFORWARD: Self = Self(1073742110);
    pub const SDLK_SOFTLEFT: Self = Self(1073742111);
    pub const SDLK_SOFTRIGHT: Self = Self(1073742112);
    pub const SDLK_CALL: Self = Self(1073742113);
    pub const SDLK_ENDCALL: Self = Self(1073742114);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub r#mod: Uint16,
    pub unused: Uint32,
}
pub type SDL_Joystick = _SDL_Joystick;
pub type SDL_JoystickID = Sint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_JoystickPowerLevel(pub ::core::ffi::c_int);
impl SDL_JoystickPowerLevel {
    pub const SDL_JOYSTICK_POWER_UNKNOWN: Self = Self(-1);
    pub const SDL_JOYSTICK_POWER_EMPTY: Self = Self(0);
    pub const SDL_JOYSTICK_POWER_LOW: Self = Self(1);
    pub const SDL_JOYSTICK_POWER_MEDIUM: Self = Self(2);
    pub const SDL_JOYSTICK_POWER_FULL: Self = Self(3);
    pub const SDL_JOYSTICK_POWER_WIRED: Self = Self(4);
    pub const SDL_JOYSTICK_POWER_MAX: Self = Self(5);
}
pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;
pub type SDL_GestureID = Sint64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SDL_FIRSTEVENT: Self = Self(0);
    pub const SDL_QUIT: Self = Self(256);
    pub const SDL_APP_TERMINATING: Self = Self(257);
    pub const SDL_APP_LOWMEMORY: Self = Self(258);
    pub const SDL_APP_WILLENTERBACKGROUND: Self = Self(259);
    pub const SDL_APP_DIDENTERBACKGROUND: Self = Self(260);
    pub const SDL_APP_WILLENTERFOREGROUND: Self = Self(261);
    pub const SDL_APP_DIDENTERFOREGROUND: Self = Self(262);
    pub const SDL_LOCALECHANGED: Self = Self(263);
    pub const SDL_DISPLAYEVENT: Self = Self(336);
    pub const SDL_WINDOWEVENT: Self = Self(512);
    pub const SDL_SYSWMEVENT: Self = Self(513);
    pub const SDL_KEYDOWN: Self = Self(768);
    pub const SDL_KEYUP: Self = Self(769);
    pub const SDL_TEXTEDITING: Self = Self(770);
    pub const SDL_TEXTINPUT: Self = Self(771);
    pub const SDL_KEYMAPCHANGED: Self = Self(772);
    pub const SDL_TEXTEDITING_EXT: Self = Self(773);
    pub const SDL_MOUSEMOTION: Self = Self(1024);
    pub const SDL_MOUSEBUTTONDOWN: Self = Self(1025);
    pub const SDL_MOUSEBUTTONUP: Self = Self(1026);
    pub const SDL_MOUSEWHEEL: Self = Self(1027);
    pub const SDL_JOYAXISMOTION: Self = Self(1536);
    pub const SDL_JOYBALLMOTION: Self = Self(1537);
    pub const SDL_JOYHATMOTION: Self = Self(1538);
    pub const SDL_JOYBUTTONDOWN: Self = Self(1539);
    pub const SDL_JOYBUTTONUP: Self = Self(1540);
    pub const SDL_JOYDEVICEADDED: Self = Self(1541);
    pub const SDL_JOYDEVICEREMOVED: Self = Self(1542);
    pub const SDL_JOYBATTERYUPDATED: Self = Self(1543);
    pub const SDL_CONTROLLERAXISMOTION: Self = Self(1616);
    pub const SDL_CONTROLLERBUTTONDOWN: Self = Self(1617);
    pub const SDL_CONTROLLERBUTTONUP: Self = Self(1618);
    pub const SDL_CONTROLLERDEVICEADDED: Self = Self(1619);
    pub const SDL_CONTROLLERDEVICEREMOVED: Self = Self(1620);
    pub const SDL_CONTROLLERDEVICEREMAPPED: Self = Self(1621);
    pub const SDL_CONTROLLERTOUCHPADDOWN: Self = Self(1622);
    pub const SDL_CONTROLLERTOUCHPADMOTION: Self = Self(1623);
    pub const SDL_CONTROLLERTOUCHPADUP: Self = Self(1624);
    pub const SDL_CONTROLLERSENSORUPDATE: Self = Self(1625);
    pub const SDL_FINGERDOWN: Self = Self(1792);
    pub const SDL_FINGERUP: Self = Self(1793);
    pub const SDL_FINGERMOTION: Self = Self(1794);
    pub const SDL_DOLLARGESTURE: Self = Self(2048);
    pub const SDL_DOLLARRECORD: Self = Self(2049);
    pub const SDL_MULTIGESTURE: Self = Self(2050);
    pub const SDL_CLIPBOARDUPDATE: Self = Self(2304);
    pub const SDL_DROPFILE: Self = Self(4096);
    pub const SDL_DROPTEXT: Self = Self(4097);
    pub const SDL_DROPBEGIN: Self = Self(4098);
    pub const SDL_DROPCOMPLETE: Self = Self(4099);
    pub const SDL_AUDIODEVICEADDED: Self = Self(4352);
    pub const SDL_AUDIODEVICEREMOVED: Self = Self(4353);
    pub const SDL_SENSORUPDATE: Self = Self(4608);
    pub const SDL_RENDER_TARGETS_RESET: Self = Self(8192);
    pub const SDL_RENDER_DEVICE_RESET: Self = Self(8193);
    pub const SDL_POLLSENTINEL: Self = Self(32512);
    pub const SDL_USEREVENT: Self = Self(32768);
    pub const SDL_LASTEVENT: Self = Self(65535);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_CommonEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub display: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_WindowEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
    pub data2: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub state: Uint8,
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub keysym: SDL_Keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [::core::ffi::c_char; 32],
    pub start: Sint32,
    pub length: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingExtEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: *mut ::core::ffi::c_char,
    pub start: Sint32,
    pub length: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub state: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub xrel: Sint32,
    pub yrel: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub button: Uint8,
    pub state: Uint8,
    pub clicks: Uint8,
    pub padding1: Uint8,
    pub x: Sint32,
    pub y: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub direction: Uint32,
    pub preciseX: ::core::ffi::c_float,
    pub preciseY: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub hat: Uint8,
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBatteryEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub level: SDL_JoystickPowerLevel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerTouchpadEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub touchpad: Sint32,
    pub finger: Sint32,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub pressure: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerSensorEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub sensor: Sint32,
    pub data: [::core::ffi::c_float; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: Uint32,
    pub iscapture: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub fingerId: SDL_FingerID,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub dx: ::core::ffi::c_float,
    pub dy: ::core::ffi::c_float,
    pub pressure: ::core::ffi::c_float,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MultiGestureEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub dTheta: ::core::ffi::c_float,
    pub dDist: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub numFingers: Uint16,
    pub padding: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: Uint32,
    pub error: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub file: *mut ::core::ffi::c_char,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SensorEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub data: [::core::ffi::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub code: Sint32,
    pub data1: *mut ::core::ffi::c_void,
    pub data2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub r#type: Uint32,
    pub timestamp: Uint32,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub r#type: Uint32,
    pub common: SDL_CommonEvent,
    pub display: SDL_DisplayEvent,
    pub window: SDL_WindowEvent,
    pub key: SDL_KeyboardEvent,
    pub edit: SDL_TextEditingEvent,
    pub editExt: SDL_TextEditingExtEvent,
    pub text: SDL_TextInputEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub wheel: SDL_MouseWheelEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub jdevice: SDL_JoyDeviceEvent,
    pub jbattery: SDL_JoyBatteryEvent,
    pub caxis: SDL_ControllerAxisEvent,
    pub cbutton: SDL_ControllerButtonEvent,
    pub cdevice: SDL_ControllerDeviceEvent,
    pub ctouchpad: SDL_ControllerTouchpadEvent,
    pub csensor: SDL_ControllerSensorEvent,
    pub adevice: SDL_AudioDeviceEvent,
    pub sensor: SDL_SensorEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
    pub tfinger: SDL_TouchFingerEvent,
    pub mgesture: SDL_MultiGestureEvent,
    pub dgesture: SDL_DollarGestureEvent,
    pub drop: SDL_DropEvent,
    pub padding: [Uint8; 56],
}
pub type sdl_key_t = SDL_Keycode;
pub type joystick = vcontrol_joystick;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_joystick {
    pub stick: *mut SDL_Joystick,
    pub numaxes: ::core::ffi::c_int,
    pub numbuttons: ::core::ffi::c_int,
    pub numhats: ::core::ffi::c_int,
    pub threshold: ::core::ffi::c_int,
    pub axes: *mut axis_type,
    pub buttons: *mut *mut keybinding,
    pub hats: *mut hat_type,
}
pub type hat_type = vcontrol_joystick_hat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_joystick_hat {
    pub left: *mut keybinding,
    pub right: *mut keybinding,
    pub up: *mut keybinding,
    pub down: *mut keybinding,
    pub last: Uint8,
}
pub type keybinding = vcontrol_keybinding;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_keybinding {
    pub target: *mut ::core::ffi::c_int,
    pub keycode: sdl_key_t,
    pub parent: *mut vcontrol_keypool,
    pub next: *mut vcontrol_keybinding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_keypool {
    pub pool: [keybinding; 64],
    pub remaining: ::core::ffi::c_int,
    pub next: *mut vcontrol_keypool,
}
pub type axis_type = vcontrol_joystick_axis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_joystick_axis {
    pub neg: *mut keybinding,
    pub pos: *mut keybinding,
    pub polarity: ::core::ffi::c_int,
}
pub type keypool = vcontrol_keypool;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VCONTROL_GESTURE_TYPE(pub ::core::ffi::c_uint);
impl VCONTROL_GESTURE_TYPE {
    pub const VCONTROL_NONE: Self = Self(0);
    pub const VCONTROL_KEY: Self = Self(1);
    pub const VCONTROL_JOYAXIS: Self = Self(2);
    pub const VCONTROL_JOYBUTTON: Self = Self(3);
    pub const VCONTROL_JOYHAT: Self = Self(4);
    pub const NUM_VCONTROL_GESTURES: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VCONTROL_GESTURE {
    pub r#type: VCONTROL_GESTURE_TYPE,
    pub gesture: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub key: sdl_key_t,
    pub axis: C2Rust_Unnamed_4,
    pub button: C2Rust_Unnamed_3,
    pub hat: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub dir: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub polarity: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
pub type parse_state = vcontrol_parse_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_parse_state {
    pub line: [::core::ffi::c_char; 256],
    pub token: [::core::ffi::c_char; 64],
    pub index: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub linenum: ::core::ffi::c_int,
}
static mut joysticks: *mut joystick = ::core::ptr::null_mut::<joystick>();
static mut joycount: ::core::ffi::c_uint = 0;
static mut bindings: [*mut keybinding; 512] = [::core::ptr::null_mut::<
    keybinding,
>(); 512];
static mut pool: *mut keypool = ::core::ptr::null_mut::<keypool>();
static mut event_ready: ::core::ffi::c_int = 0;
static mut last_interesting: SDL_Event = SDL_Event { r#type: 0 };
unsafe extern "C" fn allocate_key_chunk() -> *mut keypool {
    let mut x: *mut keypool = HMalloc(::core::mem::size_of::<keypool>()) as *mut keypool;
    if !x.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        (*x).remaining = 64 as ::core::ffi::c_int;
        (*x).next = ::core::ptr::null_mut::<vcontrol_keypool>();
        i = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            (*x).pool[i as usize].target = ::core::ptr::null_mut::<::core::ffi::c_int>();
            (*x).pool[i as usize].keycode = C2Rust_Unnamed::SDLK_UNKNOWN.0
                as ::core::ffi::c_int as sdl_key_t;
            (*x).pool[i as usize].next = ::core::ptr::null_mut::<vcontrol_keybinding>();
            (*x).pool[i as usize].parent = x as *mut vcontrol_keypool;
            i += 1;
        }
    }
    return x;
}
unsafe extern "C" fn free_key_pool(mut x: *mut keypool) {
    if !x.is_null() {
        free_key_pool((*x).next);
        HFree(x as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn create_joystick(mut index: ::core::ffi::c_int) {
    let mut stick: *mut SDL_Joystick = ::core::ptr::null_mut::<SDL_Joystick>();
    let mut axes: ::core::ffi::c_int = 0;
    let mut buttons: ::core::ffi::c_int = 0;
    let mut hats: ::core::ffi::c_int = 0;
    if index as ::core::ffi::c_uint >= joycount {
        log_add(
            log_Level::log_Warning,
            b"VControl warning: Tried to open a non-existent joystick!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*joysticks.offset(index as isize)).stick.is_null() {
        return;
    }
    stick = SDL_JoystickOpen(index);
    if !stick.is_null() {
        let mut x: *mut joystick = joysticks.offset(index as isize);
        let mut j: ::core::ffi::c_int = 0;
        log_add(
            log_Level::log_Info,
            b"VControl opened joystick: %s\0".as_ptr() as *const ::core::ffi::c_char,
            SDL_JoystickName(stick),
        );
        axes = SDL_JoystickNumAxes(stick);
        buttons = SDL_JoystickNumButtons(stick);
        hats = SDL_JoystickNumHats(stick);
        log_add(
            log_Level::log_Info,
            b"%d axes, %d buttons, %d hats.\0".as_ptr() as *const ::core::ffi::c_char,
            axes,
            buttons,
            hats,
        );
        (*x).numaxes = axes;
        (*x).numbuttons = buttons;
        (*x).numhats = hats;
        (*x).axes = HMalloc(
            ::core::mem::size_of::<axis_type>().wrapping_mul(axes as size_t),
        ) as *mut axis_type;
        (*x).buttons = HMalloc(
            ::core::mem::size_of::<*mut keybinding>().wrapping_mul(buttons as size_t),
        ) as *mut *mut keybinding;
        (*x).hats = HMalloc(
            ::core::mem::size_of::<hat_type>().wrapping_mul(hats as size_t),
        ) as *mut hat_type;
        j = 0 as ::core::ffi::c_int;
        while j < axes {
            (*(*x).axes.offset(j as isize)).pos = ::core::ptr::null_mut::<keybinding>();
            (*(*x).axes.offset(j as isize)).neg = (*(*x).axes.offset(j as isize)).pos;
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < hats {
            (*(*x).hats.offset(j as isize)).right = ::core::ptr::null_mut::<
                keybinding,
            >();
            (*(*x).hats.offset(j as isize)).left = (*(*x).hats.offset(j as isize)).right;
            (*(*x).hats.offset(j as isize)).down = ::core::ptr::null_mut::<keybinding>();
            (*(*x).hats.offset(j as isize)).up = (*(*x).hats.offset(j as isize)).down;
            (*(*x).hats.offset(j as isize)).last = 0 as Uint8;
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < buttons {
            *(*x).buttons.offset(j as isize) = ::core::ptr::null_mut::<keybinding>();
            j += 1;
        }
        (*x).stick = stick;
    } else {
        log_add(
            log_Level::log_Warning,
            b"VControl: Could not initialize joystick #%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            index,
        );
    };
}
unsafe extern "C" fn destroy_joystick(mut index: ::core::ffi::c_int) {
    let mut stick: *mut SDL_Joystick = (*joysticks.offset(index as isize)).stick;
    if !stick.is_null() {
        SDL_JoystickClose(stick);
        (*joysticks.offset(index as isize)).stick = ::core::ptr::null_mut::<
            SDL_Joystick,
        >();
        HFree((*joysticks.offset(index as isize)).axes as *mut ::core::ffi::c_void);
        HFree((*joysticks.offset(index as isize)).buttons as *mut ::core::ffi::c_void);
        HFree((*joysticks.offset(index as isize)).hats as *mut ::core::ffi::c_void);
        (*joysticks.offset(index as isize)).numbuttons = 0 as ::core::ffi::c_int;
        (*joysticks.offset(index as isize)).numaxes = (*joysticks.offset(index as isize))
            .numbuttons;
        (*joysticks.offset(index as isize)).axes = ::core::ptr::null_mut::<axis_type>();
        (*joysticks.offset(index as isize)).buttons = ::core::ptr::null_mut::<
            *mut keybinding,
        >();
        (*joysticks.offset(index as isize)).hats = ::core::ptr::null_mut::<hat_type>();
    }
}
unsafe extern "C" fn key_init() {
    let mut i: ::core::ffi::c_uint = 0;
    let mut num_keys: ::core::ffi::c_int = 0;
    pool = allocate_key_chunk();
    i = 0 as ::core::ffi::c_uint;
    while i < 512 as ::core::ffi::c_uint {
        bindings[i as usize] = ::core::ptr::null_mut::<keybinding>();
        i = i.wrapping_add(1);
    }
    joycount = SDL_NumJoysticks() as ::core::ffi::c_uint;
    if joycount != 0 {
        joysticks = HMalloc(
            ::core::mem::size_of::<joystick>().wrapping_mul(joycount as size_t),
        ) as *mut joystick;
        i = 0 as ::core::ffi::c_uint;
        while i < joycount {
            (*joysticks.offset(i as isize)).stick = ::core::ptr::null_mut::<
                SDL_Joystick,
            >();
            (*joysticks.offset(i as isize)).numbuttons = 0 as ::core::ffi::c_int;
            (*joysticks.offset(i as isize)).numaxes = (*joysticks.offset(i as isize))
                .numbuttons;
            (*joysticks.offset(i as isize)).axes = ::core::ptr::null_mut::<axis_type>();
            (*joysticks.offset(i as isize)).buttons = ::core::ptr::null_mut::<
                *mut keybinding,
            >();
            (*joysticks.offset(i as isize)).threshold = 10000 as ::core::ffi::c_int;
            i = i.wrapping_add(1);
        }
    } else {
        joysticks = ::core::ptr::null_mut::<joystick>();
    };
}
unsafe extern "C" fn key_uninit() {
    let mut i: ::core::ffi::c_uint = 0;
    free_key_pool(pool);
    i = 0 as ::core::ffi::c_uint;
    while i < 512 as ::core::ffi::c_uint {
        bindings[i as usize] = ::core::ptr::null_mut::<keybinding>();
        i = i.wrapping_add(1);
    }
    pool = ::core::ptr::null_mut::<keypool>();
    i = 0 as ::core::ffi::c_uint;
    while i < joycount {
        destroy_joystick(i as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    HFree(joysticks as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn VControl_Init() {
    key_init();
}
#[no_mangle]
pub unsafe extern "C" fn VControl_Uninit() {
    key_uninit();
}
#[no_mangle]
pub unsafe extern "C" fn VControl_SetJoyThreshold(
    mut port: ::core::ffi::c_int,
    mut threshold: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        (*joysticks.offset(port as isize)).threshold = threshold;
        return 0 as ::core::ffi::c_int;
    } else {
        return -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn add_binding(
    mut newptr: *mut *mut keybinding,
    mut target: *mut ::core::ffi::c_int,
    mut keycode: sdl_key_t,
) {
    let mut newbinding: *mut keybinding = ::core::ptr::null_mut::<keybinding>();
    let mut searchbase: *mut keypool = ::core::ptr::null_mut::<keypool>();
    let mut i: ::core::ffi::c_int = 0;
    while !(*newptr).is_null() {
        if (**newptr).target == target && (**newptr).keycode == keycode {
            return;
        }
        newptr = &raw mut (**newptr).next as *mut *mut keybinding;
    }
    searchbase = pool;
    while (*searchbase).remaining == 0 as ::core::ffi::c_int {
        if (*searchbase).next.is_null() {
            (*searchbase).next = allocate_key_chunk() as *mut vcontrol_keypool;
        }
        searchbase = (*searchbase).next as *mut keypool;
    }
    newbinding = ::core::ptr::null_mut::<keybinding>();
    i = 0 as ::core::ffi::c_int;
    while i < 64 as ::core::ffi::c_int {
        if (*searchbase).pool[i as usize].target.is_null() {
            newbinding = (&raw mut (*searchbase).pool as *mut keybinding)
                .offset(i as isize);
            break;
        } else {
            i += 1;
        }
    }
    if newbinding.is_null() {
        log_add(
            log_Level::log_Warning,
            b"add_binding failed to find a free binding slot!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*newbinding).target = target;
    (*newbinding).keycode = keycode;
    (*newbinding).next = ::core::ptr::null_mut::<vcontrol_keybinding>();
    *newptr = newbinding;
    (*searchbase).remaining -= 1;
}
unsafe extern "C" fn remove_binding(
    mut ptr: *mut *mut keybinding,
    mut target: *mut ::core::ffi::c_int,
    mut keycode: sdl_key_t,
) {
    if (*ptr).is_null() {
        return
    } else if (**ptr).target == target && (**ptr).keycode == keycode {
        let mut todel: *mut keybinding = *ptr;
        *ptr = (*todel).next as *mut keybinding;
        (*todel).target = ::core::ptr::null_mut::<::core::ffi::c_int>();
        (*todel).keycode = C2Rust_Unnamed::SDLK_UNKNOWN.0 as ::core::ffi::c_int
            as sdl_key_t;
        (*todel).next = ::core::ptr::null_mut::<vcontrol_keybinding>();
        (*(*todel).parent).remaining += 1;
    } else {
        let mut prev: *mut keybinding = *ptr;
        while !prev.is_null() && !(*prev).next.is_null() {
            if (*(*prev).next).target == target {
                let mut todel_0: *mut keybinding = (*prev).next;
                (*prev).next = (*todel_0).next;
                (*todel_0).target = ::core::ptr::null_mut::<::core::ffi::c_int>();
                (*todel_0).keycode = C2Rust_Unnamed::SDLK_UNKNOWN.0 as ::core::ffi::c_int
                    as sdl_key_t;
                (*todel_0).next = ::core::ptr::null_mut::<vcontrol_keybinding>();
                (*(*todel_0).parent).remaining += 1;
            }
            prev = (*prev).next as *mut keybinding;
        }
    };
}
unsafe extern "C" fn activate(mut i: *mut keybinding, mut keycode: sdl_key_t) {
    while !i.is_null() {
        if (*i).keycode == keycode {
            *(*i).target = *(*i).target + 1 as ::core::ffi::c_int
                | 0x10000 as ::core::ffi::c_int;
        }
        i = (*i).next as *mut keybinding;
    }
}
unsafe extern "C" fn deactivate(mut i: *mut keybinding, mut keycode: sdl_key_t) {
    while !i.is_null() {
        let mut v: ::core::ffi::c_int = *(*i).target & 0xffff as ::core::ffi::c_int;
        if (*i).keycode == keycode && v > 0 as ::core::ffi::c_int {
            *(*i).target = v - 1 as ::core::ffi::c_int
                | *(*i).target & 0x10000 as ::core::ffi::c_int;
        }
        i = (*i).next as *mut keybinding;
    }
}
unsafe extern "C" fn event2gesture(mut e: *mut SDL_Event, mut g: *mut VCONTROL_GESTURE) {
    match (*e).r#type {
        768 => {
            (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_KEY;
            (*g).gesture.key = (*e).key.keysym.sym as sdl_key_t;
        }
        1536 => {
            (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS;
            (*g).gesture.axis.port = (*e).jaxis.which as ::core::ffi::c_int;
            (*g).gesture.axis.index = (*e).jaxis.axis as ::core::ffi::c_int;
            (*g).gesture.axis.polarity = if ((*e).jaxis.value as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                -1 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        }
        1538 => {
            (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_JOYHAT;
            (*g).gesture.hat.port = (*e).jhat.which as ::core::ffi::c_int;
            (*g).gesture.hat.index = (*e).jhat.hat as ::core::ffi::c_int;
            (*g).gesture.hat.dir = (*e).jhat.value;
        }
        1539 => {
            (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON;
            (*g).gesture.button.port = (*e).jbutton.which as ::core::ffi::c_int;
            (*g).gesture.button.index = (*e).jbutton.button as ::core::ffi::c_int;
        }
        _ => {
            (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_NONE;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_AddGestureBinding(
    mut g: *mut VCONTROL_GESTURE,
    mut target: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    match (*g).r#type {
        VCONTROL_GESTURE_TYPE::VCONTROL_KEY => {
            result = VControl_AddKeyBinding((*g).gesture.key, target);
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS => {
            result = VControl_AddJoyAxisBinding(
                (*g).gesture.axis.port,
                (*g).gesture.axis.index,
                if (*g).gesture.axis.polarity < 0 as ::core::ffi::c_int {
                    -1 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                },
                target,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYHAT => {
            result = VControl_AddJoyHatBinding(
                (*g).gesture.hat.port,
                (*g).gesture.hat.index,
                (*g).gesture.hat.dir,
                target,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON => {
            result = VControl_AddJoyButtonBinding(
                (*g).gesture.button.port,
                (*g).gesture.button.index,
                target,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_NONE => {}
        _ => {
            log_add(
                log_Level::log_Warning,
                b"VControl_AddGestureBinding didn't understand argument gesture\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            result = -1 as ::core::ffi::c_int;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn VControl_RemoveGestureBinding(
    mut g: *mut VCONTROL_GESTURE,
    mut target: *mut ::core::ffi::c_int,
) {
    match (*g).r#type {
        VCONTROL_GESTURE_TYPE::VCONTROL_KEY => {
            VControl_RemoveKeyBinding((*g).gesture.key, target);
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS => {
            VControl_RemoveJoyAxisBinding(
                (*g).gesture.axis.port,
                (*g).gesture.axis.index,
                if (*g).gesture.axis.polarity < 0 as ::core::ffi::c_int {
                    -1 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                },
                target,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYHAT => {
            VControl_RemoveJoyHatBinding(
                (*g).gesture.hat.port,
                (*g).gesture.hat.index,
                (*g).gesture.hat.dir,
                target,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON => {
            VControl_RemoveJoyButtonBinding(
                (*g).gesture.button.port,
                (*g).gesture.button.index,
                target,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_NONE => {}
        _ => {
            log_add(
                log_Level::log_Warning,
                b"VControl_RemoveGestureBinding didn't understand argument gesture\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_AddKeyBinding(
    mut symbol: sdl_key_t,
    mut target: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    add_binding(
        (&raw mut bindings as *mut *mut keybinding)
            .offset((symbol % 512 as ::core::ffi::c_int) as isize),
        target,
        symbol,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VControl_RemoveKeyBinding(
    mut symbol: sdl_key_t,
    mut target: *mut ::core::ffi::c_int,
) {
    remove_binding(
        (&raw mut bindings as *mut *mut keybinding)
            .offset((symbol % 512 as ::core::ffi::c_int) as isize),
        target,
        symbol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VControl_AddJoyAxisBinding(
    mut port: ::core::ffi::c_int,
    mut axis: ::core::ffi::c_int,
    mut polarity: ::core::ffi::c_int,
    mut target: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        let mut j: *mut joystick = joysticks.offset(port as isize);
        if (*j).stick.is_null() {
            create_joystick(port);
        }
        if axis >= 0 as ::core::ffi::c_int && axis < (*j).numaxes {
            if polarity < 0 as ::core::ffi::c_int {
                add_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .axes
                        .offset(axis as isize))
                        .neg,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if polarity > 0 as ::core::ffi::c_int {
                add_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .axes
                        .offset(axis as isize))
                        .pos,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else {
                log_add(
                    log_Level::log_Debug,
                    b"VControl: Attempted to bind to polarity zero\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            return -1 as ::core::ffi::c_int
        }
    } else {
        return -1 as ::core::ffi::c_int
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VControl_RemoveJoyAxisBinding(
    mut port: ::core::ffi::c_int,
    mut axis: ::core::ffi::c_int,
    mut polarity: ::core::ffi::c_int,
    mut target: *mut ::core::ffi::c_int,
) {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        let mut j: *mut joystick = joysticks.offset(port as isize);
        if (*j).stick.is_null() {
            create_joystick(port);
        }
        if axis >= 0 as ::core::ffi::c_int && axis < (*j).numaxes {
            if polarity < 0 as ::core::ffi::c_int {
                remove_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .axes
                        .offset(axis as isize))
                        .neg,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if polarity > 0 as ::core::ffi::c_int {
                remove_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .axes
                        .offset(axis as isize))
                        .pos,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else {
                log_add(
                    log_Level::log_Debug,
                    b"VControl: Attempted to unbind from polarity zero\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            log_add(
                log_Level::log_Debug,
                b"VControl: Attempted to unbind from illegal axis %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                axis,
            );
        }
    } else {
        log_add(
            log_Level::log_Debug,
            b"VControl: Attempted to unbind from illegal port %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            port,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_AddJoyButtonBinding(
    mut port: ::core::ffi::c_int,
    mut button: ::core::ffi::c_int,
    mut target: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        let mut j: *mut joystick = joysticks.offset(port as isize);
        if (*j).stick.is_null() {
            create_joystick(port);
        }
        if button >= 0 as ::core::ffi::c_int && button < (*j).numbuttons {
            add_binding(
                (*joysticks.offset(port as isize)).buttons.offset(button as isize),
                target,
                C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
            );
            return 0 as ::core::ffi::c_int;
        } else {
            return -1 as ::core::ffi::c_int
        }
    } else {
        return -1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_RemoveJoyButtonBinding(
    mut port: ::core::ffi::c_int,
    mut button: ::core::ffi::c_int,
    mut target: *mut ::core::ffi::c_int,
) {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        let mut j: *mut joystick = joysticks.offset(port as isize);
        if (*j).stick.is_null() {
            create_joystick(port);
        }
        if button >= 0 as ::core::ffi::c_int && button < (*j).numbuttons {
            remove_binding(
                (*joysticks.offset(port as isize)).buttons.offset(button as isize),
                target,
                C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
            );
        } else {
            log_add(
                log_Level::log_Debug,
                b"VControl: Attempted to unbind from illegal button %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                button,
            );
        }
    } else {
        log_add(
            log_Level::log_Debug,
            b"VControl: Attempted to unbind from illegal port %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            port,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_AddJoyHatBinding(
    mut port: ::core::ffi::c_int,
    mut which: ::core::ffi::c_int,
    mut dir: Uint8,
    mut target: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        let mut j: *mut joystick = joysticks.offset(port as isize);
        if (*j).stick.is_null() {
            create_joystick(port);
        }
        if which >= 0 as ::core::ffi::c_int && which < (*j).numhats {
            if dir as ::core::ffi::c_int == 0x8 as ::core::ffi::c_int {
                add_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .left,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if dir as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
                add_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .right,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if dir as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int {
                add_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .up,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if dir as ::core::ffi::c_int == 0x4 as ::core::ffi::c_int {
                add_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .down,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else {
                return -1 as ::core::ffi::c_int
            }
            return 0 as ::core::ffi::c_int;
        } else {
            return -1 as ::core::ffi::c_int
        }
    } else {
        return -1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_RemoveJoyHatBinding(
    mut port: ::core::ffi::c_int,
    mut which: ::core::ffi::c_int,
    mut dir: Uint8,
    mut target: *mut ::core::ffi::c_int,
) {
    if port >= 0 as ::core::ffi::c_int && (port as ::core::ffi::c_uint) < joycount {
        let mut j: *mut joystick = joysticks.offset(port as isize);
        if (*j).stick.is_null() {
            create_joystick(port);
        }
        if which >= 0 as ::core::ffi::c_int && which < (*j).numhats {
            if dir as ::core::ffi::c_int == 0x8 as ::core::ffi::c_int {
                remove_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .left,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if dir as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
                remove_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .right,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if dir as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int {
                remove_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .up,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else if dir as ::core::ffi::c_int == 0x4 as ::core::ffi::c_int {
                remove_binding(
                    &raw mut (*(*joysticks.offset(port as isize))
                        .hats
                        .offset(which as isize))
                        .down,
                    target,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            } else {
                log_add(
                    log_Level::log_Debug,
                    b"VControl: Attempted to unbind from illegal direction\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            log_add(
                log_Level::log_Debug,
                b"VControl: Attempted to unbind from illegal hat %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                which,
            );
        }
    } else {
        log_add(
            log_Level::log_Debug,
            b"VControl: Attempted to unbind from illegal port %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            port,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_RemoveAllBindings() {
    key_uninit();
    key_init();
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ProcessKeyDown(mut symbol: sdl_key_t) {
    activate(bindings[(symbol % 512 as ::core::ffi::c_int) as usize], symbol);
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ProcessKeyUp(mut symbol: sdl_key_t) {
    deactivate(bindings[(symbol % 512 as ::core::ffi::c_int) as usize], symbol);
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ProcessJoyButtonDown(
    mut port: ::core::ffi::c_int,
    mut button: ::core::ffi::c_int,
) {
    if (*joysticks.offset(port as isize)).stick.is_null() {
        return;
    }
    activate(
        *(*joysticks.offset(port as isize)).buttons.offset(button as isize),
        C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ProcessJoyButtonUp(
    mut port: ::core::ffi::c_int,
    mut button: ::core::ffi::c_int,
) {
    if (*joysticks.offset(port as isize)).stick.is_null() {
        return;
    }
    deactivate(
        *(*joysticks.offset(port as isize)).buttons.offset(button as isize),
        C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ProcessJoyAxis(
    mut port: ::core::ffi::c_int,
    mut axis: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) {
    let mut t: ::core::ffi::c_int = 0;
    if (*joysticks.offset(port as isize)).stick.is_null() {
        return;
    }
    t = (*joysticks.offset(port as isize)).threshold;
    if value > t {
        if (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity
            != 1 as ::core::ffi::c_int
        {
            if (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity
                == -1 as ::core::ffi::c_int
            {
                deactivate(
                    (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).neg,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            }
            (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity = 1
                as ::core::ffi::c_int;
            activate(
                (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).pos,
                C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
            );
        }
    } else if value < -t {
        if (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity
            != -1 as ::core::ffi::c_int
        {
            if (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity
                == 1 as ::core::ffi::c_int
            {
                deactivate(
                    (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).pos,
                    C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
                );
            }
            (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity = -1
                as ::core::ffi::c_int;
            activate(
                (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).neg,
                C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
            );
        }
    } else {
        if (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity
            == -1 as ::core::ffi::c_int
        {
            deactivate(
                (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).neg,
                C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
            );
        } else if (*(*joysticks.offset(port as isize)).axes.offset(axis as isize))
            .polarity == 1 as ::core::ffi::c_int
        {
            deactivate(
                (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).pos,
                C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
            );
        }
        (*(*joysticks.offset(port as isize)).axes.offset(axis as isize)).polarity = 0
            as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ProcessJoyHat(
    mut port: ::core::ffi::c_int,
    mut which: ::core::ffi::c_int,
    mut value: Uint8,
) {
    let mut old: Uint8 = 0;
    if (*joysticks.offset(port as isize)).stick.is_null() {
        return;
    }
    old = (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).last;
    if old as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
        && value as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
    {
        activate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).left,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0
        && value as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0
    {
        activate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).right,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0
        && value as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
    {
        activate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).up,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
        && value as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
    {
        activate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).down,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
        && value as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
    {
        deactivate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).left,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0
        && value as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0
    {
        deactivate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).right,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
        && value as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0
    {
        deactivate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).up,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    if old as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
        && value as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
    {
        deactivate(
            (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).down,
            C2Rust_Unnamed::SDLK_UNKNOWN.0 as sdl_key_t,
        );
    }
    (*(*joysticks.offset(port as isize)).hats.offset(which as isize)).last = value;
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ResetInput() {
    let mut base: *mut keypool = pool;
    while !base.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            if !(*base).pool[i as usize].target.is_null() {
                *(*base).pool[i as usize].target = 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        base = (*base).next as *mut keypool;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VControl_BeginFrame() {
    let mut base: *mut keypool = pool;
    while !base.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            if !(*base).pool[i as usize].target.is_null() {
                *(*base).pool[i as usize].target &= 0xffff as ::core::ffi::c_int;
            }
            i += 1;
        }
        base = (*base).next as *mut keypool;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VControl_HandleEvent(mut e: *const SDL_Event) {
    match (*e).r#type {
        768 => {
            if (*e).key.repeat == 0 {
                VControl_ProcessKeyDown((*e).key.keysym.sym);
                last_interesting = *e;
                event_ready = 1 as ::core::ffi::c_int;
            }
        }
        769 => {
            VControl_ProcessKeyUp((*e).key.keysym.sym);
        }
        1536 => {
            VControl_ProcessJoyAxis(
                (*e).jaxis.which,
                (*e).jaxis.axis as ::core::ffi::c_int,
                (*e).jaxis.value as ::core::ffi::c_int,
            );
            if (*e).jaxis.value as ::core::ffi::c_int > 15000 as ::core::ffi::c_int
                || ((*e).jaxis.value as ::core::ffi::c_int)
                    < -15000 as ::core::ffi::c_int
            {
                last_interesting = *e;
                event_ready = 1 as ::core::ffi::c_int;
            }
        }
        1538 => {
            VControl_ProcessJoyHat(
                (*e).jhat.which,
                (*e).jhat.hat as ::core::ffi::c_int,
                (*e).jhat.value,
            );
            last_interesting = *e;
            event_ready = 1 as ::core::ffi::c_int;
        }
        1539 => {
            VControl_ProcessJoyButtonDown(
                (*e).jbutton.which,
                (*e).jbutton.button as ::core::ffi::c_int,
            );
            last_interesting = *e;
            event_ready = 1 as ::core::ffi::c_int;
        }
        1540 => {
            VControl_ProcessJoyButtonUp(
                (*e).jbutton.which,
                (*e).jbutton.button as ::core::ffi::c_int,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ClearGesture() {
    event_ready = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VControl_GetLastGesture(
    mut g: *mut VCONTROL_GESTURE,
) -> ::core::ffi::c_int {
    if event_ready != 0 && !g.is_null() {
        event2gesture(&raw mut last_interesting, g);
    }
    return event_ready;
}
unsafe extern "C" fn next_token(mut state: *mut parse_state) {
    let mut index: ::core::ffi::c_int = 0;
    let mut base: ::core::ffi::c_int = 0;
    (*state).token[0usize] = 0 as ::core::ffi::c_char;
    base = (*state).index;
    while (*state).line[base as usize] as ::core::ffi::c_int != 0
        && isspace((*state).line[base as usize] as ::core::ffi::c_int) != 0
    {
        base += 1;
    }
    index = 0 as ::core::ffi::c_int;
    while index < 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        && (*state).line[(base + index) as usize] as ::core::ffi::c_int != 0
        && isspace((*state).line[(base + index) as usize] as ::core::ffi::c_int) == 0
    {
        (*state).token[index as usize] = (*state).line[(base + index) as usize];
        index += 1;
    }
    (*state).token[index as usize] = 0 as ::core::ffi::c_char;
    while (*state).line[(base + index) as usize] as ::core::ffi::c_int != 0
        && isspace((*state).line[(base + index) as usize] as ::core::ffi::c_int) == 0
    {
        index += 1;
    }
    (*state).index = base + index;
}
unsafe extern "C" fn expected_error(
    mut state: *mut parse_state,
    mut expected: *const ::core::ffi::c_char,
) {
    log_add(
        log_Level::log_Warning,
        b"VControl: Expected '%s' on config file line %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        expected,
        (*state).linenum,
    );
    (*state).error = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn consume(
    mut state: *mut parse_state,
    mut expected: *const ::core::ffi::c_char,
) {
    if strcasecmp(expected, &raw mut (*state).token as *mut ::core::ffi::c_char) != 0 {
        expected_error(state, expected);
    }
    next_token(state);
}
unsafe extern "C" fn consume_keyname(mut state: *mut parse_state) -> ::core::ffi::c_int {
    let mut keysym: ::core::ffi::c_int = VControl_name2code(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
    );
    if keysym == 0 {
        log_add(
            log_Level::log_Warning,
            b"VControl: Illegal key name '%s' on config file line %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*state).token as *mut ::core::ffi::c_char,
            (*state).linenum,
        );
        (*state).error = 1 as ::core::ffi::c_int;
    }
    next_token(state);
    return keysym;
}
unsafe extern "C" fn consume_num(mut state: *mut parse_state) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = strtol(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"VControl: Expected integer on config line %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*state).linenum,
        );
        (*state).error = 1 as ::core::ffi::c_int;
    }
    next_token(state);
    return result;
}
unsafe extern "C" fn consume_polarity(
    mut state: *mut parse_state,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"positive\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        result = 1 as ::core::ffi::c_int;
    } else if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"negative\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        result = -1 as ::core::ffi::c_int;
    } else {
        expected_error(
            state,
            b"positive' or 'negative\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    next_token(state);
    return result;
}
unsafe extern "C" fn consume_dir(mut state: *mut parse_state) -> Uint8 {
    let mut result: Uint8 = 0 as Uint8;
    if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"left\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        result = 0x8 as Uint8;
    } else if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"right\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        result = 0x2 as Uint8;
    } else if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"up\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        result = 0x1 as Uint8;
    } else if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"down\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        result = 0x4 as Uint8;
    } else {
        expected_error(
            state,
            b"left', 'right', 'up' or 'down\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    next_token(state);
    return result;
}
unsafe extern "C" fn parse_joybinding(
    mut state: *mut parse_state,
    mut gesture: *mut VCONTROL_GESTURE,
) {
    let mut sticknum: ::core::ffi::c_int = 0;
    consume(state, b"joystick\0".as_ptr() as *const ::core::ffi::c_char);
    sticknum = consume_num(state);
    if (*state).error == 0 {
        if strcasecmp(
            &raw mut (*state).token as *mut ::core::ffi::c_char,
            b"axis\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            let mut axisnum: ::core::ffi::c_int = 0;
            consume(state, b"axis\0".as_ptr() as *const ::core::ffi::c_char);
            axisnum = consume_num(state);
            if (*state).error == 0 {
                let mut polarity: ::core::ffi::c_int = consume_polarity(state);
                if (*state).error == 0 {
                    (*gesture).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS;
                    (*gesture).gesture.axis.port = sticknum;
                    (*gesture).gesture.axis.index = axisnum;
                    (*gesture).gesture.axis.polarity = polarity;
                }
            }
        } else if strcasecmp(
            &raw mut (*state).token as *mut ::core::ffi::c_char,
            b"button\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            let mut buttonnum: ::core::ffi::c_int = 0;
            consume(state, b"button\0".as_ptr() as *const ::core::ffi::c_char);
            buttonnum = consume_num(state);
            if (*state).error == 0 {
                (*gesture).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON;
                (*gesture).gesture.button.port = sticknum;
                (*gesture).gesture.button.index = buttonnum;
            }
        } else if strcasecmp(
            &raw mut (*state).token as *mut ::core::ffi::c_char,
            b"hat\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            let mut hatnum: ::core::ffi::c_int = 0;
            consume(state, b"hat\0".as_ptr() as *const ::core::ffi::c_char);
            hatnum = consume_num(state);
            if (*state).error == 0 {
                let mut dir: Uint8 = consume_dir(state);
                if (*state).error == 0 {
                    (*gesture).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_JOYHAT;
                    (*gesture).gesture.hat.port = sticknum;
                    (*gesture).gesture.hat.index = hatnum;
                    (*gesture).gesture.hat.dir = dir;
                }
            }
        } else {
            expected_error(
                state,
                b"axis', 'button', or 'hat\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn parse_gesture(
    mut state: *mut parse_state,
    mut gesture: *mut VCONTROL_GESTURE,
) {
    (*gesture).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_NONE;
    if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut keysym: ::core::ffi::c_int = 0;
        consume(state, b"key\0".as_ptr() as *const ::core::ffi::c_char);
        keysym = consume_keyname(state);
        if (*state).error == 0 {
            (*gesture).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_KEY;
            (*gesture).gesture.key = keysym as sdl_key_t;
        }
    } else if strcasecmp(
        &raw mut (*state).token as *mut ::core::ffi::c_char,
        b"joystick\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        parse_joybinding(state, gesture);
    } else {
        expected_error(
            state,
            b"key' or 'joystick\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_ParseGesture(
    mut g: *mut VCONTROL_GESTURE,
    mut spec: *const ::core::ffi::c_char,
) {
    let mut ps: parse_state = parse_state {
        line: [0; 256],
        token: [0; 64],
        index: 0,
        error: 0,
        linenum: 0,
    };
    strncpy(&raw mut ps.line as *mut ::core::ffi::c_char, spec, 256 as size_t);
    ps.line[(256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = '\0'
        as ::core::ffi::c_char;
    ps.error = 0 as ::core::ffi::c_int;
    ps.index = ps.error;
    ps.linenum = -1 as ::core::ffi::c_int;
    next_token(&raw mut ps);
    parse_gesture(&raw mut ps, g);
    if ps.error != 0 {
        printf(b"Error parsing %s\n\0".as_ptr() as *const ::core::ffi::c_char, spec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VControl_DumpGesture(
    mut buf: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut g: *mut VCONTROL_GESTURE,
) -> ::core::ffi::c_int {
    match (*g).r#type {
        VCONTROL_GESTURE_TYPE::VCONTROL_KEY => {
            return snprintf(
                buf,
                n as size_t,
                b"key %s\0".as_ptr() as *const ::core::ffi::c_char,
                VControl_code2name((*g).gesture.key),
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS => {
            return snprintf(
                buf,
                n as size_t,
                b"joystick %d axis %d %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*g).gesture.axis.port,
                (*g).gesture.axis.index,
                if (*g).gesture.axis.polarity > 0 as ::core::ffi::c_int {
                    b"positive\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"negative\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON => {
            return snprintf(
                buf,
                n as size_t,
                b"joystick %d button %d\0".as_ptr() as *const ::core::ffi::c_char,
                (*g).gesture.button.port,
                (*g).gesture.button.index,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYHAT => {
            return snprintf(
                buf,
                n as size_t,
                b"joystick %d hat %d %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*g).gesture.hat.port,
                (*g).gesture.hat.index,
                if (*g).gesture.hat.dir as ::core::ffi::c_int
                    == 0x1 as ::core::ffi::c_int
                {
                    b"up\0".as_ptr() as *const ::core::ffi::c_char
                } else if (*g).gesture.hat.dir as ::core::ffi::c_int
                    == 0x4 as ::core::ffi::c_int
                {
                    b"down\0".as_ptr() as *const ::core::ffi::c_char
                } else if (*g).gesture.hat.dir as ::core::ffi::c_int
                    == 0x8 as ::core::ffi::c_int
                {
                    b"left\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"right\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        _ => {
            *buf.offset(0isize) = '\0' as ::core::ffi::c_char;
            return 0 as ::core::ffi::c_int;
        }
    };
}
