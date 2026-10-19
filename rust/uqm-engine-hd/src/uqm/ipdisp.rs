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

// Rust translation of src/uqm/ipdisp.c.
extern "C" {
    pub type frame_desc;
    pub type string_table_entry;
    pub type font_desc;
    pub type drawable_desc;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
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
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optBubbleWarp: OPT_ENABLABLE;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optShipDirectionIP: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn RemoveElement(hLink: HLINK);
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn CountSISPieces(piece_type: BYTE) -> COUNT;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    fn playerInInnerSystem() -> bool;
    fn planetIndex(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> COUNT;
    fn locationToDisplay(pt: POINT, scaleRadius: SIZE) -> POINT;
    fn displayToLocation(pt: POINT, scaleRadius: SIZE) -> POINT;
    fn planetOuterLocation(planetI: COUNT) -> POINT;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn GetSeededFleetFromIndex(Index: COUNT) -> HFLEETINFO;
    fn GetIndexFromStarShip(pShipQ: *mut QUEUE, hStarShip: HLINK) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
    fn SeedShip(SpeciesID: SPECIES_ID, loadWindow: BOOLEAN) -> SPECIES_ID;
    static mut stars_in_space: FRAME;
    static mut EncounterRace: SIZE;
    static mut EncounterGroup: BYTE;
    fn SetUpElement(ElementPtr: *mut ELEMENT);
    fn InitialIntersect();
    static mut LastActivity: ACTIVITY;
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
pub type HSHIPFRAG = HLINK;
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
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
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
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
pub type ACTIVITY = UWORD;
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
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type TEXT = text;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
pub type LINE = line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type GRAPHICS_PRIM = BYTE;
pub type PRIM_LINKS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
pub type LINK = link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
pub type HELEMENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type TIME_VALUE = UWORD;
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
    pub c2rust_unnamed: C2Rust_Unnamed_4,
    pub c2rust_unnamed_0: C2Rust_Unnamed_3,
    pub c2rust_unnamed_1: C2Rust_Unnamed_2,
    pub c2rust_unnamed_2: C2Rust_Unnamed_1,
    pub c2rust_unnamed_3: C2Rust_Unnamed_0,
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
    pub image: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
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
pub type PLANET_DESC = planet_desc;
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
pub type COLORMAP = STRING;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MAP3D_POINT {
    pub p: [POINT; 4],
    pub m: [DWORD; 4],
}
pub type SOLARSYS_STATE = solarsys_state;
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
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
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
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
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
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
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
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
pub type size_t = usize;
pub type HFLEETINFO = HLINK;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type DRAWABLE_DESC = drawable_desc;
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
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_PL_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_PL_2: Self = Self(1);
    pub const NUM_TEMPLATES: Self = Self(2);
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gfx_object(pub ::core::ffi::c_uint);
impl gfx_object {
    pub const POINT_PRIM: Self = Self(0);
    pub const STAMP_PRIM: Self = Self(1);
    pub const STAMPFILL_PRIM: Self = Self(2);
    pub const LINE_PRIM: Self = Self(3);
    pub const TEXT_PRIM: Self = Self(4);
    pub const RECT_PRIM: Self = Self(5);
    pub const RECTFILL_PRIM: Self = Self(6);
    pub const POINT_PRIM_HD: Self = Self(7);
    pub const NUM_PRIMS: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const KEY_UP: Self = Self(0);
    pub const KEY_DOWN: Self = Self(1);
    pub const KEY_LEFT: Self = Self(2);
    pub const KEY_RIGHT: Self = Self(3);
    pub const KEY_WEAPON: Self = Self(4);
    pub const KEY_SPECIAL: Self = Self(5);
    pub const KEY_ESCAPE: Self = Self(6);
    pub const KEY_THRUST: Self = Self(7);
    pub const NUM_KEYS: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const IN_ORBIT: Self = Self(0);
    pub const EXPLORE: Self = Self(1);
    pub const FLEE: Self = Self(2);
    pub const ON_STATION: Self = Self(3);
    pub const IGNORE_FLAGSHIP: Self = Self(4);
    pub const REFORM_GROUP: Self = Self(8);
}
#[no_mangle]
pub static mut legacySave: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut GTFO: BYTE = 0 as BYTE;
#[inline]
unsafe extern "C" fn RaceIPSpeed(mut Index: RACE_ID) -> SIZE {
    static mut seedStamp: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let defaultMap: [SIZE; 27] = [
        ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((24 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((35 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((48 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((28 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((21 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((20 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((25 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((60 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((18 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((36 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((24 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((50 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
        ((90 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
    ];
    let numRaces: COUNT = ::core::mem::size_of::<[SIZE; 27]>()
        .wrapping_div(::core::mem::size_of::<COUNT>()) as COUNT;
    static mut speedMap: [SIZE; 27] = [
        0 as SIZE,
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
    let mut x: ::core::ffi::c_int = 0;
    if optShipSeed.0 == 0
        && (seedStamp != -1 as ::core::ffi::c_int
            || speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
    {
        x = 0 as ::core::ffi::c_int;
        while x < numRaces as ::core::ffi::c_int {
            speedMap[x as usize] = defaultMap[x as usize];
            x += 1;
        }
        seedStamp = -1 as ::core::ffi::c_int;
        if speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            speedMap[0usize] = 1 as SIZE;
        }
    }
    if optShipSeed.0 != 0
        && (seedStamp != optCustomSeed
            || speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
    {
        let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        x = 0 as ::core::ffi::c_int;
        while x < numRaces as ::core::ffi::c_int {
            hFleet = GetSeededFleetFromIndex(x as COUNT);
            if !hFleet.is_null() {
                speedMap[x as usize] = defaultMap[GetIndexFromStarShip(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hFleet,
                ) as usize];
            } else {
                speedMap[x as usize] = defaultMap[x as usize];
            }
            x += 1;
        }
        if speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            speedMap[0usize] = 1 as SIZE;
        }
    }
    return speedMap[Index.0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn NotifyOthers(mut which_race: COUNT, mut target_loc: BYTE) {
    let mut hGroup: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextGroup: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hGroup = GlobData.Game_state.ip_group_q.head as HSHIPFRAG;
    while !hGroup.is_null() {
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        GroupPtr = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        hNextGroup = (*GroupPtr).succ as HSHIPFRAG;
        if (*GroupPtr).race_id as ::core::ffi::c_int != which_race as ::core::ffi::c_int
        {
            UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        } else {
            if target_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int))
                    as BYTE;
                (*GroupPtr).orbit_pos = (*GroupPtr).dest_loc;
                (*GroupPtr).dest_loc = 0 as BYTE;
            } else if target_loc as ::core::ffi::c_int
                == -1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            {
                (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
                    | C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int) as BYTE;
                if (*GroupPtr).dest_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    (*GroupPtr).dest_loc = (*GroupPtr).orbit_pos;
                    (*GroupPtr).orbit_pos = (TFB_Random()
                        & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD) as BYTE;
                }
                if (*GroupPtr).task as ::core::ffi::c_int
                    & C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int == 0
                {
                    if (*GroupPtr).task as ::core::ffi::c_int
                        & !(C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int)
                        != C2Rust_Unnamed_9::EXPLORE.0 as ::core::ffi::c_int
                    {
                        (*GroupPtr).group_counter = 0 as UWORD;
                    } else {
                        (*GroupPtr).group_counter = ((TFB_Random() as COUNT
                            as ::core::ffi::c_int % 5 as ::core::ffi::c_int)
                            << 4 as ::core::ffi::c_int) as UWORD;
                    }
                }
            } else {
                (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
                    & C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int) as BYTE;
                (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
                    | (C2Rust_Unnamed_9::FLEE.0 as ::core::ffi::c_int
                        | C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int))
                    as BYTE;
                (*GroupPtr).dest_loc = 0 as BYTE;
            }
            UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        }
        hGroup = hNextGroup;
    }
}
unsafe extern "C" fn zoomRadiusForLocation(mut location: BYTE) -> SIZE {
    if location as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return (*pSolarSysState).SunDesc[0usize].radius
    } else {
        return ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE
    };
}
#[inline]
unsafe extern "C" fn adjustDeltaVforZoom(
    mut zoom: SIZE,
    mut dx: *mut SIZE,
    mut dy: *mut SIZE,
) {
    if zoom as ::core::ffi::c_int
        == (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int
    {
        *dx = (*dx as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as SIZE;
        *dy = (*dy as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as SIZE;
    } else if (zoom as ::core::ffi::c_int)
        < (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
    {
        *dx = (*dx as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
        *dy = (*dy as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    }
}
unsafe extern "C" fn getFlagshipLocation() -> BYTE {
    if !playerInInnerSystem() {
        return 0 as BYTE
    } else {
        return (1 as ::core::ffi::c_int
            + planetIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                as ::core::ffi::c_int) as BYTE
    };
}
#[no_mangle]
pub static mut Xform: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut Ywing: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
unsafe extern "C" fn ip_group_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut task: BYTE = 0;
    let mut target_loc: BYTE = 0;
    let mut group_loc: BYTE = 0;
    let mut flagship_loc: BYTE = 0;
    let mut radius: SIZE = 0;
    let mut dest_pt: POINT = POINT { x: 0, y: 0 };
    let mut vdx: SIZE = 0;
    let mut vdy: SIZE = 0;
    let mut EPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
    EPtr = ElementPtr;
    (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    (*EPtr).c2rust_unnamed.life_span = (*EPtr).c2rust_unnamed.life_span.wrapping_add(1);
    GroupPtr = (*EPtr).pParent as *mut IP_GROUP;
    group_loc = (*GroupPtr).sys_loc;
    DisplayArray[(*EPtr).PrimIndex as usize].Object.Point = (*GroupPtr).loc;
    radius = zoomRadiusForLocation(group_loc);
    dest_pt = locationToDisplay((*GroupPtr).loc, radius);
    (*EPtr).current.location.x = (((dest_pt.x as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    (*EPtr).current.location.y = (((dest_pt.y as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    (*EPtr).IntersectControl.IntersectStamp.origin.x = ((*EPtr).current.location.x
        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    (*EPtr).IntersectControl.IntersectStamp.origin.y = ((*EPtr).current.location.y
        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    flagship_loc = getFlagshipLocation();
    if uqm_lua_checked!(getGameStateUint(b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0
        && CheckAlliance(RACE_ID((*GroupPtr).race_id as ::core::ffi::c_uint))
            as ::core::ffi::c_int == C2Rust_Unnamed_6::DEAD_GUY.0 as ::core::ffi::c_int
        || GTFO as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*GroupPtr).race_id as ::core::ffi::c_int
                == RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int
    {
        GTFO = 0 as BYTE;
        NotifyOthers((*GroupPtr).race_id as COUNT, 1 as BYTE);
    }
    if GTFO as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        && (*GroupPtr).race_id as ::core::ffi::c_int
            == RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int
    {
        GTFO = 0 as BYTE;
        NotifyOthers(RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT, 0 as BYTE);
    }
    task = (*GroupPtr).task;
    if task as ::core::ffi::c_int
        & C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int != 0
        && {
            (*GroupPtr).group_counter = (*GroupPtr).group_counter.wrapping_sub(1);
            (*GroupPtr).group_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        }
    {
        task = (task as ::core::ffi::c_int
            & !(C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int)) as BYTE;
        (*GroupPtr).task = task;
        if task as ::core::ffi::c_int
            & !(C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int)
            != C2Rust_Unnamed_9::EXPLORE.0 as ::core::ffi::c_int
        {
            (*GroupPtr).group_counter = 0 as UWORD;
        } else {
            (*GroupPtr).group_counter = ((TFB_Random() as COUNT as ::core::ffi::c_int
                % 5 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UWORD;
        }
    }
    if task as ::core::ffi::c_int
        & !(C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int)
        == C2Rust_Unnamed_9::FLEE.0 as ::core::ffi::c_int
        && task as ::core::ffi::c_int
            & C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int != 0
    {
        (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
    target_loc = (*GroupPtr).dest_loc;
    if task as ::core::ffi::c_int
        & (C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int) == 0
    {
        if target_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && task as ::core::ffi::c_int
                != C2Rust_Unnamed_9::FLEE.0 as ::core::ffi::c_int
        {
            target_loc = flagship_loc;
            if (*EPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                > 5 as ::core::ffi::c_int
            {
                (*EPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
                memset(
                    &raw mut (*EPtr).velocity as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<VELOCITY_DESC>(),
                );
            }
        } else if group_loc as ::core::ffi::c_int == flagship_loc as ::core::ffi::c_int {
            let mut detect_dist: SWORD = 1200 as SWORD;
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
                && CheckAlliance(RACE_ID((*GroupPtr).race_id as ::core::ffi::c_uint))
                    as ::core::ffi::c_int
                    == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
            {
                detect_dist = 0 as SWORD;
            }
            if group_loc as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                detect_dist = (detect_dist as ::core::ffi::c_int
                    * (((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        / ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int))) as SWORD;
                if (*GroupPtr).race_id as ::core::ffi::c_int
                    == RACE_ID::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int
                {
                    detect_dist = ((detect_dist as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as SWORD;
                }
            }
            vdx = (GlobData.Game_state.ip_location.x as ::core::ffi::c_int
                - (*GroupPtr).loc.x as ::core::ffi::c_int) as SIZE;
            vdy = (GlobData.Game_state.ip_location.y as ::core::ffi::c_int
                - (*GroupPtr).loc.y as ::core::ffi::c_int) as SIZE;
            if (vdx as ::core::ffi::c_long * vdx as ::core::ffi::c_long
                + vdy as ::core::ffi::c_long * vdy as ::core::ffi::c_long)
                < detect_dist as ::core::ffi::c_long * detect_dist as ::core::ffi::c_long
            {
                (*EPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
                memset(
                    &raw mut (*EPtr).velocity as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<VELOCITY_DESC>(),
                );
                NotifyOthers((*GroupPtr).race_id as COUNT, 0 as BYTE);
                task = (*GroupPtr).task;
                target_loc = (*GroupPtr).dest_loc;
                if target_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    target_loc = flagship_loc;
                }
            }
        }
    }
    GetCurrentVelocityComponents(&raw mut (*EPtr).velocity, &raw mut vdx, &raw mut vdy);
    task = (task as ::core::ffi::c_int
        & !(C2Rust_Unnamed_9::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int)) as BYTE;
    if optShipSeed.0 != 0 {
        if Xform.is_null() {
            Xform = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicInstance(b"ship.mmrnmhrm.meleeicons\0".as_ptr() as RESOURCE), ())
                    as DRAWABLE,
            );
        }
        if Ywing.is_null() {
            Ywing = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicInstance(b"ship.mmrnmhrm.meleeicons.y\0".as_ptr() as RESOURCE), ())
                    as DRAWABLE,
            );
        }
    }
    if task as ::core::ffi::c_int <= C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
    {
        let mut Transition: BOOLEAN = BOOLEAN::FALSE;
        let mut isOrbiting: BOOLEAN = BOOLEAN::FALSE;
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut angle: COUNT = 0;
        let mut suggestedFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut FilthyCheater: BOOLEAN = BOOLEAN(
            ((*GroupPtr).race_id as ::core::ffi::c_int
                == RACE_ID::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int
                && (optBubbleWarp.0 != 0
                    || CountSISPieces(
                        C2Rust_Unnamed_7::FUSION_THRUSTER.0 as ::core::ffi::c_int as BYTE,
                    ) as ::core::ffi::c_int > 6 as ::core::ffi::c_int))
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        Transition = BOOLEAN::FALSE;
        isOrbiting = BOOLEAN::FALSE;
        if task as ::core::ffi::c_int == C2Rust_Unnamed_9::FLEE.0 as ::core::ffi::c_int {
            dest_pt.x = (((*GroupPtr).loc.x as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as COORD;
            dest_pt.y = (((*GroupPtr).loc.y as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as COORD;
        } else if (task as ::core::ffi::c_int
            != C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
            || (*GroupPtr).dest_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            && group_loc as ::core::ffi::c_int == target_loc as ::core::ffi::c_int
            || task as ::core::ffi::c_int
                == C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
                && (*GroupPtr).dest_loc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && group_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            if (*GroupPtr).dest_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                dest_pt = GlobData.Game_state.ip_location;
            } else {
                let mut orbit_dist: COUNT = 0;
                let mut org: POINT = POINT { x: 0, y: 0 };
                isOrbiting = BOOLEAN::TRUE;
                if task as ::core::ffi::c_int
                    != C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
                {
                    orbit_dist = 2400 as COUNT;
                    org.y = 0 as COORD;
                    org.x = org.y;
                } else {
                    orbit_dist = 1600 as COUNT;
                    org = planetOuterLocation(
                        (target_loc as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as COUNT,
                    );
                }
                angle = (((*GroupPtr).orbit_pos as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                dest_pt.x = (org.x as SDWORD
                    + (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * orbit_dist as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                dest_pt.y = (org.y as SDWORD
                    + (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * orbit_dist as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                if (*GroupPtr).loc.x as ::core::ffi::c_int
                    == dest_pt.x as ::core::ffi::c_int
                    && (*GroupPtr).loc.y as ::core::ffi::c_int
                        == dest_pt.y as ::core::ffi::c_int
                {
                    let mut next_loc: BYTE = 0;
                    (*GroupPtr).orbit_pos = (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as BYTE;
                    angle = (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int))
                        as COUNT;
                    dest_pt.x = (org.x as SDWORD
                        + (*(&raw mut sinetab as *mut SDWORD)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 2 as ::core::ffi::c_int)
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD as isize,
                            ) as ::core::ffi::c_long * orbit_dist as ::core::ffi::c_long
                            >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                    dest_pt.y = (org.y as SDWORD
                        + (*(&raw mut sinetab as *mut SDWORD)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD as isize,
                            ) as ::core::ffi::c_long * orbit_dist as ::core::ffi::c_long
                            >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                    (*EPtr).c2rust_unnamed_3.thrust_wait = !(0 as ::core::ffi::c_int)
                        as BYTE;
                    if (*GroupPtr).group_counter != 0 {
                        (*GroupPtr).group_counter = (*GroupPtr)
                            .group_counter
                            .wrapping_sub(1);
                    } else if task as ::core::ffi::c_int
                        == C2Rust_Unnamed_9::EXPLORE.0 as ::core::ffi::c_int
                        && {
                            next_loc = (TFB_Random() as COUNT as ::core::ffi::c_int
                                % (*pSolarSysState).SunDesc[0usize].NumPlanets
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
                            next_loc as ::core::ffi::c_int
                                != target_loc as ::core::ffi::c_int
                        }
                    {
                        (*EPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
                        target_loc = next_loc;
                        (*GroupPtr).dest_loc = next_loc;
                    }
                }
            }
        } else if group_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*GroupPtr).dest_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                dest_pt = (*pSolarSysState).SunDesc[0usize].location;
            } else {
                dest_pt = planetOuterLocation(
                    (target_loc as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
                );
            }
        } else {
            if task as ::core::ffi::c_int
                == C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
            {
                target_loc = 0 as BYTE;
            }
            dest_pt.x = (((*GroupPtr).loc.x as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as COORD;
            dest_pt.y = (((*GroupPtr).loc.y as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as COORD;
        }
        delta_x = (dest_pt.x as ::core::ffi::c_int
            - (*GroupPtr).loc.x as ::core::ffi::c_int) as SIZE;
        delta_y = (dest_pt.y as ::core::ffi::c_int
            - (*GroupPtr).loc.y as ::core::ffi::c_int) as SIZE;
        angle = ARCTAN(delta_x as SDWORD, delta_y as SDWORD);
        if (*EPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int != 0
            && (*EPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        {
            (*EPtr).c2rust_unnamed_3.thrust_wait = (*EPtr)
                .c2rust_unnamed_3
                .thrust_wait
                .wrapping_sub(1);
        } else if vdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && vdy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || angle as ::core::ffi::c_int
                != (*EPtr).velocity.TravelAngle as ::core::ffi::c_int
        {
            let mut speed: SIZE = 0;
            if (*EPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int != 0
                && (*GroupPtr).dest_loc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                speed = 60 as SIZE;
                if task as ::core::ffi::c_int
                    == C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
                {
                    speed = (speed as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as SIZE;
                }
            } else {
                speed = RaceIPSpeed(RACE_ID((*GroupPtr).race_id as ::core::ffi::c_uint));
                (*EPtr).c2rust_unnamed_3.thrust_wait = 5 as BYTE;
            }
            vdx = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as SIZE;
            vdy = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as SIZE;
            SetVelocityComponents(
                &raw mut (*EPtr).velocity,
                vdx as SDWORD,
                vdy as SDWORD,
            );
        }
        dx = vdx;
        dy = vdy;
        's_751: {
            's_652: {
                '_PartialRevolution: {
                    if group_loc as ::core::ffi::c_int
                        == target_loc as ::core::ffi::c_int
                    {
                        if target_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            if task as ::core::ffi::c_int
                                != C2Rust_Unnamed_9::FLEE.0 as ::core::ffi::c_int
                            {
                                break 's_751;
                            }
                        } else if target_loc as ::core::ffi::c_int
                            == (*GroupPtr).dest_loc as ::core::ffi::c_int
                        {
                            break '_PartialRevolution;
                        } else {
                            break 's_751;
                        }
                    } else if group_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        adjustDeltaVforZoom(radius, &raw mut dx, &raw mut dy);
                        if task as ::core::ffi::c_int
                            == C2Rust_Unnamed_9::ON_STATION.0 as ::core::ffi::c_int
                            && (*GroupPtr).dest_loc as ::core::ffi::c_int != 0
                        {
                            break '_PartialRevolution;
                        } else {
                            if ((dx as ::core::ffi::c_int * dx as ::core::ffi::c_int)
                                as COUNT as ::core::ffi::c_int
                                + (dy as ::core::ffi::c_int * dy as ::core::ffi::c_int)
                                    as COUNT as ::core::ffi::c_int) as ::core::ffi::c_long
                                >= delta_x as ::core::ffi::c_long
                                    * delta_x as ::core::ffi::c_long
                                    + delta_y as ::core::ffi::c_long
                                        * delta_y as ::core::ffi::c_long
                            {
                                Transition = BOOLEAN::TRUE;
                            }
                            break 's_652;
                        }
                    }
                    dest_pt = displayToLocation(
                        MAKE_POINT(
                            (CanvasWidth
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
                                    } << resolutionFactor))) as COORD,
                            (CanvasHeight
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
                                - ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
                        ),
                        ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE,
                    );
                    if !((*GroupPtr).loc.x as ::core::ffi::c_int
                        > -(dest_pt.x as ::core::ffi::c_int)
                        && (*GroupPtr).loc.y as ::core::ffi::c_int
                            > -(dest_pt.y as ::core::ffi::c_int)
                        && (((*GroupPtr).loc.x as ::core::ffi::c_int)
                            < dest_pt.x as ::core::ffi::c_int
                            && ((*GroupPtr).loc.y as ::core::ffi::c_int)
                                < dest_pt.y as ::core::ffi::c_int)) || FilthyCheater.0 != 0
                    {
                        Transition = BOOLEAN::TRUE;
                    }
                    break 's_652;
                }
                if ((dx as ::core::ffi::c_int * dx as ::core::ffi::c_int) as COUNT
                    as ::core::ffi::c_int
                    + (dy as ::core::ffi::c_int * dy as ::core::ffi::c_int) as COUNT
                        as ::core::ffi::c_int) as ::core::ffi::c_long
                    >= delta_x as ::core::ffi::c_long * delta_x as ::core::ffi::c_long
                        + delta_y as ::core::ffi::c_long * delta_y as ::core::ffi::c_long
                {
                    (*GroupPtr).loc = dest_pt;
                    vdx = 0 as SIZE;
                    vdy = 0 as SIZE;
                    memset(
                        &raw mut (*EPtr).velocity as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<VELOCITY_DESC>(),
                    );
                }
                break 's_751;
            }
            if Transition.0 != 0 {
                (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                vdx = 0 as SIZE;
                vdy = 0 as SIZE;
                memset(
                    &raw mut (*EPtr).velocity as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<VELOCITY_DESC>(),
                );
                if group_loc as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    (*GroupPtr).loc = planetOuterLocation(
                        (group_loc as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as COUNT,
                    );
                    group_loc = 0 as BYTE;
                    (*GroupPtr).sys_loc = 0 as BYTE;
                } else if target_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*EPtr).c2rust_unnamed.life_span = 0 as COUNT;
                    (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                        as ELEMENT_FLAGS;
                    (*GroupPtr).in_system = 0 as BYTE;
                    return;
                } else {
                    let mut entryPt: POINT = POINT { x: 0, y: 0 };
                    if target_loc as ::core::ffi::c_int
                        == (*GroupPtr).dest_loc as ::core::ffi::c_int
                    {
                        (*GroupPtr).orbit_pos = (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as BYTE;
                        (*GroupPtr).group_counter = ((TFB_Random() as COUNT
                            as ::core::ffi::c_int % 5 as ::core::ffi::c_int)
                            << 4 as ::core::ffi::c_int) as UWORD;
                    }
                    entryPt.x = (((CanvasWidth - ((64 as SDWORD) << resolutionFactor)
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
                        - (*(&raw mut sinetab as *mut SDWORD)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 2 as ::core::ffi::c_int)
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD as isize,
                            ) as ::core::ffi::c_long
                            * ((CanvasWidth
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
                                    } << resolutionFactor))) * 9 as ::core::ffi::c_int
                                / 16 as ::core::ffi::c_int) as ::core::ffi::c_long
                            >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
                    entryPt.y = (((CanvasHeight
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
                        - (*(&raw mut sinetab as *mut SDWORD)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD as isize,
                            ) as ::core::ffi::c_long
                            * ((CanvasHeight
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
                                * 9 as ::core::ffi::c_int / 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int)
                            as SDWORD) as COORD;
                    (*GroupPtr).loc = displayToLocation(
                        entryPt,
                        ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE,
                    );
                    group_loc = target_loc;
                    (*GroupPtr).sys_loc = target_loc;
                }
            }
        }
        if optShipDirectionIP.0 != 0 {
            if (*GroupPtr).flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
            {
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
                {
                    suggestedFrame = SetAbsFrameIndex(
                        *(*EPtr).next.image.farray.offset(0isize),
                        GetFrameIndex((*EPtr).current.image.frame),
                    );
                } else {
                    suggestedFrame = SetAbsFrameIndex(
                        *(*EPtr).next.image.farray.offset(0isize),
                        (2 as DWORD)
                            .wrapping_add(
                                (ARCTAN(delta_x as SDWORD, delta_y as SDWORD)
                                    as ::core::ffi::c_int
                                    + ((1 as ::core::ffi::c_int)
                                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD,
                            ) as COUNT,
                    );
                }
                if GetFrameIndex((*EPtr).current.image.frame) as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    (*EPtr).next.image.frame = suggestedFrame;
                }
                if isOrbiting.0 != 0 {
                    if (*GroupPtr).flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        (*EPtr).next.image.frame = suggestedFrame;
                        (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                            & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int))
                            as IP_FLAGS;
                    }
                } else {
                    (*EPtr).next.image.frame = suggestedFrame;
                }
            }
            if (*GroupPtr).flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                (*EPtr).next.image.frame = IncFrameIndex((*EPtr).next.image.frame);
            }
            if (*GroupPtr).flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                && (*GroupPtr).dest_loc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*GroupPtr).melee_icon = Ywing;
            }
            if (*GroupPtr).flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                && (*GroupPtr).dest_loc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                (*GroupPtr).melee_icon = Xform;
            }
            if dest_pt.x as ::core::ffi::c_int == (*GroupPtr).loc.x as ::core::ffi::c_int
                && dest_pt.y as ::core::ffi::c_int
                    == (*GroupPtr).loc.y as ::core::ffi::c_int || Transition.0 != 0
            {
                (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as IP_FLAGS;
            }
        }
    } else if task as ::core::ffi::c_int
        >= C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int
        && optShipDirectionIP.0 != 0
    {
        if (*GroupPtr).flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            if GetFrameIndex((*EPtr).current.image.frame) as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                let mut delta_x_0: SIZE = 0;
                let mut delta_y_0: SIZE = 0;
                let mut sis_pt: POINT = POINT { x: 0, y: 0 };
                sis_pt = displayToLocation(GlobData.Game_state.ShipStamp.origin, radius);
                delta_x_0 = (sis_pt.x as ::core::ffi::c_int
                    - (*GroupPtr).loc.x as ::core::ffi::c_int) as SIZE;
                delta_y_0 = (sis_pt.y as ::core::ffi::c_int
                    - (*GroupPtr).loc.y as ::core::ffi::c_int) as SIZE;
                (*EPtr).next.image.frame = SetAbsFrameIndex(
                    *(*EPtr).next.image.farray.offset(0isize),
                    (2 as DWORD)
                        .wrapping_add(
                            (ARCTAN(delta_x_0 as SDWORD, delta_y_0 as SDWORD)
                                as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD,
                        ) as COUNT,
                );
                (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as IP_FLAGS;
            }
        } else if (*GroupPtr).flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*EPtr).next.image.frame = IncFrameIndex((*EPtr).next.image.frame);
        }
    }
    radius = zoomRadiusForLocation(group_loc);
    adjustDeltaVforZoom(radius, &raw mut vdx, &raw mut vdy);
    (*GroupPtr).loc.x = ((*GroupPtr).loc.x as ::core::ffi::c_int
        + vdx as ::core::ffi::c_int) as COORD;
    (*GroupPtr).loc.y = ((*GroupPtr).loc.y as ::core::ffi::c_int
        + vdy as ::core::ffi::c_int) as COORD;
    dest_pt = locationToDisplay((*GroupPtr).loc, radius);
    (*EPtr).next.location.x = (((dest_pt.x as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    (*EPtr).next.location.y = (((dest_pt.y as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    if group_loc as ::core::ffi::c_int != flagship_loc as ::core::ffi::c_int
        || task as ::core::ffi::c_int
            & C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int != 0
            && (*GroupPtr).group_counter as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                != 0
    {
        DisplayArray[(*EPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
            as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    } else {
        DisplayArray[(*EPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM.0
            as ::core::ffi::c_int as GRAPHICS_PRIM;
        if task as ::core::ffi::c_int
            & C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int != 0
        {
            (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    }
    (*EPtr).state_flags = ((*EPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn flag_ship_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
    {
        (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
    } else {
        (*ElementPtr1).state_flags = ((*ElementPtr1).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
    };
}
unsafe extern "C" fn ip_group_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
    let mut OtherPtr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    GroupPtr = (*ElementPtr0).pParent as *mut IP_GROUP;
    OtherPtr = (*ElementPtr1).pParent;
    if !(GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int))
        && CheckAlliance(RACE_ID((*GroupPtr).race_id as ::core::ffi::c_uint))
            as ::core::ffi::c_int == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
    {
        return;
    }
    if !OtherPtr.is_null() {
        if (*ElementPtr0).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
            || (*ElementPtr1).current.location.x as ::core::ffi::c_int
                == (*ElementPtr1).next.location.x as ::core::ffi::c_int
                && (*ElementPtr1).current.location.y as ::core::ffi::c_int
                    == (*ElementPtr1).next.location.y as ::core::ffi::c_int
        {
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        } else {
            (*ElementPtr1).state_flags = ((*ElementPtr1).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*GroupPtr).loc = DisplayArray[(*ElementPtr0).PrimIndex as usize]
                .Object
                .Point;
            (*ElementPtr0).next.location = (*ElementPtr0).current.location;
            (*ElementPtr0).IntersectControl.EndPoint.x = ((*ElementPtr0).next.location.x
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr0).IntersectControl.EndPoint.y = ((*ElementPtr0).next.location.y
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        }
    } else {
        EncounterGroup = (*GroupPtr).group_id;
        (*GroupPtr).task = ((*GroupPtr).task as ::core::ffi::c_int
            | C2Rust_Unnamed_9::REFORM_GROUP.0 as ::core::ffi::c_int) as BYTE;
        (*GroupPtr).group_counter = 100 as UWORD;
        NotifyOthers((*GroupPtr).race_id as COUNT, -1 as ::core::ffi::c_int as BYTE);
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
        {
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        } else {
            (*ElementPtr1).state_flags = ((*ElementPtr1).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        }
    };
}
unsafe extern "C" fn spawn_ip_group(mut GroupPtr: *mut IP_GROUP) {
    let mut hIPSHIPElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hIPSHIPElement = AllocElement();
    if !hIPSHIPElement.is_null() {
        let mut IPSHIPElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        IPSHIPElementPtr = LockLink(&raw mut disp_q, hIPSHIPElement) as *mut ELEMENT;
        (*IPSHIPElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
        (*IPSHIPElementPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
        (*IPSHIPElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        DisplayArray[(*IPSHIPElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*IPSHIPElementPtr).current.image.farray = &raw mut (*GroupPtr).melee_icon;
        (*IPSHIPElementPtr).current.image.frame = SetAbsFrameIndex(
            (*GroupPtr).melee_icon,
            1 as COUNT,
        );
        (*GroupPtr).flags = 0 as IP_FLAGS;
        if (*GroupPtr).race_id as ::core::ffi::c_int
            != RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
        {
            (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as IP_FLAGS;
        }
        if (*GroupPtr).race_id as ::core::ffi::c_int
            == RACE_ID::MMRNMHRM_SHIP.0 as ::core::ffi::c_int
        {
            (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as IP_FLAGS;
        }
        if (*GroupPtr).race_id as ::core::ffi::c_int
            == RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
        {
            (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as IP_FLAGS;
        }
        if optShipSeed.0 != 0 {
            let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
            let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
            hFleet = GetStarShipFromIndex(
                &raw mut GlobData.Game_state.avail_race_q,
                (*GroupPtr).race_id as COUNT,
            ) as HFLEETINFO;
            if !hFleet.is_null() {
                TemplatePtr = LockFleetInfo(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hFleet,
                );
            }
            if !TemplatePtr.is_null() {
                let mut ship: SPECIES_ID = SeedShip(
                    (*TemplatePtr).SpeciesID,
                    BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
                );
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
                (*GroupPtr).flags = 0 as IP_FLAGS;
                match ship {
                    SPECIES_ID::SLYLANDRO_ID => {
                        (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as IP_FLAGS;
                    }
                    SPECIES_ID::MMRNMHRM_ID => {
                        (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
                            as IP_FLAGS;
                    }
                    SPECIES_ID::ORZ_ID
                    | SPECIES_ID::UTWIG_ID
                    | SPECIES_ID::YEHAT_ID
                    | SPECIES_ID::MYCON_ID
                    | SPECIES_ID::UR_QUAN_ID
                    | SPECIES_ID::KOHR_AH_ID
                    | SPECIES_ID::ARILOU_ID
                    | SPECIES_ID::PKUNK_ID
                    | SPECIES_ID::SPATHI_ID
                    | SPECIES_ID::SUPOX_ID
                    | SPECIES_ID::MELNORME_ID
                    | SPECIES_ID::DRUUGE_ID
                    | SPECIES_ID::SHOFIXTI_ID
                    | SPECIES_ID::THRADDASH_ID
                    | SPECIES_ID::VUX_ID
                    | SPECIES_ID::ILWRATH_ID
                    | SPECIES_ID::UMGAH_ID
                    | SPECIES_ID::ZOQFOTPIK_ID
                    | SPECIES_ID::CHMMR_ID
                    | SPECIES_ID::CHENJESU_ID
                    | SPECIES_ID::ANDROSYNTH_ID
                    | SPECIES_ID::EARTHLING_ID
                    | SPECIES_ID::SYREEN_ID => {
                        (*GroupPtr).flags = ((*GroupPtr).flags as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            as IP_FLAGS;
                    }
                    _ => {}
                }
            }
        }
        if optShipDirectionIP.0 != 0
            && (*GroupPtr).flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*GroupPtr).melee_icon = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicInstance(
                    b"ship.slylandro.graphics.probe.small\0".as_ptr() as RESOURCE,
                ), ()) as DRAWABLE,
            );
            (*IPSHIPElementPtr).current.image.frame = SetAbsFrameIndex(
                (*GroupPtr).melee_icon,
                TFB_Random()
                    .wrapping_rem(
                        (15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as DWORD,
                    )
                    .wrapping_add(1 as DWORD) as COUNT,
            );
        }
        (*IPSHIPElementPtr).death_func = Some(
            ip_group_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*IPSHIPElementPtr).collision_func = Some(
            ip_group_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        let mut radius: SIZE = 0;
        let mut pt: POINT = POINT { x: 0, y: 0 };
        radius = zoomRadiusForLocation((*GroupPtr).sys_loc);
        pt = locationToDisplay((*GroupPtr).loc, radius);
        (*IPSHIPElementPtr).current.location.x = (((pt.x as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int)
            + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
            - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
        (*IPSHIPElementPtr).current.location.y = (((pt.y as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int)
            + (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
            - (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
        (*IPSHIPElementPtr).pParent = GroupPtr as *mut ::core::ffi::c_void;
        SetUpElement(IPSHIPElementPtr);
        (*IPSHIPElementPtr).IntersectControl.IntersectStamp.frame = DecFrameIndex(
            stars_in_space,
        );
        UnlockLink(&raw mut disp_q, hIPSHIPElement);
        PutQueue(&raw mut disp_q, hIPSHIPElement);
    }
}
unsafe extern "C" fn flag_ship_preprocess(mut ElementPtr: *mut ELEMENT) {
    (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
        .c2rust_unnamed_3
        .thrust_wait
        .wrapping_sub(1);
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        let mut hSuccElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hSuccElement = (*ElementPtr).succ;
        if !hSuccElement.is_null() && hSuccElement != disp_q.tail {
            let mut hPredElement: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut TailPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            TailPtr = LockLink(&raw mut disp_q, disp_q.tail) as *mut ELEMENT;
            hPredElement = (*TailPtr).pred;
            UnlockLink(&raw mut disp_q, disp_q.tail);
            RemoveElement(hSuccElement);
            PutQueue(&raw mut disp_q, hSuccElement);
        }
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 42 as BYTE;
    }
    let mut flagship_loc: BYTE = 0;
    let mut ec: BYTE = 0;
    let mut vdx: SIZE = 0;
    let mut vdy: SIZE = 0;
    let mut radius: SIZE = 0;
    let mut pt: POINT = POINT { x: 0, y: 0 };
    GetCurrentVelocityComponents(
        &raw mut GlobData.Game_state.velocity,
        &raw mut vdx,
        &raw mut vdy,
    );
    if legacySave.0 == 0 {
        vdx = (vdx as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
        vdy = (vdy as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    } else if CurrentInputState
        .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
        as usize][C2Rust_Unnamed_5::KEY_UP.0 as ::core::ffi::c_int as usize] != 0
        || CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_5::KEY_THRUST.0 as ::core::ffi::c_int as usize] != 0
        || vdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && vdy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        legacySave = BOOLEAN::FALSE;
    }
    flagship_loc = getFlagshipLocation();
    radius = zoomRadiusForLocation(flagship_loc);
    adjustDeltaVforZoom(radius, &raw mut vdx, &raw mut vdy);
    pt = locationToDisplay(GlobData.Game_state.ip_location, radius);
    (*ElementPtr).current.location.x = (((pt.x as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    (*ElementPtr).current.location.y = (((pt.y as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
        .current
        .location
        .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
        .current
        .location
        .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.ip_location.x = (GlobData.Game_state.ip_location.x
        as ::core::ffi::c_int + vdx as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.ip_location.y = (GlobData.Game_state.ip_location.y
        as ::core::ffi::c_int + vdy as ::core::ffi::c_int) as COORD;
    pt = locationToDisplay(GlobData.Game_state.ip_location, radius);
    (*ElementPtr).next.location.x = (((pt.x as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    (*ElementPtr).next.location.y = (((pt.y as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int)
        + (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
            as ::core::ffi::c_int
        - (((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    GlobData.Game_state.ShipStamp.origin = pt;
    (*ElementPtr).next.image.frame = GlobData.Game_state.ShipStamp.frame;
    if (*ElementPtr).c2rust_unnamed_2.sys_loc as ::core::ffi::c_int
        == flagship_loc as ::core::ffi::c_int
    {
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
        {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        }
    } else {
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.sys_loc = flagship_loc;
    }
    ec = uqm_lua_checked!(getGameStateUint(b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char), ())
        as BYTE;
    if ec as ::core::ffi::c_int != 0
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int == 0
    {
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        ec = ec.wrapping_sub(1);
        uqm_lua_checked!(setGameStateUint(
            b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            ec as DWORD,
        ), ());
    }
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn AdjustInitialPosition() {
    let mut flagship_loc: BYTE = 0;
    let mut radius: SIZE = 0;
    let mut pt: POINT = POINT { x: 0, y: 0 };
    flagship_loc = getFlagshipLocation();
    radius = zoomRadiusForLocation(flagship_loc);
    pt = locationToDisplay(GlobData.Game_state.ip_location, radius);
    if LastActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_8::CHECK_LOAD.0 as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(InitialIntersect(), ());
    }
    GlobData.Game_state.ShipStamp.origin = pt;
}
unsafe extern "C" fn spawn_flag_ship() {
    let mut hFlagShipElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hFlagShipElement = AllocElement();
    if !hFlagShipElement.is_null() {
        let mut FlagShipElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        FlagShipElementPtr = LockLink(&raw mut disp_q, hFlagShipElement) as *mut ELEMENT;
        (*FlagShipElementPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
        (*FlagShipElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
        (*FlagShipElementPtr).c2rust_unnamed_2.sys_loc = getFlagshipLocation();
        (*FlagShipElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        if uqm_lua_checked!(getGameStateUint(b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            (*FlagShipElementPtr).state_flags = ((*FlagShipElementPtr).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
        (*FlagShipElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        (*FlagShipElementPtr).c2rust_unnamed_3.thrust_wait = 42 as BYTE;
        DisplayArray[(*FlagShipElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*FlagShipElementPtr).current.image.farray = &raw mut GlobData
            .Game_state
            .ShipStamp
            .frame;
        (*FlagShipElementPtr).current.image.frame = GlobData.Game_state.ShipStamp.frame;
        (*FlagShipElementPtr).preprocess_func = Some(
            flag_ship_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*FlagShipElementPtr).collision_func = Some(
            flag_ship_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        uqm_lua_checked!(AdjustInitialPosition(), ());
        (*FlagShipElementPtr).current.location.x = (((GlobData
            .Game_state
            .ShipStamp
            .origin
            .x as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
            - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
        (*FlagShipElementPtr).current.location.y = (((GlobData
            .Game_state
            .ShipStamp
            .origin
            .y as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            + (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
            - (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
        UnlockLink(&raw mut disp_q, hFlagShipElement);
        PutQueue(&raw mut disp_q, hFlagShipElement);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoMissions() {
    let mut hGroup: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextGroup: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    uqm_lua_checked!(spawn_flag_ship(), ());
    if EncounterRace as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        NotifyOthers(EncounterRace as COUNT, 0 as BYTE);
        EncounterRace = -1 as SIZE;
    }
    hGroup = GlobData.Game_state.ip_group_q.head as HSHIPFRAG;
    while !hGroup.is_null() {
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        GroupPtr = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        hNextGroup = (*GroupPtr).succ as HSHIPFRAG;
        if (*GroupPtr).in_system as ::core::ffi::c_int != 0
            && CheckAlliance(RACE_ID((*GroupPtr).race_id as ::core::ffi::c_uint))
                as ::core::ffi::c_int
                != C2Rust_Unnamed_6::DEAD_GUY.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(spawn_ip_group(GroupPtr), ());
        }
        UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        hGroup = hNextGroup;
    }
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
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
