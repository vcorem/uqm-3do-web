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
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type RandomContext;
    pub type FlashContext;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn ApplyMask(layer: FRAME, base: FRAME, mode: DrawMode, fill: *mut Color);
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
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn LoadGraphicFile(pStr: *const ::core::ffi::c_char) -> DRAWABLE;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CopyFrameRect(Frame: FRAME, area: *const RECT) -> DRAWABLE;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapTableIndex(map: COLORMAP) -> UBYTE;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn GetTimeCounter() -> TimeCount;
    fn abort() -> !;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn GetDirectionalJoystickInput(
        direction: ::core::ffi::c_int,
        player: ::core::ffi::c_int,
    ) -> BATTLE_INPUT_STATE;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optNebulae: OPT_ENABLABLE;
    static mut optOrbitingPlanets: OPT_ENABLABLE;
    static mut optTexturedPlanets: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut spaceMusicBySOI: ::core::ffi::c_int;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optSmartAutoPilot: OPT_ENABLABLE;
    static mut optPlanetStyle: ::core::ffi::c_int;
    static mut optStarBackground: ::core::ffi::c_int;
    static mut optHyperStars: OPT_ENABLABLE;
    static mut optPlanetTexture: OPT_ENABLABLE;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optUnscaledStarSystem: OPT_ENABLABLE;
    static mut optNebulaeVolume: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut DirJoyActive: BOOLEAN;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn ReinitQueue(pq: *mut QUEUE);
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn PLRPlaying(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn SetMusicPosition();
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn LoadLanderData();
    fn daysElapsed() -> ::core::ffi::c_float;
    fn GameClockTick();
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn DrawAutoPilotMessage(Reset: BOOLEAN);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHyperSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Delete(context: *mut RandomContext);
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn RandomContext_FastRandom(seed: DWORD) -> DWORD;
    fn DestroyOrbitStruct(Orbit: *mut PLANET_ORBIT, height: SIZE);
    fn FreePlanet();
    fn DrawOval(pRect: *mut DRECT, num_off_pixels: BYTE, scaled: BOOLEAN);
    fn PrepareNextRotationFrameForIP(pPlanetDesc: *mut PLANET_DESC, frameCounter: SIZE);
    fn GeneratePlanetSurface(
        pPlanetDesc: *mut PLANET_DESC,
        SurfDefFrame: FRAME,
        width: COUNT,
        height: COUNT,
    );
    fn PlanetOrbitMenu();
    fn DoPlanetaryAnalysis(SysInfoPtr: *mut SYSTEM_INFO, pPlanetDesc: *mut PLANET_DESC);
    static mut PlanData: *const PlanetFrame;
    fn getGenerateFunctions(Index: BYTE) -> *const GenerateFunctions;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut plot_map: [PLOT_LOCATION; 70];
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn isStarMarked(
        star_index: ::core::ffi::c_int,
        marker_state: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn setStarMarked(
        star_index: ::core::ffi::c_int,
        marker_state: *const ::core::ffi::c_char,
    );
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn GameOptions() -> BOOLEAN;
    fn SettingsMenu(NameFlagship: BOOLEAN);
    static mut NewGameInit: BOOLEAN;
    fn DoMissions();
    fn CargoMenu();
    fn RosterMenu() -> BOOLEAN;
    fn DevicesMenu() -> BOOLEAN;
    fn StarMap() -> BOOLEAN;
    static mut stars_in_space: FRAME;
    fn StopMusic();
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    static mut GameStrings: STRING;
    fn RedrawQueue(clear: BOOLEAN);
    fn InitDisplayList();
    static mut Screen: FRAME;
    static mut OffScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut SpaceMusicOK: BOOLEAN;
    static mut classicPackPresent: BOOLEAN;
    static mut LastActivity: ACTIVITY;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn GetPlanetInfo();
    fn PutPlanetInfo();
    fn GetGroupInfo(offset: DWORD, which_group: BYTE) -> BOOLEAN;
    fn findRaceSOI();
    static mut NextActivity: ACTIVITY;
    fn SetGraphicScale(scale: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetGraphicScaleMode(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn GetColorMapColor(ColorMapIndex: COUNT, ColorIndex: COUNT) -> Color;
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
pub struct dextent {
    pub width: SDWORD,
    pub height: SDWORD,
}
pub type DEXTENT = dextent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpoint {
    pub x: SDWORD,
    pub y: SDWORD,
}
pub type DPOINT = dpoint;
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
pub struct drect {
    pub corner: DPOINT,
    pub extent: DEXTENT,
}
pub type DRECT = drect;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type HOT_SPOT = POINT;
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
pub type TimeCount = DWORD;
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
    pub const KEY_THRUST: Self = Self(7);
    pub const NUM_KEYS: Self = Self(8);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_PL_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_PL_2: Self = Self(1);
    pub const NUM_TEMPLATES: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
pub type BATTLE_INPUT_STATE = UBYTE;
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
}
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const CHMMR_HOME: Self = Self(0);
    pub const ORZ_HOME: Self = Self(1);
    pub const PKUNK_HOME: Self = Self(2);
    pub const SHOFIXTI_HOME: Self = Self(3);
    pub const SPATHI_HOME: Self = Self(4);
    pub const SUPOX_HOME: Self = Self(5);
    pub const THRADDASH_HOME: Self = Self(6);
    pub const UTWIG_HOME: Self = Self(7);
    pub const VUX_HOME: Self = Self(8);
    pub const YEHAT_HOME: Self = Self(9);
    pub const DRUUGE_HOME: Self = Self(10);
    pub const ILWRATH_HOME: Self = Self(11);
    pub const MYCON_HOME: Self = Self(12);
    pub const SLYLANDRO_HOME: Self = Self(13);
    pub const UMGAH_HOME: Self = Self(14);
    pub const ZOQFOTPIK_HOME: Self = Self(15);
    pub const SYREEN_HOME: Self = Self(16);
    pub const ANDROSYNTH_HOME: Self = Self(17);
    pub const NUM_AVAILABLE_HOMES: Self = Self(18);
}
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
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
pub type MENU_SOUND_FLAGS = UWORD;
pub type MENU_STATE = menu_state;
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
    pub flash_rect2: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
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
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
    pub const PM_READ_VERY_SLOW: Self = Self(36);
    pub const PM_READ_SLOW: Self = Self(37);
    pub const PM_READ_MODERATE: Self = Self(38);
    pub const PM_READ_FAST: Self = Self(39);
    pub const PM_READ_VERY_FAST: Self = Self(40);
    pub const PM_CHANGE_CAPTAIN: Self = Self(41);
    pub const PM_CHANGE_SHIP: Self = Self(42);
    pub const PM_EXIT_SETTINGS: Self = Self(43);
    pub const PM_ALT_SCAN: Self = Self(44);
    pub const PM_ALT_STARMAP: Self = Self(45);
    pub const PM_ALT_MANIFEST: Self = Self(46);
    pub const PM_ALT_GAME_MENU: Self = Self(47);
    pub const PM_ALT_NAVIGATE: Self = Self(48);
    pub const PM_ALT_CARGO: Self = Self(49);
    pub const PM_ALT_DEVICES: Self = Self(50);
    pub const PM_ALT_ROSTER: Self = Self(51);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(52);
    pub const PM_ALT_MSCAN: Self = Self(53);
    pub const PM_ALT_ESCAN: Self = Self(54);
    pub const PM_ALT_BSCAN: Self = Self(55);
    pub const PM_ALT_ASCAN: Self = Self(56);
    pub const PM_ALT_DISPATCH: Self = Self(57);
    pub const PM_ALT_EXIT_SCAN: Self = Self(58);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
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
unsafe extern "C" fn IncreaseBrightness(mut ch: *mut BYTE, mut value: BYTE) {
    let mut c: ::core::ffi::c_int = 0;
    if (*ch as ::core::ffi::c_int) < 128 as ::core::ffi::c_int {
        c = *ch as ::core::ffi::c_int * 255 as ::core::ffi::c_int
            >> 7 as ::core::ffi::c_int;
    } else {
        c = ((*ch as ::core::ffi::c_int + 255 as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) - 255 as ::core::ffi::c_int
            - (255 as ::core::ffi::c_int * *ch as ::core::ffi::c_int
                >> 7 as ::core::ffi::c_int);
    }
    if c > 0xff as ::core::ffi::c_int {
        c = 0xff as ::core::ffi::c_int;
    }
    if value as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
        *ch = c as BYTE;
    } else {
        *ch = (((c - *ch as ::core::ffi::c_int) * value as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            + *ch as ::core::ffi::c_int) as BYTE;
    };
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[inline]
unsafe extern "C" fn RECT_TO_DRECT(mut r: RECT) -> DRECT {
    let mut dr: DRECT = drect {
        corner: dpoint {
            x: r.corner.x as SDWORD,
            y: r.corner.y as SDWORD,
        },
        extent: dextent {
            width: r.extent.width as SDWORD,
            height: r.extent.height as SDWORD,
        },
    };
    return dr;
}
#[inline]
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
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
unsafe extern "C" fn ValidPoint(mut pt: POINT) -> BOOLEAN {
    return BOOLEAN(
        (pt.x as ::core::ffi::c_int != !(0 as ::core::ffi::c_int)
            && pt.y as ::core::ffi::c_int != !(0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
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
unsafe extern "C" fn VelocitySquared(mut dx: SIZE, mut dy: SIZE) -> DWORD {
    return (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
        + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD;
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
pub static mut OrbitalShield: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut NebulaFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
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
static mut PBodySize: [COUNT; 7] = [
    0 as COUNT,
    3 as COUNT,
    4 as COUNT,
    7 as COUNT,
    11 as COUNT,
    15 as COUNT,
    29 as COUNT,
];
#[no_mangle]
pub static mut SysGenRNG: *mut RandomContext = ::core::ptr::null_mut::<RandomContext>();
#[no_mangle]
pub static mut SysGenRNGDebug: *mut RandomContext = ::core::ptr::null_mut::<
    RandomContext,
>();
#[no_mangle]
pub unsafe extern "C" fn locationToDisplay(
    mut pt: POINT,
    mut scaleRadius: SIZE,
) -> POINT {
    let mut out: POINT = POINT { x: 0, y: 0 };
    out.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        << resolutionFactor) as ::core::ffi::c_long
        + pt.x as ::core::ffi::c_long
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                << resolutionFactor >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
            / scaleRadius as ::core::ffi::c_long) as COORD;
    out.y = (((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
        + pt.y as ::core::ffi::c_long
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                << resolutionFactor >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long
            / scaleRadius as ::core::ffi::c_long) as COORD;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn displayToLocation(
    mut pt: POINT,
    mut scaleRadius: SIZE,
) -> POINT {
    let mut out: POINT = POINT { x: 0, y: 0 };
    out.x = ((pt.x as ::core::ffi::c_long
        - ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            << resolutionFactor) as ::core::ffi::c_long)
        * scaleRadius as ::core::ffi::c_long
        / (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - 8 as ::core::ffi::c_int) << resolutionFactor >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_long) as COORD;
    out.y = ((pt.y as ::core::ffi::c_long
        - ((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long)
        * scaleRadius as ::core::ffi::c_long
        / (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - 8 as ::core::ffi::c_int) << resolutionFactor >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_long) as COORD;
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
pub unsafe extern "C" fn moonLetter(
    mut solarSys: *const SOLARSYS_STATE,
    mut moon: *const PLANET_DESC,
) -> UNICODE {
    (!worldIsPlanet(solarSys, moon)
        || {
            __assert_fail(
                b"!worldIsPlanet (solarSys, moon)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int,
                b"moonLetter\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    while moon.offset_from(&raw const (*solarSys).MoonDesc as *const PLANET_DESC)
        >= 0isize
    {
        if (*moon).data_index as ::core::ffi::c_int
            != C2Rust_Unnamed_7::HIERARCHY_STARBASE.0 as ::core::ffi::c_int
            && (*moon).data_index as ::core::ffi::c_int
                != C2Rust_Unnamed_7::DESTROYED_STARBASE.0 as ::core::ffi::c_int
            && (*moon).data_index as ::core::ffi::c_int
                != C2Rust_Unnamed_7::PRECURSOR_STARBASE.0 as ::core::ffi::c_int
        {
            i += 1;
        }
        moon = moon.offset(-1);
    }
    return (i + 'A' as ::core::ffi::c_int) as UNICODE;
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
                233 as ::core::ffi::c_int,
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
    let mut PlanetI: BYTE = (if planetI as ::core::ffi::c_int
        == -2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        (*solarSys).SunDesc[0usize].PlanetByte as ::core::ffi::c_int
    } else {
        planetI as ::core::ffi::c_int
    }) as BYTE;
    let mut MoonI: BYTE = (if moonI as ::core::ffi::c_int
        == -3 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
    {
        (*solarSys).SunDesc[0usize].MoonByte as ::core::ffi::c_int
    } else {
        moonI as ::core::ffi::c_int
    }) as BYTE;
    if planetIndex(solarSys, world) as ::core::ffi::c_int
        != PlanetI as ::core::ffi::c_int
    {
        return false;
    }
    if MoonI as ::core::ffi::c_int
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
            != MoonI as ::core::ffi::c_int
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
                286 as ::core::ffi::c_int,
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
                288 as ::core::ffi::c_int,
                b"playerInInnerSystem\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return (*pSolarSysState).pBaseDesc
        != &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC;
}
unsafe extern "C" fn GenerateTexturedMoons(
    mut system: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
) {
    let mut i: COUNT = 0;
    let mut MoonDiameter: SIZE = 0;
    let mut pMoonDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut previousOrbitalDesc: *mut PLANET_DESC = (*pSolarSysState).pOrbitalDesc;
    let mut planetInfo: *mut PLANET_INFO = &raw mut (*pSolarSysState).SysInfo.PlanetInfo;
    i = 0 as COUNT;
    pMoonDesc = (&raw mut (*system).MoonDesc as *mut PLANET_DESC).offset(0isize);
    while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
        let mut maskAnim: RESOURCE = ::core::ptr::null::<::core::ffi::c_char>();
        if (*pMoonDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
        {
            DoPlanetaryAnalysis(&raw mut (*pSolarSysState).SysInfo, pMoonDesc);
            if (*CurStarDescPtr).Index as ::core::ffi::c_int
                == C2Rust_Unnamed_8::SOL_DEFINED.0 as ::core::ffi::c_int
            {
                (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 0 as SIZE;
            }
            (*pSolarSysState).pOrbitalDesc = pMoonDesc;
            if (*CurStarDescPtr).Index as ::core::ffi::c_int
                == C2Rust_Unnamed_8::SOL_DEFINED.0 as ::core::ffi::c_int
            {
                let mut curr_planet_index: COUNT = planetIndex(pSolarSysState, planet);
                match curr_planet_index as ::core::ffi::c_int {
                    2 => {
                        if i as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            maskAnim = b"graphics.iplunamask\0".as_ptr()
                                as *const ::core::ffi::c_char as RESOURCE;
                            (*planetInfo).RotationPeriod = (240 as ::core::ffi::c_int
                                * 29 as ::core::ffi::c_int) as UWORD;
                        }
                    }
                    4 => {
                        match i as ::core::ffi::c_int {
                            0 => {
                                maskAnim = b"graphics.ipiomask\0".as_ptr()
                                    as *const ::core::ffi::c_char as RESOURCE;
                                (*planetInfo).RotationPeriod = 390 as UWORD;
                            }
                            1 => {
                                maskAnim = b"graphics.ipeuropamask\0".as_ptr()
                                    as *const ::core::ffi::c_char as RESOURCE;
                                (*planetInfo).RotationPeriod = 840 as UWORD;
                            }
                            2 => {
                                maskAnim = b"graphics.ipganymedemask\0".as_ptr()
                                    as *const ::core::ffi::c_char as RESOURCE;
                                (*planetInfo).RotationPeriod = 1728 as UWORD;
                            }
                            3 => {
                                maskAnim = b"graphics.ipcallistomask\0".as_ptr()
                                    as *const ::core::ffi::c_char as RESOURCE;
                                (*planetInfo).RotationPeriod = 4008 as UWORD;
                            }
                            _ => {}
                        }
                    }
                    5 => {
                        maskAnim = b"graphics.iptitanmask\0".as_ptr()
                            as *const ::core::ffi::c_char as RESOURCE;
                        (*planetInfo).RotationPeriod = 3816 as UWORD;
                    }
                    7 => {
                        maskAnim = b"graphics.iptritonmask\0".as_ptr()
                            as *const ::core::ffi::c_char as RESOURCE;
                        (*planetInfo).RotationPeriod = 4300 as UWORD;
                    }
                    8 => {
                        maskAnim = b"graphics.ipcharonmask\0".as_ptr()
                            as *const ::core::ffi::c_char as RESOURCE;
                        (*planetInfo).RotationPeriod = (6.4f64
                            * 240 as ::core::ffi::c_int as ::core::ffi::c_double)
                            as UWORD;
                    }
                    _ => {}
                }
            }
            MoonDiameter = (if (*pMoonDesc).data_index as ::core::ffi::c_int
                > C2Rust_Unnamed_7::LAST_SMALL_ROCKY_WORLD.0 as ::core::ffi::c_int
            {
                (11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            } else {
                (7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            }) as SIZE;
            uqm_lua_checked!(GeneratePlanetSurface(
                pMoonDesc,
                CaptureDrawable(uqm_lua_checked!(LoadGraphicInstance(maskAnim), ()) as DRAWABLE),
                (MoonDiameter as ::core::ffi::c_int
                    * if optPlanetTexture.0 != 0 {
                        243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    } else {
                        210 as ::core::ffi::c_int
                    }
                    / if optPlanetTexture.0 != 0 {
                        75 as ::core::ffi::c_int
                    } else {
                        67 as ::core::ffi::c_int
                    }) as COUNT,
                MoonDiameter as COUNT,
            ), ());
            (*pMoonDesc).orbit = (*pSolarSysState).Orbit;
            uqm_lua_checked!(PrepareNextRotationFrameForIP(pMoonDesc, 0 as SIZE), ());
            DestroyStringTable(ReleaseStringTable((*pSolarSysState).XlatRef));
            (*pSolarSysState).XlatRef = ::core::ptr::null_mut::<
                STRING_TABLE_ENTRY_DESC,
            >();
            DestroyDrawable(ReleaseDrawable((*pSolarSysState).TopoFrame));
            (*pSolarSysState).TopoFrame = ::core::ptr::null_mut::<FRAME_DESC>();
            DestroyStringTable(ReleaseStringTable((*pSolarSysState).OrbitalCMap));
            (*pSolarSysState).OrbitalCMap = ::core::ptr::null_mut::<
                STRING_TABLE_ENTRY_DESC,
            >();
        }
        i = i.wrapping_add(1);
        pMoonDesc = pMoonDesc.offset(1);
    }
    (*pSolarSysState).pOrbitalDesc = previousOrbitalDesc;
}
unsafe extern "C" fn GenerateMoons(
    mut system: *mut SOLARSYS_STATE,
    mut planet: *mut PLANET_DESC,
) {
    let mut i: COUNT = 0;
    let mut pMoonDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    RandomContext_SeedRandom(SysGenRNG, (*planet).rand_seed);
    i = 0 as COUNT;
    pMoonDesc = (&raw mut (*system).MoonDesc as *mut PLANET_DESC).offset(0isize);
    while (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        (*pMoonDesc).pPrevDesc = planet;
        if (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
            (*pMoonDesc).temp_color = (*planet).temp_color;
            (*pMoonDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
        }
        i = i.wrapping_add(1);
        pMoonDesc = pMoonDesc.offset(1);
    }
    uqm_lua_checked!(Some((*(*system).genFuncs).generateName.expect("non-null function pointer"))
        .expect("non-null function pointer")(system, planet), ());
    uqm_lua_checked!(Some((*(*system).genFuncs).generateMoons.expect("non-null function pointer"))
        .expect("non-null function pointer")(system, planet), ());
}
#[no_mangle]
pub unsafe extern "C" fn LoadNebulaeFrame(mut location: POINT) -> FRAME {
    let mut path: [UNICODE; 4096] = [0; 4096];
    let mut nebulaRes: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    let mut nebulaCount: COUNT = 0;
    let mut neb_seed: POINT = POINT { x: 0, y: 0 };
    let solPoint: POINT = plot_map[C2Rust_Unnamed_8::SOL_DEFINED.0 as ::core::ffi::c_int
            as usize]
        .star_pt;
    if optNebulae.0 == 0
        || !(LastActivity as ::core::ffi::c_int
            != C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
            || NextActivity as ::core::ffi::c_int != 0)
        || pointsEqual(location, solPoint) as ::core::ffi::c_int != 0
            && classicPackPresent.0 == 0 && uqm_lua_checked!(inHyperSpace(), ::core::ptr::null_mut()).0 == 0
    {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    if !(optSeedType == C2Rust_Unnamed_1::OPTVAL_PRIME.0 as ::core::ffi::c_int) {
        let mut rand_val: DWORD = 0;
        rand_val = RandomContext_FastRandom(
            (location.x as DWORD) << 16 as ::core::ffi::c_int
                | location.y as UWORD as DWORD,
        );
        neb_seed.x = rand_val as UWORD as COORD;
        neb_seed.y = (rand_val >> 16 as ::core::ffi::c_int) as UWORD as COORD;
    } else {
        neb_seed.x = location.x;
        neb_seed.y = location.y;
    }
    nebulaRes = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(b"nebula.resource\0".as_ptr() as RESOURCE), ::core::ptr::null_mut()),
    );
    nebulaCount = GetStringTableCount(nebulaRes);
    sprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                nebulaRes,
                (neb_seed.x as ::core::ffi::c_int % nebulaCount as ::core::ffi::c_int)
                    as COUNT,
            ),
        ),
    );
    DestroyStringTable(ReleaseStringTable(nebulaRes));
    if neb_seed.y as ::core::ffi::c_int
        % (nebulaCount as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
        > nebulaCount as ::core::ffi::c_int && classicPackPresent.0 == 0
        && uqm_lua_checked!(inHyperSpace(), ::core::ptr::null_mut()).0 == 0
    {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    return CaptureDrawable(uqm_lua_checked!(LoadGraphicFile(&raw mut path as *mut UNICODE), ::core::ptr::null_mut()));
}
unsafe extern "C" fn ChangeSunColor() {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut starSize: BYTE = (((*CurStarDescPtr).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE;
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    let mut maskOffset: COUNT = 0;
    let mut sizeIter: COUNT = 0;
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    c = GetColorMapColor(GetColorMapTableIndex(SunCMap) as COUNT, 0xff as COUNT);
    mode = MAKE_DRAW_MODE(DrawKind::DRAW_ALPHA, 0x7fff as SWORD);
    maskOffset = (GetFrameCount(SunFrame) as ::core::ffi::c_int
        - 5 as ::core::ffi::c_int) as COUNT;
    sizeIter = (maskOffset as ::core::ffi::c_int / 5 as ::core::ffi::c_int) as COUNT;
    i = starSize as COUNT;
    while i as ::core::ffi::c_int
        <= starSize as ::core::ffi::c_int + 2 as ::core::ffi::c_int
    {
        let mut mask: FRAME = SetAbsFrameIndex(
            SunFrame,
            (maskOffset as ::core::ffi::c_int + i as ::core::ffi::c_int) as COUNT,
        );
        j = 0 as COUNT;
        while (j as ::core::ffi::c_int) < sizeIter as ::core::ffi::c_int {
            let mut star: FRAME = SetAbsFrameIndex(
                SunFrame,
                (i as ::core::ffi::c_int * sizeIter as ::core::ffi::c_int
                    + j as ::core::ffi::c_int) as COUNT,
            );
            uqm_lua_checked!(ApplyMask(mask, star, mode, &raw mut c), ());
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
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
    DestroyDrawable(ReleaseDrawable(OrbitalShield));
    OrbitalShield = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(SpaceJunkFrame));
    SpaceJunkFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(NebulaFrame));
    NebulaFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    uqm_lua_checked!(DestroyMusic(SpaceMusic), ());
    SpaceMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    RandomContext_Delete(SysGenRNG);
    SysGenRNG = ::core::ptr::null_mut::<RandomContext>();
}
#[no_mangle]
pub unsafe extern "C" fn LoadIPData() {
    if SpaceJunkFrame.is_null() {
        SpaceJunkFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.orbitbackground\0".as_ptr() as RESOURCE), ())
                as DRAWABLE,
        );
        SISIPFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(
                if is3DO(optFlagshipColor).0 != 0 {
                    b"graphics.flagship.red\0".as_ptr() as RESOURCE
                } else {
                    b"graphics.flagship\0".as_ptr() as RESOURCE
                },
            ), ()) as DRAWABLE,
        );
        OrbitalCMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(b"colortable.orbplan\0".as_ptr() as RESOURCE), ()),
        ) as COLORMAP;
        OrbitalFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(
                if isPC(optPlanetStyle).0 != 0 && optTexturedPlanets.0 == 0 {
                    b"graphics.dos.planets\0".as_ptr() as RESOURCE
                } else {
                    b"graphics.planets\0".as_ptr() as RESOURCE
                },
            ), ()) as DRAWABLE,
        );
        OrbitalShield = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.planets.slaveshield\0".as_ptr() as RESOURCE), ())
                as DRAWABLE,
        );
        NebulaFrame = uqm_lua_checked!(LoadNebulaeFrame((*CurStarDescPtr).star_pt), ());
        SunCMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(b"colortable.truespace\0".as_ptr() as RESOURCE), ()),
        ) as COLORMAP;
        SunFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0 && !NebulaFrame.is_null()
                    || if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0 && optHyperStars.0 != 0
                {
                    b"graphics.truespacesun.anim\0".as_ptr() as RESOURCE
                } else {
                    b"graphics.truespacesun\0".as_ptr() as RESOURCE
                },
            ), ()) as DRAWABLE,
        );
        SpaceMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    }
    if SysGenRNG.is_null() {
        SysGenRNG = RandomContext_New();
        SysGenRNGDebug = SysGenRNG;
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
    memset(
        &raw mut sort_array as *mut SIZE as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[SIZE; 17]>(),
    );
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
            let mut temp: SIZE = 0;
            real_i = sort_array[i as usize];
            real_j = sort_array[j as usize];
            if (*solarDisplayBody(real_i)).image.origin.y
                as ::core::ffi::c_int
                > (*solarDisplayBody(real_j)).image.origin.y
                    as ::core::ffi::c_int
                || (*solarDisplayBody(real_i)).image.origin.y
                    as ::core::ffi::c_int
                    == (*solarDisplayBody(real_j)).image.origin.y
                        as ::core::ffi::c_int
                    && real_i as ::core::ffi::c_int == -1 as ::core::ffi::c_int
            {
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
            == C2Rust_Unnamed_4::FUSION_THRUSTER.0 as ::core::ffi::c_int
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
            == C2Rust_Unnamed_4::TURNING_JETS.0 as ::core::ffi::c_int
        {
            (*pSolarSysState).turn_wait = ((*pSolarSysState).turn_wait
                as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetRandomSeedForStar(mut star: *const STAR_DESC) -> DWORD {
    return (((*star).star_pt.y as DWORD) << 16 as ::core::ffi::c_int
        | (*star).star_pt.x as UWORD as DWORD)
        .wrapping_add(
            (if optSeedType > C2Rust_Unnamed_1::OPTVAL_PLANET.0 as ::core::ffi::c_int {
                optCustomSeed
            } else {
                0 as ::core::ffi::c_int
            }) as DWORD,
        );
}
#[no_mangle]
pub unsafe extern "C" fn GetRandomSeedForVar(point: POINT) -> DWORD {
    return (point.y as DWORD) << 16 as ::core::ffi::c_int | point.x as UWORD as DWORD;
}
unsafe extern "C" fn GenerateTexturedPlanets() {
    let mut i: COUNT = 0;
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut previousOrbitalDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<
        PLANET_DESC,
    >();
    previousOrbitalDesc = (*pSolarSysState).pOrbitalDesc;
    i = 0 as COUNT;
    pCurDesc = &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC;
    while (i as ::core::ffi::c_int)
        < (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
    {
        let mut maskanim: RESOURCE = ::core::ptr::null::<::core::ffi::c_char>();
        DoPlanetaryAnalysis(&raw mut (*pSolarSysState).SysInfo, pCurDesc);
        (*pSolarSysState).pOrbitalDesc = pCurDesc;
        if (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_8::SOL_DEFINED.0 as ::core::ffi::c_int
        {
            match i as ::core::ffi::c_int {
                0 => {
                    maskanim = b"graphics.ipmercurymask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 3 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = (59
                        as ::core::ffi::c_int * 240 as ::core::ffi::c_int) as UWORD;
                }
                1 => {
                    maskanim = b"graphics.ipvenusmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 177 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = (243
                        as ::core::ffi::c_int * 240 as ::core::ffi::c_int) as UWORD;
                }
                2 => {
                    maskanim = b"graphics.earthmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 23 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 240 as UWORD;
                }
                3 => {
                    maskanim = b"graphics.ipmarsmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 24 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 246 as UWORD;
                }
                4 => {
                    maskanim = b"graphics.ipjupitermask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 3 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 98 as UWORD;
                }
                5 => {
                    maskanim = b"graphics.ipsaturnmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 27 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 102 as UWORD;
                }
                6 => {
                    maskanim = b"graphics.ipuranusmask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 98 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 172 as UWORD;
                }
                7 => {
                    maskanim = b"graphics.ipneptunemask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 30 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 182 as UWORD;
                }
                8 => {
                    maskanim = b"graphics.ipplutomask\0".as_ptr()
                        as *const ::core::ffi::c_char as RESOURCE;
                    (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt = 119 as SIZE;
                    (*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod = 1533 as UWORD;
                }
                _ => {}
            }
        }
        uqm_lua_checked!(GeneratePlanetSurface(
            pCurDesc,
            CaptureDrawable(uqm_lua_checked!(LoadGraphicInstance(maskanim), ()) as DRAWABLE),
            (((29 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                * if optPlanetTexture.0 != 0 {
                    243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                } else {
                    210 as ::core::ffi::c_int
                }
                / if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                }) as COUNT,
            ((29 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
        ), ());
        (*pCurDesc).orbit = (*pSolarSysState).Orbit;
        uqm_lua_checked!(PrepareNextRotationFrameForIP(pCurDesc, 0 as SIZE), ());
        DestroyStringTable(ReleaseStringTable((*pSolarSysState).XlatRef));
        (*pSolarSysState).XlatRef = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
        DestroyDrawable(ReleaseDrawable((*pSolarSysState).TopoFrame));
        (*pSolarSysState).TopoFrame = ::core::ptr::null_mut::<FRAME_DESC>();
        DestroyStringTable(ReleaseStringTable((*pSolarSysState).OrbitalCMap));
        (*pSolarSysState).OrbitalCMap = ::core::ptr::null_mut::<
            STRING_TABLE_ENTRY_DESC,
        >();
        i = i.wrapping_add(1);
        pCurDesc = pCurDesc.offset(1);
    }
    (*pSolarSysState).pOrbitalDesc = previousOrbitalDesc;
}
unsafe extern "C" fn LoadSolarSys() -> *mut PLANET_DESC {
    let mut i: COUNT = 0;
    let mut orbital: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut temp_color_array: [Color; 5] = [
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
    if !NebulaFrame.is_null() {
        let mut brightness: BYTE = ((optNebulaeVolume + 1 as ::core::ffi::c_int)
            * 5 as ::core::ffi::c_int) as BYTE;
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            IncreaseBrightness(
                &raw mut (*(&raw mut temp_color_array as *mut Color).offset(i as isize))
                    .r,
                brightness,
            );
            IncreaseBrightness(
                &raw mut (*(&raw mut temp_color_array as *mut Color).offset(i as isize))
                    .g,
                brightness,
            );
            IncreaseBrightness(
                &raw mut (*(&raw mut temp_color_array as *mut Color).offset(i as isize))
                    .b,
                brightness,
            );
            i = i.wrapping_add(1);
        }
    }
    RandomContext_SeedRandom(SysGenRNG, GetRandomSeedForStar(CurStarDescPtr));
    SunFrame = SetAbsFrameIndex(
        SunFrame,
        ((((*CurStarDescPtr).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            * if (GetFrameCount(SunFrame) as ::core::ffi::c_int
                - 5 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int != 0
            {
                (GetFrameCount(SunFrame) as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
                    / 5 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as COUNT,
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
    uqm_lua_checked!(Some(
            (*(*pSolarSysState).genFuncs)
                .generatePlanets
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(pSolarSysState), ::core::ptr::null_mut());
    if uqm_lua_checked!(getGameStateUint(b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        != 0
    {
        PutPlanetInfo();
        uqm_lua_checked!(setGameStateUint(
            b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ::core::ptr::null_mut());
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
                    PartiallyScavengedList: [[0; 32]; 3],
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
        (*pCurDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
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
        uqm_lua_checked!(GenerateMoons(pSolarSysState, (*pSolarSysState).pOrbitalDesc), ::core::ptr::null_mut());
        (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).MoonDesc
            as *mut PLANET_DESC;
        uqm_lua_checked!(setGameStateUint(
            b"PLANETARY_LANDING\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ::core::ptr::null_mut());
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
    let mut i: COUNT = 0;
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    if (*pSolarSysState).InIpFlight.0 != 0 {
        (*pSolarSysState).InIpFlight = BOOLEAN::FALSE;
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        {
            saveNonOrbitalLocation();
            uqm_lua_checked!(SetMusicPosition(), ());
        }
    }
    DestroyDrawable(ReleaseDrawable(SolarSysFrame));
    SolarSysFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    uqm_lua_checked!(StopMusic(), ());
    if optTexturedPlanets.0 != 0 {
        i = 0 as COUNT;
        pCurDesc = &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC;
        while (i as ::core::ffi::c_int)
            < (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
        {
            DestroyOrbitStruct(
                &raw mut (*pCurDesc).orbit,
                ((29 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE,
            );
            DestroyStringTable(ReleaseStringTable((*pSolarSysState).XlatRef));
            (*pSolarSysState).XlatRef = ::core::ptr::null_mut::<
                STRING_TABLE_ENTRY_DESC,
            >();
            DestroyDrawable(ReleaseDrawable((*pSolarSysState).TopoFrame));
            (*pSolarSysState).TopoFrame = ::core::ptr::null_mut::<FRAME_DESC>();
            DestroyStringTable(ReleaseStringTable((*pSolarSysState).OrbitalCMap));
            (*pSolarSysState).OrbitalCMap = ::core::ptr::null_mut::<
                STRING_TABLE_ENTRY_DESC,
            >();
            i = i.wrapping_add(1);
            pCurDesc = pCurDesc.offset(1);
        }
        if playerInInnerSystem() {
            let mut numMoons: COUNT = 0;
            if worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
                numMoons = (*(*(*pSolarSysState).pOrbitalDesc).pPrevDesc).NumPlanets
                    as COUNT;
            } else {
                numMoons = (*(*pSolarSysState).pOrbitalDesc).NumPlanets as COUNT;
            }
            i = 0 as COUNT;
            pCurDesc = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
            while (i as ::core::ffi::c_int) < numMoons as ::core::ffi::c_int {
                if (*pCurDesc).data_index as ::core::ffi::c_int
                    & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
                {
                    let mut diameterPick: SIZE = (if (*pCurDesc).data_index
                        as ::core::ffi::c_int
                        > C2Rust_Unnamed_7::LAST_SMALL_ROCKY_WORLD.0
                            as ::core::ffi::c_int
                    {
                        (11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    } else {
                        (7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    }) as SIZE;
                    DestroyOrbitStruct(&raw mut (*pCurDesc).orbit, diameterPick);
                    (*pCurDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
                }
                i = i.wrapping_add(1);
                pCurDesc = pCurDesc.offset(1);
            }
        }
    } else {
        i = 0 as COUNT;
        pCurDesc = &raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC;
        while (i as ::core::ffi::c_int)
            < (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
        {
            if isPC(optPlanetStyle).0 != 0 {
                DestroyDrawable(ReleaseDrawable((*pCurDesc).image.frame));
                (*pCurDesc).image.frame = ::core::ptr::null_mut::<FRAME_DESC>();
            }
            (*pCurDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
            i = i.wrapping_add(1);
            pCurDesc = pCurDesc.offset(1);
        }
        if playerInInnerSystem() {
            let mut numMoons_0: COUNT = 0;
            if worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
                numMoons_0 = (*(*(*pSolarSysState).pOrbitalDesc).pPrevDesc).NumPlanets
                    as COUNT;
            } else {
                numMoons_0 = (*(*pSolarSysState).pOrbitalDesc).NumPlanets as COUNT;
            }
            i = 0 as COUNT;
            pCurDesc = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
            while (i as ::core::ffi::c_int) < numMoons_0 as ::core::ffi::c_int {
                if (*pCurDesc).data_index as ::core::ffi::c_int
                    & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
                    && isPC(optPlanetStyle).0 != 0
                {
                    DestroyDrawable(ReleaseDrawable((*pCurDesc).image.frame));
                    (*pCurDesc).image.frame = ::core::ptr::null_mut::<FRAME_DESC>();
                }
                (*pCurDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
                i = i.wrapping_add(1);
                pCurDesc = pCurDesc.offset(1);
            }
        }
    };
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
    PlanetIntersect.IntersectStamp.origin.x = ((CanvasWidth
        - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    PlanetIntersect.IntersectStamp.origin.y = ((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    PlanetIntersect.EndPoint = PlanetIntersect.IntersectStamp.origin;
    PlanetIntersect.IntersectStamp.frame = getCollisionFrame(
        pCurDesc,
        (((MoonOffset as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int | PlanetOffset as ::core::ffi::c_int) as COUNT,
    );
    NewWaitPlanet = 0 as COUNT;
    if pCurDesc != &raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC
        && uqm_lua_checked!(DrawablesIntersect(
            &raw mut ShipIntersect,
            &raw mut PlanetIntersect,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ), ::core::ptr::null_mut()) as ::core::ffi::c_int != 0
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
        if uqm_lua_checked!(DrawablesIntersect(
            &raw mut ShipIntersect,
            &raw mut PlanetIntersect,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ), ::core::ptr::null_mut()) != 0
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
    (*pRect).corner.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        << resolutionFactor) as ::core::ffi::c_long
        + -(dx as ::core::ffi::c_int) as ::core::ffi::c_long
            * xnumer as ::core::ffi::c_long / denom as ::core::ffi::c_long) as COORD;
    (*pRect).corner.y = (((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
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
        > ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - 8 as ::core::ffi::c_int) << resolutionFactor
    {
        dx = (dx as ::core::ffi::c_int + (*planet).location.x as ::core::ffi::c_int)
            as COORD;
        dy = ((dy as ::core::ffi::c_int + (*planet).location.y as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as COORD;
    }
    GetOrbitRect(r, dx, dy, (*planet).radius, sizeNumer, dyNumer, denom);
}
unsafe extern "C" fn SetPlanetOldFrame(
    mut planet: *mut PLANET_DESC,
    mut index: COUNT,
    mut color: COUNT,
) {
    let mut oldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    oldFrame = SetAbsFrameIndex(OrbitalFrame, index);
    GetFrameRect(oldFrame, &raw mut r);
    r.corner.y = 0 as COORD;
    r.corner.x = (r.extent.width as ::core::ffi::c_int
        / C2Rust_Unnamed_6::NUM_PLANET_COLORS.0 as ::core::ffi::c_int
        * color as ::core::ffi::c_int) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        / C2Rust_Unnamed_6::NUM_PLANET_COLORS.0 as ::core::ffi::c_int) as COORD;
    if !(*planet).image.frame.is_null()
        && (*planet).size as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        DestroyDrawable(ReleaseDrawable((*planet).image.frame));
        (*planet).image.frame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
    (*planet).image.frame = CaptureDrawable(CopyFrameRect(oldFrame, &raw mut r));
    SetFrameHot((*planet).image.frame, GetFrameHot(oldFrame));
}
unsafe extern "C" fn ValidateOrbit(
    mut planet: *mut PLANET_DESC,
    mut sizeNumer: ::core::ffi::c_int,
    mut dyNumer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut index: COUNT = 0;
    if optOrbitingPlanets.0 != 0 {
        let mut newAngle: ::core::ffi::c_double = 0.;
        newAngle = ((*planet).angle as ::core::ffi::c_int as ::core::ffi::c_double
            + daysElapsed() as ::core::ffi::c_double * (*planet).orb_speed)
            * 3.14159265358979323846f64
            / 32 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3.14159265358979323846f64
                / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*planet).location.x = (cos(newAngle)
            * (*planet).radius as ::core::ffi::c_int as ::core::ffi::c_double) as COORD;
        (*planet).location.y = (sin(newAngle)
            * (*planet).radius as ::core::ffi::c_int as ::core::ffi::c_double) as COORD;
    }
    if sizeNumer
        <= ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - 8 as ::core::ffi::c_int) << resolutionFactor
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
        & !(C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)) as COUNT;
    if (index as ::core::ffi::c_int)
        < C2Rust_Unnamed_7::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
    {
        let mut Type: BYTE = 0;
        let mut Size: COUNT = 0;
        let mut angle: COUNT = 0;
        let mut offset: BYTE = 0 as BYTE;
        Type = (*PlanData.offset(index as isize)).Type;
        Size = (Type as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as BYTE as COUNT;
        if sizeNumer
            > ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                << resolutionFactor
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
            angle = ARCTAN(
                (*planet).location.x as SDWORD,
                (*planet).location.y as SDWORD,
            );
        } else {
            angle = ARCTAN(
                (*(*planet).pPrevDesc).location.x as SDWORD,
                (*(*planet).pPrevDesc).location.y as SDWORD,
            );
        }
        offset = (((Size as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as DWORD)
            .wrapping_add(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD,
            ) as BYTE;
        if (*planet).frame_offset as ::core::ffi::c_int != offset as ::core::ffi::c_int {
            (*planet).frame_offset = offset;
            if optTexturedPlanets.0 == 0 && isPC(optPlanetStyle).0 != 0 {
                SetPlanetOldFrame(
                    planet,
                    offset as COUNT,
                    (Type as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                        as COUNT,
                );
            } else {
                (*planet).image.frame = SetAbsFrameIndex(OrbitalFrame, offset as COUNT);
            }
            (*planet).size = (Size as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as COUNT;
        }
    } else if (*planet).data_index as ::core::ffi::c_int
        > C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int
        && (*planet).frame_offset as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        let mut frameNum: BYTE = 0 as BYTE;
        (*planet).frame_offset = 0 as BYTE;
        (*planet).size = 0 as COUNT;
        match (*planet).data_index as ::core::ffi::c_int {
            252 => {
                frameNum = 23 as BYTE;
            }
            253 => {
                frameNum = 22 as BYTE;
            }
            254 => {
                frameNum = 19 as BYTE;
            }
            255 | _ => {
                frameNum = 16 as BYTE;
            }
        }
        (*planet).image.frame = SetAbsFrameIndex(SpaceJunkFrame, frameNum as COUNT);
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
    let mut dr: DRECT = DRECT {
        corner: DPOINT { x: 0, y: 0 },
        extent: DEXTENT { width: 0, height: 0 },
    };
    GetPlanetOrbitRect(&raw mut r, planet, sizeNumer, dyNumer, denom);
    dr = RECT_TO_DRECT(r);
    SetContextForeGroundColor((*planet).temp_color);
    if optUnscaledStarSystem.0 == 0 {
        uqm_lua_checked!(DrawOval(
            &raw mut dr,
            (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                1 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            }) as BYTE,
            BOOLEAN::FALSE,
        ), ());
    } else {
        uqm_lua_checked!(DrawOval(&raw mut dr, 1 as BYTE, BOOLEAN::FALSE), ());
    };
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
            ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                << resolutionFactor,
            ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) << resolutionFactor,
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
    let mut max_ship_speed: SIZE = 0;
    let mut cur_delta_x: SIZE = 0;
    let mut cur_delta_y: SIZE = 0;
    let mut TravelAngle: COUNT = 0;
    let mut thrust_increment: COUNT = 0;
    let mut VelocityPtr: *mut VELOCITY_DESC = ::core::ptr::null_mut::<VELOCITY_DESC>();
    max_ship_speed = (((*pSolarSysState).max_ship_speed as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as SIZE;
    thrust_increment = ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
    VelocityPtr = &raw mut GlobData.Game_state.velocity;
    GetCurrentVelocityComponents(
        VelocityPtr,
        &raw mut cur_delta_x,
        &raw mut cur_delta_y,
    );
    TravelAngle = (*VelocityPtr).TravelAngle;
    if TravelAngle as ::core::ffi::c_int == CurrentAngle as ::core::ffi::c_int
        && cur_delta_x as ::core::ffi::c_int
            == (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * max_ship_speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD
        && cur_delta_y as ::core::ffi::c_int
            == (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * max_ship_speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD
    {
        return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD
    } else {
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut desired_speed: DWORD = 0;
        let mut max_speed: DWORD = 0;
        delta_x = (cur_delta_x as SDWORD
            + (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * thrust_increment as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD) as SIZE;
        delta_y = (cur_delta_y as SDWORD
            + (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * thrust_increment as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD) as SIZE;
        desired_speed = VelocitySquared(delta_x, delta_y);
        max_speed = pow(
            max_ship_speed as ::core::ffi::c_double,
            2 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as DWORD;
        if desired_speed <= max_speed {
            SetVelocityComponents(VelocityPtr, delta_x as SDWORD, delta_y as SDWORD);
        } else if TravelAngle as ::core::ffi::c_int == CurrentAngle as ::core::ffi::c_int
        {
            SetVelocityComponents(
                VelocityPtr,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * max_ship_speed as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * max_ship_speed as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
        } else {
            let mut v: VELOCITY_DESC = *VelocityPtr;
            DeltaVelocityComponents(
                &raw mut v,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (thrust_increment as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (TravelAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * thrust_increment as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (thrust_increment as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (TravelAngle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * thrust_increment as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            GetCurrentVelocityComponents(
                &raw mut v,
                &raw mut cur_delta_x,
                &raw mut cur_delta_y,
            );
            desired_speed = VelocitySquared(cur_delta_x, cur_delta_y);
            if desired_speed > max_speed {
                SetVelocityComponents(
                    VelocityPtr,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (CurrentAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * max_ship_speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (CurrentAngle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * max_ship_speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
                );
                return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
            }
            *VelocityPtr = v;
        }
        return 0 as UWORD;
    };
}
unsafe extern "C" fn ScreenCompass(mut index: COUNT) -> SIZE {
    let mut facing: SIZE = 0;
    let mut scrLoc: POINT = GlobData.Game_state.ShipStamp.origin;
    let mut sisScr: EXTENT = extent {
        width: (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))) as COORD,
        height: (CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
    };
    let mut westOfCenter: BOOLEAN = BOOLEAN(
        ((scrLoc.x as ::core::ffi::c_int)
            < sisScr.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut northOfCenter: BOOLEAN = BOOLEAN(
        ((scrLoc.y as ::core::ffi::c_int)
            < sisScr.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut quadrant: ::core::ffi::c_int = if westOfCenter.0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }
        | if northOfCenter.0 != 0 {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
    match quadrant {
        0 => {
            facing = (if (scrLoc.x as ::core::ffi::c_int)
                < scrLoc.y as ::core::ffi::c_int
            {
                12 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as SIZE;
        }
        1 => {
            facing = (if (sisScr.width as ::core::ffi::c_int
                - scrLoc.x as ::core::ffi::c_int) < scrLoc.y as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as SIZE;
        }
        2 => {
            facing = (if (scrLoc.x as ::core::ffi::c_int)
                < sisScr.height as ::core::ffi::c_int - scrLoc.y as ::core::ffi::c_int
            {
                12 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            }) as SIZE;
        }
        3 => {
            facing = (if (sisScr.width as ::core::ffi::c_int
                - scrLoc.x as ::core::ffi::c_int)
                < sisScr.height as ::core::ffi::c_int - scrLoc.y as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            }) as SIZE;
        }
        _ => return index as SIZE,
    }
    return facing;
}
unsafe extern "C" fn TurnAbout(mut delta_x: SIZE, mut delta_y: SIZE) -> POINT {
    let mut facing: SIZE = 0;
    let mut index: COUNT = 0;
    let mut delta: POINT = point { x: delta_x, y: delta_y };
    if optSmartAutoPilot.0 == 0 {
        delta.y = -1 as COORD;
        return delta;
    }
    index = GetFrameIndex(GlobData.Game_state.ShipStamp.frame);
    facing = ScreenCompass(index);
    if facing as ::core::ffi::c_int != index as ::core::ffi::c_int {
        let mut NormalizeFacing: ::core::ffi::c_int = (index as ::core::ffi::c_int
            - facing as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as ::core::ffi::c_int;
        let mut AngleTo: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        if NormalizeFacing >= AngleTo {
            facing = (facing as ::core::ffi::c_int + -1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as SIZE;
            delta.x += 1;
        } else if index as ::core::ffi::c_int != facing as ::core::ffi::c_int {
            facing = (facing as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as SIZE;
            delta.x -= 1;
        }
    } else {
        delta.y = -1 as COORD;
    }
    return delta;
}
unsafe extern "C" fn ProcessShipControls() {
    let mut index: COUNT = 0;
    let mut delta_x: SIZE = 0;
    let mut delta_y: SIZE = 0;
    let mut InputState: BATTLE_INPUT_STATE = 0 as BATTLE_INPUT_STATE;
    let mut turn_left: BOOLEAN = BOOLEAN::FALSE;
    let mut turn_right: BOOLEAN = BOOLEAN::FALSE;
    let mut thrust: BOOLEAN = BOOLEAN::FALSE;
    index = GetFrameIndex(GlobData.Game_state.ShipStamp.frame);
    InputState = GetDirectionalJoystickInput(
        index as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    turn_left = BOOLEAN(
        (CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed::KEY_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || InputState as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    turn_right = BOOLEAN(
        (CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed::KEY_RIGHT.0 as ::core::ffi::c_int as usize] != 0
            || InputState as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    thrust = BOOLEAN(
        (CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed::KEY_UP.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0 as usize][C2Rust_Unnamed::KEY_THRUST.0 as ::core::ffi::c_int as usize]
                != 0
            || InputState as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if thrust.0 != 0 {
        delta_y = -1 as SIZE;
    } else {
        delta_y = 0 as SIZE;
    }
    delta_x = 0 as SIZE;
    if turn_left.0 != 0 {
        delta_x = (delta_x as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
    }
    if turn_right.0 != 0 {
        delta_x = (delta_x as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
    }
    if delta_x as ::core::ffi::c_int != 0
        || (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        if ValidPoint(uqm_lua_checked!(LoadAdvancedAutoPilot(), ())).0 != 0
            || ValidPoint(uqm_lua_checked!(LoadAdvancedQuasiPilot(), ())).0 != 0
        {
            uqm_lua_checked!(ZeroAdvancedAutoPilot(), ());
            uqm_lua_checked!(ZeroAdvancedQuasiPilot(), ());
        }
    } else if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
    {
        let mut delta: POINT = POINT { x: 0, y: 0 };
        delta = TurnAbout(delta_x, delta_y);
        delta_x = delta.x as SIZE;
        delta_y = delta.y as SIZE;
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
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        } else {
            index = (index as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
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
    GlobData.Game_state.ShipStamp.origin.x = (((CanvasWidth
        - ((64 as SDWORD) << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as SDWORD
        } else {
            0 as SDWORD
        } << resolutionFactor) * 2 as SDWORD
        - (((13 as SDWORD) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as SDWORD
            } else {
                0 as SDWORD
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor)
        + (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * (((35 as ::core::ffi::c_int) << resolutionFactor)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    * ((20 as ::core::ffi::c_int) << resolutionFactor)
                + ((20 as ::core::ffi::c_int) << resolutionFactor
                    >> 2 as ::core::ffi::c_int)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                } << resolutionFactor)) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
    GlobData.Game_state.ShipStamp.origin.y = (((CanvasHeight
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as SDWORD
        } else {
            0 as SDWORD
        } << resolutionFactor) * 2 as SDWORD - ((13 as SDWORD) << resolutionFactor)
        >> resolutionFactor >> 1 as ::core::ffi::c_int) << resolutionFactor)
        + (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * (((35 as ::core::ffi::c_int) << resolutionFactor)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    * ((20 as ::core::ffi::c_int) << resolutionFactor)
                + ((20 as ::core::ffi::c_int) << resolutionFactor
                    >> 2 as ::core::ffi::c_int)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                } << resolutionFactor)) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
    if (GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.ShipStamp.origin.y = ((1 as ::core::ffi::c_int)
            << resolutionFactor) as COORD;
    } else if GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int
        >= CanvasHeight
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
    {
        GlobData.Game_state.ShipStamp.origin.y = (CanvasHeight
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
            - ((1 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
    uqm_lua_checked!(GenerateMoons(pSolarSysState, planet), ());
    if optTexturedPlanets.0 != 0 {
        uqm_lua_checked!(GenerateTexturedMoons(pSolarSysState, planet), ());
    }
    (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).MoonDesc
        as *mut PLANET_DESC;
    (*pSolarSysState).pOrbitalDesc = planet;
}
unsafe extern "C" fn leaveInnerSystem(mut planet: *mut PLANET_DESC) {
    let mut outerPlanetWait: COUNT = 0;
    let mut i: COUNT = 0;
    let mut pMoonDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    (*pSolarSysState).pBaseDesc = &raw mut (*pSolarSysState).PlanetDesc
        as *mut PLANET_DESC;
    (*pSolarSysState).pOrbitalDesc = ::core::ptr::null_mut::<PLANET_DESC>();
    outerPlanetWait = (((0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | (planet.offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
            + 1isize) as BYTE as ::core::ffi::c_int) as COUNT;
    ValidateOrbit(
        planet,
        ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - 8 as ::core::ffi::c_int) << resolutionFactor,
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
            - 8 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int) << resolutionFactor,
        (*pSolarSysState).SunDesc[0usize].radius as ::core::ffi::c_int,
    );
    (*pSolarSysState).SunDesc[0usize].location = planetOuterLocation(
        planetIndex(pSolarSysState, planet),
    );
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
    if optTexturedPlanets.0 != 0 {
        i = 0 as COUNT;
        pMoonDesc = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
        while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
            if (*pMoonDesc).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
            {
                let mut diameterPick: SIZE = (if (*pMoonDesc).data_index
                    as ::core::ffi::c_int
                    > C2Rust_Unnamed_7::LAST_SMALL_ROCKY_WORLD.0 as ::core::ffi::c_int
                {
                    (11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                } else {
                    (7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                }) as SIZE;
                DestroyOrbitStruct(&raw mut (*pMoonDesc).orbit, diameterPick);
            }
            (*pMoonDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
            i = i.wrapping_add(1);
            pMoonDesc = pMoonDesc.offset(1);
        }
    } else if isPC(optPlanetStyle).0 != 0 {
        i = 0 as COUNT;
        pMoonDesc = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
        while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
            if (*pMoonDesc).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
            {
                DestroyDrawable(ReleaseDrawable((*pMoonDesc).image.frame));
                (*pMoonDesc).image.frame = ::core::ptr::null_mut::<FRAME_DESC>();
            }
            (*pMoonDesc).frame_offset = !(0 as ::core::ffi::c_int) as BYTE;
            i = i.wrapping_add(1);
            pMoonDesc = pMoonDesc.offset(1);
        }
    }
    (*pSolarSysState).WaitIntersect = outerPlanetWait;
    uqm_lua_checked!(CheckIntersect(), ());
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
    let mut SISonScreen: BOOLEAN = BOOLEAN::FALSE;
    radius = (*pSolarSysState).SunDesc[0usize].radius;
    *newRadius = (*pSolarSysState).SunDesc[0usize].radius;
    SISonScreen = BOOLEAN(
        ((GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
            || GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_int
                >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor))
            || (GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            || GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int
                >= CanvasHeight
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
                    - ((13 as ::core::ffi::c_int) << resolutionFactor))
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if SISonScreen.0 != 0 {
        if !playerInInnerSystem() {
            if radius as ::core::ffi::c_int
                == (128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            {
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    | C2Rust_Unnamed_5::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                    as ACTIVITY;
                return BOOLEAN::FALSE;
            }
            *newRadius = FindRadius(
                GlobData.Game_state.ip_location,
                (((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int) as SIZE,
            );
        } else {
            uqm_lua_checked!(leaveInnerSystem((*pSolarSysState).pOrbitalDesc), BOOLEAN(0));
            if pointWithinRect(scaleRect, GlobData.Game_state.ShipStamp.origin) {
                *newRadius = FindRadius(GlobData.Game_state.ip_location, radius);
                (*pSolarSysState).SunDesc[0usize].radius = *newRadius;
            } else if SISonScreen.0 != 0 {
                *newRadius = FindRadius(
                    GlobData.Game_state.ip_location,
                    (((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as SIZE,
                );
                (*pSolarSysState).SunDesc[0usize].radius = *newRadius;
            }
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
        let mut planet: *mut PLANET_DESC = uqm_lua_checked!(CheckIntersect(), BOOLEAN(0));
        if !planet.is_null() {
            if playerInInnerSystem() {
                enterOrbital(planet);
                return BOOLEAN::FALSE;
            } else {
                uqm_lua_checked!(enterInnerSystem(planet), BOOLEAN(0));
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
        uqm_lua_checked!(DrawInnerSystem(), ());
    } else {
        uqm_lua_checked!(DrawOuterSystem(), ());
    }
    uqm_lua_checked!(RedrawQueue(BOOLEAN::FALSE), ());
    uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), ());
    UnbatchGraphics();
}
unsafe extern "C" fn TransitionSystemIn() {
    SetContext(SpaceContext);
    uqm_lua_checked!(DrawSystemTransition(BOOLEAN(playerInInnerSystem() as ::core::ffi::c_uint)), ());
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
    uqm_lua_checked!(DrawOuterSystem(), ());
    uqm_lua_checked!(RedrawQueue(BOOLEAN::FALSE), ());
    uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), ());
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
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
unsafe extern "C" fn DrawTexturedBody(mut planet: *mut PLANET_DESC, mut s: STAMP) {
    let mut oldScale: ::core::ffi::c_int = 0;
    let mut oldMode: ::core::ffi::c_int = 0;
    let mut moonDiameter: SIZE = 0;
    let mut size: COUNT = PBodySize[(*planet).size as usize];
    BatchGraphics();
    oldMode = SetGraphicScaleMode(
        C2Rust_Unnamed_11::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int,
    );
    if worldIsMoon(pSolarSysState, planet) {
        moonDiameter = (if (*planet).data_index as ::core::ffi::c_int
            > C2Rust_Unnamed_7::LAST_SMALL_ROCKY_WORLD.0 as ::core::ffi::c_int
        {
            (11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        } else {
            (7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        }) as SIZE;
        oldScale = SetGraphicScale(
            256 as ::core::ffi::c_int
                * ((size as ::core::ffi::c_int) << resolutionFactor)
                / moonDiameter as ::core::ffi::c_int,
        );
    } else {
        oldScale = SetGraphicScale(
            256 as ::core::ffi::c_int
                * ((size as ::core::ffi::c_int) << resolutionFactor)
                / ((29 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int),
        );
    }
    s.frame = (*planet).orbit.SphereFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    if !(*planet).orbit.ObjectFrame.is_null() {
        s.frame = (*planet).orbit.ObjectFrame;
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    SetGraphicScale(oldScale);
    SetGraphicScaleMode(oldMode);
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn RotatePlanets(mut IsInnerSystem: BOOLEAN) {
    let mut planet: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    let mut moon: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    static mut frameCounter: SIZE = 0;
    let mut i: COUNT = 0;
    if (*(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC))
        .orbit
        .lpTopoData
        .is_null()
    {
        return;
    }
    frameCounter += 1;
    if IsInnerSystem.0 != 0 {
        planet = (*pSolarSysState).pOrbitalDesc;
        uqm_lua_checked!(PrepareNextRotationFrameForIP(planet, frameCounter), ());
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < (*planet).NumPlanets as ::core::ffi::c_int {
            moon = (&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                .offset(i as isize);
            if (*moon).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
            {
                uqm_lua_checked!(PrepareNextRotationFrameForIP(moon, frameCounter), ());
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = (*pSolarSysState).SunDesc[0usize].NumPlanets as COUNT;
        planet = (&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
            .offset(0isize);
        while i != 0 {
            uqm_lua_checked!(PrepareNextRotationFrameForIP(planet, frameCounter), ());
            i = i.wrapping_sub(1);
            planet = planet.offset(1);
        }
    };
}
unsafe extern "C" fn IP_frame() {
    let mut locChange: BOOLEAN = BOOLEAN::FALSE;
    let mut newRadius: SIZE = 0;
    SetContext(SpaceContext);
    uqm_lua_checked!(ProcessShipControls(), ());
    locChange = uqm_lua_checked!(CheckShipLocation(&raw mut newRadius), ());
    if locChange.0 != 0 {
        if playerInInnerSystem() {
            uqm_lua_checked!(DrawSystemTransition(BOOLEAN::TRUE), ());
        } else if (*pSolarSysState).SunDesc[0usize].radius as ::core::ffi::c_int
            == newRadius as ::core::ffi::c_int
        {
            uqm_lua_checked!(DrawSystemTransition(BOOLEAN::FALSE), ());
        } else {
            uqm_lua_checked!(ScaleSystem(newRadius), ());
        }
    } else if (*pSolarSysState).InOrbit.0 == 0 {
        BatchGraphics();
        uqm_lua_checked!(RestoreSystemView(), ());
        if GetFrameCount(SunFrame) as ::core::ffi::c_int > 10 as ::core::ffi::c_int
            || optOrbitingPlanets.0 != 0 || optTexturedPlanets.0 != 0
        {
            if playerInInnerSystem() {
                ValidateInnerOrbits();
                uqm_lua_checked!(DrawInnerPlanets((*pSolarSysState).pOrbitalDesc), ());
            } else {
                ValidateOrbits();
                uqm_lua_checked!(DrawOuterPlanets((*pSolarSysState).SunDesc[0usize].radius), ());
            }
        }
        uqm_lua_checked!(RedrawQueue(BOOLEAN::FALSE), ());
        uqm_lua_checked!(DrawAutoPilotMessage(BOOLEAN::FALSE), ());
        UnbatchGraphics();
    }
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
            ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                << resolutionFactor,
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                / 4 as ::core::ffi::c_int) << resolutionFactor,
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
                2033 as ::core::ffi::c_int,
                b"ValidateInnerOrbits\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    planet = (*pSolarSysState).pOrbitalDesc;
    ValidateOrbit(
        planet,
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
            - 8 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) << resolutionFactor,
        ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - 8 as ::core::ffi::c_int) << resolutionFactor,
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
    uqm_lua_checked!(DrawSystem((*(*pSolarSysState).pOrbitalDesc).radius, BOOLEAN::TRUE), ());
    if GetFrameCount(SunFrame) as ::core::ffi::c_int > 10 as ::core::ffi::c_int
        || optOrbitingPlanets.0 != 0 || optTexturedPlanets.0 != 0
    {
        uqm_lua_checked!(DrawInnerPlanets((*pSolarSysState).pOrbitalDesc), ());
    }
    uqm_lua_checked!(DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE), ());
}
unsafe extern "C" fn DrawOuterSystem() {
    ValidateOrbits();
    uqm_lua_checked!(DrawSystem((*pSolarSysState).SunDesc[0usize].radius, BOOLEAN::FALSE), ());
    if GetFrameCount(SunFrame) as ::core::ffi::c_int > 10 as ::core::ffi::c_int
        || optOrbitingPlanets.0 != 0 || optTexturedPlanets.0 != 0
    {
        uqm_lua_checked!(DrawOuterPlanets((*pSolarSysState).SunDesc[0usize].radius), ());
    }
    uqm_lua_checked!(DrawHyperCoords((*CurStarDescPtr).star_pt), ());
}
#[no_mangle]
pub unsafe extern "C" fn spaceMusicSwitch(mut SpeciesID: BYTE) -> RESOURCE {
    match SpeciesID as ::core::ffi::c_int {
        1 => return b"space.arilou.music\0".as_ptr() as RESOURCE,
        2 => return b"space.chmmr.music\0".as_ptr() as RESOURCE,
        4 => return b"space.orz.music\0".as_ptr() as RESOURCE,
        5 => return b"space.pkunk.music\0".as_ptr() as RESOURCE,
        7 => return b"space.spathi.music\0".as_ptr() as RESOURCE,
        8 => return b"space.supox.music\0".as_ptr() as RESOURCE,
        9 => return b"space.thraddash.music\0".as_ptr() as RESOURCE,
        10 => return b"space.utwig.music\0".as_ptr() as RESOURCE,
        11 => return b"space.vux.music\0".as_ptr() as RESOURCE,
        12 => return b"space.yehat.music\0".as_ptr() as RESOURCE,
        14 => return b"space.druuge.music\0".as_ptr() as RESOURCE,
        15 => return b"space.ilwrath.music\0".as_ptr() as RESOURCE,
        16 => return b"space.mycon.music\0".as_ptr() as RESOURCE,
        18 => return b"space.umgah.music\0".as_ptr() as RESOURCE,
        19 | 22 => return b"space.urquan.music\0".as_ptr() as RESOURCE,
        20 => return b"space.zoqfotpik.music\0".as_ptr() as RESOURCE,
        21 => return b"space.syreen.music\0".as_ptr() as RESOURCE,
        27 => return b"space.kohrah.music\0".as_ptr() as RESOURCE,
        _ => return b"music.space\0".as_ptr() as RESOURCE,
    };
}
unsafe extern "C" fn playSpaceMusic() {
    if SpaceMusic.is_null() {
        if SpaceMusicOK.0 != 0 {
            uqm_lua_checked!(findRaceSOI(), ());
            SpaceMusic = uqm_lua_checked!(LoadMusicInstance(spaceMusicSwitch(spaceMusicBySOI as BYTE)), ());
        } else {
            SpaceMusic = uqm_lua_checked!(LoadMusicInstance(b"music.space\0".as_ptr() as RESOURCE), ());
        }
    }
    if PLRPlaying(
            ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize),
        )
        .0 == 0
        && (LastActivity as ::core::ffi::c_int
            != C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
            || NextActivity as ::core::ffi::c_int != 0)
    {
        uqm_lua_checked!(PlayMusicResume(SpaceMusic, 160 as BYTE), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn ResetSolarSys() {
    ((*pSolarSysState).InIpFlight.0 == 0
        || {
            __assert_fail(
                b"!pSolarSysState->InIpFlight\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2145 as ::core::ffi::c_int,
                b"ResetSolarSys\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_9::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
        -(C2Rust_Unnamed_9::PM_NAVIGATE.0 as ::core::ffi::c_int
            - C2Rust_Unnamed_9::PM_SCAN.0 as ::core::ffi::c_int) as SWORD,
    ), ());
    InitDisplayList();
    uqm_lua_checked!(DoMissions(), ());
    uqm_lua_checked!(CheckIntersect(), ());
    (*pSolarSysState).InIpFlight = BOOLEAN::TRUE;
    uqm_lua_checked!(playSpaceMusic(), ());
}
unsafe extern "C" fn EnterPlanetOrbit() {
    if (*pSolarSysState).InIpFlight.0 != 0 {
        uqm_lua_checked!(FreeSolarSys(), ());
    }
    if worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
        GlobData.Game_state.ShipStamp.origin = (*(*pSolarSysState).pOrbitalDesc)
            .image
            .origin;
        if uqm_lua_checked!(GetNamedPlanetaryBody(), ()).is_null() && isPC(optWhichFonts).0 != 0
            && (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
            && LastActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int == 0
        {
            snprintf(
                (&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE)
                    .offset(
                        strlen(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE)
                            as isize,
                    ),
                4 as size_t,
                b"-%c%c\0".as_ptr() as *const ::core::ffi::c_char,
                moonLetter(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                    as ::core::ffi::c_int,
                '\0' as ::core::ffi::c_int,
            );
        }
    } else {
        GlobData.Game_state.ShipStamp.origin.x = ((CanvasWidth
            - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        GlobData.Game_state.ShipStamp.origin.y = ((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    GetPlanetInfo();
    uqm_lua_checked!(Some(
            (*(*pSolarSysState).genFuncs)
                .generateOrbital
                .expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )(pSolarSysState, (*pSolarSysState).pOrbitalDesc), ());
    LastActivity = (LastActivity as ::core::ffi::c_int
        & !(C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_RESTART.0 as ::core::ffi::c_int)) as ACTIVITY;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::START_ENCOUNTER.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        || uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 2 as DWORD
    {
        return;
    }
    if !(*pSolarSysState).TopoFrame.is_null() {
        uqm_lua_checked!(PlanetOrbitMenu(), ());
        uqm_lua_checked!(FreePlanet(), ());
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_5::START_ENCOUNTER.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        let mut orbital: *mut PLANET_DESC = uqm_lua_checked!(LoadSolarSys(), ());
        (orbital.is_null()
            || {
                __assert_fail(
                    b"!orbital\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/solarsys.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    2229 as ::core::ffi::c_int,
                    b"EnterPlanetOrbit\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        CheckZoomLevel();
        ValidateOrbits();
        ValidateInnerOrbits();
        uqm_lua_checked!(ResetSolarSys(), ());
        if optTexturedPlanets.0 != 0 {
            if worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
                uqm_lua_checked!(GenerateTexturedMoons(
                    pSolarSysState,
                    (*(*pSolarSysState).pOrbitalDesc).pPrevDesc,
                ), ());
            } else {
                uqm_lua_checked!(GenerateTexturedMoons(pSolarSysState, (*pSolarSysState).pOrbitalDesc), ());
            }
        }
        uqm_lua_checked!(RepairSISBorder(), ());
        uqm_lua_checked!(TransitionSystemIn(), ());
    }
}
unsafe extern "C" fn InitSolarSys() {
    let mut InnerSystem: BOOLEAN = BOOLEAN::FALSE;
    let mut Reentry: BOOLEAN = BOOLEAN::FALSE;
    let mut orbital: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    uqm_lua_checked!(LoadIPData(), ());
    uqm_lua_checked!(LoadLanderData(), ());
    Reentry = BOOLEAN(
        (GlobData.Game_state.ShipFacing as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if Reentry.0 == 0 {
        GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.ShipStamp.origin.x = ((CanvasWidth
            - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        GlobData.Game_state.ShipStamp.origin.y = (CanvasHeight
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
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        GlobData.Game_state.ip_location = displayToLocation(
            GlobData.Game_state.ShipStamp.origin,
            ((128 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as SIZE,
        );
    }
    SetColorMap(
        GetColorMapAddress(
            SetAbsStringTableIndex(
                SunCMap,
                (((*CurStarDescPtr).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                    as COUNT,
            ),
        ),
    );
    if GetFrameCount(SunFrame) as ::core::ffi::c_int > 5 as ::core::ffi::c_int {
        uqm_lua_checked!(ChangeSunColor(), ());
    }
    StarsFrame = uqm_lua_checked!(GetStarBackGround(BOOLEAN::FALSE), ());
    SetContext(SpaceContext);
    SetContextFGFrame(Screen);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    orbital = uqm_lua_checked!(LoadSolarSys(), ());
    InnerSystem = CheckZoomLevel();
    ValidateOrbits();
    if InnerSystem.0 != 0 {
        ValidateInnerOrbits();
    }
    if Reentry.0 != 0 {
        uqm_lua_checked!(Some(
                (*(*pSolarSysState).genFuncs)
                    .reinitNpcs
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(pSolarSysState), ());
    } else {
        EncounterRace = -1 as SIZE;
        EncounterGroup = 0 as BYTE;
        GlobData.Game_state.BattleGroupRef = 0 as DWORD;
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        uqm_lua_checked!(Some((*(*pSolarSysState).genFuncs).initNpcs.expect("non-null function pointer"))
            .expect("non-null function pointer")(pSolarSysState), ());
    }
    if !orbital.is_null() {
        enterOrbital(orbital);
    } else {
        SetContext(SpaceContext);
        SetTransitionSource(::core::ptr::null::<RECT>());
        BatchGraphics();
        uqm_lua_checked!(DrawSISFrame(), ());
        uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), ());
        uqm_lua_checked!(ResetSolarSys(), ());
        if uqm_lua_checked!(isStarMarked(
                -1 as ::core::ffi::c_int,
                b"VISITED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .0 == 0
        {
            uqm_lua_checked!(setStarMarked(
                -1 as ::core::ffi::c_int,
                b"VISITED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ());
        }
        if uqm_lua_checked!(isStarMarked(
                -1 as ::core::ffi::c_int,
                b"PLYR_MARKER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ())
            .0 != 0
        {
            uqm_lua_checked!(setStarMarked(
                -1 as ::core::ffi::c_int,
                b"PLYR_MARKER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ());
        }
        if LastActivity as ::core::ffi::c_int
            == C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_RESTART.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(DrawOuterSystem(), ());
            uqm_lua_checked!(RedrawQueue(BOOLEAN::FALSE), ());
            UnbatchGraphics();
            FadeScreen(
                ScreenFadeType::FadeAllToColor,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            );
            NewGameInit = BOOLEAN::TRUE;
            LastActivity = 0 as ACTIVITY;
        } else if LastActivity as ::core::ffi::c_int
            == C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int && NextActivity == 0
        {
            UnbatchGraphics();
        } else {
            if InnerSystem.0 != 0 {
                if optTexturedPlanets.0 != 0 {
                    uqm_lua_checked!(GenerateTexturedMoons(
                        pSolarSysState,
                        (*pSolarSysState).pOrbitalDesc,
                    ), ());
                }
                uqm_lua_checked!(DrawInnerSystem(), ());
            } else {
                uqm_lua_checked!(DrawOuterSystem(), ());
            }
            uqm_lua_checked!(RedrawQueue(BOOLEAN::FALSE), ());
            uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), ());
            UnbatchGraphics();
            LastActivity = (LastActivity as ::core::ffi::c_int
                & !(C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
        }
    };
}
unsafe extern "C" fn endInterPlanetary() {
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int
        & !(C2Rust_Unnamed_5::START_INTERPLANETARY.0 as ::core::ffi::c_int)) as ACTIVITY;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        uqm_lua_checked!(Some(
                (*(*pSolarSysState).genFuncs)
                    .uninitNpcs
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(pSolarSysState), ());
        uqm_lua_checked!(setGameStateUint(
            b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
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
                2396 as ::core::ffi::c_int,
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
    uqm_lua_checked!(FreeSolarSys(), ());
    uqm_lua_checked!(FreeIPData(), ());
    DestroyDrawable(ReleaseDrawable(StarsFrame));
    StarsFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_5::START_INTERPLANETARY.0 as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(endInterPlanetary(), ());
        return;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_5::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
        && EncounterGroup as ::core::ffi::c_int != 0
    {
        GetGroupInfo(GlobData.Game_state.BattleGroupRef, EncounterGroup);
        if GlobData.Game_state.ip_planet as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let mut planet: *mut PLANET_DESC = closestPlanetInterPlanetary(
                &raw mut GlobData.Game_state.ShipStamp.origin,
            );
            uqm_lua_checked!(Some(
                    (*(*pSolarSysState).genFuncs)
                        .generateName
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(pSolarSysState, planet), ());
        }
    }
}
unsafe extern "C" fn CalcSunSize(mut pSunDesc: *mut PLANET_DESC, mut radius: SIZE) {
    static mut frameCount: BYTE = 0 as BYTE;
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
    (*pSunDesc).image.origin.x = ((CanvasWidth
        - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    (*pSunDesc).image.origin.y = ((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if GetFrameCount(SunFrame) as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
        (*pSunDesc).image.frame = SetRelFrameIndex(
            SunFrame,
            (index as ::core::ffi::c_int * 32 as ::core::ffi::c_int
                + frameCount as ::core::ffi::c_int) as SIZE,
        );
        frameCount = (frameCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int) as BYTE;
    } else {
        (*pSunDesc).image.frame = SetRelFrameIndex(SunFrame, index);
    };
}
unsafe extern "C" fn SetPlanetColorMap(mut planet: *mut PLANET_DESC) {
    let mut index: COUNT = ((*planet).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)) as COUNT;
    ((index as ::core::ffi::c_int)
        < C2Rust_Unnamed_7::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"index < NUMBER_OF_PLANET_TYPES\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/solarsys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2486 as ::core::ffi::c_int,
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
    s.origin.y = ((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if optTexturedPlanets.0 != 0 {
        uqm_lua_checked!(RotatePlanets(BOOLEAN::TRUE), ());
        uqm_lua_checked!(DrawTexturedBody(planet, s), ());
        i = (*planet).NumPlanets as COUNT;
        moon = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
        while i != 0 {
            if (*moon).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int != 0
            {
                uqm_lua_checked!(DrawStamp(&raw mut (*moon).image), ());
            } else {
                uqm_lua_checked!(DrawTexturedBody(moon, (*moon).image), ());
            }
            i = i.wrapping_sub(1);
            moon = moon.offset(1);
        }
    } else {
        SetPlanetColorMap(planet);
        s.frame = (*planet).image.frame;
        i = ((*planet).data_index as ::core::ffi::c_int
            & !(C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int)) as COUNT;
        if (i as ::core::ffi::c_int)
            < C2Rust_Unnamed_7::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
            && (*planet).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
        {
            let mut angle: COUNT = ARCTAN(
                (*planet).location.x as SDWORD,
                (*planet).location.y as SDWORD,
            );
            s.frame = SetAbsFrameIndex(
                OrbitalShield,
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as COUNT,
            );
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        i = (*planet).NumPlanets as COUNT;
        moon = &raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC;
        while i != 0 {
            if (*moon).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_7::WORLD_TYPE_SPECIAL.0 as ::core::ffi::c_int == 0
            {
                SetPlanetColorMap(moon);
            }
            uqm_lua_checked!(DrawStamp(&raw mut (*moon).image), ());
            i = i.wrapping_sub(1);
            moon = moon.offset(1);
        }
    };
}
unsafe extern "C" fn DrawOuterPlanets(mut radius: SIZE) {
    let mut index: SIZE = 0;
    let mut pCurDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    CalcSunSize(
        (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC).offset(0isize),
        radius,
    );
    if optOrbitingPlanets.0 != 0 {
        sortPlanetPositions();
    }
    index = (*pSolarSysState).FirstPlanetIndex;
    loop {
        pCurDesc = solarDisplayBody(index);
        if pCurDesc
            == (&raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC).offset(0isize)
        {
            uqm_lua_checked!(DrawStamp(&raw mut (*pCurDesc).image), ());
        } else if optTexturedPlanets.0 != 0 {
            uqm_lua_checked!(RotatePlanets(BOOLEAN::FALSE), ());
            uqm_lua_checked!(DrawTexturedBody(pCurDesc, (*pCurDesc).image), ());
        } else {
            SetPlanetColorMap(pCurDesc);
            uqm_lua_checked!(DrawStamp(&raw mut (*pCurDesc).image), ());
        }
        if index as ::core::ffi::c_int
            == (*pSolarSysState).LastPlanetIndex as ::core::ffi::c_int
        {
            break;
        }
        index = (*pCurDesc).NextIndex as SIZE;
    };
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
    if optTexturedPlanets.0 != 0 {
        if (*(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC))
            .orbit
            .lpTopoData
            .is_null()
        {
            uqm_lua_checked!(GenerateTexturedPlanets(), ());
        }
    }
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
    uqm_lua_checked!(DrawStarBackGround(), ());
    pBaseDesc = (*pSolarSysState).pBaseDesc;
    if IsInnerSystem.0 != 0 {
        pCurDesc = (*pSolarSysState).pOrbitalDesc;
        uqm_lua_checked!(DrawOrbit(
            pCurDesc,
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                * 4 as ::core::ffi::c_int) << resolutionFactor,
            ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                << resolutionFactor,
            radius as ::core::ffi::c_int,
        ), ());
    }
    i = (*(*pBaseDesc).pPrevDesc).NumPlanets;
    pCurDesc = pBaseDesc;
    while i != 0 {
        if IsInnerSystem.0 != 0 {
            uqm_lua_checked!(DrawOrbit(
                pCurDesc,
                2 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            ), ());
        } else {
            uqm_lua_checked!(DrawOrbit(
                pCurDesc,
                ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor)) >> resolutionFactor
                    >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                    << resolutionFactor,
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
                        } << resolutionFactor)) >> resolutionFactor
                    >> 1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int)
                    / 4 as ::core::ffi::c_int) << resolutionFactor,
                radius as ::core::ffi::c_int,
            ), ());
        }
        i = i.wrapping_sub(1);
        pCurDesc = pCurDesc.offset(1);
    }
    if optOrbitingPlanets.0 == 0 && optTexturedPlanets.0 == 0 {
        if IsInnerSystem.0 != 0 {
            uqm_lua_checked!(DrawInnerPlanets((*pSolarSysState).pOrbitalDesc), ());
        } else {
            uqm_lua_checked!(DrawOuterPlanets(radius), ());
        }
    }
    SetContext(oldContext);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SolarSysFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
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
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
#[no_mangle]
pub unsafe extern "C" fn GetStarBackFround() -> FRAME {
    if !StarsFrame.is_null() {
        return StarsFrame
    } else {
        return ::core::ptr::null_mut::<FRAME_DESC>()
    };
}
unsafe extern "C" fn DrawBackgroundStars(
    mut num_pieces: BYTE,
    mut num_drawn: BYTE,
    mut start_index: BYTE,
    mut SysRNG: *mut RandomContext,
    mut junk: FRAME,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    let mut rand_val: DWORD = 0;
    s.frame = SetAbsFrameIndex(junk, start_index as COUNT);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < num_pieces as ::core::ffi::c_int {
        j = 0 as COUNT;
        while (j as ::core::ffi::c_int) < num_drawn as ::core::ffi::c_int {
            rand_val = RandomContext_Random(SysRNG);
            s.origin.x = scaleSISWidth(
                (rand_val as UWORD as ::core::ffi::c_int
                    % widthPick() as ::core::ffi::c_int) as COORD,
            );
            s.origin.y = scaleSISHeight(
                ((rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                    % heightPick() as ::core::ffi::c_int) as COORD,
            );
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            j = j.wrapping_add(1);
        }
        s.frame = IncFrameIndex(s.frame);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawNebula(mut nebula: FRAME) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    if nebula.is_null() {
        return;
    }
    oldMode = SetContextDrawMode(
        MAKE_DRAW_MODE(
            DrawKind::DRAW_ALPHA,
            ((optNebulaeVolume + 1 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                as SWORD,
        ),
    );
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = nebula;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    SetContextDrawMode(oldMode);
}
unsafe extern "C" fn CreateStarBackGround(
    mut SysRNG: *mut RandomContext,
    mut nebula: FRAME,
    mut junk: FRAME,
) -> FRAME {
    let mut num_brt_drawn: BYTE = 0;
    let mut start_index: BYTE = 0;
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut clipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut hdScaled: BOOLEAN = BOOLEAN(
        (optUnscaledStarSystem.0 == 0
            || if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
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
    uqm_lua_checked!(ClearDrawable(), ::core::ptr::null_mut());
    start_index = (if hdScaled.0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        25 as ::core::ffi::c_int
    }) as BYTE;
    uqm_lua_checked!(DrawBackgroundStars(8 as BYTE, 5 as BYTE, start_index, SysRNG, junk), ::core::ptr::null_mut());
    start_index = (start_index as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as BYTE;
    num_brt_drawn = (30 as ::core::ffi::c_int
        + if hdScaled.0 != 0 {
            0 as ::core::ffi::c_int
        } else {
            60 as ::core::ffi::c_int
        }) as BYTE;
    uqm_lua_checked!(DrawBackgroundStars(8 as BYTE, num_brt_drawn, start_index, SysRNG, junk), ::core::ptr::null_mut());
    uqm_lua_checked!(DrawNebula(nebula), ::core::ptr::null_mut());
    SetContext(oldContext);
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn GetStarBackGround(mut encounter: BOOLEAN) -> FRAME {
    let mut SysRNG: *mut RandomContext = ::core::ptr::null_mut::<RandomContext>();
    let mut location: POINT = POINT { x: 0, y: 0 };
    let mut junk: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut nebula: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut result: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    if encounter.0 == 0 {
        RandomContext_SeedRandom(SysGenRNG, GetRandomSeedForStar(CurStarDescPtr));
        return uqm_lua_checked!(CreateStarBackGround(SysGenRNG, NebulaFrame, SpaceJunkFrame), ::core::ptr::null_mut());
    }
    if optNebulae.0 == 0 {
        return CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.segue\0".as_ptr() as RESOURCE), ::core::ptr::null_mut()) as DRAWABLE,
        );
    }
    SysRNG = RandomContext_New();
    junk = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.orbitbackground\0".as_ptr() as RESOURCE), ::core::ptr::null_mut())
            as DRAWABLE,
    );
    if !CurStarDescPtr.is_null() {
        location = (*CurStarDescPtr).star_pt;
    } else {
        location = MAKE_POINT(
            logxToUniverse(GlobData.SIS_state.log_x) as COORD,
            logyToUniverse(GlobData.SIS_state.log_y) as COORD,
        );
    }
    RandomContext_SeedRandom(SysRNG, GetRandomSeedForVar(location));
    nebula = uqm_lua_checked!(LoadNebulaeFrame(location), ::core::ptr::null_mut());
    result = uqm_lua_checked!(CreateStarBackGround(SysRNG, nebula, junk), ::core::ptr::null_mut());
    DestroyDrawable(ReleaseDrawable(junk));
    junk = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(nebula));
    nebula = ::core::ptr::null_mut::<FRAME_DESC>();
    RandomContext_Delete(SysRNG);
    SysRNG = ::core::ptr::null_mut::<RandomContext>();
    return result;
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
    if CurStarDescPtr.is_null() {
        let mut universe: POINT = POINT { x: 0, y: 0 };
        universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
        universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
        CurStarDescPtr = uqm_lua_checked!(FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut universe,
            1 as SIZE,
            1 as SIZE,
        ), ());
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
    uqm_lua_checked!(InitSolarSys(), ());
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    SolarSysState.InputFunc = Some(
        DoIpFlight as unsafe extern "C" fn(*mut SOLARSYS_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut SolarSysState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), ());
    uqm_lua_checked!(UninitSolarSys(), ());
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
                2870 as ::core::ffi::c_int,
                b"GetNamedPlanetaryBody\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*CurStarDescPtr).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::SOL_DEFINED.0 as ::core::ffi::c_int
    {
        let mut planet: ::core::ffi::c_int = 0;
        let mut moon: ::core::ffi::c_int = 0;
        planet = planetIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
            as ::core::ffi::c_int;
        if worldIsPlanet(pSolarSysState, (*pSolarSysState).pOrbitalDesc) {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
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
                                (C2Rust_Unnamed_10::STARBASE_STRING_BASE.0
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    1 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
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
                                (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    1 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    2 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                    as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                    }
                    3 => {
                        return GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
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
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            7 => {
                if moon == 0 as ::core::ffi::c_int {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            8 => {
                if moon == 0 as ::core::ffi::c_int {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            _ => {}
        }
    } else if (*CurStarDescPtr).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_8::SAMATRA_DEFINED.0 as ::core::ffi::c_int
        && matchWorld(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            (*pSolarSysState).SunDesc[0usize].PlanetByte,
            (*pSolarSysState).SunDesc[0usize].MoonByte,
        ) as ::core::ffi::c_int != 0
    {
        return GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int) as COUNT,
            ),
        )
    } else if (*CurStarDescPtr).Index as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        && matchWorld(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            (*pSolarSysState).SunDesc[0usize].PlanetByte,
            -1 as ::core::ffi::c_int as BYTE,
        ) as ::core::ffi::c_int != 0
    {
        match (*CurStarDescPtr).Index as ::core::ffi::c_int {
            2 => {
                if uqm_lua_checked!(IsHomeworldKnown(
                        C2Rust_Unnamed_3::SHOFIXTI_HOME.0 as ::core::ffi::c_int as DWORD,
                    ), ::core::ptr::null_mut())
                    .0 != 0
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            4 => {
                return GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                            + 33 as ::core::ffi::c_int) as COUNT,
                    ),
                );
            }
            5 => {
                if uqm_lua_checked!(IsHomeworldKnown(
                        C2Rust_Unnamed_3::SPATHI_HOME.0 as ::core::ffi::c_int as DWORD,
                    ), ::core::ptr::null_mut())
                    .0 != 0
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            18 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"SYREEN_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ::core::ptr::null_mut()) != 0
                    || uqm_lua_checked!(getGameStateUint(
                        b"SYREEN_KNOW_ABOUT_MYCON\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ), ::core::ptr::null_mut()) != 0
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            20 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"SLYLANDRO_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ::core::ptr::null_mut()) != 0
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            21 => {
                if uqm_lua_checked!(IsHomeworldKnown(
                        C2Rust_Unnamed_3::DRUUGE_HOME.0 as ::core::ffi::c_int as DWORD,
                    ), ::core::ptr::null_mut())
                    .0 != 0
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            32 => {
                return GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                            + 42 as ::core::ffi::c_int) as COUNT,
                    ),
                );
            }
            36 => {
                if uqm_lua_checked!(IsHomeworldKnown(
                        C2Rust_Unnamed_3::UTWIG_HOME.0 as ::core::ffi::c_int as DWORD,
                    ), ::core::ptr::null_mut())
                    .0 != 0
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            37 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
                ), ::core::ptr::null_mut()) > 2 as DWORD
                {
                    return GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_10::PLANET_NUMBER_BASE.0
                                as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            _ => return ::core::ptr::null_mut::<UNICODE>(),
        }
    }
    return ::core::ptr::null_mut::<UNICODE>();
}
#[no_mangle]
pub unsafe extern "C" fn GetPlanetOrMoonName(mut buf: *mut UNICODE, mut bufsize: COUNT) {
    let mut named: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut tempbuf: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut moon: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut name_has_suffix: BOOLEAN = BOOLEAN::FALSE;
    named = uqm_lua_checked!(GetNamedPlanetaryBody(), ());
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
            as ::core::ffi::c_int != 0 || is3DO(optWhichFonts).0 != 0
    {
        return;
    }
    i = strlen(buf) as ::core::ffi::c_int;
    tempbuf = buf;
    buf = buf.offset(i as isize);
    bufsize = (bufsize as ::core::ffi::c_int - i) as COUNT;
    moon = moonIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
        as ::core::ffi::c_int;
    if i > 0 as ::core::ffi::c_int {
        if *tempbuf.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == 'A' as ::core::ffi::c_int
            || *tempbuf.offset((i - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 'B' as ::core::ffi::c_int
            || *tempbuf.offset((i - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 'C' as ::core::ffi::c_int
            || *tempbuf.offset((i - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 'D' as ::core::ffi::c_int
        {
            name_has_suffix = BOOLEAN::TRUE;
        }
    }
    if bufsize as ::core::ffi::c_int >= 3 as ::core::ffi::c_int && name_has_suffix.0 == 0
    {
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
                3030 as ::core::ffi::c_int,
                b"SaveSolarSysLocation\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !playerInPlanetOrbit() {
        saveNonOrbitalLocation();
    } else {
        let mut moon: BYTE = 0;
        if uqm_lua_checked!(getGameStateUint(b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            PutPlanetInfo();
            uqm_lua_checked!(setGameStateUint(
                b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        }
        (GlobData.Game_state.ip_planet as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"GLOBAL (ip_planet) != 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/solarsys.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    3052 as ::core::ffi::c_int,
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
        & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    handled = uqm_lua_checked!(DoMenuChooser(
        pMS,
        C2Rust_Unnamed_9::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
    ), BOOLEAN(0));
    if handled.0 != 0 {
        return BOOLEAN::TRUE;
    }
    if LastActivity as ::core::ffi::c_int
        == C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
    {
        select = BOOLEAN::TRUE;
    }
    if select.0 == 0 {
        return BOOLEAN::TRUE;
    }
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    's_112: {
        match (*pMS).CurState as ::core::ffi::c_int {
            2 => {
                select = uqm_lua_checked!(DevicesMenu(), BOOLEAN(0));
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_5::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                break 's_112;
            }
            3 => {
                uqm_lua_checked!(CargoMenu(), BOOLEAN(0));
                break 's_112;
            }
            4 => {
                select = uqm_lua_checked!(RosterMenu(), BOOLEAN(0));
                break 's_112;
            }
            5 => {
                if uqm_lua_checked!(GameOptions(), BOOLEAN(0)).0 == 0 {
                    return BOOLEAN::FALSE;
                }
                break 's_112;
            }
            1 => {
                uqm_lua_checked!(StarMap(), BOOLEAN(0));
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                uqm_lua_checked!(TransitionSystemIn(), BOOLEAN(0));
                if optWhichMenu != 0x2 as ::core::ffi::c_int {
                    uqm_lua_checked!(DrawMenuStateStrings(
                        C2Rust_Unnamed_9::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
                        SolarSysMenuMenuItems::NAVIGATION.0 as ::core::ffi::c_int
                            as SWORD,
                    ), BOOLEAN(0));
                }
            }
            6 => {}
            _ => {
                break 's_112;
            }
        }
        return BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if select.0 != 0 {
            if optWhichMenu != 0x2 as ::core::ffi::c_int {
                (*pMS).CurState = SolarSysMenuMenuItems::NAVIGATION.0
                    as ::core::ffi::c_int as BYTE;
            }
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_9::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            ), BOOLEAN(0));
        }
        uqm_lua_checked!(SetFlashRect(
            ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(0 as ::core::ffi::c_long) as usize),
            BOOLEAN::FALSE,
        ), BOOLEAN(0));
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
        flash_rect2: RECT {
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
        == C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int
    {
        MenuState.CurState = SolarSysMenuMenuItems::GAME_MENU.0 as ::core::ffi::c_int
            as BYTE;
    } else {
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_9::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
            SolarSysMenuMenuItems::STARMAP.0 as ::core::ffi::c_int as SWORD,
        ), ());
        MenuState.CurState = SolarSysMenuMenuItems::STARMAP.0 as ::core::ffi::c_int
            as BYTE;
    }
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
    uqm_lua_checked!(SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
        BOOLEAN::FALSE,
    ), ());
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
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), ());
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_9::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
            -(SolarSysMenuMenuItems::NAVIGATION.0 as ::core::ffi::c_int) as SWORD,
        ), ());
    }
}
unsafe extern "C" fn DoIpFlight(mut pSS: *mut SOLARSYS_STATE) -> BOOLEAN {
    let mut cancel: BOOLEAN = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if (*pSS).InOrbit.0 != 0 {
        DirJoyActive = BOOLEAN::FALSE;
        uqm_lua_checked!(EnterPlanetOrbit(), BOOLEAN(0));
        SetMenuSounds(
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        );
        (*pSS).InOrbit = BOOLEAN::FALSE;
    } else if NewGameInit.0 == 0
        && (cancel.0 != 0
            || LastActivity as ::core::ffi::c_int
                == C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int)
    {
        DirJoyActive = BOOLEAN::FALSE;
        uqm_lua_checked!(SolarSysMenu(), BOOLEAN(0));
        SetMenuSounds(
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        );
    } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        static mut TimeOutIP: TimeCount = 0;
        static mut TimeOutClock: TimeCount = 0;
        let mut Now: TimeCount = GetTimeCounter();
        ((*pSS).InIpFlight.0 != 0
            || {
                __assert_fail(
                    b"pSS->InIpFlight\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/solarsys.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    3190 as ::core::ffi::c_int,
                    b"DoIpFlight\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        DirJoyActive = BOOLEAN::TRUE;
        if Now >= TimeOutClock {
            uqm_lua_checked!(GameClockTick(), BOOLEAN(0));
            TimeOutClock = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as TimeCount,
                );
        }
        if Now >= TimeOutIP {
            uqm_lua_checked!(IP_frame(), BOOLEAN(0));
            TimeOutIP = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
                );
        }
        if NewGameInit.0 != 0 {
            DirJoyActive = BOOLEAN::FALSE;
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            uqm_lua_checked!(SettingsMenu(BOOLEAN::FALSE), BOOLEAN(0));
            uqm_lua_checked!(SolarSysMenu(), BOOLEAN(0));
            SetMenuSounds(
                0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
                0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            );
        }
    }
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_5::START_ENCOUNTER.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::START_INTERPLANETARY.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn widthPick() -> COORD {
    let mut sis_screen_dimensions: [EXTENT; 4] = [
        extent {
            width: 243 as COORD,
            height: 187 as COORD,
        },
        extent {
            width: 210 as COORD,
            height: 195 as COORD,
        },
        extent {
            width: 242 as COORD,
            height: 227 as COORD,
        },
        extent {
            width: 1074 as COORD,
            height: 924 as COORD,
        },
    ];
    return sis_screen_dimensions[optStarBackground as usize].width;
}
unsafe extern "C" fn heightPick() -> COORD {
    let mut sis_screen_dimensions: [EXTENT; 4] = [
        extent {
            width: 243 as COORD,
            height: 187 as COORD,
        },
        extent {
            width: 210 as COORD,
            height: 195 as COORD,
        },
        extent {
            width: 242 as COORD,
            height: 227 as COORD,
        },
        extent {
            width: 1074 as COORD,
            height: 924 as COORD,
        },
    ];
    return sis_screen_dimensions[optStarBackground as usize].height;
}
unsafe extern "C" fn scaleSISWidth(mut value: COORD) -> COORD {
    let mut percent: ::core::ffi::c_float = 1 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    if widthPick() as ::core::ffi::c_int
        != CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor
    {
        percent = scaleThing(
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
                    } << resolutionFactor)) >> resolutionFactor) as ::core::ffi::c_float,
            widthPick() as ::core::ffi::c_float,
        );
    }
    return (((value as ::core::ffi::c_int as ::core::ffi::c_float * percent) as COORD
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
unsafe extern "C" fn scaleSISHeight(mut value: COORD) -> COORD {
    let mut percent: ::core::ffi::c_float = 1 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    if heightPick() as ::core::ffi::c_int
        != CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
    {
        percent = scaleThing(
            (CanvasHeight
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
                as ::core::ffi::c_float,
            heightPick() as ::core::ffi::c_float,
        );
    }
    return (((value as ::core::ffi::c_int as ::core::ffi::c_float * percent) as COORD
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn InitialIntersect() {
    uqm_lua_checked!(CheckIntersect(), ());
}
#[inline]
unsafe extern "C" fn LoadAdvancedQuasiPilot() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
        y: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
    };
}
#[inline]
unsafe extern "C" fn LoadAdvancedAutoPilot() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
        y: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
    };
}
#[inline]
unsafe extern "C" fn ZeroAdvancedQuasiPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn ZeroAdvancedAutoPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn IsHomeworldKnown(mut homeworld: DWORD) -> BOOLEAN {
    if homeworld > 18 as DWORD {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(
        uqm_lua_checked!(getGameStateUint(b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            & ((1 as ::core::ffi::c_int) << homeworld) as DWORD,
    );
}
#[inline]
unsafe extern "C" fn scaleThing(
    mut original: ::core::ffi::c_float,
    mut thingToScale: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return original / thingToScale;
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
