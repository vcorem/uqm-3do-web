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

// Rust translation of src/uqm/planets/solarsys.c.
extern "C" {
    pub type RandomContext;
    pub type tfb_soundsample;
    pub type frame_desc;
    pub type drawable_desc;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type font_desc;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn PLRPlaying(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn GetTimeCounter() -> TimeCount;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut sinetab: [SIZE; 0];
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn ReinitQueue(pq: *mut QUEUE);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    fn LoadLanderData();
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Delete(context: *mut RandomContext);
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn FreePlanet();
    fn DrawOval(pRect: *mut RECT, num_off_pixels: BYTE);
    fn PlanetOrbitMenu();
    fn DoPlanetaryAnalysis(SysInfoPtr: *mut SYSTEM_INFO, pPlanetDesc: *mut PLANET_DESC);
    static mut PlanData: *const PlanetFrame;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    static mut GameStrings: STRING;
    fn GameClockTick();
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn SetFlashRect(pRect: *const RECT);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn DrawAutoPilotMessage(Reset: BOOLEAN);
    fn SleepThreadUntil(wakeTime: TimeCount);
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
    static mut stars_in_space: FRAME;
    fn CargoMenu();
    fn RosterMenu() -> BOOLEAN;
    fn DevicesMenu() -> BOOLEAN;
    fn StarMap() -> BOOLEAN;
    fn GameOptions() -> BOOLEAN;
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn DoMissions();
    fn RedrawQueue(clear: BOOLEAN);
    fn InitDisplayList();
    static mut Screen: FRAME;
    static mut OffScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut LastActivity: ACTIVITY;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn GetPlanetInfo();
    fn PutPlanetInfo();
    fn GetGroupInfo(offset: DWORD, which_group: BYTE) -> BOOLEAN;
    static mut NextActivity: ACTIVITY;
    static mut optWhichMenu: ::core::ffi::c_int;
    fn abort() -> !;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type RESOURCE = *const ::core::ffi::c_char;
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type DRAWABLE_DESC = drawable_desc;
pub type COLORMAP = STRING;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING_TABLE_DESC = string_table;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FONT = *mut FONT_DESC;
pub type TIME_VALUE = UWORD;
pub type COORD = SWORD;
pub type CREATE_FLAGS = BYTE;
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
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
}
pub type TimeCount = DWORD;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type MENU_SOUND_FLAGS = UWORD;
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
pub struct SolarSysMenuMenuItems(pub ::core::ffi::c_uint);
impl SolarSysMenuMenuItems {
    pub const STARMAP: Self = Self(1);
    pub const EQUIP_DEVICE: Self = Self(2);
    pub const CARGO: Self = Self(3);
    pub const ROSTER: Self = Self(4);
    pub const GAME_MENU: Self = Self(5);
    pub const NAVIGATION: Self = Self(6);
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
unsafe extern "C" fn pointWithinRect(mut r: RECT, mut p: POINT) -> bool {
    return p.x as ::core::ffi::c_int >= r.corner.x as ::core::ffi::c_int
        && p.y as ::core::ffi::c_int >= r.corner.y as ::core::ffi::c_int
        && (p.x as ::core::ffi::c_int)
            < r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        && (p.y as ::core::ffi::c_int)
            < r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int;
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
unsafe extern "C" fn explode() -> ! {
    abort();
}
#[no_mangle]
pub static mut pSolarSysState: *mut SOLARSYS_STATE = ::core::ptr::null_mut::<
    SOLARSYS_STATE,
>();
#[no_mangle]
pub static mut SISIPFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut SunFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut OrbitalFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut SpaceJunkFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut OrbitalCMap: COLORMAP = ::core::ptr::null_mut::<
    STRING_TABLE_ENTRY_DESC,
>();
#[no_mangle]
pub static mut SunCMap: COLORMAP = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut SpaceMusic: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
#[no_mangle]
pub static mut EncounterRace: SIZE = 0;
#[no_mangle]
pub static mut EncounterGroup: BYTE = 0;
static mut StarsFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut SolarSysFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut scaleRect: RECT = RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
};
#[no_mangle]
pub static mut SysGenRNG: *mut RandomContext = ::core::ptr::null_mut::<RandomContext>();
#[no_mangle]
pub unsafe extern "C" fn locationToDisplay(
    mut pt: POINT,
    mut scaleRadius: SIZE,
) -> POINT {
    let mut out: POINT = POINT { x: 0, y: 0 };
    out.x = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        + pt.x as ::core::ffi::c_long
            * ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as ::core::ffi::c_long / scaleRadius as ::core::ffi::c_long) as COORD;
    out.y = ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        + pt.y as ::core::ffi::c_long
            * ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as ::core::ffi::c_long / scaleRadius as ::core::ffi::c_long) as COORD;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn displayToLocation(
    mut pt: POINT,
    mut scaleRadius: SIZE,
) -> POINT {
    let mut out: POINT = POINT { x: 0, y: 0 };
    out.x = ((pt.x as ::core::ffi::c_long
        - (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_long) * scaleRadius as ::core::ffi::c_long
        / ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - 8 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD;
    out.y = ((pt.y as ::core::ffi::c_long
        - (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_long) * scaleRadius as ::core::ffi::c_long
        / ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - 8 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn planetOuterLocation(mut planetI: COUNT) -> POINT {
    let mut scaleRadius: SIZE = (*pSolarSysState).SunDesc[0usize].radius;
    return displayToLocation(
        (*pSolarSysState).PlanetDesc[planetI as usize].image.origin,
        scaleRadius,
    );
}
#[no_mangle]
pub unsafe extern "C" fn worldIsPlanet(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> bool {
    return (*world).pPrevDesc
        == &raw const (*solarSys).SunDesc as *const PLANET_DESC as *mut PLANET_DESC;
}
#[no_mangle]
pub unsafe extern "C" fn worldIsMoon(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> bool {
    return (*world).pPrevDesc
        != &raw const (*solarSys).SunDesc as *const PLANET_DESC as *mut PLANET_DESC;
}
#[no_mangle]
pub unsafe extern "C" fn planetIndex(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
) -> COUNT {
    let mut planet: *const PLANET_DESC = if worldIsPlanet(solarSys, world)
        as ::core::ffi::c_int != 0
    {
        world
    } else {
        (*world).pPrevDesc as *const PLANET_DESC
    };
    return planet.offset_from(&raw const (*solarSys).PlanetDesc as *const PLANET_DESC)
        as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn moonIndex(
    mut solarSys: *const SOLARSYS_STATE,
    mut moon: *const PLANET_DESC,
) -> COUNT {
    (!worldIsPlanet(solarSys, moon)
        || {
            __assert_fail(
                b"!worldIsPlanet (solarSys, moon)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int,
                b"moonIndex\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return moon.offset_from(&raw const (*solarSys).MoonDesc as *const PLANET_DESC)
        as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn matchWorld(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut planetI: BYTE,
    mut moonI: BYTE,
) -> bool {
    if planetIndex(solarSys, world) as ::core::ffi::c_int
        != planetI as ::core::ffi::c_int
    {
        return false;
    }
    if moonI as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        if !worldIsPlanet(solarSys, world) {
            return false;
        }
    } else {
        if !worldIsMoon(solarSys, world) {
            return false;
        }
        if moonIndex(solarSys, world) as ::core::ffi::c_int
            != moonI as ::core::ffi::c_int
        {
            return false;
        }
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn playerInSolarSystem() -> bool {
    return !pSolarSysState.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn playerInPlanetOrbit() -> bool {
    return playerInSolarSystem() as ::core::ffi::c_int != 0
        && (*pSolarSysState).InOrbit.0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn playerInInnerSystem() -> bool {
    (playerInSolarSystem() as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"playerInSolarSystem ()\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_int,
                b"playerInInnerSystem\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    ((*pSolarSysState).pBaseDesc
        == &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC
        || (*pSolarSysState).pBaseDesc
            == &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC
        || {
            __assert_fail(
                b"pSolarSysState->pBaseDesc == pSolarSysState->PlanetDesc || pSolarSysState->pBaseDesc == pSolarSysState->MoonDesc\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int,
                b"playerInInnerSystem\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return (*pSolarSysState).pBaseDesc
        != &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC;
}
unsafe extern "C" fn GenerateMoons(
    mut system: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
) {
    let mut i: COUNT = 0;
    let mut facing: COUNT = 0;
    let mut pMoonDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    RandomContext_SeedRandom(SysGenRNG, (*planet).rand_seed);
    Some((*(*system).genFuncs).generateName.expect("non-null function pointer"))
        .expect("non-null function pointer")(system, planet);
    Some((*(*system).genFuncs).generateMoons.expect("non-null function pointer"))
        .expect("non-null function pointer")(system, planet);
    facing = (ARCTAN((*planet).location.x, (*planet).location.y) as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT;
    i = 0 as COUNT;
    pMoonDesc = (&raw mut (*system).MoonDesc as *mut PLANET_DESC).offset(0isize);
    while (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        (*pMoonDesc).pPrevDesc = planet;
        if (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
            (*pMoonDesc).temp_color = (*planet).temp_color;
        }
        i = i.wrapping_add(1);
        pMoonDesc = pMoonDesc.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeIPData() {
    DestroyDrawable(ReleaseDrawable(SISIPFrame));
    SISIPFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(SunFrame));
    SunFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyStringTable(ReleaseStringTable(SunCMap));
    SunCMap = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    DestroyStringTable(ReleaseStringTable(OrbitalCMap));
    OrbitalCMap = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    DestroyDrawable(ReleaseDrawable(OrbitalFrame));
    OrbitalFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(SpaceJunkFrame));
    SpaceJunkFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyMusic(SpaceMusic);
    SpaceMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    RandomContext_Delete(SysGenRNG);
    SysGenRNG = ::core::ptr::null_mut::<RandomContext>();
}
#[no_mangle]
pub unsafe extern "C" fn LoadIPData() {
    if SpaceJunkFrame.is_null() {
        SpaceJunkFrame = CaptureDrawable(
            LoadGraphicInstance(b"graphics.orbitbackground\0".as_ptr() as RESOURCE)
                as DRAWABLE,
        );
        SISIPFrame = CaptureDrawable(
            LoadGraphicInstance(b"graphics.flagship\0".as_ptr() as RESOURCE) as DRAWABLE,
        );
        OrbitalCMap = CaptureStringTable(
            LoadStringTableInstance(b"colortable.orbplan\0".as_ptr() as RESOURCE),
        ) as COLORMAP;
        OrbitalFrame = CaptureDrawable(
            LoadGraphicInstance(b"graphics.planets\0".as_ptr() as RESOURCE) as DRAWABLE,
        );
        SunCMap = CaptureStringTable(
            LoadStringTableInstance(b"colortable.truespace\0".as_ptr() as RESOURCE),
        ) as COLORMAP;
        SunFrame = CaptureDrawable(
            LoadGraphicInstance(b"graphics.truespacesun\0".as_ptr() as RESOURCE)
                as DRAWABLE,
        );
        SpaceMusic = LoadMusicInstance(b"music.space\0".as_ptr() as RESOURCE);
    }
    if SysGenRNG.is_null() {
        SysGenRNG = RandomContext_New();
    }
}
// The C display list aliases PlanetDesc[-1] to the preceding SunDesc.
// Select the actual field instead of relying on out-of-array pointer arithmetic.
unsafe fn solarDisplayBody(index: SIZE) -> *mut PLANET_DESC {
    if index == -1 {
        &raw mut (*pSolarSysState).SunDesc[0usize]
    } else {
        assert!(index >= 0, "invalid solar display index");
        &raw mut (*pSolarSysState).PlanetDesc[index as usize]
    }
}
unsafe extern "C" fn sortPlanetPositions() {
    let mut i: COUNT = 0;
    let mut sort_array: [SIZE; 17] = [0; 17];
    i = 0 as COUNT;
    while i as ::core::ffi::c_int
        <= (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
    {
        sort_array[i as usize] = (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as SIZE;
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    while i as ::core::ffi::c_int
        <= (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
    {
        let mut j: COUNT = 0;
        j = (*pSolarSysState).SunDesc[0usize].NumPlanets as COUNT;
        while j as ::core::ffi::c_int > i as ::core::ffi::c_int {
            let mut real_i: SIZE = 0;
            let mut real_j: SIZE = 0;
            real_i = sort_array[i as usize];
            real_j = sort_array[j as usize];
            if (*solarDisplayBody(real_i)).image.origin.y
                as ::core::ffi::c_int
                > (*solarDisplayBody(real_j)).image.origin.y
                    as ::core::ffi::c_int
            {
                let mut temp: SIZE = 0;
                temp = sort_array[i as usize];
                sort_array[i as usize] = sort_array[j as usize];
                sort_array[j as usize] = temp;
            }
            j = j.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
    (*pSolarSysState).FirstPlanetIndex = sort_array[0usize];
    (*pSolarSysState).LastPlanetIndex = sort_array[(*pSolarSysState)
        .SunDesc[0usize]
        .NumPlanets as usize];
    i = 0 as COUNT;
    while i as ::core::ffi::c_int
        <= (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
    {
        let mut planet: *mut PLANET_DESC = solarDisplayBody(sort_array[i as usize]);
        // The last link is never traversed. C reads an uninitialized slot
        // here (out of bounds at 16 planets); use a deterministic terminator.
        (*planet).NextIndex = if i < (*pSolarSysState).SunDesc[0usize].NumPlanets as COUNT {
            sort_array[i as usize + 1] as COUNT
        } else {
            0
        };
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn initSolarSysSISCharacteristics() {
    let mut i: BYTE = 0;
    let mut num_thrusters: BYTE = 0;
    num_thrusters = 0 as BYTE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        if GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int
            == C2Rust_Unnamed_5::FUSION_THRUSTER.0 as ::core::ffi::c_int
        {
            num_thrusters = num_thrusters.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*pSolarSysState).max_ship_speed = ((num_thrusters as ::core::ffi::c_int
        + 5 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int) as BYTE;
    (*pSolarSysState).turn_wait = 17 as BYTE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int
            == C2Rust_Unnamed_5::TURNING_JETS.0 as ::core::ffi::c_int
        {
            (*pSolarSysState).turn_wait = ((*pSolarSysState).turn_wait
                as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetRandomSeedForStar(mut star: *const STAR_DESC) -> DWORD {
    return ((*star).star_pt.y as DWORD) << 16 as ::core::ffi::c_int
        | (*star).star_pt.x as UWORD as DWORD;
}
unsafe extern "C" fn LoadSolarSys() -> *mut PLANET_DESC {
    let mut i: COUNT = 0;
    let mut orbital: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    static mut temp_color_array: [Color; 5] = [
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    RandomContext_SeedRandom(SysGenRNG, GetRandomSeedForStar(CurStarDescPtr));
    SunFrame = SetAbsFrameIndex(
        SunFrame,
        (((*CurStarDescPtr).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as BYTE as COUNT,
    );
    pCurDesc = (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC).offset(0isize);
    (*pCurDesc).pPrevDesc = ::core::ptr::null_mut::<PLANET_DESC>();
    (*pCurDesc).rand_seed = RandomContext_Random(SysGenRNG);
    (*pCurDesc).data_index = (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE;
    (*pCurDesc).location.x = 0 as COORD;
    (*pCurDesc).location.y = 0 as COORD;
    (*pCurDesc).image.origin = (*pCurDesc).location;
    (*pCurDesc).image.frame = SunFrame;
    Some(
            (*(*pSolarSysState).genFuncs)
                .generatePlanets
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(pSolarSysState);
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_6::PLANETARY_CHANGE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_6::END_PLANETARY_CHANGE.0 as ::core::ffi::c_int,
    ) != 0
    {
        PutPlanetInfo();
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_6::PLANETARY_CHANGE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_6::END_PLANETARY_CHANGE.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    }
    i = 0 as COUNT;
    pCurDesc = &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        (*pCurDesc).pPrevDesc = (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC)
            .offset(0isize);
        (*pCurDesc).image.origin = (*pCurDesc).location;
        if i as ::core::ffi::c_int
            >= (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
        {
            (*pCurDesc).image.frame = ::core::ptr::null_mut::<FRAME_DESC>();
        } else {
            let mut index: COUNT = 0;
            let mut SysInfo: SYSTEM_INFO = SYSTEM_INFO {
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
            };
            DoPlanetaryAnalysis(&raw mut SysInfo, pCurDesc);
            index = ((SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int
                + 250 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int) as COUNT;
            if index as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                index = (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
            }
            (*pCurDesc).temp_color = temp_color_array[index as usize];
        }
        i = i.wrapping_add(1);
        pCurDesc = pCurDesc.offset(1);
    }
    sortPlanetPositions();
    if GlobData.Game_state.ip_planet == 0 {
        (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).PlanetDesc
            as *mut PLANET_DESC;
        (*pSolarSysState).pOrbitalDesc = ::core::ptr::null_mut::<PLANET_DESC>();
    } else {
        (*pSolarSysState).SunDesc[0usize].location = GlobData.Game_state.ip_location;
        GlobData.Game_state.ip_location = displayToLocation(
            GlobData.Game_state.ShipStamp.origin,
            ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE,
        );
        i = (GlobData.Game_state.ip_planet as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COUNT;
        (*pSolarSysState).pOrbitalDesc = (&raw mut (*pSolarSysState).PlanetDesc
            as *mut PLANET_DESC)
            .offset(i as isize);
        GenerateMoons(pSolarSysState, (*pSolarSysState).pOrbitalDesc);
        (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).MoonDesc
            as *mut PLANET_DESC;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_6::PLANETARY_LANDING.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_6::END_PLANETARY_LANDING.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    }
    initSolarSysSISCharacteristics();
    if GlobData.Game_state.in_orbit != 0 {
        i = (GlobData.Game_state.in_orbit as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COUNT;
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            orbital = (*(*pSolarSysState).pBaseDesc).pPrevDesc;
        } else {
            i = (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
            orbital = (&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                .offset(i as isize);
        }
        GlobData.Game_state.ip_location = (*pSolarSysState).SunDesc[0usize].location;
        GlobData.Game_state.in_orbit = 0 as BYTE;
    } else {
        i = GlobData.Game_state.ShipFacing as COUNT;
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = i.wrapping_add(1);
        }
        GlobData.Game_state.ShipStamp.frame = SetAbsFrameIndex(
            SISIPFrame,
            (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
        );
    }
    return orbital;
}
unsafe extern "C" fn saveNonOrbitalLocation() {
    GlobData.Game_state.ShipFacing = (GetFrameIndex(GlobData.Game_state.ShipStamp.frame)
        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as UWORD;
    GlobData.Game_state.in_orbit = 0 as BYTE;
    if !playerInInnerSystem() {
        GlobData.Game_state.ip_planet = 0 as BYTE;
    } else {
        GlobData.Game_state.ip_planet = (1 as ::core::ffi::c_int
            + planetIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                as ::core::ffi::c_int) as BYTE;
        GlobData.Game_state.ip_location = (*pSolarSysState).SunDesc[0usize].location;
    };
}
unsafe extern "C" fn FreeSolarSys() {
    if (*pSolarSysState).InIpFlight.0 != 0 {
        (*pSolarSysState).InIpFlight = BOOLEAN::FALSE;
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        {
            saveNonOrbitalLocation();
        }
    }
    DestroyDrawable(ReleaseDrawable(SolarSysFrame));
    SolarSysFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    StopMusic();
}
unsafe extern "C" fn getCollisionFrame(
    mut planet: *mut PLANET_DESC,
    mut WaitPlanet: COUNT,
) -> FRAME {
    if (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        && (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
            != WaitPlanet as ::core::ffi::c_int
    {
        return DecFrameIndex(stars_in_space)
    } else {
        return (*planet).image.frame
    };
}
unsafe extern "C" fn CheckIntersect() -> *mut PLANET_DESC {
    let mut i: COUNT = 0;
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut ShipIntersect: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    let mut PlanetIntersect: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    let mut NewWaitPlanet: COUNT = 0;
    let mut PlanetOffset: BYTE = 0;
    let mut MoonOffset: BYTE = 0;
    pCurDesc = (*(*pSolarSysState).pBaseDesc).pPrevDesc;
    PlanetOffset = (pCurDesc
        .offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC) + 1isize)
        as BYTE;
    MoonOffset = 1 as BYTE;
    ShipIntersect.IntersectStamp.origin = GlobData.Game_state.ShipStamp.origin;
    ShipIntersect.EndPoint = ShipIntersect.IntersectStamp.origin;
    ShipIntersect.IntersectStamp.frame = GlobData.Game_state.ShipStamp.frame;
    PlanetIntersect.IntersectStamp.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    PlanetIntersect.IntersectStamp.origin.y = (ScreenHeight
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    PlanetIntersect.EndPoint = PlanetIntersect.IntersectStamp.origin;
    PlanetIntersect.IntersectStamp.frame = getCollisionFrame(
        pCurDesc,
        (((MoonOffset as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int | PlanetOffset as ::core::ffi::c_int) as COUNT,
    );
    NewWaitPlanet = 0 as COUNT;
    if pCurDesc != &raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC
        && DrawablesIntersect(
            &raw mut ShipIntersect,
            &raw mut PlanetIntersect,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ) as ::core::ffi::c_int != 0
    {
        NewWaitPlanet = (((MoonOffset as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as UWORD as ::core::ffi::c_int | PlanetOffset as ::core::ffi::c_int)
            as COUNT;
        if (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            && (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
                != NewWaitPlanet as ::core::ffi::c_int
        {
            (*pSolarSysState).WaitIntersect = NewWaitPlanet;
            return pCurDesc;
        }
    }
    i = (*pCurDesc).NumPlanets as COUNT;
    pCurDesc = (*pSolarSysState).pBaseDesc;
    while i != 0 {
        PlanetIntersect.IntersectStamp.origin = (*pCurDesc).image.origin;
        PlanetIntersect.EndPoint = PlanetIntersect.IntersectStamp.origin;
        if playerInInnerSystem() {
            PlanetOffset = (*pCurDesc)
                .pPrevDesc
                .offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
                as BYTE;
            MoonOffset = (pCurDesc
                .offset_from(&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                + 2isize) as BYTE;
        } else {
            PlanetOffset = pCurDesc
                .offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
                as BYTE;
            MoonOffset = 0 as BYTE;
        }
        PlanetOffset = PlanetOffset.wrapping_add(1);
        PlanetIntersect.IntersectStamp.frame = getCollisionFrame(
            pCurDesc,
            (((MoonOffset as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int | PlanetOffset as ::core::ffi::c_int) as COUNT,
        );
        if DrawablesIntersect(
            &raw mut ShipIntersect,
            &raw mut PlanetIntersect,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ) != 0
        {
            NewWaitPlanet = (((MoonOffset as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                | PlanetOffset as ::core::ffi::c_int) as COUNT;
            if (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                break;
            }
            if (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
                != NewWaitPlanet as ::core::ffi::c_int
            {
                (*pSolarSysState).WaitIntersect = NewWaitPlanet;
                return pCurDesc;
            }
        }
        i = i.wrapping_sub(1);
        pCurDesc = pCurDesc.offset(1);
    }
    if (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        || NewWaitPlanet as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        (*pSolarSysState).WaitIntersect = NewWaitPlanet;
    }
    return ::core::ptr::null_mut::<PLANET_DESC>();
}
unsafe extern "C" fn GetOrbitRect(
    mut pRect: *mut RECT,
    mut dx: COORD,
    mut dy: COORD,
    mut radius: SIZE,
    mut xnumer: ::core::ffi::c_int,
    mut ynumer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    (*pRect).corner.x = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        + -(dx as ::core::ffi::c_int) as ::core::ffi::c_long
            * xnumer as ::core::ffi::c_long / denom as ::core::ffi::c_long) as COORD;
    (*pRect).corner.y = ((ScreenHeight
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        + -(dy as ::core::ffi::c_int) as ::core::ffi::c_long
            * ynumer as ::core::ffi::c_long / denom as ::core::ffi::c_long) as COORD;
    (*pRect).extent.width = (radius as ::core::ffi::c_long
        * (xnumer << 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        / denom as ::core::ffi::c_long) as COORD;
    (*pRect).extent.height = ((*pRect).extent.width as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
}
unsafe extern "C" fn GetPlanetOrbitRect(
    mut r: *mut RECT,
    mut planet: *mut PLANET_DESC,
    mut sizeNumer: ::core::ffi::c_int,
    mut dyNumer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut dx: COORD = 0;
    let mut dy: COORD = 0;
    dx = (*planet).radius as COORD;
    dy = (*planet).radius as COORD;
    if sizeNumer
        > (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - 8 as ::core::ffi::c_int
    {
        dx = (dx as ::core::ffi::c_int + (*planet).location.x as ::core::ffi::c_int)
            as COORD;
        dy = ((dy as ::core::ffi::c_int + (*planet).location.y as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as COORD;
    }
    GetOrbitRect(r, dx, dy, (*planet).radius, sizeNumer, dyNumer, denom);
}
unsafe extern "C" fn ValidateOrbit(
    mut planet: *mut PLANET_DESC,
    mut sizeNumer: ::core::ffi::c_int,
    mut dyNumer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut index: COUNT = 0;
    if sizeNumer
        <= (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - 8 as ::core::ffi::c_int
    {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        GetPlanetOrbitRect(&raw mut r, planet, sizeNumer, dyNumer, denom);
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        r.corner.x = (r.corner.x as ::core::ffi::c_long
            + (*planet).location.x as ::core::ffi::c_long
                * sizeNumer as ::core::ffi::c_long / denom as ::core::ffi::c_long)
            as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_long
            + (*planet).location.y as ::core::ffi::c_long
                * (sizeNumer / 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                / denom as ::core::ffi::c_long) as COORD;
        (*planet).image.origin = r.corner;
    }
    index = ((*planet).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_2::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)) as COUNT;
    if (index as ::core::ffi::c_int)
        < C2Rust_Unnamed_2::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
    {
        let mut Type: BYTE = 0;
        let mut Size: COUNT = 0;
        let mut angle: COUNT = 0;
        Type = (*PlanData.offset(index as isize)).Type;
        Size = (Type as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as BYTE as COUNT;
        if sizeNumer
            > (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int
        {
            Size = (Size as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as COUNT;
        } else if worldIsMoon(pSolarSysState, planet) {
            Size = (Size as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT;
        } else if denom
            <= (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int
        {
            Size = Size.wrapping_add(1);
            if denom
                == (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int
            {
                Size = Size.wrapping_add(1);
            }
        }
        if worldIsPlanet(pSolarSysState, planet) {
            angle = ARCTAN((*planet).location.x, (*planet).location.y);
        } else {
            angle = ARCTAN(
                (*(*planet).pPrevDesc).location.x,
                (*(*planet).pPrevDesc).location.y,
            );
        }
        (*planet).image.frame = SetAbsFrameIndex(
            OrbitalFrame,
            (((Size as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                + (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int)
                as COUNT,
        );
    } else if (*planet).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_2::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
    {
        (*planet).image.frame = SetAbsFrameIndex(SpaceJunkFrame, 16 as COUNT);
    } else if (*planet).data_index as ::core::ffi::c_int
        == C2Rust_Unnamed_2::SA_MATRA.0 as ::core::ffi::c_int
    {
        (*planet).image.frame = SetAbsFrameIndex(SpaceJunkFrame, 19 as COUNT);
    }
}
unsafe extern "C" fn DrawOrbit(
    mut planet: *mut PLANET_DESC,
    mut sizeNumer: ::core::ffi::c_int,
    mut dyNumer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetPlanetOrbitRect(&raw mut r, planet, sizeNumer, dyNumer, denom);
    SetContextForeGroundColor((*planet).temp_color);
    DrawOval(&raw mut r, 1 as BYTE);
}
unsafe extern "C" fn FindRadius(mut shipLoc: POINT, mut fromRadius: SIZE) -> SIZE {
    let mut nextRadius: SIZE = 0;
    let mut displayLoc: POINT = POINT { x: 0, y: 0 };
    loop {
        fromRadius = (fromRadius as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as SIZE;
        if fromRadius as ::core::ffi::c_int
            > (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int
        {
            nextRadius = (fromRadius as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as SIZE;
        } else {
            nextRadius = 0 as SIZE;
        }
        GetOrbitRect(
            &raw mut scaleRect,
            nextRadius,
            nextRadius,
            nextRadius,
            (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int,
            (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int,
            fromRadius as ::core::ffi::c_int,
        );
        displayLoc = locationToDisplay(shipLoc, fromRadius);
        if !pointWithinRect(scaleRect, displayLoc) {
            break;
        }
    }
    return fromRadius;
}
unsafe extern "C" fn flagship_inertial_thrust(mut CurrentAngle: COUNT) -> UWORD {
    let mut max_speed: BYTE = 0;
    let mut cur_delta_x: SIZE = 0;
    let mut cur_delta_y: SIZE = 0;
    let mut TravelAngle: COUNT = 0;
    let mut VelocityPtr: *mut VELOCITY_DESC = ::core::ptr::null_mut::<VELOCITY_DESC>();
    max_speed = (*pSolarSysState).max_ship_speed;
    VelocityPtr = &raw mut GlobData.Game_state.velocity;
    GetCurrentVelocityComponents(
        VelocityPtr,
        &raw mut cur_delta_x,
        &raw mut cur_delta_y,
    );
    TravelAngle = (*VelocityPtr).TravelAngle;
    if TravelAngle as ::core::ffi::c_int == CurrentAngle as ::core::ffi::c_int
        && cur_delta_x as ::core::ffi::c_int
            == (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long * max_speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
        && cur_delta_y as ::core::ffi::c_int
            == (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long * max_speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
    {
        return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD
    } else {
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut desired_speed: DWORD = 0;
        delta_x = (cur_delta_x as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long * 8 as ::core::ffi::c_int as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as SIZE;
        delta_y = (cur_delta_y as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long * 8 as ::core::ffi::c_int as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as SIZE;
        desired_speed = ((delta_x as ::core::ffi::c_long
            * delta_x as ::core::ffi::c_long) as DWORD)
            .wrapping_add(
                (delta_y as ::core::ffi::c_long * delta_y as ::core::ffi::c_long)
                    as DWORD,
            );
        if desired_speed
            <= (max_speed as UWORD as ::core::ffi::c_int
                * max_speed as ::core::ffi::c_int) as DWORD
        {
            SetVelocityComponents(VelocityPtr, delta_x, delta_y);
        } else if TravelAngle as ::core::ffi::c_int == CurrentAngle as ::core::ffi::c_int
        {
            SetVelocityComponents(
                VelocityPtr,
                (*(&raw mut sinetab as *mut SIZE)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as isize,
                    ) as ::core::ffi::c_long * max_speed as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SIZE,
                (*(&raw mut sinetab as *mut SIZE)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as isize,
                    ) as ::core::ffi::c_long * max_speed as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SIZE,
            );
            return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
        } else {
            let mut v: VELOCITY_DESC = VELOCITY_DESC {
                TravelAngle: 0,
                vector: EXTENT { width: 0, height: 0 },
                fract: EXTENT { width: 0, height: 0 },
                error: EXTENT { width: 0, height: 0 },
                incr: EXTENT { width: 0, height: 0 },
            };
            v = *VelocityPtr;
            DeltaVelocityComponents(
                &raw mut v,
                ((*(&raw mut sinetab as *mut SIZE)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as isize,
                    ) as ::core::ffi::c_long
                    * (8 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                    as ::core::ffi::c_int
                    - (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (TravelAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long
                        * 8 as ::core::ffi::c_int as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int)
                    as SIZE,
                ((*(&raw mut sinetab as *mut SIZE)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as isize,
                    ) as ::core::ffi::c_long
                    * (8 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                    as ::core::ffi::c_int
                    - (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (TravelAngle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long
                        * 8 as ::core::ffi::c_int as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int)
                    as SIZE,
            );
            GetCurrentVelocityComponents(
                &raw mut v,
                &raw mut cur_delta_x,
                &raw mut cur_delta_y,
            );
            desired_speed = ((cur_delta_x as ::core::ffi::c_long
                * cur_delta_x as ::core::ffi::c_long) as DWORD)
                .wrapping_add(
                    (cur_delta_y as ::core::ffi::c_long
                        * cur_delta_y as ::core::ffi::c_long) as DWORD,
                );
            if desired_speed
                > (max_speed as UWORD as ::core::ffi::c_int
                    * max_speed as ::core::ffi::c_int) as DWORD
            {
                SetVelocityComponents(
                    VelocityPtr,
                    (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (CurrentAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long * max_speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE,
                    (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (CurrentAngle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long * max_speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE,
                );
                return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
            }
            *VelocityPtr = v;
        }
        return 0 as UWORD;
    };
}
unsafe extern "C" fn ProcessShipControls() {
    let mut index: COUNT = 0;
    let mut delta_x: SIZE = 0;
    let mut delta_y: SIZE = 0;
    if CurrentInputState
        .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
        as usize][C2Rust_Unnamed::KEY_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        delta_y = -1 as SIZE;
    } else {
        delta_y = 0 as SIZE;
    }
    delta_x = 0 as SIZE;
    if CurrentInputState
        .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
        as usize][C2Rust_Unnamed::KEY_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        delta_x = (delta_x as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
    }
    if CurrentInputState
        .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
        as usize][C2Rust_Unnamed::KEY_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        delta_x = (delta_x as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
    }
    if delta_x as ::core::ffi::c_int != 0
        || (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
    } else if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
    {
        delta_y = -1 as SIZE;
    } else {
        delta_y = 0 as SIZE;
    }
    index = GetFrameIndex(GlobData.Game_state.ShipStamp.frame);
    if (*pSolarSysState).turn_counter != 0 {
        (*pSolarSysState).turn_counter = (*pSolarSysState).turn_counter.wrapping_sub(1);
    } else if delta_x != 0 {
        if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            index = (index as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
        } else {
            index = (index as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
        }
        GlobData.Game_state.ShipStamp.frame = SetAbsFrameIndex(
            GlobData.Game_state.ShipStamp.frame,
            index,
        );
        (*pSolarSysState).turn_counter = (*pSolarSysState).turn_wait;
    }
    if (*pSolarSysState).thrust_counter != 0 {
        (*pSolarSysState).thrust_counter = (*pSolarSysState)
            .thrust_counter
            .wrapping_sub(1);
    } else if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        flagship_inertial_thrust(
            ((index as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT,
        );
        (*pSolarSysState).thrust_counter = 1 as BYTE;
    }
}
unsafe extern "C" fn enterInnerSystem(mut planet: *mut PLANET_DESC) {
    let mut angle: COUNT = 0;
    angle = (((GetFrameIndex(GlobData.Game_state.ShipStamp.frame) as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COUNT;
    GlobData.Game_state.ShipStamp.origin.x = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        + (*(&raw mut sinetab as *mut SIZE)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as isize,
            ) as ::core::ffi::c_long
            * (35 as ::core::ffi::c_int
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    * 20 as ::core::ffi::c_int
                + 20 as ::core::ffi::c_int / 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
            as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.ShipStamp.origin.y = ((ScreenHeight
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        + (*(&raw mut sinetab as *mut SIZE)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as isize,
            ) as ::core::ffi::c_long
            * (35 as ::core::ffi::c_int
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    * 20 as ::core::ffi::c_int
                + 20 as ::core::ffi::c_int / 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
            as ::core::ffi::c_int) as COORD;
    if (GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.ShipStamp.origin.y = 1 as COORD;
    } else if GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int
        >= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
    {
        GlobData.Game_state.ShipStamp.origin.y = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COORD;
    }
    GlobData.Game_state.ip_location = displayToLocation(
        GlobData.Game_state.ShipStamp.origin,
        ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE,
    );
    (*pSolarSysState).SunDesc[0usize].location = planetOuterLocation(
        planetIndex(pSolarSysState, planet),
    );
    memset(
        &raw mut GlobData.Game_state.velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    GenerateMoons(pSolarSysState, planet);
    (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).MoonDesc
        as *mut PLANET_DESC;
    (*pSolarSysState).pOrbitalDesc = planet;
}
unsafe extern "C" fn leaveInnerSystem(mut planet: *mut PLANET_DESC) {
    let mut outerPlanetWait: COUNT = 0;
    (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).PlanetDesc
        as *mut PLANET_DESC;
    (*pSolarSysState).pOrbitalDesc = ::core::ptr::null_mut::<PLANET_DESC>();
    outerPlanetWait = (((0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | (planet.offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
            + 1isize) as BYTE as ::core::ffi::c_int) as COUNT;
    GlobData.Game_state.ip_location = (*pSolarSysState).SunDesc[0usize].location;
    XFormIPLoc(
        &raw mut GlobData.Game_state.ip_location,
        &raw mut GlobData.Game_state.ShipStamp.origin,
        BOOLEAN::TRUE,
    );
    memset(
        &raw mut GlobData.Game_state.velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    (*pSolarSysState).WaitIntersect = outerPlanetWait;
    CheckIntersect();
    if (*pSolarSysState).WaitIntersect as ::core::ffi::c_int
        != outerPlanetWait as ::core::ffi::c_int
    {
        (*pSolarSysState).WaitIntersect = !(0 as ::core::ffi::c_int) as COUNT;
    }
}
unsafe extern "C" fn enterOrbital(mut planet: *mut PLANET_DESC) {
    memset(
        &raw mut GlobData.Game_state.velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    (*pSolarSysState).pOrbitalDesc = planet;
    (*pSolarSysState).InOrbit = BOOLEAN::TRUE;
}
unsafe extern "C" fn CheckShipLocation(mut newRadius: *mut SIZE) -> BOOLEAN {
    let mut radius: SIZE = 0;
    radius = (*pSolarSysState).SunDesc[0usize].radius;
    *newRadius = (*pSolarSysState).SunDesc[0usize].radius;
    if (GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
        || GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_int
            >= ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int
        || (GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        || GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int
            >= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int
    {
        if !playerInInnerSystem() {
            if radius as ::core::ffi::c_int
                == (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            {
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    | C2Rust_Unnamed_7::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                    as ACTIVITY;
                return BOOLEAN::FALSE;
            }
            *newRadius = FindRadius(
                GlobData.Game_state.ip_location,
                (((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int) as SIZE,
            );
        } else {
            leaveInnerSystem((*pSolarSysState).pOrbitalDesc);
        }
        return BOOLEAN::TRUE;
    }
    if !playerInInnerSystem()
        && pointWithinRect(scaleRect, GlobData.Game_state.ShipStamp.origin)
            as ::core::ffi::c_int != 0
    {
        *newRadius = FindRadius(GlobData.Game_state.ip_location, radius);
        return BOOLEAN::TRUE;
    }
    if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int)
    {
        let mut planet: *mut PLANET_DESC = CheckIntersect();
        if !planet.is_null() {
            if playerInInnerSystem() {
                enterOrbital(planet);
                return BOOLEAN::FALSE;
            } else {
                enterInnerSystem(planet);
                return BOOLEAN::TRUE;
            }
        }
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn DrawSystemTransition(mut inner: BOOLEAN) {
    SetTransitionSource(::core::ptr::null::<RECT>());
    BatchGraphics();
    if inner.0 != 0 {
        DrawInnerSystem();
    } else {
        DrawOuterSystem();
    }
    RedrawQueue(BOOLEAN::FALSE);
    ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>());
    UnbatchGraphics();
}
unsafe extern "C" fn TransitionSystemIn() {
    SetContext(SpaceContext);
    DrawSystemTransition(BOOLEAN(playerInInnerSystem() as ::core::ffi::c_uint));
}
unsafe extern "C" fn ScaleSystem(mut new_radius: SIZE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    (*pSolarSysState).SunDesc[0usize].radius = new_radius;
    XFormIPLoc(
        &raw mut GlobData.Game_state.ip_location,
        &raw mut GlobData.Game_state.ShipStamp.origin,
        BOOLEAN::TRUE,
    );
    GetContextClipRect(&raw mut r);
    SetTransitionSource(&raw mut r);
    BatchGraphics();
    DrawOuterSystem();
    RedrawQueue(BOOLEAN::FALSE);
    ScreenTransition(3 as ::core::ffi::c_int, &raw mut r);
    UnbatchGraphics();
}
unsafe extern "C" fn RestoreSystemView() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SolarSysFrame;
    DrawStamp(&raw mut s);
}
unsafe extern "C" fn IP_frame() {
    let mut locChange: BOOLEAN = BOOLEAN::FALSE;
    let mut newRadius: SIZE = 0;
    SetContext(SpaceContext);
    GameClockTick();
    ProcessShipControls();
    locChange = CheckShipLocation(&raw mut newRadius);
    if locChange.0 != 0 {
        if playerInInnerSystem() {
            DrawSystemTransition(BOOLEAN::TRUE);
        } else if (*pSolarSysState).SunDesc[0usize].radius as ::core::ffi::c_int
            == newRadius as ::core::ffi::c_int
        {
            DrawSystemTransition(BOOLEAN::FALSE);
        } else {
            ScaleSystem(newRadius);
        }
    } else {
        BatchGraphics();
        RestoreSystemView();
        RedrawQueue(BOOLEAN::FALSE);
        DrawAutoPilotMessage(BOOLEAN::FALSE);
        UnbatchGraphics();
    };
}
unsafe extern "C" fn CheckZoomLevel() -> BOOLEAN {
    let mut InnerSystem: BOOLEAN = BOOLEAN::FALSE;
    let mut shipLoc: POINT = POINT { x: 0, y: 0 };
    InnerSystem = BOOLEAN(playerInInnerSystem() as ::core::ffi::c_uint);
    if InnerSystem.0 != 0 {
        shipLoc = (*pSolarSysState).SunDesc[0usize].location;
    } else {
        shipLoc = GlobData.Game_state.ip_location;
    }
    (*pSolarSysState).SunDesc[0usize].radius = FindRadius(
        shipLoc,
        (((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as SIZE,
    );
    if InnerSystem.0 == 0 {
        XFormIPLoc(
            &raw mut shipLoc,
            &raw mut GlobData.Game_state.ShipStamp.origin,
            BOOLEAN::TRUE,
        );
    }
    return InnerSystem;
}
unsafe extern "C" fn ValidateOrbits() {
    let mut i: COUNT = 0;
    let mut planet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    i = (*pSolarSysState).SunDesc[0usize].NumPlanets as COUNT;
    planet = (&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC).offset(0isize);
    while i != 0 {
        ValidateOrbit(
            planet,
            (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int,
            ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int,
            (*pSolarSysState).SunDesc[0usize].radius as ::core::ffi::c_int,
        );
        i = i.wrapping_sub(1);
        planet = planet.offset(1);
    }
}
unsafe extern "C" fn ValidateInnerOrbits() {
    let mut i: COUNT = 0;
    let mut planet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    (playerInInnerSystem() as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"playerInInnerSystem ()\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int,
                b"ValidateInnerOrbits\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    planet = (*pSolarSysState).pOrbitalDesc;
    ValidateOrbit(
        planet,
        ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - 8 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int,
        (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - 8 as ::core::ffi::c_int,
        (*planet).radius as ::core::ffi::c_int,
    );
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
        let mut moon: *mut PLANET_DESC = (&raw mut (*pSolarSysState).MoonDesc
            as *mut PLANET_DESC)
            .offset(i as isize);
        ValidateOrbit(
            moon,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawInnerSystem() {
    ValidateInnerOrbits();
    DrawSystem((*(*pSolarSysState).pOrbitalDesc).radius, BOOLEAN::TRUE);
    DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE);
}
unsafe extern "C" fn DrawOuterSystem() {
    ValidateOrbits();
    DrawSystem((*pSolarSysState).SunDesc[0usize].radius, BOOLEAN::FALSE);
    DrawHyperCoords((*CurStarDescPtr).star_pt);
}
unsafe extern "C" fn ResetSolarSys() {
    ((*pSolarSysState).InIpFlight.0 == 0
        || {
            __assert_fail(
                b"!pSolarSysState->InIpFlight\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1227 as ::core::ffi::c_int,
                b"ResetSolarSys\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    DrawMenuStateStrings(
        C2Rust_Unnamed_1::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
        -(C2Rust_Unnamed_1::PM_NAVIGATE.0 as ::core::ffi::c_int
            - C2Rust_Unnamed_1::PM_SCAN.0 as ::core::ffi::c_int) as SWORD,
    );
    InitDisplayList();
    DoMissions();
    CheckIntersect();
    (*pSolarSysState).InIpFlight = BOOLEAN::TRUE;
    if PLRPlaying(
            ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize),
        )
        .0 == 0
        && (LastActivity as ::core::ffi::c_int
            != C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
            || NextActivity as ::core::ffi::c_int != 0)
    {
        PlayMusic(SpaceMusic, BOOLEAN::TRUE, 1 as BYTE);
    }
}
unsafe extern "C" fn EnterPlanetOrbit() {
    if (*pSolarSysState).InIpFlight.0 != 0 {
        FreeSolarSys();
        if worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
            GlobData.Game_state.ShipStamp.origin = (*(*pSolarSysState).pOrbitalDesc)
                .image
                .origin;
        } else {
            GlobData.Game_state.ShipStamp.origin.x = (ScreenWidth
                - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
            GlobData.Game_state.ShipStamp.origin.y = (ScreenHeight
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        }
    }
    GetPlanetInfo();
    Some(
            (*(*pSolarSysState).genFuncs)
                .generateOrbital
                .expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )(pSolarSysState, (*pSolarSysState).pOrbitalDesc);
    LastActivity = (LastActivity as ::core::ffi::c_int
        & !(C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_RESTART.0 as ::core::ffi::c_int)) as ACTIVITY;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::START_ENCOUNTER.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        || getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_6::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_6::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
    {
        return;
    }
    if !(*pSolarSysState).TopoFrame.is_null() {
        PlanetOrbitMenu();
        FreePlanet();
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_7::START_ENCOUNTER.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        let mut orbital: *mut PLANET_DESC = LoadSolarSys();
        (orbital.is_null()
            || {
                __assert_fail(
                    b"!orbital\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/solarsys.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1304 as ::core::ffi::c_int,
                    b"EnterPlanetOrbit\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        CheckZoomLevel();
        ValidateOrbits();
        ValidateInnerOrbits();
        ResetSolarSys();
        RepairSISBorder();
        TransitionSystemIn();
    }
}
unsafe extern "C" fn InitSolarSys() {
    let mut InnerSystem: BOOLEAN = BOOLEAN::FALSE;
    let mut Reentry: BOOLEAN = BOOLEAN::FALSE;
    let mut orbital: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    LoadIPData();
    LoadLanderData();
    Reentry = BOOLEAN(
        (GlobData.Game_state.ShipFacing as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if Reentry.0 == 0 {
        GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.ShipStamp.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.ShipStamp.origin.y = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.ip_location = displayToLocation(
            GlobData.Game_state.ShipStamp.origin,
            ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE,
        );
    }
    StarsFrame = CreateStarBackGround();
    SetContext(SpaceContext);
    SetContextFGFrame(Screen);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    orbital = LoadSolarSys();
    InnerSystem = CheckZoomLevel();
    ValidateOrbits();
    if InnerSystem.0 != 0 {
        ValidateInnerOrbits();
    }
    if Reentry.0 != 0 {
        Some(
                (*(*pSolarSysState).genFuncs)
                    .reinitNpcs
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(pSolarSysState);
    } else {
        EncounterRace = -1 as SIZE;
        EncounterGroup = 0 as BYTE;
        GlobData.Game_state.BattleGroupRef = 0 as DWORD;
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        Some((*(*pSolarSysState).genFuncs).initNpcs.expect("non-null function pointer"))
            .expect("non-null function pointer")(pSolarSysState);
    }
    if !orbital.is_null() {
        enterOrbital(orbital);
    } else {
        SetContext(SpaceContext);
        SetTransitionSource(::core::ptr::null::<RECT>());
        BatchGraphics();
        DrawSISFrame();
        DrawSISMessage(::core::ptr::null::<UNICODE>());
        ResetSolarSys();
        if LastActivity as ::core::ffi::c_int
            == C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_7::CHECK_RESTART.0 as ::core::ffi::c_int
        {
            DrawOuterSystem();
            RedrawQueue(BOOLEAN::FALSE);
            UnbatchGraphics();
            FadeScreen(
                ScreenFadeType::FadeAllToColor,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            );
            LastActivity = 0 as ACTIVITY;
        } else if LastActivity as ::core::ffi::c_int
            == C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int && NextActivity == 0
        {
            UnbatchGraphics();
        } else {
            if InnerSystem.0 != 0 {
                DrawInnerSystem();
            } else {
                DrawOuterSystem();
            }
            RedrawQueue(BOOLEAN::FALSE);
            ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>());
            UnbatchGraphics();
            LastActivity = (LastActivity as ::core::ffi::c_int
                & !(C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
        }
    };
}
unsafe extern "C" fn endInterPlanetary() {
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int
        & !(C2Rust_Unnamed_7::START_INTERPLANETARY.0 as ::core::ffi::c_int)) as ACTIVITY;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        Some(
                (*(*pSolarSysState).genFuncs)
                    .uninitNpcs
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(pSolarSysState);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_6::USED_BROADCASTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_6::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    }
}
unsafe extern "C" fn closestPlanetInterPlanetary(
    mut point: *const POINT,
) -> *mut PLANET_DESC {
    let mut i: BYTE = 0;
    let mut numPlanets: BYTE = 0;
    let mut bestDistSquared: DWORD = 0;
    let mut bestPlanet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    (!pSolarSysState.is_null()
        || {
            __assert_fail(
                b"pSolarSysState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1436 as ::core::ffi::c_int,
                b"closestPlanetInterPlanetary\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numPlanets = (*pSolarSysState).SunDesc[0usize].NumPlanets;
    bestDistSquared = -1 as ::core::ffi::c_int as DWORD;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < numPlanets as ::core::ffi::c_int {
        let mut planet: *mut PLANET_DESC = (&raw mut (*pSolarSysState).PlanetDesc
            as *mut PLANET_DESC)
            .offset(i as isize);
        let mut dx: SIZE = ((*point).x as ::core::ffi::c_int
            - (*planet).image.origin.x as ::core::ffi::c_int) as SIZE;
        let mut dy: SIZE = ((*point).y as ::core::ffi::c_int
            - (*planet).image.origin.y as ::core::ffi::c_int) as SIZE;
        let mut distSquared: DWORD = (dx as ::core::ffi::c_long
            * dx as ::core::ffi::c_long
            + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD;
        if distSquared < bestDistSquared {
            bestDistSquared = distSquared;
            bestPlanet = planet;
        }
        i = i.wrapping_add(1);
    }
    return bestPlanet;
}
unsafe extern "C" fn UninitSolarSys() {
    FreeSolarSys();
    DestroyDrawable(ReleaseDrawable(StarsFrame));
    StarsFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::START_INTERPLANETARY.0 as ::core::ffi::c_int != 0
    {
        endInterPlanetary();
        return;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
        && EncounterGroup as ::core::ffi::c_int != 0
    {
        GetGroupInfo(GlobData.Game_state.BattleGroupRef, EncounterGroup);
        if GlobData.Game_state.ip_planet as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let mut planet: *mut PLANET_DESC = closestPlanetInterPlanetary(
                &raw mut GlobData.Game_state.ShipStamp.origin,
            );
            Some(
                    (*(*pSolarSysState).genFuncs)
                        .generateName
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(pSolarSysState, planet);
        }
    }
}
unsafe extern "C" fn CalcSunSize(mut pSunDesc: *mut PLANET_DESC, mut radius: SIZE) {
    let mut index: SIZE = 0 as SIZE;
    if radius as ::core::ffi::c_int
        <= (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        index += 1;
        if radius as ::core::ffi::c_int
            <= (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int
        {
            index += 1;
        }
    }
    (*pSunDesc).image.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    (*pSunDesc).image.origin.y = (ScreenHeight
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    (*pSunDesc).image.frame = SetRelFrameIndex(SunFrame, index);
}
unsafe extern "C" fn SetPlanetColorMap(mut planet: *mut PLANET_DESC) {
    let mut index: COUNT = ((*planet).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_2::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)) as COUNT;
    ((index as ::core::ffi::c_int)
        < C2Rust_Unnamed_2::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"index < NUMBER_OF_PLANET_TYPES\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1513 as ::core::ffi::c_int,
                b"SetPlanetColorMap\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    SetColorMap(
        GetColorMapAddress(
            SetAbsStringTableIndex(
                OrbitalCMap,
                ((*PlanData.offset(index as isize)).Type as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as COUNT,
            ),
        ),
    );
}
unsafe extern "C" fn DrawInnerPlanets(mut planet: *mut PLANET_DESC) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    let mut moon: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    SetPlanetColorMap(planet);
    s.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    s.origin.y = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    s.frame = (*planet).image.frame;
    i = ((*planet).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_2::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)) as COUNT;
    if (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_2::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
        && (*planet).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_2::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
    {
        s.frame = SetAbsFrameIndex(SpaceJunkFrame, 17 as COUNT);
    }
    DrawStamp(&raw mut s);
    i = (*planet).NumPlanets as COUNT;
    moon = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
    while i != 0 {
        if (*moon).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_2::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
        {
            SetPlanetColorMap(moon);
        }
        DrawStamp(&raw mut (*moon).image);
        i = i.wrapping_sub(1);
        moon = moon.offset(1);
    }
}
unsafe extern "C" fn DrawSystem(mut radius: SIZE, mut IsInnerSystem: BOOLEAN) {
    let mut i: BYTE = 0;
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut pBaseDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if SolarSysFrame.is_null() {
        let mut clipRect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        GetContextClipRect(&raw mut clipRect);
        SolarSysFrame = CaptureDrawable(
            CreateDrawable(
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
                clipRect.extent.width,
                clipRect.extent.height,
                1 as COUNT,
            ),
        );
    }
    oldContext = SetContext(OffScreenContext);
    SetContextFGFrame(SolarSysFrame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    DrawStarBackGround();
    pBaseDesc = (*pSolarSysState).pBaseDesc;
    if IsInnerSystem.0 != 0 {
        pCurDesc = (*pSolarSysState).pOrbitalDesc;
        DrawOrbit(
            pCurDesc,
            ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int,
            (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int,
            radius as ::core::ffi::c_int,
        );
    }
    i = (*(*pBaseDesc).pPrevDesc).NumPlanets;
    pCurDesc = pBaseDesc;
    while i != 0 {
        if IsInnerSystem.0 != 0 {
            DrawOrbit(
                pCurDesc,
                2 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
        } else {
            DrawOrbit(
                pCurDesc,
                (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - 8 as ::core::ffi::c_int,
                ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - 8 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int,
                radius as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_sub(1);
        pCurDesc = pCurDesc.offset(1);
    }
    if IsInnerSystem.0 != 0 {
        DrawInnerPlanets((*pSolarSysState).pOrbitalDesc);
    } else {
        let mut index: SIZE = 0;
        CalcSunSize(
            (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC).offset(0isize),
            radius,
        );
        index = (*pSolarSysState).FirstPlanetIndex;
        loop {
            pCurDesc = solarDisplayBody(index);
            if pCurDesc
                == (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC)
                    .offset(0isize)
            {
                SetColorMap(
                    GetColorMapAddress(
                        SetAbsStringTableIndex(
                            SunCMap,
                            (((*CurStarDescPtr).Type as ::core::ffi::c_int
                                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                    as BYTE as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int)
                                as BYTE as COUNT,
                        ),
                    ),
                );
            } else {
                SetPlanetColorMap(pCurDesc);
            }
            DrawStamp(&raw mut (*pCurDesc).image);
            if index as ::core::ffi::c_int
                == (*pSolarSysState).LastPlanetIndex as ::core::ffi::c_int
            {
                break;
            }
            index = (*pCurDesc).NextIndex as SIZE;
        }
    }
    SetContext(oldContext);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SolarSysFrame;
    DrawStamp(&raw mut s);
}
#[no_mangle]
pub unsafe extern "C" fn DrawStarBackGround() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = StarsFrame;
    DrawStamp(&raw mut s);
}
unsafe extern "C" fn CreateStarBackGround() -> FRAME {
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    let mut rand_val: DWORD = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut clipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    oldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut clipRect);
    frame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            clipRect.extent.width,
            clipRect.extent.height,
            1 as COUNT,
        ),
    );
    SetContext(OffScreenContext);
    SetContextFGFrame(frame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    ClearDrawable();
    RandomContext_SeedRandom(SysGenRNG, GetRandomSeedForStar(CurStarDescPtr));
    s.frame = SpaceJunkFrame;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        j = 0 as COUNT;
        while (j as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            rand_val = RandomContext_Random(SysGenRNG);
            s.origin.x = (rand_val as UWORD as ::core::ffi::c_int
                % (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int)) as COORD;
            s.origin.y = ((rand_val >> 16 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                % (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int)) as COORD;
            DrawStamp(&raw mut s);
            j = j.wrapping_add(1);
        }
        s.frame = IncFrameIndex(s.frame);
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        j = 0 as COUNT;
        while (j as ::core::ffi::c_int) < 30 as ::core::ffi::c_int {
            rand_val = RandomContext_Random(SysGenRNG);
            s.origin.x = (rand_val as UWORD as ::core::ffi::c_int
                % (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int)) as COORD;
            s.origin.y = ((rand_val >> 16 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                % (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int)) as COORD;
            DrawStamp(&raw mut s);
            j = j.wrapping_add(1);
        }
        s.frame = IncFrameIndex(s.frame);
        i = i.wrapping_add(1);
    }
    SetContext(oldContext);
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn XFormIPLoc(
    mut pIn: *mut POINT,
    mut pOut: *mut POINT,
    mut ToDisplay: BOOLEAN,
) {
    if ToDisplay.0 != 0 {
        *pOut = locationToDisplay(*pIn, (*pSolarSysState).SunDesc[0usize].radius);
    } else {
        *pOut = displayToLocation(*pIn, (*pSolarSysState).SunDesc[0usize].radius);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ExploreSolarSys() {
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
    if CurStarDescPtr.is_null() {
        let mut universe: POINT = POINT { x: 0, y: 0 };
        universe.x = logxToUniverse(GlobData.SIS_state.log_x);
        universe.y = logyToUniverse(GlobData.SIS_state.log_y);
        CurStarDescPtr = FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut universe,
            1 as SIZE,
            1 as SIZE,
        );
        if CurStarDescPtr.is_null() {
            log_add(
                log_Level::log_Fatal,
                b"ExploreSolarSys(): do not know where you are!\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            explode();
        }
    }
    GlobData.SIS_state.log_x = universeToLogx((*CurStarDescPtr).star_pt.x);
    GlobData.SIS_state.log_y = universeToLogy((*CurStarDescPtr).star_pt.y);
    pSolarSysState = &raw mut SolarSysState;
    memset(
        pSolarSysState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SOLARSYS_STATE>(),
    );
    SolarSysState.genFuncs = getGenerateFunctions((*CurStarDescPtr).Index);
    InitSolarSys();
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    SolarSysState.InputFunc = Some(
        DoIpFlight as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>;
    DoInput(&raw mut SolarSysState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    UninitSolarSys();
    pSolarSysState = ::core::ptr::null_mut::<SOLARSYS_STATE>();
}
#[no_mangle]
pub unsafe extern "C" fn GetNamedPlanetaryBody() -> *mut UNICODE {
    if CurStarDescPtr.is_null() || !playerInSolarSystem() || !playerInInnerSystem() {
        return ::core::ptr::null_mut::<UNICODE>();
    }
    (!(*pSolarSysState).pOrbitalDesc.is_null()
        || {
            __assert_fail(
                b"pSolarSysState->pOrbitalDesc != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1753 as ::core::ffi::c_int,
                b"GetNamedPlanetaryBody\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*CurStarDescPtr).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_4::SOL_DEFINED.0 as ::core::ffi::c_int
    {
        let mut planet: ::core::ffi::c_int = 0;
        let mut moon: ::core::ffi::c_int = 0;
        planet = planetIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
            as ::core::ffi::c_int;
        if worldIsPlanet(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                        + planet) as COUNT,
                ),
            );
        }
        moon = moonIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
            as ::core::ffi::c_int;
        match planet {
            2 => {
                match moon {
                    0 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_3::STARBASE_STRING_BASE.0
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    1 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    _ => {}
                }
            }
            4 => {
                match moon {
                    0 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    1 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    2 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    3 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    _ => {}
                }
            }
            5 => {
                if moon == 0 as ::core::ffi::c_int {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                                + 14 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            7 => {
                if moon == 0 as ::core::ffi::c_int {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                                + 15 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            _ => {}
        }
    } else if (*CurStarDescPtr).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_4::SPATHI_DEFINED.0 as ::core::ffi::c_int
    {
        matchWorld(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            0 as BYTE,
            -1 as ::core::ffi::c_int as BYTE,
        );
    } else if (*CurStarDescPtr).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_4::SAMATRA_DEFINED.0 as ::core::ffi::c_int
    {
        if matchWorld(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            4 as BYTE,
            0 as BYTE,
        ) {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
    }
    return ::core::ptr::null_mut::<UNICODE>();
}
#[no_mangle]
pub unsafe extern "C" fn GetPlanetOrMoonName(mut buf: *mut UNICODE, mut bufsize: COUNT) {
    let mut named: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut moon: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    named = GetNamedPlanetaryBody();
    if !named.is_null() {
        utf8StringCopy(buf, bufsize as size_t, named);
        return;
    }
    utf8StringCopy(
        buf,
        bufsize as size_t,
        &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
    );
    if !playerInSolarSystem() || !playerInInnerSystem()
        || worldIsPlanet(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
            as ::core::ffi::c_int != 0
    {
        return;
    }
    i = strlen(buf) as ::core::ffi::c_int;
    buf = buf.offset(i as isize);
    bufsize = (bufsize as ::core::ffi::c_int - i) as COUNT;
    moon = moonIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
        as ::core::ffi::c_int;
    if bufsize as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
        snprintf(
            buf,
            bufsize as size_t,
            b"-%c\0".as_ptr() as *const ::core::ffi::c_char,
            'A' as ::core::ffi::c_int + moon,
        );
        *buf
            .offset(
                (bufsize as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) = '\0' as UNICODE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SaveSolarSysLocation() {
    (playerInSolarSystem() as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"playerInSolarSystem ()\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1862 as ::core::ffi::c_int,
                b"SaveSolarSysLocation\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !playerInPlanetOrbit() {
        saveNonOrbitalLocation();
    } else {
        let mut moon: BYTE = 0;
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_6::PLANETARY_CHANGE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_6::END_PLANETARY_CHANGE.0 as ::core::ffi::c_int,
        ) != 0
        {
            PutPlanetInfo();
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_6::PLANETARY_CHANGE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_6::END_PLANETARY_CHANGE.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        }
        (GlobData.Game_state.ip_planet as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"GLOBAL (ip_planet) != 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/solarsys.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1884 as ::core::ffi::c_int,
                    b"SaveSolarSysLocation\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        moon = 1 as BYTE;
        if worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
            moon = (moon as ::core::ffi::c_int
                + moonIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                    as ::core::ffi::c_int) as BYTE;
            moon = (moon as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
        }
        GlobData.Game_state.in_orbit = moon;
    };
}
unsafe extern "C" fn DoSolarSysMenu(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    let mut handled: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    handled = DoMenuChooser(
        pMS,
        C2Rust_Unnamed_1::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
    );
    if handled.0 != 0 {
        return BOOLEAN::TRUE;
    }
    if LastActivity as ::core::ffi::c_int
        == C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
    {
        select = BOOLEAN::TRUE;
    }
    if select.0 == 0 {
        return BOOLEAN::TRUE;
    }
    SetFlashRect(::core::ptr::null::<RECT>());
    's_104: {
        match (*pMS).CurState as ::core::ffi::c_int {
            2 => {
                select = DevicesMenu();
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_7::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                break 's_104;
            }
            3 => {
                CargoMenu();
                break 's_104;
            }
            4 => {
                select = RosterMenu();
                break 's_104;
            }
            5 => {
                if GameOptions().0 == 0 {
                    return BOOLEAN::FALSE;
                }
                break 's_104;
            }
            1 => {
                StarMap();
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                TransitionSystemIn();
            }
            6 => {}
            _ => {
                break 's_104;
            }
        }
        return BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if select.0 != 0 {
            if optWhichMenu != 0x2 as ::core::ffi::c_int {
                (*pMS).CurState = SolarSysMenuMenuItems::NAVIGATION.0
                    as ::core::ffi::c_int as BYTE;
            }
            DrawMenuStateStrings(
                C2Rust_Unnamed_1::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            );
        }
        SetFlashRect(
            ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(0 as ::core::ffi::c_long) as usize),
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn SolarSysMenu() {
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
    if LastActivity as ::core::ffi::c_int
        == C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
    {
        MenuState.CurState = SolarSysMenuMenuItems::GAME_MENU.0 as ::core::ffi::c_int
            as BYTE;
    } else {
        DrawMenuStateStrings(
            C2Rust_Unnamed_1::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
            SolarSysMenuMenuItems::STARMAP.0 as ::core::ffi::c_int as SWORD,
        );
        MenuState.CurState = SolarSysMenuMenuItems::STARMAP.0 as ::core::ffi::c_int
            as BYTE;
    }
    DrawStatusMessage(::core::ptr::null::<UNICODE>());
    SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
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
    MenuState.InputFunc = Some(
        DoSolarSysMenu as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    DrawMenuStateStrings(
        C2Rust_Unnamed_1::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
        -(SolarSysMenuMenuItems::NAVIGATION.0 as ::core::ffi::c_int) as SWORD,
    );
}
unsafe extern "C" fn DoIpFlight(mut pSS: *mut SOLARSYS_STATE) -> BOOLEAN {
    static mut NextTime: TimeCount = 0;
    let mut cancel: BOOLEAN = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if (*pSS).InOrbit.0 != 0 {
        EnterPlanetOrbit();
        SetMenuSounds(
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        );
        (*pSS).InOrbit = BOOLEAN::FALSE;
    } else if cancel.0 != 0
        || LastActivity as ::core::ffi::c_int
            == C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int
    {
        SolarSysMenu();
        SetMenuSounds(
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        );
    } else {
        ((*pSS).InIpFlight.0 != 0
            || {
                __assert_fail(
                    b"pSS->InIpFlight\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/solarsys.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    2011 as ::core::ffi::c_int,
                    b"DoIpFlight\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        IP_frame();
        SleepThreadUntil(NextTime);
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            );
    }
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_7::START_ENCOUNTER.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_7::START_INTERPLANETARY.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
