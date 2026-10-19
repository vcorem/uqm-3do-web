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

// Rust translation of src/uqm/comm/slyhome/slyhome.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn AddEvent(
        r#type: EVENT_TYPE,
        month_index: COUNT,
        day_index: COUNT,
        year_index: COUNT,
        func_index: BYTE,
    ) -> HEVENT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    fn EngageFilters(f_desc: *mut FILTER_DESC);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut CommData: LOCDATA;
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn setSegue(segue: Segue);
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
pub type HEVENT = HLINK;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_VALIGN(pub ::core::ffi::c_uint);
impl TEXT_VALIGN {
    pub const VALIGN_TOP: Self = Self(0);
    pub const VALIGN_MIDDLE: Self = Self(1);
    pub const VALIGN_BOTTOM: Self = Self(2);
}
pub type COLORMAP = STRING;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_MULTIPLY: Self = Self(3);
    pub const DRAW_OVERLAY: Self = Self(4);
    pub const DRAW_SCREEN: Self = Self(5);
    pub const DRAW_GRAYSCALE: Self = Self(6);
    pub const DRAW_LINEARBURN: Self = Self(7);
    pub const DRAW_HYPTOQUAS: Self = Self(8);
    pub const DRAW_DESATURATE: Self = Self(9);
    pub const DRAW_DEFAULT: Self = Self(0);
}
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIMATION_DESC {
    pub StartIndex: COUNT,
    pub NumFrames: BYTE,
    pub AnimFlags: COUNT,
    pub BaseFrameRate: COUNT,
    pub RandomFrameRate: COUNT,
    pub BaseRestartRate: COUNT,
    pub RandomRestartRate: COUNT,
    pub BlockMask: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER {
    pub ColorIndex: BYTE,
    pub OpacityIndex: BYTE,
    pub FrameIndex: SIZE,
    pub Kind: BYTE,
    pub Flags: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER_DESC {
    pub NumFilters: BYTE,
    pub FilterArray: [FILTER; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEECH_DIGITNAME {
    pub Divider: ::core::ffi::c_int,
    pub MaxRemainder: ::core::ffi::c_int,
    pub StrIndex: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEECH_DIGIT {
    pub Divider: ::core::ffi::c_int,
    pub Subtrahend: ::core::ffi::c_int,
    pub StrDigits: *mut COUNT,
    pub Names: *mut SPEECH_DIGITNAME,
    pub CommonNameIndex: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NUMBER_SPEECH_DESC {
    pub NumDigits: COUNT,
    pub Digits: [SPEECH_DIGIT; 7],
}
pub type NUMBER_SPEECH = *const NUMBER_SPEECH_DESC;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONVERSATION(pub ::core::ffi::c_uint);
impl CONVERSATION {
    pub const ARILOU_CONVERSATION: Self = Self(0);
    pub const CHMMR_CONVERSATION: Self = Self(1);
    pub const COMMANDER_CONVERSATION: Self = Self(2);
    pub const ORZ_CONVERSATION: Self = Self(3);
    pub const PKUNK_CONVERSATION: Self = Self(4);
    pub const SHOFIXTI_CONVERSATION: Self = Self(5);
    pub const SPATHI_CONVERSATION: Self = Self(6);
    pub const SUPOX_CONVERSATION: Self = Self(7);
    pub const THRADD_CONVERSATION: Self = Self(8);
    pub const UTWIG_CONVERSATION: Self = Self(9);
    pub const VUX_CONVERSATION: Self = Self(10);
    pub const YEHAT_CONVERSATION: Self = Self(11);
    pub const MELNORME_CONVERSATION: Self = Self(12);
    pub const DRUUGE_CONVERSATION: Self = Self(13);
    pub const ILWRATH_CONVERSATION: Self = Self(14);
    pub const MYCON_CONVERSATION: Self = Self(15);
    pub const SLYLANDRO_CONVERSATION: Self = Self(16);
    pub const UMGAH_CONVERSATION: Self = Self(17);
    pub const URQUAN_CONVERSATION: Self = Self(18);
    pub const ZOQFOTPIK_CONVERSATION: Self = Self(19);
    pub const SYREEN_CONVERSATION: Self = Self(20);
    pub const BLACKURQ_CONVERSATION: Self = Self(21);
    pub const TALKING_PET_CONVERSATION: Self = Self(22);
    pub const SLYLANDRO_HOME_CONVERSATION: Self = Self(23);
    pub const URQUAN_DRONE_CONVERSATION: Self = Self(24);
    pub const YEHAT_REBEL_CONVERSATION: Self = Self(25);
    pub const INVALID_CONVERSATION: Self = Self(26);
    pub const NUM_CONVERSATIONS: Self = Self(27);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ALT_RESOURCE {
    pub AlienFrameRes: RESOURCE,
    pub AlienColorMapRes: RESOURCE,
    pub AlienSongRes: RESOURCE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LOCDATA {
    pub AlienConv: CONVERSATION,
    pub init_encounter_func: Option<unsafe extern "C" fn() -> ()>,
    pub post_encounter_func: Option<unsafe extern "C" fn() -> ()>,
    pub uninit_encounter_func: Option<unsafe extern "C" fn() -> COUNT>,
    pub AlienFrameRes: RESOURCE,
    pub AlienFontRes: RESOURCE,
    pub AlienTextFColor: Color,
    pub AlienTextBColor: Color,
    pub AlienTextBaseline: POINT,
    pub AlienTextWidth: COUNT,
    pub AlienTextAlign: TEXT_ALIGN,
    pub AlienTextValign: TEXT_VALIGN,
    pub AlienColorMapRes: RESOURCE,
    pub AlienSongRes: RESOURCE,
    pub AltRes: ALT_RESOURCE,
    pub ConversationPhrasesRes: RESOURCE,
    pub NumAnimations: COUNT,
    pub AlienAmbientArray: [ANIMATION_DESC; 30],
    pub AlienTransitionDesc: ANIMATION_DESC,
    pub AlienTalkDesc: ANIMATION_DESC,
    pub AlienNumberSpeech: NUMBER_SPEECH,
    pub AlienFrame: FRAME,
    pub AlienFont: FONT,
    pub AlienColorMap: COLORMAP,
    pub AlienSong: MUSIC_REF,
    pub ConversationPhrases: STRING,
}
pub type CallbackArg = *mut ::core::ffi::c_void;
pub type CallbackFunction = Option<unsafe extern "C" fn(CallbackArg) -> ()>;
pub type RESPONSE_REF = COUNT;
pub type RESPONSE_FUNC = Option<unsafe extern "C" fn(RESPONSE_REF) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Segue(pub ::core::ffi::c_uint);
impl Segue {
    pub const Segue_peace: Self = Self(0);
    pub const Segue_hostile: Self = Self(1);
    pub const Segue_victory: Self = Self(2);
    pub const Segue_defeat: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const HELLO_1: Self = Self(1);
    pub const HELLO_2: Self = Self(2);
    pub const HELLO_3: Self = Self(3);
    pub const HELLO_4: Self = Self(4);
    pub const RECALL_PROGRAM_1: Self = Self(5);
    pub const we_are_us: Self = Self(6);
    pub const TERRIBLY_EXCITING: Self = Self(7);
    pub const happy_to_tell_more: Self = Self(8);
    pub const TELL_MORE: Self = Self(9);
    pub const would_you_like_to_know_more: Self = Self(10);
    pub const YES_TELL_MORE: Self = Self(11);
    pub const we_come_from_earth: Self = Self(12);
    pub const OK_EARTH: Self = Self(13);
    pub const we_explore: Self = Self(14);
    pub const OK_EXPLORE: Self = Self(15);
    pub const we_fight_urquan: Self = Self(16);
    pub const URQUAN_NICE_GUYS: Self = Self(17);
    pub const not_same_urquan: Self = Self(18);
    pub const PERSONALITY_CHANGE: Self = Self(19);
    pub const we_gather: Self = Self(20);
    pub const MAYBE_INTERESTED: Self = Self(21);
    pub const enough_about_me: Self = Self(22);
    pub const OK_ENOUGH_YOU: Self = Self(23);
    pub const what_other_visitors: Self = Self(24);
    pub const VISITORS: Self = Self(25);
    pub const any_other_visitors: Self = Self(26);
    pub const LONG_AGO: Self = Self(27);
    pub const what_about_sentient_milieu: Self = Self(28);
    pub const MET_TAALO_THEY_ARE_FROM: Self = Self(29);
    pub const who_else: Self = Self(30);
    pub const PRECURSORS: Self = Self(31);
    pub const precursors_yow: Self = Self(32);
    pub const ABOUT_PRECURSORS: Self = Self(33);
    pub const must_know_more: Self = Self(34);
    pub const ALL_WE_KNOW: Self = Self(35);
    pub const who_are_you: Self = Self(36);
    pub const WE_ARE_SLY: Self = Self(37);
    pub const like_more_about_you: Self = Self(38);
    pub const SURE_KNOW_WHAT: Self = Self(39);
    pub const what_about_home: Self = Self(40);
    pub const ABOUT_HOME: Self = Self(41);
    pub const what_about_culture: Self = Self(42);
    pub const ABOUT_CULTURE: Self = Self(43);
    pub const what_about_history: Self = Self(44);
    pub const ABOUT_HISTORY: Self = Self(45);
    pub const what_about_biology: Self = Self(46);
    pub const ABOUT_BIOLOGY: Self = Self(47);
    pub const enough_info: Self = Self(48);
    pub const OK_ENOUGH_INFO: Self = Self(49);
    pub const where_are_you: Self = Self(50);
    pub const DOWN_HERE: Self = Self(51);
    pub const thats_impossible_1: Self = Self(52);
    pub const NO_ITS_NOT_1: Self = Self(53);
    pub const thats_impossible_2: Self = Self(54);
    pub const NO_ITS_NOT_2: Self = Self(55);
    pub const bye: Self = Self(56);
    pub const GOODBYE_1: Self = Self(57);
    pub const GOODBYE_2: Self = Self(58);
    pub const what_are_probes: Self = Self(59);
    pub const PROBES_ARE: Self = Self(60);
    pub const know_more_probe: Self = Self(61);
    pub const OK_WHAT_MORE_PROBE: Self = Self(62);
    pub const where_probes_from: Self = Self(63);
    pub const PROBES_FROM_MELNORME: Self = Self(64);
    pub const why_sell: Self = Self(65);
    pub const SELL_FOR_INFO: Self = Self(66);
    pub const how_long_ago: Self = Self(67);
    pub const FIFTY_THOUSAND_ROTATIONS: Self = Self(68);
    pub const whats_probes_mission: Self = Self(69);
    pub const SEEK_OUT_NEW_LIFE: Self = Self(70);
    pub const if_only_one: Self = Self(71);
    pub const THEY_REPLICATE: Self = Self(72);
    pub const enough_probe: Self = Self(73);
    pub const OK_ENOUGH_PROBE: Self = Self(74);
    pub const why_probe_always_attack: Self = Self(75);
    pub const ONLY_DEFEND: Self = Self(76);
    pub const talk_more_probe_attack: Self = Self(77);
    pub const NO_PROBLEM_BUT_SURE: Self = Self(78);
    pub const tell_me_about_basics: Self = Self(79);
    pub const BASIC_COMMANDS: Self = Self(80);
    pub const tell_basics_again: Self = Self(81);
    pub const OK_BASICS_AGAIN: Self = Self(82);
    pub const what_effect: Self = Self(83);
    pub const AFFECTS_BEHAVIOR: Self = Self(84);
    pub const how_does_probe_defend: Self = Self(85);
    pub const ONLY_SELF_DEFENSE: Self = Self(86);
    pub const combat_behavior: Self = Self(87);
    pub const MISSILE_BATTERIES: Self = Self(88);
    pub const what_missile_batteries: Self = Self(89);
    pub const LIGHTNING_ONLY_FOR_HARVESTING: Self = Self(90);
    pub const tell_me_about_rep_1: Self = Self(91);
    pub const ABOUT_REP: Self = Self(92);
    pub const what_set_priority: Self = Self(93);
    pub const MAXIMUM: Self = Self(94);
    pub const enough_problem: Self = Self(95);
    pub const OK_ENOUGH_PROBLEM: Self = Self(96);
    pub const probe_has_bug: Self = Self(97);
    pub const NO_IT_DOESNT: Self = Self(98);
    pub const tell_me_about_attack: Self = Self(99);
    pub const ATTACK_NO_PROBLEM: Self = Self(100);
    pub const tell_me_about_rep_2: Self = Self(101);
    pub const REP_NO_PROBLEM: Self = Self(102);
    pub const what_about_rep_priorities: Self = Self(103);
    pub const MAXIMUM_SO_WHAT: Self = Self(104);
    pub const think_about_rep_priorities: Self = Self(105);
    pub const UH_OH: Self = Self(106);
    pub const hunt_them_down: Self = Self(107);
    pub const GROW_TOO_FAST: Self = Self(108);
    pub const sue_melnorme: Self = Self(109);
    pub const SIGNED_WAIVER: Self = Self(110);
    pub const recall_signal: Self = Self(111);
    pub const NOT_THIS_MODEL: Self = Self(112);
    pub const mega_self_destruct: Self = Self(113);
    pub const WHY_YES_THERE_IS: Self = Self(114);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const ARILOU_ENTRANCE_EVENT: Self = Self(0);
    pub const ARILOU_EXIT_EVENT: Self = Self(1);
    pub const HYPERSPACE_ENCOUNTER_EVENT: Self = Self(2);
    pub const KOHR_AH_VICTORIOUS_EVENT: Self = Self(3);
    pub const ADVANCE_PKUNK_MISSION: Self = Self(4);
    pub const ADVANCE_THRADD_MISSION: Self = Self(5);
    pub const ZOQFOT_DISTRESS_EVENT: Self = Self(6);
    pub const ZOQFOT_DEATH_EVENT: Self = Self(7);
    pub const SHOFIXTI_RETURN_EVENT: Self = Self(8);
    pub const ADVANCE_UTWIG_SUPOX_MISSION: Self = Self(9);
    pub const KOHR_AH_GENOCIDE_EVENT: Self = Self(10);
    pub const SPATHI_SHIELD_EVENT: Self = Self(11);
    pub const ADVANCE_ILWRATH_MISSION: Self = Self(12);
    pub const ADVANCE_MYCON_MISSION: Self = Self(13);
    pub const ARILOU_UMGAH_CHECK: Self = Self(14);
    pub const YEHAT_REBEL_EVENT: Self = Self(15);
    pub const SLYLANDRO_RAMP_UP: Self = Self(16);
    pub const SLYLANDRO_RAMP_DOWN: Self = Self(17);
    pub const NUM_EVENTS: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const CHMMR_HOME: Self = Self(0);
    pub const ORZ_HOME: Self = Self(1);
    pub const PKUNK_HOME: Self = Self(2);
    pub const SHOFIXTI_HOME: Self = Self(3);
    pub const SPATHI_HOME: Self = Self(4);
    pub const SUPOX_HOME: Self = Self(5);
    pub const THRADDASH_HOME: Self = Self(6);
    pub const UTWIG_HOME: Self = Self(7);
    pub const VUX_HOME: Self = Self(8);
    pub const YEHAT_HOME: Self = Self(9);
    pub const DRUUGE_HOME: Self = Self(10);
    pub const ILWRATH_HOME: Self = Self(11);
    pub const MYCON_HOME: Self = Self(12);
    pub const SLYLANDRO_HOME: Self = Self(13);
    pub const UMGAH_HOME: Self = Self(14);
    pub const ZOQFOTPIK_HOME: Self = Self(15);
    pub const SYREEN_HOME: Self = Self(16);
    pub const ANDROSYNTH_HOME: Self = Self(17);
    pub const NUM_AVAILABLE_HOMES: Self = Self(18);
}
#[inline]
unsafe extern "C" fn IsHomeworldKnown(mut homeworld: DWORD) -> BOOLEAN {
    if homeworld > 18 as DWORD {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(
        uqm_lua_checked!(getGameStateUint(b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            & ((1 as ::core::ffi::c_int) << homeworld) as DWORD,
    );
}
#[inline]
unsafe extern "C" fn SetHomeworldKnown(mut homeworld: DWORD) {
    let mut current: DWORD = 0;
    if uqm_lua_checked!(IsHomeworldKnown(homeworld), ()).0 != 0 {
        return;
    }
    current = uqm_lua_checked!(getGameStateUint(
        b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    current |= ((1 as ::core::ffi::c_int) << homeworld) as DWORD;
    uqm_lua_checked!(setGameStateUint(
        b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char,
        current,
    ), ());
}
static mut slylandro_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::SLYLANDRO_HOME_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.slylandro.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.slylandro.font\0".as_ptr() as RESOURCE,
    AlienTextFColor: Color {
        r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        a: 0xff as BYTE,
    },
    AlienTextBColor: Color {
        r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        a: 0xff as BYTE,
    },
    AlienTextBaseline: point {
        x: 0 as COORD,
        y: 0 as COORD,
    },
    AlienTextWidth: 0 as COUNT,
    AlienTextAlign: TEXT_ALIGN::ALIGN_CENTER,
    AlienTextValign: TEXT_VALIGN::VALIGN_TOP,
    AlienColorMapRes: b"comm.slylandro.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.slylandro.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.slylandro.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 13 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 0 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                / 8 as ::core::ffi::c_int) as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                / 8 as ::core::ffi::c_int) as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 6 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 11 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 16 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 22 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 30 as COUNT,
            NumFrames: 9 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 39 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 43 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 48 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 54 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 60 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 67 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
        ANIMATION_DESC {
            StartIndex: 0,
            NumFrames: 0,
            AnimFlags: 0,
            BaseFrameRate: 0,
            RandomFrameRate: 0,
            BaseRestartRate: 0,
            RandomRestartRate: 0,
            BlockMask: 0,
        },
    ],
    AlienTransitionDesc: ANIMATION_DESC {
        StartIndex: 0 as COUNT,
        NumFrames: 0 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: 0 as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: 0 as COUNT,
        RandomRestartRate: 0 as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienTalkDesc: ANIMATION_DESC {
        StartIndex: 0 as COUNT,
        NumFrames: 0 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: 0 as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: 0 as COUNT,
        RandomRestartRate: 0 as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienNumberSpeech: ::core::ptr::null::<NUMBER_SPEECH_DESC>(),
    AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
    AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
    ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
};
static mut slyhome_filters: FILTER_DESC = FILTER_DESC {
    NumFilters: 1 as BYTE,
    FilterArray: [
        FILTER {
            ColorIndex: 0 as BYTE,
            OpacityIndex: 1 as BYTE,
            FrameIndex: -1 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_OVERLAY.0 as ::core::ffi::c_int as BYTE,
            Flags: 0 as BYTE,
        },
        FILTER {
            ColorIndex: 0,
            OpacityIndex: 0,
            FrameIndex: 0,
            Kind: 0,
            Flags: 0,
        },
        FILTER {
            ColorIndex: 0,
            OpacityIndex: 0,
            FrameIndex: 0,
            Kind: 0,
            Flags: 0,
        },
    ],
};
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        1 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_1.0 as ::core::ffi::c_int, None), ());
        }
        _ => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_2.0 as ::core::ffi::c_int, None), ());
        }
    };
}
unsafe extern "C" fn HumanInfo(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::happy_to_tell_more.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::TELL_MORE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::would_you_like_to_know_more.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::YES_TELL_MORE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_come_from_earth.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_EARTH.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_KNOW_EARTH\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_explore.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_EXPLORE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_KNOW_EXPLORE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_fight_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::URQUAN_NICE_GUYS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_KNOW_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::not_same_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::PERSONALITY_CHANGE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_KNOW_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_gather.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::MAYBE_INTERESTED.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_KNOW_GATHER\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    }
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_KNOW_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 1 as DWORD
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::not_same_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"SLYLANDRO_KNOW_EARTH\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_come_from_earth.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_KNOW_EXPLORE\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_explore.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_KNOW_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_fight_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_KNOW_GATHER\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_gather.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_about_me.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(HomeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if InfoLeft == 0 {
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    }
}
unsafe extern "C" fn SlylandroInfo(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::like_more_about_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SURE_KNOW_WHAT.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_home.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_HOME.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_home.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_culture.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_CULTURE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_culture.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_history.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_HISTORY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_history.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_biology.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_BIOLOGY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_biology.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_home.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_home.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SlylandroInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_culture.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_culture.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SlylandroInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_history.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_history.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SlylandroInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_biology.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_biology.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SlylandroInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(HomeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if InfoLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::like_more_about_you.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn FixBug(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::think_about_rep_priorities.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::UH_OH.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::hunt_them_down.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GROW_TOO_FAST.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::hunt_them_down.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::sue_melnorme.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SIGNED_WAIVER.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::sue_melnorme.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::recall_signal.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NOT_THIS_MODEL.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::recall_signal.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::hunt_them_down.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::hunt_them_down.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(FixBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::sue_melnorme.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::sue_melnorme.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(FixBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::recall_signal.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::recall_signal.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(FixBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::mega_self_destruct.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(HomeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ProbeBug(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::probe_has_bug.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_IT_DOESNT.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_me_about_rep_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::REP_NO_PROBLEM.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tell_me_about_rep_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_rep_priorities.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::MAXIMUM_SO_WHAT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_rep_priorities.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_me_about_attack.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ATTACK_NO_PROBLEM.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tell_me_about_attack.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::tell_me_about_rep_2.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::tell_me_about_rep_2.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ProbeBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_rep_priorities.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_rep_priorities.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ProbeBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::think_about_rep_priorities.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(FixBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::tell_me_about_attack.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::tell_me_about_attack.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ProbeBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
}
unsafe extern "C" fn ProbeFunction(mut R: RESPONSE_REF) {
    let mut LastStack: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 2] = [0; 2];
    LastStack = 0 as BYTE;
    pStr[1usize] = 0 as RESPONSE_REF;
    pStr[0usize] = pStr[1usize];
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::talk_more_probe_attack.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NO_PROBLEM_BUT_SURE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_me_about_basics.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::BASIC_COMMANDS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PLAYER_KNOWS_PROGRAM\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tell_basics_again.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_basics_again.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_BASICS_AGAIN.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tell_basics_again.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_effect.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::AFFECTS_BEHAVIOR.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PLAYER_KNOWS_EFFECTS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_effect.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_me_about_rep_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_REP.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_set_priority.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::MAXIMUM.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PLAYER_KNOWS_PRIORITY\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_set_priority.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_does_probe_defend.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ONLY_SELF_DEFENSE.0 as ::core::ffi::c_int, None), ());
        LastStack = 1 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK9\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::combat_behavior.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::MISSILE_BATTERIES.0 as ::core::ffi::c_int, None), ());
        LastStack = 1 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK9\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_missile_batteries.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::LIGHTNING_ONLY_FOR_HARVESTING.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK9\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK9\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            pStr[0usize] = C2Rust_Unnamed_0::how_does_probe_defend.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        1 => {
            pStr[0usize] = C2Rust_Unnamed_0::combat_behavior.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        2 => {
            pStr[0usize] = C2Rust_Unnamed_0::what_missile_batteries.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        _ => {}
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        2 => {
            pStr[1usize] = C2Rust_Unnamed_0::tell_me_about_rep_1.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        3 => {
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_0::what_set_priority.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                pStr[1usize] = C2Rust_Unnamed_0::what_set_priority.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
        }
        _ => {}
    }
    if LastStack as ::core::ffi::c_int != 0
        && pStr[(LastStack as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            pStr[(LastStack as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize],
            Some(ProbeFunction as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"PLAYER_KNOWS_PROGRAM\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::tell_me_about_basics.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ProbeFunction as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        if uqm_lua_checked!(getGameStateUint(
            b"PLAYER_KNOWS_PRIORITY\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            if uqm_lua_checked!(getGameStateUint(
                b"PLAYER_KNOWS_EFFECTS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::probe_has_bug.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(ProbeBug as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_0::what_effect.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::what_effect.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(ProbeFunction as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tell_basics_again.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::tell_basics_again.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ProbeFunction as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    if LastStack as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        loop {
            if pStr[LastStack as usize] != 0 {
                uqm_lua_checked!(DoResponsePhrase(
                    pStr[LastStack as usize],
                    Some(ProbeFunction as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            LastStack = LastStack.wrapping_add(1);
            if (LastStack as ::core::ffi::c_int) >= 2 as ::core::ffi::c_int {
                break;
            }
        }
    } else {
        LastStack = (LastStack as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            ^ 1 as ::core::ffi::c_int) as BYTE;
        if pStr[LastStack as usize] != 0 {
            uqm_lua_checked!(DoResponsePhrase(
                pStr[LastStack as usize],
                Some(ProbeFunction as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_problem.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ProbeInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ProbeInfo(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut LastStack: BYTE = 0;
    let mut InfoLeft: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 3] = [0; 3];
    LastStack = 0 as BYTE;
    pStr[2usize] = 0 as RESPONSE_REF;
    pStr[1usize] = pStr[2usize];
    pStr[0usize] = pStr[1usize];
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_are_probes.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::PROBES_ARE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK5\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::know_more_probe.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_WHAT_MORE_PROBE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::why_probe_always_attack.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ONLY_DEFEND.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK6\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::talk_more_probe_attack.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(ProbeFunction(R), ());
        return;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::where_probes_from.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::PROBES_FROM_MELNORME.0 as ::core::ffi::c_int,
            None,
        ), ());
        LastStack = 1 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK7\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::why_sell.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SELL_FOR_INFO.0 as ::core::ffi::c_int, None), ());
        LastStack = 1 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK7\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_long_ago.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::FIFTY_THOUSAND_ROTATIONS.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK7\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_probes_mission.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SEEK_OUT_NEW_LIFE.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::if_only_one.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::THEY_REPLICATE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_problem.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_ENOUGH_PROBLEM.0 as ::core::ffi::c_int, None), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_KNOW_BROKEN\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0
        && uqm_lua_checked!(getGameStateUint(
            b"PROBE_EXHIBITED_BUG\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_STACK6\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                pStr[0usize] = C2Rust_Unnamed_0::why_probe_always_attack.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            1 => {
                pStr[0usize] = C2Rust_Unnamed_0::talk_more_probe_attack.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            _ => {}
        }
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK7\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            pStr[1usize] = C2Rust_Unnamed_0::where_probes_from.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pStr[1usize] = C2Rust_Unnamed_0::why_sell.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        2 => {
            pStr[1usize] = C2Rust_Unnamed_0::how_long_ago.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {}
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            pStr[2usize] = C2Rust_Unnamed_0::whats_probes_mission.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pStr[2usize] = C2Rust_Unnamed_0::if_only_one.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {}
    }
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if pStr[LastStack as usize] != 0 {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            pStr[LastStack as usize],
            Some(ProbeInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        if i as ::core::ffi::c_int != LastStack as ::core::ffi::c_int
            && pStr[i as usize] as ::core::ffi::c_int != 0
        {
            InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
            uqm_lua_checked!(DoResponsePhrase(
                pStr[i as usize],
                Some(ProbeInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_probe.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(HomeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if InfoLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::know_more_probe.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn HomeWorld(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut LastStack: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 3] = [0; 3];
    LastStack = 0 as BYTE;
    pStr[2usize] = 0 as RESPONSE_REF;
    pStr[1usize] = pStr[2usize];
    pStr[0usize] = pStr[1usize];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::we_are_us.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::TERRIBLY_EXCITING.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::we_are_us.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_other_visitors.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::VISITORS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PLAYER_KNOWS_PROBE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::any_other_visitors.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::LONG_AGO.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_sentient_milieu.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::MET_TAALO_THEY_ARE_FROM.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            4 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::who_else.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::PRECURSORS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            5 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::precursors_yow.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_PRECURSORS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            6 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::must_know_more.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ALL_WE_KNOW.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            7 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::who_are_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WE_ARE_SLY.0 as ::core::ffi::c_int, None), ());
        LastStack = 1 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::where_are_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::DOWN_HERE.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::thats_impossible_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_ITS_NOT_1.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::thats_impossible_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_ITS_NOT_2.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::like_more_about_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(SlylandroInfo(R), ());
        return;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_about_me.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_ENOUGH_YOU.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_ENOUGH_INFO.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_probe.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_ENOUGH_PROBE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::mega_self_destruct.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WHY_YES_THERE_IS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_KNOW_BROKEN\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        i = uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .wrapping_add(1 as DWORD) as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
            i as DWORD,
        ), ());
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_1::SLYLANDRO_RAMP_DOWN.0 as ::core::ffi::c_int as BYTE,
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            pStr[0usize] = C2Rust_Unnamed_0::we_are_us.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pStr[0usize] = C2Rust_Unnamed_0::what_other_visitors.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        2 => {
            pStr[0usize] = C2Rust_Unnamed_0::any_other_visitors.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        3 => {
            pStr[0usize] = C2Rust_Unnamed_0::what_about_sentient_milieu.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        4 => {
            pStr[0usize] = C2Rust_Unnamed_0::who_else.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        5 => {
            pStr[0usize] = C2Rust_Unnamed_0::precursors_yow.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        6 => {
            pStr[0usize] = C2Rust_Unnamed_0::must_know_more.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {}
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            pStr[1usize] = C2Rust_Unnamed_0::who_are_you.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_0::like_more_about_you.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                pStr[1usize] = C2Rust_Unnamed_0::like_more_about_you.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
        }
        _ => {}
    }
    match uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            pStr[2usize] = C2Rust_Unnamed_0::where_are_you.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pStr[2usize] = C2Rust_Unnamed_0::thats_impossible_1.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        2 => {
            pStr[2usize] = C2Rust_Unnamed_0::thats_impossible_2.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {}
    }
    if pStr[LastStack as usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pStr[LastStack as usize],
            Some(HomeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        if i as ::core::ffi::c_int != LastStack as ::core::ffi::c_int
            && pStr[i as usize] as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                pStr[i as usize],
                Some(HomeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        i = i.wrapping_add(1);
    }
    if uqm_lua_checked!(getGameStateUint(b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
    {
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::happy_to_tell_more.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            1 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::would_you_like_to_know_more.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(HumanInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            _ => {}
        }
    }
    if uqm_lua_checked!(getGameStateUint(b"PLAYER_KNOWS_PROBE\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
        && uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_KNOW_BROKEN\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
    {
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_STACK5\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::what_are_probes.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(ProbeInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            1 => {
                if *GetStringAddress(
                    SetAbsStringTableIndex(
                        CommData.ConversationPhrases,
                        (C2Rust_Unnamed_0::know_more_probe.0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::know_more_probe.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(ProbeInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
            }
            _ => {}
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        EngageFilters(&raw mut slyhome_filters);
    }
    CommData.AlienAmbientArray[0usize].AnimFlags = (CommData
        .AlienAmbientArray[0usize]
        .AnimFlags as ::core::ffi::c_long
        & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
    if uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_KNOW_BROKEN\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
        && {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"RECALL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            NumVisits as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        }
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RECALL_PROGRAM_1.0 as ::core::ffi::c_int, None), ());
        NumVisits = NumVisits.wrapping_add(1);
        uqm_lua_checked!(setGameStateUint(
            b"RECALL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    } else {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_1.0 as ::core::ffi::c_int, None), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_2.0 as ::core::ffi::c_int, None), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_3.0 as ::core::ffi::c_int, None), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_4.0 as ::core::ffi::c_int, None), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(SetHomeworldKnown(
            C2Rust_Unnamed_2::SLYLANDRO_HOME.0 as ::core::ffi::c_int as DWORD,
        ), ());
    }
    uqm_lua_checked!(HomeWorld(0 as ::core::ffi::c_int as RESPONSE_REF), ());
}
unsafe extern "C" fn uninit_slylandro() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_slylandro_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_slylandro_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    slylandro_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    slylandro_desc.post_encounter_func = Some(
        post_slylandro_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    slylandro_desc.uninit_encounter_func = Some(
        uninit_slylandro as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    slylandro_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
        + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor))
            - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)) as COORD;
    slylandro_desc.AlienTextBaseline.y = 0 as COORD;
    slylandro_desc.AlienTextWidth = (CanvasWidth
        - ((64 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) * 2 as ::core::ffi::c_int
        - (((13 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor))
        - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int))
        as COUNT;
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    retval = &raw mut slylandro_desc;
    return retval;
}
