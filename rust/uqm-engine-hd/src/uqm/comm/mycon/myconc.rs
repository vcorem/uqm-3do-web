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

// Rust translation of src/uqm/comm/mycon/myconc.c.
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
    fn TFB_Random() -> DWORD;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const TELL_US_ABOUT_WORLD: Self = Self(1);
    pub const BYE_AND_DIE_HOMEWORLD: Self = Self(2);
    pub const RAMBLE_1: Self = Self(3);
    pub const RAMBLE_2: Self = Self(4);
    pub const RAMBLE_3: Self = Self(5);
    pub const RAMBLE_4: Self = Self(6);
    pub const RAMBLE_5: Self = Self(7);
    pub const RAMBLE_6: Self = Self(8);
    pub const RAMBLE_7: Self = Self(9);
    pub const RAMBLE_8: Self = Self(10);
    pub const RAMBLE_9: Self = Self(11);
    pub const RAMBLE_10: Self = Self(12);
    pub const RAMBLE_11: Self = Self(13);
    pub const RAMBLE_12: Self = Self(14);
    pub const RAMBLE_13: Self = Self(15);
    pub const RAMBLE_14: Self = Self(16);
    pub const RAMBLE_15: Self = Self(17);
    pub const RAMBLE_16: Self = Self(18);
    pub const RAMBLE_17: Self = Self(19);
    pub const RAMBLE_18: Self = Self(20);
    pub const RAMBLE_19: Self = Self(21);
    pub const RAMBLE_20: Self = Self(22);
    pub const RAMBLE_21: Self = Self(23);
    pub const RAMBLE_22: Self = Self(24);
    pub const RAMBLE_23: Self = Self(25);
    pub const RAMBLE_24: Self = Self(26);
    pub const RAMBLE_25: Self = Self(27);
    pub const RAMBLE_26: Self = Self(28);
    pub const RAMBLE_27: Self = Self(29);
    pub const RAMBLE_28: Self = Self(30);
    pub const RAMBLE_29: Self = Self(31);
    pub const RAMBLE_30: Self = Self(32);
    pub const RAMBLE_31: Self = Self(33);
    pub const RAMBLE_32: Self = Self(34);
    pub const question_1: Self = Self(35);
    pub const question_2: Self = Self(36);
    pub const question_3: Self = Self(37);
    pub const question_4: Self = Self(38);
    pub const question_5: Self = Self(39);
    pub const question_6: Self = Self(40);
    pub const question_7: Self = Self(41);
    pub const question_8: Self = Self(42);
    pub const question_9: Self = Self(43);
    pub const question_10: Self = Self(44);
    pub const question_11: Self = Self(45);
    pub const question_12: Self = Self(46);
    pub const question_13: Self = Self(47);
    pub const question_14: Self = Self(48);
    pub const question_15: Self = Self(49);
    pub const question_16: Self = Self(50);
    pub const bye_space: Self = Self(51);
    pub const BYE_AND_DIE_SPACE: Self = Self(52);
    pub const gonna_die: Self = Self(53);
    pub const insult_1: Self = Self(54);
    pub const insult_2: Self = Self(55);
    pub const insult_3: Self = Self(56);
    pub const insult_4: Self = Self(57);
    pub const insult_5: Self = Self(58);
    pub const insult_6: Self = Self(59);
    pub const insult_7: Self = Self(60);
    pub const insult_8: Self = Self(61);
    pub const come_in_peace: Self = Self(62);
    pub const HELLO_HOMEWORLD_1: Self = Self(63);
    pub const HELLO_HOMEWORLD_2: Self = Self(64);
    pub const HELLO_HOMEWORLD_3: Self = Self(65);
    pub const HELLO_HOMEWORLD_4: Self = Self(66);
    pub const HELLO_HOMEWORLD_5: Self = Self(67);
    pub const HELLO_HOMEWORLD_6: Self = Self(68);
    pub const HELLO_HOMEWORLD_7: Self = Self(69);
    pub const HELLO_HOMEWORLD_8: Self = Self(70);
    pub const HELLO_SPACE_1: Self = Self(71);
    pub const HELLO_SPACE_2: Self = Self(72);
    pub const HELLO_SPACE_3: Self = Self(73);
    pub const HELLO_SPACE_4: Self = Self(74);
    pub const HELLO_SPACE_5: Self = Self(75);
    pub const HELLO_SPACE_6: Self = Self(76);
    pub const HELLO_SPACE_7: Self = Self(77);
    pub const HELLO_SPACE_8: Self = Self(78);
    pub const lets_be_friends: Self = Self(79);
    pub const came_to_homeworld: Self = Self(80);
    pub const submit_to_us: Self = Self(81);
    pub const bye_sun_device: Self = Self(82);
    pub const GOODBYE_SUN_DEVICE: Self = Self(83);
    pub const RESPONSE_1: Self = Self(84);
    pub const RESPONSE_2: Self = Self(85);
    pub const RESPONSE_3: Self = Self(86);
    pub const clue_1: Self = Self(87);
    pub const clue_2: Self = Self(88);
    pub const clue_3: Self = Self(89);
    pub const what_about_shattered: Self = Self(90);
    pub const ABOUT_SHATTERED: Self = Self(91);
    pub const HELLO_SUN_DEVICE_WORLD_1: Self = Self(92);
    pub const HELLO_SUN_DEVICE_WORLD_2: Self = Self(93);
    pub const whats_up_sun_device: Self = Self(94);
    pub const GENERAL_INFO_SUN_DEVICE: Self = Self(95);
    pub const like_to_land: Self = Self(96);
    pub const NEVER_LET_LAND: Self = Self(97);
    pub const bye_homeworld: Self = Self(98);
    pub const i_have_a_cunning_plan: Self = Self(99);
    pub const DIE_LIAR: Self = Self(100);
    pub const how_goes_implanting: Self = Self(101);
    pub const UNFORSEEN_DELAYS: Self = Self(102);
    pub const DIE_THIEF: Self = Self(103);
    pub const DIE_THIEF_AGAIN: Self = Self(104);
    pub const GOODBYE_AND_DIE: Self = Self(105);
    pub const AMBUSH_TAIL: Self = Self(106);
    pub const RAMBLE_TAIL: Self = Self(107);
    pub const WE_GO_TO_IMPLANT: Self = Self(108);
    pub const WONT_FALL_FOR_TRICK: Self = Self(109);
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
static mut mycon_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::MYCON_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.mycon.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.mycon.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.mycon.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.mycon.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.mycon.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 4 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 12 as COUNT,
            NumFrames: 10 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 80 as ::core::ffi::c_int) as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 22 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 28 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 33 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 40 as ::core::ffi::c_int) as COUNT,
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
static mut MadeChoice: BYTE = 0;
unsafe extern "C" fn DoRamble(mut R: RESPONSE_REF) {
    let mut Counter: BYTE = 0;
    Counter = uqm_lua_checked!(getGameStateUint(b"MYCON_RAMBLE\0".as_ptr() as *const ::core::ffi::c_char), ())
        as BYTE;
    let c2rust_fresh4 = Counter;
    Counter = Counter.wrapping_add(1);
    match c2rust_fresh4 as ::core::ffi::c_int {
        0 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_1.0 as ::core::ffi::c_int, None), ());
        }
        1 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_2.0 as ::core::ffi::c_int, None), ());
        }
        2 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_3.0 as ::core::ffi::c_int, None), ());
        }
        3 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_4.0 as ::core::ffi::c_int, None), ());
        }
        4 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_5.0 as ::core::ffi::c_int, None), ());
        }
        5 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_6.0 as ::core::ffi::c_int, None), ());
        }
        6 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_7.0 as ::core::ffi::c_int, None), ());
        }
        7 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_8.0 as ::core::ffi::c_int, None), ());
        }
        8 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_9.0 as ::core::ffi::c_int, None), ());
        }
        9 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_10.0 as ::core::ffi::c_int, None), ());
        }
        10 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_11.0 as ::core::ffi::c_int, None), ());
        }
        11 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_12.0 as ::core::ffi::c_int, None), ());
        }
        12 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_13.0 as ::core::ffi::c_int, None), ());
        }
        13 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_14.0 as ::core::ffi::c_int, None), ());
        }
        14 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_15.0 as ::core::ffi::c_int, None), ());
        }
        15 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_16.0 as ::core::ffi::c_int, None), ());
        }
        16 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_17.0 as ::core::ffi::c_int, None), ());
        }
        17 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_18.0 as ::core::ffi::c_int, None), ());
        }
        18 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_19.0 as ::core::ffi::c_int, None), ());
        }
        19 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_20.0 as ::core::ffi::c_int, None), ());
        }
        20 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_21.0 as ::core::ffi::c_int, None), ());
        }
        21 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_22.0 as ::core::ffi::c_int, None), ());
        }
        22 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_23.0 as ::core::ffi::c_int, None), ());
        }
        23 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_24.0 as ::core::ffi::c_int, None), ());
        }
        24 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_25.0 as ::core::ffi::c_int, None), ());
            if uqm_lua_checked!(getGameStateUint(
                b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) < 2 as DWORD
            {
                uqm_lua_checked!(setGameStateUint(
                    b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
                    2 as DWORD,
                ), ());
            }
        }
        25 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_26.0 as ::core::ffi::c_int, None), ());
        }
        26 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_27.0 as ::core::ffi::c_int, None), ());
        }
        27 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_28.0 as ::core::ffi::c_int, None), ());
        }
        28 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_29.0 as ::core::ffi::c_int, None), ());
        }
        29 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_30.0 as ::core::ffi::c_int, None), ());
        }
        30 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_31.0 as ::core::ffi::c_int, None), ());
        }
        31 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_32.0 as ::core::ffi::c_int, None), ());
            Counter = 0 as BYTE;
        }
        _ => {}
    }
    uqm_lua_checked!(setGameStateUint(
        b"MYCON_RAMBLE\0".as_ptr() as *const ::core::ffi::c_char,
        Counter as DWORD,
    ), ());
    if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD == 0
    {
        if !(R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::come_in_peace.0 as ::core::ffi::c_int)
            && !(R as ::core::ffi::c_int
                == C2Rust_Unnamed_0::gonna_die.0 as ::core::ffi::c_int)
        {
            Counter = (uqm_lua_checked!(getGameStateUint(
                    b"MYCON_INSULTS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ())
                .wrapping_add(1 as DWORD) & 7 as DWORD) as BYTE;
            uqm_lua_checked!(setGameStateUint(
                b"MYCON_INSULTS\0".as_ptr() as *const ::core::ffi::c_char,
                Counter as DWORD,
            ), ());
            MadeChoice = 1 as BYTE;
        }
    } else if !(R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::lets_be_friends.0 as ::core::ffi::c_int)
        && !(R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::came_to_homeworld.0 as ::core::ffi::c_int)
        && !(R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::submit_to_us.0 as ::core::ffi::c_int)
    {
        Counter = (uqm_lua_checked!(getGameStateUint(
                b"MYCON_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .wrapping_add(1 as DWORD) & 15 as DWORD) as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"MYCON_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            Counter as DWORD,
        ), ());
        MadeChoice = 1 as BYTE;
    }
}
unsafe extern "C" fn CombatIsInevitable(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::bye_space.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::BYE_AND_DIE_SPACE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_homeworld.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::BYE_AND_DIE_HOMEWORLD.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::like_to_land.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NEVER_LET_LAND.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_sun_device.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_SUN_DEVICE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    } else {
        uqm_lua_checked!(DoRamble(R), ());
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::BYE_AND_DIE_SPACE.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::BYE_AND_DIE_HOMEWORLD.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
    }
    MadeChoice = 0 as BYTE;
}
unsafe extern "C" fn SunDevice(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_sun_device.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GENERAL_INFO_SUN_DEVICE.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_sun_device.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_goes_implanting.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::UNFORSEEN_DELAYS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_implanting.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::i_have_a_cunning_plan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WONT_FALL_FOR_TRICK.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"NO_TRICK_AT_SUN\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_sun_device.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_sun_device.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SunDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"MYCON_FELL_FOR_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_implanting.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"MYCON_FELL_FOR_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::how_goes_implanting.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(SunDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::like_to_land.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if uqm_lua_checked!(getGameStateUint(b"MYCON_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
        && uqm_lua_checked!(getGameStateUint(b"NO_TRICK_AT_SUN\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::i_have_a_cunning_plan.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SunDevice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_sun_device.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn TrickMycon(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::i_have_a_cunning_plan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TELL_US_ABOUT_WORLD.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::i_have_a_cunning_plan.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_0::clue_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RESPONSE_1.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::clue_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_0::clue_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RESPONSE_2.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::clue_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_0::clue_3.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RESPONSE_3.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::clue_3.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if (*GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::clue_1.0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::clue_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && (*GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::clue_3.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WE_GO_TO_IMPLANT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        uqm_lua_checked!(setGameStateUint(
            b"MYCON_FELL_FOR_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_1::ADVANCE_MYCON_MISSION.0 as ::core::ffi::c_int as BYTE,
        ), ());
    } else {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::AMBUSH_TAIL.0 as ::core::ffi::c_int, None), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::clue_1.0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::clue_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TrickMycon as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::clue_2.0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::clue_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TrickMycon as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::clue_3.0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::clue_3.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TrickMycon as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
}
unsafe extern "C" fn NormalMycon(mut R: RESPONSE_REF) {
    let mut RespFunc: RESPONSE_FUNC = None;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_shattered.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_SHATTERED.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R != 0 {
        uqm_lua_checked!(DoRamble(R), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RAMBLE_TAIL.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (R as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if (TFB_Random() as BYTE as ::core::ffi::c_int)
        < 256 as ::core::ffi::c_int * 30 as ::core::ffi::c_int
            / 100 as ::core::ffi::c_int
    {
        RespFunc = Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ());
    } else {
        RespFunc = Some(NormalMycon as unsafe extern "C" fn(RESPONSE_REF) -> ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD == 0
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::come_in_peace.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::come_in_peace.0 as ::core::ffi::c_int as RESPONSE_REF,
                RespFunc,
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::gonna_die.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::gonna_die.0 as ::core::ffi::c_int as RESPONSE_REF,
                RespFunc,
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if MadeChoice == 0 {
            match uqm_lua_checked!(getGameStateUint(
                b"MYCON_INSULTS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                0 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_1.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_2.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_3.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_4.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                4 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_5.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                5 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_6.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                6 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_7.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                7 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::insult_8.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                _ => {}
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::bye_space.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        if MadeChoice == 0 {
            match uqm_lua_checked!(getGameStateUint(
                b"MYCON_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                0 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_1.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_2.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_3.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_4.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                4 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_5.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                5 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_6.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                6 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_7.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                7 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_8.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                8 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_9.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                9 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_10.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                10 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_11.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                11 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_12.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                12 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_13.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                13 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_14.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                14 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_15.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                15 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::question_16.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        RespFunc,
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                _ => {}
            }
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::lets_be_friends.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::lets_be_friends.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                RespFunc,
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::came_to_homeworld.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::came_to_homeworld.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                RespFunc,
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::submit_to_us.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::submit_to_us.0 as ::core::ffi::c_int as RESPONSE_REF,
                RespFunc,
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"MYCON_FELL_FOR_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
        {
            if uqm_lua_checked!(getGameStateUint(
                b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 1 as DWORD
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::what_about_shattered.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NormalMycon as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            if uqm_lua_checked!(getGameStateUint(b"MYCON_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char), ())
                != 0
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::i_have_a_cunning_plan.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(TrickMycon as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::bye_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    if uqm_lua_checked!(getGameStateUint(b"SUN_DEVICE\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0 {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"MYCON_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::DIE_THIEF.0 as ::core::ffi::c_int, None), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::DIE_THIEF_AGAIN.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"MYCON_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"MYCON_KNOW_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::DIE_LIAR.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"MYCON_SUN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh1 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh1 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SUN_DEVICE_WORLD_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
                uqm_lua_checked!(SetHomeworldKnown(
                    C2Rust_Unnamed_2::MYCON_HOME.0 as ::core::ffi::c_int as DWORD,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SUN_DEVICE_WORLD_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"MYCON_SUN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(SunDevice(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"MYCON_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh2 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_HOMEWORLD_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"MYCON_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(NormalMycon(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"MYCON_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
                uqm_lua_checked!(SetHomeworldKnown(
                    C2Rust_Unnamed_2::MYCON_HOME.0 as ::core::ffi::c_int as DWORD,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HELLO_SPACE_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"MYCON_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(NormalMycon(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_mycon() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_mycon_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_mycon_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    mycon_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    mycon_desc.post_encounter_func = Some(post_mycon_enc as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    mycon_desc.uninit_encounter_func = Some(
        uninit_mycon as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    mycon_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    mycon_desc.AlienTextBaseline.y = 0 as COORD;
    mycon_desc.AlienTextWidth = (CanvasWidth
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
    MadeChoice = 0 as BYTE;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    }
    retval = &raw mut mycon_desc;
    return retval;
}
