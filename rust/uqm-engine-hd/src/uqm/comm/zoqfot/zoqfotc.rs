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

// Rust translation of src/uqm/comm/zoqfot/zoqfotc.c.
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
    static mut optDeCleansing: OPT_ENABLABLE;
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
    fn AlienTalkSegue(wait_track: COUNT);
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn SetRaceAllied(race: RACE_ID, flag: BOOLEAN) -> BOOLEAN;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
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
    pub const WE_ARE0: Self = Self(1);
    pub const WE_ARE1: Self = Self(2);
    pub const WE_ARE2: Self = Self(3);
    pub const WE_ARE3: Self = Self(4);
    pub const WE_ARE4: Self = Self(5);
    pub const WE_ARE5: Self = Self(6);
    pub const WE_ARE6: Self = Self(7);
    pub const WE_ARE7: Self = Self(8);
    pub const SCOUT_HELLO0: Self = Self(9);
    pub const SCOUT_HELLO1: Self = Self(10);
    pub const SCOUT_HELLO2: Self = Self(11);
    pub const SCOUT_HELLO3: Self = Self(12);
    pub const INIT_HOME_HELLO0: Self = Self(13);
    pub const INIT_HOME_HELLO1: Self = Self(14);
    pub const INIT_HOME_HELLO2: Self = Self(15);
    pub const INIT_HOME_HELLO3: Self = Self(16);
    pub const which_fot: Self = Self(17);
    pub const HE_IS0: Self = Self(18);
    pub const HE_IS1: Self = Self(19);
    pub const HE_IS2: Self = Self(20);
    pub const HE_IS3: Self = Self(21);
    pub const HE_IS4: Self = Self(22);
    pub const HE_IS5: Self = Self(23);
    pub const HE_IS6: Self = Self(24);
    pub const HE_IS7: Self = Self(25);
    pub const we_are_vindicator: Self = Self(26);
    pub const WE_GLAD0: Self = Self(27);
    pub const WE_GLAD1: Self = Self(28);
    pub const WE_GLAD2: Self = Self(29);
    pub const WE_GLAD3: Self = Self(30);
    pub const WE_GLAD4: Self = Self(31);
    pub const WE_GLAD5: Self = Self(32);
    pub const quiet_toadies: Self = Self(33);
    pub const TOLD_YOU0: Self = Self(34);
    pub const TOLD_YOU1: Self = Self(35);
    pub const TOLD_YOU2: Self = Self(36);
    pub const TOLD_YOU3: Self = Self(37);
    pub const TOLD_YOU4: Self = Self(38);
    pub const TOLD_YOU5: Self = Self(39);
    pub const TOLD_YOU6: Self = Self(40);
    pub const TOLD_YOU7: Self = Self(41);
    pub const your_race: Self = Self(42);
    pub const YEARS_AGO0: Self = Self(43);
    pub const YEARS_AGO1: Self = Self(44);
    pub const YEARS_AGO2: Self = Self(45);
    pub const YEARS_AGO3: Self = Self(46);
    pub const YEARS_AGO4: Self = Self(47);
    pub const YEARS_AGO5: Self = Self(48);
    pub const YEARS_AGO6: Self = Self(49);
    pub const YEARS_AGO7: Self = Self(50);
    pub const YEARS_AGO8: Self = Self(51);
    pub const YEARS_AGO9: Self = Self(52);
    pub const YEARS_AGO10: Self = Self(53);
    pub const YEARS_AGO11: Self = Self(54);
    pub const YEARS_AGO12: Self = Self(55);
    pub const YEARS_AGO13: Self = Self(56);
    pub const where_from: Self = Self(57);
    pub const TRAVELED_FAR0: Self = Self(58);
    pub const TRAVELED_FAR1: Self = Self(59);
    pub const TRAVELED_FAR2: Self = Self(60);
    pub const TRAVELED_FAR3: Self = Self(61);
    pub const TRAVELED_FAR4: Self = Self(62);
    pub const TRAVELED_FAR5: Self = Self(63);
    pub const what_emergency: Self = Self(64);
    pub const UNDER_ATTACK0: Self = Self(65);
    pub const UNDER_ATTACK1: Self = Self(66);
    pub const UNDER_ATTACK2: Self = Self(67);
    pub const UNDER_ATTACK3: Self = Self(68);
    pub const UNDER_ATTACK4: Self = Self(69);
    pub const UNDER_ATTACK5: Self = Self(70);
    pub const UNDER_ATTACK6: Self = Self(71);
    pub const UNDER_ATTACK7: Self = Self(72);
    pub const UNDER_ATTACK8: Self = Self(73);
    pub const UNDER_ATTACK9: Self = Self(74);
    pub const UNDER_ATTACK10: Self = Self(75);
    pub const UNDER_ATTACK11: Self = Self(76);
    pub const tough_luck: Self = Self(77);
    pub const NOT_HELPFUL0: Self = Self(78);
    pub const NOT_HELPFUL1: Self = Self(79);
    pub const NOT_HELPFUL2: Self = Self(80);
    pub const NOT_HELPFUL3: Self = Self(81);
    pub const NOT_HELPFUL4: Self = Self(82);
    pub const NOT_HELPFUL5: Self = Self(83);
    pub const what_look_like: Self = Self(84);
    pub const LOOK_LIKE0: Self = Self(85);
    pub const LOOK_LIKE1: Self = Self(86);
    pub const LOOK_LIKE2: Self = Self(87);
    pub const LOOK_LIKE3: Self = Self(88);
    pub const valuable_info: Self = Self(89);
    pub const GOODBYE0: Self = Self(90);
    pub const GOODBYE1: Self = Self(91);
    pub const GOODBYE2: Self = Self(92);
    pub const GOODBYE3: Self = Self(93);
    pub const all_very_interesting: Self = Self(94);
    pub const SEE_TOLD_YOU0: Self = Self(95);
    pub const SEE_TOLD_YOU1: Self = Self(96);
    pub const SEE_TOLD_YOU2: Self = Self(97);
    pub const SEE_TOLD_YOU3: Self = Self(98);
    pub const how_can_i_help: Self = Self(99);
    pub const ALLY_WITH_US0: Self = Self(100);
    pub const ALLY_WITH_US1: Self = Self(101);
    pub const ALLY_WITH_US2: Self = Self(102);
    pub const ALLY_WITH_US3: Self = Self(103);
    pub const ALLY_WITH_US4: Self = Self(104);
    pub const ALLY_WITH_US5: Self = Self(105);
    pub const decide_later: Self = Self(106);
    pub const PLEASE_HURRY0: Self = Self(107);
    pub const PLEASE_HURRY1: Self = Self(108);
    pub const EMMISSARIES0: Self = Self(109);
    pub const EMMISSARIES1: Self = Self(110);
    pub const EMMISSARIES2: Self = Self(111);
    pub const EMMISSARIES3: Self = Self(112);
    pub const EMMISSARIES4: Self = Self(113);
    pub const EMMISSARIES5: Self = Self(114);
    pub const EMMISSARIES6: Self = Self(115);
    pub const EMMISSARIES7: Self = Self(116);
    pub const sure: Self = Self(117);
    pub const WE_ALLY0: Self = Self(118);
    pub const WE_ALLY1: Self = Self(119);
    pub const WE_ALLY2: Self = Self(120);
    pub const WE_ALLY3: Self = Self(121);
    pub const WE_ALLY4: Self = Self(122);
    pub const WE_ALLY5: Self = Self(123);
    pub const never: Self = Self(124);
    pub const WE_ENEMIES0: Self = Self(125);
    pub const WE_ENEMIES1: Self = Self(126);
    pub const HOSTILE_HELLO_10: Self = Self(127);
    pub const HOSTILE_HELLO_11: Self = Self(128);
    pub const HOSTILE_HELLO_20: Self = Self(129);
    pub const HOSTILE_HELLO_21: Self = Self(130);
    pub const HOSTILE_HELLO_22: Self = Self(131);
    pub const HOSTILE_HELLO_23: Self = Self(132);
    pub const HOSTILE_HELLO_24: Self = Self(133);
    pub const HOSTILE_HELLO_25: Self = Self(134);
    pub const HOSTILE_HELLO_30: Self = Self(135);
    pub const HOSTILE_HELLO_31: Self = Self(136);
    pub const HOSTILE_HELLO_40: Self = Self(137);
    pub const HOSTILE_HELLO_41: Self = Self(138);
    pub const NEUTRAL_HOME_HELLO_10: Self = Self(139);
    pub const NEUTRAL_HOME_HELLO_11: Self = Self(140);
    pub const NEUTRAL_HOME_HELLO_12: Self = Self(141);
    pub const NEUTRAL_HOME_HELLO_13: Self = Self(142);
    pub const NEUTRAL_HOME_HELLO_20: Self = Self(143);
    pub const NEUTRAL_HOME_HELLO_21: Self = Self(144);
    pub const NEUTRAL_HOME_HELLO_22: Self = Self(145);
    pub const NEUTRAL_HOME_HELLO_23: Self = Self(146);
    pub const ALLIED_HOME_HELLO_10: Self = Self(147);
    pub const ALLIED_HOME_HELLO_11: Self = Self(148);
    pub const ALLIED_HOME_HELLO_12: Self = Self(149);
    pub const ALLIED_HOME_HELLO_13: Self = Self(150);
    pub const ALLIED_HOME_HELLO_20: Self = Self(151);
    pub const ALLIED_HOME_HELLO_21: Self = Self(152);
    pub const ALLIED_HOME_HELLO_22: Self = Self(153);
    pub const ALLIED_HOME_HELLO_23: Self = Self(154);
    pub const ALLIED_HOME_HELLO_24: Self = Self(155);
    pub const ALLIED_HOME_HELLO_25: Self = Self(156);
    pub const ALLIED_HOME_HELLO_26: Self = Self(157);
    pub const ALLIED_HOME_HELLO_27: Self = Self(158);
    pub const ALLIED_HOME_HELLO_30: Self = Self(159);
    pub const ALLIED_HOME_HELLO_31: Self = Self(160);
    pub const ALLIED_HOME_HELLO_40: Self = Self(161);
    pub const ALLIED_HOME_HELLO_41: Self = Self(162);
    pub const THANKS_FOR_RESCUE0: Self = Self(163);
    pub const THANKS_FOR_RESCUE1: Self = Self(164);
    pub const THANKS_FOR_RESCUE2: Self = Self(165);
    pub const THANKS_FOR_RESCUE3: Self = Self(166);
    pub const THANKS_FOR_RESCUE4: Self = Self(167);
    pub const THANKS_FOR_RESCUE5: Self = Self(168);
    pub const THANKS_FOR_RESCUE6: Self = Self(169);
    pub const THANKS_FOR_RESCUE7: Self = Self(170);
    pub const THANKS_FOR_RESCUE8: Self = Self(171);
    pub const THANKS_FOR_RESCUE9: Self = Self(172);
    pub const THANKS_FOR_RESCUE10: Self = Self(173);
    pub const THANKS_FOR_RESCUE11: Self = Self(174);
    pub const bye_homeworld: Self = Self(175);
    pub const GOODBYE_HOME0: Self = Self(176);
    pub const GOODBYE_HOME1: Self = Self(177);
    pub const whats_up_homeworld: Self = Self(178);
    pub const GENERAL_INFO_10: Self = Self(179);
    pub const GENERAL_INFO_11: Self = Self(180);
    pub const GENERAL_INFO_12: Self = Self(181);
    pub const GENERAL_INFO_13: Self = Self(182);
    pub const GENERAL_INFO_20: Self = Self(183);
    pub const GENERAL_INFO_21: Self = Self(184);
    pub const GENERAL_INFO_22: Self = Self(185);
    pub const GENERAL_INFO_23: Self = Self(186);
    pub const GENERAL_INFO_24: Self = Self(187);
    pub const GENERAL_INFO_25: Self = Self(188);
    pub const GENERAL_INFO_26: Self = Self(189);
    pub const GENERAL_INFO_27: Self = Self(190);
    pub const GENERAL_INFO_30: Self = Self(191);
    pub const GENERAL_INFO_31: Self = Self(192);
    pub const GENERAL_INFO_32: Self = Self(193);
    pub const GENERAL_INFO_33: Self = Self(194);
    pub const GENERAL_INFO_34: Self = Self(195);
    pub const GENERAL_INFO_35: Self = Self(196);
    pub const GENERAL_INFO_40: Self = Self(197);
    pub const GENERAL_INFO_41: Self = Self(198);
    pub const GENERAL_INFO_42: Self = Self(199);
    pub const GENERAL_INFO_43: Self = Self(200);
    pub const GENERAL_INFO_44: Self = Self(201);
    pub const GENERAL_INFO_45: Self = Self(202);
    pub const GENERAL_INFO_46: Self = Self(203);
    pub const GENERAL_INFO_47: Self = Self(204);
    pub const GENERAL_INFO_48: Self = Self(205);
    pub const GENERAL_INFO_49: Self = Self(206);
    pub const GENERAL_INFO_410: Self = Self(207);
    pub const GENERAL_INFO_411: Self = Self(208);
    pub const any_war_news: Self = Self(209);
    pub const UTWIG_DELAY0: Self = Self(210);
    pub const UTWIG_DELAY1: Self = Self(211);
    pub const UTWIG_DELAY2: Self = Self(212);
    pub const UTWIG_DELAY3: Self = Self(213);
    pub const UTWIG_DELAY4: Self = Self(214);
    pub const UTWIG_DELAY5: Self = Self(215);
    pub const UTWIG_DELAY6: Self = Self(216);
    pub const UTWIG_DELAY7: Self = Self(217);
    pub const UTWIG_DELAY8: Self = Self(218);
    pub const UTWIG_DELAY9: Self = Self(219);
    pub const UTWIG_DELAY10: Self = Self(220);
    pub const UTWIG_DELAY11: Self = Self(221);
    pub const UTWIG_DELAY12: Self = Self(222);
    pub const UTWIG_DELAY13: Self = Self(223);
    pub const KOHRAH_WINNING0: Self = Self(224);
    pub const KOHRAH_WINNING1: Self = Self(225);
    pub const KOHRAH_WINNING2: Self = Self(226);
    pub const KOHRAH_WINNING3: Self = Self(227);
    pub const KOHRAH_WINNING4: Self = Self(228);
    pub const KOHRAH_WINNING5: Self = Self(229);
    pub const KOHRAH_WINNING6: Self = Self(230);
    pub const KOHRAH_WINNING7: Self = Self(231);
    pub const KOHRAH_WINNING8: Self = Self(232);
    pub const KOHRAH_WINNING9: Self = Self(233);
    pub const URQUAN_NEARLY_GONE0: Self = Self(234);
    pub const URQUAN_NEARLY_GONE1: Self = Self(235);
    pub const URQUAN_NEARLY_GONE2: Self = Self(236);
    pub const URQUAN_NEARLY_GONE3: Self = Self(237);
    pub const URQUAN_NEARLY_GONE4: Self = Self(238);
    pub const URQUAN_NEARLY_GONE5: Self = Self(239);
    pub const KOHRAH_FRENZY0: Self = Self(240);
    pub const KOHRAH_FRENZY1: Self = Self(241);
    pub const KOHRAH_FRENZY2: Self = Self(242);
    pub const KOHRAH_FRENZY3: Self = Self(243);
    pub const KOHRAH_FRENZY4: Self = Self(244);
    pub const KOHRAH_FRENZY5: Self = Self(245);
    pub const KOHRAH_FRENZY6: Self = Self(246);
    pub const KOHRAH_FRENZY7: Self = Self(247);
    pub const KOHRAH_FRENZY8: Self = Self(248);
    pub const KOHRAH_FRENZY9: Self = Self(249);
    pub const KOHRAH_FRENZY10: Self = Self(250);
    pub const KOHRAH_FRENZY11: Self = Self(251);
    pub const NO_WAR_NEWS0: Self = Self(252);
    pub const NO_WAR_NEWS1: Self = Self(253);
    pub const i_want_alliance: Self = Self(254);
    pub const GOOD0: Self = Self(255);
    pub const GOOD1: Self = Self(256);
    pub const GOOD2: Self = Self(257);
    pub const GOOD3: Self = Self(258);
    pub const GOOD4: Self = Self(259);
    pub const GOOD5: Self = Self(260);
    pub const GOOD6: Self = Self(261);
    pub const GOOD7: Self = Self(262);
    pub const GOOD8: Self = Self(263);
    pub const GOOD9: Self = Self(264);
    pub const want_specific_info: Self = Self(265);
    pub const WHAT_SPECIFIC_INFO0: Self = Self(266);
    pub const WHAT_SPECIFIC_INFO1: Self = Self(267);
    pub const enough_info: Self = Self(268);
    pub const OK_ENOUGH_INFO: Self = Self(269);
    pub const what_about_others: Self = Self(270);
    pub const ABOUT_OTHERS0: Self = Self(271);
    pub const ABOUT_OTHERS1: Self = Self(272);
    pub const ABOUT_OTHERS2: Self = Self(273);
    pub const ABOUT_OTHERS3: Self = Self(274);
    pub const ABOUT_OTHERS4: Self = Self(275);
    pub const ABOUT_OTHERS5: Self = Self(276);
    pub const ABOUT_OTHERS6: Self = Self(277);
    pub const ABOUT_OTHERS7: Self = Self(278);
    pub const ABOUT_OTHERS8: Self = Self(279);
    pub const ABOUT_OTHERS9: Self = Self(280);
    pub const ABOUT_OTHERS10: Self = Self(281);
    pub const ABOUT_OTHERS11: Self = Self(282);
    pub const ABOUT_OTHERS12: Self = Self(283);
    pub const ABOUT_OTHERS13: Self = Self(284);
    pub const what_about_zebranky: Self = Self(285);
    pub const ABOUT_ZEBRANKY0: Self = Self(286);
    pub const ABOUT_ZEBRANKY1: Self = Self(287);
    pub const ABOUT_ZEBRANKY2: Self = Self(288);
    pub const ABOUT_ZEBRANKY3: Self = Self(289);
    pub const ABOUT_ZEBRANKY4: Self = Self(290);
    pub const ABOUT_ZEBRANKY5: Self = Self(291);
    pub const ABOUT_ZEBRANKY6: Self = Self(292);
    pub const ABOUT_ZEBRANKY7: Self = Self(293);
    pub const what_about_past: Self = Self(294);
    pub const ABOUT_PAST0: Self = Self(295);
    pub const ABOUT_PAST1: Self = Self(296);
    pub const ABOUT_PAST2: Self = Self(297);
    pub const ABOUT_PAST3: Self = Self(298);
    pub const ABOUT_PAST4: Self = Self(299);
    pub const ABOUT_PAST5: Self = Self(300);
    pub const ABOUT_PAST6: Self = Self(301);
    pub const ABOUT_PAST7: Self = Self(302);
    pub const ABOUT_PAST8: Self = Self(303);
    pub const ABOUT_PAST9: Self = Self(304);
    pub const ABOUT_PAST10: Self = Self(305);
    pub const ABOUT_PAST11: Self = Self(306);
    pub const what_about_stinger: Self = Self(307);
    pub const ABOUT_STINGER0: Self = Self(308);
    pub const ABOUT_STINGER1: Self = Self(309);
    pub const ABOUT_STINGER2: Self = Self(310);
    pub const ABOUT_STINGER3: Self = Self(311);
    pub const ABOUT_STINGER4: Self = Self(312);
    pub const ABOUT_STINGER5: Self = Self(313);
    pub const what_about_guy_in_back: Self = Self(314);
    pub const ABOUT_GUY0: Self = Self(315);
    pub const ABOUT_GUY1: Self = Self(316);
    pub const name_1: Self = Self(317);
    pub const name_2: Self = Self(318);
    pub const name_3: Self = Self(319);
    pub const name_4: Self = Self(320);
    pub const OUT_TAKES0: Self = Self(321);
    pub const OUT_TAKES1: Self = Self(322);
    pub const OUT_TAKES2: Self = Self(323);
    pub const OUT_TAKES3: Self = Self(324);
    pub const OUT_TAKES4: Self = Self(325);
    pub const OUT_TAKES5: Self = Self(326);
    pub const OUT_TAKES6: Self = Self(327);
    pub const OUT_TAKES7: Self = Self(328);
    pub const OUT_TAKES8: Self = Self(329);
    pub const OUT_TAKES9: Self = Self(330);
    pub const OUT_TAKES10: Self = Self(331);
    pub const OUT_TAKES11: Self = Self(332);
    pub const OUT_TAKES12: Self = Self(333);
    pub const OUT_TAKES13: Self = Self(334);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const ZOQ_ALIEN: Self = Self(0);
    pub const FOT_ALIEN: Self = Self(1);
    pub const PIK_ALIEN: Self = Self(2);
}
#[inline]
unsafe extern "C" fn colorFromRgb15(mut r: BYTE, mut g: BYTE, mut b: BYTE) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = ((r as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | r as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.g = ((g as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | g as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.b = ((b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.a = 0xff as BYTE;
    return c;
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
static mut zoqfot_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::ZOQFOTPIK_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.zoqfotpik.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.zoqfotpik.font\0".as_ptr() as RESOURCE,
    AlienTextFColor: Color {
        r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
    AlienTextValign: TEXT_VALIGN::VALIGN_MIDDLE,
    AlienColorMapRes: b"comm.zoqfotpik.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.zoqfotpik.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.zoqfotpik.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 3 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 5 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 120 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 10 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                as COUNT,
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
        StartIndex: 23 as COUNT,
        NumFrames: 3 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: 0 as COUNT,
        RandomRestartRate: 0 as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienTalkDesc: ANIMATION_DESC {
        StartIndex: 18 as COUNT,
        NumFrames: 5 as BYTE,
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
static mut LastAlien: ::core::ffi::c_int = 0;
unsafe extern "C" fn SelectAlienZOQ(mut arg: CallbackArg) {
    if LastAlien != C2Rust_Unnamed_4::ZOQ_ALIEN.0 as ::core::ffi::c_int {
        if LastAlien != C2Rust_Unnamed_4::FOT_ALIEN.0 as ::core::ffi::c_int {
            CommData.AlienTransitionDesc.AnimFlags = (CommData
                .AlienTransitionDesc
                .AnimFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as COUNT;
        }
        LastAlien = C2Rust_Unnamed_4::ZOQ_ALIEN.0 as ::core::ffi::c_int;
        CommData.AlienTransitionDesc.AnimFlags = (CommData.AlienTransitionDesc.AnimFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as COUNT;
        CommData.AlienTransitionDesc.StartIndex = 23 as COUNT;
        CommData.AlienTalkDesc.StartIndex = 18 as COUNT;
        CommData.AlienTalkDesc.NumFrames = 5 as BYTE;
        CommData.AlienAmbientArray[1usize].AnimFlags = (CommData
            .AlienAmbientArray[1usize]
            .AnimFlags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as COUNT;
        CommData.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((1 as ::core::ffi::c_int) << resolutionFactor
                    << 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as SWORD as COORD;
        CommData.AlienTextBaseline.y = ((24 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
        CommData.AlienTextFColor = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        CommData.AlienTextBColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
}
unsafe extern "C" fn SelectAlienPIK(mut arg: CallbackArg) {
    if LastAlien != C2Rust_Unnamed_4::PIK_ALIEN.0 as ::core::ffi::c_int {
        if LastAlien != C2Rust_Unnamed_4::FOT_ALIEN.0 as ::core::ffi::c_int {
            CommData.AlienTransitionDesc.AnimFlags = (CommData
                .AlienTransitionDesc
                .AnimFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as COUNT;
        }
        LastAlien = C2Rust_Unnamed_4::PIK_ALIEN.0 as ::core::ffi::c_int;
        CommData.AlienTransitionDesc.AnimFlags = (CommData.AlienTransitionDesc.AnimFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as COUNT;
        CommData.AlienTransitionDesc.StartIndex = 26 as COUNT;
        CommData.AlienTalkDesc.StartIndex = 29 as COUNT;
        CommData.AlienTalkDesc.NumFrames = 2 as BYTE;
        CommData.AlienAmbientArray[1usize].AnimFlags = (CommData
            .AlienAmbientArray[1usize]
            .AnimFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT;
        CommData.AlienTextBaseline.x = (CanvasWidth
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
            - (((1 as ::core::ffi::c_int) << resolutionFactor)
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
                    - ((1 as ::core::ffi::c_int) << resolutionFactor
                        << 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int))) as SWORD as COORD;
        CommData.AlienTextBaseline.y = ((24 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
        CommData.AlienTextFColor = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        CommData.AlienTextBColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
}
unsafe extern "C" fn ZFPTalkSegue(mut wait_track: COUNT) {
    LastAlien = C2Rust_Unnamed_4::FOT_ALIEN.0 as ::core::ffi::c_int;
    SelectAlienZOQ(::core::ptr::null_mut::<::core::ffi::c_void>());
    uqm_lua_checked!(AlienTalkSegue(wait_track), ());
}
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::bye_homeworld.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE_HOME0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE_HOME1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::decide_later.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::PLEASE_HURRY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::PLEASE_HURRY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::valuable_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_can_i_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::EMMISSARIES7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(SetHomeworldKnown(
            C2Rust_Unnamed_2::ZOQFOTPIK_HOME.0 as ::core::ffi::c_int as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_0::sure.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ALLY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ALLY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ALLY2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ALLY3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ALLY4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ALLY5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        SetRaceAllied(RACE_ID::ZOQFOTPIK_SHIP, BOOLEAN::TRUE);
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            3 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_3::ZOQFOT_DISTRESS_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::all_very_interesting.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::SEE_TOLD_YOU0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::SEE_TOLD_YOU1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::SEE_TOLD_YOU2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::SEE_TOLD_YOU3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_0::never.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ENEMIES0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ENEMIES1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    }
}
unsafe extern "C" fn FormAlliance(mut R: RESPONSE_REF) {
    uqm_lua_checked!(NPCPhrase_cb(
        C2Rust_Unnamed_0::ALLY_WITH_US0.0 as ::core::ffi::c_int,
        Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
    ), ());
    uqm_lua_checked!(NPCPhrase_cb(
        C2Rust_Unnamed_0::ALLY_WITH_US1.0 as ::core::ffi::c_int,
        Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
    ), ());
    uqm_lua_checked!(NPCPhrase_cb(
        C2Rust_Unnamed_0::ALLY_WITH_US2.0 as ::core::ffi::c_int,
        Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
    ), ());
    uqm_lua_checked!(NPCPhrase_cb(
        C2Rust_Unnamed_0::ALLY_WITH_US3.0 as ::core::ffi::c_int,
        Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
    ), ());
    uqm_lua_checked!(NPCPhrase_cb(
        C2Rust_Unnamed_0::ALLY_WITH_US4.0 as ::core::ffi::c_int,
        Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
    ), ());
    uqm_lua_checked!(NPCPhrase_cb(
        C2Rust_Unnamed_0::ALLY_WITH_US5.0 as ::core::ffi::c_int,
        Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
    ), ());
    uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::sure.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::never.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::decide_later.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ZoqFotIntro(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_are_vindicator.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_GLAD0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_GLAD1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_GLAD2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_GLAD3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_GLAD4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_GLAD5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::we_are_vindicator.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::your_race.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO12.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::YEARS_AGO13.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::your_race.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::where_from.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TRAVELED_FAR0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TRAVELED_FAR1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TRAVELED_FAR2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TRAVELED_FAR3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TRAVELED_FAR4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TRAVELED_FAR5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::where_from.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_emergency.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::UNDER_ATTACK11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_emergency.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tough_luck.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NOT_HELPFUL0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NOT_HELPFUL1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NOT_HELPFUL2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NOT_HELPFUL3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NOT_HELPFUL4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NOT_HELPFUL5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tough_luck.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_look_like.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::LOOK_LIKE0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::LOOK_LIKE1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::LOOK_LIKE2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::LOOK_LIKE3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_look_like.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::your_race.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::where_from.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_emergency.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::your_race.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::your_race.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::where_from.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::where_from.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_emergency.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_emergency.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    } else {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tough_luck.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::tough_luck.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_look_like.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_look_like.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::all_very_interesting.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::how_can_i_help.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(FormAlliance as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::how_can_i_help.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::valuable_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn AquaintZoqFot(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::which_fot.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::HE_IS7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::which_fot.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::quiet_toadies.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::TOLD_YOU7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::quiet_toadies.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::which_fot.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::which_fot.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AquaintZoqFot as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::we_are_vindicator.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_are_vindicator.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::quiet_toadies.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::quiet_toadies.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AquaintZoqFot as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::all_very_interesting.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::valuable_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ZoqFotInfo(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::want_specific_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WHAT_SPECIFIC_INFO0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WHAT_SPECIFIC_INFO1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_others.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS12.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_OTHERS13.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_others.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_zebranky.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_ZEBRANKY7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_zebranky.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_stinger.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_STINGER0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_STINGER1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_STINGER2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_STINGER3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_STINGER4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_STINGER5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_stinger.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_guy_in_back.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_GUY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_GUY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_guy_in_back.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_past.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ABOUT_PAST11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_past.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_others.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_others.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_zebranky.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_zebranky.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_stinger.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_stinger.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_guy_in_back.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_guy_in_back.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_past.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_past.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if InfoLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::want_specific_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn ZoqFotHome(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_homeworld.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"ZOQFOT_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_10.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_11.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_12.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_13.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_20.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_21.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_22.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_23.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_24.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_25.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_26.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_27.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_30.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_31.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_32.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_33.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_34.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_35.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_40.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_41.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_42.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_43.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_44.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_45.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_46.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_47.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_48.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_49.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_410.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_411.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_homeworld.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::any_war_news.0 as ::core::ffi::c_int
    {
        let mut KnowMask: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        KnowMask = uqm_lua_checked!(getGameStateUint(
            b"ZOQFOT_KNOW_MASK\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if KnowMask as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
            && uqm_lua_checked!(getGameStateUint(
                b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY4.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY5.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY6.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY7.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY8.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY9.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY10.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::KOHRAH_FRENZY11.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            KnowMask = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        } else if KnowMask as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
            && NumVisits as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (NumVisits as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY4.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY5.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY6.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY7.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY8.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY9.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY10.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY11.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY12.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::UTWIG_DELAY13.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            KnowMask = (KnowMask as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        } else {
            let mut i: SIZE = 0;
            i = (2155 as ::core::ffi::c_int
                + (if optDeCleansing.0 != 0 {
                    100 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                }
                    + if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        0 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    })) as SIZE;
            if NumVisits != 0 {
                i += 1;
            }
            i = (i as ::core::ffi::c_int
                - GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int)
                as SIZE;
            if i as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                    > 2 as ::core::ffi::c_int
            {
                i = 0 as SIZE;
            }
            if KnowMask as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
                && i as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::URQUAN_NEARLY_GONE0.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::URQUAN_NEARLY_GONE1.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::URQUAN_NEARLY_GONE2.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::URQUAN_NEARLY_GONE3.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::URQUAN_NEARLY_GONE4.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::URQUAN_NEARLY_GONE5.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                KnowMask = (KnowMask as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
            } else if KnowMask as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
                && i as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING0.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING1.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING2.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING3.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING4.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING5.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING6.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING7.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING8.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::KOHRAH_WINNING9.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                KnowMask = (KnowMask as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::NO_WAR_NEWS0.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::NO_WAR_NEWS1.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
        }
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_KNOW_MASK\0".as_ptr() as *const ::core::ffi::c_char,
            KnowMask as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::any_war_news.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::i_want_alliance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOOD9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        SetRaceAllied(RACE_ID::ZOQFOTPIK_SHIP, BOOLEAN::TRUE);
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            3 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_3::ZOQFOT_DISTRESS_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OK_ENOUGH_INFO.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_homeworld.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::any_war_news.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::any_war_news.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if CheckAlliance(RACE_ID::ZOQFOTPIK_SHIP) as ::core::ffi::c_int
        != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::i_want_alliance.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::want_specific_info.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::want_specific_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    uqm_lua_checked!(SetHomeworldKnown(C2Rust_Unnamed_2::ZOQFOTPIK_HOME.0 as ::core::ffi::c_int as DWORD), ());
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES12.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OUT_TAKES13.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        return;
    }
    if uqm_lua_checked!(getGameStateUint(b"ZOQFOT_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_10.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_11.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_20.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_21.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_22.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_23.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_24.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_25.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_30.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_31.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_40.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELLO_41.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                ), ());
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if uqm_lua_checked!(getGameStateUint(b"MET_ZOQFOT\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        uqm_lua_checked!(setGameStateUint(
            b"MET_ZOQFOT\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::WE_ARE7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        ), ());
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::INIT_HOME_HELLO0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::INIT_HOME_HELLO1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::INIT_HOME_HELLO2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::INIT_HOME_HELLO3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::SCOUT_HELLO0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::SCOUT_HELLO1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::SCOUT_HELLO2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::SCOUT_HELLO3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
        }
        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(AquaintZoqFot(0 as RESPONSE_REF), ());
    } else {
        if uqm_lua_checked!(getGameStateUint(b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE4.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE5.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE6.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE7.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE8.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE9.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE10.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::THANKS_FOR_RESCUE11.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            ), ());
            uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            uqm_lua_checked!(setGameStateUint(
                b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
            uqm_lua_checked!(AddEscortShips(
                RACE_ID::ZOQFOTPIK_SHIP,
                (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as SIZE,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            if CheckAlliance(RACE_ID::ZOQFOTPIK_SHIP) as ::core::ffi::c_int
                != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
            {
                let c2rust_fresh1 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh1 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_10.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_11.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_12.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_13.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_20.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_21.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_22.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_HOME_HELLO_23.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            } else {
                let c2rust_fresh2 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh2 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_10.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_11.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_12.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_13.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_20.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_21.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_22.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_23.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_24.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_25.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_26.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_27.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                    }
                    2 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_30.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_31.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                    }
                    3 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_40.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::ALLIED_HOME_HELLO_41.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        ), ());
                        uqm_lua_checked!(ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
            }
            uqm_lua_checked!(setGameStateUint(
                b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        uqm_lua_checked!(ZoqFotHome(0 as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_zoqfot() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_zoqfot_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_zoqfot_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    zoqfot_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    zoqfot_desc.post_encounter_func = Some(
        post_zoqfot_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    zoqfot_desc.uninit_encounter_func = Some(
        uninit_zoqfot as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    zoqfot_desc.AlienTextWidth = ((CanvasWidth
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
        >> 1 as ::core::ffi::c_int) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COUNT;
    if CheckAlliance(RACE_ID::ZOQFOTPIK_SHIP) as ::core::ffi::c_int
        == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut zoqfot_desc;
    return retval;
}
