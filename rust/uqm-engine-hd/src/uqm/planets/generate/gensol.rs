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

// Rust translation of src/uqm/planets/generate/gensol.c.
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
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optHeadStart: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optScanSphere: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut sinetab: [SDWORD; 0];
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn RandomContext_GetSeed(context: *mut RandomContext) -> DWORD;
    static mut SysGenRNG: *mut RandomContext;
    fn worldIsPlanet(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> bool;
    fn planetIndex(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> COUNT;
    fn moonIndex(solarSys: *const SOLARSYS_STATE, moon: *const PLANET_DESC) -> COUNT;
    fn matchWorld(
        solarSys: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        planetI: BYTE,
        moonI: BYTE,
    ) -> bool;
    fn LoadPlanet(SurfDefFrame: FRAME);
    fn LoadStdLanderFont(info: *mut PLANET_INFO);
    fn FreeLanderFont(info: *mut PLANET_INFO);
    fn GenerateMineralDeposits(
        _: *const SYSTEM_INFO,
        whichDeposit: COUNT,
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
    fn CustomMineralDeposits(
        SysInfoPtr: *const SYSTEM_INFO,
        which_deposit: COUNT,
        info: *mut NODE_INFO,
        numNodes: COUNT,
        r#type: COUNT,
        quality: BYTE,
    ) -> COUNT;
    fn DoPlanetaryAnalysis(SysInfoPtr: *mut SYSTEM_INFO, pPlanetDesc: *mut PLANET_DESC);
    fn CalcGravity(_: *const PLANET_INFO) -> UWORD;
    fn GenerateDefault_initNpcs(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_reinitNpcs(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_uninitNpcs(solarSys: *mut SOLARSYS_STATE) -> bool;
    fn GenerateDefault_generateMoons(
        solarSys: *mut SOLARSYS_STATE,
        planet: *mut PLANET_DESC,
    ) -> bool;
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
    fn GenerateDefault_landerReport(_: *mut SOLARSYS_STATE) -> bool;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    static mut EncounterGroup: BYTE;
    fn DoDiscoveryReport(ReadOutSounds: SOUND);
    fn SetLanderTakeoff();
    fn KillLanderCrewSeq(numKilled: COUNT, period: DWORD) -> bool;
    static mut GameStrings: STRING;
    static mut MenuSounds: SOUND;
    static mut SpaceMusicOK: BOOLEAN;
    fn GetGroupInfo(offset: DWORD, which_group: BYTE) -> BOOLEAN;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
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
pub type SOUND = STRING;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const HYDROGEN: Self = Self(0);
    pub const HELIUM: Self = Self(1);
    pub const LITHIUM: Self = Self(2);
    pub const BERYLLIUM: Self = Self(3);
    pub const BORON: Self = Self(4);
    pub const CARBON: Self = Self(5);
    pub const NITROGEN: Self = Self(6);
    pub const OXYGEN: Self = Self(7);
    pub const FLUORINE: Self = Self(8);
    pub const NEON: Self = Self(9);
    pub const SODIUM: Self = Self(10);
    pub const MAGNESIUM: Self = Self(11);
    pub const ALUMINUM: Self = Self(12);
    pub const SILICON: Self = Self(13);
    pub const PHOSPHORUS: Self = Self(14);
    pub const SULFUR: Self = Self(15);
    pub const CHLORINE: Self = Self(16);
    pub const ARGON: Self = Self(17);
    pub const POTASSIUM: Self = Self(18);
    pub const CALCIUM: Self = Self(19);
    pub const SCANDIUM: Self = Self(20);
    pub const TITANIUM: Self = Self(21);
    pub const VANADIUM: Self = Self(22);
    pub const CHROMIUM: Self = Self(23);
    pub const MANGANESE: Self = Self(24);
    pub const IRON: Self = Self(25);
    pub const COBALT: Self = Self(26);
    pub const NICKEL: Self = Self(27);
    pub const COPPER: Self = Self(28);
    pub const ZINC: Self = Self(29);
    pub const GALLIUM: Self = Self(30);
    pub const GERMANIUM: Self = Self(31);
    pub const ARSENIC: Self = Self(32);
    pub const SELENIUM: Self = Self(33);
    pub const BROMINE: Self = Self(34);
    pub const KRYPTON: Self = Self(35);
    pub const RUBIDIUM: Self = Self(36);
    pub const STRONTIUM: Self = Self(37);
    pub const YTTRIUM: Self = Self(38);
    pub const ZIRCONIUM: Self = Self(39);
    pub const NIOBIUM: Self = Self(40);
    pub const MOLYBDENUM: Self = Self(41);
    pub const TECHNETIUM: Self = Self(42);
    pub const RUTHENIUM: Self = Self(43);
    pub const RHODIUM: Self = Self(44);
    pub const PALLADIUM: Self = Self(45);
    pub const SILVER: Self = Self(46);
    pub const CADMIUM: Self = Self(47);
    pub const INDIUM: Self = Self(48);
    pub const TIN: Self = Self(49);
    pub const ANTIMONY: Self = Self(50);
    pub const TELLURIUM: Self = Self(51);
    pub const IODINE: Self = Self(52);
    pub const XENON: Self = Self(53);
    pub const CESIUM: Self = Self(54);
    pub const BARIUM: Self = Self(55);
    pub const LANTHANUM: Self = Self(56);
    pub const CERIUM: Self = Self(57);
    pub const PRASEODYMIUM: Self = Self(58);
    pub const NEODYMIUM: Self = Self(59);
    pub const PROMETHIUM: Self = Self(60);
    pub const SAMARIUM: Self = Self(61);
    pub const EUROPIUM: Self = Self(62);
    pub const GADOLINIUM: Self = Self(63);
    pub const TERBIUM: Self = Self(64);
    pub const DYPROSIUM: Self = Self(65);
    pub const HOLMIUM: Self = Self(66);
    pub const ERBIUM: Self = Self(67);
    pub const THULIUM: Self = Self(68);
    pub const YTTERBIUM: Self = Self(69);
    pub const LUTETIUM: Self = Self(70);
    pub const HAFNIUM: Self = Self(71);
    pub const TANTALUM: Self = Self(72);
    pub const TUNGSTEN: Self = Self(73);
    pub const RHENIUM: Self = Self(74);
    pub const OSMIUM: Self = Self(75);
    pub const IRIDIUM: Self = Self(76);
    pub const PLATINUM: Self = Self(77);
    pub const GOLD: Self = Self(78);
    pub const MERCURY: Self = Self(79);
    pub const THALLIUM: Self = Self(80);
    pub const LEAD: Self = Self(81);
    pub const BISMUTH: Self = Self(82);
    pub const POLONIUM: Self = Self(83);
    pub const ASTATINE: Self = Self(84);
    pub const RADON: Self = Self(85);
    pub const FRANCIUM: Self = Self(86);
    pub const RADIUM: Self = Self(87);
    pub const ACTINIUM: Self = Self(88);
    pub const THORIUM: Self = Self(89);
    pub const PROTACTINIUM: Self = Self(90);
    pub const URANIUM: Self = Self(91);
    pub const NEPTUNIUM: Self = Self(92);
    pub const PLUTONIUM: Self = Self(93);
    pub const NUMBER_OF_NORMAL: Self = Self(94);
    pub const OZONE: Self = Self(94);
    pub const FREE_RADICALS: Self = Self(95);
    pub const CARBON_DIOXIDE: Self = Self(96);
    pub const CARBON_MONOXIDE: Self = Self(97);
    pub const AMMONIA: Self = Self(98);
    pub const METHANE: Self = Self(99);
    pub const SULFURIC_ACID: Self = Self(100);
    pub const HYDROCHLORIC_ACID: Self = Self(101);
    pub const HYDROCYANIC_ACID: Self = Self(102);
    pub const FORMIC_ACID: Self = Self(103);
    pub const PHOSPHORIC_ACID: Self = Self(104);
    pub const FORMALDEHYDE: Self = Self(105);
    pub const CYANOACETYLENE: Self = Self(106);
    pub const METHANOL: Self = Self(107);
    pub const ETHANOL: Self = Self(108);
    pub const SILICON_MONOXIDE: Self = Self(109);
    pub const TITANIUM_OXIDE: Self = Self(110);
    pub const ZIRCONIUM_OXIDE: Self = Self(111);
    pub const WATER: Self = Self(112);
    pub const SILICON_COMPOUNDS: Self = Self(113);
    pub const METAL_OXIDES: Self = Self(114);
    pub const QUANTUM_BH: Self = Self(115);
    pub const NEUTRONIUM: Self = Self(116);
    pub const MAGNETIC_MONOPOLES: Self = Self(117);
    pub const DEGENERATE_MATTER: Self = Self(118);
    pub const RT_SUPER_FLUID: Self = Self(119);
    pub const AGUUTI_NODULES: Self = Self(120);
    pub const IRON_COMPOUNDS: Self = Self(121);
    pub const ALUMINUM_COMPOUNDS: Self = Self(122);
    pub const NITROUS_OXIDE: Self = Self(123);
    pub const RADIOACTIVE_COMPOUNDS: Self = Self(124);
    pub const HYDROCARBONS: Self = Self(125);
    pub const CARBON_COMPOUNDS: Self = Self(126);
    pub const ANTIMATTER: Self = Self(127);
    pub const CHARON_DUST: Self = Self(128);
    pub const REISBURG_HELICES: Self = Self(129);
    pub const TZO_CRYSTALS: Self = Self(130);
    pub const CALCIUM_COMPOUNDS: Self = Self(131);
    pub const NITRIC_ACID: Self = Self(132);
    pub const NUMBER_OF_ELEMENTS: Self = Self(133);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const LIGHT: Self = Self(0);
    pub const MEDIUM: Self = Self(1);
    pub const HEAVY: Self = Self(2);
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub type HIPGROUP = HLINK;
pub type IP_FLAGS = BYTE;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
#[no_mangle]
pub static mut generateSolFunctions: GenerateFunctions = GenerateFunctions {
    initNpcs: Some(
        GenerateSol_initNpcs as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    reinitNpcs: Some(
        GenerateSol_reinitNpcs as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    uninitNpcs: Some(
        GenerateDefault_uninitNpcs as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    generatePlanets: Some(
        GenerateSol_generatePlanets as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
    ),
    generateMoons: Some(
        GenerateSol_generateMoons
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
    ),
    generateName: Some(
        GenerateSol_generateName
            as unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
    ),
    generateOrbital: Some(
        GenerateSol_generateOrbital
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
    ),
    generateMinerals: Some(
        GenerateSol_generateMinerals
            as unsafe extern "C" fn(
                *const SOLARSYS_STATE,
                *const PLANET_DESC,
                COUNT,
                *mut NODE_INFO,
            ) -> COUNT,
    ),
    generateEnergy: Some(
        GenerateSol_generateEnergy
            as unsafe extern "C" fn(
                *const SOLARSYS_STATE,
                *const PLANET_DESC,
                COUNT,
                *mut NODE_INFO,
            ) -> COUNT,
    ),
    generateLife: Some(
        GenerateSol_generateLife
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
        GenerateSol_pickupEnergy
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
    pickupLife: Some(
        GenerateDefault_pickupLife
            as unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
    ),
};
unsafe extern "C" fn GenerateSol_initNpcs(mut solarSys: *mut SOLARSYS_STATE) -> bool {
    if optHeadStart.0 != 0 {
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_MESSAGE_DELIVERED\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), false);
    }
    GlobData.Game_state.BattleGroupRef = uqm_lua_checked!(getGameStateUint(
        b"URQUAN_PROBE_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if GlobData.Game_state.BattleGroupRef == 0 as DWORD
        && uqm_lua_checked!(getGameStateUint(
            b"PROBE_MESSAGE_DELIVERED\0".as_ptr() as *const ::core::ffi::c_char,
        ), false) == 0
    {
        CloneShipFragment(
            RACE_ID::URQUAN_DRONE_SHIP,
            &raw mut GlobData.Game_state.npc_built_ship_q,
            0 as COUNT,
        );
        GlobData.Game_state.BattleGroupRef = PutGroupInfo(
            !(0 as ::core::ffi::c_long) as DWORD,
            1 as BYTE,
        );
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        uqm_lua_checked!(setGameStateUint(
            b"URQUAN_PROBE_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.Game_state.BattleGroupRef,
        ), false);
    }
    if uqm_lua_checked!(init_probe(), false) == 0 {
        uqm_lua_checked!(GenerateDefault_initNpcs(solarSys), false);
    } else if SpaceMusicOK.0 != 0 {
        uqm_lua_checked!(findRaceSOI(), false);
    }
    return true;
}
unsafe extern "C" fn GenerateSol_reinitNpcs(mut solarSys: *mut SOLARSYS_STATE) -> bool {
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), false)
        != 3 as DWORD
    {
        uqm_lua_checked!(GenerateDefault_reinitNpcs(solarSys), false);
        check_probe();
    } else {
        GlobData.Game_state.BattleGroupRef = 0 as DWORD;
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/planets/generate/gensol.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    111 as ::core::ffi::c_int,
                    b"GenerateSol_reinitNpcs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
    return true;
}
unsafe extern "C" fn GenerateSol_generatePlanets(
    mut solarSys: *mut SOLARSYS_STATE,
) -> bool {
    let mut planetI: COUNT = 0;
    RandomContext_SeedRandom(
        SysGenRNG,
        (334241042 as ::core::ffi::c_long
            + (if optSeedType > C2Rust_Unnamed_1::OPTVAL_PLANET.0 as ::core::ffi::c_int {
                optCustomSeed
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_long) as DWORD,
    );
    (*solarSys).SunDesc[0usize].NumPlanets = 9 as BYTE;
    planetI = 0 as COUNT;
    while (planetI as ::core::ffi::c_int) < 9 as ::core::ffi::c_int {
        let mut rand_val: DWORD = 0;
        let mut word_val: UWORD = 0;
        let mut pCurDesc: *mut PLANET_DESC = (&raw mut (*solarSys).PlanetDesc
            as *mut PLANET_DESC)
            .offset(planetI as isize);
        (*pCurDesc).rand_seed = RandomContext_Random(SysGenRNG);
        rand_val = (*pCurDesc).rand_seed;
        word_val = rand_val as UWORD;
        (*pCurDesc).angle = ((word_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
            as BYTE as COUNT as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        match planetI as ::core::ffi::c_int {
            0 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::METAL_WORLD.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 39 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 0 as BYTE;
            }
            1 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::PRIMORDIAL_WORLD.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 72 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 0 as BYTE;
                (*pCurDesc).angle = (((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) - (*pCurDesc).angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            }
            2 => {
                (*pCurDesc).data_index = (C2Rust_Unnamed_3::WATER_WORLD.0
                    as ::core::ffi::c_int
                    | C2Rust_Unnamed_3::PLANET_SHIELDED.0 as ::core::ffi::c_int) as BYTE;
                (*pCurDesc).radius = ((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as SIZE;
                (*pCurDesc).NumPlanets = 2 as BYTE;
            }
            3 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::DUST_WORLD.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 152 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 0 as BYTE;
            }
            4 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::RED_GAS_GIANT.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 500 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 4 as BYTE;
            }
            5 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::ORA_GAS_GIANT.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 750 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 1 as BYTE;
            }
            6 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::GRN_GAS_GIANT.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1000 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 0 as BYTE;
            }
            7 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::BLU_GAS_GIANT.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1250 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = 1 as BYTE;
            }
            8 => {
                (*pCurDesc).data_index = C2Rust_Unnamed_3::PELLUCID_WORLD.0
                    as ::core::ffi::c_int as BYTE;
                (*pCurDesc).radius = (((8 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_long
                    * 1550 as ::core::ffi::c_long / 100 as ::core::ffi::c_long) as SIZE;
                (*pCurDesc).NumPlanets = (if GlobData.SIS_state.Extended
                    as ::core::ffi::c_int != 0
                {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                }) as BYTE;
                if optSeedType == C2Rust_Unnamed_1::OPTVAL_PRIME.0 as ::core::ffi::c_int
                {
                    (*pCurDesc).angle = (((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int)
                        - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int)) as COUNT;
                }
            }
            _ => {}
        }
        (*pCurDesc).orb_speed = ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            as ::core::ffi::c_double
            / (365.25f64
                * pow(
                    ((*pCurDesc).radius as ::core::ffi::c_float
                        / ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                    1.5f64,
                ));
        (*pCurDesc).location.x = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((*pCurDesc).angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pCurDesc).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        (*pCurDesc).location.y = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((*pCurDesc).angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pCurDesc).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        planetI = planetI.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn GenerateSol_generateMoons(
    mut solarSys: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
) -> bool {
    let mut planetNr: COUNT = 0;
    let mut rand_val: DWORD = 0;
    GenerateDefault_generateMoons(solarSys, planet);
    planetNr = planetIndex(solarSys, planet);
    match planetNr as ::core::ffi::c_int {
        2 => {
            let mut angle: COUNT = 0;
            (*solarSys).MoonDesc[0usize].data_index = C2Rust_Unnamed_3::HIERARCHY_STARBASE
                .0 as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[0usize].radius = ((35 as ::core::ffi::c_int)
                << resolutionFactor) as SIZE;
            angle = (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)) as COUNT;
            (*solarSys).MoonDesc[0usize].location.x = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * (*solarSys).MoonDesc[0usize].radius as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            (*solarSys).MoonDesc[0usize].location.y = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * (*solarSys).MoonDesc[0usize].radius as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            (*solarSys).MoonDesc[0usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 11.46f64;
            (*solarSys).MoonDesc[1usize].data_index = C2Rust_Unnamed_3::SELENIC_WORLD.0
                as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[1usize].radius = (((35 as ::core::ffi::c_int)
                << resolutionFactor)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    * ((20 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
            rand_val = RandomContext_Random(SysGenRNG);
            angle = (rand_val as UWORD as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            (*solarSys).MoonDesc[1usize].location.x = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * (*solarSys).MoonDesc[1usize].radius as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            (*solarSys).MoonDesc[1usize].location.y = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * (*solarSys).MoonDesc[1usize].radius as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            (*solarSys).MoonDesc[1usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 27.32f64;
        }
        4 => {
            (*solarSys).MoonDesc[0usize].data_index = C2Rust_Unnamed_3::RADIOACTIVE_WORLD
                .0 as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[0usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 1.75f64;
            (*solarSys).MoonDesc[1usize].data_index = C2Rust_Unnamed_3::HALIDE_WORLD.0
                as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[1usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 3.5f64;
            (*solarSys).MoonDesc[2usize].data_index = C2Rust_Unnamed_3::CYANIC_WORLD.0
                as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[2usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 7.16f64;
            (*solarSys).MoonDesc[3usize].data_index = C2Rust_Unnamed_3::PELLUCID_WORLD.0
                as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[3usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 16.7f64;
        }
        5 => {
            (*solarSys).MoonDesc[0usize].data_index = C2Rust_Unnamed_3::ALKALI_WORLD.0
                as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[0usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 15.22f64;
        }
        7 => {
            (*solarSys).MoonDesc[0usize].data_index = C2Rust_Unnamed_3::VINYLOGOUS_WORLD
                .0 as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[0usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / -5.87f64;
        }
        8 => {
            (*solarSys).MoonDesc[0usize].data_index = C2Rust_Unnamed_3::SELENIC_WORLD.0
                as ::core::ffi::c_int as BYTE;
            (*solarSys).MoonDesc[0usize].orb_speed = ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int) as ::core::ffi::c_double / 6.38f64;
        }
        _ => {}
    }
    return true;
}
unsafe extern "C" fn GenerateSol_generateName(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> bool {
    let mut planetNr: COUNT = planetIndex(solarSys, world);
    utf8StringCopy(
        &raw mut GlobData.SIS_state.PlanetName as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 32]>() as size_t,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                    + planetNr as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(setGameStateUint(
        b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
        (*solarSys).PlanetDesc[planetNr as usize].data_index as DWORD,
    ), false);
    return true;
}
unsafe extern "C" fn GenerateSol_generateOrbital(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
) -> bool {
    let mut rand_val: DWORD = 0;
    let mut planetNr: COUNT = 0;
    if matchWorld(solarSys, world, 2 as BYTE, 0 as BYTE) {
        if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
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
                        b"src/uqm/planets/generate/gensol.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        300 as ::core::ffi::c_int,
                        b"GenerateSol_generateOrbital\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            EncounterGroup = 0 as BYTE;
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
            uqm_lua_checked!(setGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                !(0 as ::core::ffi::c_int) as BYTE as DWORD,
            ), false);
            return true;
        } else {
            uqm_lua_checked!(LoadStdLanderFont(&raw mut (*solarSys).SysInfo.PlanetInfo), false);
            (*solarSys).SysInfo.PlanetInfo.DiscoveryString = CaptureStringTable(
                uqm_lua_checked!(LoadStringTableInstance(b"text.earthbase\0".as_ptr() as RESOURCE), false),
            );
            uqm_lua_checked!(DoDiscoveryReport(MenuSounds), false);
            DestroyStringTable(
                ReleaseStringTable((*solarSys).SysInfo.PlanetInfo.DiscoveryString),
            );
            (*solarSys).SysInfo.PlanetInfo.DiscoveryString = ::core::ptr::null_mut::<
                STRING_TABLE_ENTRY_DESC,
            >();
            FreeLanderFont(&raw mut (*solarSys).SysInfo.PlanetInfo);
            return true;
        }
    }
    DoPlanetaryAnalysis(&raw mut (*solarSys).SysInfo, world);
    rand_val = RandomContext_GetSeed(SysGenRNG);
    (*solarSys)
        .SysInfo
        .PlanetInfo
        .ScanSeed[PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as usize] = rand_val;
    GenerateMineralDeposits(
        &raw mut (*solarSys).SysInfo,
        !(0 as ::core::ffi::c_int) as COUNT,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    rand_val = RandomContext_GetSeed(SysGenRNG);
    planetNr = planetIndex(solarSys, world);
    if worldIsPlanet(solarSys, world) {
        match planetNr as ::core::ffi::c_int {
            0 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 98 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 38 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 3 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 2 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = (59 as ::core::ffi::c_int
                    * 240 as ::core::ffi::c_int) as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = 165 as SIZE;
            }
            1 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = (90 as ::core::ffi::c_int
                    * 50 as ::core::ffi::c_int) as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 95 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 95 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 177 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 7 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = (243
                    as ::core::ffi::c_int * 240 as ::core::ffi::c_int) as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = 457 as SIZE;
            }
            2 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 50 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 100 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 100 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 23 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 240 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = 22 as SIZE;
            }
            3 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 72 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 53 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 24 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 246 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -53 as SIZE;
            }
            4 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0xffff as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 24 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 1120 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 3 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 7 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 98 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -143 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 520 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
            }
            5 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0xffff as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 13 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 945 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 27 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 7 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 102 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -197 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 952 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
            }
            6 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0xffff as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 21 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 411 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 98 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 7 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 172 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -217 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 1916 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
            }
            7 => {
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0xffff as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 28 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 396 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 30 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 7 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 182 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -229 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 2999 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
            }
            8 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
                ), false) == 0
                {
                    uqm_lua_checked!(LoadStdLanderFont(&raw mut (*solarSys).SysInfo.PlanetInfo), false);
                    (*solarSys).PlanetSideFrame[1usize] = CaptureDrawable(
                        uqm_lua_checked!(LoadGraphicInstance(b"graphics.fwiffo\0".as_ptr() as RESOURCE), false)
                            as DRAWABLE,
                    );
                    (*solarSys).SysInfo.PlanetInfo.DiscoveryString = CaptureStringTable(
                        uqm_lua_checked!(LoadStringTableInstance(b"text.fwiffo\0".as_ptr() as RESOURCE), false),
                    );
                }
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 33 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 18 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 119 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Weather = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 1533 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -235 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 3937 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
            }
            _ => {}
        }
        (*solarSys).SysInfo.PlanetInfo.SurfaceGravity = CalcGravity(
            &raw mut (*solarSys).SysInfo.PlanetInfo,
        );
        let mut maskAnim: RESOURCE = ::core::ptr::null::<::core::ffi::c_char>();
        if optScanSphere != 0 {
            match planetNr as ::core::ffi::c_int {
                0 => {
                    maskAnim = b"graphics.mercurymask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                1 => {
                    maskAnim = b"graphics.venusmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                2 => {
                    maskAnim = b"graphics.earthmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                3 => {
                    maskAnim = b"graphics.marsmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                4 => {
                    maskAnim = b"graphics.jupitermask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                5 => {
                    maskAnim = b"graphics.saturnmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                6 => {
                    maskAnim = b"graphics.uranusmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                7 => {
                    maskAnim = b"graphics.neptunemask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                8 => {
                    maskAnim = b"graphics.plutomask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                _ => {}
            }
        } else if planetNr as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            maskAnim = b"graphics.earthmask\0".as_ptr() as *const ::core::ffi::c_char
                as RESOURCE;
        }
        uqm_lua_checked!(LoadPlanet(CaptureDrawable(uqm_lua_checked!(LoadGraphicInstance(maskAnim), false) as DRAWABLE)), false);
    } else {
        let mut moonNr: COUNT = moonIndex(solarSys, world);
        (*solarSys).SysInfo.PlanetInfo.AxialTilt = 0 as SIZE;
        (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0 as UWORD;
        (*solarSys).SysInfo.PlanetInfo.Weather = 0 as UWORD;
        match planetNr as ::core::ffi::c_int {
            2 => {
                (*solarSys)
                    .SysInfo
                    .PlanetInfo
                    .ScanSeed[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
                    as usize] = rand_val;
                if uqm_lua_checked!(getGameStateUint(
                    b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
                ), false) == 0
                {
                    uqm_lua_checked!(LoadStdLanderFont(&raw mut (*solarSys).SysInfo.PlanetInfo), false);
                    (*solarSys).PlanetSideFrame[1usize] = CaptureDrawable(
                        uqm_lua_checked!(LoadGraphicInstance(b"graphics.moonbase\0".as_ptr() as RESOURCE), false)
                            as DRAWABLE,
                    );
                    (*solarSys).SysInfo.PlanetInfo.DiscoveryString = CaptureStringTable(
                        uqm_lua_checked!(LoadStringTableInstance(b"text.moonbase\0".as_ptr() as RESOURCE), false),
                    );
                }
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 60 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 25 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AxialTilt = 0 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = (240
                    as ::core::ffi::c_int * 29 as ::core::ffi::c_int) as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -18 as SIZE;
            }
            4 => {
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 520 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
                match moonNr as ::core::ffi::c_int {
                    0 => {
                        (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 69 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 25 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.Tectonics = 3 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 390 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -163 as SIZE;
                    }
                    1 => {
                        (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 54 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 25 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.Tectonics = 1 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 840 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -161 as SIZE;
                    }
                    2 => {
                        (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 35 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 41 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 1728 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -164 as SIZE;
                    }
                    3 => {
                        (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 35 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 38 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.Tectonics = 1 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 4008 as UWORD;
                        (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -167 as SIZE;
                    }
                    _ => {}
                }
            }
            5 => {
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 952 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 160 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Weather = 2 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 34 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 40 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 3816 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -178 as SIZE;
            }
            7 => {
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 2999 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 10 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Weather = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 95 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 27 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 4300 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -216 as SIZE;
            }
            8 => {
                (*solarSys).SysInfo.PlanetInfo.PlanetDensity = 1 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.PlanetRadius = 9 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.AtmoDensity = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Weather = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.Tectonics = 0 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.RotationPeriod = 1533 as UWORD;
                (*solarSys).SysInfo.PlanetInfo.SurfaceTemperature = -220 as SIZE;
                (*solarSys).SysInfo.PlanetInfo.PlanetToSunDist = (((8
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 3937 as ::core::ffi::c_long
                    / 100 as ::core::ffi::c_long) as DWORD;
            }
            _ => {}
        }
        (*solarSys).SysInfo.PlanetInfo.SurfaceGravity = CalcGravity(
            &raw mut (*solarSys).SysInfo.PlanetInfo,
        );
        let mut maskAnim_0: RESOURCE = ::core::ptr::null::<::core::ffi::c_char>();
        if optScanSphere != 0 {
            match planetNr as ::core::ffi::c_int {
                2 => {
                    if moonNr as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        maskAnim_0 = b"graphics.lunamask\0".as_ptr()
                            as *const ::core::ffi::c_char as RESOURCE;
                    }
                }
                4 => {
                    match moonNr as ::core::ffi::c_int {
                        0 => {
                            maskAnim_0 = b"graphics.iomask\0".as_ptr()
                                as *const ::core::ffi::c_char as RESOURCE;
                        }
                        1 => {
                            maskAnim_0 = b"graphics.europamask\0".as_ptr()
                                as *const ::core::ffi::c_char as RESOURCE;
                        }
                        2 => {
                            maskAnim_0 = b"graphics.ganymedemask\0".as_ptr()
                                as *const ::core::ffi::c_char as RESOURCE;
                        }
                        3 => {
                            maskAnim_0 = b"graphics.callistomask\0".as_ptr()
                                as *const ::core::ffi::c_char as RESOURCE;
                        }
                        _ => {}
                    }
                }
                5 => {
                    maskAnim_0 = b"graphics.titanmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                7 => {
                    maskAnim_0 = b"graphics.tritonmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                8 => {
                    maskAnim_0 = b"graphics.charonmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                }
                _ => {}
            }
        }
        uqm_lua_checked!(LoadPlanet(CaptureDrawable(uqm_lua_checked!(LoadGraphicInstance(maskAnim_0), false) as DRAWABLE)), false);
    }
    return true;
}
unsafe extern "C" fn GenerateSol_generateMinerals(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && matchWorld(solarSys, world, 8 as BYTE, 0 as BYTE) as ::core::ffi::c_int != 0
    {
        return CustomMineralDeposits(
            &raw const (*solarSys).SysInfo,
            whichNode,
            info,
            5 as COUNT,
            C2Rust_Unnamed::CHARON_DUST.0 as ::core::ffi::c_int as COUNT,
            C2Rust_Unnamed_0::LIGHT.0 as ::core::ffi::c_int as BYTE,
        )
    } else if !(optSeedType == C2Rust_Unnamed_1::OPTVAL_PRIME.0 as ::core::ffi::c_int)
        && matchWorld(solarSys, world, 0 as BYTE, -1 as ::core::ffi::c_int as BYTE)
            as ::core::ffi::c_int != 0
    {
        return CustomMineralDeposits(
            &raw const (*solarSys).SysInfo,
            whichNode,
            info,
            5 as COUNT,
            C2Rust_Unnamed::RADIOACTIVE_COMPOUNDS.0 as ::core::ffi::c_int as COUNT,
            C2Rust_Unnamed_0::LIGHT.0 as ::core::ffi::c_int as BYTE,
        )
    } else {
        return GenerateMineralDeposits(&raw const (*solarSys).SysInfo, whichNode, info)
    };
}
unsafe extern "C" fn GenerateSol_generateEnergy(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    if matchWorld(solarSys, world, 8 as BYTE, -1 as ::core::ffi::c_int as BYTE) {
        if uqm_lua_checked!(getGameStateUint(
            b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) != 0
        {
            return 0 as COUNT;
        }
        if !info.is_null() {
            (*info).loc_pt.x = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            (*info).loc_pt.y = (((67 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        return 1 as COUNT;
    }
    if matchWorld(solarSys, world, 2 as BYTE, 1 as BYTE) {
        if uqm_lua_checked!(getGameStateUint(
            b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) != 0
        {
            return 0 as COUNT;
        }
        if !info.is_null() {
            (*info).loc_pt.x = ((if is3DO(optSuperPC).0 != 0 {
                243 as ::core::ffi::c_int
            } else {
                210 as ::core::ffi::c_int
            }
                - if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    33 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 3 as ::core::ffi::c_int
                / 4 as ::core::ffi::c_int) as COORD;
            (*info).loc_pt.y = (((67 as ::core::ffi::c_int) << resolutionFactor)
                * 1 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as COORD;
        }
        return 1 as COUNT;
    }
    return 0 as COUNT;
}
unsafe extern "C" fn GenerateSol_pickupEnergy(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut whichNode: COUNT,
) -> bool {
    if matchWorld(solarSys, world, 8 as BYTE, -1 as ::core::ffi::c_int as BYTE) {
        (uqm_lua_checked!(getGameStateUint(b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char), false)
            == 0 && whichNode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"!GET_GAME_STATE (FOUND_PLUTO_SPATHI) && whichNode == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/planets/generate/gensol.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    737 as ::core::ffi::c_int,
                    b"GenerateSol_pickupEnergy\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if !uqm_lua_checked!(KillLanderCrewSeq(
            8 as COUNT,
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as DWORD,
        ), false) {
            return false;
        }
        uqm_lua_checked!(setGameStateUint(
            b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), false);
        uqm_lua_checked!(GenerateDefault_landerReport(solarSys), false);
        SetLanderTakeoff();
        return false;
    }
    if matchWorld(solarSys, world, 2 as BYTE, 1 as BYTE) {
        (uqm_lua_checked!(getGameStateUint(b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char), false)
            == 0 && whichNode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"!GET_GAME_STATE (MOONBASE_DESTROYED) && whichNode == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/planets/generate/gensol.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    757 as ::core::ffi::c_int,
                    b"GenerateSol_pickupEnergy\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        uqm_lua_checked!(GenerateDefault_landerReport(solarSys), false);
        if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            SetLanderTakeoff();
            uqm_lua_checked!(setGameStateUint(
                b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), false);
            uqm_lua_checked!(setGameStateUint(
                b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), false);
            return true;
        } else {
            return false
        }
    }
    return false;
}
unsafe extern "C" fn GenerateSol_generateLife(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    if matchWorld(solarSys, world, 2 as BYTE, 1 as BYTE) {
        return GenerateRandomNodes(
            &raw const (*solarSys).SysInfo,
            PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as COUNT,
            10 as COUNT,
            (23 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
            whichNode,
            info,
        );
    }
    return 0 as COUNT;
}
unsafe extern "C" fn init_probe() -> ::core::ffi::c_int {
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if uqm_lua_checked!(getGameStateUint(
        b"PROBE_MESSAGE_DELIVERED\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) == 0
        && GetGroupInfo(
                GlobData.Game_state.BattleGroupRef,
                !(0 as ::core::ffi::c_int) as BYTE,
            )
            .0 != 0
        && {
            hGroup = GlobData.Game_state.ip_group_q.head as HIPGROUP;
            !hGroup.is_null()
        }
    {
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        GroupPtr = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        (*GroupPtr).task = C2Rust_Unnamed_5::IN_ORBIT.0 as ::core::ffi::c_int as BYTE;
        (*GroupPtr).sys_loc = (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as BYTE;
        (*GroupPtr).dest_loc = (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as BYTE;
        (*GroupPtr).loc.x = 0 as COORD;
        (*GroupPtr).loc.y = 0 as COORD;
        (*GroupPtr).group_counter = 0 as UWORD;
        UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn check_probe() {
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
    if GlobData.Game_state.BattleGroupRef == 0 {
        return;
    }
    hGroup = GlobData.Game_state.ip_group_q.head as HIPGROUP;
    if hGroup.is_null() {
        return;
    }
    GroupPtr = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup);
    if (*GroupPtr).race_id as ::core::ffi::c_int
        == RACE_ID::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int
        && (*GroupPtr).task as ::core::ffi::c_int
            & C2Rust_Unnamed_5::REFORM_GROUP.0 as ::core::ffi::c_int != 0
    {
        (*GroupPtr).task = (C2Rust_Unnamed_5::FLEE.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::IGNORE_FLAGSHIP.0 as ::core::ffi::c_int) as BYTE;
        (*GroupPtr).dest_loc = 0 as BYTE;
    }
    UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
}
