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
    pub type font_desc;
    pub type tfb_soundsample;
    static mut Elements: *const BYTE;
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
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn ApplyMask(layer: FRAME, base: FRAME, mode: DrawMode, fill: *mut Color);
    fn font_DrawText(pText: *mut TEXT);
    fn DrawBatch(
        pBasePrim: *mut PRIMITIVE,
        PrimLinks: PRIM_LINKS,
        BatchFlags: BATCH_FLAGS,
    );
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
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
    ) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCountN(
        start: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> size_t;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn GetTimeCounter() -> TimeCount;
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
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
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optGodModes: ::core::ffi::c_int;
    static mut optPartialPickup: OPT_ENABLABLE;
    static mut optSubmenu: OPT_ENABLABLE;
    static mut optVolasMusic: OPT_ENABLABLE;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optScatterElements: OPT_ENABLABLE;
    static mut optShowUpgrades: OPT_ENABLABLE;
    static mut opt3doMusic: OPT_ENABLABLE;
    static mut optRemixMusic: OPT_ENABLABLE;
    static mut DirJoyActive: BOOLEAN;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
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
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn FreeElement(hElement: HELEMENT);
    fn RemoveElement(hLink: HLINK);
    static mut DisplayLinks: PRIM_LINKS;
    fn RepairBackRect(pRect: *mut RECT);
    fn callGenerateForScanType(
        _: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        node: COUNT,
        scanType: BYTE,
        _: *mut NODE_INFO,
    ) -> COUNT;
    fn callPickupForScanType(
        solarSys: *mut SOLARSYS_STATE,
        world: *mut PLANET_DESC,
        node: COUNT,
        scanType: BYTE,
    ) -> bool;
    fn RedrawSurfaceScan(newLoc: *const POINT);
    fn setNodeRetrieved(planetInfo: *mut PLANET_INFO, scanType: BYTE, nodeNr: BYTE);
    fn setNodeNotRetrieved(planetInfo: *mut PLANET_INFO, scanType: BYTE, nodeNr: BYTE);
    fn GameClockTick();
    fn ClearSISRect(ClearFlags: BYTE);
    fn DrawLanders();
    fn DrawStorageBays(Refresh: BOOLEAN);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetCrewPodCapacity() -> COUNT;
    fn GetStorageBayCapacity() -> COUNT;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn TFB_Random() -> DWORD;
    fn RangeMinMax(
        min: ::core::ffi::c_int,
        max: ::core::ffi::c_int,
        rand: DWORD,
    ) -> ::core::ffi::c_int;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut PlanetContext: CONTEXT;
    fn DrawStarBackGround();
    fn DrawDefaultPlanetSphere();
    fn RotatePlanetSphere(keepRate: BOOLEAN, onTop: *mut STAMP);
    fn DrawPlanetSurfaceBorder();
    fn CustomMineralDeposit(
        info: *mut NODE_INFO,
        r#type: COUNT,
        quality: BYTE,
        location: POINT,
    ) -> COUNT;
    fn load_orbit_theme(selector: BYTE) -> MUSIC_REF;
    fn num_orbit_themes() -> ::core::ffi::c_int;
    fn InsertPrim(pLinks: *mut PRIM_LINKS, primIndex: COUNT, iPI: COUNT);
    static mut GameStrings: STRING;
    static mut Screen: FRAME;
    static mut MiscDataFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlaySound(
        S: SOUND,
        Pos: SoundPosition,
        PositionalObject: *mut ELEMENT,
        Priority: BYTE,
    );
    fn NotPositional() -> SoundPosition;
    static mut GlobData: GLOBDATA;
    static mut RadarContext: CONTEXT;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn DrawMineralHelpers();
    fn DrawRadarBorder();
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const OPTVAL_NO_GOD: Self = Self(0);
    pub const OPTVAL_INF_ENERGY: Self = Self(1);
    pub const OPTVAL_INF_HEALTH: Self = Self(2);
    pub const OPTVAL_FULL_GOD: Self = Self(3);
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
    pub image: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_8,
    pub c2rust_unnamed_0: C2Rust_Unnamed_7,
    pub c2rust_unnamed_1: C2Rust_Unnamed_6,
    pub c2rust_unnamed_2: C2Rust_Unnamed_5,
    pub c2rust_unnamed_3: C2Rust_Unnamed_4,
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
pub union C2Rust_Unnamed_4 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
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
pub struct NODE_DATA {
    pub NumNodesGrabbed: BYTE,
    pub NodeType: [BYTE; 32],
    pub NodeAmounts: [COUNT; 32],
}
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
    pub NodeData: NODE_DATA,
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
pub struct LIFEFORM_DESC {
    pub Attributes: BYTE,
    pub ValueAndHitPoints: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LanderInputState {
    pub InputFunc: Option<unsafe extern "C" fn(*mut LanderInputState) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub NextTime: TimeCount,
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
    pub const PS_HUMAN_PLAYER: Self = Self(0);
    pub const PS_NON_PLAYER: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
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
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
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
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_13 {
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
unsafe extern "C" fn MAKE_EXTENT(mut width: COORD, mut height: COORD) -> EXTENT {
    let mut ext: EXTENT = extent {
        width: width,
        height: height,
    };
    return ext;
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
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
#[no_mangle]
pub static mut LanderUpgradesFlag: BYTE = 0;
static mut LanderSounds: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut LanderMusic: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
static mut PCLanderContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
static mut OrbitMusic: [MUSIC_REF; 6] = [::core::ptr::null_mut::<
    *mut TFB_SoundSample,
>(); 6];
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
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
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
#[no_mangle]
pub static mut planetSideDesc: *mut PLANETSIDE_DESC = ::core::ptr::null_mut::<
    PLANETSIDE_DESC,
>();
#[no_mangle]
pub static mut MapSurface: EXTENT = EXTENT { width: 0, height: 0 };
#[no_mangle]
pub unsafe extern "C" fn CreatePCLanderContext() -> CONTEXT {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut context: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    oldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut r);
    context = CreateContextAux(
        b"PCLanderContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    SetContext(context);
    SetContextFGFrame(Screen);
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int
            - ((243 as ::core::ffi::c_int) << resolutionFactor)
            + ((if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)))) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (r.extent.height as ::core::ffi::c_int
            - ((67 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.extent.width = (((243 as ::core::ffi::c_int - 210 as ::core::ffi::c_int)
        << resolutionFactor)
        - (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            })
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    r.extent.height = ((67 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextClipRect(&raw mut r);
    SetContext(oldContext);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn GetPCLanderContext(mut owner: *mut BOOLEAN) -> CONTEXT {
    if !PCLanderContext.is_null() {
        if !owner.is_null() {
            *owner = BOOLEAN::FALSE;
        }
    } else {
        if !owner.is_null() {
            *owner = BOOLEAN::TRUE;
        }
        PCLanderContext = CreatePCLanderContext();
    }
    return PCLanderContext;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyPCLanderContext() {
    if !PCLanderContext.is_null() {
        DestroyContext(PCLanderContext);
        PCLanderContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    }
}
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
    } else if sameColor(c, damage_tab[0usize]) {
        c = damage_tab[6usize];
    } else if sameColor(c, damage_tab[6usize]) {
        c = damage_tab[5usize];
    } else if sameColor(c, damage_tab[5usize]) {
        c = damage_tab[4usize];
    } else if sameColor(c, damage_tab[4usize]) {
        c = damage_tab[3usize];
    } else if sameColor(c, damage_tab[3usize]) {
        c = damage_tab[2usize];
    } else if sameColor(c, damage_tab[2usize]) {
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
                == C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int)
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
                uqm_lua_checked!(PlaySound(
                    SetAbsStringTableIndex(
                        LanderSounds,
                        C2Rust_Unnamed_11::LIFEFORM_CANNED.0 as ::core::ffi::c_int
                            as COUNT,
                    ),
                    NotPositional(),
                    ::core::ptr::null_mut::<ELEMENT>(),
                    2 as BYTE,
                ), ());
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
                == C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
            {
                let mut frame_amount: SIZE = 13 as SIZE;
                (*pPrim).color = colorFromRgb15(
                    0x1f as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
                if frame_index as ::core::ffi::c_int
                    == frame_amount as ::core::ffi::c_int
                {
                    uqm_lua_checked!(PlaySound(
                        SetAbsStringTableIndex(
                            LanderSounds,
                            C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                        NotPositional(),
                        ::core::ptr::null_mut::<ELEMENT>(),
                        2 as BYTE,
                    ), ());
                }
            }
            if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int
                && frame_index as ::core::ffi::c_int == 5 as ::core::ffi::c_int
                && TFB_Random().wrapping_rem(100 as DWORD) < 90 as DWORD
            {
                let mut hLavaElement: HELEMENT = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                hLavaElement = AddGroundDisaster(
                    C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as COUNT,
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
                        + (*(&raw mut sinetab as *mut SDWORD)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 2 as ::core::ffi::c_int)
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD as isize,
                            ) as ::core::ffi::c_long
                            * ((4 as ::core::ffi::c_int) << resolutionFactor)
                                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int)
                            as SDWORD as ::core::ffi::c_int) as COORD;
                    (*LavaElementPtr).next.location.y = ((*LavaElementPtr)
                        .next
                        .location
                        .y as ::core::ffi::c_int
                        + (*(&raw mut sinetab as *mut SDWORD)
                            .offset(
                                (angle as ::core::ffi::c_int
                                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                        - 1 as ::core::ffi::c_int) as DWORD as isize,
                            ) as ::core::ffi::c_long
                            * ((4 as ::core::ffi::c_int) << resolutionFactor)
                                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int)
                            as SDWORD as ::core::ffi::c_int) as COORD;
                    if ((*LavaElementPtr).next.location.y as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*LavaElementPtr).next.location.y = 0 as COORD;
                    } else if (*LavaElementPtr).next.location.y as ::core::ffi::c_int
                        >= (67 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int
                    {
                        (*LavaElementPtr).next.location.y = (((67 as ::core::ffi::c_int)
                            << resolutionFactor << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COORD;
                    }
                    if ((*LavaElementPtr).next.location.x as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*LavaElementPtr).next.location.x = ((*LavaElementPtr)
                            .next
                            .location
                            .x as ::core::ffi::c_int
                            + (if is3DO(optSuperPC).0 != 0 {
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
                                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
                    } else {
                        (*LavaElementPtr).next.location.x = ((*LavaElementPtr)
                            .next
                            .location
                            .x as ::core::ffi::c_int
                            % (if is3DO(optSuperPC).0 != 0 {
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
                                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
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
                        < -(if is3DO(optSuperPC).0 != 0 {
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
                            << 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    dx = (dx as ::core::ffi::c_int
                        + (if is3DO(optSuperPC).0 != 0 {
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
                            } << resolutionFactor << 2 as ::core::ffi::c_int)) as SIZE;
                } else if dx as ::core::ffi::c_int
                    > if is3DO(optSuperPC).0 != 0 {
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
                        << 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    dx = (dx as ::core::ffi::c_int
                        - (if is3DO(optSuperPC).0 != 0 {
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
                            } << resolutionFactor << 2 as ::core::ffi::c_int)) as SIZE;
                }
                dy = (curLanderLoc.y as ::core::ffi::c_int
                    - (*ElementPtr).next.location.y as ::core::ffi::c_int) as SIZE;
                angle = ARCTAN(dx as SDWORD, dy as SDWORD);
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if dx as ::core::ffi::c_int >= MapSurface.width as ::core::ffi::c_int
                    || dy as ::core::ffi::c_int >= MapSurface.width as ::core::ffi::c_int
                    || dx as ::core::ffi::c_int * dx as ::core::ffi::c_int
                        + dy as ::core::ffi::c_int * dy as ::core::ffi::c_int
                        >= MapSurface.width as ::core::ffi::c_int
                            * MapSurface.width as ::core::ffi::c_int
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
                        == ((67 as ::core::ffi::c_int) << resolutionFactor
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
                            - 1 as ::core::ffi::c_int) as DWORD as COUNT;
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
                            == ((67 as ::core::ffi::c_int) << resolutionFactor
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
                            - 1 as ::core::ffi::c_int) as DWORD as COUNT;
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
                speed = ((speed as ::core::ffi::c_int) << resolutionFactor) as COUNT;
                SetVelocityComponents(
                    &raw mut (*ElementPtr).velocity,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
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
        shieldHit = uqm_lua_checked!(getGameStateUint(
            b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as ::core::ffi::c_int;
        shieldHit &= (1 as ::core::ffi::c_int) << which_disaster as ::core::ffi::c_int;
        if shieldHit == 0 || TFB_Random().wrapping_rem(100 as DWORD) >= 95 as DWORD {
            if optGodModes < C2Rust_Unnamed_2::OPTVAL_INF_HEALTH.0 as ::core::ffi::c_int
            {
                shieldHit = 0 as ::core::ffi::c_int;
                crew_left -= 1;
            } else {
                shieldHit = 1 as ::core::ffi::c_int;
            }
        }
        damage_index = 6 as ::core::ffi::c_int;
        if shieldHit != 0 {
            return;
        }
        crew_delta = crew_left as SIZE;
        s.frame = SetAbsFrameIndex(LanderFrame[0usize], 56 as COUNT);
        uqm_lua_checked!(PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_11::LANDER_INJURED.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        ), ());
    }
    if is3DO(optSuperPC).0 != 0 {
        s.origin.x = (((11 as ::core::ffi::c_int) << resolutionFactor)
            + ((6 as ::core::ffi::c_int) << resolutionFactor)
                * (crew_delta as ::core::ffi::c_int
                    % if is3DO(optSuperPC).0 != 0 {
                        6 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    })) as COORD;
        s.origin.y = (((35 as ::core::ffi::c_int) << resolutionFactor)
            - ((6 as ::core::ffi::c_int) << resolutionFactor)
                * (crew_delta as ::core::ffi::c_int
                    / if is3DO(optSuperPC).0 != 0 {
                        6 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    })) as COORD;
        OldContext = SetContext(RadarContext);
    } else {
        s.origin.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + ((6 as ::core::ffi::c_int) << resolutionFactor)
                * (crew_delta as ::core::ffi::c_int
                    % if is3DO(optSuperPC).0 != 0 {
                        6 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    })) as COORD;
        s.origin.y = (((39 as ::core::ffi::c_int) << resolutionFactor)
            - ((6 as ::core::ffi::c_int) << resolutionFactor)
                * (crew_delta as ::core::ffi::c_int
                    / if is3DO(optSuperPC).0 != 0 {
                        6 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    })) as COORD;
        OldContext = SetContext(PCLanderContext);
    }
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    SetContext(OldContext);
}
unsafe extern "C" fn FillLanderHold(
    mut pPSD: *mut PLANETSIDE_DESC,
    mut scan: COUNT,
    mut NumRetrieved: COUNT,
) {
    let mut start_count: COUNT = 0;
    let mut tmpholdint: COUNT = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    uqm_lua_checked!(PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_11::LANDER_PICKUP.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        2 as BYTE,
    ), ());
    if scan as ::core::ffi::c_int
        == PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
    {
        start_count = (*pPSD).BiologicalLevel;
        s.frame = SetAbsFrameIndex(
            if isPC(optSuperPC).0 != 0 {
                LanderFrame[7usize]
            } else {
                LanderFrame[0usize]
            },
            41 as COUNT,
        );
        (*pPSD).BiologicalLevel = ((*pPSD).BiologicalLevel as ::core::ffi::c_int
            + NumRetrieved as ::core::ffi::c_int) as COUNT;
    } else {
        start_count = (*pPSD).ElementLevel;
        (*pPSD).ElementLevel = ((*pPSD).ElementLevel as ::core::ffi::c_int
            + NumRetrieved as ::core::ffi::c_int) as COUNT;
        if uqm_lua_checked!(getGameStateUint(
            b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            start_count = (start_count as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as COUNT;
            NumRetrieved = (((*pPSD).ElementLevel as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) - start_count as ::core::ffi::c_int)
                as COUNT;
        }
        s.frame = SetAbsFrameIndex(
            if isPC(optSuperPC).0 != 0 {
                LanderFrame[7usize]
            } else {
                LanderFrame[0usize]
            },
            43 as COUNT,
        );
    }
    tmpholdint = ((start_count as ::core::ffi::c_int
        + NumRetrieved as ::core::ffi::c_int)
        * if isPC(optSuperPC).0 != 0 {
            64 as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }
        / if isPC(optSuperPC).0 != 0 {
            64 as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }
        - (start_count as ::core::ffi::c_int
            * if isPC(optSuperPC).0 != 0 {
                64 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            }
            / if isPC(optSuperPC).0 != 0 {
                64 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            }
            + NumRetrieved as ::core::ffi::c_int
                * if isPC(optSuperPC).0 != 0 {
                    64 as ::core::ffi::c_int
                } else {
                    50 as ::core::ffi::c_int
                }
                / if isPC(optSuperPC).0 != 0 {
                    64 as ::core::ffi::c_int
                } else {
                    50 as ::core::ffi::c_int
                })) as COUNT;
    start_count = (start_count as ::core::ffi::c_int
        * if isPC(optSuperPC).0 != 0 {
            64 as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }
        / if isPC(optSuperPC).0 != 0 {
            64 as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }) as COUNT;
    NumRetrieved = (NumRetrieved as ::core::ffi::c_int
        * if isPC(optSuperPC).0 != 0 {
            64 as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }
        / if isPC(optSuperPC).0 != 0 {
            64 as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        } + tmpholdint as ::core::ffi::c_int) as COUNT;
    s.origin.x = 0 as COORD;
    s.origin.y = -((start_count as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if start_count as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
        s.frame = IncFrameIndex(s.frame);
    }
    if isPC(optSuperPC).0 != 0 {
        OldContext = SetContext(PCLanderContext);
    } else {
        OldContext = SetContext(RadarContext);
    }
    loop {
        let c2rust_fresh1 = NumRetrieved;
        NumRetrieved = NumRetrieved.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        let c2rust_fresh2 = start_count;
        start_count = start_count.wrapping_add(1);
        if c2rust_fresh2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            s.frame = IncFrameIndex(s.frame);
        } else {
            s.frame = DecFrameIndex(s.frame);
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    SetContext(OldContext);
}
unsafe extern "C" fn pickupNode(
    mut pPSD: *mut PLANETSIDE_DESC,
    mut NumRetrieved: COUNT,
    mut ElementPtr: *mut ELEMENT,
    mut LanderControl: *const INTERSECT_CONTROL,
    mut ElementControl: *const INTERSECT_CONTROL,
    mut Scan: COUNT,
) -> bool {
    let mut EType: BYTE = 0;
    let mut ch: UNICODE = 0;
    let mut pStr: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut Amount: *mut COUNT = ::core::ptr::null_mut::<COUNT>();
    let mut Max: COUNT = 0;
    let mut Offset: COUNT = 0;
    let mut PartialPickup: BOOLEAN = BOOLEAN::FALSE;
    Amount = &raw mut (*pPSD).BiologicalLevel;
    Max = (if isPC(optSuperPC).0 != 0 {
        64 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int
    }) as COUNT;
    EType = (*ElementPtr).c2rust_unnamed_3.thrust_wait;
    Offset = C2Rust_Unnamed_13::BIOLOGICAL_STRING_BASE.0 as ::core::ffi::c_int as COUNT;
    if Scan as ::core::ffi::c_int
        != PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
    {
        Amount = &raw mut (*pPSD).ElementLevel;
        Max = (*pPSD).MaxElementLevel;
        EType = (*ElementPtr).c2rust_unnamed_2.turn_wait;
        Offset = C2Rust_Unnamed_13::ELEMENTS_STRING_BASE.0 as ::core::ffi::c_int
            as COUNT;
    }
    PartialPickup = BOOLEAN::FALSE;
    if *Amount as ::core::ffi::c_int >= Max as ::core::ffi::c_int {
        uqm_lua_checked!(PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_11::LANDER_FULL.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        ), false);
        return false;
    }
    if *Amount as ::core::ffi::c_int + NumRetrieved as ::core::ffi::c_int
        > Max as ::core::ffi::c_int
    {
        let mut which_node: SIZE = 0;
        let mut oldsize: COUNT = (*ElementPtr).c2rust_unnamed_1.mass_points as COUNT;
        NumRetrieved = (Max as ::core::ffi::c_int - *Amount as ::core::ffi::c_int)
            as COUNT;
        if Scan as ::core::ffi::c_int
            != PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
            && optPartialPickup.0 != 0
        {
            (*ElementPtr).c2rust_unnamed_1.mass_points = ((*ElementPtr)
                .c2rust_unnamed_1
                .mass_points as ::core::ffi::c_int - NumRetrieved as ::core::ffi::c_int)
                as BYTE;
            which_node = (((*ElementPtr).c2rust_unnamed.scan_node as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as SIZE;
            (*pSolarSysState)
                .SysInfo
                .PlanetInfo
                .PartiallyScavengedList[PlanetScanTypes::MINERAL_SCAN.0
                as ::core::ffi::c_int as usize][which_node as usize] = NumRetrieved
                as BYTE;
            if oldsize as ::core::ffi::c_int > 22 as ::core::ffi::c_int
                && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                    <= 22 as ::core::ffi::c_int
                || oldsize as ::core::ffi::c_int > 15 as ::core::ffi::c_int
                    && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                        <= 15 as ::core::ffi::c_int
            {
                let mut pPrim: *mut PRIMITIVE = (&raw mut DisplayArray as *mut PRIMITIVE)
                    .offset((*ElementPtr).PrimIndex as isize);
                let mut gfx_index_change: BYTE = 0 as BYTE;
                if oldsize as ::core::ffi::c_int > 22 as ::core::ffi::c_int
                    && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                        <= 15 as ::core::ffi::c_int
                {
                    gfx_index_change = 2 as BYTE;
                } else {
                    gfx_index_change = 1 as BYTE;
                }
                (*ElementPtr).current.image.frame = SetRelFrameIndex(
                    (*ElementPtr).current.image.frame,
                    (2 as ::core::ffi::c_int - gfx_index_change as ::core::ffi::c_int)
                        as SIZE,
                );
                (*ElementPtr).next.image.frame = (*ElementPtr).current.image.frame;
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                uqm_lua_checked!(setGameStateUint(
                    b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), false);
                (*pPrim).Object.Stamp.frame = SetRelFrameIndex(
                    (*pPrim).Object.Stamp.frame,
                    -(gfx_index_change as ::core::ffi::c_int) as SIZE,
                );
            }
            PartialPickup = BOOLEAN::TRUE;
        }
        if Scan as ::core::ffi::c_int
            == PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
            && optPartialPickup.0 != 0
        {
            uqm_lua_checked!(PlaySound(
                SetAbsStringTableIndex(
                    LanderSounds,
                    C2Rust_Unnamed_11::LANDER_FULL.0 as ::core::ffi::c_int as COUNT,
                ),
                NotPositional(),
                ::core::ptr::null_mut::<ELEMENT>(),
                2 as BYTE,
            ), false);
            return false;
        }
    }
    uqm_lua_checked!(FillLanderHold(pPSD, Scan, NumRetrieved), false);
    if Scan as ::core::ffi::c_int
        != PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
    {
        let mut NumNodesGrabbed: BYTE = (*pPSD).NodeData.NumNodesGrabbed;
        (*pPSD)
            .ElementAmounts[(*Elements.offset(EType as isize) as ::core::ffi::c_int
            & 0x7 as ::core::ffi::c_int) as usize] = ((*pPSD)
            .ElementAmounts[(*Elements.offset(EType as isize) as ::core::ffi::c_int
            & 0x7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            + NumRetrieved as ::core::ffi::c_int) as COUNT;
        (*pPSD).NodeData.NodeType[NumNodesGrabbed as usize] = EType;
        (*pPSD).NodeData.NodeAmounts[NumNodesGrabbed as usize] = NumRetrieved;
        (*pPSD).NodeData.NumNodesGrabbed = (*pPSD)
            .NodeData
            .NumNodesGrabbed
            .wrapping_add(1);
    }
    (*pPSD).NumFrames = 32 as COUNT;
    sprintf(
        &raw mut (*pPSD).AmountBuf as *mut ::core::ffi::c_char,
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        NumRetrieved as ::core::ffi::c_int,
    );
    pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (EType as ::core::ffi::c_int + Offset as ::core::ffi::c_int) as COUNT,
        ),
    );
    (*pPSD).MineralText[0usize].baseline.x = ((MapSurface.width as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        + ((*ElementControl).EndPoint.x as ::core::ffi::c_int
            - (*LanderControl).EndPoint.x as ::core::ffi::c_int)) as COORD;
    (*pPSD).MineralText[0usize].baseline.y = ((MapSurface.height as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        + ((*ElementControl).EndPoint.y as ::core::ffi::c_int
            - (*LanderControl).EndPoint.y as ::core::ffi::c_int)) as COORD;
    (*pPSD).MineralText[0usize].CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    (*pPSD).MineralText[1usize].pStr = pStr;
    loop {
        let c2rust_fresh0 = pStr;
        pStr = pStr.offset(1);
        ch = *c2rust_fresh0;
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
    return if PartialPickup.0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    } != 0;
}
unsafe extern "C" fn ExplodeCritter(mut ElementPtr: *mut ELEMENT) {
    let mut hExplosionElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut temp_which_node: SIZE = 0;
    hExplosionElement = AllocElement();
    if !hExplosionElement.is_null() {
        let mut ExplosionElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ExplosionElementPtr = LockLink(&raw mut disp_q, hExplosionElement)
            as *mut ELEMENT;
        (*ExplosionElementPtr).c2rust_unnamed_1.mass_points = 0 as BYTE;
        (*ExplosionElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ExplosionElementPtr).playerNr = C2Rust_Unnamed_9::PS_NON_PLAYER.0
            as ::core::ffi::c_int as SIZE;
        (*ExplosionElementPtr).next.location = (*ElementPtr).next.location;
        (*ExplosionElementPtr).preprocess_func = Some(
            object_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*ExplosionElementPtr).c2rust_unnamed_2.turn_wait = ((2 as ::core::ffi::c_int
            as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 2 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*ExplosionElementPtr).c2rust_unnamed.life_span = (10 as ::core::ffi::c_int
            * ((*ExplosionElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as COUNT;
        DisplayArray[(*ExplosionElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        DisplayArray[(*ExplosionElementPtr).PrimIndex as usize].Object.Stamp.frame = SetAbsFrameIndex(
            LanderFrame[0usize],
            46 as COUNT,
        );
        explosion_index = 0 as ::core::ffi::c_int;
        UnlockLink(&raw mut disp_q, hExplosionElement);
        InsertQueue(&raw mut disp_q, hExplosionElement, disp_q.head);
        uqm_lua_checked!(PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_11::LANDER_DESTROYED.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
        ), ());
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_1.mass_points = 0 as BYTE;
        temp_which_node = (((*ElementPtr).c2rust_unnamed.scan_node as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as SIZE;
        (*pSolarSysState)
            .SysInfo
            .PlanetInfo
            .ScanRetrieveMask[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
            as usize] = ((*pSolarSysState)
            .SysInfo
            .PlanetInfo
            .ScanRetrieveMask[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
            as usize] as ::core::ffi::c_ulong
            | ((1 as ::core::ffi::c_long) << temp_which_node as ::core::ffi::c_int)
                as ::core::ffi::c_ulong) as DWORD;
        uqm_lua_checked!(setGameStateUint(
            b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    }
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
        if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) == 24 as ::core::ffi::c_int
        {
            uqm_lua_checked!(ExplodeCritter(ElementPtr), ());
        } else {
            (*ElementPtr).c2rust_unnamed_3.thrust_wait = ((*ElementPtr)
                .c2rust_unnamed_1
                .mass_points as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE;
            (*ElementPtr).c2rust_unnamed_1.mass_points = value;
            DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Stamp.frame = (*pSolarSysState)
                .PlanetSideFrame[0usize];
        }
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
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * ((1 as ::core::ffi::c_int) << resolutionFactor
                    << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD,
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * ((1 as ::core::ffi::c_int) << resolutionFactor
                    << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD,
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
    uqm_lua_checked!(PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_11::LANDER_HITS.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        2 as BYTE,
    ), ());
}
unsafe extern "C" fn DrawRadarArea() {
    uqm_lua_checked!(DrawRadarBorder(), ());
    uqm_lua_checked!(DrawMineralHelpers(), ());
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
        LanderControl.IntersectStamp.origin.x = (MapSurface.width as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        LanderControl.IntersectStamp.origin.y = (MapSurface.height as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
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
            uqm_lua_checked!(CheckObjectCollision(index), ());
        } else if uqm_lua_checked!(DrawablesIntersect(
            &raw mut LanderControl,
            &raw mut ElementControl,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ), ()) != 0
        {
            hElement = disp_q.head as HELEMENT;
            's_326: while !hElement.is_null() {
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
                            != C2Rust_Unnamed_9::PS_NON_PLAYER.0 as ::core::ffi::c_int
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
                                break 's_326;
                            }
                            if (*ElementPtr).state_flags as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                            {
                                scan = (*ElementPtr).c2rust_unnamed_1.mass_points as COUNT;
                                match scan as ::core::ffi::c_int {
                                    1 | 3 => {
                                        if TFB_Random().wrapping_rem(100 as DWORD) < 25 as DWORD {
                                            uqm_lua_checked!(DeltaLanderCrew(-1 as SIZE, scan), ());
                                        }
                                    }
                                    _ => {}
                                }
                                UnlockLink(&raw mut disp_q, hElement);
                                break 's_92;
                            } else {
                                if scan as ::core::ffi::c_int
                                    == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
                                {
                                    uqm_lua_checked!(DrawRadarArea(), ());
                                } else if scan as ::core::ffi::c_int
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
                                        uqm_lua_checked!(PlaySound(
                                            SetAbsStringTableIndex(
                                                LanderSounds,
                                                C2Rust_Unnamed_11::BIOLOGICAL_DISASTER.0
                                                    as ::core::ffi::c_int as COUNT,
                                            ),
                                            NotPositional(),
                                            ::core::ptr::null_mut::<ELEMENT>(),
                                            2 as BYTE,
                                        ), ());
                                        uqm_lua_checked!(DeltaLanderCrew(
                                            -1 as SIZE,
                                            C2Rust_Unnamed_11::BIOLOGICAL_DISASTER.0
                                                as ::core::ffi::c_int as COUNT,
                                        ), ());
                                    }
                                    UnlockLink(&raw mut disp_q, hElement);
                                    break 's_92;
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
                                break 's_326;
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
                                uqm_lua_checked!(shotCreature(
                                    ElementPtr,
                                    value,
                                    &raw mut LanderControl,
                                    pPrim,
                                ), ());
                                UnlockLink(&raw mut disp_q, hElement);
                                break 's_326;
                            } else {
                                NumRetrieved = 0 as COUNT;
                            }
                        }
                        if NumRetrieved != 0 {
                            match scan as ::core::ffi::c_int {
                                0 | 2 => {
                                    if !uqm_lua_checked!(pickupNode(
                                        pPSD,
                                        NumRetrieved,
                                        ElementPtr,
                                        &raw mut LanderControl,
                                        &raw mut ElementControl,
                                        scan,
                                    ), ()) {
                                        break 's_92;
                                    }
                                }
                                1 | _ => {}
                            }
                        }
                        which_node = (((*ElementPtr).c2rust_unnamed.scan_node
                            as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
                        if uqm_lua_checked!(callPickupForScanType(
                            pSolarSysState,
                            (*pSolarSysState).pOrbitalDesc,
                            which_node as COUNT,
                            scan as BYTE,
                        ), ()) {
                            setNodeRetrieved(
                                &raw mut (*pSolarSysState).SysInfo.PlanetInfo,
                                scan as BYTE,
                                which_node as BYTE,
                            );
                            uqm_lua_checked!(setGameStateUint(
                                b"PLANETARY_CHANGE\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1 as DWORD,
                            ), ());
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
            static mut color_tab: [Color; 8] = [
                Color {
                    r: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x1d as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1d as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x1d as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1d as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x1d as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1d as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            ];
            s = (*ElementPtr).c2rust_unnamed.life_span as SIZE;
            if s as ::core::ffi::c_int
                > 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                s = (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
            }
            (*pPrim).color = color_tab[s as usize];
            if (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                == C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int
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
        (*LightningElementPtr).playerNr = C2Rust_Unnamed_9::PS_NON_PLAYER.0
            as ::core::ffi::c_int as SIZE;
        (*LightningElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*LightningElementPtr).preprocess_func = Some(
            lightning_process as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        if TFB_Random().wrapping_rem(100 as DWORD) >= 25 as DWORD {
            (*LightningElementPtr).c2rust_unnamed_1.mass_points = 0 as BYTE;
        } else {
            (*LightningElementPtr).c2rust_unnamed_1.mass_points = C2Rust_Unnamed_11::LIGHTNING_DISASTER
                .0 as ::core::ffi::c_int as BYTE;
        }
        rand_val = TFB_Random();
        (*LightningElementPtr).c2rust_unnamed.life_span = (10 as ::core::ffi::c_int
            + (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                % 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
        (*LightningElementPtr).next.location.x = ((curLanderLoc.x as ::core::ffi::c_int
            + ((if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)
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
                        } << resolutionFactor)) >> 1 as ::core::ffi::c_int)
                    - 6 as ::core::ffi::c_int))) as DWORD)
            .wrapping_add(
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    rand_val as UWORD as BYTE as DWORD
                } else {
                    rand_val
                }
                    .wrapping_rem(
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
                                } << resolutionFactor))
                            - ((12 as ::core::ffi::c_int) << resolutionFactor)) as DWORD,
                    ),
            )
            .wrapping_rem(
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
                    } << resolutionFactor << 2 as ::core::ffi::c_int) as DWORD,
            ) as COORD;
        (*LightningElementPtr).next.location.y = ((curLanderLoc.y as ::core::ffi::c_int
            + (((67 as ::core::ffi::c_int) << resolutionFactor
                << 2 as ::core::ffi::c_int)
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
                    - ((13 as ::core::ffi::c_int) << resolutionFactor)
                    - ((67 as ::core::ffi::c_int) << resolutionFactor)
                    - ((5 as ::core::ffi::c_int) << resolutionFactor)
                    >> 1 as ::core::ffi::c_int) - 6 as ::core::ffi::c_int))) as DWORD)
            .wrapping_add(
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    (rand_val as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                        as BYTE as DWORD
                } else {
                    rand_val
                }
                    .wrapping_rem(
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
                            - ((13 as ::core::ffi::c_int) << resolutionFactor)
                            - ((67 as ::core::ffi::c_int) << resolutionFactor)
                            - ((5 as ::core::ffi::c_int) << resolutionFactor)
                            - ((12 as ::core::ffi::c_int) << resolutionFactor)) as DWORD,
                    ),
            )
            .wrapping_rem(
                ((67 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) as DWORD,
            ) as COORD;
        (*LightningElementPtr).c2rust_unnamed_0.cycle = (*LightningElementPtr)
            .c2rust_unnamed
            .life_span;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            DisplayArray[(*LightningElementPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            DisplayArray[(*LightningElementPtr).PrimIndex as usize].color = colorFromRgb15(
                0x1f as BYTE,
                0x1f as BYTE,
                0x1f as BYTE,
            );
        } else {
            DisplayArray[(*LightningElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        }
        DisplayArray[(*LightningElementPtr).PrimIndex as usize].Object.Stamp.frame = LanderFrame[2usize];
        UnlockLink(&raw mut disp_q, hLightningElement);
        PutQueue(&raw mut disp_q, hLightningElement);
        uqm_lua_checked!(PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        ), ());
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
        (*GroundDisasterElementPtr).playerNr = C2Rust_Unnamed_9::PS_NON_PLAYER.0
            as ::core::ffi::c_int as SIZE;
        (*GroundDisasterElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*GroundDisasterElementPtr).preprocess_func = Some(
            object_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        rand_val = TFB_Random();
        (*GroundDisasterElementPtr).next.location.x = ((curLanderLoc.x
            as ::core::ffi::c_int
            + ((if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)
                - (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor))) * 3 as ::core::ffi::c_int
                    / 8 as ::core::ffi::c_int)
            + rand_val as UWORD as ::core::ffi::c_int
                % ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor))) * 3 as ::core::ffi::c_int
                    / 4 as ::core::ffi::c_int))
            % (if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
        (*GroundDisasterElementPtr).next.location.y = ((curLanderLoc.y
            as ::core::ffi::c_int
            + (((67 as ::core::ffi::c_int) << resolutionFactor
                << 2 as ::core::ffi::c_int)
                - (CanvasHeight
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
                    - ((67 as ::core::ffi::c_int) << resolutionFactor)
                    - ((5 as ::core::ffi::c_int) << resolutionFactor))
                    * 3 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
            + (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                % ((CanvasHeight
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
                    - ((67 as ::core::ffi::c_int) << resolutionFactor)
                    - ((5 as ::core::ffi::c_int) << resolutionFactor))
                    * 3 as ::core::ffi::c_int / 4 as ::core::ffi::c_int))
            % ((67 as ::core::ffi::c_int) << resolutionFactor
                << 2 as ::core::ffi::c_int)) as COORD;
        if which_disaster as ::core::ffi::c_int
            == C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
        {
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                (*pPrim).Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
            } else {
                (*pPrim).Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
            }
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
            C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as COUNT,
        );
        uqm_lua_checked!(PlaySound(
            SetAbsStringTableIndex(
                LanderSounds,
                C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as COUNT,
            ),
            NotPositional(),
            ::core::ptr::null_mut::<ELEMENT>(),
            2 as BYTE,
        ), ());
    }
    if ((rand_val as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int) < (*pPSD).TectonicsChance as ::core::ffi::c_int
    {
        AddGroundDisaster(
            C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int as COUNT,
        );
    }
    if (rand_val as UWORD as BYTE as ::core::ffi::c_int)
        < (*pPSD).WeatherChance as ::core::ffi::c_int
    {
        uqm_lua_checked!(AddLightning(), ());
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
                uqm_lua_checked!(Some((*ElementPtr).preprocess_func.expect("non-null function pointer"))
                    .expect("non-null function pointer")(ElementPtr), ());
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
                    != C2Rust_Unnamed_9::PS_HUMAN_PLAYER.0 as ::core::ffi::c_int
                {
                    if ((*ElementPtr).next.location.y as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.y = 0 as COORD;
                    } else if (*ElementPtr).next.location.y as ::core::ffi::c_int
                        >= (67 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.y = (((67 as ::core::ffi::c_int)
                            << resolutionFactor << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COORD;
                    }
                }
                if ((*ElementPtr).next.location.x as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                        as ::core::ffi::c_int
                        + (if is3DO(optSuperPC).0 != 0 {
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
                            } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
                } else {
                    (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                        as ::core::ffi::c_int
                        % (if is3DO(optSuperPC).0 != 0 {
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
                            } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
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
                + (MapSurface.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            if (*pPrim).Object.Stamp.origin.x as ::core::ffi::c_int
                >= (if is3DO(optSuperPC).0 != 0 {
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
                    } << resolutionFactor << 2 as ::core::ffi::c_int)
                    - MapSurface.width as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int
            {
                (*pPrim).Object.Stamp.origin.x = ((*pPrim).Object.Stamp.origin.x
                    as ::core::ffi::c_int
                    - (if is3DO(optSuperPC).0 != 0 {
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
                        } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
            } else if (*pPrim).Object.Stamp.origin.x as ::core::ffi::c_int
                <= -((if is3DO(optSuperPC).0 != 0 {
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
                    } << resolutionFactor << 2 as ::core::ffi::c_int)
                    - MapSurface.width as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int)
            {
                (*pPrim).Object.Stamp.origin.x = ((*pPrim).Object.Stamp.origin.x
                    as ::core::ffi::c_int
                    + (if is3DO(optSuperPC).0 != 0 {
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
                        } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
            }
            (*pPrim).Object.Stamp.origin.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int - org.y as ::core::ffi::c_int
                + (MapSurface.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
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
        > ((67 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int
    {
        new_pt.y = (((67 as ::core::ffi::c_int) << resolutionFactor
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
            + (if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
    } else if new_pt.x as ::core::ffi::c_int
        >= if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor << 2 as ::core::ffi::c_int
    {
        new_pt.x = (new_pt.x as ::core::ffi::c_int
            - (if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
    }
    curLanderLoc = new_pt;
    if is3DO(optSuperPC).0 != 0 {
        OldContext = SetContext(PlanetContext);
    } else {
        OldContext = SetContext(RadarContext);
    }
    BatchGraphics();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    uqm_lua_checked!(ClearDrawable(), ());
    s.origin.x = (-(new_pt.x as ::core::ffi::c_int)
        + (MapSurface.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.origin.y = (-(new_pt.y as ::core::ffi::c_int)
        + (MapSurface.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.frame = (*pSolarSysState).Orbit.TopoZoomFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        + (if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        - (if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor << 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
        as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    uqm_lua_checked!(BuildObjectList(), ());
    uqm_lua_checked!(DrawBatch(&raw mut DisplayArray as *mut PRIMITIVE, DisplayLinks, 0 as BATCH_FLAGS), ());
    if crew_left != 0 || damage_index != 0 || explosion_index < 3 as ::core::ffi::c_int {
        lander_s.origin.x = (MapSurface.width as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        lander_s.origin.y = ((MapSurface.height as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) + landingOffset) as COORD;
        lander_s.frame = LanderFrame[0usize];
        if landingOffset != 0 as ::core::ffi::c_int {
            shadow_s.origin.x = (lander_s.origin.y as ::core::ffi::c_int
                + (MapSurface.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - (MapSurface.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            shadow_s.origin.y = lander_s.origin.y;
            shadow_s.frame = lander_s.frame;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            uqm_lua_checked!(DrawFilledStamp(&raw mut shadow_s), ());
        }
        if damage_index == 0 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawStamp(&raw mut lander_s), ());
        } else if shieldHit != 0 {
            damage_index -= 1;
            if damage_index > 0 as ::core::ffi::c_int {
                shield_s.origin = lander_s.origin;
                shield_s.frame = SetEquFrameIndex(LanderFrame[4usize], lander_s.frame);
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                );
                uqm_lua_checked!(DrawFilledStamp(&raw mut shield_s), ());
            }
            uqm_lua_checked!(DrawStamp(&raw mut lander_s), ());
        } else {
            damage_index -= 1;
            SetContextForeGroundColor(
                DamageColorCycle(
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    damage_index as COUNT,
                ),
            );
            uqm_lua_checked!(DrawFilledStamp(&raw mut lander_s), ());
        }
    }
    if landingOffset == 0 as ::core::ffi::c_int && crew_left != 0
        && DisplayLinks as UWORD as ::core::ffi::c_int
            != 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(CheckObjectCollision(0xffff as ::core::ffi::c_int as COUNT), ());
    }
    let mut pPSD: *mut PLANETSIDE_DESC = planetSideDesc;
    if (*pPSD).NumFrames != 0 {
        (*pPSD).NumFrames = (*pPSD).NumFrames.wrapping_sub(1);
        SetContextForeGroundColor(
            (*pPSD)
                .ColorCycle[((*pPSD).NumFrames as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as usize],
        );
        if isPC(optSuperPC).0 != 0 {
            (*pPSD).MineralText[0usize].baseline.x = (((64 as ::core::ffi::c_int)
                << resolutionFactor) - ((8 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int) as COORD;
            (*pPSD).MineralText[0usize].baseline.y = ((8 as ::core::ffi::c_int)
                << resolutionFactor) as COORD;
        } else {
            (*pPSD).MineralText[0usize].baseline.x = ((*pPSD)
                .MineralText[0usize]
                .baseline
                .x as ::core::ffi::c_int - dx as ::core::ffi::c_int) as COORD;
            (*pPSD).MineralText[0usize].baseline.y = ((*pPSD)
                .MineralText[0usize]
                .baseline
                .y as ::core::ffi::c_int - dy as ::core::ffi::c_int) as COORD;
        }
        uqm_lua_checked!(font_DrawText((&raw mut (*pPSD).MineralText as *mut TEXT).offset(0isize)), ());
        (*pPSD).MineralText[1usize].baseline.x = (*pPSD).MineralText[0usize].baseline.x;
        (*pPSD).MineralText[1usize].baseline.y = ((*pPSD).MineralText[0usize].baseline.y
            as ::core::ffi::c_int + ((7 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        uqm_lua_checked!(font_DrawText((&raw mut (*pPSD).MineralText as *mut TEXT).offset(1isize)), ());
        (*pPSD).MineralText[2usize].baseline.x = (*pPSD).MineralText[1usize].baseline.x;
        (*pPSD).MineralText[2usize].baseline.y = ((*pPSD).MineralText[1usize].baseline.y
            as ::core::ffi::c_int + ((7 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        uqm_lua_checked!(font_DrawText((&raw mut (*pPSD).MineralText as *mut TEXT).offset(2isize)), ());
    }
    uqm_lua_checked!(RedrawSurfaceScan(&raw mut new_pt), ());
    if lander_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DeltaLanderCrew(
            -1 as SIZE,
            C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int as COUNT,
        ), ());
    }
    if isPC(optSuperPC).0 != 0 {
        uqm_lua_checked!(DrawRadarBorder(), ());
        uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn animationInterframe(
    mut TimeIn: *mut TimeCount,
    mut periods: COUNT,
) {
    while periods as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
        if GetTimeCounter()
            >= (*TimeIn)
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
                )
        {
            *TimeIn = GetTimeCounter();
            periods = periods.wrapping_sub(1);
        }
    }
}
unsafe extern "C" fn AnimateLaunch(mut farray: FRAME, mut isLanding: BOOLEAN) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut num_frames: COUNT = 0;
    static mut NextTime: TimeCount = 0;
    static mut psNextTime: TimeCount = 0;
    let mut Now: TimeCount = 0;
    SetContext(PlanetContext);
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = 0 as COORD;
    r.extent.height = 0 as COORD;
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = farray;
    num_frames = GetFrameCount(s.frame);
    NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 22 as ::core::ffi::c_int) as TimeCount,
        );
    psNextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int) as TimeCount,
        );
    while num_frames as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, &raw mut s), ());
        Now = GetTimeCounter();
        if isLanding.0 == 0 && isPC(optSuperPC).0 != 0 && Now >= psNextTime {
            uqm_lua_checked!(ScrollPlanetSide(
                0 as SIZE,
                0 as SIZE,
                -(MapSurface.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + ((10 as ::core::ffi::c_int) << resolutionFactor)),
            ), ());
            psNextTime = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int) as TimeCount,
                );
        }
        if Now < NextTime {
            continue;
        }
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 22 as ::core::ffi::c_int) as TimeCount,
            );
        num_frames = num_frames.wrapping_sub(1);
        GetFrameRect(s.frame, &raw mut r);
        s.frame = IncFrameIndex(s.frame);
        RepairBackRect(&raw mut r);
        uqm_lua_checked!(DrawDefaultPlanetSphere(), ());
        if num_frames as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
}
unsafe extern "C" fn AnimateLanderWarmup() {
    let mut num_crew: SIZE = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut TimeIn: TimeCount = GetTimeCounter();
    if is3DO(optSuperPC).0 != 0 {
        SetContext(RadarContext);
    } else {
        SetContext(PCLanderContext);
    }
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex(
        if is3DO(optSuperPC).0 != 0 { LanderFrame[0usize] } else { LanderFrame[7usize] },
        (((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COUNT,
    );
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 2 as COUNT), ());
    num_crew = 0 as SIZE;
    while (num_crew as ::core::ffi::c_int)
        < if is3DO(optSuperPC).0 != 0 {
            6 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        }
            * if is3DO(optSuperPC).0 != 0 {
                2 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            } && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 1 as COUNT), ());
        GlobData.SIS_state.CrewEnlisted = GlobData
            .SIS_state
            .CrewEnlisted
            .wrapping_sub(1);
        uqm_lua_checked!(DrawMineralHelpers(), ());
        if optSubmenu.0 == 0 {
            uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
        }
        uqm_lua_checked!(DeltaLanderCrew(1 as SIZE, 0 as COUNT), ());
        num_crew += 1;
    }
    uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 2 as COUNT), ());
    if uqm_lua_checked!(getGameStateUint(b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
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
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 2 as COUNT), ());
    if uqm_lua_checked!(getGameStateUint(
        b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
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
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 2 as COUNT), ());
        s.frame = IncFrameIndex(s.frame);
    }
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    if uqm_lua_checked!(getGameStateUint(
        b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 2 as COUNT), ());
        s.frame = SetAbsFrameIndex(s.frame, 59 as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    uqm_lua_checked!(animationInterframe(&raw mut TimeIn, 2 as COUNT), ());
    uqm_lua_checked!(PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_11::LANDER_DEPARTS.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    ), ());
}
unsafe extern "C" fn InitPlanetSide(mut pt: POINT) {
    pt.x = (pt.x as DWORD)
        .wrapping_sub(
            (((64 as ::core::ffi::c_int) << resolutionFactor) as DWORD)
                .wrapping_sub(
                    TFB_Random()
                        .wrapping_rem(
                            ((64 as ::core::ffi::c_int) << resolutionFactor
                                << 1 as ::core::ffi::c_int) as DWORD,
                        ),
                ),
        ) as COORD;
    if (pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        pt.x = (pt.x as ::core::ffi::c_int
            + (if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
    } else if pt.x as ::core::ffi::c_int
        >= if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor << 2 as ::core::ffi::c_int
    {
        pt.x = (pt.x as ::core::ffi::c_int
            - (if is3DO(optSuperPC).0 != 0 {
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
                } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
    }
    pt.y = (pt.y as DWORD)
        .wrapping_sub(
            (((64 as ::core::ffi::c_int) << resolutionFactor) as DWORD)
                .wrapping_sub(
                    TFB_Random()
                        .wrapping_rem(
                            ((64 as ::core::ffi::c_int) << resolutionFactor
                                << 1 as ::core::ffi::c_int) as DWORD,
                        ),
                ),
        ) as COORD;
    if (pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        pt.y = 0 as COORD;
    } else if pt.y as ::core::ffi::c_int
        >= (67 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int
    {
        pt.y = (((67 as ::core::ffi::c_int) << resolutionFactor
            << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as COORD;
    }
    if isPC(optSuperPC).0 != 0 {
        SetContext(RadarContext);
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE), ());
        MapSurface = MAKE_EXTENT(
            (((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
            ((53 as ::core::ffi::c_int) << resolutionFactor) as COORD,
        );
        uqm_lua_checked!(DrawMineralHelpers(), ());
    } else {
        SetContext(PlanetContext);
        MapSurface = MAKE_EXTENT(
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
                    } << resolutionFactor))) as COORD,
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor)
                - ((67 as ::core::ffi::c_int) << resolutionFactor)
                - ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
        );
    }
    curLanderLoc = pt;
    if isPC(optWhichFonts).0 != 0 {
        SetContextFont(TinyFont);
    } else {
        SetContextFont(TinyFontBold);
    }
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
    uqm_lua_checked!(ClearDrawable(), ());
    s.origin.x = (-(pt.x as ::core::ffi::c_int)
        + (MapSurface.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.origin.y = (-(pt.y as ::core::ffi::c_int)
        + (MapSurface.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.frame = (*pSolarSysState).Orbit.TopoZoomFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        + (if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    s.origin.x = (s.origin.x as ::core::ffi::c_int
        - (if is3DO(optSuperPC).0 != 0 {
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
            } << resolutionFactor << 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
        as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    uqm_lua_checked!(DrawRadarArea(), ());
    if isPC(optSuperPC).0 == 0 {
        uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), ());
    }
    UnbatchGraphics();
    uqm_lua_checked!(setGameStateUint(
        b"PLANETARY_LANDING\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
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
    (*WeaponElementPtr).playerNr = C2Rust_Unnamed_9::PS_HUMAN_PLAYER.0
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
        as usize][C2Rust_Unnamed_0::KEY_UP.0 as ::core::ffi::c_int as usize] != 0
        || CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_0::KEY_THRUST.0 as ::core::ffi::c_int as usize] != 0
    {
        GetCurrentVelocityComponents(
            &raw mut GlobData.Game_state.velocity,
            &raw mut wdx,
            &raw mut wdy,
        );
    } else {
        wdx = 0 as SIZE;
        wdy = 0 as SIZE;
    }
    angle = ((facing as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
    SetVelocityComponents(
        &raw mut (*WeaponElementPtr).velocity,
        (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) << resolutionFactor
                << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD + wdx as SDWORD,
        (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) << resolutionFactor
                << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD + wdy as SDWORD,
    );
    UnlockLink(&raw mut disp_q, hWeaponElement);
    InsertQueue(&raw mut disp_q, hWeaponElement, disp_q.head);
    uqm_lua_checked!(PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_11::LANDER_SHOOTS.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        2 as BYTE,
    ), ());
}
unsafe extern "C" fn put_node(mut ElementPtr: *mut ELEMENT) {
    let mut hNodeElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut NodeElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut info: NODE_INFO = NODE_INFO {
        loc_pt: POINT { x: 0, y: 0 },
        density: 0,
        r#type: 0,
    };
    hNodeElement = AllocElement();
    if hNodeElement.is_null() {
        return;
    }
    uqm_lua_checked!(callGenerateForScanType(
        pSolarSysState,
        (*pSolarSysState).pOrbitalDesc,
        (*ElementPtr).c2rust_unnamed_0.cycle,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        &raw mut info,
    ), ());
    setNodeNotRetrieved(
        &raw mut (*pSolarSysState).SysInfo.PlanetInfo,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        (*ElementPtr).c2rust_unnamed_0.cycle as BYTE,
    );
    (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as COORD;
    (*ElementPtr).next.location.y = ((*ElementPtr).next.location.y as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as COORD;
    CustomMineralDeposit(
        &raw mut info,
        (*ElementPtr).c2rust_unnamed_2.turn_wait as COUNT,
        (*ElementPtr).c2rust_unnamed_3.thrust_wait,
        (*ElementPtr).next.location,
    );
    NodeElementPtr = LockLink(&raw mut disp_q, hNodeElement) as *mut ELEMENT;
    (*NodeElementPtr).c2rust_unnamed.scan_node = (((((*ElementPtr).c2rust_unnamed_0.cycle
        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) as COUNT;
    (*NodeElementPtr).playerNr = C2Rust_Unnamed_9::PS_NON_PLAYER.0 as ::core::ffi::c_int
        as SIZE;
    (*NodeElementPtr).current.location = info.loc_pt;
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM.0
        as ::core::ffi::c_int as GRAPHICS_PRIM;
    (*NodeElementPtr).c2rust_unnamed_2.turn_wait = info.r#type as BYTE;
    (*NodeElementPtr).c2rust_unnamed_1.mass_points = ((info.density as ::core::ffi::c_int
        >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        - (*pSolarSysState)
            .SysInfo
            .PlanetInfo
            .PartiallyScavengedList[PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int
            as usize][(*ElementPtr).c2rust_unnamed_0.cycle as usize]
            as ::core::ffi::c_int) as BYTE;
    (*NodeElementPtr).current.image.frame = SetAbsFrameIndex(
        MiscDataFrame,
        (4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            + (*Elements.offset(info.r#type as isize) as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int) as COUNT,
    );
    (*NodeElementPtr).next.image.frame = SetRelFrameIndex(
        (*NodeElementPtr).current.image.frame,
        (info.density as BYTE as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE,
    );
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = IncFrameIndex(
        (*NodeElementPtr).next.image.frame,
    );
    (*NodeElementPtr).next.location.x = (((*NodeElementPtr).current.location.x
        as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
    (*NodeElementPtr).next.location.y = (((*NodeElementPtr).current.location.y
        as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
    (*NodeElementPtr).state_flags = ((*NodeElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    uqm_lua_checked!(setGameStateUint(
        b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    UnlockLink(&raw mut disp_q, hNodeElement);
    PutQueue(&raw mut disp_q, hNodeElement);
}
unsafe extern "C" fn spawn_node(mut ElementPtr: *mut ELEMENT) {
    if ((*ElementPtr).next.location.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*ElementPtr).next.location.y = 0 as COORD;
        memset(
            &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
    }
    if (*ElementPtr).next.location.y as ::core::ffi::c_int
        > ((67 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int
    {
        (*ElementPtr).next.location.y = (((67 as ::core::ffi::c_int) << resolutionFactor
            << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as COORD;
        memset(
            &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
    }
    if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        let mut hNodeElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut NodeElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut arc: SIZE = 0;
        let mut half_dist: COUNT = ((*ElementPtr).c2rust_unnamed.life_span
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
        hNodeElement = AllocElement();
        if hNodeElement.is_null() {
            return;
        }
        NodeElementPtr = LockLink(&raw mut disp_q, hNodeElement) as *mut ELEMENT;
        arc = (abs(
            abs(
                half_dist as ::core::ffi::c_int
                    - (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int,
            ) - half_dist as ::core::ffi::c_int,
        ) << 1 as ::core::ffi::c_int) as SIZE;
        (*NodeElementPtr).playerNr = C2Rust_Unnamed_9::PS_NON_PLAYER.0
            as ::core::ffi::c_int as SIZE;
        (*NodeElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
        (*NodeElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        (*NodeElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*NodeElementPtr).next.location.x = (*ElementPtr).next.location.x;
        (*NodeElementPtr).next.location.y = ((*ElementPtr).next.location.y
            as ::core::ffi::c_int - arc as ::core::ffi::c_int) as COORD;
        DisplayArray[(*NodeElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = IncFrameIndex(
            (*ElementPtr).next.image.frame,
        );
        UnlockLink(&raw mut disp_q, hNodeElement);
        InsertQueue(&raw mut disp_q, hNodeElement, disp_q.head);
        return;
    }
    memset(
        &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    uqm_lua_checked!(put_node(ElementPtr), ());
}
unsafe extern "C" fn LobMineralNode(
    mut which_node: COUNT,
    mut r#type: BYTE,
    amount: COUNT,
) {
    let mut dist: COUNT = 0;
    let mut angle: COUNT = 0;
    let mut deposit_quality_fine: COUNT = 0;
    let mut deposit_quality_gross: COUNT = 0;
    let mut hNodeElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut NodeElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    hNodeElement = AllocElement();
    if hNodeElement.is_null() {
        return;
    }
    angle = TFB_Random() as COUNT;
    dist = (TFB_Random() as COUNT as ::core::ffi::c_int % 8 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int) as COUNT;
    deposit_quality_fine = (amount as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
        as COUNT;
    if (deposit_quality_fine as ::core::ffi::c_int) < 150 as ::core::ffi::c_int {
        deposit_quality_gross = 0 as COUNT;
    } else if (deposit_quality_fine as ::core::ffi::c_int) < 225 as ::core::ffi::c_int {
        deposit_quality_gross = 1 as COUNT;
    } else {
        deposit_quality_gross = 2 as COUNT;
    }
    NodeElementPtr = LockLink(&raw mut disp_q, hNodeElement) as *mut ELEMENT;
    (*NodeElementPtr).playerNr = C2Rust_Unnamed_9::PS_NON_PLAYER.0 as ::core::ffi::c_int
        as SIZE;
    (*NodeElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
    (*NodeElementPtr).c2rust_unnamed.life_span = dist;
    (*NodeElementPtr).state_flags = ((1 as ::core::ffi::c_int)
        << 10 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    (*NodeElementPtr).next.location = curLanderLoc;
    (*NodeElementPtr).c2rust_unnamed_0.cycle = which_node;
    (*NodeElementPtr).c2rust_unnamed_2.turn_wait = r#type;
    (*NodeElementPtr).c2rust_unnamed_3.thrust_wait = amount as BYTE;
    (*NodeElementPtr).preprocess_func = Some(
        spawn_node as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
        .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].color = colorFromRgb15(
        0 as BYTE,
        0 as BYTE,
        0 as BYTE,
    );
    (*NodeElementPtr).current.image.frame = SetAbsFrameIndex(
        MiscDataFrame,
        (4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            + (*Elements.offset(r#type as isize) as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int) as COUNT,
    );
    (*NodeElementPtr).next.image.frame = SetRelFrameIndex(
        (*NodeElementPtr).current.image.frame,
        ((((amount as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int | deposit_quality_gross as BYTE as ::core::ffi::c_int)
            as UWORD as BYTE as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE,
    );
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = IncFrameIndex(
        (*NodeElementPtr).next.image.frame,
    );
    SetVelocityVector(
        &raw mut (*NodeElementPtr).velocity,
        (dist as SDWORD >> 1 as ::core::ffi::c_int) << resolutionFactor,
        angle,
    );
    UnlockLink(&raw mut disp_q, hNodeElement);
    InsertQueue(&raw mut disp_q, hNodeElement, disp_q.head);
}
unsafe extern "C" fn ScatterDeposits() {
    let mut i: COUNT = 0;
    let mut numDeposits: COUNT = 0;
    let mut NumNodesGrabbed: BYTE = 0;
    let mut pPSD: *mut PLANETSIDE_DESC = planetSideDesc;
    if optScatterElements.0 == 0 {
        return;
    }
    if (*pPSD).NodeData.NumNodesGrabbed == 0 {
        return;
    }
    NumNodesGrabbed = (*pPSD).NodeData.NumNodesGrabbed;
    numDeposits = uqm_lua_checked!(callGenerateForScanType(
        pSolarSysState,
        (*pSolarSysState).pOrbitalDesc,
        !(0 as ::core::ffi::c_int) as COUNT,
        PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as BYTE,
        ::core::ptr::null_mut::<NODE_INFO>(),
    ), ());
    i = numDeposits;
    while (i as ::core::ffi::c_int)
        < numDeposits as ::core::ffi::c_int + NumNodesGrabbed as ::core::ffi::c_int
    {
        let mut affix: BYTE = (i as ::core::ffi::c_int
            - numDeposits as ::core::ffi::c_int) as BYTE;
        let mut penalty: ::core::ffi::c_double = RangeMinMax(
            25 as ::core::ffi::c_int,
            75 as ::core::ffi::c_int,
            TFB_Random(),
        ) as ::core::ffi::c_double / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*pPSD).NodeData.NodeAmounts[affix as usize] as ::core::ffi::c_int
            as ::core::ffi::c_double * penalty
            >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            LobMineralNode(
                i,
                (*pPSD).NodeData.NodeType[affix as usize],
                ((*pPSD).NodeData.NodeAmounts[affix as usize] as ::core::ffi::c_int
                    as ::core::ffi::c_double * penalty) as COUNT,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LanderExplosion() -> BOOLEAN {
    let mut hExplosionElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ExplosionElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    hExplosionElement = AllocElement();
    if hExplosionElement.is_null() {
        return BOOLEAN::FALSE;
    }
    ExplosionElementPtr = LockLink(&raw mut disp_q, hExplosionElement) as *mut ELEMENT;
    (*ExplosionElementPtr).playerNr = C2Rust_Unnamed_9::PS_HUMAN_PLAYER.0
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
    uqm_lua_checked!(PlaySound(
        SetAbsStringTableIndex(
            LanderSounds,
            C2Rust_Unnamed_11::LANDER_DESTROYED.0 as ::core::ffi::c_int as COUNT,
        ),
        NotPositional(),
        ::core::ptr::null_mut::<ELEMENT>(),
        (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    ), BOOLEAN(0));
    uqm_lua_checked!(ScatterDeposits(), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoPlanetSide(mut pMS: *mut LanderInputState) -> BOOLEAN {
    let mut dx: SIZE = 0 as SIZE;
    let mut dy: SIZE = 0 as SIZE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_10::CHECK_ABORT.0 as ::core::ffi::c_int != 0
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
        landerSpeedNumer = (if uqm_lua_checked!(getGameStateUint(
            b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) != 0
        {
            2 as ::core::ffi::c_int * ((14 as ::core::ffi::c_int) << resolutionFactor)
                << 5 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int * ((8 as ::core::ffi::c_int) << resolutionFactor)
                << 5 as ::core::ffi::c_int
        }) as COUNT;
        SetVelocityComponents(
            &raw mut GlobData.Game_state.velocity,
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * landerSpeedNumer as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD
                / if isPC(optSuperPC).0 != 0 { 14 as SDWORD } else { 10 as SDWORD },
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * landerSpeedNumer as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD
                / if isPC(optSuperPC).0 != 0 { 14 as SDWORD } else { 10 as SDWORD },
        );
        return BOOLEAN::TRUE;
    } else if crew_left != 0
        && (CurrentInputState
            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            as usize][C2Rust_Unnamed_0::KEY_ESCAPE.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0
                as usize][C2Rust_Unnamed_0::KEY_SPECIAL.0 as ::core::ffi::c_int as usize]
                != 0 || (*planetSideDesc).InTransit.0 != 0)
    {
        return BOOLEAN::FALSE
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_1::KEY_DEBUG_2.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(KillLanderCrewSeq(
            crew_left as COUNT,
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as DWORD,
        ), BOOLEAN(0));
    } else if crew_left == 0 && damage_index == 0 {
        if explosion_index
            > 10 as ::core::ffi::c_int
                + 840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int)
        {
            return BOOLEAN::FALSE;
        }
        if explosion_index > 10 as ::core::ffi::c_int {
            explosion_index += 1;
        } else if explosion_index == 0 as ::core::ffi::c_int {
            if uqm_lua_checked!(LanderExplosion(), BOOLEAN(0)).0 != 0 {
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
        let mut InputState: BATTLE_INPUT_STATE = 0 as BATTLE_INPUT_STATE;
        let mut turn_left: BOOLEAN = BOOLEAN::FALSE;
        let mut turn_right: BOOLEAN = BOOLEAN::FALSE;
        let mut thrust: BOOLEAN = BOOLEAN::FALSE;
        InputState = GetDirectionalJoystickInput(
            index as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        turn_left = BOOLEAN(
            (CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0 as usize][C2Rust_Unnamed_0::KEY_LEFT.0 as ::core::ffi::c_int as usize]
                != 0
                || InputState as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                        as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        turn_right = BOOLEAN(
            (CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0
                as usize][C2Rust_Unnamed_0::KEY_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
                || InputState as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        thrust = BOOLEAN(
            (CurrentInputState
                .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize))
                .0
                as usize][C2Rust_Unnamed_0::KEY_THRUST.0 as ::core::ffi::c_int as usize]
                != 0
                || CurrentInputState
                    .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE)
                    .offset(0isize))
                    .0
                    as usize][C2Rust_Unnamed_0::KEY_UP.0 as ::core::ffi::c_int as usize]
                    != 0
                || InputState as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if turn_wait != 0 {
            turn_wait -= 1;
        } else if turn_left.0 != 0 || turn_right.0 != 0 {
            let mut landerSpeedNumer_0: COUNT = 0;
            let mut angle_0: COUNT = 0;
            if turn_left.0 != 0 {
                index -= 1;
            } else {
                index += 1;
            }
            index = (index as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as SIZE;
            LanderFrame[0usize] = SetAbsFrameIndex(LanderFrame[0usize], index as COUNT);
            angle_0 = ((index as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
            landerSpeedNumer_0 = (if uqm_lua_checked!(getGameStateUint(
                b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) != 0
            {
                (2 as ::core::ffi::c_int * 14 as ::core::ffi::c_int) << resolutionFactor
                    << 5 as ::core::ffi::c_int
            } else {
                (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) << resolutionFactor
                    << 5 as ::core::ffi::c_int
            }) as COUNT;
            SetVelocityComponents(
                &raw mut GlobData.Game_state.velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle_0 as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * landerSpeedNumer_0 as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD
                    / if isPC(optSuperPC).0 != 0 { 14 as SDWORD } else { 10 as SDWORD },
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle_0 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * landerSpeedNumer_0 as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD
                    / if isPC(optSuperPC).0 != 0 { 14 as SDWORD } else { 10 as SDWORD },
            );
            turn_wait = if isPC(optSuperPC).0 != 0 {
                1 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            };
        }
        if thrust.0 == 0 {
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
            as usize][C2Rust_Unnamed_0::KEY_WEAPON.0 as ::core::ffi::c_int as usize] != 0
        {
            uqm_lua_checked!(LanderFire(index), BOOLEAN(0));
            weapon_wait = if isPC(optSuperPC).0 != 0 {
                10 as ::core::ffi::c_int
            } else {
                14 as ::core::ffi::c_int
            };
            if uqm_lua_checked!(getGameStateUint(
                b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) != 0
            {
                weapon_wait >>= 1 as ::core::ffi::c_int;
            }
        }
    }
    uqm_lua_checked!(ScrollPlanetSide(dx, dy, 0 as ::core::ffi::c_int), BOOLEAN(0));
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0
    {
        static mut TimeOutClock: TimeCount = 0;
        let mut Now: TimeCount = GetTimeCounter();
        if Now >= TimeOutClock {
            uqm_lua_checked!(GameClockTick(), BOOLEAN(0));
            TimeOutClock = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as TimeCount,
                );
        }
    }
    uqm_lua_checked!(SleepThreadUntil((*pMS).NextTime), BOOLEAN(0));
    (*pMS).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int) as TimeCount,
        );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn MaskLanderGraphics() {
    let mut currFlagState: BYTE = 0;
    if optShowUpgrades.0 == 0 {
        return;
    }
    currFlagState = (uqm_lua_checked!(getGameStateUint(
        b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) << 0 as ::core::ffi::c_int)
        .wrapping_add(
            uqm_lua_checked!(getGameStateUint(
                b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) << 1 as ::core::ffi::c_int,
        )
        .wrapping_add(
            uqm_lua_checked!(getGameStateUint(
                b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) << 2 as ::core::ffi::c_int,
        ) as BYTE;
    if currFlagState as ::core::ffi::c_int != LanderUpgradesFlag as ::core::ffi::c_int {
        let mut i: COUNT = 0;
        let mut j: COUNT = 0;
        let mut mods: FRAME = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.lander.mods\0".as_ptr() as RESOURCE), ())
                as DRAWABLE,
        );
        let mut lander: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut mask: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut mode: DrawMode = MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0xff as SWORD);
        DestroyDrawable(ReleaseDrawable(LanderFrame[0usize]));
        LanderFrame[0usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.lander\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
        LanderUpgradesFlag = currFlagState;
        j = 0 as COUNT;
        while (j as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
            if currFlagState as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << j as ::core::ffi::c_int != 0
            {
                i = 0 as COUNT;
                while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                    mask = SetAbsFrameIndex(
                        mods,
                        (i as ::core::ffi::c_int
                            + j as ::core::ffi::c_int * 16 as ::core::ffi::c_int)
                            as COUNT,
                    );
                    lander = SetAbsFrameIndex(LanderFrame[0usize], i);
                    uqm_lua_checked!(ApplyMask(mask, lander, mode, ::core::ptr::null_mut::<Color>()), ());
                    i = i.wrapping_add(1);
                }
            }
            j = j.wrapping_add(1);
        }
        DestroyDrawable(ReleaseDrawable(mods));
        mods = ::core::ptr::null_mut::<FRAME_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeLanderData() {
    let mut i: COUNT = 0;
    let mut landerFrameCount: COUNT = 0;
    if LanderFrame[0usize].is_null() {
        return;
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 6 as ::core::ffi::c_int {
        uqm_lua_checked!(DestroyMusic(OrbitMusic[i as usize]), ());
        OrbitMusic[i as usize] = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(DestroySound(ReleaseStringTable(LanderSounds)), ());
    LanderSounds = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    landerFrameCount = ::core::mem::size_of::<[FRAME; 8]>()
        .wrapping_div(::core::mem::size_of::<FRAME>()) as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < landerFrameCount as ::core::ffi::c_int {
        DestroyDrawable(ReleaseDrawable(LanderFrame[i as usize]));
        LanderFrame[i as usize] = ::core::ptr::null_mut::<FRAME_DESC>();
        i = i.wrapping_add(1);
    }
    LanderUpgradesFlag = 0 as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadLanderData() {
    if !LanderFrame[0usize].is_null() {
        return;
    }
    LanderFrame[0usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.lander\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[1usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.quake\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[2usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.lightning\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[3usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.lavaspot\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[4usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.landershield\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[5usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.landerlaunch\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[6usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.landerreturn\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderFrame[7usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.lander.pc\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    LanderUpgradesFlag = 0 as BYTE;
    LanderSounds = CaptureStringTable(
        uqm_lua_checked!(LoadSoundInstance(b"sounds.lander\0".as_ptr() as RESOURCE), ()),
    ) as SOUND;
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < num_orbit_themes() {
        OrbitMusic[i as usize] = uqm_lua_checked!(load_orbit_theme(i as BYTE), ());
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetPlanetMusic(mut planet_type: BYTE) -> BYTE {
    let mut OrbitNum: BYTE = (planet_type as ::core::ffi::c_int % num_orbit_themes())
        as BYTE;
    LanderMusic = OrbitMusic[OrbitNum as usize];
    return (if !(opt3doMusic.0 != 0 || optRemixMusic.0 != 0 || optVolasMusic.0 != 0) {
        0 as ::core::ffi::c_int
    } else {
        OrbitNum as ::core::ffi::c_int
    }) as BYTE;
}
unsafe extern "C" fn ReturnToOrbit() {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if is3DO(optSuperPC).0 != 0 {
        OldContext = SetContext(PlanetContext);
        GetContextClipRect(&raw mut r);
        SetTransitionSource(&raw mut r);
        BatchGraphics();
        uqm_lua_checked!(DrawStarBackGround(), ());
        uqm_lua_checked!(DrawDefaultPlanetSphere(), ());
        uqm_lua_checked!(DrawPlanetSurfaceBorder(), ());
        uqm_lua_checked!(RedrawSurfaceScan(::core::ptr::null::<POINT>()), ());
        uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), ());
        UnbatchGraphics();
    } else {
        let mut b: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        OldContext = SetContext(RadarContext);
        GetContextClipRect(&raw mut r);
        b = r;
        SetTransitionSource(&raw mut r);
        b.corner.y = 0 as COORD;
        b.corner.x = b.corner.y;
        BatchGraphics();
        uqm_lua_checked!(ClearDrawable(), ());
        uqm_lua_checked!(DrawRadarArea(), ());
        if isPC(optSuperPC).0 == 0 {
            uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), ());
        }
        uqm_lua_checked!(RedrawSurfaceScan(::core::ptr::null::<POINT>()), ());
        UnbatchGraphics();
    }
    SetContext(OldContext);
}
unsafe extern "C" fn IdlePlanetSide(
    mut inputState: *mut LanderInputState,
    mut howLong: TimeCount,
) {
    let mut TimeOut: TimeCount = GetTimeCounter().wrapping_add(howLong);
    while GetTimeCounter() < TimeOut {
        uqm_lua_checked!(ScrollPlanetSide(
            0 as SIZE,
            0 as SIZE,
            -(MapSurface.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                + ((10 as ::core::ffi::c_int) << resolutionFactor)),
        ), ());
        uqm_lua_checked!(SleepThreadUntil((*inputState).NextTime), ());
        (*inputState).NextTime = (*inputState)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int) as TimeCount,
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
    let mut max_offsets: ::core::ffi::c_int = 0;
    let mut landingOfsHD: [::core::ffi::c_int; 400] = [0; 400];
    delta = 0 as ::core::ffi::c_int;
    max_offsets = if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        20 as ::core::ffi::c_int
    } else {
        400 as ::core::ffi::c_int
    };
    index = 0 as ::core::ffi::c_int;
    while index < max_offsets
        && delta
            < MapSurface.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                + ((10 as ::core::ffi::c_int) << resolutionFactor)
    {
        delta += index + 1 as ::core::ffi::c_int;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            landingOfsHD[index as usize] = -delta;
        } else {
            landingOfs[index as usize] = -delta;
        }
        index += 1;
    }
    (delta
        >= MapSurface.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            + ((10 as ::core::ffi::c_int) << resolutionFactor)
        && !(b"Increase MAX_OFFSETS!\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        || {
            __assert_fail(
                b"delta >= DISTANCE_COVERED && \"Increase MAX_OFFSETS!\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/lander.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2475 as ::core::ffi::c_int,
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
        uqm_lua_checked!(IdlePlanetSide(inputState, 840 as TimeCount), ());
    }
    index = start;
    while index != end {
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            uqm_lua_checked!(ScrollPlanetSide(0 as SIZE, 0 as SIZE, landingOfsHD[index as usize]), ());
        } else {
            uqm_lua_checked!(ScrollPlanetSide(0 as SIZE, 0 as SIZE, landingOfs[index as usize]), ());
        }
        uqm_lua_checked!(SleepThreadUntil((*inputState).NextTime), ());
        (*inputState).NextTime = (*inputState)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int) as TimeCount,
            );
        index += delta;
    }
    if landing.0 == 0 {
        uqm_lua_checked!(IdlePlanetSide(
            inputState,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount,
        ), ());
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
                2513 as ::core::ffi::c_int,
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
        uqm_lua_checked!(DeltaLanderCrew(
            -1 as SIZE,
            C2Rust_Unnamed_11::LANDER_INJURED.0 as ::core::ffi::c_int as COUNT,
        ), false);
        uqm_lua_checked!(SleepThreadUntil(TimeOut), false);
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
    let mut TectonicsChanceTab: [BYTE; 8] = [
        0 as BYTE,
        0 as BYTE,
        (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (16 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (32 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
    ];
    let mut WeatherChanceTab: [BYTE; 8] = [
        0 as BYTE,
        0 as BYTE,
        (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (3 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (6 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (24 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
    ];
    let mut FireChanceTab: [BYTE; 8] = [
        0 as BYTE,
        0 as BYTE,
        (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (24 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
        (48 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE,
    ];
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        TectonicsChanceTab[1usize] = 1 as BYTE;
        WeatherChanceTab[1usize] = 1 as BYTE;
    }
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
        NodeData: NODE_DATA {
            NumNodesGrabbed: 0,
            NodeType: [0; 32],
            NodeAmounts: [0; 32],
        },
    };
    DirJoyActive = BOOLEAN::TRUE;
    memset(
        &raw mut PSD as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PLANETSIDE_DESC>(),
    );
    PSD.InTransit = BOOLEAN::TRUE;
    PSD.TectonicsChance = GetHazardChance(
        C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int,
        (*pSolarSysState).SysInfo.PlanetInfo.Tectonics as ::core::ffi::c_uint,
    );
    PSD.WeatherChance = GetHazardChance(
        C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int,
        (*pSolarSysState).SysInfo.PlanetInfo.Weather as ::core::ffi::c_uint,
    );
    PSD.FireChance = GetHazardChance(
        C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int,
        GetThermalHazardRating(
            (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
        ),
    );
    PSD.ElementLevel = (GetStorageBayCapacity() as ::core::ffi::c_int
        - GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int) as COUNT;
    PSD.MaxElementLevel = (if isPC(optSuperPC).0 != 0 {
        64 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int
    }) as COUNT;
    if uqm_lua_checked!(getGameStateUint(
        b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
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
            - 1 as ::core::ffi::c_int) as DWORD) as SIZE;
    LanderFrame[0usize] = SetAbsFrameIndex(LanderFrame[0usize], index as COUNT);
    crew_left = 0 as ::core::ffi::c_int;
    damage_index = 0 as ::core::ffi::c_int;
    explosion_index = 0 as ::core::ffi::c_int;
    uqm_lua_checked!(AnimateLanderWarmup(), ());
    uqm_lua_checked!(AnimateLaunch(LanderFrame[5usize], BOOLEAN::TRUE), ());
    uqm_lua_checked!(InitPlanetSide(planetLoc), ());
    landerInputState.NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 42 as ::core::ffi::c_int) as TimeCount,
        );
    uqm_lua_checked!(LandingTakeoffSequence(&raw mut landerInputState, BOOLEAN::TRUE), ());
    PSD.InTransit = BOOLEAN::FALSE;
    landerInputState.Initialized = BOOLEAN::FALSE;
    landerInputState.InputFunc = Some(
        DoPlanetSide as unsafe extern "C" fn(*mut LanderInputState) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut LanderInputState) -> BOOLEAN>;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    uqm_lua_checked!(DoInput(&raw mut landerInputState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), ());
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_10::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if crew_left == 0 as ::core::ffi::c_int {
            GlobData.SIS_state.NumLanders = GlobData
                .SIS_state
                .NumLanders
                .wrapping_sub(1);
            if optSubmenu.0 == 0 {
                uqm_lua_checked!(DrawLanders(), ());
            }
            uqm_lua_checked!(ReturnToOrbit(), ());
        } else {
            PSD.InTransit = BOOLEAN::TRUE;
            uqm_lua_checked!(PlaySound(
                SetAbsStringTableIndex(
                    LanderSounds,
                    C2Rust_Unnamed_11::LANDER_RETURNS.0 as ::core::ffi::c_int as COUNT,
                ),
                NotPositional(),
                ::core::ptr::null_mut::<ELEMENT>(),
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
            ), ());
            uqm_lua_checked!(LandingTakeoffSequence(&raw mut landerInputState, BOOLEAN::FALSE), ());
            if is3DO(optSuperPC).0 != 0 {
                uqm_lua_checked!(ReturnToOrbit(), ());
            }
            uqm_lua_checked!(AnimateLaunch(LanderFrame[6usize], BOOLEAN::FALSE), ());
            if optSubmenu.0 == 0 {
                uqm_lua_checked!(DeltaSISGauges(crew_left as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
            } else {
                let mut CrewCapacity: COUNT = GetCrewPodCapacity();
                GlobData.SIS_state.CrewEnlisted = (GlobData.SIS_state.CrewEnlisted
                    as ::core::ffi::c_int + crew_left) as COUNT;
                if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    > CrewCapacity as ::core::ffi::c_int
                {
                    GlobData.SIS_state.CrewEnlisted = CrewCapacity;
                }
            }
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
                if optSubmenu.0 == 0 {
                    uqm_lua_checked!(DrawStorageBays(BOOLEAN::FALSE), ());
                }
            }
            GlobData.SIS_state.TotalBioMass = (GlobData.SIS_state.TotalBioMass
                as ::core::ffi::c_int + PSD.BiologicalLevel as ::core::ffi::c_int)
                as COUNT;
            if isPC(optSuperPC).0 != 0 {
                uqm_lua_checked!(ReturnToOrbit(), ());
                uqm_lua_checked!(InitPCLander(BOOLEAN::FALSE), ());
            }
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
    DirJoyActive = BOOLEAN::FALSE;
}
unsafe extern "C" fn DrawMinDatText() {
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    if strlen(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_13::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                    + 20 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    ) == 0
        || strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_13::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                        + 21 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ) == 0
    {
        return;
    }
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    rect.corner.x = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    rect.corner.y = (((53 as ::core::ffi::c_int) << resolutionFactor)
        - ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rect.extent.width = ((46 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    rect.extent.height = ((6 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut rect), ());
    OldFont = SetContextFont(TinyFontBold);
    SetContextForeGroundColor(
        buildColorRgba(0x7c as BYTE, 0 as BYTE, 0 as BYTE, 0xff as BYTE),
    );
    text.align = TEXT_ALIGN::ALIGN_LEFT;
    text.baseline.x = (rect.corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    text.baseline.y = (rect.corner.y as ::core::ffi::c_int
        + rect.extent.height as ::core::ffi::c_int) as COORD;
    text.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_13::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 20 as ::core::ffi::c_int) as COUNT,
        ),
    );
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut text), ());
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0 as BYTE, 0x90 as BYTE, 0xff as BYTE),
    );
    text.align = TEXT_ALIGN::ALIGN_RIGHT;
    text.baseline.x = (rect.corner.x as ::core::ffi::c_int
        + rect.extent.width as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    text.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_13::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 21 as ::core::ffi::c_int) as COUNT,
        ),
    );
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut text), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
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
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((53 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
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
        s.frame = SetAbsFrameIndex(LanderFrame[0usize], 32 as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        uqm_lua_checked!(DrawMinDatText(), ());
        if LanderFlags as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            ShieldFlags = uqm_lua_checked!(getGameStateUint(
                b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            capacity_shift = uqm_lua_checked!(getGameStateUint(
                b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            uqm_lua_checked!(DrawMineralHelpers(), ());
        } else {
            ShieldFlags = (LanderFlags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int)
                    << C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_11::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int))
                as ::core::ffi::c_uchar as BYTE;
            s.frame = IncFrameIndex(s.frame);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
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
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
                s.frame = IncFrameIndex(s.frame);
            }
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            if LanderFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int == 0
            {
                capacity_shift = 0 as BYTE;
            } else {
                capacity_shift = 1 as BYTE;
                s.frame = SetAbsFrameIndex(s.frame, 59 as COUNT);
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
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
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        free_space = (GetStorageBayCapacity() as ::core::ffi::c_int
            - GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int) as COUNT;
        if (free_space as ::core::ffi::c_int)
            < if isPC(optSuperPC).0 != 0 {
                64 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            } << capacity_shift as ::core::ffi::c_int
        {
            r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            r.extent.width = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            r.extent.height = ((if isPC(optSuperPC).0 != 0 {
                64 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            }
                - (free_space as ::core::ffi::c_int
                    >> capacity_shift as ::core::ffi::c_int)
                    * if isPC(optSuperPC).0 != 0 {
                        64 as ::core::ffi::c_int
                    } else {
                        50 as ::core::ffi::c_int
                    }
                    / if isPC(optSuperPC).0 != 0 {
                        64 as ::core::ffi::c_int
                    } else {
                        50 as ::core::ffi::c_int
                    } + 2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
        s.frame = SetAbsFrameIndex(s.frame, 37 as COUNT);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
    }
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn InitPCLander(mut Loading: BOOLEAN) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(GetPCLanderContext(::core::ptr::null_mut::<BOOLEAN>()));
    GetContextClipRect(&raw mut r);
    r.corner = MAKE_POINT(0 as COORD, 0 as COORD);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    BatchGraphics();
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    if GlobData.SIS_state.NumLanders as ::core::ffi::c_int != 0 && Loading.0 == 0
        && (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_12::PLANET_SHIELDED.0 as ::core::ffi::c_int == 0
        && !((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            >= C2Rust_Unnamed_12::FIRST_GAS_GIANT.0 as ::core::ffi::c_int
            && (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                <= C2Rust_Unnamed_12::LAST_GAS_GIANT.0 as ::core::ffi::c_int)
    {
        let mut ShieldFlags: BYTE = 0;
        let mut capacity_shift: BYTE = 0;
        let mut free_space: COUNT = 0;
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        s.frame = SetAbsFrameIndex(LanderFrame[7usize], 32 as COUNT);
        s.origin = MAKE_POINT(0 as COORD, 0 as COORD);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        ShieldFlags = uqm_lua_checked!(getGameStateUint(
            b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        capacity_shift = uqm_lua_checked!(getGameStateUint(
            b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        free_space = (GetStorageBayCapacity() as ::core::ffi::c_int
            - GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int) as COUNT;
        if (free_space as ::core::ffi::c_int)
            < if isPC(optSuperPC).0 != 0 {
                64 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            } << capacity_shift as ::core::ffi::c_int
        {
            let mut i: COUNT = 0;
            let mut inc: COUNT = 0;
            inc = (if isPC(optSuperPC).0 != 0 {
                64 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            }
                - (free_space as ::core::ffi::c_int
                    >> capacity_shift as ::core::ffi::c_int)
                    * if isPC(optSuperPC).0 != 0 {
                        64 as ::core::ffi::c_int
                    } else {
                        50 as ::core::ffi::c_int
                    }
                    / if isPC(optSuperPC).0 != 0 {
                        64 as ::core::ffi::c_int
                    } else {
                        50 as ::core::ffi::c_int
                    } + 1 as ::core::ffi::c_int) as COUNT;
            r.corner = MAKE_POINT(
                ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD,
                0 as COORD,
            );
            r.extent = MAKE_EXTENT(
                ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD,
                ((inc as ::core::ffi::c_int) << resolutionFactor) as COORD,
            );
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            r.extent = MAKE_EXTENT(
                ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD,
                ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD,
            );
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < inc as ::core::ffi::c_int {
                if i as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    r.corner.y = (r.corner.y as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    SetContextForeGroundColor(
                        colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
                    );
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    SetContextForeGroundColor(
                        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    );
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    r.corner.y = (r.corner.y as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                }
                i = i.wrapping_add(1);
            }
        }
        s.frame = SetAbsFrameIndex(s.frame, 37 as COUNT);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::LIGHTNING_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.frame = IncFrameIndex(s.frame);
        if ShieldFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << C2Rust_Unnamed_11::LAVASPOT_DISASTER.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
    }
    UnbatchGraphics();
}
