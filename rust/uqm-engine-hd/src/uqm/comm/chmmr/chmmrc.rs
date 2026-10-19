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

// Rust translation of src/uqm/comm/chmmr/chmmrc.c.
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
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut oldRU: DWORD;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
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
    fn SetCommIntroMode(_: CommIntroMode, howLong: TimeCount);
    static mut altResFlags: BYTE;
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    static mut plot_map: [PLOT_LOCATION; 70];
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn SetRaceAllied(race: RACE_ID, flag: BOOLEAN) -> BOOLEAN;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
    fn SeedFleetLocation(
        FleetPtr: *mut FLEET_INFO,
        plotmap: *mut PLOT_LOCATION,
        visit: COUNT,
    ) -> POINT;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type HFLEETINFO = HLINK;
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
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CommIntroMode(pub ::core::ffi::c_uint);
impl CommIntroMode {
    pub const CIM_CROSSFADE_SPACE: Self = Self(0);
    pub const CIM_CROSSFADE_WINDOW: Self = Self(1);
    pub const CIM_CROSSFADE_SCREEN: Self = Self(2);
    pub const CIM_FADE_IN_SCREEN: Self = Self(3);
    pub const CIM_DEFAULT: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const WHY_YOU_HERE_1: Self = Self(1);
    pub const WHY_YOU_HERE_2: Self = Self(2);
    pub const WHY_YOU_HERE_3: Self = Self(3);
    pub const WHY_YOU_HERE_4: Self = Self(4);
    pub const find_out_whats_up: Self = Self(5);
    pub const HYBRID_PROCESS: Self = Self(6);
    pub const need_help: Self = Self(7);
    pub const CANT_HELP: Self = Self(8);
    pub const why_no_help: Self = Self(9);
    pub const LONG_TIME: Self = Self(10);
    pub const what_if_more_energy: Self = Self(11);
    pub const DANGER_TO_US: Self = Self(12);
    pub const need_advice: Self = Self(13);
    pub const WHAT_ADVICE: Self = Self(14);
    pub const how_defeat_urquan: Self = Self(15);
    pub const DEFEAT_LIKE_SO: Self = Self(16);
    pub const what_about_tpet: Self = Self(17);
    pub const SCARY_BUT_USEFUL: Self = Self(18);
    pub const what_about_bomb: Self = Self(19);
    pub const ABOUT_BOMB: Self = Self(20);
    pub const what_about_sun_device: Self = Self(21);
    pub const ABOUT_SUN_DEVICE: Self = Self(22);
    pub const what_about_samatra: Self = Self(23);
    pub const ABOUT_SAMATRA: Self = Self(24);
    pub const enough_advice: Self = Self(25);
    pub const OK_ENOUGH_ADVICE: Self = Self(26);
    pub const bye_shielded: Self = Self(27);
    pub const GOODBYE_SHIELDED: Self = Self(28);
    pub const WE_ARE_FREE: Self = Self(29);
    pub const WHO_ARE_YOU: Self = Self(30);
    pub const i_am_captain: Self = Self(31);
    pub const i_am_savior: Self = Self(32);
    pub const i_am_silly: Self = Self(33);
    pub const WHY_HAVE_YOU_FREED_US: Self = Self(34);
    pub const serious_1: Self = Self(35);
    pub const serious_2: Self = Self(36);
    pub const silly: Self = Self(37);
    pub const WILL_HELP_ANALYZE_LOGS: Self = Self(38);
    pub const YOU_KNOW_SAMATRA: Self = Self(39);
    pub const DONT_KNOW_ABOUT_SAMATRA: Self = Self(40);
    pub const NEED_DISTRACTION: Self = Self(41);
    pub const HAVE_TALKING_PET: Self = Self(42);
    pub const NEED_WEAPON: Self = Self(43);
    pub const HAVE_BOMB: Self = Self(44);
    pub const RETURN_WHEN_READY: Self = Self(45);
    pub const YOU_ARE_READY: Self = Self(46);
    pub const further_assistance: Self = Self(47);
    pub const NO_FURTHER_ASSISTANCE: Self = Self(48);
    pub const tech_help: Self = Self(49);
    pub const USE_OUR_SHIPS_BEFORE: Self = Self(50);
    pub const where_weapon: Self = Self(51);
    pub const PRECURSOR_WEAPON: Self = Self(52);
    pub const where_distraction: Self = Self(53);
    pub const PSYCHIC_WEAPONRY: Self = Self(54);
    pub const what_now: Self = Self(55);
    pub const WE_WILL_IMPROVE_BOMB: Self = Self(56);
    pub const MODIFY_VESSEL: Self = Self(57);
    pub const wont_hurt_my_ship: Self = Self(58);
    pub const WILL_DESTROY_IT: Self = Self(59);
    pub const bummer_about_my_ship: Self = Self(60);
    pub const DEAD_SILENCE: Self = Self(61);
    pub const other_assistance: Self = Self(62);
    pub const USE_OUR_SHIPS_AFTER: Self = Self(63);
    pub const proceed: Self = Self(64);
    pub const TAKE_2_WEEKS: Self = Self(65);
    pub const HELLO_AFTER_BOMB_1: Self = Self(66);
    pub const HELLO_AFTER_BOMB_2: Self = Self(67);
    pub const whats_up_after_bomb: Self = Self(68);
    pub const GENERAL_INFO_AFTER_BOMB_1: Self = Self(69);
    pub const GENERAL_INFO_AFTER_BOMB_2: Self = Self(70);
    pub const what_do_after_bomb: Self = Self(71);
    pub const DO_AFTER_BOMB: Self = Self(72);
    pub const bye_after_bomb: Self = Self(73);
    pub const GOODBYE_AFTER_BOMB: Self = Self(74);
    pub const bye: Self = Self(75);
    pub const GOODBYE: Self = Self(76);
    pub const perhaps_not_install: Self = Self(77);
    pub const YES: Self = Self(78);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const ARILOU_DEFINED: Self = Self(0);
    pub const SOL_DEFINED: Self = Self(1);
    pub const SHOFIXTI_DEFINED: Self = Self(2);
    pub const MAIDENS_DEFINED: Self = Self(3);
    pub const START_COLONY_DEFINED: Self = Self(4);
    pub const SPATHI_DEFINED: Self = Self(5);
    pub const ZOQFOT_DEFINED: Self = Self(6);
    pub const MELNORME0_DEFINED: Self = Self(7);
    pub const MELNORME1_DEFINED: Self = Self(8);
    pub const MELNORME2_DEFINED: Self = Self(9);
    pub const MELNORME3_DEFINED: Self = Self(10);
    pub const MELNORME4_DEFINED: Self = Self(11);
    pub const MELNORME5_DEFINED: Self = Self(12);
    pub const MELNORME6_DEFINED: Self = Self(13);
    pub const MELNORME7_DEFINED: Self = Self(14);
    pub const MELNORME8_DEFINED: Self = Self(15);
    pub const TALKING_PET_DEFINED: Self = Self(16);
    pub const CHMMR_DEFINED: Self = Self(17);
    pub const SYREEN_DEFINED: Self = Self(18);
    pub const BURVIXESE_DEFINED: Self = Self(19);
    pub const SLYLANDRO_DEFINED: Self = Self(20);
    pub const DRUUGE_DEFINED: Self = Self(21);
    pub const BOMB_DEFINED: Self = Self(22);
    pub const AQUA_HELIX_DEFINED: Self = Self(23);
    pub const SUN_DEVICE_DEFINED: Self = Self(24);
    pub const TAALO_PROTECTOR_DEFINED: Self = Self(25);
    pub const SHIP_VAULT_DEFINED: Self = Self(26);
    pub const URQUAN_WRECK_DEFINED: Self = Self(27);
    pub const VUX_BEAST_DEFINED: Self = Self(28);
    pub const SAMATRA_DEFINED: Self = Self(29);
    pub const ZOQ_SCOUT_DEFINED: Self = Self(30);
    pub const MYCON_DEFINED: Self = Self(31);
    pub const EGG_CASE0_DEFINED: Self = Self(32);
    pub const EGG_CASE1_DEFINED: Self = Self(33);
    pub const EGG_CASE2_DEFINED: Self = Self(34);
    pub const PKUNK_DEFINED: Self = Self(35);
    pub const UTWIG_DEFINED: Self = Self(36);
    pub const SUPOX_DEFINED: Self = Self(37);
    pub const YEHAT_DEFINED: Self = Self(38);
    pub const VUX_DEFINED: Self = Self(39);
    pub const ORZ_DEFINED: Self = Self(40);
    pub const THRADD_DEFINED: Self = Self(41);
    pub const RAINBOW0_DEFINED: Self = Self(42);
    pub const RAINBOW1_DEFINED: Self = Self(43);
    pub const RAINBOW2_DEFINED: Self = Self(44);
    pub const RAINBOW3_DEFINED: Self = Self(45);
    pub const RAINBOW4_DEFINED: Self = Self(46);
    pub const RAINBOW5_DEFINED: Self = Self(47);
    pub const RAINBOW6_DEFINED: Self = Self(48);
    pub const RAINBOW7_DEFINED: Self = Self(49);
    pub const RAINBOW8_DEFINED: Self = Self(50);
    pub const RAINBOW9_DEFINED: Self = Self(51);
    pub const ILWRATH_DEFINED: Self = Self(52);
    pub const ANDROSYNTH_DEFINED: Self = Self(53);
    pub const MYCON_TRAP_DEFINED: Self = Self(54);
    pub const URQUAN0_DEFINED: Self = Self(55);
    pub const URQUAN1_DEFINED: Self = Self(56);
    pub const URQUAN2_DEFINED: Self = Self(57);
    pub const KOHRAH0_DEFINED: Self = Self(58);
    pub const KOHRAH1_DEFINED: Self = Self(59);
    pub const KOHRAH2_DEFINED: Self = Self(60);
    pub const DESTROYED_STARBASE_DEFINED: Self = Self(61);
    pub const MOTHER_ARK_DEFINED: Self = Self(62);
    pub const ZOQ_COLONY0_DEFINED: Self = Self(63);
    pub const ZOQ_COLONY1_DEFINED: Self = Self(64);
    pub const ZOQ_COLONY2_DEFINED: Self = Self(65);
    pub const ZOQ_COLONY3_DEFINED: Self = Self(66);
    pub const ALGOLITES_DEFINED: Self = Self(67);
    pub const SPATHI_MONUMENT_DEFINED: Self = Self(68);
    pub const EXCAVATION_SITE_DEFINED: Self = Self(69);
    pub const NUM_PLOTS: Self = Self(70);
    pub const WAR_ERA: Self = Self(70);
    pub const SUCCESS: Self = Self(70);
    pub const HOME: Self = Self(71);
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
unsafe extern "C" fn UnlockLink(mut pq: *const QUEUE, mut h: HLINK) {
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
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn universeToLogx(mut ux: COORD) -> SDWORD {
    return (ux as SDWORD
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor))
        + (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn universeToLogy(mut uy: COORD) -> SDWORD {
    return ((9999 as SDWORD - uy as SDWORD)
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor))
        + (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int);
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
static mut chmmr_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::CHMMR_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.chmmr.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.chmmr.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.chmmr.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.chmmr.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: b"comm.chmmr.process.music\0".as_ptr() as RESOURCE,
    },
    ConversationPhrasesRes: b"comm.chmmr.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 6 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 12 as COUNT,
            NumFrames: 5 as BYTE,
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
            StartIndex: 17 as COUNT,
            NumFrames: 5 as BYTE,
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
            NumFrames: 5 as BYTE,
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
            StartIndex: 27 as COUNT,
            NumFrames: 20 as BYTE,
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
            StartIndex: 47 as COUNT,
            NumFrames: 14 as BYTE,
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
            StartIndex: 61 as COUNT,
            NumFrames: 24 as BYTE,
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
        BaseFrameRate: (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as COUNT,
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
static mut chmmr_filters: FILTER_DESC = FILTER_DESC {
    NumFilters: 3 as BYTE,
    FilterArray: [
        FILTER {
            ColorIndex: 1 as BYTE,
            OpacityIndex: 1 as BYTE,
            FrameIndex: -1 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_GRAYSCALE.0 as ::core::ffi::c_int as BYTE,
            Flags: 0 as BYTE,
        },
        FILTER {
            ColorIndex: 0 as BYTE,
            OpacityIndex: 3 as BYTE,
            FrameIndex: -1 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_OVERLAY.0 as ::core::ffi::c_int as BYTE,
            Flags: 0 as BYTE,
        },
        FILTER {
            ColorIndex: 0 as BYTE,
            OpacityIndex: 2 as BYTE,
            FrameIndex: -1 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_ALPHA.0 as ::core::ffi::c_int as BYTE,
            Flags: 0 as BYTE,
        },
    ],
};
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_2::bye.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::GOODBYE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::bye_shielded.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::GOODBYE_SHIELDED.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::bye_after_bomb.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::GOODBYE_AFTER_BOMB.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::proceed.0 as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int = 0;
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::TAKE_2_WEEKS.0 as ::core::ffi::c_int, None), ());
        SetRaceAllied(RACE_ID::CHMMR_SHIP, BOOLEAN::TRUE);
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        if optInfiniteRU.0 == 0 {
            GlobData.SIS_state.ResUnits = 1000000 as ::core::ffi::c_long as DWORD;
        } else {
            oldRU = 1000000 as ::core::ffi::c_long as DWORD;
        }
        GlobData.SIS_state.NumLanders = 0 as BYTE;
        GlobData
            .Game_state
            .ModuleCost[C2Rust_Unnamed_1::PLANET_LANDER.0 as ::core::ffi::c_int
            as usize] = 0 as BYTE;
        GlobData.SIS_state.log_x = universeToLogx(
            plot_map[C2Rust_Unnamed_5::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
                .star_pt
                .x,
        );
        GlobData.SIS_state.log_y = universeToLogy(
            plot_map[C2Rust_Unnamed_5::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
                .star_pt
                .y,
        );
        GlobData.Game_state.ShipFacing = 1 as UWORD;
        GlobData.Game_state.ip_planet = (2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as BYTE;
        GlobData.Game_state.in_orbit = 0 as BYTE;
        GlobData.Game_state.ip_location.x = -725 as COORD;
        GlobData.Game_state.ip_location.y = 597 as COORD;
        if uqm_lua_checked!(getGameStateUint(
            b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            GlobData.SIS_state.FuelOnBoard = (10 as ::core::ffi::c_int
                * 100 as ::core::ffi::c_int) as DWORD;
            GlobData.SIS_state.CrewEnlisted = 0 as COUNT;
            GlobData.SIS_state.TotalElementMass = 0 as COUNT;
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_1::STORAGE_BAY.0 as ::core::ffi::c_int
                as usize] = 0 as BYTE;
            i = 0 as ::core::ffi::c_int;
            while i < C2Rust_Unnamed_0::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int {
                GlobData.SIS_state.ElementAmounts[i as usize] = 0 as COUNT;
                i += 1;
            }
            i = 10 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                GlobData.SIS_state.ModuleSlots[i as usize] = (C2Rust_Unnamed_1::NUM_MODULES
                    .0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
                i += 1;
            }
            GlobData.Game_state.ShipStamp.origin.x = (((CanvasWidth
                - ((64 as SDWORD) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as SDWORD
                } else {
                    0 as SDWORD
                } << resolutionFactor) * 2 as SDWORD
                - (((13 as SDWORD) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as SDWORD
                    } else {
                        0 as SDWORD
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor)
                + (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((35 as ::core::ffi::c_int) << resolutionFactor)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD)
                as COORD;
            GlobData.Game_state.ShipStamp.origin.y = (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as SDWORD
                } else {
                    0 as SDWORD
                } << resolutionFactor) * 2 as SDWORD
                - ((13 as SDWORD) << resolutionFactor) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor)
                + (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((35 as ::core::ffi::c_int) << resolutionFactor
                        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
        } else {
            i = 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            while i > 0 as ::core::ffi::c_int {
                let mut m: ::core::ffi::c_int = 0;
                while i > 0 as ::core::ffi::c_int
                    && GlobData.SIS_state.ModuleSlots[i as usize] as ::core::ffi::c_int
                        != C2Rust_Unnamed_1::NUM_MODULES.0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                {
                    i -= 1;
                }
                if i == 0 as ::core::ffi::c_int {
                    break;
                }
                m = i - 1 as ::core::ffi::c_int;
                while m >= 0 as ::core::ffi::c_int
                    && GlobData.SIS_state.ModuleSlots[m as usize] as ::core::ffi::c_int
                        == C2Rust_Unnamed_1::NUM_MODULES.0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                {
                    m -= 1;
                }
                if m < 0 as ::core::ffi::c_int {
                    break;
                }
                GlobData.SIS_state.ModuleSlots[i as usize] = GlobData
                    .SIS_state
                    .ModuleSlots[m as usize];
                GlobData.SIS_state.ModuleSlots[m as usize] = (C2Rust_Unnamed_1::NUM_MODULES
                    .0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
                i -= 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < 11 as ::core::ffi::c_int {
            GlobData.SIS_state.DriveSlots[i as usize] = C2Rust_Unnamed_1::FUSION_THRUSTER
                .0 as ::core::ffi::c_int as BYTE;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            GlobData.SIS_state.JetSlots[i as usize] = C2Rust_Unnamed_1::TURNING_JETS.0
                as ::core::ffi::c_int as BYTE;
            i += 1;
        }
        GlobData.SIS_state.ModuleSlots[0usize] = C2Rust_Unnamed_1::BOMB_MODULE_4.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[1usize] = C2Rust_Unnamed_1::BOMB_MODULE_5.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[2usize] = C2Rust_Unnamed_1::BOMB_MODULE_3.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[3usize] = C2Rust_Unnamed_1::BOMB_MODULE_1.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[4usize] = C2Rust_Unnamed_1::BOMB_MODULE_0.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[5usize] = C2Rust_Unnamed_1::BOMB_MODULE_1.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[6usize] = C2Rust_Unnamed_1::BOMB_MODULE_3.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[7usize] = C2Rust_Unnamed_1::BOMB_MODULE_4.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[8usize] = C2Rust_Unnamed_1::BOMB_MODULE_5.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.ModuleSlots[9usize] = C2Rust_Unnamed_1::BOMB_MODULE_2.0
            as ::core::ffi::c_int as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::perhaps_not_install.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::YES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::GOODBYE.0 as ::core::ffi::c_int, None), ());
    }
}
unsafe extern "C" fn NotReady(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::RETURN_WHEN_READY.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::further_assistance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_2::NO_FURTHER_ASSISTANCE.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::further_assistance.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::tech_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_2::USE_OUR_SHIPS_BEFORE.0 as ::core::ffi::c_int,
            None,
        ), ());
        SetRaceAllied(RACE_ID::CHMMR_SHIP, BOOLEAN::TRUE);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::where_weapon.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::PRECURSOR_WEAPON.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::where_weapon.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::where_distraction.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::PSYCHIC_WEAPONRY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::where_distraction.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if CheckAlliance(RACE_ID::CHMMR_SHIP) as ::core::ffi::c_int
        != C2Rust_Unnamed_3::GOOD_GUY.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::tech_help.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NotReady as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::further_assistance.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::further_assistance.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NotReady as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::where_weapon.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::where_weapon.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NotReady as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::where_distraction.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::where_distraction.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NotReady as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_2::bye.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ImproveBomb(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_2::WE_WILL_IMPROVE_BOMB.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::MODIFY_VESSEL.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_now.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::wont_hurt_my_ship.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::WILL_DESTROY_IT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::wont_hurt_my_ship.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::bummer_about_my_ship.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::DEAD_SILENCE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::bummer_about_my_ship.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::other_assistance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_2::USE_OUR_SHIPS_AFTER.0 as ::core::ffi::c_int,
            None,
        ), ());
        SetRaceAllied(RACE_ID::CHMMR_SHIP, BOOLEAN::TRUE);
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_now.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::what_now.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ImproveBomb as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::wont_hurt_my_ship.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::wont_hurt_my_ship.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ImproveBomb as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::bummer_about_my_ship.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::bummer_about_my_ship.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ImproveBomb as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if CheckAlliance(RACE_ID::CHMMR_SHIP) as ::core::ffi::c_int
        != C2Rust_Unnamed_3::GOOD_GUY.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::other_assistance.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ImproveBomb as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_2::proceed.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_2::perhaps_not_install.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn ChmmrFree(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::i_am_captain.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::i_am_savior.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::i_am_silly.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_2::WHY_HAVE_YOU_FREED_US.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_EMERGING\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::serious_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrFree as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::serious_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrFree as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::silly.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrFree as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_2::WILL_HELP_ANALYZE_LOGS.0 as ::core::ffi::c_int,
            None,
        ), ());
        if uqm_lua_checked!(getGameStateUint(b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_2::YOU_KNOW_SAMATRA.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_2::DONT_KNOW_ABOUT_SAMATRA.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_2::HAVE_TALKING_PET.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_2::NEED_DISTRACTION.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::HAVE_BOMB.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::NEED_WEAPON.0 as ::core::ffi::c_int, None), ());
        }
        if uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
            || uqm_lua_checked!(getGameStateUint(
                b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(NotReady(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        } else {
            uqm_lua_checked!(ImproveBomb(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        }
    };
}
unsafe extern "C" fn ChmmrAdvice(mut R: RESPONSE_REF) {
    let mut AdviceLeft: BYTE = 0;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_2::need_advice.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::WHAT_ADVICE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::how_defeat_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::DEFEAT_LIKE_SO.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::how_defeat_urquan.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_tpet.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::SCARY_BUT_USEFUL.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_about_tpet.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_bomb.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::ABOUT_BOMB.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_about_bomb.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_sun_device.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::ABOUT_SUN_DEVICE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_about_sun_device.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_samatra.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::ABOUT_SAMATRA.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_about_samatra.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    AdviceLeft = 0 as BYTE;
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::how_defeat_urquan.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::how_defeat_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrAdvice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        AdviceLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_about_tpet.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::what_about_tpet.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrAdvice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        AdviceLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_about_bomb.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::what_about_bomb.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrAdvice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        AdviceLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_about_sun_device.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::what_about_sun_device.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ChmmrAdvice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        AdviceLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_about_samatra.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::what_about_samatra.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrAdvice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        AdviceLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_2::enough_advice.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ChmmrShielded as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if AdviceLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::need_advice.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn ChmmrShielded(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::find_out_whats_up.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::HYBRID_PROCESS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::find_out_whats_up.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::need_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::CANT_HELP.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::why_no_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::LONG_TIME.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_if_more_energy.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::DANGER_TO_US.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::enough_advice.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::OK_ENOUGH_ADVICE.0 as ::core::ffi::c_int, None), ());
    }
    match uqm_lua_checked!(getGameStateUint(b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_2::need_help.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ChmmrShielded as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        1 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_2::why_no_help.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ChmmrShielded as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        2 => {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_2::what_if_more_energy.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ChmmrShielded as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        _ => {}
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::find_out_whats_up.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::find_out_whats_up.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrShielded as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::need_advice.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::need_advice.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ChmmrAdvice as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_2::bye_shielded.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn AfterBomb(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::whats_up_after_bomb.0 as ::core::ffi::c_int
    {
        if uqm_lua_checked!(getGameStateUint(b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_2::GENERAL_INFO_AFTER_BOMB_2.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_2::GENERAL_INFO_AFTER_BOMB_1.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        }
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::whats_up_after_bomb.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_do_after_bomb.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::DO_AFTER_BOMB.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_do_after_bomb.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::whats_up_after_bomb.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::whats_up_after_bomb.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(AfterBomb as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_do_after_bomb.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_2::what_do_after_bomb.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AfterBomb as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_2::bye_after_bomb.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
        >= 2 as DWORD
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_AFTER_BOMB_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_AFTER_BOMB_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(AfterBomb(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"CHMMR_UNLEASHED\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        if uqm_lua_checked!(getGameStateUint(
            b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
            || uqm_lua_checked!(getGameStateUint(
                b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0
        {
            uqm_lua_checked!(NotReady(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::YOU_ARE_READY.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(ImproveBomb(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        }
    } else {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        if uqm_lua_checked!(getGameStateUint(b"CHMMR_EMERGING\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0
        {
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                EngageFilters(&raw mut chmmr_filters);
            }
            CommData.AlienColorMap = SetAbsStringTableIndex(
                CommData.AlienColorMap,
                1 as COUNT,
            ) as COLORMAP;
            let c2rust_fresh1 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh1 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_2::WHY_YOU_HERE_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_2::WHY_YOU_HERE_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_2::WHY_YOU_HERE_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_2::WHY_YOU_HERE_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(ChmmrShielded(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        } else {
            SetCommIntroMode(
                CommIntroMode::CIM_FADE_IN_SCREEN,
                (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as TimeCount,
            );
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_2::WE_ARE_FREE.0 as ::core::ffi::c_int, None), ());
            if NumVisits != 0 {
                uqm_lua_checked!(ChmmrFree(0 as ::core::ffi::c_int as RESPONSE_REF), ());
                NumVisits = 0 as BYTE;
            } else {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_2::WHO_ARE_YOU.0 as ::core::ffi::c_int,
                    None,
                ), ());
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_2::i_am_captain.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(ChmmrFree as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_2::i_am_savior.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(ChmmrFree as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_2::i_am_silly.0 as ::core::ffi::c_int as RESPONSE_REF,
                    Some(ChmmrFree as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            uqm_lua_checked!(setGameStateUint(
                b"CHMMR_UNLEASHED\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                let mut hChmmr: HFLEETINFO = GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    RACE_ID::CHMMR_SHIP.0 as ::core::ffi::c_int as COUNT,
                );
                let mut ChmmrPtr: *mut FLEET_INFO = LockFleetInfo(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hChmmr,
                );
                if !ChmmrPtr.is_null() {
                    (*ChmmrPtr).actual_strength = (986 as ::core::ffi::c_int
                        / 11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COUNT;
                    (*ChmmrPtr).loc = SeedFleetLocation(
                        ChmmrPtr,
                        &raw mut plot_map as *mut PLOT_LOCATION,
                        C2Rust_Unnamed_5::HOME.0 as ::core::ffi::c_int as COUNT,
                    );
                    StartSphereTracking(RACE_ID::CHMMR_SHIP);
                }
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hChmmr);
            }
        }
        uqm_lua_checked!(setGameStateUint(
            b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(SetHomeworldKnown(C2Rust_Unnamed_4::CHMMR_HOME.0 as ::core::ffi::c_int as DWORD), ());
    };
}
unsafe extern "C" fn uninit_chmmr() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_chmmr_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_chmmr_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    chmmr_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    chmmr_desc.post_encounter_func = Some(post_chmmr_enc as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    chmmr_desc.uninit_encounter_func = Some(
        uninit_chmmr as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_UNLEASHED\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut()) != 0
        || uqm_lua_checked!(getGameStateUint(b"CHMMR_EMERGING\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
            != 0
    {
        altResFlags = (altResFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    chmmr_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
    chmmr_desc.AlienTextBaseline.y = 0 as COORD;
    chmmr_desc.AlienTextWidth = (CanvasWidth
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
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    retval = &raw mut chmmr_desc;
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
