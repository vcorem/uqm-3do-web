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

/* ----------------------------- INCLUDES ---------------------------- */

// Rust translation of src/uqm/planets/calc.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type RandomContext;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut SysGenRNG: *mut RandomContext;
    static mut PlanData: *const PlanetFrame;
    static mut CurStarDescPtr: *mut STAR_DESC;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
    pub const LIGHT: Self = Self(0);
    pub const MEDIUM: Self = Self(1);
    pub const HEAVY: Self = Self(2);
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
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const GAS_DENSITY: Self = Self(0);
    pub const LIGHT_DENSITY: Self = Self(1);
    pub const LOW_DENSITY: Self = Self(2);
    pub const NORMAL_DENSITY: Self = Self(3);
    pub const HIGH_DENSITY: Self = Self(4);
    pub const SUPER_DENSITY: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const RED_SUN_INTENSITY: Self = Self(0);
    pub const ORANGE_SUN_INTENSITY: Self = Self(1);
    pub const YELLOW_SUN_INTENSITY: Self = Self(2);
    pub const GREEN_SUN_INTENSITY: Self = Self(3);
    pub const BLUE_SUN_INTENSITY: Self = Self(4);
    pub const WHITE_SUN_INTENSITY: Self = Self(5);
}
unsafe extern "C" fn CalcFromBase(mut base: UWORD, mut variance: UWORD) -> UWORD {
    return (base as ::core::ffi::c_int
        + RandomContext_Random(SysGenRNG) as UWORD as ::core::ffi::c_int
            % variance as ::core::ffi::c_int) as UWORD;
}
#[inline]
unsafe extern "C" fn CalcHalfBaseVariance(mut base: UWORD) -> UWORD {
    return CalcFromBase(
        base,
        ((base as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as UWORD,
    );
}
unsafe extern "C" fn CalcSysInfo(mut SysInfoPtr: *mut SYSTEM_INFO) {
    (*SysInfoPtr).StarSize = (*pSolarSysState).SunDesc[0usize].data_index;
    match (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int
    {
        0 => {
            (*SysInfoPtr).StarIntensity = C2Rust_Unnamed_5::BLUE_SUN_INTENSITY.0
                as ::core::ffi::c_int as BYTE;
        }
        1 => {
            (*SysInfoPtr).StarIntensity = C2Rust_Unnamed_5::GREEN_SUN_INTENSITY.0
                as ::core::ffi::c_int as BYTE;
        }
        2 => {
            (*SysInfoPtr).StarIntensity = C2Rust_Unnamed_5::ORANGE_SUN_INTENSITY.0
                as ::core::ffi::c_int as BYTE;
        }
        3 => {
            (*SysInfoPtr).StarIntensity = C2Rust_Unnamed_5::RED_SUN_INTENSITY.0
                as ::core::ffi::c_int as BYTE;
        }
        4 => {
            (*SysInfoPtr).StarIntensity = C2Rust_Unnamed_5::WHITE_SUN_INTENSITY.0
                as ::core::ffi::c_int as BYTE;
        }
        5 => {
            (*SysInfoPtr).StarIntensity = C2Rust_Unnamed_5::YELLOW_SUN_INTENSITY.0
                as ::core::ffi::c_int as BYTE;
        }
        _ => {}
    }
    match (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        0 => {
            (*SysInfoPtr).StarEnergy = (((*SysInfoPtr).StarIntensity
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                * 1 as ::core::ffi::c_int) as UWORD;
        }
        1 => {
            (*SysInfoPtr).StarEnergy = (((*SysInfoPtr).StarIntensity
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int) as UWORD;
        }
        2 => {
            (*SysInfoPtr).StarEnergy = (((*SysInfoPtr).StarIntensity
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int) as UWORD;
        }
        _ => {}
    };
}
unsafe extern "C" fn GeneratePlanetComposition(
    mut PlanetInfoPtr: *mut PLANET_INFO,
    mut SurfaceTemp: SIZE,
    mut radius: SIZE,
) -> UWORD {
    if ((*(*PlanetInfoPtr).PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::GAS_GIANT.0 as ::core::ffi::c_int
    {
        (*PlanetInfoPtr).Weather = ((7 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as UWORD;
        return 0xffff as UWORD;
    } else {
        let mut range: BYTE = 0;
        let mut atmo: UWORD = 0;
        (*PlanetInfoPtr).Weather = 0 as UWORD;
        atmo = 0 as UWORD;
        range = ((*(*PlanetInfoPtr).PlanDataPtr).AtmoAndDensity as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        if range as ::core::ffi::c_int <= C2Rust_Unnamed::HEAVY.0 as ::core::ffi::c_int {
            if (SurfaceTemp as ::core::ffi::c_int) < -40 as ::core::ffi::c_int {
                range = range.wrapping_sub(1);
            } else if SurfaceTemp as ::core::ffi::c_int > 100 as ::core::ffi::c_int {
                range = range.wrapping_add(1);
            }
            if range as ::core::ffi::c_int
                <= C2Rust_Unnamed::HEAVY.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
            {
                match range as ::core::ffi::c_int {
                    0 => {
                        atmo = 10 as UWORD;
                        (*PlanetInfoPtr).Weather = ((1 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int) as UWORD;
                    }
                    1 => {
                        atmo = 75 as UWORD;
                        (*PlanetInfoPtr).Weather = ((2 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int) as UWORD;
                    }
                    2 => {
                        atmo = 200 as UWORD;
                        (*PlanetInfoPtr).Weather = ((4 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int) as UWORD;
                    }
                    _ => {
                        atmo = 2500 as UWORD;
                        (*PlanetInfoPtr).Weather = ((6 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int) as UWORD;
                    }
                }
                radius = (radius as ::core::ffi::c_int
                    / ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as SIZE;
                if (radius as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    (*PlanetInfoPtr).Weather = ((*PlanetInfoPtr).Weather
                        as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                        as UWORD;
                } else if radius as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                    (*PlanetInfoPtr).Weather = ((*PlanetInfoPtr).Weather
                        as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                        as UWORD;
                }
                atmo = CalcHalfBaseVariance(atmo);
            }
        }
        return atmo;
    };
}
unsafe extern "C" fn CalcTemp(
    mut SysInfoPtr: *mut SYSTEM_INFO,
    mut radius: SIZE,
) -> SIZE {
    let mut alb: DWORD = 0;
    let mut centigrade: SIZE = 0;
    let mut bonus: SIZE = 0;
    alb = (100 as ::core::ffi::c_int - 33 as ::core::ffi::c_int) as DWORD;
    alb = (square_root(
        square_root(alb.wrapping_mul(100 as DWORD).wrapping_mul(10000 as DWORD)) as DWORD,
    ) as ::core::ffi::c_long * 277 as ::core::ffi::c_long
        * (*SysInfoPtr).StarEnergy as ::core::ffi::c_long
        / ((C2Rust_Unnamed_5::YELLOW_SUN_INTENSITY.0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as DWORD;
    centigrade = (alb
        .wrapping_div(
            square_root(
                (radius as ::core::ffi::c_long * 10000 as ::core::ffi::c_long
                    / ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) as DWORD,
            ) as DWORD,
        ) as SIZE as ::core::ffi::c_int - 273 as ::core::ffi::c_int) as SIZE;
    bonus = 0 as SIZE;
    if SysInfoPtr == &raw mut (*pSolarSysState).SysInfo
        && ((*(*SysInfoPtr).PlanetInfo.PlanDataPtr).AtmoAndDensity as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int <= C2Rust_Unnamed::HEAVY.0 as ::core::ffi::c_int
    {
        if centigrade as ::core::ffi::c_int >= 100 as ::core::ffi::c_int {
            bonus = 200 as SIZE;
        } else if centigrade as ::core::ffi::c_int >= -40 as ::core::ffi::c_int {
            bonus = 20 as SIZE;
        }
        bonus = ((bonus as ::core::ffi::c_int)
            << ((*(*SysInfoPtr).PlanetInfo.PlanDataPtr).AtmoAndDensity
                as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as SIZE;
        bonus = CalcHalfBaseVariance(bonus as UWORD) as SIZE;
    }
    return (centigrade as ::core::ffi::c_int + bonus as ::core::ffi::c_int) as SIZE;
}
unsafe extern "C" fn CalcRotation(mut PlanetInfoPtr: *mut PLANET_INFO) -> COUNT {
    if ((*(*PlanetInfoPtr).PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::GAS_GIANT.0 as ::core::ffi::c_int
    {
        return CalcFromBase(80 as UWORD, 80 as UWORD)
    } else if RandomContext_Random(SysGenRNG) as UWORD as BYTE as ::core::ffi::c_int
        % 10 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        return CalcFromBase(
            (50 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
                * 240 as ::core::ffi::c_int) as UWORD,
            (200 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
                * 240 as ::core::ffi::c_int) as UWORD,
        )
    } else {
        return CalcFromBase(150 as UWORD, 150 as UWORD)
    };
}
unsafe extern "C" fn CalcTilt() -> SIZE {
    let mut tilt: SIZE = 0;
    let mut i: BYTE = 0;
    tilt = -(180 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE;
    i = 10 as BYTE;
    loop {
        tilt = (tilt as ::core::ffi::c_int
            + RandomContext_Random(SysGenRNG) as UWORD as ::core::ffi::c_int
                % (180 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)) as SIZE;
        i = i.wrapping_sub(1);
        if i == 0 {
            break;
        }
    }
    return tilt;
}
#[no_mangle]
pub unsafe extern "C" fn CalcGravity(mut PlanetInfoPtr: *const PLANET_INFO) -> UWORD {
    return ((*PlanetInfoPtr).PlanetDensity as DWORD)
        .wrapping_mul((*PlanetInfoPtr).PlanetRadius as DWORD)
        .wrapping_div(100 as DWORD) as UWORD;
}
unsafe extern "C" fn CalcTectonics(mut base: UWORD, mut temp: UWORD) -> UWORD {
    let mut tect: UWORD = CalcFromBase(
        base,
        ((3 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as UWORD,
    );
    return tect;
}
unsafe extern "C" fn CalcLifeChance(
    mut PlanetInfoPtr: *const PLANET_INFO,
) -> ::core::ffi::c_int {
    let mut life_var: SIZE = 0 as SIZE;
    if ((*(*PlanetInfoPtr).PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::GAS_GIANT.0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < -151 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int - 300 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < -51 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int - 100 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < 50 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 300 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < 150 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < 250 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int - 100 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).SurfaceTemperature as ::core::ffi::c_int)
        < 500 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int - 400 as ::core::ffi::c_int) as SIZE;
    } else {
        life_var = (life_var as ::core::ffi::c_int - 800 as ::core::ffi::c_int) as SIZE;
    }
    if (*PlanetInfoPtr).AtmoDensity as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        life_var = (life_var as ::core::ffi::c_int - 1000 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).AtmoDensity as ::core::ffi::c_int)
        < 15 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 100 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).AtmoDensity as ::core::ffi::c_int)
        < 30 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 200 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).AtmoDensity as ::core::ffi::c_int)
        < 100 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 300 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).AtmoDensity as ::core::ffi::c_int)
        < 1000 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int + 150 as ::core::ffi::c_int) as SIZE;
    } else if ((*PlanetInfoPtr).AtmoDensity as ::core::ffi::c_int)
        >= 2500 as ::core::ffi::c_int
    {
        life_var = (life_var as ::core::ffi::c_int - 100 as ::core::ffi::c_int) as SIZE;
    }
    life_var = (life_var as ::core::ffi::c_int
        + (200 as ::core::ffi::c_int + 80 as ::core::ffi::c_int
            + 80 as ::core::ffi::c_int)) as SIZE;
    return life_var as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DoPlanetaryAnalysis(
    mut SysInfoPtr: *mut SYSTEM_INFO,
    mut pPlanetDesc: *mut PLANET_DESC,
) {
    ((*pPlanetDesc).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_3::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)
        < C2Rust_Unnamed_3::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"(pPlanetDesc->data_index & ~WORLD_TYPE_SPECIAL) < NUMBER_OF_PLANET_TYPES\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/calc.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                362 as ::core::ffi::c_int,
                b"DoPlanetaryAnalysis\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    RandomContext_SeedRandom(SysGenRNG, (*pPlanetDesc).rand_seed);
    CalcSysInfo(SysInfoPtr);
    let mut radius: SIZE = 0;
    (*SysInfoPtr).PlanetInfo.PlanDataPtr = PlanData
        .offset(
            ((*pPlanetDesc).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed_3::PLANET_SHIELDED.0 as ::core::ffi::c_int)) as isize,
        );
    if (*pPlanetDesc).pPrevDesc == &raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC
    {
        radius = (*pPlanetDesc).radius;
    } else {
        radius = (*(*pPlanetDesc).pPrevDesc).radius;
    }
    (*SysInfoPtr).PlanetInfo.PlanetToSunDist = radius as DWORD;
    (*SysInfoPtr).PlanetInfo.SurfaceTemperature = CalcTemp(SysInfoPtr, radius);
    match ((*(*SysInfoPtr).PlanetInfo.PlanDataPtr).AtmoAndDensity as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int
    {
        0 => {
            (*SysInfoPtr).PlanetInfo.PlanetDensity = 20 as UWORD;
        }
        1 => {
            (*SysInfoPtr).PlanetInfo.PlanetDensity = 33 as UWORD;
        }
        2 => {
            (*SysInfoPtr).PlanetInfo.PlanetDensity = 60 as UWORD;
        }
        3 => {
            (*SysInfoPtr).PlanetInfo.PlanetDensity = 100 as UWORD;
        }
        4 => {
            (*SysInfoPtr).PlanetInfo.PlanetDensity = 150 as UWORD;
        }
        5 => {
            (*SysInfoPtr).PlanetInfo.PlanetDensity = 200 as UWORD;
        }
        _ => {}
    }
    (*SysInfoPtr).PlanetInfo.PlanetDensity = ((*SysInfoPtr).PlanetInfo.PlanetDensity
        as ::core::ffi::c_int
        + ((*SysInfoPtr).PlanetInfo.PlanetDensity as ::core::ffi::c_int
            / 20 as ::core::ffi::c_int
            - RandomContext_Random(SysGenRNG) as UWORD as ::core::ffi::c_int
                % ((*SysInfoPtr).PlanetInfo.PlanetDensity as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int))) as UWORD;
    match ((*(*SysInfoPtr).PlanetInfo.PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        0 => {
            (*SysInfoPtr).PlanetInfo.PlanetRadius = CalcHalfBaseVariance(25 as UWORD);
        }
        1 => {
            (*SysInfoPtr).PlanetInfo.PlanetRadius = CalcHalfBaseVariance(75 as UWORD);
        }
        2 => {
            (*SysInfoPtr).PlanetInfo.PlanetRadius = CalcFromBase(
                300 as UWORD,
                (1500 as ::core::ffi::c_int - 300 as ::core::ffi::c_int) as UWORD,
            );
        }
        _ => {}
    }
    (*SysInfoPtr).PlanetInfo.RotationPeriod = CalcRotation(
        &raw mut (*SysInfoPtr).PlanetInfo,
    ) as UWORD;
    (*SysInfoPtr).PlanetInfo.SurfaceGravity = CalcGravity(
        &raw mut (*SysInfoPtr).PlanetInfo,
    );
    (*SysInfoPtr).PlanetInfo.AxialTilt = CalcTilt();
    (*SysInfoPtr).PlanetInfo.Tectonics = CalcTectonics(
        (*(*SysInfoPtr).PlanetInfo.PlanDataPtr).BaseTectonics as UWORD,
        (*SysInfoPtr).PlanetInfo.SurfaceTemperature as UWORD,
    );
    if (*SysInfoPtr).PlanetInfo.Tectonics as ::core::ffi::c_int
        > 255 as ::core::ffi::c_int
    {
        (*SysInfoPtr).PlanetInfo.Tectonics = 255 as UWORD;
    }
    (*SysInfoPtr).PlanetInfo.AtmoDensity = GeneratePlanetComposition(
        &raw mut (*SysInfoPtr).PlanetInfo,
        (*SysInfoPtr).PlanetInfo.SurfaceTemperature,
        radius,
    );
    (*SysInfoPtr).PlanetInfo.Tectonics = ((*SysInfoPtr).PlanetInfo.Tectonics
        as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as UWORD;
    (*SysInfoPtr).PlanetInfo.Weather = ((*SysInfoPtr).PlanetInfo.Weather
        as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as UWORD;
    (*SysInfoPtr).PlanetInfo.LifeChance = CalcLifeChance(
        &raw mut (*SysInfoPtr).PlanetInfo,
    ) as SIZE;
}
