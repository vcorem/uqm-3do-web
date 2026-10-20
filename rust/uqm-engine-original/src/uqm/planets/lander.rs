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

// Rust translation of src/uqm/planets/lander.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    static mut Elements: *const BYTE;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn DrawBatch(
        pBasePrim: *mut PRIMITIVE,
        PrimLinks: PRIM_LINKS,
        BatchFlags: BATCH_FLAGS,
    );
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn GetFrameParentDrawable(Frame: FRAME) -> DRAWABLE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn utf8StringCountN(
        start: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> size_t;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn GetTimeCounter() -> TimeCount;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut sinetab: [SIZE; 0];
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn GetNextVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
        num_frames: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn FreeElement(hElement: HELEMENT);
    fn RemoveElement(hLink: HLINK);
    static mut DisplayLinks: PRIM_LINKS;
    fn RepairBackRect(pRect: *mut RECT);
    fn callPickupForScanType(
        solarSys: *mut SOLARSYS_STATE,
        world: *mut PLANET_DESC,
        node: COUNT,
        scanType: BYTE,
    ) -> bool;
    fn RedrawSurfaceScan(newLoc: *const POINT);
    fn setNodeRetrieved(planetInfo: *mut PLANET_INFO, scanType: BYTE, nodeNr: BYTE);
    fn TFB_Random() -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut PlanetContext: CONTEXT;
    fn DrawStarBackGround();
    fn RotatePlanetSphere(keepRate: BOOLEAN);
    fn DrawPlanetSurfaceBorder();
    fn load_orbit_theme(selector: BYTE) -> MUSIC_REF;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn DrawLanders();
    fn DrawStorageBays(Refresh: BOOLEAN);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SIZE,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetStorageBayCapacity() -> COUNT;
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut GameStrings: STRING;
    fn InsertPrim(pLinks: *mut PRIM_LINKS, primIndex: COUNT, iPI: COUNT);
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
    static mut RadarContext: CONTEXT;
    static mut TinyFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlaySound(
        S: SOUND,
        Pos: SoundPosition,
        PositionalObject: *mut ELEMENT,
        Priority: BYTE,
    );
    fn NotPositional() -> SoundPosition;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
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
    pub const NUM_PRIMS: Self = Self(7);
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
}
pub type BATCH_FLAGS = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type SOUND_REF = STRING_TABLE;
pub type SOUND = STRING;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SoundPosition = soundposition;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TimeCount = DWORD;
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
pub struct PLANETSIDE_DESC {
    pub InTransit: BOOLEAN,
    pub ElementLevel: COUNT,
    pub MaxElementLevel: COUNT,
    pub BiologicalLevel: COUNT,
    pub ElementAmounts: [COUNT; 8],
    pub NumFrames: COUNT,
    pub AmountBuf: [UNICODE; 40],
    pub MineralText: [TEXT; 3],
    pub ColorCycle: [Color; 16],
    pub TectonicsChance: BYTE,
    pub WeatherChance: BYTE,
    pub FireChance: BYTE,
}
pub type MENU_SOUND_FLAGS = UWORD;
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
pub struct LanderInputState {
    pub InputFunc: Option<unsafe extern "C" fn(*mut LanderInputState) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub NextTime: TimeCount,
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
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LIFEFORM_DESC {
    pub Attributes: BYTE,
    pub ValueAndHitPoints: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_KB_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_KB_2: Self = Self(1);
    pub const CONTROL_TEMPLATE_KB_3: Self = Self(2);
    pub const CONTROL_TEMPLATE_JOY_1: Self = Self(3);
    pub const CONTROL_TEMPLATE_JOY_2: Self = Self(4);
    pub const CONTROL_TEMPLATE_JOY_3: Self = Self(5);
    pub const NUM_TEMPLATES: Self = Self(6);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const PS_HUMAN_PLAYER: Self = Self(0);
    pub const PS_NON_PLAYER: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const KEY_UP: Self = Self(0);
    pub const KEY_DOWN: Self = Self(1);
    pub const KEY_LEFT: Self = Self(2);
    pub const KEY_RIGHT: Self = Self(3);
    pub const KEY_WEAPON: Self = Self(4);
    pub const KEY_SPECIAL: Self = Self(5);
    pub const KEY_ESCAPE: Self = Self(6);
    pub const NUM_KEYS: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
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
                    b"src/uqm/planets/../displist.h\0"
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
                    b"src/uqm/planets/../displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub static mut LanderFrame: [FRAME; 8] = [::core::ptr::null_mut::<FRAME_DESC>(); 8];
static mut LanderSounds: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut LanderMusic: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
static mut OrbitMusic: [MUSIC_REF; 5] = [::core::ptr::null_mut::<
    *mut TFB_SoundSample,
>(); 5];
#[no_mangle]
pub static mut CreatureData: [LIFEFORM_DESC; 26] = [
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 6 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((10 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 8 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((8 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((15 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 10 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((6 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((12 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 9 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((8 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 7 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 15 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 6 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 8 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
    LIFEFORM_DESC {
        Attributes: ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE,
        ValueAndHitPoints: ((15 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 15 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
    },
];
static mut lander_flags: BYTE = 0;
static mut curLanderLoc: POINT = POINT { x: 0, y: 0 };
static mut crew_left: ::core::ffi::c_int = 0;
static mut shieldHit: ::core::ffi::c_int = 0;
static mut damage_index: ::core::ffi::c_int = 0;
static mut explosion_index: ::core::ffi::c_int = 0;
static mut turn_wait: ::core::ffi::c_int = 0;
static mut weapon_wait: ::core::ffi::c_int = 0;
static mut planetSideDesc: *mut PLANETSIDE_DESC = ::core::ptr::null_mut::<
    PLANETSIDE_DESC,
>();
unsafe extern "C" fn DamageColorCycle(mut c: Color, mut i: COUNT) -> Color {
    static mut damage_tab: [Color; 7] = [
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
            r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    if i != 0 {
        c = damage_tab[i as usize];
    } else if sameColor(c, colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE)) {
        c = damage_tab[6usize];
    } else if sameColor(c, colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE)) {
        c = damage_tab[5usize];
    } else if sameColor(c, colorFromRgb15(0x1f as BYTE, 0x1c as BYTE, 0 as BYTE)) {
        c = damage_tab[4usize];
    } else if sameColor(c, colorFromRgb15(0x1f as BYTE, 0x15 as BYTE, 0 as BYTE)) {
        c = damage_tab[3usize];
    } else if sameColor(c, colorFromRgb15(0x1f as BYTE, 0xe as BYTE, 0 as BYTE)) {
        c = damage_tab[2usize];
    } else if sameColor(c, colorFromRgb15(0x1f as BYTE, 0x7 as BYTE, 0 as BYTE)) {
        c = damage_tab[1usize];
    } else {
        c = damage_tab[0usize];
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn object_animation(mut ElementPtr: *mut ELEMENT) {
    let mut frame_index: COUNT = 0;
    let mut angle: COUNT = 0;
    let mut pPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    pPrim = (&raw mut DisplayArray as *mut PRIMITIVE)
        .offset((*ElementPtr).PrimIndex as isize);
    if (*pPrim).Type as ::core::ffi::c_int
        == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
        && !((*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
            && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int)
    {
        let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        c = DamageColorCycle((*pPrim).color, 0 as COUNT);
        if sameColor(c, colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE)) {
            (*pPrim).Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            if (*ElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                memset(
                    &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<VELOCITY_DESC>(),
                );
                (*pPrim).Object.Stamp.frame = SetAbsFrameIndex(
                    (*pPrim).Object.Stamp.frame,
                    0 as COUNT,
                );
                PlaySound(
                    SetAbsStringTableIndex(
                        LanderSounds,
                        C2Rust_Unnamed_7::LIFEFORM_CANNED.0 as ::core::ffi::c_int
                            as COUNT,
                    ),
                    NotPositional(),
                    ::core::ptr::null_mut::<ELEMENT>(),
                    2 as BYTE,
                );
            }
        }
        (*pPrim).color = c;
    }
    frame_index = (GetFrameIndex((*pPrim).Object.Stamp.frame) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COUNT;
    if ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = ((*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait as ::core::ffi::c_int
            + ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        (*pPrim).Object.Stamp.frame = IncFrameIndex((*pPrim).Object.Stamp.frame);
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
        {
            if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                explosion_index += 1;
                if explosion_index >= 10 as ::core::ffi::c_int {
                    (*pPrim).Object.Stamp.frame = DecFrameIndex(
                        (*pPrim).Object.Stamp.frame,
                    );
                }
            } else if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
            {
                let mut s: SIZE = 0;
                if frame_index as ::core::ffi::c_int >= 13 as ::core::ffi::c_int {
                    s = 0 as SIZE;
                } else {
                    s = (14 as ::core::ffi::c_int - frame_index as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE;
                }
                (*pPrim).color = colorFromRgb15(
                    0x1f as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
                if frame_index as ::core::ffi::c_int == 13 as ::core::ffi::c_int {
                    PlaySound(
                        SetAbsStringTableIndex(
                            LanderSounds,
                            C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
                                as COUNT,
                        ),
                        NotPositional(),
                        ::core::ptr::null_mut::<ELEMENT>(),
                        2 as BYTE,
                    );
                }
            }
            if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int
                && frame_index as ::core::ffi::c_int == 5 as ::core::ffi::c_int
                && TFB_Random().wrapping_rem(100 as DWORD) < 90 as DWORD
            {
                let mut hLavaElement: HELEMENT = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                hLavaElement = AddGroundDisaster(
                    C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as COUNT,
                );
                if !hLavaElement.is_null() {
                    let mut LavaElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<
                        ELEMENT,
                    >();
                    angle = (((*ElementPtr).c2rust_unnamed_0.facing
                        as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                    LavaElementPtr = LockLink(&raw mut disp_q, hLavaElement)
                        as *mut ELEMENT;
                    (*LavaElementPtr).next.location = (*ElementPtr).next.location;
                    (*LavaElementPtr).next.location.x = ((*LavaElementPtr)
                        .next
                        .location
                        .x as ::core::ffi::c_int
                        + (*(&raw mut sinetab as *mut SIZE)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 2 as ::core::ffi::c_int)
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as COUNT as isize,
                            ) as ::core::ffi::c_long
                            * 4 as ::core::ffi::c_int as ::core::ffi::c_long
                            >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int)
                        as COORD;
                    (*LavaElementPtr).next.location.y = ((*LavaElementPtr)
                        .next
                        .location
                        .y as ::core::ffi::c_int
                        + (*(&raw mut sinetab as *mut SIZE)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as COUNT as isize,
                            ) as ::core::ffi::c_long
                            * 4 as ::core::ffi::c_int as ::core::ffi::c_long
                            >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int)
                        as COORD;
                    if ((*LavaElementPtr).next.location.y as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*LavaElementPtr).next.location.y = 0 as COORD;
                    } else if (*LavaElementPtr).next.location.y as ::core::ffi::c_int
                        >= (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int
                    {
                        (*LavaElementPtr).next.location.y = (((75 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COORD;
                    }
                    if ((*LavaElementPtr).next.location.x as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*LavaElementPtr).next.location.x = ((*LavaElementPtr)
                            .next
                            .location
                            .x as ::core::ffi::c_int
                            + ((ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                            as COORD;
                    } else {
                        (*LavaElementPtr).next.location.x = ((*LavaElementPtr)
                            .next
                            .location
                            .x as ::core::ffi::c_int
                            % ((ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                            as COORD;
                    }
                    (*LavaElementPtr).c2rust_unnamed_0.facing = (((*ElementPtr)
                        .c2rust_unnamed_0
                        .facing as DWORD)
                        .wrapping_add(
                            TFB_Random()
                                .wrapping_rem(3 as DWORD)
                                .wrapping_sub(1 as DWORD),
                        )
                        & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD) as COUNT;
                    UnlockLink(&raw mut disp_q, hLavaElement);
                }
            }
        } else if frame_index as ::core::ffi::c_int & 3 as ::core::ffi::c_int == 0
            && (*ElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int != 0
        {
            let mut index: BYTE = 0;
            let mut speed: COUNT = 0;
            index = ((*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE;
            speed = (CreatureData[index as usize].Attributes as ::core::ffi::c_int
                & 0x30 as ::core::ffi::c_int) as COUNT;
            if speed != 0 {
                let mut dx: SIZE = 0;
                let mut dy: SIZE = 0;
                let mut old_angle: COUNT = 0;
                dx = (curLanderLoc.x as ::core::ffi::c_int
                    - (*ElementPtr).next.location.x as ::core::ffi::c_int) as SIZE;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    && (dx as ::core::ffi::c_int)
                        < -((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    dx = (dx as ::core::ffi::c_int
                        + ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                        as SIZE;
                } else if dx as ::core::ffi::c_int
                    > (ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    dx = (dx as ::core::ffi::c_int
                        - ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                        as SIZE;
                }
                dy = (curLanderLoc.y as ::core::ffi::c_int
                    - (*ElementPtr).next.location.y as ::core::ffi::c_int) as SIZE;
                angle = ARCTAN(dx, dy);
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if dx as ::core::ffi::c_int
                    >= ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int
                    || dy as ::core::ffi::c_int
                        >= ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int
                    || dx as ::core::ffi::c_int * dx as ::core::ffi::c_int
                        + dy as ::core::ffi::c_int * dy as ::core::ffi::c_int
                        >= (ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int)
                            * (ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                - 14 as ::core::ffi::c_int)
                {
                    (*ElementPtr).c2rust_unnamed_1.mass_points = ((*ElementPtr)
                        .c2rust_unnamed_1
                        .mass_points as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int)) as BYTE;
                } else if (*ElementPtr).c2rust_unnamed_1.mass_points
                    as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int == 0
                {
                    let mut DetectPercent: BYTE = 0;
                    DetectPercent = ((((CreatureData[index as usize].Attributes
                        as ::core::ffi::c_int & 0xc as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        >> 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        * (30 as ::core::ffi::c_int / 6 as ::core::ffi::c_int)) as BYTE;
                    if TFB_Random().wrapping_rem(100 as DWORD) < DetectPercent as DWORD {
                        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
                        (*ElementPtr).c2rust_unnamed_1.mass_points = ((*ElementPtr)
                            .c2rust_unnamed_1
                            .mass_points as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int) as BYTE;
                    }
                }
                if (*ElementPtr).next.location.y as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || (*ElementPtr).next.location.y as ::core::ffi::c_int
                        == ((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
                {
                    (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
                }
                old_angle = (*ElementPtr).velocity.TravelAngle;
                if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
                    (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                        .c2rust_unnamed_3
                        .thrust_wait
                        .wrapping_sub(1);
                    angle = old_angle;
                } else if (*ElementPtr).c2rust_unnamed_1.mass_points
                    as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int == 0
                    || CreatureData[index as usize].Attributes as ::core::ffi::c_int
                        & 0x3 as ::core::ffi::c_int
                        == (2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                {
                    let mut rand_val: COUNT = 0;
                    rand_val = TFB_Random() as COUNT;
                    angle = (rand_val as BYTE as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT;
                    (*ElementPtr).c2rust_unnamed_3.thrust_wait = (((rand_val
                        as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                        + 10 as ::core::ffi::c_int) as BYTE;
                } else if CreatureData[index as usize].Attributes as ::core::ffi::c_int
                    & 0x3 as ::core::ffi::c_int
                    == (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                {
                    if (*ElementPtr).next.location.y as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        || (*ElementPtr).next.location.y as ::core::ffi::c_int
                            == ((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
                    {
                        if angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int != 0
                        {
                            angle = (((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                - angle as ::core::ffi::c_int) as COUNT;
                        } else if old_angle as ::core::ffi::c_int
                            == (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int
                            || old_angle as ::core::ffi::c_int
                                == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 2 as ::core::ffi::c_int)
                        {
                            angle = old_angle;
                        } else {
                            angle = (TFB_Random() & 1 as DWORD)
                                .wrapping_mul(
                                    ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 1 as ::core::ffi::c_int) as DWORD,
                                )
                                .wrapping_sub(
                                    ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 2 as ::core::ffi::c_int) as DWORD,
                                ) as COUNT;
                        }
                        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 5 as BYTE;
                    }
                    angle = (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT;
                }
                match speed as ::core::ffi::c_int {
                    16 => {
                        speed = ((2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                            as COUNT;
                    }
                    32 => {
                        speed = ((2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                            as COUNT;
                    }
                    48 => {
                        speed = (((2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            << 5 as ::core::ffi::c_int) * 9 as ::core::ffi::c_int
                            / 10 as ::core::ffi::c_int) as COUNT;
                    }
                    _ => {}
                }
                SetVelocityComponents(
                    &raw mut (*ElementPtr).velocity,
                    (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE,
                    (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE,
                );
            }
        }
    }
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
        && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && (DisplayLinks >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
            != (*ElementPtr).PrimIndex as ::core::ffi::c_int
    {
        lander_flags = (lander_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    }
}
unsafe extern "C" fn DeltaLanderCrew(mut crew_delta: SIZE, mut which_disaster: COUNT) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if crew_delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        crew_delta = crew_left as SIZE;
        crew_left += 1 as ::core::ffi::c_int;
        s.frame = SetAbsFrameIndex(LanderFrame[0usize], 55 as COUNT);
    } else {
        if crew_left < 1 as ::core::ffi::c_int {
            return;
        }
        shieldHit = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_10::LANDER_SHIELDS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_10::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        shieldHit &= (1 as ::core::ffi::c_int) << which_disaster as ::core::ffi::c_int;
        if shieldHit == 0 || TFB_Random().wrapping_rem(100 as DWORD) >= 95 as DWORD {
            shieldHit = 0 as ::core::ffi::c_int;
            crew_left -= 1;
        }
        damage_index = 6 as ::core::ffi::c_int;
        if shieldHit != 0 {
            return;
        }
        crew_delta = crew_left as SIZE;
        s.frame = SetAbsFrameIndex(LanderFrame[0usize], 56 as COUNT);
        PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_7::LANDER_INJURED.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        );
    }
    s.origin.x = (11 as ::core::ffi::c_int
        + 6 as ::core::ffi::c_int
            * (crew_delta as ::core::ffi::c_int % 6 as ::core::ffi::c_int)) as COORD;
    s.origin.y = (35 as ::core::ffi::c_int
        - 6 as ::core::ffi::c_int
            * (crew_delta as ::core::ffi::c_int / 6 as ::core::ffi::c_int)) as COORD;
    OldContext = SetContext(RadarContext);
    DrawStamp(&raw mut s);
    SetContext(OldContext);
}
unsafe extern "C" fn FillLanderHold(
    mut pPSD: *mut PLANETSIDE_DESC,
    mut scan: COUNT,
    mut NumRetrieved: COUNT,
) {
    let mut start_count: COUNT = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_7::LANDER_PICKUP.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        2 as BYTE,
    );
    if scan as ::core::ffi::c_int
        == PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
    {
        start_count = (*pPSD).BiologicalLevel;
        s.frame = SetAbsFrameIndex(LanderFrame[0usize], 41 as COUNT);
        (*pPSD).BiologicalLevel = ((*pPSD).BiologicalLevel as ::core::ffi::c_int
            + NumRetrieved as ::core::ffi::c_int) as COUNT;
    } else {
        start_count = (*pPSD).ElementLevel;
        (*pPSD).ElementLevel = ((*pPSD).ElementLevel as ::core::ffi::c_int
            + NumRetrieved as ::core::ffi::c_int) as COUNT;
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_10::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_10::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        ) != 0
        {
            start_count = (start_count as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as COUNT;
            NumRetrieved = (((*pPSD).ElementLevel as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) - start_count as ::core::ffi::c_int)
                as COUNT;
        }
        s.frame = SetAbsFrameIndex(LanderFrame[0usize], 43 as COUNT);
    }
    s.origin.x = 0 as COORD;
    s.origin.y = -(start_count as ::core::ffi::c_int) as COORD;
    if start_count as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
        s.frame = IncFrameIndex(s.frame);
    }
    OldContext = SetContext(RadarContext);
    loop {
        let c2rust_fresh0 = NumRetrieved;
        NumRetrieved = NumRetrieved.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let c2rust_fresh1 = start_count;
        start_count = start_count.wrapping_add(1);
        if c2rust_fresh1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            s.frame = IncFrameIndex(s.frame);
        } else {
            s.frame = DecFrameIndex(s.frame);
        }
        DrawStamp(&raw mut s);
        s.origin.y -= 1;
    }
    SetContext(OldContext);
}
unsafe extern "C" fn pickupMineralNode(
    mut pPSD: *mut PLANETSIDE_DESC,
    mut NumRetrieved: COUNT,
    mut ElementPtr: *mut ELEMENT,
    mut LanderControl: *const INTERSECT_CONTROL,
    mut ElementControl: *const INTERSECT_CONTROL,
) -> bool {
    let mut EType: BYTE = 0;
    let mut ch: UNICODE = 0;
    let mut pStr: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    if (*pPSD).ElementLevel as ::core::ffi::c_int
        >= (*pPSD).MaxElementLevel as ::core::ffi::c_int
    {
        PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_7::LANDER_FULL.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        );
        return false;
    }
    if (*pPSD).ElementLevel as ::core::ffi::c_int + NumRetrieved as ::core::ffi::c_int
        > (*pPSD).MaxElementLevel as ::core::ffi::c_int
    {
        NumRetrieved = ((*pPSD).MaxElementLevel as ::core::ffi::c_int
            - (*pPSD).ElementLevel as ::core::ffi::c_int) as COUNT;
    }
    FillLanderHold(
        pPSD,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT,
        NumRetrieved,
    );
    EType = (*ElementPtr).c2rust_unnamed_2.turn_wait;
    (*pPSD)
        .ElementAmounts[(*Elements.offset(EType as isize) as ::core::ffi::c_int
        & 0x7 as ::core::ffi::c_int) as usize] = ((*pPSD)
        .ElementAmounts[(*Elements.offset(EType as isize) as ::core::ffi::c_int
        & 0x7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
        + NumRetrieved as ::core::ffi::c_int) as COUNT;
    (*pPSD).NumFrames = 32 as COUNT;
    sprintf(
        &raw mut (*pPSD).AmountBuf as *mut ::core::ffi::c_char,
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        NumRetrieved as ::core::ffi::c_int,
    );
    pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (EType as ::core::ffi::c_int
                + C2Rust_Unnamed_9::ELEMENTS_STRING_BASE.0 as ::core::ffi::c_int)
                as COUNT,
        ),
    );
    (*pPSD).MineralText[0usize].baseline.x = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        + ((*ElementControl).EndPoint.x as ::core::ffi::c_int
            - (*LanderControl).EndPoint.x as ::core::ffi::c_int)) as COORD;
    (*pPSD).MineralText[0usize].baseline.y = ((ScreenHeight
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
        - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) - 5 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        + ((*ElementControl).EndPoint.y as ::core::ffi::c_int
            - (*LanderControl).EndPoint.y as ::core::ffi::c_int)) as COORD;
    (*pPSD).MineralText[0usize].CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    (*pPSD).MineralText[1usize].pStr = pStr;
    loop {
        let c2rust_fresh2 = pStr;
        pStr = pStr.offset(1);
        ch = *c2rust_fresh2;
        if !(ch as ::core::ffi::c_int != 0
            && ch as ::core::ffi::c_int != ' ' as ::core::ffi::c_int)
        {
            break;
        }
    }
    if ch as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        (*pPSD).MineralText[1usize].CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        (*pPSD).MineralText[2usize].CharCount = 0 as COUNT;
    } else {
        (*pPSD).MineralText[1usize].CharCount = utf8StringCountN(
            (*pPSD).MineralText[1usize].pStr,
            pStr.offset(-(1 as ::core::ffi::c_int as isize)),
        ) as COUNT;
        (*pPSD).MineralText[2usize].pStr = pStr;
        (*pPSD).MineralText[2usize].CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    }
    return true;
}
unsafe extern "C" fn pickupBioNode(
    mut pPSD: *mut PLANETSIDE_DESC,
    mut NumRetrieved: COUNT,
) -> bool {
    if (*pPSD).BiologicalLevel as ::core::ffi::c_int >= 50 as ::core::ffi::c_int {
        PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_7::LANDER_FULL.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        );
        return false;
    }
    if (*pPSD).BiologicalLevel as ::core::ffi::c_int + NumRetrieved as ::core::ffi::c_int
        > 50 as ::core::ffi::c_int
    {
        NumRetrieved = (50 as ::core::ffi::c_int
            - (*pPSD).BiologicalLevel as ::core::ffi::c_int) as COUNT;
    }
    FillLanderHold(
        pPSD,
        PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as COUNT,
        NumRetrieved,
    );
    return true;
}
unsafe extern "C" fn shotCreature(
    mut ElementPtr: *mut ELEMENT,
    mut value: BYTE,
    mut LanderControl: *mut INTERSECT_CONTROL,
    mut pPrim: *mut PRIMITIVE,
) {
    if (*ElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    (*ElementPtr).c2rust_unnamed_0.hit_points = (*ElementPtr)
        .c2rust_unnamed_0
        .hit_points
        .wrapping_sub(1);
    if (*ElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_1.mass_points = value;
        DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Stamp.frame = (*pSolarSysState)
            .PlanetSideFrame[0usize];
    } else if CreatureData[((*ElementPtr).c2rust_unnamed_1.mass_points
            as ::core::ffi::c_int
            & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as usize]
        .Attributes as ::core::ffi::c_int & 0x30 as ::core::ffi::c_int != 0
    {
        let mut angle: COUNT = 0;
        angle = (GetFrameIndex((*LanderControl).IntersectStamp.frame)
            as ::core::ffi::c_int
            - (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
        DeltaVelocityComponents(
            &raw mut (*ElementPtr).velocity,
            (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE,
            (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE,
        );
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
        (*ElementPtr).c2rust_unnamed_1.mass_points = ((*ElementPtr)
            .c2rust_unnamed_1
            .mass_points as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
    }
    (*pPrim).Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
    (*pPrim).color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_7::LANDER_HITS.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        2 as BYTE,
    );
}
unsafe extern "C" fn CheckObjectCollision(mut index: COUNT) {
    let mut LanderControl: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    let mut LanderHandle: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut pPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    let mut pLanderPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    let mut pPSD: *mut PLANETSIDE_DESC = planetSideDesc;
    if index as ::core::ffi::c_int
        != 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        pLanderPrim = (&raw mut DisplayArray as *mut PRIMITIVE).offset(index as isize);
        LanderControl.IntersectStamp = (*pLanderPrim).Object.Stamp;
        index = (*pLanderPrim).Links as UWORD as COUNT;
    } else {
        pLanderPrim = ::core::ptr::null_mut::<PRIMITIVE>();
        LanderControl.IntersectStamp.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        LanderControl.IntersectStamp.origin.y = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        LanderControl.IntersectStamp.frame = LanderFrame[0usize];
        index = (DisplayLinks >> 16 as ::core::ffi::c_int) as UWORD as COUNT;
    }
    LanderControl.EndPoint = LanderControl.IntersectStamp.origin;
    LanderHandle = GetFrameParentDrawable(LanderControl.IntersectStamp.frame);
    while index as ::core::ffi::c_int
        != 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        let mut ElementControl: INTERSECT_CONTROL = INTERSECT_CONTROL {
            last_time_val: 0,
            EndPoint: POINT { x: 0, y: 0 },
            IntersectStamp: STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
        };
        let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        pPrim = (&raw mut DisplayArray as *mut PRIMITIVE).offset(index as isize);
        ElementControl.IntersectStamp = (*pPrim).Object.Stamp;
        ElementControl.EndPoint = ElementControl.IntersectStamp.origin;
        if GetFrameParentDrawable(ElementControl.IntersectStamp.frame) == LanderHandle {
            CheckObjectCollision(index);
        } else if DrawablesIntersect(
            &raw mut LanderControl,
            &raw mut ElementControl,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ) != 0
        {
            hElement = disp_q.head as HELEMENT;
            's_330: while !hElement.is_null() {
                let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
                hNextElement = (*ElementPtr).succ;
                's_92: {
                    if (&raw mut DisplayArray as *mut PRIMITIVE)
                        .offset((*ElementPtr).PrimIndex as isize) == pLanderPrim
                    {
                        (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as ELEMENT_FLAGS;
                        UnlockLink(&raw mut disp_q, hElement);
                    } else if (&raw mut DisplayArray as *mut PRIMITIVE)
                        .offset((*ElementPtr).PrimIndex as isize) != pPrim
                        || (*ElementPtr).playerNr as ::core::ffi::c_int
                            != C2Rust_Unnamed_6::PS_NON_PLAYER.0 as ::core::ffi::c_int
                    {
                        UnlockLink(&raw mut disp_q, hElement);
                    } else {
                        let mut scan: COUNT = 0;
                        let mut NumRetrieved: COUNT = 0;
                        let mut which_node: SIZE = 0;
                        scan = (*ElementPtr).c2rust_unnamed.scan_node as BYTE as COUNT;
                        if pLanderPrim.is_null() {
                            if crew_left == 0 as ::core::ffi::c_int
                                || (*pPSD).InTransit.0 != 0
                            {
                                break 's_330;
                            }
                            if (*ElementPtr).state_flags as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                            {
                                scan = (*ElementPtr).c2rust_unnamed_1.mass_points as COUNT;
                                match scan as ::core::ffi::c_int {
                                    1 | 3 => {
                                        if TFB_Random().wrapping_rem(100 as DWORD) < 25 as DWORD {
                                            DeltaLanderCrew(-1 as SIZE, scan);
                                        }
                                    }
                                    _ => {}
                                }
                                UnlockLink(&raw mut disp_q, hElement);
                                break 's_92;
                            } else {
                                if scan as ::core::ffi::c_int
                                    != PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
                                {
                                    if scan as ::core::ffi::c_int
                                        == PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
                                        && (*ElementPtr).c2rust_unnamed_0.hit_points
                                            as ::core::ffi::c_int != 0
                                    {
                                        let mut danger_vals: [BYTE; 4] = [
                                            0 as BYTE,
                                            6 as BYTE,
                                            13 as BYTE,
                                            26 as BYTE,
                                        ];
                                        let mut creatureIndex: ::core::ffi::c_int = (*ElementPtr)
                                            .c2rust_unnamed_1
                                            .mass_points as ::core::ffi::c_int
                                            & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                                as BYTE as ::core::ffi::c_int);
                                        let mut dangerLevel: ::core::ffi::c_int = (CreatureData[creatureIndex
                                                as usize]
                                            .Attributes as ::core::ffi::c_int
                                            & 0xc0 as ::core::ffi::c_int)
                                            >> 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
                                        if TFB_Random().wrapping_rem(128 as DWORD)
                                            < danger_vals[dangerLevel as usize] as DWORD
                                        {
                                            PlaySound(
                                                SetAbsStringTableIndex(
                                                    LanderSounds,
                                                    C2Rust_Unnamed_7::BIOLOGICAL_DISASTER.0
                                                        as ::core::ffi::c_int as COUNT,
                                                ),
                                                NotPositional(),
                                                ::core::ptr::null_mut::<ELEMENT>(),
                                                2 as BYTE,
                                            );
                                            DeltaLanderCrew(
                                                -1 as SIZE,
                                                C2Rust_Unnamed_7::BIOLOGICAL_DISASTER.0
                                                    as ::core::ffi::c_int as COUNT,
                                            );
                                        }
                                        UnlockLink(&raw mut disp_q, hElement);
                                        break 's_92;
                                    }
                                }
                                NumRetrieved = (*ElementPtr).c2rust_unnamed_1.mass_points
                                    as COUNT;
                            }
                        } else if (*ElementPtr).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                        {
                            UnlockLink(&raw mut disp_q, hElement);
                            break 's_92;
                        } else {
                            let mut value: BYTE = 0;
                            if scan as ::core::ffi::c_int
                                == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
                            {
                                UnlockLink(&raw mut disp_q, hElement);
                                break 's_330;
                            } else if scan as ::core::ffi::c_int
                                == PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
                                && {
                                    value = (CreatureData[((*ElementPtr)
                                            .c2rust_unnamed_1
                                            .mass_points as ::core::ffi::c_int
                                            & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                                as BYTE as ::core::ffi::c_int)) as usize]
                                        .ValueAndHitPoints as ::core::ffi::c_int
                                        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                                        as BYTE;
                                    value as ::core::ffi::c_int != 0
                                }
                            {
                                shotCreature(
                                    ElementPtr,
                                    value,
                                    &raw mut LanderControl,
                                    pPrim,
                                );
                                UnlockLink(&raw mut disp_q, hElement);
                                break 's_330;
                            } else {
                                NumRetrieved = 0 as COUNT;
                            }
                        }
                        if NumRetrieved != 0 {
                            match scan as ::core::ffi::c_int {
                                0 => {
                                    if !pickupMineralNode(
                                        pPSD,
                                        NumRetrieved,
                                        ElementPtr,
                                        &raw mut LanderControl,
                                        &raw mut ElementControl,
                                    ) {
                                        break 's_92;
                                    }
                                }
                                2 => {
                                    if !pickupBioNode(pPSD, NumRetrieved) {
                                        break 's_92;
                                    }
                                }
                                1 | _ => {}
                            }
                        }
                        which_node = (((*ElementPtr).c2rust_unnamed.scan_node
                            as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
                        if callPickupForScanType(
                            pSolarSysState,
                            (*pSolarSysState).pOrbitalDesc,
                            which_node as COUNT,
                            scan as BYTE,
                        ) {
                            setNodeRetrieved(
                                &raw mut (*pSolarSysState).SysInfo.PlanetInfo,
                                scan as BYTE,
                                which_node as BYTE,
                            );
                            setGameState(
                                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                                C2Rust_Unnamed_10::PLANETARY_CHANGE.0 as ::core::ffi::c_int,
                                C2Rust_Unnamed_10::END_PLANETARY_CHANGE.0
                                    as ::core::ffi::c_int,
                                1 as BYTE,
                            );
                            (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                                as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                as ELEMENT_FLAGS;
                        }
                        UnlockLink(&raw mut disp_q, hElement);
                    }
                }
                hElement = hNextElement;
            }
        }
        index = (*pPrim).Links as UWORD as COUNT;
    }
}
unsafe extern "C" fn lightning_process(mut ElementPtr: *mut ELEMENT) {
    let mut pPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    pPrim = (&raw mut DisplayArray as *mut PRIMITIVE)
        .offset((*ElementPtr).PrimIndex as isize);
    if ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        let mut num_frames: COUNT = 0;
        num_frames = (GetFrameCount((*pPrim).Object.Stamp.frame) as ::core::ffi::c_int
            - 7 as ::core::ffi::c_int) as COUNT;
        if GetFrameIndex((*pPrim).Object.Stamp.frame) as ::core::ffi::c_int
            >= num_frames as ::core::ffi::c_int
        {
            (*pPrim).Object.Stamp.frame = IncFrameIndex((*pPrim).Object.Stamp.frame);
        } else {
            let mut s: SIZE = 0;
            s = (7 as ::core::ffi::c_int
                - ((*ElementPtr).c2rust_unnamed_0.cycle as SIZE as ::core::ffi::c_int
                    - (*ElementPtr).c2rust_unnamed.life_span as SIZE
                        as ::core::ffi::c_int)) as SIZE;
            if (s as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                s = 0 as SIZE;
            }
            (*pPrim).color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
            if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == C2Rust_Unnamed_7::LIGHTNING_DISASTER.0 as ::core::ffi::c_int
            {
                if crew_left != 0
                    && TFB_Random().wrapping_rem(100 as DWORD) < 10 as DWORD
                    && (*planetSideDesc).InTransit.0 == 0
                {
                    lander_flags = (lander_flags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE;
                }
                (*ElementPtr).next.location = curLanderLoc;
            }
            (*pPrim).Object.Stamp.frame = SetAbsFrameIndex(
                (*pPrim).Object.Stamp.frame,
                TFB_Random().wrapping_rem(num_frames as DWORD) as COUNT,
            );
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = ((*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait as ::core::ffi::c_int
            + ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
    }
    if (DisplayLinks >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        != (*ElementPtr).PrimIndex as ::core::ffi::c_int
    {
        lander_flags = (lander_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    }
}
unsafe extern "C" fn AddLightning() {
    let mut hLightningElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hLightningElement = AllocElement();
    if !hLightningElement.is_null() {
        let mut rand_val: DWORD = 0;
        let mut LightningElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        LightningElementPtr = LockLink(&raw mut disp_q, hLightningElement)
            as *mut ELEMENT;
        (*LightningElementPtr).playerNr = C2Rust_Unnamed_6::PS_NON_PLAYER.0
            as ::core::ffi::c_int as SIZE;
        (*LightningElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*LightningElementPtr).preprocess_func = Some(
            lightning_process as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        if TFB_Random().wrapping_rem(100 as DWORD) >= 25 as DWORD {
            (*LightningElementPtr).c2rust_unnamed_1.mass_points = 0 as BYTE;
        } else {
            (*LightningElementPtr).c2rust_unnamed_1.mass_points = C2Rust_Unnamed_7::LIGHTNING_DISASTER
                .0 as ::core::ffi::c_int as BYTE;
        }
        rand_val = TFB_Random();
        (*LightningElementPtr).c2rust_unnamed.life_span = (10 as ::core::ffi::c_int
            + (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                % 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
        (*LightningElementPtr).next.location.x = ((curLanderLoc.x as ::core::ffi::c_int
            + (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                - ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - 6 as ::core::ffi::c_int))
            + rand_val as UWORD as BYTE as ::core::ffi::c_int
                % (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int - 12 as ::core::ffi::c_int))
            % ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        (*LightningElementPtr).next.location.y = ((curLanderLoc.y as ::core::ffi::c_int
            + (((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int)
                - ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - 6 as ::core::ffi::c_int))
            + (rand_val as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as BYTE as ::core::ffi::c_int
                % (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int - 12 as ::core::ffi::c_int))
            % ((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int)) as COORD;
        (*LightningElementPtr).c2rust_unnamed_0.cycle = (*LightningElementPtr)
            .c2rust_unnamed
            .life_span;
        DisplayArray[(*LightningElementPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        DisplayArray[(*LightningElementPtr).PrimIndex as usize].color = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        DisplayArray[(*LightningElementPtr).PrimIndex as usize].Object.Stamp.frame = LanderFrame[2usize];
        UnlockLink(&raw mut disp_q, hLightningElement);
        PutQueue(&raw mut disp_q, hLightningElement);
        PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_7::LIGHTNING_DISASTER.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        );
    }
}
unsafe extern "C" fn AddGroundDisaster(mut which_disaster: COUNT) -> HELEMENT {
    let mut hGroundDisasterElement: HELEMENT = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hGroundDisasterElement = AllocElement();
    if !hGroundDisasterElement.is_null() {
        let mut rand_val: DWORD = 0;
        let mut GroundDisasterElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<
            ELEMENT,
        >();
        let mut pPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
        GroundDisasterElementPtr = LockLink(&raw mut disp_q, hGroundDisasterElement)
            as *mut ELEMENT;
        pPrim = (&raw mut DisplayArray as *mut PRIMITIVE)
            .offset((*GroundDisasterElementPtr).PrimIndex as isize);
        (*GroundDisasterElementPtr).c2rust_unnamed_1.mass_points = which_disaster
            as BYTE;
        (*GroundDisasterElementPtr).playerNr = C2Rust_Unnamed_6::PS_NON_PLAYER.0
            as ::core::ffi::c_int as SIZE;
        (*GroundDisasterElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*GroundDisasterElementPtr).preprocess_func = Some(
            object_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        rand_val = TFB_Random();
        (*GroundDisasterElementPtr).next.location.x = ((curLanderLoc.x
            as ::core::ffi::c_int
            + (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                - (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                    / 8 as ::core::ffi::c_int)
            + rand_val as UWORD as ::core::ffi::c_int
                % ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                    / 4 as ::core::ffi::c_int))
            % ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        (*GroundDisasterElementPtr).next.location.y = ((curLanderLoc.y
            as ::core::ffi::c_int
            + (((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int)
                - (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                    / 8 as ::core::ffi::c_int)
            + (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                % ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                    / 4 as ::core::ffi::c_int))
            % ((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int)) as COORD;
        if which_disaster as ::core::ffi::c_int
            == C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
        {
            (*pPrim).Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            (*pPrim).Object.Stamp.frame = LanderFrame[1usize];
            (*GroundDisasterElementPtr).c2rust_unnamed_2.turn_wait = ((2
                as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        } else {
            (*pPrim).Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            (*GroundDisasterElementPtr).c2rust_unnamed_0.facing = (TFB_Random()
                & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD) as COUNT;
            (*pPrim).Object.Stamp.frame = LanderFrame[3usize];
            (*GroundDisasterElementPtr).c2rust_unnamed_2.turn_wait = ((0
                as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        }
        (*GroundDisasterElementPtr).c2rust_unnamed.life_span = (GetFrameCount(
            (*pPrim).Object.Stamp.frame,
        ) as ::core::ffi::c_int
            * (((*GroundDisasterElementPtr).c2rust_unnamed_2.turn_wait
                as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT;
        UnlockLink(&raw mut disp_q, hGroundDisasterElement);
        PutQueue(&raw mut disp_q, hGroundDisasterElement);
    }
    return hGroundDisasterElement;
}
unsafe extern "C" fn BuildObjectList() {
    let mut rand_val: DWORD = 0;
    let mut org: POINT = POINT { x: 0, y: 0 };
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut pPSD: *mut PLANETSIDE_DESC = planetSideDesc;
    DisplayLinks = ((0xffff as ::core::ffi::c_int as COUNT as DWORD)
        << 16 as ::core::ffi::c_int | 0xffff as ::core::ffi::c_int as COUNT as DWORD)
        as PRIM_LINKS;
    lander_flags = (lander_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)) as BYTE;
    rand_val = TFB_Random();
    if ((rand_val >> 16 as ::core::ffi::c_int) as UWORD as BYTE as ::core::ffi::c_int)
        < (*pPSD).FireChance as ::core::ffi::c_int
    {
        AddGroundDisaster(
            C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as COUNT,
        );
        PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        );
    }
    if ((rand_val as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int) < (*pPSD).TectonicsChance as ::core::ffi::c_int
    {
        AddGroundDisaster(
            C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int as COUNT,
        );
    }
    if (rand_val as UWORD as BYTE as ::core::ffi::c_int)
        < (*pPSD).WeatherChance as ::core::ffi::c_int
    {
        AddLightning();
    }
    org = curLanderLoc;
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            hNextElement = (*ElementPtr).succ;
            UnlockLink(&raw mut disp_q, hElement);
            RemoveElement(hElement);
            FreeElement(hElement);
        } else {
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
            {
                (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
                    .c2rust_unnamed
                    .life_span
                    .wrapping_sub(1);
            }
            lander_flags = (lander_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
            if (*ElementPtr).preprocess_func.is_some() {
                Some((*ElementPtr).preprocess_func.expect("non-null function pointer"))
                    .expect("non-null function pointer")(ElementPtr);
            }
            GetNextVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                &raw mut dx,
                &raw mut dy,
                1 as COUNT,
            );
            if dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0 {
                (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                    as ::core::ffi::c_int + dx as ::core::ffi::c_int) as COORD;
                (*ElementPtr).next.location.y = ((*ElementPtr).next.location.y
                    as ::core::ffi::c_int + dy as ::core::ffi::c_int) as COORD;
                if (*ElementPtr).playerNr as ::core::ffi::c_int
                    != C2Rust_Unnamed_6::PS_HUMAN_PLAYER.0 as ::core::ffi::c_int
                {
                    if ((*ElementPtr).next.location.y as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.y = 0 as COORD;
                    } else if (*ElementPtr).next.location.y as ::core::ffi::c_int
                        >= (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.y = (((75 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COORD;
                    }
                }
                if ((*ElementPtr).next.location.x as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                        as ::core::ffi::c_int
                        + ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                        as COORD;
                } else {
                    (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                        as ::core::ffi::c_int
                        % ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                        as COORD;
                }
                if (*ElementPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                {
                    (*ElementPtr).current.location.x = ((*ElementPtr).next.location.x
                        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*ElementPtr).current.location.y = ((*ElementPtr).next.location.y
                        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                }
            }
            let mut pPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
            pPrim = (&raw mut DisplayArray as *mut PRIMITIVE)
                .offset((*ElementPtr).PrimIndex as isize);
            (*pPrim).Object.Stamp.origin.x = ((*ElementPtr).next.location.x
                as ::core::ffi::c_int - org.x as ::core::ffi::c_int
                + (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
            if (*pPrim).Object.Stamp.origin.x as ::core::ffi::c_int
                >= ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - (ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int
            {
                (*pPrim).Object.Stamp.origin.x = ((*pPrim).Object.Stamp.origin.x
                    as ::core::ffi::c_int
                    - ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                    as COORD;
            } else if (*pPrim).Object.Stamp.origin.x as ::core::ffi::c_int
                <= -(((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - (ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int)
            {
                (*pPrim).Object.Stamp.origin.x = ((*pPrim).Object.Stamp.origin.x
                    as ::core::ffi::c_int
                    + ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                    as COORD;
            }
            (*pPrim).Object.Stamp.origin.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int - org.y as ::core::ffi::c_int
                + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
            if lander_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            {
                InsertPrim(
                    &raw mut DisplayLinks,
                    (*ElementPtr).PrimIndex,
                    0xffff as ::core::ffi::c_int as COUNT,
                );
            } else {
                InsertPrim(
                    &raw mut DisplayLinks,
                    (*ElementPtr).PrimIndex,
                    DisplayLinks as COUNT,
                );
            }
            hNextElement = (*ElementPtr).succ;
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
}
unsafe extern "C" fn ScrollPlanetSide(
    mut dx: SIZE,
    mut dy: SIZE,
    mut landingOffset: ::core::ffi::c_int,
) {
    let mut new_pt: POINT = POINT { x: 0, y: 0 };
    let mut lander_s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut shadow_s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut shield_s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    new_pt.y = (curLanderLoc.y as ::core::ffi::c_int + dy as ::core::ffi::c_int)
        as COORD;
    if (new_pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        new_pt.y = 0 as COORD;
        dy = (new_pt.y as ::core::ffi::c_int - curLanderLoc.y as ::core::ffi::c_int)
            as SIZE;
        dx = 0 as SIZE;
        memset(
            &raw mut GlobData.Game_state.velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
    } else if new_pt.y as ::core::ffi::c_int
        > ((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    {
        new_pt.y = (((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as COORD;
        dy = (new_pt.y as ::core::ffi::c_int - curLanderLoc.y as ::core::ffi::c_int)
            as SIZE;
        dx = 0 as SIZE;
        memset(
            &raw mut GlobData.Game_state.velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
    }
    new_pt.x = (curLanderLoc.x as ::core::ffi::c_int + dx as ::core::ffi::c_int)
        as COORD;
    if (new_pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        new_pt.x = (new_pt.x as ::core::ffi::c_int
            + ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
    } else if new_pt.x as ::core::ffi::c_int
        >= (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
    {
        new_pt.x = (new_pt.x as ::core::ffi::c_int
            - ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
    }
    curLanderLoc = new_pt;
    OldContext = SetContext(PlanetContext);
    BatchGraphics();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    ClearDrawable();
    s.origin.x = (-(new_pt.x as ::core::ffi::c_int)
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.origin.y = (-(new_pt.y as ::core::ffi::c_int)
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.frame = (*pSolarSysState).Orbit.TopoZoomFrame;
    DrawStamp(&raw mut s);
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        + ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
    DrawStamp(&raw mut s);
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        - ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    DrawStamp(&raw mut s);
    BuildObjectList();
    DrawBatch(&raw mut DisplayArray as *mut PRIMITIVE, DisplayLinks, 0 as BATCH_FLAGS);
    if crew_left != 0 || damage_index != 0 || explosion_index < 3 as ::core::ffi::c_int {
        lander_s.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        lander_s.origin.y = ((ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) + landingOffset)
            as COORD;
        lander_s.frame = LanderFrame[0usize];
        if landingOffset != 0 as ::core::ffi::c_int {
            shadow_s.origin.x = (lander_s.origin.y as ::core::ffi::c_int
                + (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
            shadow_s.origin.y = lander_s.origin.y;
            shadow_s.frame = lander_s.frame;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            DrawFilledStamp(&raw mut shadow_s);
        }
        if damage_index == 0 as ::core::ffi::c_int {
            DrawStamp(&raw mut lander_s);
        } else if shieldHit != 0 {
            damage_index -= 1;
            if damage_index > 0 as ::core::ffi::c_int {
                shield_s.origin = lander_s.origin;
                shield_s.frame = SetEquFrameIndex(LanderFrame[4usize], lander_s.frame);
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                );
                DrawFilledStamp(&raw mut shield_s);
            }
            DrawStamp(&raw mut lander_s);
        } else {
            damage_index -= 1;
            SetContextForeGroundColor(
                DamageColorCycle(
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    damage_index as COUNT,
                ),
            );
            DrawFilledStamp(&raw mut lander_s);
        }
    }
    if landingOffset == 0 as ::core::ffi::c_int && crew_left != 0
        && DisplayLinks as UWORD as ::core::ffi::c_int
            != 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        CheckObjectCollision(0xffff as ::core::ffi::c_int as COUNT);
    }
    let mut pPSD: *mut PLANETSIDE_DESC = planetSideDesc;
    if (*pPSD).NumFrames != 0 {
        (*pPSD).NumFrames = (*pPSD).NumFrames.wrapping_sub(1);
        SetContextForeGroundColor(
            (*pPSD)
                .ColorCycle[((*pPSD).NumFrames as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as usize],
        );
        (*pPSD).MineralText[0usize].baseline.x = ((*pPSD).MineralText[0usize].baseline.x
            as ::core::ffi::c_int - dx as ::core::ffi::c_int) as COORD;
        (*pPSD).MineralText[0usize].baseline.y = ((*pPSD).MineralText[0usize].baseline.y
            as ::core::ffi::c_int - dy as ::core::ffi::c_int) as COORD;
        font_DrawText((&raw mut (*pPSD).MineralText as *mut TEXT).offset(0isize));
        (*pPSD).MineralText[1usize].baseline.x = (*pPSD).MineralText[0usize].baseline.x;
        (*pPSD).MineralText[1usize].baseline.y = ((*pPSD).MineralText[0usize].baseline.y
            as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as COORD;
        font_DrawText((&raw mut (*pPSD).MineralText as *mut TEXT).offset(1isize));
        (*pPSD).MineralText[2usize].baseline.x = (*pPSD).MineralText[1usize].baseline.x;
        (*pPSD).MineralText[2usize].baseline.y = ((*pPSD).MineralText[1usize].baseline.y
            as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as COORD;
        font_DrawText((&raw mut (*pPSD).MineralText as *mut TEXT).offset(2isize));
    }
    RedrawSurfaceScan(&raw mut new_pt);
    if lander_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
    {
        DeltaLanderCrew(
            -1 as SIZE,
            C2Rust_Unnamed_7::LIGHTNING_DISASTER.0 as ::core::ffi::c_int as COUNT,
        );
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn animationInterframe(
    mut TimeIn: *mut TimeCount,
    mut periods: COUNT,
) {
    while periods != 0 {
        RotatePlanetSphere(BOOLEAN::TRUE);
        SleepThreadUntil(
            (*TimeIn)
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
                ),
        );
        *TimeIn = GetTimeCounter();
        periods = periods.wrapping_sub(1);
    }
}
unsafe extern "C" fn AnimateLaunch(mut farray: FRAME) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut num_frames: COUNT = 0;
    let mut NextTime: TimeCount = 0;
    SetContext(PlanetContext);
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = 0 as COORD;
    r.extent.height = 0 as COORD;
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = farray;
    num_frames = GetFrameCount(s.frame);
    while num_frames != 0 {
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 22 as ::core::ffi::c_int) as TimeCount,
            );
        BatchGraphics();
        RepairBackRect(&raw mut r);
        RotatePlanetSphere(BOOLEAN::FALSE);
        DrawStamp(&raw mut s);
        UnbatchGraphics();
        GetFrameRect(s.frame, &raw mut r);
        s.frame = IncFrameIndex(s.frame);
        SleepThreadUntil(NextTime);
        num_frames = num_frames.wrapping_sub(1);
    }
    RepairBackRect(&raw mut r);
}
unsafe extern "C" fn AnimateLanderWarmup() {
    let mut num_crew: SIZE = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut TimeIn: TimeCount = GetTimeCounter();
    OldContext = SetContext(RadarContext);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex(
        LanderFrame[0usize],
        (((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COUNT,
    );
    DrawStamp(&raw mut s);
    animationInterframe(&raw mut TimeIn, 2 as COUNT);
    num_crew = 0 as SIZE;
    while (num_crew as ::core::ffi::c_int)
        < 6 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int != 0
    {
        animationInterframe(&raw mut TimeIn, 1 as COUNT);
        DeltaSISGauges(-1 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
        DeltaLanderCrew(1 as SIZE, 0 as COUNT);
        num_crew += 1;
    }
    animationInterframe(&raw mut TimeIn, 2 as COUNT);
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_10::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_10::END_IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
    ) != 0
    {
        s.frame = SetAbsFrameIndex(s.frame, 58 as COUNT);
    } else {
        s.frame = SetAbsFrameIndex(
            s.frame,
            (((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as COUNT,
        );
    }
    DrawStamp(&raw mut s);
    animationInterframe(&raw mut TimeIn, 2 as COUNT);
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_10::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_10::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
    ) != 0
    {
        s.frame = SetAbsFrameIndex(s.frame, 57 as COUNT);
    } else {
        s.frame = SetAbsFrameIndex(
            s.frame,
            (((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int) as COUNT,
        );
        DrawStamp(&raw mut s);
        animationInterframe(&raw mut TimeIn, 2 as COUNT);
        s.frame = IncFrameIndex(s.frame);
    }
    DrawStamp(&raw mut s);
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_10::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_10::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
    ) != 0
    {
        animationInterframe(&raw mut TimeIn, 2 as COUNT);
        s.frame = SetAbsFrameIndex(s.frame, 59 as COUNT);
        DrawStamp(&raw mut s);
    }
    animationInterframe(&raw mut TimeIn, 2 as COUNT);
    PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_7::LANDER_DEPARTS.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    );
}
unsafe extern "C" fn InitPlanetSide(mut pt: POINT) {
    pt.x = (pt.x as DWORD)
        .wrapping_sub(
            (64 as DWORD)
                .wrapping_sub(
                    TFB_Random()
                        .wrapping_rem(
                            ((64 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                as DWORD,
                        ),
                ),
        ) as COORD;
    if (pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        pt.x = (pt.x as ::core::ffi::c_int
            + ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
    } else if pt.x as ::core::ffi::c_int
        >= (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
    {
        pt.x = (pt.x as ::core::ffi::c_int
            - ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
    }
    pt.y = (pt.y as DWORD)
        .wrapping_sub(
            (64 as DWORD)
                .wrapping_sub(
                    TFB_Random()
                        .wrapping_rem(
                            ((64 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                as DWORD,
                        ),
                ),
        ) as COORD;
    if (pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        pt.y = 0 as COORD;
    } else if pt.y as ::core::ffi::c_int
        >= (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int
    {
        pt.y = (((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as COORD;
    }
    curLanderLoc = pt;
    SetContext(PlanetContext);
    SetContextFont(TinyFont);
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetContextClipRect(&raw mut r);
    SetTransitionSource(&raw mut r);
    BatchGraphics();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    ClearDrawable();
    s.origin.x = (-(pt.x as ::core::ffi::c_int)
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.origin.y = (-(pt.y as ::core::ffi::c_int)
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.frame = (*pSolarSysState).Orbit.TopoZoomFrame;
    DrawStamp(&raw mut s);
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        + ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
    DrawStamp(&raw mut s);
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        - ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    DrawStamp(&raw mut s);
    ScreenTransition(3 as ::core::ffi::c_int, &raw mut r);
    UnbatchGraphics();
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_10::PLANETARY_LANDING.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_10::END_PLANETARY_LANDING.0 as ::core::ffi::c_int,
        1 as BYTE,
    );
}
unsafe extern "C" fn LanderFire(mut facing: SIZE) {
    let mut hWeaponElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut wdx: SIZE = 0;
    let mut wdy: SIZE = 0;
    let mut WeaponElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut angle: COUNT = 0;
    hWeaponElement = AllocElement();
    if hWeaponElement.is_null() {
        return;
    }
    WeaponElementPtr = LockLink(&raw mut disp_q, hWeaponElement) as *mut ELEMENT;
    (*WeaponElementPtr).playerNr = C2Rust_Unnamed_6::PS_HUMAN_PLAYER.0
        as ::core::ffi::c_int as SIZE;
    (*WeaponElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
    (*WeaponElementPtr).c2rust_unnamed.life_span = 12 as COUNT;
    (*WeaponElementPtr).state_flags = ((1 as ::core::ffi::c_int)
        << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    (*WeaponElementPtr).next.location = curLanderLoc;
    DisplayArray[(*WeaponElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM.0
        as ::core::ffi::c_int as GRAPHICS_PRIM;
    DisplayArray[(*WeaponElementPtr).PrimIndex as usize].Object.Stamp.frame = SetAbsFrameIndex(
        LanderFrame[0usize],
        (facing as ::core::ffi::c_int
            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as COUNT,
    );
    if CurrentInputState
        .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
        as usize][C2Rust_Unnamed_8::KEY_UP.0 as ::core::ffi::c_int as usize] == 0
    {
        wdx = 0 as SIZE;
        wdy = 0 as SIZE;
    } else {
        GetCurrentVelocityComponents(
            &raw mut GlobData.Game_state.velocity,
            &raw mut wdx,
            &raw mut wdy,
        );
    }
    angle = ((facing as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
    SetVelocityComponents(
        &raw mut (*WeaponElementPtr).velocity,
        ((*(&raw mut sinetab as *mut SIZE)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as isize,
            ) as ::core::ffi::c_long
            * ((2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
            + wdx as ::core::ffi::c_int) as SIZE,
        ((*(&raw mut sinetab as *mut SIZE)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as isize,
            ) as ::core::ffi::c_long
            * ((2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
            + wdy as ::core::ffi::c_int) as SIZE,
    );
    UnlockLink(&raw mut disp_q, hWeaponElement);
    InsertQueue(&raw mut disp_q, hWeaponElement, disp_q.head);
    PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_7::LANDER_SHOOTS.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        2 as BYTE,
    );
}
unsafe extern "C" fn LanderExplosion() -> BOOLEAN {
    let mut hExplosionElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ExplosionElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    hExplosionElement = AllocElement();
    if hExplosionElement.is_null() {
        return BOOLEAN::FALSE;
    }
    ExplosionElementPtr = LockLink(&raw mut disp_q, hExplosionElement) as *mut ELEMENT;
    (*ExplosionElementPtr).playerNr = C2Rust_Unnamed_6::PS_HUMAN_PLAYER.0
        as ::core::ffi::c_int as SIZE;
    (*ExplosionElementPtr).c2rust_unnamed_1.mass_points = 0 as BYTE;
    (*ExplosionElementPtr).state_flags = ((1 as ::core::ffi::c_int)
        << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    (*ExplosionElementPtr).next.location = curLanderLoc;
    (*ExplosionElementPtr).preprocess_func = Some(
        object_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*ExplosionElementPtr).c2rust_unnamed_2.turn_wait = ((2 as ::core::ffi::c_int as BYTE
        as ::core::ffi::c_int) << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
        | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    (*ExplosionElementPtr).c2rust_unnamed.life_span = (10 as ::core::ffi::c_int
        * (((*ExplosionElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COUNT;
    DisplayArray[(*ExplosionElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
        .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
    DisplayArray[(*ExplosionElementPtr).PrimIndex as usize].Object.Stamp.frame = SetAbsFrameIndex(
        LanderFrame[0usize],
        46 as COUNT,
    );
    UnlockLink(&raw mut disp_q, hExplosionElement);
    InsertQueue(&raw mut disp_q, hExplosionElement, disp_q.head);
    PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_7::LANDER_DESTROYED.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    );
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoPlanetSide(mut pMS: *mut LanderInputState) -> BOOLEAN {
    let mut dx: SIZE = 0 as SIZE;
    let mut dy: SIZE = 0 as SIZE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_11::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if (*pMS).Initialized.0 == 0 {
        let mut landerSpeedNumer: COUNT = 0;
        let mut angle: COUNT = 0;
        (*pMS).Initialized = BOOLEAN::TRUE;
        turn_wait = 0 as ::core::ffi::c_int;
        weapon_wait = 0 as ::core::ffi::c_int;
        angle = ((GetFrameIndex(LanderFrame[0usize]) as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
        landerSpeedNumer = (if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_10::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_10::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int != 0
        {
            (2 as ::core::ffi::c_int * 14 as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int
        } else {
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int
        }) as COUNT;
        SetVelocityComponents(
            &raw mut GlobData.Game_state.velocity,
            ((*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long * landerSpeedNumer as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int) as SIZE,
            ((*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long * landerSpeedNumer as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int) as SIZE,
        );
        return BOOLEAN::TRUE;
    } else if crew_left != 0
        && (CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_8::KEY_ESCAPE.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0
                as usize][C2Rust_Unnamed_8::KEY_SPECIAL.0 as ::core::ffi::c_int as usize]
                != 0 || (*planetSideDesc).InTransit.0 != 0)
    {
        return BOOLEAN::FALSE
    } else if crew_left == 0 && damage_index == 0 {
        if explosion_index
            > 10 as ::core::ffi::c_int
                + 840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / (840 as ::core::ffi::c_int / 35 as ::core::ffi::c_int)
        {
            return BOOLEAN::FALSE;
        }
        if explosion_index > 10 as ::core::ffi::c_int {
            explosion_index += 1;
        } else if explosion_index == 0 as ::core::ffi::c_int {
            if LanderExplosion().0 != 0 {
                explosion_index += 1;
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"DoPlanetSide(): could not allocate explosion element!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
    } else if crew_left != 0 {
        let mut index: SIZE = GetFrameIndex(LanderFrame[0usize]) as SIZE;
        if turn_wait != 0 {
            turn_wait -= 1;
        } else if CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_8::KEY_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0
                as usize][C2Rust_Unnamed_8::KEY_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
        {
            let mut landerSpeedNumer_0: COUNT = 0;
            let mut angle_0: COUNT = 0;
            if CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0 as usize][C2Rust_Unnamed_8::KEY_LEFT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                index -= 1;
            } else {
                index += 1;
            }
            index = (index as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT as SIZE;
            LanderFrame[0usize] = SetAbsFrameIndex(LanderFrame[0usize], index as COUNT);
            angle_0 = ((index as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
            landerSpeedNumer_0 = (if getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_10::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_10::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int != 0
            {
                (2 as ::core::ffi::c_int * 14 as ::core::ffi::c_int)
                    << 5 as ::core::ffi::c_int
            } else {
                (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                    << 5 as ::core::ffi::c_int
            }) as COUNT;
            SetVelocityComponents(
                &raw mut GlobData.Game_state.velocity,
                ((*(&raw mut sinetab as *mut SIZE)
                    .offset(
                        (angle_0 as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as isize,
                    ) as ::core::ffi::c_long * landerSpeedNumer_0 as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as SIZE,
                ((*(&raw mut sinetab as *mut SIZE)
                    .offset(
                        (angle_0 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as isize,
                    ) as ::core::ffi::c_long * landerSpeedNumer_0 as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as SIZE,
            );
            turn_wait = 2 as ::core::ffi::c_int;
        }
        if CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_8::KEY_UP.0 as ::core::ffi::c_int as usize] == 0
        {
            dx = 0 as SIZE;
            dy = 0 as SIZE;
        } else {
            GetNextVelocityComponents(
                &raw mut GlobData.Game_state.velocity,
                &raw mut dx,
                &raw mut dy,
                1 as COUNT,
            );
        }
        if weapon_wait != 0 {
            weapon_wait -= 1;
        } else if CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_8::KEY_WEAPON.0 as ::core::ffi::c_int as usize] != 0
        {
            LanderFire(index);
            weapon_wait = 15 as ::core::ffi::c_int;
            if getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_10::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_10::END_IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
            ) != 0
            {
                weapon_wait >>= 1 as ::core::ffi::c_int;
            }
        }
    }
    ScrollPlanetSide(dx, dy, 0 as ::core::ffi::c_int);
    SleepThreadUntil((*pMS).NextTime);
    (*pMS).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 35 as ::core::ffi::c_int) as TimeCount,
        );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn FreeLanderData() {
    let mut i: COUNT = 0;
    let mut landerFrameCount: COUNT = 0;
    if LanderFrame[0usize].is_null() {
        return;
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        DestroyMusic(OrbitMusic[i as usize]);
        OrbitMusic[i as usize] = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        i = i.wrapping_add(1);
    }
    DestroySound(ReleaseStringTable(LanderSounds));
    LanderSounds = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    landerFrameCount = ::core::mem::size_of::<[FRAME; 8]>()
        .wrapping_div(::core::mem::size_of::<FRAME>()) as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < landerFrameCount as ::core::ffi::c_int {
        DestroyDrawable(ReleaseDrawable(LanderFrame[i as usize]));
        LanderFrame[i as usize] = ::core::ptr::null_mut::<FRAME_DESC>();
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn LoadLanderData() {
    if !LanderFrame[0usize].is_null() {
        return;
    }
    LanderFrame[0usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.lander\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[1usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.quake\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[2usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.lightning\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[3usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.lavaspot\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[4usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.landershield\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[5usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.landerlaunch\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[6usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.landerreturn\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderFrame[7usize] = CaptureDrawable(
        LoadGraphicInstance(b"graphics.orbview\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    LanderSounds = CaptureStringTable(
        LoadSoundInstance(b"sounds.lander\0".as_ptr() as RESOURCE),
    ) as SOUND;
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        OrbitMusic[i as usize] = load_orbit_theme(i as BYTE);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetPlanetMusic(mut planet_type: BYTE) {
    LanderMusic = OrbitMusic[(planet_type as ::core::ffi::c_int
        % 5 as ::core::ffi::c_int) as usize];
}
unsafe extern "C" fn ReturnToOrbit() {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    OldContext = SetContext(PlanetContext);
    GetContextClipRect(&raw mut r);
    SetTransitionSource(&raw mut r);
    BatchGraphics();
    DrawStarBackGround();
    DrawPlanetSurfaceBorder();
    RedrawSurfaceScan(::core::ptr::null::<POINT>());
    ScreenTransition(3 as ::core::ffi::c_int, &raw mut r);
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn IdlePlanetSide(
    mut inputState: *mut LanderInputState,
    mut howLong: TimeCount,
) {
    let mut TimeOut: TimeCount = GetTimeCounter().wrapping_add(howLong);
    while GetTimeCounter() < TimeOut {
        ScrollPlanetSide(
            0 as SIZE,
            0 as SIZE,
            -((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int
                - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                - 5 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int),
        );
        SleepThreadUntil((*inputState).NextTime);
        (*inputState).NextTime = (*inputState)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 35 as ::core::ffi::c_int) as TimeCount,
            );
    }
}
unsafe extern "C" fn LandingTakeoffSequence(
    mut inputState: *mut LanderInputState,
    mut landing: BOOLEAN,
) {
    let mut landingOfs: [::core::ffi::c_int; 20] = [0; 20];
    let mut start: ::core::ffi::c_int = 0;
    let mut end: ::core::ffi::c_int = 0;
    let mut delta: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = 0;
    delta = 0 as ::core::ffi::c_int;
    index = 0 as ::core::ffi::c_int;
    while index < 20 as ::core::ffi::c_int
        && delta
            < (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int
                - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                - 5 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int
    {
        delta += index + 1 as ::core::ffi::c_int;
        landingOfs[index as usize] = -delta;
        index += 1;
    }
    (delta
        >= (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int
            + 10 as ::core::ffi::c_int
        && !(b"Increase MAX_OFFSETS!\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        || {
            __assert_fail(
                b"delta >= DISTANCE_COVERED && \"Increase MAX_OFFSETS!\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/lander.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1776 as ::core::ffi::c_int,
                b"LandingTakeoffSequence\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if landing.0 != 0 {
        start = index - 1 as ::core::ffi::c_int;
        end = -1 as ::core::ffi::c_int;
        delta = -1 as ::core::ffi::c_int;
    } else {
        start = 0 as ::core::ffi::c_int;
        end = index;
        delta = 1 as ::core::ffi::c_int;
    }
    if landing.0 != 0 {
        IdlePlanetSide(inputState, 840 as TimeCount);
    }
    index = start;
    while index != end {
        ScrollPlanetSide(0 as SIZE, 0 as SIZE, landingOfs[index as usize]);
        SleepThreadUntil((*inputState).NextTime);
        (*inputState).NextTime = (*inputState)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 35 as ::core::ffi::c_int) as TimeCount,
            );
        index += delta;
    }
    if landing.0 == 0 {
        IdlePlanetSide(
            inputState,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetLanderTakeoff() {
    (!planetSideDesc.is_null()
        || {
            __assert_fail(
                b"planetSideDesc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/lander.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1809 as ::core::ffi::c_int,
                b"SetLanderTakeoff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !planetSideDesc.is_null() {
        (*planetSideDesc).InTransit = BOOLEAN::TRUE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn KillLanderCrewSeq(
    mut numKilled: COUNT,
    mut period: DWORD,
) -> bool {
    let mut TimeOut: TimeCount = 0;
    let mut i: COUNT = 0;
    TimeOut = GetTimeCounter();
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numKilled as ::core::ffi::c_int && crew_left != 0 {
        TimeOut = TimeOut.wrapping_add(period);
        DeltaLanderCrew(
            -1 as SIZE,
            C2Rust_Unnamed_7::LANDER_INJURED.0 as ::core::ffi::c_int as COUNT,
        );
        SleepThreadUntil(TimeOut);
        i = i.wrapping_add(1);
    }
    return crew_left > 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetThermalHazardRating(
    mut temp: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    static mut tempBreakpoints: [::core::ffi::c_int; 7] = [
        50 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        150 as ::core::ffi::c_int,
        250 as ::core::ffi::c_int,
        350 as ::core::ffi::c_int,
        550 as ::core::ffi::c_int,
        800 as ::core::ffi::c_int,
    ];
    let numBreakpoints: size_t = ::core::mem::size_of::<[::core::ffi::c_int; 7]>()
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>());
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as size_t) < numBreakpoints {
        if temp < tempBreakpoints[i as usize] {
            return i;
        }
        i = i.wrapping_add(1);
    }
    return numBreakpoints as ::core::ffi::c_uint;
}
unsafe extern "C" fn GetHazardChance(
    mut hazardType: ::core::ffi::c_int,
    mut HazardRating: ::core::ffi::c_uint,
) -> BYTE {
    static mut TectonicsChanceTab: [BYTE; 8] = [
        (0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (16 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (32 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
    ];
    static mut WeatherChanceTab: [BYTE; 8] = [
        (0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (3 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (6 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (24 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
    ];
    static mut FireChanceTab: [BYTE; 8] = [
        (0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (24 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (48 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
    ];
    match hazardType {
        1 => return TectonicsChanceTab[HazardRating as usize],
        2 => return WeatherChanceTab[HazardRating as usize],
        3 => return FireChanceTab[HazardRating as usize],
        _ => {}
    }
    return 0 as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn PlanetSide(mut planetLoc: POINT) {
    let mut index: SIZE = 0;
    let mut landerInputState: LanderInputState = LanderInputState {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        NextTime: 0,
    };
    let mut PSD: PLANETSIDE_DESC = PLANETSIDE_DESC {
        InTransit: BOOLEAN::FALSE,
        ElementLevel: 0,
        MaxElementLevel: 0,
        BiologicalLevel: 0,
        ElementAmounts: [0; 8],
        NumFrames: 0,
        AmountBuf: [0; 40],
        MineralText: [TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        }; 3],
        ColorCycle: [Color { r: 0, g: 0, b: 0, a: 0 }; 16],
        TectonicsChance: 0,
        WeatherChance: 0,
        FireChance: 0,
    };
    memset(
        &raw mut PSD as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PLANETSIDE_DESC>(),
    );
    PSD.InTransit = BOOLEAN::TRUE;
    PSD.TectonicsChance = GetHazardChance(
        C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int,
        (*pSolarSysState).SysInfo.PlanetInfo.Tectonics as ::core::ffi::c_uint,
    );
    PSD.WeatherChance = GetHazardChance(
        C2Rust_Unnamed_7::LIGHTNING_DISASTER.0 as ::core::ffi::c_int,
        (*pSolarSysState).SysInfo.PlanetInfo.Weather as ::core::ffi::c_uint,
    );
    PSD.FireChance = GetHazardChance(
        C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int,
        GetThermalHazardRating(
            (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
        ),
    );
    PSD.ElementLevel = (GetStorageBayCapacity() as ::core::ffi::c_int
        - GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int) as COUNT;
    PSD.MaxElementLevel = 50 as COUNT;
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_10::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_10::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
    ) != 0
    {
        PSD.MaxElementLevel = ((PSD.MaxElementLevel as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as COUNT;
    }
    if (PSD.ElementLevel as ::core::ffi::c_int)
        < PSD.MaxElementLevel as ::core::ffi::c_int
    {
        PSD.MaxElementLevel = PSD.ElementLevel;
    }
    PSD.ElementLevel = 0 as COUNT;
    PSD.MineralText[0usize].align = TEXT_ALIGN::ALIGN_CENTER;
    PSD.MineralText[0usize].pStr = &raw mut PSD.AmountBuf as *mut UNICODE;
    PSD.MineralText[1usize] = PSD.MineralText[0usize];
    PSD.MineralText[2usize] = PSD.MineralText[1usize];
    PSD.ColorCycle[0usize] = colorFromRgb15(0x1f as BYTE, 0x3 as BYTE, 0 as BYTE);
    PSD.ColorCycle[1usize] = colorFromRgb15(0x1f as BYTE, 0xa as BYTE, 0 as BYTE);
    PSD.ColorCycle[2usize] = colorFromRgb15(0x1f as BYTE, 0x11 as BYTE, 0 as BYTE);
    PSD.ColorCycle[3usize] = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    index = 4 as SIZE;
    while (index as ::core::ffi::c_int)
        < (32 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) - 4 as ::core::ffi::c_int
    {
        PSD.ColorCycle[index as usize] = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        index += 1;
    }
    PSD
        .ColorCycle[((32 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 4 as ::core::ffi::c_int) as usize] = colorFromRgb15(
        0x1f as BYTE,
        0x1f as BYTE,
        0 as BYTE,
    );
    PSD
        .ColorCycle[((32 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 3 as ::core::ffi::c_int) as usize] = colorFromRgb15(
        0x1f as BYTE,
        0x11 as BYTE,
        0 as BYTE,
    );
    PSD
        .ColorCycle[((32 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 2 as ::core::ffi::c_int) as usize] = colorFromRgb15(
        0x1f as BYTE,
        0xa as BYTE,
        0 as BYTE,
    );
    PSD
        .ColorCycle[((32 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as usize] = colorFromRgb15(
        0x1f as BYTE,
        0x3 as BYTE,
        0 as BYTE,
    );
    planetSideDesc = &raw mut PSD;
    index = (TFB_Random()
        & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD) as COUNT as SIZE;
    LanderFrame[0usize] = SetAbsFrameIndex(LanderFrame[0usize], index as COUNT);
    crew_left = 0 as ::core::ffi::c_int;
    damage_index = 0 as ::core::ffi::c_int;
    explosion_index = 0 as ::core::ffi::c_int;
    AnimateLanderWarmup();
    AnimateLaunch(LanderFrame[5usize]);
    InitPlanetSide(planetLoc);
    landerInputState.NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 35 as ::core::ffi::c_int) as TimeCount,
        );
    LandingTakeoffSequence(&raw mut landerInputState, BOOLEAN::TRUE);
    PSD.InTransit = BOOLEAN::FALSE;
    landerInputState.Initialized = BOOLEAN::FALSE;
    landerInputState.InputFunc = Some(
        DoPlanetSide as unsafe extern "C" fn(*mut LanderInputState) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut LanderInputState) -> BOOLEAN>;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    DoInput(&raw mut landerInputState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_11::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if crew_left == 0 as ::core::ffi::c_int {
            GlobData.SIS_state.NumLanders = GlobData
                .SIS_state
                .NumLanders
                .wrapping_sub(1);
            DrawLanders();
            ReturnToOrbit();
        } else {
            PSD.InTransit = BOOLEAN::TRUE;
            PlaySound(
                SetAbsStringTableIndex(
                    LanderSounds,
                    C2Rust_Unnamed_7::LANDER_RETURNS.0 as ::core::ffi::c_int as COUNT,
                ),
                NotPositional(),
                ::core::ptr::null_mut::<ELEMENT>(),
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
            );
            LandingTakeoffSequence(&raw mut landerInputState, BOOLEAN::FALSE);
            ReturnToOrbit();
            AnimateLaunch(LanderFrame[6usize]);
            DeltaSISGauges(crew_left as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
            if PSD.ElementLevel != 0 {
                index = 0 as SIZE;
                while (index as ::core::ffi::c_int)
                    < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
                {
                    GlobData.SIS_state.ElementAmounts[index as usize] = (GlobData
                        .SIS_state
                        .ElementAmounts[index as usize] as ::core::ffi::c_int
                        + PSD.ElementAmounts[index as usize] as ::core::ffi::c_int)
                        as COUNT;
                    GlobData.SIS_state.TotalElementMass = (GlobData
                        .SIS_state
                        .TotalElementMass as ::core::ffi::c_int
                        + PSD.ElementAmounts[index as usize] as ::core::ffi::c_int)
                        as COUNT;
                    index += 1;
                }
                DrawStorageBays(BOOLEAN::FALSE);
            }
            GlobData.SIS_state.TotalBioMass = (GlobData.SIS_state.TotalBioMass
                as ::core::ffi::c_int + PSD.BiologicalLevel as ::core::ffi::c_int)
                as COUNT;
        }
    }
    planetSideDesc = ::core::ptr::null_mut::<PLANETSIDE_DESC>();
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).succ;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
        {
            UnlockLink(&raw mut disp_q, hElement);
            RemoveElement(hElement);
            FreeElement(hElement);
        } else {
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
    memset(
        &raw mut GlobData.Game_state.velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
}
#[no_mangle]
pub unsafe extern "C" fn InitLander(mut LanderFlags: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(RadarContext);
    BatchGraphics();
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as COORD;
    r.extent.height = 53 as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawFilledRectangle(&raw mut r);
    if GlobData.SIS_state.NumLanders as ::core::ffi::c_int != 0
        || LanderFlags as ::core::ffi::c_int != 0
    {
        let mut ShieldFlags: BYTE = 0;
        let mut capacity_shift: BYTE = 0;
        let mut free_space: COUNT = 0;
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = SetAbsFrameIndex(
            LanderFrame[0usize],
            ((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as COUNT,
        );
        DrawStamp(&raw mut s);
        if LanderFlags as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            ShieldFlags = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_10::LANDER_SHIELDS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_10::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
            );
            capacity_shift = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_10::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_10::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
            );
        } else {
            ShieldFlags = (LanderFlags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int)
                    << C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_7::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_7::LIGHTNING_DISASTER.0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int))
                as ::core::ffi::c_uchar as BYTE;
            s.frame = IncFrameIndex(s.frame);
            DrawStamp(&raw mut s);
            if LanderFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int != 0
            {
                s.frame = SetAbsFrameIndex(s.frame, 57 as COUNT);
            } else {
                s.frame = SetAbsFrameIndex(
                    s.frame,
                    (((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int) as COUNT,
                );
                DrawStamp(&raw mut s);
                s.frame = IncFrameIndex(s.frame);
            }
            DrawStamp(&raw mut s);
            if LanderFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int == 0
            {
                capacity_shift = 0 as BYTE;
            } else {
                capacity_shift = 1 as BYTE;
                s.frame = SetAbsFrameIndex(s.frame, 59 as COUNT);
                DrawStamp(&raw mut s);
            }
            if LanderFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int != 0
            {
                s.frame = SetAbsFrameIndex(s.frame, 58 as COUNT);
            } else {
                s.frame = SetAbsFrameIndex(
                    s.frame,
                    (((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as COUNT,
                );
            }
            DrawStamp(&raw mut s);
        }
        free_space = (GetStorageBayCapacity() as ::core::ffi::c_int
            - GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int) as COUNT;
        if (free_space as ::core::ffi::c_int)
            < (50 as ::core::ffi::c_int) << capacity_shift as ::core::ffi::c_int
        {
            r.corner.x = 1 as COORD;
            r.extent.width = 4 as COORD;
            r.extent.height = (50 as ::core::ffi::c_int
                - (free_space as ::core::ffi::c_int
                    >> capacity_shift as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
                as COORD;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            DrawFilledRectangle(&raw mut r);
        }
        s.frame = SetAbsFrameIndex(s.frame, 37 as COUNT);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int != 0
        {
            DrawStamp(&raw mut s);
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_7::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int != 0
        {
            DrawStamp(&raw mut s);
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_7::LIGHTNING_DISASTER.0 as ::core::ffi::c_int != 0
        {
            DrawStamp(&raw mut s);
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_7::LAVASPOT_DISASTER.0 as ::core::ffi::c_int != 0
        {
            DrawStamp(&raw mut s);
        }
    }
    UnbatchGraphics();
}
