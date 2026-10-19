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
    pub type _IO_FILE;
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type RandomContext;
    fn SleepThread(timePeriod: TimePeriod);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
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
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn GetContextFGFrame() -> FRAME;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn DrawLine(pLine: *mut LINE, thickness: BYTE);
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
    fn uio_debugInteractive(r#in: *mut FILE, out: *mut FILE, err: *mut FILE);
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut Elements: *const BYTE;
    static stdin: *mut FILE;
    static stdout: *mut FILE;
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut SysGenRNGDebug: *mut RandomContext;
    fn GetRandomSeedForStar(star: *const STAR_DESC) -> DWORD;
    fn DoPlanetaryAnalysis(SysInfoPtr: *mut SYSTEM_INFO, pPlanetDesc: *mut PLANET_DESC);
    fn getGenerateFunctions(Index: BYTE) -> *const GenerateFunctions;
    static mut CreatureData: [LIFEFORM_DESC; 0];
    static mut disp_q: QUEUE;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn SeedFleetLocation(
        FleetPtr: *mut FLEET_INFO,
        plotmap: *mut PLOT_LOCATION,
        visit: COUNT,
    ) -> POINT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: [STAR_DESC; 0];
    static mut plot_map: [PLOT_LOCATION; 70];
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn DeltaCrew(ElementPtr: *mut ELEMENT, crew_delta: SIZE) -> BOOLEAN;
    static mut GameStrings: STRING;
    fn callGenerateForScanType(
        _: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        node: COUNT,
        scanType: BYTE,
        _: *mut NODE_INFO,
    ) -> COUNT;
    fn GetPlanetTitle(buf: *mut UNICODE, bufsize: COUNT);
    static mut Screen: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut TinyFont: FONT;
    static mut PlayerControl: [BYTE; 0];
    fn luaUqm_debug_run();
    fn ship_death(ShipPtr: *mut ELEMENT);
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FILE = _IO_FILE;
pub type RESOURCE = *const ::core::ffi::c_char;
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
pub type SOUND = STRING;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const LIGHT: Self = Self(0);
    pub const MEDIUM: Self = Self(1);
    pub const HEAVY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LIFEFORM_DESC {
    pub Attributes: BYTE,
    pub ValueAndHitPoints: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const GAS_DENSITY: Self = Self(0);
    pub const LIGHT_DENSITY: Self = Self(1);
    pub const LOW_DENSITY: Self = Self(2);
    pub const NORMAL_DENSITY: Self = Self(3);
    pub const HIGH_DENSITY: Self = Self(4);
    pub const SUPER_DENSITY: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
    pub c2rust_padding: [u8; 8],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub c2rust_padding_2: [u8; 8],
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
    pub c2rust_unnamed: C2Rust_Unnamed_16,
    pub c2rust_unnamed_0: C2Rust_Unnamed_15,
    pub c2rust_unnamed_1: C2Rust_Unnamed_14,
    pub c2rust_unnamed_2: C2Rust_Unnamed_13,
    pub c2rust_unnamed_3: C2Rust_Unnamed_12,
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
    pub image: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
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
    pub tl_offset: BYTE,
    pub tr_offset: BYTE,
    pub thrust_offset: BYTE,
    pub weapon_offset: BYTE,
    pub special_offset: BYTE,
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
    pub idStr: *const ::core::ffi::c_char,
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
pub struct C2Rust_Unnamed_17(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_17 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
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
pub struct TallyResourcesArg {
    pub out: *mut FILE,
    pub mineralCount: COUNT,
    pub bioCount: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpUniverseArg {
    pub out: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpPlanetTypesArg {
    pub out: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_19(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_19 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_20(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_20 {
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
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> SDWORD {
    return (lx * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
        + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor)
            >> 1 as ::core::ffi::c_int))
        / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor));
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> SDWORD {
    return 9999 as SDWORD
        - (ly * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int))
            / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor));
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
                340 as ::core::ffi::c_int,
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
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[no_mangle]
pub static mut debugHook: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut DebugKeyPressed: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub unsafe extern "C" fn doInstantMove() {
    if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int)
        || GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int)
    {
        GlobData.Game_state.autopilot.x = logxToUniverse(GlobData.SIS_state.log_x)
            as COORD;
        GlobData.Game_state.autopilot.y = logyToUniverse(GlobData.SIS_state.log_y)
            as COORD;
    } else {
        GlobData.SIS_state.log_x = universeToLogx(GlobData.Game_state.autopilot.x);
        GlobData.SIS_state.log_y = universeToLogy(GlobData.Game_state.autopilot.y);
    }
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char), ())
        <= 1 as DWORD
    {
        CurStarDescPtr = uqm_lua_checked!(FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut GlobData.Game_state.autopilot,
            0 as SIZE,
            0 as SIZE,
        ), ());
        if !CurStarDescPtr.is_null() {
            uqm_lua_checked!(setGameStateUint(
                b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            GlobData.Game_state.ShipFacing = 0 as UWORD;
            GlobData.Game_state.ip_planet = 0 as BYTE;
            GlobData.Game_state.in_orbit = 0 as BYTE;
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_3::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                as ACTIVITY;
        }
    }
    GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
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
pub unsafe extern "C" fn resetEnergyBattle() {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut delta: COUNT = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int == 0 || inHQSpace().0 != 0
    {
        return;
    }
    if *(&raw mut PlayerControl as *mut BYTE).offset(1isize) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        StarShipPtr = findPlayerShip(1 as SIZE);
    } else if *(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        StarShipPtr = findPlayerShip(0 as SIZE);
    } else {
        StarShipPtr = ::core::ptr::null_mut::<STARSHIP>();
    }
    if StarShipPtr.is_null() || (*StarShipPtr).RaceDescPtr.is_null() {
        return;
    }
    delta = ((*(*StarShipPtr).RaceDescPtr).ship_info.max_energy as ::core::ffi::c_int
        - (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int)
        as COUNT;
    OldContext = SetContext(StatusContext);
    uqm_lua_checked!(DeltaEnergy((*StarShipPtr).hShip as *mut ELEMENT, delta as SIZE), ());
    SetContext(OldContext);
}
unsafe extern "C" fn scuttleOpponent() {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut delta: COUNT = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int == 0 || inHQSpace().0 != 0
    {
        return;
    }
    if *(&raw mut PlayerControl as *mut BYTE).offset(1isize) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        StarShipPtr = findPlayerShip(0 as SIZE);
    } else if *(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        StarShipPtr = findPlayerShip(1 as SIZE);
    } else {
        StarShipPtr = ::core::ptr::null_mut::<STARSHIP>();
    }
    if StarShipPtr.is_null() || (*StarShipPtr).RaceDescPtr.is_null() {
        return;
    }
    delta = (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level;
    if delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        OldContext = SetContext(StatusContext);
        uqm_lua_checked!(DeltaCrew(
            (*StarShipPtr).hShip as *mut ELEMENT,
            -(delta as ::core::ffi::c_int) as SIZE,
        ), ());
        SetContext(OldContext);
        uqm_lua_checked!(ship_death((*StarShipPtr).hShip as *mut ELEMENT), ());
    }
}
unsafe extern "C" fn HaltShips() {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut i: BYTE = 0;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int == 0 || inHQSpace().0 != 0
    {
        return;
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        StarShipPtr = findPlayerShip(i as SIZE);
        if StarShipPtr.is_null() || (*StarShipPtr).RaceDescPtr.is_null() {
            return;
        }
        ElementPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
        memset(
            &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>() as size_t,
        );
        UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub static mut disableInteractivity: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut EnableDebugEvents: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub unsafe extern "C" fn debugKeyPressedSynchronous() {
    if DebugKeyPressed.0 == 0 {
        printf(b"Debug Key Activated\n\n\0".as_ptr() as *const ::core::ffi::c_char);
        uqm_lua_checked!(equipShip(), ());
        showSpheres(BOOLEAN::FALSE);
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
            !(0 as ::core::ffi::c_int) as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char,
            !(0 as ::core::ffi::c_int) as DWORD,
        ), ());
    }
    uqm_lua_checked!(forwardToNextEvent(BOOLEAN::TRUE), ());
    if EnableDebugEvents.0 != 0 {
        uqm_lua_checked!(giveDevices(), ());
        uqm_lua_checked!(clearEscorts(), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::ARILOU_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::PKUNK_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::VUX_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::YEHAT_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::MELNORME_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::DRUUGE_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::ILWRATH_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::MYCON_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::SLYLANDRO_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::UMGAH_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::URQUAN_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(AddEscortShips(RACE_ID::BLACK_URQUAN_SHIP, 1 as SIZE), ());
        uqm_lua_checked!(resetCrewBattle(), ());
        uqm_lua_checked!(resetEnergyBattle(), ());
        activateAllShips();
        uqm_lua_checked!(setGameStateUint(
            b"MELNORME_CREDIT1\0".as_ptr() as *const ::core::ffi::c_char,
            100 as DWORD,
        ), ());
        GlobData.SIS_state.ResUnits = 100000 as ::core::ffi::c_int as DWORD;
        uqm_lua_checked!(dumpEvents(stderr), ());
        uqm_lua_checked!(debugContexts(), ());
    }
    DebugKeyPressed = BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn debugKey2PressedSynchronous() {
    uqm_lua_checked!(scuttleOpponent(), ());
}
#[no_mangle]
pub unsafe extern "C" fn debugKey3PressedSynchronous() {
    HaltShips();
}
#[no_mangle]
pub unsafe extern "C" fn debugKey4PressedSynchronous() {}
#[no_mangle]
pub unsafe extern "C" fn debugKeyPressed() {
    if EnableDebugEvents.0 != 0 {
        dumpStrings(stdout);
        uqm_lua_checked!(dumpPlanetTypes(stderr), ());
        ::core::ptr::write_volatile(
            &raw mut debugHook,
            Some(dumpUniverseToFile as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>,
        );
        ::core::ptr::write_volatile(
            &raw mut debugHook,
            Some(tallyResourcesToFile as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>,
        );
        uqm_lua_checked!(uio_debugInteractive(stdin, stdout, stderr), ());
        uqm_lua_checked!(luaUqm_debug_run(), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn debugKey2Pressed() {}
#[no_mangle]
pub unsafe extern "C" fn debugKey3Pressed() {}
#[no_mangle]
pub unsafe extern "C" fn debugKey4Pressed() {}
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
            != C2Rust_Unnamed_20::HYPERSPACE_ENCOUNTER_EVENT.0 as ::core::ffi::c_int
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
            uqm_lua_checked!(MoveGameClockDays(1 as COUNT), ());
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
#[no_mangle]
pub unsafe extern "C" fn raceName(mut func_index: BYTE) -> *const ::core::ffi::c_char {
    match func_index as ::core::ffi::c_int {
        1 => return b"Arilou Lalee'lay\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"Chmmr\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"Human\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"Orz\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"Pkunk\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"Shofixti\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"Spathi\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"Supox\0".as_ptr() as *const ::core::ffi::c_char,
        9 => return b"Thraddash\0".as_ptr() as *const ::core::ffi::c_char,
        10 => return b"Utwig\0".as_ptr() as *const ::core::ffi::c_char,
        11 => return b"Vux\0".as_ptr() as *const ::core::ffi::c_char,
        12 => return b"Yehat\0".as_ptr() as *const ::core::ffi::c_char,
        13 => return b"Melnorme\0".as_ptr() as *const ::core::ffi::c_char,
        14 => return b"Druuge\0".as_ptr() as *const ::core::ffi::c_char,
        15 => return b"Ilwrath\0".as_ptr() as *const ::core::ffi::c_char,
        16 => return b"Mycon\0".as_ptr() as *const ::core::ffi::c_char,
        17 => return b"Slylandro\0".as_ptr() as *const ::core::ffi::c_char,
        18 => return b"Umgah\0".as_ptr() as *const ::core::ffi::c_char,
        19 => return b"Ur-Quan\0".as_ptr() as *const ::core::ffi::c_char,
        20 => return b"Zoq-Fot-Pik\0".as_ptr() as *const ::core::ffi::c_char,
        21 => return b"Syreen\0".as_ptr() as *const ::core::ffi::c_char,
        22 => return b"Kohr-Ah\0".as_ptr() as *const ::core::ffi::c_char,
        23 => return b"Androsynth\0".as_ptr() as *const ::core::ffi::c_char,
        24 => return b"Chenjesu\0".as_ptr() as *const ::core::ffi::c_char,
        25 => return b"Mmrnmhrm\0".as_ptr() as *const ::core::ffi::c_char,
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
    uqm_lua_checked!(ForAllLinks(
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
    ), ());
    UnlockGameClock();
}
#[no_mangle]
pub unsafe extern "C" fn equipShip() {
    let mut i: ::core::ffi::c_int = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_3::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 11 as ::core::ffi::c_int {
        GlobData.SIS_state.DriveSlots[i as usize] = C2Rust_Unnamed_2::FUSION_THRUSTER.0
            as ::core::ffi::c_int as BYTE;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        GlobData.SIS_state.JetSlots[i as usize] = C2Rust_Unnamed_2::TURNING_JETS.0
            as ::core::ffi::c_int as BYTE;
        i += 1;
    }
    uqm_lua_checked!(setGameStateUint(
        b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
        ((1 as ::core::ffi::c_int)
            << C2Rust_Unnamed_4::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_4::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_4::LIGHTNING_DISASTER.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_4::LAVASPOT_DISASTER.0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
        < 2 as DWORD
    {
        i = 0 as ::core::ffi::c_int;
        let c2rust_fresh0 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh0 as usize] = C2Rust_Unnamed_2::HIGHEFF_FUELSYS
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh1 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh1 as usize] = C2Rust_Unnamed_2::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh2 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh2 as usize] = C2Rust_Unnamed_2::STORAGE_BAY
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh3 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh3 as usize] = C2Rust_Unnamed_2::STORAGE_BAY
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh4 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh4 as usize] = C2Rust_Unnamed_2::ANTIMISSILE_DEFENSE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh5 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh5 as usize] = C2Rust_Unnamed_2::TRACKING_SYSTEM
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh6 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh6 as usize] = C2Rust_Unnamed_2::TRACKING_SYSTEM
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh7 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh7 as usize] = C2Rust_Unnamed_2::TRACKING_SYSTEM
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh8 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh8 as usize] = C2Rust_Unnamed_2::DYNAMO_UNIT
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh9 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh9 as usize] = C2Rust_Unnamed_2::DYNAMO_UNIT
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh10 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh10 as usize] = C2Rust_Unnamed_2::DYNAMO_UNIT
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh11 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh11 as usize] = C2Rust_Unnamed_2::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh12 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh12 as usize] = C2Rust_Unnamed_2::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh13 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh13 as usize] = C2Rust_Unnamed_2::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh14 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh14 as usize] = C2Rust_Unnamed_2::CANNON_WEAPON
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh15 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh15 as usize] = C2Rust_Unnamed_2::CANNON_WEAPON
            .0 as ::core::ffi::c_int as BYTE;
        GlobData.SIS_state.NumLanders = 10 as BYTE;
    } else {
        i = 10 as ::core::ffi::c_int;
        let c2rust_fresh16 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh16 as usize] = C2Rust_Unnamed_2::HIGHEFF_FUELSYS
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh17 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh17 as usize] = C2Rust_Unnamed_2::CREW_POD
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh18 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh18 as usize] = C2Rust_Unnamed_2::DYNAMO_UNIT
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh19 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh19 as usize] = C2Rust_Unnamed_2::SHIVA_FURNACE
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh20 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh20 as usize] = C2Rust_Unnamed_2::CANNON_WEAPON
            .0 as ::core::ffi::c_int as BYTE;
        let c2rust_fresh21 = i;
        i += 1;
        GlobData.SIS_state.ModuleSlots[c2rust_fresh21 as usize] = C2Rust_Unnamed_2::TRACKING_SYSTEM
            .0 as ::core::ffi::c_int as BYTE;
    }
    (i <= 16 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"i <= NUM_MODULE_SLOTS\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/uqmdebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                606 as ::core::ffi::c_int,
                b"equipShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    GlobData.SIS_state.FuelOnBoard = (10 as ::core::ffi::c_int
        * 100 as ::core::ffi::c_int) as DWORD;
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
                == C2Rust_Unnamed_2::FUSION_THRUSTER.0 as ::core::ffi::c_int
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
                == C2Rust_Unnamed_2::TURNING_JETS.0 as ::core::ffi::c_int
            {
                (*pSolarSysState).turn_wait = ((*pSolarSysState).turn_wait
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
            }
            i += 1;
        }
    }
    if inHQSpace().0 != 0
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_3::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn giveDevices() {
    uqm_lua_checked!(setGameStateUint(
        b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(b"UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char, 1 as DWORD), ());
    uqm_lua_checked!(setGameStateUint(
        b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"UMGAH_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"PORTAL_KEY_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
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
    uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn showSpheres(mut Animated: BOOLEAN) {
    let mut i: BYTE = 0;
    if Animated.0 != 0 {
        i = 0 as BYTE;
        while i as ::core::ffi::c_int
            <= RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int
        {
            StartSphereTracking(RACE_ID(i as ::core::ffi::c_uint));
            i = i.wrapping_add(1);
        }
    } else {
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
        while !hStarShip.is_null() {
            let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
            FleetPtr = LockFleetInfo(
                &raw mut GlobData.Game_state.avail_race_q,
                hStarShip,
            );
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
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        let mut hSyreen: HFLEETINFO = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            RACE_ID::SYREEN_SHIP.0 as ::core::ffi::c_int as COUNT,
        );
        let mut SyreenPtr: *mut FLEET_INFO = LockFleetInfo(
            &raw mut GlobData.Game_state.avail_race_q,
            hSyreen,
        );
        let mut hChmmr: HFLEETINFO = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            RACE_ID::CHMMR_SHIP.0 as ::core::ffi::c_int as COUNT,
        );
        let mut ChmmrPtr: *mut FLEET_INFO = LockFleetInfo(
            &raw mut GlobData.Game_state.avail_race_q,
            hChmmr,
        );
        (*SyreenPtr).actual_strength = (300 as ::core::ffi::c_int
            / 11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COUNT;
        (*SyreenPtr).loc = SeedFleetLocation(
            SyreenPtr,
            &raw mut plot_map as *mut PLOT_LOCATION,
            C2Rust_Unnamed_10::HOME.0 as ::core::ffi::c_int as COUNT,
        );
        StartSphereTracking(RACE_ID::SYREEN_SHIP);
        (*ChmmrPtr).actual_strength = (986 as ::core::ffi::c_int
            / 11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COUNT;
        (*ChmmrPtr).loc = SeedFleetLocation(
            ChmmrPtr,
            &raw mut plot_map as *mut PLOT_LOCATION,
            C2Rust_Unnamed_10::HOME.0 as ::core::ffi::c_int as COUNT,
        );
        StartSphereTracking(RACE_ID::CHMMR_SHIP);
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hSyreen);
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hChmmr);
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
            (*FleetPtr).allied_state = C2Rust_Unnamed_17::GOOD_GUY.0
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
    i = 0 as ::core::ffi::c_int;
    while i < 502 as ::core::ffi::c_int {
        uqm_lua_checked!(callback
            .expect(
                "non-null function pointer",
            )((&raw mut star_array as *mut STAR_DESC).offset(i as isize), arg), ());
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
                840 as ::core::ffi::c_int,
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
                841 as ::core::ffi::c_int,
                b"forAllPlanets\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < (*system).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
    {
        uqm_lua_checked!(callback
            .expect(
                "non-null function pointer",
            )(
            star,
            system,
            (&raw mut (*system).PlanetDesc as *mut PLANET_DESC).offset(i as isize),
            arg,
        ), ());
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
                854 as ::core::ffi::c_int,
                b"forAllMoons\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
        uqm_lua_checked!(callback
            .expect(
                "non-null function pointer",
            )(
            star,
            system,
            planet,
            (&raw mut (*system).MoonDesc as *mut PLANET_DESC).offset(i as isize),
            arg,
        ), ());
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
    uqm_lua_checked!(forAllStars(
        Some(
            starRecurse
                as unsafe extern "C" fn(*mut STAR_DESC, *mut ::core::ffi::c_void) -> (),
        ),
        universeRecurseArg as *mut ::core::ffi::c_void,
    ), ());
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
                PartiallyScavengedList: [[0; 32]; 3],
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
        InOrbit: BOOLEAN::FALSE,
        ScanFrame: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
    };
    let mut oldPSolarSysState: *mut SOLARSYS_STATE = pSolarSysState;
    let mut oldStarDescPtr: *mut STAR_DESC = CurStarDescPtr;
    CurStarDescPtr = star;
    RandomContext_SeedRandom(SysGenRNGDebug, GetRandomSeedForStar(star));
    memset(
        &raw mut SolarSysState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SOLARSYS_STATE>() as size_t,
    );
    SolarSysState.SunDesc[0usize].pPrevDesc = ::core::ptr::null_mut::<PLANET_DESC>();
    SolarSysState.SunDesc[0usize].rand_seed = RandomContext_Random(SysGenRNGDebug);
    SolarSysState.SunDesc[0usize].data_index = (((*star).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE;
    SolarSysState.SunDesc[0usize].location.x = 0 as COORD;
    SolarSysState.SunDesc[0usize].location.y = 0 as COORD;
    SolarSysState.genFuncs = getGenerateFunctions((*star).Index);
    pSolarSysState = &raw mut SolarSysState;
    uqm_lua_checked!(Some((*SolarSysState.genFuncs).generatePlanets.expect("non-null function pointer"))
        .expect("non-null function pointer")(&raw mut SolarSysState), ());
    if (*universeRecurseArg).systemFuncPre.is_some() {
        uqm_lua_checked!(Some((*universeRecurseArg).systemFuncPre.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(star, &raw mut SolarSysState, (*universeRecurseArg).arg), ());
    }
    if (*universeRecurseArg).planetFuncPre.is_some()
        || (*universeRecurseArg).planetFuncPost.is_some()
        || (*universeRecurseArg).moonFunc.is_some()
    {
        uqm_lua_checked!(forAllPlanets(
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
        ), ());
    }
    if (*universeRecurseArg).systemFuncPost.is_some() {
        uqm_lua_checked!(Some((*universeRecurseArg).systemFuncPost.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(star, &raw mut SolarSysState, (*universeRecurseArg).arg), ());
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
                942 as ::core::ffi::c_int,
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
                943 as ::core::ffi::c_int,
                b"planetRecurse\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*planet).pPrevDesc = (&raw mut (*system).SunDesc as *mut PLANET_DESC)
        .offset(0isize);
    if (*universeRecurseArg).planetFuncPre.is_some() {
        (*system).pOrbitalDesc = planet;
        DoPlanetaryAnalysis(&raw mut (*system).SysInfo, planet);
        uqm_lua_checked!(Some((*(*system).genFuncs).generateOrbital.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, planet), ());
        uqm_lua_checked!(Some((*universeRecurseArg).planetFuncPre.expect("non-null function pointer"))
            .expect("non-null function pointer")(planet, (*universeRecurseArg).arg), ());
    }
    if (*universeRecurseArg).moonFunc.is_some() {
        RandomContext_SeedRandom(SysGenRNGDebug, (*planet).rand_seed);
        uqm_lua_checked!(Some((*(*system).genFuncs).generateMoons.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, planet), ());
        uqm_lua_checked!(forAllMoons(
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
        ), ());
    }
    if (*universeRecurseArg).planetFuncPost.is_some() {
        (*system).pOrbitalDesc = planet;
        DoPlanetaryAnalysis(&raw mut (*system).SysInfo, planet);
        uqm_lua_checked!(Some((*(*system).genFuncs).generateOrbital.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, planet), ());
        uqm_lua_checked!(Some((*universeRecurseArg).planetFuncPost.expect("non-null function pointer"))
            .expect("non-null function pointer")(planet, (*universeRecurseArg).arg), ());
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
                988 as ::core::ffi::c_int,
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
                989 as ::core::ffi::c_int,
                b"moonRecurse\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*moon).pPrevDesc = planet;
    if (*universeRecurseArg).moonFunc.is_some() {
        (*system).pOrbitalDesc = moon;
        if (*moon).data_index as ::core::ffi::c_int
            != C2Rust_Unnamed_8::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
            && (*moon).data_index as ::core::ffi::c_int
                != C2Rust_Unnamed_8::SA_MATRA.0 as ::core::ffi::c_int
            && (*moon).data_index as ::core::ffi::c_int
                != C2Rust_Unnamed_8::DESTROYED_STARBASE.0 as ::core::ffi::c_int
            && (*moon).data_index as ::core::ffi::c_int
                != C2Rust_Unnamed_8::PRECURSOR_STARBASE.0 as ::core::ffi::c_int
        {
            DoPlanetaryAnalysis(&raw mut (*system).SysInfo, moon);
        }
        uqm_lua_checked!(Some((*(*system).genFuncs).generateOrbital.expect("non-null function pointer"))
            .expect("non-null function pointer")(system, moon), ());
        uqm_lua_checked!(Some((*universeRecurseArg).moonFunc.expect("non-null function pointer"))
            .expect("non-null function pointer")(moon, (*universeRecurseArg).arg), ());
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
    uqm_lua_checked!(UniverseRecurse(&raw mut universeRecurseArg), ());
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
    uqm_lua_checked!(dumpUniverse(out), ());
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
        42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 => {
            return b"Rainbow world\0".as_ptr() as *const ::core::ffi::c_char;
        }
        52 => return b"Ilwrath home\0".as_ptr() as *const ::core::ffi::c_char,
        53 => return b"Androsynth ruins\0".as_ptr() as *const ::core::ffi::c_char,
        54 => return b"Mycon trap\0".as_ptr() as *const ::core::ffi::c_char,
        55 | 56 | 57 | 58 | 59 | 60 | 61 => {
            return b"Destroyed Starbase\0".as_ptr() as *const ::core::ffi::c_char;
        }
        62 => return b"Mother-Ark\0".as_ptr() as *const ::core::ffi::c_char,
        63 | 64 | 65 | 66 => {
            return b"Zoq Fot Ruins\0".as_ptr() as *const ::core::ffi::c_char;
        }
        67 => return b"Algolites\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn dumpPlanetCallback(
    mut planet: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut out: *mut FILE = (*(arg as *mut DumpUniverseArg)).out;
    uqm_lua_checked!(dumpPlanet(out, planet), ());
}
#[no_mangle]
pub unsafe extern "C" fn dumpPlanet(mut out: *mut FILE, mut planet: *const PLANET_DESC) {
    uqm_lua_checked!(Some((*(*pSolarSysState).genFuncs).generateName.expect("non-null function pointer"))
        .expect("non-null function pointer")(pSolarSysState, planet), ());
    fprintf(
        out,
        b"- %-37s  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
        planetTypeString(
            (*planet).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int),
        ),
    );
    uqm_lua_checked!(dumpWorld(out, planet), ());
}
unsafe extern "C" fn dumpMoonCallback(
    mut moon: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut out: *mut FILE = (*(arg as *mut DumpUniverseArg)).out;
    uqm_lua_checked!(dumpMoon(out, moon), ());
}
#[no_mangle]
pub unsafe extern "C" fn dumpMoon(mut out: *mut FILE, mut moon: *const PLANET_DESC) {
    let mut typeStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if (*moon).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        typeStr = b"StarBase\0".as_ptr() as *const ::core::ffi::c_char;
    } else if (*moon).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::SA_MATRA.0 as ::core::ffi::c_int
    {
        typeStr = b"Sa-Matra\0".as_ptr() as *const ::core::ffi::c_char;
    } else if (*moon).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::DESTROYED_STARBASE.0 as ::core::ffi::c_int
    {
        typeStr = b"Destroyed StarBase\0".as_ptr() as *const ::core::ffi::c_char;
    } else if (*moon).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::PRECURSOR_STARBASE.0 as ::core::ffi::c_int
    {
        typeStr = b"Precursor StarBase\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        typeStr = planetTypeString(
            (*moon).data_index as ::core::ffi::c_int
                & !(C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int),
        );
    }
    fprintf(
        out,
        b"  - Moon %-30c  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        'a' as ::core::ffi::c_int
            + moon
                .offset_from(
                    (&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                        .offset(0isize),
                ) as UNICODE as ::core::ffi::c_int,
        typeStr,
    );
    uqm_lua_checked!(dumpWorld(out, moon), ());
}
unsafe extern "C" fn dumpWorld(mut out: *mut FILE, mut world: *const PLANET_DESC) {
    let mut info: *mut PLANET_INFO = ::core::ptr::null_mut::<PLANET_INFO>();
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::SA_MATRA.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::DESTROYED_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::PRECURSOR_STARBASE.0 as ::core::ffi::c_int
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
        (*info).PlanetToSunDist,
    );
    if (*world).data_index as ::core::ffi::c_int
        & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    fprintf(
        out,
        b"          Bio: %4d    Min: %4d\n\0".as_ptr() as *const ::core::ffi::c_char,
        uqm_lua_checked!(calculateBioValue(pSolarSysState, world), ()) as ::core::ffi::c_int,
        uqm_lua_checked!(calculateMineralValue(pSolarSysState, world), ()) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fprintfWorld(mut world: *const PLANET_DESC) {
    let mut info: *mut PLANET_INFO = ::core::ptr::null_mut::<PLANET_INFO>();
    let mut buf: [UNICODE; 200] = [0; 200];
    let mut fp: *mut FILE = fopen(
        b"planetLog.txt\0".as_ptr() as *const ::core::ffi::c_char,
        b"a\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut universe: POINT = (*CurStarDescPtr).star_pt;
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
        || (*world).data_index as ::core::ffi::c_int
            == C2Rust_Unnamed_8::SA_MATRA.0 as ::core::ffi::c_int
        || (*world).data_index as ::core::ffi::c_int
            == C2Rust_Unnamed_8::DESTROYED_STARBASE.0 as ::core::ffi::c_int
        || (*world).data_index as ::core::ffi::c_int
            == C2Rust_Unnamed_8::PRECURSOR_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    fprintf(
        fp,
        b"Coords:     %03u.%01u : %03u.%01u\n\0".as_ptr() as *const ::core::ffi::c_char,
        universe.x as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        universe.x as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
        universe.y as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        universe.y as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
    );
    GetClusterName(CurStarDescPtr, &raw mut buf as *mut UNICODE);
    fprintf(
        fp,
        b"Star:       %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut buf as *mut UNICODE,
    );
    uqm_lua_checked!(GetPlanetTitle(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 200]>() as COUNT,
    ), ());
    if strcmp(
        &raw mut buf as *mut UNICODE,
        &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
    ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            fp,
            b"Planet:     %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
        );
    }
    info = &raw mut (*pSolarSysState).SysInfo.PlanetInfo;
    fprintf(
        fp,
        b"World:      %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut buf as *mut UNICODE,
    );
    fprintf(
        fp,
        b"DistToSun:  %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).PlanetToSunDist,
    );
    fprintf(
        fp,
        b"Atmosphere: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).AtmoDensity as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Temp:       %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).SurfaceTemperature as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Weather:    %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).Weather as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Tectonics:  %d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).Tectonics as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Density:    %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).PlanetDensity as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Radius:     %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).PlanetRadius as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Gravity:    %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).SurfaceGravity as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"Day:        %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).RotationPeriod as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"AxialTilt:  %d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*info).AxialTilt as ::core::ffi::c_int,
    );
    if (*world).data_index as ::core::ffi::c_int
        & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
    {
        fprintf(
            fp,
            b"LifeChance: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*info).LifeChance as ::core::ffi::c_int,
        );
        fprintf(
            fp,
            b"____________________________________\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fclose(fp);
        return;
    } else {
        fprintf(
            fp,
            b"LifeChance: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*info).LifeChance as ::core::ffi::c_int,
        );
    }
    fprintf(
        fp,
        b"Bio: %4d    Min: %4d\n\0".as_ptr() as *const ::core::ffi::c_char,
        uqm_lua_checked!(calculateBioValue(pSolarSysState, world), ()) as ::core::ffi::c_int,
        uqm_lua_checked!(calculateMineralValue(pSolarSysState, world), ()) as ::core::ffi::c_int,
    );
    fprintf(
        fp,
        b"____________________________________\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fclose(fp);
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
                1429 as ::core::ffi::c_int,
                b"calculateBioValue\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numBio = uqm_lua_checked!(callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    ), 0);
    result = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numBio as ::core::ffi::c_int {
        let mut info: NODE_INFO = NODE_INFO {
            loc_pt: POINT { x: 0, y: 0 },
            density: 0,
            r#type: 0,
        };
        uqm_lua_checked!(callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        ), 0);
        result = (result as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * ((*(&raw mut CreatureData as *mut LIFEFORM_DESC)
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
                1452 as ::core::ffi::c_int,
                b"generateBioIndex\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numBio = uqm_lua_checked!(callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    ), ());
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
        uqm_lua_checked!(callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        ), ());
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
                1475 as ::core::ffi::c_int,
                b"calculateMineralValue\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numDeposits = uqm_lua_checked!(callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    ), 0);
    result = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numDeposits as ::core::ffi::c_int {
        let mut info: NODE_INFO = NODE_INFO {
            loc_pt: POINT { x: 0, y: 0 },
            density: 0,
            r#type: 0,
        };
        uqm_lua_checked!(callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        ), 0);
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
                1498 as ::core::ffi::c_int,
                b"generateMineralIndex\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numDeposits = uqm_lua_checked!(callGenerateForScanType(
        system,
        world,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    ), ());
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
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
        uqm_lua_checked!(callGenerateForScanType(
            system,
            world,
            i,
            PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
            &raw mut info,
        ), ());
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
    uqm_lua_checked!(UniverseRecurse(&raw mut universeRecurseArg), ());
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
    uqm_lua_checked!(tallyResources(out), ());
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
    uqm_lua_checked!(tallyResourcesWorld(arg as *mut TallyResourcesArg, planet), ());
}
unsafe extern "C" fn tallyMoonCallback(
    mut moon: *const PLANET_DESC,
    mut arg: *mut ::core::ffi::c_void,
) {
    uqm_lua_checked!(tallyResourcesWorld(arg as *mut TallyResourcesArg, moon), ());
}
unsafe extern "C" fn tallyResourcesWorld(
    mut arg: *mut TallyResourcesArg,
    mut world: *const PLANET_DESC,
) {
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::SA_MATRA.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::DESTROYED_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    if (*world).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::PRECURSOR_STARBASE.0 as ::core::ffi::c_int
    {
        return;
    }
    (*arg).bioCount = ((*arg).bioCount as ::core::ffi::c_int
        + uqm_lua_checked!(calculateBioValue(pSolarSysState, world), ()) as ::core::ffi::c_int) as COUNT;
    (*arg).mineralCount = ((*arg).mineralCount as ::core::ffi::c_int
        + uqm_lua_checked!(calculateMineralValue(pSolarSysState, world), ()) as ::core::ffi::c_int) as COUNT;
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
    while i < C2Rust_Unnamed_8::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int {
        uqm_lua_checked!(callback
            .expect(
                "non-null function pointer",
            )(
            i,
            (&raw const planet_array as *const PlanetFrame).offset(i as isize),
            arg,
        ), ());
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dumpPlanetTypes(mut out: *mut FILE) {
    let mut dumpPlanetTypesArg: DumpPlanetTypesArg = DumpPlanetTypesArg {
        out: ::core::ptr::null_mut::<FILE>(),
    };
    dumpPlanetTypesArg.out = out;
    uqm_lua_checked!(forAllPlanetTypes(
        Some(
            dumpPlanetTypeCallback
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const PlanetFrame,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut dumpPlanetTypesArg as *mut ::core::ffi::c_void,
    ), ());
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
                        (C2Rust_Unnamed_19::ELEMENTS_STRING_BASE.0 as ::core::ffi::c_int
                            + (*entry).ElementType as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_19::CARGO_STRING_BASE.0 as ::core::ffi::c_int
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
    if typeIndex >= C2Rust_Unnamed_8::FIRST_GAS_GIANT.0 as ::core::ffi::c_int {
        snprintf(
            &raw mut typeStr as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>() as size_t,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_19::SCAN_STRING_BASE.0 as ::core::ffi::c_int
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
                    (C2Rust_Unnamed_19::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + typeIndex) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_19::SCAN_STRING_BASE.0 as ::core::ffi::c_int
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
pub unsafe extern "C" fn resetCrewBattle() {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut delta: COUNT = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int == 0 || inHQSpace().0 != 0
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
    uqm_lua_checked!(DeltaCrew((*StarShipPtr).hShip as *mut ELEMENT, delta as SIZE), ());
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn dumpStrings(mut out: *mut FILE) {
    let mut categories: [C2Rust_Unnamed_18; 25] = [
        C2Rust_Unnamed_18 {
            name: b"0\0".as_ptr() as *const ::core::ffi::c_char,
            base: 0 as size_t,
            count: 0 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"STAR_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::STAR_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 149 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"DEVICE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::DEVICE_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 29 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"CARGO_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::CARGO_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 14 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"ELEMENTS_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::ELEMENTS_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 133 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"SCAN_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::SCAN_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 66 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"STAR_NUMBER_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::STAR_NUMBER_BASE.0 as ::core::ffi::c_int as size_t,
            count: 14 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"PLANET_NUMBER_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 43 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"MONTHS_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::MONTHS_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 12 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"FEEDBACK_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 8 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"STARBASE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: (12 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"ENCOUNTER_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::ENCOUNTER_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 8 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"NAVIGATION_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 9 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"NAMING_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 8 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"MELEE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::MELEE_STRING_BASE.0 as ::core::ffi::c_int as size_t,
            count: 25 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"SAVEGAME_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 10 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"OPTION_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 10 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"QUITMENU_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 5 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"STATUS_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 22 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"FLAGSHIP_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 13 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"ORBITSCAN_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 19 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"MAINMENU_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 74 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"NETMELEE_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 34 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"BIOLOGICAL_STRING_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::BIOLOGICAL_STRING_BASE.0 as ::core::ffi::c_int
                as size_t,
            count: 26 as size_t,
        },
        C2Rust_Unnamed_18 {
            name: b"GAMESTR_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
            base: C2Rust_Unnamed_19::GAMESTR_COUNT.0 as ::core::ffi::c_int as size_t,
            count: -1 as ::core::ffi::c_int as size_t,
        },
    ];
    let mut numCategories: size_t = (::core::mem::size_of::<[C2Rust_Unnamed_18; 25]>()
        as size_t)
        .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_18>() as size_t);
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
    if C2Rust_Unnamed_19::GAMESTR_COUNT.0 as ::core::ffi::c_int as size_t != numStrings {
        fprintf(
            stderr,
            b"Warning: GAMESTR_COUNT is %i, but GameStrings contains %d strings.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            C2Rust_Unnamed_19::GAMESTR_COUNT.0 as ::core::ffi::c_int,
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
                1963 as ::core::ffi::c_int,
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
                1964 as ::core::ffi::c_int,
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
                1965 as ::core::ffi::c_int,
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
                1969 as ::core::ffi::c_int,
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
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut oldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldFgCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut rectCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut lineCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut textCol: Color = Color { r: 0, g: 0, b: 0, a: 0 };
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
    rectCol = hsvaToRgba(hue, 1.0f64, 0.5f64, 100 as BYTE);
    lineCol = hsvaToRgba(hue, 1.0f64, 1.0f64, 90 as BYTE);
    textCol = lineCol;
    oldContext = SetContext(context);
    GetContextClipRect(&raw mut rect);
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
    uqm_lua_checked!(DrawFilledRectangle(&raw mut rect), ());
    SetContextForeGroundColor(lineCol);
    line.first = p1;
    line.second = p2;
    uqm_lua_checked!(DrawLine(&raw mut line, 1 as BYTE), ());
    line.first = p2;
    line.second = p4;
    uqm_lua_checked!(DrawLine(&raw mut line, 1 as BYTE), ());
    line.first = p1;
    line.second = p3;
    uqm_lua_checked!(DrawLine(&raw mut line, 1 as BYTE), ());
    line.first = p3;
    line.second = p4;
    uqm_lua_checked!(DrawLine(&raw mut line, 1 as BYTE), ());
    line.first = p1;
    line.second = p4;
    uqm_lua_checked!(DrawLine(&raw mut line, 1 as BYTE), ());
    line.first = p2;
    line.second = p3;
    uqm_lua_checked!(DrawLine(&raw mut line, 1 as BYTE), ());
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
    uqm_lua_checked!(font_DrawText(&raw mut text), ());
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
        .menu[C2Rust_Unnamed_1::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0
    {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimePeriod), BOOLEAN(0));
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
    screenRect.extent.width = CanvasWidth as COORD;
    screenRect.extent.height = CanvasHeight as COORD;
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
    uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
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
                    uqm_lua_checked!(drawContext(
                        context,
                        visibleContextI as ::core::ffi::c_double * hueIncrement,
                    ), ());
                    visibleContextI = visibleContextI.wrapping_add(1);
                }
                describeContext(stderr, context);
            }
            context = GetNextContext(context);
        }
        uqm_lua_checked!(putScreen(debugDrawFrame), ());
        let mut state: WAIT_STATE = WAIT_STATE { InputFunc: None };
        state.InputFunc = Some(
            waitForKey as unsafe extern "C" fn(*mut wait_state) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut wait_state) -> BOOLEAN>;
        uqm_lua_checked!(DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::TRUE), ());
        SetContext(orgContext);
        DestroyContext(debugDrawContext);
        DestroyDrawable(ReleaseDrawable(debugDrawFrame));
        uqm_lua_checked!(putScreen(savedScreen), ());
        DestroyDrawable(ReleaseDrawable(savedScreen));
    }
    ::core::ptr::write_volatile(&raw mut inDebugContexts, false);
}
