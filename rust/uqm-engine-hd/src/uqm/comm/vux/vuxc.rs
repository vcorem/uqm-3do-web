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

// Rust translation of src/uqm/comm/vux/vuxc.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn XFormColorMap(ColorMapPtr: COLORMAPPTR, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    fn SwitchSequences(enableAll: BOOLEAN);
    fn RunOneTimeSequence(animIndex: COUNT, flags: COUNT);
    fn EngageFilters(f_desc: *mut FILTER_DESC);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut CommData: LOCDATA;
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn setSegue(segue: Segue);
    fn AlienTalkSegue(wait_track: COUNT);
    fn EnableTalkingAnim(enable: BOOLEAN);
    static mut altResFlags: BYTE;
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
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
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
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type LINE = line;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
pub type TEXT = text;
pub type COLORMAP = STRING;
pub type COLORMAPPTR = *mut ::core::ffi::c_void;
pub type GRAPHICS_PRIM = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type PRIM_LINKS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
    pub flags: BYTE,
}
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
pub struct SIS_STATE {
    pub log_x: SDWORD,
    pub log_y: SDWORD,
    pub ResUnits: DWORD,
    pub FuelOnBoard: DWORD,
    pub CrewEnlisted: COUNT,
    pub TotalElementMass: COUNT,
    pub TotalBioMass: COUNT,
    pub ModuleSlots: [BYTE; 16],
    pub DriveSlots: [BYTE; 11],
    pub JetSlots: [BYTE; 8],
    pub NumLanders: BYTE,
    pub ElementAmounts: [COUNT; 8],
    pub ShipName: [UNICODE; 32],
    pub CommanderName: [UNICODE; 32],
    pub PlanetName: [UNICODE; 32],
    pub Difficulty: BYTE,
    pub Extended: BYTE,
    pub Nomad: BYTE,
    pub ShipSeed: BYTE,
    pub Seed: SDWORD,
    pub SaveVersion: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SUPER_MELEE: Self = Self(0);
    pub const IN_LAST_BATTLE: Self = Self(1);
    pub const IN_ENCOUNTER: Self = Self(2);
    pub const IN_HYPERSPACE: Self = Self(3);
    pub const IN_INTERPLANETARY: Self = Self(4);
    pub const WON_LAST_BATTLE: Self = Self(5);
    pub const IN_QUASISPACE: Self = Self(6);
    pub const IN_PLANET_ORBIT: Self = Self(7);
    pub const IN_STARBASE: Self = Self(8);
    pub const CHECK_PAUSE: Self = Self(256);
    pub const IN_BATTLE: Self = Self(512);
    pub const START_ENCOUNTER: Self = Self(1024);
    pub const START_INTERPLANETARY: Self = Self(2048);
    pub const CHECK_LOAD: Self = Self(4096);
    pub const CHECK_RESTART: Self = Self(8192);
    pub const CHECK_ABORT: Self = Self(16384);
}
pub type ACTIVITY = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAME_STATE {
    pub glob_flags: BYTE,
    pub CrewCost: BYTE,
    pub FuelCost: BYTE,
    pub ModuleCost: [BYTE; 20],
    pub ElementWorth: [BYTE; 8],
    pub DisplayArray: *mut PRIMITIVE,
    pub CurrentActivity: ACTIVITY,
    pub GameClock: CLOCK_STATE,
    pub autopilot: POINT,
    pub ip_location: POINT,
    pub ShipStamp: STAMP,
    pub ShipFacing: UWORD,
    pub ip_planet: BYTE,
    pub in_orbit: BYTE,
    pub velocity: VELOCITY_DESC,
    pub BattleGroupRef: DWORD,
    pub avail_race_q: QUEUE,
    pub npc_built_ship_q: QUEUE,
    pub ip_group_q: QUEUE,
    pub encounter_q: QUEUE,
    pub built_ship_q: QUEUE,
    pub stowed_ship_q: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const ZEX_HELLO_1: Self = Self(1);
    pub const ZEX_HELLO_2: Self = Self(2);
    pub const ZEX_HELLO_3: Self = Self(3);
    pub const ZEX_HELLO_4: Self = Self(4);
    pub const FIGHT_OR_TRADE_1: Self = Self(5);
    pub const FIGHT_OR_TRADE_2: Self = Self(6);
    pub const what_you_do_here: Self = Self(7);
    pub const MY_MENAGERIE: Self = Self(8);
    pub const what_about_menagerie: Self = Self(9);
    pub const NEED_NEW_CREATURE: Self = Self(10);
    pub const what_about_creature: Self = Self(11);
    pub const ABOUT_CREATURE: Self = Self(12);
    pub const about_creature_again: Self = Self(13);
    pub const CREATURE_AGAIN: Self = Self(14);
    pub const i_have_beast: Self = Self(15);
    pub const GIVE_BEAST: Self = Self(16);
    pub const ok_take_beast: Self = Self(17);
    pub const FOOL_AIEE0: Self = Self(18);
    pub const FOOL_AIEE1: Self = Self(19);
    pub const why_trust_1: Self = Self(20);
    pub const TRUST_1: Self = Self(21);
    pub const why_trust_2: Self = Self(22);
    pub const TRUST_2: Self = Self(23);
    pub const why_trust_3: Self = Self(24);
    pub const TRUST_3: Self = Self(25);
    pub const why_dont_you_attack: Self = Self(26);
    pub const LIKE_YOU: Self = Self(27);
    pub const why_like_me: Self = Self(28);
    pub const LIKE_BECAUSE: Self = Self(29);
    pub const are_you_a_pervert: Self = Self(30);
    pub const CALL_ME_WHAT_YOU_WISH: Self = Self(31);
    pub const take_by_force: Self = Self(32);
    pub const PRECURSOR_DEVICE: Self = Self(33);
    pub const regardless: Self = Self(34);
    pub const THEN_FIGHT: Self = Self(35);
    pub const you_lied: Self = Self(36);
    pub const YUP_LIED: Self = Self(37);
    pub const kill_you: Self = Self(38);
    pub const FIGHT_AGAIN: Self = Self(39);
    pub const bye_zex: Self = Self(40);
    pub const GOODBYE_ZEX: Self = Self(41);
    pub const HOMEWORLD_HELLO_1: Self = Self(42);
    pub const HOMEWORLD_HELLO_2: Self = Self(43);
    pub const HOMEWORLD_HELLO_3: Self = Self(44);
    pub const HOMEWORLD_HELLO_4: Self = Self(45);
    pub const SPACE_HELLO_1: Self = Self(46);
    pub const SPACE_HELLO_2: Self = Self(47);
    pub const SPACE_HELLO_3: Self = Self(48);
    pub const SPACE_HELLO_4: Self = Self(49);
    pub const kill_you_squids_1: Self = Self(50);
    pub const kill_you_squids_2: Self = Self(51);
    pub const kill_you_squids_3: Self = Self(52);
    pub const kill_you_squids_4: Self = Self(53);
    pub const WE_FIGHT: Self = Self(54);
    pub const why_so_mean: Self = Self(55);
    pub const URQUAN_SLAVES: Self = Self(56);
    pub const deeper_reason: Self = Self(57);
    pub const OLD_INSULT: Self = Self(58);
    pub const if_we_apologize: Self = Self(59);
    pub const PROBABLY_NOT: Self = Self(60);
    pub const try_any_way: Self = Self(61);
    pub const NOPE: Self = Self(62);
    pub const APOLOGIZE_IN_SPACE: Self = Self(63);
    pub const apology_1: Self = Self(64);
    pub const NOT_ACCEPTED_1: Self = Self(65);
    pub const apology_2: Self = Self(66);
    pub const NOT_ACCEPTED_2: Self = Self(67);
    pub const apology_3: Self = Self(68);
    pub const NOT_ACCEPTED_3: Self = Self(69);
    pub const apology_4: Self = Self(70);
    pub const NOT_ACCEPTED_4: Self = Self(71);
    pub const apology_5: Self = Self(72);
    pub const NOT_ACCEPTED_5: Self = Self(73);
    pub const apology_6: Self = Self(74);
    pub const NOT_ACCEPTED_6: Self = Self(75);
    pub const apology_7: Self = Self(76);
    pub const NOT_ACCEPTED_7: Self = Self(77);
    pub const apology_8: Self = Self(78);
    pub const NOT_ACCEPTED_8: Self = Self(79);
    pub const apology_9: Self = Self(80);
    pub const NOT_ACCEPTED_9: Self = Self(81);
    pub const apology_10: Self = Self(82);
    pub const TRUTH: Self = Self(83);
    pub const whats_up_hostile: Self = Self(84);
    pub const GENERAL_INFO_HOSTILE_1: Self = Self(85);
    pub const GENERAL_INFO_HOSTILE_2: Self = Self(86);
    pub const GENERAL_INFO_HOSTILE_3: Self = Self(87);
    pub const GENERAL_INFO_HOSTILE_4: Self = Self(88);
    pub const cant_we_be_friends_1: Self = Self(89);
    pub const NEVER_UGLY_HUMANS_1: Self = Self(90);
    pub const cant_we_be_friends_2: Self = Self(91);
    pub const NEVER_UGLY_HUMANS_2: Self = Self(92);
    pub const cant_we_be_friends_3: Self = Self(93);
    pub const NEVER_UGLY_HUMANS_3: Self = Self(94);
    pub const cant_we_be_friends_4: Self = Self(95);
    pub const NEVER_UGLY_HUMANS_4: Self = Self(96);
    pub const bye_hostile_space: Self = Self(97);
    pub const GOODBYE_AND_DIE_HOSTILE_SPACE_1: Self = Self(98);
    pub const GOODBYE_AND_DIE_HOSTILE_SPACE_2: Self = Self(99);
    pub const GOODBYE_AND_DIE_HOSTILE_SPACE_3: Self = Self(100);
    pub const GOODBYE_AND_DIE_HOSTILE_SPACE_4: Self = Self(101);
    pub const OUT_TAKES: Self = Self(102);
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
static mut vux_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::VUX_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.vux.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.vux.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.vux.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.vux.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: b"comm.vux.zex.graphics\0".as_ptr() as RESOURCE,
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: b"comm.vux.zex.music\0".as_ptr() as RESOURCE,
    },
    ConversationPhrasesRes: b"comm.vux.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 19 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 12 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 15 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 20 as COUNT,
            NumFrames: 14 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 34 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 41 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 47 as COUNT,
            NumFrames: 11 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 58 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 61 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 65 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 69 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 71 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 74 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 80 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 85 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 90 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 95 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 99 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 103 as COUNT,
            NumFrames: 16 as BYTE,
            AnimFlags: (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 119 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
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
        StartIndex: 1 as COUNT,
        NumFrames: 11 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int) as COUNT,
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
static mut vux_filters: FILTER_DESC = FILTER_DESC {
    NumFilters: 1 as BYTE,
    FilterArray: [
        FILTER {
            ColorIndex: 0 as BYTE,
            OpacityIndex: 1 as BYTE,
            FrameIndex: -1 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_ALPHA.0 as ::core::ffi::c_int as BYTE,
            Flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE,
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
unsafe extern "C" fn ZexBeingEatenAlive() {
    if altResFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            CommData.AlienFrame = SetAbsFrameIndex(CommData.AlienFrame, 126 as COUNT);
            EnableTalkingAnim(BOOLEAN::FALSE);
            SwitchSequences(BOOLEAN::FALSE);
            RunOneTimeSequence(18 as COUNT, 0 as COUNT);
        } else {
            CommData.AlienFrame = SetAbsFrameIndex(CommData.AlienFrame, 119 as COUNT);
            XFormColorMap(
                GetColorMapAddress(
                    SetAbsStringTableIndex(CommData.AlienColorMap, 1 as COUNT),
                ),
                (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
            );
        }
    } else {
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 1 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
        );
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            EngageFilters(&raw mut vux_filters);
        }
    };
}
unsafe extern "C" fn CombatIsInevitable(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::ok_take_beast.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FOOL_AIEE0.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FOOL_AIEE1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(AlienTalkSegue(1 as COUNT), ());
        ZexBeingEatenAlive();
        uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(setGameStateUint(
            b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZEX_IS_DEAD\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::try_any_way.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::NOPE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            4 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::kill_you_squids_1.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::kill_you_squids_2.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::kill_you_squids_3.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::kill_you_squids_4.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WE_FIGHT.0 as ::core::ffi::c_int, None), ());
        NumVisits = uqm_lua_checked!(getGameStateUint(
                b"VUX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .wrapping_add(1 as DWORD) as BYTE;
        if NumVisits as ::core::ffi::c_int <= 3 as ::core::ffi::c_int {
            uqm_lua_checked!(setGameStateUint(
                b"VUX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::cant_we_be_friends_1.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::cant_we_be_friends_2.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::cant_we_be_friends_3.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::cant_we_be_friends_4.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"VUX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NEVER_UGLY_HUMANS_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NEVER_UGLY_HUMANS_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NEVER_UGLY_HUMANS_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NEVER_UGLY_HUMANS_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bye_hostile_space.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"VUX_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh4 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh4 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GOODBYE_AND_DIE_HOSTILE_SPACE_1.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GOODBYE_AND_DIE_HOSTILE_SPACE_2.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GOODBYE_AND_DIE_HOSTILE_SPACE_3.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GOODBYE_AND_DIE_HOSTILE_SPACE_4.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bye_zex.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOODBYE_ZEX.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    } else {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh5 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh5 as ::core::ffi::c_int {
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            8 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            9 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            10 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            11 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            12 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::NOT_ACCEPTED_9.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            13 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TRUTH.0 as ::core::ffi::c_int, None), ());
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    };
}
unsafe extern "C" fn Menagerie(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut LastStack: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 3] = [0; 3];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::i_have_beast.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_trust_1.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_trust_2.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_trust_3.0 as ::core::ffi::c_int
    {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::i_have_beast.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_BEAST.0 as ::core::ffi::c_int, None), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_trust_1.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TRUST_1.0 as ::core::ffi::c_int, None), ());
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::why_trust_1.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_trust_2.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TRUST_2.0 as ::core::ffi::c_int, None), ());
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::why_trust_2.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_trust_3.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TRUST_3.0 as ::core::ffi::c_int, None), ());
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::why_trust_3.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::why_trust_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::why_trust_1.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(Menagerie as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::why_trust_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::why_trust_2.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(Menagerie as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::why_trust_3.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::why_trust_3.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(Menagerie as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::ok_take_beast.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::kill_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FIGHT_AGAIN.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::regardless.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THEN_FIGHT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZEX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else {
        LastStack = 0 as BYTE;
        pStr[2usize] = 0 as RESPONSE_REF;
        pStr[1usize] = pStr[2usize];
        pStr[0usize] = pStr[1usize];
        if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut NumVisits: BYTE = 0;
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"ZEX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            if uqm_lua_checked!(getGameStateUint(b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char), ())
                >= 2 as DWORD
            {
                let c2rust_fresh6 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh6 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_1::FIGHT_OR_TRADE_1.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_1::FIGHT_OR_TRADE_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
            } else {
                let c2rust_fresh7 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh7 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_1::ZEX_HELLO_1.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_1::ZEX_HELLO_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    2 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_1::ZEX_HELLO_3.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    3 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_1::ZEX_HELLO_4.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
            }
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::what_you_do_here.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::MY_MENAGERIE.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::what_about_menagerie.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NEED_NEW_CREATURE.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
                2 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::what_about_creature.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_CREATURE.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"KNOW_ZEX_WANTS_MONSTER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
            R = C2Rust_Unnamed_1::about_creature_again.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::what_about_creature.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::about_creature_again.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::CREATURE_AGAIN.0 as ::core::ffi::c_int, None), ());
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::about_creature_again.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_dont_you_attack.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::LIKE_YOU.0 as ::core::ffi::c_int, None), ());
            LastStack = 1 as BYTE;
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::why_like_me.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::LIKE_BECAUSE.0 as ::core::ffi::c_int, None), ());
            LastStack = 1 as BYTE;
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
                2 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::are_you_a_pervert.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::CALL_ME_WHAT_YOU_WISH.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::take_by_force.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::PRECURSOR_DEVICE.0 as ::core::ffi::c_int,
                None,
            ), ());
            LastStack = 2 as BYTE;
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::you_lied.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::YUP_LIED.0 as ::core::ffi::c_int, None), ());
            LastStack = 2 as BYTE;
            uqm_lua_checked!(setGameStateUint(
                b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"KNOW_ZEX_WANTS_MONSTER\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
            && uqm_lua_checked!(getGameStateUint(
                b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
        {
            pStr[0usize] = C2Rust_Unnamed_1::i_have_beast.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        } else {
            match uqm_lua_checked!(getGameStateUint(
                b"ZEX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                0 => {
                    pStr[0usize] = C2Rust_Unnamed_1::what_you_do_here.0
                        as ::core::ffi::c_int as RESPONSE_REF;
                }
                1 => {
                    pStr[0usize] = C2Rust_Unnamed_1::what_about_menagerie.0
                        as ::core::ffi::c_int as RESPONSE_REF;
                }
                2 => {
                    pStr[0usize] = C2Rust_Unnamed_1::what_about_creature.0
                        as ::core::ffi::c_int as RESPONSE_REF;
                }
                3 => {
                    if *GetStringAddress(
                        SetAbsStringTableIndex(
                            CommData.ConversationPhrases,
                            (C2Rust_Unnamed_1::about_creature_again.0
                                as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                    {
                        pStr[0usize] = C2Rust_Unnamed_1::about_creature_again.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                _ => {}
            }
        }
        match uqm_lua_checked!(getGameStateUint(b"ZEX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char), ()) {
            0 => {
                pStr[1usize] = C2Rust_Unnamed_1::why_dont_you_attack.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            1 => {
                pStr[1usize] = C2Rust_Unnamed_1::why_like_me.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr[1usize] = C2Rust_Unnamed_1::are_you_a_pervert.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            _ => {}
        }
        match uqm_lua_checked!(getGameStateUint(b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char), ()) {
            0 => {
                pStr[2usize] = C2Rust_Unnamed_1::take_by_force.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr[2usize] = C2Rust_Unnamed_1::regardless.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr[2usize] = C2Rust_Unnamed_1::you_lied.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr[2usize] = C2Rust_Unnamed_1::kill_you.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        if pStr[LastStack as usize] != 0 {
            uqm_lua_checked!(DoResponsePhrase(
                pStr[LastStack as usize],
                Some(Menagerie as unsafe extern "C" fn(RESPONSE_REF) -> ()),
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
                    Some(Menagerie as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            i = i.wrapping_add(1);
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::bye_zex.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn NormalVux(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::why_so_mean.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::URQUAN_SLAVES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::deeper_reason.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OLD_INSULT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::if_we_apologize.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::PROBABLY_NOT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_hostile.0 as ::core::ffi::c_int
    {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"VUX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh2 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_HOSTILE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_HOSTILE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_HOSTILE_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_HOSTILE_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"VUX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_hostile.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    match uqm_lua_checked!(getGameStateUint(b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::why_so_mean.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NormalVux as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::deeper_reason.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NormalVux as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        2 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::if_we_apologize.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(NormalVux as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        3 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::try_any_way.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        4 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_1.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        5 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_2.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        6 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_3.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        7 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_4.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        8 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_5.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        9 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_6.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        10 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_7.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        11 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_8.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        12 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_9.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        13 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::apology_10.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {}
    }
    match uqm_lua_checked!(getGameStateUint(b"VUX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::kill_you_squids_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::kill_you_squids_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        2 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::kill_you_squids_3.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        3 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::kill_you_squids_4.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {}
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_hostile.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_hostile.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NormalVux as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char), ())
        > 13 as DWORD
    {
        match uqm_lua_checked!(getGameStateUint(b"VUX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char), ()) {
            0 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::cant_we_be_friends_1.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            1 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::cant_we_be_friends_2.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            2 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::cant_we_be_friends_3.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            3 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::cant_we_be_friends_4.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            _ => {}
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye_hostile_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    if altResFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        CommData.AlienAmbientArray[17usize].AnimFlags = (CommData
            .AlienAmbientArray[17usize]
            .AnimFlags as ::core::ffi::c_long
            & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OUT_TAKES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        return;
    }
    if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        uqm_lua_checked!(Menagerie(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else {
        let mut NumVisits: BYTE = 0;
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"VUX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh0 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh0 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HOMEWORLD_HELLO_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HOMEWORLD_HELLO_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HOMEWORLD_HELLO_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HOMEWORLD_HELLO_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"VUX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(SetHomeworldKnown(
                C2Rust_Unnamed_2::VUX_HOME.0 as ::core::ffi::c_int as DWORD,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"VUX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh1 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh1 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SPACE_HELLO_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SPACE_HELLO_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SPACE_HELLO_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SPACE_HELLO_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"VUX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        uqm_lua_checked!(NormalVux(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_vux() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_vux_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_vux_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    vux_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    vux_desc.post_encounter_func = Some(post_vux_enc as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    vux_desc.uninit_encounter_func = Some(uninit_vux as unsafe extern "C" fn() -> COUNT)
        as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    vux_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
            >> 1 as ::core::ffi::c_int)
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
            >> 2 as ::core::ffi::c_int)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    vux_desc.AlienTextBaseline.y = 0 as COORD;
    vux_desc.AlienTextWidth = ((CanvasWidth
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
        - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int)
        - ((16 as ::core::ffi::c_int) << resolutionFactor) >> 1 as ::core::ffi::c_int)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COUNT;
    if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ::core::ptr::null_mut()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        altResFlags = (altResFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
        if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            altResFlags = (altResFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        }
    }
    if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ::core::ptr::null_mut()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut vux_desc;
    return retval;
}
