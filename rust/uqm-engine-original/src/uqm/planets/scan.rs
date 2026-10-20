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

// Rust translation of src/uqm/planets/scan.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    static mut Elements: *const BYTE;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn CopyContextRect(area: *const RECT) -> DRAWABLE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
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
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn GetTimeCounter() -> TimeCount;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn PlanetSide(planetLoc_0: POINT);
    fn object_animation(ElementPtr: *mut ELEMENT);
    static CreatureData: [LIFEFORM_DESC; 0];
    fn TFB_Random() -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut PlanetContext: CONTEXT;
    fn DrawPlanet(dy: ::core::ffi::c_int, tintColor: Color);
    fn DrawStarBackGround();
    fn InitLander(LanderFlags: BYTE);
    fn DrawDefaultPlanetSphere();
    fn DrawScannedObjects(Reversed: BOOLEAN);
    fn GetNamedPlanetaryBody() -> *mut UNICODE;
    fn SaveSolarSysLocation();
    static mut SpaceJunkFrame: FRAME;
    static mut EncounterGroup: BYTE;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn WaitForAnyButtonUntil(
        newButton: BOOLEAN,
        timeOut: TimeCount,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn load_life_form(selector: BYTE) -> FRAME;
    fn CloneShipFragment(
        shipIndex: COUNT,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    static mut GameStrings: STRING;
    fn InitDisplayList();
    static mut Screen: FRAME;
    static mut MiscDataFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    static mut NextActivity: ACTIVITY;
    static mut optWhichCoarseScan: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    fn DrawFromExtraScreen(r: *mut RECT);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn ReinitQueue(pq: *mut QUEUE);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SIZE,
        resunit_delta: ::core::ffi::c_int,
    );
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
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
pub type Mutex = *mut ::core::ffi::c_void;
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
pub type TIME_VALUE = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
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
    pub image: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
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
    pub c2rust_unnamed: C2Rust_Unnamed_4,
    pub c2rust_unnamed_0: C2Rust_Unnamed_3,
    pub c2rust_unnamed_1: C2Rust_Unnamed_2,
    pub c2rust_unnamed_2: C2Rust_Unnamed_1,
    pub c2rust_unnamed_3: C2Rust_Unnamed_0,
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
pub union C2Rust_Unnamed_0 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const PS_HUMAN_PLAYER: Self = Self(0);
    pub const PS_NON_PLAYER: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LIFEFORM_DESC {
    pub Attributes: BYTE,
    pub ValueAndHitPoints: BYTE,
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
pub struct menu_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>,
    pub Initialized: SIZE,
    pub CurState: BYTE,
    pub CurFrame: FRAME,
    pub CurString: STRING,
    pub first_item: POINT,
    pub delta_item: SIZE,
    pub ModuleFrame: FRAME,
    pub flash_rect0: RECT,
    pub flash_rect1: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const PM_SCAN: Self = Self(0);
    pub const PM_STARMAP: Self = Self(1);
    pub const PM_DEVICES: Self = Self(2);
    pub const PM_CARGO: Self = Self(3);
    pub const PM_ROSTER: Self = Self(4);
    pub const PM_GAME_MENU: Self = Self(5);
    pub const PM_NAVIGATE: Self = Self(6);
    pub const PM_MIN_SCAN: Self = Self(7);
    pub const PM_ENE_SCAN: Self = Self(8);
    pub const PM_BIO_SCAN: Self = Self(9);
    pub const PM_EXIT_SCAN: Self = Self(10);
    pub const PM_AUTO_SCAN: Self = Self(11);
    pub const PM_LAUNCH_LANDER: Self = Self(12);
    pub const PM_SAVE_GAME: Self = Self(13);
    pub const PM_LOAD_GAME: Self = Self(14);
    pub const PM_QUIT_GAME: Self = Self(15);
    pub const PM_CHANGE_SETTINGS: Self = Self(16);
    pub const PM_EXIT_GAME_MENU: Self = Self(17);
    pub const PM_CONVERSE: Self = Self(18);
    pub const PM_ATTACK: Self = Self(19);
    pub const PM_ENCOUNTER_GAME_MENU: Self = Self(20);
    pub const PM_FUEL: Self = Self(21);
    pub const PM_MODULE: Self = Self(22);
    pub const PM_OUTFIT_GAME_MENU: Self = Self(23);
    pub const PM_EXIT_OUTFIT: Self = Self(24);
    pub const PM_CREW: Self = Self(25);
    pub const PM_SHIPYARD_GAME_MENU: Self = Self(26);
    pub const PM_EXIT_SHIPYARD: Self = Self(27);
    pub const PM_SOUND_ON: Self = Self(28);
    pub const PM_SOUND_OFF: Self = Self(29);
    pub const PM_MUSIC_ON: Self = Self(30);
    pub const PM_MUSIC_OFF: Self = Self(31);
    pub const PM_CYBORG_OFF: Self = Self(32);
    pub const PM_CYBORG_NORMAL: Self = Self(33);
    pub const PM_CYBORG_DOUBLE: Self = Self(34);
    pub const PM_CYBORG_SUPER: Self = Self(35);
    pub const PM_CHANGE_CAPTAIN: Self = Self(36);
    pub const PM_CHANGE_SHIP: Self = Self(37);
    pub const PM_EXIT_SETTINGS: Self = Self(38);
    pub const PM_ALT_SCAN: Self = Self(39);
    pub const PM_ALT_STARMAP: Self = Self(40);
    pub const PM_ALT_MANIFEST: Self = Self(41);
    pub const PM_ALT_GAME_MENU: Self = Self(42);
    pub const PM_ALT_NAVIGATE: Self = Self(43);
    pub const PM_ALT_CARGO: Self = Self(44);
    pub const PM_ALT_DEVICES: Self = Self(45);
    pub const PM_ALT_ROSTER: Self = Self(46);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(47);
    pub const PM_ALT_MSCAN: Self = Self(48);
    pub const PM_ALT_ESCAN: Self = Self(49);
    pub const PM_ALT_BSCAN: Self = Self(50);
    pub const PM_ALT_ASCAN: Self = Self(51);
    pub const PM_ALT_DISPATCH: Self = Self(52);
    pub const PM_ALT_EXIT_SCAN: Self = Self(53);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
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
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
}
pub type MENU_SOUND_FLAGS = UWORD;
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PICK_PLANET_STATE {
    pub success: bool,
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
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
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
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
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
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_13 {
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
pub struct ScanMenuItems(pub ::core::ffi::c_uint);
impl ScanMenuItems {
    pub const EXIT_SCAN: Self = Self(3);
    pub const AUTO_SCAN: Self = Self(4);
    pub const DISPATCH_SHUTTLE: Self = Self(5);
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
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
}
static mut ScanContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
static mut planetLoc: POINT = POINT { x: 0, y: 0 };
static mut cursorRect: RECT = RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
};
static mut eraseFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub unsafe extern "C" fn RepairBackRect(mut pRect: *mut RECT) {
    let mut new_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut old_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetContextClipRect(&raw mut old_r);
    new_r.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        + old_r.corner.x as ::core::ffi::c_int) as COORD;
    new_r.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        + old_r.corner.y as ::core::ffi::c_int) as COORD;
    new_r.extent = (*pRect).extent;
    new_r.extent.height = (new_r.extent.height as ::core::ffi::c_int
        + (new_r.corner.y as ::core::ffi::c_int & 1 as ::core::ffi::c_int)) as COORD;
    new_r.corner.y = (new_r.corner.y as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
        as COORD;
    DrawFromExtraScreen(&raw mut new_r);
}
unsafe extern "C" fn EraseCoarseScan() {
    SetContext(PlanetContext);
    BatchGraphics();
    DrawStarBackGround();
    DrawDefaultPlanetSphere();
    UnbatchGraphics();
}
unsafe extern "C" fn PrintScanTitlePC(
    mut t: *mut TEXT,
    mut r: *mut RECT,
    mut txt: *const ::core::ffi::c_char,
    mut xpos: ::core::ffi::c_int,
) {
    (*t).baseline.x = xpos as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE));
    (*t).pStr = txt as *const UNICODE;
    (*t).CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(t);
    TextRect(t, r, ::core::ptr::null_mut::<BYTE>());
    (*t).baseline.x = ((*t).baseline.x as ::core::ffi::c_int
        + (*r).extent.width as ::core::ffi::c_int) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE));
}
unsafe extern "C" fn MakeScanValue(
    mut buf: *mut UNICODE,
    mut val: ::core::ffi::c_long,
    mut extra: *const UNICODE,
) {
    if val
        >= (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as ::core::ffi::c_long
    {
        sprintf(
            buf,
            b"%ld.%ld%s\0".as_ptr() as *const ::core::ffi::c_char,
            val / 100 as ::core::ffi::c_long,
            val / 10 as ::core::ffi::c_long % 10 as ::core::ffi::c_long,
            extra,
        );
    } else {
        sprintf(
            buf,
            b"%ld.%02ld%s\0".as_ptr() as *const ::core::ffi::c_char,
            val / 100 as ::core::ffi::c_long,
            val % 100 as ::core::ffi::c_long,
            extra,
        );
    };
}
unsafe extern "C" fn GetPlanetTitle(mut buf: *mut UNICODE, mut bufsize: COUNT) {
    let mut val: ::core::ffi::c_int = 0;
    let mut named: *mut UNICODE = GetNamedPlanetaryBody();
    if !named.is_null() {
        utf8StringCopy(buf, bufsize as size_t, named);
        return;
    }
    val = (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int);
    if val >= C2Rust_Unnamed_8::FIRST_GAS_GIANT.0 as ::core::ffi::c_int {
        sprintf(
            buf,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + 51 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        sprintf(
            buf,
            b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + val) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    };
}
unsafe extern "C" fn PrintCoarseScanPC() {
    let mut val: SDWORD = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut buf: [UNICODE; 200] = [0; 200];
    GetPlanetTitle(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 200]>() as COUNT,
    );
    SetContext(PlanetContext);
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = 13 as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE));
    SetContextFont(MicroFont);
    font_DrawText(&raw mut t);
    SetContextFont(TinyFont);
    t.baseline.y = 40 as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
            ),
        ),
        5 as ::core::ffi::c_int,
    );
    val = (((*pSolarSysState).SysInfo.PlanetInfo.PlanetToSunDist as ::core::ffi::c_long
        * 100 as ::core::ffi::c_long
        + ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        / ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        5 as ::core::ffi::c_int,
    );
    if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0xffff as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        val = (((*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
            * 100 as ::core::ffi::c_int
            + (50 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 50 as ::core::ffi::c_int) as SDWORD;
        MakeScanValue(
            &raw mut buf as *mut UNICODE,
            val as ::core::ffi::c_long,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 5 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    }
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        5 as ::core::ffi::c_int,
    );
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\xC2\xB0 c\0".as_ptr() as *const ::core::ffi::c_char,
        (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        5 as ::core::ffi::c_int,
    );
    if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int) as COUNT,
            ),
        );
    } else {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%s %u\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            (*pSolarSysState).SysInfo.PlanetInfo.Weather as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
        );
        t.pStr = &raw mut buf as *mut UNICODE;
    }
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        5 as ::core::ffi::c_int,
    );
    if ((*(*pSolarSysState).SysInfo.PlanetInfo.PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_7::GAS_GIANT.0 as ::core::ffi::c_int
    {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int) as COUNT,
            ),
        );
    } else {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%s %u\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            (*pSolarSysState).SysInfo.PlanetInfo.Tectonics as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
        );
        t.pStr = &raw mut buf as *mut UNICODE;
    }
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = 40 as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 11 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 75 as ::core::ffi::c_int,
    );
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    val = ((val as DWORD).wrapping_mul(val as DWORD).wrapping_mul(val as DWORD)
        as ::core::ffi::c_ulong)
        .wrapping_div(100 as ::core::ffi::c_ulong)
        .wrapping_mul(
            (*pSolarSysState).SysInfo.PlanetInfo.PlanetDensity as ::core::ffi::c_ulong,
        )
        .wrapping_add(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        )
        .wrapping_div(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong,
        ) as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 13 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 75 as ::core::ffi::c_int,
    );
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 75 as ::core::ffi::c_int,
    );
    val = (*pSolarSysState).SysInfo.PlanetInfo.SurfaceGravity as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 15 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 75 as ::core::ffi::c_int,
    );
    val = ((*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod as ::core::ffi::c_int
        * 10 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 17 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 14 as ::core::ffi::c_int)
        as COORD;
    PrintScanTitlePC(
        &raw mut t,
        &raw mut r,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 75 as ::core::ffi::c_int,
    );
    val = (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt as SDWORD;
    if val < 0 as ::core::ffi::c_int {
        val = -val;
    }
    t.pStr = &raw mut buf as *mut UNICODE;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
        val,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
}
unsafe extern "C" fn PrintCoarseScan3DO() {
    let mut val: SDWORD = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut buf: [UNICODE; 200] = [0; 200];
    GetPlanetTitle(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 200]>() as COUNT,
    );
    SetContext(PlanetContext);
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = 13 as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE));
    SetContextFont(MicroFont);
    font_DrawText(&raw mut t);
    s.origin.y = 0 as COORD;
    s.origin.x = s.origin.y;
    s.origin.x = (16 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as COORD;
    s.frame = SetAbsFrameIndex(SpaceJunkFrame, 20 as COUNT);
    DrawStamp(&raw mut s);
    t.baseline.x = (27 as ::core::ffi::c_int
        + (16 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = 25 as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.pStr = &raw mut buf as *mut UNICODE;
    val = (((*pSolarSysState).SysInfo.PlanetInfo.PlanetToSunDist as ::core::ffi::c_long
        * 100 as ::core::ffi::c_long
        + ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        / ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0xffff as ::core::ffi::c_int
    {
        strcpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"\xE2\x88\x9E\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        val = (((*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
            * 100 as ::core::ffi::c_int
            + (50 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 50 as ::core::ffi::c_int) as SDWORD;
        MakeScanValue(
            &raw mut buf as *mut UNICODE,
            val as ::core::ffi::c_long,
            b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
        );
    }
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
        (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"<%u>\0".as_ptr() as *const ::core::ffi::c_char,
        if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            (*pSolarSysState).SysInfo.PlanetInfo.Weather as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
        },
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"<%u>\0".as_ptr() as *const ::core::ffi::c_char,
        if ((*(*pSolarSysState).SysInfo.PlanetInfo.PlanDataPtr).Type
            as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int == C2Rust_Unnamed_7::GAS_GIANT.0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            (*pSolarSysState).SysInfo.PlanetInfo.Tectonics as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
        },
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        - (27 as ::core::ffi::c_int
            + (16 as ::core::ffi::c_int - 0 as ::core::ffi::c_int))) as COORD;
    t.baseline.y = 25 as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.pStr = &raw mut buf as *mut UNICODE;
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    val = ((val as DWORD).wrapping_mul(val as DWORD).wrapping_mul(val as DWORD)
        as ::core::ffi::c_ulong)
        .wrapping_div(100 as ::core::ffi::c_ulong)
        .wrapping_mul(
            (*pSolarSysState).SysInfo.PlanetInfo.PlanetDensity as ::core::ffi::c_ulong,
        )
        .wrapping_add(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        )
        .wrapping_div(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong,
        ) as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    val = (*pSolarSysState).SysInfo.PlanetInfo.SurfaceGravity as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    val = (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt as SDWORD;
    if val < 0 as ::core::ffi::c_int {
        val = -val;
    }
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
        val,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
        as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    val = ((*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod as ::core::ffi::c_int
        * 10 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
}
unsafe extern "C" fn initPlanetLocationImage() {
    let mut cursorFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    cursorFrame = SetAbsFrameIndex(MiscDataFrame, 105 as COUNT);
    cursorRect.extent = (*cursorFrame).Bounds;
}
unsafe extern "C" fn savePlanetLocationImage() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut cursorFrame: FRAME = SetAbsFrameIndex(MiscDataFrame, 105 as COUNT);
    let mut hs: HOT_SPOT = GetFrameHot(cursorFrame);
    DestroyDrawable(ReleaseDrawable(eraseFrame));
    r = cursorRect;
    r.corner.x = (r.corner.x as ::core::ffi::c_int - hs.x as ::core::ffi::c_int)
        as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int - hs.y as ::core::ffi::c_int)
        as COORD;
    eraseFrame = CaptureDrawable(CopyContextRect(&raw mut r));
    SetFrameHot(eraseFrame, hs);
}
unsafe extern "C" fn restorePlanetLocationImage() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin = cursorRect.corner;
    s.frame = eraseFrame;
    DrawStamp(&raw mut s);
}
unsafe extern "C" fn drawPlanetCursor(mut filled: BOOLEAN) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin = cursorRect.corner;
    s.frame = SetAbsFrameIndex(MiscDataFrame, 105 as COUNT);
    if filled.0 != 0 {
        DrawFilledStamp(&raw mut s);
    } else {
        DrawStamp(&raw mut s);
    };
}
unsafe extern "C" fn setPlanetCursorLoc(mut new_pt: POINT) {
    new_pt.x = (new_pt.x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    new_pt.y = (new_pt.y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    cursorRect.corner = new_pt;
}
unsafe extern "C" fn setPlanetLoc(mut new_pt: POINT, mut restoreOld: BOOLEAN) {
    planetLoc = new_pt;
    SetContext(ScanContext);
    if restoreOld.0 != 0 {
        restorePlanetLocationImage();
    }
    setPlanetCursorLoc(new_pt);
    savePlanetLocationImage();
}
unsafe extern "C" fn flashPlanetLocation() {
    static mut c: BYTE = 0 as BYTE;
    static mut val: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
    static mut prevPt: POINT = POINT { x: 0, y: 0 };
    static mut NextTime: TimeCount = 0 as TimeCount;
    let mut locChanged: BOOLEAN = BOOLEAN::FALSE;
    let mut Now: TimeCount = GetTimeCounter();
    locChanged = BOOLEAN(
        (prevPt.x as ::core::ffi::c_int != cursorRect.corner.x as ::core::ffi::c_int
            || prevPt.y as ::core::ffi::c_int
                != cursorRect.corner.y as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if locChanged.0 == 0 && Now < NextTime {
        return;
    }
    if locChanged.0 != 0 {
        c = 0 as BYTE;
        val = -2 as ::core::ffi::c_int;
        prevPt = cursorRect.corner;
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
    } else {
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == 0x1a as ::core::ffi::c_int
        {
            val = -val;
        }
        c = (c as ::core::ffi::c_int + val) as BYTE;
        if Now.wrapping_sub(NextTime)
            > (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount
        {
            NextTime = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
        } else {
            NextTime = NextTime
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
        }
    }
    SetContext(ScanContext);
    SetContextForeGroundColor(colorFromRgb15(c, c, c));
    drawPlanetCursor(BOOLEAN::TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn RedrawSurfaceScan(mut newLoc: *const POINT) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(ScanContext);
    BatchGraphics();
    DrawPlanet(0 as ::core::ffi::c_int, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawScannedObjects(BOOLEAN::TRUE);
    if !newLoc.is_null() {
        setPlanetLoc(*newLoc, BOOLEAN::FALSE);
        drawPlanetCursor(BOOLEAN::FALSE);
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn getLandingFuelNeeded() -> COUNT {
    let mut fuel: COUNT = 0;
    fuel = (((*pSolarSysState).SysInfo.PlanetInfo.SurfaceGravity as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as COUNT;
    if fuel as ::core::ffi::c_int > 3 as ::core::ffi::c_int * 100 as ::core::ffi::c_int {
        fuel = (3 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as COUNT;
    }
    return fuel;
}
unsafe extern "C" fn spawnFwiffo() {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    EncounterGroup = 0 as BYTE;
    PutGroupInfo(0 as ::core::ffi::c_long as DWORD, !(0 as ::core::ffi::c_int) as BYTE);
    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
    (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/scan.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                593 as ::core::ffi::c_int,
                b"spawnFwiffo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hStarShip = CloneShipFragment(
        C2Rust_Unnamed_9::SPATHI_SHIP.0 as ::core::ffi::c_int as COUNT,
        &raw mut GlobData.Game_state.npc_built_ship_q,
        1 as COUNT,
    );
    if !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(
            &raw mut GlobData.Game_state.npc_built_ship_q,
            hStarShip,
        );
        (*StarShipPtr).captains_name_index = (5 as ::core::ffi::c_int
            + 16 as ::core::ffi::c_int) as BYTE;
        UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
    }
}
unsafe extern "C" fn DispatchLander() -> BOOLEAN {
    let mut oldCallback: Option<InputFrameCallback> = None;
    let mut landingFuel: SIZE = getLandingFuelNeeded() as SIZE;
    EraseCoarseScan();
    oldCallback = SetInputCallback(None);
    DeltaSISGauges(
        0 as SIZE,
        -(landingFuel as ::core::ffi::c_int) as SIZE,
        0 as ::core::ffi::c_int,
    );
    SetContext(ScanContext);
    drawPlanetCursor(BOOLEAN::FALSE);
    PlanetSide(planetLoc);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_12::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_11::FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_11::END_FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 1 as ::core::ffi::c_int
    {
        spawnFwiffo();
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_12::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_12::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        SaveSolarSysLocation();
        return BOOLEAN::FALSE;
    }
    if optWhichCoarseScan == 0x2 as ::core::ffi::c_int {
        PrintCoarseScanPC();
    } else {
        PrintCoarseScan3DO();
    }
    SetInputCallback(oldCallback);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoPickPlanetSide(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut pickState: *mut PICK_PLANET_STATE = (*pMS).privData
        as *mut PICK_PLANET_STATE;
    let mut TimeIn: DWORD = GetTimeCounter();
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    select = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_12::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pickState).success = false;
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0 {
        (*pickState).success = false;
        return BOOLEAN::FALSE;
    } else if select.0 != 0 {
        (*pickState).success = true;
        return BOOLEAN::FALSE;
    } else {
        let mut dx: SIZE = 0 as SIZE;
        let mut dy: SIZE = 0 as SIZE;
        let mut new_pt: POINT = POINT { x: 0, y: 0 };
        new_pt = planetLoc;
        if CurrentInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            dx = -1 as SIZE;
        }
        if CurrentInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
            != 0
        {
            dx = 1 as SIZE;
        }
        if CurrentInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            dy = -1 as SIZE;
        }
        if CurrentInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            dy = 1 as SIZE;
        }
        BatchGraphics();
        dx = ((dx as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE;
        if dx != 0 {
            new_pt.x = (new_pt.x as ::core::ffi::c_int + dx as ::core::ffi::c_int)
                as COORD;
            if (new_pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                new_pt.x = (new_pt.x as ::core::ffi::c_int
                    + ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                    as COORD;
            } else if new_pt.x as ::core::ffi::c_int
                >= (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            {
                new_pt.x = (new_pt.x as ::core::ffi::c_int
                    - ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                    as COORD;
            }
        }
        dy = ((dy as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE;
        if dy != 0 {
            new_pt.y = (new_pt.y as ::core::ffi::c_int + dy as ::core::ffi::c_int)
                as COORD;
            if (new_pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || new_pt.y as ::core::ffi::c_int
                    >= (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
            {
                new_pt.y = planetLoc.y;
            }
        }
        if !pointsEqual(new_pt, planetLoc) {
            setPlanetLoc(new_pt, BOOLEAN::TRUE);
        }
        flashPlanetLocation();
        UnbatchGraphics();
        SleepThreadUntil(
            TimeIn
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimeCount,
                ),
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn drawLandingFuelUsage(mut fuel: COUNT) {
    let mut buf: [UNICODE; 100] = [0; 100];
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%s%1.1f\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        (fuel as ::core::ffi::c_float
            / 100 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double,
    );
    DrawStatusMessage(&raw mut buf as *mut UNICODE);
}
unsafe extern "C" fn eraseLandingFuelUsage() {
    DrawStatusMessage(::core::ptr::null::<UNICODE>());
}
unsafe extern "C" fn PickPlanetSide() -> BOOLEAN {
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut PickState: PICK_PLANET_STATE = PICK_PLANET_STATE {
        success: false,
    };
    let mut fuel: COUNT = getLandingFuelNeeded();
    let mut retval: BOOLEAN = BOOLEAN::TRUE;
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.privData = &raw mut PickState as *mut ::core::ffi::c_void;
    ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE);
    SetContext(ScanContext);
    BatchGraphics();
    DrawPlanet(0 as ::core::ffi::c_int, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawScannedObjects(BOOLEAN::FALSE);
    UnbatchGraphics();
    drawLandingFuelUsage(fuel);
    setPlanetCursorLoc(planetLoc);
    savePlanetLocationImage();
    InitLander(0 as BYTE);
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    PickState.success = false;
    MenuState.InputFunc = Some(
        DoPickPlanetSide as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    eraseLandingFuelUsage();
    if PickState.success {
        retval = DispatchLander();
    } else {
        restorePlanetLocationImage();
    }
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
    return retval;
}
unsafe extern "C" fn DrawScannedStuff(mut y: COUNT, mut scan: COUNT) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut dy: SIZE = 0;
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).succ;
        dy = (y as ::core::ffi::c_int
            - (*ElementPtr).current.location.y as ::core::ffi::c_int) as SIZE;
        if (*ElementPtr).c2rust_unnamed.scan_node as BYTE as ::core::ffi::c_int
            != scan as ::core::ffi::c_int
            || (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            UnlockLink(&raw mut disp_q, hElement);
        } else {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            s.origin = (*ElementPtr).current.location;
            if dy as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
                s.frame = (*ElementPtr).next.image.frame;
                DrawStamp(&raw mut s);
            } else {
                let mut grad: BYTE = 0;
                let mut c: Color = colorFromRgb15(
                    0x1f as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
                let mut nodeSize: COUNT = 0;
                grad = (0xff as ::core::ffi::c_int
                    - 0xff as ::core::ffi::c_int * dy as ::core::ffi::c_int
                        / (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)) as BYTE;
                match scan as ::core::ffi::c_int {
                    0 => {
                        c.r = grad;
                    }
                    1 => {
                        c.g = grad;
                        c.b = grad;
                    }
                    2 => {
                        c.g = grad;
                    }
                    _ => {}
                }
                SetContextForeGroundColor(c);
                nodeSize = (GetFrameIndex((*ElementPtr).next.image.frame)
                    as ::core::ffi::c_int
                    - GetFrameIndex((*ElementPtr).current.image.frame)
                        as ::core::ffi::c_int) as COUNT;
                if dy as ::core::ffi::c_int > nodeSize as ::core::ffi::c_int {
                    dy = nodeSize as SIZE;
                }
                s.frame = SetRelFrameIndex((*ElementPtr).current.image.frame, dy);
                DrawFilledStamp(&raw mut s);
            }
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
    SetContextForeGroundColor(OldColor);
}
#[no_mangle]
pub unsafe extern "C" fn callGenerateForScanType(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut node: COUNT,
    mut scanType: BYTE,
    mut info: *mut NODE_INFO,
) -> COUNT {
    match scanType as ::core::ffi::c_int {
        0 => {
            return Some(
                    (*(*solarSys).genFuncs)
                        .generateMinerals
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node, info);
        }
        1 => {
            return Some(
                    (*(*solarSys).genFuncs)
                        .generateEnergy
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node, info);
        }
        2 => {
            return Some(
                    (*(*solarSys).genFuncs)
                        .generateLife
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node, info);
        }
        _ => {}
    }
    (false
        || {
            __assert_fail(
                b"false\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/scan.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                896 as ::core::ffi::c_int,
                b"callGenerateForScanType\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn callPickupForScanType(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut node: COUNT,
    mut scanType: BYTE,
) -> bool {
    match scanType as ::core::ffi::c_int {
        0 => {
            return Some(
                    (*(*solarSys).genFuncs)
                        .pickupMinerals
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node);
        }
        1 => {
            return Some(
                    (*(*solarSys).genFuncs)
                        .pickupEnergy
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node);
        }
        2 => {
            return Some(
                    (*(*solarSys).genFuncs)
                        .pickupLife
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node);
        }
        _ => {}
    }
    (false
        || {
            __assert_fail(
                b"false\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/scan.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                917 as ::core::ffi::c_int,
                b"callPickupForScanType\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return false;
}
unsafe extern "C" fn ScanPlanet(mut scanType: COUNT) {
    let mut startScan: COUNT = 0;
    let mut endScan: COUNT = 0;
    let mut scan: COUNT = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    static mut textColors: [Color; 3] = [
        Color {
            r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
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
    if scanType as ::core::ffi::c_int == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
    {
        startScan = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
        endScan = PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as COUNT;
    } else {
        startScan = scanType;
        endScan = scanType;
    }
    scan = startScan;
    while scan as ::core::ffi::c_int <= endScan as ::core::ffi::c_int {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut i: SWORD = 0;
        let mut tintColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        let mut TimeOut: TimeCount = 0;
        t.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        t.baseline.y = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 7 as ::core::ffi::c_int) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + scan as ::core::ffi::c_int) as COUNT,
            ),
        );
        SetContext(PlanetContext);
        r.corner.x = 0 as COORD;
        r.corner.y = (t.baseline.y as ::core::ffi::c_int - 10 as ::core::ffi::c_int)
            as COORD;
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) as COORD;
        r.extent.height = (t.baseline.y as ::core::ffi::c_int
            - r.corner.y as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
        RepairBackRect(&raw mut r);
        SetContextFont(MicroFont);
        SetContextForeGroundColor(textColors[scan as usize]);
        font_DrawText(&raw mut t);
        SetContext(ScanContext);
        BatchGraphics();
        DrawPlanet(
            0 as ::core::ffi::c_int,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        );
        UnbatchGraphics();
        tintColor = tintColors[scan as usize];
        TimeOut = GetTimeCounter();
        i = 0 as SWORD;
        while (i as ::core::ffi::c_int)
            < 75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            TimeOut = TimeOut
                .wrapping_add(
                    (840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                        / 4 as ::core::ffi::c_int
                        / (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                        as TimeCount,
                );
            if WaitForAnyButtonUntil(BOOLEAN::TRUE, TimeOut, BOOLEAN::FALSE).0 != 0 {
                break;
            }
            BatchGraphics();
            DrawPlanet(i as ::core::ffi::c_int, tintColor);
            DrawScannedStuff(i as COUNT, scan);
            UnbatchGraphics();
            i += 1;
        }
        if (i as ::core::ffi::c_int)
            < 75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            BatchGraphics();
            DrawPlanet(
                75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int,
                tintColor,
            );
            DrawScannedStuff(
                (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
                scan,
            );
            UnbatchGraphics();
        }
        scan = scan.wrapping_add(1);
    }
    SetContext(PlanetContext);
    RepairBackRect(&raw mut r);
    SetContext(ScanContext);
    if scanType as ::core::ffi::c_int == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
    {
        DrawPlanet(
            0 as ::core::ffi::c_int,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        );
        DrawScannedObjects(BOOLEAN::FALSE);
    }
    FlushInput();
}
unsafe extern "C" fn DoScan(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    select = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_10::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_12::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0
        || select.0 != 0
            && (*pMS).CurState as ::core::ffi::c_int
                == ScanMenuItems::EXIT_SCAN.0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE
    } else if select.0 != 0 {
        if (*pMS).CurState as ::core::ffi::c_int
            == ScanMenuItems::DISPATCH_SHUTTLE.0 as ::core::ffi::c_int
        {
            let mut fuel_required: COUNT = 0;
            if (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
                || (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
                    == 0xffff as ::core::ffi::c_int
            {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
                return BOOLEAN::TRUE;
            }
            fuel_required = getLandingFuelNeeded();
            if GlobData.SIS_state.FuelOnBoard < fuel_required as DWORD
                || GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
                return BOOLEAN::TRUE;
            }
            SetFlashRect(::core::ptr::null::<RECT>());
            if PickPlanetSide().0 == 0 {
                return BOOLEAN::FALSE;
            }
            DrawMenuStateStrings(
                C2Rust_Unnamed_6::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            );
            SetFlashRect(
                ::core::ptr::with_exposed_provenance_mut::<
                    RECT,
                >(!(0 as ::core::ffi::c_long) as usize),
            );
            return BOOLEAN::TRUE;
        }
        if (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
        {
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
            return BOOLEAN::TRUE;
        }
        ScanPlanet((*pMS).CurState as COUNT);
        if (*pMS).CurState as ::core::ffi::c_int
            == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
        {
            (*pMS).CurState = ScanMenuItems::DISPATCH_SHUTTLE.0 as ::core::ffi::c_int
                as BYTE;
            DrawMenuStateStrings(
                C2Rust_Unnamed_6::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            );
        }
    } else if optWhichMenu == 0x2 as ::core::ffi::c_int
        || (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int == 0
            && (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
                != 0xffff as ::core::ffi::c_int
    {
        DoMenuChooser(
            pMS,
            C2Rust_Unnamed_6::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn CreateScanContext() -> CONTEXT {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut context: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    oldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut r);
    context = CreateContextAux(b"ScanContext\0".as_ptr() as *const ::core::ffi::c_char);
    SetContext(context);
    SetContextFGFrame(Screen);
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int
            - (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int))) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (r.extent.height as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int))) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as COORD;
    SetContextClipRect(&raw mut r);
    SetContext(oldContext);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn GetScanContext(mut owner: *mut BOOLEAN) -> CONTEXT {
    if !ScanContext.is_null() {
        if !owner.is_null() {
            *owner = BOOLEAN::FALSE;
        }
    } else {
        if !owner.is_null() {
            *owner = BOOLEAN::TRUE;
        }
        ScanContext = CreateScanContext();
    }
    return ScanContext;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyScanContext() {
    if !ScanContext.is_null() {
        DestroyContext(ScanContext);
        ScanContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScanSystem() {
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    GetScanContext(::core::ptr::null_mut::<BOOLEAN>());
    if optWhichMenu == 0x1 as ::core::ffi::c_int
        && ((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
            || (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
                == 0xffff as ::core::ffi::c_int)
    {
        MenuState.CurState = ScanMenuItems::EXIT_SCAN.0 as ::core::ffi::c_int as BYTE;
    } else {
        MenuState.CurState = ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int as BYTE;
        planetLoc.x = ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COORD;
        planetLoc.y = ((75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
        initPlanetLocationImage();
        SetContext(ScanContext);
        DrawScannedObjects(BOOLEAN::FALSE);
    }
    DrawMenuStateStrings(
        C2Rust_Unnamed_6::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
        MenuState.CurState as SWORD,
    );
    SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
    );
    if optWhichCoarseScan == 0x2 as ::core::ffi::c_int {
        PrintCoarseScanPC();
    } else {
        PrintCoarseScan3DO();
    }
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
    MenuState.InputFunc = Some(
        DoScan as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    SetFlashRect(::core::ptr::null::<RECT>());
    BatchGraphics();
    SetContext(ScanContext);
    DrawPlanet(0 as ::core::ffi::c_int, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    EraseCoarseScan();
    UnbatchGraphics();
    DestroyDrawable(ReleaseDrawable(eraseFrame));
    eraseFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
unsafe extern "C" fn generateBioNode(
    mut system: *mut SOLARSYS_STATE,
    mut NodeElementPtr: *mut ELEMENT,
    mut life_init_tab: *mut BYTE,
    mut creatureType: COUNT,
) {
    let mut i: COUNT = 0;
    if (*(&raw const CreatureData as *const LIFEFORM_DESC).offset(creatureType as isize))
        .Attributes as ::core::ffi::c_int & 0x30 as ::core::ffi::c_int != 0
    {
        i = TFB_Random() as COUNT;
        (*NodeElementPtr).current.location.x = (i as BYTE as ::core::ffi::c_int
            % (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
            + 8 as ::core::ffi::c_int) as COORD;
        (*NodeElementPtr).current.location.y = ((i as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            % (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
            + 8 as ::core::ffi::c_int) as COORD;
    }
    if (*system).PlanetSideFrame[0usize].is_null() {
        (*system).PlanetSideFrame[0usize] = CaptureDrawable(
            LoadGraphicInstance(b"graphics.lifecan\0".as_ptr() as RESOURCE) as DRAWABLE,
        );
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
        && *life_init_tab.offset(i as isize) as ::core::ffi::c_int
            != (creatureType as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
    {
        if *life_init_tab.offset(i as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            i = i.wrapping_add(1);
        } else {
            *life_init_tab.offset(i as isize) = (creatureType as BYTE
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
            (*system)
                .PlanetSideFrame[(i as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                as usize] = load_life_form(creatureType as BYTE);
            break;
        }
    }
    (*NodeElementPtr).c2rust_unnamed_1.mass_points = creatureType as BYTE;
    (*NodeElementPtr).c2rust_unnamed_0.hit_points = ((*(&raw const CreatureData
        as *const LIFEFORM_DESC)
        .offset(creatureType as isize))
        .ValueAndHitPoints as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE as COUNT;
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = SetAbsFrameIndex(
        (*system)
            .PlanetSideFrame[(i as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            as usize],
        TFB_Random() as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GeneratePlanetSide() {
    let mut scan: SIZE = 0;
    let mut life_init_tab: [BYTE; 3] = [0; 3];
    InitDisplayList();
    if (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
        & C2Rust_Unnamed_8::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    memset(
        &raw mut life_init_tab as *mut BYTE as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[BYTE; 3]>(),
    );
    scan = PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as SIZE;
    while (scan as ::core::ffi::c_int)
        >= PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int
    {
        let mut num_nodes: COUNT = 0;
        let mut f: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        f = SetAbsFrameIndex(
            MiscDataFrame,
            (4 as ::core::ffi::c_int
                * (scan as ::core::ffi::c_int
                    - PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int)) as COUNT,
        );
        num_nodes = callGenerateForScanType(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            !(0 as ::core::ffi::c_int) as COUNT,
            scan as BYTE,
            ::core::ptr::null_mut::<NODE_INFO>(),
        );
        loop {
            let c2rust_fresh0 = num_nodes;
            num_nodes = num_nodes.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            let mut hNodeElement: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut NodeElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut info: NODE_INFO = NODE_INFO {
                loc_pt: POINT { x: 0, y: 0 },
                density: 0,
                r#type: 0,
            };
            if isNodeRetrieved(
                &raw mut (*pSolarSysState).SysInfo.PlanetInfo,
                scan as BYTE,
                num_nodes as BYTE,
            ) {
                continue;
            }
            hNodeElement = AllocElement();
            if hNodeElement.is_null() {
                continue;
            }
            NodeElementPtr = LockLink(&raw mut disp_q, hNodeElement) as *mut ELEMENT;
            callGenerateForScanType(
                pSolarSysState,
                (*pSolarSysState).pOrbitalDesc,
                num_nodes,
                scan as BYTE,
                &raw mut info,
            );
            (*NodeElementPtr).c2rust_unnamed.scan_node = ((((num_nodes
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int | scan as BYTE as ::core::ffi::c_int) as COUNT;
            (*NodeElementPtr).playerNr = C2Rust_Unnamed_5::PS_NON_PLAYER.0
                as ::core::ffi::c_int as SIZE;
            (*NodeElementPtr).current.location.x = info.loc_pt.x;
            (*NodeElementPtr).current.location.y = info.loc_pt.y;
            DisplayArray[(*NodeElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            if scan as ::core::ffi::c_int
                == PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int
            {
                (*NodeElementPtr).c2rust_unnamed_2.turn_wait = info.r#type as BYTE;
                (*NodeElementPtr).c2rust_unnamed_1.mass_points = (info.density
                    as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE;
                (*NodeElementPtr).current.image.frame = SetAbsFrameIndex(
                    MiscDataFrame,
                    (4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        + (*Elements.offset(info.r#type as isize) as ::core::ffi::c_int
                            & 0x7 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                        as COUNT,
                );
                (*NodeElementPtr).next.image.frame = SetRelFrameIndex(
                    (*NodeElementPtr).current.image.frame,
                    (info.density as BYTE as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as SIZE,
                );
                DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = IncFrameIndex(
                    (*NodeElementPtr).next.image.frame,
                );
            } else {
                (*NodeElementPtr).current.image.frame = f;
                (*NodeElementPtr).next.image.frame = SetRelFrameIndex(
                    f,
                    (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE,
                );
                (*NodeElementPtr).c2rust_unnamed_2.turn_wait = ((4 as ::core::ffi::c_int
                    as BYTE as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                    | 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
                (*NodeElementPtr).preprocess_func = Some(
                    object_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                if scan as ::core::ffi::c_int
                    == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
                {
                    (*NodeElementPtr).c2rust_unnamed_1.mass_points = 50 as BYTE;
                    DisplayArray[(*NodeElementPtr).PrimIndex as usize]
                        .Object
                        .Stamp
                        .frame = (*pSolarSysState).PlanetSideFrame[1usize];
                } else {
                    generateBioNode(
                        pSolarSysState,
                        NodeElementPtr,
                        &raw mut life_init_tab as *mut BYTE,
                        info.r#type,
                    );
                }
            }
            (*NodeElementPtr).next.location.x = (((*NodeElementPtr).current.location.x
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
            (*NodeElementPtr).next.location.y = (((*NodeElementPtr).current.location.y
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
            UnlockLink(&raw mut disp_q, hNodeElement);
            PutQueue(&raw mut disp_q, hNodeElement);
        }
        scan -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn isNodeRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
    mut nodeNr: BYTE,
) -> bool {
    return (*planetInfo).ScanRetrieveMask[scanType as usize]
        & (1 as ::core::ffi::c_int as DWORD) << nodeNr as ::core::ffi::c_int
        != 0 as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn countNodesRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
) -> COUNT {
    let mut count: COUNT = 0;
    let mut mask: DWORD = (*planetInfo).ScanRetrieveMask[scanType as usize];
    count = 0 as COUNT;
    while mask != 0 as DWORD {
        if mask & 1 as DWORD != 0 {
            count = count.wrapping_add(1);
        }
        mask >>= 1 as ::core::ffi::c_int;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn setNodeRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
    mut nodeNr: BYTE,
) {
    (*planetInfo).ScanRetrieveMask[scanType as usize]
        |= (1 as ::core::ffi::c_int as DWORD) << nodeNr as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setNodeNotRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
    mut nodeNr: BYTE,
) {
    (*planetInfo).ScanRetrieveMask[scanType as usize]
        &= !((1 as ::core::ffi::c_int as DWORD) << nodeNr as ::core::ffi::c_int);
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
                b"src/uqm/planets/../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                329 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
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
