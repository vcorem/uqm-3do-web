//Copyright Paul Reiche, Fred Ford. 1992-2002

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

// Rust translation of src/libs/input/sdl/input.c.
extern "C" {
    pub type SDL_SysWMmsg;
    pub type uio_DirHandle;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn SDL_GetError() -> *const ::core::ffi::c_char;
    fn SDL_StartTextInput();
    fn SDL_StopTextInput();
    fn SDL_NumJoysticks() -> ::core::ffi::c_int;
    fn SDL_JoystickNameForIndex(
        device_index: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn SDL_JoystickEventState(state: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SDL_InitSubSystem(flags: Uint32) -> ::core::ffi::c_int;
    fn TaskSwitch();
    fn VControl_Init();
    fn VControl_Uninit();
    fn VControl_AddGestureBinding(
        g: *mut VCONTROL_GESTURE,
        target: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VControl_RemoveGestureBinding(
        g: *mut VCONTROL_GESTURE,
        target: *mut ::core::ffi::c_int,
    );
    fn VControl_BeginFrame();
    fn VControl_HandleEvent(e: *const SDL_Event);
    fn VControl_ResetInput();
    fn VControl_ParseGesture(g: *mut VCONTROL_GESTURE, spec: *const ::core::ffi::c_char);
    fn VControl_DumpGesture(
        buf: *mut ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        g: *mut VCONTROL_GESTURE,
    ) -> ::core::ffi::c_int;
    fn VControl_ClearGesture();
    fn VControl_GetLastGesture(g: *mut VCONTROL_GESTURE) -> ::core::ffi::c_int;
    fn VControl_code2name(code: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn LoadResourceIndex(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    );
    fn SaveResourceIndex(
        dir: *mut uio_DirHandle,
        rmpfile: *const ::core::ffi::c_char,
        root: *const ::core::ffi::c_char,
        strip_root: BOOLEAN,
    );
    fn res_HasKey(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_IsString(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_GetString(key: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn res_PutString(key: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char);
    fn res_Remove(key: *const ::core::ffi::c_char) -> BOOLEAN;
    static mut contentDir: *mut uio_DirHandle;
    static mut configDir: *mut uio_DirHandle;
    static mut input_templates: [INPUT_TEMPLATE; 6];
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const KMOD_NONE: Self = Self(0);
    pub const KMOD_LSHIFT: Self = Self(1);
    pub const KMOD_RSHIFT: Self = Self(2);
    pub const KMOD_LCTRL: Self = Self(64);
    pub const KMOD_RCTRL: Self = Self(128);
    pub const KMOD_LALT: Self = Self(256);
    pub const KMOD_RALT: Self = Self(512);
    pub const KMOD_LGUI: Self = Self(1024);
    pub const KMOD_RGUI: Self = Self(2048);
    pub const KMOD_NUM: Self = Self(4096);
    pub const KMOD_CAPS: Self = Self(8192);
    pub const KMOD_MODE: Self = Self(16384);
    pub const KMOD_SCROLL: Self = Self(32768);
    pub const KMOD_CTRL: Self = Self(192);
    pub const KMOD_SHIFT: Self = Self(3);
    pub const KMOD_ALT: Self = Self(768);
    pub const KMOD_GUI: Self = Self(3072);
    pub const KMOD_RESERVED: Self = Self(32768);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub r#mod: Uint16,
    pub unused: Uint32,
}
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type UniChar = uint32;
pub type uint32 = ::core::ffi::c_uint;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
pub union C2Rust_Unnamed_3 {
    pub key: sdl_key_t,
    pub axis: C2Rust_Unnamed_5,
    pub button: C2Rust_Unnamed_4,
    pub hat: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub polarity: ::core::ffi::c_int,
}
pub type sdl_key_t = SDL_Keycode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VCONTROL_GESTURE {
    pub r#type: VCONTROL_GESTURE_TYPE,
    pub gesture: C2Rust_Unnamed_3,
}
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
pub type INPUT_TEMPLATE = _input_template;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _input_template {
    pub name: [::core::ffi::c_char; 30],
}
static mut kbdhead: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut kbdtail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut kbdbuf: [UniChar; 256] = [0; 256];
static mut lastchar: UniChar = 0;
static mut set_character_mode: BOOLEAN = BOOLEAN::FALSE;
static mut menu_vec: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
    ::core::ffi::c_int,
>();
static mut num_menu: ::core::ffi::c_int = 0;
static mut flight_vec: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
    ::core::ffi::c_int,
>();
static mut num_templ: ::core::ffi::c_int = 0;
static mut num_flight: ::core::ffi::c_int = 0;
static mut InputInitialized: BOOLEAN = BOOLEAN::FALSE;
static mut in_character_mode: BOOLEAN = BOOLEAN::FALSE;
static mut menu_res_names: [*const ::core::ffi::c_char; 24] = [
    b"pause\0".as_ptr() as *const ::core::ffi::c_char,
    b"exit\0".as_ptr() as *const ::core::ffi::c_char,
    b"abort\0".as_ptr() as *const ::core::ffi::c_char,
    b"debug\0".as_ptr() as *const ::core::ffi::c_char,
    b"fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
    b"up\0".as_ptr() as *const ::core::ffi::c_char,
    b"down\0".as_ptr() as *const ::core::ffi::c_char,
    b"left\0".as_ptr() as *const ::core::ffi::c_char,
    b"right\0".as_ptr() as *const ::core::ffi::c_char,
    b"select\0".as_ptr() as *const ::core::ffi::c_char,
    b"cancel\0".as_ptr() as *const ::core::ffi::c_char,
    b"special\0".as_ptr() as *const ::core::ffi::c_char,
    b"pageup\0".as_ptr() as *const ::core::ffi::c_char,
    b"pagedown\0".as_ptr() as *const ::core::ffi::c_char,
    b"home\0".as_ptr() as *const ::core::ffi::c_char,
    b"end\0".as_ptr() as *const ::core::ffi::c_char,
    b"zoomin\0".as_ptr() as *const ::core::ffi::c_char,
    b"zoomout\0".as_ptr() as *const ::core::ffi::c_char,
    b"delete\0".as_ptr() as *const ::core::ffi::c_char,
    b"backspace\0".as_ptr() as *const ::core::ffi::c_char,
    b"editcancel\0".as_ptr() as *const ::core::ffi::c_char,
    b"search\0".as_ptr() as *const ::core::ffi::c_char,
    b"next\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut flight_res_names: [*const ::core::ffi::c_char; 8] = [
    b"up\0".as_ptr() as *const ::core::ffi::c_char,
    b"down\0".as_ptr() as *const ::core::ffi::c_char,
    b"left\0".as_ptr() as *const ::core::ffi::c_char,
    b"right\0".as_ptr() as *const ::core::ffi::c_char,
    b"weapon\0".as_ptr() as *const ::core::ffi::c_char,
    b"special\0".as_ptr() as *const ::core::ffi::c_char,
    b"escape\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn register_menu_controls(mut index: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 40] = [0; 40];
    buf[39usize] = '\0' as ::core::ffi::c_char;
    i = 1 as ::core::ffi::c_int;
    while BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0 {
        let mut g: VCONTROL_GESTURE = VCONTROL_GESTURE {
            r#type: VCONTROL_GESTURE_TYPE::VCONTROL_NONE,
            gesture: C2Rust_Unnamed_3 { key: 0 },
        };
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            39 as size_t,
            b"menu.%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
            menu_res_names[index as usize],
            i,
        );
        if res_IsString(&raw mut buf as *mut ::core::ffi::c_char).0 == 0 {
            break;
        }
        VControl_ParseGesture(
            &raw mut g,
            res_GetString(&raw mut buf as *mut ::core::ffi::c_char),
        );
        VControl_AddGestureBinding(
            &raw mut g,
            menu_vec.offset(index as isize) as *mut ::core::ffi::c_int,
        );
        i += 1;
    }
}
static mut controls: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<VCONTROL_GESTURE>();
unsafe extern "C" fn register_flight_controls() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 40] = [0; 40];
    buf[39usize] = '\0' as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < num_templ {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            39 as size_t,
            b"keys.%d.name\0".as_ptr() as *const ::core::ffi::c_char,
            i + 1 as ::core::ffi::c_int,
        );
        if res_IsString(&raw mut buf as *mut ::core::ffi::c_char).0 != 0 {
            strncpy(
                &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE)
                    .offset(i as isize))
                    .name as *mut ::core::ffi::c_char,
                res_GetString(&raw mut buf as *mut ::core::ffi::c_char),
                29 as size_t,
            );
            input_templates[i as usize].name[29usize] = '\0' as ::core::ffi::c_char;
        } else {
            input_templates[i as usize].name[0usize] = '\0' as ::core::ffi::c_char;
        }
        j = 0 as ::core::ffi::c_int;
        while j < num_flight {
            k = 0 as ::core::ffi::c_int;
            while k < 2 as ::core::ffi::c_int {
                let mut g: *mut VCONTROL_GESTURE = controls
                    .offset(((i * num_flight + j) * 2 as ::core::ffi::c_int) as isize)
                    .offset(k as isize);
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    39 as size_t,
                    b"keys.%d.%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
                    i + 1 as ::core::ffi::c_int,
                    flight_res_names[j as usize],
                    k + 1 as ::core::ffi::c_int,
                );
                if res_IsString(&raw mut buf as *mut ::core::ffi::c_char).0 == 0 {
                    (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_NONE;
                } else {
                    VControl_ParseGesture(
                        g,
                        res_GetString(&raw mut buf as *mut ::core::ffi::c_char),
                    );
                    VControl_AddGestureBinding(
                        g,
                        flight_vec.offset((i * num_flight) as isize).offset(j as isize)
                            as *mut ::core::ffi::c_int,
                    );
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn initKeyConfig() {
    let mut i: ::core::ffi::c_int = 0;
    if menu_vec.is_null() || flight_vec.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"initKeyConfig(): invalid input vectors\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    controls = HCalloc(
        ::core::mem::size_of::<VCONTROL_GESTURE>()
            .wrapping_mul(num_templ as size_t)
            .wrapping_mul(num_flight as size_t)
            .wrapping_mul(2 as size_t),
    ) as *mut VCONTROL_GESTURE;
    LoadResourceIndex(
        contentDir,
        b"menu.key\0".as_ptr() as *const ::core::ffi::c_char,
        b"menu.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    LoadResourceIndex(
        configDir,
        b"override.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"menu.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < num_menu {
        if menu_res_names[i as usize].is_null() {
            break;
        }
        register_menu_controls(i);
        i += 1;
    }
    LoadResourceIndex(
        configDir,
        b"flight.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"keys.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_HasKey(b"keys.1.name\0".as_ptr() as *const ::core::ffi::c_char).0 == 0 {
        LoadResourceIndex(
            contentDir,
            b"uqm.key\0".as_ptr() as *const ::core::ffi::c_char,
            b"keys.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    register_flight_controls();
}
unsafe extern "C" fn resetKeyboardState() {}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetInputVectors(
    mut menu: *mut ::core::ffi::c_int,
    mut num_menu_: ::core::ffi::c_int,
    mut flight: *mut ::core::ffi::c_int,
    mut num_templ_: ::core::ffi::c_int,
    mut num_flight_: ::core::ffi::c_int,
) {
    if num_menu_ < 0 as ::core::ffi::c_int || num_templ_ < 0 as ::core::ffi::c_int
        || num_flight_ < 0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"TFB_SetInputVectors(): invalid vector size\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    menu_vec = menu as *mut ::core::ffi::c_int;
    num_menu = num_menu_;
    flight_vec = flight as *mut ::core::ffi::c_int;
    num_templ = num_templ_;
    num_flight = num_flight_;
}
unsafe extern "C" fn initJoystick() {
    let mut nJoysticks: ::core::ffi::c_int = 0;
    if SDL_InitSubSystem(0x200 as Uint32) == -1 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Fatal,
            b"Couldn't initialize joystick subsystem: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    log_add(
        log_Level::log_Info,
        b"%i joysticks were found.\0".as_ptr() as *const ::core::ffi::c_char,
        SDL_NumJoysticks(),
    );
    nJoysticks = SDL_NumJoysticks();
    if nJoysticks > 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        log_add(
            log_Level::log_Info,
            b"The names of the joysticks are:\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < nJoysticks {
            log_add(
                log_Level::log_Info,
                b"    %s\0".as_ptr() as *const ::core::ffi::c_char,
                SDL_JoystickNameForIndex(i),
            );
            i += 1;
        }
        SDL_JoystickEventState(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_InitInput(
    mut driver: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    initJoystick();
    in_character_mode = BOOLEAN::FALSE;
    resetKeyboardState();
    VControl_Init();
    initKeyConfig();
    VControl_ResetInput();
    InputInitialized = BOOLEAN::TRUE;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_UninitInput() {
    VControl_Uninit();
    HFree(controls as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn EnterCharacterMode() {
    kbdtail = 0 as ::core::ffi::c_int;
    kbdhead = kbdtail;
    lastchar = 0 as UniChar;
    in_character_mode = BOOLEAN::TRUE;
    VControl_ResetInput();
}
#[no_mangle]
pub unsafe extern "C" fn ExitCharacterMode() {
    VControl_ResetInput();
    in_character_mode = BOOLEAN::FALSE;
    kbdtail = 0 as ::core::ffi::c_int;
    kbdhead = kbdtail;
    lastchar = 0 as UniChar;
}
#[no_mangle]
pub unsafe extern "C" fn GetNextCharacter() -> UniChar {
    let mut result: UniChar = 0;
    if kbdhead == kbdtail {
        return 0 as UniChar;
    }
    result = kbdbuf[kbdhead as usize];
    kbdhead = kbdhead + 1 as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn GetLastCharacter() -> UniChar {
    return lastchar;
}
#[no_mangle]
pub static mut MouseButtonDown: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn ProcessMouseEvent(mut e: *const SDL_Event) {
    match (*e).r#type {
        1025 => {
            ::core::ptr::write_volatile(
                &raw mut MouseButtonDown,
                1 as ::core::ffi::c_int,
            );
        }
        1026 => {
            ::core::ptr::write_volatile(
                &raw mut MouseButtonDown,
                0 as ::core::ffi::c_int,
            );
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn is_numpad_char_event(
    mut Event: *const SDL_Event,
) -> ::core::ffi::c_int {
    return (in_character_mode.0 != 0
        && ((*Event).r#type
            == C2Rust_Unnamed_1::SDL_KEYDOWN.0 as ::core::ffi::c_int as Uint32
            || (*Event).r#type
                == C2Rust_Unnamed_1::SDL_KEYUP.0 as ::core::ffi::c_int as Uint32)
        && (*Event).key.keysym.r#mod as ::core::ffi::c_int
            & C2Rust_Unnamed_0::KMOD_NUM.0 as ::core::ffi::c_int != 0
        && (*Event).key.keysym.sym >= C2Rust_Unnamed::SDLK_KP_1.0 as ::core::ffi::c_int
        && (*Event).key.keysym.sym
            <= C2Rust_Unnamed::SDLK_KP_PERIOD.0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessInputEvent(mut Event: *const SDL_Event) {
    if InputInitialized.0 == 0 {
        return;
    }
    ProcessMouseEvent(Event);
    if in_character_mode.0 != 0 && set_character_mode.0 == 0 {
        set_character_mode = BOOLEAN::TRUE;
        SDL_StartTextInput();
    }
    if in_character_mode.0 == 0 && set_character_mode.0 != 0 {
        set_character_mode = BOOLEAN::FALSE;
        SDL_StopTextInput();
    }
    if is_numpad_char_event(Event) == 0 {
        VControl_HandleEvent(Event);
    }
    if (*Event).r#type
        == C2Rust_Unnamed_1::SDL_TEXTINPUT.0 as ::core::ffi::c_int as Uint32
    {
        let mut newtail: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (*Event).text.text[i as usize] != 0 {
            let c2rust_fresh0 = i;
            i += 1;
            let mut map_key: UniChar = (*Event).text.text[c2rust_fresh0 as usize]
                as UniChar;
            map_key &= 0xff as UniChar;
            if map_key >= 0xc0 as UniChar && map_key < 0xe0 as UniChar {
                map_key = (map_key & 0x1f as UniChar) << 6 as ::core::ffi::c_int;
                let c2rust_fresh1 = i;
                i += 1;
                map_key
                    |= ((*Event).text.text[c2rust_fresh1 as usize] as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) as UniChar;
            } else if map_key >= 0xe0 as UniChar && map_key < 0xf0 as UniChar {
                map_key = (map_key & 0xf as UniChar) << 6 as ::core::ffi::c_int;
                let c2rust_fresh2 = i;
                i += 1;
                map_key
                    |= ((*Event).text.text[c2rust_fresh2 as usize] as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) as UniChar;
                map_key <<= 6 as ::core::ffi::c_int;
                let c2rust_fresh3 = i;
                i += 1;
                map_key
                    |= ((*Event).text.text[c2rust_fresh3 as usize] as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) as UniChar;
            } else if map_key >= 0xf0 as UniChar {
                map_key = 0xfffd as UniChar;
                while (*Event).text.text[i as usize] as UniChar > 0x7f as UniChar {
                    i += 1;
                }
            }
            if map_key == 0 {
                return;
            }
            newtail = kbdtail + 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int;
            if newtail != kbdhead {
                kbdbuf[kbdtail as usize] = map_key;
                kbdtail = newtail;
                lastchar = map_key;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_ResetControls() {
    VControl_ResetInput();
    resetKeyboardState();
    kbdtail = 0 as ::core::ffi::c_int;
    kbdhead = kbdtail;
    lastchar = 0 as UniChar;
}
#[no_mangle]
pub unsafe extern "C" fn InterrogateInputState(
    mut templat: ::core::ffi::c_int,
    mut control: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut buffer: *mut ::core::ffi::c_char,
    mut maxlen: ::core::ffi::c_int,
) {
    let mut g: *mut VCONTROL_GESTURE = controls
        .offset(((templat * num_flight + control) * 2 as ::core::ffi::c_int) as isize)
        .offset(index as isize);
    if templat >= num_templ || control >= num_flight || index >= 2 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"InterrogateInputState(): invalid control index\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        *buffer.offset(0isize) = 0 as ::core::ffi::c_char;
        return;
    }
    match (*g).r#type {
        VCONTROL_GESTURE_TYPE::VCONTROL_KEY => {
            snprintf(
                buffer,
                maxlen as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                VControl_code2name((*g).gesture.key),
            );
            *buffer.offset((maxlen - 1 as ::core::ffi::c_int) as isize) = 0
                as ::core::ffi::c_char;
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON => {
            snprintf(
                buffer,
                maxlen as size_t,
                b"[J%d B%d]\0".as_ptr() as *const ::core::ffi::c_char,
                (*g).gesture.button.port,
                (*g).gesture.button.index + 1 as ::core::ffi::c_int,
            );
            *buffer.offset((maxlen - 1 as ::core::ffi::c_int) as isize) = 0
                as ::core::ffi::c_char;
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS => {
            snprintf(
                buffer,
                maxlen as size_t,
                b"[J%d A%d %c]\0".as_ptr() as *const ::core::ffi::c_char,
                (*g).gesture.axis.port,
                (*g).gesture.axis.index,
                if (*g).gesture.axis.polarity > 0 as ::core::ffi::c_int {
                    '+' as ::core::ffi::c_int
                } else {
                    '-' as ::core::ffi::c_int
                },
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYHAT => {
            snprintf(
                buffer,
                maxlen as size_t,
                b"[J%d H%d %d]\0".as_ptr() as *const ::core::ffi::c_char,
                (*g).gesture.hat.port,
                (*g).gesture.hat.index,
                (*g).gesture.hat.dir as ::core::ffi::c_int,
            );
        }
        _ => {
            *buffer.offset(0isize) = 0 as ::core::ffi::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RemoveInputState(
    mut templat: ::core::ffi::c_int,
    mut control: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) {
    let mut g: *mut VCONTROL_GESTURE = controls
        .offset(((templat * num_flight + control) * 2 as ::core::ffi::c_int) as isize)
        .offset(index as isize);
    let mut keybuf: [::core::ffi::c_char; 40] = [0; 40];
    keybuf[39usize] = '\0' as ::core::ffi::c_char;
    if templat >= num_templ || control >= num_flight || index >= 2 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"RemoveInputState(): invalid control index\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    VControl_RemoveGestureBinding(
        g,
        flight_vec.offset((templat * num_flight) as isize).offset(control as isize)
            as *mut ::core::ffi::c_int,
    );
    (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_NONE;
    snprintf(
        &raw mut keybuf as *mut ::core::ffi::c_char,
        39 as size_t,
        b"keys.%d.%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
        templat + 1 as ::core::ffi::c_int,
        flight_res_names[control as usize],
        index + 1 as ::core::ffi::c_int,
    );
    res_Remove(&raw mut keybuf as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn RebindInputState(
    mut templat: ::core::ffi::c_int,
    mut control: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) {
    let mut g: VCONTROL_GESTURE = VCONTROL_GESTURE {
        r#type: VCONTROL_GESTURE_TYPE::VCONTROL_NONE,
        gesture: C2Rust_Unnamed_3 { key: 0 },
    };
    let mut keybuf: [::core::ffi::c_char; 40] = [0; 40];
    let mut valbuf: [::core::ffi::c_char; 40] = [0; 40];
    valbuf[39usize] = '\0' as ::core::ffi::c_char;
    keybuf[39usize] = valbuf[39usize];
    if templat >= num_templ || control >= num_flight || index >= 2 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"RebindInputState(): invalid control index\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    RemoveInputState(templat, control, index);
    VControl_ClearGesture();
    while VControl_GetLastGesture(&raw mut g) == 0 {
        TaskSwitch();
    }
    VControl_AddGestureBinding(
        &raw mut g,
        flight_vec.offset((templat * num_flight) as isize).offset(control as isize)
            as *mut ::core::ffi::c_int,
    );
    *controls
        .offset(((templat * num_flight + control) * 2 as ::core::ffi::c_int) as isize)
        .offset(index as isize) = g;
    snprintf(
        &raw mut keybuf as *mut ::core::ffi::c_char,
        39 as size_t,
        b"keys.%d.%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
        templat + 1 as ::core::ffi::c_int,
        flight_res_names[control as usize],
        index + 1 as ::core::ffi::c_int,
    );
    VControl_DumpGesture(
        &raw mut valbuf as *mut ::core::ffi::c_char,
        39 as ::core::ffi::c_int,
        &raw mut g,
    );
    res_PutString(
        &raw mut keybuf as *mut ::core::ffi::c_char,
        &raw mut valbuf as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SaveKeyConfiguration(
    mut path: *mut uio_DirHandle,
    mut fname: *const ::core::ffi::c_char,
) {
    SaveResourceIndex(
        path,
        fname,
        b"keys.\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BeginInputFrame() {
    VControl_BeginFrame();
}
