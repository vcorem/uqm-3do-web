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

// Rust translation of src/uqm/globdata.c.
extern "C" {
    pub type _IO_FILE;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type RandomContext;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn InitQueue(pq: *mut QUEUE, num_elements: COUNT, size: OBJ_SIZE) -> BOOLEAN;
    fn UninitQueue(pq: *mut QUEUE) -> BOOLEAN;
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringPos(
        pStr: *const ::core::ffi::c_char,
        ch: UniChar,
    ) -> ::core::ffi::c_int;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn rand() -> ::core::ffi::c_int;
    fn srand(_: ::core::ffi::c_uint);
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn clock() -> clock_t;
    fn time(_: *mut time_t) -> time_t;
    static mut InPopUp: BOOLEAN;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optUnlockShips: OPT_ENABLABLE;
    static mut optHeadStart: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optDifficulty: ::core::ffi::c_int;
    static mut optExtended: OPT_ENABLABLE;
    static mut optNomad: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn InitSISContexts();
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut DisplayArray: [PRIMITIVE; 330];
    static mut CommData: LOCDATA;
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Delete(context: *mut RandomContext);
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: [STAR_DESC; 0];
    static mut plot_map: [PLOT_LOCATION; 70];
    static mut portal_map: [PORTAL_LOCATION; 16];
    static mut StarGenRNG: *mut RandomContext;
    fn DefaultStarmap(starmap: *mut STAR_DESC);
    fn SeedStarmap(starmap: *mut STAR_DESC);
    fn DefaultPlot(plot: *mut PLOT_LOCATION, starmap: *mut STAR_DESC);
    fn InitMelnormeRainbow(plotmap: *mut PLOT_LOCATION);
    fn InitPlot(plotmap: *mut PLOT_LOCATION);
    fn SeedPlot(plotmap: *mut PLOT_LOCATION, starmap: *mut STAR_DESC) -> COUNT;
    fn DefaultQuasispace(portalmap: *mut PORTAL_LOCATION);
    fn SeedQuasispace(
        portalmap: *mut PORTAL_LOCATION,
        plotmap: *mut PLOT_LOCATION,
        starmap: *mut STAR_DESC,
    ) -> BOOLEAN;
    static mut master_q: QUEUE;
    fn ReloadMasterShipList(YieldProcessing: Option<unsafe extern "C" fn() -> ()>);
    fn FindMasterShip(ship_ref: SPECIES_ID) -> HMASTERSHIP;
    fn InitPlanetInfo();
    fn UninitPlanetInfo();
    fn InitGroupInfo(FirstTime: BOOLEAN);
    fn UninitGroupInfo();
    static mut GameStrings: STRING;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn SetEscortCrewComplement(
        which_ship: RACE_ID,
        crew_level: COUNT,
        captain: BYTE,
    ) -> ::core::ffi::c_int;
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn SetRaceAllied(race: RACE_ID, flag: BOOLEAN) -> BOOLEAN;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn load_ship(SpeciesID: SPECIES_ID, LoadBattleData: BOOLEAN) -> *mut RACE_DESC;
    fn free_ship(
        RaceDescPtr: *mut RACE_DESC,
        FreeIconData: BOOLEAN,
        FreeBattleData: BOOLEAN,
    );
    fn loadGameCheats();
    fn SeedFleet(FleetPtr: *mut FLEET_INFO, plotmap: *mut PLOT_LOCATION);
    static mut Screen: FRAME;
    static mut FlagStatFrame: FRAME;
    static mut MiscDataFrame: FRAME;
    static mut visitedStarsFrame: FRAME;
    static mut FontGradFrame: FRAME;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut planetSideDesc: *mut PLANETSIDE_DESC;
    static mut SaveLoadActive: BOOLEAN;
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
}
pub type time_t = ::core::ffi::c_longlong;
pub type size_t = ::core::ffi::c_ulong;
pub type clock_t = ::core::ffi::c_int;
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
pub type OBJ_SIZE = UWORD;
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
pub type FILE = _IO_FILE;
pub type RESOURCE = *const ::core::ffi::c_char;
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type SOUND = STRING;
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
    pub const OPTVAL_NOM_OFF: Self = Self(0);
    pub const OPTVAL_NOM_EASY: Self = Self(1);
    pub const OPTVAL_NOM_NORM: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameStateBitMap {
    pub name: *const ::core::ffi::c_char,
    pub numBits: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
pub type PORTAL_LOCATION = portal_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portal_location {
    pub star_pt: POINT,
    pub quasi_pt: POINT,
    pub nearest_star: *mut STAR_DESC,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
    pub const MENU_SOUND_BUBBLEWARP: Self = Self(4);
    pub const MENU_SOUND_TEXT: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANETSIDE_DESC {
    pub InTransit: BOOLEAN,
    pub ElementLevel: COUNT,
    pub MaxElementLevel: COUNT,
    pub BiologicalLevel: COUNT,
    pub ElementAmounts: [COUNT; 8],
    pub NumFrames: COUNT,
    pub AmountBuf: [UNICODE; 40],
    pub MineralText: [TEXT; 3],
    pub ColorCycle: [Color; 16],
    pub TectonicsChance: BYTE,
    pub WeatherChance: BYTE,
    pub FireChance: BYTE,
    pub NodeData: NODE_DATA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NODE_DATA {
    pub NumNodesGrabbed: BYTE,
    pub NodeType: [BYTE; 32],
    pub NodeAmounts: [COUNT; 32],
}
pub type HFLEETINFO = HLINK;
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
pub type RACE_DESC = race_desc;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct race_desc {
    pub ship_info: SHIP_INFO,
    pub c2rust_padding: [u8; 8],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub c2rust_padding_2: [u8; 8],
    pub cyborg_control: INTEL_STUFF,
    pub uninit_func: Option<UNINIT_FUNC>,
    pub preprocess_func: Option<PREPROCESS_FUNC>,
    pub postprocess_func: Option<POSTPROCESS_FUNC>,
    pub init_weapon_func: Option<INIT_WEAPON_FUNC>,
    pub data: *mut ::core::ffi::c_void,
    pub CodeRef: *mut ::core::ffi::c_void,
}
pub type INIT_WEAPON_FUNC = unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT;
pub type HELEMENT = HLINK;
pub type ELEMENT = element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element {
    pub pred: HELEMENT,
    pub succ: HELEMENT,
    pub preprocess_func: Option<ElementProcessFunc>,
    pub postprocess_func: Option<ElementProcessFunc>,
    pub collision_func: Option<ElementCollisionFunc>,
    pub death_func: Option<ElementProcessFunc>,
    pub playerNr: SIZE,
    pub state_flags: ELEMENT_FLAGS,
    pub c2rust_unnamed: C2Rust_Unnamed_9,
    pub c2rust_unnamed_0: C2Rust_Unnamed_8,
    pub c2rust_unnamed_1: C2Rust_Unnamed_7,
    pub c2rust_unnamed_2: C2Rust_Unnamed_6,
    pub c2rust_unnamed_3: C2Rust_Unnamed_5,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ELEMENT_FLAGS = UWORD;
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
pub type POSTPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type PREPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type UNINIT_FUNC = unsafe extern "C" fn(*mut RACE_DESC) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTEL_STUFF {
    pub ManeuverabilityIndex: COUNT,
    pub WeaponRange: COUNT,
    pub intelligence_func: Option<IntelligenceFunc>,
}
pub type IntelligenceFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut EVALUATE_DESC,
    COUNT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EVALUATE_DESC {
    pub ObjectPtr: *mut ELEMENT,
    pub facing: COUNT,
    pub which_turn: COUNT,
    pub MoveState: MOVEMENT_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MOVEMENT_STATE(pub ::core::ffi::c_uint);
impl MOVEMENT_STATE {
    pub const PURSUE: Self = Self(0);
    pub const AVOID: Self = Self(1);
    pub const ENTICE: Self = Self(2);
    pub const NO_MOVEMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATA_STUFF {
    pub ship_rsc: [RESOURCE; 3],
    pub weapon_rsc: [RESOURCE; 3],
    pub special_rsc: [RESOURCE; 3],
    pub captain_control: CAPTAIN_STUFF,
    pub victory_ditty_rsc: RESOURCE,
    pub ship_sounds_rsc: RESOURCE,
    pub ship: [FRAME; 3],
    pub weapon: [FRAME; 3],
    pub special: [FRAME; 3],
    pub victory_ditty: MUSIC_REF,
    pub ship_sounds: SOUND,
}
pub type CAPTAIN_STUFF = captain_stuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct captain_stuff {
    pub captain_rsc: RESOURCE,
    pub background: FRAME,
    pub turn: FRAME,
    pub thrust: FRAME,
    pub weapon: FRAME,
    pub special: FRAME,
    pub tl_offset: BYTE,
    pub tr_offset: BYTE,
    pub thrust_offset: BYTE,
    pub weapon_offset: BYTE,
    pub special_offset: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHARACTERISTIC_STUFF {
    pub max_thrust: COUNT,
    pub thrust_increment: COUNT,
    pub energy_regeneration: BYTE,
    pub weapon_energy_cost: BYTE,
    pub special_energy_cost: BYTE,
    pub energy_wait: BYTE,
    pub turn_wait: BYTE,
    pub thrust_wait: BYTE,
    pub weapon_wait: BYTE,
    pub special_wait: BYTE,
    pub ship_mass: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_INFO {
    pub idStr: *const ::core::ffi::c_char,
    pub ship_flags: UWORD,
    pub ship_cost: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings_rsc: RESOURCE,
    pub icons_rsc: RESOURCE,
    pub melee_icon_rsc: RESOURCE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
pub type HMASTERSHIP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
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
pub type HSHIPFRAG = HLINK;
pub type ENCOUNTER = encounter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encounter {
    pub pred: HENCOUNTER,
    pub succ: HENCOUNTER,
    pub hElement: HELEMENT,
    pub transition_state: SIZE,
    pub origin: POINT,
    pub radius: COUNT,
    pub race_id: BYTE,
    pub num_ships: BYTE,
    pub flags: BYTE,
    pub loc_pt: POINT,
    pub ShipList: [BRIEF_SHIP_INFO; 7],
    pub log_x: SDWORD,
    pub log_y: SDWORD,
}
pub type BRIEF_SHIP_INFO = brief_ship_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brief_ship_info {
    pub race_id: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub max_energy: BYTE,
}
pub type HENCOUNTER = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IP_GROUP {
    pub pred: HIPGROUP,
    pub succ: HIPGROUP,
    pub group_counter: UWORD,
    pub race_id: BYTE,
    pub sys_loc: BYTE,
    pub task: BYTE,
    pub in_system: BYTE,
    pub dest_loc: BYTE,
    pub orbit_pos: BYTE,
    pub group_id: BYTE,
    pub loc: POINT,
    pub melee_icon: FRAME,
    pub flags: IP_FLAGS,
}
pub type IP_FLAGS = BYTE;
pub type HIPGROUP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_FRAGMENT {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub race_id: BYTE,
    pub index: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
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
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_13 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(149);
    pub const CARGO_STRING_BASE: Self = Self(178);
    pub const ELEMENTS_STRING_BASE: Self = Self(192);
    pub const SCAN_STRING_BASE: Self = Self(325);
    pub const STAR_NUMBER_BASE: Self = Self(391);
    pub const PLANET_NUMBER_BASE: Self = Self(405);
    pub const MONTHS_STRING_BASE: Self = Self(448);
    pub const FEEDBACK_STRING_BASE: Self = Self(460);
    pub const STARBASE_STRING_BASE: Self = Self(468);
    pub const ENCOUNTER_STRING_BASE: Self = Self(510);
    pub const NAVIGATION_STRING_BASE: Self = Self(518);
    pub const NAMING_STRING_BASE: Self = Self(527);
    pub const MELEE_STRING_BASE: Self = Self(535);
    pub const SAVEGAME_STRING_BASE: Self = Self(560);
    pub const OPTION_STRING_BASE: Self = Self(570);
    pub const QUITMENU_STRING_BASE: Self = Self(580);
    pub const STATUS_STRING_BASE: Self = Self(585);
    pub const FLAGSHIP_STRING_BASE: Self = Self(607);
    pub const ORBITSCAN_STRING_BASE: Self = Self(620);
    pub const MAINMENU_STRING_BASE: Self = Self(639);
    pub const NETMELEE_STRING_BASE: Self = Self(713);
    pub const BIOLOGICAL_STRING_BASE: Self = Self(747);
    pub const PLAYMENU_STRING_BASE: Self = Self(773);
    pub const LABEL_STRING_BASE: Self = Self(803);
    pub const GAMESTR_COUNT: Self = Self(829);
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
                    b"src/uqm/displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
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
                    b"src/uqm/displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn ZeroLastLoc() {
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn ZeroAdvancedQuasiPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn ZeroAdvancedAutoPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
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
#[no_mangle]
pub static mut RadarContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut PlayFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut GlobData: GLOBDATA = GLOBDATA {
    SIS_state: SIS_STATE {
        log_x: 0,
        log_y: 0,
        ResUnits: 0,
        FuelOnBoard: 0,
        CrewEnlisted: 0,
        TotalElementMass: 0,
        TotalBioMass: 0,
        ModuleSlots: [0; 16],
        DriveSlots: [0; 11],
        JetSlots: [0; 8],
        NumLanders: 0,
        ElementAmounts: [0; 8],
        ShipName: [0; 32],
        CommanderName: [0; 32],
        PlanetName: [0; 32],
        Difficulty: 0,
        Extended: 0,
        Nomad: 0,
        ShipSeed: 0,
        Seed: 0,
        SaveVersion: 0,
    },
    Game_state: GAME_STATE {
        glob_flags: 0,
        CrewCost: 0,
        FuelCost: 0,
        ModuleCost: [0; 20],
        ElementWorth: [0; 8],
        DisplayArray: ::core::ptr::null_mut::<PRIMITIVE>(),
        CurrentActivity: 0,
        GameClock: CLOCK_STATE {
            day_index: 0,
            month_index: 0,
            year_index: 0,
            tick_count: 0,
            day_in_ticks: 0,
            event_q: QUEUE {
                head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pq_tab: ::core::ptr::null_mut::<BYTE>(),
                free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                object_size: 0,
                num_objects: 0,
            },
        },
        autopilot: POINT { x: 0, y: 0 },
        ip_location: POINT { x: 0, y: 0 },
        ShipStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        ShipFacing: 0,
        ip_planet: 0,
        in_orbit: 0,
        velocity: VELOCITY_DESC {
            TravelAngle: 0,
            vector: EXTENT { width: 0, height: 0 },
            fract: EXTENT { width: 0, height: 0 },
            error: EXTENT { width: 0, height: 0 },
            incr: EXTENT { width: 0, height: 0 },
        },
        BattleGroupRef: 0,
        avail_race_q: QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        },
        npc_built_ship_q: QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        },
        ip_group_q: QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        },
        encounter_q: QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        },
        built_ship_q: QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        },
        stowed_ship_q: QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        },
    },
};
#[inline]
unsafe extern "C" fn bitmask32(mut bits: BYTE) -> DWORD {
    return if bits as ::core::ffi::c_int >= 32 as ::core::ffi::c_int {
        0xffffffff as DWORD
    } else {
        ((1 as DWORD) << bits as ::core::ffi::c_int).wrapping_sub(1 as DWORD)
    };
}
#[inline]
unsafe extern "C" fn shl32(mut value: DWORD, mut shift: BYTE) -> DWORD {
    return if shift as ::core::ffi::c_int >= 32 as ::core::ffi::c_int {
        0 as DWORD
    } else {
        value << shift as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn totalBitsForGameState(
    mut bm: *const GameStateBitMap,
    mut rev: ::core::ffi::c_int,
) -> size_t {
    let mut totalBits: size_t = 0 as size_t;
    let mut bmPtr: *const GameStateBitMap = ::core::ptr::null::<GameStateBitMap>();
    bmPtr = bm;
    while !bmPtr.is_null() {
        if !(*bmPtr).name.is_null() {
            totalBits = totalBits.wrapping_add((*bmPtr).numBits as size_t);
        } else {
            if (*bmPtr).numBits as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            if rev >= 0 as ::core::ffi::c_int
                && (*bmPtr).numBits as ::core::ffi::c_int > rev
            {
                break;
            }
        }
        bmPtr = bmPtr.offset(1);
    }
    return totalBits;
}
#[no_mangle]
pub unsafe extern "C" fn getGameStateRevByBytes(
    mut bm: *const GameStateBitMap,
    mut bytes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut totalBits: size_t = 0 as size_t;
    let mut bmPtr: *const GameStateBitMap = ::core::ptr::null::<GameStateBitMap>();
    bmPtr = bm;
    while !bmPtr.is_null() {
        if !(*bmPtr).name.is_null() {
            totalBits = totalBits.wrapping_add((*bmPtr).numBits as size_t);
        } else {
            if (*bmPtr).numBits as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            if (totalBits.wrapping_add(7 as size_t) >> 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int >= bytes
            {
                break;
            }
            rev = (*bmPtr).numBits as ::core::ffi::c_int;
        }
        bmPtr = bmPtr.offset(1);
    }
    if (totalBits.wrapping_add(7 as size_t) >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_int != bytes
    {
        rev = -1 as ::core::ffi::c_int;
    }
    return rev;
}
#[inline]
unsafe extern "C" fn serialiseBits(
    mut bufPtrPtr: *mut *mut BYTE,
    mut restBitsPtr: *mut DWORD,
    mut restBitCount: *mut size_t,
    mut value: BYTE,
    mut valueBitCount: size_t,
) {
    let mut valueBitMask: BYTE = 0;
    (*restBitCount < 8 as size_t
        || {
            __assert_fail(
                b"*restBitCount < 8\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/globdata.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int,
                b"serialiseBits\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (valueBitCount <= 8 as size_t
        || {
            __assert_fail(
                b"valueBitCount <= 8\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/globdata.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int,
                b"serialiseBits\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    valueBitMask = (((1 as ::core::ffi::c_int) << valueBitCount)
        - 1 as ::core::ffi::c_int) as BYTE;
    *restBitsPtr
        |= ((value as ::core::ffi::c_int & valueBitMask as ::core::ffi::c_int)
            << *restBitCount) as DWORD;
    *restBitCount = (*restBitCount).wrapping_add(valueBitCount);
    if *restBitCount >= 8 as size_t {
        **bufPtrPtr = (*restBitsPtr & 0xff as DWORD) as BYTE;
        *restBitsPtr >>= 8 as ::core::ffi::c_int;
        *bufPtrPtr = (*bufPtrPtr).offset(1);
        *restBitCount = (*restBitCount).wrapping_sub(8 as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn serialiseGameState(
    mut bm: *const GameStateBitMap,
    mut buf: *mut *mut BYTE,
    mut numBytes: *mut size_t,
) -> BOOLEAN {
    let mut totalBits: size_t = 0;
    let mut totalBytes: size_t = 0;
    let mut bmPtr: *const GameStateBitMap = ::core::ptr::null::<GameStateBitMap>();
    let mut result: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut bufPtr: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut restBits: DWORD = 0 as DWORD;
    let mut restBitCount: size_t = 0 as size_t;
    totalBits = totalBitsForGameState(bm, -1 as ::core::ffi::c_int);
    totalBytes = totalBits.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
    result = HMalloc(totalBytes) as *mut BYTE;
    if result.is_null() {
        return BOOLEAN::FALSE;
    }
    bufPtr = result;
    bmPtr = bm;
    while !bmPtr.is_null() {
        if !(*bmPtr).name.is_null() {
            let mut value: DWORD = uqm_lua_checked!(getGameStateUint((*bmPtr).name), BOOLEAN(0));
            let mut numBits: BYTE = (*bmPtr).numBits;
            if value > bitmask32(numBits) {
                log_add(
                    log_Level::log_Error,
                    b"Warning: serialiseGameState(): the value of the property '%s' (%u) does not fit in the reserved number of bits (%d).\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*bmPtr).name,
                    value,
                    numBits as ::core::ffi::c_int,
                );
            }
            while (numBits as ::core::ffi::c_int) >= 8 as ::core::ffi::c_int {
                serialiseBits(
                    &raw mut bufPtr,
                    &raw mut restBits,
                    &raw mut restBitCount,
                    (value & 0xff as DWORD) as BYTE,
                    8 as size_t,
                );
                value >>= 8 as ::core::ffi::c_int;
                numBits = (numBits as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    as BYTE;
            }
            if numBits as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                serialiseBits(
                    &raw mut bufPtr,
                    &raw mut restBits,
                    &raw mut restBitCount,
                    value as BYTE,
                    numBits as size_t,
                );
            }
        } else if (*bmPtr).numBits as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        bmPtr = bmPtr.offset(1);
    }
    if restBitCount > 0 as size_t {
        serialiseBits(
            &raw mut bufPtr,
            &raw mut restBits,
            &raw mut restBitCount,
            0 as BYTE,
            (8 as size_t).wrapping_sub(restBitCount),
        );
    }
    *buf = result;
    *numBytes = totalBytes;
    return BOOLEAN::TRUE;
}
#[inline]
unsafe extern "C" fn deserialiseBits(
    mut bytePtr: *mut *const BYTE,
    mut bitPtr: *mut BYTE,
    mut numBits: size_t,
) -> DWORD {
    ((*bitPtr as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"*bitPtr < 8\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/globdata.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                240 as ::core::ffi::c_int,
                b"deserialiseBits\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (numBits <= 8 as size_t
        || {
            __assert_fail(
                b"numBits <= 8\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/globdata.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                241 as ::core::ffi::c_int,
                b"deserialiseBits\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if numBits <= (8 as ::core::ffi::c_int - *bitPtr as ::core::ffi::c_int) as size_t {
        let mut result: DWORD = (*(*bytePtr).offset(0isize) as ::core::ffi::c_int
            >> *bitPtr as ::core::ffi::c_int) as DWORD & bitmask32(numBits as BYTE);
        if numBits == (8 as ::core::ffi::c_int - *bitPtr as ::core::ffi::c_int) as size_t
        {
            *bytePtr = (*bytePtr).offset(1);
            *bitPtr = 0 as BYTE;
        } else {
            *bitPtr = (*bitPtr as ::core::ffi::c_int
                + numBits as BYTE as ::core::ffi::c_int) as BYTE;
        }
        return result;
    } else {
        let mut result_0: DWORD = (*(*bytePtr).offset(0isize) as ::core::ffi::c_int
            >> *bitPtr as ::core::ffi::c_int
            | (*(*bytePtr).offset(1isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - *bitPtr as ::core::ffi::c_int) as DWORD
            & bitmask32(numBits as BYTE);
        *bytePtr = (*bytePtr).offset(1);
        *bitPtr = (*bitPtr as ::core::ffi::c_int
            + (numBits as BYTE as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as BYTE;
        return result_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn deserialiseGameState(
    mut bm: *const GameStateBitMap,
    mut buf: *const BYTE,
    mut numBytes: size_t,
    mut rev: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut totalBits: size_t = 0;
    let mut bmPtr: *const GameStateBitMap = ::core::ptr::null::<GameStateBitMap>();
    let mut bytePtr: *const BYTE = buf;
    let mut bitPtr: BYTE = 0 as BYTE;
    let mut matchRev: BOOLEAN = BOOLEAN::TRUE;
    totalBits = totalBitsForGameState(bm, rev);
    if numBytes.wrapping_mul(8 as size_t) < totalBits {
        log_add(
            log_Level::log_Error,
            b"Warning: deserialiseGameState(): Corrupt save game: state: less bytes available than expected.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    bmPtr = bm;
    while !bmPtr.is_null() {
        if !(*bmPtr).name.is_null() {
            let mut value: DWORD = 0 as DWORD;
            let mut numBits: BYTE = (*bmPtr).numBits;
            let mut bitsLeft: BYTE = numBits;
            if matchRev.0 != 0 {
                while (bitsLeft as ::core::ffi::c_int) >= 8 as ::core::ffi::c_int {
                    let mut bits: DWORD = deserialiseBits(
                        &raw mut bytePtr,
                        &raw mut bitPtr,
                        8 as size_t,
                    );
                    value
                        |= shl32(
                            bits,
                            (numBits as ::core::ffi::c_int
                                - bitsLeft as ::core::ffi::c_int) as BYTE,
                        );
                    bitsLeft = (bitsLeft as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        as BYTE;
                }
                if bitsLeft as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    value
                        |= shl32(
                            deserialiseBits(
                                &raw mut bytePtr,
                                &raw mut bitPtr,
                                bitsLeft as size_t,
                            ),
                            (numBits as ::core::ffi::c_int
                                - bitsLeft as ::core::ffi::c_int) as BYTE,
                        );
                }
            }
            uqm_lua_checked!(setGameStateUint((*bmPtr).name, value), BOOLEAN(0));
        } else {
            if (*bmPtr).numBits as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            if (*bmPtr).numBits as ::core::ffi::c_int > rev {
                matchRev = BOOLEAN::FALSE;
            }
        }
        bmPtr = bmPtr.offset(1);
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn CreateRadar() {
    if RadarContext.is_null() {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        RadarContext = CreateContextAux(
            b"RadarContext\0".as_ptr() as *const ::core::ffi::c_char,
        );
        OldContext = SetContext(RadarContext);
        SetContextFGFrame(Screen);
        r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
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
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor))) as COORD;
        r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
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
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor))
            - ((53 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((53 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextClipRect(&raw mut r);
        SetContext(OldContext);
    }
}
#[no_mangle]
pub unsafe extern "C" fn LoadSC2Data() -> BOOLEAN {
    if FlagStatFrame.is_null() {
        FlagStatFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.flagshipstatus\0".as_ptr() as RESOURCE), BOOLEAN(0))
                as DRAWABLE,
        );
        if FlagStatFrame.is_null() {
            return BOOLEAN::FALSE;
        }
        MiscDataFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.miscdata\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
        );
        if MiscDataFrame.is_null() {
            return BOOLEAN::FALSE;
        }
        visitedStarsFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.visitedstars\0".as_ptr() as RESOURCE), BOOLEAN(0))
                as DRAWABLE,
        );
        if visitedStarsFrame.is_null() {
            return BOOLEAN::FALSE;
        }
        FontGradFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.fontgradient\0".as_ptr() as RESOURCE), BOOLEAN(0))
                as DRAWABLE,
        );
    }
    CreateRadar();
    if inHQSpace().0 != 0 {
        GlobData.Game_state.ShipStamp.origin.y = -1 as COORD;
        GlobData.Game_state.ShipStamp.origin.x = GlobData.Game_state.ShipStamp.origin.y;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn copyFleetInfo(
    mut dst: *mut FLEET_INFO,
    mut src: *mut SHIP_INFO,
    mut fleet: *mut FLEET_STUFF,
) {
    (*dst).crew_level = (*src).crew_level;
    (*dst).max_crew = (*src).max_crew;
    (*dst).max_energy = (*src).max_energy;
    (*dst).shipIdStr = (*src).idStr;
    (*dst).race_strings = (*src).race_strings;
    (*dst).icons = (*src).icons;
    (*dst).melee_icon = (*src).melee_icon;
    (*dst).actual_strength = (*fleet).strength;
    (*dst).known_loc = (*fleet).known_loc;
}
#[no_mangle]
pub unsafe extern "C" fn LoadFleetInfo() {
    let mut num_ships: COUNT = (SPECIES_ID::LAST_MELEE_ID.0 as ::core::ffi::c_int
        - SPECIES_ID::ARILOU_ID.0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COUNT;
    InitQueue(
        &raw mut GlobData.Game_state.avail_race_q,
        num_ships,
        ::core::mem::size_of::<FLEET_INFO>() as OBJ_SIZE,
    );
    let mut ship_ref: SPECIES_ID = SPECIES_ID(
        (SPECIES_ID::ARILOU_ID.0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num_ships as ::core::ffi::c_int {
        let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        if i < num_ships as ::core::ffi::c_int - 2 as ::core::ffi::c_int {
            ship_ref = SPECIES_ID(ship_ref.0.wrapping_add(1));
        } else if i == num_ships as ::core::ffi::c_int - 2 as ::core::ffi::c_int {
            ship_ref = SPECIES_ID::YEHAT_ID;
        } else {
            ship_ref = SPECIES_ID::UR_QUAN_PROBE_ID;
        }
        hFleet = AllocLink(&raw mut GlobData.Game_state.avail_race_q) as HFLEETINFO;
        if !hFleet.is_null() {
            FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
            (*FleetPtr).SpeciesID = ship_ref;
            if i < num_ships as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                let mut hMasterShip: HMASTERSHIP = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
                    MASTER_SHIP_INFO,
                >();
                hMasterShip = FindMasterShip(ship_ref);
                MasterPtr = LockMasterShip(&raw mut master_q, hMasterShip);
                copyFleetInfo(
                    FleetPtr,
                    &raw mut (*MasterPtr).ShipInfo,
                    &raw mut (*MasterPtr).Fleet,
                );
                UnlockLink(&raw mut master_q, hMasterShip);
                SeedFleet(FleetPtr, &raw mut plot_map as *mut PLOT_LOCATION);
                if i == RACE_ID::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int {
                    (*FleetPtr).actual_strength = 0 as COUNT;
                }
            } else {
                let mut RDPtr: *mut RACE_DESC = uqm_lua_checked!(load_ship(
                    (*FleetPtr).SpeciesID,
                    BOOLEAN::FALSE,
                ), ());
                if !RDPtr.is_null() {
                    copyFleetInfo(
                        FleetPtr,
                        &raw mut (*RDPtr).ship_info,
                        &raw mut (*RDPtr).fleet,
                    );
                    uqm_lua_checked!(free_ship(RDPtr, BOOLEAN::FALSE, BOOLEAN::FALSE), ());
                }
            }
            (*FleetPtr).allied_state = C2Rust_Unnamed_11::BAD_GUY.0 as ::core::ffi::c_int
                as UWORD;
            (*FleetPtr).known_strength = 0 as COUNT;
            (*FleetPtr).loc = (*FleetPtr).known_loc;
            (*FleetPtr).growth = 0 as BYTE;
            (*FleetPtr).growth_fract = 0 as BYTE;
            (*FleetPtr).growth_err_term = (255 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE;
            (*FleetPtr).days_left = 0 as BYTE;
            (*FleetPtr).func_index = !(0 as ::core::ffi::c_int) as BYTE;
            (*FleetPtr).can_build = BOOLEAN::FALSE;
            if optUnlockShips.0 != 0
                && i < SPECIES_ID::LAST_MELEE_ID.0 as ::core::ffi::c_int
            {
                (*FleetPtr).can_build = BOOLEAN::TRUE;
            }
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
            PutQueue(&raw mut GlobData.Game_state.avail_race_q, hFleet);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitGameStructures() -> BOOLEAN {
    let mut i: COUNT = 0;
    InitGlobData();
    uqm_lua_checked!(setGameStateUint(
        b"SEED_TYPE\0".as_ptr() as *const ::core::ffi::c_char,
        optSeedType as DWORD,
    ), BOOLEAN(0));
    GlobData.SIS_state.Difficulty = optDifficulty as BYTE;
    GlobData.SIS_state.Extended = optExtended.0 as BYTE;
    GlobData.SIS_state.Nomad = optNomad as BYTE;
    if optSeedType == C2Rust_Unnamed_1::OPTVAL_PRIME.0 as ::core::ffi::c_int {
        optShipSeed = OPT_ENABLABLE(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
        optCustomSeed = 16807 as ::core::ffi::c_int;
    }
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0 && !(optSeedType == C2Rust_Unnamed_1::OPTVAL_PRIME.0 as ::core::ffi::c_int)
        && !(optSeedType > C2Rust_Unnamed_1::OPTVAL_PLANET.0 as ::core::ffi::c_int)
    {
        srand(time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint);
        optCustomSeed = rand()
            % (2147483644 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int);
    }
    GlobData.SIS_state.Seed = optCustomSeed as SDWORD;
    GlobData.SIS_state.ShipSeed = (if optShipSeed.0 != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as BYTE;
    if InitStarseed(BOOLEAN::TRUE).0 == 0 {
        return BOOLEAN::FALSE;
    }
    GlobData.SIS_state.Seed = optCustomSeed as SDWORD;
    PlayFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.playmenu\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
    );
    uqm_lua_checked!(ReloadMasterShipList(None), BOOLEAN(0));
    uqm_lua_checked!(LoadFleetInfo(), BOOLEAN(0));
    InitSISContexts();
    uqm_lua_checked!(LoadSC2Data(), BOOLEAN(0));
    InitPlanetInfo();
    InitGroupInfo(BOOLEAN::TRUE);
    GlobData.Game_state.glob_flags = (5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as BYTE;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::COMMON.0 as ::core::ffi::c_int as usize] = 1
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::COMMON.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::CORROSIVE.0 as ::core::ffi::c_int as usize] = 2
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::CORROSIVE.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::BASE_METAL.0 as ::core::ffi::c_int as usize] = 3
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::BASE_METAL.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::NOBLE.0 as ::core::ffi::c_int as usize] = 4
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::NOBLE.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::RARE_EARTH.0 as ::core::ffi::c_int as usize] = 5
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::RARE_EARTH.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::PRECIOUS.0 as ::core::ffi::c_int as usize] = 6
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::PRECIOUS.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::RADIOACTIVE.0 as ::core::ffi::c_int as usize] = 8
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::RADIOACTIVE.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    GlobData
        .Game_state
        .ElementWorth[C2Rust_Unnamed::EXOTIC.0 as ::core::ffi::c_int as usize] = 25
        as BYTE;
    GlobData
        .SIS_state
        .ElementAmounts[C2Rust_Unnamed::EXOTIC.0 as ::core::ffi::c_int as usize] = 0
        as COUNT;
    let mut i_0: ::core::ffi::c_int = 0;
    match if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } {
        1 => {
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int {
                GlobData.Game_state.ElementWorth[i_0 as usize] = (GlobData
                    .Game_state
                    .ElementWorth[i_0 as usize] as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int) as BYTE;
                i_0 += 1;
            }
        }
        2 => {
            GlobData
                .Game_state
                .ElementWorth[C2Rust_Unnamed::EXOTIC.0 as ::core::ffi::c_int as usize] = 16
                as BYTE;
            uqm_lua_checked!(setGameStateUint(
                b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_int as UWORD as BYTE as DWORD,
            ), BOOLEAN(0));
            uqm_lua_checked!(setGameStateUint(
                b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
                (100 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as BYTE as DWORD,
            ), BOOLEAN(0));
        }
        _ => {}
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        GlobData.SIS_state.DriveSlots[i as usize] = (C2Rust_Unnamed_2::NUM_MODULES.0
            as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as BYTE;
        i = i.wrapping_add(1);
    }
    GlobData.SIS_state.DriveSlots[6usize] = C2Rust_Unnamed_2::FUSION_THRUSTER.0
        as ::core::ffi::c_int as BYTE;
    GlobData.SIS_state.DriveSlots[5usize] = GlobData.SIS_state.DriveSlots[6usize];
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        GlobData.SIS_state.JetSlots[i as usize] = (C2Rust_Unnamed_2::NUM_MODULES.0
            as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
        i = i.wrapping_add(1);
    }
    GlobData.SIS_state.JetSlots[6usize] = C2Rust_Unnamed_2::TURNING_JETS.0
        as ::core::ffi::c_int as BYTE;
    GlobData.SIS_state.JetSlots[0usize] = GlobData.SIS_state.JetSlots[6usize];
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        GlobData.SIS_state.ModuleSlots[i as usize] = (C2Rust_Unnamed_2::NUM_MODULES.0
            as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
        i = i.wrapping_add(1);
    }
    GlobData.SIS_state.ModuleSlots[15usize] = C2Rust_Unnamed_2::GUN_WEAPON.0
        as ::core::ffi::c_int as BYTE;
    GlobData.SIS_state.ModuleSlots[2usize] = C2Rust_Unnamed_2::CREW_POD.0
        as ::core::ffi::c_int as BYTE;
    GlobData.SIS_state.CrewEnlisted = (if if if GlobData.SIS_state.Difficulty
        as ::core::ffi::c_int != 0
    {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0
        || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
    {
        31 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int
    }) as COUNT;
    GlobData.SIS_state.ModuleSlots[8usize] = C2Rust_Unnamed_2::STORAGE_BAY.0
        as ::core::ffi::c_int as BYTE;
    GlobData.SIS_state.ModuleSlots[1usize] = C2Rust_Unnamed_2::FUEL_TANK.0
        as ::core::ffi::c_int as BYTE;
    GlobData.SIS_state.FuelOnBoard = (if if if GlobData.SIS_state.Difficulty
        as ::core::ffi::c_int != 0
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
        10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
    } else {
        4338 as ::core::ffi::c_int
    }) as DWORD;
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 1 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0
    {
        GlobData.SIS_state.DriveSlots[8usize] = C2Rust_Unnamed_2::FUSION_THRUSTER.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.DriveSlots[7usize] = GlobData.SIS_state.DriveSlots[8usize];
        GlobData.SIS_state.JetSlots[7usize] = C2Rust_Unnamed_2::TURNING_JETS.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.JetSlots[1usize] = GlobData.SIS_state.JetSlots[7usize];
    }
    if if GlobData.SIS_state.Nomad as ::core::ffi::c_int
        == C2Rust_Unnamed_0::OPTVAL_NOM_EASY.0 as ::core::ffi::c_int
    {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        GlobData.SIS_state.DriveSlots[4usize] = C2Rust_Unnamed_2::FUSION_THRUSTER.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.DriveSlots[3usize] = GlobData.SIS_state.DriveSlots[4usize];
        GlobData.SIS_state.JetSlots[5usize] = C2Rust_Unnamed_2::TURNING_JETS.0
            as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.JetSlots[2usize] = GlobData.SIS_state.JetSlots[5usize];
        GlobData.SIS_state.FuelOnBoard = GlobData
            .SIS_state
            .FuelOnBoard
            .wrapping_add(
                (20 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
            );
    }
    if optHeadStart.0 != 0 {
        uqm_lua_checked!(setGameStateUint(
            b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), BOOLEAN(0));
        uqm_lua_checked!(SetHomeworldKnown(
            C2Rust_Unnamed_12::SPATHI_HOME.0 as ::core::ffi::c_int as DWORD,
        ), BOOLEAN(0));
        if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), BOOLEAN(0));
            uqm_lua_checked!(setGameStateUint(
                b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), BOOLEAN(0));
            GlobData
                .SIS_state
                .ElementAmounts[C2Rust_Unnamed::RADIOACTIVE.0 as ::core::ffi::c_int
                as usize] = 1 as COUNT;
            GlobData.SIS_state.TotalElementMass = 1 as COUNT;
        }
    }
    uqm_lua_checked!(loadGameCheats(), BOOLEAN(0));
    InitQueue(
        &raw mut GlobData.Game_state.stowed_ship_q,
        200 as COUNT,
        ::core::mem::size_of::<SHIP_FRAGMENT>() as OBJ_SIZE,
    );
    InitQueue(
        &raw mut GlobData.Game_state.built_ship_q,
        12 as COUNT,
        ::core::mem::size_of::<SHIP_FRAGMENT>() as OBJ_SIZE,
    );
    InitQueue(
        &raw mut GlobData.Game_state.npc_built_ship_q,
        14 as COUNT,
        ::core::mem::size_of::<SHIP_FRAGMENT>() as OBJ_SIZE,
    );
    InitQueue(
        &raw mut GlobData.Game_state.ip_group_q,
        32 as COUNT,
        ::core::mem::size_of::<IP_GROUP>() as OBJ_SIZE,
    );
    InitQueue(
        &raw mut GlobData.Game_state.encounter_q,
        16 as COUNT,
        ::core::mem::size_of::<ENCOUNTER>() as OBJ_SIZE,
    );
    GlobData.Game_state.CurrentActivity = (C2Rust_Unnamed_3::IN_INTERPLANETARY.0
        as ::core::ffi::c_int
        | C2Rust_Unnamed_3::START_INTERPLANETARY.0 as ::core::ffi::c_int) as ACTIVITY;
    GlobData.SIS_state.ResUnits = (if if if GlobData.SIS_state.Difficulty
        as ::core::ffi::c_int != 0
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
        0 as ::core::ffi::c_int
    } else {
        2500 as ::core::ffi::c_int
    }) as DWORD;
    GlobData.Game_state.CrewCost = 3 as BYTE;
    GlobData.Game_state.FuelCost = 20 as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int as usize] = (500
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::FUSION_THRUSTER.0 as ::core::ffi::c_int
        as usize] = (500 as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::TURNING_JETS.0 as ::core::ffi::c_int as usize] = (500
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int as usize] = (2000
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::STORAGE_BAY.0 as ::core::ffi::c_int as usize] = (750
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::FUEL_TANK.0 as ::core::ffi::c_int as usize] = (500
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::DYNAMO_UNIT.0 as ::core::ffi::c_int as usize] = (2000
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData
        .Game_state
        .ModuleCost[C2Rust_Unnamed_2::GUN_WEAPON.0 as ::core::ffi::c_int as usize] = (2000
        as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as BYTE;
    GlobData.SIS_state.NumLanders = (if if if GlobData.SIS_state.Difficulty
        as ::core::ffi::c_int != 0
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
        2 as ::core::ffi::c_int
    }) as BYTE;
    utf8StringCopy(
        &raw mut GlobData.SIS_state.ShipName as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 32]>() as size_t,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    utf8StringCopy(
        &raw mut GlobData.SIS_state.CommanderName as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 32]>() as size_t,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    SetRaceAllied(RACE_ID::HUMAN_SHIP, BOOLEAN::TRUE);
    CloneShipFragment(
        RACE_ID::HUMAN_SHIP,
        &raw mut GlobData.Game_state.built_ship_q,
        0 as COUNT,
    );
    if optHeadStart.0 != 0 {
        let mut SpaCrew: BYTE = (if if if GlobData.SIS_state.Difficulty
            as ::core::ffi::c_int != 0
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
        uqm_lua_checked!(AddEscortShips(RACE_ID::SPATHI_SHIP, 1 as SIZE), BOOLEAN(0));
        SetEscortCrewComplement(
            RACE_ID::SPATHI_SHIP,
            SpaCrew as COUNT,
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) as BYTE,
        );
        StartSphereTracking(RACE_ID::SPATHI_SHIP);
    }
    GlobData.SIS_state.log_x = universeToLogx(
        plot_map[C2Rust_Unnamed_10::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
            .star_pt
            .x,
    );
    GlobData.SIS_state.log_y = universeToLogy(
        plot_map[C2Rust_Unnamed_10::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
            .star_pt
            .y,
    );
    CurStarDescPtr = ::core::ptr::null_mut::<STAR_DESC>();
    GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
    uqm_lua_checked!(ZeroLastLoc(), BOOLEAN(0));
    uqm_lua_checked!(ZeroAdvancedAutoPilot(), BOOLEAN(0));
    uqm_lua_checked!(ZeroAdvancedQuasiPilot(), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn FreeSC2Data() {
    DestroyContext(RadarContext);
    RadarContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    DestroyDrawable(ReleaseDrawable(FontGradFrame));
    FontGradFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(MiscDataFrame));
    MiscDataFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(visitedStarsFrame));
    visitedStarsFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(FlagStatFrame));
    FlagStatFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn UninitGameStructures() {
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    UninitQueue(&raw mut GlobData.Game_state.encounter_q);
    UninitQueue(&raw mut GlobData.Game_state.ip_group_q);
    UninitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    UninitQueue(&raw mut GlobData.Game_state.built_ship_q);
    UninitQueue(&raw mut GlobData.Game_state.stowed_ship_q);
    UninitGroupInfo();
    UninitPlanetInfo();
    hStarShip = GlobData.Game_state.avail_race_q.tail as HFLEETINFO;
    if !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        DestroyDrawable(ReleaseDrawable((*FleetPtr).melee_icon));
        DestroyDrawable(ReleaseDrawable((*FleetPtr).icons));
        DestroyStringTable(ReleaseStringTable((*FleetPtr).race_strings));
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
    }
    UninitQueue(&raw mut GlobData.Game_state.avail_race_q);
    DestroyDrawable(ReleaseDrawable(PlayFrame));
    PlayFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn InitGlobData() {
    let mut i: COUNT = 0;
    i = GlobData.Game_state.glob_flags as COUNT;
    memset(
        &raw mut GlobData as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GLOBDATA>() as size_t,
    );
    GlobData.Game_state.glob_flags = i as BYTE;
    GlobData.Game_state.DisplayArray = &raw mut DisplayArray as *mut PRIMITIVE;
}
#[no_mangle]
pub unsafe extern "C" fn SeedDEBUG() {
    let mut save: SDWORD = optCustomSeed;
    let mut histogram: [COUNT; 100] = [
        0 as COUNT,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut decisec: COUNT = 0;
    let mut start_clock: clock_t = 0;
    let mut myRNG: BOOLEAN = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    decisec = 0 as COUNT;
    while (decisec as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
        histogram[decisec as usize] = 0 as COUNT;
        decisec = decisec.wrapping_add(1);
    }
    if StarGenRNG.is_null() {
        fprintf(
            stderr,
            b"****Seed Debug Creating a STAR GEN RNG****\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        StarGenRNG = RandomContext_New();
        myRNG = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    RandomContext_SeedRandom(StarGenRNG, 123456 as ::core::ffi::c_int as DWORD);
    optCustomSeed = 123000 as ::core::ffi::c_int;
    while optCustomSeed < 123000 as ::core::ffi::c_int + 1000 as ::core::ffi::c_int {
        start_clock = clock();
        fprintf(
            stderr,
            b"\n\n\nStarting seed %d... \0".as_ptr() as *const ::core::ffi::c_char,
            optCustomSeed,
        );
        InitPlot(&raw mut plot_map as *mut PLOT_LOCATION);
        fprintf(
            stderr,
            b"seeding stars %d... \0".as_ptr() as *const ::core::ffi::c_char,
            optCustomSeed,
        );
        SeedStarmap(&raw mut star_array as *mut STAR_DESC);
        fprintf(
            stderr,
            b"seeding plots %d... \0".as_ptr() as *const ::core::ffi::c_char,
            optCustomSeed,
        );
        if (SeedPlot(
            &raw mut plot_map as *mut PLOT_LOCATION,
            &raw mut star_array as *mut STAR_DESC,
        ) as ::core::ffi::c_int) < C2Rust_Unnamed_10::NUM_PLOTS.0 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Failed to seed %d. \0".as_ptr() as *const ::core::ffi::c_char,
                optCustomSeed,
            );
            decisec = 98 as COUNT;
        } else {
            decisec = ((clock() - start_clock) as ::core::ffi::c_double
                / 100000 as ::core::ffi::c_int as ::core::ffi::c_double) as COUNT;
        }
        fprintf(
            stderr,
            b"Complete %6.6f seconds.\n\0".as_ptr() as *const ::core::ffi::c_char,
            (clock() - start_clock) as ::core::ffi::c_double
                / 1000000 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        if decisec as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
            decisec = 99 as COUNT;
        }
        histogram[decisec as usize] = histogram[decisec as usize].wrapping_add(1);
        optCustomSeed += 1;
    }
    decisec = 0 as COUNT;
    while (decisec as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
        if decisec as ::core::ffi::c_int % 10 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(
            stderr,
            b"%3d \0".as_ptr() as *const ::core::ffi::c_char,
            histogram[decisec as usize] as ::core::ffi::c_int,
        );
        decisec = decisec.wrapping_add(1);
    }
    optCustomSeed = save as ::core::ffi::c_int;
    if !StarGenRNG.is_null() && myRNG.0 != 0 {
        RandomContext_Delete(StarGenRNG);
        StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitStarseed(mut newgame: BOOLEAN) -> BOOLEAN {
    let mut i: COUNT = 0;
    DefaultStarmap(&raw mut star_array as *mut STAR_DESC);
    if StarGenRNG.is_null() {
        StarGenRNG = RandomContext_New();
        RandomContext_SeedRandom(StarGenRNG, optCustomSeed as DWORD);
    }
    if !(optSeedType > C2Rust_Unnamed_1::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        DefaultPlot(
            &raw mut plot_map as *mut PLOT_LOCATION,
            &raw mut star_array as *mut STAR_DESC,
        );
        DefaultQuasispace(&raw mut portal_map as *mut PORTAL_LOCATION);
        if !StarGenRNG.is_null() {
            RandomContext_Delete(StarGenRNG);
        }
        StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
        return BOOLEAN::TRUE;
    }
    if optSeedType == C2Rust_Unnamed_1::OPTVAL_MRQ.0 as ::core::ffi::c_int {
        DefaultPlot(
            &raw mut plot_map as *mut PLOT_LOCATION,
            &raw mut star_array as *mut STAR_DESC,
        );
        InitMelnormeRainbow(&raw mut plot_map as *mut PLOT_LOCATION);
    } else {
        InitPlot(&raw mut plot_map as *mut PLOT_LOCATION);
    }
    fprintf(
        stderr,
        b"Starting map generation using seed %d.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        optCustomSeed,
    );
    SeedStarmap(&raw mut star_array as *mut STAR_DESC);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int != 0 || newgame.0 == 0 {
        if SeedPlot(
            &raw mut plot_map as *mut PLOT_LOCATION,
            &raw mut star_array as *mut STAR_DESC,
        ) as ::core::ffi::c_int != C2Rust_Unnamed_10::NUM_PLOTS.0 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Seed Plot Failed.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !StarGenRNG.is_null() {
                RandomContext_Delete(StarGenRNG);
            }
            StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
            return BOOLEAN::FALSE;
        }
        if SeedQuasispace(
                &raw mut portal_map as *mut PORTAL_LOCATION,
                &raw mut plot_map as *mut PLOT_LOCATION,
                &raw mut star_array as *mut STAR_DESC,
            )
            .0 == 0
        {
            fprintf(
                stderr,
                b"Seed Quasisapce Failed.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !StarGenRNG.is_null() {
                RandomContext_Delete(StarGenRNG);
            }
            StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
            return BOOLEAN::FALSE;
        }
    } else {
        i = 0 as COUNT;
        while SeedPlot(
            &raw mut plot_map as *mut PLOT_LOCATION,
            &raw mut star_array as *mut STAR_DESC,
        ) as ::core::ffi::c_int != C2Rust_Unnamed_10::NUM_PLOTS.0 as ::core::ffi::c_int
            && (i as ::core::ffi::c_int) < 100 as ::core::ffi::c_int
        {
            let c2rust_fresh0 = optCustomSeed;
            optCustomSeed += 1;
            i = i.wrapping_add(1);
            fprintf(
                stderr,
                b"Seed %d failed (%d).\n\0".as_ptr() as *const ::core::ffi::c_char,
                c2rust_fresh0,
                i as ::core::ffi::c_int,
            );
            SeedStarmap(&raw mut star_array as *mut STAR_DESC);
        }
        if i as ::core::ffi::c_int >= 100 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"Seed Plot Failed.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !StarGenRNG.is_null() {
                RandomContext_Delete(StarGenRNG);
            }
            StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
            return BOOLEAN::FALSE;
        }
        if SeedQuasispace(
                &raw mut portal_map as *mut PORTAL_LOCATION,
                &raw mut plot_map as *mut PLOT_LOCATION,
                &raw mut star_array as *mut STAR_DESC,
            )
            .0 == 0
        {
            fprintf(
                stderr,
                b"Seed Quasisapce Failed.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !StarGenRNG.is_null() {
                RandomContext_Delete(StarGenRNG);
            }
            StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
            return BOOLEAN::FALSE;
        }
    }
    if 0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 502 as ::core::ffi::c_int {
            if (*(&raw mut star_array as *mut STAR_DESC).offset(i as isize)).Index == 0 {
                (*(&raw mut star_array as *mut STAR_DESC).offset(i as isize)).Index = 0
                    as BYTE;
            }
            i = i.wrapping_add(1);
        }
    }
    GlobData.SIS_state.Seed = optCustomSeed as SDWORD;
    if !StarGenRNG.is_null() {
        RandomContext_Delete(StarGenRNG);
    }
    StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn inFullGame() -> BOOLEAN {
    let mut act: ACTIVITY = GlobData.Game_state.CurrentActivity as BYTE as ACTIVITY;
    return BOOLEAN(
        (act as ::core::ffi::c_int
            == C2Rust_Unnamed_3::IN_LAST_BATTLE.0 as ::core::ffi::c_int
            || act as ::core::ffi::c_int
                == C2Rust_Unnamed_3::IN_ENCOUNTER.0 as ::core::ffi::c_int
            || act as ::core::ffi::c_int
                == C2Rust_Unnamed_3::IN_HYPERSPACE.0 as ::core::ffi::c_int
            || act as ::core::ffi::c_int
                == C2Rust_Unnamed_3::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            || act as ::core::ffi::c_int
                == C2Rust_Unnamed_3::WON_LAST_BATTLE.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn inSuperMelee() -> BOOLEAN {
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_3::SUPER_MELEE.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn inSavablePos() -> BOOLEAN {
    let mut curr_act: ACTIVITY = GlobData.Game_state.CurrentActivity;
    let mut lob_curr_act: ACTIVITY = curr_act as BYTE as ACTIVITY;
    if InPopUp.0 != 0 || SaveLoadActive.0 != 0
        || lob_curr_act as ::core::ffi::c_int
            == C2Rust_Unnamed_3::SUPER_MELEE.0 as ::core::ffi::c_int
        || !planetSideDesc.is_null()
        || lob_curr_act as ::core::ffi::c_int
            == C2Rust_Unnamed_3::WON_LAST_BATTLE.0 as ::core::ffi::c_int
        || !CommData.ConversationPhrases.is_null()
        || curr_act as ::core::ffi::c_int
            & C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int != 0
            && lob_curr_act as ::core::ffi::c_int
                != C2Rust_Unnamed_3::IN_HYPERSPACE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn inHQSpace() -> BOOLEAN {
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_3::IN_HYPERSPACE.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn inHyperSpace() -> BOOLEAN {
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_3::IN_HYPERSPACE.0 as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) <= 1 as DWORD) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn inQuasiSpace() -> BOOLEAN {
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_3::IN_HYPERSPACE.0 as ::core::ffi::c_int
            && uqm_lua_checked!(getGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) > 1 as DWORD) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn isPC(mut optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE {
    return OPT_CONSOLETYPE(
        (optWhich == 0x2 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is3DO(mut optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE {
    return OPT_CONSOLETYPE(
        (optWhich == 0x1 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn replaceChar(
    mut pStr: *mut ::core::ffi::c_char,
    find: ::core::ffi::c_char,
    replace: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0;
    let mut len: size_t = strlen(pStr);
    if pStr.is_null() || find == 0 || replace == 0 {
        return -1 as ::core::ffi::c_int;
    }
    if utf8StringPos(pStr, find as UniChar) == -1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while i < len {
        if *pStr.offset(i as isize) as ::core::ffi::c_int == find as ::core::ffi::c_int {
            *pStr.offset(i as isize) = replace;
            if *pStr.offset(i as isize) as ::core::ffi::c_int
                == replace as ::core::ffi::c_int
            {
                count += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    return count;
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
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[inline]
unsafe extern "C" fn LockMasterShip(
    mut pq: *const QUEUE,
    mut h: HMASTERSHIP,
) -> *mut MASTER_SHIP_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<MASTER_SHIP_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (MASTER_SHIP_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
