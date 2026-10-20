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

// Rust translation of src/uqm/uqmdebug.c.
extern "C" {
    pub type RandomContext;
    pub type frame_desc;
    pub type font_desc;
    pub type string_table_entry;
    pub type _IO_FILE;
    pub type context_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn SleepThread(timePeriod: TimePeriod);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn ForAllLinks(
        pq: *mut QUEUE,
        callback: Option<
            unsafe extern "C" fn(*mut LINK, *mut ::core::ffi::c_void) -> (),
        >,
        arg: *mut ::core::ffi::c_void,
    );
    fn MoveGameClockDays(days: COUNT);
    fn LockGameClock();
    fn UnlockGameClock();
    fn GameClockRunning() -> BOOLEAN;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut SysGenRNG: *mut RandomContext;
    fn GetRandomSeedForStar(star: *const STAR_DESC) -> DWORD;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn GetContextFGFrame() -> FRAME;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn DrawLine(pLine: *mut LINE);
    fn font_DrawText(pText: *mut TEXT);
    fn FlushGraphics();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetContextName(context: CONTEXT) -> *const ::core::ffi::c_char;
    fn GetFirstContext() -> CONTEXT;
    fn GetNextContext(context: CONTEXT) -> CONTEXT;
    fn LoadDisplayPixmap(area: *const RECT, frame: FRAME) -> DRAWABLE;
    fn SetContextFont(Font: FONT) -> FONT;
    fn CloneFrame(Frame: FRAME) -> DRAWABLE;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn DoPlanetaryAnalysis(SysInfoPtr: *mut SYSTEM_INFO, pPlanetDesc: *mut PLANET_DESC);
    static stdout: *mut FILE;
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut Elements: *const BYTE;
    static CreatureData: [LIFEFORM_DESC; 0];
    static mut disp_q: QUEUE;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn DeltaCrew(ElementPtr: *mut ELEMENT, crew_delta: SIZE) -> BOOLEAN;
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SIZE,
        resunit_delta: ::core::ffi::c_int,
    );
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn AddEscortShips(race: COUNT, count: SIZE) -> COUNT;
    fn getGenerateFunctions(Index: BYTE) -> *const GenerateFunctions;
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn setGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
        val: BYTE,
    );
    fn inHQSpace() -> BOOLEAN;
    fn callGenerateForScanType(
        _: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        node: COUNT,
        scanType: BYTE,
        _: *mut NODE_INFO,
    ) -> COUNT;
    static mut Screen: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut TinyFont: FONT;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut GameStrings: STRING;
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type TimePeriod = DWORD;
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
pub type HEVENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub pred: HEVENT,
    pub succ: HEVENT,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub func_index: BYTE,
}
pub type EVENT = event;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const BIOLOGICAL_DISASTER: Self = Self(0);
    pub const EARTHQUAKE_DISASTER: Self = Self(1);
    pub const LIGHTNING_DISASTER: Self = Self(2);
    pub const LAVASPOT_DISASTER: Self = Self(3);
    pub const LANDER_INJURED: Self = Self(4);
    pub const LANDER_SHOOTS: Self = Self(5);
    pub const LANDER_HITS: Self = Self(6);
    pub const LIFEFORM_CANNED: Self = Self(7);
    pub const LANDER_PICKUP: Self = Self(8);
    pub const LANDER_FULL: Self = Self(9);
    pub const LANDER_DEPARTS: Self = Self(10);
    pub const LANDER_RETURNS: Self = Self(11);
    pub const LANDER_DESTROYED: Self = Self(12);
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
}
pub type PLANET_DESC = planet_desc;
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
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
}
pub type PLANET_ORBIT = planet_orbit;
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
    pub PlanetToSunDist: UWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
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
pub type COLORMAP = STRING;
pub type FILE = _IO_FILE;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
pub type SOUND = STRING;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const COMMON: Self = Self(0);
    pub const CORROSIVE: Self = Self(1);
    pub const BASE_METAL: Self = Self(2);
    pub const NOBLE: Self = Self(3);
    pub const RARE_EARTH: Self = Self(4);
    pub const PRECIOUS: Self = Self(5);
    pub const RADIOACTIVE: Self = Self(6);
    pub const EXOTIC: Self = Self(7);
    pub const NUM_ELEMENT_CATEGORIES: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const LIGHT: Self = Self(0);
    pub const MEDIUM: Self = Self(1);
    pub const HEAVY: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LIFEFORM_DESC {
    pub Attributes: BYTE,
    pub ValueAndHitPoints: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const GAS_DENSITY: Self = Self(0);
    pub const LIGHT_DENSITY: Self = Self(1);
    pub const LOW_DENSITY: Self = Self(2);
    pub const NORMAL_DENSITY: Self = Self(3);
    pub const HIGH_DENSITY: Self = Self(4);
    pub const SUPER_DENSITY: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STARSHIP {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub RaceDescPtr: *mut RACE_DESC,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub ship_cost: BYTE,
    pub index: COUNT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub weapon_counter: BYTE,
    pub special_counter: BYTE,
    pub energy_counter: BYTE,
    pub ship_input_state: BYTE,
    pub cur_status_flags: STATUS_FLAGS,
    pub old_status_flags: STATUS_FLAGS,
    pub hShip: HELEMENT,
    pub ShipFacing: COUNT,
    pub playerNr: SIZE,
    pub control: BYTE,
}
pub type HELEMENT = HLINK;
pub type STATUS_FLAGS = UWORD;
pub type RACE_DESC = race_desc;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct race_desc {
    pub ship_info: SHIP_INFO,
    pub c2rust_padding: [u8; 12],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub cyborg_control: INTEL_STUFF,
    pub uninit_func: Option<UNINIT_FUNC>,
    pub preprocess_func: Option<PREPROCESS_FUNC>,
    pub postprocess_func: Option<POSTPROCESS_FUNC>,
    pub init_weapon_func: Option<INIT_WEAPON_FUNC>,
    pub data: *mut ::core::ffi::c_void,
    pub CodeRef: *mut ::core::ffi::c_void,
}
pub type INIT_WEAPON_FUNC = unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT;
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
    pub c2rust_unnamed: C2Rust_Unnamed_12,
    pub c2rust_unnamed_0: C2Rust_Unnamed_11,
    pub c2rust_unnamed_1: C2Rust_Unnamed_10,
    pub c2rust_unnamed_2: C2Rust_Unnamed_9,
    pub c2rust_unnamed_3: C2Rust_Unnamed_8,
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
    pub image: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
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
pub type POSTPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type PREPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type UNINIT_FUNC = unsafe extern "C" fn(*mut RACE_DESC) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTEL_STUFF {
    pub ManeuverabilityIndex: COUNT,
    pub WeaponRange: COUNT,
    pub intelligence_func: Option<IntelligenceFunc>,
}
pub type IntelligenceFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut EVALUATE_DESC,
    COUNT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EVALUATE_DESC {
    pub ObjectPtr: *mut ELEMENT,
    pub facing: COUNT,
    pub which_turn: COUNT,
    pub MoveState: MOVEMENT_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MOVEMENT_STATE(pub ::core::ffi::c_uint);
impl MOVEMENT_STATE {
    pub const PURSUE: Self = Self(0);
    pub const AVOID: Self = Self(1);
    pub const ENTICE: Self = Self(2);
    pub const NO_MOVEMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATA_STUFF {
    pub ship_rsc: [RESOURCE; 3],
    pub weapon_rsc: [RESOURCE; 3],
    pub special_rsc: [RESOURCE; 3],
    pub captain_control: CAPTAIN_STUFF,
    pub victory_ditty_rsc: RESOURCE,
    pub ship_sounds_rsc: RESOURCE,
    pub ship: [FRAME; 3],
    pub weapon: [FRAME; 3],
    pub special: [FRAME; 3],
    pub victory_ditty: MUSIC_REF,
    pub ship_sounds: SOUND,
}
pub type CAPTAIN_STUFF = captain_stuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct captain_stuff {
    pub captain_rsc: RESOURCE,
    pub background: FRAME,
    pub turn: FRAME,
    pub thrust: FRAME,
    pub weapon: FRAME,
    pub special: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHARACTERISTIC_STUFF {
    pub max_thrust: COUNT,
    pub thrust_increment: COUNT,
    pub energy_regeneration: BYTE,
    pub weapon_energy_cost: BYTE,
    pub special_energy_cost: BYTE,
    pub energy_wait: BYTE,
    pub turn_wait: BYTE,
    pub thrust_wait: BYTE,
    pub weapon_wait: BYTE,
    pub special_wait: BYTE,
    pub ship_mass: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_INFO {
    pub ship_flags: UWORD,
    pub ship_cost: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings_rsc: RESOURCE,
    pub icons_rsc: RESOURCE,
    pub melee_icon_rsc: RESOURCE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
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
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_13 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_14(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_14 {
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
    pub const YEHAT_REBEL_SHIP: Self = Self(22);
    pub const URQUAN_DRONE_SHIP: Self = Self(23);
    pub const SAMATRA_SHIP: Self = Self(23);
    pub const NUM_AVAILABLE_RACES: Self = Self(24);
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
    pub GameState: [BYTE; 155],
}
pub type ACTIVITY = UWORD;
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
    pub ShipName: [UNICODE; 16],
    pub CommanderName: [UNICODE; 16],
    pub PlanetName: [UNICODE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UniverseRecurseArg {
    pub systemFuncPre: Option<
        unsafe extern "C" fn(
            *const STAR_DESC,
            *const SOLARSYS_STATE,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub systemFuncPost: Option<
        unsafe extern "C" fn(
            *const STAR_DESC,
            *const SOLARSYS_STATE,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub planetFuncPre: Option<
        unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    >,
    pub planetFuncPost: Option<
        unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    >,
    pub moonFunc: Option<
        unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    >,
    pub arg: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpUniverseArg {
    pub out: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TallyResourcesArg {
    pub out: *mut FILE,
    pub mineralCount: COUNT,
    pub bioCount: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpPlanetTypesArg {
    pub out: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub name: *const ::core::ffi::c_char,
    pub base: size_t,
    pub count: size_t,
}
pub type WAIT_STATE = wait_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wait_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut wait_state) -> BOOLEAN>,
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_16(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_16 {
    pub const KEY_PAUSE: Self = Self(0);
    pub const KEY_EXIT: Self = Self(1);
    pub const KEY_ABORT: Self = Self(2);
    pub const KEY_DEBUG: Self = Self(3);
    pub const KEY_FULLSCREEN: Self = Self(4);
    pub const KEY_MENU_UP: Self = Self(5);
    pub const KEY_MENU_DOWN: Self = Self(6);
    pub const KEY_MENU_LEFT: Self = Self(7);
    pub const KEY_MENU_RIGHT: Self = Self(8);
    pub const KEY_MENU_SELECT: Self = Self(9);
    pub const KEY_MENU_CANCEL: Self = Self(10);
    pub const KEY_MENU_SPECIAL: Self = Self(11);
    pub const KEY_MENU_PAGE_UP: Self = Self(12);
    pub const KEY_MENU_PAGE_DOWN: Self = Self(13);
    pub const KEY_MENU_HOME: Self = Self(14);
    pub const KEY_MENU_END: Self = Self(15);
    pub const KEY_MENU_ZOOM_IN: Self = Self(16);
    pub const KEY_MENU_ZOOM_OUT: Self = Self(17);
    pub const KEY_MENU_DELETE: Self = Self(18);
    pub const KEY_MENU_BACKSPACE: Self = Self(19);
    pub const KEY_MENU_EDIT_CANCEL: Self = Self(20);
    pub const KEY_MENU_SEARCH: Self = Self(21);
    pub const KEY_MENU_NEXT: Self = Self(22);
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_17(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_17 {
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
pub struct C2Rust_Unnamed_18(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_18 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(133);
    pub const CARGO_STRING_BASE: Self = Self(162);
    pub const ELEMENTS_STRING_BASE: Self = Self(172);
    pub const SCAN_STRING_BASE: Self = Self(305);
    pub const STAR_NUMBER_BASE: Self = Self(361);
    pub const PLANET_NUMBER_BASE: Self = Self(375);
    pub const MONTHS_STRING_BASE: Self = Self(408);
    pub const FEEDBACK_STRING_BASE: Self = Self(420);
    pub const STARBASE_STRING_BASE: Self = Self(422);
    pub const ENCOUNTER_STRING_BASE: Self = Self(427);
    pub const NAVIGATION_STRING_BASE: Self = Self(435);
    pub const NAMING_STRING_BASE: Self = Self(441);
    pub const MELEE_STRING_BASE: Self = Self(445);
    pub const SAVEGAME_STRING_BASE: Self = Self(454);
    pub const OPTION_STRING_BASE: Self = Self(459);
    pub const QUITMENU_STRING_BASE: Self = Self(464);
    pub const STATUS_STRING_BASE: Self = Self(467);
    pub const FLAGSHIP_STRING_BASE: Self = Self(473);
    pub const ORBITSCAN_STRING_BASE: Self = Self(486);
    pub const MAINMENU_STRING_BASE: Self = Self(505);
    pub const NETMELEE_STRING_BASE: Self = Self(560);
    pub const GAMESTR_COUNT: Self = Self(579);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_19(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_19 {
    pub const ARILOU_ENTRANCE_EVENT: Self = Self(0);
    pub const ARILOU_EXIT_EVENT: Self = Self(1);
    pub const HYPERSPACE_ENCOUNTER_EVENT: Self = Self(2);
    pub const KOHR_AH_VICTORIOUS_EVENT: Self = Self(3);
    pub const ADVANCE_PKUNK_MISSION: Self = Self(4);
    pub const ADVANCE_THRADD_MISSION: Self = Self(5);
    pub const ZOQFOT_DISTRESS_EVENT: Self = Self(6);
    pub const ZOQFOT_DEATH_EVENT: Self = Self(7);
    pub const SHOFIXTI_RETURN_EVENT: Self = Self(8);
    pub const ADVANCE_UTWIG_SUPOX_MISSION: Self = Self(9);
    pub const KOHR_AH_GENOCIDE_EVENT: Self = Self(10);
    pub const SPATHI_SHIELD_EVENT: Self = Self(11);
    pub const ADVANCE_ILWRATH_MISSION: Self = Self(12);
    pub const ADVANCE_MYCON_MISSION: Self = Self(13);
    pub const ARILOU_UMGAH_CHECK: Self = Self(14);
    pub const YEHAT_REBEL_EVENT: Self = Self(15);
    pub const SLYLANDRO_RAMP_UP: Self = Self(16);
    pub const SLYLANDRO_RAMP_DOWN: Self = Self(17);
    pub const NUM_EVENTS: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_20(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_20 {
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
    pub const RAINBOW_DEFINED: Self = Self(42);
    pub const ILWRATH_DEFINED: Self = Self(43);
    pub const ANDROSYNTH_DEFINED: Self = Self(44);
    pub const MYCON_TRAP_DEFINED: Self = Self(45);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_21(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_21 {
    pub const SHOFIXTI_VISITS: Self = Self(0);
    pub const END_SHOFIXTI_VISITS: Self = Self(2);
    pub const SHOFIXTI_STACK1: Self = Self(3);
    pub const END_SHOFIXTI_STACK1: Self = Self(4);
    pub const SHOFIXTI_STACK2: Self = Self(5);
    pub const END_SHOFIXTI_STACK2: Self = Self(7);
    pub const SHOFIXTI_STACK3: Self = Self(8);
    pub const END_SHOFIXTI_STACK3: Self = Self(9);
    pub const SHOFIXTI_KIA: Self = Self(10);
    pub const END_SHOFIXTI_KIA: Self = Self(10);
    pub const SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const END_SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const SHOFIXTI_RECRUITED: Self = Self(12);
    pub const END_SHOFIXTI_RECRUITED: Self = Self(12);
    pub const SHOFIXTI_MAIDENS: Self = Self(13);
    pub const END_SHOFIXTI_MAIDENS: Self = Self(13);
    pub const MAIDENS_ON_SHIP: Self = Self(14);
    pub const END_MAIDENS_ON_SHIP: Self = Self(14);
    pub const BATTLE_SEGUE: Self = Self(15);
    pub const END_BATTLE_SEGUE: Self = Self(15);
    pub const PLANETARY_LANDING: Self = Self(16);
    pub const END_PLANETARY_LANDING: Self = Self(16);
    pub const PLANETARY_CHANGE: Self = Self(17);
    pub const END_PLANETARY_CHANGE: Self = Self(17);
    pub const SPATHI_VISITS: Self = Self(18);
    pub const END_SPATHI_VISITS: Self = Self(20);
    pub const SPATHI_HOME_VISITS: Self = Self(21);
    pub const END_SPATHI_HOME_VISITS: Self = Self(23);
    pub const FOUND_PLUTO_SPATHI: Self = Self(24);
    pub const END_FOUND_PLUTO_SPATHI: Self = Self(25);
    pub const SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const END_SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const END_SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const END_SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const UMGAH_BROADCASTERS: Self = Self(29);
    pub const END_UMGAH_BROADCASTERS: Self = Self(29);
    pub const SPATHI_MANNER: Self = Self(30);
    pub const END_SPATHI_MANNER: Self = Self(31);
    pub const SPATHI_QUEST: Self = Self(32);
    pub const END_SPATHI_QUEST: Self = Self(32);
    pub const LIED_ABOUT_CREATURES: Self = Self(33);
    pub const END_LIED_ABOUT_CREATURES: Self = Self(34);
    pub const SPATHI_PARTY: Self = Self(35);
    pub const END_SPATHI_PARTY: Self = Self(35);
    pub const KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const END_KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const ILWRATH_HOME_VISITS: Self = Self(37);
    pub const END_ILWRATH_HOME_VISITS: Self = Self(39);
    pub const ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const END_ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const ARILOU_SPACE: Self = Self(41);
    pub const END_ARILOU_SPACE: Self = Self(41);
    pub const ARILOU_SPACE_SIDE: Self = Self(42);
    pub const END_ARILOU_SPACE_SIDE: Self = Self(43);
    pub const ARILOU_SPACE_COUNTER: Self = Self(44);
    pub const END_ARILOU_SPACE_COUNTER: Self = Self(47);
    pub const LANDER_SHIELDS: Self = Self(48);
    pub const END_LANDER_SHIELDS: Self = Self(51);
    pub const MET_MELNORME: Self = Self(52);
    pub const END_MET_MELNORME: Self = Self(52);
    pub const MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const END_MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const MELNORME_RESCUE_COUNT: Self = Self(54);
    pub const END_MELNORME_RESCUE_COUNT: Self = Self(56);
    pub const TRADED_WITH_MELNORME: Self = Self(57);
    pub const END_TRADED_WITH_MELNORME: Self = Self(57);
    pub const WHY_MELNORME_PURPLE: Self = Self(58);
    pub const END_WHY_MELNORME_PURPLE: Self = Self(58);
    pub const MELNORME_CREDIT0: Self = Self(59);
    pub const END_MELNORME_CREDIT0: Self = Self(66);
    pub const MELNORME_CREDIT1: Self = Self(67);
    pub const END_MELNORME_CREDIT1: Self = Self(74);
    pub const MELNORME_BUSINESS_COUNT: Self = Self(75);
    pub const END_MELNORME_BUSINESS_COUNT: Self = Self(76);
    pub const MELNORME_YACK_STACK0: Self = Self(77);
    pub const END_MELNORME_YACK_STACK0: Self = Self(78);
    pub const MELNORME_YACK_STACK1: Self = Self(79);
    pub const END_MELNORME_YACK_STACK1: Self = Self(80);
    pub const MELNORME_YACK_STACK2: Self = Self(81);
    pub const END_MELNORME_YACK_STACK2: Self = Self(84);
    pub const MELNORME_YACK_STACK3: Self = Self(85);
    pub const END_MELNORME_YACK_STACK3: Self = Self(87);
    pub const MELNORME_YACK_STACK4: Self = Self(88);
    pub const END_MELNORME_YACK_STACK4: Self = Self(89);
    pub const WHY_MELNORME_BLUE: Self = Self(90);
    pub const END_WHY_MELNORME_BLUE: Self = Self(90);
    pub const MELNORME_ANGER: Self = Self(91);
    pub const END_MELNORME_ANGER: Self = Self(92);
    pub const MELNORME_MIFFED_COUNT: Self = Self(93);
    pub const END_MELNORME_MIFFED_COUNT: Self = Self(94);
    pub const MELNORME_PISSED_COUNT: Self = Self(95);
    pub const END_MELNORME_PISSED_COUNT: Self = Self(96);
    pub const MELNORME_HATE_COUNT: Self = Self(97);
    pub const END_MELNORME_HATE_COUNT: Self = Self(98);
    pub const PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const END_PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const END_PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const STARBASE_AVAILABLE: Self = Self(101);
    pub const END_STARBASE_AVAILABLE: Self = Self(101);
    pub const STARBASE_VISITED: Self = Self(102);
    pub const END_STARBASE_VISITED: Self = Self(102);
    pub const RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const END_RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const LANDERS_LOST: Self = Self(104);
    pub const END_LANDERS_LOST: Self = Self(104);
    pub const GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const END_GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const AWARE_OF_SAMATRA: Self = Self(106);
    pub const END_AWARE_OF_SAMATRA: Self = Self(106);
    pub const YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const END_YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const URQUAN_MESSED_UP: Self = Self(108);
    pub const END_URQUAN_MESSED_UP: Self = Self(108);
    pub const MOONBASE_DESTROYED: Self = Self(109);
    pub const END_MOONBASE_DESTROYED: Self = Self(109);
    pub const WILL_DESTROY_BASE: Self = Self(110);
    pub const END_WILL_DESTROY_BASE: Self = Self(110);
    pub const ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const END_ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const END_ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const END_KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const STARBASE_YACK_STACK1: Self = Self(114);
    pub const END_STARBASE_YACK_STACK1: Self = Self(114);
    pub const DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const END_DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const DISCUSSED_TALKING_PET: Self = Self(116);
    pub const END_DISCUSSED_TALKING_PET: Self = Self(116);
    pub const DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const END_DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const END_DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const END_DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const END_DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const END_DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const DISCUSSED_ULTRON: Self = Self(122);
    pub const END_DISCUSSED_ULTRON: Self = Self(122);
    pub const DISCUSSED_MAIDENS: Self = Self(123);
    pub const END_DISCUSSED_MAIDENS: Self = Self(123);
    pub const DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const END_DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const END_DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const SYREEN_WANT_PROOF: Self = Self(126);
    pub const END_SYREEN_WANT_PROOF: Self = Self(126);
    pub const PLAYER_HAVING_SEX: Self = Self(127);
    pub const END_PLAYER_HAVING_SEX: Self = Self(127);
    pub const MET_ARILOU: Self = Self(128);
    pub const END_MET_ARILOU: Self = Self(128);
    pub const DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const END_DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const END_DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const END_DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const END_DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const END_DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const END_DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const END_DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const END_DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const END_DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const END_DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const ATTACKED_DRUUGE: Self = Self(139);
    pub const END_ATTACKED_DRUUGE: Self = Self(139);
    pub const NEW_ALLIANCE_NAME: Self = Self(140);
    pub const END_NEW_ALLIANCE_NAME: Self = Self(141);
    pub const PORTAL_COUNTER: Self = Self(142);
    pub const END_PORTAL_COUNTER: Self = Self(145);
    pub const BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const END_BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const END_BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const UTWIG_BOMB: Self = Self(148);
    pub const END_UTWIG_BOMB: Self = Self(148);
    pub const UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const END_UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const AQUA_HELIX: Self = Self(150);
    pub const END_AQUA_HELIX: Self = Self(150);
    pub const AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const END_AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const SUN_DEVICE: Self = Self(152);
    pub const END_SUN_DEVICE: Self = Self(152);
    pub const SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const END_SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const TAALO_PROTECTOR: Self = Self(154);
    pub const END_TAALO_PROTECTOR: Self = Self(154);
    pub const TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const END_TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const END_SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const SYREEN_SHUTTLE: Self = Self(157);
    pub const END_SYREEN_SHUTTLE: Self = Self(157);
    pub const PORTAL_KEY: Self = Self(158);
    pub const END_PORTAL_KEY: Self = Self(158);
    pub const PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const END_PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const VUX_BEAST: Self = Self(160);
    pub const END_VUX_BEAST: Self = Self(160);
    pub const VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const END_VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const TALKING_PET: Self = Self(162);
    pub const END_TALKING_PET: Self = Self(162);
    pub const TALKING_PET_ON_SHIP: Self = Self(163);
    pub const END_TALKING_PET_ON_SHIP: Self = Self(163);
    pub const MOONBASE_ON_SHIP: Self = Self(164);
    pub const END_MOONBASE_ON_SHIP: Self = Self(164);
    pub const KOHR_AH_FRENZY: Self = Self(165);
    pub const END_KOHR_AH_FRENZY: Self = Self(165);
    pub const KOHR_AH_VISITS: Self = Self(166);
    pub const END_KOHR_AH_VISITS: Self = Self(167);
    pub const KOHR_AH_BYES: Self = Self(168);
    pub const END_KOHR_AH_BYES: Self = Self(168);
    pub const SLYLANDRO_HOME_VISITS: Self = Self(169);
    pub const END_SLYLANDRO_HOME_VISITS: Self = Self(171);
    pub const DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const END_DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const ILWRATH_VISITS: Self = Self(173);
    pub const END_ILWRATH_VISITS: Self = Self(175);
    pub const ILWRATH_DECEIVED: Self = Self(176);
    pub const END_ILWRATH_DECEIVED: Self = Self(176);
    pub const FLAGSHIP_CLOAKED: Self = Self(177);
    pub const END_FLAGSHIP_CLOAKED: Self = Self(177);
    pub const MYCON_VISITS: Self = Self(178);
    pub const END_MYCON_VISITS: Self = Self(180);
    pub const MYCON_HOME_VISITS: Self = Self(181);
    pub const END_MYCON_HOME_VISITS: Self = Self(183);
    pub const MYCON_AMBUSH: Self = Self(184);
    pub const END_MYCON_AMBUSH: Self = Self(184);
    pub const MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const END_MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const GLOBAL_FLAGS_AND_DATA: Self = Self(186);
    pub const END_GLOBAL_FLAGS_AND_DATA: Self = Self(193);
    pub const ORZ_VISITS: Self = Self(194);
    pub const END_ORZ_VISITS: Self = Self(196);
    pub const TAALO_VISITS: Self = Self(197);
    pub const END_TAALO_VISITS: Self = Self(199);
    pub const ORZ_MANNER: Self = Self(200);
    pub const END_ORZ_MANNER: Self = Self(201);
    pub const PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const END_PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const END_CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const URQUAN_VISITS: Self = Self(204);
    pub const END_URQUAN_VISITS: Self = Self(206);
    pub const PLAYER_HYPNOTIZED: Self = Self(207);
    pub const END_PLAYER_HYPNOTIZED: Self = Self(207);
    pub const VUX_VISITS: Self = Self(208);
    pub const END_VUX_VISITS: Self = Self(210);
    pub const VUX_HOME_VISITS: Self = Self(211);
    pub const END_VUX_HOME_VISITS: Self = Self(213);
    pub const ZEX_VISITS: Self = Self(214);
    pub const END_ZEX_VISITS: Self = Self(216);
    pub const ZEX_IS_DEAD: Self = Self(217);
    pub const END_ZEX_IS_DEAD: Self = Self(217);
    pub const KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const END_KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const UTWIG_VISITS: Self = Self(219);
    pub const END_UTWIG_VISITS: Self = Self(221);
    pub const UTWIG_HOME_VISITS: Self = Self(222);
    pub const END_UTWIG_HOME_VISITS: Self = Self(224);
    pub const BOMB_VISITS: Self = Self(225);
    pub const END_BOMB_VISITS: Self = Self(227);
    pub const ULTRON_CONDITION: Self = Self(228);
    pub const END_ULTRON_CONDITION: Self = Self(230);
    pub const UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const END_UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const BOMB_UNPROTECTED: Self = Self(232);
    pub const END_BOMB_UNPROTECTED: Self = Self(232);
    pub const TAALO_UNPROTECTED: Self = Self(233);
    pub const END_TAALO_UNPROTECTED: Self = Self(233);
    pub const TALKING_PET_VISITS: Self = Self(234);
    pub const END_TALKING_PET_VISITS: Self = Self(236);
    pub const TALKING_PET_HOME_VISITS: Self = Self(237);
    pub const END_TALKING_PET_HOME_VISITS: Self = Self(239);
    pub const UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const END_UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const END_KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const ARILOU_VISITS: Self = Self(242);
    pub const END_ARILOU_VISITS: Self = Self(244);
    pub const ARILOU_HOME_VISITS: Self = Self(245);
    pub const END_ARILOU_HOME_VISITS: Self = Self(247);
    pub const KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const END_KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const ARILOU_CHECKED_UMGAH: Self = Self(249);
    pub const END_ARILOU_CHECKED_UMGAH: Self = Self(250);
    pub const PORTAL_SPAWNER: Self = Self(251);
    pub const END_PORTAL_SPAWNER: Self = Self(251);
    pub const PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const END_PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const UMGAH_VISITS: Self = Self(253);
    pub const END_UMGAH_VISITS: Self = Self(255);
    pub const UMGAH_HOME_VISITS: Self = Self(256);
    pub const END_UMGAH_HOME_VISITS: Self = Self(258);
    pub const MET_NORMAL_UMGAH: Self = Self(259);
    pub const END_MET_NORMAL_UMGAH: Self = Self(259);
    pub const SYREEN_HOME_VISITS: Self = Self(260);
    pub const END_SYREEN_HOME_VISITS: Self = Self(262);
    pub const SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const END_SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const KNOW_SYREEN_VAULT: Self = Self(264);
    pub const END_KNOW_SYREEN_VAULT: Self = Self(264);
    pub const EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const END_EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const END_SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const END_ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const CHMMR_HOME_VISITS: Self = Self(268);
    pub const END_CHMMR_HOME_VISITS: Self = Self(270);
    pub const CHMMR_EMERGING: Self = Self(271);
    pub const END_CHMMR_EMERGING: Self = Self(271);
    pub const CHMMR_UNLEASHED: Self = Self(272);
    pub const END_CHMMR_UNLEASHED: Self = Self(272);
    pub const CHMMR_BOMB_STATE: Self = Self(273);
    pub const END_CHMMR_BOMB_STATE: Self = Self(274);
    pub const DRUUGE_DISCLAIMER: Self = Self(275);
    pub const END_DRUUGE_DISCLAIMER: Self = Self(275);
    pub const YEHAT_VISITS: Self = Self(276);
    pub const END_YEHAT_VISITS: Self = Self(278);
    pub const YEHAT_REBEL_VISITS: Self = Self(279);
    pub const END_YEHAT_REBEL_VISITS: Self = Self(281);
    pub const YEHAT_HOME_VISITS: Self = Self(282);
    pub const END_YEHAT_HOME_VISITS: Self = Self(284);
    pub const YEHAT_CIVIL_WAR: Self = Self(285);
    pub const END_YEHAT_CIVIL_WAR: Self = Self(285);
    pub const YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const END_YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const YEHAT_SHIP_MONTH: Self = Self(287);
    pub const END_YEHAT_SHIP_MONTH: Self = Self(290);
    pub const YEHAT_SHIP_DAY: Self = Self(291);
    pub const END_YEHAT_SHIP_DAY: Self = Self(295);
    pub const YEHAT_SHIP_YEAR: Self = Self(296);
    pub const END_YEHAT_SHIP_YEAR: Self = Self(300);
    pub const CLEAR_SPINDLE: Self = Self(301);
    pub const END_CLEAR_SPINDLE: Self = Self(301);
    pub const PKUNK_VISITS: Self = Self(302);
    pub const END_PKUNK_VISITS: Self = Self(304);
    pub const PKUNK_HOME_VISITS: Self = Self(305);
    pub const END_PKUNK_HOME_VISITS: Self = Self(307);
    pub const PKUNK_SHIP_MONTH: Self = Self(308);
    pub const END_PKUNK_SHIP_MONTH: Self = Self(311);
    pub const PKUNK_SHIP_DAY: Self = Self(312);
    pub const END_PKUNK_SHIP_DAY: Self = Self(316);
    pub const PKUNK_SHIP_YEAR: Self = Self(317);
    pub const END_PKUNK_SHIP_YEAR: Self = Self(321);
    pub const PKUNK_MISSION: Self = Self(322);
    pub const END_PKUNK_MISSION: Self = Self(324);
    pub const SUPOX_VISITS: Self = Self(325);
    pub const END_SUPOX_VISITS: Self = Self(327);
    pub const SUPOX_HOME_VISITS: Self = Self(328);
    pub const END_SUPOX_HOME_VISITS: Self = Self(330);
    pub const THRADD_VISITS: Self = Self(331);
    pub const END_THRADD_VISITS: Self = Self(333);
    pub const THRADD_HOME_VISITS: Self = Self(334);
    pub const END_THRADD_HOME_VISITS: Self = Self(336);
    pub const HELIX_VISITS: Self = Self(337);
    pub const END_HELIX_VISITS: Self = Self(339);
    pub const HELIX_UNPROTECTED: Self = Self(340);
    pub const END_HELIX_UNPROTECTED: Self = Self(340);
    pub const THRADD_CULTURE: Self = Self(341);
    pub const END_THRADD_CULTURE: Self = Self(342);
    pub const THRADD_MISSION: Self = Self(343);
    pub const END_THRADD_MISSION: Self = Self(345);
    pub const DRUUGE_VISITS: Self = Self(346);
    pub const END_DRUUGE_VISITS: Self = Self(348);
    pub const DRUUGE_HOME_VISITS: Self = Self(349);
    pub const END_DRUUGE_HOME_VISITS: Self = Self(351);
    pub const ROSY_SPHERE: Self = Self(352);
    pub const END_ROSY_SPHERE: Self = Self(352);
    pub const SCANNED_MAIDENS: Self = Self(353);
    pub const END_SCANNED_MAIDENS: Self = Self(353);
    pub const SCANNED_FRAGMENTS: Self = Self(354);
    pub const END_SCANNED_FRAGMENTS: Self = Self(354);
    pub const SCANNED_CASTER: Self = Self(355);
    pub const END_SCANNED_CASTER: Self = Self(355);
    pub const SCANNED_SPAWNER: Self = Self(356);
    pub const END_SCANNED_SPAWNER: Self = Self(356);
    pub const SCANNED_ULTRON: Self = Self(357);
    pub const END_SCANNED_ULTRON: Self = Self(357);
    pub const ZOQFOT_INFO: Self = Self(358);
    pub const END_ZOQFOT_INFO: Self = Self(359);
    pub const ZOQFOT_HOSTILE: Self = Self(360);
    pub const END_ZOQFOT_HOSTILE: Self = Self(360);
    pub const ZOQFOT_HOME_VISITS: Self = Self(361);
    pub const END_ZOQFOT_HOME_VISITS: Self = Self(363);
    pub const MET_ZOQFOT: Self = Self(364);
    pub const END_MET_ZOQFOT: Self = Self(364);
    pub const ZOQFOT_DISTRESS: Self = Self(365);
    pub const END_ZOQFOT_DISTRESS: Self = Self(366);
    pub const EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const END_EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const END_EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const MYCON_SUN_VISITS: Self = Self(369);
    pub const END_MYCON_SUN_VISITS: Self = Self(371);
    pub const ORZ_HOME_VISITS: Self = Self(372);
    pub const END_ORZ_HOME_VISITS: Self = Self(374);
    pub const MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const END_MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const END_MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const END_MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const MELNORME_TECH_STACK: Self = Self(378);
    pub const END_MELNORME_TECH_STACK: Self = Self(381);
    pub const MELNORME_EVENTS_INFO_STACK: Self = Self(382);
    pub const END_MELNORME_EVENTS_INFO_STACK: Self = Self(386);
    pub const MELNORME_ALIEN_INFO_STACK: Self = Self(387);
    pub const END_MELNORME_ALIEN_INFO_STACK: Self = Self(391);
    pub const MELNORME_HISTORY_INFO_STACK: Self = Self(392);
    pub const END_MELNORME_HISTORY_INFO_STACK: Self = Self(396);
    pub const RAINBOW_WORLD0: Self = Self(397);
    pub const END_RAINBOW_WORLD0: Self = Self(404);
    pub const RAINBOW_WORLD1: Self = Self(405);
    pub const END_RAINBOW_WORLD1: Self = Self(406);
    pub const MELNORME_RAINBOW_COUNT: Self = Self(407);
    pub const END_MELNORME_RAINBOW_COUNT: Self = Self(410);
    pub const USED_BROADCASTER: Self = Self(411);
    pub const END_USED_BROADCASTER: Self = Self(411);
    pub const BROADCASTER_RESPONSE: Self = Self(412);
    pub const END_BROADCASTER_RESPONSE: Self = Self(412);
    pub const IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const END_IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const END_IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const END_IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const MET_ORZ_BEFORE: Self = Self(416);
    pub const END_MET_ORZ_BEFORE: Self = Self(416);
    pub const YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const END_YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const PLAYER_HAD_SEX: Self = Self(418);
    pub const END_PLAYER_HAD_SEX: Self = Self(418);
    pub const UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const END_UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const LIGHT_MINERAL_LOAD: Self = Self(420);
    pub const END_LIGHT_MINERAL_LOAD: Self = Self(422);
    pub const MEDIUM_MINERAL_LOAD: Self = Self(423);
    pub const END_MEDIUM_MINERAL_LOAD: Self = Self(425);
    pub const HEAVY_MINERAL_LOAD: Self = Self(426);
    pub const END_HEAVY_MINERAL_LOAD: Self = Self(428);
    pub const STARBASE_BULLETS0: Self = Self(429);
    pub const END_STARBASE_BULLETS0: Self = Self(436);
    pub const STARBASE_BULLETS1: Self = Self(437);
    pub const END_STARBASE_BULLETS1: Self = Self(444);
    pub const STARBASE_BULLETS2: Self = Self(445);
    pub const END_STARBASE_BULLETS2: Self = Self(452);
    pub const STARBASE_BULLETS3: Self = Self(453);
    pub const END_STARBASE_BULLETS3: Self = Self(460);
    pub const STARBASE_MONTH: Self = Self(461);
    pub const END_STARBASE_MONTH: Self = Self(464);
    pub const STARBASE_DAY: Self = Self(465);
    pub const END_STARBASE_DAY: Self = Self(469);
    pub const CREW_SOLD_TO_DRUUGE0: Self = Self(470);
    pub const END_CREW_SOLD_TO_DRUUGE0: Self = Self(477);
    pub const CREW_PURCHASED0: Self = Self(478);
    pub const END_CREW_PURCHASED0: Self = Self(485);
    pub const CREW_PURCHASED1: Self = Self(486);
    pub const END_CREW_PURCHASED1: Self = Self(493);
    pub const URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const END_URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const THRADDASH_BODY_COUNT: Self = Self(495);
    pub const END_THRADDASH_BODY_COUNT: Self = Self(499);
    pub const UTWIG_SUPOX_MISSION: Self = Self(500);
    pub const END_UTWIG_SUPOX_MISSION: Self = Self(502);
    pub const SPATHI_INFO: Self = Self(503);
    pub const END_SPATHI_INFO: Self = Self(505);
    pub const ILWRATH_INFO: Self = Self(506);
    pub const END_ILWRATH_INFO: Self = Self(507);
    pub const ILWRATH_GODS_SPOKEN: Self = Self(508);
    pub const END_ILWRATH_GODS_SPOKEN: Self = Self(511);
    pub const ILWRATH_WORSHIP: Self = Self(512);
    pub const END_ILWRATH_WORSHIP: Self = Self(513);
    pub const ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const END_ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const END_READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const END_URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const END_MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const URQUAN_INFO: Self = Self(518);
    pub const END_URQUAN_INFO: Self = Self(519);
    pub const KNOW_URQUAN_STORY: Self = Self(520);
    pub const END_KNOW_URQUAN_STORY: Self = Self(521);
    pub const MYCON_INFO: Self = Self(522);
    pub const END_MYCON_INFO: Self = Self(525);
    pub const MYCON_RAMBLE: Self = Self(526);
    pub const END_MYCON_RAMBLE: Self = Self(530);
    pub const KNOW_ABOUT_SHATTERED: Self = Self(531);
    pub const END_KNOW_ABOUT_SHATTERED: Self = Self(532);
    pub const MYCON_INSULTS: Self = Self(533);
    pub const END_MYCON_INSULTS: Self = Self(535);
    pub const MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const END_MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const SYREEN_INFO: Self = Self(537);
    pub const END_SYREEN_INFO: Self = Self(538);
    pub const KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const END_KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const END_SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const TALKING_PET_INFO: Self = Self(541);
    pub const END_TALKING_PET_INFO: Self = Self(543);
    pub const TALKING_PET_SUGGESTIONS: Self = Self(544);
    pub const END_TALKING_PET_SUGGESTIONS: Self = Self(546);
    pub const LEARNED_TALKING_PET: Self = Self(547);
    pub const END_LEARNED_TALKING_PET: Self = Self(547);
    pub const DNYARRI_LIED: Self = Self(548);
    pub const END_DNYARRI_LIED: Self = Self(548);
    pub const SHIP_TO_COMPEL: Self = Self(549);
    pub const END_SHIP_TO_COMPEL: Self = Self(549);
    pub const ORZ_GENERAL_INFO: Self = Self(550);
    pub const END_ORZ_GENERAL_INFO: Self = Self(551);
    pub const ORZ_PERSONAL_INFO: Self = Self(552);
    pub const END_ORZ_PERSONAL_INFO: Self = Self(554);
    pub const ORZ_ANDRO_STATE: Self = Self(555);
    pub const END_ORZ_ANDRO_STATE: Self = Self(556);
    pub const REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const END_REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const PKUNK_MANNER: Self = Self(558);
    pub const END_PKUNK_MANNER: Self = Self(559);
    pub const PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const END_PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const PKUNK_FLEET: Self = Self(561);
    pub const END_PKUNK_FLEET: Self = Self(562);
    pub const PKUNK_MIGRATE: Self = Self(563);
    pub const END_PKUNK_MIGRATE: Self = Self(564);
    pub const PKUNK_RETURN: Self = Self(565);
    pub const END_PKUNK_RETURN: Self = Self(565);
    pub const PKUNK_WORRY: Self = Self(566);
    pub const END_PKUNK_WORRY: Self = Self(567);
    pub const PKUNK_INFO: Self = Self(568);
    pub const END_PKUNK_INFO: Self = Self(570);
    pub const PKUNK_WAR: Self = Self(571);
    pub const END_PKUNK_WAR: Self = Self(572);
    pub const PKUNK_FORTUNE: Self = Self(573);
    pub const END_PKUNK_FORTUNE: Self = Self(575);
    pub const PKUNK_MIGRATE_VISITS: Self = Self(576);
    pub const END_PKUNK_MIGRATE_VISITS: Self = Self(578);
    pub const PKUNK_REASONS: Self = Self(579);
    pub const END_PKUNK_REASONS: Self = Self(582);
    pub const PKUNK_SWITCH: Self = Self(583);
    pub const END_PKUNK_SWITCH: Self = Self(583);
    pub const PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const END_PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const KOHR_AH_REASONS: Self = Self(585);
    pub const END_KOHR_AH_REASONS: Self = Self(586);
    pub const KOHR_AH_PLEAD: Self = Self(587);
    pub const END_KOHR_AH_PLEAD: Self = Self(588);
    pub const KOHR_AH_INFO: Self = Self(589);
    pub const END_KOHR_AH_INFO: Self = Self(590);
    pub const KNOW_KOHR_AH_STORY: Self = Self(591);
    pub const END_KNOW_KOHR_AH_STORY: Self = Self(592);
    pub const KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const END_KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const URQUAN_SENSES_EVIL: Self = Self(594);
    pub const END_URQUAN_SENSES_EVIL: Self = Self(594);
    pub const SLYLANDRO_PROBE_VISITS: Self = Self(595);
    pub const END_SLYLANDRO_PROBE_VISITS: Self = Self(597);
    pub const SLYLANDRO_PROBE_THREAT: Self = Self(598);
    pub const END_SLYLANDRO_PROBE_THREAT: Self = Self(599);
    pub const SLYLANDRO_PROBE_WRONG: Self = Self(600);
    pub const END_SLYLANDRO_PROBE_WRONG: Self = Self(601);
    pub const SLYLANDRO_PROBE_ID: Self = Self(602);
    pub const END_SLYLANDRO_PROBE_ID: Self = Self(603);
    pub const SLYLANDRO_PROBE_INFO: Self = Self(604);
    pub const END_SLYLANDRO_PROBE_INFO: Self = Self(605);
    pub const SLYLANDRO_PROBE_EXIT: Self = Self(606);
    pub const END_SLYLANDRO_PROBE_EXIT: Self = Self(607);
    pub const UMGAH_HOSTILE: Self = Self(608);
    pub const END_UMGAH_HOSTILE: Self = Self(608);
    pub const UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const END_UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const UMGAH_MENTIONED_TRICKS: Self = Self(610);
    pub const END_UMGAH_MENTIONED_TRICKS: Self = Self(611);
    pub const BOMB_CARRIER: Self = Self(612);
    pub const END_BOMB_CARRIER: Self = Self(612);
    pub const THRADD_MANNER: Self = Self(613);
    pub const END_THRADD_MANNER: Self = Self(613);
    pub const THRADD_INTRO: Self = Self(614);
    pub const END_THRADD_INTRO: Self = Self(615);
    pub const THRADD_DEMEANOR: Self = Self(616);
    pub const END_THRADD_DEMEANOR: Self = Self(618);
    pub const THRADD_INFO: Self = Self(619);
    pub const END_THRADD_INFO: Self = Self(620);
    pub const THRADD_BODY_LEVEL: Self = Self(621);
    pub const END_THRADD_BODY_LEVEL: Self = Self(622);
    pub const THRADD_MISSION_VISITS: Self = Self(623);
    pub const END_THRADD_MISSION_VISITS: Self = Self(623);
    pub const THRADD_STACK_1: Self = Self(624);
    pub const END_THRADD_STACK_1: Self = Self(626);
    pub const THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const END_THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const END_THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const END_THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const END_THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const CHMMR_STACK: Self = Self(631);
    pub const END_CHMMR_STACK: Self = Self(632);
    pub const ARILOU_MANNER: Self = Self(633);
    pub const END_ARILOU_MANNER: Self = Self(634);
    pub const NO_PORTAL_VISITS: Self = Self(635);
    pub const END_NO_PORTAL_VISITS: Self = Self(635);
    pub const ARILOU_STACK_1: Self = Self(636);
    pub const END_ARILOU_STACK_1: Self = Self(637);
    pub const ARILOU_STACK_2: Self = Self(638);
    pub const END_ARILOU_STACK_2: Self = Self(638);
    pub const ARILOU_STACK_3: Self = Self(639);
    pub const END_ARILOU_STACK_3: Self = Self(640);
    pub const ARILOU_STACK_4: Self = Self(641);
    pub const END_ARILOU_STACK_4: Self = Self(641);
    pub const ARILOU_STACK_5: Self = Self(642);
    pub const END_ARILOU_STACK_5: Self = Self(643);
    pub const ARILOU_INFO: Self = Self(644);
    pub const END_ARILOU_INFO: Self = Self(645);
    pub const ARILOU_HINTS: Self = Self(646);
    pub const END_ARILOU_HINTS: Self = Self(647);
    pub const DRUUGE_MANNER: Self = Self(648);
    pub const END_DRUUGE_MANNER: Self = Self(648);
    pub const DRUUGE_SPACE_INFO: Self = Self(649);
    pub const END_DRUUGE_SPACE_INFO: Self = Self(650);
    pub const DRUUGE_HOME_INFO: Self = Self(651);
    pub const END_DRUUGE_HOME_INFO: Self = Self(652);
    pub const DRUUGE_SALVAGE: Self = Self(653);
    pub const END_DRUUGE_SALVAGE: Self = Self(653);
    pub const KNOW_DRUUGE_SLAVERS: Self = Self(654);
    pub const END_KNOW_DRUUGE_SLAVERS: Self = Self(655);
    pub const FRAGMENTS_BOUGHT: Self = Self(656);
    pub const END_FRAGMENTS_BOUGHT: Self = Self(657);
    pub const ZEX_STACK_1: Self = Self(658);
    pub const END_ZEX_STACK_1: Self = Self(659);
    pub const ZEX_STACK_2: Self = Self(660);
    pub const END_ZEX_STACK_2: Self = Self(661);
    pub const ZEX_STACK_3: Self = Self(662);
    pub const END_ZEX_STACK_3: Self = Self(663);
    pub const VUX_INFO: Self = Self(664);
    pub const END_VUX_INFO: Self = Self(665);
    pub const VUX_STACK_1: Self = Self(666);
    pub const END_VUX_STACK_1: Self = Self(669);
    pub const VUX_STACK_2: Self = Self(670);
    pub const END_VUX_STACK_2: Self = Self(671);
    pub const VUX_STACK_3: Self = Self(672);
    pub const END_VUX_STACK_3: Self = Self(673);
    pub const VUX_STACK_4: Self = Self(674);
    pub const END_VUX_STACK_4: Self = Self(675);
    pub const SHOFIXTI_STACK4: Self = Self(676);
    pub const END_SHOFIXTI_STACK4: Self = Self(677);
    pub const YEHAT_REBEL_INFO: Self = Self(678);
    pub const END_YEHAT_REBEL_INFO: Self = Self(680);
    pub const YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const END_YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const END_YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const END_NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const END_NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const NO_YEHAT_INFO: Self = Self(685);
    pub const END_NO_YEHAT_INFO: Self = Self(685);
    pub const NO_YEHAT_ALLY_SPACE: Self = Self(686);
    pub const END_NO_YEHAT_ALLY_SPACE: Self = Self(687);
    pub const NO_YEHAT_HELP_SPACE: Self = Self(688);
    pub const END_NO_YEHAT_HELP_SPACE: Self = Self(689);
    pub const ZOQFOT_KNOW_MASK: Self = Self(690);
    pub const END_ZOQFOT_KNOW_MASK: Self = Self(693);
    pub const SUPOX_HOSTILE: Self = Self(694);
    pub const END_SUPOX_HOSTILE: Self = Self(694);
    pub const SUPOX_INFO: Self = Self(695);
    pub const END_SUPOX_INFO: Self = Self(695);
    pub const SUPOX_WAR_NEWS: Self = Self(696);
    pub const END_SUPOX_WAR_NEWS: Self = Self(697);
    pub const SUPOX_ULTRON_HELP: Self = Self(698);
    pub const END_SUPOX_ULTRON_HELP: Self = Self(698);
    pub const SUPOX_STACK1: Self = Self(699);
    pub const END_SUPOX_STACK1: Self = Self(701);
    pub const SUPOX_STACK2: Self = Self(702);
    pub const END_SUPOX_STACK2: Self = Self(703);
    pub const UTWIG_HOSTILE: Self = Self(704);
    pub const END_UTWIG_HOSTILE: Self = Self(704);
    pub const UTWIG_INFO: Self = Self(705);
    pub const END_UTWIG_INFO: Self = Self(705);
    pub const UTWIG_WAR_NEWS: Self = Self(706);
    pub const END_UTWIG_WAR_NEWS: Self = Self(707);
    pub const UTWIG_STACK1: Self = Self(708);
    pub const END_UTWIG_STACK1: Self = Self(710);
    pub const UTWIG_STACK2: Self = Self(711);
    pub const END_UTWIG_STACK2: Self = Self(712);
    pub const BOMB_INFO: Self = Self(713);
    pub const END_BOMB_INFO: Self = Self(713);
    pub const BOMB_STACK1: Self = Self(714);
    pub const END_BOMB_STACK1: Self = Self(715);
    pub const BOMB_STACK2: Self = Self(716);
    pub const END_BOMB_STACK2: Self = Self(717);
    pub const SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const END_SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const END_PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const END_PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const END_PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const END_PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const SLYLANDRO_STACK1: Self = Self(723);
    pub const END_SLYLANDRO_STACK1: Self = Self(725);
    pub const SLYLANDRO_STACK2: Self = Self(726);
    pub const END_SLYLANDRO_STACK2: Self = Self(726);
    pub const SLYLANDRO_STACK3: Self = Self(727);
    pub const END_SLYLANDRO_STACK3: Self = Self(728);
    pub const SLYLANDRO_STACK4: Self = Self(729);
    pub const END_SLYLANDRO_STACK4: Self = Self(730);
    pub const SLYLANDRO_STACK5: Self = Self(731);
    pub const END_SLYLANDRO_STACK5: Self = Self(731);
    pub const SLYLANDRO_STACK6: Self = Self(732);
    pub const END_SLYLANDRO_STACK6: Self = Self(732);
    pub const SLYLANDRO_STACK7: Self = Self(733);
    pub const END_SLYLANDRO_STACK7: Self = Self(734);
    pub const SLYLANDRO_STACK8: Self = Self(735);
    pub const END_SLYLANDRO_STACK8: Self = Self(736);
    pub const SLYLANDRO_STACK9: Self = Self(737);
    pub const END_SLYLANDRO_STACK9: Self = Self(738);
    pub const SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const END_SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const END_SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const END_SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const SLYLANDRO_KNOW_URQUAN: Self = Self(742);
    pub const END_SLYLANDRO_KNOW_URQUAN: Self = Self(743);
    pub const RECALL_VISITS: Self = Self(744);
    pub const END_RECALL_VISITS: Self = Self(745);
    pub const SLYLANDRO_MULTIPLIER: Self = Self(746);
    pub const END_SLYLANDRO_MULTIPLIER: Self = Self(748);
    pub const KNOW_SPATHI_QUEST: Self = Self(749);
    pub const END_KNOW_SPATHI_QUEST: Self = Self(749);
    pub const KNOW_SPATHI_EVIL: Self = Self(750);
    pub const END_KNOW_SPATHI_EVIL: Self = Self(750);
    pub const BATTLE_PLANET: Self = Self(751);
    pub const END_BATTLE_PLANET: Self = Self(758);
    pub const ESCAPE_COUNTER: Self = Self(759);
    pub const END_ESCAPE_COUNTER: Self = Self(766);
    pub const CREW_SOLD_TO_DRUUGE1: Self = Self(767);
    pub const END_CREW_SOLD_TO_DRUUGE1: Self = Self(774);
    pub const PKUNK_DONE_WAR: Self = Self(775);
    pub const END_PKUNK_DONE_WAR: Self = Self(775);
    pub const SYREEN_STACK0: Self = Self(776);
    pub const END_SYREEN_STACK0: Self = Self(777);
    pub const SYREEN_STACK1: Self = Self(778);
    pub const END_SYREEN_STACK1: Self = Self(779);
    pub const SYREEN_STACK2: Self = Self(780);
    pub const END_SYREEN_STACK2: Self = Self(781);
    pub const REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const END_REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const NO_TRICK_AT_SUN: Self = Self(783);
    pub const END_NO_TRICK_AT_SUN: Self = Self(783);
    pub const SPATHI_STACK0: Self = Self(784);
    pub const END_SPATHI_STACK0: Self = Self(785);
    pub const SPATHI_STACK1: Self = Self(786);
    pub const END_SPATHI_STACK1: Self = Self(786);
    pub const SPATHI_STACK2: Self = Self(787);
    pub const END_SPATHI_STACK2: Self = Self(787);
    pub const ORZ_STACK0: Self = Self(788);
    pub const END_ORZ_STACK0: Self = Self(788);
    pub const ORZ_STACK1: Self = Self(789);
    pub const END_ORZ_STACK1: Self = Self(789);
    pub const SHOFIXTI_GRPOFFS0: Self = Self(790);
    pub const END_SHOFIXTI_GRPOFFS0: Self = Self(797);
    pub const SHOFIXTI_GRPOFFS1: Self = Self(798);
    pub const END_SHOFIXTI_GRPOFFS1: Self = Self(805);
    pub const SHOFIXTI_GRPOFFS2: Self = Self(806);
    pub const END_SHOFIXTI_GRPOFFS2: Self = Self(813);
    pub const SHOFIXTI_GRPOFFS3: Self = Self(814);
    pub const END_SHOFIXTI_GRPOFFS3: Self = Self(821);
    pub const ZOQFOT_GRPOFFS0: Self = Self(822);
    pub const END_ZOQFOT_GRPOFFS0: Self = Self(829);
    pub const ZOQFOT_GRPOFFS1: Self = Self(830);
    pub const END_ZOQFOT_GRPOFFS1: Self = Self(837);
    pub const ZOQFOT_GRPOFFS2: Self = Self(838);
    pub const END_ZOQFOT_GRPOFFS2: Self = Self(845);
    pub const ZOQFOT_GRPOFFS3: Self = Self(846);
    pub const END_ZOQFOT_GRPOFFS3: Self = Self(853);
    pub const MELNORME0_GRPOFFS0: Self = Self(854);
    pub const END_MELNORME0_GRPOFFS0: Self = Self(861);
    pub const MELNORME0_GRPOFFS1: Self = Self(862);
    pub const END_MELNORME0_GRPOFFS1: Self = Self(869);
    pub const MELNORME0_GRPOFFS2: Self = Self(870);
    pub const END_MELNORME0_GRPOFFS2: Self = Self(877);
    pub const MELNORME0_GRPOFFS3: Self = Self(878);
    pub const END_MELNORME0_GRPOFFS3: Self = Self(885);
    pub const MELNORME1_GRPOFFS0: Self = Self(886);
    pub const END_MELNORME1_GRPOFFS0: Self = Self(893);
    pub const MELNORME1_GRPOFFS1: Self = Self(894);
    pub const END_MELNORME1_GRPOFFS1: Self = Self(901);
    pub const MELNORME1_GRPOFFS2: Self = Self(902);
    pub const END_MELNORME1_GRPOFFS2: Self = Self(909);
    pub const MELNORME1_GRPOFFS3: Self = Self(910);
    pub const END_MELNORME1_GRPOFFS3: Self = Self(917);
    pub const MELNORME2_GRPOFFS0: Self = Self(918);
    pub const END_MELNORME2_GRPOFFS0: Self = Self(925);
    pub const MELNORME2_GRPOFFS1: Self = Self(926);
    pub const END_MELNORME2_GRPOFFS1: Self = Self(933);
    pub const MELNORME2_GRPOFFS2: Self = Self(934);
    pub const END_MELNORME2_GRPOFFS2: Self = Self(941);
    pub const MELNORME2_GRPOFFS3: Self = Self(942);
    pub const END_MELNORME2_GRPOFFS3: Self = Self(949);
    pub const MELNORME3_GRPOFFS0: Self = Self(950);
    pub const END_MELNORME3_GRPOFFS0: Self = Self(957);
    pub const MELNORME3_GRPOFFS1: Self = Self(958);
    pub const END_MELNORME3_GRPOFFS1: Self = Self(965);
    pub const MELNORME3_GRPOFFS2: Self = Self(966);
    pub const END_MELNORME3_GRPOFFS2: Self = Self(973);
    pub const MELNORME3_GRPOFFS3: Self = Self(974);
    pub const END_MELNORME3_GRPOFFS3: Self = Self(981);
    pub const MELNORME4_GRPOFFS0: Self = Self(982);
    pub const END_MELNORME4_GRPOFFS0: Self = Self(989);
    pub const MELNORME4_GRPOFFS1: Self = Self(990);
    pub const END_MELNORME4_GRPOFFS1: Self = Self(997);
    pub const MELNORME4_GRPOFFS2: Self = Self(998);
    pub const END_MELNORME4_GRPOFFS2: Self = Self(1005);
    pub const MELNORME4_GRPOFFS3: Self = Self(1006);
    pub const END_MELNORME4_GRPOFFS3: Self = Self(1013);
    pub const MELNORME5_GRPOFFS0: Self = Self(1014);
    pub const END_MELNORME5_GRPOFFS0: Self = Self(1021);
    pub const MELNORME5_GRPOFFS1: Self = Self(1022);
    pub const END_MELNORME5_GRPOFFS1: Self = Self(1029);
    pub const MELNORME5_GRPOFFS2: Self = Self(1030);
    pub const END_MELNORME5_GRPOFFS2: Self = Self(1037);
    pub const MELNORME5_GRPOFFS3: Self = Self(1038);
    pub const END_MELNORME5_GRPOFFS3: Self = Self(1045);
    pub const MELNORME6_GRPOFFS0: Self = Self(1046);
    pub const END_MELNORME6_GRPOFFS0: Self = Self(1053);
    pub const MELNORME6_GRPOFFS1: Self = Self(1054);
    pub const END_MELNORME6_GRPOFFS1: Self = Self(1061);
    pub const MELNORME6_GRPOFFS2: Self = Self(1062);
    pub const END_MELNORME6_GRPOFFS2: Self = Self(1069);
    pub const MELNORME6_GRPOFFS3: Self = Self(1070);
    pub const END_MELNORME6_GRPOFFS3: Self = Self(1077);
    pub const MELNORME7_GRPOFFS0: Self = Self(1078);
    pub const END_MELNORME7_GRPOFFS0: Self = Self(1085);
    pub const MELNORME7_GRPOFFS1: Self = Self(1086);
    pub const END_MELNORME7_GRPOFFS1: Self = Self(1093);
    pub const MELNORME7_GRPOFFS2: Self = Self(1094);
    pub const END_MELNORME7_GRPOFFS2: Self = Self(1101);
    pub const MELNORME7_GRPOFFS3: Self = Self(1102);
    pub const END_MELNORME7_GRPOFFS3: Self = Self(1109);
    pub const MELNORME8_GRPOFFS0: Self = Self(1110);
    pub const END_MELNORME8_GRPOFFS0: Self = Self(1117);
    pub const MELNORME8_GRPOFFS1: Self = Self(1118);
    pub const END_MELNORME8_GRPOFFS1: Self = Self(1125);
    pub const MELNORME8_GRPOFFS2: Self = Self(1126);
    pub const END_MELNORME8_GRPOFFS2: Self = Self(1133);
    pub const MELNORME8_GRPOFFS3: Self = Self(1134);
    pub const END_MELNORME8_GRPOFFS3: Self = Self(1141);
    pub const URQUAN_PROBE_GRPOFFS0: Self = Self(1142);
    pub const END_URQUAN_PROBE_GRPOFFS0: Self = Self(1149);
    pub const URQUAN_PROBE_GRPOFFS1: Self = Self(1150);
    pub const END_URQUAN_PROBE_GRPOFFS1: Self = Self(1157);
    pub const URQUAN_PROBE_GRPOFFS2: Self = Self(1158);
    pub const END_URQUAN_PROBE_GRPOFFS2: Self = Self(1165);
    pub const URQUAN_PROBE_GRPOFFS3: Self = Self(1166);
    pub const END_URQUAN_PROBE_GRPOFFS3: Self = Self(1173);
    pub const COLONY_GRPOFFS0: Self = Self(1174);
    pub const END_COLONY_GRPOFFS0: Self = Self(1181);
    pub const COLONY_GRPOFFS1: Self = Self(1182);
    pub const END_COLONY_GRPOFFS1: Self = Self(1189);
    pub const COLONY_GRPOFFS2: Self = Self(1190);
    pub const END_COLONY_GRPOFFS2: Self = Self(1197);
    pub const COLONY_GRPOFFS3: Self = Self(1198);
    pub const END_COLONY_GRPOFFS3: Self = Self(1205);
    pub const SAMATRA_GRPOFFS0: Self = Self(1206);
    pub const END_SAMATRA_GRPOFFS0: Self = Self(1213);
    pub const SAMATRA_GRPOFFS1: Self = Self(1214);
    pub const END_SAMATRA_GRPOFFS1: Self = Self(1221);
    pub const SAMATRA_GRPOFFS2: Self = Self(1222);
    pub const END_SAMATRA_GRPOFFS2: Self = Self(1229);
    pub const SAMATRA_GRPOFFS3: Self = Self(1230);
    pub const END_SAMATRA_GRPOFFS3: Self = Self(1237);
    pub const NUM_GAME_STATE_BITS: Self = Self(1238);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_22(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_22 {
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
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
#[inline]
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> COORD {
    return ((lx
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int)
        + ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int)) as COORD;
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> COORD {
    return (9999 as ::core::ffi::c_int
        - (ly
            * (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
            + ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))
            / ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int)) as COORD;
}
#[inline]
unsafe extern "C" fn universeToLogx(mut ux: COORD) -> SDWORD {
    return (ux as SDWORD
        * ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as SDWORD)
        + ((9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int) * 10 as SDWORD
            >> 1 as ::core::ffi::c_int))
        / ((9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int) * 10 as SDWORD);
}
#[inline]
unsafe extern "C" fn universeToLogy(mut uy: COORD) -> SDWORD {
    return ((9999 as SDWORD - uy as SDWORD)
        * ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 25 as SDWORD)
        + (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int);
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
                329 as ::core::ffi::c_int,
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
                395 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[no_mangle]
pub static mut instantMove: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut disableInteractivity: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut debugHook: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub unsafe extern "C" fn debugKeyPressedSynchronous() {
    equipShip();
    giveDevices();
    clearEscorts();
    AddEscortShips(
        C2Rust_Unnamed_14::ARILOU_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::PKUNK_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::VUX_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::MELNORME_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::DRUUGE_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::MYCON_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    AddEscortShips(
        C2Rust_Unnamed_14::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
        1 as SIZE,
    );
    resetCrewBattle();
    resetEnergyBattle();
    instantMove = BOOLEAN(
        (instantMove.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    showSpheres();
    activateAllShips();
}
#[no_mangle]
pub unsafe extern "C" fn debugKeyPressed() {}
#[no_mangle]
pub unsafe extern "C" fn forwardToNextEvent(mut skipHEE: BOOLEAN) {
    let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
    let mut year: COUNT = 0;
    let mut month: COUNT = 0;
    let mut day: COUNT = 0;
    let mut done: BOOLEAN = BOOLEAN::FALSE;
    if GameClockRunning().0 == 0 {
        return;
    }
    LockGameClock();
    done = BOOLEAN((skipHEE.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint);
    loop {
        hEvent = GlobData.Game_state.GameClock.event_q.head as HEVENT;
        if hEvent.is_null() {
            return;
        }
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
            as *mut EVENT;
        if (*EventPtr).func_index as ::core::ffi::c_int
            != C2Rust_Unnamed_19::HYPERSPACE_ENCOUNTER_EVENT.0 as ::core::ffi::c_int
        {
            done = BOOLEAN::TRUE;
        }
        year = (*EventPtr).year_index;
        month = (*EventPtr).month_index as COUNT;
        day = (*EventPtr).day_index as COUNT;
        UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
        while !(GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
            > year as ::core::ffi::c_int
            || GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                == year as ::core::ffi::c_int
                && (GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                    > month as ::core::ffi::c_int
                    || GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                        == month as ::core::ffi::c_int
                        && GlobData.Game_state.GameClock.day_index as ::core::ffi::c_int
                            >= day as ::core::ffi::c_int))
        {
            MoveGameClockDays(1 as COUNT);
        }
        if done.0 != 0 {
            break;
        }
    }
    UnlockGameClock();
}
#[no_mangle]
pub unsafe extern "C" fn eventName(mut func_index: BYTE) -> *const ::core::ffi::c_char {
    match func_index as ::core::ffi::c_int {
        0 => return b"ARILOU_ENTRANCE_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"ARILOU_EXIT_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        2 => {
            return b"HYPERSPACE_ENCOUNTER_EVENT\0".as_ptr() as *const ::core::ffi::c_char;
        }
        3 => return b"KOHR_AH_VICTORIOUS_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"ADVANCE_PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"ADVANCE_THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"ZOQFOT_DEATH_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"SHOFIXTI_RETURN_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        9 => {
            return b"ADVANCE_UTWIG_SUPOX_MISSION\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        10 => return b"KOHR_AH_GENOCIDE_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        11 => return b"SPATHI_SHIELD_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        12 => return b"ADVANCE_ILWRATH_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        13 => return b"ADVANCE_MYCON_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        14 => return b"ARILOU_UMGAH_CHECK\0".as_ptr() as *const ::core::ffi::c_char,
        15 => return b"YEHAT_REBEL_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
        16 => return b"SLYLANDRO_RAMP_UP\0".as_ptr() as *const ::core::ffi::c_char,
        17 => return b"SLYLANDRO_RAMP_DOWN\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn dumpEventCallback(
    mut eventPtr: *const EVENT,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut out: *mut FILE = arg as *mut FILE;
    dumpEvent(out, eventPtr);
}
#[no_mangle]
pub unsafe extern "C" fn dumpEvent(mut out: *mut FILE, mut eventPtr: *const EVENT) {
    fprintf(
        out,
        b"%4u/%02u/%02u: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*eventPtr).year_index as ::core::ffi::c_int,
        (*eventPtr).month_index as ::core::ffi::c_int,
        (*eventPtr).day_index as ::core::ffi::c_int,
        eventName((*eventPtr).func_index),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dumpEvents(mut out: *mut FILE) {
    LockGameClock();
    ForAllLinks(
        &raw mut GlobData.Game_state.GameClock.event_q,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const EVENT, *mut ::core::ffi::c_void) -> ()>,
            Option<unsafe extern "C" fn(*mut LINK, *mut ::core::ffi::c_void) -> ()>,
        >(
            Some(
                dumpEventCallback
                    as unsafe extern "C" fn(*const EVENT, *mut ::core::ffi::c_void) -> (),
            ),
        ),
        out as *mut ::core::ffi::c_void,
    );
    UnlockGameClock();
}
#[no_mangle]
pub unsafe extern "C" fn equipShip() {
    let mut i: ::core::ffi::c_int = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_22::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 11 as ::core::ffi::c_int {
        GlobData.SIS_state.DriveSlots[i as usize] = C2Rust_Unnamed_17::FUSION_THRUSTER.0
            as ::core::ffi::c_int as BYTE;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        GlobData.SIS_state.JetSlots[i as usize] = C2Rust_Unnamed_17::TURNING_JETS.0
            as ::core::ffi::c_int as BYTE;
        i += 1;
    }
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::LANDER_SHIELDS.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
        ((1 as ::core::ffi::c_int)
            << C2Rust_Unnamed::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed::LIGHTNING_DISASTER.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed::LAVASPOT_DISASTER.0 as ::core::ffi::c_int) as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    if (getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        let c2rust_fresh0 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh0 as usize] = C2Rust_Unnamed_17::HIGHEFF_FUELSYS
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh1 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh1 as usize] = C2Rust_Unnamed_17::HIGHEFF_FUELSYS
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh2 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh2 as usize] = C2Rust_Unnamed_17::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh3 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh3 as usize] = C2Rust_Unnamed_17::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh4 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh4 as usize] = C2Rust_Unnamed_17::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh5 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh5 as usize] = C2Rust_Unnamed_17::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh6 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh6 as usize] = C2Rust_Unnamed_17::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh7 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh7 as usize] = C2Rust_Unnamed_17::STORAGE_BAY
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh8 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh8 as usize] = C2Rust_Unnamed_17::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh9 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh9 as usize] = C2Rust_Unnamed_17::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh10 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh10 as usize] = C2Rust_Unnamed_17::DYNAMO_UNIT
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh11 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh11 as usize] = C2Rust_Unnamed_17::TRACKING_SYSTEM
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh12 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh12 as usize] = C2Rust_Unnamed_17::TRACKING_SYSTEM
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh13 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh13 as usize] = C2Rust_Unnamed_17::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh14 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh14 as usize] = C2Rust_Unnamed_17::CANNON_WEAPON
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh15 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh15 as usize] = C2Rust_Unnamed_17::CANNON_WEAPON
            .0 as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.NumLanders = 10 as BYTE;
    } else {
        i = 10 as ::core::ffi::c_int;
        let c2rust_fresh16 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh16 as usize] = C2Rust_Unnamed_17::HIGHEFF_FUELSYS
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh17 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh17 as usize] = C2Rust_Unnamed_17::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh18 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh18 as usize] = C2Rust_Unnamed_17::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh19 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh19 as usize] = C2Rust_Unnamed_17::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh20 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh20 as usize] = C2Rust_Unnamed_17::CANNON_WEAPON
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh21 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh21 as usize] = C2Rust_Unnamed_17::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
    }
    (i <= 16 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"i <= NUM_MODULE_SLOTS\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int,
                b"equipShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    GlobData.SIS_state.FuelOnBoard = (100 as ::core::ffi::c_int
        * 100 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as DWORD;
    GlobData.SIS_state.CrewEnlisted = 0 as COUNT;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        match GlobData.SIS_state.ModuleSlots[i as usize] as ::core::ffi::c_int {
            3 => {
                GlobData.SIS_state.CrewEnlisted = (GlobData.SIS_state.CrewEnlisted
                    as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as COUNT;
            }
            5 => {
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_add(
                        (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
                    );
            }
            6 => {
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_add(
                        (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
                    );
            }
            _ => {}
        }
        i += 1;
    }
    if !pSolarSysState.is_null() {
        (*pSolarSysState).max_ship_speed = (5 as ::core::ffi::c_int
            * 8 as ::core::ffi::c_int) as BYTE;
        i = 0 as ::core::ffi::c_int;
        while i < 11 as ::core::ffi::c_int {
            if GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int
                == C2Rust_Unnamed_17::FUSION_THRUSTER.0 as ::core::ffi::c_int
            {
                (*pSolarSysState).max_ship_speed = ((*pSolarSysState).max_ship_speed
                    as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as BYTE;
            }
            i += 1;
        }
        (*pSolarSysState).turn_wait = 17 as BYTE;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            if GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int
                == C2Rust_Unnamed_17::TURNING_JETS.0 as ::core::ffi::c_int
            {
                (*pSolarSysState).turn_wait = ((*pSolarSysState).turn_wait
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
            }
            i += 1;
        }
    }
    if inHQSpace().0 != 0
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_22::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn giveDevices() {
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::ARTIFACT_2_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_ARTIFACT_2_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::ARTIFACT_3_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_ARTIFACT_3_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::SUN_DEVICE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_SUN_DEVICE_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::ULTRON_CONDITION.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::MAIDENS_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_MAIDENS_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::UMGAH_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_UMGAH_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::TAALO_PROTECTOR_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_TAALO_PROTECTOR_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::EGG_CASE0_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_EGG_CASE0_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::EGG_CASE1_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_EGG_CASE1_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::EGG_CASE2_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_EGG_CASE2_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::SYREEN_SHUTTLE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_SYREEN_SHUTTLE_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::VUX_BEAST_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_VUX_BEAST_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::PORTAL_SPAWNER_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_PORTAL_SPAWNER_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::PORTAL_KEY_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_PORTAL_KEY_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::BURV_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_BURV_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clearEscorts() {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        FreeLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
    }
    DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn doInstantMove() {
    if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int)
        || GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int)
    {
        GlobData.Game_state.autopilot.x = logxToUniverse(GlobData.SIS_state.log_x);
        GlobData.Game_state.autopilot.y = logyToUniverse(GlobData.SIS_state.log_y);
    } else {
        GlobData.SIS_state.log_x = universeToLogx(GlobData.Game_state.autopilot.x);
        GlobData.SIS_state.log_y = universeToLogy(GlobData.Game_state.autopilot.y);
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_21::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_21::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
    {
        CurStarDescPtr = FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut GlobData.Game_state.autopilot,
            0 as SIZE,
            0 as SIZE,
        );
        if !CurStarDescPtr.is_null() {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_21::USED_BROADCASTER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_21::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_22::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            GlobData.Game_state.ShipFacing = 0 as UWORD;
            GlobData.Game_state.ip_planet = 0 as BYTE;
            GlobData.Game_state.in_orbit = 0 as BYTE;
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_22::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                as ACTIVITY;
        }
    }
    GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn showSpheres() {
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if (*FleetPtr).actual_strength as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            && (*FleetPtr).known_strength as ::core::ffi::c_int
                != (*FleetPtr).actual_strength as ::core::ffi::c_int
        {
            (*FleetPtr).known_strength = (*FleetPtr).actual_strength;
            (*FleetPtr).known_loc = (*FleetPtr).loc;
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hStarShip = hNextShip;
    }
}
#[no_mangle]
pub unsafe extern "C" fn activateAllShips() {
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if !(*FleetPtr).icons.is_null() {
            (*FleetPtr).allied_state = C2Rust_Unnamed_13::GOOD_GUY.0
                as ::core::ffi::c_int as UWORD;
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hStarShip = hNextShip;
    }
}
#[no_mangle]
pub unsafe extern "C" fn forAllStars(
    mut callback: Option<
        unsafe extern "C" fn(*mut STAR_DESC, *mut ::core::ffi::c_void) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_int = 0;
    extern "C" {
        static mut starmap_array: [STAR_DESC; 0];
    }
    i = 0 as ::core::ffi::c_int;
    while i < 502 as ::core::ffi::c_int {
        callback
            .expect(
                "non-null function pointer",
            )((&raw mut starmap_array as *mut STAR_DESC).offset(i as isize), arg);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn forAllPlanets(
    mut star: *mut STAR_DESC,
    mut system: *mut SOLARSYS_STATE,
    mut callback: Option<
        unsafe extern "C" fn(
            *mut STAR_DESC,
            *mut SOLARSYS_STATE,
            *mut PLANET_DESC,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut i: COUNT = 0;
    (CurStarDescPtr == star
        || {
            __assert_fail(
                b"CurStarDescPtr == star\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                581 as ::core::ffi::c_int,
                b"forAllPlanets\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (pSolarSysState == system
        || {
            __assert_fail(
                b"pSolarSysState == system\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                582 as ::core::ffi::c_int,
                b"forAllPlanets\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < (*system).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
    {
        callback
            .expect(
                "non-null function pointer",
            )(
            star,
            system,
            (&raw mut (*system).PlanetDesc as *mut PLANET_DESC).offset(i as isize),
            arg,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn forAllMoons(
    mut star: *mut STAR_DESC,
    mut system: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
    mut callback: Option<
        unsafe extern "C" fn(
            *mut STAR_DESC,
            *mut SOLARSYS_STATE,
            *mut PLANET_DESC,
            *mut PLANET_DESC,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut i: COUNT = 0;
    (pSolarSysState == system
        || {
            __assert_fail(
                b"pSolarSysState == system\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                595 as ::core::ffi::c_int,
                b"forAllMoons\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
        callback
            .expect(
                "non-null function pointer",
            )(
            star,
            system,
            planet,
            (&raw mut (*system).MoonDesc as *mut PLANET_DESC).offset(i as isize),
            arg,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UniverseRecurse(
    mut universeRecurseArg: *mut UniverseRecurseArg,
) {
    let mut savedActivity: ACTIVITY = 0;
    if (*universeRecurseArg).systemFuncPre.is_none()
        && (*universeRecurseArg).systemFuncPost.is_none()
        && (*universeRecurseArg).planetFuncPre.is_none()
        && (*universeRecurseArg).planetFuncPost.is_none()
        && (*universeRecurseArg).moonFunc.is_none()
    {
        return;
    }
    LockGameClock();
    savedActivity = GlobData.Game_state.CurrentActivity;
    disableInteractivity = BOOLEAN::TRUE;
    forAllStars(
        Some(
            starRecurse
                as unsafe extern "C" fn(*mut STAR_DESC, *mut ::core::ffi::c_void) -> (),
        ),
        universeRecurseArg as *mut ::core::ffi::c_void,
    );
    disableInteractivity = BOOLEAN::FALSE;
    GlobData.Game_state.CurrentActivity = savedActivity;
    UnlockGameClock();
}
unsafe extern "C" fn starRecurse(
    mut star: *mut STAR_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut universeRecurseArg: *mut UniverseRecurseArg = arg as *mut UniverseRecurseArg;
    let mut SolarSysState: SOLARSYS_STATE = SOLARSYS_STATE {
        InputFunc: None,
        InIpFlight: BOOLEAN::FALSE,
        WaitIntersect: 0,
        SunDesc: [PLANET_DESC {
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
        }; 1],
        PlanetDesc: [PLANET_DESC {
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
        }; 16],
        MoonDesc: [PLANET_DESC {
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
        }; 4],
        pBaseDesc: ::core::ptr::null_mut::<PLANET_DESC>(),
        pOrbitalDesc: ::core::ptr::null_mut::<PLANET_DESC>(),
        FirstPlanetIndex: 0,
        LastPlanetIndex: 0,
        turn_counter: 0,
        turn_wait: 0,
        thrust_counter: 0,
        max_ship_speed: 0,
        XlatRef: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        XlatPtr: ::core::ptr::null::<::core::ffi::c_void>(),
        OrbitalCMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        SysInfo: SYSTEM_INFO {
            StarSize: 0,
            StarIntensity: 0,
            StarEnergy: 0,
            PlanetInfo: PLANET_INFO {
                AxialTilt: 0,
                Tectonics: 0,
                Weather: 0,
                PlanetDensity: 0,
                PlanetRadius: 0,
                SurfaceGravity: 0,
                SurfaceTemperature: 0,
                RotationPeriod: 0,
                AtmoDensity: 0,
                LifeChance: 0,
                PlanetToSunDist: 0,
                PlanDataPtr: ::core::ptr::null::<PlanetFrame>(),
                ScanSeed: [0; 3],
                ScanRetrieveMask: [0; 3],
                DiscoveryString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
                LanderFont: ::core::ptr::null_mut::<FONT_DESC>(),
                LanderFontEff: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
        },
        genFuncs: ::core::ptr::null::<GenerateFunctions>(),
        PlanetSideFrame: [::core::ptr::null_mut::<FRAME_DESC>(); 6],
        TopoFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        Orbit: PLANET_ORBIT {
            TopoZoomFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
            lpTopoData: ::core::ptr::null_mut::<SBYTE>(),
            SphereFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
            ObjectFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
            TintFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
            TintColor: Color { r: 0, g: 0, b: 0, a: 0 },
            TopoColors: ::core::ptr::null_mut::<Color>(),
            ScratchArray: ::core::ptr::null_mut::<Color>(),
            WorkFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        InOrbit: BOOLEAN::FALSE,
    };
    let mut oldPSolarSysState: *mut SOLARSYS_STATE = pSolarSysState;
    let mut oldStarDescPtr: *mut STAR_DESC = CurStarDescPtr;
    CurStarDescPtr = star;
    RandomContext_SeedRandom(SysGenRNG, GetRandomSeedForStar(star));
    memset(
        &raw mut SolarSysState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SOLARSYS_STATE>() as size_t,
    );
    SolarSysState.SunDesc[0usize].pPrevDesc = ::core::ptr::null_mut::<PLANET_DESC>();
    SolarSysState.SunDesc[0usize].rand_seed = RandomContext_Random(SysGenRNG);
    SolarSysState.SunDesc[0usize].data_index = (((*star).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE;
    SolarSysState.SunDesc[0usize].location.x = 0 as COORD;
    SolarSysState.SunDesc[0usize].location.y = 0 as COORD;
    SolarSysState.genFuncs = getGenerateFunctions((*star).Index);
    pSolarSysState = &raw mut SolarSysState;
    Some((*SolarSysState.genFuncs).generatePlanets.expect("non-null function pointer"))
        .expect("non-null function pointer")(&raw mut SolarSysState);
    if (*universeRecurseArg).systemFuncPre.is_some() {
        Some((*universeRecurseArg).systemFuncPre.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(star, &raw mut SolarSysState, (*universeRecurseArg).arg);
    }
    if (*universeRecurseArg).planetFuncPre.is_some()
        || (*universeRecurseArg).planetFuncPost.is_some()
        || (*universeRecurseArg).moonFunc.is_some()
    {
        forAllPlanets(
            star,
            &raw mut SolarSysState,
            Some(
                planetRecurse
                    as unsafe extern "C" fn(
                        *mut STAR_DESC,
                        *mut SOLARSYS_STATE,
                        *mut PLANET_DESC,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            universeRecurseArg as *mut ::core::ffi::c_void,
        );
    }
    if (*universeRecurseArg).systemFuncPost.is_some() {
        Some((*universeRecurseArg).systemFuncPost.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(star, &raw mut SolarSysState, (*universeRecurseArg).arg);
    }
    pSolarSysState = oldPSolarSysState;
    CurStarDescPtr = oldStarDescPtr;
}
unsafe extern "C" fn planetRecurse(
    mut star: *mut STAR_DESC,
    mut system: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut universeRecurseArg: *mut UniverseRecurseArg = arg as *mut UniverseRecurseArg;
    (CurStarDescPtr == star
        || {
            __assert_fail(
                b"CurStarDescPtr == star\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                683 as ::core::ffi::c_int,
                b"planetRecurse\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (pSolarSysState == system
        || {
            __assert_fail(
                b"pSolarSysState == system\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int,
                b"planetRecurse\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*planet).pPrevDesc = (&raw mut (*system).SunDesc as *mut PLANET_DESC)
        .offset(0isize);
    if (*universeRecurseArg).planetFuncPre.is_some() {
        (*system).pOrbitalDesc = planet;
        DoPlanetaryAnalysis(&raw mut (*system).SysInfo, planet);
        Some((*(*system).genFuncs).generateOrbital.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, planet);
        Some((*universeRecurseArg).planetFuncPre.expect("non-null function pointer"))
            .expect("non-null function pointer")(planet, (*universeRecurseArg).arg);
    }
    if (*universeRecurseArg).moonFunc.is_some() {
        RandomContext_SeedRandom(SysGenRNG, (*planet).rand_seed);
        Some((*(*system).genFuncs).generateMoons.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, planet);
        forAllMoons(
            star,
            system,
            planet,
            Some(
                moonRecurse
                    as unsafe extern "C" fn(
                        *mut STAR_DESC,
                        *mut SOLARSYS_STATE,
                        *mut PLANET_DESC,
                        *mut PLANET_DESC,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            universeRecurseArg as *mut ::core::ffi::c_void,
        );
    }
    if (*universeRecurseArg).planetFuncPost.is_some() {
        (*system).pOrbitalDesc = planet;
        DoPlanetaryAnalysis(&raw mut (*system).SysInfo, planet);
        Some((*(*system).genFuncs).generateOrbital.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, planet);
        Some((*universeRecurseArg).planetFuncPost.expect("non-null function pointer"))
            .expect("non-null function pointer")(planet, (*universeRecurseArg).arg);
    }
}
unsafe extern "C" fn moonRecurse(
    mut star: *mut STAR_DESC,
    mut system: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
    mut moon: *mut PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut universeRecurseArg: *mut UniverseRecurseArg = arg as *mut UniverseRecurseArg;
    (CurStarDescPtr == star
        || {
            __assert_fail(
                b"CurStarDescPtr == star\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int,
                b"moonRecurse\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (pSolarSysState == system
        || {
            __assert_fail(
                b"pSolarSysState == system\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int,
                b"moonRecurse\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*moon).pPrevDesc = planet;
    if (*universeRecurseArg).moonFunc.is_some() {
        (*system).pOrbitalDesc = moon;
        if (*moon).data_index as ::core::ffi::c_int
            != C2Rust_Unnamed_5::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
            && (*moon).data_index as ::core::ffi::c_int
                != C2Rust_Unnamed_5::SA_MATRA.0 as ::core::ffi::c_int
        {
            DoPlanetaryAnalysis(&raw mut (*system).SysInfo, moon);
        }
        Some((*(*system).genFuncs).generateOrbital.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, moon);
        Some((*universeRecurseArg).moonFunc.expect("non-null function pointer"))
            .expect("non-null function pointer")(moon, (*universeRecurseArg).arg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dumpUniverse(mut out: *mut FILE) {
    let mut dumpUniverseArg: DumpUniverseArg = DumpUniverseArg {
        out: ::core::ptr::null_mut::<FILE>(),
    };
    let mut universeRecurseArg: UniverseRecurseArg = UniverseRecurseArg {
        systemFuncPre: None,
        systemFuncPost: None,
        planetFuncPre: None,
        planetFuncPost: None,
        moonFunc: None,
        arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    dumpUniverseArg.out = out;
    universeRecurseArg.systemFuncPre = Some(
        dumpSystemCallback
            as unsafe extern "C" fn(
                *const STAR_DESC,
                *const SOLARSYS_STATE,
                *mut ::core::ffi::c_void,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const STAR_DESC,
                *const SOLARSYS_STATE,
                *mut ::core::ffi::c_void,
            ) -> (),
        >;
    universeRecurseArg.systemFuncPost = None;
    universeRecurseArg.planetFuncPre = Some(
        dumpPlanetCallback
            as unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
        >;
    universeRecurseArg.planetFuncPost = None;
    universeRecurseArg.moonFunc = Some(
        dumpMoonCallback
            as unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
        >;
    universeRecurseArg.arg = &raw mut dumpUniverseArg as *mut ::core::ffi::c_void;
    UniverseRecurse(&raw mut universeRecurseArg);
}
#[no_mangle]
pub unsafe extern "C" fn dumpUniverseToFile() {
    let mut out: *mut FILE = ::core::ptr::null_mut::<FILE>();
    out = fopen(
        b"PlanetInfo\0".as_ptr() as *const ::core::ffi::c_char,
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if out.is_null() {
        fprintf(
            stderr,
            b"Error: Could not open file '%s' for writing: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"PlanetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return;
    }
    dumpUniverse(out);
    fclose(out);
    fprintf(
        stdout,
        b"*** Star dump complete. The game may be in an undefined state.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn dumpSystemCallback(
    mut star: *const STAR_DESC,
    mut system: *const SOLARSYS_STATE,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut out: *mut FILE = (*(arg as *mut DumpUniverseArg)).out;
    dumpSystem(out, star, system);
}
#[no_mangle]
pub unsafe extern "C" fn dumpSystem(
    mut out: *mut FILE,
    mut star: *const STAR_DESC,
    mut system: *const SOLARSYS_STATE,
) {
    let mut name: [UNICODE; 256] = [0; 256];
    let mut buf: [UNICODE; 40] = [0; 40];
    GetClusterName(star, &raw mut name as *mut UNICODE);
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 40]>() as size_t,
        b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
        bodyColorString(
            (((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE,
        ),
        starTypeString(
            (((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE,
        ),
    );
    fprintf(
        out,
        b"%-22s  (%3d.%1d, %3d.%1d) %-19s  %s\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut name as *mut UNICODE,
        (*star).star_pt.x as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        (*star).star_pt.x as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
        (*star).star_pt.y as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        (*star).star_pt.y as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
        &raw mut buf as *mut UNICODE,
        starPresenceString((*star).Index),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bodyColorString(mut col: BYTE) -> *const ::core::ffi::c_char {
    match col as ::core::ffi::c_int {
        0 => return b"blue\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"green\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"orange\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"red\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"white\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"yellow\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"cyan\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"purple\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"violet\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn starTypeString(mut r#type: BYTE) -> *const ::core::ffi::c_char {
    match r#type as ::core::ffi::c_int {
        0 => return b"dwarf\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"giant\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"super giant\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn starPresenceString(
    mut index: BYTE,
) -> *const ::core::ffi::c_char {
    match index as ::core::ffi::c_int {
        0 => return b"\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"Sol\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"Shofixti male\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"Shofixti maidens\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"Starting colony\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"Spathi home\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"Zoq-Fot-Pik home\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"Melnorme trader #0\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"Melnorme trader #1\0".as_ptr() as *const ::core::ffi::c_char,
        9 => return b"Melnorme trader #2\0".as_ptr() as *const ::core::ffi::c_char,
        10 => return b"Melnorme trader #3\0".as_ptr() as *const ::core::ffi::c_char,
        11 => return b"Melnorme trader #4\0".as_ptr() as *const ::core::ffi::c_char,
        12 => return b"Melnorme trader #5\0".as_ptr() as *const ::core::ffi::c_char,
        13 => return b"Melnorme trader #6\0".as_ptr() as *const ::core::ffi::c_char,
        14 => return b"Melnorme trader #7\0".as_ptr() as *const ::core::ffi::c_char,
        15 => return b"Melnorme trader #8\0".as_ptr() as *const ::core::ffi::c_char,
        16 => return b"Talking Pet\0".as_ptr() as *const ::core::ffi::c_char,
        17 => return b"Chmmr home\0".as_ptr() as *const ::core::ffi::c_char,
        18 => return b"Syreen home\0".as_ptr() as *const ::core::ffi::c_char,
        19 => return b"Burvixese ruins\0".as_ptr() as *const ::core::ffi::c_char,
        20 => return b"Slylandro home\0".as_ptr() as *const ::core::ffi::c_char,
        21 => return b"Druuge home\0".as_ptr() as *const ::core::ffi::c_char,
        22 => return b"Precursor bomb\0".as_ptr() as *const ::core::ffi::c_char,
        23 => return b"Aqua Helix\0".as_ptr() as *const ::core::ffi::c_char,
        24 => return b"Sun Device\0".as_ptr() as *const ::core::ffi::c_char,
        25 => return b"Taalo Shield\0".as_ptr() as *const ::core::ffi::c_char,
        26 => return b"Syreen ship vault\0".as_ptr() as *const ::core::ffi::c_char,
        27 => return b"Ur-Quan ship wreck\0".as_ptr() as *const ::core::ffi::c_char,
        28 => return b"Zex' beauty\0".as_ptr() as *const ::core::ffi::c_char,
        29 => return b"Sa-Matra\0".as_ptr() as *const ::core::ffi::c_char,
        30 => return b"Zoq-Fot-Pik scout\0".as_ptr() as *const ::core::ffi::c_char,
        31 => return b"Mycon home\0".as_ptr() as *const ::core::ffi::c_char,
        32 => return b"Mycon egg shell #0\0".as_ptr() as *const ::core::ffi::c_char,
        33 => return b"Mycon egg shell #1\0".as_ptr() as *const ::core::ffi::c_char,
        34 => return b"Mycon egg shell #2\0".as_ptr() as *const ::core::ffi::c_char,
        35 => return b"Pkunk home\0".as_ptr() as *const ::core::ffi::c_char,
        36 => return b"Utwig home\0".as_ptr() as *const ::core::ffi::c_char,
        37 => return b"Supox home\0".as_ptr() as *const ::core::ffi::c_char,
        38 => return b"Yehat home\0".as_ptr() as *const ::core::ffi::c_char,
        39 => return b"Vux home\0".as_ptr() as *const ::core::ffi::c_char,
        40 => return b"Orz home\0".as_ptr() as *const ::core::ffi::c_char,
        41 => return b"Thraddash home\0".as_ptr() as *const ::core::ffi::c_char,
        42 => return b"Rainbow world\0".as_ptr() as *const ::core::ffi::c_char,
        43 => return b"Ilwrath home\0".as_ptr() as *const ::core::ffi::c_char,
        44 => return b"Androsynth ruins\0".as_ptr() as *const ::core::ffi::c_char,
        45 => return b"Mycon trap\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn dumpPlanetCallback(
    mut planet: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut out: *mut FILE = (*(arg as *mut DumpUniverseArg)).out;
    dumpPlanet(out, planet);
}
#[no_mangle]
pub unsafe extern "C" fn dumpPlanet(mut out: *mut FILE, mut planet: *const PLANET_DESC) {
    Some((*(*pSolarSysState).genFuncs).generateName.expect("non-null function pointer"))
        .expect("non-null function pointer")(pSolarSysState, planet);
    fprintf(
        out,
        b"- %-37s  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
        planetTypeString(
            (*planet).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed_5::PLANET_SHIELDED.0 as ::core::ffi::c_int),
        ),
    );
    dumpWorld(out, planet);
}
unsafe extern "C" fn dumpMoonCallback(
    mut moon: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut out: *mut FILE = (*(arg as *mut DumpUniverseArg)).out;
    dumpMoon(out, moon);
}
#[no_mangle]
pub unsafe extern "C" fn dumpMoon(mut out: *mut FILE, mut moon: *const PLANET_DESC) {
    let mut typeStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if (*moon).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_5::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        typeStr = b"StarBase\0".as_ptr() as *const ::core::ffi::c_char;
    } else if (*moon).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_5::SA_MATRA.0 as ::core::ffi::c_int
    {
        typeStr = b"Sa-Matra\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        typeStr = planetTypeString(
            (*moon).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed_5::PLANET_SHIELDED.0 as ::core::ffi::c_int),
        );
    }
    fprintf(
        out,
        b"  - Moon %-30c  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        'a' as isize
            + moon
                .offset_from(
                    (&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                        .offset(0isize),
                ),
        typeStr,
    );
    dumpWorld(out, moon);
}
unsafe extern "C" fn dumpWorld(mut out: *mut FILE, mut world: *const PLANET_DESC) {
    let mut info: *mut PLANET_INFO = ::core::ptr::null_mut::<PLANET_INFO>();
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_5::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_5::SA_MATRA.0 as ::core::ffi::c_int
    {
        return;
    }
    info = &raw mut (*pSolarSysState).SysInfo.PlanetInfo;
    fprintf(
        out,
        b"          AxialTilt:  %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).AxialTilt as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Tectonics:  %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).Tectonics as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Weather:    %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).Weather as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Density:    %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).PlanetDensity as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Radius:     %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).PlanetRadius as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Gravity:    %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).SurfaceGravity as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Temp:       %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).SurfaceTemperature as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Day:        %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).RotationPeriod as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          Atmosphere: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).AtmoDensity as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          LifeChance: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).LifeChance as ::core::ffi::c_int,
    );
    fprintf(
        out,
        b"          DistToSun:  %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).PlanetToSunDist as ::core::ffi::c_int,
    );
    if (*world).data_index as ::core::ffi::c_int
        & C2Rust_Unnamed_5::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    fprintf(
        out,
        b"          Bio: %4d    Min: %4d\n\0".as_ptr() as *const ::core::ffi::c_char,
        calculateBioValue(pSolarSysState, world) as ::core::ffi::c_int,
        calculateMineralValue(pSolarSysState, world) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn calculateBioValue(
    mut system: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> COUNT {
    let mut result: COUNT = 0;
    let mut numBio: COUNT = 0;
    let mut i: COUNT = 0;
    ((*system).pOrbitalDesc == world as *mut PLANET_DESC
        || {
            __assert_fail(
                b"system->pOrbitalDesc == world\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1065 as ::core::ffi::c_int,
                b"calculateBioValue\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numBio = callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    result = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numBio as ::core::ffi::c_int {
        let mut info: NODE_INFO = NODE_INFO {
            loc_pt: POINT { x: 0, y: 0 },
            density: 0,
            r#type: 0,
        };
        callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        );
        result = (result as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * ((*(&raw const CreatureData as *const LIFEFORM_DESC)
                    .offset(info.r#type as isize))
                    .ValueAndHitPoints as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as COUNT;
        i = i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn generateBioIndex(
    mut system: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut bio: *mut COUNT,
) {
    let mut numBio: COUNT = 0;
    let mut i: COUNT = 0;
    ((*system).pOrbitalDesc == world as *mut PLANET_DESC
        || {
            __assert_fail(
                b"system->pOrbitalDesc == world\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1088 as ::core::ffi::c_int,
                b"generateBioIndex\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numBio = callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 23 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
    {
        *bio.offset(i as isize) = 0 as COUNT;
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numBio as ::core::ffi::c_int {
        let mut info: NODE_INFO = NODE_INFO {
            loc_pt: POINT { x: 0, y: 0 },
            density: 0,
            r#type: 0,
        };
        callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        );
        *bio.offset(info.r#type as isize) = (*bio.offset(info.r#type as isize))
            .wrapping_add(1);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn calculateMineralValue(
    mut system: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> COUNT {
    let mut result: COUNT = 0;
    let mut numDeposits: COUNT = 0;
    let mut i: COUNT = 0;
    ((*system).pOrbitalDesc == world as *mut PLANET_DESC
        || {
            __assert_fail(
                b"system->pOrbitalDesc == world\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1111 as ::core::ffi::c_int,
                b"calculateMineralValue\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numDeposits = callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    result = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numDeposits as ::core::ffi::c_int {
        let mut info: NODE_INFO = NODE_INFO {
            loc_pt: POINT { x: 0, y: 0 },
            density: 0,
            r#type: 0,
        };
        callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        );
        result = (result as ::core::ffi::c_int
            + (info.density as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                * GlobData
                    .Game_state
                    .ElementWorth[(*Elements.offset(info.r#type as isize)
                    as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as COUNT;
        i = i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn generateMineralIndex(
    mut system: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut minerals: *mut COUNT,
) {
    let mut numDeposits: COUNT = 0;
    let mut i: COUNT = 0;
    ((*system).pOrbitalDesc == world as *mut PLANET_DESC
        || {
            __assert_fail(
                b"system->pOrbitalDesc == world\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1134 as ::core::ffi::c_int,
                b"generateMineralIndex\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numDeposits = callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    );
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_0::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        *minerals.offset(i as isize) = 0 as COUNT;
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numDeposits as ::core::ffi::c_int {
        let mut info: NODE_INFO = NODE_INFO {
            loc_pt: POINT { x: 0, y: 0 },
            density: 0,
            r#type: 0,
        };
        callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        );
        *minerals
            .offset(
                (*Elements.offset(info.r#type as isize) as ::core::ffi::c_int
                    & 0x7 as ::core::ffi::c_int) as isize,
            ) = (*minerals
            .offset(
                (*Elements.offset(info.r#type as isize) as ::core::ffi::c_int
                    & 0x7 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + (info.density as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as COUNT;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tallyResources(mut out: *mut FILE) {
    let mut tallyResourcesArg: TallyResourcesArg = TallyResourcesArg {
        out: ::core::ptr::null_mut::<FILE>(),
        mineralCount: 0,
        bioCount: 0,
    };
    let mut universeRecurseArg: UniverseRecurseArg = UniverseRecurseArg {
        systemFuncPre: None,
        systemFuncPost: None,
        planetFuncPre: None,
        planetFuncPost: None,
        moonFunc: None,
        arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    tallyResourcesArg.out = out;
    universeRecurseArg.systemFuncPre = Some(
        tallySystemPreCallback
            as unsafe extern "C" fn(
                *const STAR_DESC,
                *const SOLARSYS_STATE,
                *mut ::core::ffi::c_void,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const STAR_DESC,
                *const SOLARSYS_STATE,
                *mut ::core::ffi::c_void,
            ) -> (),
        >;
    universeRecurseArg.systemFuncPost = Some(
        tallySystemPostCallback
            as unsafe extern "C" fn(
                *const STAR_DESC,
                *const SOLARSYS_STATE,
                *mut ::core::ffi::c_void,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const STAR_DESC,
                *const SOLARSYS_STATE,
                *mut ::core::ffi::c_void,
            ) -> (),
        >;
    universeRecurseArg.planetFuncPre = Some(
        tallyPlanetCallback
            as unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
        >;
    universeRecurseArg.planetFuncPost = None;
    universeRecurseArg.moonFunc = Some(
        tallyMoonCallback
            as unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*const PLANET_DESC, *mut ::core::ffi::c_void) -> (),
        >;
    universeRecurseArg.arg = &raw mut tallyResourcesArg as *mut ::core::ffi::c_void;
    UniverseRecurse(&raw mut universeRecurseArg);
}
#[no_mangle]
pub unsafe extern "C" fn tallyResourcesToFile() {
    let mut out: *mut FILE = ::core::ptr::null_mut::<FILE>();
    out = fopen(
        b"ResourceTally\0".as_ptr() as *const ::core::ffi::c_char,
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if out.is_null() {
        fprintf(
            stderr,
            b"Error: Could not open file '%s' for writing: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"ResourceTally\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return;
    }
    tallyResources(out);
    fclose(out);
    fprintf(
        stdout,
        b"*** Resource tally complete. The game may be in an undefined state.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn tallySystemPreCallback(
    mut star: *const STAR_DESC,
    mut system: *const SOLARSYS_STATE,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut tallyResourcesArg: *mut TallyResourcesArg = arg as *mut TallyResourcesArg;
    (*tallyResourcesArg).mineralCount = 0 as COUNT;
    (*tallyResourcesArg).bioCount = 0 as COUNT;
}
unsafe extern "C" fn tallySystemPostCallback(
    mut star: *const STAR_DESC,
    mut system: *const SOLARSYS_STATE,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut name: [UNICODE; 256] = [0; 256];
    let mut tallyResourcesArg: *mut TallyResourcesArg = arg as *mut TallyResourcesArg;
    let mut out: *mut FILE = (*tallyResourcesArg).out;
    GetClusterName(star, &raw mut name as *mut UNICODE);
    fprintf(
        out,
        b"%s\t%d\t%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut name as *mut UNICODE,
        (*tallyResourcesArg).mineralCount as ::core::ffi::c_int,
        (*tallyResourcesArg).bioCount as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn tallyPlanetCallback(
    mut planet: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    tallyResourcesWorld(arg as *mut TallyResourcesArg, planet);
}
unsafe extern "C" fn tallyMoonCallback(
    mut moon: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    tallyResourcesWorld(arg as *mut TallyResourcesArg, moon);
}
unsafe extern "C" fn tallyResourcesWorld(
    mut arg: *mut TallyResourcesArg,
    mut world: *const PLANET_DESC,
) {
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_5::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_5::SA_MATRA.0 as ::core::ffi::c_int
    {
        return;
    }
    (*arg).bioCount = ((*arg).bioCount as ::core::ffi::c_int
        + calculateBioValue(pSolarSysState, world) as ::core::ffi::c_int) as COUNT;
    (*arg).mineralCount = ((*arg).mineralCount as ::core::ffi::c_int
        + calculateMineralValue(pSolarSysState, world) as ::core::ffi::c_int) as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn forAllPlanetTypes(
    mut callback: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const PlanetFrame,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_int = 0;
    extern "C" {
        static planet_array: [PlanetFrame; 0];
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_5::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int {
        callback
            .expect(
                "non-null function pointer",
            )(
            i,
            (&raw const planet_array as *const PlanetFrame).offset(i as isize),
            arg,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dumpPlanetTypes(mut out: *mut FILE) {
    let mut dumpPlanetTypesArg: DumpPlanetTypesArg = DumpPlanetTypesArg {
        out: ::core::ptr::null_mut::<FILE>(),
    };
    dumpPlanetTypesArg.out = out;
    forAllPlanetTypes(
        Some(
            dumpPlanetTypeCallback
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const PlanetFrame,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut dumpPlanetTypesArg as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn dumpPlanetTypeCallback(
    mut index: ::core::ffi::c_int,
    mut planetType: *const PlanetFrame,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut dumpPlanetTypesArg: *mut DumpPlanetTypesArg = arg as *mut DumpPlanetTypesArg;
    dumpPlanetType((*dumpPlanetTypesArg).out, index, planetType);
}
#[no_mangle]
pub unsafe extern "C" fn dumpPlanetType(
    mut out: *mut FILE,
    mut index: ::core::ffi::c_int,
    mut planetType: *const PlanetFrame,
) {
    let mut i: ::core::ffi::c_int = 0;
    fprintf(
        out,
        b"%s\n\tType: %s\n\tColor: %s\n\tSurface generation algoritm: %s\n\tTectonics: %s\n\tAtmosphere: %s\n\tDensity: %s\n\tElements:\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        planetTypeString(index),
        worldSizeString(
            ((*planetType).Type as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                as BYTE,
        ),
        bodyColorString(
            ((*planetType).Type as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
        ),
        worldGenAlgoString(
            ((*planetType).Type as ::core::ffi::c_int & 0xc as ::core::ffi::c_int)
                as BYTE,
        ),
        tectonicsString((*planetType).BaseTectonics),
        atmosphereString(
            ((*planetType).AtmoAndDensity as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
        ),
        densityString(
            ((*planetType).AtmoAndDensity as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
        ),
    );
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut entry: *const ELEMENT_ENTRY = ::core::ptr::null::<ELEMENT_ENTRY>();
        entry = (&raw const (*planetType).UsefulElements as *const ELEMENT_ENTRY)
            .offset(i as isize);
        if (*entry).Density as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            fprintf(
                out,
                b"\t\t0 to %d %s-quality (+%d) deposits of %s (%s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ((*entry).Density as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int,
                depositQualityString(
                    ((*entry).Density as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
                ),
                ((*entry).Density as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int * 5 as ::core::ffi::c_int,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_18::ELEMENTS_STRING_BASE.0 as ::core::ffi::c_int
                            + (*entry).ElementType as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_18::CARGO_STRING_BASE.0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + (*Elements.offset((*entry).ElementType as isize)
                                as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)) as COUNT,
                    ),
                ),
            );
        }
        i += 1;
    }
    fprintf(out, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn planetTypeString(
    mut typeIndex: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    static mut typeStr: [UNICODE; 40] = [0; 40];
    if typeIndex >= C2Rust_Unnamed_5::FIRST_GAS_GIANT.0 as ::core::ffi::c_int {
        snprintf(
            &raw mut typeStr as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>() as size_t,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_18::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + 51 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        snprintf(
            &raw mut typeStr as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>() as size_t,
            b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_18::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + typeIndex) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_18::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    }
    return &raw mut typeStr as *mut UNICODE;
}
#[no_mangle]
pub unsafe extern "C" fn worldSizeString(mut size: BYTE) -> *const ::core::ffi::c_char {
    match size as ::core::ffi::c_int {
        0 => return b"small rocky world\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"large rocky world\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"gas giant\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn worldGenAlgoString(
    mut algo: BYTE,
) -> *const ::core::ffi::c_char {
    match algo as ::core::ffi::c_int {
        0 => return b"TOPO_ALGO\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"CRATERED_ALGO\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"GAS_GIANT_ALGO\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn tectonicsString(
    mut tectonics: BYTE,
) -> *const ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 5] = [0; 5];
    match tectonics as ::core::ffi::c_int {
        0 => return b"none\0".as_ptr() as *const ::core::ffi::c_char,
        40 => return b"low\0".as_ptr() as *const ::core::ffi::c_char,
        80 => return b"medium\0".as_ptr() as *const ::core::ffi::c_char,
        140 => return b"high\0".as_ptr() as *const ::core::ffi::c_char,
        200 => return b"super\0".as_ptr() as *const ::core::ffi::c_char,
        _ => {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                tectonics as ::core::ffi::c_int,
            );
            return &raw mut buf as *mut ::core::ffi::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn atmosphereString(
    mut atmosphere: BYTE,
) -> *const ::core::ffi::c_char {
    match atmosphere as ::core::ffi::c_int {
        0 => return b"thin\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"normal\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"thick\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"super thick\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn densityString(mut density: BYTE) -> *const ::core::ffi::c_char {
    match density as ::core::ffi::c_int {
        0 => return b"gaseous\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"light\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"low\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"normal\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"high\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"super high\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn depositQualityString(
    mut quality: BYTE,
) -> *const ::core::ffi::c_char {
    match quality as ::core::ffi::c_int {
        0 => return b"low\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"medium\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"high\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn findPlayerShip(mut playerNr: SIZE) -> *mut STARSHIP {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).succ;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && (*ElementPtr).playerNr as ::core::ffi::c_int
                == playerNr as ::core::ffi::c_int
        {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            UnlockLink(&raw mut disp_q, hElement);
            return StarShipPtr;
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
    return ::core::ptr::null_mut::<STARSHIP>();
}
#[no_mangle]
pub unsafe extern "C" fn resetCrewBattle() {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut delta: COUNT = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_22::IN_BATTLE.0 as ::core::ffi::c_int == 0 || inHQSpace().0 != 0
    {
        return;
    }
    StarShipPtr = findPlayerShip(0 as SIZE);
    if StarShipPtr.is_null() || (*StarShipPtr).RaceDescPtr.is_null() {
        return;
    }
    delta = ((*(*StarShipPtr).RaceDescPtr).ship_info.max_crew as ::core::ffi::c_int
        - (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int)
        as COUNT;
    OldContext = SetContext(StatusContext);
    DeltaCrew((*StarShipPtr).hShip as *mut ELEMENT, delta as SIZE);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn resetEnergyBattle() {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut delta: COUNT = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_22::IN_BATTLE.0 as ::core::ffi::c_int == 0 || inHQSpace().0 != 0
    {
        return;
    }
    StarShipPtr = findPlayerShip(0 as SIZE);
    if StarShipPtr.is_null() || (*StarShipPtr).RaceDescPtr.is_null() {
        return;
    }
    delta = ((*(*StarShipPtr).RaceDescPtr).ship_info.max_energy as ::core::ffi::c_int
        - (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int)
        as COUNT;
    OldContext = SetContext(StatusContext);
    DeltaEnergy((*StarShipPtr).hShip as *mut ELEMENT, delta as SIZE);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn dumpStrings(mut out: *mut FILE) {
    let mut categories: [C2Rust_Unnamed_15; 24] = [
        C2Rust_Unnamed_15 {
            name: b"0\0".as_ptr() as *const ::core::ffi::c_char,
            base: 0 as size_t,
            count: 0 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"STAR_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::STAR_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 133 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"DEVICE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::DEVICE_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 29 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"CARGO_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::CARGO_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 10 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"ELEMENTS_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::ELEMENTS_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 133 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"SCAN_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::SCAN_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 56 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"STAR_NUMBER_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::STAR_NUMBER_BASE.0 as ::core::ffi::c_int as size_t,
            count: 14 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"PLANET_NUMBER_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 33 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"MONTHS_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::MONTHS_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 12 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"FEEDBACK_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 2 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"STARBASE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 5 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"ENCOUNTER_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::ENCOUNTER_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 8 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"NAVIGATION_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 6 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"NAMING_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 4 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"MELEE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::MELEE_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 9 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"SAVEGAME_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 5 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"OPTION_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 5 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"QUITMENU_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 3 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"STATUS_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 6 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"FLAGSHIP_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 13 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"ORBITSCAN_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 19 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"MAINMENU_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 55 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"NETMELEE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 19 as size_t,
        },
        C2Rust_Unnamed_15 {
            name: b"GAMESTR_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_18::GAMESTR_COUNT.0 as ::core::ffi::c_int as size_t,
            count: -1 as ::core::ffi::c_int as size_t,
        },
    ];
    let mut numCategories: size_t = (::core::mem::size_of::<[C2Rust_Unnamed_15; 24]>()
        as size_t)
        .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_15>() as size_t);
    let mut numStrings: size_t = GetStringTableCount(GameStrings) as size_t;
    let mut stringI: size_t = 0;
    let mut categoryI: size_t = 0 as size_t;
    categoryI = 0 as size_t;
    while categoryI < numCategories.wrapping_sub(1 as size_t) {
        if categories[categoryI as usize]
            .base
            .wrapping_add(categories[categoryI as usize].count)
            != categories[categoryI.wrapping_add(1 as size_t) as usize].base
        {
            fprintf(
                stderr,
                b"Error: String category list in dumpStrings() is not up to date.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        categoryI = categoryI.wrapping_add(1);
    }
    if C2Rust_Unnamed_18::GAMESTR_COUNT.0 as ::core::ffi::c_int as size_t != numStrings {
        fprintf(
            stderr,
            b"Warning: GAMESTR_COUNT is %d, but GameStrings contains %d strings.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            C2Rust_Unnamed_18::GAMESTR_COUNT.0 as ::core::ffi::c_int,
            numStrings,
        );
    }
    categoryI = 0 as size_t;
    stringI = 0 as size_t;
    while stringI < numStrings {
        while categoryI < numCategories
            && stringI >= categories[categoryI.wrapping_add(1 as size_t) as usize].base
        {
            categoryI = categoryI.wrapping_add(1);
        }
        fprintf(
            out,
            b"[ %s + %d ]  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            categories[categoryI as usize].name,
            stringI.wrapping_sub(categories[categoryI as usize].base),
            GetStringAddress(SetAbsStringTableIndex(GameStrings, stringI as COUNT)),
        );
        stringI = stringI.wrapping_add(1);
    }
}
unsafe extern "C" fn hsvaToRgba(
    mut hue: ::core::ffi::c_double,
    mut sat: ::core::ffi::c_double,
    mut val: ::core::ffi::c_double,
    mut alpha: BYTE,
) -> Color {
    let mut hi: ::core::ffi::c_uint = (hue / 60.0f64) as ::core::ffi::c_int
        as ::core::ffi::c_uint;
    let mut f: ::core::ffi::c_double = hue / 60.0f64
        - (hue / 60.0f64) as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut p: ::core::ffi::c_double = val * (1.0f64 - sat);
    let mut q: ::core::ffi::c_double = val * (1.0f64 - f * sat);
    let mut t: ::core::ffi::c_double = val * (1.0f64 - (1.0f64 - f * sat));
    let mut pb: BYTE = (p * 255.0f64 + 0.5f64) as BYTE;
    let mut qb: BYTE = (q * 255.0f64 + 0.5f64) as BYTE;
    let mut tb: BYTE = (t * 255.0f64 + 0.5f64) as BYTE;
    let mut vb: BYTE = (val * 255.0f64 + 0.5f64) as BYTE;
    (hue >= 0.0f64 && hue < 360.0f64
        || {
            __assert_fail(
                b"hue >= 0.0 && hue < 360.0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1642 as ::core::ffi::c_int,
                b"hsvaToRgba\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (sat >= 0 as ::core::ffi::c_int as ::core::ffi::c_double && sat <= 1.0f64
        || {
            __assert_fail(
                b"sat >= 0 && sat <= 1.0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1643 as ::core::ffi::c_int,
                b"hsvaToRgba\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (val >= 0 as ::core::ffi::c_int as ::core::ffi::c_double && val <= 1.0f64
        || {
            __assert_fail(
                b"val >= 0 && val <= 1.0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1644 as ::core::ffi::c_int,
                b"hsvaToRgba\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (hi < 6 as ::core::ffi::c_uint
        || {
            __assert_fail(
                b"hi < 6\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1648 as ::core::ffi::c_int,
                b"hsvaToRgba\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    match hi {
        0 => return buildColorRgba(vb, tb, pb, alpha),
        1 => return buildColorRgba(qb, vb, pb, alpha),
        2 => return buildColorRgba(pb, vb, tb, alpha),
        3 => return buildColorRgba(pb, qb, vb, alpha),
        4 => return buildColorRgba(tb, pb, vb, alpha),
        5 => return buildColorRgba(vb, pb, qb, alpha),
        _ => {}
    }
    return buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, alpha);
}
unsafe extern "C" fn isContextVisible(mut context: CONTEXT) -> bool {
    let mut contextFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut oldContext: CONTEXT = SetContext(context);
    contextFrame = GetContextFGFrame();
    SetContext(oldContext);
    return contextFrame == Screen;
}
unsafe extern "C" fn countVisibleContexts() -> size_t {
    let mut contextCount: size_t = 0;
    let mut context: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    contextCount = 0 as size_t;
    context = GetFirstContext();
    while !context.is_null() {
        if isContextVisible(context) {
            contextCount = contextCount.wrapping_add(1);
        }
        context = GetNextContext(context);
    }
    return contextCount;
}
unsafe extern "C" fn drawContext(mut context: CONTEXT, mut hue: ::core::ffi::c_double) {
    let mut drawFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut oldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldFgCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut rectCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut lineCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut textCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut haveClippingRect: bool = false;
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut line: LINE = LINE {
        first: POINT { x: 0, y: 0 },
        second: POINT { x: 0, y: 0 },
    };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut p1: POINT = POINT { x: 0, y: 0 };
    let mut p2: POINT = POINT { x: 0, y: 0 };
    let mut p3: POINT = POINT { x: 0, y: 0 };
    let mut p4: POINT = POINT { x: 0, y: 0 };
    drawFrame = GetContextFGFrame();
    rectCol = hsvaToRgba(hue, 1.0f64, 0.5f64, 100 as BYTE);
    lineCol = hsvaToRgba(hue, 1.0f64, 1.0f64, 90 as BYTE);
    textCol = lineCol;
    oldContext = SetContext(context);
    haveClippingRect = GetContextClipRect(&raw mut rect).0 != 0;
    SetContext(oldContext);
    p1 = rect.corner;
    p2.x = (rect.corner.x as ::core::ffi::c_int + rect.extent.width as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    p2.y = rect.corner.y;
    p3.x = rect.corner.x;
    p3.y = (rect.corner.y as ::core::ffi::c_int
        + rect.extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    p4.x = (rect.corner.x as ::core::ffi::c_int + rect.extent.width as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    p4.y = (rect.corner.y as ::core::ffi::c_int
        + rect.extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    oldFgCol = SetContextForeGroundColor(rectCol);
    DrawFilledRectangle(&raw mut rect);
    SetContextForeGroundColor(lineCol);
    line.first = p1;
    line.second = p2;
    DrawLine(&raw mut line);
    line.first = p2;
    line.second = p4;
    DrawLine(&raw mut line);
    line.first = p1;
    line.second = p3;
    DrawLine(&raw mut line);
    line.first = p3;
    line.second = p4;
    DrawLine(&raw mut line);
    line.first = p1;
    line.second = p4;
    DrawLine(&raw mut line);
    line.first = p2;
    line.second = p3;
    DrawLine(&raw mut line);
    oldFont = SetContextFont(TinyFont);
    SetContextForeGroundColor(textCol);
    oldMode = SetContextDrawMode(
        MAKE_DRAW_MODE(DrawKind::DRAW_ALPHA, textCol.a as SWORD),
    );
    text.baseline.x = ((p1.x as ::core::ffi::c_int
        + (p2.x as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
        / 2 as ::core::ffi::c_int) as COORD;
    text.baseline.y = (p1.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as COORD;
    text.pStr = GetContextName(context) as *const UNICODE;
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut text);
    SetContextDrawMode(oldMode);
    SetContextForeGroundColor(oldFgCol);
    SetContextFont(oldFont);
}
unsafe extern "C" fn describeContext(mut out: *mut FILE, context: CONTEXT) {
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldContext: CONTEXT = SetContext(context);
    GetContextClipRect(&raw mut rect);
    fprintf(
        out,
        b"Context '%s':\n\tClipRect = (%d, %d)-(%d, %d)  (%d x %d)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        GetContextName(context),
        rect.corner.x as ::core::ffi::c_int,
        rect.corner.y as ::core::ffi::c_int,
        rect.corner.x as ::core::ffi::c_int + rect.extent.width as ::core::ffi::c_int,
        rect.corner.y as ::core::ffi::c_int + rect.extent.height as ::core::ffi::c_int,
        rect.extent.width as ::core::ffi::c_int,
        rect.extent.height as ::core::ffi::c_int,
    );
    SetContext(oldContext);
}
unsafe extern "C" fn waitForKey(mut self_0: *mut wait_state) -> BOOLEAN {
    if PulsedInputState
        .menu[C2Rust_Unnamed_16::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_16::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0
    {
        return BOOLEAN::FALSE;
    }
    SleepThread((840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn getScreen() -> FRAME {
    let mut oldContext: CONTEXT = SetContext(ScreenContext);
    let mut savedFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut screenRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    screenRect.corner.x = 0 as COORD;
    screenRect.corner.y = 0 as COORD;
    screenRect.extent.width = ScreenWidth as COORD;
    screenRect.extent.height = ScreenHeight as COORD;
    savedFrame = CaptureDrawable(
        LoadDisplayPixmap(&raw mut screenRect, ::core::ptr::null_mut::<FRAME_DESC>()),
    );
    SetContext(oldContext);
    return savedFrame;
}
unsafe extern "C" fn putScreen(mut savedFrame: FRAME) {
    let mut stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut oldContext: CONTEXT = SetContext(ScreenContext);
    stamp.origin.x = 0 as COORD;
    stamp.origin.y = 0 as COORD;
    stamp.frame = savedFrame;
    DrawStamp(&raw mut stamp);
    SetContext(oldContext);
}
#[no_mangle]
pub unsafe extern "C" fn debugContexts() {
    static mut inDebugContexts: bool = false;
    let mut orgContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut debugDrawContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut debugDrawFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut hueIncrement: ::core::ffi::c_double = 0.;
    let mut visibleContextI: size_t = 0;
    let mut context: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut contextCount: size_t = 0;
    let mut savedScreen: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    if ::core::ptr::read_volatile::<bool>(&raw const inDebugContexts) {
        return;
    }
    ::core::ptr::write_volatile(&raw mut inDebugContexts, true);
    contextCount = countVisibleContexts();
    if contextCount != 0 as size_t {
        savedScreen = getScreen();
        FlushGraphics();
        debugDrawContext = CreateContextAux(
            b"debugDrawContext\0".as_ptr() as *const ::core::ffi::c_char,
        );
        debugDrawFrame = CaptureDrawable(CloneFrame(savedScreen));
        orgContext = SetContext(debugDrawContext);
        SetContextFGFrame(debugDrawFrame);
        hueIncrement = 360.0f64 / contextCount as ::core::ffi::c_double;
        visibleContextI = 0 as size_t;
        context = GetFirstContext();
        while !context.is_null() {
            if context != debugDrawContext {
                if isContextVisible(context) {
                    drawContext(
                        context,
                        visibleContextI as ::core::ffi::c_double * hueIncrement,
                    );
                    visibleContextI = visibleContextI.wrapping_add(1);
                }
                describeContext(stderr, context);
            }
            context = GetNextContext(context);
        }
        putScreen(debugDrawFrame);
        let mut state: WAIT_STATE = WAIT_STATE { InputFunc: None };
        state.InputFunc = Some(
            waitForKey as unsafe extern "C" fn(*mut wait_state) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut wait_state) -> BOOLEAN>;
        DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
        SetContext(orgContext);
        DestroyContext(debugDrawContext);
        DestroyDrawable(ReleaseDrawable(debugDrawFrame));
        putScreen(savedScreen);
        DestroyDrawable(ReleaseDrawable(savedScreen));
    }
    ::core::ptr::write_volatile(&raw mut inDebugContexts, false);
}
