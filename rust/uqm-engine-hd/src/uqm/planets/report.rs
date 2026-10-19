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

// Rust translation of src/uqm/planets/report.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCountN(
        start: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> size_t;
    fn UniChar_isGraph(ch: UniChar) -> ::core::ffi::c_int;
    fn GetTimeCounter() -> TimeCount;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn WaitForActButton(
        newButton: BOOLEAN,
        duration: TimePeriod,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    fn WaitForActButtonUntil(
        newButton: BOOLEAN,
        timeOut: TimeCount,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    fn WaitForNoInput(duration: TimePeriod, resetInput: BOOLEAN) -> BOOLEAN;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut SpaceJunkFrame: FRAME;
    fn GetScanContext(owner: *mut BOOLEAN) -> CONTEXT;
    fn DestroyScanContext();
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut GameStrings: STRING;
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn luaUqm_comm_stringNeedsInterpolate(str: *const ::core::ffi::c_char) -> BOOLEAN;
    fn luaUqm_comm_stringInterpolate(
        str: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static mut disableInteractivity: BOOLEAN;
    fn PlaySound(
        S: SOUND,
        Pos: SoundPosition,
        PositionalObject: *mut ELEMENT,
        Priority: BYTE,
    );
    fn NotPositional() -> SoundPosition;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type SOUND = STRING;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SoundPosition = soundposition;
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
    pub const KEY_MENU_TOGGLEMAP: Self = Self(23);
    pub const KEY_SCREENSHOT: Self = Self(24);
    pub const KEY_QUICKSAVE: Self = Self(25);
    pub const KEY_QUICKLOAD: Self = Self(26);
    pub const KEY_DEBUG_2: Self = Self(27);
    pub const KEY_DEBUG_3: Self = Self(28);
    pub const KEY_DEBUG_4: Self = Self(29);
    pub const KEY_MENU_ANY: Self = Self(30);
    pub const NUM_MENU_KEYS: Self = Self(31);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
pub type HELEMENT = HLINK;
pub type ELEMENT_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
pub type STATE = state;
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
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    pub c2rust_unnamed_1: C2Rust_Unnamed_3,
    pub c2rust_unnamed_2: C2Rust_Unnamed_2,
    pub c2rust_unnamed_3: C2Rust_Unnamed_1,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
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
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
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
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
unsafe extern "C" fn ClearReportArea(mut startx: COORD) {
    let mut x: COUNT = 0;
    let mut y: COUNT = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if optWhichFonts == 0x2 as ::core::ffi::c_int {
        s.frame = SetAbsFrameIndex(SpaceJunkFrame, 21 as COUNT);
    } else {
        s.frame = SetAbsFrameIndex(SpaceJunkFrame, 18 as COUNT);
    }
    GetFrameRect(s.frame, &raw mut r);
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(ClearDrawable(), ());
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x7 as BYTE, 0 as BYTE));
    s.origin.y = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    y = 0 as COUNT;
    while (y as ::core::ffi::c_int) < 67 as ::core::ffi::c_int / 6 as ::core::ffi::c_int
    {
        s.origin.x = startx;
        x = 0 as COUNT;
        while (x as ::core::ffi::c_int)
            < 243 as ::core::ffi::c_int
                / if isPC(optSuperPC).0 != 0
                    || if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                {
                    7 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                }
        {
            if optWhichFonts == 0x2 as ::core::ffi::c_int {
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
            } else {
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
            }
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            x = x.wrapping_add(1);
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            + (r.extent.height as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        y = y.wrapping_add(1);
    }
    UnbatchGraphics();
}
unsafe extern "C" fn MakeReport(
    mut ReadOutSounds: SOUND,
    mut pStr: *mut UNICODE,
    mut StrLen: COUNT,
) {
    let mut end_page_len: ::core::ffi::c_int = 0;
    let mut end_page_buf: [UNICODE; 200] = [0; 200];
    let mut last_c: UniChar = 0 as UniChar;
    let mut row_cells: COUNT = 0;
    let mut Sleepy: BOOLEAN = BOOLEAN::FALSE;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut contextRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut fgcolor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut startx: COORD = 0;
    let mut total_lines: SIZE = -1 as SIZE;
    let mut curr_line: SIZE = -1 as SIZE;
    let mut first_line_length: COUNT = 0 as COUNT;
    sprintf(
        &raw mut end_page_buf as *mut ::core::ffi::c_char,
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_7::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    end_page_len = utf8StringCount(&raw mut end_page_buf as *mut UNICODE)
        as ::core::ffi::c_int;
    GetFrameRect(SetAbsFrameIndex(SpaceJunkFrame, 18 as COUNT), &raw mut r);
    GetContextClipRect(&raw mut contextRect);
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = 1 as COUNT;
    t.pStr = pStr;
    Sleepy = BOOLEAN::TRUE;
    FlushInput();
    t.baseline.y = (r.extent.height as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    row_cells = 0 as COUNT;
    fgcolor = colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE);
    if (contextRect.extent.width as ::core::ffi::c_int)
        < if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor
    {
        startx = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    } else {
        startx = (((r.extent.width as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            + if isPC(optSuperPC).0 != 0 {
                (1 as ::core::ffi::c_int) << resolutionFactor
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
    }
    startx = (startx as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    if StrLen != 0 {
        uqm_lua_checked!(ClearReportArea(startx), ());
        SetContextForeGroundColor(fgcolor);
        if is3DO(optSuperPC).0 != 0 {
            let mut pCurrStr: *const UNICODE = ::core::ptr::null::<UNICODE>();
            let mut length: COUNT = 0;
            pCurrStr = t.pStr;
            total_lines = 0 as SIZE;
            curr_line = 0 as SIZE;
            length = StrLen;
            while length as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if getCharFromString(&raw mut pCurrStr) == '\n' as UniChar {
                    total_lines += 1;
                }
                if total_lines as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    first_line_length = first_line_length.wrapping_add(1);
                }
                length = length.wrapping_sub(1);
            }
        }
    }
    while StrLen != 0 {
        let mut col_cells: COUNT = 0;
        let mut pLastStr: *const UNICODE = ::core::ptr::null::<UNICODE>();
        let mut pNextStr: *const UNICODE = ::core::ptr::null::<UNICODE>();
        let mut lf_pos: COUNT = 0;
        pLastStr = t.pStr;
        lf_pos = StrLen;
        pNextStr = t.pStr;
        while lf_pos as ::core::ffi::c_int != 0
            && getCharFromString(&raw mut pNextStr) != '\n' as UniChar
        {
            lf_pos = lf_pos.wrapping_sub(1);
        }
        col_cells = 0 as COUNT;
        if row_cells as ::core::ffi::c_int
            == 67 as ::core::ffi::c_int / 6 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
            && (StrLen as ::core::ffi::c_int
                > 243 as ::core::ffi::c_int
                    / if isPC(optSuperPC).0 != 0
                        || if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        7 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    } || lf_pos as ::core::ffi::c_int > 1 as ::core::ffi::c_int)
        {
            col_cells = ((243 as ::core::ffi::c_int
                / if isPC(optSuperPC).0 != 0
                    || if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                {
                    7 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                } >> 1 as ::core::ffi::c_int)
                - (end_page_len >> 1 as ::core::ffi::c_int)) as COUNT;
            t.pStr = &raw mut end_page_buf as *mut UNICODE;
            StrLen = (StrLen as ::core::ffi::c_int + end_page_len) as COUNT;
            if curr_line as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                curr_line -= 1;
            }
        }
        if total_lines as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            if curr_line as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                col_cells = (243 as ::core::ffi::c_int
                    / if isPC(optSuperPC).0 != 0
                        || if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        7 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    } - first_line_length as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as COUNT;
            }
            if curr_line as ::core::ffi::c_int == total_lines as ::core::ffi::c_int {
                col_cells = ((243 as ::core::ffi::c_int
                    / if isPC(optSuperPC).0 != 0
                        || if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        7 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    } - (StrLen as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int) as COUNT;
            }
        }
        t.baseline.x = (startx as ::core::ffi::c_int
            + col_cells as ::core::ffi::c_int
                * (r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        loop {
            let mut word_chars: COUNT = 0;
            let mut pStr_0: *const UNICODE = ::core::ptr::null::<UNICODE>();
            let mut c: UniChar = 0;
            pStr_0 = t.pStr;
            pNextStr = t.pStr;
            while UniChar_isGraph(getCharFromString(&raw mut pNextStr)) != 0 {
                pStr_0 = pNextStr;
            }
            word_chars = utf8StringCountN(t.pStr, pStr_0) as COUNT;
            col_cells = (col_cells as ::core::ffi::c_int
                + word_chars as ::core::ffi::c_int) as COUNT;
            if col_cells as ::core::ffi::c_int
                <= 243 as ::core::ffi::c_int
                    / if isPC(optSuperPC).0 != 0
                        || if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        7 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    }
            {
                let mut TimeOut: TimeCount = 0;
                StrLen = (StrLen as ::core::ffi::c_int
                    - word_chars as ::core::ffi::c_int) as COUNT;
                if StrLen != 0 {
                    StrLen = StrLen.wrapping_sub(1);
                }
                TimeOut = GetTimeCounter();
                loop {
                    let c2rust_fresh0 = word_chars;
                    word_chars = word_chars.wrapping_sub(1);
                    if c2rust_fresh0 == 0 {
                        break;
                    }
                    pNextStr = t.pStr;
                    c = getCharFromString(&raw mut pNextStr);
                    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                    {
                        return;
                    }
                    if Sleepy.0 == 0 {
                        uqm_lua_checked!(font_DrawText(&raw mut t), ());
                    } else {
                        let mut scale: BYTE = 0 as BYTE;
                        uqm_lua_checked!(font_DrawText(&raw mut t), ());
                        if CurrentInputState
                            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                            as usize] != 0
                        {
                            scale = 2 as BYTE;
                        }
                        uqm_lua_checked!(PlaySound(
                            ReadOutSounds,
                            NotPositional(),
                            ::core::ptr::null_mut::<ELEMENT>(),
                            2 as BYTE,
                        ), ());
                        match c {
                            44 => {
                                TimeOut = TimeOut
                                    .wrapping_add(
                                        (840 as ::core::ffi::c_int
                                            / ((4 as ::core::ffi::c_int)
                                                << scale as ::core::ffi::c_int)) as TimeCount,
                                    );
                            }
                            46 => {
                                TimeOut = TimeOut
                                    .wrapping_add(
                                        (840 as ::core::ffi::c_int
                                            / ((2 as ::core::ffi::c_int)
                                                << scale as ::core::ffi::c_int)) as TimeCount,
                                    );
                            }
                            33 => {
                                if last_c != '!' as UniChar && last_c != ' ' as UniChar {
                                    TimeOut = TimeOut
                                        .wrapping_add(
                                            (840 as ::core::ffi::c_int
                                                / ((2 as ::core::ffi::c_int)
                                                    << scale as ::core::ffi::c_int)) as TimeCount,
                                        );
                                } else {
                                    TimeOut = TimeOut
                                        .wrapping_add(
                                            (840 as ::core::ffi::c_int
                                                / ((20 as ::core::ffi::c_int)
                                                    << scale as ::core::ffi::c_int)) as TimeCount,
                                        );
                                }
                            }
                            63 => {
                                if last_c != '?' as UniChar && last_c != ' ' as UniChar {
                                    TimeOut = TimeOut
                                        .wrapping_add(
                                            (840 as ::core::ffi::c_int
                                                / ((2 as ::core::ffi::c_int)
                                                    << scale as ::core::ffi::c_int)) as TimeCount,
                                        );
                                } else {
                                    TimeOut = TimeOut
                                        .wrapping_add(
                                            (840 as ::core::ffi::c_int
                                                / ((20 as ::core::ffi::c_int)
                                                    << scale as ::core::ffi::c_int)) as TimeCount,
                                        );
                                }
                            }
                            _ => {
                                TimeOut = TimeOut
                                    .wrapping_add(
                                        (840 as ::core::ffi::c_int
                                            / ((20 as ::core::ffi::c_int)
                                                << scale as ::core::ffi::c_int)) as TimeCount,
                                    );
                            }
                        }
                        last_c = c;
                        if word_chars as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            TimeOut = TimeOut
                                .wrapping_add(
                                    (840 as ::core::ffi::c_int
                                        / ((20 as ::core::ffi::c_int)
                                            << scale as ::core::ffi::c_int)) as TimeCount,
                                );
                        }
                        if uqm_lua_checked!(WaitForActButtonUntil(BOOLEAN::TRUE, TimeOut, BOOLEAN::FALSE), ())
                            .0 != 0
                        {
                            Sleepy = BOOLEAN::FALSE;
                            BatchGraphics();
                        }
                    }
                    t.pStr = pNextStr;
                    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                        + (r.extent.width as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                }
                col_cells = col_cells.wrapping_add(1);
                last_c = getCharFromString(&raw mut t.pStr);
                t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                    + (r.extent.width as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                if curr_line as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                    && last_c == '\n' as UniChar
                {
                    curr_line += 1;
                }
            }
            if !(col_cells as ::core::ffi::c_int
                <= 243 as ::core::ffi::c_int
                    / if isPC(optSuperPC).0 != 0
                        || if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        7 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    } && last_c != '\n' as UniChar && StrLen as ::core::ffi::c_int != 0)
            {
                break;
            }
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + (r.extent.height as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        row_cells = row_cells.wrapping_add(1);
        if !(row_cells as ::core::ffi::c_int
            == 67 as ::core::ffi::c_int / 6 as ::core::ffi::c_int
            || StrLen as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            continue;
        }
        t.pStr = pLastStr;
        if Sleepy.0 == 0 {
            UnbatchGraphics();
        }
        if uqm_lua_checked!(WaitForActButton(
                BOOLEAN::TRUE,
                -1 as ::core::ffi::c_int as TimePeriod,
                BOOLEAN::FALSE,
            ), ())
            .0 == 0
        {
            break;
        }
        t.baseline.y = (r.extent.height as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        row_cells = 0 as COUNT;
        if StrLen != 0 {
            if Sleepy.0 == 0 {
                BatchGraphics();
            }
            uqm_lua_checked!(ClearReportArea(startx), ());
            SetContextForeGroundColor(fgcolor);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoDiscoveryReport(mut ReadOutSounds: SOUND) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut context: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut ownContext: BOOLEAN = BOOLEAN::FALSE;
    let mut saveStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if disableInteractivity.0 != 0 {
        return;
    }
    context = GetScanContext(&raw mut ownContext);
    OldContext = SetContext(context);
    saveStamp = SaveContextFrame(::core::ptr::null::<RECT>());
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    OldFont = SetContextFont((*pSolarSysState).SysInfo.PlanetInfo.LanderFont);
    if optWhichFonts == 0x2 as ::core::ffi::c_int {
        OldFontEffect = SetContextFontEffect(
            (*pSolarSysState).SysInfo.PlanetInfo.LanderFontEff,
        );
    } else {
        OldFontEffect = SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    }
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ());
    let mut allocated: BOOLEAN = BOOLEAN::FALSE;
    let mut StrPtr: *mut ::core::ffi::c_char = GetStringAddress(
        (*pSolarSysState).SysInfo.PlanetInfo.DiscoveryString,
    );
    if luaUqm_comm_stringNeedsInterpolate(StrPtr).0 != 0 {
        allocated = BOOLEAN::TRUE;
        StrPtr = uqm_lua_checked!(luaUqm_comm_stringInterpolate(StrPtr), ());
    }
    uqm_lua_checked!(MakeReport(ReadOutSounds, StrPtr, strlen(StrPtr) as COUNT), ());
    luaUqm_comm_uninit();
    if allocated.0 != 0 {
        HFree(StrPtr as *mut ::core::ffi::c_void);
    }
    SetContextFontEffect(OldFontEffect);
    SetContextFont(OldFont);
    uqm_lua_checked!(DrawStamp(&raw mut saveStamp), ());
    SetContext(OldContext);
    if ownContext.0 != 0 {
        DestroyScanContext();
    }
    DestroyDrawable(ReleaseDrawable(saveStamp.frame));
    uqm_lua_checked!(WaitForNoInput(-1 as ::core::ffi::c_int as TimePeriod, BOOLEAN::TRUE), ());
}
