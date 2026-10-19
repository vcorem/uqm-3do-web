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

// Rust translation of src/uqm/planets/generate/genvault.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type RandomContext;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn ReinitQueue(pq: *mut QUEUE);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut optSeedType: ::core::ffi::c_int;
    fn RepairSISBorder();
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn RandomContext_GetSeed(context: *mut RandomContext) -> DWORD;
    static mut SysGenRNG: *mut RandomContext;
    fn matchWorld(
        solarSys: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        planetI: BYTE,
        moonI: BYTE,
    ) -> bool;
    fn LoadStdLanderFont(info: *mut PLANET_INFO);
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
    fn GenerateDefault_generateArtifact(
        _: *const SOLARSYS_STATE,
        whichNode: COUNT,
        info: *mut NODE_INFO,
    ) -> COUNT;
    fn GenerateDefault_landerReport(_: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_landerReportCycle(_: *mut SOLARSYS_STATE) -> bool;
    fn GenerateWorlds(whichType: BYTE) -> BYTE;
    fn PlanetByteGen(pPDesc: *mut PLANET_DESC) -> BYTE;
    fn GetGroupInfo(offset: DWORD, which_group: BYTE) -> BOOLEAN;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn SetLanderTakeoff();
    fn InitCommunication(which_comm: CONVERSATION) -> COUNT;
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type HSHIPFRAG = HLINK;
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
#[no_mangle]
pub static mut generateVaultFunctions: GenerateFunctions = GenerateFunctions {
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
        GenerateVault_generatePlanets
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
        GenerateVault_generateOrbital
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
        GenerateVault_generateEnergy
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
        GenerateVault_pickupEnergy
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
    pickupLife: Some(
        GenerateDefault_pickupLife
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
};
unsafe extern "C" fn GenerateVault_generatePlanets(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    let mut pSunDesc: *mut PLANET_DESC = (&raw mut (*solarSys).SunDesc
        as *mut PLANET_DESC)
        .offset(0isize);
    let mut pPlanet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    GenerateDefault_generatePlanets(solarSys);
    (*pSunDesc).PlanetByte = 0 as BYTE;
    (*pSunDesc).MoonByte = 0 as BYTE;
    pPlanet = (&raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC)
        .offset((*pSunDesc).PlanetByte as isize);
    if optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int {
        (*pSunDesc).PlanetByte = PlanetByteGen(pSunDesc);
        pPlanet = (&raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC)
            .offset((*pSunDesc).PlanetByte as isize);
        (*pPlanet).NumPlanets = (if RandomContext_GetSeed(SysGenRNG)
            .wrapping_rem(5 as DWORD) == 0 as DWORD
        {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as BYTE;
        (*pSunDesc).MoonByte = PlanetByteGen(pPlanet);
    }
    if !(optSeedType == C2Rust_Unnamed::OPTVAL_PRIME.0 as ::core::ffi::c_int) {
        (*pPlanet).data_index = GenerateWorlds(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
        );
        if (*pPlanet).NumPlanets == 0 {
            (*pPlanet).NumPlanets = (*pPlanet).NumPlanets.wrapping_add(1);
        }
    }
    return true;
}
unsafe extern "C" fn GenerateVault_generateOrbital(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
) -> bool {
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -3 as ::core::ffi::c_int as BYTE,
    ) {
        uqm_lua_checked!(LoadStdLanderFont(&raw mut (*solarSys).SysInfo.PlanetInfo), false);
        (*solarSys).PlanetSideFrame[1usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.syreenvault\0".as_ptr() as RESOURCE), false)
                as DRAWABLE,
        );
        (*solarSys).SysInfo.PlanetInfo.DiscoveryString = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(b"text.syreenvault\0".as_ptr() as RESOURCE), false),
        );
        if uqm_lua_checked!(getGameStateUint(
            b"SHIP_VAULT_UNLOCKED\0".as_ptr() as *const ::core::ffi::c_char,
        ), false) != 0
        {
            (*solarSys).SysInfo.PlanetInfo.DiscoveryString = SetAbsStringTableIndex(
                (*solarSys).SysInfo.PlanetInfo.DiscoveryString,
                2 as COUNT,
            );
        } else if uqm_lua_checked!(getGameStateUint(
            b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), false) != 0
        {
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
                ), false)
                    & ((1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_1::URQUAN_ENCOUNTER.0 as ::core::ffi::c_int)
                        as DWORD == 0
                && uqm_lua_checked!(getGameStateUint(
                    b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
                ), false) == 0
            {
                let mut i: COUNT = 0;
                let mut Survivors: BOOLEAN = BOOLEAN::FALSE;
                let mut state: UWORD = 0;
                PutGroupInfo(
                    0 as ::core::ffi::c_long as DWORD,
                    !(0 as ::core::ffi::c_int) as BYTE,
                );
                ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
                (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q)
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || {
                        __assert_fail(
                            b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"src/uqm/planets/generate/genvault.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            120 as ::core::ffi::c_int,
                            b"GenerateVault_generateOrbital\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        0 as ::core::ffi::c_int != 0
                    }) as ::core::ffi::c_int;
                i = 0 as COUNT;
                while (i as ::core::ffi::c_int) < 6 as ::core::ffi::c_int {
                    CloneShipFragment(
                        RACE_ID::URQUAN_SHIP,
                        &raw mut GlobData.Game_state.npc_built_ship_q,
                        0 as COUNT,
                    );
                    i = i.wrapping_add(1);
                }
                uqm_lua_checked!(setGameStateUint(
                    b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                    ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD,
                ), false);
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    | C2Rust_Unnamed_0::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                    as ACTIVITY;
                uqm_lua_checked!(InitCommunication(CONVERSATION::URQUAN_CONVERSATION), false);
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                        | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
                {
                    return true;
                }
                Survivors = BOOLEAN(
                    !GlobData.Game_state.npc_built_ship_q.head.is_null()
                        as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_0::START_INTERPLANETARY.0 as ::core::ffi::c_int))
                    as ACTIVITY;
                ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
                GetGroupInfo(
                    0 as ::core::ffi::c_long as DWORD,
                    0 as ::core::ffi::c_int as BYTE,
                );
                if Survivors.0 != 0 {
                    return true;
                }
                state = uqm_lua_checked!(getGameStateUint(
                    b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
                ), false) as UWORD;
                state = (state as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_1::URQUAN_ENCOUNTER.0 as ::core::ffi::c_int)
                    as UWORD;
                uqm_lua_checked!(setGameStateUint(
                    b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
                    state as DWORD,
                ), false);
                uqm_lua_checked!(RepairSISBorder(), false);
            }
            (*solarSys).SysInfo.PlanetInfo.DiscoveryString = SetAbsStringTableIndex(
                (*solarSys).SysInfo.PlanetInfo.DiscoveryString,
                1 as COUNT,
            );
        }
    }
    uqm_lua_checked!(GenerateDefault_generateOrbital(solarSys, world), false);
    return true;
}
unsafe extern "C" fn GenerateVault_generateEnergy(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -3 as ::core::ffi::c_int as BYTE,
    ) {
        return GenerateDefault_generateArtifact(solarSys, whichNode, info);
    }
    return 0 as COUNT;
}
unsafe extern "C" fn GenerateVault_pickupEnergy(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut whichNode: COUNT,
) -> bool {
    if matchWorld(
        solarSys,
        world,
        -2 as ::core::ffi::c_int as BYTE,
        -3 as ::core::ffi::c_int as BYTE,
    ) {
        (whichNode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"whichNode == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/generate/genvault.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int,
                    b"GenerateVault_pickupEnergy\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if uqm_lua_checked!(getGameStateUint(
            b"SHIP_VAULT_UNLOCKED\0".as_ptr() as *const ::core::ffi::c_char,
        ), false) != 0
        {
            uqm_lua_checked!(GenerateDefault_landerReportCycle(solarSys), false);
        } else if uqm_lua_checked!(getGameStateUint(
            b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), false) != 0
        {
            uqm_lua_checked!(GenerateDefault_landerReportCycle(solarSys), false);
            SetLanderTakeoff();
            uqm_lua_checked!(setGameStateUint(
                b"SHIP_VAULT_UNLOCKED\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), false);
            uqm_lua_checked!(setGameStateUint(
                b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), false);
            uqm_lua_checked!(setGameStateUint(
                b"SYREEN_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), false);
        } else {
            uqm_lua_checked!(GenerateDefault_landerReport(solarSys), false);
            if uqm_lua_checked!(getGameStateUint(
                b"KNOW_SYREEN_VAULT\0".as_ptr() as *const ::core::ffi::c_char,
            ), false) == 0
            {
                uqm_lua_checked!(setGameStateUint(
                    b"KNOW_SYREEN_VAULT\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), false);
            }
        }
        return false;
    }
    return false;
}
