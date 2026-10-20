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

// Rust translation of src/libs/input/sdl/keynames.c.
extern "C" {
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
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
pub type keyname = vcontrol_keyname;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcontrol_keyname {
    pub name: *const ::core::ffi::c_char,
    pub code: ::core::ffi::c_int,
}
static mut keynames: [keyname; 119] = [
    vcontrol_keyname {
        name: b"Backspace\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_BACKSPACE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Tab\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_TAB.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Clear\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_CLEAR.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Return\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RETURN.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Pause\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_PAUSE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Escape\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_ESCAPE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Space\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_SPACE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"!\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_EXCLAIM.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"\"\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_QUOTEDBL.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Hash\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_HASH.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"$\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_DOLLAR.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"&\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_AMPERSAND.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"'\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_QUOTE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"(\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LEFTPAREN.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b")\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RIGHTPAREN.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"*\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_ASTERISK.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"+\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_PLUS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b",\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_COMMA.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"-\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_MINUS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b".\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_PERIOD.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"/\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_SLASH.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"0\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_0.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"1\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_1.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"2\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_2.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"3\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_3.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"4\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_4.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"5\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_5.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"6\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_6.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"7\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_7.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"8\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_8.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"9\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_9.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b":\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_COLON.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b";\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_SEMICOLON.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"<\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LESS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"=\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_EQUALS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b">\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_GREATER.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"?\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_QUESTION.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"@\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_AT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"[\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LEFTBRACKET.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"\\\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_BACKSLASH.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"]\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RIGHTBRACKET.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"^\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_CARET.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"_\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_UNDERSCORE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"`\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_BACKQUOTE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"a\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_a.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"b\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_b.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"c\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_c.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"d\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_d.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"e\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_e.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"f\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_f.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"g\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_g.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"h\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_h.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"i\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_i.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"j\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_j.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"k\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_k.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"l\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_l.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"m\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_m.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"n\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_n.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"o\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_o.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"p\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_p.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"q\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_q.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"r\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_r.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"s\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_s.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"t\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_t.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"u\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_u.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"v\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_v.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"w\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_w.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"x\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_x.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"y\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_y.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"z\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_z.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Delete\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_DELETE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-0\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_0.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-1\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_1.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-2\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_2.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-3\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_3.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-4\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_4.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-5\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_5.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-6\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_6.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-7\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_7.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-8\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_8.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-9\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_9.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-.\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_PERIOD.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-/\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_DIVIDE.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-*\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_MULTIPLY.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad--\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_MINUS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-+\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_PLUS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-Enter\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_ENTER.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Keypad-=\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_KP_EQUALS.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Up\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_UP.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Down\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_DOWN.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Right\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RIGHT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Left\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LEFT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Insert\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_INSERT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Home\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_HOME.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"End\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_END.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"PageUp\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_PAGEUP.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"PageDown\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_PAGEDOWN.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F1\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F1.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F2\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F2.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F3\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F3.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F4\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F4.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F5\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F5.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F6\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F6.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F7\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F7.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F8\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F8.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F9\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F9.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F10\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F10.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F11\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F11.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F12\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F12.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F13\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F13.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F14\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F14.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"F15\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_F15.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"RightShift\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RSHIFT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"LeftShift\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LSHIFT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"RightControl\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RCTRL.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"LeftControl\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LCTRL.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"RightAlt\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_RALT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"LeftAlt\0".as_ptr() as *const ::core::ffi::c_char,
        code: C2Rust_Unnamed::SDLK_LALT.0 as ::core::ffi::c_int,
    },
    vcontrol_keyname {
        name: b"Unknown\0".as_ptr() as *const ::core::ffi::c_char,
        code: 0 as ::core::ffi::c_int,
    },
];
#[no_mangle]
pub unsafe extern "C" fn VControl_code2name(
    mut code: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut test: ::core::ffi::c_int = keynames[i as usize].code;
        if test == code || test == 0 {
            return keynames[i as usize].name;
        }
        i += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn VControl_name2code(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut test: *const ::core::ffi::c_char = keynames[i as usize].name;
        let mut code: ::core::ffi::c_int = keynames[i as usize].code;
        if strcasecmp(test, name) == 0 || code == 0 {
            return code;
        }
        i += 1;
    };
}
