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

// Rust translation of src/uqm/comm/spathi/spathic.c.
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
    static mut altResFlags: BYTE;
    fn SetEscortCrewComplement(
        which_ship: RACE_ID,
        crew_level: COUNT,
        captain: BYTE,
    ) -> ::core::ffi::c_int;
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn CountEscortShips(race: RACE_ID) -> COUNT;
    fn EscortFeasibilityStudy(race: RACE_ID) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_NOM_OFF: Self = Self(0);
    pub const OPTVAL_NOM_EASY: Self = Self(1);
    pub const OPTVAL_NOM_NORM: Self = Self(2);
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
    pub const SORRY_ABOUT_THAT: Self = Self(1);
    pub const identify: Self = Self(2);
    pub const I_FWIFFO: Self = Self(3);
    pub const hi_there: Self = Self(4);
    pub const ARE_YOU_SURE: Self = Self(5);
    pub const dont_kill: Self = Self(6);
    pub const we_fight_1: Self = Self(7);
    pub const we_fight_2: Self = Self(8);
    pub const OK_WONT: Self = Self(9);
    pub const do_cultural: Self = Self(10);
    pub const WEZZY_WEZZAH: Self = Self(11);
    pub const die_slugboy: Self = Self(12);
    pub const begin_ritual: Self = Self(13);
    pub const MUST_DO_RITUAL_AT_HOME: Self = Self(14);
    pub const you_wont_die_yet: Self = Self(15);
    pub const ETERNAL_GRATITUDE: Self = Self(16);
    pub const we_fight: Self = Self(17);
    pub const pay_for_crimes: Self = Self(18);
    pub const CLUTCH_MAVEN: Self = Self(19);
    pub const you_may_live: Self = Self(20);
    pub const HONEST_AND_FRIENDLY: Self = Self(21);
    pub const what_are_coordinates: Self = Self(22);
    pub const COORDINATES_ARE: Self = Self(23);
    pub const tell_me_coordinates: Self = Self(24);
    pub const FAKE_COORDINATES: Self = Self(25);
    pub const TOO_SCARY: Self = Self(26);
    pub const youve_got_me_all_wrong: Self = Self(27);
    pub const SORRY_NO_COORDS: Self = Self(28);
    pub const what_doing_on_pluto_1: Self = Self(29);
    pub const ABOUT_20_YEARS_AGO: Self = Self(30);
    pub const what_doing_on_pluto_2: Self = Self(31);
    pub const WHEN_URQUAN_ARRIVED: Self = Self(32);
    pub const where_are_urquan: Self = Self(33);
    pub const URQUAN_LEFT: Self = Self(34);
    pub const what_about_other_races: Self = Self(35);
    pub const ABOUT_OTHER_RACES: Self = Self(36);
    pub const what_doing_on_pluto_3: Self = Self(37);
    pub const what_about_yourself: Self = Self(38);
    pub const ABOUT_MYSELF: Self = Self(39);
    pub const STATIONED_ON_EARTH_MOON: Self = Self(40);
    pub const what_blaze_of_glory: Self = Self(41);
    pub const BLAZE_IS: Self = Self(42);
    pub const what_about_moonbase: Self = Self(43);
    pub const SET_UP_BASE: Self = Self(44);
    pub const what_about_ilwrath: Self = Self(45);
    pub const ABOUT_ILWRATH: Self = Self(46);
    pub const what_about_other_spathi: Self = Self(47);
    pub const really_thousands: Self = Self(48);
    pub const SPATHI_ARE: Self = Self(49);
    pub const what_enemy: Self = Self(50);
    pub const ENEMY_IS: Self = Self(51);
    pub const when_ilwrath: Self = Self(52);
    pub const THEN_ILWRATH: Self = Self(53);
    pub const why_you_here: Self = Self(54);
    pub const DREW_SHORT_STRAW: Self = Self(55);
    pub const how_many_crew: Self = Self(56);
    pub const JUST_ME: Self = Self(57);
    pub const THOUSANDS: Self = Self(58);
    pub const full_of_monsters: Self = Self(59);
    pub const HOW_TRUE: Self = Self(60);
    pub const join_us: Self = Self(61);
    pub const WILL_JOIN: Self = Self(62);
    pub const WONT_JOIN_1: Self = Self(63);
    pub const give_ship_or_die: Self = Self(64);
    pub const WONT_JOIN_2: Self = Self(65);
    pub const WONT_JOIN_3: Self = Self(66);
    pub const GEE_THANKS: Self = Self(67);
    pub const changed_mind: Self = Self(68);
    pub const youre_forgiven: Self = Self(69);
    pub const THANKS_FOR_FORGIVENESS: Self = Self(70);
    pub const HATE_YOU_FOREVER_SPACE: Self = Self(71);
    pub const INIT_ANGRY_HELLO_SPACE: Self = Self(72);
    pub const SUBSEQUENT_ANGRY_HELLO_SPACE: Self = Self(73);
    pub const INIT_NEUTRAL_HELLO_SPACE: Self = Self(74);
    pub const SUBSEQUENT_NEUTRAL_HELLO_SPACE: Self = Self(75);
    pub const INIT_FRIENDLY_HELLO_SPACE: Self = Self(76);
    pub const SUBSEQUENT_FRIENDLY_HELLO_SPACE: Self = Self(77);
    pub const INIT_ALLIED_HELLO_SPACE: Self = Self(78);
    pub const SUBSEQUENT_ALLIED_HELLO_SPACE: Self = Self(79);
    pub const give_info_space: Self = Self(80);
    pub const HERES_SOME_INFO: Self = Self(81);
    pub const we_sorry_space: Self = Self(82);
    pub const APOLOGIZE_AT_HOMEWORLD: Self = Self(83);
    pub const we_fight_again_space: Self = Self(84);
    pub const OK_FIGHT_AGAIN_SPACE: Self = Self(85);
    pub const bye_angry_space: Self = Self(86);
    pub const GOODBYE_ANGRY_SPACE: Self = Self(87);
    pub const look_weird: Self = Self(88);
    pub const YOU_LOOK_WEIRD: Self = Self(89);
    pub const no_look_really_weird: Self = Self(90);
    pub const NO_YOU_LOOK_REALLY_WEIRD: Self = Self(91);
    pub const come_in_peace: Self = Self(92);
    pub const AGAINST_NATURE: Self = Self(93);
    pub const prepare_to_die: Self = Self(94);
    pub const ALWAYS_PREPARED: Self = Self(95);
    pub const since_friendly_give_stuff: Self = Self(96);
    pub const GIVE_ADVICE: Self = Self(97);
    pub const whats_up_space_1: Self = Self(98);
    pub const GENERAL_INFO_SPACE_1: Self = Self(99);
    pub const bye_friendly_space: Self = Self(100);
    pub const GOODBYE_FRIENDLY_SPACE: Self = Self(101);
    pub const looking_for_a_few_good_squids: Self = Self(102);
    pub const URQUAN_SLAVES: Self = Self(103);
    pub const why_slaves: Self = Self(104);
    pub const UMGAH_TRICK: Self = Self(105);
    pub const tell_us_about_you: Self = Self(106);
    pub const ABOUT_US: Self = Self(107);
    pub const what_you_really_want: Self = Self(108);
    pub const WANT_THIS: Self = Self(109);
    pub const how_about_alliance: Self = Self(110);
    pub const SURE: Self = Self(111);
    pub const part_in_peace: Self = Self(112);
    pub const KEEP_IT_SECRET: Self = Self(113);
    pub const heard_youre_cowards: Self = Self(114);
    pub const DARN_TOOTIN: Self = Self(115);
    pub const wanna_fight: Self = Self(116);
    pub const YES_WE_DO: Self = Self(117);
    pub const so_lets_fight: Self = Self(118);
    pub const OK_LETS_FIGHT: Self = Self(119);
    pub const so_lets_fight_already: Self = Self(120);
    pub const DONT_REALLY_WANT_TO_FIGHT: Self = Self(121);
    pub const attack_you_now: Self = Self(122);
    pub const YIPES: Self = Self(123);
    pub const whats_up_space_2: Self = Self(124);
    pub const GENERAL_INFO_SPACE_2: Self = Self(125);
    pub const give_us_info_from_space: Self = Self(126);
    pub const GET_INFO_FROM_SPATHIWA: Self = Self(127);
    pub const give_us_resources_space: Self = Self(128);
    pub const GET_RESOURCES_FROM_SPATHIWA: Self = Self(129);
    pub const what_do_for_fun: Self = Self(130);
    pub const DO_THIS_FOR_FUN: Self = Self(131);
    pub const bye_ally_space: Self = Self(132);
    pub const GOODBYE_ALLY_SPACE: Self = Self(133);
    pub const OK_WE_FIGHT_AT_PLUTO: Self = Self(134);
    pub const join_us_nomad: Self = Self(135);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
static mut spathi_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::SPATHI_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.spathi.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.spathi.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.spathi.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.spathi.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: b"comm.fwiffo.music\0".as_ptr() as RESOURCE,
    },
    ConversationPhrasesRes: b"comm.spathi.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 8 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: 840 as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 7 as COUNT,
            NumFrames: 9 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 16 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 20 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 24 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 34 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 38 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 41 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
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
unsafe extern "C" fn ExitConversation(mut Response: RESPONSE_REF) {
    let mut SpaCrew: BYTE = (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        != 0
    {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 1 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 0
    {
        1 as ::core::ffi::c_int
    } else {
        42 as ::core::ffi::c_int
    }) as BYTE;
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bye_ally_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOODBYE_ALLY_SPACE.0 as ::core::ffi::c_int, None), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bye_friendly_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE_FRIENDLY_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::part_in_peace.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::KEEP_IT_SECRET.0 as ::core::ffi::c_int, None), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_sorry_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::APOLOGIZE_AT_HOMEWORLD.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::give_info_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HERES_SOME_INFO.0 as ::core::ffi::c_int, None), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::bye_angry_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE_ANGRY_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::attack_you_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::YIPES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_fight_again_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::OK_FIGHT_AGAIN_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::die_slugboy.0 as ::core::ffi::c_int
        || Response as ::core::ffi::c_int
            == C2Rust_Unnamed_1::we_fight_1.0 as ::core::ffi::c_int
        || Response as ::core::ffi::c_int
            == C2Rust_Unnamed_1::we_fight_2.0 as ::core::ffi::c_int
        || Response as ::core::ffi::c_int
            == C2Rust_Unnamed_1::pay_for_crimes.0 as ::core::ffi::c_int
        || Response as ::core::ffi::c_int
            == C2Rust_Unnamed_1::tell_me_coordinates.0 as ::core::ffi::c_int
        || Response as ::core::ffi::c_int
            == C2Rust_Unnamed_1::changed_mind.0 as ::core::ffi::c_int
    {
        if Response as ::core::ffi::c_int
            == C2Rust_Unnamed_1::tell_me_coordinates.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::FAKE_COORDINATES.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::OK_WE_FIGHT_AT_PLUTO.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::join_us.0 as ::core::ffi::c_int
    {
        if EscortFeasibilityStudy(RACE_ID::SPATHI_SHIP) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TOO_SCARY.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WILL_JOIN.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            uqm_lua_checked!(AddEscortShips(RACE_ID::SPATHI_SHIP, 1 as SIZE), ());
            SetEscortCrewComplement(
                RACE_ID::SPATHI_SHIP,
                SpaCrew as COUNT,
                (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int) as BYTE,
            );
        }
    } else if Response as ::core::ffi::c_int
        == C2Rust_Unnamed_1::join_us_nomad.0 as ::core::ffi::c_int
    {
        if EscortFeasibilityStudy(RACE_ID::SPATHI_SHIP) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || CountEscortShips(RACE_ID::SPATHI_SHIP) as ::core::ffi::c_int
                > 2 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TOO_SCARY.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WILL_JOIN.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            uqm_lua_checked!(AddEscortShips(RACE_ID::SPATHI_SHIP, 1 as SIZE), ());
        }
    }
}
static mut join_us_refusals: BYTE = 0;
unsafe extern "C" fn SpathiOnPluto(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::hi_there.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ARE_YOU_SURE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::dont_kill.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_WONT.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::youre_forgiven.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::THANKS_FOR_FORGIVENESS.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::you_wont_die_yet.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ETERNAL_GRATITUDE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::you_may_live.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GEE_THANKS.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::youve_got_me_all_wrong.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SORRY_NO_COORDS.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_doing_on_pluto_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_20_YEARS_AGO.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_doing_on_pluto_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_doing_on_pluto_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::WHEN_URQUAN_ARRIVED.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_doing_on_pluto_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_doing_on_pluto_3.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::STATIONED_ON_EARTH_MOON.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_doing_on_pluto_3.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_ilwrath.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_ILWRATH.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_ilwrath.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::when_ilwrath.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THEN_ILWRATH.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::when_ilwrath.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_moonbase.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SET_UP_BASE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_moonbase.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_other_spathi.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SPATHI_ARE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_other_spathi.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_other_spathi.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THEN_ILWRATH.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_other_spathi.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::how_many_crew.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THOUSANDS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::how_many_crew.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::really_thousands.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::JUST_ME.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::really_thousands.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::full_of_monsters.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HOW_TRUE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::full_of_monsters.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_enemy.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ENEMY_IS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_enemy.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::why_you_here.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DREW_SHORT_STRAW.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::why_you_here.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::URQUAN_LEFT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_other_races.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_OTHER_RACES.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_other_races.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_blaze_of_glory.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BLAZE_IS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_blaze_of_glory.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_yourself.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_MYSELF.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_yourself.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::join_us.0 as ::core::ffi::c_int
    {
        if join_us_refusals as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WONT_JOIN_1.0 as ::core::ffi::c_int, None), ());
            join_us_refusals = join_us_refusals.wrapping_add(1);
        } else if join_us_refusals as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WONT_JOIN_2.0 as ::core::ffi::c_int, None), ());
            join_us_refusals = join_us_refusals.wrapping_add(1);
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WONT_JOIN_3.0 as ::core::ffi::c_int, None), ());
        }
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_doing_on_pluto_1.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_doing_on_pluto_1.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_doing_on_pluto_2.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_doing_on_pluto_2.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_doing_on_pluto_3.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_doing_on_pluto_3.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_ilwrath.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::what_about_ilwrath.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::when_ilwrath.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::when_ilwrath.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_moonbase.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::what_about_moonbase.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_other_spathi.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::what_about_other_spathi.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::how_many_crew.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::how_many_crew.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::really_thousands.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::really_thousands.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::full_of_monsters.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::full_of_monsters.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::what_enemy.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::what_enemy.0 as ::core::ffi::c_int as RESPONSE_REF,
                    Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::why_you_here.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::why_you_here.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
        }
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_other_races.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_other_races.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_blaze_of_glory.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_blaze_of_glory.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_yourself.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_yourself.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if !(*GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::full_of_monsters.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::join_us.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::join_us.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::changed_mind.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn SpathiMustGrovel(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::identify.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::I_FWIFFO.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::do_cultural.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiMustGrovel as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::youre_forgiven.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::die_slugboy.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::do_cultural.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WEZZY_WEZZAH.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::begin_ritual.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiMustGrovel as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::you_wont_die_yet.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_fight_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::begin_ritual.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::MUST_DO_RITUAL_AT_HOME.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::you_may_live.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::pay_for_crimes.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_are_coordinates.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiMustGrovel as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::COORDINATES_ARE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::youve_got_me_all_wrong.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::tell_me_coordinates.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn SpathiAllies(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::INIT_ALLIED_HELLO_SPACE.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUBSEQUENT_ALLIED_HELLO_SPACE.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_space_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GENERAL_INFO_SPACE_2.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_space_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::give_us_info_from_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GET_INFO_FROM_SPATHIWA.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::give_us_info_from_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::give_us_resources_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GET_RESOURCES_FROM_SPATHIWA.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::give_us_resources_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_do_for_fun.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DO_THIS_FOR_FUN.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_do_for_fun.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_space_2.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_space_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::give_us_info_from_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::give_us_info_from_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::give_us_resources_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::give_us_resources_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_do_for_fun.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_do_for_fun.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if if GlobData.SIS_state.Nomad as ::core::ffi::c_int
        == C2Rust_Unnamed::OPTVAL_NOM_EASY.0 as ::core::ffi::c_int
    {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::join_us_nomad.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye_ally_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn SpathiFriendly(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh2 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::INIT_FRIENDLY_HELLO_SPACE.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUBSEQUENT_FRIENDLY_HELLO_SPACE.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::since_friendly_give_stuff.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_ADVICE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::since_friendly_give_stuff.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_space_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GENERAL_INFO_SPACE_1.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_space_1.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_space_1.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_space_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiFriendly as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::since_friendly_give_stuff.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::since_friendly_give_stuff.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiFriendly as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye_friendly_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn SpathiBefriend(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    let mut LastStack: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 2] = [0; 2];
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    LastStack = 0 as BYTE;
    pStr[1usize] = 0 as RESPONSE_REF;
    pStr[0usize] = pStr[1usize];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::come_in_peace.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::AGAINST_NATURE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::looking_for_a_few_good_squids.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::URQUAN_SLAVES.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::looking_for_a_few_good_squids.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::why_slaves.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::UMGAH_TRICK.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::why_slaves.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::tell_us_about_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_US.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::tell_us_about_you.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
        LastStack = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_you_really_want.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WANT_THIS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_you_really_want.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::how_about_alliance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SURE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::how_about_alliance.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::looking_for_a_few_good_squids.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        pStr[0usize] = C2Rust_Unnamed_1::looking_for_a_few_good_squids.0
            as ::core::ffi::c_int as RESPONSE_REF;
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::why_slaves.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        pStr[0usize] = C2Rust_Unnamed_1::why_slaves.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::tell_us_about_you.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        pStr[1usize] = C2Rust_Unnamed_1::tell_us_about_you.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_you_really_want.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        pStr[1usize] = C2Rust_Unnamed_1::what_you_really_want.0 as ::core::ffi::c_int
            as RESPONSE_REF;
    }
    if pStr[LastStack as usize] != 0 {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            pStr[LastStack as usize],
            Some(SpathiBefriend as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    LastStack = (LastStack as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
    if pStr[LastStack as usize] != 0 {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            pStr[LastStack as usize],
            Some(SpathiBefriend as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::how_about_alliance.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::how_about_alliance.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiBefriend as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if InfoLeft == 0 {
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(SpathiNeutral(R), ());
    }
}
unsafe extern "C" fn SpathiAntagonize(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::prepare_to_die.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ALWAYS_PREPARED.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::heard_youre_cowards.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DARN_TOOTIN.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::heard_youre_cowards.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::wanna_fight.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::YES_WE_DO.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::wanna_fight.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::so_lets_fight.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_LETS_FIGHT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::so_lets_fight.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::so_lets_fight_already.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::DONT_REALLY_WANT_TO_FIGHT.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::so_lets_fight_already.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::wanna_fight.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::wanna_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiAntagonize as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::so_lets_fight.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::so_lets_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiAntagonize as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::so_lets_fight_already.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::so_lets_fight_already.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiAntagonize as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::heard_youre_cowards.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::heard_youre_cowards.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(SpathiAntagonize as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::attack_you_now.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn SpathiNeutral(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh1 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh1 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::INIT_NEUTRAL_HELLO_SPACE.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUBSEQUENT_NEUTRAL_HELLO_SPACE.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::look_weird.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::YOU_LOOK_WEIRD.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::no_look_really_weird.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::NO_YOU_LOOK_REALLY_WEIRD.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(b"SPATHI_STACK0\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::look_weird.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(SpathiNeutral as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::no_look_really_weird.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(SpathiNeutral as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {}
    }
    if uqm_lua_checked!(getGameStateUint(b"SPATHI_STACK1\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::come_in_peace.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiBefriend as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"SPATHI_STACK2\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::prepare_to_die.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiAntagonize as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::attack_you_now.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::part_in_peace.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    let mut Manner: BYTE = 0;
    Manner = uqm_lua_checked!(getGameStateUint(b"SPATHI_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
        as BYTE;
    if uqm_lua_checked!(getGameStateUint(b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 1 as DWORD
    {
        join_us_refusals = 0 as BYTE;
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SORRY_ABOUT_THAT.0 as ::core::ffi::c_int, None), ());
        if uqm_lua_checked!(getGameStateUint(
            b"SPATHI_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 7 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"KNOW_SPATHI_PASSWORD\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"SPATHI_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        }
        uqm_lua_checked!(SetHomeworldKnown(
            C2Rust_Unnamed_3::SPATHI_HOME.0 as ::core::ffi::c_int as DWORD,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::identify.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiMustGrovel as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::hi_there.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::dont_kill.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SpathiOnPluto as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_fight_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if Manner as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::HATE_YOU_FOREVER_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if Manner as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::INIT_ANGRY_HELLO_SPACE.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SUBSEQUENT_ANGRY_HELLO_SPACE.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::give_info_space.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_sorry_space.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_fight_again_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::bye_angry_space.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if CheckAlliance(RACE_ID::SPATHI_SHIP) as ::core::ffi::c_int
        == C2Rust_Unnamed_2::GOOD_GUY.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(SpathiAllies(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(b"SPATHI_QUEST\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
    {
        uqm_lua_checked!(SpathiFriendly(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else {
        uqm_lua_checked!(SpathiNeutral(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_spathi() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_spathi_enc() {
    let mut Manner: BYTE = 0;
    if uqm_lua_checked!(getGameStateUint(b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 1 as DWORD
    {
        uqm_lua_checked!(setGameStateUint(
            b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if uqm_lua_checked!(getSegue(), ()).0 == Segue::Segue_hostile.0
        && {
            Manner = uqm_lua_checked!(getGameStateUint(
                b"SPATHI_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            Manner as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        }
    {
        uqm_lua_checked!(setGameStateUint(
            b"SPATHI_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        if Manner as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            uqm_lua_checked!(setGameStateUint(
                b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
            if uqm_lua_checked!(getGameStateUint(
                b"SPATHI_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 7 as DWORD
            {
                uqm_lua_checked!(setGameStateUint(
                    b"SPATHI_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), ());
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_spathi_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    spathi_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    spathi_desc.post_encounter_func = Some(
        post_spathi_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    spathi_desc.uninit_encounter_func = Some(
        uninit_spathi as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    spathi_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    spathi_desc.AlienTextBaseline.y = 0 as COORD;
    spathi_desc.AlienTextWidth = (CanvasWidth
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
    if uqm_lua_checked!(getGameStateUint(b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        == 1 as DWORD
    {
        altResFlags = (altResFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if uqm_lua_checked!(getGameStateUint(b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        == 1 as DWORD
        || uqm_lua_checked!(getGameStateUint(b"SPATHI_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
            == 3 as DWORD
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut spathi_desc;
    return retval;
}
