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

// Rust translation of src/uqm/planets/generate/gendefault.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type font_desc;
    pub type RandomContext;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn ReinitQueue(pq: *mut QUEUE);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableIndex(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn SetRelStringTableIndex(String: STRING, StringTableOffs: SIZE) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_GetSeed(context: *mut RandomContext) -> DWORD;
    fn RangeMinMax(
        min: ::core::ffi::c_int,
        max: ::core::ffi::c_int,
        rand: DWORD,
    ) -> ::core::ffi::c_int;
    static mut SysGenRNG: *mut RandomContext;
    fn planetIndex(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> COUNT;
    fn LoadPlanet(SurfDefFrame: FRAME);
    fn ComputeSpeed(planet: *mut PLANET_DESC, GeneratingMoons: BOOLEAN, rand_val: UWORD);
    fn FillOrbits(
        system: *mut SOLARSYS_STATE,
        NumPlanets: BYTE,
        pBaseDesc: *mut PLANET_DESC,
        TypesDefined: BOOLEAN,
    );
    fn GenerateMineralDeposits(
        _: *const SYSTEM_INFO,
        whichDeposit: COUNT,
        info: *mut NODE_INFO,
    ) -> COUNT;
    fn GenerateLifeForms(
        _: *const SYSTEM_INFO,
        whichLife: COUNT,
        info: *mut NODE_INFO,
    ) -> COUNT;
    fn GenerateRandomNodes(
        _: *const SYSTEM_INFO,
        scan: COUNT,
        numNodes: COUNT,
        r#type: COUNT,
        whichNode: COUNT,
        info: *mut NODE_INFO,
    ) -> COUNT;
    fn DoPlanetaryAnalysis(SysInfoPtr: *mut SYSTEM_INFO, pPlanetDesc: *mut PLANET_DESC);
    static mut PlanData: *const PlanetFrame;
    static mut EncounterRace: SIZE;
    fn DoDiscoveryReport(ReadOutSounds: SOUND);
    static mut GameStrings: STRING;
    static mut SpaceMusicOK: BOOLEAN;
    static mut MenuSounds: SOUND;
    fn GetGroupInfo(offset: DWORD, which_group: BYTE) -> BOOLEAN;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    fn BuildGroups();
    fn findRaceSOI();
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
pub type size_t = ::core::ffi::c_ulong;
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
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
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
pub type SOUND = STRING;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
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
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const FIRST_ROCKY_WORLD: Self = Self(0);
    pub const FIRST_SMALL_ROCKY_WORLD: Self = Self(0);
    pub const OOLITE_WORLD: Self = Self(0);
    pub const YTTRIC_WORLD: Self = Self(1);
    pub const QUASI_DEGENERATE_WORLD: Self = Self(2);
    pub const LANTHANIDE_WORLD: Self = Self(3);
    pub const TREASURE_WORLD: Self = Self(4);
    pub const UREA_WORLD: Self = Self(5);
    pub const METAL_WORLD: Self = Self(6);
    pub const RADIOACTIVE_WORLD: Self = Self(7);
    pub const OPALESCENT_WORLD: Self = Self(8);
    pub const CYANIC_WORLD: Self = Self(9);
    pub const ACID_WORLD: Self = Self(10);
    pub const ALKALI_WORLD: Self = Self(11);
    pub const HALIDE_WORLD: Self = Self(12);
    pub const GREEN_WORLD: Self = Self(13);
    pub const COPPER_WORLD: Self = Self(14);
    pub const CARBIDE_WORLD: Self = Self(15);
    pub const ULTRAMARINE_WORLD: Self = Self(16);
    pub const NOBLE_WORLD: Self = Self(17);
    pub const AZURE_WORLD: Self = Self(18);
    pub const CHONDRITE_WORLD: Self = Self(19);
    pub const PURPLE_WORLD: Self = Self(20);
    pub const SUPER_DENSE_WORLD: Self = Self(21);
    pub const PELLUCID_WORLD: Self = Self(22);
    pub const DUST_WORLD: Self = Self(23);
    pub const CRIMSON_WORLD: Self = Self(24);
    pub const CIMMERIAN_WORLD: Self = Self(25);
    pub const INFRARED_WORLD: Self = Self(26);
    pub const SELENIC_WORLD: Self = Self(27);
    pub const AURIC_WORLD: Self = Self(28);
    pub const LAST_SMALL_ROCKY_WORLD: Self = Self(28);
    pub const FIRST_LARGE_ROCKY_WORLD: Self = Self(29);
    pub const FLUORESCENT_WORLD: Self = Self(29);
    pub const ULTRAVIOLET_WORLD: Self = Self(30);
    pub const PLUTONIC_WORLD: Self = Self(31);
    pub const RAINBOW_WORLD: Self = Self(32);
    pub const SHATTERED_WORLD: Self = Self(33);
    pub const SAPPHIRE_WORLD: Self = Self(34);
    pub const ORGANIC_WORLD: Self = Self(35);
    pub const XENOLITHIC_WORLD: Self = Self(36);
    pub const REDUX_WORLD: Self = Self(37);
    pub const PRIMORDIAL_WORLD: Self = Self(38);
    pub const EMERALD_WORLD: Self = Self(39);
    pub const CHLORINE_WORLD: Self = Self(40);
    pub const MAGNETIC_WORLD: Self = Self(41);
    pub const WATER_WORLD: Self = Self(42);
    pub const TELLURIC_WORLD: Self = Self(43);
    pub const HYDROCARBON_WORLD: Self = Self(44);
    pub const IODINE_WORLD: Self = Self(45);
    pub const VINYLOGOUS_WORLD: Self = Self(46);
    pub const RUBY_WORLD: Self = Self(47);
    pub const MAGMA_WORLD: Self = Self(48);
    pub const MAROON_WORLD: Self = Self(49);
    pub const LAST_LARGE_ROCKY_WORLD: Self = Self(49);
    pub const LAST_ROCKY_WORLD: Self = Self(49);
    pub const FIRST_GAS_GIANT: Self = Self(50);
    pub const BLU_GAS_GIANT: Self = Self(50);
    pub const CYA_GAS_GIANT: Self = Self(51);
    pub const GRN_GAS_GIANT: Self = Self(52);
    pub const GRY_GAS_GIANT: Self = Self(53);
    pub const ORA_GAS_GIANT: Self = Self(54);
    pub const PUR_GAS_GIANT: Self = Self(55);
    pub const RED_GAS_GIANT: Self = Self(56);
    pub const VIO_GAS_GIANT: Self = Self(57);
    pub const YEL_GAS_GIANT: Self = Self(58);
    pub const LAST_GAS_GIANT: Self = Self(58);
    pub const NUMBER_OF_PLANET_TYPES: Self = Self(59);
    pub const WORLD_TYPE_SPECIAL: Self = Self(128);
    pub const PLANET_SHIELDED: Self = Self(128);
    pub const HIERARCHY_STARBASE: Self = Self(255);
    pub const SA_MATRA: Self = Self(254);
    pub const DESTROYED_STARBASE: Self = Self(253);
    pub const PRECURSOR_STARBASE: Self = Self(252);
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
                    b"src/libs/math/../../uqm/displist.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"src/libs/math/../../uqm/displist.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub static mut generateDefaultFunctions: GenerateFunctions = GenerateFunctions {
    initNpcs: Some(
        GenerateDefault_initNpcs as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    reinitNpcs: Some(
        GenerateDefault_reinitNpcs as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    uninitNpcs: Some(
        GenerateDefault_uninitNpcs as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    generatePlanets: Some(
        GenerateDefault_generatePlanets
            as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    generateMoons: Some(
        GenerateDefault_generateMoons
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
    ),
    generateName: Some(
        GenerateDefault_generateName
            as unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
    ),
    generateOrbital: Some(
        GenerateDefault_generateOrbital
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
    ),
    generateMinerals: Some(
        GenerateDefault_generateMinerals
            as unsafe extern "C" fn(
                *const SOLARSYS_STATE,
                *const PLANET_DESC,
                COUNT,
                *mut NODE_INFO,
            ) -> COUNT,
    ),
    generateEnergy: Some(
        GenerateDefault_generateEnergy
            as unsafe extern "C" fn(
                *const SOLARSYS_STATE,
                *const PLANET_DESC,
                COUNT,
                *mut NODE_INFO,
            ) -> COUNT,
    ),
    generateLife: Some(
        GenerateDefault_generateLife
            as unsafe extern "C" fn(
                *const SOLARSYS_STATE,
                *const PLANET_DESC,
                COUNT,
                *mut NODE_INFO,
            ) -> COUNT,
    ),
    pickupMinerals: Some(
        GenerateDefault_pickupMinerals
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
    pickupEnergy: Some(
        GenerateDefault_pickupEnergy
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
    pickupLife: Some(
        GenerateDefault_pickupLife
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
};
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_initNpcs(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    if GetGroupInfo(
            GlobData.Game_state.BattleGroupRef,
            !(0 as ::core::ffi::c_int) as BYTE,
        )
        .0 == 0
    {
        GlobData.Game_state.BattleGroupRef = 0 as DWORD;
        uqm_lua_checked!(BuildGroups(), false);
    }
    if SpaceMusicOK.0 != 0 {
        uqm_lua_checked!(findRaceSOI(), false);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_reinitNpcs(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    GetGroupInfo(0 as ::core::ffi::c_long as DWORD, 0 as ::core::ffi::c_int as BYTE);
    uqm_lua_checked!(check_yehat_rebellion(), false);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_uninitNpcs(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    PutGroupInfo(0 as ::core::ffi::c_long as DWORD, !(0 as ::core::ffi::c_int) as BYTE);
    ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generatePlanets(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    FillOrbits(
        solarSys,
        !(0 as ::core::ffi::c_int) as BYTE,
        &raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC,
        BOOLEAN::FALSE,
    );
    GeneratePlanets(solarSys);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateMoons(
    mut solarSys: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
) -> bool {
    FillOrbits(
        solarSys,
        (*planet).NumPlanets,
        &raw mut (*solarSys).MoonDesc as *mut PLANET_DESC,
        BOOLEAN::FALSE,
    );
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateName(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> bool {
    let mut i: COUNT = planetIndex(solarSys, world);
    utf8StringCopy(
        &raw mut GlobData.SIS_state.PlanetName as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 32]>() as size_t,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                    + (9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                    + i as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(setGameStateUint(
        b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
        (*world).data_index as DWORD,
    ), false);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateOrbital(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
) -> bool {
    let mut rand_val: DWORD = 0;
    let mut sysInfo: *mut SYSTEM_INFO = ::core::ptr::null_mut::<SYSTEM_INFO>();
    sysInfo = &raw mut (*solarSys).SysInfo;
    DoPlanetaryAnalysis(sysInfo, world);
    rand_val = RandomContext_GetSeed(SysGenRNG);
    (*sysInfo)
        .PlanetInfo
        .ScanSeed[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as usize] = rand_val;
    GenerateLifeForms(
        sysInfo,
        !(0 as ::core::ffi::c_int) as COUNT,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    rand_val = RandomContext_GetSeed(SysGenRNG);
    (*sysInfo)
        .PlanetInfo
        .ScanSeed[PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as usize] = rand_val;
    GenerateMineralDeposits(
        sysInfo,
        !(0 as ::core::ffi::c_int) as COUNT,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    (*sysInfo)
        .PlanetInfo
        .ScanSeed[PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int as usize] = rand_val;
    uqm_lua_checked!(LoadPlanet(::core::ptr::null_mut::<FRAME_DESC>()), false);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateMinerals(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    return GenerateMineralDeposits(&raw const (*solarSys).SysInfo, whichNode, info);
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_pickupMinerals(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut whichNode: COUNT,
) -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateEnergy(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_pickupEnergy(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut whichNode: COUNT,
) -> bool {
    (false
        || {
            __assert_fail(
                b"false\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/generate/gendefault.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int,
                b"GenerateDefault_pickupEnergy\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateLife(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    return GenerateLifeForms(&raw const (*solarSys).SysInfo, whichNode, info);
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_pickupLife(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut whichNode: COUNT,
) -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateArtifact(
    mut solarSys: *const SOLARSYS_STATE,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    return GenerateRandomNodes(
        &raw const (*solarSys).SysInfo,
        PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int as COUNT,
        1 as COUNT,
        0 as COUNT,
        whichNode,
        info,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_generateRuins(
    mut solarSys: *const SOLARSYS_STATE,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    return GenerateRandomNodes(
        &raw const (*solarSys).SysInfo,
        PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int as COUNT,
        16 as COUNT,
        0 as COUNT,
        whichNode,
        info,
    );
}
#[inline]
unsafe extern "C" fn runLanderReport() {
    UnbatchGraphics();
    uqm_lua_checked!(DoDiscoveryReport(MenuSounds), ());
    BatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_landerReport(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    let mut planetInfo: *mut PLANET_INFO = &raw mut (*solarSys).SysInfo.PlanetInfo;
    if (*planetInfo).DiscoveryString.is_null() {
        return false;
    }
    uqm_lua_checked!(runLanderReport(), false);
    DestroyStringTable(ReleaseStringTable((*planetInfo).DiscoveryString));
    (*planetInfo).DiscoveryString = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDefault_landerReportCycle(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    let mut planetInfo: *mut PLANET_INFO = &raw mut (*solarSys).SysInfo.PlanetInfo;
    if (*planetInfo).DiscoveryString.is_null() {
        return false;
    }
    uqm_lua_checked!(runLanderReport(), false);
    (*planetInfo).DiscoveryString = SetRelStringTableIndex(
        (*planetInfo).DiscoveryString,
        1 as SIZE,
    );
    if GetStringTableIndex((*planetInfo).DiscoveryString) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        DestroyStringTable(ReleaseStringTable((*planetInfo).DiscoveryString));
        (*planetInfo).DiscoveryString = ::core::ptr::null_mut::<
            STRING_TABLE_ENTRY_DESC,
        >();
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn GeneratePlanets(mut solarSys: *mut SOLARSYS_STATE) {
    let mut i: COUNT = 0;
    let mut planet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    i = (*solarSys).SunDesc[0usize].NumPlanets as COUNT;
    planet = (&raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC).offset(0isize);
    while i != 0 {
        let mut rand_val: DWORD = 0;
        let mut byte_val: BYTE = 0;
        let mut num_moons: BYTE = 0;
        let mut r#type: BYTE = 0;
        rand_val = RandomContext_Random(SysGenRNG);
        byte_val = rand_val as UWORD as BYTE;
        num_moons = 0 as BYTE;
        r#type = (*PlanData
            .offset(
                ((*planet).data_index as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_0::PLANET_SHIELDED.0 as ::core::ffi::c_int))
                    as isize,
            ))
            .Type;
        match (r#type as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
        {
            1 => {
                if (byte_val as ::core::ffi::c_int)
                    < 0xff as ::core::ffi::c_int * 25 as ::core::ffi::c_int
                        / 100 as ::core::ffi::c_int
                {
                    if (byte_val as ::core::ffi::c_int)
                        < 0xff as ::core::ffi::c_int * 5 as ::core::ffi::c_int
                            / 100 as ::core::ffi::c_int
                    {
                        num_moons = num_moons.wrapping_add(1);
                    }
                    num_moons = num_moons.wrapping_add(1);
                }
            }
            2 => {
                if (byte_val as ::core::ffi::c_int)
                    < 0xff as ::core::ffi::c_int * 90 as ::core::ffi::c_int
                        / 100 as ::core::ffi::c_int
                {
                    if (byte_val as ::core::ffi::c_int)
                        < 0xff as ::core::ffi::c_int * 75 as ::core::ffi::c_int
                            / 100 as ::core::ffi::c_int
                    {
                        if (byte_val as ::core::ffi::c_int)
                            < 0xff as ::core::ffi::c_int * 50 as ::core::ffi::c_int
                                / 100 as ::core::ffi::c_int
                        {
                            if (byte_val as ::core::ffi::c_int)
                                < 0xff as ::core::ffi::c_int * 25 as ::core::ffi::c_int
                                    / 100 as ::core::ffi::c_int
                            {
                                num_moons = num_moons.wrapping_add(1);
                            }
                            num_moons = num_moons.wrapping_add(1);
                        }
                        num_moons = num_moons.wrapping_add(1);
                    }
                    num_moons = num_moons.wrapping_add(1);
                }
            }
            _ => {}
        }
        (*planet).NumPlanets = num_moons;
        i = i.wrapping_sub(1);
        planet = planet.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GenerateWorlds(mut whichType: BYTE) -> BYTE {
    let mut planet: BYTE = C2Rust_Unnamed_0::FIRST_SMALL_ROCKY_WORLD.0
        as ::core::ffi::c_int as BYTE;
    if whichType as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        planet = (C2Rust_Unnamed_0::FIRST_SMALL_ROCKY_WORLD.0 as ::core::ffi::c_int
            as DWORD)
            .wrapping_add(
                RandomContext_Random(SysGenRNG)
                    .wrapping_rem(
                        (C2Rust_Unnamed_0::LAST_SMALL_ROCKY_WORLD.0 as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::FIRST_SMALL_ROCKY_WORLD.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as DWORD,
                    ),
            ) as BYTE;
    } else if whichType as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        planet = (C2Rust_Unnamed_0::FIRST_LARGE_ROCKY_WORLD.0 as ::core::ffi::c_int
            as DWORD)
            .wrapping_add(
                RandomContext_Random(SysGenRNG)
                    .wrapping_rem(
                        (C2Rust_Unnamed_0::LAST_LARGE_ROCKY_WORLD.0 as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::FIRST_LARGE_ROCKY_WORLD.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            - 2 as ::core::ffi::c_int) as DWORD,
                    ),
            ) as BYTE;
        if planet as ::core::ffi::c_int
            >= C2Rust_Unnamed_0::RAINBOW_WORLD.0 as ::core::ffi::c_int
        {
            planet = (planet as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
        }
    } else if whichType as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        planet = (C2Rust_Unnamed_0::FIRST_ROCKY_WORLD.0 as ::core::ffi::c_int as DWORD)
            .wrapping_add(
                RandomContext_Random(SysGenRNG)
                    .wrapping_rem(
                        (C2Rust_Unnamed_0::LAST_SMALL_ROCKY_WORLD.0 as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::FIRST_SMALL_ROCKY_WORLD.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            + (C2Rust_Unnamed_0::LAST_LARGE_ROCKY_WORLD.0
                                as ::core::ffi::c_int
                                - C2Rust_Unnamed_0::FIRST_LARGE_ROCKY_WORLD.0
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            - 2 as ::core::ffi::c_int) as DWORD,
                    ),
            ) as BYTE;
        if planet as ::core::ffi::c_int
            >= C2Rust_Unnamed_0::RAINBOW_WORLD.0 as ::core::ffi::c_int
        {
            planet = (planet as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
        }
    } else if whichType as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        planet = (C2Rust_Unnamed_0::FIRST_LARGE_ROCKY_WORLD.0 as ::core::ffi::c_int
            as DWORD)
            .wrapping_add(
                RandomContext_Random(SysGenRNG)
                    .wrapping_rem(
                        (C2Rust_Unnamed_0::LAST_LARGE_ROCKY_WORLD.0 as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::FIRST_LARGE_ROCKY_WORLD.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            + (C2Rust_Unnamed_0::LAST_GAS_GIANT.0 as ::core::ffi::c_int
                                - C2Rust_Unnamed_0::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int)
                            as DWORD,
                    ),
            ) as BYTE;
        if planet as ::core::ffi::c_int
            >= C2Rust_Unnamed_0::RAINBOW_WORLD.0 as ::core::ffi::c_int
        {
            planet = (planet as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
        }
    } else if whichType as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
    {
        planet = (C2Rust_Unnamed_0::FIRST_GAS_GIANT.0 as ::core::ffi::c_int as DWORD)
            .wrapping_add(
                RandomContext_Random(SysGenRNG)
                    .wrapping_rem(
                        (C2Rust_Unnamed_0::LAST_GAS_GIANT.0 as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as DWORD,
                    ),
            ) as BYTE;
    }
    return planet;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateGasGiantRanged(mut solarSys: *mut SOLARSYS_STATE) {
    let mut pSunDesc: *mut PLANET_DESC = (&raw mut (*solarSys).SunDesc
        as *mut PLANET_DESC)
        .offset(0isize);
    let mut pPlanet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut i: BYTE = 0;
    let mut rand: DWORD = RandomContext_GetSeed(SysGenRNG);
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < (*pSunDesc).NumPlanets as ::core::ffi::c_int {
        if (*solarSys).PlanetDesc[i as usize].radius as ::core::ffi::c_int
            >= (12 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i as ::core::ffi::c_int == (*pSunDesc).NumPlanets as ::core::ffi::c_int {
        i = rand.wrapping_rem((*pSunDesc).NumPlanets as DWORD) as BYTE;
    } else {
        i = (i as DWORD)
            .wrapping_add(
                rand
                    .wrapping_rem(
                        ((*pSunDesc).NumPlanets as ::core::ffi::c_int
                            - i as ::core::ffi::c_int) as DWORD,
                    ),
            ) as BYTE;
    }
    (*pSunDesc).PlanetByte = i;
    pPlanet = (&raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC)
        .offset((*pSunDesc).PlanetByte as isize);
    (*pPlanet).data_index = GenerateWorlds(
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE,
    );
    if ((*solarSys).PlanetDesc[i as usize].radius as ::core::ffi::c_int)
        < (12 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
    {
        let mut angle: COUNT = 0;
        (*pPlanet).radius = RangeMinMax(
            (12 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
            (124 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
            rand,
        ) as SIZE;
        angle = ARCTAN((*pPlanet).location.x as SDWORD, (*pPlanet).location.y as SDWORD);
        (*pPlanet).location.x = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pPlanet).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        (*pPlanet).location.y = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pPlanet).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        ComputeSpeed(pPlanet, BOOLEAN::FALSE, 1 as UWORD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GenerateCrystalWorld() -> BYTE {
    let mut crystalArray: [::core::ffi::c_int; 3] = [
        C2Rust_Unnamed_0::SAPPHIRE_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::EMERALD_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::RUBY_WORLD.0 as ::core::ffi::c_int,
    ];
    return crystalArray[RandomContext_Random(SysGenRNG).wrapping_rem(3 as DWORD)
        as usize] as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateDesolateWorld() -> BYTE {
    let mut desolateArray: [::core::ffi::c_int; 3] = [
        C2Rust_Unnamed_0::DUST_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::CRIMSON_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::UREA_WORLD.0 as ::core::ffi::c_int,
    ];
    return desolateArray[RandomContext_Random(SysGenRNG).wrapping_rem(3 as DWORD)
        as usize] as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateHabitableWorld() -> BYTE {
    let mut habitableArray: [::core::ffi::c_int; 4] = [
        C2Rust_Unnamed_0::PRIMORDIAL_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::WATER_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::TELLURIC_WORLD.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::REDUX_WORLD.0 as ::core::ffi::c_int,
    ];
    return habitableArray[RandomContext_Random(SysGenRNG).wrapping_rem(4 as DWORD)
        as usize] as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateGasGiantWorld() -> BYTE {
    return (C2Rust_Unnamed_0::FIRST_GAS_GIANT.0 as ::core::ffi::c_int as DWORD)
        .wrapping_add(
            RandomContext_Random(SysGenRNG)
                .wrapping_rem(
                    (C2Rust_Unnamed_0::LAST_GAS_GIANT.0 as ::core::ffi::c_int
                        - C2Rust_Unnamed_0::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as DWORD,
                ),
        ) as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateMinPlanets(mut min: BYTE) -> BYTE {
    let max: BYTE = (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
    if min as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        min = 1 as BYTE;
    }
    if min as ::core::ffi::c_int >= 9 as ::core::ffi::c_int {
        min = 9 as BYTE;
    }
    return RandomContext_Random(SysGenRNG)
        .wrapping_rem((max as ::core::ffi::c_int - min as ::core::ffi::c_int) as DWORD)
        .wrapping_add(min as DWORD) as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateNumberOfPlanets(mut minimum: BYTE) -> BYTE {
    let mut roll: BYTE = (9 as ::core::ffi::c_int - minimum as ::core::ffi::c_int)
        as BYTE;
    let mut adjust: BYTE = (minimum as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as BYTE;
    return RandomContext_Random(SysGenRNG)
        .wrapping_rem(roll as DWORD)
        .wrapping_add(adjust as DWORD) as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn PlanetByteGen(mut pPDesc: *mut PLANET_DESC) -> BYTE {
    return RandomContext_Random(SysGenRNG).wrapping_rem((*pPDesc).NumPlanets as DWORD)
        as BYTE;
}
unsafe extern "C" fn check_yehat_rebellion() {
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if uqm_lua_checked!(getGameStateUint(b"YEHAT_CIVIL_WAR\0".as_ptr() as *const ::core::ffi::c_char), ()) == 0
        || EncounterRace as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
    {
        return;
    }
    hGroup = GlobData.Game_state.ip_group_q.head as HIPGROUP;
    while !hGroup.is_null() {
        let mut GroupPtr: *mut IP_GROUP = LockIpGroup(
            &raw mut GlobData.Game_state.ip_group_q,
            hGroup,
        );
        hNextGroup = (*GroupPtr).succ;
        if (*GroupPtr).race_id as ::core::ffi::c_int
            == C2Rust_Unnamed_1::YEHAT_SHIP.0 as ::core::ffi::c_int
            && (*GroupPtr).task as ::core::ffi::c_int
                & C2Rust_Unnamed_3::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int != 0
        {
            (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
                & C2Rust_Unnamed_3::REFORM_GROUP.0 as ::core::ffi::c_int) as BYTE;
            (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
                | (C2Rust_Unnamed_3::FLEE.0 as ::core::ffi::c_int
                    | C2Rust_Unnamed_3::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int))
                as BYTE;
            (*GroupPtr).dest_loc = 0 as BYTE;
        }
        UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        hGroup = hNextGroup;
    }
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
                b"src/uqm/planets/generate/../../grpinfo.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
