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

// Rust translation of src/uqm/comm/druuge/druugec.c.
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
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
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
    fn NPCNumber(number: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char);
    fn setSegue(segue: Segue);
    fn getSegue() -> Segue;
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn EscortFeasibilityStudy(race: RACE_ID) -> COUNT;
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
    pub const PLANET_LANDER: Self = Self(0);
    pub const FUSION_THRUSTER: Self = Self(1);
    pub const TURNING_JETS: Self = Self(2);
    pub const CREW_POD: Self = Self(3);
    pub const STORAGE_BAY: Self = Self(4);
    pub const FUEL_TANK: Self = Self(5);
    pub const HIGHEFF_FUELSYS: Self = Self(6);
    pub const DYNAMO_UNIT: Self = Self(7);
    pub const SHIVA_FURNACE: Self = Self(8);
    pub const GUN_WEAPON: Self = Self(9);
    pub const BLASTER_WEAPON: Self = Self(10);
    pub const CANNON_WEAPON: Self = Self(11);
    pub const TRACKING_SYSTEM: Self = Self(12);
    pub const ANTIMISSILE_DEFENSE: Self = Self(13);
    pub const NUM_PURCHASE_MODULES: Self = Self(14);
    pub const BOMB_MODULE_0: Self = Self(14);
    pub const BOMB_MODULE_1: Self = Self(15);
    pub const BOMB_MODULE_2: Self = Self(16);
    pub const BOMB_MODULE_3: Self = Self(17);
    pub const BOMB_MODULE_4: Self = Self(18);
    pub const BOMB_MODULE_5: Self = Self(19);
    pub const NUM_MODULES: Self = Self(20);
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
    pub const AMBUSH_IS_FIRST_HELLO: Self = Self(1);
    pub const INIT_BOMB_WORLD_HELLO: Self = Self(2);
    pub const SUBSEQ_BOMB_WORLD_HELLO: Self = Self(3);
    pub const whats_up_at_bomb_planet: Self = Self(4);
    pub const GEN_INFO_AT_BOMB_PLANET: Self = Self(5);
    pub const we_get_bomb: Self = Self(6);
    pub const NOT_GET_BOMB: Self = Self(7);
    pub const then_we_take_bomb: Self = Self(8);
    pub const FIGHT_FOR_BOMB: Self = Self(9);
    pub const GOODBYE_FROM_BOMB_PLANET: Self = Self(10);
    pub const NOT_ENOUGH_ROOM: Self = Self(11);
    pub const TRADE_FOR_SPHERE: Self = Self(12);
    pub const no_way: Self = Self(13);
    pub const OK_REGULAR_DEAL: Self = Self(14);
    pub const way: Self = Self(15);
    pub const OK_HERES_SPHERE: Self = Self(16);
    pub const whats_the_sphere_again: Self = Self(17);
    pub const SPHERE_IS: Self = Self(18);
    pub const WE_SELL_FOR_CREW: Self = Self(19);
    pub const i_will_never_trade_crew: Self = Self(20);
    pub const YOUR_LOSS: Self = Self(21);
    pub const isnt_this_slave_trading: Self = Self(22);
    pub const NO_SLAVE_TRADE: Self = Self(23);
    pub const what_do_with_crew: Self = Self(24);
    pub const HAVE_FUN: Self = Self(25);
    pub const im_ready_to_buy: Self = Self(26);
    pub const THIS_FOR_SALE: Self = Self(27);
    pub const HAVE_SPHERE: Self = Self(28);
    pub const HAVE_ART_2: Self = Self(29);
    pub const HAVE_ART_1: Self = Self(30);
    pub const SHIPS_AND_FUEL: Self = Self(31);
    pub const BOUGHT_SHIP: Self = Self(32);
    pub const BOUGHT_FUEL: Self = Self(33);
    pub const BOUGHT_ART_2: Self = Self(34);
    pub const BOUGHT_ART_1: Self = Self(35);
    pub const BOUGHT_SPHERE: Self = Self(36);
    pub const repeat_what_to_sell: Self = Self(37);
    pub const INIT_SPACE_HELLO: Self = Self(38);
    pub const SUBSEQUENT_SPACE_HELLO: Self = Self(39);
    pub const whats_up_in_space: Self = Self(40);
    pub const GENERAL_INFO_IN_SPACE_1: Self = Self(41);
    pub const GENERAL_INFO_IN_SPACE_2: Self = Self(42);
    pub const GENERAL_INFO_IN_SPACE_3: Self = Self(43);
    pub const GENERAL_INFO_IN_SPACE_4: Self = Self(44);
    pub const GOODBYE_FROM_SPACE: Self = Self(45);
    pub const HSTL_TRADE_WORLD_HELLO_1: Self = Self(46);
    pub const HSTL_TRADE_WORLD_HELLO_2: Self = Self(47);
    pub const HOSTILE_SPACE_HELLO_1: Self = Self(48);
    pub const HOSTILE_SPACE_HELLO_2: Self = Self(49);
    pub const INITIAL_TRADE_WORLD_HELLO: Self = Self(50);
    pub const SSQ_TRADE_WORLD_HELLO_1: Self = Self(51);
    pub const SSQ_TRADE_WORLD_HELLO_2: Self = Self(52);
    pub const SSQ_TRADE_WORLD_HELLO_3: Self = Self(53);
    pub const SSQ_TRADE_WORLD_HELLO_4: Self = Self(54);
    pub const whats_up_at_trade_world: Self = Self(55);
    pub const GEN_INFO_AT_TRADE_WORLD_1: Self = Self(56);
    pub const GEN_INFO_AT_TRADE_WORLD_2: Self = Self(57);
    pub const GEN_INFO_AT_TRADE_WORLD_3: Self = Self(58);
    pub const GEN_INFO_AT_TRADE_WORLD_4: Self = Self(59);
    pub const SCAN_MAIDENS: Self = Self(60);
    pub const SCAN_FRAGMENTS: Self = Self(61);
    pub const SCAN_DRUUGE_CASTER: Self = Self(62);
    pub const SCAN_ARILOU_SPAWNER: Self = Self(63);
    pub const ENOUGH_FRAGMENTS: Self = Self(64);
    pub const READY_TO_BUY: Self = Self(65);
    pub const READY_TO_SELL: Self = Self(66);
    pub const BYE_FROM_TRADE_WORLD_1: Self = Self(67);
    pub const BYE_FROM_TRADE_WORLD_2: Self = Self(68);
    pub const NOT_ENOUGH_CREW: Self = Self(69);
    pub const EXCHANGE_MADE: Self = Self(70);
    pub const OK_DONE_BUYING: Self = Self(71);
    pub const OK_DONE_SELLING: Self = Self(72);
    pub const bye: Self = Self(73);
    pub const want_to_sell: Self = Self(74);
    pub const want_to_buy: Self = Self(75);
    pub const buy_druuge_ship: Self = Self(76);
    pub const buy_fuel: Self = Self(77);
    pub const buy_art_1: Self = Self(78);
    pub const buy_art_2: Self = Self(79);
    pub const buy_rosy_sphere: Self = Self(80);
    pub const done_buying: Self = Self(81);
    pub const done_selling: Self = Self(82);
    pub const sell_maidens: Self = Self(83);
    pub const sell_caster: Self = Self(84);
    pub const sell_fragments: Self = Self(85);
    pub const sell_spawner: Self = Self(86);
    pub const BOUGHT_MAIDENS: Self = Self(87);
    pub const BOUGHT_FRAGMENTS: Self = Self(88);
    pub const BOUGHT_CASTER: Self = Self(89);
    pub const YOU_GET: Self = Self(90);
    pub const YOU_ALSO_GET: Self = Self(91);
    pub const BOUGHT_SPAWNER: Self = Self(92);
    pub const SALVAGE_YOUR_SHIP_1: Self = Self(93);
    pub const SALVAGE_YOUR_SHIP_2: Self = Self(94);
    pub const DEAL_FOR_STATED_SHIPS: Self = Self(95);
    pub const DEAL_FOR_LESS_SHIPS: Self = Self(96);
    pub const DEAL_FOR_NO_SHIPS: Self = Self(97);
    pub const FUEL0: Self = Self(98);
    pub const FUEL1: Self = Self(99);
    pub const HIDEOUS_DEAL: Self = Self(100);
    pub const BAD_DEAL: Self = Self(101);
    pub const FAIR_DEAL: Self = Self(102);
    pub const GOOD_DEAL: Self = Self(103);
    pub const FINE_DEAL: Self = Self(104);
    pub const OUT_TAKES: Self = Self(105);
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
    pub const ILWRATH_ENCOUNTER: Self = Self(0);
    pub const PROBE_ENCOUNTER: Self = Self(1);
    pub const URQUAN_ENCOUNTER: Self = Self(2);
    pub const EGGCASE00_ENCOUNTER: Self = Self(3);
    pub const EGGCASE01_ENCOUNTER: Self = Self(4);
    pub const EGGCASE02_ENCOUNTER: Self = Self(5);
    pub const THRADDASH_ENCOUNTER: Self = Self(6);
    pub const NO_HELP_FROM_PKUNK: Self = Self(7);
    pub const READY_TO_BARGAIN: Self = Self(8);
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
static mut druuge_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::DRUUGE_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.druuge.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.druuge.font\0".as_ptr() as RESOURCE,
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
    AlienTextValign: TEXT_VALIGN::VALIGN_MIDDLE,
    AlienColorMapRes: b"comm.druuge.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.druuge.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.druuge.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 11 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 5 as COUNT,
            NumFrames: 4 as BYTE,
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
            StartIndex: 9 as COUNT,
            NumFrames: 4 as BYTE,
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
            StartIndex: 13 as COUNT,
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
            StartIndex: 19 as COUNT,
            NumFrames: 3 as BYTE,
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
            StartIndex: 22 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 25 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 28 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 31 as COUNT,
            NumFrames: 2 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 33 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 40 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int) as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 44 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 5 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 5 as ::core::ffi::c_int)
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
        NumFrames: 4 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int) as COUNT,
        RandomRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
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
static mut SlaveryCount: COUNT = 0 as COUNT;
static mut AttemptedSalvage: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::bye.0 as ::core::ffi::c_int {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            if SlaveryCount != 0 {
                let mut PreviousSlaves: UWORD = 0;
                PreviousSlaves = (((uqm_lua_checked!(getGameStateUint(
                    b"CREW_SOLD_TO_DRUUGE1\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                    as ::core::ffi::c_int
                    | uqm_lua_checked!(getGameStateUint(
                        b"CREW_SOLD_TO_DRUUGE0\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) as BYTE as ::core::ffi::c_int) as UWORD;
                SlaveryCount = (SlaveryCount as ::core::ffi::c_int
                    + PreviousSlaves as ::core::ffi::c_int) as COUNT;
                if SlaveryCount as ::core::ffi::c_int
                    > if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        250 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        500 as ::core::ffi::c_int
                    } else {
                        25 as ::core::ffi::c_int
                    }
                    && PreviousSlaves as ::core::ffi::c_int
                        <= if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0 as ::core::ffi::c_int
                        {
                            250 as ::core::ffi::c_int
                        } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 1 as ::core::ffi::c_int
                        {
                            500 as ::core::ffi::c_int
                        } else {
                            25 as ::core::ffi::c_int
                        }
                {
                    if PreviousSlaves as ::core::ffi::c_int
                        > if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0 as ::core::ffi::c_int
                        {
                            100 as ::core::ffi::c_int
                        } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 1 as ::core::ffi::c_int
                        {
                            200 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        }
                    {
                        GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                            as ::core::ffi::c_int
                            + (22 as ::core::ffi::c_int - 7 as ::core::ffi::c_int))
                            as BYTE;
                    } else {
                        GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                            as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as BYTE;
                    }
                } else if SlaveryCount as ::core::ffi::c_int
                    > if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        100 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        200 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    }
                    && PreviousSlaves as ::core::ffi::c_int
                        <= if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0 as ::core::ffi::c_int
                        {
                            100 as ::core::ffi::c_int
                        } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 1 as ::core::ffi::c_int
                        {
                            200 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        }
                {
                    GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                        as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as BYTE;
                }
                uqm_lua_checked!(setGameStateUint(
                    b"CREW_SOLD_TO_DRUUGE0\0".as_ptr() as *const ::core::ffi::c_char,
                    SlaveryCount as BYTE as DWORD,
                ), ());
                uqm_lua_checked!(setGameStateUint(
                    b"CREW_SOLD_TO_DRUUGE1\0".as_ptr() as *const ::core::ffi::c_char,
                    (SlaveryCount as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                        as BYTE as DWORD,
                ), ());
            }
            match uqm_lua_checked!(getGameStateUint(
                b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::BYE_FROM_TRADE_WORLD_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                _ => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::BYE_FROM_TRADE_WORLD_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            }
        } else if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::GOODBYE_FROM_BOMB_PLANET.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::GOODBYE_FROM_SPACE.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FIGHT_FOR_BOMB.0 as ::core::ffi::c_int, None), ());
    };
}
unsafe extern "C" fn Buy(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::want_to_buy.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::im_ready_to_buy.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::READY_TO_SELL.0 as ::core::ffi::c_int, None), ());
        if uqm_lua_checked!(getGameStateUint(b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HAVE_SPHERE.0 as ::core::ffi::c_int, None), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HAVE_ART_1.0 as ::core::ffi::c_int, None), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HAVE_ART_2.0 as ::core::ffi::c_int, None), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SHIPS_AND_FUEL.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::buy_druuge_ship.0 as ::core::ffi::c_int
    {
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 100 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NOT_ENOUGH_CREW.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if EscortFeasibilityStudy(RACE_ID::DRUUGE_SHIP) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NOT_ENOUGH_ROOM.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(DeltaSISGauges(-100 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
            SlaveryCount = (SlaveryCount as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(AddEscortShips(RACE_ID::DRUUGE_SHIP, 1 as SIZE), ());
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_SHIP.0 as ::core::ffi::c_int, None), ());
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::buy_rosy_sphere.0 as ::core::ffi::c_int
    {
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 100 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NOT_ENOUGH_CREW.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(DeltaSISGauges(-100 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
            SlaveryCount = (SlaveryCount as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(setGameStateUint(
                b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_SPHERE.0 as ::core::ffi::c_int, None), ());
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::buy_art_1.0 as ::core::ffi::c_int
    {
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 100 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NOT_ENOUGH_CREW.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(DeltaSISGauges(-100 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
            SlaveryCount = (SlaveryCount as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(setGameStateUint(
                b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_ART_1.0 as ::core::ffi::c_int, None), ());
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::buy_art_2.0 as ::core::ffi::c_int
    {
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 100 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NOT_ENOUGH_CREW.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(DeltaSISGauges(-100 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
            SlaveryCount = (SlaveryCount as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(setGameStateUint(
                b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_ART_2.0 as ::core::ffi::c_int, None), ());
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::buy_fuel.0 as ::core::ffi::c_int
    {
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 10 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::NOT_ENOUGH_CREW.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(DeltaSISGauges(
                -10 as SIZE,
                10 as SDWORD * 100 as SDWORD,
                0 as ::core::ffi::c_int,
            ), ());
            SlaveryCount = (SlaveryCount as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_FUEL.0 as ::core::ffi::c_int, None), ());
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::buy_druuge_ship.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0 {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::buy_rosy_sphere.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::buy_art_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::buy_art_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::buy_fuel.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::done_buying.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TradeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
static mut LastResponse: RESPONSE_REF = 0 as RESPONSE_REF;
unsafe extern "C" fn Trade(mut R: RESPONSE_REF) {
    if !(R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_the_sphere_again.0 as ::core::ffi::c_int)
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::TRADE_FOR_SPHERE.0 as ::core::ffi::c_int, None), ());
        LastResponse = R;
    } else {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SPHERE_IS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_the_sphere_again.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::no_way.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(Sell as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::way.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(Sell as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_the_sphere_again.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_the_sphere_again.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(Trade as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
}
unsafe extern "C" fn DoTransaction(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::sell_maidens.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::sell_fragments.0 as ::core::ffi::c_int
    {
        let mut num_frags: BYTE = 0;
        if uqm_lua_checked!(getGameStateUint(
            b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        }
        num_frags = uqm_lua_checked!(getGameStateUint(
                b"FRAGMENTS_BOUGHT\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .wrapping_add(1 as DWORD) as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"FRAGMENTS_BOUGHT\0".as_ptr() as *const ::core::ffi::c_char,
            num_frags as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::sell_caster.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::sell_spawner.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0
        && uqm_lua_checked!(getGameStateUint(
            b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        uqm_lua_checked!(setGameStateUint(
            b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else {
        let mut trade_gas: BYTE = 0;
        let mut ship_slots: BYTE = 0;
        let mut ships_to_trade: BYTE = 0;
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
                b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
                & ((1 as ::core::ffi::c_int)
                    << C2Rust_Unnamed_3::READY_TO_BARGAIN.0 as ::core::ffi::c_int)
                    as DWORD == 0
        {
            let mut state: UWORD = 0;
            state = uqm_lua_checked!(getGameStateUint(
                b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as UWORD;
            state = (state as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int)
                    << C2Rust_Unnamed_3::READY_TO_BARGAIN.0 as ::core::ffi::c_int)
                as UWORD;
            uqm_lua_checked!(setGameStateUint(
                b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
                state as DWORD,
            ), ());
        }
        trade_gas = 0 as BYTE;
        ships_to_trade = 0 as BYTE;
        ship_slots = EscortFeasibilityStudy(RACE_ID::DRUUGE_SHIP) as BYTE;
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_maidens.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_MAIDENS.0 as ::core::ffi::c_int, None), ());
            ships_to_trade = 6 as BYTE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_fragments.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::BOUGHT_FRAGMENTS.0 as ::core::ffi::c_int,
                None,
            ), ());
            ships_to_trade = 1 as BYTE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_caster.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_CASTER.0 as ::core::ffi::c_int, None), ());
            ships_to_trade = 0 as BYTE;
            trade_gas = 1 as BYTE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_spawner.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BOUGHT_SPAWNER.0 as ::core::ffi::c_int, None), ());
            ships_to_trade = 3 as BYTE;
            trade_gas = 1 as BYTE;
        }
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::YOU_GET.0 as ::core::ffi::c_int, None), ());
        if ships_to_trade != 0 {
            uqm_lua_checked!(AddEscortShips(RACE_ID::DRUUGE_SHIP, ships_to_trade as SIZE), ());
            if ship_slots as ::core::ffi::c_int >= ships_to_trade as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::DEAL_FOR_STATED_SHIPS.0 as ::core::ffi::c_int,
                    None,
                ), ());
            } else if ship_slots as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::DEAL_FOR_NO_SHIPS.0 as ::core::ffi::c_int,
                    None,
                ), ());
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::DEAL_FOR_LESS_SHIPS.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            if trade_gas != 0 {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::YOU_ALSO_GET.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
        }
        if trade_gas != 0 {
            let mut slot: BYTE = 0;
            let mut f: COUNT = 0;
            let mut HardLimit: COUNT = 500 as COUNT;
            let mut capacity: DWORD = 0;
            let mut FuelOnBoard: DWORD = 0;
            FuelOnBoard = GlobData.SIS_state.FuelOnBoard;
            capacity = (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD;
            slot = (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE;
            loop {
                if GlobData.SIS_state.ModuleSlots[slot as usize] as ::core::ffi::c_int
                    == C2Rust_Unnamed::FUEL_TANK.0 as ::core::ffi::c_int
                    || GlobData.SIS_state.ModuleSlots[slot as usize]
                        as ::core::ffi::c_int
                        == C2Rust_Unnamed::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
                {
                    let mut volume: COUNT = 0;
                    volume = (if GlobData.SIS_state.ModuleSlots[slot as usize]
                        as ::core::ffi::c_int
                        == C2Rust_Unnamed::FUEL_TANK.0 as ::core::ffi::c_int
                    {
                        50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                    } else {
                        100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                    }) as COUNT;
                    capacity = capacity.wrapping_add(volume as DWORD);
                }
                let c2rust_fresh8 = slot;
                slot = slot.wrapping_sub(1);
                if c2rust_fresh8 == 0 {
                    break;
                }
            }
            capacity = capacity.wrapping_sub(FuelOnBoard);
            f = capacity
                .wrapping_add(
                    (100 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as DWORD,
                )
                .wrapping_div(100 as DWORD) as COUNT;
            if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0 && f as ::core::ffi::c_int > HardLimit as ::core::ffi::c_int
            {
                f = (f as ::core::ffi::c_int
                    - (f as ::core::ffi::c_int - HardLimit as ::core::ffi::c_int))
                    as COUNT;
                capacity = ((f as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                    as DWORD)
                    .wrapping_sub(FuelOnBoard);
            }
            while capacity as ::core::ffi::c_ulong > 0x3fff as ::core::ffi::c_ulong {
                uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 0x3fff as SDWORD, 0 as ::core::ffi::c_int), ());
                capacity = capacity.wrapping_sub(0x3fff as DWORD);
            }
            uqm_lua_checked!(DeltaSISGauges(0 as SIZE, capacity as SDWORD, 0 as ::core::ffi::c_int), ());
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FUEL0.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(NPCNumber(
                f as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ), ());
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FUEL1.0 as ::core::ffi::c_int, None), ());
            if f as ::core::ffi::c_int >= 250 as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HIDEOUS_DEAL.0 as ::core::ffi::c_int,
                    None,
                ), ());
            } else if f as ::core::ffi::c_int >= 100 as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BAD_DEAL.0 as ::core::ffi::c_int, None), ());
            } else if f as ::core::ffi::c_int >= 50 as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FAIR_DEAL.0 as ::core::ffi::c_int, None), ());
            } else if f as ::core::ffi::c_int >= 10 as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOOD_DEAL.0 as ::core::ffi::c_int, None), ());
            } else {
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FINE_DEAL.0 as ::core::ffi::c_int, None), ());
            }
        }
    };
}
unsafe extern "C" fn Sell(mut R: RESPONSE_REF) {
    let mut RespFunc: RESPONSE_FUNC = None;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::want_to_sell.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::READY_TO_BUY.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_1::no_way.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int == C2Rust_Unnamed_1::way.0 as ::core::ffi::c_int
    {
        if R as ::core::ffi::c_int == C2Rust_Unnamed_1::no_way.0 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::OK_REGULAR_DEAL.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::OK_HERES_SPHERE.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        uqm_lua_checked!(DoTransaction(LastResponse), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::sell_maidens.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_fragments.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_caster.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::sell_spawner.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoTransaction(R), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0 {
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
                b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
                & ((1 as ::core::ffi::c_int)
                    << C2Rust_Unnamed_3::READY_TO_BARGAIN.0 as ::core::ffi::c_int)
                    as DWORD != 0
            || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0
        {
            RespFunc = Some(Trade as unsafe extern "C" fn(RESPONSE_REF) -> ());
        } else {
            RespFunc = Some(Sell as unsafe extern "C" fn(RESPONSE_REF) -> ());
        }
    } else {
        RespFunc = Some(Sell as unsafe extern "C" fn(RESPONSE_REF) -> ());
    }
    if uqm_lua_checked!(getGameStateUint(b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::sell_maidens.0 as ::core::ffi::c_int as RESPONSE_REF,
            RespFunc,
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if (uqm_lua_checked!(getGameStateUint(b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
        || uqm_lua_checked!(getGameStateUint(
            b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        || uqm_lua_checked!(getGameStateUint(
            b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0)
        && uqm_lua_checked!(getGameStateUint(b"FRAGMENTS_BOUGHT\0".as_ptr() as *const ::core::ffi::c_char), ())
            < 2 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::sell_fragments.0 as ::core::ffi::c_int as RESPONSE_REF,
            RespFunc,
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::sell_caster.0 as ::core::ffi::c_int as RESPONSE_REF,
            RespFunc,
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(
        b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::sell_spawner.0 as ::core::ffi::c_int as RESPONSE_REF,
            RespFunc,
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::done_selling.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TradeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ExplainSlaveTrade(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::want_to_buy.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::WE_SELL_FOR_CREW.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::isnt_this_slave_trading.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::NO_SLAVE_TRADE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_do_with_crew.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HAVE_FUN.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(
        b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::isnt_this_slave_trading.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ExplainSlaveTrade as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::what_do_with_crew.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ExplainSlaveTrade as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {}
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::i_will_never_trade_crew.0 as ::core::ffi::c_int
            as RESPONSE_REF,
        Some(TradeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::im_ready_to_buy.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn TradeWorld(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_at_trade_world.0 as ::core::ffi::c_int
    {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"DRUUGE_HOME_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh7 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh7 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GEN_INFO_AT_TRADE_WORLD_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GEN_INFO_AT_TRADE_WORLD_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GEN_INFO_AT_TRADE_WORLD_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
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
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GEN_INFO_AT_TRADE_WORLD_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"DRUUGE_HOME_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_at_trade_world.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::done_selling.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_DONE_SELLING.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::done_buying.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_DONE_BUYING.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_will_never_trade_crew.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::YOUR_LOSS.0 as ::core::ffi::c_int, None), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_at_trade_world.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_at_trade_world.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(TradeWorld as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::want_to_sell.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(Sell as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 3 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::want_to_buy.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(Buy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::want_to_buy.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExplainSlaveTrade as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn BombAmbush(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_at_bomb_planet.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GEN_INFO_AT_BOMB_PLANET.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_get_bomb.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::NOT_GET_BOMB.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    }
    match uqm_lua_checked!(getGameStateUint(b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::whats_up_at_bomb_planet.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(BombAmbush as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        2 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::we_get_bomb.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(BombAmbush as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::then_we_take_bomb.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Space(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_in_space.0 as ::core::ffi::c_int
    {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"DRUUGE_SPACE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh6 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh6 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_IN_SPACE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_IN_SPACE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_IN_SPACE_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_IN_SPACE_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"DRUUGE_SPACE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_in_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_in_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_in_space.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(Space as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::bye.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
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
    if uqm_lua_checked!(getGameStateUint(b"DRUUGE_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0 {
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh0 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh0 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HSTL_TRADE_WORLD_HELLO_1.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HSTL_TRADE_WORLD_HELLO_2.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh1 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh1 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HOSTILE_SPACE_HELLO_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::HOSTILE_SPACE_HELLO_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh2 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::INITIAL_TRADE_WORLD_HELLO.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SSQ_TRADE_WORLD_HELLO_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SSQ_TRADE_WORLD_HELLO_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SSQ_TRADE_WORLD_HELLO_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SSQ_TRADE_WORLD_HELLO_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        if uqm_lua_checked!(getGameStateUint(b"ATTACKED_DRUUGE\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
            && uqm_lua_checked!(getGameStateUint(
                b"DRUUGE_DISCLAIMER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"DRUUGE_DISCLAIMER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
            && uqm_lua_checked!(getGameStateUint(
                b"SCANNED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SCAN_MAIDENS.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"SCANNED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        if (uqm_lua_checked!(getGameStateUint(
            b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
            || uqm_lua_checked!(getGameStateUint(
                b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
            || uqm_lua_checked!(getGameStateUint(
                b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0)
            && uqm_lua_checked!(getGameStateUint(
                b"SCANNED_FRAGMENTS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            if uqm_lua_checked!(getGameStateUint(
                b"FRAGMENTS_BOUGHT\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) < 2 as DWORD
            {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::SCAN_FRAGMENTS.0 as ::core::ffi::c_int,
                    None,
                ), ());
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::ENOUGH_FRAGMENTS.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            uqm_lua_checked!(setGameStateUint(
                b"SCANNED_FRAGMENTS\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
            && uqm_lua_checked!(getGameStateUint(
                b"SCANNED_CASTER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::SCAN_DRUUGE_CASTER.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"SCANNED_CASTER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
            && uqm_lua_checked!(getGameStateUint(
                b"SCANNED_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::SCAN_ARILOU_SPAWNER.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"SCANNED_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        uqm_lua_checked!(TradeWorld(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        if uqm_lua_checked!(getGameStateUint(b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::SUBSEQ_BOMB_WORLD_HELLO.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::INIT_BOMB_WORLD_HELLO.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        uqm_lua_checked!(BombAmbush(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"ATTACKED_DRUUGE\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_SPACE_HELLO_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_SPACE_HELLO_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else {
        NumVisits = 0 as BYTE;
        if GlobData.Game_state.built_ship_q.head.is_null() {
            NumVisits = 0 as BYTE;
            while (NumVisits as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                let mut which_module: BYTE = 0;
                which_module = GlobData.SIS_state.ModuleSlots[NumVisits as usize];
                if which_module as ::core::ffi::c_int
                    >= C2Rust_Unnamed::GUN_WEAPON.0 as ::core::ffi::c_int
                    && which_module as ::core::ffi::c_int
                        <= C2Rust_Unnamed::CANNON_WEAPON.0 as ::core::ffi::c_int
                {
                    NumVisits = 0 as BYTE;
                    break;
                } else {
                    NumVisits = NumVisits.wrapping_add(1);
                }
            }
        }
        if NumVisits != 0 {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"DRUUGE_SALVAGE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh4 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh4 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SALVAGE_YOUR_SHIP_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SALVAGE_YOUR_SHIP_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"DRUUGE_SALVAGE\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
            AttemptedSalvage = BOOLEAN::TRUE;
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh5 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh5 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::INIT_SPACE_HELLO.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    uqm_lua_checked!(SetHomeworldKnown(
                        C2Rust_Unnamed_2::DRUUGE_HOME.0 as ::core::ffi::c_int as DWORD,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_1::SUBSEQUENT_SPACE_HELLO.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(Space(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        }
    };
}
unsafe extern "C" fn uninit_druuge() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_druuge_enc() {
    if uqm_lua_checked!(getSegue(), ()).0 == Segue::Segue_hostile.0 && AttemptedSalvage.0 == 0
        && uqm_lua_checked!(getGameStateUint(b"DRUUGE_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
    {
        if uqm_lua_checked!(getGameStateUint(b"ATTACKED_DRUUGE\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"ATTACKED_DRUUGE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_druuge_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    SlaveryCount = 0 as COUNT;
    AttemptedSalvage = BOOLEAN::FALSE;
    druuge_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    druuge_desc.post_encounter_func = Some(
        post_druuge_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    druuge_desc.uninit_encounter_func = Some(
        uninit_druuge as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    druuge_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    druuge_desc.AlienTextBaseline.y = ((64 as ::core::ffi::c_int) << resolutionFactor)
        as COORD;
    druuge_desc.AlienTextWidth = (CanvasWidth
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
    if uqm_lua_checked!(getGameStateUint(b"DRUUGE_MANNER\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        == 0 as DWORD
        && uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ::core::ptr::null_mut()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut druuge_desc;
    return retval;
}
