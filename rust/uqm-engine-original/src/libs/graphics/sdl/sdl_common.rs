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

// Rust translation of src/libs/graphics/sdl/sdl_common.c.
extern "C" {
    pub type SDL_BlitMap;
    pub type SDL_SysWMmsg;
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    fn SDL_ConvertSurface(
        src: *mut SDL_Surface,
        fmt: *const SDL_PixelFormat,
        flags: Uint32,
    ) -> *mut SDL_Surface;
    fn SDL_ShowCursor(toggle: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SDL_PollEvent(event: *mut SDL_Event) -> ::core::ffi::c_int;
    fn TFB_DrawCanvas_Initialize();
    static mut TransitionAmount: ::core::ffi::c_int;
    static mut TransitionClipRect: RECT;
    fn Init_DrawCommandQueue();
    fn Uninit_DrawCommandQueue();
    fn ProcessInputEvent(Event: *const SDL_Event);
    static mut TFB_BBox: TFB_BoundingBox;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn TFB_Pure_InitGraphics(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        renderer: *const ::core::ffi::c_char,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn TFB_Pure_UninitGraphics();
    fn GetFadeAmount() -> ::core::ffi::c_int;
    fn TFB_HasSurfaceAlphaMod(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn TFB_DisableSurfaceAlphaMod(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn TFB_GetColorKey(
        surface: *mut SDL_Surface,
        key: *mut Uint32,
    ) -> ::core::ffi::c_int;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: ::core::ffi::c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut SDL_PixelFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub pitch: ::core::ffi::c_int,
    pub pixels: *mut ::core::ffi::c_void,
    pub userdata: *mut ::core::ffi::c_void,
    pub locked: ::core::ffi::c_int,
    pub list_blitmap: *mut ::core::ffi::c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SDL_WINDOWEVENT_NONE: Self = Self(0);
    pub const SDL_WINDOWEVENT_SHOWN: Self = Self(1);
    pub const SDL_WINDOWEVENT_HIDDEN: Self = Self(2);
    pub const SDL_WINDOWEVENT_EXPOSED: Self = Self(3);
    pub const SDL_WINDOWEVENT_MOVED: Self = Self(4);
    pub const SDL_WINDOWEVENT_RESIZED: Self = Self(5);
    pub const SDL_WINDOWEVENT_SIZE_CHANGED: Self = Self(6);
    pub const SDL_WINDOWEVENT_MINIMIZED: Self = Self(7);
    pub const SDL_WINDOWEVENT_MAXIMIZED: Self = Self(8);
    pub const SDL_WINDOWEVENT_RESTORED: Self = Self(9);
    pub const SDL_WINDOWEVENT_ENTER: Self = Self(10);
    pub const SDL_WINDOWEVENT_LEAVE: Self = Self(11);
    pub const SDL_WINDOWEVENT_FOCUS_GAINED: Self = Self(12);
    pub const SDL_WINDOWEVENT_FOCUS_LOST: Self = Self(13);
    pub const SDL_WINDOWEVENT_CLOSE: Self = Self(14);
    pub const SDL_WINDOWEVENT_TAKE_FOCUS: Self = Self(15);
    pub const SDL_WINDOWEVENT_HIT_TEST: Self = Self(16);
    pub const SDL_WINDOWEVENT_ICCPROF_CHANGED: Self = Self(17);
    pub const SDL_WINDOWEVENT_DISPLAY_CHANGED: Self = Self(18);
}
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
pub type sint16 = ::core::ffi::c_short;
pub type SWORD = sint16;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SCREEN(pub ::core::ffi::c_uint);
impl SCREEN {
    pub const TFB_SCREEN_MAIN: Self = Self(0);
    pub const TFB_SCREEN_EXTRA: Self = Self(1);
    pub const TFB_SCREEN_TRANSITION: Self = Self(2);
    pub const TFB_GFX_NUMSCREENS: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const TFB_GFXDRIVER_SDL_OPENGL: Self = Self(0);
    pub const TFB_GFXDRIVER_SDL_PURE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const TFB_REDRAW_NO: Self = Self(0);
    pub const TFB_REDRAW_FADING: Self = Self(1);
    pub const TFB_REDRAW_EXPOSE: Self = Self(2);
    pub const TFB_REDRAW_YES: Self = Self(3);
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
pub type TFB_GRAPHICS_BACKEND = _tfb_graphics_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tfb_graphics_backend {
    pub preprocess: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub postprocess: Option<unsafe extern "C" fn() -> ()>,
    pub uploadTransitionScreen: Option<unsafe extern "C" fn() -> ()>,
    pub screen: Option<unsafe extern "C" fn(SCREEN, Uint8, *mut SDL_Rect) -> ()>,
    pub color: Option<
        unsafe extern "C" fn(Uint8, Uint8, Uint8, Uint8, *mut SDL_Rect) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_BoundingBox {
    pub valid: ::core::ffi::c_int,
    pub region: RECT,
    pub clip: RECT,
}
#[no_mangle]
pub static mut SDL_Screen: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
#[no_mangle]
pub static mut TransitionScreen: *mut SDL_Surface = ::core::ptr::null_mut::<
    SDL_Surface,
>();
#[no_mangle]
pub static mut SDL_Screens: [*mut SDL_Surface; 3] = [::core::ptr::null_mut::<
    SDL_Surface,
>(); 3];
#[no_mangle]
pub static mut format_conv_surf: *mut SDL_Surface = ::core::ptr::null_mut::<
    SDL_Surface,
>();
#[no_mangle]
pub static mut GfxFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut graphics_backend: *mut TFB_GRAPHICS_BACKEND = ::core::ptr::null_mut::<
    TFB_GRAPHICS_BACKEND,
>();
#[no_mangle]
pub static mut QuitPosted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut GameActive: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn TFB_InitGraphics(
    mut driver: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut renderer: *const ::core::ffi::c_char,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut caption: [::core::ffi::c_char; 200] = [0; 200];
    i = 0 as ::core::ffi::c_int;
    while i < SCREEN::TFB_GFX_NUMSCREENS.0 as ::core::ffi::c_int {
        SDL_Screens[i as usize] = ::core::ptr::null_mut::<SDL_Surface>();
        i += 1;
    }
    GfxFlags = flags;
    if driver == C2Rust_Unnamed_1::TFB_GFXDRIVER_SDL_OPENGL.0 as ::core::ffi::c_int {
        driver = C2Rust_Unnamed_1::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int;
        log_add(
            log_Level::log_Warning,
            b"OpenGL support not compiled in, so using pure SDL driver\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        result = TFB_Pure_InitGraphics(driver, flags, renderer, width, height);
    } else {
        result = TFB_Pure_InitGraphics(driver, flags, renderer, width, height);
    }
    if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        SDL_ShowCursor(0 as ::core::ffi::c_int);
    }
    Init_DrawCommandQueue();
    TFB_DrawCanvas_Initialize();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_UninitGraphics() {
    let mut i: ::core::ffi::c_int = 0;
    Uninit_DrawCommandQueue();
    i = 0 as ::core::ffi::c_int;
    while i < SCREEN::TFB_GFX_NUMSCREENS.0 as ::core::ffi::c_int {
        UnInit_Screen(
            (&raw mut SDL_Screens as *mut *mut SDL_Surface).offset(i as isize),
        );
        i += 1;
    }
    TFB_Pure_UninitGraphics();
    UnInit_Screen(&raw mut format_conv_surf);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_ProcessEvents() {
    let mut Event: SDL_Event = SDL_Event { r#type: 0 };
    while SDL_PollEvent(&raw mut Event) > 0 as ::core::ffi::c_int {
        ProcessInputEvent(&raw mut Event);
        match Event.r#type {
            256 => {
                ::core::ptr::write_volatile(
                    &raw mut QuitPosted,
                    1 as ::core::ffi::c_int,
                );
            }
            512 => {
                if Event.window.event as ::core::ffi::c_int
                    == C2Rust_Unnamed::SDL_WINDOWEVENT_EXPOSED.0 as ::core::ffi::c_int
                {
                    TFB_SwapBuffers(
                        C2Rust_Unnamed_2::TFB_REDRAW_EXPOSE.0 as ::core::ffi::c_int,
                    );
                }
            }
            _ => {}
        }
    }
}
static mut system_box_active: BOOLEAN = BOOLEAN(
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
);
static mut system_box: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
#[no_mangle]
pub unsafe extern "C" fn SetSystemRect(mut r: *const RECT) {
    system_box_active = BOOLEAN::TRUE;
    system_box.x = (*r).corner.x as ::core::ffi::c_int;
    system_box.y = (*r).corner.y as ::core::ffi::c_int;
    system_box.w = (*r).extent.width as ::core::ffi::c_int;
    system_box.h = (*r).extent.height as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ClearSystemRect() {
    system_box_active = BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SwapBuffers(mut force_full_redraw: ::core::ffi::c_int) {
    static mut last_fade_amount: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
    static mut last_transition_amount: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
    static mut fade_amount: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
    static mut transition_amount: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
    fade_amount = GetFadeAmount();
    transition_amount = ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(&raw const TransitionAmount);
    if force_full_redraw == C2Rust_Unnamed_2::TFB_REDRAW_NO.0 as ::core::ffi::c_int
        && TFB_BBox.valid == 0 && fade_amount == 255 as ::core::ffi::c_int
        && transition_amount == 255 as ::core::ffi::c_int
        && last_fade_amount == 255 as ::core::ffi::c_int
        && last_transition_amount == 255 as ::core::ffi::c_int
    {
        return;
    }
    if force_full_redraw == C2Rust_Unnamed_2::TFB_REDRAW_NO.0 as ::core::ffi::c_int
        && (fade_amount != 255 as ::core::ffi::c_int
            || transition_amount != 255 as ::core::ffi::c_int
            || last_fade_amount != 255 as ::core::ffi::c_int
            || last_transition_amount != 255 as ::core::ffi::c_int)
    {
        force_full_redraw = C2Rust_Unnamed_2::TFB_REDRAW_FADING.0 as ::core::ffi::c_int;
    }
    last_fade_amount = fade_amount;
    last_transition_amount = transition_amount;
    (*graphics_backend)
        .preprocess
        .expect(
            "non-null function pointer",
        )(force_full_redraw, transition_amount, fade_amount);
    (*graphics_backend)
        .screen
        .expect(
            "non-null function pointer",
        )(SCREEN::TFB_SCREEN_MAIN, 255 as Uint8, ::core::ptr::null_mut::<SDL_Rect>());
    if transition_amount != 255 as ::core::ffi::c_int {
        let mut r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
        r.x = TransitionClipRect.corner.x as ::core::ffi::c_int;
        r.y = TransitionClipRect.corner.y as ::core::ffi::c_int;
        r.w = TransitionClipRect.extent.width as ::core::ffi::c_int;
        r.h = TransitionClipRect.extent.height as ::core::ffi::c_int;
        (*graphics_backend)
            .screen
            .expect(
                "non-null function pointer",
            )(
            SCREEN::TFB_SCREEN_TRANSITION,
            (255 as ::core::ffi::c_int - transition_amount) as Uint8,
            &raw mut r,
        );
    }
    if fade_amount != 255 as ::core::ffi::c_int {
        if fade_amount < 255 as ::core::ffi::c_int {
            (*graphics_backend)
                .color
                .expect(
                    "non-null function pointer",
                )(
                0 as Uint8,
                0 as Uint8,
                0 as Uint8,
                (255 as ::core::ffi::c_int - fade_amount) as Uint8,
                ::core::ptr::null_mut::<SDL_Rect>(),
            );
        } else {
            (*graphics_backend)
                .color
                .expect(
                    "non-null function pointer",
                )(
                255 as Uint8,
                255 as Uint8,
                255 as Uint8,
                (fade_amount - 255 as ::core::ffi::c_int) as Uint8,
                ::core::ptr::null_mut::<SDL_Rect>(),
            );
        }
    }
    if system_box_active.0 != 0 {
        (*graphics_backend)
            .screen
            .expect(
                "non-null function pointer",
            )(SCREEN::TFB_SCREEN_MAIN, 255 as Uint8, &raw mut system_box);
    }
    (*graphics_backend).postprocess.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DisplayFormatAlpha(
    mut surface: *mut SDL_Surface,
) -> *mut SDL_Surface {
    let mut newsurf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut dstfmt: *mut SDL_PixelFormat = ::core::ptr::null_mut::<SDL_PixelFormat>();
    let mut srcfmt: *const SDL_PixelFormat = (*surface).format;
    if (*(*surface).format).Amask != 0 {
        dstfmt = (*format_conv_surf).format;
    } else {
        dstfmt = (*SDL_Screen).format;
    }
    if (*srcfmt).BytesPerPixel as ::core::ffi::c_int
        == (*dstfmt).BytesPerPixel as ::core::ffi::c_int
        && (*srcfmt).Rmask == (*dstfmt).Rmask && (*srcfmt).Gmask == (*dstfmt).Gmask
        && (*srcfmt).Bmask == (*dstfmt).Bmask && (*srcfmt).Amask == (*dstfmt).Amask
    {
        return surface;
    }
    newsurf = SDL_ConvertSurface(surface, dstfmt, (*surface).flags);
    if TFB_HasColorKey(surface) != 0 && !newsurf.is_null()
        && TFB_HasColorKey(newsurf) != 0 && TFB_HasSurfaceAlphaMod(newsurf) != 0
    {
        TFB_DisableSurfaceAlphaMod(newsurf);
    }
    return newsurf;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetScreenCanvas(mut screen: SCREEN) -> TFB_Canvas {
    return SDL_Screens[screen.0 as usize] as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_UploadTransitionScreen() {
    (*graphics_backend).uploadTransitionScreen.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn TFB_HasColorKey(
    mut surface: *mut SDL_Surface,
) -> ::core::ffi::c_int {
    let mut key: Uint32 = 0;
    return (TFB_GetColorKey(surface, &raw mut key) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UnInit_Screen(mut screen: *mut *mut SDL_Surface) {
    if (*screen).is_null() {
        return;
    }
    SDL_FreeSurface(*screen);
    *screen = ::core::ptr::null_mut::<SDL_Surface>();
}
