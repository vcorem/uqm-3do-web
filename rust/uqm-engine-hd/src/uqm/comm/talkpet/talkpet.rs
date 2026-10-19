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

// Rust translation of src/uqm/comm/talkpet/talkpet.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
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
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
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
pub struct RACE_ID(pub ::core::ffi::c_uint);
impl RACE_ID {
    pub const ARILOU_SHIP: Self = Self(0);
    pub const CHMMR_SHIP: Self = Self(1);
    pub const HUMAN_SHIP: Self = Self(2);
    pub const ORZ_SHIP: Self = Self(3);
    pub const PKUNK_SHIP: Self = Self(4);
    pub const SHOFIXTI_SHIP: Self = Self(5);
    pub const SPATHI_SHIP: Self = Self(6);
    pub const SUPOX_SHIP: Self = Self(7);
    pub const THRADDASH_SHIP: Self = Self(8);
    pub const UTWIG_SHIP: Self = Self(9);
    pub const VUX_SHIP: Self = Self(10);
    pub const YEHAT_SHIP: Self = Self(11);
    pub const MELNORME_SHIP: Self = Self(12);
    pub const DRUUGE_SHIP: Self = Self(13);
    pub const ILWRATH_SHIP: Self = Self(14);
    pub const MYCON_SHIP: Self = Self(15);
    pub const SLYLANDRO_SHIP: Self = Self(16);
    pub const UMGAH_SHIP: Self = Self(17);
    pub const URQUAN_SHIP: Self = Self(18);
    pub const ZOQFOTPIK_SHIP: Self = Self(19);
    pub const SYREEN_SHIP: Self = Self(20);
    pub const BLACK_URQUAN_SHIP: Self = Self(21);
    pub const ANDROSYNTH_SHIP: Self = Self(22);
    pub const CHENJESU_SHIP: Self = Self(23);
    pub const MMRNMHRM_SHIP: Self = Self(24);
    pub const NUM_BUILDABLE_SHIPS: Self = Self(25);
    pub const YEHAT_REBEL_SHIP: Self = Self(25);
    pub const URQUAN_DRONE_SHIP: Self = Self(26);
    pub const SAMATRA_SHIP: Self = Self(26);
    pub const NUM_AVAILABLE_RACES: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const HELLO_AT_UMGAH: Self = Self(1);
    pub const what_are_you: Self = Self(2);
    pub const JUST_TALKING_PET: Self = Self(3);
    pub const talking_pets_dumb: Self = Self(4);
    pub const OH_NO_YOU_DONT: Self = Self(5);
    pub const what_do_to_umgah: Self = Self(6);
    pub const DID_NOTHING: Self = Self(7);
    pub const umgah_zombies: Self = Self(8);
    pub const WORKS_LIKE_THIS: Self = Self(9);
    pub const we_are_vindicator: Self = Self(10);
    pub const GOOD_FOR_YOU: Self = Self(11);
    pub const must_explain_presence: Self = Self(12);
    pub const EXPLAIN_NOTHING_MONKEY_BOY: Self = Self(13);
    pub const bye_at_umgah: Self = Self(14);
    pub const GOODBYE_AT_UMGAH: Self = Self(15);
    pub const HYPNOTIZE_AGAIN_1: Self = Self(16);
    pub const HYPNOTIZE_AGAIN_2: Self = Self(17);
    pub const HYPNOTIZE_AGAIN_3: Self = Self(18);
    pub const HYPNOTIZE_AGAIN_4: Self = Self(19);
    pub const HYPNO_TAIL: Self = Self(20);
    pub const CANT_COMPEL: Self = Self(21);
    pub const LETS_MAKE_A_DEAL: Self = Self(22);
    pub const what_kind_of_deal: Self = Self(23);
    pub const HELP_DEFEAT_URQUAN: Self = Self(24);
    pub const ok_lets_do_it: Self = Self(25);
    pub const COMING_ABOARD: Self = Self(26);
    pub const how_trust: Self = Self(27);
    pub const TRUST: Self = Self(28);
    pub const boneless_dweeb: Self = Self(29);
    pub const YOUR_BONELESS_DWEEB: Self = Self(30);
    pub const what_are_you_really: Self = Self(31);
    pub const POOR_DNYARRI: Self = Self(32);
    pub const hard_to_believe: Self = Self(33);
    pub const ITS_TRUE: Self = Self(34);
    pub const bullshit: Self = Self(35);
    pub const WORTH_A_TRY: Self = Self(36);
    pub const kill_you: Self = Self(37);
    pub const PLEASE_DONT: Self = Self(38);
    pub const must_kill: Self = Self(39);
    pub const DONT_KILL: Self = Self(40);
    pub const want_kill_1: Self = Self(41);
    pub const want_kill_2: Self = Self(42);
    pub const want_kill_3: Self = Self(43);
    pub const GLAD_YOU_WONT_KILL: Self = Self(44);
    pub const whats_up_onboard: Self = Self(45);
    pub const GENERAL_INFO_ONBOARD_1: Self = Self(46);
    pub const GENERAL_INFO_ONBOARD_2: Self = Self(47);
    pub const GENERAL_INFO_ONBOARD_3: Self = Self(48);
    pub const GENERAL_INFO_ONBOARD_4: Self = Self(49);
    pub const GENERAL_INFO_ONBOARD_5: Self = Self(50);
    pub const GENERAL_INFO_ONBOARD_6: Self = Self(51);
    pub const GENERAL_INFO_ONBOARD_7: Self = Self(52);
    pub const GENERAL_INFO_ONBOARD_8: Self = Self(53);
    pub const HELLO_AS_DEVICE_1: Self = Self(54);
    pub const HELLO_AS_DEVICE_2: Self = Self(55);
    pub const HELLO_AS_DEVICE_3: Self = Self(56);
    pub const HELLO_AS_DEVICE_4: Self = Self(57);
    pub const HELLO_AS_DEVICE_5: Self = Self(58);
    pub const HELLO_AS_DEVICE_6: Self = Self(59);
    pub const HELLO_AS_DEVICE_7: Self = Self(60);
    pub const HELLO_AS_DEVICE_8: Self = Self(61);
    pub const CYBORG_PEP_TALK: Self = Self(62);
    pub const HUMAN_PEP_TALK: Self = Self(63);
    pub const I_SENSE_MY_SLAVES: Self = Self(64);
    pub const HAVENT_GOT_EVERYTHING: Self = Self(65);
    pub const NEED_BOMB: Self = Self(66);
    pub const SOUP_UP_BOMB: Self = Self(67);
    pub const SOUP_UP_FLEET: Self = Self(68);
    pub const SOUP_UP_FLAGSHIP: Self = Self(69);
    pub const COMEBACK_WHEN_READY: Self = Self(70);
    pub const what_now: Self = Self(71);
    pub const DO_THIS: Self = Self(72);
    pub const compel_urquan: Self = Self(73);
    pub const HERE_WE_GO: Self = Self(74);
    pub const im_scared: Self = Self(75);
    pub const STUPID_FOP: Self = Self(76);
    pub const compel_that_ship: Self = Self(77);
    pub const SAVING_MY_POWER: Self = Self(78);
    pub const any_suggestions: Self = Self(79);
    pub const SUGGESTION_1: Self = Self(80);
    pub const SUGGESTION_2: Self = Self(81);
    pub const SUGGESTION_3: Self = Self(82);
    pub const SUGGESTION_4: Self = Self(83);
    pub const SUGGESTION_5: Self = Self(84);
    pub const SUGGESTION_6: Self = Self(85);
    pub const SUGGESTION_7: Self = Self(86);
    pub const SUGGESTION_8: Self = Self(87);
    pub const about_your_race: Self = Self(88);
    pub const WHAT_ABOUT_RACE: Self = Self(89);
    pub const you_lied: Self = Self(90);
    pub const SO_WHAT: Self = Self(91);
    pub const bye_onboard: Self = Self(92);
    pub const GOODBYE_ONBOARD: Self = Self(93);
    pub const what_about_physiology: Self = Self(94);
    pub const NO_TALK_ABOUT_SELF: Self = Self(95);
    pub const what_about_powers: Self = Self(96);
    pub const NOT_POWERS_BUT_FLOWERS: Self = Self(97);
    pub const yes_flowers: Self = Self(98);
    pub const GOOD_HUMAN: Self = Self(99);
    pub const wish_to_go_now: Self = Self(100);
    pub const EXCELLENT_IDEA: Self = Self(101);
    pub const what_about_your_history: Self = Self(102);
    pub const ABOUT_HISTORY: Self = Self(103);
    pub const sentient_milieu: Self = Self(104);
    pub const ABOUT_SENTIENT_MILIEU: Self = Self(105);
    pub const what_about_war: Self = Self(106);
    pub const ABOUT_WAR: Self = Self(107);
    pub const enough_info: Self = Self(108);
    pub const OK_ENOUGH_INFO: Self = Self(109);
    pub const UMGAH_ALL_GONE: Self = Self(110);
    pub const HELLO_AFTER_COMPEL_URQUAN: Self = Self(111);
    pub const OUT_TAKES: Self = Self(112);
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
static mut talkpet_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::TALKING_PET_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.talkingpet.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.talkingpet.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.talkingpet.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.talkingpet.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.talkingpet.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 18 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 7 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
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
            StartIndex: 10 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
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
            StartIndex: 13 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
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
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
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
            StartIndex: 18 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 21 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 24 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 26 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 28 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 32 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 35 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 40 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 42 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 48 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 50 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 52 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: (840 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                * 30 as ::core::ffi::c_int / 840 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as BYTE,
            AnimFlags: (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int
                / (30 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as COUNT,
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
        NumFrames: 6 as BYTE,
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
static mut talkpet_filters: FILTER_DESC = FILTER_DESC {
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
    uqm_lua_checked!(setGameStateUint(
        b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::compel_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HERE_WE_GO.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"URQUAN_MESSED_UP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::wish_to_go_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::EXCELLENT_IDEA.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bye_onboard.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOODBYE_ONBOARD.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::compel_that_ship.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SAVING_MY_POWER.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::ok_lets_do_it.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::want_kill_1.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::want_kill_2.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::want_kill_3.0 as ::core::ffi::c_int
    {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::ok_lets_do_it.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::COMING_ABOARD.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::GLAD_YOU_WONT_KILL.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        uqm_lua_checked!(setGameStateUint(
            b"TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"UMGAH_ZOMBIE_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"UMGAH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"UMGAH_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ARILOU_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    }
}
unsafe extern "C" fn MindFuckUrquan(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::what_now.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DO_THIS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_now.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::im_scared.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::STUPID_FOP.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::im_scared.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_now.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_now.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(MindFuckUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::im_scared.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::im_scared.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(MindFuckUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::compel_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn MindControlStrobe() {
    CommData.AlienAmbientArray[16usize].AnimFlags = (CommData
        .AlienAmbientArray[16usize]
        .AnimFlags as ::core::ffi::c_long
        & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        EngageFilters(&raw mut talkpet_filters);
    }
}
unsafe extern "C" fn MindControl(mut R: RESPONSE_REF) {
    let mut RespFunc: RESPONSE_FUNC = None;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_powers.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_POWERS_BUT_FLOWERS.0 as ::core::ffi::c_int,
            None,
        ), ());
        RespFunc = Some(MindControl as unsafe extern "C" fn(RESPONSE_REF) -> ());
        R = C2Rust_Unnamed_1::yes_flowers.0 as ::core::ffi::c_int as RESPONSE_REF;
    } else {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOOD_HUMAN.0 as ::core::ffi::c_int, None), ());
        RespFunc = Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ());
        R = C2Rust_Unnamed_1::wish_to_go_now.0 as ::core::ffi::c_int as RESPONSE_REF;
    }
    uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    MindControlStrobe();
    uqm_lua_checked!(DoResponsePhrase(R, RespFunc, ::core::ptr::null_mut::<UNICODE>()), ());
}
unsafe extern "C" fn PetInfo(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::about_your_race.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WHAT_ABOUT_RACE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_physiology.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::NO_TALK_ABOUT_SELF.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_physiology.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_your_history.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_HISTORY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_your_history.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::sentient_milieu.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_SENTIENT_MILIEU.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::sentient_milieu.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_war.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_WAR.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_war.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_physiology.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_physiology.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(PetInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_powers.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(MindControl as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_your_history.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_your_history.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(PetInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::sentient_milieu.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::sentient_milieu.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_war.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::enough_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(PetDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn PetDevice(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_onboard.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh2 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_ONBOARD_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"TALKING_PET_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_onboard.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::any_suggestions.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_SUGGESTIONS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUGGESTION_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"TALKING_PET_SUGGESTIONS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::any_suggestions.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::enough_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_ENOUGH_INFO.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::you_lied.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SO_WHAT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"DNYARRI_LIED\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::compel_that_ship.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_onboard.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_onboard.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::any_suggestions.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::any_suggestions.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::about_your_race.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(PetInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"DNYARRI_LIED\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
        && uqm_lua_checked!(getGameStateUint(
            b"LEARNED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::you_lied.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye_onboard.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn CompelPlayer(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut LastStack: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 3] = [0; 3];
    LastStack = 0 as BYTE;
    pStr[2usize] = 0 as RESPONSE_REF;
    pStr[1usize] = pStr[2usize];
    pStr[0usize] = pStr[1usize];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::what_are_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::JUST_TALKING_PET.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_are_you.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_do_to_umgah.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DID_NOTHING.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_do_to_umgah.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
        LastStack = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOOD_FOR_YOU.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
        LastStack = 2 as BYTE;
    } else if R as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::bye_at_umgah.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::GOODBYE_AT_UMGAH.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::must_explain_presence.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::EXPLAIN_NOTHING_MONKEY_BOY.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::umgah_zombies.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::WORKS_LIKE_THIS.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::talking_pets_dumb.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OH_NO_YOU_DONT.0 as ::core::ffi::c_int, None), ());
        }
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_UMGAH_ZOMBIES\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        if uqm_lua_checked!(getGameStateUint(
            b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"PLAYER_HYPNOTIZED\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::CANT_COMPEL.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
        }
        return;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_are_you.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        pStr[0usize] = C2Rust_Unnamed_1::what_are_you.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    } else {
        pStr[0usize] = C2Rust_Unnamed_1::talking_pets_dumb.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    }
    if uqm_lua_checked!(getGameStateUint(b"KNOW_UMGAH_ZOMBIES\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_do_to_umgah.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            pStr[1usize] = C2Rust_Unnamed_1::what_do_to_umgah.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        } else {
            pStr[1usize] = C2Rust_Unnamed_1::umgah_zombies.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        pStr[2usize] = C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    } else {
        pStr[2usize] = C2Rust_Unnamed_1::must_explain_presence.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    }
    if pStr[LastStack as usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pStr[LastStack as usize],
            Some(CompelPlayer as unsafe extern "C" fn(RESPONSE_REF) -> ()),
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
                Some(CompelPlayer as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye_at_umgah.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(CompelPlayer as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn KillPet(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::must_kill.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DONT_KILL.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        MindControlStrobe();
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::want_kill_1.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::want_kill_2.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::want_kill_3.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn PetDeal(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_kind_of_deal.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HELP_DEFEAT_URQUAN.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_kind_of_deal.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::how_trust.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TRUST.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::how_trust.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::boneless_dweeb.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::YOUR_BONELESS_DWEEB.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::boneless_dweeb.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_are_you_really.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::POOR_DNYARRI.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_are_you_really.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::hard_to_believe.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ITS_TRUE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"DNYARRI_LIED\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::hard_to_believe.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bullshit.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WORTH_A_TRY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::bullshit.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::kill_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::PLEASE_DONT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::kill_you.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_kind_of_deal.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_kind_of_deal.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::how_trust.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::how_trust.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::boneless_dweeb.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::boneless_dweeb.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::ok_lets_do_it.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_are_you_really.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_are_you_really.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::hard_to_believe.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"LEARNED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::hard_to_believe.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::bullshit.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"LEARNED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::bullshit.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::kill_you.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::kill_you.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PetDeal as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::must_kill.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::must_kill.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(KillPet as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OUT_TAKES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        return;
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
        if GlobData.Game_state.glob_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HUMAN_PEP_TALK.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::CYBORG_PEP_TALK.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
    } else if uqm_lua_checked!(getGameStateUint(
        b"READY_TO_CONFUSE_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        uqm_lua_checked!(setGameStateUint(
            b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"READY_TO_CONFUSE_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 3 as DWORD
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::HAVENT_GOT_EVERYTHING.0 as ::core::ffi::c_int,
                None,
            ), ());
            if uqm_lua_checked!(getGameStateUint(
                b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
            {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::NEED_BOMB.0 as ::core::ffi::c_int, None), ());
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SOUP_UP_BOMB.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"URQUAN_MESSED_UP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::HELLO_AFTER_COMPEL_URQUAN.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::I_SENSE_MY_SLAVES.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(MindFuckUrquan(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        }
    } else if uqm_lua_checked!(getGameStateUint(
        b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if uqm_lua_checked!(getGameStateUint(
            b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0 && NumVisits as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            NumVisits = NumVisits.wrapping_add(1);
        }
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AS_DEVICE_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"TALKING_PET_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(PetDevice(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if !GlobData.Game_state.npc_built_ship_q.head.is_null() {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh1 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh1 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(setGameStateUint(
                    b"UMGAH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HELLO_AT_UMGAH.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HYPNOTIZE_AGAIN_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HYPNOTIZE_AGAIN_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HYPNOTIZE_AGAIN_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HYPNOTIZE_AGAIN_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"TALKING_PET_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(SetHomeworldKnown(C2Rust_Unnamed_2::UMGAH_HOME.0 as ::core::ffi::c_int as DWORD), ());
        if NumVisits as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            uqm_lua_checked!(CompelPlayer(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"PLAYER_HYPNOTIZED\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::CANT_COMPEL.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
        }
    } else {
        if StartSphereTracking(RACE_ID::UMGAH_SHIP) != 0 {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::LETS_MAKE_A_DEAL.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::UMGAH_ALL_GONE.0 as ::core::ffi::c_int, None), ());
            if uqm_lua_checked!(getGameStateUint(
                b"TALKING_PET_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0 as DWORD
                || uqm_lua_checked!(getGameStateUint(
                    b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) == 0
            {
                *GetStringAddress(
                    SetAbsStringTableIndex(
                        CommData.ConversationPhrases,
                        (C2Rust_Unnamed_1::how_trust.0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ) = '\0' as UNICODE;
            }
        }
        uqm_lua_checked!(PetDeal(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_talkpet() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_talkpet_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_talkpet_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    talkpet_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    talkpet_desc.post_encounter_func = Some(
        post_talkpet_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    talkpet_desc.uninit_encounter_func = Some(
        uninit_talkpet as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    talkpet_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    talkpet_desc.AlienTextBaseline.y = 0 as COORD;
    talkpet_desc.AlienTextWidth = (CanvasWidth
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
        - ((16 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_0::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut talkpet_desc;
    return retval;
}
