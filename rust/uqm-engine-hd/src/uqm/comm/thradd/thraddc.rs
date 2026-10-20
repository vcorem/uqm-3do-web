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

// Rust translation of src/uqm/comm/thradd/thraddc.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
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
    fn NPCPhrase_splice(index: ::core::ffi::c_int);
    fn setSegue(segue: Segue);
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn SetRaceAllied(race: RACE_ID, flag: BOOLEAN) -> BOOLEAN;
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
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
pub type LINK = link;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
    pub shipIdStr: *const ::core::ffi::c_char,
    pub allied_state: UWORD,
    pub days_left: BYTE,
    pub growth_fract: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub growth: BYTE,
    pub max_energy: BYTE,
    pub loc: POINT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
    pub actual_strength: COUNT,
    pub known_strength: COUNT,
    pub known_loc: POINT,
    pub growth_err_term: BYTE,
    pub func_index: BYTE,
    pub dest_loc: POINT,
    pub can_build: BOOLEAN,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SPECIES_ID(pub ::core::ffi::c_uint);
impl SPECIES_ID {
    pub const NO_ID: Self = Self(0);
    pub const ARILOU_ID: Self = Self(1);
    pub const CHMMR_ID: Self = Self(2);
    pub const EARTHLING_ID: Self = Self(3);
    pub const ORZ_ID: Self = Self(4);
    pub const PKUNK_ID: Self = Self(5);
    pub const SHOFIXTI_ID: Self = Self(6);
    pub const SPATHI_ID: Self = Self(7);
    pub const SUPOX_ID: Self = Self(8);
    pub const THRADDASH_ID: Self = Self(9);
    pub const UTWIG_ID: Self = Self(10);
    pub const VUX_ID: Self = Self(11);
    pub const YEHAT_ID: Self = Self(12);
    pub const MELNORME_ID: Self = Self(13);
    pub const DRUUGE_ID: Self = Self(14);
    pub const ILWRATH_ID: Self = Self(15);
    pub const MYCON_ID: Self = Self(16);
    pub const SLYLANDRO_ID: Self = Self(17);
    pub const UMGAH_ID: Self = Self(18);
    pub const UR_QUAN_ID: Self = Self(19);
    pub const ZOQFOTPIK_ID: Self = Self(20);
    pub const SYREEN_ID: Self = Self(21);
    pub const KOHR_AH_ID: Self = Self(22);
    pub const ANDROSYNTH_ID: Self = Self(23);
    pub const CHENJESU_ID: Self = Self(24);
    pub const MMRNMHRM_ID: Self = Self(25);
    pub const LAST_MELEE_ID: Self = Self(25);
    pub const SIS_SHIP_ID: Self = Self(26);
    pub const SA_MATRA_ID: Self = Self(27);
    pub const UR_QUAN_PROBE_ID: Self = Self(28);
    pub const NUM_SPECIES_ID: Self = Self(29);
}
pub type HFLEETINFO = HLINK;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const HOSTILE_SPACE_HELLO_1: Self = Self(1);
    pub const HOSTILE_SPACE_HELLO_2: Self = Self(2);
    pub const HOSTILE_SPACE_HELLO_3: Self = Self(3);
    pub const HOSTILE_SPACE_HELLO_4: Self = Self(4);
    pub const HOSTILE_HOMEWORLD_HELLO_1: Self = Self(5);
    pub const HOSTILE_HOMEWORLD_HELLO_2: Self = Self(6);
    pub const HOSTILE_HOMEWORLD_HELLO_3: Self = Self(7);
    pub const HOSTILE_HOMEWORLD_HELLO_4: Self = Self(8);
    pub const whats_up_hostile_1: Self = Self(9);
    pub const whats_up_hostile_2: Self = Self(10);
    pub const GENERAL_INFO_HOSTILE_1: Self = Self(11);
    pub const GENERAL_INFO_HOSTILE_2: Self = Self(12);
    pub const GENERAL_INFO_HOSTILE_3: Self = Self(13);
    pub const GENERAL_INFO_HOSTILE_4: Self = Self(14);
    pub const what_about_you_1: Self = Self(15);
    pub const ABOUT_US_1: Self = Self(16);
    pub const what_about_you_2: Self = Self(17);
    pub const ABOUT_US_2: Self = Self(18);
    pub const what_about_urquan_1: Self = Self(19);
    pub const ABOUT_URQUAN_1: Self = Self(20);
    pub const what_about_urquan_2: Self = Self(21);
    pub const ABOUT_URQUAN_2: Self = Self(22);
    pub const got_idea: Self = Self(23);
    pub const GOOD_IDEA: Self = Self(24);
    pub const WE_GO_TO_IMPRESS_URQUAN_1: Self = Self(25);
    pub const WE_GO_TO_IMPRESS_URQUAN_2: Self = Self(26);
    pub const WE_IMPRESSING_URQUAN_1: Self = Self(27);
    pub const WE_IMPRESSING_URQUAN_2: Self = Self(28);
    pub const WE_IMPRESSED_URQUAN_1: Self = Self(29);
    pub const WE_IMPRESSED_URQUAN_2: Self = Self(30);
    pub const HOSTILE_HELIX_HELLO_1: Self = Self(31);
    pub const HOSTILE_HELIX_HELLO_2: Self = Self(32);
    pub const submit_1: Self = Self(33);
    pub const NO_SUBMIT_1: Self = Self(34);
    pub const submit_2: Self = Self(35);
    pub const NO_SUBMIT_2: Self = Self(36);
    pub const be_friends_1: Self = Self(37);
    pub const NO_FRIENDS_1: Self = Self(38);
    pub const be_friends_2: Self = Self(39);
    pub const NO_FRIENDS_2: Self = Self(40);
    pub const how_impressed_urquan_1: Self = Self(41);
    pub const IMPRESSED_LIKE_SO_1: Self = Self(42);
    pub const how_impressed_urquan_2: Self = Self(43);
    pub const IMPRESSED_LIKE_SO_2: Self = Self(44);
    pub const bye_hostile_1: Self = Self(45);
    pub const GOODBYE_HOSTILE_1: Self = Self(46);
    pub const bye_hostile_2: Self = Self(47);
    pub const GOODBYE_HOSTILE_2: Self = Self(48);
    pub const why_you_here_hostile: Self = Self(49);
    pub const NONE_OF_YOUR_CONCERN: Self = Self(50);
    pub const demand_to_land: Self = Self(51);
    pub const NO_DEMAND: Self = Self(52);
    pub const what_about_this_world: Self = Self(53);
    pub const BLUE_HELIX: Self = Self(54);
    pub const whats_helix_hostile: Self = Self(55);
    pub const HELIX_IS_HOSTILE: Self = Self(56);
    pub const i_need_to_land_lie: Self = Self(57);
    pub const CAUGHT_LIE: Self = Self(58);
    pub const bye_hostile_helix: Self = Self(59);
    pub const GOODBYE_HOSTILE_HELIX: Self = Self(60);
    pub const DIE_THIEF_1: Self = Self(61);
    pub const DIE_THIEF_2: Self = Self(62);
    pub const AMAZING_PERFORMANCE: Self = Self(63);
    pub const IMPRESSIVE_PERFORMANCE: Self = Self(64);
    pub const ADEQUATE_PERFORMANCE: Self = Self(65);
    pub const HELLO_POLITE_1: Self = Self(66);
    pub const HELLO_POLITE_2: Self = Self(67);
    pub const HELLO_POLITE_3: Self = Self(68);
    pub const HELLO_POLITE_4: Self = Self(69);
    pub const HELLO_RHYME_1: Self = Self(70);
    pub const HELLO_RHYME_2: Self = Self(71);
    pub const HELLO_RHYME_3: Self = Self(72);
    pub const HELLO_RHYME_4: Self = Self(73);
    pub const HELLO_PIG_LATIN_1: Self = Self(74);
    pub const HELLO_PIG_LATIN_2: Self = Self(75);
    pub const HELLO_PIG_LATIN_3: Self = Self(76);
    pub const HELLO_PIG_LATIN_4: Self = Self(77);
    pub const HELLO_LIKE_YOU_1: Self = Self(78);
    pub const HELLO_LIKE_YOU_2: Self = Self(79);
    pub const HELLO_LIKE_YOU_3: Self = Self(80);
    pub const HELLO_LIKE_YOU_4: Self = Self(81);
    pub const WELCOME_SPACE0: Self = Self(82);
    pub const WELCOME_SPACE1: Self = Self(83);
    pub const WELCOME_HOMEWORLD0: Self = Self(84);
    pub const WELCOME_HOMEWORLD1: Self = Self(85);
    pub const WELCOME_HELIX0: Self = Self(86);
    pub const WELCOME_HELIX1: Self = Self(87);
    pub const why_you_here_ally: Self = Self(88);
    pub const GUARDING_HELIX_ALLY: Self = Self(89);
    pub const whats_helix_ally: Self = Self(90);
    pub const HELIX_IS_ALLY: Self = Self(91);
    pub const may_i_land: Self = Self(92);
    pub const SURE_LAND: Self = Self(93);
    pub const whats_up_ally: Self = Self(94);
    pub const GENERAL_INFO_ALLY_1: Self = Self(95);
    pub const GENERAL_INFO_ALLY_2: Self = Self(96);
    pub const GENERAL_INFO_ALLY_3: Self = Self(97);
    pub const GENERAL_INFO_ALLY_4: Self = Self(98);
    pub const HOW_SHOULD_WE_ACT: Self = Self(99);
    pub const friendly: Self = Self(100);
    pub const OK_FRIENDLY: Self = Self(101);
    pub const wacky: Self = Self(102);
    pub const OK_WACKY: Self = Self(103);
    pub const just_like_us: Self = Self(104);
    pub const OK_JUST_LIKE_YOU: Self = Self(105);
    pub const WORK_TO_DO: Self = Self(106);
    pub const contemplative: Self = Self(107);
    pub const OK_CONTEMPLATIVE: Self = Self(108);
    pub const how_goes_culture: Self = Self(109);
    pub const CONTEMP_GOES_1: Self = Self(110);
    pub const CONTEMP_GOES_2: Self = Self(111);
    pub const FRIENDLY_GOES_1: Self = Self(112);
    pub const FRIENDLY_GOES_2: Self = Self(113);
    pub const WACKY_GOES_1: Self = Self(114);
    pub const WACKY_GOES_2: Self = Self(115);
    pub const LIKE_YOU_GOES_1: Self = Self(116);
    pub const LIKE_YOU_GOES_2: Self = Self(117);
    pub const bye_ally: Self = Self(118);
    pub const GOODBYE_ALLY_1: Self = Self(119);
    pub const GOODBYE_ALLY_2: Self = Self(120);
    pub const GOODBYE_ALLY_3: Self = Self(121);
    pub const GOODBYE_ALLY_4: Self = Self(122);
    pub const be_polite: Self = Self(123);
    pub const OK_POLITE: Self = Self(124);
    pub const speak_pig_latin: Self = Self(125);
    pub const OK_PIG_LATIN: Self = Self(126);
    pub const use_rhymes: Self = Self(127);
    pub const OK_RHYMES: Self = Self(128);
    pub const just_the_way_we_do: Self = Self(129);
    pub const OK_WAY_YOU_DO: Self = Self(130);
    pub const WHAT_NAME_FOR_CULTURE: Self = Self(131);
    pub const alliance_name: Self = Self(132);
    pub const OK_ALLIANCE_NAME: Self = Self(133);
    pub const NAME_TAIL: Self = Self(134);
    pub const you_decide: Self = Self(135);
    pub const OK_CULTURE_20: Self = Self(136);
    pub const fat: Self = Self(137);
    pub const OK_FAT: Self = Self(138);
    pub const the_slave_empire: Self = Self(139);
    pub const OK_SLAVE: Self = Self(140);
    pub const FAT_JERKS: Self = Self(141);
    pub const CULTURE: Self = Self(142);
    pub const SLAVE_EMPIRE: Self = Self(143);
    pub const name_1: Self = Self(144);
    pub const name_2: Self = Self(145);
    pub const name_3: Self = Self(146);
    pub const name_4: Self = Self(147);
    pub const HAVING_FUN_WITH_ILWRATH_1: Self = Self(148);
    pub const HAVING_FUN_WITH_ILWRATH_2: Self = Self(149);
    pub const GO_AWAY_FIGHTING_ILWRATH_1: Self = Self(150);
    pub const GO_AWAY_FIGHTING_ILWRATH_2: Self = Self(151);
    pub const OUT_TAKES: Self = Self(152);
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
#[inline]
unsafe extern "C" fn LockLink(mut pq: *const QUEUE, mut h: HLINK) -> *mut LINK {
    if !h.is_null() {
        (!(*pq).pq_tab.is_null() && h as *mut BYTE >= (*pq).pq_tab
            && (h as *mut BYTE)
                < (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (*pq).num_objects as ::core::ffi::c_int) as isize,
                    )
            || {
                __assert_fail(
                    b"pq->pq_tab && (BYTE*)h >= pq->pq_tab && (BYTE*)h < pq->pq_tab + pq->object_size * pq->num_objects\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/displist.h\0".as_ptr() as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int,
                    b"LockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
    return h as *mut LINK;
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
static mut thradd_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::THRADD_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.thraddash.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.thraddash.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.thraddash.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.thraddash.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.thraddash.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 8 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 8 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 12 as COUNT,
            NumFrames: 9 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 21 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 27 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 30 as COUNT,
            NumFrames: 12 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: 840 as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 42 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 47 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 52 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                as COUNT,
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
        NumFrames: 7 as BYTE,
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
unsafe extern "C" fn GetCultureName() -> ::core::ffi::c_int {
    let mut culture: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match uqm_lua_checked!(getGameStateUint(b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char), 0) {
        1 => {
            culture = C2Rust_Unnamed_0::CULTURE.0 as ::core::ffi::c_int;
        }
        2 => {
            culture = C2Rust_Unnamed_0::FAT_JERKS.0 as ::core::ffi::c_int;
        }
        3 => {
            culture = C2Rust_Unnamed_0::SLAVE_EMPIRE.0 as ::core::ffi::c_int;
        }
        _ => {
            (false
                && !(b"Unknown culture\0".as_ptr() as *const ::core::ffi::c_char)
                    .is_null()
                || {
                    __assert_fail(
                        b"0 && \"Unknown culture\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/comm/thradd/thraddc.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        158 as ::core::ffi::c_int,
                        b"GetCultureName\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
        }
    }
    return culture;
}
unsafe extern "C" fn PolitePhrase(mut which_phrase: BYTE) {
    match which_phrase as ::core::ffi::c_int {
        0 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_POLITE_1.0 as ::core::ffi::c_int, None), ());
        }
        1 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_POLITE_2.0 as ::core::ffi::c_int, None), ());
        }
        2 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_POLITE_3.0 as ::core::ffi::c_int, None), ());
        }
        3 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_POLITE_4.0 as ::core::ffi::c_int, None), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn RhymePhrase(mut which_phrase: BYTE) {
    match which_phrase as ::core::ffi::c_int {
        0 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_RHYME_1.0 as ::core::ffi::c_int, None), ());
        }
        1 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_RHYME_2.0 as ::core::ffi::c_int, None), ());
        }
        2 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_RHYME_3.0 as ::core::ffi::c_int, None), ());
        }
        3 => {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELLO_RHYME_4.0 as ::core::ffi::c_int, None), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn PigLatinPhrase(mut which_phrase: BYTE) {
    match which_phrase as ::core::ffi::c_int {
        0 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_PIG_LATIN_1.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        1 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_PIG_LATIN_2.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        2 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_PIG_LATIN_3.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        3 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_PIG_LATIN_4.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn LikeYouPhrase(mut which_phrase: BYTE) {
    match which_phrase as ::core::ffi::c_int {
        0 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_LIKE_YOU_1.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        1 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_LIKE_YOU_2.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        2 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_LIKE_YOU_3.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        3 => {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HELLO_LIKE_YOU_4.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::bye_hostile_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_HOSTILE_2.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_hostile_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_HOSTILE_1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_HOSTILE_STACK_5\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::submit_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_SUBMIT_1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_HOSTILE_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::submit_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_SUBMIT_2.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::got_idea.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOOD_IDEA.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_3::ADVANCE_THRADD_MISSION.0 as ::core::ffi::c_int as BYTE,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            5 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_hostile_helix.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE_HOSTILE_HELIX.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_ally.0 as ::core::ffi::c_int
    {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh8 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh8 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GOODBYE_ALLY_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GOODBYE_ALLY_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GOODBYE_ALLY_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GOODBYE_ALLY_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::may_i_land.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SURE_LAND.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"HELIX_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::demand_to_land.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_DEMAND.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::i_need_to_land_lie.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::CAUGHT_LIE.0 as ::core::ffi::c_int, None), ());
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::contemplative.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::OK_CONTEMPLATIVE.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::friendly.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_FRIENDLY.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::wacky.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_WACKY.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
                2 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_0::just_like_us.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::OK_JUST_LIKE_YOU.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WORK_TO_DO.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    };
}
unsafe extern "C" fn ThraddAllies(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::why_you_here_ally.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GUARDING_HELIX_ALLY.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::why_you_here_ally.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_helix_ally.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELIX_IS_ALLY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_helix_ally.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_ally.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh9 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh9 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_ALLY_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_ALLY_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_ALLY_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_ALLY_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_ally.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_goes_culture.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        match NumVisits as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int
        {
            0 => {
                if NumVisits as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) == 0
                {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::CONTEMP_GOES_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::CONTEMP_GOES_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            }
            1 => {
                if NumVisits as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) == 0
                {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_GOES_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::FRIENDLY_GOES_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            }
            2 => {
                if NumVisits as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) == 0
                {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::WACKY_GOES_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::WACKY_GOES_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            }
            3 => {
                if NumVisits as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) == 0
                {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::LIKE_YOU_GOES_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::LIKE_YOU_GOES_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            }
            _ => {}
        }
        NumVisits = (NumVisits as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_culture.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::why_you_here_ally.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::why_you_here_ally.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_0::whats_helix_ally.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::whats_helix_ally.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(ThraddAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::may_i_land.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_ally.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_ally.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ThraddAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::how_goes_culture.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::how_goes_culture.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ThraddAllies as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_ally.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ThraddDemeanor(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::you_decide.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_CULTURE_20.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_0::fat.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_FAT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::the_slave_empire.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_SLAVE.0 as ::core::ffi::c_int, None), ());
    }
    uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HOW_SHOULD_WE_ACT.0 as ::core::ffi::c_int, None), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::contemplative.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::friendly.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::wacky.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::just_like_us.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ThraddCulture(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::be_polite.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_POLITE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::use_rhymes.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_RHYMES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::speak_pig_latin.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_PIG_LATIN.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::just_the_way_we_do.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OK_WAY_YOU_DO.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    }
    uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WHAT_NAME_FOR_CULTURE.0 as ::core::ffi::c_int, None), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::you_decide.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddDemeanor as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::fat.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddDemeanor as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::the_slave_empire.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddDemeanor as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ThraddWorship(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setGameStateUint(
        b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
    SetRaceAllied(RACE_ID::THRADDASH_SHIP, BOOLEAN::TRUE);
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::be_polite.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddCulture as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::speak_pig_latin.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddCulture as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::use_rhymes.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddCulture as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::just_the_way_we_do.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ThraddCulture as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn HelixWorld(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::why_you_here_hostile.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::NONE_OF_YOUR_CONCERN.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_this_world.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::BLUE_HELIX.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_helix_hostile.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HELIX_IS_HOSTILE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::i_need_to_land_lie.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::CAUGHT_LIE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::why_you_here_hostile.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(HelixWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::demand_to_land.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_this_world.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(HelixWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::whats_helix_hostile.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(HelixWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {}
    }
    if uqm_lua_checked!(getGameStateUint(b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::i_need_to_land_lie.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_hostile_helix.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ThraddHostile(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_hostile_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GENERAL_INFO_HOSTILE_1.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_hostile_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_hostile_2.0 as ::core::ffi::c_int
    {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh7 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh7 as ::core::ffi::c_int {
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_HOSTILE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_HOSTILE_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_HOSTILE_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_hostile_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_you_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_US_1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_you_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_US_2.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_urquan_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_URQUAN_1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_about_urquan_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_URQUAN_2.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
            4 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::be_friends_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_FRIENDS_1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_HOSTILE_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::be_friends_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_FRIENDS_2.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::be_friends_2.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_impressed_urquan_1.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::IMPRESSED_LIKE_SO_1.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_HOSTILE_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_impressed_urquan_2.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::IMPRESSED_LIKE_SO_2.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
            5 as DWORD,
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_hostile_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_hostile_2.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_hostile_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_you_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_you_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        2 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_urquan_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        3 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::what_about_urquan_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        4 => {
            if uqm_lua_checked!(getGameStateUint(
                b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::got_idea.0 as ::core::ffi::c_int as RESPONSE_REF,
                    Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else {
                uqm_lua_checked!(setGameStateUint(
                    b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as DWORD,
                ), ());
            }
        }
        _ => {}
    }
    if uqm_lua_checked!(getGameStateUint(
        b"THRADD_HOSTILE_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::submit_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::submit_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"THRADD_HOSTILE_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::be_friends_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::be_friends_2.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::be_friends_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 4 as DWORD
    {
        if uqm_lua_checked!(getGameStateUint(
            b"THRADD_HOSTILE_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0 as DWORD
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::how_impressed_urquan_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::how_impressed_urquan_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ThraddHostile as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    if uqm_lua_checked!(getGameStateUint(
        b"THRADD_HOSTILE_STACK_5\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::bye_hostile_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::bye_hostile_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    let mut hThradd: HFLEETINFO = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
    );
    let mut ThraddPtr: *mut FLEET_INFO = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hThradd,
    );
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OUT_TAKES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        return;
    }
    if uqm_lua_checked!(getGameStateUint(b"AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
        && !(if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
            && (*ThraddPtr).allied_state as ::core::ffi::c_int
                == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int)
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::DIE_THIEF_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::DIE_THIEF_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0
            && uqm_lua_checked!(getGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
        {
            let mut pStr0: RESPONSE_REF = 0;
            let mut pStr1: RESPONSE_REF = 0;
            if uqm_lua_checked!(getGameStateUint(
                b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
            {
                pStr0 = C2Rust_Unnamed_0::WELCOME_HELIX0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_0::WELCOME_HELIX1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                uqm_lua_checked!(NPCPhrase_cb(pStr0 as ::core::ffi::c_int, None), ());
                uqm_lua_checked!(NPCPhrase_splice(uqm_lua_checked!(GetCultureName(), ())), ());
                uqm_lua_checked!(NPCPhrase_cb(pStr1 as ::core::ffi::c_int, None), ());
                uqm_lua_checked!(ThraddAllies(0 as ::core::ffi::c_int as RESPONSE_REF), ());
            } else {
                NumVisits = uqm_lua_checked!(getGameStateUint(
                    b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) as BYTE;
                let c2rust_fresh1 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh1 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HOSTILE_HELIX_HELLO_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HOSTILE_HELIX_HELLO_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(setGameStateUint(
                    b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    NumVisits as DWORD,
                ), ());
                uqm_lua_checked!(HelixWorld(0 as ::core::ffi::c_int as RESPONSE_REF), ());
            }
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            NumVisits = (if NumVisits as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                NumVisits as ::core::ffi::c_int
            }) as BYTE;
            if uqm_lua_checked!(getGameStateUint(
                b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
            {
                let c2rust_fresh2 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh2 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HAVING_FUN_WITH_ILWRATH_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HAVING_FUN_WITH_ILWRATH_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
            } else {
                let c2rust_fresh3 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh3 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::GO_AWAY_FIGHTING_ILWRATH_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::GO_AWAY_FIGHTING_ILWRATH_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
            }
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        }
    } else if uqm_lua_checked!(getGameStateUint(b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
    {
        let mut pStr0_0: RESPONSE_REF = 0;
        let mut pStr1_0: RESPONSE_REF = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        match uqm_lua_checked!(getGameStateUint(
            b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                uqm_lua_checked!(PolitePhrase(NumVisits), ());
            }
            1 => {
                uqm_lua_checked!(RhymePhrase(NumVisits), ());
            }
            2 => {
                uqm_lua_checked!(PigLatinPhrase(NumVisits), ());
            }
            3 => {
                uqm_lua_checked!(LikeYouPhrase(NumVisits), ());
            }
            _ => {}
        }
        NumVisits = NumVisits.wrapping_add(1);
        if (NumVisits as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
        {
            pStr0_0 = C2Rust_Unnamed_0::WELCOME_HELIX0.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr1_0 = C2Rust_Unnamed_0::WELCOME_HELIX1.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        } else if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            pStr0_0 = C2Rust_Unnamed_0::WELCOME_HOMEWORLD0.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr1_0 = C2Rust_Unnamed_0::WELCOME_HOMEWORLD1.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            uqm_lua_checked!(SetHomeworldKnown(
                C2Rust_Unnamed_2::THRADDASH_HOME.0 as ::core::ffi::c_int as DWORD,
            ), ());
        } else {
            pStr0_0 = C2Rust_Unnamed_0::WELCOME_SPACE0.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr1_0 = C2Rust_Unnamed_0::WELCOME_SPACE1.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        uqm_lua_checked!(NPCPhrase_cb(pStr0_0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_splice(uqm_lua_checked!(GetCultureName(), ())), ());
        uqm_lua_checked!(NPCPhrase_cb(pStr1_0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(ThraddAllies(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh4 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh4 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELIX_HELLO_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_HELIX_HELLO_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(HelixWorld(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"THRADDASH_BODY_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
    ), ())
        >= (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0 as ::core::ffi::c_int
        {
            25 as ::core::ffi::c_int
        } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 1 as ::core::ffi::c_int
        {
            15 as ::core::ffi::c_int
        } else {
            30 as ::core::ffi::c_int
        }) as BYTE as DWORD
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::AMAZING_PERFORMANCE.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(ThraddWorship(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"THRADDASH_BODY_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if NumVisits as ::core::ffi::c_int >= 16 as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"THRADD_BODY_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 1 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_BODY_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
                2 as DWORD,
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::IMPRESSIVE_PERFORMANCE.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if NumVisits as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"THRADD_BODY_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_BODY_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::ADEQUATE_PERFORMANCE.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"THRADD_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh5 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh5 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::HOSTILE_HOMEWORLD_HELLO_1.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::HOSTILE_HOMEWORLD_HELLO_2.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::HOSTILE_HOMEWORLD_HELLO_3.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::HOSTILE_HOMEWORLD_HELLO_4.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"THRADD_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(SetHomeworldKnown(
                C2Rust_Unnamed_2::THRADDASH_HOME.0 as ::core::ffi::c_int as DWORD,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            if NumVisits as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || NumVisits as ::core::ffi::c_int > 3 as ::core::ffi::c_int
            {
                NumVisits = uqm_lua_checked!(getGameStateUint(
                    b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) as BYTE;
                let c2rust_fresh6 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh6 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HOSTILE_SPACE_HELLO_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HOSTILE_SPACE_HELLO_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    2 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HOSTILE_SPACE_HELLO_3.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    3 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HOSTILE_SPACE_HELLO_4.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(setGameStateUint(
                    b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    NumVisits as DWORD,
                ), ());
            } else {
                match NumVisits as ::core::ffi::c_int {
                    1 => {
                        if uqm_lua_checked!(getGameStateUint(
                            b"THRADD_MISSION_VISITS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ), ()) == 0 as DWORD
                        {
                            uqm_lua_checked!(NPCPhrase_cb(
                                C2Rust_Unnamed_0::WE_GO_TO_IMPRESS_URQUAN_1.0
                                    as ::core::ffi::c_int,
                                None,
                            ), ());
                        } else {
                            uqm_lua_checked!(NPCPhrase_cb(
                                C2Rust_Unnamed_0::WE_GO_TO_IMPRESS_URQUAN_2.0
                                    as ::core::ffi::c_int,
                                None,
                            ), ());
                        }
                    }
                    2 => {
                        if uqm_lua_checked!(getGameStateUint(
                            b"THRADD_MISSION_VISITS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ), ()) == 0 as DWORD
                        {
                            uqm_lua_checked!(NPCPhrase_cb(
                                C2Rust_Unnamed_0::WE_IMPRESSING_URQUAN_1.0
                                    as ::core::ffi::c_int,
                                None,
                            ), ());
                        } else {
                            uqm_lua_checked!(NPCPhrase_cb(
                                C2Rust_Unnamed_0::WE_IMPRESSING_URQUAN_2.0
                                    as ::core::ffi::c_int,
                                None,
                            ), ());
                        }
                    }
                    3 => {
                        if uqm_lua_checked!(getGameStateUint(
                            b"THRADD_MISSION_VISITS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ), ()) == 0 as DWORD
                        {
                            uqm_lua_checked!(NPCPhrase_cb(
                                C2Rust_Unnamed_0::WE_IMPRESSED_URQUAN_1.0
                                    as ::core::ffi::c_int,
                                None,
                            ), ());
                        } else {
                            uqm_lua_checked!(NPCPhrase_cb(
                                C2Rust_Unnamed_0::WE_IMPRESSED_URQUAN_2.0
                                    as ::core::ffi::c_int,
                                None,
                            ), ());
                        }
                    }
                    _ => {}
                }
                uqm_lua_checked!(setGameStateUint(
                    b"THRADD_MISSION_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), ());
            }
        }
        uqm_lua_checked!(ThraddHostile(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_thradd() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_thradd_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_thradd_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    thradd_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    thradd_desc.post_encounter_func = Some(
        post_thradd_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    thradd_desc.uninit_encounter_func = Some(
        uninit_thradd as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    thradd_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    thradd_desc.AlienTextBaseline.y = 0 as COORD;
    thradd_desc.AlienTextWidth = (CanvasWidth
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
    if uqm_lua_checked!(getGameStateUint(b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut()) != 0
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut thradd_desc;
    return retval;
}
#[inline]
unsafe extern "C" fn LockFleetInfo(
    mut pq: *const QUEUE,
    mut h: HFLEETINFO,
) -> *mut FLEET_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<FLEET_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (FLEET_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0".as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
