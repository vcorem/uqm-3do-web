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

// Rust translation of src/uqm/grpinfo.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type GAME_STATE_FILE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut spaceMusicBySOI: ::core::ffi::c_int;
    static mut optSpaceMusic: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn ReinitQueue(pq: *mut QUEUE);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn ValidateEvent(
        r#type: EVENT_TYPE,
        pmonth_index: *mut COUNT,
        pday_index: *mut COUNT,
        pyear_index: *mut COUNT,
    ) -> BOOLEAN;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    fn planetOuterLocation(planetI: COUNT) -> POINT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: [STAR_DESC; 0];
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn CheckSphereTracking(race: RACE_ID) -> BOOLEAN;
    fn RaceDead(race: RACE_ID) -> BOOLEAN;
    static mut SpaceMusicOK: BOOLEAN;
    fn OpenStateFile(
        stateFile: ::core::ffi::c_int,
        mode: *const ::core::ffi::c_char,
    ) -> *mut GAME_STATE_FILE;
    fn CloseStateFile(fp: *mut GAME_STATE_FILE);
    fn DeleteStateFile(stateFile: ::core::ffi::c_int);
    fn LengthStateFile(fp: *mut GAME_STATE_FILE) -> DWORD;
    fn ReadStateFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        fp: *mut GAME_STATE_FILE,
    ) -> ::core::ffi::c_int;
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
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HOME_ID(pub ::core::ffi::c_uint);
impl HOME_ID {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
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
pub struct planet_desc {
    pub rand_seed: DWORD,
    pub data_index: BYTE,
    pub NumPlanets: BYTE,
    pub radius: SIZE,
    pub location: POINT,
    pub temp_color: Color,
    pub NextIndex: COUNT,
    pub image: STAMP,
    pub pPrevDesc: *mut PLANET_DESC,
    pub orbit: PLANET_ORBIT,
    pub size: COUNT,
    pub angle: COUNT,
    pub rotFrameIndex: ::core::ffi::c_int,
    pub rotPointIndex: ::core::ffi::c_int,
    pub rotwidth: ::core::ffi::c_int,
    pub rotheight: ::core::ffi::c_int,
    pub orb_speed: ::core::ffi::c_double,
    pub rot_speed: ::core::ffi::c_double,
    pub PlanetByte: BYTE,
    pub MoonByte: BYTE,
    pub frame_offset: BYTE,
}
pub type PLANET_ORBIT = planet_orbit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScanColors: *mut *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
    pub BackFrame: FRAME,
    pub light_diff: *mut *mut DWORD,
    pub map_rotate: *mut *mut MAP3D_POINT,
    pub TopoMask: FRAME,
    pub sphereBytes: *mut BYTE,
    pub sphereMap: COLORMAP,
    pub scanType: COUNT,
    pub Shade: FRAME,
    pub ShadeColors: *mut Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MAP3D_POINT {
    pub p: [POINT; 4],
    pub m: [DWORD; 4],
}
pub type PLANET_DESC = planet_desc;
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
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solarsys_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>,
    pub InIpFlight: BOOLEAN,
    pub WaitIntersect: COUNT,
    pub SunDesc: [PLANET_DESC; 1],
    pub PlanetDesc: [PLANET_DESC; 16],
    pub MoonDesc: [PLANET_DESC; 4],
    pub pBaseDesc: *mut PLANET_DESC,
    pub pOrbitalDesc: *mut PLANET_DESC,
    pub FirstPlanetIndex: SIZE,
    pub LastPlanetIndex: SIZE,
    pub turn_counter: BYTE,
    pub turn_wait: BYTE,
    pub thrust_counter: BYTE,
    pub max_ship_speed: BYTE,
    pub XlatRef: STRING,
    pub XlatPtr: *const ::core::ffi::c_void,
    pub OrbitalCMap: COLORMAP,
    pub SysInfo: SYSTEM_INFO,
    pub genFuncs: *const GenerateFunctions,
    pub PlanetSideFrame: [FRAME; 6],
    pub TopoFrame: FRAME,
    pub Orbit: PLANET_ORBIT,
    pub InOrbit: BOOLEAN,
    pub ScanFrame: [FRAME; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenerateFunctions {
    pub initNpcs: InitNpcsFunction,
    pub reinitNpcs: ReinitNpcsFunction,
    pub uninitNpcs: UninitNpcsFunction,
    pub generatePlanets: GeneratePlanetsFunction,
    pub generateMoons: GenerateMoonsFunction,
    pub generateName: GenerateNameFunction,
    pub generateOrbital: GenerateOrbitalFunction,
    pub generateMinerals: GenerateMineralsFunction,
    pub generateEnergy: GenerateEnergyFunction,
    pub generateLife: GenerateLifeFunction,
    pub pickupMinerals: PickupMineralsFunction,
    pub pickupEnergy: PickupEnergyFunction,
    pub pickupLife: PickupLifeFunction,
}
pub type PickupLifeFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type SOLARSYS_STATE = solarsys_state;
pub type PickupEnergyFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupMineralsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type GenerateLifeFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateEnergyFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateMineralsFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateOrbitalFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GenerateNameFunction = Option<
    unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
>;
pub type GenerateMoonsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GeneratePlanetsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
>;
pub type UninitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type ReinitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type InitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANET_INFO {
    pub AxialTilt: SIZE,
    pub Tectonics: UWORD,
    pub Weather: UWORD,
    pub PlanetDensity: UWORD,
    pub PlanetRadius: UWORD,
    pub SurfaceGravity: UWORD,
    pub SurfaceTemperature: SIZE,
    pub RotationPeriod: UWORD,
    pub AtmoDensity: UWORD,
    pub LifeChance: SIZE,
    pub PlanetToSunDist: DWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub PartiallyScavengedList: [[BYTE; 32]; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanetFrame {
    pub Type: BYTE,
    pub BaseTectonics: BYTE,
    pub AtmoAndDensity: BYTE,
    pub UsefulElements: [ELEMENT_ENTRY; 8],
    pub CMapInstance: RESOURCE,
    pub XlatTabInstance: RESOURCE,
    pub num_faults: COUNT,
    pub fault_depth: SIZE,
    pub num_blemishes: COUNT,
    pub base_elevation: SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const IN_ORBIT: Self = Self(0);
    pub const EXPLORE: Self = Self(1);
    pub const FLEE: Self = Self(2);
    pub const ON_STATION: Self = Self(3);
    pub const IGNORE_FLAGSHIP: Self = Self(4);
    pub const REFORM_GROUP: Self = Self(8);
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
unsafe extern "C" fn SpeciesToHomeID(mut species_id: SPECIES_ID) -> HOME_ID {
    match species_id {
        SPECIES_ID::CHMMR_ID => return HOME_ID::CHMMR_HOME,
        SPECIES_ID::ORZ_ID => return HOME_ID::ORZ_HOME,
        SPECIES_ID::PKUNK_ID => return HOME_ID::PKUNK_HOME,
        SPECIES_ID::SHOFIXTI_ID => return HOME_ID::SHOFIXTI_HOME,
        SPECIES_ID::SPATHI_ID => return HOME_ID::SPATHI_HOME,
        SPECIES_ID::SUPOX_ID => return HOME_ID::SUPOX_HOME,
        SPECIES_ID::THRADDASH_ID => return HOME_ID::THRADDASH_HOME,
        SPECIES_ID::UTWIG_ID => return HOME_ID::UTWIG_HOME,
        SPECIES_ID::VUX_ID => return HOME_ID::VUX_HOME,
        SPECIES_ID::YEHAT_ID => return HOME_ID::YEHAT_HOME,
        SPECIES_ID::DRUUGE_ID => return HOME_ID::DRUUGE_HOME,
        SPECIES_ID::ILWRATH_ID => return HOME_ID::ILWRATH_HOME,
        SPECIES_ID::MYCON_ID => return HOME_ID::MYCON_HOME,
        SPECIES_ID::SLYLANDRO_ID => return HOME_ID::SLYLANDRO_HOME,
        SPECIES_ID::UMGAH_ID => return HOME_ID::UMGAH_HOME,
        SPECIES_ID::ZOQFOTPIK_ID => return HOME_ID::ZOQFOTPIK_HOME,
        SPECIES_ID::SYREEN_ID => return HOME_ID::SYREEN_HOME,
        SPECIES_ID::ANDROSYNTH_ID => return HOME_ID::ANDROSYNTH_HOME,
        _ => return HOME_ID::NUM_AVAILABLE_HOMES,
    };
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
static mut LastEncGroup: BYTE = 0;
#[no_mangle]
pub unsafe extern "C" fn ReadGroupHeader(
    mut fp: *mut GAME_STATE_FILE,
    mut pGH: *mut GROUP_HEADER,
) {
    sread_8(fp, &raw mut (*pGH).NumGroups);
    sread_8(fp, &raw mut (*pGH).day_index);
    sread_8(fp, &raw mut (*pGH).month_index);
    sread_8(fp, ::core::ptr::null_mut::<BYTE>());
    sread_16(fp, &raw mut (*pGH).star_index);
    sread_16(fp, &raw mut (*pGH).year_index);
    sread_a32(
        fp,
        &raw mut (*pGH).GroupOffset as *mut DWORD,
        (64 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WriteGroupHeader(
    mut fp: *mut GAME_STATE_FILE,
    mut pGH: *const GROUP_HEADER,
) {
    swrite_8(fp, (*pGH).NumGroups);
    swrite_8(fp, (*pGH).day_index);
    swrite_8(fp, (*pGH).month_index);
    swrite_8(fp, 0 as BYTE);
    swrite_16(fp, (*pGH).star_index);
    swrite_16(fp, (*pGH).year_index);
    swrite_a32(
        fp,
        &raw const (*pGH).GroupOffset as *const DWORD,
        (64 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ReadShipFragment(
    mut fp: *mut GAME_STATE_FILE,
    mut FragPtr: *mut SHIP_FRAGMENT,
) {
    let mut tmpb: BYTE = 0;
    sread_16(fp, ::core::ptr::null_mut::<UWORD>());
    sread_8(fp, &raw mut (*FragPtr).captains_name_index);
    sread_8(fp, ::core::ptr::null_mut::<BYTE>());
    sread_16(fp, ::core::ptr::null_mut::<UWORD>());
    sread_8(fp, &raw mut (*FragPtr).race_id);
    sread_8(fp, &raw mut (*FragPtr).index);
    sread_8(fp, &raw mut tmpb);
    (*FragPtr).crew_level = tmpb as COUNT;
    sread_8(fp, &raw mut tmpb);
    (*FragPtr).max_crew = tmpb as COUNT;
    sread_8(fp, &raw mut (*FragPtr).energy_level);
    sread_8(fp, &raw mut (*FragPtr).max_energy);
    sread_16(fp, ::core::ptr::null_mut::<UWORD>());
    sread_16(fp, ::core::ptr::null_mut::<UWORD>());
}
#[no_mangle]
pub unsafe extern "C" fn WriteShipFragment(
    mut fp: *mut GAME_STATE_FILE,
    mut FragPtr: *const SHIP_FRAGMENT,
) {
    swrite_16(fp, 0 as UWORD);
    swrite_8(fp, (*FragPtr).captains_name_index);
    swrite_8(fp, 0 as BYTE);
    swrite_16(fp, 0 as UWORD);
    swrite_8(fp, (*FragPtr).race_id);
    swrite_8(fp, (*FragPtr).index);
    swrite_8(fp, (*FragPtr).crew_level as BYTE);
    swrite_8(fp, (*FragPtr).max_crew as BYTE);
    swrite_8(fp, (*FragPtr).energy_level);
    swrite_8(fp, (*FragPtr).max_energy);
    swrite_16(fp, 0 as UWORD);
    swrite_16(fp, 0 as UWORD);
}
#[no_mangle]
pub unsafe extern "C" fn ReadIpGroup(
    mut fp: *mut GAME_STATE_FILE,
    mut GroupPtr: *mut IP_GROUP,
) {
    let mut tmpb: BYTE = 0;
    sread_16(fp, ::core::ptr::null_mut::<UWORD>());
    sread_8(fp, ::core::ptr::null_mut::<BYTE>());
    sread_8(fp, ::core::ptr::null_mut::<BYTE>());
    sread_16(fp, &raw mut (*GroupPtr).group_counter);
    sread_8(fp, &raw mut (*GroupPtr).race_id);
    sread_8(fp, &raw mut tmpb);
    (*GroupPtr).sys_loc = (tmpb as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    (*GroupPtr).task = (tmpb as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    sread_8(fp, &raw mut (*GroupPtr).in_system);
    sread_8(fp, ::core::ptr::null_mut::<BYTE>());
    sread_8(fp, &raw mut tmpb);
    (*GroupPtr).dest_loc = (tmpb as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    (*GroupPtr).orbit_pos = (tmpb as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    sread_8(fp, &raw mut (*GroupPtr).group_id);
    sread_16s(fp, &raw mut (*GroupPtr).loc.x);
    sread_16s(fp, &raw mut (*GroupPtr).loc.y);
}
#[no_mangle]
pub unsafe extern "C" fn WriteIpGroup(
    mut fp: *mut GAME_STATE_FILE,
    mut GroupPtr: *const IP_GROUP,
) {
    swrite_16(fp, 0 as UWORD);
    swrite_8(fp, 0 as BYTE);
    swrite_8(fp, 0 as BYTE);
    swrite_16(fp, (*GroupPtr).group_counter);
    swrite_8(fp, (*GroupPtr).race_id);
    (((*GroupPtr).sys_loc as ::core::ffi::c_int) < 0x10 as ::core::ffi::c_int
        && ((*GroupPtr).task as ::core::ffi::c_int) < 0x10 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"GroupPtr->sys_loc < 0x10 && GroupPtr->task < 0x10\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                133 as ::core::ffi::c_int,
                b"WriteIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    swrite_8(
        fp,
        (((*GroupPtr).task as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | (*GroupPtr).sys_loc as ::core::ffi::c_int) as BYTE,
    );
    swrite_8(fp, (*GroupPtr).in_system);
    swrite_8(fp, 0 as BYTE);
    (((*GroupPtr).dest_loc as ::core::ffi::c_int) < 0x10 as ::core::ffi::c_int
        && ((*GroupPtr).orbit_pos as ::core::ffi::c_int) < 0x10 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"GroupPtr->dest_loc < 0x10 && GroupPtr->orbit_pos < 0x10\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int,
                b"WriteIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    swrite_8(
        fp,
        (((*GroupPtr).orbit_pos as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | (*GroupPtr).dest_loc as ::core::ffi::c_int) as BYTE,
    );
    swrite_8(fp, (*GroupPtr).group_id);
    swrite_16(fp, (*GroupPtr).loc.x as UWORD);
    swrite_16(fp, (*GroupPtr).loc.y as UWORD);
}
#[no_mangle]
pub unsafe extern "C" fn InitGroupInfo(mut FirstTime: BOOLEAN) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    (64 as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"NUM_SAVED_BATTLE_GROUPS >= MAX_BATTLE_GROUPS\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                151 as ::core::ffi::c_int,
                b"InitGroupInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    fp = OpenStateFile(
        1 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut GH: GROUP_HEADER = GROUP_HEADER {
            NumGroups: 0,
            day_index: 0,
            month_index: 0,
            star_index: 0,
            year_index: 0,
            GroupOffset: [0; 65],
        };
        memset(
            &raw mut GH as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GROUP_HEADER>(),
        );
        GH.star_index = !(0 as ::core::ffi::c_int) as COUNT;
        WriteGroupHeader(fp, &raw mut GH);
        CloseStateFile(fp);
    }
    if FirstTime.0 != 0
        && {
            fp = OpenStateFile(
                2 as ::core::ffi::c_int,
                b"wb\0".as_ptr() as *const ::core::ffi::c_char,
            );
            !fp.is_null()
        }
    {
        swrite_8(fp, 0 as BYTE);
        CloseStateFile(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UninitGroupInfo() {
    DeleteStateFile(2 as ::core::ffi::c_int);
    DeleteStateFile(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BuildGroup(
    mut pDstQueue: *mut QUEUE,
    mut race_id: BYTE,
) -> HIPGROUP {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut hGroup: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
    ((*pDstQueue).object_size as usize == ::core::mem::size_of::<IP_GROUP>()
        || {
            __assert_fail(
                b"GetLinkSize (pDstQueue) == sizeof (IP_GROUP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_int,
                b"BuildGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race_id as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    hGroup = AllocLink(pDstQueue);
    if hGroup.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    GroupPtr = LockIpGroup(pDstQueue, hGroup);
    memset(
        GroupPtr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*pDstQueue).object_size as size_t,
    );
    (*GroupPtr).race_id = race_id;
    (*GroupPtr).melee_icon = (*TemplatePtr).melee_icon;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    UnlockLink(pDstQueue, hGroup);
    PutQueue(pDstQueue, hGroup);
    return hGroup;
}
#[no_mangle]
pub unsafe extern "C" fn BuildGroups() {
    let mut Index: BYTE = 0;
    let mut BestIndex: BYTE = 0 as BYTE;
    let mut BestPercent: COUNT = 0 as COUNT;
    let mut universe: POINT = POINT { x: 0, y: 0 };
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut HomeWorld: [BYTE; 26] = [
        0 as BYTE,
        C2Rust_Unnamed::CHMMR_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::ORZ_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::PKUNK_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::SPATHI_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::SUPOX_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::THRADD_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::UTWIG_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::VUX_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::YEHAT_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::DRUUGE_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::ILWRATH_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::MYCON_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::TALKING_PET_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::SYREEN_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
    ];
    let mut EncounterPercent: [BYTE; 27] = [
        2 as BYTE,
        0 as BYTE,
        0 as BYTE,
        20 as BYTE,
        20 as BYTE,
        0 as BYTE,
        10 as BYTE,
        20 as BYTE,
        20 as BYTE,
        20 as BYTE,
        20 as BYTE,
        40 as BYTE,
        0 as BYTE,
        20 as BYTE,
        60 as BYTE,
        20 as BYTE,
        5 as BYTE,
        20 as BYTE,
        40 as BYTE,
        0 as BYTE,
        0 as BYTE,
        40 as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
        40 as BYTE,
        0 as BYTE,
    ];
    EncounterPercent[RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as usize] = (EncounterPercent[RACE_ID::SLYLANDRO_SHIP
        .0 as ::core::ffi::c_int as usize] as DWORD)
        .wrapping_mul(
            uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()),
        ) as BYTE;
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed::ZOQFOT_DEFINED.0 as ::core::ffi::c_int
    {
        EncounterPercent[RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as usize] = 0
            as BYTE;
        EncounterPercent[RACE_ID::URQUAN_SHIP.0 as ::core::ffi::c_int as usize] = EncounterPercent[RACE_ID::BLACK_URQUAN_SHIP
            .0 as ::core::ffi::c_int as usize];
    }
    Index = uqm_lua_checked!(getGameStateUint(
        b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    if Index as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        && (Index as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
    {
        HomeWorld[RACE_ID::UTWIG_SHIP.0 as ::core::ffi::c_int as usize] = 0 as BYTE;
        HomeWorld[RACE_ID::SUPOX_SHIP.0 as ::core::ffi::c_int as usize] = 0 as BYTE;
    }
    universe = (*CurStarDescPtr).star_pt;
    hFleet = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    Index = 0 as BYTE;
    while !hFleet.is_null() {
        let mut i: COUNT = 0;
        let mut encounter_radius: COUNT = 0;
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
        hNextFleet = (*FleetPtr).succ;
        encounter_radius = (*FleetPtr).actual_strength;
        if encounter_radius as ::core::ffi::c_int != 0
            && {
                i = EncounterPercent[Index as usize] as COUNT;
                i as ::core::ffi::c_int != 0
            }
        {
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            let mut d_squared: DWORD = 0;
            let mut race_enc: BYTE = 0;
            race_enc = HomeWorld[Index as usize];
            if race_enc as ::core::ffi::c_int != 0
                && (*CurStarDescPtr).Index as ::core::ffi::c_int
                    == race_enc as ::core::ffi::c_int
            {
                BestIndex = Index;
                BestPercent = 70 as COUNT;
                if race_enc as ::core::ffi::c_int
                    == C2Rust_Unnamed::SPATHI_DEFINED.0 as ::core::ffi::c_int
                    || race_enc as ::core::ffi::c_int
                        == C2Rust_Unnamed::SUPOX_DEFINED.0 as ::core::ffi::c_int
                {
                    BestPercent = 2 as COUNT;
                }
                hNextFleet = ::core::ptr::null_mut::<::core::ffi::c_void>();
            } else {
                if encounter_radius as ::core::ffi::c_int
                    == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                {
                    encounter_radius = ((9999 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
                } else {
                    encounter_radius = (encounter_radius as ::core::ffi::c_int
                        * 11 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
                }
                dx = (universe.x as ::core::ffi::c_int
                    - (*FleetPtr).loc.x as ::core::ffi::c_int) as SIZE;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                dy = (universe.y as ::core::ffi::c_int
                    - (*FleetPtr).loc.y as ::core::ffi::c_int) as SIZE;
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if (dx as COUNT as ::core::ffi::c_int)
                    < encounter_radius as ::core::ffi::c_int
                    && (dy as COUNT as ::core::ffi::c_int)
                        < encounter_radius as ::core::ffi::c_int
                    && {
                        d_squared = (dx as DWORD)
                            .wrapping_mul(dx as DWORD)
                            .wrapping_add((dy as DWORD).wrapping_mul(dy as DWORD));
                        d_squared
                            < (encounter_radius as DWORD)
                                .wrapping_mul(encounter_radius as DWORD)
                    }
                {
                    let mut rand_val: DWORD = 0;
                    if (*FleetPtr).actual_strength as ::core::ffi::c_int
                        != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    {
                        i = (70 as ::core::ffi::c_int
                            - (square_root(d_squared) as DWORD as ::core::ffi::c_ulong)
                                .wrapping_mul(60 as ::core::ffi::c_ulong)
                                .wrapping_div(encounter_radius as ::core::ffi::c_ulong)
                                as COUNT as ::core::ffi::c_int) as COUNT;
                    }
                    rand_val = TFB_Random();
                    if (rand_val as UWORD as ::core::ffi::c_int
                        % 100 as ::core::ffi::c_int) < i as ::core::ffi::c_int
                        && (BestPercent as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || (rand_val >> 16 as ::core::ffi::c_int) as UWORD
                                as ::core::ffi::c_int
                                % (i as ::core::ffi::c_int
                                    + BestPercent as ::core::ffi::c_int)
                                < i as ::core::ffi::c_int)
                    {
                        if (*FleetPtr).actual_strength as ::core::ffi::c_int
                            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                        {
                            i = 4 as COUNT;
                        }
                        BestPercent = i;
                        BestIndex = Index;
                    }
                }
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
        hFleet = hNextFleet;
        Index = Index.wrapping_add(1);
    }
    if BestPercent != 0 {
        let mut which_group: BYTE = 0;
        let mut num_groups: BYTE = 0;
        let mut EncounterMakeup: [BYTE; 26] = [
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            0 as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
        ];
        which_group = 0 as BYTE;
        num_groups = (TFB_Random() as COUNT as ::core::ffi::c_int
            % (BestPercent as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as BYTE;
        if num_groups as ::core::ffi::c_int > 32 as ::core::ffi::c_int {
            num_groups = 32 as BYTE;
        } else if (num_groups as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
            && {
                Index = HomeWorld[BestIndex as usize];
                Index as ::core::ffi::c_int != 0
            }
            && (*CurStarDescPtr).Index as ::core::ffi::c_int
                == Index as ::core::ffi::c_int
        {
            num_groups = 5 as BYTE;
        }
        loop {
            Index = (EncounterMakeup[BestIndex as usize] as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
            while Index != 0 {
                if Index as ::core::ffi::c_int
                    <= (EncounterMakeup[BestIndex as usize] as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                        as BYTE as ::core::ffi::c_int
                    || (TFB_Random() as COUNT as ::core::ffi::c_int
                        % 100 as ::core::ffi::c_int) < 50 as ::core::ffi::c_int
                {
                    CloneShipFragment(
                        RACE_ID(BestIndex as ::core::ffi::c_uint),
                        &raw mut GlobData.Game_state.npc_built_ship_q,
                        0 as COUNT,
                    );
                }
                Index = Index.wrapping_sub(1);
            }
            which_group = which_group.wrapping_add(1);
            PutGroupInfo(0 as ::core::ffi::c_long as DWORD, which_group);
            ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
            num_groups = num_groups.wrapping_sub(1);
            if num_groups == 0 {
                break;
            }
        }
    }
    GetGroupInfo(0 as ::core::ffi::c_long as DWORD, !(0 as ::core::ffi::c_int) as BYTE);
}
#[no_mangle]
pub unsafe extern "C" fn findRaceSOI() {
    let mut universe: POINT = (*CurStarDescPtr).star_pt;
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut Index: BYTE = 0;
    let mut i: BYTE = 0 as BYTE;
    let mut j: BYTE = 0;
    let mut RaceHasMusic: [BYTE; 27] = [
        1 as BYTE,
        1 as BYTE,
        0 as BYTE,
        1 as BYTE,
        1 as BYTE,
        0 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        0 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        0 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        1 as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
        1 as BYTE,
        0 as BYTE,
    ];
    let mut HomeWorld: [BYTE; 26] = [
        0 as BYTE,
        C2Rust_Unnamed::CHMMR_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::ORZ_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::PKUNK_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::SPATHI_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::SUPOX_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::THRADD_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::UTWIG_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::VUX_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::YEHAT_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::DRUUGE_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::ILWRATH_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::MYCON_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::TALKING_PET_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        C2Rust_Unnamed::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed::SYREEN_DEFINED.0 as ::core::ffi::c_int as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
    ];
    let mut WhichHomeWorld: BYTE = SPECIES_ID::NO_ID.0 as ::core::ffi::c_int as BYTE;
    let mut SpeciesArr: [BYTE; 10] = [
        SPECIES_ID::NO_ID.0 as ::core::ffi::c_int as BYTE,
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
    let mut RadiusArr: [COUNT; 10] = [0 as COUNT, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    if SpaceMusicOK.0 == 0 {
        spaceMusicBySOI = SPECIES_ID::NO_ID.0 as ::core::ffi::c_int;
        return;
    }
    if uqm_lua_checked!(getGameStateUint(b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
    {
        spaceMusicBySOI = SPECIES_ID::SA_MATRA_ID.0 as ::core::ffi::c_int;
        return;
    }
    hFleet = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    Index = 0 as BYTE;
    while !hFleet.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut SpeciesID: SPECIES_ID = SPECIES_ID::NO_ID;
        let mut RaceID: RACE_ID = RACE_ID::ARILOU_SHIP;
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
        hNextFleet = (*FleetPtr).succ;
        SpeciesID = (*FleetPtr).SpeciesID;
        RaceID = RACE_ID(SpeciesID.0.wrapping_sub(1 as ::core::ffi::c_uint));
        if HomeWorld[Index as usize] as ::core::ffi::c_int != 0
            && (*CurStarDescPtr).Index as ::core::ffi::c_int
                == HomeWorld[Index as usize] as ::core::ffi::c_int
            && RaceDead(RaceID).0 == 0
            && (optSpaceMusic == 2 as ::core::ffi::c_int
                || optSpaceMusic == 1 as ::core::ffi::c_int
                    && (CheckSphereTracking(RaceID).0 != 0
                        || uqm_lua_checked!(IsHomeworldKnown(SpeciesToHomeID(SpeciesID).0), ()).0 != 0))
        {
            WhichHomeWorld = SpeciesID.0 as BYTE;
        }
        if (*FleetPtr).actual_strength as ::core::ffi::c_int != 0
            && RaceHasMusic[Index as usize] as ::core::ffi::c_int != 0
        {
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            let mut encounter_radius: COUNT = 0;
            let mut d_squared: DWORD = 0;
            if (*FleetPtr).actual_strength as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                encounter_radius = ((9999 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
            } else {
                encounter_radius = ((*FleetPtr).actual_strength as ::core::ffi::c_int
                    * 11 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
            }
            dx = (universe.x as ::core::ffi::c_int
                - (*FleetPtr).loc.x as ::core::ffi::c_int) as SIZE;
            if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                dx = -(dx as ::core::ffi::c_int) as SIZE;
            }
            dy = (universe.y as ::core::ffi::c_int
                - (*FleetPtr).loc.y as ::core::ffi::c_int) as SIZE;
            if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                dy = -(dy as ::core::ffi::c_int) as SIZE;
            }
            if (dx as COUNT as ::core::ffi::c_int)
                < encounter_radius as ::core::ffi::c_int
                && (dy as COUNT as ::core::ffi::c_int)
                    < encounter_radius as ::core::ffi::c_int
                && {
                    d_squared = (dx as DWORD)
                        .wrapping_mul(dx as DWORD)
                        .wrapping_add((dy as DWORD).wrapping_mul(dy as DWORD));
                    d_squared
                        < (encounter_radius as DWORD)
                            .wrapping_mul(encounter_radius as DWORD)
                }
            {
                if optSpaceMusic == 2 as ::core::ffi::c_int
                    || optSpaceMusic == 1 as ::core::ffi::c_int
                        && CheckSphereTracking(RaceID).0 != 0
                {
                    SpeciesArr[i as usize] = SpeciesID.0 as BYTE;
                    RadiusArr[i as usize] = encounter_radius;
                    i = i.wrapping_add(1);
                }
            }
        }
        hFleet = hNextFleet;
        Index = Index.wrapping_add(1);
    }
    if WhichHomeWorld as ::core::ffi::c_int != SPECIES_ID::NO_ID.0 as ::core::ffi::c_int
    {
        spaceMusicBySOI = WhichHomeWorld as ::core::ffi::c_int;
        return;
    }
    Index = 0 as BYTE;
    j = 1 as BYTE;
    while (j as ::core::ffi::c_int) < i as ::core::ffi::c_int {
        if (RadiusArr[j as usize] as ::core::ffi::c_int)
            < RadiusArr[Index as usize] as ::core::ffi::c_int
        {
            Index = j;
        }
        j = j.wrapping_add(1);
    }
    spaceMusicBySOI = SpeciesArr[Index as usize] as ::core::ffi::c_int;
}
unsafe extern "C" fn FlushGroupInfo(
    mut pGH: *mut GROUP_HEADER,
    mut offset: DWORD,
    mut which_group: BYTE,
    mut fp: *mut GAME_STATE_FILE,
) {
    if which_group as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*pGH).GroupOffset[0usize] == 0 as DWORD {
            (*pGH).GroupOffset[0usize] = LengthStateFile(fp);
        }
        (GlobData.Game_state.npc_built_ship_q.head.is_null()
            || {
                __assert_fail(
                    b"!GetHeadLink (&GLOBAL (npc_built_ship_q))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/grpinfo.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    477 as ::core::ffi::c_int,
                    b"FlushGroupInfo\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        hGroup = GlobData.Game_state.ip_group_q.head as HIPGROUP;
        while !hGroup.is_null() {
            let mut in_system: BYTE = 0;
            let mut group_id: BYTE = 0;
            let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
            GroupPtr = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup);
            hNextGroup = (*GroupPtr).succ;
            in_system = (*GroupPtr).in_system;
            group_id = (*GroupPtr).group_id;
            UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
            if in_system == 0 {
                if GlobData.Game_state.BattleGroupRef != 0 {
                    PutGroupInfo(GlobData.Game_state.BattleGroupRef, group_id);
                } else {
                    FlushGroupInfo(pGH, 0 as ::core::ffi::c_long as DWORD, group_id, fp);
                }
                (*pGH).GroupOffset[group_id as usize] = 0 as DWORD;
                RemoveQueue(&raw mut GlobData.Game_state.ip_group_q, hGroup);
                FreeLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
            }
            hGroup = hNextGroup;
        }
    } else if which_group as ::core::ffi::c_int > (*pGH).NumGroups as ::core::ffi::c_int
    {
        (*pGH).NumGroups = which_group;
        (*pGH).GroupOffset[which_group as usize] = LengthStateFile(fp);
    }
    SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    WriteGroupHeader(fp, pGH);
    if which_group as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        let mut hGroup_0: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextGroup_0: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut NumGroups: BYTE = CountLinks(&raw mut GlobData.Game_state.ip_group_q)
            as BYTE;
        SeekStateFile(
            fp,
            (*pGH).GroupOffset[0usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        swrite_8(fp, LastEncGroup);
        swrite_8(fp, NumGroups);
        hGroup_0 = GlobData.Game_state.ip_group_q.head as HIPGROUP;
        while NumGroups != 0 {
            let mut GroupPtr_0: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
            GroupPtr_0 = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup_0);
            hNextGroup_0 = (*GroupPtr_0).succ;
            swrite_8(fp, (*GroupPtr_0).race_id);
            WriteIpGroup(fp, GroupPtr_0);
            UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup_0);
            NumGroups = NumGroups.wrapping_sub(1);
            hGroup_0 = hNextGroup_0;
        }
    } else {
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut NumShips: BYTE = CountLinks(
            &raw mut GlobData.Game_state.npc_built_ship_q,
        ) as BYTE;
        let mut RaceType: BYTE = 0 as BYTE;
        hStarShip = GlobData.Game_state.npc_built_ship_q.head as HSHIPFRAG;
        if NumShips as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
                SHIP_FRAGMENT,
            >();
            FragPtr = LockShipFrag(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                hStarShip,
            );
            RaceType = (*FragPtr).race_id;
            UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
        }
        SeekStateFile(
            fp,
            (*pGH).GroupOffset[which_group as usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        swrite_8(fp, RaceType);
        swrite_8(fp, NumShips);
        while NumShips != 0 {
            let mut FragPtr_0: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
                SHIP_FRAGMENT,
            >();
            FragPtr_0 = LockShipFrag(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                hStarShip,
            );
            hNextShip = (*FragPtr_0).succ as HSHIPFRAG;
            swrite_8(fp, (*FragPtr_0).race_id);
            WriteShipFragment(fp, FragPtr_0);
            UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
            NumShips = NumShips.wrapping_sub(1);
            hStarShip = hNextShip;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetGroupInfo(
    mut offset: DWORD,
    mut which_group: BYTE,
) -> BOOLEAN {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    let mut GH: GROUP_HEADER = GROUP_HEADER {
        NumGroups: 0,
        day_index: 0,
        month_index: 0,
        star_index: 0,
        year_index: 0,
        GroupOffset: [0; 65],
    };
    if offset != 0 as ::core::ffi::c_long as DWORD
        && which_group as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        fp = OpenStateFile(
            2 as ::core::ffi::c_int,
            b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        fp = OpenStateFile(
            1 as ::core::ffi::c_int,
            b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if fp.is_null() {
        return BOOLEAN::FALSE;
    }
    SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    ReadGroupHeader(fp, &raw mut GH);
    if which_group as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        let mut month_index: COUNT = 0;
        let mut day_index: COUNT = 0;
        let mut year_index: COUNT = 0;
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        day_index = GH.day_index as COUNT;
        month_index = GH.month_index as COUNT;
        year_index = GH.year_index;
        if offset == 0 as ::core::ffi::c_long as DWORD
            && (GH.star_index as ::core::ffi::c_int
                != CurStarDescPtr.offset_from(&raw mut star_array as *mut STAR_DESC)
                    as COUNT as ::core::ffi::c_int
                || ValidateEvent(
                        EVENT_TYPE::ABSOLUTE_EVENT,
                        &raw mut month_index,
                        &raw mut day_index,
                        &raw mut year_index,
                    )
                    .0 == 0)
        {
            CloseStateFile(fp);
            fp = OpenStateFile(
                1 as ::core::ffi::c_int,
                b"wb\0".as_ptr() as *const ::core::ffi::c_char,
            );
            memset(
                &raw mut GH as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GROUP_HEADER>(),
            );
            GH.star_index = !(0 as ::core::ffi::c_int) as COUNT;
            WriteGroupHeader(fp, &raw mut GH);
            CloseStateFile(fp);
            return BOOLEAN::FALSE;
        }
        which_group = 1 as BYTE;
        while which_group as ::core::ffi::c_int <= GH.NumGroups as ::core::ffi::c_int {
            let mut task: BYTE = 0;
            let mut group_loc: BYTE = 0;
            let mut rand_val: DWORD = 0;
            let mut RaceType: BYTE = 0;
            let mut NumShips: BYTE = 0;
            let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
            if GH.GroupOffset[which_group as usize] != 0 as DWORD {
                SeekStateFile(
                    fp,
                    GH.GroupOffset[which_group as usize] as ::core::ffi::c_long,
                    0 as ::core::ffi::c_int,
                );
                sread_8(fp, &raw mut RaceType);
                sread_8(fp, &raw mut NumShips);
                if NumShips != 0 {
                    hGroup = BuildGroup(
                        &raw mut GlobData.Game_state.ip_group_q,
                        RaceType,
                    );
                    GroupPtr = LockIpGroup(
                        &raw mut GlobData.Game_state.ip_group_q,
                        hGroup,
                    );
                    (*GroupPtr).group_id = which_group;
                    (*GroupPtr).in_system = 1 as BYTE;
                    rand_val = TFB_Random();
                    task = (rand_val as UWORD as BYTE as ::core::ffi::c_int
                        % C2Rust_Unnamed_0::ON_STATION.0 as ::core::ffi::c_int) as BYTE;
                    if task as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::FLEE.0 as ::core::ffi::c_int
                    {
                        task = C2Rust_Unnamed_0::ON_STATION.0 as ::core::ffi::c_int
                            as BYTE;
                    }
                    (*GroupPtr).orbit_pos = ((rand_val >> 16 as ::core::ffi::c_int)
                        as UWORD as BYTE as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as BYTE;
                    group_loc = (*pSolarSysState).SunDesc[0usize].NumPlanets;
                    if group_loc as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        && task as ::core::ffi::c_int
                            == C2Rust_Unnamed_0::EXPLORE.0 as ::core::ffi::c_int
                    {
                        task = C2Rust_Unnamed_0::IN_ORBIT.0 as ::core::ffi::c_int
                            as BYTE;
                    } else {
                        group_loc = ((rand_val as UWORD as ::core::ffi::c_int
                            >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                            % group_loc as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as BYTE;
                    }
                    (*GroupPtr).dest_loc = group_loc;
                    rand_val = TFB_Random();
                    (*GroupPtr).loc.x = (rand_val as UWORD as ::core::ffi::c_int
                        % 10000 as ::core::ffi::c_int - 5000 as ::core::ffi::c_int)
                        as COORD;
                    (*GroupPtr).loc.y = ((rand_val >> 16 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int % 10000 as ::core::ffi::c_int
                        - 5000 as ::core::ffi::c_int) as COORD;
                    (*GroupPtr).group_counter = 0 as UWORD;
                    if task as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::EXPLORE.0 as ::core::ffi::c_int
                    {
                        (*GroupPtr).group_counter = ((TFB_Random() as COUNT
                            as ::core::ffi::c_int % 5 as ::core::ffi::c_int)
                            << 4 as ::core::ffi::c_int) as UWORD;
                    } else if task as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::ON_STATION.0 as ::core::ffi::c_int
                    {
                        let mut angle: COUNT = 0;
                        let mut org: POINT = POINT { x: 0, y: 0 };
                        org = planetOuterLocation(
                            (group_loc as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as COUNT,
                        );
                        angle = (((*GroupPtr).orbit_pos as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                            as COUNT;
                        (*GroupPtr).loc.x = (org.x as SDWORD
                            + (*(&raw mut sinetab as *mut SDWORD)
                                .offset(
                                    (angle as ::core::ffi::c_int
                                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                            >> 2 as ::core::ffi::c_int)
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                                ) as ::core::ffi::c_long
                                * 1600 as ::core::ffi::c_int as ::core::ffi::c_long
                                >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                        (*GroupPtr).loc.y = (org.y as SDWORD
                            + (*(&raw mut sinetab as *mut SDWORD)
                                .offset(
                                    (angle as ::core::ffi::c_int
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                                ) as ::core::ffi::c_long
                                * 1600 as ::core::ffi::c_int as ::core::ffi::c_long
                                >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                        group_loc = 0 as BYTE;
                    }
                    (*GroupPtr).task = task;
                    (*GroupPtr).sys_loc = group_loc;
                    UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
                }
            }
            which_group = which_group.wrapping_add(1);
        }
        if offset != 0 as ::core::ffi::c_long as DWORD {
            InitGroupInfo(BOOLEAN::FALSE);
        } else if ValidateEvent(
                EVENT_TYPE::ABSOLUTE_EVENT,
                &raw mut month_index,
                &raw mut day_index,
                &raw mut year_index,
            )
            .0 != 0
        {
            CloseStateFile(fp);
            return BOOLEAN::TRUE;
        }
        CloseStateFile(fp);
        return BOOLEAN(
            !GlobData.Game_state.ip_group_q.head.is_null() as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    }
    if GH.GroupOffset[which_group as usize] == 0 {
        CloseStateFile(fp);
        return BOOLEAN::FALSE;
    }
    if which_group as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        let mut NumGroups: BYTE = 0;
        let mut ShipsLeftInLEG: COUNT = 0;
        ShipsLeftInLEG = CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q);
        SeekStateFile(
            fp,
            GH.GroupOffset[0usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut LastEncGroup);
        if LastEncGroup != 0 {
            if GlobData.Game_state.BattleGroupRef != 0 {
                PutGroupInfo(GlobData.Game_state.BattleGroupRef, LastEncGroup);
            } else {
                FlushGroupInfo(&raw mut GH, offset, LastEncGroup, fp);
            }
        }
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        SeekStateFile(
            fp,
            GH.GroupOffset[0usize].wrapping_add(1 as DWORD) as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut NumGroups);
        loop {
            let c2rust_fresh0 = NumGroups;
            NumGroups = NumGroups.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            let mut group_id: BYTE = 0;
            let mut RaceType_0: BYTE = 0;
            let mut hGroup_0: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut GroupPtr_0: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
            sread_8(fp, &raw mut RaceType_0);
            hGroup_0 = BuildGroup(&raw mut GlobData.Game_state.ip_group_q, RaceType_0)
                as HSHIPFRAG;
            GroupPtr_0 = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup_0);
            ReadIpGroup(fp, GroupPtr_0);
            group_id = (*GroupPtr_0).group_id;
            UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup_0);
            if group_id as ::core::ffi::c_int == LastEncGroup as ::core::ffi::c_int
                && ShipsLeftInLEG == 0
            {
                RemoveQueue(&raw mut GlobData.Game_state.ip_group_q, hGroup_0);
                FreeLink(&raw mut GlobData.Game_state.ip_group_q, hGroup_0);
            }
        }
        CloseStateFile(fp);
        return BOOLEAN(
            !GlobData.Game_state.ip_group_q.head.is_null() as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    } else {
        let mut NumShips_0: BYTE = 0;
        LastEncGroup = which_group;
        if offset != 0 as ::core::ffi::c_long as DWORD {
            PutGroupInfo(
                0 as ::core::ffi::c_long as DWORD,
                0 as ::core::ffi::c_int as BYTE,
            );
        } else {
            FlushGroupInfo(
                &raw mut GH,
                0 as ::core::ffi::c_long as DWORD,
                0 as ::core::ffi::c_int as BYTE,
                fp,
            );
        }
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        SeekStateFile(
            fp,
            GH.GroupOffset[which_group as usize].wrapping_add(1 as DWORD)
                as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut NumShips_0);
        if NumShips_0 as ::core::ffi::c_int > 14 as ::core::ffi::c_int {
            NumShips_0 = 1 as BYTE;
        }
        loop {
            let c2rust_fresh1 = NumShips_0;
            NumShips_0 = NumShips_0.wrapping_sub(1);
            if c2rust_fresh1 == 0 {
                break;
            }
            let mut RaceType_1: BYTE = 0;
            let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
                SHIP_FRAGMENT,
            >();
            sread_8(fp, &raw mut RaceType_1);
            hStarShip = CloneShipFragment(
                RACE_ID(RaceType_1 as ::core::ffi::c_uint),
                &raw mut GlobData.Game_state.npc_built_ship_q,
                0 as COUNT,
            );
            FragPtr = LockShipFrag(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                hStarShip,
            );
            ReadShipFragment(fp, FragPtr);
            UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
        }
        CloseStateFile(fp);
        return BOOLEAN(
            !GlobData.Game_state.npc_built_ship_q.head.is_null() as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn PutGroupInfo(
    mut offset: DWORD,
    mut which_group: BYTE,
) -> DWORD {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    let mut GH: GROUP_HEADER = GROUP_HEADER {
        NumGroups: 0,
        day_index: 0,
        month_index: 0,
        star_index: 0,
        year_index: 0,
        GroupOffset: [0; 65],
    };
    if offset != 0 as ::core::ffi::c_long as DWORD
        && which_group as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        fp = OpenStateFile(
            2 as ::core::ffi::c_int,
            b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        fp = OpenStateFile(
            1 as ::core::ffi::c_int,
            b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if fp.is_null() {
        return offset;
    }
    if offset == !(0 as ::core::ffi::c_long) as DWORD {
        offset = LengthStateFile(fp);
        SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
        memset(
            &raw mut GH as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GROUP_HEADER>(),
        );
        GH.star_index = !(0 as ::core::ffi::c_int) as COUNT;
        WriteGroupHeader(fp, &raw mut GH);
    }
    if which_group as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
        if which_group as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        {
            LastEncGroup = 0 as BYTE;
            which_group = 0 as ::core::ffi::c_int as BYTE;
        }
    }
    ReadGroupHeader(fp, &raw mut GH);
    let mut month_index: COUNT = 0;
    let mut day_index: COUNT = 0;
    let mut year_index: COUNT = 0;
    month_index = 0 as COUNT;
    day_index = 7 as COUNT;
    year_index = 0 as COUNT;
    ValidateEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        &raw mut month_index,
        &raw mut day_index,
        &raw mut year_index,
    );
    GH.day_index = day_index as BYTE;
    GH.month_index = month_index as BYTE;
    GH.year_index = year_index;
    GH.star_index = CurStarDescPtr.offset_from(&raw mut star_array as *mut STAR_DESC)
        as COUNT;
    FlushGroupInfo(&raw mut GH, offset, which_group, fp);
    CloseStateFile(fp);
    return offset;
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
unsafe extern "C" fn sread_8(mut fp: *mut GAME_STATE_FILE, mut v: *mut BYTE) -> COUNT {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 1 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_16(mut fp: *mut GAME_STATE_FILE, mut v: *mut UWORD) -> COUNT {
    let mut t: UWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 2 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_16s(
    mut fp: *mut GAME_STATE_FILE,
    mut v: *mut SWORD,
) -> COUNT {
    let mut t: UWORD = 0;
    let mut ret: COUNT = 0;
    ret = sread_16(fp, &raw mut t);
    if !v.is_null() {
        *v = t as SWORD;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn sread_32(mut fp: *mut GAME_STATE_FILE, mut v: *mut DWORD) -> COUNT {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 4 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_a32(
    mut fp: *mut GAME_STATE_FILE,
    mut ar: *mut DWORD,
    mut count: COUNT,
) -> COUNT {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/state.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                124 as ::core::ffi::c_int,
                b"sread_a32\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if sread_32(fp, ar) as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            return 0 as COUNT;
        }
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
    return 1 as COUNT;
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
unsafe extern "C" fn swrite_16(mut fp: *mut GAME_STATE_FILE, mut v: UWORD) -> COUNT {
    return WriteStateFile(
        &raw mut v as *const ::core::ffi::c_void,
        2 as COUNT,
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
unsafe extern "C" fn swrite_a32(
    mut fp: *mut GAME_STATE_FILE,
    mut ar: *const DWORD,
    mut count: COUNT,
) -> COUNT {
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if swrite_32(fp, *ar) as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            return 0 as COUNT;
        }
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
    return 1 as COUNT;
}
