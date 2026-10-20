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

// Rust translation of src/uqm/load.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type frame_desc;
    pub type GAME_STATE_FILE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn ReadResFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optDeCleansing: OPT_ENABLABLE;
    static mut saveDir: *mut uio_DirHandle;
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn ReinitQueue(pq: *mut QUEUE);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn AddEvent(
        r#type: EVENT_TYPE,
        month_index: COUNT,
        day_index: COUNT,
        year_index: COUNT,
        func_index: BYTE,
    ) -> HEVENT;
    fn luaUqm_initState();
    fn luaUqm_uninitState();
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    fn totalBitsForGameState(
        bm: *const GameStateBitMap,
        rev: ::core::ffi::c_int,
    ) -> size_t;
    fn getGameStateRevByBytes(
        bm: *const GameStateBitMap,
        bytes: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn deserialiseGameState(
        bm: *const GameStateBitMap,
        buf: *const BYTE,
        numBytes: size_t,
        rev: ::core::ffi::c_int,
    ) -> BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn InitStarseed(newgame: BOOLEAN) -> BOOLEAN;
    fn LoadFleetInfo();
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn loadGameCheats();
    static mut legacySave: BOOLEAN;
    static mut EncounterRace: SIZE;
    static mut EncounterGroup: BYTE;
    fn InitGroupInfo(FirstTime: BOOLEAN);
    fn OpenStateFile(
        stateFile: ::core::ffi::c_int,
        mode: *const ::core::ffi::c_char,
    ) -> *mut GAME_STATE_FILE;
    fn CloseStateFile(fp: *mut GAME_STATE_FILE);
    fn LengthStateFile(fp: *mut GAME_STATE_FILE) -> DWORD;
    fn WriteStateFile(
        lpBuf: *const ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        fp: *mut GAME_STATE_FILE,
    ) -> ::core::ffi::c_int;
    fn SeekStateFile(
        fp: *mut GAME_STATE_FILE,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn LoadLegacyGame(
        which_game: COUNT,
        SummPtr: *mut SUMMARY_DESC,
        try_vanilla: BOOLEAN,
    ) -> BOOLEAN;
    static gameStateBitMap: [GameStateBitMap; 0];
    static mut gameStateBitMapRevTag: [*const ::core::ffi::c_char; 0];
    static mut race_q: [QUEUE; 0];
    fn BuildGroup(pDstQueue: *mut QUEUE, race_id: BYTE) -> HIPGROUP;
    fn ReadGroupHeader(fp: *mut GAME_STATE_FILE, pGH: *mut GROUP_HEADER);
    fn WriteGroupHeader(fp: *mut GAME_STATE_FILE, pGH: *const GROUP_HEADER);
    fn WriteShipFragment(fp: *mut GAME_STATE_FILE, FragPtr: *const SHIP_FRAGMENT);
    fn WriteIpGroup(fp: *mut GAME_STATE_FILE, GroupPtr: *const IP_GROUP);
    fn initEventSystem();
    fn uninitEventSystem();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut DebugKeyPressed: BOOLEAN;
    fn ReloadMasterShipList(YieldProcessing: Option<unsafe extern "C" fn() -> ()>);
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
pub type TEXT = text;
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
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
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
pub type HELEMENT = HLINK;
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
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
pub type HSHIPFRAG = HLINK;
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
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
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
pub type HEVENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub pred: HEVENT,
    pub succ: HEVENT,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub func_index: BYTE,
}
pub type EVENT = event;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct GameStateBitMap {
    pub name: *const ::core::ffi::c_char,
    pub numBits: BYTE,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brief_ship_info {
    pub race_id: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub max_energy: BYTE,
}
pub type BRIEF_SHIP_INFO = brief_ship_info;
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
pub type HENCOUNTER = HLINK;
pub type ENCOUNTER = encounter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SUMMARY_DESC {
    pub SS: SIS_STATE,
    pub Activity: BYTE,
    pub Flags: BYTE,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub MCreditLo: BYTE,
    pub MCreditHi: BYTE,
    pub NumShips: BYTE,
    pub NumDevices: BYTE,
    pub ShipList: [BYTE; 12],
    pub DeviceList: [BYTE; 16],
    pub SaveName: [UNICODE; 64],
    pub SaveNameChecker: [UNICODE; 32],
    pub LegacySaveName: [UNICODE; 32],
    pub res_factor: BYTE,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GROUP_HEADER {
    pub NumGroups: BYTE,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub star_index: COUNT,
    pub year_index: COUNT,
    pub GroupOffset: [DWORD; 65],
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
unsafe extern "C" fn oldLogxToUniverse(mut lx: SDWORD) -> SDWORD {
    return (lx
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int)
        + ((8192 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            * 195 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        / ((8192 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            * 195 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn oldLogyToUniverse(mut ly: SDWORD) -> SDWORD {
    return 9999 as ::core::ffi::c_int
        - (ly
            * (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
            + ((7680 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                * 25 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / ((7680 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                * 25 as ::core::ffi::c_int);
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
unsafe extern "C" fn LockShipFrag(
    mut pq: *const QUEUE,
    mut h: HSHIPFRAG,
) -> *mut SHIP_FRAGMENT {
    ((*pq).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (SHIP_FRAGMENT)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
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
#[no_mangle]
pub static mut NextActivity: ACTIVITY = 0;
#[no_mangle]
pub static mut IndependantResFactor: BYTE = 0;
#[inline]
unsafe extern "C" fn read_8(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut BYTE,
) -> size_t {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return uqm_lua_checked!(ReadResFile(
        v as *mut ::core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        fp as *mut uio_Stream,
    ), 0);
}
#[inline]
unsafe extern "C" fn read_16(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut UWORD,
) -> size_t {
    let mut t: UWORD = 0 as UWORD;
    let mut shift: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    shift = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let mut b: BYTE = 0;
        if uqm_lua_checked!(read_8(fp, &raw mut b), 0) != 1 as size_t {
            return 0 as size_t;
        }
        t = (t as ::core::ffi::c_int | (b as UWORD as ::core::ffi::c_int) << shift)
            as UWORD;
        i += 1;
        shift += 8 as ::core::ffi::c_int;
    }
    if !v.is_null() {
        *v = t;
    }
    return 1 as size_t;
}
#[inline]
unsafe extern "C" fn read_16s(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut SWORD,
) -> size_t {
    return uqm_lua_checked!(read_16(fp, v as *mut UWORD), 0);
}
#[inline]
unsafe extern "C" fn read_32(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut DWORD,
) -> size_t {
    let mut t: DWORD = 0 as DWORD;
    let mut shift: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    shift = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut b: BYTE = 0;
        if uqm_lua_checked!(read_8(fp, &raw mut b), 0) != 1 as size_t {
            return 0 as size_t;
        }
        t |= (b as DWORD) << shift;
        i += 1;
        shift += 8 as ::core::ffi::c_int;
    }
    if !v.is_null() {
        *v = t;
    }
    return 1 as size_t;
}
#[inline]
unsafe extern "C" fn read_32s(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut SDWORD,
) -> size_t {
    return uqm_lua_checked!(read_32(fp, v as *mut DWORD), 0);
}
#[inline]
unsafe extern "C" fn read_a8(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *mut BYTE,
    mut count: COUNT,
) -> size_t {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/load.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int,
                b"read_a8\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return (uqm_lua_checked!(ReadResFile(
        ar as *mut ::core::ffi::c_void,
        1 as size_t,
        count as size_t,
        fp as *mut uio_Stream,
    ), 0) == count as size_t) as ::core::ffi::c_int as size_t;
}
#[inline]
unsafe extern "C" fn read_a8s(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *mut ::core::ffi::c_char,
    mut count: COUNT,
) -> size_t {
    return uqm_lua_checked!(read_a8(fp, ar as *mut BYTE, count), 0);
}
#[inline]
unsafe extern "C" fn skip_8(
    mut fp: *mut ::core::ffi::c_void,
    mut count: COUNT,
) -> size_t {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < count as ::core::ffi::c_int {
        if uqm_lua_checked!(read_8(fp, ::core::ptr::null_mut::<BYTE>()), 0) != 1 as size_t {
            return 0 as size_t;
        }
        i += 1;
    }
    return 1 as size_t;
}
#[inline]
unsafe extern "C" fn read_str(
    mut fp: *mut ::core::ffi::c_void,
    mut str: *mut ::core::ffi::c_char,
    mut count: COUNT,
) -> size_t {
    return uqm_lua_checked!(read_a8(fp, str as *mut BYTE, count), 0);
}
#[inline]
unsafe extern "C" fn read_a16(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *mut UWORD,
    mut count: COUNT,
) -> size_t {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/load.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int,
                b"read_a16\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if uqm_lua_checked!(read_16(fp, ar), 0) != 1 as size_t {
            return 0 as size_t;
        }
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
    return 1 as size_t;
}
unsafe extern "C" fn LoadShipQueue(
    mut fh: *mut ::core::ffi::c_void,
    mut pQueue: *mut QUEUE,
    mut size: DWORD,
) {
    let mut num_links: COUNT = size.wrapping_div(11 as DWORD) as COUNT;
    loop {
        let c2rust_fresh2 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        let mut Index: COUNT = 0;
        uqm_lua_checked!(read_16(fh, &raw mut Index), ());
        hStarShip = CloneShipFragment(
            RACE_ID(Index as ::core::ffi::c_uint),
            pQueue,
            0 as COUNT,
        );
        FragPtr = LockShipFrag(pQueue, hStarShip);
        uqm_lua_checked!(read_8(fh, &raw mut (*FragPtr).captains_name_index), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FragPtr).race_id), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FragPtr).index), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*FragPtr).crew_level), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*FragPtr).max_crew), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FragPtr).energy_level), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FragPtr).max_energy), ());
        UnlockLink(pQueue, hStarShip);
    };
}
unsafe extern "C" fn LoadRaceQueue(
    mut fh: *mut ::core::ffi::c_void,
    mut pQueue: *mut QUEUE,
    mut size: DWORD,
) {
    let mut num_links: COUNT = size.wrapping_div(30 as DWORD) as COUNT;
    loop {
        let c2rust_fresh4 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh4 == 0 {
            break;
        }
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut Index: COUNT = 0;
        uqm_lua_checked!(read_16(fh, &raw mut Index), ());
        hStarShip = GetStarShipFromIndex(pQueue, Index) as HFLEETINFO;
        FleetPtr = LockFleetInfo(pQueue, hStarShip);
        uqm_lua_checked!(read_16(fh, &raw mut (*FleetPtr).allied_state), ());
        if (*FleetPtr).allied_state as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
            (*FleetPtr).allied_state = C2Rust_Unnamed_0::BAD_GUY.0 as ::core::ffi::c_int
                as UWORD;
        }
        uqm_lua_checked!(read_8(fh, &raw mut (*FleetPtr).days_left), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FleetPtr).growth_fract), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*FleetPtr).crew_level), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*FleetPtr).max_crew), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FleetPtr).growth), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FleetPtr).max_energy), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*FleetPtr).loc.x), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*FleetPtr).loc.y), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*FleetPtr).actual_strength), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*FleetPtr).known_strength), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*FleetPtr).known_loc.x), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*FleetPtr).known_loc.y), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FleetPtr).growth_err_term), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*FleetPtr).func_index), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*FleetPtr).dest_loc.x), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*FleetPtr).dest_loc.y), ());
        UnlockLink(pQueue, hStarShip);
    };
}
unsafe extern "C" fn LoadGroupQueue(
    mut fh: *mut ::core::ffi::c_void,
    mut pQueue: *mut QUEUE,
    mut size: DWORD,
) {
    let mut num_links: COUNT = size.wrapping_div(13 as DWORD) as COUNT;
    loop {
        let c2rust_fresh3 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        hGroup = BuildGroup(pQueue, 0 as BYTE);
        GroupPtr = LockIpGroup(pQueue, hGroup);
        uqm_lua_checked!(read_16(fh, &raw mut (*GroupPtr).group_counter), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).race_id), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).sys_loc), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).task), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).in_system), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).dest_loc), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).orbit_pos), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*GroupPtr).group_id), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*GroupPtr).loc.x), ());
        uqm_lua_checked!(read_16s(fh, &raw mut (*GroupPtr).loc.y), ());
        UnlockLink(pQueue, hGroup);
    };
}
unsafe extern "C" fn LoadEncounter(
    mut EncounterPtr: *mut ENCOUNTER,
    mut fh: *mut ::core::ffi::c_void,
    mut try_core: BOOLEAN,
) {
    let mut i: COUNT = 0;
    (*EncounterPtr).pred = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*EncounterPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*EncounterPtr).hElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(read_16s(fh, &raw mut (*EncounterPtr).transition_state), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*EncounterPtr).origin.x), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*EncounterPtr).origin.y), ());
    uqm_lua_checked!(read_16(fh, &raw mut (*EncounterPtr).radius), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*EncounterPtr).loc_pt.x), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*EncounterPtr).loc_pt.y), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*EncounterPtr).race_id), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*EncounterPtr).num_ships), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*EncounterPtr).flags), ());
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
        let mut ShipInfo: *mut BRIEF_SHIP_INFO = (&raw mut (*EncounterPtr).ShipList
            as *mut BRIEF_SHIP_INFO)
            .offset(i as isize);
        uqm_lua_checked!(read_8(fh, &raw mut (*ShipInfo).race_id), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*ShipInfo).crew_level), ());
        uqm_lua_checked!(read_16(fh, &raw mut (*ShipInfo).max_crew), ());
        uqm_lua_checked!(read_8(fh, &raw mut (*ShipInfo).max_energy), ());
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(read_32s(fh, &raw mut (*EncounterPtr).log_x), ());
    uqm_lua_checked!(read_32s(fh, &raw mut (*EncounterPtr).log_y), ());
    if try_core.0 != 0 {
        (*EncounterPtr).log_x = universeToLogx(
            oldLogxToUniverse((*EncounterPtr).log_x) as COORD,
        ) << resolutionFactor;
        (*EncounterPtr).log_y = universeToLogy(
            oldLogyToUniverse((*EncounterPtr).log_y) as COORD,
        ) << resolutionFactor;
    } else {
        (*EncounterPtr).log_x <<= resolutionFactor;
        (*EncounterPtr).log_y <<= resolutionFactor;
    };
}
unsafe extern "C" fn LoadEvent(
    mut EventPtr: *mut EVENT,
    mut fh: *mut ::core::ffi::c_void,
) {
    (*EventPtr).pred = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*EventPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(read_8(fh, &raw mut (*EventPtr).day_index), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*EventPtr).month_index), ());
    uqm_lua_checked!(read_16(fh, &raw mut (*EventPtr).year_index), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*EventPtr).func_index), ());
}
unsafe extern "C" fn LoadClockState(
    mut ClockPtr: *mut CLOCK_STATE,
    mut fh: *mut ::core::ffi::c_void,
) {
    uqm_lua_checked!(read_8(fh, &raw mut (*ClockPtr).day_index), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*ClockPtr).month_index), ());
    uqm_lua_checked!(read_16(fh, &raw mut (*ClockPtr).year_index), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*ClockPtr).tick_count), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*ClockPtr).day_in_ticks), ());
}
unsafe extern "C" fn LoadGameState(
    mut GSPtr: *mut GAME_STATE,
    mut fh: *mut ::core::ffi::c_void,
    mut try_core: BOOLEAN,
) -> BOOLEAN {
    let mut magic: DWORD = 0;
    legacySave = try_core;
    uqm_lua_checked!(read_32(fh, &raw mut magic), BOOLEAN(0));
    if magic != 0x74536c47 as ::core::ffi::c_int as DWORD {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(read_32(fh, &raw mut magic), BOOLEAN(0));
    if magic != 75 as DWORD {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(read_8(fh, &raw mut (*GSPtr).glob_flags), BOOLEAN(0));
    uqm_lua_checked!(read_8(fh, &raw mut (*GSPtr).CrewCost), BOOLEAN(0));
    uqm_lua_checked!(read_8(fh, &raw mut (*GSPtr).FuelCost), BOOLEAN(0));
    uqm_lua_checked!(read_a8(
        fh,
        &raw mut (*GSPtr).ModuleCost as *mut BYTE,
        C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0));
    uqm_lua_checked!(read_a8(
        fh,
        &raw mut (*GSPtr).ElementWorth as *mut BYTE,
        C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0));
    uqm_lua_checked!(read_16(fh, &raw mut (*GSPtr).CurrentActivity), BOOLEAN(0));
    uqm_lua_checked!(LoadClockState(&raw mut (*GSPtr).GameClock, fh), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).autopilot.x), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).autopilot.y), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).ip_location.x), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).ip_location.y), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).ShipStamp.origin.x), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).ShipStamp.origin.y), BOOLEAN(0));
    uqm_lua_checked!(read_16(fh, &raw mut (*GSPtr).ShipFacing), BOOLEAN(0));
    uqm_lua_checked!(read_8(fh, &raw mut (*GSPtr).ip_planet), BOOLEAN(0));
    uqm_lua_checked!(read_8(fh, &raw mut (*GSPtr).in_orbit), BOOLEAN(0));
    (*GSPtr).ShipStamp.origin.x = (((*GSPtr).ShipStamp.origin.x as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    (*GSPtr).ShipStamp.origin.y = (((*GSPtr).ShipStamp.origin.y as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    uqm_lua_checked!(read_16(fh, &raw mut (*GSPtr).velocity.TravelAngle), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.vector.width), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.vector.height), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.fract.width), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.fract.height), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.error.width), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.error.height), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.incr.width), BOOLEAN(0));
    uqm_lua_checked!(read_16s(fh, &raw mut (*GSPtr).velocity.incr.height), BOOLEAN(0));
    if (*GSPtr).CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_3::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        (*GSPtr).velocity.vector.width = (((*GSPtr).velocity.vector.width
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.vector.height = (((*GSPtr).velocity.vector.height
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.fract.width = (((*GSPtr).velocity.fract.width
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.fract.height = (((*GSPtr).velocity.fract.height
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.error.width = (((*GSPtr).velocity.error.width
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.error.height = (((*GSPtr).velocity.error.height
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.incr.width = (((*GSPtr).velocity.incr.width
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        (*GSPtr).velocity.incr.height = (((*GSPtr).velocity.incr.height
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    uqm_lua_checked!(read_32(fh, &raw mut magic), BOOLEAN(0));
    if magic != 0x74536d47 as ::core::ffi::c_int as DWORD {
        return BOOLEAN::FALSE;
    }
    let mut buf: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    let mut rev: ::core::ffi::c_int = 0;
    let mut gameStateByteCount: size_t = 0;
    uqm_lua_checked!(read_32(fh, &raw mut magic), BOOLEAN(0));
    rev = if try_core.0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        getGameStateRevByBytes(
            &raw const gameStateBitMap as *const GameStateBitMap,
            magic as ::core::ffi::c_int,
        )
    };
    gameStateByteCount = totalBitsForGameState(
            &raw const gameStateBitMap as *const GameStateBitMap,
            rev,
        )
        .wrapping_add(7 as size_t) >> 3 as ::core::ffi::c_int;
    if rev < 0 as ::core::ffi::c_int || (magic as size_t) < gameStateByteCount {
        log_add(
            log_Level::log_Error,
            b"Warning: Savegame is corrupt: saved game state is too small.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    log_add(
        log_Level::log_Debug,
        b"Detected save game state rev %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
        rev,
        *(&raw mut gameStateBitMapRevTag as *mut *const ::core::ffi::c_char)
            .offset(rev as isize),
    );
    buf = HMalloc(gameStateByteCount) as *mut BYTE;
    if buf.is_null() {
        log_add(
            log_Level::log_Error,
            b"Warning: Cannot allocate enough bytes for the saved game state (%lu bytes).\0"
                .as_ptr() as *const ::core::ffi::c_char,
            gameStateByteCount as ::core::ffi::c_ulong,
        );
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(read_a8(fh, buf, gameStateByteCount as COUNT), BOOLEAN(0));
    result = uqm_lua_checked!(deserialiseGameState(
        &raw const gameStateBitMap as *const GameStateBitMap,
        buf,
        gameStateByteCount,
        rev,
    ), BOOLEAN(0));
    HFree(buf as *mut ::core::ffi::c_void);
    if result.0 == BOOLEAN::FALSE.0 {
        return BOOLEAN::FALSE;
    }
    if rev < 2 as ::core::ffi::c_int {
        (*GSPtr).glob_flags = (5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as BYTE;
    }
    if rev < 3 as ::core::ffi::c_int {
        uqm_lua_checked!(ZeroLastLoc(), BOOLEAN(0));
    }
    if rev < 4 as ::core::ffi::c_int {
        uqm_lua_checked!(ZeroAdvancedAutoPilot(), BOOLEAN(0));
        uqm_lua_checked!(ZeroAdvancedQuasiPilot(), BOOLEAN(0));
    }
    if magic as size_t > gameStateByteCount {
        uqm_lua_checked!(skip_8(fh, (magic as size_t).wrapping_sub(gameStateByteCount) as COUNT), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn LoadSisState(
    mut SSPtr: *mut SIS_STATE,
    mut fp: *mut ::core::ffi::c_void,
    mut try_core: BOOLEAN,
    mut legacyMM: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut SisNameSize: COUNT = (if legacyMM == 1 as ::core::ffi::c_int
        || try_core.0 != 0
    {
        16 as ::core::ffi::c_int
    } else {
        32 as ::core::ffi::c_int
    }) as COUNT;
    if uqm_lua_checked!(read_32s(fp, &raw mut (*SSPtr).log_x), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_32s(fp, &raw mut (*SSPtr).log_y), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_32(fp, &raw mut (*SSPtr).ResUnits), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_32(fp, &raw mut (*SSPtr).FuelOnBoard), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_16(fp, &raw mut (*SSPtr).CrewEnlisted), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_16(fp, &raw mut (*SSPtr).TotalElementMass), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_16(fp, &raw mut (*SSPtr).TotalBioMass), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_a8(fp, &raw mut (*SSPtr).ModuleSlots as *mut BYTE, 16 as COUNT), BOOLEAN(0))
            != 1 as size_t
        || uqm_lua_checked!(read_a8(fp, &raw mut (*SSPtr).DriveSlots as *mut BYTE, 11 as COUNT), BOOLEAN(0))
            != 1 as size_t
        || uqm_lua_checked!(read_a8(fp, &raw mut (*SSPtr).JetSlots as *mut BYTE, 8 as COUNT), BOOLEAN(0))
            != 1 as size_t || uqm_lua_checked!(read_8(fp, &raw mut (*SSPtr).NumLanders), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_a16(
            fp,
            &raw mut (*SSPtr).ElementAmounts as *mut UWORD,
            C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
        ), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SSPtr).ShipName as *mut ::core::ffi::c_char,
            SisNameSize,
        ), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SSPtr).CommanderName as *mut ::core::ffi::c_char,
            SisNameSize,
        ), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SSPtr).PlanetName as *mut ::core::ffi::c_char,
            SisNameSize,
        ), BOOLEAN(0)) != 1 as size_t
        || try_core.0 == 0 && uqm_lua_checked!(read_8(fp, &raw mut (*SSPtr).Difficulty), BOOLEAN(0)) != 1 as size_t
        || try_core.0 == 0 && uqm_lua_checked!(read_8(fp, &raw mut (*SSPtr).Extended), BOOLEAN(0)) != 1 as size_t
        || try_core.0 == 0 && uqm_lua_checked!(read_8(fp, &raw mut (*SSPtr).Nomad), BOOLEAN(0)) != 1 as size_t
        || try_core.0 == 0 && uqm_lua_checked!(read_32s(fp, &raw mut (*SSPtr).Seed), BOOLEAN(0)) != 1 as size_t
        || try_core.0 == 0 && !(legacyMM > 0 as ::core::ffi::c_int)
            && uqm_lua_checked!(read_8(fp, &raw mut (*SSPtr).ShipSeed), BOOLEAN(0)) != 1 as size_t
    {
        return BOOLEAN::FALSE
    } else {
        if try_core.0 != 0 {
            (*SSPtr).log_x = universeToLogx(oldLogxToUniverse((*SSPtr).log_x) as COORD);
            (*SSPtr).log_y = universeToLogy(oldLogyToUniverse((*SSPtr).log_y) as COORD);
        } else {
            (*SSPtr).log_x <<= resolutionFactor;
            (*SSPtr).log_y <<= resolutionFactor;
        }
        return BOOLEAN::TRUE;
    };
}
unsafe extern "C" fn LoadSummary(
    mut SummPtr: *mut SUMMARY_DESC,
    mut fp: *mut ::core::ffi::c_void,
    mut try_core: BOOLEAN,
) -> BOOLEAN {
    let mut magic: DWORD = 0;
    let mut magicTag: DWORD = (if try_core.0 != 0 {
        0x1534d55 as ::core::ffi::c_int
    } else {
        0x34564d4d as ::core::ffi::c_int
    }) as DWORD;
    let mut nameSize: DWORD = 0 as DWORD;
    let mut legacyMM: ::core::ffi::c_int = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
    if uqm_lua_checked!(read_32(fp, &raw mut magic), BOOLEAN(0)) == 0 {
        return BOOLEAN::FALSE;
    }
    if magic == magicTag || magic == 0x4147454d as ::core::ffi::c_int as DWORD
        || magic == 0x33564d4d as ::core::ffi::c_int as DWORD
    {
        legacyMM = if magic == 0x4147454d as ::core::ffi::c_int as DWORD {
            1 as ::core::ffi::c_int
        } else if magic == 0x33564d4d as ::core::ffi::c_int as DWORD {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if uqm_lua_checked!(read_32(fp, &raw mut magic), BOOLEAN(0)) != 1 as size_t
            || magic != 0x6d6d7553 as ::core::ffi::c_int as DWORD
        {
            return BOOLEAN::FALSE;
        }
        if uqm_lua_checked!(read_32(fp, &raw mut magic), BOOLEAN(0)) != 1 as size_t || magic < 160 as DWORD {
            return BOOLEAN::FALSE;
        }
        nameSize = magic.wrapping_sub(160 as DWORD);
    } else {
        return BOOLEAN::FALSE
    }
    if uqm_lua_checked!(LoadSisState(&raw mut (*SummPtr).SS, fp, try_core, legacyMM), BOOLEAN(0)).0 == 0 {
        return BOOLEAN::FALSE;
    }
    (*SummPtr).SS.SaveVersion = 0 as BYTE;
    if try_core.0 != 0 {
        (*SummPtr).SS.Difficulty = 0 as BYTE;
        (*SummPtr).SS.Seed = (*SummPtr).SS.Difficulty as SDWORD;
        (*SummPtr).SS.Nomad = 0 as BYTE;
        (*SummPtr).SS.Extended = (*SummPtr).SS.Nomad;
        (*SummPtr).SS.SaveVersion = 1 as BYTE;
    }
    if try_core.0 != 0 || legacyMM > 0 as ::core::ffi::c_int {
        (*SummPtr).SS.ShipSeed = 0 as BYTE;
    }
    if uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).Activity), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).Flags), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).day_index), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).month_index), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_16(fp, &raw mut (*SummPtr).year_index), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).MCreditLo), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).MCreditHi), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).NumShips), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).NumDevices), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_a8(fp, &raw mut (*SummPtr).ShipList as *mut BYTE, 12 as COUNT), BOOLEAN(0))
            != 1 as size_t
        || uqm_lua_checked!(read_a8(fp, &raw mut (*SummPtr).DeviceList as *mut BYTE, 16 as COUNT), BOOLEAN(0))
            != 1 as size_t
        || try_core.0 == 0 && uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).res_factor), BOOLEAN(0)) != 1 as size_t
    {
        return BOOLEAN::FALSE;
    }
    IndependantResFactor = (if try_core.0 == 0 {
        (*SummPtr).res_factor as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as BYTE;
    if nameSize < 64 as DWORD {
        if uqm_lua_checked!(read_a8s(
            fp,
            &raw mut (*SummPtr).SaveName as *mut ::core::ffi::c_char,
            nameSize as COUNT,
        ), BOOLEAN(0)) != 1 as size_t
        {
            return BOOLEAN::FALSE;
        }
        (*SummPtr).SaveName[nameSize as usize] = 0 as UNICODE;
    } else {
        let mut remaining: DWORD = nameSize
            .wrapping_sub(64 as DWORD)
            .wrapping_add(1 as DWORD);
        if uqm_lua_checked!(read_a8s(
            fp,
            &raw mut (*SummPtr).SaveName as *mut ::core::ffi::c_char,
            (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
        ), BOOLEAN(0)) != 1 as size_t
        {
            return BOOLEAN::FALSE;
        }
        (*SummPtr)
            .SaveName[(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0
            as UNICODE;
        if uqm_lua_checked!(skip_8(fp, remaining as COUNT), BOOLEAN(0)) != 1 as size_t {
            return BOOLEAN::FALSE;
        }
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn LoadStarDesc(
    mut SDPtr: *mut STAR_DESC,
    mut fh: *mut ::core::ffi::c_void,
) {
    uqm_lua_checked!(read_16s(fh, &raw mut (*SDPtr).star_pt.x), ());
    uqm_lua_checked!(read_16s(fh, &raw mut (*SDPtr).star_pt.y), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*SDPtr).Type), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*SDPtr).Index), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*SDPtr).Prefix), ());
    uqm_lua_checked!(read_8(fh, &raw mut (*SDPtr).Postfix), ());
}
unsafe extern "C" fn LoadScanInfo(mut fh: *mut uio_Stream, mut flen: DWORD) {
    let mut fp: *mut GAME_STATE_FILE = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        while flen != 0 {
            let mut val: DWORD = 0;
            uqm_lua_checked!(read_32(fh as *mut ::core::ffi::c_void, &raw mut val), ());
            swrite_32(fp, val);
            flen = flen.wrapping_sub(4 as DWORD);
        }
        CloseStateFile(fp);
    }
}
unsafe extern "C" fn LoadGroupList(mut fh: *mut uio_Stream, mut chunksize: DWORD) {
    let mut fp: *mut GAME_STATE_FILE = OpenStateFile(
        1 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut h: GROUP_HEADER = GROUP_HEADER {
            NumGroups: 0,
            day_index: 0,
            month_index: 0,
            star_index: 0,
            year_index: 0,
            GroupOffset: [0; 65],
        };
        let mut LastEnc: BYTE = 0;
        let mut NumGroups: BYTE = 0;
        let mut i: ::core::ffi::c_int = 0;
        ReadGroupHeader(fp, &raw mut h);
        h.GroupOffset[0usize] = LengthStateFile(fp);
        SeekStateFile(fp, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
        WriteGroupHeader(fp, &raw mut h);
        SeekStateFile(
            fp,
            h.GroupOffset[0usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut LastEnc), ());
        NumGroups = chunksize.wrapping_sub(1 as DWORD).wrapping_div(14 as DWORD) as BYTE;
        swrite_8(fp, LastEnc);
        swrite_8(fp, NumGroups);
        i = 0 as ::core::ffi::c_int;
        while i < NumGroups as ::core::ffi::c_int {
            let mut race_outer: BYTE = 0;
            let mut ip: IP_GROUP = IP_GROUP {
                pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                group_counter: 0,
                race_id: 0,
                sys_loc: 0,
                task: 0,
                in_system: 0,
                dest_loc: 0,
                orbit_pos: 0,
                group_id: 0,
                loc: POINT { x: 0, y: 0 },
                melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
                flags: 0,
            };
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut race_outer), ());
            uqm_lua_checked!(read_16(fh as *mut ::core::ffi::c_void, &raw mut ip.group_counter), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.race_id), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.sys_loc), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.task), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.in_system), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.dest_loc), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.orbit_pos), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut ip.group_id), ());
            uqm_lua_checked!(read_16s(fh as *mut ::core::ffi::c_void, &raw mut ip.loc.x), ());
            uqm_lua_checked!(read_16s(fh as *mut ::core::ffi::c_void, &raw mut ip.loc.y), ());
            swrite_8(fp, race_outer);
            WriteIpGroup(fp, &raw mut ip);
            i += 1;
        }
        CloseStateFile(fp);
    }
}
unsafe extern "C" fn SetBattleGroupOffset(
    mut encounterIndex: ::core::ffi::c_int,
    mut offset: DWORD,
) {
    match encounterIndex {
        1 => {
            uqm_lua_checked!(setGameStateUint(
                b"SHOFIXTI_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        2 => {
            uqm_lua_checked!(setGameStateUint(
                b"ZOQFOT_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        3 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME0_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        4 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME1_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        5 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME2_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        6 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME3_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        7 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME4_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        8 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME5_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        9 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME6_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        10 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME7_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        11 => {
            uqm_lua_checked!(setGameStateUint(
                b"MELNORME8_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        12 => {
            uqm_lua_checked!(setGameStateUint(
                b"URQUAN_PROBE_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        13 => {
            uqm_lua_checked!(setGameStateUint(
                b"COLONY_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        14 => {
            uqm_lua_checked!(setGameStateUint(
                b"SAMATRA_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
                offset,
            ), ());
        }
        _ => {
            log_add(
                log_Level::log_Warning,
                b"SetBattleGroupOffset: invalid encounter index.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn LoadBattleGroup(mut fh: *mut uio_Stream, mut chunksize: DWORD) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    let mut h: GROUP_HEADER = GROUP_HEADER {
        NumGroups: 0,
        day_index: 0,
        month_index: 0,
        star_index: 0,
        year_index: 0,
        GroupOffset: [0; 65],
    };
    let mut encounter: DWORD = 0;
    let mut offset: DWORD = 0;
    let mut current: BYTE = 0;
    let mut i: ::core::ffi::c_int = 0;
    uqm_lua_checked!(read_32(fh as *mut ::core::ffi::c_void, &raw mut encounter), ());
    uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut current), ());
    chunksize = chunksize.wrapping_sub(5 as DWORD);
    if encounter != 0 {
        fp = OpenStateFile(
            2 as ::core::ffi::c_int,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        offset = LengthStateFile(fp);
        memset(
            &raw mut h as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GROUP_HEADER>(),
        );
    } else {
        fp = OpenStateFile(
            1 as ::core::ffi::c_int,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        current = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
        offset = 0 as DWORD;
        ReadGroupHeader(fp, &raw mut h);
    }
    if fp.is_null() {
        uqm_lua_checked!(skip_8(fh as *mut ::core::ffi::c_void, chunksize as COUNT), ());
        return;
    }
    uqm_lua_checked!(read_16(fh as *mut ::core::ffi::c_void, &raw mut h.star_index), ());
    uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut h.day_index), ());
    uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut h.month_index), ());
    uqm_lua_checked!(read_16(fh as *mut ::core::ffi::c_void, &raw mut h.year_index), ());
    uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut h.NumGroups), ());
    chunksize = chunksize.wrapping_sub(7 as DWORD);
    SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    WriteGroupHeader(fp, &raw mut h);
    i = 1 as ::core::ffi::c_int;
    while i <= h.NumGroups as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        let mut icon: BYTE = 0;
        let mut NumShips: BYTE = 0;
        uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut icon), ());
        uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut NumShips), ());
        chunksize = chunksize.wrapping_sub(2 as DWORD);
        h.GroupOffset[i as usize] = LengthStateFile(fp);
        SeekStateFile(
            fp,
            h.GroupOffset[i as usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        swrite_8(fp, icon);
        swrite_8(fp, NumShips);
        j = 0 as ::core::ffi::c_int;
        while j < NumShips as ::core::ffi::c_int {
            let mut race_outer: BYTE = 0;
            let mut sf: SHIP_FRAGMENT = SHIP_FRAGMENT {
                pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                SpeciesID: SPECIES_ID::NO_ID,
                captains_name_index: 0,
                race_id: 0,
                index: 0,
                crew_level: 0,
                max_crew: 0,
                energy_level: 0,
                max_energy: 0,
                race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
                icons: ::core::ptr::null_mut::<FRAME_DESC>(),
                melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut race_outer), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut sf.captains_name_index), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut sf.race_id), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut sf.index), ());
            uqm_lua_checked!(read_16(fh as *mut ::core::ffi::c_void, &raw mut sf.crew_level), ());
            uqm_lua_checked!(read_16(fh as *mut ::core::ffi::c_void, &raw mut sf.max_crew), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut sf.energy_level), ());
            uqm_lua_checked!(read_8(fh as *mut ::core::ffi::c_void, &raw mut sf.max_energy), ());
            chunksize = chunksize.wrapping_sub(10 as DWORD);
            swrite_8(fp, race_outer);
            WriteShipFragment(fp, &raw mut sf);
            j += 1;
        }
        i += 1;
    }
    SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    WriteGroupHeader(fp, &raw mut h);
    CloseStateFile(fp);
    if encounter != 0 {
        uqm_lua_checked!(SetBattleGroupOffset(encounter as ::core::ffi::c_int, offset), ());
        if current != 0 {
            GlobData.Game_state.BattleGroupRef = offset;
        }
    }
    if chunksize != 0 {
        log_add(
            log_Level::log_Warning,
            b"BattleGroup chunk mis-sized!\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn LoadCoreGame(
    mut which_game: COUNT,
    mut SummPtr: *mut SUMMARY_DESC,
) -> BOOLEAN {
    let mut in_fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut loc_sd: SUMMARY_DESC = SUMMARY_DESC {
        SS: SIS_STATE {
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
        Activity: 0,
        Flags: 0,
        day_index: 0,
        month_index: 0,
        year_index: 0,
        MCreditLo: 0,
        MCreditHi: 0,
        NumShips: 0,
        NumDevices: 0,
        ShipList: [0; 12],
        DeviceList: [0; 16],
        SaveName: [0; 64],
        SaveNameChecker: [0; 32],
        LegacySaveName: [0; 32],
        res_factor: 0,
    };
    sprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        b"uqmsave.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        which_game as ::core::ffi::c_int,
    );
    in_fp = uqm_lua_checked!(res_OpenResFile(
        saveDir,
        &raw mut file as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), BOOLEAN(0));
    if in_fp.is_null() {
        return uqm_lua_checked!(LoadLegacyGame(which_game, SummPtr, BOOLEAN::FALSE), BOOLEAN(0));
    }
    if uqm_lua_checked!(LoadSummary(&raw mut loc_sd, in_fp as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0)).0
        == 0
    {
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return uqm_lua_checked!(LoadLegacyGame(which_game, SummPtr, BOOLEAN::FALSE), BOOLEAN(0));
    }
    if SummPtr.is_null() {
        SummPtr = &raw mut loc_sd;
    } else {
        memcpy(
            SummPtr as *mut ::core::ffi::c_void,
            &raw mut loc_sd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SUMMARY_DESC>(),
        );
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    if uqm_lua_checked!(LoadGame(which_game, SummPtr, in_fp, BOOLEAN::TRUE), BOOLEAN(0)).0 == 0 {
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadGame(
    mut which_game: COUNT,
    mut SummPtr: *mut SUMMARY_DESC,
    mut in_fp: *mut uio_Stream,
    mut try_core: BOOLEAN,
) -> BOOLEAN {
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut loc_sd: SUMMARY_DESC = SUMMARY_DESC {
        SS: SIS_STATE {
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
        Activity: 0,
        Flags: 0,
        day_index: 0,
        month_index: 0,
        year_index: 0,
        MCreditLo: 0,
        MCreditHi: 0,
        NumShips: 0,
        NumDevices: 0,
        ShipList: [0; 12],
        DeviceList: [0; 16],
        SaveName: [0; 64],
        SaveNameChecker: [0; 32],
        LegacySaveName: [0; 32],
        res_factor: 0,
    };
    let mut num_links: COUNT = 0;
    let mut SD: STAR_DESC = STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    };
    let mut Activity: ACTIVITY = 0;
    let mut chunk: DWORD = 0;
    let mut chunkSize: DWORD = 0;
    let mut first_group_spec: BOOLEAN = BOOLEAN::TRUE;
    if try_core.0 == 0 {
        sprintf(
            &raw mut file as *mut ::core::ffi::c_char,
            b"uqmsave.%02u\0".as_ptr() as *const ::core::ffi::c_char,
            which_game as ::core::ffi::c_int,
        );
        in_fp = uqm_lua_checked!(res_OpenResFile(
            saveDir,
            &raw mut file as *mut ::core::ffi::c_char,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0));
        if in_fp.is_null() {
            return uqm_lua_checked!(LoadLegacyGame(which_game, SummPtr, BOOLEAN::FALSE), BOOLEAN(0));
        }
        if uqm_lua_checked!(LoadSummary(
                &raw mut loc_sd,
                in_fp as *mut ::core::ffi::c_void,
                BOOLEAN::FALSE,
            ), BOOLEAN(0))
            .0 == 0
        {
            uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
            return uqm_lua_checked!(LoadCoreGame(which_game, SummPtr), BOOLEAN(0));
        }
        if SummPtr.is_null() {
            SummPtr = &raw mut loc_sd;
        } else {
            memcpy(
                SummPtr as *mut ::core::ffi::c_void,
                &raw mut loc_sd as *const ::core::ffi::c_void,
                ::core::mem::size_of::<SUMMARY_DESC>(),
            );
            uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
            return BOOLEAN::TRUE;
        }
    }
    GlobData.SIS_state = (*SummPtr).SS;
    optCustomSeed = GlobData.SIS_state.Seed as ::core::ffi::c_int;
    optShipSeed = OPT_ENABLABLE(
        (if GlobData.SIS_state.ShipSeed as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(ReloadMasterShipList(None), BOOLEAN(0));
    uqm_lua_checked!(LoadFleetInfo(), BOOLEAN(0));
    ReinitQueue(&raw mut GlobData.Game_state.GameClock.event_q);
    ReinitQueue(&raw mut GlobData.Game_state.encounter_q);
    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
    ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    ReinitQueue(&raw mut GlobData.Game_state.built_ship_q);
    ReinitQueue(&raw mut GlobData.Game_state.stowed_ship_q);
    uninitEventSystem();
    uqm_lua_checked!(luaUqm_uninitState(), BOOLEAN(0));
    uqm_lua_checked!(luaUqm_initState(), BOOLEAN(0));
    uqm_lua_checked!(initEventSystem(), BOOLEAN(0));
    Activity = GlobData.Game_state.CurrentActivity;
    if uqm_lua_checked!(LoadGameState(
            &raw mut GlobData.Game_state,
            in_fp as *mut ::core::ffi::c_void,
            try_core,
        ), BOOLEAN(0))
        .0 == 0
    {
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    NextActivity = GlobData.Game_state.CurrentActivity;
    GlobData.Game_state.CurrentActivity = Activity;
    chunk = 0 as DWORD;
    while BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0 {
        if uqm_lua_checked!(read_32(in_fp as *mut ::core::ffi::c_void, &raw mut chunk), BOOLEAN(0)) != 1 as size_t {
            break;
        }
        if uqm_lua_checked!(read_32(in_fp as *mut ::core::ffi::c_void, &raw mut chunkSize), BOOLEAN(0)) != 1 as size_t
        {
            uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
            return BOOLEAN::FALSE;
        }
        match chunk {
            1365467474 => {
                uqm_lua_checked!(LoadRaceQueue(
                    in_fp as *mut ::core::ffi::c_void,
                    &raw mut GlobData.Game_state.avail_race_q,
                    chunkSize,
                ), BOOLEAN(0));
            }
            1366312777 => {
                uqm_lua_checked!(LoadGroupQueue(
                    in_fp as *mut ::core::ffi::c_void,
                    &raw mut GlobData.Game_state.ip_group_q,
                    chunkSize,
                ), BOOLEAN(0));
            }
            1952673349 => {
                num_links = chunkSize.wrapping_div(65 as DWORD) as COUNT;
                loop {
                    let c2rust_fresh0 = num_links;
                    num_links = num_links.wrapping_sub(1);
                    if c2rust_fresh0 == 0 {
                        break;
                    }
                    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<
                        ENCOUNTER,
                    >();
                    hEncounter = AllocLink(&raw mut GlobData.Game_state.encounter_q)
                        as HENCOUNTER;
                    EncounterPtr = LockLink(
                        &raw mut GlobData.Game_state.encounter_q,
                        hEncounter,
                    ) as *mut ENCOUNTER;
                    uqm_lua_checked!(LoadEncounter(
                        EncounterPtr,
                        in_fp as *mut ::core::ffi::c_void,
                        try_core,
                    ), BOOLEAN(0));
                    UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
                    PutQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
                }
            }
            1937012293 => {
                num_links = chunkSize.wrapping_div(5 as DWORD) as COUNT;
                loop {
                    let c2rust_fresh1 = num_links;
                    num_links = num_links.wrapping_sub(1);
                    if c2rust_fresh1 == 0 {
                        break;
                    }
                    let mut hEvent: HEVENT = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
                    let mut DeCleanse: BOOLEAN = BOOLEAN::FALSE;
                    hEvent = AllocLink(&raw mut GlobData.Game_state.GameClock.event_q)
                        as HEVENT;
                    EventPtr = LockLink(
                        &raw mut GlobData.Game_state.GameClock.event_q,
                        hEvent,
                    ) as *mut EVENT;
                    uqm_lua_checked!(LoadEvent(EventPtr, in_fp as *mut ::core::ffi::c_void), BOOLEAN(0));
                    if optDeCleansing.0 != 0
                        && (*EventPtr).func_index as ::core::ffi::c_int
                            == C2Rust_Unnamed_4::KOHR_AH_VICTORIOUS_EVENT.0
                                as ::core::ffi::c_int
                    {
                        UnlockLink(
                            &raw mut GlobData.Game_state.GameClock.event_q,
                            hEvent,
                        );
                        if (*EventPtr).year_index as ::core::ffi::c_int
                            == 2158 as ::core::ffi::c_int
                        {
                            FreeLink(
                                &raw mut GlobData.Game_state.GameClock.event_q,
                                hEvent,
                            );
                            DeCleanse = BOOLEAN::TRUE;
                        }
                    } else {
                        UnlockLink(
                            &raw mut GlobData.Game_state.GameClock.event_q,
                            hEvent,
                        );
                        PutQueue(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
                        if optDeCleansing.0 != 0 && DeCleanse.0 != 0 {
                            uqm_lua_checked!(AddEvent(
                                EVENT_TYPE::ABSOLUTE_EVENT,
                                2 as COUNT,
                                17 as COUNT,
                                (2155 as ::core::ffi::c_int
                                    + if optDeCleansing.0 != 0 {
                                        100 as ::core::ffi::c_int
                                    } else {
                                        4 as ::core::ffi::c_int
                                    }) as COUNT,
                                C2Rust_Unnamed_4::KOHR_AH_VICTORIOUS_EVENT.0
                                    as ::core::ffi::c_int as BYTE,
                            ), BOOLEAN(0));
                        }
                    }
                }
            }
            1918989395 => {
                uqm_lua_checked!(LoadStarDesc(&raw mut SD, in_fp as *mut ::core::ffi::c_void), BOOLEAN(0));
                uqm_lua_checked!(loadGameCheats(), BOOLEAN(0));
            }
            1365471310 => {
                uqm_lua_checked!(LoadShipQueue(
                    in_fp as *mut ::core::ffi::c_void,
                    &raw mut GlobData.Game_state.npc_built_ship_q,
                    chunkSize,
                ), BOOLEAN(0));
            }
            1366321235 => {
                uqm_lua_checked!(LoadShipQueue(
                    in_fp as *mut ::core::ffi::c_void,
                    &raw mut GlobData.Game_state.built_ship_q,
                    chunkSize,
                ), BOOLEAN(0));
            }
            1366783059 => {
                uqm_lua_checked!(LoadShipQueue(
                    in_fp as *mut ::core::ffi::c_void,
                    &raw mut GlobData.Game_state.stowed_ship_q,
                    chunkSize,
                ), BOOLEAN(0));
            }
            1851876179 => {
                uqm_lua_checked!(LoadScanInfo(in_fp, chunkSize), BOOLEAN(0));
            }
            1936749127 => {
                if first_group_spec.0 != 0 {
                    InitGroupInfo(BOOLEAN::TRUE);
                    GlobData.Game_state.BattleGroupRef = 0 as DWORD;
                    first_group_spec = BOOLEAN::FALSE;
                }
                uqm_lua_checked!(LoadGroupList(in_fp, chunkSize), BOOLEAN(0));
            }
            1883731010 => {
                if first_group_spec.0 != 0 {
                    InitGroupInfo(BOOLEAN::TRUE);
                    GlobData.Game_state.BattleGroupRef = 0 as DWORD;
                    first_group_spec = BOOLEAN::FALSE;
                }
                uqm_lua_checked!(LoadBattleGroup(in_fp, chunkSize), BOOLEAN(0));
            }
            _ => {
                log_add(
                    log_Level::log_Debug,
                    b"Skipping chunk of tag %08X (size %u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    chunk,
                    chunkSize,
                );
                if uqm_lua_checked!(skip_8(in_fp as *mut ::core::ffi::c_void, chunkSize as COUNT), BOOLEAN(0))
                    != 1 as size_t
                {
                    uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
                    return BOOLEAN::FALSE;
                }
            }
        }
    }
    uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
    EncounterGroup = 0 as BYTE;
    EncounterRace = -1 as SIZE;
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(0isize));
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(1isize));
    CurStarDescPtr = uqm_lua_checked!(FindStar(
        ::core::ptr::null_mut::<STAR_DESC>(),
        &raw mut SD.star_pt,
        0 as SIZE,
        0 as SIZE,
    ), BOOLEAN(0));
    if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_3::START_ENCOUNTER.0 as ::core::ffi::c_int == 0
        && NextActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_3::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_3::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
    }
    DebugKeyPressed = BOOLEAN::FALSE;
    optSeedType = uqm_lua_checked!(getGameStateUint(b"SEED_TYPE\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
        as ::core::ffi::c_int;
    if optSeedType == C2Rust_Unnamed::OPTVAL_PRIME.0 as ::core::ffi::c_int
        && optCustomSeed == 0 as ::core::ffi::c_int
    {
        optCustomSeed = 16807 as ::core::ffi::c_int;
        GlobData.SIS_state.Seed = optCustomSeed as SDWORD;
    } else if optSeedType == C2Rust_Unnamed::OPTVAL_PRIME.0 as ::core::ffi::c_int
        && optCustomSeed != 16807 as ::core::ffi::c_int
    {
        optSeedType = C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int;
        uqm_lua_checked!(setGameStateUint(
            b"SEED_TYPE\0".as_ptr() as *const ::core::ffi::c_char,
            optSeedType as DWORD,
        ), BOOLEAN(0));
    }
    return InitStarseed(BOOLEAN::FALSE);
}
#[inline]
unsafe extern "C" fn swrite_8(mut fp: *mut GAME_STATE_FILE, mut v: BYTE) -> COUNT {
    return WriteStateFile(
        &raw mut v as *const ::core::ffi::c_void,
        1 as COUNT,
        1 as COUNT,
        fp,
    ) as COUNT;
}
#[inline]
unsafe extern "C" fn swrite_32(mut fp: *mut GAME_STATE_FILE, mut v: DWORD) -> COUNT {
    return WriteStateFile(
        &raw mut v as *const ::core::ffi::c_void,
        4 as COUNT,
        1 as COUNT,
        fp,
    ) as COUNT;
}
#[inline]
unsafe extern "C" fn LockIpGroup(
    mut pq: *const QUEUE,
    mut h: HIPGROUP,
) -> *mut IP_GROUP {
    ((*pq).object_size as usize == ::core::mem::size_of::<IP_GROUP>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (IP_GROUP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
