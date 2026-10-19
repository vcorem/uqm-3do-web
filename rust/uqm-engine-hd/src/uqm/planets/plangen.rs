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

// BW 2011: fixed using pixmaps to define planet surfaces so it actually works
// on planets you can land on. The second frame of planetmask.ani has to be a
// (Black & White) indexed pic with 128 colors top. Lesser indices (black)
// will correspond to lower altitudes.

// Rust translation of src/uqm/planets/plangen.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type font_desc;
    pub type RandomContext;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn acos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan2(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn MAKE_HOT_SPOT(_: COORD, _: COORD) -> HOT_SPOT;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn DrawBatch(
        pBasePrim: *mut PRIMITIVE,
        PrimLinks: PRIM_LINKS,
        BatchFlags: BATCH_FLAGS,
    );
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CopyFrameRect(Frame: FRAME, area: *const RECT) -> DRAWABLE;
    fn CloneFrame(Frame: FRAME) -> DRAWABLE;
    fn RotateFrame(Frame: FRAME, angle_deg: ::core::ffi::c_int) -> DRAWABLE;
    fn RescaleFrame(
        frame: FRAME,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> DRAWABLE;
    fn ReadFramePixelColors(
        frame: FRAME,
        pixels: *mut Color,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn WriteFramePixelColors(
        frame: FRAME,
        pixels: *const Color,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn ReadFramePixelIndexes(
        frame: FRAME,
        pixels: *mut BYTE,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        paletted: BOOLEAN,
    ) -> BOOLEAN;
    fn WriteFramePixelIndexes(
        frame: FRAME,
        pixels: *const BYTE,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optNebulae: OPT_ENABLABLE;
    static mut optTintPlanSphere: ::core::ffi::c_int;
    static mut optScanStyle: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optPlanetTexture: OPT_ENABLABLE;
    static mut optScanSphere: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut useDosSpheres: BOOLEAN;
    static mut use3DOSpheres: BOOLEAN;
    static mut SysGenRNG: *mut RandomContext;
    fn playerInPlanetOrbit() -> bool;
    fn worldIsPlanet(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> bool;
    fn planetIndex(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> COUNT;
    fn GetStarBackFround() -> FRAME;
    fn InitSphereRotation(
        direction: ::core::ffi::c_int,
        shielded: BOOLEAN,
        width: COUNT,
        height: COUNT,
    );
    fn DeltaTopography(
        num_iterations: COUNT,
        DepthArray: *mut SBYTE,
        pRect: *mut RECT,
        depth_delta: SIZE,
    );
    static mut PlanData: *const PlanetFrame;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn SetColorMapColors(
        colors: *mut Color,
        ColorMapPtr: COLORMAPPTR,
        from: COUNT,
        to: COUNT,
    );
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut OffScreenContext: CONTEXT;
    static mut solTexturesPresent: BOOLEAN;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UBYTE = uint8;
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
pub type Mutex = *mut ::core::ffi::c_void;
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
pub type BATCH_FLAGS = BYTE;
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
pub struct XLAT_DESC {
    pub level_tab: [SIZE; 3],
    pub xlat_tab: [BYTE; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT3 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub z: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elev_block_t {
    pub min: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub avg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_def_t {
    pub x0: ::core::ffi::c_int,
    pub y0: ::core::ffi::c_int,
    pub x1: ::core::ffi::c_int,
    pub y1: ::core::ffi::c_int,
    pub dx: ::core::ffi::c_int,
    pub dy: ::core::ffi::c_int,
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
unsafe extern "C" fn buildColorRgba(
    mut r: BYTE,
    mut g: BYTE,
    mut b: BYTE,
    mut a: BYTE,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = r;
    c.g = g;
    c.b = b;
    c.a = a;
    return c;
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
#[inline]
unsafe extern "C" fn SetPrimNextLink(mut pPrim: *mut PRIMITIVE, mut Link: COUNT) {
    (*pPrim).Links = ((Link as DWORD) << 16 as ::core::ffi::c_int
        | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
}
static mut tintColors: [Color; 3] = [
    Color {
        r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        a: 0xff as BYTE,
    },
    Color {
        r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        a: 0xff as BYTE,
    },
    Color {
        r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        a: 0xff as BYTE,
    },
];
#[no_mangle]
pub unsafe extern "C" fn clip_channel(mut c: ::core::ffi::c_int) -> BYTE {
    if c > 255 as ::core::ffi::c_int {
        c = 255 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
    }
    return c as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn TransformColor(mut c: *mut Color, mut scan: COUNT) {
    if scan as ::core::ffi::c_int
        == PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
    {
        return;
    }
    (*c).b = (((*c).r as ::core::ffi::c_int + (*c).g as ::core::ffi::c_int
        + (*c).b as ::core::ffi::c_int) / 3 as ::core::ffi::c_int) as BYTE;
    (*c).g = (*c).b;
    (*c).r = (*c).g;
    match scan as ::core::ffi::c_int {
        0 => {
            (*c).b = 0 as BYTE;
            (*c).g = 0 as BYTE;
        }
        1 => {
            (*c).r = ((*c).r as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as BYTE;
            (*c).b = (*c).r;
            (*c).g = (*c).r;
        }
        2 => {
            (*c).r = 0 as BYTE;
            (*c).b = 0 as BYTE;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ColorDelta(mut scan: COUNT, mut avg: DWORD) -> SBYTE {
    if scan as ::core::ffi::c_int
        == PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
    {
        return 0 as SBYTE
    } else {
        let mut diff: SBYTE = 0 as SBYTE;
        if avg
            > (if scan as ::core::ffi::c_int
                == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
            {
                90 as ::core::ffi::c_int
            } else {
                135 as ::core::ffi::c_int
            }) as DWORD
        {
            diff = -(avg
                .wrapping_sub(
                    (if scan as ::core::ffi::c_int
                        == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
                    {
                        90 as ::core::ffi::c_int
                    } else {
                        135 as ::core::ffi::c_int
                    }) as DWORD,
                ) as SBYTE as ::core::ffi::c_int) as SBYTE;
        } else if avg
            < (if scan as ::core::ffi::c_int
                == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
            {
                50 as ::core::ffi::c_int
            } else {
                55 as ::core::ffi::c_int
            }) as DWORD
        {
            diff = ((if scan as ::core::ffi::c_int
                == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
            {
                50 as ::core::ffi::c_int
            } else {
                55 as ::core::ffi::c_int
            }) as DWORD)
                .wrapping_sub(avg) as SBYTE;
        }
        return diff;
    };
}
#[no_mangle]
pub unsafe extern "C" fn RepairColorOpacity(
    mut TopoColors: *mut Color,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut c: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    c = TopoColors;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if (*c).a as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
                (*c).a = 0xff as BYTE;
            }
            x += 1;
            c = c.offset(1);
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetPlanetColors(mut cmap: COLORMAPPTR) {
    let mut cbase: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut ctab: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut colors: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut c: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut i: COUNT = 0;
    let numcolors: COUNT = 128 as COUNT;
    colors = HMalloc(
        (::core::mem::size_of::<Color>() as size_t).wrapping_mul(numcolors as size_t),
    ) as *mut Color;
    c = colors;
    cbase = GetColorMapAddress((*pSolarSysState).OrbitalCMap) as *mut BYTE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numcolors as ::core::ffi::c_int {
        ctab = cbase
            .offset(2 as ::core::ffi::c_int as isize)
            .offset((i as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize);
        *c = buildColorRgba(
            *ctab.offset(0isize),
            *ctab.offset(1isize),
            *ctab.offset(2isize),
            0xff as BYTE,
        );
        i = i.wrapping_add(1);
        c = c.offset(1);
    }
    SetColorMapColors(colors, cmap, 0 as COUNT, numcolors);
    HFree(colors as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn AdjustColor(
    mut c: *mut Color,
    mut scan: COUNT,
    mut height: COUNT,
    mut width: COUNT,
    mut diff: SBYTE,
) {
    let mut y: COUNT = 0;
    let mut x: COUNT = 0;
    y = 0 as COUNT;
    while (y as ::core::ffi::c_int) < height as ::core::ffi::c_int {
        x = 0 as COUNT;
        while (x as ::core::ffi::c_int) < width as ::core::ffi::c_int {
            match scan as ::core::ffi::c_int {
                0 => {
                    (*c).r = ((*c).r as ::core::ffi::c_int + diff as ::core::ffi::c_int)
                        as BYTE;
                }
                1 => {
                    (*c).r = ((*c).r as ::core::ffi::c_int + diff as ::core::ffi::c_int)
                        as BYTE;
                    (*c).g = ((*c).g as ::core::ffi::c_int + diff as ::core::ffi::c_int)
                        as BYTE;
                    (*c).b = ((*c).b as ::core::ffi::c_int + diff as ::core::ffi::c_int)
                        as BYTE;
                }
                2 => {
                    (*c).g = ((*c).g as ::core::ffi::c_int + diff as ::core::ffi::c_int)
                        as BYTE;
                }
                _ => {}
            }
            x = x.wrapping_add(1);
            c = c.offset(1);
        }
        y = y.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetAltColorMap(mut pPlanetDesc: *mut PLANET_DESC) -> RESOURCE {
    match (*pPlanetDesc).data_index as ::core::ffi::c_int {
        6 => return b"planet.mercury.colortable\0".as_ptr() as RESOURCE,
        38 => return b"planet.venus.colortable\0".as_ptr() as RESOURCE,
        23 => return b"planet.mars.colortable\0".as_ptr() as RESOURCE,
        22 => {
            if worldIsPlanet(pSolarSysState, pPlanetDesc) {
                return b"planet.pluto.colortable\0".as_ptr() as RESOURCE
            } else {
                return b"planet.callisto.colortable\0".as_ptr() as RESOURCE
            }
        }
        27 => {
            if planetIndex(pSolarSysState, pPlanetDesc) as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
            {
                return b"planet.luna.colortable\0".as_ptr() as RESOURCE
            } else {
                return b"planet.charon.colortable\0".as_ptr() as RESOURCE
            }
        }
        7 => return b"planet.io.colortable\0".as_ptr() as RESOURCE,
        12 => return b"planet.europa.colortable\0".as_ptr() as RESOURCE,
        9 => return b"planet.ganymede.colortable\0".as_ptr() as RESOURCE,
        11 => return b"planet.titan.colortable\0".as_ptr() as RESOURCE,
        46 => return b"planet.triton.colortable\0".as_ptr() as RESOURCE,
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn ExpandLevelMasks(mut Orbit: *mut PLANET_ORBIT) {
    let mut spherespanx: COUNT = 0;
    let mut width: SIZE = 0;
    let mut height: SIZE = 0;
    let mut colors: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut y: DWORD = 0;
    let mut colorSize: DWORD = 0;
    let mut halfBound: DWORD = 0;
    let mut PlanetInfo: *mut PLANET_INFO = ::core::ptr::null_mut::<PLANET_INFO>();
    PlanetInfo = &raw mut (*pSolarSysState).SysInfo.PlanetInfo;
    spherespanx = (*(*Orbit).SphereFrame).Bounds.width as COUNT;
    width = (*(*Orbit).TopoMask).Bounds.width as SIZE;
    height = (*(*Orbit).TopoMask).Bounds.height as SIZE;
    halfBound = (width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int)
        as DWORD;
    colorSize = (height as DWORD).wrapping_mul(halfBound);
    colors = HMalloc(
        (::core::mem::size_of::<Color>() as size_t).wrapping_mul(colorSize as size_t),
    ) as *mut Color;
    uqm_lua_checked!(ReadFramePixelColors(
        (*Orbit).TopoMask,
        colors,
        halfBound as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
    ), ());
    DestroyDrawable(ReleaseDrawable((*Orbit).TopoMask));
    (*Orbit).TopoMask = ::core::ptr::null_mut::<FRAME_DESC>();
    y = 0 as DWORD;
    while y < colorSize {
        memcpy(
            colors.offset(y as isize).offset(width as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            colors.offset(y as isize) as *const ::core::ffi::c_void,
            (spherespanx as size_t)
                .wrapping_mul(::core::mem::size_of::<Color>() as size_t),
        );
        y = y.wrapping_add(halfBound);
    }
    if (*PlanetInfo).AxialTilt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*Orbit).TopoMask = CaptureDrawable(
            CreateDrawable(
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
                halfBound as SIZE,
                height,
                1 as COUNT,
            ),
        );
        uqm_lua_checked!(WriteFramePixelColors(
            (*Orbit).TopoMask,
            colors,
            halfBound as ::core::ffi::c_int,
            height as ::core::ffi::c_int,
        ), ());
    } else {
        let mut dupe: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut err: ::core::ffi::c_double = 0.;
        err = sin(
            3.14159265358979323846f64 * 2.0f64 / 360.0f64
                * (*PlanetInfo).AxialTilt as ::core::ffi::c_double,
        )
            * (spherespanx as ::core::ffi::c_int - height as ::core::ffi::c_int)
                as ::core::ffi::c_double;
        if err < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            err = -err;
        }
        err = ceil(err);
        dupe = CaptureDrawable(
            CreateDrawable(
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
                halfBound as SIZE,
                height,
                1 as COUNT,
            ),
        );
        uqm_lua_checked!(WriteFramePixelColors(
            dupe,
            colors,
            halfBound as ::core::ffi::c_int,
            height as ::core::ffi::c_int,
        ), ());
        (*Orbit).TopoMask = CaptureDrawable(
            RescaleFrame(
                dupe,
                halfBound as ::core::ffi::c_int,
                height as ::core::ffi::c_int + err as SIZE as ::core::ffi::c_int,
            ),
        );
        DestroyDrawable(ReleaseDrawable(dupe));
    }
    HFree(colors as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn RenderLevelMasks(
    mut mask: FRAME,
    mut pTopoData: *mut SBYTE,
    mut SurfDef: BOOLEAN,
) {
    let mut OldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut i: COUNT = 0;
    let mut AlgoType: BYTE = 0;
    let mut base: SIZE = 0;
    let mut d: SIZE = 0;
    let mut xlatDesc: *const XLAT_DESC = ::core::ptr::null::<XLAT_DESC>();
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut PlanDataPtr: *const PlanetFrame = ::core::ptr::null::<PlanetFrame>();
    let mut BatchArray: [PRIMITIVE; 64] = [PRIMITIVE {
        Links: 0,
        Type: 0,
        color: Color { r: 0, g: 0, b: 0, a: 0 },
        Object: PRIM_DESC {
            Point: POINT { x: 0, y: 0 },
        },
        flags: 0,
    }; 64];
    let mut pBatch: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    let mut pSrc: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut oldOrigin: POINT = POINT { x: 0, y: 0 };
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut w: SIZE = 0;
    let mut h: SIZE = 0;
    let mut num_frames: SIZE = 3 as SIZE;
    let mut level_tab: *const SIZE = ::core::ptr::null::<SIZE>();
    OldFrame = SetContextFGFrame(mask);
    oldOrigin = SetContextOrigin(MAKE_POINT(0 as COORD, 0 as COORD));
    GetContextClipRect(&raw mut ClipRect);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    w = (*mask).Bounds.width as SIZE;
    h = (*mask).Bounds.height as SIZE;
    pBatch = (&raw mut BatchArray as *mut PRIMITIVE).offset(0isize);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
        SetPrimNextLink(
            pBatch,
            (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
        );
        (*pBatch).Type = gfx_object::POINT_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*pBatch).flags = 0 as BYTE;
        i = i.wrapping_add(1);
        pBatch = pBatch.offset(1);
    }
    SetPrimNextLink(pBatch.offset(-1isize), 0xffff as ::core::ffi::c_int as COUNT);
    PlanDataPtr = PlanData
        .offset(
            ((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)) as isize,
        );
    AlgoType = ((*PlanDataPtr).Type as ::core::ffi::c_int & 0xc as ::core::ffi::c_int)
        as BYTE;
    if SurfDef.0 != 0 {
        base = 256 as SIZE;
    } else {
        base = (*PlanDataPtr).base_elevation;
    }
    xlatDesc = (*pSolarSysState).XlatPtr as *const XLAT_DESC;
    level_tab = &raw const (*xlatDesc).level_tab as *const SIZE;
    while num_frames as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        num_frames -= 1;
        i = 64 as COUNT;
        pBatch = (&raw mut BatchArray as *mut PRIMITIVE).offset(i as isize);
        pSrc = pTopoData;
        pt.y = 0 as COORD;
        while (pt.y as ::core::ffi::c_int) < h as ::core::ffi::c_int {
            pt.x = 0 as COORD;
            while (pt.x as ::core::ffi::c_int) < w as ::core::ffi::c_int {
                let c2rust_fresh6 = pSrc;
                pSrc = pSrc.offset(1);
                d = *c2rust_fresh6 as SIZE;
                d = (d as ::core::ffi::c_int + base as ::core::ffi::c_int) as SIZE;
                if AlgoType as ::core::ffi::c_int
                    == (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                {
                    d = (d as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as SIZE;
                } else if (d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    d = 0 as SIZE;
                } else if d as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
                    d = 255 as SIZE;
                }
                if d as ::core::ffi::c_int
                    >= *level_tab.offset(num_frames as isize) as ::core::ffi::c_int
                    && (num_frames as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        || (d as ::core::ffi::c_int)
                            < *level_tab
                                .offset(
                                    (num_frames as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int)
                {
                    pBatch = pBatch.offset(-1);
                    (*pBatch).Object.Point.x = pt.x;
                    (*pBatch).Object.Point.y = pt.y;
                    (*pBatch).color = buildColorRgba(
                        (0x1 as ::core::ffi::c_int
                            * (num_frames as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)) as BYTE,
                        0 as BYTE,
                        0 as BYTE,
                        0xff as BYTE,
                    );
                    i = i.wrapping_sub(1);
                    if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        uqm_lua_checked!(DrawBatch(
                            &raw mut BatchArray as *mut PRIMITIVE,
                            0 as PRIM_LINKS,
                            0 as BATCH_FLAGS,
                        ), ());
                        i = 64 as COUNT;
                        pBatch = (&raw mut BatchArray as *mut PRIMITIVE)
                            .offset(i as isize);
                    }
                }
                pt.x += 1;
            }
            pt.y += 1;
        }
        if (i as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawBatch(
                &raw mut BatchArray as *mut PRIMITIVE,
                i as PRIM_LINKS,
                0 as BATCH_FLAGS,
            ), ());
        }
    }
    SetContextClipRect(&raw mut ClipRect);
    SetContextOrigin(oldOrigin);
    SetContextFGFrame(OldFrame);
}
unsafe extern "C" fn RenderTopography(
    mut DstFrame: FRAME,
    mut pTopoData: *mut SBYTE,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut SurfDef: BOOLEAN,
    mut scanTable: COLORMAP,
) {
    let mut AlgoType: BYTE = 0;
    let mut base: SIZE = 0;
    let mut d: SIZE = 0;
    let mut pSrc: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut ctab: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut cbase: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut pix: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut map: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut i: DWORD = 0;
    let mut sqr: DWORD = 0;
    let mut PlanDataPtr: *const PlanetFrame = ::core::ptr::null::<PlanetFrame>();
    let mut xlatDesc: *const XLAT_DESC = ::core::ptr::null::<XLAT_DESC>();
    let mut xlat_tab: *const BYTE = ::core::ptr::null::<BYTE>();
    if (*pSolarSysState).XlatRef.is_null() {
        log_add(
            log_Level::log_Warning,
            b"No xlat table -- could not generate surface.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    PlanDataPtr = PlanData
        .offset(
            ((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)) as isize,
        );
    AlgoType = ((*PlanDataPtr).Type as ::core::ffi::c_int & 0xc as ::core::ffi::c_int)
        as BYTE;
    if SurfDef.0 != 0 {
        base = 256 as SIZE;
    } else {
        base = (*PlanDataPtr).base_elevation;
    }
    xlatDesc = (*pSolarSysState).XlatPtr as *const XLAT_DESC;
    xlat_tab = &raw const (*xlatDesc).xlat_tab as *const BYTE;
    if scanTable.is_null() {
        cbase = GetColorMapAddress((*pSolarSysState).OrbitalCMap) as *mut BYTE;
    } else {
        cbase = GetColorMapAddress(scanTable) as *mut BYTE;
    }
    pSrc = pTopoData;
    sqr = (w * h) as DWORD;
    map = HMalloc(
        (::core::mem::size_of::<Color>() as size_t).wrapping_mul(sqr as size_t),
    ) as *mut Color;
    pix = map;
    i = 0 as DWORD;
    while i < sqr {
        d = *pSrc as SIZE;
        if AlgoType as ::core::ffi::c_int
            == (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        {
            if optScanSphere == 1 as ::core::ffi::c_int {
                d = (d as ::core::ffi::c_int + base as ::core::ffi::c_int) as SIZE;
            }
            d = (d as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as SIZE;
        } else {
            d = (d as ::core::ffi::c_int + base as ::core::ffi::c_int) as SIZE;
            if (d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                d = 0 as SIZE;
            } else if d as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
                d = 255 as SIZE;
            }
        }
        d = (*xlat_tab.offset(d as isize) as ::core::ffi::c_int
            - *cbase.offset(0isize) as ::core::ffi::c_int) as SIZE;
        ctab = cbase
            .offset(2 as ::core::ffi::c_int as isize)
            .offset((d as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize);
        *pix = buildColorRgba(
            *ctab.offset(0isize),
            *ctab.offset(1isize),
            *ctab.offset(2isize),
            0xff as BYTE,
        );
        i = i.wrapping_add(1);
        pSrc = pSrc.offset(1);
        pix = pix.offset(1);
    }
    uqm_lua_checked!(WriteFramePixelColors(DstFrame, map, w, h), ());
    HFree(map as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn P3mult(
    mut res: *mut POINT3,
    mut vec: *mut POINT3,
    mut cnst: ::core::ffi::c_double,
) {
    (*res).x = (*vec).x * cnst;
    (*res).y = (*vec).y * cnst;
    (*res).z = (*vec).z * cnst;
}
#[inline]
unsafe extern "C" fn P3dot(
    mut v1: *mut POINT3,
    mut v2: *mut POINT3,
) -> ::core::ffi::c_double {
    return (*v1).x * (*v2).x + (*v1).y * (*v2).y + (*v1).z * (*v2).z;
}
#[inline]
unsafe extern "C" fn P3norm(mut res: *mut POINT3, mut vec: *mut POINT3) {
    let mut mag: ::core::ffi::c_double = sqrt(P3dot(vec, vec));
    P3mult(res, vec, 1 as ::core::ffi::c_int as ::core::ffi::c_double / mag);
}
unsafe extern "C" fn GenerateSphereMask(mut loc: POINT, mut radius: COUNT) {
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut light: POINT3 = POINT3 { x: 0., y: 0., z: 0. };
    let mut lrad: ::core::ffi::c_double = 0.;
    let step: DWORD = ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as DWORD;
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut tworadius: COUNT = ((radius as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as COUNT;
    let mut radius_thres: COUNT = ((radius as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int)
        * (radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COUNT;
    let mut radius_2: COUNT = (radius as ::core::ffi::c_int
        * radius as ::core::ffi::c_int) as COUNT;
    lrad = sqrt(
        (loc.x as ::core::ffi::c_int * loc.x as ::core::ffi::c_int
            + loc.y as ::core::ffi::c_int * loc.y as ::core::ffi::c_int)
            as ::core::ffi::c_double,
    );
    light.x = -(loc.x as ::core::ffi::c_double);
    light.y = -(loc.y as ::core::ffi::c_double);
    light.z = 1.2f64 * lrad;
    P3norm(&raw mut light, &raw mut light);
    pt.y = 0 as COORD;
    y = -(radius as ::core::ffi::c_int);
    while pt.y as ::core::ffi::c_int <= tworadius as ::core::ffi::c_int {
        let mut y_2: DWORD = (y * y) as DWORD;
        pt.x = 0 as COORD;
        x = -(radius as ::core::ffi::c_int);
        while pt.x as ::core::ffi::c_int <= tworadius as ::core::ffi::c_int {
            let mut x_2: DWORD = (x * x) as DWORD;
            let mut rad_2: DWORD = x_2.wrapping_add(y_2);
            let mut diff_int: DWORD = 0 as DWORD;
            let mut norm: POINT3 = POINT3 { x: 0., y: 0., z: 0. };
            let mut diff: ::core::ffi::c_double = 0.;
            if rad_2 < radius_thres as DWORD {
                norm.x = x as ::core::ffi::c_double;
                norm.y = y as ::core::ffi::c_double;
                norm.z = sqrt(
                    (radius_2 as DWORD).wrapping_sub(x_2) as ::core::ffi::c_double,
                ) * sqrt((radius_2 as DWORD).wrapping_sub(y_2) as ::core::ffi::c_double)
                    / radius as ::core::ffi::c_int as ::core::ffi::c_double;
                P3norm(&raw mut norm, &raw mut norm);
                diff = P3dot(&raw mut norm, &raw mut light);
                if diff < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    diff = 0.0f64;
                }
                if diff < 0.1f64 {
                    diff = 0.1f64;
                }
                if rad_2 > radius_2 as DWORD {
                    diff
                        *= 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            - (sqrt(rad_2 as ::core::ffi::c_double)
                                - radius as ::core::ffi::c_int as ::core::ffi::c_double);
                    if diff < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        diff = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    }
                }
                diff_int = (diff * step as ::core::ffi::c_double) as DWORD;
            }
            *(*(*pSolarSysState).Orbit.light_diff.offset(pt.y as isize))
                .offset(pt.x as isize) = diff_int;
            pt.x += 1;
            x += 1;
        }
        pt.y += 1;
        y += 1;
    }
}
unsafe extern "C" fn create_aa_points(
    mut ppt: *mut MAP3D_POINT,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut height: COUNT,
) {
    let mut deltax: ::core::ffi::c_double = 0.;
    let mut deltay: ::core::ffi::c_double = 0.;
    let mut inv_deltax: ::core::ffi::c_double = 0.;
    let mut inv_deltay: ::core::ffi::c_double = 0.;
    let mut nextx: COORD = 0;
    let mut nexty: COORD = 0;
    let mut i: COUNT = 0;
    let mut d1: ::core::ffi::c_double = 0.;
    let mut d2: ::core::ffi::c_double = 0.;
    let mut d3: ::core::ffi::c_double = 0.;
    let mut d4: ::core::ffi::c_double = 0.;
    let mut m: [::core::ffi::c_double; 4] = [0.; 4];
    let mut spherespanx: COUNT = height;
    if x < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        x = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if x >= spherespanx as ::core::ffi::c_int as ::core::ffi::c_double {
        x = (spherespanx as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_double;
    }
    if y < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        y = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if y >= height as ::core::ffi::c_int as ::core::ffi::c_double {
        y = (height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_double;
    }
    (*ppt).p[0usize].x = x as COORD;
    (*ppt).p[0usize].y = y as COORD;
    deltax = x - (*ppt).p[0usize].x as ::core::ffi::c_int as ::core::ffi::c_double;
    deltay = y - (*ppt).p[0usize].y as ::core::ffi::c_int as ::core::ffi::c_double;
    if deltax == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && deltay == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        (*ppt).m[0usize] = 0 as DWORD;
        return;
    }
    if deltax != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        nextx = ((*ppt).p[0usize].x as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as COORD;
    } else {
        nextx = (*ppt).p[0usize].x;
    }
    if deltay != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        nexty = ((*ppt).p[0usize].y as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as COORD;
    } else {
        nexty = (*ppt).p[0usize].y;
    }
    (*ppt).p[1usize].x = nextx;
    (*ppt).p[1usize].y = (*ppt).p[0usize].y;
    (*ppt).p[2usize].x = (*ppt).p[0usize].x;
    (*ppt).p[2usize].y = nexty;
    (*ppt).p[3usize].x = nextx;
    (*ppt).p[3usize].y = nexty;
    inv_deltax = 1.0f64 - fabs(deltax);
    inv_deltax *= inv_deltax;
    inv_deltay = 1.0f64 - fabs(deltay);
    inv_deltay *= inv_deltay;
    deltax *= deltax;
    deltay *= deltay;
    d1 = sqrt(deltax + deltay);
    d2 = sqrt(inv_deltax + deltay);
    d3 = sqrt(deltax + inv_deltay);
    d4 = sqrt(inv_deltax + inv_deltay);
    m[0usize] = 1 as ::core::ffi::c_int as ::core::ffi::c_double
        / (1 as ::core::ffi::c_int as ::core::ffi::c_double
            + d1
                * (1 as ::core::ffi::c_int as ::core::ffi::c_double / d2
                    + 1 as ::core::ffi::c_int as ::core::ffi::c_double / d3
                    + 1 as ::core::ffi::c_int as ::core::ffi::c_double / d4));
    m[1usize] = m[0usize] * d1 / d2;
    m[2usize] = m[0usize] * d1 / d3;
    m[3usize] = m[0usize] * d1 / d4;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        (*ppt).m[i as usize] = (m[i as usize]
            * ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as DWORD;
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn get_color_channel(
    mut c: Color,
    mut channel: ::core::ffi::c_int,
) -> BYTE {
    match channel {
        0 => return c.r,
        1 => return c.g,
        2 => return c.b,
        _ => return 0 as BYTE,
    };
}
unsafe extern "C" fn get_avg_channel(
    mut p: *mut Color,
    mut mult: *mut DWORD,
    mut channel: ::core::ffi::c_int,
) -> BYTE {
    let mut j: COUNT = 0;
    let mut ci: DWORD = 0 as DWORD;
    j = 0 as COUNT;
    while (j as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        let mut c: BYTE = get_color_channel(*p.offset(j as isize), channel);
        ci = ci.wrapping_add((c as DWORD).wrapping_mul(*mult.offset(j as isize)));
        j = j.wrapping_add(1);
    }
    ci >>= 16 as ::core::ffi::c_int;
    if ci > 255 as DWORD {
        ci = 255 as DWORD;
    }
    return ci as BYTE;
}
unsafe extern "C" fn CreateSphereTiltMap(
    mut angle: ::core::ffi::c_int,
    mut height: COUNT,
    mut radius: COUNT,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let multx: ::core::ffi::c_double = height as ::core::ffi::c_double
        / 3.14159265358979323846f64;
    let multy: ::core::ffi::c_double = height as ::core::ffi::c_double
        / 3.14159265358979323846f64;
    let xadj: ::core::ffi::c_double = height as ::core::ffi::c_double / 2.0f64;
    y = -(radius as ::core::ffi::c_int);
    while y <= radius as ::core::ffi::c_int {
        let mut y_2: ::core::ffi::c_int = y * y;
        x = -(radius as ::core::ffi::c_int);
        while x <= radius as ::core::ffi::c_int {
            let mut dx: ::core::ffi::c_double = 0.;
            let mut dy: ::core::ffi::c_double = 0.;
            let mut newx: ::core::ffi::c_double = 0.;
            let mut newy: ::core::ffi::c_double = 0.;
            let mut da: ::core::ffi::c_double = 0.;
            let mut rad: ::core::ffi::c_double = 0.;
            let mut rad_2: ::core::ffi::c_double = 0.;
            let mut xa: ::core::ffi::c_double = 0.;
            let mut ya: ::core::ffi::c_double = 0.;
            let mut ppt: *mut MAP3D_POINT = (*(*pSolarSysState)
                .Orbit
                .map_rotate
                .offset((y + radius as ::core::ffi::c_int) as isize))
                .offset((x + radius as ::core::ffi::c_int) as isize);
            rad_2 = (x * x + y_2) as ::core::ffi::c_double;
            if rad_2
                >= ((radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * (radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                    as ::core::ffi::c_double
            {
                (*ppt).p[0usize].x = (x + radius as ::core::ffi::c_int) as COORD;
                (*ppt).p[0usize].y = (y + radius as ::core::ffi::c_int) as COORD;
                (*ppt).m[0usize] = 0 as DWORD;
            } else {
                rad = sqrt(rad_2);
                if rad >= radius as ::core::ffi::c_int as ::core::ffi::c_double {
                    rad = radius as ::core::ffi::c_double - 0.1f64;
                }
                da = atan2(y as ::core::ffi::c_double, x as ::core::ffi::c_double);
                da
                    += 3.14159265358979323846f64 * 2.0f64 / 360.0f64
                        * -angle as ::core::ffi::c_double;
                dx = rad * cos(da);
                dy = rad * sin(da);
                xa = acos(-dx / radius as ::core::ffi::c_int as ::core::ffi::c_double);
                ya = acos(-dy / radius as ::core::ffi::c_int as ::core::ffi::c_double);
                newx = multx * xa;
                newy = multy * ya;
                if ya <= 0.05f64 || ya >= 3.1f64 {
                    newx = xadj;
                } else {
                    newx = xadj + (newx - xadj) / sin(ya);
                }
                create_aa_points(ppt, newx, newy, height);
            }
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn CreateShieldMask(mut radius: COUNT) -> FRAME {
    let mut clear: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut pix: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ShieldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut shieldradius: COUNT = ((((33 as ::core::ffi::c_int) << resolutionFactor)
        + (if playerInPlanetOrbit() as ::core::ffi::c_int != 0 {
            7 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)) * radius as ::core::ffi::c_int
        / ((33 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
    let mut shielddiam: COUNT = (((shieldradius as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COUNT;
    ShieldFrame = CaptureDrawable(
        CreateDrawable(
            (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int) as CREATE_FLAGS,
            shielddiam as SIZE,
            shielddiam as SIZE,
            1 as COUNT,
        ),
    );
    pix = (*Orbit).ScratchArray;
    clear = buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    y = -(shieldradius as ::core::ffi::c_int);
    while y <= shieldradius as ::core::ffi::c_int {
        x = -(shieldradius as ::core::ffi::c_int);
        while x <= shieldradius as ::core::ffi::c_int {
            let mut rad_2: ::core::ffi::c_int = x * x + y * y;
            let mut red: ::core::ffi::c_int = 120 as ::core::ffi::c_int
                + 100 as ::core::ffi::c_int;
            let mut alpha: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
            let mut rad: ::core::ffi::c_double = 0.;
            if rad_2
                >= (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    * (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            {
                *pix = clear;
            } else if rad_2
                <= radius as ::core::ffi::c_int * radius as ::core::ffi::c_int
            {
                *pix = clear;
            } else {
                rad = sqrt(rad_2 as ::core::ffi::c_double);
                if rad <= radius as ::core::ffi::c_int as ::core::ffi::c_double + 0.8f64
                {
                    alpha = (red as ::core::ffi::c_double
                        * (rad - radius as ::core::ffi::c_int as ::core::ffi::c_double))
                        as ::core::ffi::c_int;
                    red = 255 as ::core::ffi::c_int;
                } else {
                    red
                        -= ((red
                            - (120 as ::core::ffi::c_int + 100 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)) as ::core::ffi::c_double
                            * (rad
                                - radius as ::core::ffi::c_int as ::core::ffi::c_double)
                            / ((if playerInPlanetOrbit() as ::core::ffi::c_int != 0 {
                                7 as ::core::ffi::c_int
                            } else {
                                14 as ::core::ffi::c_int
                            } << resolutionFactor) * radius as ::core::ffi::c_int
                                / ((33 as ::core::ffi::c_int) << resolutionFactor))
                                as ::core::ffi::c_double) as ::core::ffi::c_int;
                    if red < 0 as ::core::ffi::c_int {
                        red = 0 as ::core::ffi::c_int;
                    }
                }
                if optNebulae.0 != 0 {
                    if alpha != 255 as ::core::ffi::c_int {
                        *pix = buildColorRgba(
                            red as BYTE,
                            0 as BYTE,
                            0 as BYTE,
                            alpha as BYTE,
                        );
                    } else {
                        *pix = buildColorRgba(
                            255 as BYTE,
                            0 as BYTE,
                            0 as BYTE,
                            red as BYTE,
                        );
                    }
                } else {
                    *pix = buildColorRgba(
                        red as BYTE,
                        0 as BYTE,
                        0 as BYTE,
                        alpha as BYTE,
                    );
                }
            }
            x += 1;
            pix = pix.offset(1);
        }
        y += 1;
    }
    uqm_lua_checked!(WriteFramePixelColors(
        ShieldFrame,
        (*Orbit).ScratchArray,
        shielddiam as ::core::ffi::c_int,
        shielddiam as ::core::ffi::c_int,
    ), ::core::ptr::null_mut());
    SetFrameHot(
        ShieldFrame,
        MAKE_HOT_SPOT(
            (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
            (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ),
    );
    return ShieldFrame;
}
#[no_mangle]
pub unsafe extern "C" fn SaveBackFrame(mut radius: COUNT) -> FRAME {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut BackFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut shieldradius: COUNT = ((((33 as ::core::ffi::c_int) << resolutionFactor)
        + (if playerInPlanetOrbit() as ::core::ffi::c_int != 0 {
            7 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)) * radius as ::core::ffi::c_int
        / ((33 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
    let mut shielddiam: COUNT = (((shieldradius as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COUNT;
    r.corner = MAKE_POINT(
        (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD,
        ((((CanvasHeight
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
            - (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD,
    );
    r.extent.width = shielddiam as COORD;
    r.extent.height = r.extent.width;
    BackFrame = CaptureDrawable(CopyFrameRect(GetStarBackFround(), &raw mut r));
    SetFrameHot(
        BackFrame,
        MAKE_HOT_SPOT(
            (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
            (shieldradius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ),
    );
    return BackFrame;
}
#[inline]
unsafe extern "C" fn shield_level(mut offset: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut level: ::core::ffi::c_int = 0;
    offset = (offset << 8 as ::core::ffi::c_int)
        % ((if optPlanetTexture.0 != 0 {
            243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        } else {
            210 as ::core::ffi::c_int
        } << resolutionFactor << 8 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int);
    level = abs(
        offset
            - ((if optPlanetTexture.0 != 0 {
                243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            } else {
                210 as ::core::ffi::c_int
            } << resolutionFactor << 8 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int),
    );
    level = 100 as ::core::ffi::c_int
        + level * (256 as ::core::ffi::c_int - 100 as ::core::ffi::c_int)
            / ((if optPlanetTexture.0 != 0 {
                243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            } else {
                210 as ::core::ffi::c_int
            } << resolutionFactor << 8 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int);
    return level;
}
#[no_mangle]
pub unsafe extern "C" fn SetShieldThrobEffect(
    mut ShieldFrame: FRAME,
    mut offset: ::core::ffi::c_int,
    mut ThrobFrame: FRAME,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut pix: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut level: ::core::ffi::c_int = 0;
    level = shield_level(offset);
    width = (*ShieldFrame).Bounds.width as ::core::ffi::c_int;
    height = (*ShieldFrame).Bounds.height as ::core::ffi::c_int;
    uqm_lua_checked!(ReadFramePixelColors(ShieldFrame, (*Orbit).ScratchArray, width, height), ());
    i = 0 as ::core::ffi::c_int;
    pix = (*Orbit).ScratchArray;
    while i < width * height {
        let mut p: Color = *pix;
        if optNebulae.0 != 0 {
            p.a = (p.a as ::core::ffi::c_int * level / 256 as ::core::ffi::c_int)
                as BYTE;
        } else if p.a as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
            p.r = (p.r as ::core::ffi::c_int * level / 256 as ::core::ffi::c_int)
                as BYTE;
            p.g = (p.g as ::core::ffi::c_int * level / 256 as ::core::ffi::c_int)
                as BYTE;
            p.b = (p.b as ::core::ffi::c_int * level / 256 as ::core::ffi::c_int)
                as BYTE;
        } else if p.a as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            p.a = (p.a as ::core::ffi::c_int * level / 256 as ::core::ffi::c_int)
                as BYTE;
        }
        *pix = p;
        i += 1;
        pix = pix.offset(1);
    }
    uqm_lua_checked!(WriteFramePixelColors(ThrobFrame, (*Orbit).ScratchArray, width, height), ());
    SetFrameHot(ThrobFrame, GetFrameHot(ShieldFrame));
}
#[no_mangle]
pub unsafe extern "C" fn Draw3DOShield(mut ShieldFrame: STAMP) {
    static mut i: BYTE = 0;
    static mut cr: BYTE = 0;
    static mut flip: BOOLEAN = BOOLEAN::FALSE;
    let mut oldmode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let factor: [BYTE; 6] = [
        0xff as BYTE,
        0xe6 as BYTE,
        0xc5 as BYTE,
        0xa4 as BYTE,
        0x83 as BYTE,
        0x62 as BYTE,
    ];
    if factor[i as usize] as ::core::ffi::c_int == 0x62 as ::core::ffi::c_int {
        flip = BOOLEAN::TRUE;
    } else if factor[i as usize] as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
        flip = BOOLEAN::FALSE;
    }
    oldmode = SetContextDrawMode(
        MAKE_DRAW_MODE(DrawKind::DRAW_ADDITIVE, factor[i as usize] as SWORD),
    );
    uqm_lua_checked!(DrawStamp(&raw mut ShieldFrame), ());
    SetContextDrawMode(oldmode);
    if cr as ::core::ffi::c_int
        % if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            3 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        } == 0 as ::core::ffi::c_int
    {
        i = (i as ::core::ffi::c_int
            + if flip.0 != 0 {
                -1 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as BYTE;
    }
    cr = cr.wrapping_add(1);
}
unsafe extern "C" fn ApplyShieldTint() {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut tint: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    oldFrame = SetContextFGFrame((*pSolarSysState).TopoFrame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut r);
    tint = buildColorRgba(0xff as BYTE, 0 as BYTE, 0 as BYTE, 0xff as BYTE);
    mode = MAKE_DRAW_MODE(DrawKind::DRAW_ADDITIVE, 0xff as SWORD);
    oldMode = SetContextDrawMode(mode);
    SetContextForeGroundColor(tint);
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextDrawMode(oldMode);
    SetContextFGFrame(oldFrame);
}
#[inline]
unsafe extern "C" fn calc_map_light(
    mut val: UBYTE,
    mut dif: DWORD,
    mut lvf: ::core::ffi::c_int,
) -> UBYTE {
    let mut i: ::core::ffi::c_int = 0;
    i = (dif.wrapping_mul(val as DWORD) >> 16 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    i += lvf * val as ::core::ffi::c_int >> 7 as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
    } else if i > 255 as ::core::ffi::c_int {
        i = 255 as ::core::ffi::c_int;
    }
    return i as UBYTE;
}
#[inline]
unsafe extern "C" fn get_map_pixel(
    mut pixels: *mut Color,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: COUNT,
    mut spherespanx: COUNT,
) -> Color {
    return *pixels
        .offset(
            (y * (width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int) + x)
                as isize,
        );
}
#[inline]
unsafe extern "C" fn apply_alpha_pixel(
    mut pix: Color,
    mut scan: ::core::ffi::c_int,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = (tintColors[scan as usize].r as ::core::ffi::c_int * 0x45 as ::core::ffi::c_int
        / 0xff as ::core::ffi::c_int
        + pix.r as ::core::ffi::c_int * 0xff as ::core::ffi::c_int
            * (0xff as ::core::ffi::c_int - 0x45 as ::core::ffi::c_int)
            / (255 as ::core::ffi::c_int * 255 as ::core::ffi::c_int)) as BYTE;
    c.g = (tintColors[scan as usize].g as ::core::ffi::c_int * 0x45 as ::core::ffi::c_int
        / 0xff as ::core::ffi::c_int
        + pix.g as ::core::ffi::c_int * 0xff as ::core::ffi::c_int
            * (0xff as ::core::ffi::c_int - 0x45 as ::core::ffi::c_int)
            / (255 as ::core::ffi::c_int * 255 as ::core::ffi::c_int)) as BYTE;
    c.b = (tintColors[scan as usize].b as ::core::ffi::c_int * 0x45 as ::core::ffi::c_int
        / 0xff as ::core::ffi::c_int
        + pix.b as ::core::ffi::c_int * 0xff as ::core::ffi::c_int
            * (0xff as ::core::ffi::c_int - 0x45 as ::core::ffi::c_int)
            / (255 as ::core::ffi::c_int * 255 as ::core::ffi::c_int)) as BYTE;
    return c;
}
#[inline]
unsafe extern "C" fn apply_additive_pixel(
    mut pix: Color,
    mut scan: ::core::ffi::c_int,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = clip_channel(
        pix.r as ::core::ffi::c_int
            + (tintColors[scan as usize].r as ::core::ffi::c_int
                * 0x45 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int),
    );
    c.g = clip_channel(
        pix.g as ::core::ffi::c_int
            + (tintColors[scan as usize].g as ::core::ffi::c_int
                * 0x45 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int),
    );
    c.b = clip_channel(
        pix.b as ::core::ffi::c_int
            + (tintColors[scan as usize].b as ::core::ffi::c_int
                * 0x45 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int),
    );
    return c;
}
#[inline]
unsafe extern "C" fn get_map_elev(
    mut elevs: *mut SBYTE,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut width: COUNT,
) -> ::core::ffi::c_int {
    return *elevs
        .offset(
            (y * width as ::core::ffi::c_int
                + (offset + x) % width as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RenderPlanetSphere(
    mut Orbit: *mut PLANET_ORBIT,
    mut MaskFrame: FRAME,
    mut offset: ::core::ffi::c_int,
    mut shielded: BOOLEAN,
    mut doThrob: BOOLEAN,
    mut width: COUNT,
    mut height: COUNT,
    mut radius: COUNT,
) {
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut pix: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut clear: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut pixels: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut elevs: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut shLevel: ::core::ffi::c_int = 0;
    let mut spherespanx: COUNT = height;
    let mut tworadius: COUNT = ((radius as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as COUNT;
    let mut diameter: COUNT = (tworadius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as COUNT;
    shLevel = shield_level(offset);
    pix = (*Orbit).ScratchArray;
    clear = buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    if ((*Orbit).scanType as ::core::ffi::c_int)
        < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
        && !(*Orbit).ScanColors.is_null()
    {
        pixels = (*(*Orbit).ScanColors.offset((*Orbit).scanType as isize))
            .offset(offset as isize);
    } else {
        pixels = (*Orbit).TopoColors.offset(offset as isize);
    }
    elevs = (*Orbit).lpTopoData;
    pt.y = 0 as COORD;
    y = -(radius as ::core::ffi::c_int);
    while pt.y as ::core::ffi::c_int <= tworadius as ::core::ffi::c_int {
        pt.x = 0 as COORD;
        x = -(radius as ::core::ffi::c_int);
        while pt.x as ::core::ffi::c_int <= tworadius as ::core::ffi::c_int {
            let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
            let mut diffus: DWORD = *(*(*Orbit).light_diff.offset(pt.y as isize))
                .offset(pt.x as isize);
            let mut i: ::core::ffi::c_int = 0;
            let mut ppt: *mut MAP3D_POINT = (*(*Orbit).map_rotate.offset(pt.y as isize))
                .offset(pt.x as isize);
            let mut lvf: ::core::ffi::c_int = 0;
            if diffus == 0 as DWORD {
                *pix = clear;
            } else {
                if (*ppt).m[0usize] == 0 as DWORD {
                    c = get_map_pixel(
                        pixels,
                        (*ppt).p[0usize].x as ::core::ffi::c_int,
                        (*ppt).p[0usize].y as ::core::ffi::c_int,
                        width,
                        spherespanx,
                    );
                    lvf = get_map_elev(
                        elevs,
                        (*ppt).p[0usize].x as ::core::ffi::c_int,
                        (*ppt).p[0usize].y as ::core::ffi::c_int,
                        offset,
                        width,
                    );
                } else {
                    let mut p: [Color; 4] = [Color { r: 0, g: 0, b: 0, a: 0 }; 4];
                    let mut lvsum: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < 4 as ::core::ffi::c_int {
                        p[i as usize] = get_map_pixel(
                            pixels,
                            (*ppt).p[i as usize].x as ::core::ffi::c_int,
                            (*ppt).p[i as usize].y as ::core::ffi::c_int,
                            width,
                            spherespanx,
                        );
                        i += 1;
                    }
                    c.r = get_avg_channel(
                        &raw mut p as *mut Color,
                        &raw mut (*ppt).m as *mut DWORD,
                        0 as ::core::ffi::c_int,
                    );
                    c.g = get_avg_channel(
                        &raw mut p as *mut Color,
                        &raw mut (*ppt).m as *mut DWORD,
                        1 as ::core::ffi::c_int,
                    );
                    c.b = get_avg_channel(
                        &raw mut p as *mut Color,
                        &raw mut (*ppt).m as *mut DWORD,
                        2 as ::core::ffi::c_int,
                    );
                    i = 0 as ::core::ffi::c_int;
                    lvsum = 0 as ::core::ffi::c_int;
                    while i < 4 as ::core::ffi::c_int {
                        lvsum = (lvsum as DWORD)
                            .wrapping_add(
                                (get_map_elev(
                                    elevs,
                                    (*ppt).p[0usize].x as ::core::ffi::c_int,
                                    (*ppt).p[0usize].y as ::core::ffi::c_int,
                                    offset,
                                    width,
                                ) as DWORD)
                                    .wrapping_mul((*ppt).m[i as usize]),
                            ) as ::core::ffi::c_int;
                        i += 1;
                    }
                    lvf = lvsum >> 16 as ::core::ffi::c_int;
                }
                if shielded.0 != 0 {
                    let mut r: ::core::ffi::c_int = 0;
                    c.g = ((c.g as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        + (c.g as ::core::ffi::c_int >> 2 as ::core::ffi::c_int))
                        as BYTE;
                    c.b = ((c.b as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        + (c.b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int))
                        as BYTE;
                    c.r = calc_map_light(c.r, diffus, lvf) as BYTE;
                    c.g = calc_map_light(c.g, diffus, lvf) as BYTE;
                    c.b = calc_map_light(c.b, diffus, lvf) as BYTE;
                    r = calc_map_light(100 as UBYTE, diffus, 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    r += 120 as ::core::ffi::c_int;
                    if doThrob.0 != 0 {
                        r = r * shLevel / 256 as ::core::ffi::c_int;
                    }
                    r += c.r as ::core::ffi::c_int;
                    if r > 255 as ::core::ffi::c_int {
                        r = 255 as ::core::ffi::c_int;
                    }
                    c.r = r as BYTE;
                } else {
                    c.r = calc_map_light(c.r, diffus, lvf) as BYTE;
                    c.g = calc_map_light(c.g, diffus, lvf) as BYTE;
                    c.b = calc_map_light(c.b, diffus, lvf) as BYTE;
                }
                c.a = 0xff as BYTE;
                if optScanStyle == 0x1 as ::core::ffi::c_int
                    && optTintPlanSphere == 0x2 as ::core::ffi::c_int
                    && ((*Orbit).scanType as ::core::ffi::c_int)
                        < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                {
                    *pix = apply_alpha_pixel(c, (*Orbit).scanType as ::core::ffi::c_int);
                } else {
                    *pix = c;
                }
            }
            pt.x += 1;
            x += 1;
            pix = pix.offset(1);
        }
        pt.y += 1;
        y += 1;
    }
    uqm_lua_checked!(WriteFramePixelColors(
        MaskFrame,
        (*Orbit).ScratchArray,
        diameter as ::core::ffi::c_int,
        diameter as ::core::ffi::c_int,
    ), ());
    SetFrameHot(
        MaskFrame,
        MAKE_HOT_SPOT(
            (radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
            (radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn RenderDOSPlanetSphere(
    mut Orbit: *mut PLANET_ORBIT,
    mut MaskFrame: FRAME,
    mut offset: ::core::ffi::c_int,
) {
    if (*Orbit).TopoMask.is_null() {
        return
    } else {
        let mut pix: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut color: *mut Color = ::core::ptr::null_mut::<Color>();
        let mut x: COUNT = 0;
        let mut y: COUNT = 0;
        let mut width: SIZE = (*MaskFrame).Bounds.width;
        let mut height: SIZE = (*(*Orbit).TopoMask).Bounds.height;
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut dupeframe: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut PlanetInfo: *mut PLANET_INFO = &raw mut (*pSolarSysState)
            .SysInfo
            .PlanetInfo;
        r.corner.y = 0 as COORD;
        r.corner.x = offset as COORD;
        r.extent.width = width as COORD;
        r.extent.height = height as COORD;
        dupeframe = CaptureDrawable(CopyFrameRect((*Orbit).TopoMask, &raw mut r));
        if (*PlanetInfo).AxialTilt as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            let mut baseframe: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
            let mut rotFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
            let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
            let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
            let mut trueheight: SIZE = (*MaskFrame).Bounds.height;
            rotFrame = CaptureDrawable(
                RotateFrame(dupeframe, (*PlanetInfo).AxialTilt as ::core::ffi::c_int),
            );
            GetFrameRect(rotFrame, &raw mut r);
            baseframe = CaptureDrawable(
                CreateDrawable(
                    ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as CREATE_FLAGS,
                    width,
                    trueheight,
                    1 as COUNT,
                ),
            );
            oldContext = SetContext(OffScreenContext);
            SetContextFGFrame(baseframe);
            SetContextClipRect(::core::ptr::null_mut::<RECT>());
            oldMode = SetContextDrawMode(
                MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            );
            SetFrameHot(rotFrame, MAKE_HOT_SPOT(0 as COORD, 0 as COORD));
            s.origin.x = ((width as ::core::ffi::c_int
                - r.extent.width as ::core::ffi::c_int) / 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COORD;
            s.origin.y = ((trueheight as ::core::ffi::c_int
                - r.extent.height as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                as COORD;
            s.frame = rotFrame;
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            SetContextDrawMode(oldMode);
            SetContext(oldContext);
            uqm_lua_checked!(ReadFramePixelColors(
                baseframe,
                (*Orbit).ScratchArray,
                width as ::core::ffi::c_int,
                trueheight as ::core::ffi::c_int,
            ), ());
            DestroyDrawable(ReleaseDrawable(baseframe));
            DestroyDrawable(ReleaseDrawable(rotFrame));
        } else {
            uqm_lua_checked!(ReadFramePixelColors(
                dupeframe,
                (*Orbit).ScratchArray,
                width as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
            ), ());
        }
        pix = (*Orbit).sphereBytes;
        color = (*Orbit).ScratchArray;
        y = 0 as COUNT;
        while (y as ::core::ffi::c_int)
            < (*MaskFrame).Bounds.height as ::core::ffi::c_int
        {
            x = 0 as COUNT;
            while (x as ::core::ffi::c_int)
                < (*MaskFrame).Bounds.width as ::core::ffi::c_int
            {
                if (*pix as ::core::ffi::c_int) < 0xff as ::core::ffi::c_int {
                    *pix = (*pix as ::core::ffi::c_int
                        - *pix as ::core::ffi::c_int / 32 as ::core::ffi::c_int
                            * 32 as ::core::ffi::c_int
                        + (*color).r as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                        as BYTE;
                }
                x = x.wrapping_add(1);
                color = color.offset(1);
                pix = pix.offset(1);
            }
            y = y.wrapping_add(1);
        }
        WriteFramePixelIndexes(
            MaskFrame,
            (*Orbit).sphereBytes,
            (*MaskFrame).Bounds.width as ::core::ffi::c_int,
            (*MaskFrame).Bounds.height as ::core::ffi::c_int,
        );
        DestroyDrawable(ReleaseDrawable(dupeframe));
        dupeframe = ::core::ptr::null_mut::<FRAME_DESC>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn Render3DOPlanetSphere(
    mut Orbit: *mut PLANET_ORBIT,
    mut MaskFrame: FRAME,
    mut offset: ::core::ffi::c_int,
    mut rotwidth: COUNT,
    mut height: COUNT,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut c: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut pixels: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut shade: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut clear: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut spherespanx: COUNT = height;
    let mut radius: COUNT = ((spherespanx as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as COUNT;
    let mut tworadius: COUNT = ((radius as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as COUNT;
    let mut diameter: COUNT = (tworadius as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as COUNT;
    c = (*Orbit).ScratchArray;
    shade = (*Orbit).ShadeColors;
    pixels = (*Orbit).TopoColors.offset(offset as isize);
    clear = buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    pt.y = 0 as COORD;
    y = -(radius as ::core::ffi::c_int);
    while pt.y as ::core::ffi::c_int <= tworadius as ::core::ffi::c_int {
        pt.x = 0 as COORD;
        x = -(radius as ::core::ffi::c_int);
        while pt.x as ::core::ffi::c_int <= tworadius as ::core::ffi::c_int {
            let mut ppt: *mut MAP3D_POINT = (*(*Orbit).map_rotate.offset(pt.y as isize))
                .offset(pt.x as isize);
            if (*ppt).m[0usize] == 0 as DWORD
                || (*shade).r as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            {
                *c = clear;
            } else {
                *c = get_map_pixel(
                    pixels,
                    (*ppt).p[0usize].x as ::core::ffi::c_int,
                    (*ppt).p[0usize].y as ::core::ffi::c_int,
                    rotwidth,
                    spherespanx,
                );
                (*c).r = clip_channel(
                    (*c).r as ::core::ffi::c_int - (*shade).r as ::core::ffi::c_int,
                );
                (*c).g = clip_channel(
                    (*c).g as ::core::ffi::c_int - (*shade).g as ::core::ffi::c_int,
                );
                (*c).b = clip_channel(
                    (*c).b as ::core::ffi::c_int - (*shade).b as ::core::ffi::c_int,
                );
                if optTintPlanSphere == 0x2 as ::core::ffi::c_int
                    && ((*Orbit).scanType as ::core::ffi::c_int)
                        < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                {
                    if optScanStyle == 0x1 as ::core::ffi::c_int {
                        *c = apply_additive_pixel(
                            *c,
                            (*Orbit).scanType as ::core::ffi::c_int,
                        );
                    } else if optScanStyle == 0x2 as ::core::ffi::c_int {
                        TransformColor(c, (*Orbit).scanType);
                    }
                }
            }
            pt.x += 1;
            x += 1;
            c = c.offset(1);
            shade = shade.offset(1);
        }
        pt.y += 1;
        y += 1;
    }
    uqm_lua_checked!(WriteFramePixelColors(
        MaskFrame,
        (*Orbit).ScratchArray,
        diameter as ::core::ffi::c_int,
        diameter as ::core::ffi::c_int,
    ), ());
    SetFrameHot(
        MaskFrame,
        MAKE_HOT_SPOT(
            (radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
            (radius as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ),
    );
}
unsafe extern "C" fn DitherMap(
    mut DepthArray: *mut SBYTE,
    mut width: COUNT,
    mut height: COUNT,
) {
    let mut i: DWORD = 0;
    let mut elev: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut rand_val: DWORD = 0 as DWORD;
    i = 0 as DWORD;
    elev = DepthArray;
    while i < (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as DWORD {
        if i & 3 as DWORD == 0 as DWORD {
            rand_val = RandomContext_Random(SysGenRNG);
        } else {
            rand_val >>= 8 as ::core::ffi::c_int;
        }
        *elev = (*elev as DWORD)
            .wrapping_add(
                ((((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int) as DWORD)
                    .wrapping_sub(
                        rand_val
                            & (((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD,
                    ),
            ) as SBYTE;
        i = i.wrapping_add(1);
        elev = elev.offset(1);
    }
}
unsafe extern "C" fn MakeCrater(
    mut pRect: *mut RECT,
    mut DepthArray: *mut SBYTE,
    mut rim_delta: SIZE,
    mut crater_delta: SIZE,
    mut SetDepth: BOOLEAN,
    mut width: COUNT,
) {
    let mut x: COORD = 0;
    let mut y: COORD = 0;
    let mut lf_x: COORD = 0;
    let mut rt_x: COORD = 0;
    let mut A: SIZE = 0;
    let mut B: SIZE = 0;
    let mut Asquared: SDWORD = 0;
    let mut TwoAsquared: SDWORD = 0;
    let mut Bsquared: SDWORD = 0;
    let mut TwoBsquared: SDWORD = 0;
    let mut d: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut TopIndex: DWORD = 0;
    let mut BotIndex: DWORD = 0;
    let mut rim_pixels: DWORD = 0;
    A = ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    B = ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as SIZE;
    x = 0 as COORD;
    y = B as COORD;
    Asquared = (A as DWORD).wrapping_mul(A as DWORD) as SDWORD;
    TwoAsquared = Asquared << 1 as ::core::ffi::c_int;
    Bsquared = (B as DWORD).wrapping_mul(B as DWORD) as SDWORD;
    TwoBsquared = Bsquared << 1 as ::core::ffi::c_int;
    dx = 0 as ::core::ffi::c_int as SDWORD;
    dy = (TwoAsquared * B as ::core::ffi::c_int) as SDWORD;
    d = Bsquared - (dy >> 1 as ::core::ffi::c_int)
        + (Asquared >> 2 as ::core::ffi::c_int);
    A = (A as ::core::ffi::c_int + (*pRect).corner.x as ::core::ffi::c_int) as SIZE;
    B = (B as ::core::ffi::c_int + (*pRect).corner.y as ::core::ffi::c_int) as SIZE;
    TopIndex = ((B as ::core::ffi::c_int - y as ::core::ffi::c_int)
        * width as ::core::ffi::c_int) as DWORD;
    BotIndex = ((B as ::core::ffi::c_int + y as ::core::ffi::c_int)
        * width as ::core::ffi::c_int) as DWORD;
    rim_pixels = 1 as DWORD;
    while dx < dy {
        if d > 0 as ::core::ffi::c_int {
            lf_x = (A as ::core::ffi::c_int - x as ::core::ffi::c_int) as COORD;
            rt_x = (A as ::core::ffi::c_int + x as ::core::ffi::c_int) as COORD;
            if SetDepth.0 != 0 {
                memset(
                    DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (rt_x as ::core::ffi::c_int - lf_x as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as size_t,
                );
                memset(
                    DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (rt_x as ::core::ffi::c_int - lf_x as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as size_t,
                );
            }
            if lf_x as ::core::ffi::c_int == rt_x as ::core::ffi::c_int {
                *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
                *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
                rim_pixels = 0 as DWORD;
            } else {
                loop {
                    *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                        .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                        as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int)
                        as SBYTE;
                    *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                        .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                        as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int)
                        as SBYTE;
                    if lf_x as ::core::ffi::c_int != rt_x as ::core::ffi::c_int {
                        *DepthArray
                            .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                            .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize)
                            as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int)
                            as SBYTE;
                        *DepthArray
                            .offset(BotIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                            .offset(BotIndex.wrapping_add(rt_x as DWORD) as isize)
                            as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int)
                            as SBYTE;
                    }
                    lf_x += 1;
                    rt_x -= 1;
                    rim_pixels = rim_pixels.wrapping_sub(1);
                    if rim_pixels == 0 {
                        break;
                    }
                }
                while (lf_x as ::core::ffi::c_int) < rt_x as ::core::ffi::c_int {
                    *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                        .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                        as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int)
                        as SBYTE;
                    *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                        .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                        as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int)
                        as SBYTE;
                    *DepthArray.offset(TopIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                        .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize)
                        as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int)
                        as SBYTE;
                    *DepthArray.offset(BotIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                        .offset(BotIndex.wrapping_add(rt_x as DWORD) as isize)
                        as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int)
                        as SBYTE;
                    lf_x += 1;
                    rt_x -= 1;
                }
                if lf_x as ::core::ffi::c_int == rt_x as ::core::ffi::c_int {
                    *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                        .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                        as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int)
                        as SBYTE;
                    *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                        .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                        as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int)
                        as SBYTE;
                }
            }
            y -= 1;
            TopIndex = TopIndex.wrapping_add(width as DWORD);
            BotIndex = BotIndex.wrapping_sub(width as DWORD);
            dy -= TwoAsquared;
            d -= dy;
        }
        rim_pixels = rim_pixels.wrapping_add(1);
        x += 1;
        dx += TwoBsquared;
        d += Bsquared + dx;
    }
    d
        += (((Asquared - Bsquared) * 3 as SDWORD >> 1 as ::core::ffi::c_int) - (dx + dy)
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    while y as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        lf_x = (A as ::core::ffi::c_int - x as ::core::ffi::c_int) as COORD;
        rt_x = (A as ::core::ffi::c_int + x as ::core::ffi::c_int) as COORD;
        if SetDepth.0 != 0 {
            memset(
                DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (rt_x as ::core::ffi::c_int - lf_x as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as size_t,
            );
            memset(
                DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (rt_x as ::core::ffi::c_int - lf_x as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as size_t,
            );
        }
        if lf_x as ::core::ffi::c_int == rt_x as ::core::ffi::c_int {
            *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
            *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
        } else {
            loop {
                *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
                *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
                if lf_x as ::core::ffi::c_int != rt_x as ::core::ffi::c_int {
                    *DepthArray.offset(TopIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                        .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize)
                        as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int)
                        as SBYTE;
                    *DepthArray.offset(BotIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                        .offset(BotIndex.wrapping_add(rt_x as DWORD) as isize)
                        as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int)
                        as SBYTE;
                }
                lf_x += 1;
                rt_x -= 1;
                rim_pixels = rim_pixels.wrapping_sub(1);
                if rim_pixels == 0 {
                    break;
                }
            }
            while (lf_x as ::core::ffi::c_int) < rt_x as ::core::ffi::c_int {
                *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
                *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
                *DepthArray.offset(TopIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                    .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize)
                    as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
                *DepthArray.offset(BotIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                    .offset(BotIndex.wrapping_add(rt_x as DWORD) as isize)
                    as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
                lf_x += 1;
                rt_x -= 1;
            }
            if lf_x as ::core::ffi::c_int == rt_x as ::core::ffi::c_int {
                *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
                *DepthArray.offset(BotIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                    .offset(BotIndex.wrapping_add(lf_x as DWORD) as isize)
                    as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
            }
        }
        if d < 0 as ::core::ffi::c_int {
            x += 1;
            dx += TwoBsquared;
            d += dx;
        }
        rim_pixels = 1 as DWORD;
        y -= 1;
        TopIndex = TopIndex.wrapping_add(width as DWORD);
        BotIndex = BotIndex.wrapping_sub(width as DWORD);
        dy -= TwoAsquared;
        d += Asquared - dy;
    }
    lf_x = (A as ::core::ffi::c_int - x as ::core::ffi::c_int) as COORD;
    rt_x = (A as ::core::ffi::c_int + x as ::core::ffi::c_int) as COORD;
    if SetDepth.0 != 0 {
        memset(
            DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (rt_x as ::core::ffi::c_int - lf_x as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as size_t,
        );
    }
    if lf_x as ::core::ffi::c_int == rt_x as ::core::ffi::c_int {
        *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
            .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) as ::core::ffi::c_int
            + rim_delta as ::core::ffi::c_int) as SBYTE;
    } else {
        loop {
            *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
            if lf_x as ::core::ffi::c_int != rt_x as ::core::ffi::c_int {
                *DepthArray.offset(TopIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                    .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize)
                    as ::core::ffi::c_int + rim_delta as ::core::ffi::c_int) as SBYTE;
            }
            lf_x += 1;
            rt_x -= 1;
            rim_pixels = rim_pixels.wrapping_sub(1);
            if rim_pixels == 0 {
                break;
            }
        }
        while (lf_x as ::core::ffi::c_int) < rt_x as ::core::ffi::c_int {
            *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
            *DepthArray.offset(TopIndex.wrapping_add(rt_x as DWORD) as isize) = (*DepthArray
                .offset(TopIndex.wrapping_add(rt_x as DWORD) as isize)
                as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
            lf_x += 1;
            rt_x -= 1;
        }
        if lf_x as ::core::ffi::c_int == rt_x as ::core::ffi::c_int {
            *DepthArray.offset(TopIndex.wrapping_add(lf_x as DWORD) as isize) = (*DepthArray
                .offset(TopIndex.wrapping_add(lf_x as DWORD) as isize)
                as ::core::ffi::c_int + crater_delta as ::core::ffi::c_int) as SBYTE;
        }
    };
}
unsafe extern "C" fn MakeStorms(
    mut storm_count: COUNT,
    mut DepthArray: *mut SBYTE,
    mut width: COUNT,
    mut height: COUNT,
) {
    let mut i: COUNT = 0;
    let mut storm_r: [RECT; 8] = [RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    }; 8];
    let mut pstorm_r: *mut RECT = ::core::ptr::null_mut::<RECT>();
    i = storm_count;
    pstorm_r = (&raw mut storm_r as *mut RECT).offset(i as isize);
    loop {
        let c2rust_fresh5 = i;
        i = i.wrapping_sub(1);
        if c2rust_fresh5 == 0 {
            break;
        }
        let mut intersect: BOOLEAN = BOOLEAN::FALSE;
        let mut rand_val: DWORD = 0;
        let mut loword: UWORD = 0;
        let mut hiword: UWORD = 0;
        let mut band_delta: SIZE = 0;
        pstorm_r = pstorm_r.offset(-1);
        loop {
            let mut j: COUNT = 0;
            intersect = BOOLEAN::FALSE;
            rand_val = RandomContext_Random(SysGenRNG);
            loword = rand_val as UWORD;
            hiword = (rand_val >> 16 as ::core::ffi::c_int) as UWORD;
            match (hiword as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int & 31 as ::core::ffi::c_int
            {
                0 => {
                    (*pstorm_r).extent.height = (hiword as BYTE as ::core::ffi::c_int
                        % (if optPlanetTexture.0 != 0 {
                            75 as ::core::ffi::c_int
                        } else {
                            67 as ::core::ffi::c_int
                        } >> 2 as ::core::ffi::c_int)
                        + (if optPlanetTexture.0 != 0 {
                            75 as ::core::ffi::c_int
                        } else {
                            67 as ::core::ffi::c_int
                        } >> 2 as ::core::ffi::c_int)) as COORD;
                }
                1 | 2 | 3 | 4 => {
                    (*pstorm_r).extent.height = (hiword as BYTE as ::core::ffi::c_int
                        % (if optPlanetTexture.0 != 0 {
                            75 as ::core::ffi::c_int
                        } else {
                            67 as ::core::ffi::c_int
                        } >> 3 as ::core::ffi::c_int)
                        + (if optPlanetTexture.0 != 0 {
                            75 as ::core::ffi::c_int
                        } else {
                            67 as ::core::ffi::c_int
                        } >> 3 as ::core::ffi::c_int)) as COORD;
                }
                _ => {
                    (*pstorm_r).extent.height = (hiword as BYTE as ::core::ffi::c_int
                        % (if optPlanetTexture.0 != 0 {
                            75 as ::core::ffi::c_int
                        } else {
                            67 as ::core::ffi::c_int
                        } >> 4 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int)
                        as COORD;
                }
            }
            if (*pstorm_r).extent.height as ::core::ffi::c_int <= 4 as ::core::ffi::c_int
            {
                (*pstorm_r).extent.height = ((*pstorm_r).extent.height
                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as COORD;
            }
            rand_val = RandomContext_Random(SysGenRNG);
            loword = rand_val as UWORD;
            hiword = (rand_val >> 16 as ::core::ffi::c_int) as UWORD;
            (*pstorm_r).extent.width = ((*pstorm_r).extent.height as ::core::ffi::c_int
                + loword as BYTE as ::core::ffi::c_int
                    % (*pstorm_r).extent.height as ::core::ffi::c_int) as COORD;
            (*pstorm_r).corner.x = ((loword as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                % (if optPlanetTexture.0 != 0 {
                    243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                } else {
                    210 as ::core::ffi::c_int
                } - (*pstorm_r).extent.width as ::core::ffi::c_int)) as COORD;
            (*pstorm_r).corner.y = (loword as BYTE as ::core::ffi::c_int
                % (if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                } - (*pstorm_r).extent.height as ::core::ffi::c_int)) as COORD;
            (*pstorm_r).corner.x = ((*pstorm_r).corner.x as ::core::ffi::c_int
                * width as ::core::ffi::c_int
                / if optPlanetTexture.0 != 0 {
                    243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                } else {
                    210 as ::core::ffi::c_int
                }) as COORD;
            (*pstorm_r).extent.width = ((*pstorm_r).extent.width as ::core::ffi::c_int
                * width as ::core::ffi::c_int
                / if optPlanetTexture.0 != 0 {
                    243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                } else {
                    210 as ::core::ffi::c_int
                }) as COORD;
            (*pstorm_r).corner.y = ((*pstorm_r).corner.y as ::core::ffi::c_int
                * height as ::core::ffi::c_int
                / if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                }) as COORD;
            (*pstorm_r).extent.height = ((*pstorm_r).extent.height as ::core::ffi::c_int
                * height as ::core::ffi::c_int
                / if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                }) as COORD;
            j = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
            while (j as ::core::ffi::c_int) < storm_count as ::core::ffi::c_int {
                let mut x: COORD = 0;
                let mut y: COORD = 0;
                let mut w: SIZE = 0;
                let mut h: SIZE = 0;
                x = (storm_r[j as usize].corner.x as ::core::ffi::c_int
                    - (*pstorm_r).corner.x as ::core::ffi::c_int) as COORD;
                y = (storm_r[j as usize].corner.y as ::core::ffi::c_int
                    - (*pstorm_r).corner.y as ::core::ffi::c_int) as COORD;
                w = (x as ::core::ffi::c_int
                    + storm_r[j as usize].extent.width as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as SIZE;
                h = (y as ::core::ffi::c_int
                    + storm_r[j as usize].extent.height as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as SIZE;
                intersect = BOOLEAN(
                    (w as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && h as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (x as ::core::ffi::c_int)
                            < (*pstorm_r).extent.width as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                        && (y as ::core::ffi::c_int)
                            < (*pstorm_r).extent.height as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
                if intersect.0 != 0 {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if intersect.0 == 0 {
                break;
            }
        }
        MakeCrater(pstorm_r, DepthArray, 6 as SIZE, 6 as SIZE, BOOLEAN::FALSE, width);
        (*pstorm_r).corner.x += 1;
        (*pstorm_r).corner.y += 1;
        (*pstorm_r).extent.width = ((*pstorm_r).extent.width as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        (*pstorm_r).extent.height = ((*pstorm_r).extent.height as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        band_delta = ((loword as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            & ((3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + 20 as ::core::ffi::c_int) as SIZE;
        MakeCrater(pstorm_r, DepthArray, band_delta, band_delta, BOOLEAN::TRUE, width);
        (*pstorm_r).corner.x += 1;
        (*pstorm_r).corner.y += 1;
        (*pstorm_r).extent.width = ((*pstorm_r).extent.width as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        (*pstorm_r).extent.height = ((*pstorm_r).extent.height as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        band_delta = (band_delta as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as SIZE;
        if (*pstorm_r).extent.width as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && (*pstorm_r).extent.height as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            MakeCrater(
                pstorm_r,
                DepthArray,
                band_delta,
                band_delta,
                BOOLEAN::TRUE,
                width,
            );
            (*pstorm_r).corner.x += 1;
            (*pstorm_r).corner.y += 1;
            (*pstorm_r).extent.width = ((*pstorm_r).extent.width as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
            (*pstorm_r).extent.height = ((*pstorm_r).extent.height as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
        }
        band_delta = (band_delta as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as SIZE;
        if (*pstorm_r).extent.width as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && (*pstorm_r).extent.height as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            MakeCrater(
                pstorm_r,
                DepthArray,
                band_delta,
                band_delta,
                BOOLEAN::TRUE,
                width,
            );
            (*pstorm_r).corner.x += 1;
            (*pstorm_r).corner.y += 1;
            (*pstorm_r).extent.width = ((*pstorm_r).extent.width as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
            (*pstorm_r).extent.height = ((*pstorm_r).extent.height as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
        }
        band_delta = (band_delta as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            as SIZE;
        MakeCrater(pstorm_r, DepthArray, band_delta, band_delta, BOOLEAN::TRUE, width);
    };
}
unsafe extern "C" fn MakeGasGiant(
    mut num_bands: COUNT,
    mut DepthArray: *mut SBYTE,
    mut pRect: *mut RECT,
    mut depth_delta: SIZE,
) {
    let mut last_y: COORD = 0;
    let mut next_y: COORD = 0;
    let mut band_error: SIZE = 0;
    let mut band_bump: SIZE = 0;
    let mut band_delta: SIZE = 0;
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    let mut band_height: COUNT = 0;
    let mut lpDst: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut loword: UWORD = 0;
    let mut hiword: UWORD = 0;
    let mut rand_val: DWORD = 0;
    band_height = (if optPlanetTexture.0 != 0 {
        75 as ::core::ffi::c_int
    } else {
        67 as ::core::ffi::c_int
    } / num_bands as ::core::ffi::c_int) as COUNT;
    band_bump = (if optPlanetTexture.0 != 0 {
        75 as ::core::ffi::c_int
    } else {
        67 as ::core::ffi::c_int
    } % num_bands as ::core::ffi::c_int) as SIZE;
    band_error = (num_bands as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    lpDst = DepthArray;
    band_delta = (((RandomContext_Random(SysGenRNG) as UWORD as ::core::ffi::c_int
        & 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
        + ((1 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)) as SIZE;
    next_y = 0 as COORD;
    last_y = next_y;
    i = num_bands;
    while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut cur_y: COORD = 0;
        rand_val = RandomContext_Random(SysGenRNG);
        loword = rand_val as UWORD;
        hiword = (rand_val >> 16 as ::core::ffi::c_int) as UWORD;
        next_y = (next_y as ::core::ffi::c_int + band_height as ::core::ffi::c_int)
            as COORD;
        band_error = (band_error as ::core::ffi::c_int - band_bump as ::core::ffi::c_int)
            as SIZE;
        if (band_error as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            next_y += 1;
            band_error = (band_error as ::core::ffi::c_int
                + num_bands as ::core::ffi::c_int) as SIZE;
        }
        if i as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            cur_y = (*pRect).extent.height;
        } else {
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            cur_y = (next_y as ::core::ffi::c_int
                + (band_height as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                - (hiword as BYTE as ::core::ffi::c_int
                    % (band_height as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int)) as COORD;
            cur_y = (cur_y as ::core::ffi::c_int
                * (*pRect).extent.height as ::core::ffi::c_int
                / if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                }) as COORD;
            r.corner.y = 0 as COORD;
            r.corner.x = r.corner.y;
            r.extent.width = (*pRect).extent.width;
            r.extent.height = (5 as ::core::ffi::c_int
                * (*pRect).extent.height as ::core::ffi::c_int
                / if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                }) as COORD;
            DeltaTopography(
                50 as COUNT,
                DepthArray
                    .offset(
                        ((cur_y as ::core::ffi::c_int
                            - (r.extent.height as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int))
                            * r.extent.width as ::core::ffi::c_int) as isize,
                    ),
                &raw mut r,
                depth_delta,
            );
        }
        j = (cur_y as ::core::ffi::c_int - last_y as ::core::ffi::c_int) as COUNT;
        while j as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let mut k: COUNT = 0;
            k = (*pRect).extent.width as COUNT;
            while k as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let c2rust_fresh4 = lpDst;
                lpDst = lpDst.offset(1);
                let c2rust_lvalue_ptr = &raw mut *c2rust_fresh4;
                *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr as ::core::ffi::c_int
                    + band_delta as ::core::ffi::c_int) as SBYTE;
                k = k.wrapping_sub(1);
            }
            j = j.wrapping_sub(1);
        }
        last_y = cur_y;
        band_delta = (band_delta as ::core::ffi::c_int
            + ((((loword as BYTE as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int)
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                * 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
        i = i.wrapping_sub(1);
    }
    MakeStorms(
        (4 as DWORD)
            .wrapping_add(RandomContext_Random(SysGenRNG) & 3 as DWORD)
            .wrapping_add(1 as DWORD) as COUNT,
        DepthArray,
        (*pRect).extent.width as COUNT,
        (*pRect).extent.height as COUNT,
    );
    DitherMap(
        DepthArray,
        (*pRect).extent.width as COUNT,
        (*pRect).extent.height as COUNT,
    );
}
unsafe extern "C" fn ValidateMap(
    mut DepthArray: *mut SBYTE,
    mut width: COUNT,
    mut height: COUNT,
) {
    let mut state: BYTE = 0;
    let mut pixel_count: [BYTE; 2] = [0; 2];
    let mut lb: [BYTE; 2] = [0; 2];
    let mut last_byte: SBYTE = 0;
    let mut i: DWORD = 0;
    let mut lpDst: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    i = (width as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as DWORD;
    lpDst = DepthArray;
    let c2rust_fresh0 = lpDst;
    lpDst = lpDst.offset(1);
    last_byte = *c2rust_fresh0;
    pixel_count[1usize] = 0 as BYTE;
    pixel_count[0usize] = pixel_count[1usize];
    state = pixel_count[0usize];
    loop {
        let c2rust_fresh1 = pixel_count[state as usize];
        pixel_count[state as usize] = pixel_count[state as usize].wrapping_add(1);
        if c2rust_fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            lb[state as usize] = last_byte as BYTE;
        }
        if last_byte as ::core::ffi::c_int > *lpDst as ::core::ffi::c_int {
            if last_byte as ::core::ffi::c_int - *lpDst as ::core::ffi::c_int
                > 128 as ::core::ffi::c_int
            {
                state = (state as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
            }
        } else if *lpDst as ::core::ffi::c_int - last_byte as ::core::ffi::c_int
            > 128 as ::core::ffi::c_int
        {
            state = (state as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
        }
        let c2rust_fresh2 = lpDst;
        lpDst = lpDst.offset(1);
        last_byte = *c2rust_fresh2;
        i = i.wrapping_sub(1);
        if i == 0 {
            break;
        }
    }
    i = (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as DWORD;
    lpDst = DepthArray;
    if pixel_count[0usize] as ::core::ffi::c_int
        > pixel_count[1usize] as ::core::ffi::c_int
    {
        last_byte = lb[0usize] as SBYTE;
    } else {
        last_byte = lb[1usize] as SBYTE;
    }
    loop {
        if last_byte as ::core::ffi::c_int > *lpDst as ::core::ffi::c_int {
            if last_byte as ::core::ffi::c_int - *lpDst as ::core::ffi::c_int
                > 128 as ::core::ffi::c_int
            {
                *lpDst = last_byte;
            }
        } else if *lpDst as ::core::ffi::c_int - last_byte as ::core::ffi::c_int
            > 128 as ::core::ffi::c_int
        {
            *lpDst = last_byte;
        }
        let c2rust_fresh3 = lpDst;
        lpDst = lpDst.offset(1);
        last_byte = *c2rust_fresh3;
        i = i.wrapping_sub(1);
        if i == 0 {
            break;
        }
    };
}
unsafe extern "C" fn planet_orbit_init(
    mut width: COUNT,
    mut height: COUNT,
    mut forOrbit: BOOLEAN,
) {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut spherespanx: COUNT = height;
    let mut shieldradius: COUNT = ((height as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        * (((33 as ::core::ffi::c_int) << resolutionFactor)
            + (if playerInPlanetOrbit() as ::core::ffi::c_int != 0 {
                7 as ::core::ffi::c_int
            } else {
                14 as ::core::ffi::c_int
            } << resolutionFactor)) / ((33 as ::core::ffi::c_int) << resolutionFactor))
        as COUNT;
    let mut shielddiam: COUNT = (((shieldradius as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COUNT;
    let mut diameter: COUNT = (height as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as COUNT;
    let mut i: COUNT = 0;
    (*Orbit).lpTopoData = HCalloc(
        (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as size_t,
    ) as *mut SBYTE;
    (*Orbit).TopoZoomFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    (*Orbit).ObjectFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    if forOrbit.0 != 0 && optScanStyle != 0x2 as ::core::ffi::c_int {
        (*Orbit).TintFrame = CaptureDrawable(
            CreateDrawable(
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
                width as SIZE,
                height as SIZE,
                1 as COUNT,
            ),
        );
    } else {
        (*Orbit).TintFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
    (*Orbit).TintColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    (*Orbit).TopoColors = ::core::ptr::null_mut::<Color>();
    (*Orbit).ScanColors = ::core::ptr::null_mut::<*mut Color>();
    (*Orbit).ScratchArray = HMalloc(
        (::core::mem::size_of::<Color>() as size_t)
            .wrapping_mul(shielddiam as size_t)
            .wrapping_mul(shielddiam as size_t),
    ) as *mut Color;
    (*Orbit).WorkFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    (*Orbit).BackFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    (*Orbit).light_diff = ::core::ptr::null_mut::<*mut DWORD>();
    (*Orbit).map_rotate = ::core::ptr::null_mut::<*mut MAP3D_POINT>();
    (*Orbit).TopoMask = ::core::ptr::null_mut::<FRAME_DESC>();
    (*Orbit).sphereBytes = ::core::ptr::null_mut::<BYTE>();
    (*Orbit).sphereMap = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    (*Orbit).scanType = PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int as COUNT;
    (*Orbit).Shade = ::core::ptr::null_mut::<FRAME_DESC>();
    (*Orbit).ShadeColors = ::core::ptr::null_mut::<Color>();
    if forOrbit.0 == 0 || optScanSphere != 0 {
        (*Orbit).SphereFrame = CaptureDrawable(
            CreateDrawable(
                (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as CREATE_FLAGS as ::core::ffi::c_int) as CREATE_FLAGS,
                diameter as SIZE,
                diameter as SIZE,
                2 as COUNT,
            ),
        );
        (*Orbit).TopoColors = HMalloc(
            (::core::mem::size_of::<Color>() as size_t)
                .wrapping_mul(
                    (height as ::core::ffi::c_int
                        * (width as ::core::ffi::c_int
                            + spherespanx as ::core::ffi::c_int)) as size_t,
                ),
        ) as *mut Color;
        if forOrbit.0 != 0 && isPC(optScanStyle).0 != 0 && isPC(optTintPlanSphere).0 != 0
            && use3DOSpheres.0 == 0
        {
            (*Orbit).ScanColors = HMalloc(
                (::core::mem::size_of::<*mut Color>() as size_t)
                    .wrapping_mul(
                        PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int as size_t,
                    ),
            ) as *mut *mut Color;
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int)
                < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
            {
                *(*Orbit).ScanColors.offset(i as isize) = HMalloc(
                    (::core::mem::size_of::<Color>() as size_t)
                        .wrapping_mul(
                            (height as ::core::ffi::c_int
                                * (width as ::core::ffi::c_int
                                    + spherespanx as ::core::ffi::c_int)) as size_t,
                        ),
                ) as *mut Color;
                i = i.wrapping_add(1);
            }
        }
        if use3DOSpheres.0 == 0 {
            (*Orbit).light_diff = HMalloc(
                (::core::mem::size_of::<*mut DWORD>() as size_t)
                    .wrapping_mul(diameter as size_t),
            ) as *mut *mut DWORD;
        }
        (*Orbit).map_rotate = HMalloc(
            (::core::mem::size_of::<*mut MAP3D_POINT>() as size_t)
                .wrapping_mul(diameter as size_t),
        ) as *mut *mut MAP3D_POINT;
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < diameter as ::core::ffi::c_int {
            if use3DOSpheres.0 == 0 {
                *(*Orbit).light_diff.offset(i as isize) = HMalloc(
                    (::core::mem::size_of::<DWORD>() as size_t)
                        .wrapping_mul(diameter as size_t),
                ) as *mut DWORD;
            }
            *(*Orbit).map_rotate.offset(i as isize) = HMalloc(
                (::core::mem::size_of::<MAP3D_POINT>() as size_t)
                    .wrapping_mul(diameter as size_t),
            ) as *mut MAP3D_POINT;
            i = i.wrapping_add(1);
        }
        if use3DOSpheres.0 != 0 {
            (*Orbit).Shade = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicInstance(b"graphics.3do.shade\0".as_ptr() as RESOURCE), ())
                    as DRAWABLE,
            );
            (*Orbit).ShadeColors = HMalloc(
                (::core::mem::size_of::<Color>() as size_t)
                    .wrapping_mul((*(*Orbit).Shade).Bounds.width as size_t)
                    .wrapping_mul((*(*Orbit).Shade).Bounds.height as size_t),
            ) as *mut Color;
        }
    } else {
        (*Orbit).SphereFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.dossphere\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
        (*Orbit).TopoMask = CaptureDrawable(
            CreateDrawable(
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
                width as SIZE,
                height as SIZE,
                1 as COUNT,
            ),
        );
        (*Orbit).sphereBytes = HMalloc(
            (::core::mem::size_of::<BYTE>() as size_t)
                .wrapping_mul((*(*Orbit).SphereFrame).Bounds.width as size_t)
                .wrapping_mul((*(*Orbit).SphereFrame).Bounds.height as size_t),
        ) as *mut BYTE;
        (*Orbit).sphereMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(b"dossphere.colortable\0".as_ptr() as RESOURCE), ()),
        ) as COLORMAP;
        SetColorMap(GetColorMapAddress((*Orbit).sphereMap));
    };
}
unsafe extern "C" fn frandom() -> ::core::ffi::c_uint {
    static mut seed: ::core::ffi::c_uint = 0x12345678 as ::core::ffi::c_int
        as ::core::ffi::c_uint;
    if seed == 0 as ::core::ffi::c_uint {
        seed = 15807 as ::core::ffi::c_uint;
    }
    seed = (seed >> 4 as ::core::ffi::c_int).wrapping_mul(227 as ::core::ffi::c_uint);
    return seed;
}
#[inline]
unsafe extern "C" fn TopoVarianceFactor(
    mut step: ::core::ffi::c_int,
    mut allowed: ::core::ffi::c_int,
    mut min: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (abs(step) * allowed >> 8 as ::core::ffi::c_int) + min;
}
#[inline]
unsafe extern "C" fn TopoVarianceCalc(
    mut factor: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if factor == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int
    } else {
        return frandom()
            .wrapping_rem(factor as ::core::ffi::c_uint)
            .wrapping_sub((factor >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    };
}
unsafe extern "C" fn TopoScale4x(
    mut pDstTopo: *mut SBYTE,
    mut pSrcTopo: *mut SBYTE,
    mut num_faults: ::core::ffi::c_int,
    mut fault_var: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let w: ::core::ffi::c_int = if is3DO(optSuperPC).0 != 0 {
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
        } << resolutionFactor;
    let h: ::core::ffi::c_int = (67 as ::core::ffi::c_int) << resolutionFactor;
    let spitch: ::core::ffi::c_int = if is3DO(optSuperPC).0 != 0 {
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
        } << resolutionFactor;
    let dpitch: ::core::ffi::c_int = (if is3DO(optSuperPC).0 != 0 {
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
        } << resolutionFactor) * 4 as ::core::ffi::c_int;
    let mut pSrc: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut pDst: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut prevrow: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut prow: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut elev: [[::core::ffi::c_int; 5]; 5] = [[0; 5]; 5];
    let mut var_allow: ::core::ffi::c_int = 0;
    let mut var_min: ::core::ffi::c_int = 0;
    static mut fill_lines: [[line_def_t; 6]; 4] = [
        [
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 2 as ::core::ffi::c_int,
                x1: 2 as ::core::ffi::c_int,
                y1: 0 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: -1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 3 as ::core::ffi::c_int,
                x1: 3 as ::core::ffi::c_int,
                y1: 0 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: -1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 4 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 0 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: -1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 1 as ::core::ffi::c_int,
                y0: 4 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 1 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: -1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 2 as ::core::ffi::c_int,
                y0: 4 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 2 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: -1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: -1 as ::core::ffi::c_int,
                y0: -1 as ::core::ffi::c_int,
                x1: -1 as ::core::ffi::c_int,
                y1: -1 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
        ],
        [
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 2 as ::core::ffi::c_int,
                x1: 2 as ::core::ffi::c_int,
                y1: 4 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 1 as ::core::ffi::c_int,
                x1: 3 as ::core::ffi::c_int,
                y1: 4 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 0 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 4 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 1 as ::core::ffi::c_int,
                y0: 0 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 3 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 2 as ::core::ffi::c_int,
                y0: 0 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 2 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: -1 as ::core::ffi::c_int,
                y0: -1 as ::core::ffi::c_int,
                x1: -1 as ::core::ffi::c_int,
                y1: -1 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
        ],
        [
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 1 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 1 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 2 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 2 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0 as ::core::ffi::c_int,
                y0: 3 as ::core::ffi::c_int,
                x1: 4 as ::core::ffi::c_int,
                y1: 3 as ::core::ffi::c_int,
                dx: 1 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: -1 as ::core::ffi::c_int,
                y0: -1 as ::core::ffi::c_int,
                x1: -1 as ::core::ffi::c_int,
                y1: -1 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0,
                y0: 0,
                x1: 0,
                y1: 0,
                dx: 0,
                dy: 0,
            },
            line_def_t {
                x0: 0,
                y0: 0,
                x1: 0,
                y1: 0,
                dx: 0,
                dy: 0,
            },
        ],
        [
            line_def_t {
                x0: 1 as ::core::ffi::c_int,
                y0: 0 as ::core::ffi::c_int,
                x1: 1 as ::core::ffi::c_int,
                y1: 4 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 2 as ::core::ffi::c_int,
                y0: 0 as ::core::ffi::c_int,
                x1: 2 as ::core::ffi::c_int,
                y1: 4 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 3 as ::core::ffi::c_int,
                y0: 0 as ::core::ffi::c_int,
                x1: 3 as ::core::ffi::c_int,
                y1: 4 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 1 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: -1 as ::core::ffi::c_int,
                y0: -1 as ::core::ffi::c_int,
                x1: -1 as ::core::ffi::c_int,
                y1: -1 as ::core::ffi::c_int,
                dx: 0 as ::core::ffi::c_int,
                dy: 0 as ::core::ffi::c_int,
            },
            line_def_t {
                x0: 0,
                y0: 0,
                x1: 0,
                y1: 0,
                dx: 0,
                dy: 0,
            },
            line_def_t {
                x0: 0,
                y0: 0,
                x1: 0,
                y1: 0,
                dx: 0,
                dy: 0,
            },
        ],
    ];
    prevrow = HMalloc(
        (((if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor) * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    var_allow = (num_faults << 8 as ::core::ffi::c_int) / 250 as ::core::ffi::c_int;
    var_min = fault_var << 8 as ::core::ffi::c_int;
    pSrc = pSrcTopo;
    prow = prevrow;
    *prow.offset(0isize) = (*pSrc.offset(0isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int;
    x = 0 as ::core::ffi::c_int;
    while x < w {
        let mut x2: ::core::ffi::c_int = 0;
        let mut val: ::core::ffi::c_int = 0;
        let mut step: ::core::ffi::c_int = 0;
        let mut rndfact: ::core::ffi::c_int = 0;
        if x < w - 1 as ::core::ffi::c_int {
            *prow.offset(4isize) = (*pSrc.offset(1isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int;
        } else {
            *prow.offset(4isize) = (*pSrc
                .offset((1 as ::core::ffi::c_int - spitch) as isize)
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        val = *prow.offset(0isize);
        step = (*prow.offset(4isize) - val)
            / (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
        rndfact = TopoVarianceFactor(step, var_allow, var_min);
        x2 = 1 as ::core::ffi::c_int;
        val += step;
        while x2 < 4 as ::core::ffi::c_int {
            *prow.offset(x2 as isize) = val + TopoVarianceCalc(rndfact);
            x2 += 1;
            val += step;
        }
        x += 1;
        pSrc = pSrc.offset(1);
        prow = prow.offset(4 as ::core::ffi::c_int as isize);
    }
    pSrc = pSrcTopo;
    pDst = pDstTopo;
    y = 0 as ::core::ffi::c_int;
    while y < h {
        let mut x2_0: ::core::ffi::c_int = 0;
        let mut y2: ::core::ffi::c_int = 0;
        let mut p: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
        let mut val_0: ::core::ffi::c_int = 0;
        let mut step_0: ::core::ffi::c_int = 0;
        let mut rndfact_0: ::core::ffi::c_int = 0;
        let mut pld: *const line_def_t = ::core::ptr::null::<line_def_t>();
        prow = prevrow;
        elev[4usize][0usize] = *prow.offset(0isize);
        if y < h - 1 as ::core::ffi::c_int {
            elev[4usize][4usize] = (*pSrc.offset(spitch as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int;
        } else {
            elev[4usize][4usize] = elev[4usize][0usize];
        }
        val_0 = elev[4usize][0usize];
        step_0 = (elev[4usize][4usize] - val_0)
            / (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
        rndfact_0 = TopoVarianceFactor(step_0, var_allow, var_min);
        y2 = 1 as ::core::ffi::c_int;
        val_0 += step_0;
        while y2 < 4 as ::core::ffi::c_int {
            elev[4usize][y2 as usize] = val_0 + TopoVarianceCalc(rndfact_0);
            y2 += 1;
            val_0 += step_0;
        }
        x = 0 as ::core::ffi::c_int;
        while x < w {
            x2_0 = 0 as ::core::ffi::c_int;
            while x2_0 <= 4 as ::core::ffi::c_int {
                elev[x2_0 as usize][0usize] = *prow.offset(x2_0 as isize);
                x2_0 += 1;
            }
            y2 = 1 as ::core::ffi::c_int;
            while y2 <= 4 as ::core::ffi::c_int {
                elev[0usize][y2 as usize] = elev[4usize][y2 as usize];
                y2 += 1;
            }
            if y < h - 1 as ::core::ffi::c_int {
                if x < w - 1 as ::core::ffi::c_int {
                    elev[4usize][4usize] = (*pSrc
                        .offset((1 as ::core::ffi::c_int + spitch) as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
                } else {
                    elev[4usize][4usize] = (*pSrc.offset(1isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int;
                }
            } else {
                elev[4usize][4usize] = elev[4usize][0usize];
            }
            val_0 = elev[0usize][4usize];
            step_0 = (elev[4usize][4usize] - val_0)
                / (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
            rndfact_0 = TopoVarianceFactor(step_0, var_allow, var_min);
            x2_0 = 1 as ::core::ffi::c_int;
            val_0 += step_0;
            while x2_0 < 4 as ::core::ffi::c_int {
                elev[x2_0 as usize][4usize] = val_0 + TopoVarianceCalc(rndfact_0);
                x2_0 += 1;
                val_0 += step_0;
            }
            val_0 = elev[4usize][0usize];
            step_0 = (elev[4usize][4usize] - val_0)
                / (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
            rndfact_0 = TopoVarianceFactor(step_0, var_allow, var_min);
            y2 = 1 as ::core::ffi::c_int;
            val_0 += step_0;
            while y2 < 4 as ::core::ffi::c_int {
                elev[4usize][y2 as usize] = val_0 + TopoVarianceCalc(rndfact_0);
                y2 += 1;
                val_0 += step_0;
            }
            pld = &raw const *(&raw const fill_lines as *const [line_def_t; 6])
                .offset((frandom() & 3 as ::core::ffi::c_uint) as isize)
                as *const line_def_t as *const line_def_t;
            while (*pld).x0 >= 0 as ::core::ffi::c_int {
                let mut num_steps: ::core::ffi::c_int = 0;
                x2_0 = (*pld).x0;
                y2 = (*pld).y0;
                val_0 = elev[x2_0 as usize][y2 as usize];
                num_steps = (*pld).x1 - (*pld).x0;
                if num_steps == 0 as ::core::ffi::c_int {
                    num_steps = (*pld).y1 - (*pld).y0;
                }
                step_0 = (elev[(*pld).x1 as usize][(*pld).y1 as usize] - val_0)
                    / num_steps;
                rndfact_0 = TopoVarianceFactor(step_0, var_allow, var_min);
                x2_0 += (*pld).dx;
                y2 += (*pld).dy;
                val_0 += step_0;
                while x2_0 != (*pld).x1 || y2 != (*pld).y1 {
                    elev[x2_0 as usize][y2 as usize] = val_0
                        + TopoVarianceCalc(rndfact_0);
                    x2_0 += (*pld).dx;
                    y2 += (*pld).dy;
                    val_0 += step_0;
                }
                pld = pld.offset(1);
            }
            y2 = 0 as ::core::ffi::c_int;
            while y2 < 4 as ::core::ffi::c_int {
                p = pDst.offset((y2 * dpitch) as isize);
                x2_0 = 0 as ::core::ffi::c_int;
                while x2_0 < 4 as ::core::ffi::c_int {
                    let mut e: ::core::ffi::c_int = elev[x2_0 as usize][y2 as usize]
                        >> 8 as ::core::ffi::c_int;
                    if e > 127 as ::core::ffi::c_int {
                        e = 127 as ::core::ffi::c_int;
                    } else if e < -128 as ::core::ffi::c_int {
                        e = -128 as ::core::ffi::c_int;
                    }
                    *p = e as SBYTE;
                    x2_0 += 1;
                    p = p.offset(1);
                }
                y2 += 1;
            }
            x2_0 = 0 as ::core::ffi::c_int;
            while x2_0 < 4 as ::core::ffi::c_int {
                *prow.offset(x2_0 as isize) = elev[x2_0 as usize][4usize];
                x2_0 += 1;
            }
            x += 1;
            pSrc = pSrc.offset(1);
            pDst = pDst.offset(4 as ::core::ffi::c_int as isize);
            prow = prow.offset(4 as ::core::ffi::c_int as isize);
        }
        *prow.offset(0isize) = elev[4usize][4usize];
        y += 1;
        pDst = pDst.offset((dpitch * 3 as ::core::ffi::c_int) as isize);
    }
    HFree(prevrow as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn get_vblock_avg(
    mut pblk: *mut elev_block_t,
    mut pTopo: *mut SBYTE,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: COUNT,
    mut height: COUNT,
) {
    let mut elev: *mut SBYTE = pTopo;
    let mut y0: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
    let mut max: ::core::ffi::c_int = -127 as ::core::ffi::c_int;
    let mut avg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut total_weight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    x = (x + width as ::core::ffi::c_int) % width as ::core::ffi::c_int;
    y0 = y
        - ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
            / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int);
    y1 = y
        + ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
            / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int);
    if y0 < 0 as ::core::ffi::c_int {
        y0 = 0 as ::core::ffi::c_int;
    }
    if y1 > height as ::core::ffi::c_int {
        y1 = height as ::core::ffi::c_int;
    }
    elev = pTopo.offset((y0 * height as ::core::ffi::c_int) as isize).offset(x as isize);
    i = y0;
    while i < y1 {
        let mut delta: ::core::ffi::c_int = abs(i - y);
        let mut weight: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
        let mut v: ::core::ffi::c_int = *elev as ::core::ffi::c_int;
        if delta
            >= ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                / 3 as ::core::ffi::c_int
        {
            weight = weight
                * (((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                    / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) - delta + 1 as ::core::ffi::c_int)
                / (((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                    / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                    - ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                        / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                        / 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int);
        }
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
        avg += (*pblk).avg * weight;
        total_weight += weight;
        i += 1;
        elev = elev.offset(height as ::core::ffi::c_int as isize);
    }
    avg /= total_weight;
    (*pblk).min = min;
    (*pblk).max = max;
    (*pblk).avg = avg / (y1 - y0);
}
unsafe extern "C" fn GenerateLightMap(
    mut pTopo: *mut SBYTE,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut vblocks: [elev_block_t; 27] = [elev_block_t {
        min: 0,
        max: 0,
        avg: 0,
    }; 27];
    let mut elev: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut med: ::core::ffi::c_int = 0;
    let mut sfact: ::core::ffi::c_int = 0;
    let mut spread: ::core::ffi::c_int = 0;
    min = 127 as ::core::ffi::c_int;
    max = -128 as ::core::ffi::c_int;
    elev = pTopo;
    while elev != pTopo.offset((w * h) as isize) {
        if *elev as ::core::ffi::c_int > max {
            max = *elev as ::core::ffi::c_int;
        }
        if (*elev as ::core::ffi::c_int) < min {
            min = *elev as ::core::ffi::c_int;
        }
        elev = elev.offset(1);
    }
    med = (min + max) / 2 as ::core::ffi::c_int;
    spread = max - med;
    if spread == 0 as ::core::ffi::c_int {
        if max != 0 as ::core::ffi::c_int {
            memset(
                pTopo as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (w * h) as size_t,
            );
        }
        return;
    }
    if spread < 10 as ::core::ffi::c_int {
        sfact = 30 as ::core::ffi::c_int;
    } else if spread < 30 as ::core::ffi::c_int {
        sfact = 60 as ::core::ffi::c_int;
    } else {
        sfact = 100 as ::core::ffi::c_int;
    }
    elev = pTopo;
    while elev != pTopo.offset((w * h) as isize) {
        *elev = ((*elev as ::core::ffi::c_int - med) * sfact / spread) as SBYTE;
        elev = elev.offset(1);
    }
    y = 0 as ::core::ffi::c_int;
    elev = pTopo;
    while y < h {
        let mut pblk: *mut elev_block_t = ::core::ptr::null_mut::<elev_block_t>();
        let mut i: ::core::ffi::c_int = 0;
        i = -((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
            / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int);
        while i
            < (67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            pblk = (&raw mut vblocks as *mut elev_block_t)
                .offset(
                    ((i
                        + (2 as ::core::ffi::c_int
                            * ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                                / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                        % (2 as ::core::ffi::c_int
                            * ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                                / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                        as isize,
                );
            get_vblock_avg(pblk, pTopo, i, y, w as COUNT, h as COUNT);
            i += 1;
        }
        x = 0 as ::core::ffi::c_int;
        while x < w {
            let mut avg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut total_weight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            min = 127 as ::core::ffi::c_int;
            max = -127 as ::core::ffi::c_int;
            pblk = (&raw mut vblocks as *mut elev_block_t)
                .offset(
                    ((x
                        + ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                            / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int))
                        % (2 as ::core::ffi::c_int
                            * ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                                / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                        as isize,
                );
            get_vblock_avg(
                pblk,
                pTopo,
                x
                    + ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                        / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int),
                y,
                w as COUNT,
                h as COUNT,
            );
            i = x
                - ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                    / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            while i
                <= x
                    + ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                        / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
            {
                let mut delta: ::core::ffi::c_int = abs(i - x);
                let mut weight: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
                pblk = (&raw mut vblocks as *mut elev_block_t)
                    .offset(
                        ((i
                            + (2 as ::core::ffi::c_int
                                * ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                                    / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                            % (2 as ::core::ffi::c_int
                                * ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                                    / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                            as isize,
                    );
                if delta
                    >= ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                        / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                        / 3 as ::core::ffi::c_int
                {
                    weight = weight
                        * (((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                            / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int) - delta
                            + 1 as ::core::ffi::c_int)
                        / (((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                            / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            - ((67 as ::core::ffi::c_int + 64 as ::core::ffi::c_int)
                                / 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                                / 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int);
                }
                if (*pblk).max > max {
                    max = (*pblk).max;
                }
                if (*pblk).min < min {
                    min = (*pblk).min;
                }
                avg += (*pblk).avg * weight;
                total_weight += weight;
                i += 1;
            }
            avg /= total_weight;
            med = (min + max) / 2 as ::core::ffi::c_int;
            med = (med + avg) / 2 as ::core::ffi::c_int;
            *elev = (*elev as ::core::ffi::c_int - med) as SBYTE;
            x += 1;
            elev = elev.offset(1);
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_color_resources(
    mut pPlanetDesc: *mut PLANET_DESC,
    mut PlanDataPtr: *const PlanetFrame,
    mut PlanetInfo: *mut PLANET_INFO,
    mut dosshielded: BOOLEAN,
    mut LoadCustomColorTable: BOOLEAN,
) {
    if LoadCustomColorTable.0 != 0 {
        (*pSolarSysState).OrbitalCMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(GetAltColorMap(pPlanetDesc)), ()),
        ) as COLORMAP;
        if !(*pSolarSysState).OrbitalCMap.is_null() {
            (*pSolarSysState).XlatRef = CaptureStringTable(
                uqm_lua_checked!(LoadStringTableInstance(
                    b"planet.specialcolormap.translatetable\0".as_ptr() as RESOURCE,
                ), ()),
            );
            (*pSolarSysState).XlatPtr = GetStringAddress((*pSolarSysState).XlatRef)
                as *const ::core::ffi::c_void;
            return;
        }
    }
    (*pSolarSysState).OrbitalCMap = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(
            if dosshielded.0 != 0 {
                b"planet.shielded.colortable\0".as_ptr() as RESOURCE
            } else {
                (*PlanDataPtr).CMapInstance
            },
        ), ()),
    ) as COLORMAP;
    (*pSolarSysState).XlatRef = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance((*PlanDataPtr).XlatTabInstance), ()),
    );
    if (*PlanetInfo).SurfaceTemperature as ::core::ffi::c_int > 100 as ::core::ffi::c_int
    {
        (*pSolarSysState).OrbitalCMap = SetAbsStringTableIndex(
            (*pSolarSysState).OrbitalCMap,
            2 as COUNT,
        ) as COLORMAP;
        (*pSolarSysState).XlatRef = SetAbsStringTableIndex(
            (*pSolarSysState).XlatRef,
            2 as COUNT,
        );
    } else if (*PlanetInfo).SurfaceTemperature as ::core::ffi::c_int
        > -40 as ::core::ffi::c_int
    {
        (*pSolarSysState).OrbitalCMap = SetAbsStringTableIndex(
            (*pSolarSysState).OrbitalCMap,
            1 as COUNT,
        ) as COLORMAP;
        (*pSolarSysState).XlatRef = SetAbsStringTableIndex(
            (*pSolarSysState).XlatRef,
            1 as COUNT,
        );
    }
    (*pSolarSysState).XlatPtr = GetStringAddress((*pSolarSysState).XlatRef)
        as *const ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn generate_surface_frame(
    mut width: COUNT,
    mut height: COUNT,
    mut Orbit: *mut PLANET_ORBIT,
    mut PlanDataPtr: *const PlanetFrame,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut i: COUNT = 0;
    r.corner.y = 0 as COORD;
    r.corner.x = r.corner.y;
    r.extent.width = width as COORD;
    r.extent.height = height as COORD;
    memset(
        (*Orbit).lpTopoData as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as size_t,
    );
    match ((*PlanDataPtr).Type as ::core::ffi::c_int & 0xc as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int
    {
        8 => {
            MakeGasGiant(
                (*PlanDataPtr).num_faults,
                (*Orbit).lpTopoData,
                &raw mut r,
                (*PlanDataPtr).fault_depth,
            );
        }
        0 | 4 => {
            if (*PlanDataPtr).num_faults != 0 {
                DeltaTopography(
                    (*PlanDataPtr).num_faults,
                    (*Orbit).lpTopoData,
                    &raw mut r,
                    (*PlanDataPtr).fault_depth,
                );
            }
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int)
                < (*PlanDataPtr).num_blemishes as ::core::ffi::c_int
            {
                let mut crater_r: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                let mut loword: UWORD = 0;
                loword = RandomContext_Random(SysGenRNG) as UWORD;
                match (loword as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int & 31 as ::core::ffi::c_int
                {
                    0 => {
                        crater_r.extent.width = (loword as BYTE as ::core::ffi::c_int
                            % (if optPlanetTexture.0 != 0 {
                                75 as ::core::ffi::c_int
                            } else {
                                67 as ::core::ffi::c_int
                            } >> 2 as ::core::ffi::c_int)
                            + (if optPlanetTexture.0 != 0 {
                                75 as ::core::ffi::c_int
                            } else {
                                67 as ::core::ffi::c_int
                            } >> 2 as ::core::ffi::c_int)) as COORD;
                    }
                    1 | 2 | 3 | 4 => {
                        crater_r.extent.width = (loword as BYTE as ::core::ffi::c_int
                            % (if optPlanetTexture.0 != 0 {
                                75 as ::core::ffi::c_int
                            } else {
                                67 as ::core::ffi::c_int
                            } >> 3 as ::core::ffi::c_int)
                            + (if optPlanetTexture.0 != 0 {
                                75 as ::core::ffi::c_int
                            } else {
                                67 as ::core::ffi::c_int
                            } >> 3 as ::core::ffi::c_int)) as COORD;
                    }
                    _ => {
                        crater_r.extent.width = (loword as BYTE as ::core::ffi::c_int
                            % (if optPlanetTexture.0 != 0 {
                                75 as ::core::ffi::c_int
                            } else {
                                67 as ::core::ffi::c_int
                            } >> 4 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int)
                            as COORD;
                    }
                }
                loword = RandomContext_Random(SysGenRNG) as UWORD;
                crater_r.extent.height = crater_r.extent.width;
                crater_r.corner.x = ((loword as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    % (if optPlanetTexture.0 != 0 {
                        243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    } else {
                        210 as ::core::ffi::c_int
                    } - crater_r.extent.width as ::core::ffi::c_int)) as COORD;
                crater_r.corner.y = (loword as BYTE as ::core::ffi::c_int
                    % (if optPlanetTexture.0 != 0 {
                        75 as ::core::ffi::c_int
                    } else {
                        67 as ::core::ffi::c_int
                    } - crater_r.extent.height as ::core::ffi::c_int)) as COORD;
                crater_r.extent.width = (crater_r.extent.width as ::core::ffi::c_int
                    * height as ::core::ffi::c_int
                    / if optPlanetTexture.0 != 0 {
                        75 as ::core::ffi::c_int
                    } else {
                        67 as ::core::ffi::c_int
                    }) as COORD;
                crater_r.extent.height = crater_r.extent.width;
                crater_r.corner.x = (crater_r.corner.x as ::core::ffi::c_int
                    * width as ::core::ffi::c_int
                    / if optPlanetTexture.0 != 0 {
                        243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    } else {
                        210 as ::core::ffi::c_int
                    }) as COORD;
                crater_r.corner.y = (crater_r.corner.y as ::core::ffi::c_int
                    * height as ::core::ffi::c_int
                    / if optPlanetTexture.0 != 0 {
                        75 as ::core::ffi::c_int
                    } else {
                        67 as ::core::ffi::c_int
                    }) as COORD;
                MakeCrater(
                    &raw mut crater_r,
                    (*Orbit).lpTopoData,
                    (((*PlanDataPtr).fault_depth as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) as SIZE,
                    -(((*PlanDataPtr).fault_depth as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) as SIZE,
                    BOOLEAN::FALSE,
                    width,
                );
                i = i.wrapping_add(1);
            }
            if ((*PlanDataPtr).Type as ::core::ffi::c_int & 0xc as ::core::ffi::c_int)
                as BYTE as ::core::ffi::c_int
                == (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            {
                DitherMap((*Orbit).lpTopoData, width, height);
            }
            ValidateMap((*Orbit).lpTopoData, width, height);
        }
        _ => {}
    }
    (*pSolarSysState).TopoFrame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            width as SIZE,
            height as SIZE,
            1 as COUNT,
        ),
    );
    uqm_lua_checked!(RenderTopography(
        (*pSolarSysState).TopoFrame,
        (*Orbit).lpTopoData,
        width as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        BOOLEAN::FALSE,
        ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn GetPlanetTopography(
    mut pPlanetDesc: *mut PLANET_DESC,
    mut SurfDefFrame: FRAME,
) {
    let mut PlanDataPtr: *const PlanetFrame = ::core::ptr::null::<PlanetFrame>();
    let mut PlanetInfo: *mut PLANET_INFO = &raw mut (*pSolarSysState).SysInfo.PlanetInfo;
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut width: COUNT = 0;
    let mut height: COUNT = 0;
    let mut shielded: BOOLEAN = BOOLEAN(
        ((*pPlanetDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    width = (if is3DO(optSuperPC).0 != 0 {
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
        } << resolutionFactor) as COUNT;
    height = ((67 as ::core::ffi::c_int) << resolutionFactor) as COUNT;
    PlanDataPtr = PlanData
        .offset(
            ((*pPlanetDesc).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)) as isize,
        );
    if !SurfDefFrame.is_null() {
        if (*SurfDefFrame).Bounds.width as ::core::ffi::c_int
            != width as ::core::ffi::c_int
            || (*SurfDefFrame).Bounds.height as ::core::ffi::c_int
                != height as ::core::ffi::c_int
        {
            (*pSolarSysState).TopoFrame = CaptureDrawable(
                RescaleFrame(
                    SurfDefFrame,
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                ),
            );
        } else {
            (*pSolarSysState).TopoFrame = CaptureDrawable(CloneFrame(SurfDefFrame));
        }
    } else {
        (*pSolarSysState).OrbitalCMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(
                if shielded.0 != 0 && useDosSpheres.0 != 0 {
                    b"planet.shielded.colortable\0".as_ptr() as RESOURCE
                } else {
                    (*PlanDataPtr).CMapInstance
                },
            ), ()),
        ) as COLORMAP;
        (*pSolarSysState).XlatRef = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance((*PlanDataPtr).XlatTabInstance), ()),
        );
        if (*PlanetInfo).SurfaceTemperature as ::core::ffi::c_int
            > 100 as ::core::ffi::c_int
        {
            (*pSolarSysState).OrbitalCMap = SetAbsStringTableIndex(
                (*pSolarSysState).OrbitalCMap,
                2 as COUNT,
            ) as COLORMAP;
            (*pSolarSysState).XlatRef = SetAbsStringTableIndex(
                (*pSolarSysState).XlatRef,
                2 as COUNT,
            );
        } else if (*PlanetInfo).SurfaceTemperature as ::core::ffi::c_int
            > -40 as ::core::ffi::c_int
        {
            (*pSolarSysState).OrbitalCMap = SetAbsStringTableIndex(
                (*pSolarSysState).OrbitalCMap,
                1 as COUNT,
            ) as COLORMAP;
            (*pSolarSysState).XlatRef = SetAbsStringTableIndex(
                (*pSolarSysState).XlatRef,
                1 as COUNT,
            );
        }
        (*pSolarSysState).XlatPtr = GetStringAddress((*pSolarSysState).XlatRef)
            as *const ::core::ffi::c_void;
        (*Orbit).lpTopoData = HCalloc(
            (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as size_t,
        ) as *mut SBYTE;
        RandomContext_SeedRandom(SysGenRNG, (*pPlanetDesc).rand_seed);
        uqm_lua_checked!(generate_surface_frame(width, height, Orbit, PlanDataPtr), ());
        DestroyStringTable(ReleaseStringTable((*pSolarSysState).OrbitalCMap));
        (*pSolarSysState).OrbitalCMap = ::core::ptr::null_mut::<
            STRING_TABLE_ENTRY_DESC,
        >();
        DestroyStringTable(ReleaseStringTable((*pSolarSysState).XlatRef));
        (*pSolarSysState).XlatRef = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
        HFree((*Orbit).lpTopoData as *mut ::core::ffi::c_void);
        (*Orbit).lpTopoData = ::core::ptr::null_mut::<SBYTE>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn GeneratePlanetSurface(
    mut pPlanetDesc: *mut PLANET_DESC,
    mut SurfDefFrame: FRAME,
    mut width: COUNT,
    mut height: COUNT,
) {
    let mut PlanDataPtr: *const PlanetFrame = ::core::ptr::null::<PlanetFrame>();
    let mut PlanetInfo: *mut PLANET_INFO = &raw mut (*pSolarSysState).SysInfo.PlanetInfo;
    let mut loc: POINT = POINT { x: 0, y: 0 };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut TopoContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut SurfDef: BOOLEAN = BOOLEAN::FALSE;
    let mut shielded: BOOLEAN = BOOLEAN(
        ((*pPlanetDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut PlanetRotation: SDWORD = 0;
    let mut spherespanx: COUNT = 0;
    let mut radius: COUNT = 0;
    let mut ForIP: BOOLEAN = BOOLEAN::FALSE;
    let mut customTexture: BOOLEAN = BOOLEAN(
        (solTexturesPresent.0 != 0
            && (*CurStarDescPtr).Index as ::core::ffi::c_int
                == C2Rust_Unnamed_0::SOL_DEFINED.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if width == 0 && height == 0 {
        width = (if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor) as COUNT;
        height = ((67 as ::core::ffi::c_int) << resolutionFactor) as COUNT;
        spherespanx = ((67 as ::core::ffi::c_int) << resolutionFactor) as COUNT;
        radius = ((33 as ::core::ffi::c_int) << resolutionFactor) as COUNT;
        ForIP = BOOLEAN::FALSE;
        useDosSpheres = BOOLEAN(
            (optScanSphere == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        use3DOSpheres = BOOLEAN(
            (optScanSphere == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    } else {
        spherespanx = height;
        radius = ((height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                0 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as COUNT;
        ForIP = BOOLEAN::TRUE;
        useDosSpheres = BOOLEAN::FALSE;
        use3DOSpheres = BOOLEAN::FALSE;
    }
    RandomContext_SeedRandom(SysGenRNG, (*pPlanetDesc).rand_seed);
    TopoContext = CreateContextAux(
        b"Plangen.TopoContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    OldContext = SetContext(TopoContext);
    uqm_lua_checked!(planet_orbit_init(
        width,
        height,
        BOOLEAN((ForIP.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint),
    ), ());
    PlanDataPtr = PlanData
        .offset(
            ((*pPlanetDesc).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int)) as isize,
        );
    uqm_lua_checked!(load_color_resources(
        pPlanetDesc,
        PlanDataPtr,
        PlanetInfo,
        BOOLEAN(
            (shielded.0 != 0 && useDosSpheres.0 != 0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
        BOOLEAN(
            (customTexture.0 != 0 && ForIP.0 == 0 && useDosSpheres.0 == 0
                && shielded.0 == 0
                && (*PlanetInfo).AtmoDensity as ::core::ffi::c_int
                    != 0xffff as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    ), ());
    if !SurfDefFrame.is_null() {
        let mut DeleteDef: BOOLEAN = BOOLEAN::FALSE;
        let mut DeleteElev: BOOLEAN = BOOLEAN::FALSE;
        let mut ElevFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut index: COUNT = 0 as COUNT;
        if GetFrameCount(SurfDefFrame) as ::core::ffi::c_int == 4 as ::core::ffi::c_int
            && useDosSpheres.0 != 0 && ForIP.0 == 0
        {
            index = 2 as COUNT;
        }
        SurfDef = BOOLEAN::TRUE;
        SurfDefFrame = SetAbsFrameIndex(SurfDefFrame, index);
        if (*SurfDefFrame).Bounds.width as ::core::ffi::c_int
            != width as ::core::ffi::c_int
            || (*SurfDefFrame).Bounds.height as ::core::ffi::c_int
                != height as ::core::ffi::c_int
        {
            (*pSolarSysState).TopoFrame = CaptureDrawable(
                RescaleFrame(
                    SurfDefFrame,
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                ),
            );
            DeleteDef = BOOLEAN::TRUE;
        } else {
            (*pSolarSysState).TopoFrame = SurfDefFrame;
        }
        if GetFrameCount(SurfDefFrame) as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0;
            let mut elev: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
            ElevFrame = SetAbsFrameIndex(
                SurfDefFrame,
                (index as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
            );
            if (*ElevFrame).Bounds.width as ::core::ffi::c_int
                != width as ::core::ffi::c_int
                || (*ElevFrame).Bounds.height as ::core::ffi::c_int
                    != height as ::core::ffi::c_int
            {
                ElevFrame = CaptureDrawable(
                    RescaleFrame(
                        ElevFrame,
                        width as ::core::ffi::c_int,
                        height as ::core::ffi::c_int,
                    ),
                );
                DeleteElev = BOOLEAN::TRUE;
            }
            ReadFramePixelIndexes(
                ElevFrame,
                (*Orbit).lpTopoData as *mut BYTE,
                width as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
                BOOLEAN((ForIP.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint),
            );
            i = 0 as ::core::ffi::c_int;
            elev = (*Orbit).lpTopoData;
            while i < width as ::core::ffi::c_int * height as ::core::ffi::c_int {
                *elev = (*(elev as *mut BYTE) as ::core::ffi::c_int
                    - 128 as ::core::ffi::c_int) as SBYTE;
                i += 1;
                elev = elev.offset(1);
            }
        } else {
            memset(
                (*Orbit).lpTopoData as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as size_t,
            );
        }
        if DeleteDef.0 != 0 {
            DestroyDrawable(ReleaseDrawable(SurfDefFrame));
        }
        if DeleteElev.0 != 0 {
            DestroyDrawable(ReleaseDrawable(ElevFrame));
        }
    } else {
        uqm_lua_checked!(generate_surface_frame(width, height, Orbit, PlanDataPtr), ());
    }
    if ForIP.0 == 0 && useDosSpheres.0 != 0 {
        uqm_lua_checked!(RenderLevelMasks((*Orbit).TopoMask, (*Orbit).lpTopoData, SurfDef), ());
        SetPlanetColors(GetColorMapAddress((*Orbit).sphereMap));
        if !(*Orbit).TopoMask.is_null() {
            uqm_lua_checked!(ExpandLevelMasks(Orbit), ());
        } else {
            log_add(
                log_Level::log_Warning,
                b"No planet mask generated.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if ForIP.0 == 0 && isPC(optScanStyle).0 != 0 && shielded.0 == 0 {
        let mut i_0: COUNT = 0;
        if SurfDef.0 != 0 {
            i_0 = 0 as COUNT;
            while (i_0 as ::core::ffi::c_int)
                < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
            {
                let mut x: COUNT = 0;
                let mut y: COUNT = 0;
                let mut pix: *mut Color = ::core::ptr::null_mut::<Color>();
                let mut map: *mut Color = ::core::ptr::null_mut::<Color>();
                (*pSolarSysState).ScanFrame[i_0 as usize] = CaptureDrawable(
                    CreateDrawable(
                        ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            as CREATE_FLAGS,
                        width as SIZE,
                        height as SIZE,
                        1 as COUNT,
                    ),
                );
                map = HMalloc(
                    (::core::mem::size_of::<Color>() as size_t)
                        .wrapping_mul(width as size_t)
                        .wrapping_mul(height as size_t),
                ) as *mut Color;
                uqm_lua_checked!(ReadFramePixelColors(
                    (*pSolarSysState).TopoFrame,
                    map,
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                ), ());
                pix = map;
                y = 0 as COUNT;
                while (y as ::core::ffi::c_int) < height as ::core::ffi::c_int {
                    x = 0 as COUNT;
                    while (x as ::core::ffi::c_int) < width as ::core::ffi::c_int {
                        TransformColor(pix, i_0);
                        x = x.wrapping_add(1);
                        pix = pix.offset(1);
                    }
                    y = y.wrapping_add(1);
                }
                uqm_lua_checked!(WriteFramePixelColors(
                    (*pSolarSysState).ScanFrame[i_0 as usize],
                    map,
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                ), ());
                HFree(map as *mut ::core::ffi::c_void);
                i_0 = i_0.wrapping_add(1);
            }
        } else {
            let mut scanTable: COLORMAP = ::core::ptr::null_mut::<
                STRING_TABLE_ENTRY_DESC,
            >();
            scanTable = CaptureStringTable(
                uqm_lua_checked!(LoadStringTableInstance(
                    b"planet.scans.colortable\0".as_ptr() as RESOURCE,
                ), ()),
            ) as COLORMAP;
            i_0 = 0 as COUNT;
            while (i_0 as ::core::ffi::c_int)
                < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
            {
                (*pSolarSysState).ScanFrame[i_0 as usize] = CaptureDrawable(
                    CreateDrawable(
                        ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            as CREATE_FLAGS,
                        width as SIZE,
                        height as SIZE,
                        1 as COUNT,
                    ),
                );
                scanTable = SetAbsStringTableIndex(scanTable, i_0) as COLORMAP;
                uqm_lua_checked!(RenderTopography(
                    (*pSolarSysState).ScanFrame[i_0 as usize],
                    (*Orbit).lpTopoData,
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                    BOOLEAN::FALSE,
                    scanTable,
                ), ());
                i_0 = i_0.wrapping_add(1);
            }
            DestroyStringTable(ReleaseStringTable(scanTable));
            scanTable = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
        }
    }
    if ForIP.0 == 0 && shielded.0 == 0
        && (*PlanetInfo).AtmoDensity as ::core::ffi::c_int
            != 0xffff as ::core::ffi::c_int
    {
        if isPC(optSuperPC).0 != 0
            && if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0 && SurfDefFrame.is_null()
        {
            (*Orbit).TopoZoomFrame = CaptureDrawable(
                RescaleFrame(
                    (*pSolarSysState).TopoFrame,
                    (width as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
                    (height as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
                ),
            );
        } else {
            let mut pScaledTopo: *mut SBYTE = HMalloc(
                ((if is3DO(optSuperPC).0 != 0 {
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
                    } << resolutionFactor) * 4 as ::core::ffi::c_int
                    * ((67 as ::core::ffi::c_int) << resolutionFactor)
                    * 4 as ::core::ffi::c_int) as size_t,
            ) as *mut SBYTE;
            (*Orbit).TopoZoomFrame = CaptureDrawable(
                CreateDrawable(
                    ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as CREATE_FLAGS,
                    ((width as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE,
                    ((height as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE,
                    1 as COUNT,
                ),
            );
            if !pScaledTopo.is_null() {
                TopoScale4x(
                    pScaledTopo,
                    (*Orbit).lpTopoData,
                    (*PlanDataPtr).num_faults as ::core::ffi::c_int,
                    (*PlanDataPtr).fault_depth as ::core::ffi::c_int
                        * if ((*PlanDataPtr).Type as ::core::ffi::c_int
                            & 0xc as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                            == (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        },
                );
                uqm_lua_checked!(RenderTopography(
                    (*Orbit).TopoZoomFrame,
                    pScaledTopo,
                    (if is3DO(optSuperPC).0 != 0 {
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
                        } << resolutionFactor) * 4 as ::core::ffi::c_int,
                    ((67 as ::core::ffi::c_int) << resolutionFactor)
                        * 4 as ::core::ffi::c_int,
                    SurfDef,
                    ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
                ), ());
                HFree(pScaledTopo as *mut ::core::ffi::c_void);
            }
        }
    }
    if !(*Orbit).TopoColors.is_null() {
        let mut y_0: DWORD = 0;
        uqm_lua_checked!(ReadFramePixelColors(
            (*pSolarSysState).TopoFrame,
            (*Orbit).TopoColors,
            width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int,
            height as ::core::ffi::c_int,
        ), ());
        if SurfDef.0 != 0 {
            RepairColorOpacity(
                (*Orbit).TopoColors,
                width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
            );
        }
        y_0 = 0 as DWORD;
        while y_0
            < (height as ::core::ffi::c_int
                * (width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int))
                as DWORD
        {
            memcpy(
                (*Orbit)
                    .TopoColors
                    .offset(y_0 as isize)
                    .offset(width as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                (*Orbit).TopoColors.offset(y_0 as isize) as *const ::core::ffi::c_void,
                (spherespanx as size_t)
                    .wrapping_mul(::core::mem::size_of::<Color>() as size_t),
            );
            y_0 = y_0
                .wrapping_add(
                    (width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int)
                        as DWORD,
                );
        }
    }
    if !(*Orbit).ScanColors.is_null() {
        let mut i_1: COUNT = 0;
        let mut y_1: DWORD = 0;
        i_1 = 0 as COUNT;
        while (i_1 as ::core::ffi::c_int)
            < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(ReadFramePixelColors(
                (*pSolarSysState).ScanFrame[i_1 as usize],
                *(*Orbit).ScanColors.offset(i_1 as isize),
                width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
            ), ());
            y_1 = 0 as DWORD;
            while y_1
                < (height as ::core::ffi::c_int
                    * (width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int))
                    as DWORD
            {
                memcpy(
                    (*(*Orbit).ScanColors.offset(i_1 as isize))
                        .offset(y_1 as isize)
                        .offset(width as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    (*(*Orbit).ScanColors.offset(i_1 as isize)).offset(y_1 as isize)
                        as *const ::core::ffi::c_void,
                    (spherespanx as size_t)
                        .wrapping_mul(::core::mem::size_of::<Color>() as size_t),
                );
                y_1 = y_1
                    .wrapping_add(
                        (width as ::core::ffi::c_int + spherespanx as ::core::ffi::c_int)
                            as DWORD,
                    );
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
    if ((*PlanDataPtr).Type as ::core::ffi::c_int & 0xc as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        || customTexture.0 != 0 && use3DOSpheres.0 != 0
    {
        memset(
            (*Orbit).lpTopoData as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (width as ::core::ffi::c_int * height as ::core::ffi::c_int) as size_t,
        );
    } else {
        GenerateLightMap(
            (*Orbit).lpTopoData,
            width as ::core::ffi::c_int,
            height as ::core::ffi::c_int,
        );
    }
    if (*(*pSolarSysState).pOrbitalDesc).pPrevDesc
        == (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC).offset(0isize)
    {
        loc = (*(*pSolarSysState).pOrbitalDesc).location;
    } else {
        loc = (*(*(*pSolarSysState).pOrbitalDesc).pPrevDesc).location;
    }
    if !(useDosSpheres.0 != 0 || use3DOSpheres.0 != 0) || ForIP.0 != 0 {
        GenerateSphereMask(loc, radius);
        CreateSphereTiltMap(
            (*PlanetInfo).AxialTilt as ::core::ffi::c_int,
            height,
            radius,
        );
    } else if useDosSpheres.0 != 0 || use3DOSpheres.0 != 0 {
        let mut facing: COUNT = 0;
        facing = (ARCTAN(loc.x as SDWORD, loc.y as SDWORD) as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        if useDosSpheres.0 != 0 {
            (*Orbit).SphereFrame = SetAbsFrameIndex(
                (*Orbit).SphereFrame,
                (facing as ::core::ffi::c_int & 14 as ::core::ffi::c_int) as COUNT,
            );
            ReadFramePixelIndexes(
                (*Orbit).SphereFrame,
                (*Orbit).sphereBytes,
                (*(*Orbit).SphereFrame).Bounds.width as ::core::ffi::c_int,
                (*(*Orbit).SphereFrame).Bounds.height as ::core::ffi::c_int,
                BOOLEAN::TRUE,
            );
        } else if use3DOSpheres.0 != 0 {
            CreateSphereTiltMap(
                (*PlanetInfo).AxialTilt as ::core::ffi::c_int,
                height,
                radius,
            );
            (*Orbit).Shade = SetAbsFrameIndex((*Orbit).Shade, facing);
            uqm_lua_checked!(ReadFramePixelColors(
                (*Orbit).Shade,
                (*Orbit).ShadeColors,
                (*(*Orbit).Shade).Bounds.width as ::core::ffi::c_int,
                (*(*Orbit).Shade).Bounds.height as ::core::ffi::c_int,
            ), ());
        }
    }
    if shielded.0 != 0 {
        (*Orbit).ObjectFrame = if optScanSphere < 2 as ::core::ffi::c_int && ForIP.0 == 0
        {
            if useDosSpheres.0 != 0 {
                CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.dos.shield\0".as_ptr() as RESOURCE), ())
                        as DRAWABLE,
                )
            } else {
                CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.3do.shield\0".as_ptr() as RESOURCE), ())
                        as DRAWABLE,
                )
            }
        } else {
            uqm_lua_checked!(CreateShieldMask(radius), ())
        };
        if (optNebulae.0 != 0 || use3DOSpheres.0 != 0) && ForIP.0 == 0
            && useDosSpheres.0 == 0
        {
            (*Orbit).BackFrame = SaveBackFrame(radius);
        }
    }
    PlanetRotation = (1 as ::core::ffi::c_int
        - 2 as ::core::ffi::c_int
            * ((*PlanetInfo).AxialTilt as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
        as SDWORD;
    uqm_lua_checked!(InitSphereRotation(PlanetRotation, shielded, width, height), ());
    if ForIP.0 != 0 {
        (*pPlanetDesc).rotwidth = width as ::core::ffi::c_int;
        (*pPlanetDesc).rotheight = height as ::core::ffi::c_int;
        (*pPlanetDesc).rotFrameIndex = 0 as ::core::ffi::c_int;
        (*pPlanetDesc).rotPointIndex = 0 as ::core::ffi::c_int;
        (*pPlanetDesc).rot_speed = ((*pPlanetDesc).rotwidth * PlanetRotation
            * 240 as ::core::ffi::c_int) as ::core::ffi::c_double
            / (*PlanetInfo).RotationPeriod as ::core::ffi::c_int
                as ::core::ffi::c_double;
    }
    if shielded.0 != 0 && useDosSpheres.0 == 0 {
        uqm_lua_checked!(ApplyShieldTint(), ());
    }
    SetContext(OldContext);
    DestroyContext(TopoContext);
}
