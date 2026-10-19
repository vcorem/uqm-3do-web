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

// Rust translation of src/uqm/comm/pkunk/pkunkc.c.
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
    fn getSegue() -> Segue;
    fn AlienTalkSegue(wait_track: COUNT);
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn ShipsReady(race: RACE_ID) -> BOOLEAN;
    fn PrepareShip(race: RACE_ID);
    fn EscortFeasibilityStudy(race: RACE_ID) -> COUNT;
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const GIVE_SPINDLE: Self = Self(1);
    pub const name_1: Self = Self(2);
    pub const name_2: Self = Self(3);
    pub const name_3: Self = Self(4);
    pub const name_40: Self = Self(5);
    pub const name_41: Self = Self(6);
    pub const NEUTRAL_SPACE_HELLO_1: Self = Self(7);
    pub const NEUTRAL_SPACE_HELLO_3: Self = Self(8);
    pub const NEUTRAL_SPACE_HELLO_2: Self = Self(9);
    pub const NEUTRAL_SPACE_HELLO_4: Self = Self(10);
    pub const FRIENDLY_SPACE_HELLO_1: Self = Self(11);
    pub const FRIENDLY_SPACE_HELLO_2: Self = Self(12);
    pub const FRIENDLY_SPACE_HELLO_3: Self = Self(13);
    pub const FRIENDLY_SPACE_HELLO_4: Self = Self(14);
    pub const FRIENDLY_SPACE_HELLO_5: Self = Self(15);
    pub const FRIENDLY_SPACE_HELLO_6: Self = Self(16);
    pub const FRIENDLY_SPACE_HELLO_7: Self = Self(17);
    pub const FRIENDLY_SPACE_HELLO_8: Self = Self(18);
    pub const NEUTRAL_HOMEWORLD_HELLO_1: Self = Self(19);
    pub const NEUTRAL_HOMEWORLD_HELLO_2: Self = Self(20);
    pub const NEUTRAL_HOMEWORLD_HELLO_3: Self = Self(21);
    pub const NEUTRAL_HOMEWORLD_HELLO_4: Self = Self(22);
    pub const FRIENDLY_HOMEWORLD_HELLO_1: Self = Self(23);
    pub const FRIENDLY_HOMEWORLD_HELLO_2: Self = Self(24);
    pub const FRIENDLY_HOMEWORLD_HELLO_3: Self = Self(25);
    pub const FRIENDLY_HOMEWORLD_HELLO_4: Self = Self(26);
    pub const FRIENDLY_HOMEWORLD_HELLO_5: Self = Self(27);
    pub const FRIENDLY_HOMEWORLD_HELLO_6: Self = Self(28);
    pub const FRIENDLY_HOMEWORLD_HELLO_7: Self = Self(29);
    pub const FRIENDLY_HOMEWORLD_HELLO_8: Self = Self(30);
    pub const whats_up_neutral: Self = Self(31);
    pub const GENERAL_INFO_NEUTRAL_1: Self = Self(32);
    pub const GENERAL_INFO_NEUTRAL_2: Self = Self(33);
    pub const GENERAL_INFO_NEUTRAL_3: Self = Self(34);
    pub const GENERAL_INFO_NEUTRAL_4: Self = Self(35);
    pub const good_reason_1: Self = Self(36);
    pub const WE_GO_HOME_1: Self = Self(37);
    pub const good_reason_2: Self = Self(38);
    pub const WE_GO_HOME_2: Self = Self(39);
    pub const bad_reason_1: Self = Self(40);
    pub const NO_GO_HOME_1: Self = Self(41);
    pub const bad_reason_2: Self = Self(42);
    pub const NO_GO_HOME_2: Self = Self(43);
    pub const SENSE_KOHRAH_VICTORY: Self = Self(44);
    pub const SPIRITUAL_PROBLEMS_1: Self = Self(45);
    pub const SPIRITUAL_PROBLEMS_2: Self = Self(46);
    pub const SPIRITUAL_PROBLEMS_3: Self = Self(47);
    pub const SPIRITUAL_PROBLEMS_4: Self = Self(48);
    pub const HATE_YOU_FOREVER_1: Self = Self(49);
    pub const HATE_YOU_FOREVER_2: Self = Self(50);
    pub const HATE_YOU_FOREVER_3: Self = Self(51);
    pub const HATE_YOU_FOREVER_4: Self = Self(52);
    pub const MIGRATING_SPACE_1: Self = Self(53);
    pub const MIGRATING_SPACE_2: Self = Self(54);
    pub const MIGRATING_SPACE_3: Self = Self(55);
    pub const MIGRATING_SPACE_4: Self = Self(56);
    pub const MIGRATING_SPACE_5: Self = Self(57);
    pub const MIGRATING_SPACE_6: Self = Self(58);
    pub const MIGRATING_SPACE_7: Self = Self(59);
    pub const MIGRATING_SPACE_8: Self = Self(60);
    pub const die_idiot_fools: Self = Self(61);
    pub const VERY_WELL: Self = Self(62);
    pub const why_insults: Self = Self(63);
    pub const RELEASE_TENSION: Self = Self(64);
    pub const what_about_you_angry: Self = Self(65);
    pub const ABOUT_US_ANGRY: Self = Self(66);
    pub const what_about_you: Self = Self(67);
    pub const should_be_friends: Self = Self(68);
    pub const YES_FRIENDS: Self = Self(69);
    pub const try_to_be_nicer: Self = Self(70);
    pub const CANT_ASK_FOR_MORE: Self = Self(71);
    pub const VISIT_OUR_HOMEWORLD: Self = Self(72);
    pub const CAN_BE_FRIENDS: Self = Self(73);
    pub const bye_angry: Self = Self(74);
    pub const GOODBYE_ANGRY: Self = Self(75);
    pub const we_conquer: Self = Self(76);
    pub const WHY_CONQUER: Self = Self(77);
    pub const conquer_because_1: Self = Self(78);
    pub const NOT_CONQUER_1: Self = Self(79);
    pub const conquer_because_2: Self = Self(80);
    pub const NOT_CONQUER_2: Self = Self(81);
    pub const must_conquer: Self = Self(82);
    pub const BAD_IDEA: Self = Self(83);
    pub const no_conquest: Self = Self(84);
    pub const GOOD_IDEA: Self = Self(85);
    pub const we_are_vindicator: Self = Self(86);
    pub const WHY_YOU_HERE: Self = Self(87);
    pub const we_here_to_help: Self = Self(88);
    pub const NEED_HELP: Self = Self(89);
    pub const we_need_help: Self = Self(90);
    pub const GIVE_HELP: Self = Self(91);
    pub const exploring_universe: Self = Self(92);
    pub const SENSE_DEEPER_CONFLICT: Self = Self(93);
    pub const fun_cruise: Self = Self(94);
    pub const REPRESS: Self = Self(95);
    pub const why_ilwrath_fight: Self = Self(96);
    pub const ILWRATH_FIGHT_BECAUSE: Self = Self(97);
    pub const when_fight_start: Self = Self(98);
    pub const FIGHT_START_WHEN: Self = Self(99);
    pub const how_goes_fight: Self = Self(100);
    pub const FIGHT_GOES: Self = Self(101);
    pub const how_goes_war: Self = Self(102);
    pub const WAR_GOES_1: Self = Self(103);
    pub const WAR_GOES_2: Self = Self(104);
    pub const WAR_GOES_3: Self = Self(105);
    pub const WAR_GOES_4: Self = Self(106);
    pub const how_stop_fight: Self = Self(107);
    pub const STOP_FIGHT_LIKE_SO: Self = Self(108);
    pub const enough_ilwrath: Self = Self(109);
    pub const OK_ENOUGH_ILWRATH: Self = Self(110);
    pub const what_about_history: Self = Self(111);
    pub const ABOUT_HISTORY: Self = Self(112);
    pub const what_about_yehat: Self = Self(113);
    pub const ABOUT_YEHAT: Self = Self(114);
    pub const what_about_culture: Self = Self(115);
    pub const ABOUT_CULTURE: Self = Self(116);
    pub const elaborate_culture: Self = Self(117);
    pub const OK_ELABORATE_CULTURE: Self = Self(118);
    pub const what_about_future: Self = Self(119);
    pub const ABOUT_FUTURE: Self = Self(120);
    pub const enough_about_you: Self = Self(121);
    pub const OK_ENOUGH_ABOUT_US: Self = Self(122);
    pub const ABOUT_US: Self = Self(123);
    pub const where_fleet_1: Self = Self(124);
    pub const where_fleet_2: Self = Self(125);
    pub const where_fleet_3: Self = Self(126);
    pub const MIGRATING_HOMEWORLD_1: Self = Self(127);
    pub const MIGRATING_HOMEWORLD_2: Self = Self(128);
    pub const MIGRATING_HOMEWORLD_3: Self = Self(129);
    pub const RETURNING_FROM_YEHAT_1: Self = Self(130);
    pub const RETURNING_FROM_YEHAT_2: Self = Self(131);
    pub const am_worried_1: Self = Self(132);
    pub const am_worried_2: Self = Self(133);
    pub const am_worried_3: Self = Self(134);
    pub const DONT_WORRY_1: Self = Self(135);
    pub const DONT_WORRY_2: Self = Self(136);
    pub const DONT_WORRY_3: Self = Self(137);
    pub const form_alliance: Self = Self(138);
    pub const GO_TO_HOMEWORLD: Self = Self(139);
    pub const can_you_help: Self = Self(140);
    pub const GO_TO_HOMEWORLD_AGAIN: Self = Self(141);
    pub const hostile_greeting: Self = Self(142);
    pub const DONT_BE_HOSTILE: Self = Self(143);
    pub const obey: Self = Self(144);
    pub const NO_OBEY: Self = Self(145);
    pub const neutral_bye_space: Self = Self(146);
    pub const NEUTRAL_GOODBYE_SPACE: Self = Self(147);
    pub const SHIP_GIFT: Self = Self(148);
    pub const NO_ROOM: Self = Self(149);
    pub const friendly_bye_space: Self = Self(150);
    pub const FRIENDLY_GOODBYE_SPACE: Self = Self(151);
    pub const bye_friendly: Self = Self(152);
    pub const GOODBYE_FRIENDLY: Self = Self(153);
    pub const ALMOST_ALLIANCE: Self = Self(154);
    pub const INIT_NO_ROOM: Self = Self(155);
    pub const INIT_SHIP_GIFT: Self = Self(156);
    pub const suit_yourself: Self = Self(157);
    pub const GOODBYE_MIGRATION: Self = Self(158);
    pub const what_about_ilwrath: Self = Self(159);
    pub const ABOUT_ILWRATH: Self = Self(160);
    pub const whats_up_space: Self = Self(161);
    pub const SHIPS_AT_HOME: Self = Self(162);
    pub const GENERAL_INFO_SPACE_1: Self = Self(163);
    pub const GENERAL_INFO_SPACE_2: Self = Self(164);
    pub const GENERAL_INFO_SPACE_3: Self = Self(165);
    pub const GENERAL_INFO_SPACE_4: Self = Self(166);
    pub const GENERAL_INFO_SPACE_5: Self = Self(167);
    pub const GENERAL_INFO_SPACE_6: Self = Self(168);
    pub const GENERAL_INFO_SPACE_7: Self = Self(169);
    pub const GENERAL_INFO_SPACE_8: Self = Self(170);
    pub const tell_my_fortune: Self = Self(171);
    pub const FORTUNE_IS_1: Self = Self(172);
    pub const FORTUNE_IS_2: Self = Self(173);
    pub const FORTUNE_IS_3: Self = Self(174);
    pub const FORTUNE_IS_4: Self = Self(175);
    pub const FORTUNE_IS_5: Self = Self(176);
    pub const FORTUNE_IS_6: Self = Self(177);
    pub const FORTUNE_IS_7: Self = Self(178);
    pub const FORTUNE_IS_8: Self = Self(179);
    pub const OUT_TAKES: Self = Self(180);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
static mut pkunk_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::PKUNK_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.pkunk.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.pkunk.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.pkunk.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.pkunk.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.pkunk.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 3 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 3 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 7 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 11 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as DWORD,
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
        NumFrames: 2 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: (840 as ::core::ffi::c_int / 6 as ::core::ffi::c_int) as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int) as COUNT,
        RandomRestartRate: (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
            as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienNumberSpeech: ::core::ptr::null::<NUMBER_SPEECH_DESC>(),
    AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
    AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
    ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
};
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::friendly_bye_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::FRIENDLY_GOODBYE_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::neutral_bye_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NEUTRAL_GOODBYE_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_angry.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_ANGRY.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_friendly.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_FRIENDLY.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_here_to_help.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::we_need_help.0 as ::core::ffi::c_int
    {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::we_here_to_help.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NEED_HELP.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GIVE_HELP.0 as ::core::ffi::c_int, None), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ALMOST_ALLIANCE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            6 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_2::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
        ), ());
        if EscortFeasibilityStudy(RACE_ID::PKUNK_SHIP) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::INIT_NO_ROOM.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::INIT_SHIP_GIFT.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            if uqm_lua_checked!(AddEscortShips(
                RACE_ID::PKUNK_SHIP,
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
            ), ()) != 0
            {
                uqm_lua_checked!(PrepareShip(RACE_ID::PKUNK_SHIP), ());
            }
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::try_to_be_nicer.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::CANT_ASK_FOR_MORE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::VISIT_OUR_HOMEWORLD.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::must_conquer.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int == C2Rust_Unnamed_0::obey.0 as ::core::ffi::c_int
    {
        if R as ::core::ffi::c_int == C2Rust_Unnamed_0::obey.0 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_OBEY.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::BAD_IDEA.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
                2 as DWORD,
            ), ());
        }
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::die_idiot_fools.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::VERY_WELL.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::suit_yourself.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_MIGRATION.0 as ::core::ffi::c_int, None), ());
    } else {
        let mut ReasonMask: BYTE = 0;
        ReasonMask = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::good_reason_1.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WE_GO_HOME_1.0 as ::core::ffi::c_int, None), ());
            ReasonMask = (ReasonMask as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
            uqm_lua_checked!(AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                0 as COUNT,
                0 as COUNT,
                C2Rust_Unnamed_2::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::good_reason_2.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WE_GO_HOME_2.0 as ::core::ffi::c_int, None), ());
            ReasonMask = (ReasonMask as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
            uqm_lua_checked!(AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                0 as COUNT,
                0 as COUNT,
                C2Rust_Unnamed_2::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::bad_reason_1.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_GO_HOME_1.0 as ::core::ffi::c_int, None), ());
            ReasonMask = (ReasonMask as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::bad_reason_2.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_GO_HOME_2.0 as ::core::ffi::c_int, None), ());
            ReasonMask = (ReasonMask as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
            ReasonMask as DWORD,
        ), ());
    };
}
unsafe extern "C" fn PkunkAngry(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::why_insults.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::RELEASE_TENSION.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::why_insults.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_US.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::should_be_friends.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::YES_FRIENDS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::should_be_friends.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::should_be_friends.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::should_be_friends.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkAngry as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::try_to_be_nicer.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::try_to_be_nicer.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::die_idiot_fools.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::why_insults.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::why_insults.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkAngry as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkAngry as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_angry.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn DiscussConquer(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::we_conquer.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WHY_CONQUER.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::we_conquer.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::conquer_because_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NOT_CONQUER_1.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::conquer_because_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::conquer_because_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NOT_CONQUER_2.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::conquer_because_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::conquer_because_1.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::conquer_because_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DiscussConquer as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::conquer_because_2.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::conquer_because_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DiscussConquer as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::must_conquer.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::no_conquest.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn OfferAlliance(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::we_are_vindicator.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WHY_YOU_HERE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::exploring_universe.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::SENSE_DEEPER_CONFLICT.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::exploring_universe.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::fun_cruise.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::REPRESS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::fun_cruise.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::we_here_to_help.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::we_need_help.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::exploring_universe.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::exploring_universe.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(OfferAlliance as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::fun_cruise.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::fun_cruise.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(OfferAlliance as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
}
unsafe extern "C" fn AboutPkunk(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_US.0 as ::core::ffi::c_int, None), ());
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
        == C2Rust_Unnamed_0::what_about_yehat.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_YEHAT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_yehat.0 as ::core::ffi::c_int
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
        == C2Rust_Unnamed_0::elaborate_culture.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::OK_ELABORATE_CULTURE.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::elaborate_culture.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_future.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_FUTURE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_future.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_history.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_history.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutPkunk as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_yehat.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_yehat.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutPkunk as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_culture.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_culture.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutPkunk as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::elaborate_culture.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::elaborate_culture.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutPkunk as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_future.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_future.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutPkunk as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_about_you.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if InfoLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn AboutIlwrath(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_ILWRATH.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::why_ilwrath_fight.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::ILWRATH_FIGHT_BECAUSE.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::why_ilwrath_fight.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::when_fight_start.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::FIGHT_START_WHEN.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::when_fight_start.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_goes_fight.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::FIGHT_GOES.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_fight.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_stop_fight.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::STOP_FIGHT_LIKE_SO.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_stop_fight.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::why_ilwrath_fight.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::why_ilwrath_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutIlwrath as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::when_fight_start.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::when_fight_start.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutIlwrath as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::how_goes_fight.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::how_goes_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutIlwrath as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::how_stop_fight.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::how_stop_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutIlwrath as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::enough_ilwrath.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if InfoLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn PkunkHome(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::no_conquest.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOOD_IDEA.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_ilwrath.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_ENOUGH_ILWRATH.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::enough_about_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_ENOUGH_ABOUT_US.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::where_fleet_1.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::where_fleet_2.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::where_fleet_3.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_SWITCH\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        if uqm_lua_checked!(getGameStateUint(b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char), ())
            & 1 as DWORD == 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_RETURN\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh11 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh11 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::RETURNING_FROM_YEHAT_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::RETURNING_FROM_YEHAT_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_RETURN\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_MIGRATE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh12 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh12 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_HOMEWORLD_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_HOMEWORLD_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_HOMEWORLD_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_MIGRATE\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        NumVisits = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_FLEET\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .wrapping_add(1 as DWORD) as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_FLEET\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::where_fleet_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::am_worried_1.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::am_worried_2.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::am_worried_3.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_WORRY\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh13 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh13 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::DONT_WORRY_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::DONT_WORRY_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::DONT_WORRY_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_WORRY\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::am_worried_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::try_to_be_nicer.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::CANT_ASK_FOR_MORE.0 as ::core::ffi::c_int, None), ());
        if uqm_lua_checked!(getGameStateUint(b"CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GIVE_SPINDLE.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::CAN_BE_FRIENDS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(SetHomeworldKnown(C2Rust_Unnamed_1::PKUNK_HOME.0 as ::core::ffi::c_int as DWORD), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_ILWRATH.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::we_conquer.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_conquer.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DiscussConquer as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"PKUNK_ON_THE_MOVE\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::where_fleet_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(b"PKUNK_SWITCH\0".as_ptr() as *const ::core::ffi::c_char), ())
                == 0
        {
            match uqm_lua_checked!(getGameStateUint(
                b"PKUNK_FLEET\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                0 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::where_fleet_1.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::where_fleet_2.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::where_fleet_3.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                _ => {}
            }
        } else if !(*GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::where_fleet_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            && *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_0::am_worried_1.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) & 1 as DWORD != 0
        {
            match uqm_lua_checked!(getGameStateUint(
                b"PKUNK_WORRY\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                0 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::am_worried_1.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::am_worried_2.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_0::am_worried_3.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
                _ => {}
            }
        }
    }
    if uqm_lua_checked!(getGameStateUint(b"PKUNK_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::we_are_vindicator.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(OfferAlliance as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_about_you.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AboutPkunk as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        if uqm_lua_checked!(getGameStateUint(b"ILWRATH_DECEIVED\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AboutIlwrath as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_ilwrath.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(PkunkHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_friendly.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn PkunkFriendlySpace(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_space.0 as ::core::ffi::c_int
    {
        if uqm_lua_checked!(ShipsReady(RACE_ID::PKUNK_SHIP), ()).0 != 0
            && uqm_lua_checked!(getGameStateUint(b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
                == 3 as DWORD
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SHIPS_AT_HOME.0 as ::core::ffi::c_int, None), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh8 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh8 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                4 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_5.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                5 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_6.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                6 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_7.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                7 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GENERAL_INFO_SPACE_8.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh9 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh9 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WAR_GOES_1.0 as ::core::ffi::c_int, None), ());
                uqm_lua_checked!(setGameStateUint(
                    b"KNOW_URQUAN_STORY\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), ());
                uqm_lua_checked!(setGameStateUint(
                    b"KNOW_KOHR_AH_STORY\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WAR_GOES_2.0 as ::core::ffi::c_int, None), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WAR_GOES_3.0 as ::core::ffi::c_int, None), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WAR_GOES_4.0 as ::core::ffi::c_int, None), ());
                uqm_lua_checked!(setGameStateUint(
                    b"PKUNK_DONE_WAR\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_WAR\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_my_fortune.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_FORTUNE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh10 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh10 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::FORTUNE_IS_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_FORTUNE\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::tell_my_fortune.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_space.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkFriendlySpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"PKUNK_DONE_WAR\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0
        && *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkFriendlySpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::tell_my_fortune.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::tell_my_fortune.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkFriendlySpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::friendly_bye_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn PkunkNeutralSpace(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::form_alliance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GO_TO_HOMEWORLD.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(SetHomeworldKnown(C2Rust_Unnamed_1::PKUNK_HOME.0 as ::core::ffi::c_int as DWORD), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::form_alliance.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::can_you_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GO_TO_HOMEWORLD_AGAIN.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(SetHomeworldKnown(C2Rust_Unnamed_1::PKUNK_HOME.0 as ::core::ffi::c_int as DWORD), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::can_you_help.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::hostile_greeting.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::DONT_BE_HOSTILE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::hostile_greeting.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_neutral.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh7 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh7 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_SPACE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_SPACE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_SPACE_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_SPACE_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_neutral.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::form_alliance.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::form_alliance.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkNeutralSpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::can_you_help.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::can_you_help.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkNeutralSpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::hostile_greeting.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::hostile_greeting.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkNeutralSpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::obey.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_neutral.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_neutral.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(PkunkNeutralSpace as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::neutral_bye_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn PkunkMigrate(mut R: RESPONSE_REF) {
    let mut ReasonMask: BYTE = 0;
    ReasonMask = uqm_lua_checked!(getGameStateUint(
        b"PKUNK_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    if ReasonMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::good_reason_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if ReasonMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::bad_reason_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if ReasonMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::good_reason_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if ReasonMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::bad_reason_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::suit_yourself.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    let mut Manner: BYTE = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OUT_TAKES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        return;
    }
    Manner = uqm_lua_checked!(getGameStateUint(b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
        as BYTE;
    if Manner as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HATE_YOU_FOREVER_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HATE_YOU_FOREVER_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HATE_YOU_FOREVER_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HATE_YOU_FOREVER_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if Manner as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh1 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh1 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::SPIRITUAL_PROBLEMS_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::SPIRITUAL_PROBLEMS_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::SPIRITUAL_PROBLEMS_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::SPIRITUAL_PROBLEMS_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(PkunkAngry(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
    {
        if uqm_lua_checked!(getGameStateUint(b"CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GIVE_SPINDLE.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"PKUNK_SENSE_VICTOR\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
            && GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                > 2155 as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::SENSE_KOHRAH_VICTORY.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_SENSE_VICTOR\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if Manner as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let c2rust_fresh2 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh2 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_HOMEWORLD_HELLO_1.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_HOMEWORLD_HELLO_2.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_HOMEWORLD_HELLO_3.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_HOMEWORLD_HELLO_4.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
        } else if NumVisits as ::core::ffi::c_int != 0
            && uqm_lua_checked!(ShipsReady(RACE_ID::PKUNK_SHIP), ()).0 != 0
            && uqm_lua_checked!(getGameStateUint(b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
                == 3 as DWORD
        {
            if EscortFeasibilityStudy(RACE_ID::PKUNK_SHIP) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_ROOM.0 as ::core::ffi::c_int, None), ());
            } else {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SHIP_GIFT.0 as ::core::ffi::c_int, None), ());
                if uqm_lua_checked!(AddEscortShips(
                    RACE_ID::PKUNK_SHIP,
                    (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    }) as SIZE,
                ), ()) != 0
                {
                    uqm_lua_checked!(PrepareShip(RACE_ID::PKUNK_SHIP), ());
                }
            }
        } else {
            let c2rust_fresh3 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh3 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_1.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_2.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_3.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_4.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                4 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_5.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                5 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_6.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                6 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_7.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                7 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_HOMEWORLD_HELLO_8.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(PkunkHome(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if NumVisits as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || NumVisits as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            if Manner as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                let c2rust_fresh4 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh4 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    2 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_3.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    3 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_4.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    4 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_5.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    5 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_6.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    6 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_7.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    7 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::FRIENDLY_SPACE_HELLO_8.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(PkunkFriendlySpace(0 as ::core::ffi::c_int as RESPONSE_REF), ());
            } else {
                let c2rust_fresh5 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh5 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_SPACE_HELLO_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_SPACE_HELLO_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    2 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_SPACE_HELLO_3.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    3 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::NEUTRAL_SPACE_HELLO_4.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        uqm_lua_checked!(SetHomeworldKnown(
                            C2Rust_Unnamed_1::PKUNK_HOME.0 as ::core::ffi::c_int as DWORD,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(PkunkNeutralSpace(0 as ::core::ffi::c_int as RESPONSE_REF), ());
            }
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_MIGRATE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh6 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh6 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                4 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_5.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                5 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_6.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                6 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_7.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                7 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::MIGRATING_SPACE_8.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_MIGRATE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(PkunkMigrate(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        }
    };
}
unsafe extern "C" fn uninit_pkunk() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_pkunk_enc() {
    let mut Manner: BYTE = 0;
    if uqm_lua_checked!(getSegue(), ()).0 == Segue::Segue_hostile.0
        && {
            Manner = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            Manner as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        }
    {
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        if Manner as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_pkunk_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    pkunk_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    pkunk_desc.post_encounter_func = Some(post_pkunk_enc as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    pkunk_desc.uninit_encounter_func = Some(
        uninit_pkunk as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    pkunk_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    pkunk_desc.AlienTextBaseline.y = 0 as COORD;
    pkunk_desc.AlienTextWidth = (CanvasWidth
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
    if uqm_lua_checked!(getGameStateUint(b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        == 3 as DWORD
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut pkunk_desc;
    return retval;
}
