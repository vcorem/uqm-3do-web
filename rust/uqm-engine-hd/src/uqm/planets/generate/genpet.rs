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

// Rust translation of src/uqm/planets/generate/genpet.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn ReinitQueue(pq: *mut QUEUE);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn MoveGameClockDays(days: COUNT);
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    fn matchWorld(
        solarSys: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        planetI: BYTE,
        moonI: BYTE,
    ) -> bool;
    fn LoadStdLanderFont(info: *mut PLANET_INFO);
    fn ComputeSpeed(planet: *mut PLANET_DESC, GeneratingMoons: BOOLEAN, rand_val: UWORD);
    fn PickClosestHabitable(solarSys: *mut SOLARSYS_STATE) -> BYTE;
    fn GenerateDefault_initNpcs(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_reinitNpcs(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_uninitNpcs(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_generatePlanets(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_generateMoons(
        solarSys: *mut SOLARSYS_STATE,
        planet: *mut PLANET_DESC,
    ) -> bool;
    fn GenerateDefault_generateName(
        _: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
    ) -> bool;
    fn GenerateDefault_generateOrbital(
        solarSys: *mut SOLARSYS_STATE,
        world: *mut PLANET_DESC,
    ) -> bool;
    fn GenerateDefault_generateMinerals(
        _: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        whichNode: COUNT,
        _: *mut NODE_INFO,
    ) -> COUNT;
    fn GenerateDefault_generateLife(
        _: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        whichNode: COUNT,
        _: *mut NODE_INFO,
    ) -> COUNT;
    fn GenerateDefault_pickupMinerals(
        _: *mut SOLARSYS_STATE,
        world: *mut PLANET_DESC,
        whichNode: COUNT,
    ) -> bool;
    fn GenerateDefault_pickupLife(
        _: *mut SOLARSYS_STATE,
        world: *mut PLANET_DESC,
        whichNode: COUNT,
    ) -> bool;
    fn GenerateDefault_generateRuins(
        _: *const SOLARSYS_STATE,
        whichNode: COUNT,
        info: *mut NODE_INFO,
    ) -> COUNT;
    fn GenerateDefault_landerReportCycle(_: *mut SOLARSYS_STATE) -> bool;
    fn GenerateHabitableWorld() -> BYTE;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn SeedFleetLocation(
        FleetPtr: *mut FLEET_INFO,
        plotmap: *mut PLOT_LOCATION,
        visit: COUNT,
    ) -> POINT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut plot_map: [PLOT_LOCATION; 70];
    fn InitCommunication(which_comm: CONVERSATION) -> COUNT;
    fn NotifyOthers(which_race: COUNT, target_loc: BYTE);
    fn SetBattlePlanet();
    fn GetGroupInfo(offset: DWORD, which_group: BYTE) -> BOOLEAN;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
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
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
pub type PLOT_LOCATION = plot_location;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub type HELEMENT = HLINK;
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
unsafe extern "C" fn LockFleetInfo(
    mut pq: *const QUEUE,
    mut h: HFLEETINFO,
) -> *mut FLEET_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<FLEET_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (FLEET_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/generate/../../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[no_mangle]
pub static mut generateTalkingPetFunctions: GenerateFunctions = GenerateFunctions {
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
        GenerateTalkingPet_generatePlanets
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
        GenerateTalkingPet_generateOrbital
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
        GenerateTalkingPet_generateEnergy
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
        GenerateTalkingPet_pickupEnergy
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
    pickupLife: Some(
        GenerateDefault_pickupLife
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
};
unsafe extern "C" fn GenerateTalkingPet_generatePlanets(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    let mut pPlanet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut pSunDesc: *mut PLANET_DESC = (&raw mut (*solarSys).SunDesc
        as *mut PLANET_DESC)
        .offset(0isize);
    GenerateDefault_generatePlanets(solarSys);
    if optSeedType == C2Rust_Unnamed::OPTVAL_PRIME.0 as ::core::ffi::c_int {
        let mut angle: COUNT = 0;
        (*pSunDesc).PlanetByte = 0 as BYTE;
        pPlanet = (&raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC)
            .offset((*pSunDesc).PlanetByte as isize);
        (*pPlanet).data_index = C2Rust_Unnamed_1::TELLURIC_WORLD.0 as ::core::ffi::c_int
            as BYTE;
        (*pPlanet).radius = (((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            as ::core::ffi::c_long * 204 as ::core::ffi::c_long
            / 100 as ::core::ffi::c_long) as SIZE;
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
    } else {
        (*pSunDesc).PlanetByte = PickClosestHabitable(solarSys);
        pPlanet = (&raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC)
            .offset((*pSunDesc).PlanetByte as isize);
        (*pPlanet).data_index = GenerateHabitableWorld();
    }
    return true;
}
unsafe extern "C" fn GenerateTalkingPet_generateOrbital(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
) -> bool {
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -1 as ::core::ffi::c_int as BYTE,
    ) as ::core::ffi::c_int != 0
        && (uqm_lua_checked!(getGameStateUint(
            b"UMGAH_ZOMBIE_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
        ), false) != 0
            || uqm_lua_checked!(getGameStateUint(b"TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char), false)
                == 0
            || StartSphereTracking(RACE_ID::UMGAH_SHIP) as ::core::ffi::c_int != 0)
    {
        NotifyOthers(
            RACE_ID::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
            -1 as ::core::ffi::c_int as BYTE,
        );
        PutGroupInfo(
            0 as ::core::ffi::c_long as DWORD,
            !(0 as ::core::ffi::c_int) as BYTE,
        );
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/planets/generate/genpet.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    107 as ::core::ffi::c_int,
                    b"GenerateTalkingPet_generateOrbital\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if StartSphereTracking(RACE_ID::UMGAH_SHIP) != 0 {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_0::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                as ACTIVITY;
            uqm_lua_checked!(setGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD,
            ), false);
            if uqm_lua_checked!(getGameStateUint(
                b"UMGAH_ZOMBIE_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
            ), false) == 0
            {
                CloneShipFragment(
                    RACE_ID::UMGAH_SHIP,
                    &raw mut GlobData.Game_state.npc_built_ship_q,
                    !(0 as ::core::ffi::c_int) as COUNT,
                );
                uqm_lua_checked!(InitCommunication(CONVERSATION::UMGAH_CONVERSATION), false);
            } else {
                let mut i: COUNT = 0;
                i = 0 as COUNT;
                while (i as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                    CloneShipFragment(
                        RACE_ID::UMGAH_SHIP,
                        &raw mut GlobData.Game_state.npc_built_ship_q,
                        0 as COUNT,
                    );
                    i = i.wrapping_add(1);
                }
                uqm_lua_checked!(InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION), false);
            }
        }
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        {
            let mut UmgahSurvivors: BOOLEAN = BOOLEAN::FALSE;
            UmgahSurvivors = BOOLEAN(
                !GlobData.Game_state.npc_built_ship_q.head.is_null()
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_0::START_INTERPLANETARY.0 as ::core::ffi::c_int))
                as ACTIVITY;
            if uqm_lua_checked!(getGameStateUint(
                b"PLAYER_HYPNOTIZED\0".as_ptr() as *const ::core::ffi::c_char,
            ), false) != 0
            {
                uqm_lua_checked!(ZapToUrquanEncounter(), false);
            } else if uqm_lua_checked!(getGameStateUint(
                b"UMGAH_ZOMBIE_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
            ), false) != 0 && UmgahSurvivors.0 == 0
            {
                uqm_lua_checked!(InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION), false);
            } else if StartSphereTracking(RACE_ID::UMGAH_SHIP) == 0 {
                uqm_lua_checked!(InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION), false);
            }
            ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
            GetGroupInfo(
                0 as ::core::ffi::c_long as DWORD,
                0 as ::core::ffi::c_int as BYTE,
            );
        }
        return true;
    }
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -1 as ::core::ffi::c_int as BYTE,
    ) {
        uqm_lua_checked!(LoadStdLanderFont(&raw mut (*solarSys).SysInfo.PlanetInfo), false);
        (*solarSys).PlanetSideFrame[1usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.ruins\0".as_ptr() as RESOURCE), false) as DRAWABLE,
        );
        (*solarSys).SysInfo.PlanetInfo.DiscoveryString = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(b"text.ruins\0".as_ptr() as RESOURCE), false),
        );
    }
    uqm_lua_checked!(GenerateDefault_generateOrbital(solarSys, world), false);
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -1 as ::core::ffi::c_int as BYTE,
    ) {
        (*solarSys).SysInfo.PlanetInfo.Weather = 0 as UWORD;
    }
    return true;
}
unsafe extern "C" fn GenerateTalkingPet_generateEnergy(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -1 as ::core::ffi::c_int as BYTE,
    ) {
        return GenerateDefault_generateRuins(solarSys, whichNode, info);
    }
    return 0 as COUNT;
}
unsafe extern "C" fn GenerateTalkingPet_pickupEnergy(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut whichNode: COUNT,
) -> bool {
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -1 as ::core::ffi::c_int as BYTE,
    ) {
        uqm_lua_checked!(GenerateDefault_landerReportCycle(solarSys), false);
        return false;
    }
    return false;
}
unsafe extern "C" fn ZapToUrquanEncounter() {
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hEncounter = AllocLink(&raw mut GlobData.Game_state.encounter_q) as HENCOUNTER;
    if !hEncounter.is_null()
        || {
            hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
            !hEncounter.is_null()
        }
    {
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut BSIPtr: *mut BRIEF_SHIP_INFO = ::core::ptr::null_mut::<
            BRIEF_SHIP_INFO,
        >();
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        if hEncounter == GlobData.Game_state.encounter_q.head {
            RemoveQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        }
        memset(
            EncounterPtr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ENCOUNTER>() as size_t,
        );
        InsertQueue(
            &raw mut GlobData.Game_state.encounter_q,
            hEncounter,
            GlobData.Game_state.encounter_q.head,
        );
        hStarShip = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            RACE_ID::URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
        ) as HFLEETINFO;
        TemplatePtr = LockFleetInfo(
            &raw mut GlobData.Game_state.avail_race_q,
            hStarShip,
        );
        (*EncounterPtr).origin = (*TemplatePtr).loc;
        (*EncounterPtr).radius = (*TemplatePtr).actual_strength;
        (*EncounterPtr).race_id = RACE_ID::URQUAN_SHIP.0 as ::core::ffi::c_int as BYTE;
        (*EncounterPtr).num_ships = 1 as BYTE;
        (*EncounterPtr).flags = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
            as BYTE;
        BSIPtr = (&raw mut (*EncounterPtr).ShipList as *mut BRIEF_SHIP_INFO)
            .offset(0isize);
        (*BSIPtr).race_id = RACE_ID::URQUAN_SHIP.0 as ::core::ffi::c_int as BYTE;
        (*BSIPtr).crew_level = (*TemplatePtr).crew_level;
        (*BSIPtr).max_crew = (*TemplatePtr).max_crew;
        (*BSIPtr).max_energy = (*TemplatePtr).max_energy;
        let mut hUmgah: HFLEETINFO = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            RACE_ID::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
        );
        let mut UmgahPtr: *mut FLEET_INFO = LockFleetInfo(
            &raw mut GlobData.Game_state.avail_race_q,
            hUmgah,
        );
        (*EncounterPtr).loc_pt = SeedFleetLocation(
            UmgahPtr,
            &raw mut plot_map as *mut PLOT_LOCATION,
            C2Rust_Unnamed_2::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        );
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hUmgah);
        (*EncounterPtr).log_x = universeToLogx((*EncounterPtr).loc_pt.x);
        (*EncounterPtr).log_y = universeToLogy((*EncounterPtr).loc_pt.y);
        GlobData.SIS_state.log_x = (*EncounterPtr).log_x;
        GlobData.SIS_state.log_y = (*EncounterPtr).log_y;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
        uqm_lua_checked!(MoveGameClockDays(15 as COUNT), ());
        GlobData.Game_state.CurrentActivity = (((0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | C2Rust_Unnamed_0::IN_HYPERSPACE.0 as ::core::ffi::c_int as BYTE
                as ::core::ffi::c_int
            | C2Rust_Unnamed_0::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        uqm_lua_checked!(SetBattlePlanet(), ());
        dx = ((*CurStarDescPtr).star_pt.x as ::core::ffi::c_int
            - (*EncounterPtr).loc_pt.x as ::core::ffi::c_int) as SIZE;
        dy = ((*CurStarDescPtr).star_pt.y as ::core::ffi::c_int
            - (*EncounterPtr).loc_pt.y as ::core::ffi::c_int) as SIZE;
        dx = (square_root(
            (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD,
        ) as SIZE as ::core::ffi::c_int
            + (100 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as SIZE;
        if optInfiniteFuel.0 == 0 {
            uqm_lua_checked!(DeltaSISGauges(0 as SIZE, -(dx as SDWORD), 0 as ::core::ffi::c_int), ());
        }
        if GlobData.SIS_state.FuelOnBoard
            < (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
        {
            dx = ((5 as ::core::ffi::c_int
                + TFB_Random() as COUNT as ::core::ffi::c_int % 5 as ::core::ffi::c_int)
                * 100 as ::core::ffi::c_int
                - GlobData.SIS_state.FuelOnBoard as SIZE as ::core::ffi::c_int) as SIZE;
            uqm_lua_checked!(DeltaSISGauges(0 as SIZE, dx as SDWORD, 0 as ::core::ffi::c_int), ());
        }
        uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), ());
        GlobData.Game_state.ShipStamp.origin = (*EncounterPtr).loc_pt;
        GlobData.Game_state.ShipFacing = 6 as UWORD;
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
    }
}
