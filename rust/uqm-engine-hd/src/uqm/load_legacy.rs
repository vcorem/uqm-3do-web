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

// Rust translation of src/uqm/load_legacy.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type frame_desc;
    pub type _LZHCODE_DESC;
    pub type GAME_STATE_FILE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn uio_backtrack(
        rewinded_bytes: ::core::ffi::c_int,
        stream: *mut uio_Stream,
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
    fn abort() -> !;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
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
    fn totalBitsForGameState(
        bm: *const GameStateBitMap,
        rev: ::core::ffi::c_int,
    ) -> size_t;
    fn deserialiseGameState(
        bm: *const GameStateBitMap,
        buf: *const BYTE,
        numBytes: size_t,
        rev: ::core::ffi::c_int,
    ) -> BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn InitStarseed(newgame: BOOLEAN) -> BOOLEAN;
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
    fn copen(
        InStream: *mut ::core::ffi::c_void,
        SType: STREAM_TYPE,
        SMode: STREAM_MODE,
    ) -> DECODE_REF;
    fn cclose(DecodeRef: DECODE_REF) -> DWORD;
    fn cread(
        pStr: *mut ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        DecodeRef: DECODE_REF,
    ) -> COUNT;
    static mut EncounterRace: SIZE;
    static mut EncounterGroup: BYTE;
    fn initEventSystem();
    fn uninitEventSystem();
    static mut NextActivity: ACTIVITY;
    static mut race_q: [QUEUE; 0];
    fn OpenStateFile(
        stateFile: ::core::ffi::c_int,
        mode: *const ::core::ffi::c_char,
    ) -> *mut GAME_STATE_FILE;
    fn CloseStateFile(fp: *mut GAME_STATE_FILE);
    fn WriteStateFile(
        lpBuf: *const ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        fp: *mut GAME_STATE_FILE,
    ) -> ::core::ffi::c_int;
    fn BuildGroup(pDstQueue: *mut QUEUE, race_id: BYTE) -> HIPGROUP;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut DebugKeyPressed: BOOLEAN;
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
pub struct GameStateBitMap {
    pub name: *const ::core::ffi::c_char,
    pub numBits: BYTE,
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
pub type DECODE_REF = *mut _LZHCODE_DESC;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const FILE_STREAM: Self = Self(0);
    pub const MEMORY_STREAM: Self = Self(1);
}
pub type STREAM_TYPE = BYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const STREAM_READ: Self = Self(0);
    pub const STREAM_WRITE: Self = Self(1);
}
pub type STREAM_MODE = BYTE;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub type HIPGROUP = HLINK;
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
pub static mut LegacyResFactor: BYTE = 0;
static mut legacyGameStateBitMap: [GameStateBitMap; 406] = [
    GameStateBitMap {
        name: b"SHOFIXTI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_KIA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_BRO_KIA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_RECRUITED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLANETARY_LANDING\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_SHIELDED_SELVES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_CREATURES_EXAMINED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_CREATURES_ELIMINATED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_BROADCASTERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_QUEST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"LIED_ABOUT_CREATURES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_PARTY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SPATHI_PASSWORD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_CHMMR_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_SPACE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME0_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME1_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME2_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME3_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME4_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME5_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME6_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME7_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME8_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_MELNORME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_RESCUE_REFUSED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_RESCUE_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TRADED_WITH_MELNORME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"WHY_MELNORME_PURPLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_CREDIT0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_CREDIT1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_BUSINESS_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"WHY_MELNORME_BLUE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_ANGER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_MIFFED_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_PISSED_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_HATE_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_PROBE_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"PROBE_MESSAGE_DELIVERED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"RADIOACTIVES_PROVIDED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_CAVALRY_ARRIVED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_MESSED_UP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"WILL_DESTROY_BASE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_KILLED_ALL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_PORTAL_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_SUN_EFFICIENCY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_UMGAH_HYPERWAVE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_BURVIX_HYPERWAVE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_WANT_PROOF\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_HAVING_SEX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_ARILOU\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_TAALO_PROTECTOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_EGG_CASING0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_EGG_CASING1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_EGG_CASING2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_SYREEN_SHUTTLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_VUX_BEAST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_DESTRUCT_CODE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_URQUAN_WARP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_WIMBLIS_TRIDENT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_GLOWING_ROD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ATTACKED_DRUUGE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NEW_ALLIANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"BURVIXESE_BROADCASTERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUN_DEVICE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_PROTECTOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHIP_VAULT_UNLOCKED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_SHUTTLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_KEY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_KEY_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_BEAST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_BYES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_DECEIVED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"FLAGSHIP_CLOAKED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_FELL_FOR_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PROBE_EXHIBITED_BUG\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_HYPNOTIZED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_IS_DEAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_ZEX_WANTS_MONSTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_HAVE_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_ZOMBIE_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_UMGAH_ZOMBIES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_ARILOU_WANT_WRECK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_CHECKED_UMGAH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_NORMAL_UMGAH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SYREEN_VAULT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUN_DEVICE_UNGUARDED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_EMERGING\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_UNLEASHED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_DISCLAIMER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_REBEL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_CIVIL_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_ABSORBED_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"HELIX_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_FRAGMENTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_CASTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_ZOQFOT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_SUN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_FUEL_PROCEDURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_TECH_PROCEDURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_INFO_PROCEDURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_TECH_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_EVENTS_INFO_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_ALIEN_INFO_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_HISTORY_INFO_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"RAINBOW_WORLD0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"RAINBOW_WORLD1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_RAINBOW_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BROADCASTER_RESPONSE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_ORZ_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_REBEL_TOLD_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_HAD_SEX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"LIGHT_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MEDIUM_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"HEAVY_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_BULLETS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_SOLD_TO_DRUUGE0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_PROTECTING_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"COLONY_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADDASH_BODY_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_GODS_SPOKEN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_WORSHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SAMATRA_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"READY_TO_CONFUSE_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_HYPNO_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MENTIONED_PET_COMPULSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_URQUAN_STORY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_RAMBLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_INSULTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_KNOW_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SYREEN_WORLD_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_KNOW_ABOUT_MYCON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_SUGGESTIONS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"LEARNED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DNYARRI_LIED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_GENERAL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_PERSONAL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_ANDRO_STATE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"REFUSED_ORZ_ALLIANCE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_ON_THE_MOVE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_FLEET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MIGRATE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_RETURN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_WORRY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_FORTUNE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MIGRATE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SWITCH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SENSE_VICTOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_PLEAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_KOHR_AH_STORY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_SENSES_EVIL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_SENSES_EVIL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_THREAT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_WRONG\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_ID\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_EXIT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_EVIL_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_MENTIONED_TRICKS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_CARRIER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_BODY_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_MISSION_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_5\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_PORTAL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_5\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_HINTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_SPACE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_HOME_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_SALVAGE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"FRAGMENTS_BOUGHT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_REBEL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_ROYALIST_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_ROYALIST_TOLD_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_ALLY_HOME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_HELP_HOME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_ALLY_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_HELP_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_KNOW_MASK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_BROKEN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_PROBE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_PROGRAM\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_EFFECTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_PRIORITY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK5\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK6\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK7\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK9\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_EARTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_EXPLORE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_GATHER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"RECALL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SPATHI_QUEST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SPATHI_EVIL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_SOLD_TO_DRUUGE1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_DONE_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"REFUSED_ULTRON_AT_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_TRICK_AT_SUN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AUTOPILOT_OK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 0 as BYTE,
    },
];
#[inline]
unsafe extern "C" fn cread_8(mut fh: DECODE_REF, mut v: *mut BYTE) -> COUNT {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return uqm_lua_checked!(cread(v as *mut ::core::ffi::c_void, 1 as COUNT, 1 as COUNT, fh), 0);
}
#[inline]
unsafe extern "C" fn cread_16(mut fh: DECODE_REF, mut v: *mut UWORD) -> COUNT {
    let mut t: UWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return uqm_lua_checked!(cread(v as *mut ::core::ffi::c_void, 2 as COUNT, 1 as COUNT, fh), 0);
}
#[inline]
unsafe extern "C" fn cread_16s(mut fh: DECODE_REF, mut v: *mut SWORD) -> COUNT {
    let mut t: UWORD = 0;
    let mut ret: COUNT = 0;
    ret = uqm_lua_checked!(cread_16(fh, &raw mut t), 0);
    if !v.is_null() {
        *v = t as SWORD;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn cread_32(mut fh: DECODE_REF, mut v: *mut DWORD) -> COUNT {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return uqm_lua_checked!(cread(v as *mut ::core::ffi::c_void, 4 as COUNT, 1 as COUNT, fh), 0);
}
#[inline]
unsafe extern "C" fn cread_32s(mut fh: DECODE_REF, mut v: *mut SDWORD) -> COUNT {
    let mut t: DWORD = 0;
    let mut ret: COUNT = 0;
    ret = uqm_lua_checked!(cread_32(fh, &raw mut t), 0);
    if !v.is_null() {
        *v = t as SDWORD;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn cread_ptr(mut fh: DECODE_REF) -> COUNT {
    let mut t: DWORD = 0;
    return uqm_lua_checked!(cread_32(fh, &raw mut t), 0);
}
#[inline]
unsafe extern "C" fn cread_a8(
    mut fh: DECODE_REF,
    mut ar: *mut BYTE,
    mut count: COUNT,
) -> COUNT {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/load_legacy.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                525 as ::core::ffi::c_int,
                b"cread_a8\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return (uqm_lua_checked!(cread(ar as *mut ::core::ffi::c_void, 1 as COUNT, count, fh), 0)
        as ::core::ffi::c_int == count as ::core::ffi::c_int) as ::core::ffi::c_int
        as COUNT;
}
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
    let mut t: UWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return uqm_lua_checked!(ReadResFile(
        v as *mut ::core::ffi::c_void,
        2 as size_t,
        1 as size_t,
        fp as *mut uio_Stream,
    ), 0);
}
#[inline]
unsafe extern "C" fn read_32(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut DWORD,
) -> size_t {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return uqm_lua_checked!(ReadResFile(
        v as *mut ::core::ffi::c_void,
        4 as size_t,
        1 as size_t,
        fp as *mut uio_Stream,
    ), 0);
}
#[inline]
unsafe extern "C" fn read_32s(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut SDWORD,
) -> size_t {
    let mut t: DWORD = 0;
    let mut ret: COUNT = 0;
    ret = uqm_lua_checked!(read_32(fp, &raw mut t), 0) as COUNT;
    if !v.is_null() {
        *v = t as SDWORD;
    }
    return ret as size_t;
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
                b"src/uqm/load_legacy.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                579 as ::core::ffi::c_int,
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
                b"src/uqm/load_legacy.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                593 as ::core::ffi::c_int,
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
unsafe extern "C" fn LoadEmptyQueue(mut fh: DECODE_REF) {
    let mut num_links: COUNT = 0;
    uqm_lua_checked!(cread_16(fh, &raw mut num_links), ());
    if num_links != 0 {
        log_add(
            log_Level::log_Error,
            b"LoadEmptyQueue(): BUG: the queue is not empty!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        explode();
    }
}
unsafe extern "C" fn LoadShipQueue(mut fh: DECODE_REF, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    uqm_lua_checked!(cread_16(fh, &raw mut num_links), ());
    loop {
        let c2rust_fresh2 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        let mut Index: COUNT = 0;
        let mut tmpb: BYTE = 0;
        uqm_lua_checked!(cread_16(fh, &raw mut Index), ());
        hStarShip = CloneShipFragment(
            RACE_ID(Index as ::core::ffi::c_uint),
            pQueue,
            0 as COUNT,
        );
        FragPtr = LockShipFrag(pQueue, hStarShip);
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FragPtr).captains_name_index), ());
        uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FragPtr).race_id), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FragPtr).index), ());
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*FragPtr).crew_level = tmpb as COUNT;
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*FragPtr).max_crew = tmpb as COUNT;
        uqm_lua_checked!(cread_8(fh, &raw mut (*FragPtr).energy_level), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FragPtr).max_energy), ());
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        UnlockLink(pQueue, hStarShip);
    };
}
unsafe extern "C" fn LoadRaceQueue(mut fh: DECODE_REF, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    uqm_lua_checked!(cread_16(fh, &raw mut num_links), ());
    loop {
        let c2rust_fresh4 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh4 == 0 {
            break;
        }
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut Index: COUNT = 0;
        let mut tmpb: BYTE = 0;
        uqm_lua_checked!(cread_16(fh, &raw mut Index), ());
        hStarShip = GetStarShipFromIndex(pQueue, Index) as HFLEETINFO;
        FleetPtr = LockFleetInfo(pQueue, hStarShip);
        uqm_lua_checked!(cread_16(fh, &raw mut (*FleetPtr).allied_state), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FleetPtr).days_left), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FleetPtr).growth_fract), ());
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*FleetPtr).crew_level = tmpb as COUNT;
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*FleetPtr).max_crew = tmpb as COUNT;
        uqm_lua_checked!(cread_8(fh, &raw mut (*FleetPtr).growth), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FleetPtr).max_energy), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*FleetPtr).loc.x), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*FleetPtr).loc.y), ());
        uqm_lua_checked!(cread_16(fh, &raw mut (*FleetPtr).actual_strength), ());
        uqm_lua_checked!(cread_16(fh, &raw mut (*FleetPtr).known_strength), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*FleetPtr).known_loc.x), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*FleetPtr).known_loc.y), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FleetPtr).growth_err_term), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*FleetPtr).func_index), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*FleetPtr).dest_loc.x), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*FleetPtr).dest_loc.y), ());
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        UnlockLink(pQueue, hStarShip);
    };
}
unsafe extern "C" fn LoadGroupQueue(mut fh: DECODE_REF, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    uqm_lua_checked!(cread_16(fh, &raw mut num_links), ());
    loop {
        let c2rust_fresh3 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        let mut tmpb: BYTE = 0;
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        hGroup = BuildGroup(pQueue, 0 as BYTE);
        GroupPtr = LockIpGroup(pQueue, hGroup);
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
        uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
        uqm_lua_checked!(cread_16(fh, &raw mut (*GroupPtr).group_counter), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*GroupPtr).race_id), ());
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*GroupPtr).sys_loc = (tmpb as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*GroupPtr).task = (tmpb as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(cread_8(fh, &raw mut (*GroupPtr).in_system), ());
        uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*GroupPtr).dest_loc = (tmpb as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*GroupPtr).orbit_pos = (tmpb as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(cread_8(fh, &raw mut (*GroupPtr).group_id), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*GroupPtr).loc.x), ());
        uqm_lua_checked!(cread_16s(fh, &raw mut (*GroupPtr).loc.y), ());
        UnlockLink(pQueue, hGroup);
    };
}
unsafe extern "C" fn LoadEncounter(
    mut EncounterPtr: *mut ENCOUNTER,
    mut fh: DECODE_REF,
    mut try_vanilla: BOOLEAN,
) {
    let mut i: COUNT = 0;
    let mut tmpb: BYTE = 0;
    uqm_lua_checked!(cread_ptr(fh), ());
    (*EncounterPtr).pred = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(cread_ptr(fh), ());
    (*EncounterPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(cread_ptr(fh), ());
    (*EncounterPtr).hElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(cread_16s(fh, &raw mut (*EncounterPtr).transition_state), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*EncounterPtr).origin.x), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*EncounterPtr).origin.y), ());
    uqm_lua_checked!(cread_16(fh, &raw mut (*EncounterPtr).radius), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*EncounterPtr).loc_pt.x), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*EncounterPtr).loc_pt.y), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*EncounterPtr).race_id), ());
    uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
    (*EncounterPtr).num_ships = (tmpb as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
        as BYTE;
    (*EncounterPtr).flags = (tmpb as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
        as BYTE;
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
        let mut ShipInfo: *mut BRIEF_SHIP_INFO = (&raw mut (*EncounterPtr).ShipList
            as *mut BRIEF_SHIP_INFO)
            .offset(i as isize);
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*ShipInfo).race_id), ());
        uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*ShipInfo).crew_level = tmpb as COUNT;
        uqm_lua_checked!(cread_8(fh, &raw mut tmpb), ());
        (*ShipInfo).max_crew = tmpb as COUNT;
        uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
        uqm_lua_checked!(cread_8(fh, &raw mut (*ShipInfo).max_energy), ());
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
        uqm_lua_checked!(cread_32(fh, ::core::ptr::null_mut::<DWORD>()), ());
        uqm_lua_checked!(cread_ptr(fh), ());
        uqm_lua_checked!(cread_ptr(fh), ());
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(cread_32s(fh, &raw mut (*EncounterPtr).log_x), ());
    uqm_lua_checked!(cread_32s(fh, &raw mut (*EncounterPtr).log_y), ());
    if try_vanilla.0 != 0 {
        (*EncounterPtr).log_x = universeToLogx(
            oldLogxToUniverse((*EncounterPtr).log_x) as COORD,
        );
        (*EncounterPtr).log_y = universeToLogy(
            oldLogyToUniverse((*EncounterPtr).log_y) as COORD,
        );
    } else {
        (*EncounterPtr).log_x <<= resolutionFactor;
        (*EncounterPtr).log_y <<= resolutionFactor;
    };
}
unsafe extern "C" fn LoadEvent(mut EventPtr: *mut EVENT, mut fh: DECODE_REF) {
    uqm_lua_checked!(cread_ptr(fh), ());
    (*EventPtr).pred = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(cread_ptr(fh), ());
    (*EventPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(cread_8(fh, &raw mut (*EventPtr).day_index), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*EventPtr).month_index), ());
    uqm_lua_checked!(cread_16(fh, &raw mut (*EventPtr).year_index), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*EventPtr).func_index), ());
    uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
}
unsafe extern "C" fn DummyLoadQueue(mut QueuePtr: *mut QUEUE, mut fh: DECODE_REF) {
    uqm_lua_checked!(cread_ptr(fh), ());
    uqm_lua_checked!(cread_ptr(fh), ());
    uqm_lua_checked!(cread_ptr(fh), ());
    uqm_lua_checked!(cread_ptr(fh), ());
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
    uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
    uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), ());
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), ());
}
unsafe extern "C" fn LoadClockState(mut ClockPtr: *mut CLOCK_STATE, mut fh: DECODE_REF) {
    uqm_lua_checked!(cread_8(fh, &raw mut (*ClockPtr).day_index), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*ClockPtr).month_index), ());
    uqm_lua_checked!(cread_16(fh, &raw mut (*ClockPtr).year_index), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*ClockPtr).tick_count), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*ClockPtr).day_in_ticks), ());
    uqm_lua_checked!(cread_ptr(fh), ());
    uqm_lua_checked!(cread_ptr(fh), ());
    uqm_lua_checked!(cread_32(fh, ::core::ptr::null_mut::<DWORD>()), ());
    uqm_lua_checked!(DummyLoadQueue(&raw mut (*ClockPtr).event_q, fh), ());
}
unsafe extern "C" fn LoadGameState(
    mut GSPtr: *mut GAME_STATE,
    mut fh: DECODE_REF,
    mut vanilla: BOOLEAN,
) -> BOOLEAN {
    let mut dummy8: BYTE = 0;
    uqm_lua_checked!(cread_8(fh, &raw mut dummy8), BOOLEAN(0));
    uqm_lua_checked!(cread_8(fh, &raw mut (*GSPtr).glob_flags), BOOLEAN(0));
    uqm_lua_checked!(cread_8(fh, &raw mut (*GSPtr).CrewCost), BOOLEAN(0));
    uqm_lua_checked!(cread_8(fh, &raw mut (*GSPtr).FuelCost), BOOLEAN(0));
    (*GSPtr).glob_flags = (5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as BYTE;
    if (*GSPtr).FuelCost as ::core::ffi::c_int != 20 as ::core::ffi::c_int {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(cread_a8(
        fh,
        &raw mut (*GSPtr).ModuleCost as *mut BYTE,
        C2Rust_Unnamed_1::NUM_MODULES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0));
    uqm_lua_checked!(cread_a8(
        fh,
        &raw mut (*GSPtr).ElementWorth as *mut BYTE,
        C2Rust_Unnamed_0::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0));
    uqm_lua_checked!(cread_ptr(fh), BOOLEAN(0));
    uqm_lua_checked!(cread_16(fh, &raw mut (*GSPtr).CurrentActivity), BOOLEAN(0));
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), BOOLEAN(0));
    uqm_lua_checked!(LoadClockState(&raw mut (*GSPtr).GameClock, fh), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).autopilot.x), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).autopilot.y), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).ip_location.x), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).ip_location.y), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).ShipStamp.origin.x), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).ShipStamp.origin.y), BOOLEAN(0));
    uqm_lua_checked!(cread_16(fh, &raw mut (*GSPtr).ShipFacing), BOOLEAN(0));
    uqm_lua_checked!(cread_8(fh, &raw mut (*GSPtr).ip_planet), BOOLEAN(0));
    uqm_lua_checked!(cread_8(fh, &raw mut (*GSPtr).in_orbit), BOOLEAN(0));
    (*GSPtr).ShipStamp.origin.x = (((*GSPtr).ShipStamp.origin.x as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    (*GSPtr).ShipStamp.origin.y = (((*GSPtr).ShipStamp.origin.y as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    uqm_lua_checked!(cread_16(fh, &raw mut (*GSPtr).velocity.TravelAngle), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.vector.width), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.vector.height), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.fract.width), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.fract.height), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.error.width), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.error.height), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.incr.width), BOOLEAN(0));
    uqm_lua_checked!(cread_16s(fh, &raw mut (*GSPtr).velocity.incr.height), BOOLEAN(0));
    uqm_lua_checked!(cread_16(fh, ::core::ptr::null_mut::<UWORD>()), BOOLEAN(0));
    if (*GSPtr).CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
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
    uqm_lua_checked!(cread_32(fh, &raw mut (*GSPtr).BattleGroupRef), BOOLEAN(0));
    uqm_lua_checked!(DummyLoadQueue(&raw mut (*GSPtr).avail_race_q, fh), BOOLEAN(0));
    uqm_lua_checked!(DummyLoadQueue(&raw mut (*GSPtr).npc_built_ship_q, fh), BOOLEAN(0));
    uqm_lua_checked!(DummyLoadQueue(&raw mut (*GSPtr).encounter_q, fh), BOOLEAN(0));
    uqm_lua_checked!(DummyLoadQueue(&raw mut (*GSPtr).built_ship_q, fh), BOOLEAN(0));
    let mut rev: ::core::ffi::c_int = if vanilla.0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    let mut numBytes: size_t = totalBitsForGameState(
            &raw const legacyGameStateBitMap as *const GameStateBitMap,
            rev,
        )
        .wrapping_add(7 as size_t) >> 3 as ::core::ffi::c_int;
    let mut buf: *mut BYTE = HMalloc(numBytes) as *mut BYTE;
    if !buf.is_null() {
        uqm_lua_checked!(cread_a8(fh, buf, numBytes as COUNT), BOOLEAN(0));
        uqm_lua_checked!(deserialiseGameState(
            &raw const legacyGameStateBitMap as *const GameStateBitMap,
            buf,
            numBytes,
            rev,
        ), BOOLEAN(0));
        HFree(buf as *mut ::core::ffi::c_void);
    }
    (*GSPtr).glob_flags = (5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as BYTE;
    uqm_lua_checked!(ZeroLastLoc(), BOOLEAN(0));
    uqm_lua_checked!(ZeroAdvancedAutoPilot(), BOOLEAN(0));
    uqm_lua_checked!(ZeroAdvancedQuasiPilot(), BOOLEAN(0));
    uqm_lua_checked!(cread_8(fh, ::core::ptr::null_mut::<BYTE>()), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn LoadSisState(
    mut SSPtr: *mut SIS_STATE,
    mut fp: *mut ::core::ffi::c_void,
) -> BOOLEAN {
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
            C2Rust_Unnamed_0::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
        ), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SSPtr).ShipName as *mut ::core::ffi::c_char,
            16 as COUNT,
        ), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SSPtr).CommanderName as *mut ::core::ffi::c_char,
            16 as COUNT,
        ), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SSPtr).PlanetName as *mut ::core::ffi::c_char,
            16 as COUNT,
        ), BOOLEAN(0)) != 1 as size_t || uqm_lua_checked!(read_16(fp, ::core::ptr::null_mut::<UWORD>()), BOOLEAN(0)) != 1 as size_t
    {
        return BOOLEAN::FALSE
    } else {
        (*SSPtr).log_x <<= resolutionFactor;
        (*SSPtr).log_y <<= resolutionFactor;
        return BOOLEAN::TRUE;
    };
}
unsafe extern "C" fn LoadSummary(
    mut SummPtr: *mut SUMMARY_DESC,
    mut fp: *mut ::core::ffi::c_void,
    mut try_vanilla: BOOLEAN,
) -> BOOLEAN {
    let mut temp_log_x: SDWORD = 0 as SDWORD;
    let mut temp_log_y: SDWORD = 0 as SDWORD;
    let mut temp_ru: DWORD = 0 as DWORD;
    let mut temp_fuel: DWORD = 0 as DWORD;
    let mut no_savename: BOOLEAN = BOOLEAN::FALSE;
    uqm_lua_checked!(read_str(
        fp,
        &raw mut (*SummPtr).SaveNameChecker as *mut ::core::ffi::c_char,
        16 as COUNT,
    ), BOOLEAN(0));
    if strncmp(
        &raw mut (*SummPtr).SaveNameChecker as *mut UNICODE,
        b"superbutcherX !\0".as_ptr() as *const ::core::ffi::c_char,
        16 as size_t,
    ) != 0
    {
        let mut i: COUNT = 0;
        no_savename = BOOLEAN::TRUE;
        memcpy(
            &raw mut temp_log_x as *mut ::core::ffi::c_void,
            &raw mut (*SummPtr).SaveNameChecker as *mut UNICODE
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SDWORD>(),
        );
        memcpy(
            &raw mut temp_log_y as *mut ::core::ffi::c_void,
            (&raw mut (*SummPtr).SaveNameChecker as *mut UNICODE)
                .offset(::core::mem::size_of::<SDWORD>() as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SDWORD>(),
        );
        memcpy(
            &raw mut temp_ru as *mut ::core::ffi::c_void,
            (&raw mut (*SummPtr).SaveNameChecker as *mut UNICODE)
                .offset(2usize.wrapping_mul(::core::mem::size_of::<SDWORD>()) as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<DWORD>(),
        );
        memcpy(
            &raw mut temp_fuel as *mut ::core::ffi::c_void,
            (&raw mut (*SummPtr).SaveNameChecker as *mut UNICODE)
                .offset(
                    2usize
                        .wrapping_mul(::core::mem::size_of::<SDWORD>())
                        .wrapping_add(::core::mem::size_of::<DWORD>()) as isize,
                ) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<DWORD>(),
        );
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            uio_backtrack(1 as ::core::ffi::c_int, fp as *mut uio_Stream);
            i = i.wrapping_add(1);
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            (*SummPtr).SaveNameChecker[i as usize] = 0 as UNICODE;
            i = i.wrapping_add(1);
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            (*SummPtr).LegacySaveName[i as usize] = 0 as UNICODE;
            i = i.wrapping_add(1);
        }
    } else {
        uqm_lua_checked!(read_str(
            fp,
            &raw mut (*SummPtr).LegacySaveName as *mut ::core::ffi::c_char,
            32 as COUNT,
        ), BOOLEAN(0));
    }
    if uqm_lua_checked!(LoadSisState(&raw mut (*SummPtr).SS, fp), BOOLEAN(0)).0 == 0 {
        return BOOLEAN::FALSE;
    }
    (*SummPtr).SS.Difficulty = 0 as BYTE;
    (*SummPtr).SS.Seed = (*SummPtr).SS.Difficulty as SDWORD;
    (*SummPtr).SS.Nomad = 0 as BYTE;
    (*SummPtr).SS.Extended = (*SummPtr).SS.Nomad;
    (*SummPtr).SS.ShipSeed = 0 as BYTE;
    (*SummPtr).SS.SaveVersion = (if no_savename.0 != 0 {
        2 as ::core::ffi::c_int
    } else {
        3 as ::core::ffi::c_int
    }) as BYTE;
    if no_savename.0 != 0 {
        (*SummPtr).SS.log_x = universeToLogx(oldLogxToUniverse(temp_log_x) as COORD);
        (*SummPtr).SS.log_y = universeToLogy(oldLogyToUniverse(temp_log_y) as COORD);
        (*SummPtr).SS.ResUnits = temp_ru;
        (*SummPtr).SS.FuelOnBoard = temp_fuel;
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
            != 1 as size_t || uqm_lua_checked!(read_8(fp, &raw mut (*SummPtr).res_factor), BOOLEAN(0)) != 1 as size_t
        || uqm_lua_checked!(read_8(fp, ::core::ptr::null_mut::<BYTE>()), BOOLEAN(0)) != 1 as size_t
    {
        LegacyResFactor = (if try_vanilla.0 == 0 {
            (*SummPtr).res_factor as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as BYTE;
        return BOOLEAN::FALSE;
    } else {
        if try_vanilla.0 == 0 {
            uqm_lua_checked!(read_8(fp, ::core::ptr::null_mut::<BYTE>()), BOOLEAN(0));
        }
        return BOOLEAN::TRUE;
    };
}
unsafe extern "C" fn LoadStarDesc(mut SDPtr: *mut STAR_DESC, mut fh: DECODE_REF) {
    uqm_lua_checked!(cread_16s(fh, &raw mut (*SDPtr).star_pt.x), ());
    uqm_lua_checked!(cread_16s(fh, &raw mut (*SDPtr).star_pt.y), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*SDPtr).Type), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*SDPtr).Index), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*SDPtr).Prefix), ());
    uqm_lua_checked!(cread_8(fh, &raw mut (*SDPtr).Postfix), ());
}
#[no_mangle]
pub unsafe extern "C" fn LoadLegacyGame(
    mut which_game: COUNT,
    mut SummPtr: *mut SUMMARY_DESC,
    mut try_vanilla: BOOLEAN,
) -> BOOLEAN {
    let mut in_fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
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
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    let mut fh: DECODE_REF = ::core::ptr::null_mut::<_LZHCODE_DESC>();
    let mut num_links: COUNT = 0;
    let mut SD: STAR_DESC = STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    };
    let mut Activity: ACTIVITY = 0;
    sprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        b"starcon2.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        which_game as ::core::ffi::c_int,
    );
    in_fp = uqm_lua_checked!(res_OpenResFile(
        saveDir,
        &raw mut file as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), BOOLEAN(0));
    if in_fp.is_null() {
        return BOOLEAN::FALSE;
    }
    loc_sd.SaveName[0usize] = '\0' as UNICODE;
    if uqm_lua_checked!(LoadSummary(&raw mut loc_sd, in_fp as *mut ::core::ffi::c_void, try_vanilla), BOOLEAN(0)).0
        == 0
    {
        log_add(
            log_Level::log_Error,
            b"Warning: Savegame is corrupt\0".as_ptr() as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return BOOLEAN::FALSE;
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
    if ((*SummPtr).year_index as ::core::ffi::c_int) < 2155 as ::core::ffi::c_int
        || (*SummPtr).year_index as ::core::ffi::c_int
            >= 2155 as ::core::ffi::c_int
                + if optDeCleansing.0 != 0 {
                    100 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                } + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 25 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"Warning: Savegame corrupt or from an incompatible platform.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    GlobData.SIS_state = (*SummPtr).SS;
    fh = uqm_lua_checked!(copen(
        in_fp as *mut ::core::ffi::c_void,
        C2Rust_Unnamed_3::FILE_STREAM.0 as ::core::ffi::c_int as STREAM_TYPE,
        C2Rust_Unnamed_4::STREAM_READ.0 as ::core::ffi::c_int as STREAM_MODE,
    ), BOOLEAN(0));
    if fh.is_null() {
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
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
    if uqm_lua_checked!(LoadGameState(&raw mut GlobData.Game_state, fh, try_vanilla), BOOLEAN(0)).0 == 0 {
        uqm_lua_checked!(res_CloseResFile(in_fp), BOOLEAN(0));
        if try_vanilla.0 == 0 {
            uqm_lua_checked!(LoadLegacyGame(
                which_game,
                ::core::ptr::null_mut::<SUMMARY_DESC>(),
                BOOLEAN::TRUE,
            ), BOOLEAN(0));
            return BOOLEAN::TRUE;
        } else {
            return BOOLEAN::FALSE
        }
    }
    NextActivity = GlobData.Game_state.CurrentActivity;
    GlobData.Game_state.CurrentActivity = Activity;
    uqm_lua_checked!(LoadRaceQueue(fh, &raw mut GlobData.Game_state.avail_race_q), BOOLEAN(0));
    if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int == 0
    {
        if NextActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(LoadShipQueue(fh, &raw mut GlobData.Game_state.npc_built_ship_q), BOOLEAN(0));
        } else if NextActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(LoadGroupQueue(fh, &raw mut GlobData.Game_state.ip_group_q), BOOLEAN(0));
        } else {
            uqm_lua_checked!(LoadEmptyQueue(fh), BOOLEAN(0));
        }
    }
    uqm_lua_checked!(LoadShipQueue(fh, &raw mut GlobData.Game_state.built_ship_q), BOOLEAN(0));
    uqm_lua_checked!(cread_16(fh, &raw mut num_links), BOOLEAN(0));
    loop {
        let c2rust_fresh0 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
        let mut DeCleanse: BOOLEAN = BOOLEAN::FALSE;
        hEvent = AllocLink(&raw mut GlobData.Game_state.GameClock.event_q) as HEVENT;
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
            as *mut EVENT;
        uqm_lua_checked!(LoadEvent(EventPtr, fh), BOOLEAN(0));
        if optDeCleansing.0 != 0
            && (*EventPtr).func_index as ::core::ffi::c_int
                == C2Rust_Unnamed_5::KOHR_AH_VICTORIOUS_EVENT.0 as ::core::ffi::c_int
        {
            UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
            printf(
                b"EventPtr->year_index: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*EventPtr).year_index as ::core::ffi::c_int,
            );
            if (*EventPtr).year_index as ::core::ffi::c_int == 2158 as ::core::ffi::c_int
            {
                FreeLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
                DeCleanse = BOOLEAN::TRUE;
            }
        } else {
            UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
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
                    C2Rust_Unnamed_5::KOHR_AH_VICTORIOUS_EVENT.0 as ::core::ffi::c_int
                        as BYTE,
                ), BOOLEAN(0));
            }
        }
    }
    uqm_lua_checked!(cread_16(fh, &raw mut num_links), BOOLEAN(0));
    loop {
        let c2rust_fresh1 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        hEncounter = AllocLink(&raw mut GlobData.Game_state.encounter_q) as HENCOUNTER;
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        uqm_lua_checked!(LoadEncounter(EncounterPtr, fh, try_vanilla), BOOLEAN(0));
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        PutQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
    }
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen: DWORD = 0;
        uqm_lua_checked!(cread_32(fh, &raw mut flen), BOOLEAN(0));
        while flen != 0 {
            let mut num_bytes: COUNT = 0;
            num_bytes = (if flen as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            {
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            } else {
                flen as COUNT as usize
            }) as COUNT;
            uqm_lua_checked!(cread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                num_bytes,
                1 as COUNT,
                fh,
            ), BOOLEAN(0));
            WriteStateFile(
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                num_bytes,
                1 as COUNT,
                fp,
            );
            flen = flen.wrapping_sub(num_bytes as DWORD);
        }
        CloseStateFile(fp);
    }
    fp = OpenStateFile(
        2 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen_0: DWORD = 0;
        uqm_lua_checked!(cread_32(fh, &raw mut flen_0), BOOLEAN(0));
        while flen_0 != 0 {
            let mut num_bytes_0: COUNT = 0;
            num_bytes_0 = (if flen_0 as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            {
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            } else {
                flen_0 as COUNT as usize
            }) as COUNT;
            uqm_lua_checked!(cread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                num_bytes_0,
                1 as COUNT,
                fh,
            ), BOOLEAN(0));
            WriteStateFile(
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                num_bytes_0,
                1 as COUNT,
                fp,
            );
            flen_0 = flen_0.wrapping_sub(num_bytes_0 as DWORD);
        }
        CloseStateFile(fp);
    }
    fp = OpenStateFile(
        1 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen_1: DWORD = 0;
        uqm_lua_checked!(cread_32(fh, &raw mut flen_1), BOOLEAN(0));
        while flen_1 != 0 {
            let mut num_bytes_1: COUNT = 0;
            num_bytes_1 = (if flen_1 as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            {
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            } else {
                flen_1 as COUNT as usize
            }) as COUNT;
            uqm_lua_checked!(cread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                num_bytes_1,
                1 as COUNT,
                fh,
            ), BOOLEAN(0));
            WriteStateFile(
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                num_bytes_1,
                1 as COUNT,
                fp,
            );
            flen_1 = flen_1.wrapping_sub(num_bytes_1 as DWORD);
        }
        CloseStateFile(fp);
    }
    uqm_lua_checked!(LoadStarDesc(&raw mut SD, fh), BOOLEAN(0));
    uqm_lua_checked!(loadGameCheats(), BOOLEAN(0));
    uqm_lua_checked!(cclose(fh), BOOLEAN(0));
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
    legacySave = BOOLEAN::TRUE;
    if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int == 0
        && NextActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
    }
    DebugKeyPressed = BOOLEAN::FALSE;
    optSeedType = C2Rust_Unnamed::OPTVAL_PRIME.0 as ::core::ffi::c_int;
    uqm_lua_checked!(setGameStateUint(
        b"SEED_TYPE\0".as_ptr() as *const ::core::ffi::c_char,
        optSeedType as DWORD,
    ), BOOLEAN(0));
    optCustomSeed = 16807 as ::core::ffi::c_int;
    GlobData.SIS_state.Seed = optCustomSeed as SDWORD;
    return InitStarseed(BOOLEAN::FALSE);
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
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
