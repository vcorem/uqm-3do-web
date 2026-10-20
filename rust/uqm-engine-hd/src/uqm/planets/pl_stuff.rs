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

// Rust translation of src/uqm/planets/pl_stuff.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type font_desc;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn daysElapsed() -> ::core::ffi::c_float;
    fn GameClockTick();
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmod(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn XFormColorMap(ColorMapPtr: COLORMAPPTR, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut optWhichShield: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optTintPlanSphere: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optScanSphere: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut GlobData: GLOBDATA;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn TFB_Random() -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut PlanetContext: CONTEXT;
    static mut useDosSpheres: BOOLEAN;
    static mut use3DOSpheres: BOOLEAN;
    fn RenderDOSPlanetSphere(
        Orbit: *mut PLANET_ORBIT,
        MaskFrame: FRAME,
        offset: ::core::ffi::c_int,
    );
    fn Render3DOPlanetSphere(
        Orbit: *mut PLANET_ORBIT,
        MaskFrame: FRAME,
        offset: ::core::ffi::c_int,
        rotwidth_0: COUNT,
        height: COUNT,
    );
    fn RenderPlanetSphere(
        Orbit: *mut PLANET_ORBIT,
        Frame: FRAME,
        offset: ::core::ffi::c_int,
        shielded: BOOLEAN,
        doThrob: BOOLEAN,
        width: COUNT,
        height: COUNT,
        radius: COUNT,
    );
    fn SetShieldThrobEffect(
        FromFrame: FRAME,
        offset: ::core::ffi::c_int,
        ToFrame: FRAME,
    );
    fn Draw3DOShield(ShieldFrame: STAMP);
    static mut OffScreenContext: CONTEXT;
    fn RepairBackRect(pRect: *mut RECT);
    fn XFormColorMap_step() -> BOOLEAN;
    fn SetGraphicScale(scale: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetGraphicScaleMode(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type TimeCount = DWORD;
pub type Mutex = *mut ::core::ffi::c_void;
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
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type CONTEXT_DESC = context_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_desc {
    pub Type: DRAWABLE_TYPE,
    pub Index: UWORD,
    pub HotSpot: HOT_SPOT,
    pub Bounds: EXTENT,
    pub image: *mut TFB_Image,
    pub parent: *mut drawable_desc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawable_desc {
    pub Flags: CREATE_FLAGS,
    pub MaxIndex: UWORD,
    pub Frame: *mut FRAME_DESC,
}
pub type FRAME_DESC = frame_desc;
pub type CREATE_FLAGS = BYTE;
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_image {
    pub NormalImg: TFB_Canvas,
    pub ScaledImg: TFB_Canvas,
    pub MipmapImg: TFB_Canvas,
    pub FilledImg: TFB_Canvas,
    pub colormap_index: ::core::ffi::c_int,
    pub colormap_version: ::core::ffi::c_int,
    pub NormalHs: HOT_SPOT,
    pub MipmapHs: HOT_SPOT,
    pub last_scale_hs: HOT_SPOT,
    pub last_scale: ::core::ffi::c_int,
    pub last_scale_type: ::core::ffi::c_int,
    pub last_fill: Color,
    pub extent: EXTENT,
    pub mutex: Mutex,
    pub dirty: BOOLEAN,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type HOT_SPOT = POINT;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type DRAWABLE_TYPE = UWORD;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub type COLORMAPPTR = *mut ::core::ffi::c_void;
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
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_MULTIPLY: Self = Self(3);
    pub const DRAW_OVERLAY: Self = Self(4);
    pub const DRAW_SCREEN: Self = Self(5);
    pub const DRAW_GRAYSCALE: Self = Self(6);
    pub const DRAW_LINEARBURN: Self = Self(7);
    pub const DRAW_HYPTOQUAS: Self = Self(8);
    pub const DRAW_DESATURATE: Self = Self(9);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[inline]
unsafe extern "C" fn sameColor(mut c1: Color, mut c2: Color) -> bool {
    return c1.r as ::core::ffi::c_int == c2.r as ::core::ffi::c_int
        && c1.g as ::core::ffi::c_int == c2.g as ::core::ffi::c_int
        && c1.b as ::core::ffi::c_int == c2.b as ::core::ffi::c_int
        && c1.a as ::core::ffi::c_int == c2.a as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn colorFromRgb15(mut r: BYTE, mut g: BYTE, mut b: BYTE) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = ((r as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | r as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.g = ((g as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | g as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.b = ((b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.a = 0xff as BYTE;
    return c;
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
static mut rotFrameIndex: ::core::ffi::c_int = 0;
static mut rotDirection: ::core::ffi::c_int = 0;
static mut throbShield: bool = false;
static mut rotPointIndex: ::core::ffi::c_int = 0;
static mut rotwidth: ::core::ffi::c_int = 0;
static mut rotheight: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn DrawCurrentPlanetSphere() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut oldContext: CONTEXT = SetContext(PlanetContext);
    s.origin.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    s.origin.y = ((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 67 as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    rotFrameIndex ^= 1 as ::core::ffi::c_int;
    rotPointIndex -= rotDirection;
    if rotPointIndex < 0 as ::core::ffi::c_int {
        rotPointIndex = rotwidth - 1 as ::core::ffi::c_int;
    } else if rotPointIndex >= rotwidth {
        rotPointIndex = 0 as ::core::ffi::c_int;
    }
    if useDosSpheres.0 != 0 {
        if rotFrameIndex != 0 {
            (*Orbit).SphereFrame = IncFrameIndex((*Orbit).SphereFrame);
        }
        uqm_lua_checked!(RenderDOSPlanetSphere(Orbit, (*Orbit).SphereFrame, rotPointIndex), ());
    } else if use3DOSpheres.0 != 0 {
        (*Orbit).SphereFrame = SetAbsFrameIndex(
            (*Orbit).SphereFrame,
            rotFrameIndex as COUNT,
        );
        uqm_lua_checked!(Render3DOPlanetSphere(
            Orbit,
            (*Orbit).SphereFrame,
            rotPointIndex,
            rotwidth as COUNT,
            rotheight as COUNT,
        ), ());
    } else {
        (*Orbit).SphereFrame = SetAbsFrameIndex(
            (*Orbit).SphereFrame,
            rotFrameIndex as COUNT,
        );
        uqm_lua_checked!(RenderPlanetSphere(
            Orbit,
            (*Orbit).SphereFrame,
            rotPointIndex,
            BOOLEAN(
                ((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                    & C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ),
            BOOLEAN(throbShield as ::core::ffi::c_uint),
            rotwidth as COUNT,
            rotheight as COUNT,
            ((rotheight >> 1 as ::core::ffi::c_int)
                - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as COUNT,
        ), ());
    }
    BatchGraphics();
    s.frame = (*Orbit).SphereFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    if !(*Orbit).ObjectFrame.is_null() {
        s.frame = (*Orbit).ObjectFrame;
        if use3DOSpheres.0 != 0 {
            uqm_lua_checked!(Draw3DOShield(s), ());
        } else {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
    }
    UnbatchGraphics();
    SetContext(oldContext);
    uqm_lua_checked!(PrepareNextRotationFrame(), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawPlanetSphere(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut back: bool,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    BatchGraphics();
    if !(*Orbit).BackFrame.is_null() && back as ::core::ffi::c_int != 0 {
        s.origin.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor) as COORD;
        s.origin.y = ((((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
            - 67 as ::core::ffi::c_int
            - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - (if useDosSpheres.0 != 0 {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        s.frame = (*Orbit).BackFrame;
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    s.origin.x = x as COORD;
    s.origin.y = y as COORD;
    s.frame = (*Orbit).SphereFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    if !(*Orbit).ObjectFrame.is_null() {
        s.frame = (*Orbit).ObjectFrame;
        if use3DOSpheres.0 != 0 {
            uqm_lua_checked!(Draw3DOShield(s), ());
        } else {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
    }
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn DrawDefaultPlanetSphere() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    oldContext = SetContext(PlanetContext);
    uqm_lua_checked!(DrawPlanetSphere(
        (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor,
        (((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
            - 67 as ::core::ffi::c_int
            - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - (if useDosSpheres.0 != 0 {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor),
        BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
    ), ());
    SetContext(oldContext);
}
#[no_mangle]
pub unsafe extern "C" fn RerenderPlanetSphere() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    if optTintPlanSphere != 0x2 as ::core::ffi::c_int {
        return;
    }
    if useDosSpheres.0 != 0 {
        let mut sc: COUNT = 0;
        sc = (*Orbit).scanType;
        if sc as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
            sc = 0 as COUNT;
        } else {
            sc = sc.wrapping_add(1);
        }
        XFormColorMap(
            GetColorMapAddress(SetAbsStringTableIndex((*Orbit).sphereMap, sc)),
            0 as SIZE,
        );
        XFormColorMap_step();
    }
    uqm_lua_checked!(DrawCurrentPlanetSphere(), ());
}
#[no_mangle]
pub unsafe extern "C" fn InitSphereRotation(
    mut direction: ::core::ffi::c_int,
    mut shielded: BOOLEAN,
    mut width: COUNT,
    mut height: COUNT,
) {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    rotwidth = width as ::core::ffi::c_int;
    rotheight = height as ::core::ffi::c_int;
    rotDirection = direction;
    rotPointIndex = 0 as ::core::ffi::c_int;
    throbShield = shielded.0 != 0 && optWhichShield == 0x1 as ::core::ffi::c_int
        && !(useDosSpheres.0 != 0 || use3DOSpheres.0 != 0);
    if throbShield {
        (*Orbit).WorkFrame = (*Orbit).ObjectFrame;
        (*Orbit).ObjectFrame = CaptureDrawable(
            CreateDrawable(
                (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as CREATE_FLAGS as ::core::ffi::c_int) as CREATE_FLAGS,
                (*(*Orbit).ObjectFrame).Bounds.width,
                (*(*Orbit).ObjectFrame).Bounds.height,
                2 as COUNT,
            ),
        );
    }
    rotFrameIndex = if useDosSpheres.0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    uqm_lua_checked!(PrepareNextRotationFrame(), ());
}
#[no_mangle]
pub unsafe extern "C" fn UninitSphereRotation() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    if !(*Orbit).WorkFrame.is_null() {
        DestroyDrawable(ReleaseDrawable((*Orbit).ObjectFrame));
        (*Orbit).ObjectFrame = (*Orbit).WorkFrame;
        (*Orbit).WorkFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn PrepareNextRotationFrame() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    rotFrameIndex ^= 1 as ::core::ffi::c_int;
    rotPointIndex += rotDirection;
    if rotPointIndex < 0 as ::core::ffi::c_int {
        rotPointIndex = rotwidth - 1 as ::core::ffi::c_int;
    } else if rotPointIndex >= rotwidth {
        rotPointIndex = 0 as ::core::ffi::c_int;
    }
    if useDosSpheres.0 != 0 {
        if rotFrameIndex != 0 {
            (*Orbit).SphereFrame = IncFrameIndex((*Orbit).SphereFrame);
        } else {
            (*Orbit).SphereFrame = DecFrameIndex((*Orbit).SphereFrame);
        }
        uqm_lua_checked!(RenderDOSPlanetSphere(Orbit, (*Orbit).SphereFrame, rotPointIndex), ());
    } else if use3DOSpheres.0 != 0 {
        (*Orbit).SphereFrame = SetAbsFrameIndex(
            (*Orbit).SphereFrame,
            rotFrameIndex as COUNT,
        );
        uqm_lua_checked!(Render3DOPlanetSphere(
            Orbit,
            (*Orbit).SphereFrame,
            rotPointIndex,
            rotwidth as COUNT,
            rotheight as COUNT,
        ), ());
    } else {
        (*Orbit).SphereFrame = SetAbsFrameIndex(
            (*Orbit).SphereFrame,
            rotFrameIndex as COUNT,
        );
        uqm_lua_checked!(RenderPlanetSphere(
            Orbit,
            (*Orbit).SphereFrame,
            rotPointIndex,
            BOOLEAN(
                ((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                    & C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ),
            BOOLEAN(throbShield as ::core::ffi::c_uint),
            rotwidth as COUNT,
            rotheight as COUNT,
            ((rotheight >> 1 as ::core::ffi::c_int)
                - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as COUNT,
        ), ());
    }
    if throbShield {
        (*Orbit).ObjectFrame = SetAbsFrameIndex(
            (*Orbit).ObjectFrame,
            rotFrameIndex as COUNT,
        );
        uqm_lua_checked!(SetShieldThrobEffect((*Orbit).WorkFrame, rotPointIndex, (*Orbit).ObjectFrame), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn PrepareNextRotationFrameForIP(
    mut pPlanetDesc: *mut PLANET_DESC,
    mut frameCounter: SIZE,
) {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pPlanetDesc).orbit;
    let mut oldPointIndex: ::core::ffi::c_int = (*pPlanetDesc).rotPointIndex;
    if (*pPlanetDesc).radius as ::core::ffi::c_int
        > 4 as ::core::ffi::c_int
            * (*pSolarSysState).SunDesc[0usize].radius as ::core::ffi::c_int
    {
        return;
    }
    (*pPlanetDesc).rotPointIndex = fmod(
        (*pPlanetDesc).rot_speed * daysElapsed() as ::core::ffi::c_double,
        (*pPlanetDesc).rotwidth as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    if (*pPlanetDesc).rotPointIndex < 0 as ::core::ffi::c_int {
        (*pPlanetDesc).rotPointIndex += (*pPlanetDesc).rotwidth;
    }
    if (*pPlanetDesc).rotPointIndex == oldPointIndex {
        return;
    }
    (*pPlanetDesc).rotFrameIndex ^= 1 as ::core::ffi::c_int;
    (*Orbit).SphereFrame = SetAbsFrameIndex(
        (*Orbit).SphereFrame,
        (*pPlanetDesc).rotFrameIndex as COUNT,
    );
    uqm_lua_checked!(RenderPlanetSphere(
        Orbit,
        (*Orbit).SphereFrame,
        (*pPlanetDesc).rotPointIndex,
        BOOLEAN(
            ((*pPlanetDesc).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ),
        BOOLEAN::FALSE,
        (*pPlanetDesc).rotwidth as COUNT,
        (*pPlanetDesc).rotheight as COUNT,
        (((*pPlanetDesc).rotheight >> 1 as ::core::ffi::c_int)
            - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                0 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as COUNT,
    ), ());
    (*(*(*Orbit).SphereFrame).image).dirty = BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ZoomInPlanetSphere() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let base: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut oldScale: ::core::ffi::c_int = 0;
    let mut oldMode: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut frameCount: ::core::ffi::c_int = 0;
    let mut zoomCorner: ::core::ffi::c_int = 0;
    let mut frameRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut repairRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut NextTime: TimeCount = 0;
    let mut Now: TimeCount = 0;
    let mut RenderNextTime: TimeCount = 0;
    frameCount = 840 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
        / 5 as ::core::ffi::c_int
        / (840 as ::core::ffi::c_int / ((35 as ::core::ffi::c_int) << resolutionFactor));
    zoomCorner = TFB_Random() as ::core::ffi::c_int;
    dx = 1 as ::core::ffi::c_int
        - (zoomCorner & 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int;
    dy = 1 as ::core::ffi::c_int - (zoomCorner & 2 as ::core::ffi::c_int);
    if !(*Orbit).ObjectFrame.is_null() {
        GetFrameRect((*Orbit).ObjectFrame, &raw mut frameRect);
    } else {
        GetFrameRect((*Orbit).SphereFrame, &raw mut frameRect);
    }
    repairRect = frameRect;
    RenderNextTime = 0 as TimeCount;
    i = 0 as ::core::ffi::c_int;
    while i <= frameCount {
        let mut scale: ::core::ffi::c_double = 0.;
        let mut pt: POINT = POINT { x: 0, y: 0 };
        Now = GetTimeCounter();
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int
                    / ((35 as ::core::ffi::c_int) << resolutionFactor)) as TimeCount,
            );
        if i < frameCount {
            scale = 1.134f64
                - exp(
                    -2.0f64 * i as ::core::ffi::c_double
                        / frameCount as ::core::ffi::c_double,
                );
        } else {
            scale = 1.0f64;
        }
        pt.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor)
            + (dx as ::core::ffi::c_double * (1.0f64 - scale)
                * (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor)) >> resolutionFactor)
                    * 6 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                    << resolutionFactor) as ::core::ffi::c_double + 0.5f64)
                as ::core::ffi::c_int) as COORD;
        pt.y = ((((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
            - 67 as ::core::ffi::c_int
            - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - (if useDosSpheres.0 != 0 {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)
            + (dy as ::core::ffi::c_double * (1.0f64 - scale)
                * (((CanvasHeight
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
                    - ((13 as ::core::ffi::c_int) << resolutionFactor)
                    >> resolutionFactor) - 67 as ::core::ffi::c_int
                    - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
                    << resolutionFactor) * 6 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as ::core::ffi::c_double + 0.5f64)
                as ::core::ffi::c_int) as COORD;
        SetContext(PlanetContext);
        BatchGraphics();
        if i > 0 as ::core::ffi::c_int {
            RepairBackRect(&raw mut repairRect);
        }
        oldMode = SetGraphicScaleMode(
            C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int,
        );
        oldScale = SetGraphicScale(
            (base as ::core::ffi::c_double * scale + 0.5f64) as ::core::ffi::c_int,
        );
        uqm_lua_checked!(DrawPlanetSphere(
            pt.x as ::core::ffi::c_int,
            pt.y as ::core::ffi::c_int,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
        ), ());
        SetGraphicScale(oldScale);
        SetGraphicScaleMode(oldMode);
        UnbatchGraphics();
        repairRect.corner.x = (pt.x as ::core::ffi::c_int
            + frameRect.corner.x as ::core::ffi::c_int) as COORD;
        repairRect.corner.y = (pt.y as ::core::ffi::c_int
            + frameRect.corner.y as ::core::ffi::c_int) as COORD;
        if Now >= RenderNextTime {
            RenderNextTime = Now
                .wrapping_add(
                    (if optScanSphere == 0 {
                        840 as ::core::ffi::c_int
                            / ((12 as ::core::ffi::c_int) << resolutionFactor)
                    } else if optScanSphere == 1 as ::core::ffi::c_int {
                        840 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                            / ((210 as ::core::ffi::c_int) << resolutionFactor)
                    } else {
                        840 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                            / ((243 as ::core::ffi::c_int) << resolutionFactor)
                    }) as TimeCount,
                );
            uqm_lua_checked!(PrepareNextRotationFrame(), ());
        }
        uqm_lua_checked!(SleepThreadUntil(NextTime), ());
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RotatePlanetSphere(
    mut keepRate: BOOLEAN,
    mut onTop: *mut STAMP,
) {
    static mut NextTime: TimeCount = 0;
    static mut OutNextTime: TimeCount = 0;
    static mut TimeOutClock: TimeCount = 0;
    let mut Now: TimeCount = GetTimeCounter();
    if keepRate.0 != 0 && Now < NextTime {
        return;
    }
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0 && Now >= TimeOutClock
    {
        uqm_lua_checked!(GameClockTick(), ());
        TimeOutClock = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as TimeCount,
            );
    }
    if Now >= NextTime {
        NextTime = Now
            .wrapping_add(
                (if optScanSphere == 0 {
                    840 as ::core::ffi::c_int
                        / ((12 as ::core::ffi::c_int) << resolutionFactor)
                } else if optScanSphere == 1 as ::core::ffi::c_int {
                    840 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                        / ((210 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    840 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        / ((243 as ::core::ffi::c_int) << resolutionFactor)
                }) as TimeCount,
            );
        if Now >= OutNextTime {
            OutNextTime = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
                );
            uqm_lua_checked!(DrawDefaultPlanetSphere(), ());
            if !onTop.is_null() {
                uqm_lua_checked!(DrawStamp(onTop), ());
            }
        }
        uqm_lua_checked!(PrepareNextRotationFrame(), ());
    }
}
unsafe extern "C" fn renderTintFrame(mut tintColor: Color) {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut is_red: ::core::ffi::c_double = 0.;
    if sameColor(colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE), tintColor) {
        is_red = 0.75f64;
    } else {
        is_red = 0.5f64;
    }
    oldContext = SetContext(OffScreenContext);
    SetContextFGFrame((*Orbit).TintFrame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut r);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = (*pSolarSysState).TopoFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    mode = MAKE_DRAW_MODE(
        DrawKind::DRAW_ADDITIVE,
        (0xff as ::core::ffi::c_int as ::core::ffi::c_double * is_red) as SWORD,
    );
    oldMode = SetContextDrawMode(mode);
    SetContextForeGroundColor(tintColor);
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextDrawMode(oldMode);
    SetContext(oldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawPCScanTint(mut scan: COUNT) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = (*pSolarSysState).ScanFrame[scan as usize];
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawPlanet(
    mut tintY: ::core::ffi::c_int,
    mut tintColor: Color,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    BatchGraphics();
    if sameColor(tintColor, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)) {
        s.frame = (*pSolarSysState).TopoFrame;
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    } else {
        let mut tintFrame: FRAME = (*Orbit).TintFrame;
        let mut height: ::core::ffi::c_int = (*tintFrame).Bounds.height
            as ::core::ffi::c_int;
        if !sameColor(tintColor, (*Orbit).TintColor) {
            uqm_lua_checked!(renderTintFrame(tintColor), ());
            (*Orbit).TintColor = tintColor;
        }
        if tintY < height - 1 as ::core::ffi::c_int {
            s.frame = (*pSolarSysState).TopoFrame;
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        if tintY >= 0 as ::core::ffi::c_int {
            let mut oldClipRect: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut clipRect: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut edge: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut i: COUNT = 0;
            GetContextClipRect(&raw mut oldClipRect);
            clipRect = oldClipRect;
            clipRect.extent.height = (tintY + 1 as ::core::ffi::c_int) as COORD;
            SetContextClipRect(&raw mut clipRect);
            s.frame = tintFrame;
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            SetContextClipRect(&raw mut oldClipRect);
            edge.extent.height = 1 as COORD;
            edge.extent.width = (if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor) as COORD;
            edge.corner.x = (*tintFrame).HotSpot.x;
            edge.corner.y = clipRect.extent.height;
            i = 0x90 as COUNT;
            while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                tintColor.a = i as BYTE;
                SetContextForeGroundColor(tintColor);
                uqm_lua_checked!(DrawFilledRectangle(&raw mut edge), ());
                edge.corner.y -= 1;
                i = (i as ::core::ffi::c_int
                    - 0x90 as ::core::ffi::c_int
                        / ((3 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
            }
            tintColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
            edge.corner.y = (clipRect.extent.height as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COORD;
            i = 0x40 as COUNT;
            while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                tintColor.a = i as BYTE;
                SetContextForeGroundColor(tintColor);
                uqm_lua_checked!(DrawFilledRectangle(&raw mut edge), ());
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    break;
                }
                edge.corner.y += 1;
                i = (i as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as COUNT;
            }
        }
    }
    UnbatchGraphics();
}
