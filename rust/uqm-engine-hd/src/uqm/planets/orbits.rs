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

// Rust translation of src/uqm/planets/orbits.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type RandomContext;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSeedType: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_GetSeed(context: *mut RandomContext) -> DWORD;
    fn RangeMinMax(
        min: ::core::ffi::c_int,
        max: ::core::ffi::c_int,
        rand: DWORD,
    ) -> ::core::ffi::c_int;
    static mut SysGenRNG: *mut RandomContext;
    static mut CurStarDescPtr: *mut STAR_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type COORD = SWORD;
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
pub type COLORMAP = STRING;
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const BLUE_BODY: Self = Self(0);
    pub const GREEN_BODY: Self = Self(1);
    pub const ORANGE_BODY: Self = Self(2);
    pub const RED_BODY: Self = Self(3);
    pub const WHITE_BODY: Self = Self(4);
    pub const GRAY_BODY: Self = Self(4);
    pub const YELLOW_BODY: Self = Self(5);
    pub const NUM_STAR_COLORS: Self = Self(6);
    pub const CYAN_BODY: Self = Self(6);
    pub const PURPLE_BODY: Self = Self(7);
    pub const VIOLET_BODY: Self = Self(8);
    pub const NUM_PLANET_COLORS: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub MinRockyDist: COUNT,
    pub MinGasGDist: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const PLANET_NEVER: Self = Self(0);
    pub const PLANET_RARE: Self = Self(15);
    pub const PLANET_FEW: Self = Self(63);
    pub const PLANET_COMMON: Self = Self(127);
    pub const PLANET_ALWAYS: Self = Self(255);
}
unsafe extern "C" fn BlueDistribution(mut which_world: BYTE) -> BYTE {
    let PlanetDistribution: [BYTE; 59] = [
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
    ];
    return PlanetDistribution[which_world as usize];
}
unsafe extern "C" fn GreenDistribution(mut which_world: BYTE) -> BYTE {
    let PlanetDistribution: [BYTE; 59] = [
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
    ];
    return PlanetDistribution[which_world as usize];
}
unsafe extern "C" fn OrangeDistribution(mut which_world: BYTE) -> BYTE {
    let PlanetDistribution: [BYTE; 59] = [
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
    ];
    return PlanetDistribution[which_world as usize];
}
unsafe extern "C" fn RedDistribution(mut which_world: BYTE) -> BYTE {
    let PlanetDistribution: [BYTE; 59] = [
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
    ];
    return PlanetDistribution[which_world as usize];
}
unsafe extern "C" fn WhiteDistribution(mut which_world: BYTE) -> BYTE {
    let PlanetDistribution: [BYTE; 59] = [
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
    ];
    return PlanetDistribution[which_world as usize];
}
unsafe extern "C" fn YellowDistribution(mut which_world: BYTE) -> BYTE {
    let PlanetDistribution: [BYTE; 59] = [
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
        C2Rust_Unnamed_4::PLANET_ALWAYS.0 as ::core::ffi::c_int as BYTE,
    ];
    return PlanetDistribution[which_world as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ComputeSpeed(
    mut planet: *mut PLANET_DESC,
    mut GeneratingMoons: BOOLEAN,
    mut rand_val: UWORD,
) {
    if GeneratingMoons.0 != 0 {
        (*planet).orb_speed = ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            as ::core::ffi::c_double
            / (29 as ::core::ffi::c_int as ::core::ffi::c_double
                * pow(
                    (*planet).radius as ::core::ffi::c_double
                        / (((35 as ::core::ffi::c_int) << resolutionFactor)
                            + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                * ((20 as ::core::ffi::c_int) << resolutionFactor))
                            as ::core::ffi::c_double,
                    1.5f64,
                ));
        if (*(*planet).pPrevDesc).data_index as ::core::ffi::c_int
            & !(C2Rust_Unnamed_2::PLANET_SHIELDED.0 as ::core::ffi::c_int)
            >= C2Rust_Unnamed_2::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
        {
            (*planet).orb_speed *= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if rand_val as ::core::ffi::c_int % 7 as ::core::ffi::c_int == 0 {
            (*planet).orb_speed = -(*planet).orb_speed;
        }
    } else {
        (*planet).orb_speed = ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            as ::core::ffi::c_double
            / (365.25f64
                * pow(
                    (*planet).radius as ::core::ffi::c_double
                        / ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            as ::core::ffi::c_double,
                    1.5f64,
                ));
    };
}
#[no_mangle]
pub unsafe extern "C" fn FillOrbits(
    mut system: *mut SOLARSYS_STATE,
    mut NumPlanets: BYTE,
    mut pBaseDesc: *mut PLANET_DESC,
    mut TypesDefined: BOOLEAN,
) {
    let mut StarColor: BYTE = 0;
    let mut PlanetCount: BYTE = 0;
    let mut MaxPlanet: BYTE = 0;
    let mut GeneratingMoons: BOOLEAN = BOOLEAN::FALSE;
    let mut StarSize: COUNT = 0;
    let mut pPD: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut loopCounter: DWORD = 0 as DWORD;
    let mut orbitDiff: BYTE = 0;
    let mut Suns: [C2Rust_Unnamed_3; 3] = [
        C2Rust_Unnamed_3 {
            MinRockyDist: ((4 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as COUNT,
            MinGasGDist: ((12 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as COUNT,
        },
        C2Rust_Unnamed_3 {
            MinRockyDist: ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as COUNT,
            MinGasGDist: ((13 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as COUNT,
        },
        C2Rust_Unnamed_3 {
            MinRockyDist: ((16 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as COUNT,
            MinGasGDist: ((33 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as COUNT,
        },
    ];
    if NumPlanets as ::core::ffi::c_int
        == -2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        NumPlanets = (*system).SunDesc[0usize].NumPlanets;
        pBaseDesc = &raw mut (*system).PlanetDesc as *mut PLANET_DESC;
    }
    pPD = pBaseDesc;
    StarSize = (*system).SunDesc[0usize].data_index as COUNT;
    StarColor = (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE;
    if NumPlanets as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        loop {
            NumPlanets = (RandomContext_Random(SysGenRNG) as UWORD as ::core::ffi::c_int
                % (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as BYTE;
            if NumPlanets as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                break;
            }
        }
        (*system).SunDesc[0usize].NumPlanets = NumPlanets;
    }
    GeneratingMoons = BOOLEAN(
        (pBaseDesc == &raw mut (*system).MoonDesc as *mut PLANET_DESC)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut GasGiant: BOOLEAN = BOOLEAN(
        (!pPD.is_null() && !(*pPD).pPrevDesc.is_null()
            && (*(*pPD).pPrevDesc).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed_2::PLANET_SHIELDED.0 as ::core::ffi::c_int)
                >= C2Rust_Unnamed_2::FIRST_GAS_GIANT.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if GeneratingMoons.0 != 0 {
        MaxPlanet = (if optSeedType
            == C2Rust_Unnamed::OPTVAL_PRIME.0 as ::core::ffi::c_int
            || optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                && GasGiant.0 == 0
        {
            C2Rust_Unnamed_2::FIRST_LARGE_ROCKY_WORLD.0 as ::core::ffi::c_int
        } else if optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int {
            C2Rust_Unnamed_2::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_2::LAST_LARGE_ROCKY_WORLD.0 as ::core::ffi::c_int
        }) as BYTE;
    } else {
        MaxPlanet = C2Rust_Unnamed_2::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
            as BYTE;
    }
    PlanetCount = NumPlanets;
    loop {
        let c2rust_fresh0 = NumPlanets;
        NumPlanets = NumPlanets.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut chance: BYTE = 0;
        let mut rand_val: DWORD = 0;
        let mut min_radius: DWORD = 0;
        let mut delta_r: SIZE = 0;
        let mut pLocPD: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
        loop {
            rand_val = RandomContext_Random(SysGenRNG);
            if TypesDefined.0 != 0 {
                rand_val = 0 as DWORD;
            } else {
                (*pPD).data_index = ((rand_val as UWORD as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    % MaxPlanet as ::core::ffi::c_int) as BYTE;
            }
            chance = C2Rust_Unnamed_4::PLANET_NEVER.0 as ::core::ffi::c_int as BYTE;
            match StarColor as ::core::ffi::c_int {
                0 => {
                    chance = BlueDistribution((*pPD).data_index);
                }
                1 => {
                    chance = GreenDistribution((*pPD).data_index);
                }
                2 => {
                    chance = OrangeDistribution((*pPD).data_index);
                }
                3 => {
                    chance = RedDistribution((*pPD).data_index);
                }
                4 => {
                    chance = WhiteDistribution((*pPD).data_index);
                }
                5 => {
                    chance = YellowDistribution((*pPD).data_index);
                }
                _ => {}
            }
            if (rand_val as UWORD as BYTE as ::core::ffi::c_int)
                < chance as ::core::ffi::c_int
            {
                break;
            }
        }
        if ((*pPD).data_index as ::core::ffi::c_int)
            < C2Rust_Unnamed_2::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
        {
            min_radius = Suns[StarSize as usize].MinRockyDist as DWORD;
        } else {
            min_radius = Suns[StarSize as usize].MinGasGDist as DWORD;
        }
        's_279: loop {
            rand_val = RandomContext_Random(SysGenRNG);
            if GeneratingMoons.0 != 0 {
                (*pPD).radius = (((35 as ::core::ffi::c_int) << resolutionFactor)
                    + rand_val as UWORD as ::core::ffi::c_int % 4 as ::core::ffi::c_int
                        * ((20 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
                pLocPD = pPD.offset(-(1 as ::core::ffi::c_int as isize));
                while pLocPD >= pBaseDesc {
                    if (*pPD).radius as ::core::ffi::c_int
                        == (*pLocPD).radius as ::core::ffi::c_int
                    {
                        continue 's_279;
                    }
                    pLocPD = pLocPD.offset(-1);
                }
                (*pPD).NumPlanets = 0 as BYTE;
                break;
            } else {
                (*pPD).radius = (rand_val as UWORD as DWORD)
                    .wrapping_rem(
                        (((124 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            as DWORD)
                            .wrapping_sub(min_radius),
                    )
                    .wrapping_add(min_radius) as SIZE;
                pLocPD = pPD.offset(-(1 as ::core::ffi::c_int as isize));
                loop {
                    if pLocPD < pBaseDesc {
                        break 's_279;
                    }
                    if loopCounter < 0xffff as DWORD {
                        delta_r = (((*pLocPD).radius as ::core::ffi::c_int
                            >> 6 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int
                            - ((*pPD).radius as ::core::ffi::c_int
                                >> 6 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int)
                            as SIZE;
                        orbitDiff = 1 as BYTE;
                    } else {
                        delta_r = (((*pLocPD).radius as ::core::ffi::c_int
                            >> 6 as ::core::ffi::c_int)
                            - ((*pPD).radius as ::core::ffi::c_int
                                >> 6 as ::core::ffi::c_int)) as SIZE;
                        orbitDiff = 5 as BYTE;
                    }
                    if delta_r as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        delta_r = -(delta_r as ::core::ffi::c_int) as SIZE;
                    }
                    if delta_r as ::core::ffi::c_int <= orbitDiff as ::core::ffi::c_int
                        && loopCounter
                            < (0xffff as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as DWORD
                    {
                        loopCounter = loopCounter.wrapping_add(1);
                        break;
                    } else {
                        pLocPD = pLocPD.offset(-1);
                    }
                }
            }
        }
        rand_val = RandomContext_Random(SysGenRNG);
        (*pPD).angle = (rand_val as UWORD as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        (*pPD).location.x = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((*pPD).angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pPD).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        (*pPD).location.y = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((*pPD).angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pPD).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        if GeneratingMoons.0 != 0 {
            (*pPD).rand_seed = ((*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    ((*pPD).angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * ((*pPD).radius as ::core::ffi::c_int >> resolutionFactor)
                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
                as DWORD) << 16 as ::core::ffi::c_int
                | (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        ((*pPD).angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((*pPD).radius as ::core::ffi::c_int >> resolutionFactor)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
                    as UWORD as DWORD;
        } else {
            (*pPD).rand_seed = ((*pPD).location.y as DWORD) << 16 as ::core::ffi::c_int
                | (*pPD).location.x as UWORD as DWORD;
        }
        ComputeSpeed(
            pPD,
            GeneratingMoons,
            (rand_val >> 16 as ::core::ffi::c_int) as UWORD,
        );
        pPD = pPD.offset(1);
    }
    let mut i: BYTE = 0;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < PlanetCount as ::core::ffi::c_int {
        let mut j: BYTE = 0;
        j = (PlanetCount as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE;
        while j as ::core::ffi::c_int > i as ::core::ffi::c_int {
            if (*pBaseDesc.offset(i as isize)).radius as ::core::ffi::c_int
                > (*pBaseDesc.offset(j as isize)).radius as ::core::ffi::c_int
            {
                let mut temp: PLANET_DESC = PLANET_DESC {
                    rand_seed: 0,
                    data_index: 0,
                    NumPlanets: 0,
                    radius: 0,
                    location: POINT { x: 0, y: 0 },
                    temp_color: Color { r: 0, g: 0, b: 0, a: 0 },
                    NextIndex: 0,
                    image: STAMP {
                        origin: POINT { x: 0, y: 0 },
                        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                    },
                    pPrevDesc: ::core::ptr::null_mut::<PLANET_DESC>(),
                    orbit: PLANET_ORBIT {
                        TopoZoomFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        lpTopoData: ::core::ptr::null_mut::<SBYTE>(),
                        SphereFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        ObjectFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        TintFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        TintColor: Color { r: 0, g: 0, b: 0, a: 0 },
                        TopoColors: ::core::ptr::null_mut::<Color>(),
                        ScanColors: ::core::ptr::null_mut::<*mut Color>(),
                        ScratchArray: ::core::ptr::null_mut::<Color>(),
                        WorkFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        BackFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        light_diff: ::core::ptr::null_mut::<*mut DWORD>(),
                        map_rotate: ::core::ptr::null_mut::<*mut MAP3D_POINT>(),
                        TopoMask: ::core::ptr::null_mut::<FRAME_DESC>(),
                        sphereBytes: ::core::ptr::null_mut::<BYTE>(),
                        sphereMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
                        scanType: 0,
                        Shade: ::core::ptr::null_mut::<FRAME_DESC>(),
                        ShadeColors: ::core::ptr::null_mut::<Color>(),
                    },
                    size: 0,
                    angle: 0,
                    rotFrameIndex: 0,
                    rotPointIndex: 0,
                    rotwidth: 0,
                    rotheight: 0,
                    orb_speed: 0.,
                    rot_speed: 0.,
                    PlanetByte: 0,
                    MoonByte: 0,
                    frame_offset: 0,
                };
                temp = *pBaseDesc.offset(i as isize);
                *pBaseDesc.offset(i as isize) = *pBaseDesc.offset(j as isize);
                *pBaseDesc.offset(j as isize) = temp;
            }
            j = j.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PickClosestHabitable(
    mut solarSys: *mut SOLARSYS_STATE,
) -> BYTE {
    let hRangesD: [[SIZE; 2]; 6] = [
        [860 as SIZE, 1790 as SIZE],
        [540 as SIZE, 1150 as SIZE],
        [140 as SIZE, 280 as SIZE],
        [0 as SIZE, 0 as SIZE],
        [1240 as SIZE, 2560 as SIZE],
        [320 as SIZE, 640 as SIZE],
    ];
    let hRangesG: [[SIZE; 2]; 6] = [
        [0 as SIZE, 0 as SIZE],
        [0 as SIZE, 0 as SIZE],
        [3410 as SIZE, 7120 as SIZE],
        [860 as SIZE, 1800 as SIZE],
        [0 as SIZE, 0 as SIZE],
        [7630 as SIZE, 7936 as SIZE],
    ];
    let mut starColor: BYTE = 0;
    let mut starType: BYTE = 0;
    let mut numPlanets: BYTE = 0;
    let mut i: BYTE = 0;
    let mut hRangeMin: SIZE = 0 as SIZE;
    let mut hRangeMax: SIZE = 0 as SIZE;
    let mut hRangeMed: SIZE = 0 as SIZE;
    let mut pByte: BYTE = 0 as BYTE;
    let mut pPD: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut pPlanet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut rand: DWORD = RandomContext_GetSeed(SysGenRNG);
    numPlanets = (*solarSys).SunDesc[0usize].NumPlanets;
    pPD = &raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC;
    starColor = (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE;
    starType = (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE;
    if starType as ::core::ffi::c_int
        == C2Rust_Unnamed_0::SUPER_GIANT_STAR.0 as ::core::ffi::c_int
    {
        let c2rust_fresh1 = numPlanets;
        numPlanets = numPlanets.wrapping_sub(1);
        return c2rust_fresh1;
    }
    if starType as ::core::ffi::c_int
        == C2Rust_Unnamed_0::DWARF_STAR.0 as ::core::ffi::c_int
    {
        if starColor as ::core::ffi::c_int
            == C2Rust_Unnamed_1::RED_BODY.0 as ::core::ffi::c_int
        {
            return 0 as BYTE;
        }
        hRangeMin = hRangesD[starColor as usize][0usize];
        hRangeMax = hRangesD[starColor as usize][1usize];
    }
    if starType as ::core::ffi::c_int
        == C2Rust_Unnamed_0::GIANT_STAR.0 as ::core::ffi::c_int
    {
        if starColor as ::core::ffi::c_int
            == C2Rust_Unnamed_1::BLUE_BODY.0 as ::core::ffi::c_int
            || starColor as ::core::ffi::c_int
                == C2Rust_Unnamed_1::GREEN_BODY.0 as ::core::ffi::c_int
            || starColor as ::core::ffi::c_int
                == C2Rust_Unnamed_1::WHITE_BODY.0 as ::core::ffi::c_int
        {
            let c2rust_fresh2 = numPlanets;
            numPlanets = numPlanets.wrapping_sub(1);
            pPlanet = pPD.offset(c2rust_fresh2 as isize);
            (*pPlanet).radius = ((124 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as SIZE;
            (*pPlanet).angle = (rand as UWORD as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            (*pPlanet).location.x = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    ((*pPlanet).angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * (*pPlanet).radius as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            (*pPlanet).location.y = (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    ((*pPlanet).angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * (*pPlanet).radius as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            ComputeSpeed(
                pPlanet,
                BOOLEAN::FALSE,
                (rand >> 16 as ::core::ffi::c_int) as UWORD,
            );
            let c2rust_fresh3 = numPlanets;
            numPlanets = numPlanets.wrapping_sub(1);
            return c2rust_fresh3;
        }
        hRangeMin = hRangesG[starColor as usize][0usize];
        hRangeMax = hRangesG[starColor as usize][1usize];
    }
    hRangeMed = ((hRangeMin as ::core::ffi::c_int + hRangeMax as ::core::ffi::c_int)
        / 2 as ::core::ffi::c_int) as SIZE;
    if numPlanets as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        let mut dist: SIZE = (*pPD.offset(0isize)).radius;
        i = 1 as BYTE;
        while (i as ::core::ffi::c_int) < numPlanets as ::core::ffi::c_int {
            if abs(dist as ::core::ffi::c_int - hRangeMed as ::core::ffi::c_int)
                >= abs(
                    (*pPD.offset(i as isize)).radius as ::core::ffi::c_int
                        - hRangeMed as ::core::ffi::c_int,
                )
            {
                dist = (*pPD.offset(i as isize)).radius;
                pByte = i;
            }
            i = i.wrapping_add(1);
        }
    } else {
        pByte = 0 as BYTE;
    }
    pPlanet = pPD.offset(pByte as isize);
    if ((*pPlanet).radius as ::core::ffi::c_int) < hRangeMin as ::core::ffi::c_int
        || (*pPlanet).radius as ::core::ffi::c_int > hRangeMax as ::core::ffi::c_int
    {
        let mut min: SIZE = 0;
        let mut max: SIZE = 0;
        if ((*pPlanet).radius as ::core::ffi::c_int) < hRangeMin as ::core::ffi::c_int {
            min = hRangeMin;
            max = hRangeMed;
        } else if (*pPlanet).radius as ::core::ffi::c_int
            > hRangeMax as ::core::ffi::c_int
        {
            min = hRangeMed;
            max = hRangeMax;
        }
        (*pPlanet).radius = RangeMinMax(
            min as ::core::ffi::c_int,
            max as ::core::ffi::c_int,
            rand,
        ) as SIZE;
        (*pPlanet).angle = (rand as UWORD as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        (*pPlanet).location.x = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((*pPlanet).angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pPlanet).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        (*pPlanet).location.y = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((*pPlanet).angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long * (*pPlanet).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        ComputeSpeed(
            pPlanet,
            BOOLEAN::FALSE,
            (rand >> 16 as ::core::ffi::c_int) as UWORD,
        );
    }
    return pByte;
}
#[no_mangle]
pub unsafe extern "C" fn CheckForHabitable(
    mut solarSys: *mut SOLARSYS_STATE,
) -> BOOLEAN {
    let HabitableRanges: [[SIZE; 2]; 6] = [
        [853 as SIZE, 1790 as SIZE],
        [544 as SIZE, 1151 as SIZE],
        [139 as SIZE, 287 as SIZE],
        [68 as SIZE, 68 as SIZE],
        [1231 as SIZE, 2569 as SIZE],
        [312 as SIZE, 778 as SIZE],
    ];
    let mut pPD: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut starColor: BYTE = 0;
    let mut i: SIZE = 0;
    let mut planetByte: BYTE = (*solarSys).SunDesc[0usize].PlanetByte;
    let mut numPlanets: BYTE = (*solarSys).SunDesc[0usize].NumPlanets;
    let mut habitableRangeMin: SIZE = 0;
    let mut habitableRangeMax: SIZE = 0;
    let mut newRadius: SIZE = 0;
    let mut oldRadius: SIZE = 0;
    let mut planetRadii: [DWORD; 9] = [0; 9];
    let mut rand_val: DWORD = 0;
    starColor = (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE;
    if optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
        && starColor as ::core::ffi::c_int
            == C2Rust_Unnamed_1::RED_BODY.0 as ::core::ffi::c_int
    {
        starColor = C2Rust_Unnamed_1::ORANGE_BODY.0 as ::core::ffi::c_int as BYTE;
    }
    pPD = &raw mut (*solarSys).PlanetDesc as *mut PLANET_DESC;
    oldRadius = (*pPD.offset(planetByte as isize)).radius;
    habitableRangeMin = HabitableRanges[starColor as usize][0usize];
    habitableRangeMax = HabitableRanges[starColor as usize][1usize];
    if optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
        && numPlanets as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        && ((*pPD.offset(1isize)).radius as ::core::ffi::c_int)
            < habitableRangeMax as ::core::ffi::c_int + 320 as ::core::ffi::c_int
        && (*pPD.offset(1isize)).radius as ::core::ffi::c_int
            > habitableRangeMin as ::core::ffi::c_int + 320 as ::core::ffi::c_int
    {
        habitableRangeMax = ((*pPD.offset(1isize)).radius as ::core::ffi::c_int
            - 320 as ::core::ffi::c_int) as SIZE;
    }
    if oldRadius as ::core::ffi::c_int >= habitableRangeMin as ::core::ffi::c_int
        && oldRadius as ::core::ffi::c_int <= habitableRangeMax as ::core::ffi::c_int
        || starColor as ::core::ffi::c_int
            == C2Rust_Unnamed_1::RED_BODY.0 as ::core::ffi::c_int
        || planetByte as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    i = 0 as SIZE;
    while (i as ::core::ffi::c_int) < numPlanets as ::core::ffi::c_int {
        planetRadii[i as usize] = (*pPD.offset(i as isize)).radius as DWORD;
        i += 1;
    }
    rand_val = RandomContext_Random(SysGenRNG);
    newRadius = (rand_val as UWORD as ::core::ffi::c_int
        % (habitableRangeMax as ::core::ffi::c_int
            - habitableRangeMin as ::core::ffi::c_int)
        + habitableRangeMin as ::core::ffi::c_int) as SIZE;
    planetRadii[planetByte as usize] = newRadius as DWORD;
    if optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
        && numPlanets as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || planetRadii[planetByte as usize]
            < planetRadii[(planetByte as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize]
    {
        (*pPD.offset(planetByte as isize)).radius = planetRadii[planetByte as usize]
            as SIZE;
        (*pPD.offset(planetByte as isize)).angle = (rand_val as UWORD
            as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        (*pPD.offset(planetByte as isize)).location.x = (*(&raw mut sinetab
            as *mut SDWORD)
            .offset(
                ((*pPD.offset(planetByte as isize)).angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * (*pPD.offset(planetByte as isize)).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        (*pPD.offset(planetByte as isize)).location.y = (*(&raw mut sinetab
            as *mut SDWORD)
            .offset(
                ((*pPD.offset(planetByte as isize)).angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * (*pPD.offset(planetByte as isize)).radius as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
        ComputeSpeed(
            pPD.offset(planetByte as isize),
            BOOLEAN::FALSE,
            (rand_val >> 16 as ::core::ffi::c_int) as UWORD,
        );
        return BOOLEAN::TRUE;
    } else {
        return BOOLEAN::FALSE
    };
}
