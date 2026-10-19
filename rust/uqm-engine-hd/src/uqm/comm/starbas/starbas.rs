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

// Rust translation of src/uqm/comm/starbas/starbas.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn TaskSwitch();
    fn FlushInput();
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut oldRU: DWORD;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optSpeech: OPT_ENABLABLE;
    fn ClearSISRect(ClearFlags: BYTE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut CommData: LOCDATA;
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn NPCPhrase_splice(index: ::core::ffi::c_int);
    fn NPCNumber(number: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char);
    fn setSegue(segue: Segue);
    fn AlienTalkSegue(wait_track: COUNT);
    fn BlockTalkingAnim(trackStart: COUNT, trackEnd: COUNT);
    fn UpdateDuty(talk: BOOLEAN);
    static mut altResFlags: BYTE;
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn TFB_Random() -> DWORD;
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn CalculateEscortsWorth() -> COUNT;
    fn EscortFeasibilityStudy(race: RACE_ID) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
    fn DrawCargoStrings(OldElement: BYTE, NewElement: BYTE);
    fn ShowRemainingCapacity();
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
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
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
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
    pub const COMMON: Self = Self(0);
    pub const CORROSIVE: Self = Self(1);
    pub const BASE_METAL: Self = Self(2);
    pub const NOBLE: Self = Self(3);
    pub const RARE_EARTH: Self = Self(4);
    pub const PRECIOUS: Self = Self(5);
    pub const RADIOACTIVE: Self = Self(6);
    pub const EXOTIC: Self = Self(7);
    pub const NUM_ELEMENT_CATEGORIES: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const PORTAL_SPAWNER_DEVICE: Self = Self(0);
    pub const TALKING_PET_DEVICE: Self = Self(1);
    pub const UTWIG_BOMB_DEVICE: Self = Self(2);
    pub const SUN_EFFICIENCY_DEVICE: Self = Self(3);
    pub const ROSY_SPHERE_DEVICE: Self = Self(4);
    pub const AQUA_HELIX_DEVICE: Self = Self(5);
    pub const CLEAR_SPINDLE_DEVICE: Self = Self(6);
    pub const ULTRON_0_DEVICE: Self = Self(7);
    pub const ULTRON_1_DEVICE: Self = Self(8);
    pub const ULTRON_2_DEVICE: Self = Self(9);
    pub const ULTRON_3_DEVICE: Self = Self(10);
    pub const MAIDENS_DEVICE: Self = Self(11);
    pub const UMGAH_HYPERWAVE_DEVICE: Self = Self(12);
    pub const BURVIX_HYPERWAVE_DEVICE: Self = Self(13);
    pub const DATA_PLATE_1_DEVICE: Self = Self(14);
    pub const DATA_PLATE_2_DEVICE: Self = Self(15);
    pub const DATA_PLATE_3_DEVICE: Self = Self(16);
    pub const TAALO_PROTECTOR_DEVICE: Self = Self(17);
    pub const EGG_CASING0_DEVICE: Self = Self(18);
    pub const EGG_CASING1_DEVICE: Self = Self(19);
    pub const EGG_CASING2_DEVICE: Self = Self(20);
    pub const SYREEN_SHUTTLE_DEVICE: Self = Self(21);
    pub const VUX_BEAST_DEVICE: Self = Self(22);
    pub const DESTRUCT_CODE_DEVICE: Self = Self(23);
    pub const URQUAN_WARP_DEVICE: Self = Self(24);
    pub const WIMBLIS_TRIDENT_DEVICE: Self = Self(25);
    pub const GLOWING_ROD_DEVICE: Self = Self(26);
    pub const LUNAR_BASE_DEVICE: Self = Self(27);
    pub const NUM_DEVICES: Self = Self(28);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const BEFORE_WE_GO_ON_1: Self = Self(1);
    pub const BEFORE_WE_GO_ON_2: Self = Self(2);
    pub const BEFORE_WE_GO_ON_3: Self = Self(3);
    pub const BEFORE_WE_GO_ON_4: Self = Self(4);
    pub const BEFORE_WE_GO_ON_5: Self = Self(5);
    pub const BEFORE_WE_GO_ON_6: Self = Self(6);
    pub const BEFORE_WE_GO_ON_7: Self = Self(7);
    pub const NORMAL_HELLO_A: Self = Self(8);
    pub const NORMAL_HELLO_B: Self = Self(9);
    pub const NORMAL_HELLO_C: Self = Self(10);
    pub const NORMAL_HELLO_D: Self = Self(11);
    pub const NORMAL_HELLO_E: Self = Self(12);
    pub const NORMAL_HELLO_F: Self = Self(13);
    pub const NORMAL_HELLO_G: Self = Self(14);
    pub const NORMAL_HELLO_H: Self = Self(15);
    pub const RETURN_HELLO: Self = Self(16);
    pub const NORMAL_GOODBYE_A: Self = Self(17);
    pub const NORMAL_GOODBYE_B: Self = Self(18);
    pub const NORMAL_GOODBYE_C: Self = Self(19);
    pub const NORMAL_GOODBYE_D: Self = Self(20);
    pub const NORMAL_GOODBYE_E: Self = Self(21);
    pub const NORMAL_GOODBYE_F: Self = Self(22);
    pub const NORMAL_GOODBYE_G: Self = Self(23);
    pub const NORMAL_GOODBYE_H: Self = Self(24);
    pub const LIGHT_LOAD_A: Self = Self(25);
    pub const LIGHT_LOAD_B: Self = Self(26);
    pub const LIGHT_LOAD_C0: Self = Self(27);
    pub const LIGHT_LOAD_C1: Self = Self(28);
    pub const LIGHT_LOAD_D: Self = Self(29);
    pub const LIGHT_LOAD_E: Self = Self(30);
    pub const LIGHT_LOAD_F: Self = Self(31);
    pub const LIGHT_LOAD_G: Self = Self(32);
    pub const MEDIUM_LOAD_A: Self = Self(33);
    pub const MEDIUM_LOAD_B: Self = Self(34);
    pub const MEDIUM_LOAD_C: Self = Self(35);
    pub const MEDIUM_LOAD_D: Self = Self(36);
    pub const MEDIUM_LOAD_E: Self = Self(37);
    pub const MEDIUM_LOAD_F: Self = Self(38);
    pub const MEDIUM_LOAD_G: Self = Self(39);
    pub const HEAVY_LOAD_A: Self = Self(40);
    pub const HEAVY_LOAD_B: Self = Self(41);
    pub const HEAVY_LOAD_C: Self = Self(42);
    pub const HEAVY_LOAD_D: Self = Self(43);
    pub const HEAVY_LOAD_E: Self = Self(44);
    pub const HEAVY_LOAD_F: Self = Self(45);
    pub const HEAVY_LOAD_G: Self = Self(46);
    pub const STARBASE_IS_READY: Self = Self(47);
    pub const WHAT_KIND_OF_INFO: Self = Self(48);
    pub const WHICH_FUNCTION: Self = Self(49);
    pub const WHICH_HISTORY: Self = Self(50);
    pub const WHICH_MISSION: Self = Self(51);
    pub const OK_NO_NEED_INFO: Self = Self(52);
    pub const ABOUT_FUEL: Self = Self(53);
    pub const ABOUT_MODULES: Self = Self(54);
    pub const ABOUT_CREW: Self = Self(55);
    pub const ABOUT_SHIPS: Self = Self(56);
    pub const ABOUT_RU: Self = Self(57);
    pub const ABOUT_MINERALS: Self = Self(58);
    pub const ABOUT_LIFE: Self = Self(59);
    pub const OK_ENOUGH_STARBASE: Self = Self(60);
    pub const OK_ENOUGH_MISSION: Self = Self(61);
    pub const GET_MINERALS: Self = Self(62);
    pub const ABOUT_ALIENS: Self = Self(63);
    pub const MUST_DEFEAT: Self = Self(64);
    pub const DEFEAT_LIKE_SO: Self = Self(65);
    pub const FIND_URQUAN: Self = Self(66);
    pub const FIGHT_URQUAN: Self = Self(67);
    pub const ALLY_LIKE_SO: Self = Self(68);
    pub const STRONG_LIKE_SO: Self = Self(69);
    pub const OK_ENOUGH_DEFEAT: Self = Self(70);
    pub const WHICH_ALIEN: Self = Self(71);
    pub const WHICH_WAR: Self = Self(72);
    pub const WHICH_ANCIENT: Self = Self(73);
    pub const OK_ENOUGH_HISTORY: Self = Self(74);
    pub const WHICH_ALLIANCE: Self = Self(75);
    pub const WHICH_HIERARCHY: Self = Self(76);
    pub const ABOUT_OTHER: Self = Self(77);
    pub const OK_ENOUGH_ALIENS: Self = Self(78);
    pub const ABOUT_SHOFIXTI: Self = Self(79);
    pub const ABOUT_YEHAT: Self = Self(80);
    pub const ABOUT_ARILOU: Self = Self(81);
    pub const ABOUT_CHENJESU: Self = Self(82);
    pub const ABOUT_MMRNMHRM: Self = Self(83);
    pub const ABOUT_SYREEN: Self = Self(84);
    pub const OK_ENOUGH_ALLIANCE: Self = Self(85);
    pub const ABOUT_URQUAN: Self = Self(86);
    pub const ABOUT_MYCON: Self = Self(87);
    pub const ABOUT_SPATHI: Self = Self(88);
    pub const ABOUT_UMGAH: Self = Self(89);
    pub const ABOUT_ANDROSYNTH: Self = Self(90);
    pub const ABOUT_VUX: Self = Self(91);
    pub const ABOUT_ILWRATH: Self = Self(92);
    pub const OK_ENOUGH_HIERARCHY: Self = Self(93);
    pub const ABOUT_PRECURSORS: Self = Self(94);
    pub const ABOUT_OLD_RACES: Self = Self(95);
    pub const ABOUT_ALIENS_ON_EARTH: Self = Self(96);
    pub const OK_ENOUGH_ANCIENT: Self = Self(97);
    pub const URQUAN_STARTED_WAR: Self = Self(98);
    pub const WAR_WAS_LIKE_SO: Self = Self(99);
    pub const LOST_WAR_BECAUSE: Self = Self(100);
    pub const AFTER_WAR: Self = Self(101);
    pub const OK_ENOUGH_WAR: Self = Self(102);
    pub const STARBASE_BULLETIN_TAIL: Self = Self(103);
    pub const BETWEEN_BULLETINS: Self = Self(104);
    pub const STARBASE_BULLETIN_1: Self = Self(105);
    pub const STARBASE_BULLETIN_2: Self = Self(106);
    pub const STARBASE_BULLETIN_3: Self = Self(107);
    pub const STARBASE_BULLETIN_4: Self = Self(108);
    pub const STARBASE_BULLETIN_5: Self = Self(109);
    pub const STARBASE_BULLETIN_6: Self = Self(110);
    pub const STARBASE_BULLETIN_7: Self = Self(111);
    pub const STARBASE_BULLETIN_8: Self = Self(112);
    pub const STARBASE_BULLETIN_9: Self = Self(113);
    pub const STARBASE_BULLETIN_10: Self = Self(114);
    pub const STARBASE_BULLETIN_11: Self = Self(115);
    pub const STARBASE_BULLETIN_12: Self = Self(116);
    pub const STARBASE_BULLETIN_13: Self = Self(117);
    pub const STARBASE_BULLETIN_14: Self = Self(118);
    pub const STARBASE_BULLETIN_15: Self = Self(119);
    pub const STARBASE_BULLETIN_16: Self = Self(120);
    pub const STARBASE_BULLETIN_18: Self = Self(121);
    pub const STARBASE_BULLETIN_19: Self = Self(122);
    pub const STARBASE_BULLETIN_22: Self = Self(123);
    pub const STARBASE_BULLETIN_27: Self = Self(124);
    pub const STARBASE_BULLETIN_28: Self = Self(125);
    pub const STARBASE_BULLETIN_29: Self = Self(126);
    pub const STARBASE_BULLETIN_30: Self = Self(127);
    pub const DEVICE_HEAD: Self = Self(128);
    pub const BETWEEN_DEVICES: Self = Self(129);
    pub const DEVICE_TAIL: Self = Self(130);
    pub const ABOUT_PORTAL: Self = Self(131);
    pub const ABOUT_TALKPET: Self = Self(132);
    pub const ABOUT_BOMB: Self = Self(133);
    pub const ABOUT_SUN: Self = Self(134);
    pub const ABOUT_MAIDENS: Self = Self(135);
    pub const ABOUT_SPHERE: Self = Self(136);
    pub const ABOUT_HELIX: Self = Self(137);
    pub const ABOUT_SPINDLE: Self = Self(138);
    pub const ABOUT_ULTRON_0: Self = Self(139);
    pub const ABOUT_ULTRON_1: Self = Self(140);
    pub const ABOUT_ULTRON_2: Self = Self(141);
    pub const ABOUT_ULTRON_3: Self = Self(142);
    pub const ABOUT_UCASTER: Self = Self(143);
    pub const ABOUT_BCASTER: Self = Self(144);
    pub const ABOUT_SHIELD: Self = Self(145);
    pub const ABOUT_EGGCASE_0: Self = Self(146);
    pub const ABOUT_SHUTTLE: Self = Self(147);
    pub const ABOUT_VUXBEAST0: Self = Self(148);
    pub const ABOUT_VUXBEAST1: Self = Self(149);
    pub const ABOUT_DESTRUCT: Self = Self(150);
    pub const ABOUT_WARPPOD: Self = Self(151);
    pub const ABOUT_WIMBLIS_TRIDENT: Self = Self(152);
    pub const ABOUT_GLOWING_ROD: Self = Self(153);
    pub const LETS_SEE: Self = Self(154);
    pub const GO_GET_MINERALS: Self = Self(155);
    pub const IMPROVE_FLAGSHIP_WITH_RU: Self = Self(156);
    pub const GOT_OK_FLAGSHIP: Self = Self(157);
    pub const GO_ALLY_WITH_ALIENS: Self = Self(158);
    pub const MADE_SOME_ALLIES: Self = Self(159);
    pub const GET_SHIPS_BY_MINING_OR_ALLIANCE: Self = Self(160);
    pub const GOT_OK_FLEET: Self = Self(161);
    pub const BUY_COMBAT_SHIPS: Self = Self(162);
    pub const GO_LEARN_ABOUT_URQUAN: Self = Self(163);
    pub const MAKE_FLAGSHIP_AWESOME: Self = Self(164);
    pub const KNOW_ABOUT_SAMATRA: Self = Self(165);
    pub const GOT_AWESOME_FLAGSHIP: Self = Self(166);
    pub const GOT_BOMB: Self = Self(167);
    pub const FIND_WAY_TO_DESTROY_SAMATRA: Self = Self(168);
    pub const MUST_INCREASE_BOMB_STRENGTH: Self = Self(169);
    pub const MUST_ACQUIRE_AWESOME_FLEET: Self = Self(170);
    pub const MUST_ELIMINATE_URQUAN_GUARDS: Self = Self(171);
    pub const CHMMR_IMPROVED_BOMB: Self = Self(172);
    pub const GOT_AWESOME_FLEET: Self = Self(173);
    pub const GO_DESTROY_SAMATRA: Self = Self(174);
    pub const GOOD_LUCK_AGAIN: Self = Self(175);
    pub const IMPROVE_1: Self = Self(176);
    pub const IMPROVE_2: Self = Self(177);
    pub const NEED_THRUSTERS_1: Self = Self(178);
    pub const NEED_THRUSTERS_2: Self = Self(179);
    pub const NEED_TURN_1: Self = Self(180);
    pub const NEED_TURN_2: Self = Self(181);
    pub const NEED_GUNS_1: Self = Self(182);
    pub const NEED_GUNS_2: Self = Self(183);
    pub const NEED_CREW_1: Self = Self(184);
    pub const NEED_CREW_2: Self = Self(185);
    pub const NEED_FUEL_1: Self = Self(186);
    pub const NEED_FUEL_2: Self = Self(187);
    pub const NEED_STORAGE_1: Self = Self(188);
    pub const NEED_LANDERS_2: Self = Self(189);
    pub const NEED_LANDERS_1: Self = Self(190);
    pub const NEED_DYNAMOS_1: Self = Self(191);
    pub const NEED_DYNAMOS_2: Self = Self(192);
    pub const NEED_POINT: Self = Self(193);
    pub const have_minerals: Self = Self(194);
    pub const goodbye_commander: Self = Self(195);
    pub const repeat_bulletins: Self = Self(196);
    pub const need_info: Self = Self(197);
    pub const starbase_functions: Self = Self(198);
    pub const history: Self = Self(199);
    pub const our_mission: Self = Self(200);
    pub const no_need_info: Self = Self(201);
    pub const enough_starbase: Self = Self(202);
    pub const enough_mission: Self = Self(203);
    pub const tell_me_about_fuel: Self = Self(204);
    pub const tell_me_about_modules: Self = Self(205);
    pub const tell_me_about_crew: Self = Self(206);
    pub const tell_me_about_ships: Self = Self(207);
    pub const tell_me_about_ru: Self = Self(208);
    pub const tell_me_about_minerals: Self = Self(209);
    pub const tell_me_about_life: Self = Self(210);
    pub const where_get_minerals: Self = Self(211);
    pub const what_about_aliens: Self = Self(212);
    pub const what_about_urquan: Self = Self(213);
    pub const how_defeat: Self = Self(214);
    pub const how_find_urquan: Self = Self(215);
    pub const how_fight_urquan: Self = Self(216);
    pub const how_ally: Self = Self(217);
    pub const enough_defeat: Self = Self(218);
    pub const alien_races: Self = Self(219);
    pub const the_war: Self = Self(220);
    pub const ancient_history: Self = Self(221);
    pub const enough_history: Self = Self(222);
    pub const what_about_alliance: Self = Self(223);
    pub const what_about_hierarchy: Self = Self(224);
    pub const what_about_other: Self = Self(225);
    pub const enough_aliens: Self = Self(226);
    pub const shofixti: Self = Self(227);
    pub const yehat: Self = Self(228);
    pub const arilou: Self = Self(229);
    pub const chenjesu: Self = Self(230);
    pub const mmrnmhrm: Self = Self(231);
    pub const syreen: Self = Self(232);
    pub const enough_alliance: Self = Self(233);
    pub const urquan: Self = Self(234);
    pub const mycon: Self = Self(235);
    pub const spathi: Self = Self(236);
    pub const umgah: Self = Self(237);
    pub const androsynth: Self = Self(238);
    pub const vux: Self = Self(239);
    pub const ilwrath: Self = Self(240);
    pub const enough_hierarchy: Self = Self(241);
    pub const precursors: Self = Self(242);
    pub const old_races: Self = Self(243);
    pub const aliens_on_earth: Self = Self(244);
    pub const enough_ancient: Self = Self(245);
    pub const what_started_war: Self = Self(246);
    pub const what_was_war_like: Self = Self(247);
    pub const why_lose_war: Self = Self(248);
    pub const what_after_war: Self = Self(249);
    pub const enough_war: Self = Self(250);
    pub const new_devices: Self = Self(251);
    pub const how_get_strong: Self = Self(252);
    pub const what_do_now: Self = Self(253);
    pub const CARGO_LIST: Self = Self(254);
    pub const ELLIPSES: Self = Self(255);
    pub const BLANK: Self = Self(256);
    pub const KILOTONS_OF: Self = Self(257);
    pub const COMMONR: Self = Self(258);
    pub const CORROSR: Self = Self(259);
    pub const BASEMER: Self = Self(260);
    pub const NOBLESR: Self = Self(261);
    pub const RAREEAR: Self = Self(262);
    pub const PRECIOR: Self = Self(263);
    pub const RADIOAR: Self = Self(264);
    pub const EXOTICR: Self = Self(265);
    pub const FOR: Self = Self(266);
    pub const RESUNITS: Self = Self(267);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
static mut commander_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::COMMANDER_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.commander.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.commander.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.commander.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.commander.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: b"comm.commander.red.graphics\0".as_ptr() as RESOURCE,
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: b"comm.starbase.music\0".as_ptr() as RESOURCE,
    },
    ConversationPhrasesRes: b"comm.starbase.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 10 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 10 as COUNT,
            NumFrames: 30 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 40 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 47 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 55 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 61 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 67 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 74 as COUNT,
            NumFrames: 11 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 85 as COUNT,
            NumFrames: 10 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 95 as COUNT,
            NumFrames: 1 as BYTE,
            AnimFlags: 0 as COUNT,
            BaseFrameRate: 0 as COUNT,
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
        StartIndex: 4 as COUNT,
        NumFrames: 6 as BYTE,
        AnimFlags: 0 as COUNT,
        BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
            / 60 as ::core::ffi::c_int) as COUNT,
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
static mut CurBulletinMask: DWORD = 0;
unsafe extern "C" fn DoSellMinerals() -> COUNT {
    let mut i: COUNT = 0;
    let mut total: COUNT = 0 as COUNT;
    let mut Sleepy: BOOLEAN = BOOLEAN::TRUE;
    FlushInput();
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        let mut amount: COUNT = 0;
        let mut TimeIn: DWORD = 0 as DWORD;
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawCargoStrings(
                !(0 as ::core::ffi::c_int) as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            ), 0);
            TimeIn = GetTimeCounter()
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount,
                ) as DWORD;
            while GetTimeCounter() <= TimeIn {
                if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), 0).0 != 0
                    || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    Sleepy = BOOLEAN::FALSE;
                    break;
                } else {
                    uqm_lua_checked!(UpdateDuty(BOOLEAN::FALSE), 0);
                }
            }
            if Sleepy.0 != 0 {
                uqm_lua_checked!(DrawCargoStrings(
                    0 as ::core::ffi::c_int as BYTE,
                    0 as ::core::ffi::c_int as BYTE,
                ), 0);
            }
        } else if Sleepy.0 != 0 {
            uqm_lua_checked!(DrawCargoStrings(
                (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE,
                i as BYTE,
            ), 0);
        }
        amount = GlobData.SIS_state.ElementAmounts[i as usize];
        if amount as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_int
                + amount as ::core::ffi::c_int
                    * GlobData.Game_state.ElementWorth[i as usize] as ::core::ffi::c_int)
                as COUNT;
            loop {
                if Sleepy.0 == 0 || uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), 0).0 != 0
                    || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    Sleepy = BOOLEAN::FALSE;
                    GlobData.SIS_state.ElementAmounts[i as usize] = 0 as COUNT;
                    GlobData.SIS_state.TotalElementMass = (GlobData
                        .SIS_state
                        .TotalElementMass as ::core::ffi::c_int
                        - amount as ::core::ffi::c_int) as COUNT;
                    uqm_lua_checked!(DeltaSISGauges(
                        0 as SIZE,
                        0 as SDWORD,
                        amount as ::core::ffi::c_int
                            * GlobData.Game_state.ElementWorth[i as usize]
                                as ::core::ffi::c_int,
                    ), 0);
                    break;
                } else {
                    uqm_lua_checked!(UpdateDuty(BOOLEAN::FALSE), 0);
                    GlobData.SIS_state.ElementAmounts[i as usize] = GlobData
                        .SIS_state
                        .ElementAmounts[i as usize]
                        .wrapping_sub(1);
                    GlobData.SIS_state.TotalElementMass = GlobData
                        .SIS_state
                        .TotalElementMass
                        .wrapping_sub(1);
                    TaskSwitch();
                    uqm_lua_checked!(DrawCargoStrings(i as BYTE, i as BYTE), 0);
                    uqm_lua_checked!(ShowRemainingCapacity(), 0);
                    uqm_lua_checked!(DeltaSISGauges(
                        0 as SIZE,
                        0 as SDWORD,
                        GlobData.Game_state.ElementWorth[i as usize]
                            as ::core::ffi::c_int,
                    ), 0);
                    amount = amount.wrapping_sub(1);
                    if amount == 0 {
                        break;
                    }
                }
            }
        }
        if Sleepy.0 != 0 {
            TimeIn = GetTimeCounter()
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as TimeCount,
                ) as DWORD;
            while GetTimeCounter() <= TimeIn {
                if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), 0).0 != 0
                    || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    Sleepy = BOOLEAN::FALSE;
                    break;
                } else {
                    uqm_lua_checked!(UpdateDuty(BOOLEAN::FALSE), 0);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod), 0);
    uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE), 0);
    return total;
}
unsafe extern "C" fn EveryOnesACritic(mut total: COUNT) -> RESPONSE_REF {
    let mut pStr: RESPONSE_REF = C2Rust_Unnamed_3::LIGHT_LOAD_A.0 as ::core::ffi::c_int
        as RESPONSE_REF;
    if (total as ::core::ffi::c_int) < 1000 as ::core::ffi::c_int {
        total = uqm_lua_checked!(getGameStateUint(
            b"LIGHT_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) as COUNT;
        let c2rust_fresh0 = total;
        total = total.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_A.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_B.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_C0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_D.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_E.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_F.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                total = total.wrapping_sub(1);
                pStr = C2Rust_Unnamed_3::LIGHT_LOAD_G.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"LIGHT_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
            total as DWORD,
        ), 0);
    } else if (total as ::core::ffi::c_int) < 2500 as ::core::ffi::c_int {
        total = uqm_lua_checked!(getGameStateUint(
            b"MEDIUM_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) as COUNT;
        let c2rust_fresh1 = total;
        total = total.wrapping_add(1);
        match c2rust_fresh1 as ::core::ffi::c_int {
            0 => {
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_A.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_B.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_C.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_D.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_E.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_F.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                total = total.wrapping_sub(1);
                pStr = C2Rust_Unnamed_3::MEDIUM_LOAD_G.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"MEDIUM_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
            total as DWORD,
        ), 0);
    } else {
        total = uqm_lua_checked!(getGameStateUint(
            b"HEAVY_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) as COUNT;
        let c2rust_fresh2 = total;
        total = total.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_A.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_B.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_C.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_D.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_E.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_F.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                total = total.wrapping_sub(1);
                pStr = C2Rust_Unnamed_3::HEAVY_LOAD_G.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"HEAVY_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
            total as DWORD,
        ), 0);
    }
    return pStr;
}
unsafe extern "C" fn ByeBye(mut R: RESPONSE_REF) {
    CurBulletinMask
        |= uqm_lua_checked!(getGameStateUint(
            b"STARBASE_BULLETS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ());
    uqm_lua_checked!(setGameStateUint(
        b"STARBASE_BULLETS\0".as_ptr() as *const ::core::ffi::c_char,
        CurBulletinMask,
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
        >= 2 as DWORD
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::GOOD_LUCK_AGAIN.0 as ::core::ffi::c_int, None), ());
    } else {
        let mut pStr: RESPONSE_REF = 0 as RESPONSE_REF;
        match TFB_Random() as BYTE as ::core::ffi::c_int & 7 as ::core::ffi::c_int {
            0 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_A.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_B.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_C.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_D.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_E.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_F.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_G.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            7 => {
                pStr = C2Rust_Unnamed_3::NORMAL_GOODBYE_H.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        uqm_lua_checked!(NPCPhrase_cb(pStr as ::core::ffi::c_int, None), ());
    };
}
static mut stack0: BYTE = 0;
static mut stack1: BYTE = 0;
static mut stack2: BYTE = 0;
static mut stack3: BYTE = 0;
unsafe extern "C" fn AllianceInfo(mut R: RESPONSE_REF) {
    static mut AllianceMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_about_alliance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_ALLIANCE.0 as ::core::ffi::c_int, None), ());
        AllianceMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::shofixti.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_SHOFIXTI.0 as ::core::ffi::c_int, None), ());
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(SetHomeworldKnown(
            C2Rust_Unnamed_5::SHOFIXTI_HOME.0 as ::core::ffi::c_int as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::yehat.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_YEHAT.0 as ::core::ffi::c_int, None), ());
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::arilou.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_ARILOU.0 as ::core::ffi::c_int, None), ());
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::chenjesu.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_CHENJESU.0 as ::core::ffi::c_int, None), ());
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::mmrnmhrm.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_MMRNMHRM.0 as ::core::ffi::c_int, None), ());
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::syreen.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_SYREEN.0 as ::core::ffi::c_int, None), ());
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE;
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::shofixti.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::yehat.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::arilou.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::chenjesu.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::mmrnmhrm.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::syreen.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_alliance.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn HierarchyInfo(mut R: RESPONSE_REF) {
    static mut HierarchyMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_about_hierarchy.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_HIERARCHY.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_URQUAN.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::mycon.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_MYCON.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::spathi.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_SPATHI.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::umgah.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_UMGAH.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::androsynth.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_ANDROSYNTH.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(SetHomeworldKnown(
            C2Rust_Unnamed_5::ANDROSYNTH_HOME.0 as ::core::ffi::c_int as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::ilwrath.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_ILWRATH.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_3::vux.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_VUX.0 as ::core::ffi::c_int, None), ());
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE;
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::mycon.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::spathi.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::umgah.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::androsynth.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::ilwrath.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::vux.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_hierarchy.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn AlienRaces(mut R: RESPONSE_REF) {
    static mut RacesMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::alien_races.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_ALIEN.0 as ::core::ffi::c_int, None), ());
        RacesMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_alliance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_ALLIANCE.0 as ::core::ffi::c_int, None), ());
        RacesMask = (RacesMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_hierarchy.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_3::OK_ENOUGH_HIERARCHY.0 as ::core::ffi::c_int,
            None,
        ), ());
        RacesMask = (RacesMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_about_other.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_OTHER.0 as ::core::ffi::c_int, None), ());
        RacesMask = (RacesMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if RacesMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::what_about_alliance.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if RacesMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::what_about_hierarchy.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if RacesMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::what_about_other.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_aliens.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn WarInfo(mut R: RESPONSE_REF) {
    static mut WarMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::the_war.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_WAR.0 as ::core::ffi::c_int, None), ());
        WarMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_started_war.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::URQUAN_STARTED_WAR.0 as ::core::ffi::c_int, None), ());
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_was_war_like.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WAR_WAS_LIKE_SO.0 as ::core::ffi::c_int, None), ());
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::why_lose_war.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::LOST_WAR_BECAUSE.0 as ::core::ffi::c_int, None), ());
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_after_war.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::AFTER_WAR.0 as ::core::ffi::c_int, None), ());
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::what_started_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::what_was_war_like.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::why_lose_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::what_after_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_war.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn AncientHistory(mut R: RESPONSE_REF) {
    static mut AncientMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::ancient_history.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_ANCIENT.0 as ::core::ffi::c_int, None), ());
        AncientMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::precursors.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_PRECURSORS.0 as ::core::ffi::c_int, None), ());
        AncientMask = (AncientMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::old_races.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_OLD_RACES.0 as ::core::ffi::c_int, None), ());
        AncientMask = (AncientMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::aliens_on_earth.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_3::ABOUT_ALIENS_ON_EARTH.0 as ::core::ffi::c_int,
            None,
        ), ());
        AncientMask = (AncientMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if AncientMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::precursors.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AncientMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::old_races.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if AncientMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::aliens_on_earth.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_ancient.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn TellHistory(mut R: RESPONSE_REF) {
    let mut pstack: [RESPONSE_REF; 3] = [0; 3];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::history.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_HISTORY.0 as ::core::ffi::c_int, None), ());
        stack0 = 0 as BYTE;
        stack1 = 0 as BYTE;
        stack2 = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_aliens.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_ALIENS.0 as ::core::ffi::c_int, None), ());
        stack0 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_war.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_WAR.0 as ::core::ffi::c_int, None), ());
        stack1 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_ancient.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_ANCIENT.0 as ::core::ffi::c_int, None), ());
        stack2 = 1 as BYTE;
    }
    match stack0 as ::core::ffi::c_int {
        0 => {
            pstack[0usize] = C2Rust_Unnamed_3::alien_races.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[0usize] = 0 as RESPONSE_REF;
        }
    }
    match stack1 as ::core::ffi::c_int {
        0 => {
            pstack[1usize] = C2Rust_Unnamed_3::the_war.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[1usize] = 0 as RESPONSE_REF;
        }
    }
    match stack2 as ::core::ffi::c_int {
        0 => {
            pstack[2usize] = C2Rust_Unnamed_3::ancient_history.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[2usize] = 0 as RESPONSE_REF;
        }
    }
    if pstack[0usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[0usize],
            Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[1usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[1usize],
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[2usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[2usize],
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_history.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn DefeatUrquan(mut R: RESPONSE_REF) {
    static mut DefeatMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::how_defeat.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::DEFEAT_LIKE_SO.0 as ::core::ffi::c_int, None), ());
        DefeatMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::how_find_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::FIND_URQUAN.0 as ::core::ffi::c_int, None), ());
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::how_fight_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::FIGHT_URQUAN.0 as ::core::ffi::c_int, None), ());
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::how_ally.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ALLY_LIKE_SO.0 as ::core::ffi::c_int, None), ());
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::how_get_strong.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::STRONG_LIKE_SO.0 as ::core::ffi::c_int, None), ());
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::how_find_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::how_fight_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::how_ally.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::how_get_strong.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_defeat.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn AnalyzeCondition() {
    let mut i: BYTE = 0;
    let mut num_thrusters: BYTE = 0 as BYTE;
    let mut num_jets: BYTE = 0 as BYTE;
    let mut num_guns: BYTE = 0 as BYTE;
    let mut num_bays: BYTE = 0 as BYTE;
    let mut num_batts: BYTE = 0 as BYTE;
    let mut num_track: BYTE = 0 as BYTE;
    let mut num_defense: BYTE = 0 as BYTE;
    let mut HasMinimum: BOOLEAN = BOOLEAN::FALSE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        if (GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            num_thrusters = num_thrusters.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if (GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            num_jets = num_jets.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece: BYTE = 0;
        which_piece = GlobData.SIS_state.ModuleSlots[i as usize];
        match which_piece as ::core::ffi::c_int {
            4 => {
                num_bays = num_bays.wrapping_add(1);
            }
            7 | 8 => {
                num_batts = (num_batts as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int
                        + (which_piece as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::DYNAMO_UNIT.0 as ::core::ffi::c_int)))
                    as BYTE;
            }
            9 | 10 | 11 => {
                num_guns = (num_guns as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int
                        + (which_piece as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::GUN_WEAPON.0 as ::core::ffi::c_int)))
                    as BYTE;
            }
            12 => {
                num_track = num_track.wrapping_add(1);
            }
            13 => {
                num_defense = num_defense.wrapping_add(1);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if num_track as ::core::ffi::c_int != 0 && num_guns as ::core::ffi::c_int != 0 {
        num_guns = (num_guns as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
    }
    HasMinimum = BOOLEAN(
        (num_thrusters as ::core::ffi::c_int >= 7 as ::core::ffi::c_int
            && num_jets as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                >= 50 as ::core::ffi::c_int
            && GlobData.SIS_state.FuelOnBoard
                >= (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
            && num_bays as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && GlobData.SIS_state.NumLanders as ::core::ffi::c_int != 0
            && num_batts as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && num_guns as ::core::ffi::c_int >= 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::LETS_SEE.0 as ::core::ffi::c_int, None), ());
    if HasMinimum.0 == 0
        && uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            < 2 as DWORD
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::IMPROVE_1.0 as ::core::ffi::c_int, None), ());
        if (num_thrusters as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::NEED_THRUSTERS_1.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        if (num_jets as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_TURN_1.0 as ::core::ffi::c_int, None), ());
        }
        if (num_guns as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_GUNS_1.0 as ::core::ffi::c_int, None), ());
        }
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 50 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_CREW_1.0 as ::core::ffi::c_int, None), ());
        }
        if GlobData.SIS_state.FuelOnBoard
            < (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_FUEL_1.0 as ::core::ffi::c_int, None), ());
        }
        if (num_bays as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_STORAGE_1.0 as ::core::ffi::c_int, None), ());
        }
        if GlobData.SIS_state.NumLanders as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_LANDERS_2.0 as ::core::ffi::c_int, None), ());
        }
        if (num_batts as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::NEED_DYNAMOS_1.0 as ::core::ffi::c_int, None), ());
        }
        if GlobData.SIS_state.ResUnits >= 3000 as DWORD {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::IMPROVE_FLAGSHIP_WITH_RU.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::GO_GET_MINERALS.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
    } else {
        let mut num_aliens: BYTE = 0 as BYTE;
        let mut FleetStrength: COUNT = 0;
        let mut HasMaximum: BOOLEAN = BOOLEAN::FALSE;
        FleetStrength = CalculateEscortsWorth();
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int)
            < RACE_ID::NUM_AVAILABLE_RACES.0 as ::core::ffi::c_int
        {
            if i as ::core::ffi::c_int != RACE_ID::HUMAN_SHIP.0 as ::core::ffi::c_int
                && CheckAlliance(RACE_ID(i as ::core::ffi::c_uint)) as ::core::ffi::c_int
                    == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
            {
                num_aliens = num_aliens.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        HasMaximum = BOOLEAN(
            (num_thrusters as ::core::ffi::c_int == 11 as ::core::ffi::c_int
                && num_jets as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    >= 50 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                && GlobData.SIS_state.FuelOnBoard
                    >= (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int) as DWORD
                && GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                    >= 3 as ::core::ffi::c_int
                && num_batts as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                && num_guns as ::core::ffi::c_int >= 7 as ::core::ffi::c_int
                && num_defense as ::core::ffi::c_int >= 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if HasMaximum.0 == 0
            && uqm_lua_checked!(getGameStateUint(
                b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) < 2 as DWORD
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::GOT_OK_FLAGSHIP.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::GOT_AWESOME_FLAGSHIP.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            >= 2 as DWORD
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::CHMMR_IMPROVED_BOMB.0 as ::core::ffi::c_int,
                None,
            ), ());
            if (FleetStrength as ::core::ffi::c_int) < 20000 as ::core::ffi::c_int {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_3::MUST_ACQUIRE_AWESOME_FLEET.0 as ::core::ffi::c_int,
                    None,
                ), ());
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_3::GOT_AWESOME_FLEET.0 as ::core::ffi::c_int,
                    None,
                ), ());
                if uqm_lua_checked!(getGameStateUint(
                    b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) == 0
                {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::MUST_ELIMINATE_URQUAN_GUARDS.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::GO_DESTROY_SAMATRA.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            }
        } else if (num_aliens as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::GO_ALLY_WITH_ALIENS.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::MADE_SOME_ALLIES.0 as ::core::ffi::c_int,
                None,
            ), ());
            if (FleetStrength as ::core::ffi::c_int) < 6000 as ::core::ffi::c_int {
                if GlobData.SIS_state.ResUnits >= 3000 as DWORD {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::BUY_COMBAT_SHIPS.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::GET_SHIPS_BY_MINING_OR_ALLIANCE.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_3::GOT_OK_FLEET.0 as ::core::ffi::c_int,
                    None,
                ), ());
                if HasMaximum.0 == 0 {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::IMPROVE_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    if (num_thrusters as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_THRUSTERS_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if (num_jets as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_TURN_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if (num_guns as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_GUNS_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
                        < 50 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_CREW_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if GlobData.SIS_state.FuelOnBoard
                        < (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                            * 3 as ::core::ffi::c_int) as DWORD
                    {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_FUEL_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if (GlobData.SIS_state.NumLanders as ::core::ffi::c_int)
                        < 3 as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_LANDERS_1.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if (num_batts as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_DYNAMOS_2.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    if (num_defense as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::NEED_POINT.0 as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                } else if uqm_lua_checked!(getGameStateUint(
                    b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) == 0
                {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::GO_LEARN_ABOUT_URQUAN.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::KNOW_ABOUT_SAMATRA.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    if uqm_lua_checked!(getGameStateUint(
                        b"UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 0
                    {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::FIND_WAY_TO_DESTROY_SAMATRA.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    } else if uqm_lua_checked!(getGameStateUint(
                        b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_3::MUST_INCREASE_BOMB_STRENGTH.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn TellMission(mut R: RESPONSE_REF) {
    let mut pstack: [RESPONSE_REF; 4] = [0; 4];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::our_mission.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_MISSION.0 as ::core::ffi::c_int, None), ());
        stack0 = 0 as BYTE;
        stack1 = 0 as BYTE;
        stack2 = 0 as BYTE;
        stack3 = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::where_get_minerals.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::GET_MINERALS.0 as ::core::ffi::c_int, None), ());
        stack0 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_about_aliens.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_ALIENS.0 as ::core::ffi::c_int, None), ());
        stack1 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_do_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(AnalyzeCondition(), ());
        stack2 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::what_about_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::MUST_DEFEAT.0 as ::core::ffi::c_int, None), ());
        stack3 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_defeat.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_DEFEAT.0 as ::core::ffi::c_int, None), ());
        stack3 = 2 as BYTE;
    }
    match stack0 as ::core::ffi::c_int {
        0 => {
            pstack[0usize] = C2Rust_Unnamed_3::where_get_minerals.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[0usize] = 0 as RESPONSE_REF;
        }
    }
    match stack1 as ::core::ffi::c_int {
        0 => {
            pstack[1usize] = C2Rust_Unnamed_3::what_about_aliens.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[1usize] = 0 as RESPONSE_REF;
        }
    }
    match stack2 as ::core::ffi::c_int {
        0 => {
            pstack[2usize] = C2Rust_Unnamed_3::what_do_now.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[2usize] = 0 as RESPONSE_REF;
        }
    }
    match stack3 as ::core::ffi::c_int {
        0 => {
            pstack[3usize] = C2Rust_Unnamed_3::what_about_urquan.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pstack[3usize] = C2Rust_Unnamed_3::how_defeat.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[3usize] = 0 as RESPONSE_REF;
        }
    }
    if pstack[0usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[0usize],
            Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[1usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[1usize],
            Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[2usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[2usize],
            Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[3usize] != 0 {
        if stack3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            uqm_lua_checked!(DoResponsePhrase(
                pstack[3usize],
                Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                pstack[3usize],
                Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_mission.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn TellStarBase(mut R: RESPONSE_REF) {
    let mut pstack: [RESPONSE_REF; 4] = [0; 4];
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::starbase_functions.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHICH_FUNCTION.0 as ::core::ffi::c_int, None), ());
        stack0 = 0 as BYTE;
        stack1 = 0 as BYTE;
        stack2 = 0 as BYTE;
        stack3 = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_fuel.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_FUEL.0 as ::core::ffi::c_int, None), ());
        stack1 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_crew.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_CREW.0 as ::core::ffi::c_int, None), ());
        stack2 = 2 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_modules.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_MODULES.0 as ::core::ffi::c_int, None), ());
        stack0 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_ships.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_SHIPS.0 as ::core::ffi::c_int, None), ());
        stack2 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_ru.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_RU.0 as ::core::ffi::c_int, None), ());
        stack3 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_minerals.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_MINERALS.0 as ::core::ffi::c_int, None), ());
        stack3 = 2 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::tell_me_about_life.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::ABOUT_LIFE.0 as ::core::ffi::c_int, None), ());
        stack3 = 3 as BYTE;
    }
    match stack0 as ::core::ffi::c_int {
        0 => {
            pstack[0usize] = C2Rust_Unnamed_3::tell_me_about_modules.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        _ => {
            pstack[0usize] = 0 as RESPONSE_REF;
        }
    }
    match stack1 as ::core::ffi::c_int {
        0 => {
            pstack[1usize] = C2Rust_Unnamed_3::tell_me_about_fuel.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[1usize] = 0 as RESPONSE_REF;
        }
    }
    match stack2 as ::core::ffi::c_int {
        0 => {
            pstack[2usize] = C2Rust_Unnamed_3::tell_me_about_ships.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        1 => {
            pstack[2usize] = C2Rust_Unnamed_3::tell_me_about_crew.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[2usize] = 0 as RESPONSE_REF;
        }
    }
    match stack3 as ::core::ffi::c_int {
        0 => {
            pstack[3usize] = C2Rust_Unnamed_3::tell_me_about_ru.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pstack[3usize] = C2Rust_Unnamed_3::tell_me_about_minerals.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        2 => {
            pstack[3usize] = C2Rust_Unnamed_3::tell_me_about_life.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[3usize] = 0 as RESPONSE_REF;
        }
    }
    if pstack[0usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[0usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[1usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[1usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[2usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[2usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if pstack[3usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pstack[3usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::enough_starbase.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn NeedInfo(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::need_info.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::WHAT_KIND_OF_INFO.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_starbase.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_STARBASE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_history.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_HISTORY.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::enough_mission.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_ENOUGH_MISSION.0 as ::core::ffi::c_int, None), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::starbase_functions.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::history.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::our_mission.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::no_need_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NormalStarbase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn DiscussDevices(mut TalkAbout: BOOLEAN) -> BOOLEAN {
    let mut i: COUNT = 0;
    let mut VuxBeastIndex: COUNT = 0;
    let mut PhraseIndex: COUNT = 0;
    if TalkAbout.0 != 0 {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::DEVICE_HEAD.0 as ::core::ffi::c_int, None), BOOLEAN(0));
    }
    PhraseIndex = 2 as COUNT;
    VuxBeastIndex = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        let mut pStr: RESPONSE_REF = 0;
        pStr = 0 as RESPONSE_REF;
        match i as ::core::ffi::c_int {
            4 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_SPHERE.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_ROSY_SPHERE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            25 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_WIMBLIS_TRIDENT\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_WIMBLIS_TRIDENT.0
                        as ::core::ffi::c_int as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_WIMBLIS_TRIDENT\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            26 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_GLOWING_ROD\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_GLOWING_ROD.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_GLOWING_ROD\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            3 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_SUN_EFFICIENCY\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_SUN.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_SUN_EFFICIENCY\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            2 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_BOMB.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            7 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) == 1 as DWORD
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_ULTRON_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            8 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) == 2 as DWORD
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_ULTRON_1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            9 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) == 3 as DWORD
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_ULTRON_2.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            10 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) == 4 as DWORD
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_ULTRON_3.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            11 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_MAIDENS.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            1 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_TALKPET.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_TALKING_PET\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            5 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_HELIX.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            6 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_CLEAR_SPINDLE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_SPINDLE.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_CLEAR_SPINDLE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            12 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"UMGAH_BROADCASTERS_ON_SHIP\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_UMGAH_HYPERWAVE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_UCASTER.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_UMGAH_HYPERWAVE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            17 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_TAALO_PROTECTOR\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_SHIELD.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_TAALO_PROTECTOR\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            18 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_EGG_CASING0\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_EGGCASE_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING0\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING1\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING2\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            19 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_EGG_CASING1\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_EGGCASE_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING0\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING1\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING2\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            20 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_EGG_CASING2\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_EGGCASE_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING0\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING1\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_EGG_CASING2\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            21 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_SYREEN_SHUTTLE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_SHUTTLE.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_SYREEN_SHUTTLE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            22 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_VUX_BEAST\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_VUXBEAST0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_VUX_BEAST\0".as_ptr() as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            23 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_DESTRUCT_CODE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_DESTRUCT.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_DESTRUCT_CODE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            0 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_PORTAL_SPAWNER\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_PORTAL.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_PORTAL_SPAWNER\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            24 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"PORTAL_KEY_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_URQUAN_WARP\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_WARPPOD.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_URQUAN_WARP\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            13 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"DISCUSSED_BURVIX_HYPERWAVE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) == 0
                {
                    pStr = C2Rust_Unnamed_3::ABOUT_BCASTER.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    uqm_lua_checked!(setGameStateUint(
                        b"DISCUSSED_BURVIX_HYPERWAVE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TalkAbout.0,
                    ), BOOLEAN(0));
                }
            }
            _ => {}
        }
        if pStr != 0 {
            if TalkAbout.0 != 0 {
                if PhraseIndex as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::BETWEEN_DEVICES.0 as ::core::ffi::c_int,
                        None,
                    ), BOOLEAN(0));
                }
                uqm_lua_checked!(NPCPhrase_cb(pStr as ::core::ffi::c_int, None), BOOLEAN(0));
                if pStr as ::core::ffi::c_int
                    == C2Rust_Unnamed_3::ABOUT_VUXBEAST0.0 as ::core::ffi::c_int
                {
                    PhraseIndex = PhraseIndex.wrapping_add(1);
                    VuxBeastIndex = PhraseIndex;
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::ABOUT_VUXBEAST1.0 as ::core::ffi::c_int,
                        None,
                    ), BOOLEAN(0));
                }
            }
            PhraseIndex = (PhraseIndex as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as COUNT;
        }
        i = i.wrapping_add(1);
    }
    if TalkAbout.0 != 0 {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::DEVICE_TAIL.0 as ::core::ffi::c_int, None), BOOLEAN(0));
        if VuxBeastIndex != 0 {
            BlockTalkingAnim(
                (VuxBeastIndex as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
                VuxBeastIndex,
            );
            uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), BOOLEAN(0));
        }
    }
    return BOOLEAN(
        (PhraseIndex as ::core::ffi::c_int > 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn CheckTiming(
    mut month_index: COUNT,
    mut day_index: COUNT,
) -> BOOLEAN {
    let mut mi: COUNT = 0;
    let mut year_index: COUNT = 0;
    let mut days_in_month: [BYTE; 12] = [
        31 as BYTE,
        28 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
    ];
    mi = uqm_lua_checked!(getGameStateUint(b"STARBASE_MONTH\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
        as COUNT;
    year_index = 2155 as COUNT;
    day_index = (day_index as DWORD)
        .wrapping_add(
            uqm_lua_checked!(getGameStateUint(b"STARBASE_DAY\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0)),
        ) as COUNT;
    while day_index as ::core::ffi::c_int
        > days_in_month[(mi as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
    {
        day_index = (day_index as ::core::ffi::c_int
            - days_in_month[(mi as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int) as COUNT;
        mi = mi.wrapping_add(1);
        if mi as ::core::ffi::c_int > 12 as ::core::ffi::c_int {
            mi = 1 as COUNT;
            year_index = year_index.wrapping_add(1);
        }
    }
    month_index = (month_index as ::core::ffi::c_int + mi as ::core::ffi::c_int)
        as COUNT;
    year_index = (year_index as ::core::ffi::c_int
        + (month_index as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 12 as ::core::ffi::c_int) as COUNT;
    month_index = ((month_index as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        % 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
    return BOOLEAN(
        ((year_index as ::core::ffi::c_int)
            < GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
            || year_index as ::core::ffi::c_int
                == GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                && ((month_index as ::core::ffi::c_int)
                    < GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                    || month_index as ::core::ffi::c_int
                        == GlobData.Game_state.GameClock.month_index
                            as ::core::ffi::c_int
                        && (day_index as ::core::ffi::c_int)
                            < GlobData.Game_state.GameClock.day_index
                                as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn CheckBulletins(mut Repeat: BOOLEAN) {
    let mut pIntro: RESPONSE_REF = 0;
    let mut b0: BYTE = 0;
    let mut BulletinMask: DWORD = 0;
    let mut CrewSold: COUNT = (((uqm_lua_checked!(getGameStateUint(
        b"CREW_SOLD_TO_DRUUGE1\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
        as ::core::ffi::c_int
        | uqm_lua_checked!(getGameStateUint(
            b"CREW_SOLD_TO_DRUUGE0\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE as ::core::ffi::c_int) as COUNT;
    if Repeat.0 != 0 {
        BulletinMask = (CurBulletinMask as ::core::ffi::c_ulong
            ^ 0xffffffff as ::core::ffi::c_ulong) as DWORD;
    } else {
        BulletinMask = uqm_lua_checked!(getGameStateUint(
            b"STARBASE_BULLETS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ());
    }
    pIntro = 0 as RESPONSE_REF;
    b0 = 0 as BYTE;
    while (b0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
        if BulletinMask as ::core::ffi::c_ulong
            & ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong == 0
        {
            let mut pStr: RESPONSE_REF = 0;
            pStr = 0 as RESPONSE_REF;
            match b0 as ::core::ffi::c_int {
                0 => {
                    if CheckAlliance(RACE_ID::SPATHI_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_1.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                1 => {
                    if CheckAlliance(RACE_ID::ZOQFOTPIK_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_2.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                2 => {
                    if CheckAlliance(RACE_ID::SUPOX_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_3.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                3 => {
                    if CheckAlliance(RACE_ID::UTWIG_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_4.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                4 => {
                    if CheckAlliance(RACE_ID::ORZ_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_5.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                5 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"ARILOU_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 2 as DWORD
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if uqm_lua_checked!(getGameStateUint(
                        b"PORTAL_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                        && (Repeat.0 != 0
                            || EscortFeasibilityStudy(RACE_ID::ARILOU_SHIP)
                                as ::core::ffi::c_int != 0)
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_6.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                        if Repeat.0 == 0 {
                            uqm_lua_checked!(AddEscortShips(RACE_ID::ARILOU_SHIP, 3 as SIZE), ());
                        }
                    }
                }
                6 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 1 as DWORD
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_7.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                7 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"MET_MELNORME\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if uqm_lua_checked!(CheckTiming(
                            (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
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
                                3 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as COUNT,
                            0 as COUNT,
                        ), ())
                        .0 != 0
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_8.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                8 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"MET_MELNORME\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if uqm_lua_checked!(CheckTiming(
                            (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
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
                                6 as ::core::ffi::c_int
                            } else {
                                3 as ::core::ffi::c_int
                            }) as COUNT,
                            0 as COUNT,
                        ), ())
                        .0 != 0
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_9.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                9 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if uqm_lua_checked!(CheckTiming(0 as COUNT, 7 as COUNT), ()).0 != 0 {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_10.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                10 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"SPATHI_SHIELDED_SELVES\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_11.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                11 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                        || uqm_lua_checked!(getGameStateUint(
                            b"ZOQFOT_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if uqm_lua_checked!(CheckTiming(
                            0 as COUNT,
                            (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
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
                                42 as ::core::ffi::c_int
                            } else {
                                21 as ::core::ffi::c_int
                            }) as COUNT,
                        ), ())
                        .0 != 0
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_12.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                12 => {
                    if CheckAlliance(RACE_ID::CHMMR_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_13.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                13 => {
                    if CheckAlliance(RACE_ID::SHOFIXTI_SHIP) as ::core::ffi::c_int
                        == C2Rust_Unnamed_4::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_14.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                14 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_15.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                15 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if uqm_lua_checked!(CheckTiming(7 as COUNT, 0 as COUNT), ()).0 != 0 {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_16.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                17 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"YEHAT_ABSORBED_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_18.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                18 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 2 as DWORD
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_19.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                21 => {
                    if uqm_lua_checked!(getGameStateUint(
                        b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 2 as DWORD
                    {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_22.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                26 => {
                    let mut crew_sold: COUNT = CrewSold;
                    if crew_sold as ::core::ffi::c_int
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
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if crew_sold != 0 {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_27.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                27 => {
                    let mut crew_sold_0: COUNT = CrewSold;
                    if crew_sold_0 as ::core::ffi::c_int
                        > if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
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
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if crew_sold_0 as ::core::ffi::c_int
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
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_28.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                28 => {
                    let mut crew_bought: COUNT = 0;
                    crew_bought = (((uqm_lua_checked!(getGameStateUint(
                        b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int
                        | uqm_lua_checked!(getGameStateUint(
                            b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) as BYTE as ::core::ffi::c_int) as COUNT;
                    if crew_bought as ::core::ffi::c_int >= 1000 as ::core::ffi::c_int {
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_29.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                29 => {
                    if CrewSold as ::core::ffi::c_int
                        > if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
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
                        pStr = C2Rust_Unnamed_3::STARBASE_BULLETIN_30.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                16 | 19 | 20 | 22 | 23 | 24 | 25 | 30 | 31 | _ => {}
            }
            if pStr != 0 {
                if pIntro != 0 {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::BETWEEN_BULLETINS.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else if Repeat.0 != 0 {
                    pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                } else {
                    match TFB_Random() as BYTE as ::core::ffi::c_int
                        % 7 as ::core::ffi::c_int
                    {
                        0 => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_1.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        1 => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_2.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        2 => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_3.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        3 => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_4.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        4 => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_5.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        5 => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_6.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        _ => {
                            pIntro = C2Rust_Unnamed_3::BEFORE_WE_GO_ON_7.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                    }
                    uqm_lua_checked!(NPCPhrase_cb(pIntro as ::core::ffi::c_int, None), ());
                }
                uqm_lua_checked!(NPCPhrase_cb(pStr as ::core::ffi::c_int, None), ());
                CurBulletinMask = (CurBulletinMask as ::core::ffi::c_ulong
                    | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                        as ::core::ffi::c_ulong) as DWORD;
            }
        }
        b0 = b0.wrapping_add(1);
    }
    if pIntro as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::RETURN_HELLO.0 as ::core::ffi::c_int, None), ());
    } else if Repeat.0 == 0 {
        uqm_lua_checked!(setGameStateUint(
            b"STARBASE_BULLETS\0".as_ptr() as *const ::core::ffi::c_char,
            BulletinMask,
        ), ());
    }
}
unsafe extern "C" fn NormalStarbase(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_3::no_need_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::OK_NO_NEED_INFO.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::new_devices.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DiscussDevices(BOOLEAN::TRUE), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_3::repeat_bulletins.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(CheckBulletins(BOOLEAN::TRUE), ());
    } else if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if uqm_lua_checked!(getGameStateUint(b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_3::STARBASE_IS_READY.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 0 as SDWORD, 2500 as ::core::ffi::c_int), ());
            if optInfiniteRU.0 != 0 {
                oldRU = 2500 as DWORD;
            }
            uqm_lua_checked!(setGameStateUint(
                b"STARBASE_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
                GlobData.Game_state.GameClock.month_index as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"STARBASE_DAY\0".as_ptr() as *const ::core::ffi::c_char,
                GlobData.Game_state.GameClock.day_index as DWORD,
            ), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(CheckBulletins(BOOLEAN::FALSE), ());
        } else {
            let mut pStr: RESPONSE_REF = 0 as RESPONSE_REF;
            match TFB_Random() as BYTE as ::core::ffi::c_int & 7 as ::core::ffi::c_int {
                0 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_A.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                1 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_B.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                2 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_C.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                3 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_D.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                4 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_E.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                5 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_F.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                6 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_G.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                7 => {
                    pStr = C2Rust_Unnamed_3::NORMAL_HELLO_H.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                _ => {}
            }
            uqm_lua_checked!(NPCPhrase_cb(pStr as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(CheckBulletins(BOOLEAN::FALSE), ());
        }
        uqm_lua_checked!(setGameStateUint(
            b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    }
    if GlobData.SIS_state.TotalElementMass != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::have_minerals.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SellMinerals as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(DiscussDevices(BOOLEAN::FALSE), ()).0 != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::new_devices.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NormalStarbase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if CurBulletinMask != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_3::repeat_bulletins.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NormalStarbase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::need_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_3::goodbye_commander.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn SellMinerals(mut R: RESPONSE_REF) {
    let mut total: COUNT = 0 as COUNT;
    let mut pStr: RESPONSE_REF = 0 as RESPONSE_REF;
    if optSpeech.0 != 0 {
        total = uqm_lua_checked!(DoSellMinerals(), ());
        pStr = uqm_lua_checked!(EveryOnesACritic(total), ());
        uqm_lua_checked!(NPCPhrase_cb(pStr as ::core::ffi::c_int, None), ());
        if pStr as ::core::ffi::c_int
            == C2Rust_Unnamed_3::LIGHT_LOAD_C0.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::LIGHT_LOAD_C1.0 as ::core::ffi::c_int, None), ());
        }
    } else {
        let mut amount: [COUNT; 8] = [0; 8];
        let mut Ru: [COUNT; 8] = [0; 8];
        let mut i: COUNT = 0;
        let mut sseg: COUNT = 0;
        let mut seg: COUNT = 0 as COUNT;
        let mut count: COUNT = 0 as COUNT;
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::CARGO_LIST.0 as ::core::ffi::c_int, None), ());
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int)
            < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
        {
            amount[seg as usize] = GlobData.SIS_state.ElementAmounts[i as usize];
            if amount[seg as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                Ru[seg as usize] = (amount[seg as usize] as ::core::ffi::c_int
                    * GlobData.Game_state.ElementWorth[i as usize] as ::core::ffi::c_int)
                    as COUNT;
                if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_3::ELLIPSES.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                } else {
                    uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::BLANK.0 as ::core::ffi::c_int, None), ());
                }
                uqm_lua_checked!(NPCNumber(
                    amount[seg as usize] as ::core::ffi::c_int,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ), ());
                uqm_lua_checked!(NPCPhrase_splice(C2Rust_Unnamed_3::KILOTONS_OF.0 as ::core::ffi::c_int), ());
                uqm_lua_checked!(NPCPhrase_splice(
                    C2Rust_Unnamed_3::COMMONR.0 as ::core::ffi::c_int
                        + i as ::core::ffi::c_int,
                ), ());
                uqm_lua_checked!(NPCPhrase_splice(C2Rust_Unnamed_3::FOR.0 as ::core::ffi::c_int), ());
                uqm_lua_checked!(NPCNumber(
                    Ru[seg as usize] as ::core::ffi::c_int,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ), ());
                uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::RESUNITS.0 as ::core::ffi::c_int, None), ());
                total = (total as ::core::ffi::c_int
                    + Ru[seg as usize] as ::core::ffi::c_int) as COUNT;
                seg = seg.wrapping_add(1);
                GlobData.SIS_state.ElementAmounts[i as usize] = 0 as COUNT;
                count = count.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        pStr = uqm_lua_checked!(EveryOnesACritic(total), ());
        uqm_lua_checked!(NPCPhrase_cb(pStr as ::core::ffi::c_int, None), ());
        if pStr as ::core::ffi::c_int
            == C2Rust_Unnamed_3::LIGHT_LOAD_C0.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_3::LIGHT_LOAD_C1.0 as ::core::ffi::c_int, None), ());
        }
        sseg = 1 as COUNT;
        while (sseg as ::core::ffi::c_int)
            < seg as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            uqm_lua_checked!(AlienTalkSegue(sseg), ());
            GlobData.SIS_state.TotalElementMass = (GlobData.SIS_state.TotalElementMass
                as ::core::ffi::c_int
                - amount[(sseg as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(DeltaSISGauges(
                0 as SIZE,
                0 as SDWORD,
                Ru[(sseg as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
            ), ());
            uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE), ());
            sseg = sseg.wrapping_add(1);
        }
    }
    uqm_lua_checked!(NormalStarbase(R), ());
}
unsafe extern "C" fn Intro() {
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        CommData.AlienAmbientArray[0usize].BaseFrameRate = (840 as ::core::ffi::c_int
            / 40 as ::core::ffi::c_int) as COUNT;
    }
    uqm_lua_checked!(NormalStarbase(0 as RESPONSE_REF), ());
}
unsafe extern "C" fn uninit_starbase() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_starbase_enc() {
    uqm_lua_checked!(setGameStateUint(
        b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 2 as DWORD
    {
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_starbase_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    commander_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    commander_desc.post_encounter_func = Some(
        post_starbase_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    commander_desc.uninit_encounter_func = Some(
        uninit_starbase as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    commander_desc.AlienTextWidth = ((143 as ::core::ffi::c_int) << resolutionFactor)
        as COUNT;
    commander_desc.AlienTextBaseline.x = (((164 as ::core::ffi::c_int)
        << resolutionFactor)
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) - ((2 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } * 2 as ::core::ffi::c_int) as COORD;
    commander_desc.AlienTextBaseline.y = ((20 as ::core::ffi::c_int) << resolutionFactor)
        as COORD;
    altResFlags = (altResFlags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    if is3DO(optFlagshipColor).0 != 0 {
        altResFlags = (altResFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    }
    CurBulletinMask = 0 as DWORD;
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    retval = &raw mut commander_desc;
    return retval;
}
