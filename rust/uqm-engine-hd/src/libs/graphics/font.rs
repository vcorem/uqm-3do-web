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

// Rust translation of src/libs/graphics/font.c.
extern "C" {
    pub type NativePalette;
    fn hypot(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn MAKE_HOT_SPOT(_: COORD, _: COORD) -> HOT_SPOT;
    fn BoxIntersect(
        pr1: *mut RECT,
        pr2: *mut RECT,
        printer: *mut RECT,
    ) -> INTERSECT_CODE;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    static mut _pCurContext: CONTEXT;
    static mut _GraphicsStatusFlags: GRAPHICS_STATUS;
    fn GetContextValidRect(pValidRect: *mut RECT, origin: *mut POINT) -> BOOLEAN;
    fn FixContextFontEffect();
    fn TFB_DrawScreen_DeleteImage(img: *mut TFB_Image);
    fn TFB_DrawImage_CreateForScreen(
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        withalpha: BOOLEAN,
    ) -> *mut TFB_Image;
    fn TFB_DrawImage_Delete(image: *mut TFB_Image);
    fn TFB_DrawImage_Rect(
        rect: *mut RECT,
        _: Color,
        _: DrawMode,
        target: *mut TFB_Image,
    );
    fn TFB_DrawImage_Image(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: *mut TFB_ColorMap,
        _: DrawMode,
        target: *mut TFB_Image,
    );
    fn SleepThread(timePeriod: TimePeriod);
    static mut _CurFontPtr: FONT;
    fn _ReleaseFontData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn TFB_Prim_FontChar(
        charOrigin: POINT,
        fontChar: *mut TFB_Char,
        backing: *mut TFB_Image,
        _: DrawMode,
        ctxOrigin: POINT,
    );
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    static mut StarConFont: FONT;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type UniChar = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_desc {
    pub Flags: UWORD,
    pub ForeGroundColor: Color,
    pub BackGroundColor: Color,
    pub Mode: DrawMode,
    pub ForeGroundFrame: FRAME,
    pub Font: FONT,
    pub ClipRect: RECT,
    pub FontEffect: FRAME,
    pub FontBacking: *mut TFB_Image,
    pub BackingFlags: FBK_FLAGS,
    pub name: *const ::core::ffi::c_char,
    pub next: CONTEXT,
}
pub type CONTEXT = *mut CONTEXT_DESC;
pub type CONTEXT_DESC = context_desc;
pub type FBK_FLAGS = UWORD;
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
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
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
pub type CREATE_FLAGS = BYTE;
pub type DRAWABLE_TYPE = UWORD;
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_desc {
    pub Leading: BYTE,
    pub fontPages: *mut FONT_PAGE,
    pub disp: EXTENT,
    pub filename: [::core::ffi::c_char; 4096],
    pub CharSpace: BYTE,
    pub KernAmount: BYTE,
    pub KernTab: [BYTE; 65535],
    pub VertAlign: SBYTE,
    pub HaveFntData: BOOLEAN,
}
pub type FONT_PAGE = FontPage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontPage {
    pub next: *mut FontPage,
    pub pageStart: UniChar,
    pub firstChar: UniChar,
    pub numChars: size_t,
    pub charDesc: *mut TFB_Char,
}
pub type TFB_Char = tfb_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_char {
    pub extent: EXTENT,
    pub disp: EXTENT,
    pub HotSpot: HOT_SPOT,
    pub data: *mut BYTE,
    pub pitch: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
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
    pub flags: BYTE,
}
pub type INTERSECT_CODE = BYTE;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const NORTH_SHADOW: Self = Self(0);
    pub const NORTH_EAST_SHADOW: Self = Self(1);
    pub const EAST_SHADOW: Self = Self(2);
    pub const SOUTH_EAST_SHADOW: Self = Self(3);
    pub const SOUTH_SHADOW: Self = Self(4);
    pub const SOUTH_WEST_SHADOW: Self = Self(5);
    pub const WEST_SHADOW: Self = Self(6);
    pub const NORTH_WEST_SHADOW: Self = Self(7);
}
pub type GRAPHICS_STATUS = BYTE;
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
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
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
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type TFB_ColorMap = tfb_colormap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_colormap {
    pub index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut tfb_colormap,
    pub palette: *mut NativePalette,
}
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
    pub const MENU_SOUND_BUBBLEWARP: Self = Self(4);
    pub const MENU_SOUND_TEXT: Self = Self(5);
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[no_mangle]
pub unsafe extern "C" fn GetFrameForFPS(mut ch: UniChar) -> *mut TFB_Char {
    if !StarConFont.is_null()
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        return getCharFrame(StarConFont, ch)
    } else {
        return ::core::ptr::null_mut::<TFB_Char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn GoodToGoFPS() -> BOOLEAN {
    return BOOLEAN(
        (!StarConFont.is_null()
            && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int == 0
            && optRequiresReload.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetFontDims(mut w: *mut SIZE, mut h: *mut SIZE) {
    *w = (*StarConFont).disp.width;
    *h = (*StarConFont).disp.height;
}
#[no_mangle]
pub unsafe extern "C" fn SetContextFont(mut Font: FONT) -> FONT {
    let mut LastFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    LastFont = _CurFontPtr;
    _CurFontPtr = Font;
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int != 0
    {
        (*_pCurContext).Font = Font;
        (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
            as FBK_FLAGS;
    }
    return LastFont;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyFont(mut FontRef: FONT) -> BOOLEAN {
    if FontRef.is_null() {
        return BOOLEAN::FALSE;
    }
    if !_CurFontPtr.is_null() && _CurFontPtr == FontRef {
        SetContextFont(::core::ptr::null_mut::<::core::ffi::c_void>() as FONT);
    }
    return _ReleaseFontData(FontRef as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn font_GetTextRect(mut lpText: *mut TEXT) -> RECT {
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    memset(
        &raw mut ClipRect as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<RECT>(),
    );
    uqm_lua_checked!(FixContextFontEffect(), rect { corner: point { x: 0, y: 0 }, extent: extent { width: 0, height: 0 } });
    if !(_GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int)
        || GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 == 0
    {
        return ClipRect;
    }
    text = *lpText;
    if TextRect(&raw mut text, &raw mut ClipRect, ::core::ptr::null_mut::<BYTE>()).0 == 0
    {
        return ClipRect;
    }
    return ClipRect;
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawText(mut lpText: *mut TEXT) {
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    uqm_lua_checked!(FixContextFontEffect(), ());
    if !(_GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int)
        || GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 == 0
    {
        return;
    }
    text = *lpText;
    if TextRect(&raw mut text, &raw mut ClipRect, ::core::ptr::null_mut::<BYTE>()).0 == 0
    {
        return;
    }
    uqm_lua_checked!(_text_blt(&raw mut ClipRect, &raw mut text, origin), ());
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawText_Fade(
    mut lpText: *mut TEXT,
    mut repair: FRAME,
    mut skip: *mut BOOLEAN,
) {
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    if !(_GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int)
        || GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 == 0
    {
        return;
    }
    text = *lpText;
    if TextRect(&raw mut text, &raw mut ClipRect, ::core::ptr::null_mut::<BYTE>()).0 == 0
    {
        return;
    }
    uqm_lua_checked!(_text_blt_fade(&raw mut ClipRect, &raw mut text, origin, repair, skip), ());
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawTracedText(
    mut pText: *mut TEXT,
    mut text: Color,
    mut trace: Color,
) {
    let oldfg: Color = SetContextForeGroundColor(trace);
    let stroke: BYTE = ((1 as ::core::ffi::c_int) << resolutionFactor) as BYTE;
    let t_baseline: POINT = (*pText).baseline;
    let mut offset: POINT = POINT { x: 0, y: 0 };
    offset.x = -(stroke as ::core::ffi::c_int) as COORD;
    while offset.x as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
        offset.y = -(stroke as ::core::ffi::c_int) as COORD;
        while offset.y as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
            if hypot(
                offset.x as ::core::ffi::c_double,
                offset.y as ::core::ffi::c_double,
            ) <= stroke as ::core::ffi::c_int as ::core::ffi::c_double
            {
                (*pText).baseline = MAKE_POINT(
                    (t_baseline.x as ::core::ffi::c_int + offset.x as ::core::ffi::c_int)
                        as COORD,
                    (t_baseline.y as ::core::ffi::c_int + offset.y as ::core::ffi::c_int)
                        as COORD,
                );
                uqm_lua_checked!(font_DrawText(pText), ());
            }
            offset.y += 1;
        }
        offset.x += 1;
    }
    (*pText).baseline = t_baseline;
    SetContextForeGroundColor(text);
    uqm_lua_checked!(font_DrawText(pText), ());
    SetContextForeGroundColor(oldfg);
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawTextAlt(
    mut lpText: *mut TEXT,
    mut swap: BYTE,
    mut AltFontPtr: FONT,
    mut key: UniChar,
) -> BYTE {
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    uqm_lua_checked!(FixContextFontEffect(), 0);
    if !(_GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int)
        || GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 == 0
    {
        return 0 as BYTE;
    }
    text = *lpText;
    if TextRectAlt(
            &raw mut text,
            &raw mut ClipRect,
            ::core::ptr::null_mut::<BYTE>(),
            swap,
            key,
            AltFontPtr,
        )
        .0 == 0
    {
        return 0 as BYTE;
    }
    return uqm_lua_checked!(_text_blt_alt(
        &raw mut ClipRect,
        &raw mut text,
        origin,
        swap,
        AltFontPtr,
        key,
    ), 0);
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawTracedTextAlt(
    mut pText: *mut TEXT,
    mut text: Color,
    mut trace: Color,
    mut AltFontPtr: FONT,
    mut key: UniChar,
) {
    let oldfg: Color = SetContextForeGroundColor(trace);
    let stroke: BYTE = ((1 as ::core::ffi::c_int) << resolutionFactor) as BYTE;
    let t_baseline: POINT = (*pText).baseline;
    let mut offset: POINT = POINT { x: 0, y: 0 };
    static mut swap: BYTE = 0 as BYTE;
    offset.x = -(stroke as ::core::ffi::c_int) as COORD;
    while offset.x as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
        offset.y = -(stroke as ::core::ffi::c_int) as COORD;
        while offset.y as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
            if hypot(
                offset.x as ::core::ffi::c_double,
                offset.y as ::core::ffi::c_double,
            ) <= stroke as ::core::ffi::c_int as ::core::ffi::c_double
            {
                (*pText).baseline = MAKE_POINT(
                    (t_baseline.x as ::core::ffi::c_int + offset.x as ::core::ffi::c_int)
                        as COORD,
                    (t_baseline.y as ::core::ffi::c_int + offset.y as ::core::ffi::c_int)
                        as COORD,
                );
                uqm_lua_checked!(font_DrawTextAlt(pText, swap, AltFontPtr, key), ());
            }
            offset.y += 1;
        }
        offset.x += 1;
    }
    (*pText).baseline = t_baseline;
    SetContextForeGroundColor(text);
    swap = uqm_lua_checked!(font_DrawTextAlt(pText, swap, AltFontPtr, key), ());
    SetContextForeGroundColor(oldfg);
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawShadowedText(
    mut pText: *mut TEXT,
    mut direction: BYTE,
    mut text_color: Color,
    mut shadow_color: Color,
) {
    let mut shadow_angle: POINT = point {
        x: 0 as COORD,
        y: 0 as COORD,
    };
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    match direction as ::core::ffi::c_int {
        0 => {
            shadow_angle = MAKE_POINT(0 as COORD, -1 as COORD);
        }
        1 => {
            shadow_angle = MAKE_POINT(1 as COORD, -1 as COORD);
        }
        2 => {
            shadow_angle = MAKE_POINT(1 as COORD, 0 as COORD);
        }
        3 => {
            shadow_angle = MAKE_POINT(1 as COORD, 1 as COORD);
        }
        4 => {
            shadow_angle = MAKE_POINT(0 as COORD, 1 as COORD);
        }
        5 => {
            shadow_angle = MAKE_POINT(-1 as COORD, 1 as COORD);
        }
        6 => {
            shadow_angle = MAKE_POINT(-1 as COORD, 0 as COORD);
        }
        7 => {
            shadow_angle = MAKE_POINT(-1 as COORD, -1 as COORD);
        }
        _ => {}
    }
    (*pText).baseline.x = ((*pText).baseline.x as ::core::ffi::c_int
        + ((shadow_angle.x as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
        + ((shadow_angle.y as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    OldColor = SetContextForeGroundColor(shadow_color);
    uqm_lua_checked!(font_DrawText(pText), ());
    (*pText).baseline.x = ((*pText).baseline.x as ::core::ffi::c_int
        - ((shadow_angle.x as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
        - ((shadow_angle.y as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(text_color);
    uqm_lua_checked!(font_DrawText(pText), ());
    SetContextForeGroundColor(OldColor);
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFontLeading(mut pheight: *mut SIZE) -> BOOLEAN {
    if !_CurFontPtr.is_null() {
        *pheight = (*_CurFontPtr).Leading as SIZE;
        return BOOLEAN::TRUE;
    }
    *pheight = 0 as SIZE;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFontDispHeight(mut pheight: *mut SIZE) -> BOOLEAN {
    if !_CurFontPtr.is_null() {
        *pheight = (*_CurFontPtr).disp.height;
        return BOOLEAN::TRUE;
    }
    *pheight = 0 as SIZE;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFontDispWidth(mut pwidth: *mut SIZE) -> BOOLEAN {
    if !_CurFontPtr.is_null() {
        *pwidth = (*_CurFontPtr).disp.width;
        return BOOLEAN::TRUE;
    }
    *pwidth = 0 as SIZE;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFontVertAlign(mut valign: *mut SBYTE) -> SBYTE {
    if !_CurFontPtr.is_null() {
        *valign = (*_CurFontPtr).VertAlign;
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int as SBYTE;
    }
    *valign = 0 as SBYTE;
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int as SBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn TextRect(
    mut lpText: *mut TEXT,
    mut pRect: *mut RECT,
    mut pdelta: *mut BYTE,
) -> BOOLEAN {
    let mut char_delta_array: [BYTE; 100] = [0; 100];
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    FontPtr = _CurFontPtr;
    if !FontPtr.is_null()
        && (*lpText).CharCount as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        let mut top_y: COORD = 0;
        let mut bot_y: COORD = 0;
        let mut width: SIZE = 0;
        let mut next_ch: UniChar = 0 as UniChar;
        let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut num_chars: COUNT = 0;
        num_chars = (*lpText).CharCount;
        if pdelta.is_null() {
            pdelta = &raw mut char_delta_array as *mut BYTE;
            if num_chars as ::core::ffi::c_int > 100 as ::core::ffi::c_int {
                num_chars = 100 as COUNT;
                (*lpText).CharCount = 100 as COUNT;
            }
        }
        top_y = 0 as COORD;
        bot_y = 0 as COORD;
        width = 0 as SIZE;
        pStr = (*lpText).pStr as *const ::core::ffi::c_char;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        loop {
            let c2rust_fresh0 = num_chars;
            num_chars = num_chars.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            let mut ch: UniChar = 0;
            let mut last_width: SIZE = 0;
            let mut charFrame: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
            last_width = width;
            ch = next_ch;
            if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                next_ch = getCharFromString(&raw mut pStr);
                if next_ch == '\0' as UniChar {
                    (*lpText).CharCount = ((*lpText).CharCount as ::core::ffi::c_int
                        - num_chars as ::core::ffi::c_int) as COUNT;
                    num_chars = 0 as COUNT;
                }
            }
            charFrame = getCharFrame(FontPtr, ch);
            if !charFrame.is_null() && (*charFrame).disp.width as ::core::ffi::c_int != 0
            {
                let mut y: COORD = 0;
                y = -((*charFrame).HotSpot.y as ::core::ffi::c_int) as COORD;
                if (y as ::core::ffi::c_int) < top_y as ::core::ffi::c_int {
                    top_y = y;
                }
                y = (y as ::core::ffi::c_int
                    + (*charFrame).disp.height as ::core::ffi::c_int) as COORD;
                if y as ::core::ffi::c_int > bot_y as ::core::ffi::c_int {
                    bot_y = y;
                }
                width = (width as ::core::ffi::c_int
                    + ((*charFrame).disp.width as ::core::ffi::c_int
                        + (*FontPtr).CharSpace as ::core::ffi::c_int)) as SIZE;
                if num_chars as ::core::ffi::c_int != 0 && next_ch < 0xffff as UniChar
                    && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                        != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                        & (*FontPtr).KernTab[next_ch as usize] as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int == 0
                {
                    width = (width as ::core::ffi::c_int
                        - (*FontPtr).KernAmount as ::core::ffi::c_int) as SIZE;
                }
            }
            let c2rust_fresh1 = pdelta;
            pdelta = pdelta.offset(1);
            *c2rust_fresh1 = (width as ::core::ffi::c_int
                - last_width as ::core::ffi::c_int) as BYTE;
        }
        if width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && {
                bot_y = (bot_y as ::core::ffi::c_int - top_y as ::core::ffi::c_int)
                    as COORD;
                bot_y as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            }
        {
            if *pdelta.offset(-1isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *pdelta.offset(-1isize) = (*pdelta.offset(-1isize)).wrapping_sub(1);
                width = (width as ::core::ffi::c_int
                    - (*FontPtr).CharSpace as ::core::ffi::c_int) as SIZE;
            }
            if (*lpText).align.0 == TEXT_ALIGN::ALIGN_LEFT.0 {
                (*pRect).corner.x = 0 as COORD;
            } else if (*lpText).align.0 == TEXT_ALIGN::ALIGN_CENTER.0 {
                (*pRect).corner.x = -((width as ::core::ffi::c_int >> resolutionFactor
                    >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            } else {
                (*pRect).corner.x = -(width as ::core::ffi::c_int) as COORD;
            }
            (*pRect).corner.y = top_y;
            (*pRect).extent.width = width as COORD;
            (*pRect).extent.height = bot_y;
            (*pRect).corner.x = ((*pRect).corner.x as ::core::ffi::c_int
                + (*lpText).baseline.x as ::core::ffi::c_int) as COORD;
            (*pRect).corner.y = ((*pRect).corner.y as ::core::ffi::c_int
                + (*lpText).baseline.y as ::core::ffi::c_int) as COORD;
            return BOOLEAN::TRUE;
        }
    }
    (*pRect).corner = (*lpText).baseline;
    (*pRect).extent.width = 0 as COORD;
    (*pRect).extent.height = 0 as COORD;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn _text_blt(
    mut pClipRect: *mut RECT,
    mut TextPtr: *mut TEXT,
    mut ctxOrigin: POINT,
) {
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut num_chars: COUNT = 0;
    let mut next_ch: UniChar = 0;
    let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut backing: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut mode: DrawMode = (*_pCurContext).Mode;
    FontPtr = _CurFontPtr;
    if FontPtr.is_null() {
        return;
    }
    backing = (*_pCurContext).FontBacking;
    if backing.is_null() {
        return;
    }
    origin.x = (*pClipRect).corner.x;
    origin.y = (*TextPtr).baseline.y;
    num_chars = (*TextPtr).CharCount;
    if num_chars as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    pStr = (*TextPtr).pStr as *const ::core::ffi::c_char;
    next_ch = getCharFromString(&raw mut pStr);
    if next_ch == '\0' as UniChar {
        num_chars = 0 as COUNT;
    }
    loop {
        let c2rust_fresh2 = num_chars;
        num_chars = num_chars.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut ch: UniChar = 0;
        let mut fontChar: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
        ch = next_ch;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        fontChar = getCharFrame(FontPtr, ch);
        if !fontChar.is_null() && (*fontChar).disp.width as ::core::ffi::c_int != 0 {
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            r.corner.x = (origin.x as ::core::ffi::c_int
                - (*fontChar).HotSpot.x as ::core::ffi::c_int) as COORD;
            r.corner.y = (origin.y as ::core::ffi::c_int
                - (*fontChar).HotSpot.y as ::core::ffi::c_int) as COORD;
            r.extent.width = (*fontChar).disp.width;
            r.extent.height = (*fontChar).disp.height;
            if BoxIntersect(&raw mut r, pClipRect, &raw mut r) != 0 {
                uqm_lua_checked!(TFB_Prim_FontChar(origin, fontChar, backing, mode, ctxOrigin), ());
            }
            origin.x = (origin.x as ::core::ffi::c_int
                + ((*fontChar).disp.width as ::core::ffi::c_int
                    + (*FontPtr).CharSpace as ::core::ffi::c_int)) as COORD;
            if num_chars as ::core::ffi::c_int != 0 && next_ch < 0xffff as UniChar
                && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                    & (*FontPtr).KernTab[next_ch as usize] as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int == 0
            {
                origin.x = (origin.x as ::core::ffi::c_int
                    - (*FontPtr).KernAmount as ::core::ffi::c_int) as COORD;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _text_blt_fade(
    mut pClipRect: *mut RECT,
    mut TextPtr: *mut TEXT,
    mut ctxOrigin: POINT,
    mut repair: FRAME,
    mut skip: *mut BOOLEAN,
) {
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut num_chars: COUNT = 0;
    let mut next_ch: UniChar = 0;
    let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut leading: SIZE = 0;
    let mut b_first: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut b_second: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut b_clear: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut mode: DrawMode = (*_pCurContext).Mode;
    FontPtr = _CurFontPtr;
    if !FontPtr.is_null() {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut w: SIZE = 0;
        let mut h: SIZE = 0;
        if GetContextFontDispHeight(&raw mut h).0 == 0
            || GetContextFontDispWidth(&raw mut w).0 == 0
        {
            return;
        }
        b_first = TFB_DrawImage_CreateForScreen(
            w as ::core::ffi::c_int,
            h as ::core::ffi::c_int,
            BOOLEAN::TRUE,
        );
        b_second = TFB_DrawImage_CreateForScreen(
            w as ::core::ffi::c_int,
            h as ::core::ffi::c_int,
            BOOLEAN::TRUE,
        );
        b_clear = TFB_DrawImage_CreateForScreen(
            w as ::core::ffi::c_int,
            h as ::core::ffi::c_int,
            BOOLEAN::TRUE,
        );
        r.corner.y = 0 as COORD;
        r.corner.x = r.corner.y;
        r.extent.width = w as COORD;
        r.extent.height = h as COORD;
        uqm_lua_checked!(TFB_DrawImage_Rect(
            &raw mut r,
            (*_pCurContext).BackGroundColor,
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            b_first,
        ), ());
        uqm_lua_checked!(TFB_DrawImage_Rect(
            &raw mut r,
            (*_pCurContext).ForeGroundColor,
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            b_second,
        ), ());
    } else {
        return
    }
    origin.x = (*pClipRect).corner.x;
    origin.y = (*TextPtr).baseline.y;
    GetContextFontLeading(&raw mut leading);
    num_chars = (*TextPtr).CharCount;
    if num_chars as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    pStr = (*TextPtr).pStr as *const ::core::ffi::c_char;
    next_ch = getCharFromString(&raw mut pStr);
    if next_ch == '\0' as UniChar {
        num_chars = 0 as COUNT;
    }
    loop {
        let c2rust_fresh3 = num_chars;
        num_chars = num_chars.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let mut ch: UniChar = 0;
        let mut fontChar: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
        while next_ch == ' ' as UniChar {
            fontChar = getCharFrame(FontPtr, next_ch);
            origin.x = (origin.x as ::core::ffi::c_int
                + ((*fontChar).disp.width as ::core::ffi::c_int
                    + (*FontPtr).CharSpace as ::core::ffi::c_int)) as COORD;
            next_ch = getCharFromString(&raw mut pStr);
            num_chars = num_chars.wrapping_sub(1);
        }
        ch = next_ch;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        fontChar = getCharFrame(FontPtr, ch);
        if !fontChar.is_null() && (*fontChar).disp.width as ::core::ffi::c_int != 0 {
            let mut r_0: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            r_0.corner.x = (origin.x as ::core::ffi::c_int
                - (*fontChar).HotSpot.x as ::core::ffi::c_int) as COORD;
            r_0.corner.y = (origin.y as ::core::ffi::c_int
                - (*fontChar).HotSpot.y as ::core::ffi::c_int) as COORD;
            r_0.extent.width = (*fontChar).disp.width;
            r_0.extent.height = (*fontChar).disp.height;
            if BoxIntersect(&raw mut r_0, pClipRect, &raw mut r_0) != 0 {
                if (*skip).0 == 0 {
                    uqm_lua_checked!(TFB_Prim_FontChar(origin, fontChar, b_first, mode, ctxOrigin), ());
                    uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_TEXT), ());
                    uqm_lua_checked!(SleepThread(
                        (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int)
                            as TimePeriod,
                    ), ());
                }
                BatchGraphics();
                if !repair.is_null() && (*skip).0 == 0 {
                    uqm_lua_checked!(TFB_DrawImage_Image(
                        (*repair).image,
                        -(r_0.corner.x as ::core::ffi::c_int),
                        -(r_0.corner.y as ::core::ffi::c_int),
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<TFB_ColorMap>(),
                        MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
                        b_clear,
                    ), ());
                    uqm_lua_checked!(TFB_Prim_FontChar(
                        origin,
                        fontChar,
                        b_clear,
                        MAKE_DRAW_MODE(DrawKind::DRAW_GRAYSCALE, 0xff as SWORD),
                        ctxOrigin,
                    ), ());
                }
                uqm_lua_checked!(TFB_Prim_FontChar(origin, fontChar, b_second, mode, ctxOrigin), ());
                UnbatchGraphics();
            }
            if next_ch == '\n' as UniChar || next_ch == '\r' as UniChar {
                origin.x = (*pClipRect).corner.x;
                origin.y = (origin.y as ::core::ffi::c_int
                    + leading as ::core::ffi::c_int) as COORD;
                (*pClipRect).extent.height = ((*pClipRect).extent.height
                    as ::core::ffi::c_int + leading as ::core::ffi::c_int) as COORD;
                next_ch = getCharFromString(&raw mut pStr);
                num_chars = num_chars.wrapping_sub(1);
            } else if next_ch != '\0' as UniChar {
                origin.x = (origin.x as ::core::ffi::c_int
                    + ((*fontChar).disp.width as ::core::ffi::c_int
                        + (*FontPtr).CharSpace as ::core::ffi::c_int)) as COORD;
                if num_chars as ::core::ffi::c_int != 0 && next_ch < 0xffff as UniChar
                    && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                        != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                        & (*FontPtr).KernTab[next_ch as usize] as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int == 0
                {
                    origin.x = (origin.x as ::core::ffi::c_int
                        - (*FontPtr).KernAmount as ::core::ffi::c_int) as COORD;
                }
            }
        }
        uqm_lua_checked!(UpdateInputState(), ());
        if CurrentInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0
            || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            *skip = BOOLEAN::TRUE;
        }
    }
    if !b_first.is_null() {
        TFB_DrawScreen_DeleteImage(b_first);
    }
    if !b_second.is_null() {
        TFB_DrawScreen_DeleteImage(b_second);
    }
    if !b_clear.is_null() {
        TFB_DrawScreen_DeleteImage(b_clear);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TextRectAlt(
    mut lpText: *mut TEXT,
    mut pRect: *mut RECT,
    mut pdelta: *mut BYTE,
    mut swap: BYTE,
    mut key: UniChar,
    mut AltFontPtr: FONT,
) -> BOOLEAN {
    let mut char_delta_array: [BYTE; 100] = [0; 100];
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    FontPtr = _CurFontPtr;
    if !FontPtr.is_null()
        && (*lpText).CharCount as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        let mut top_y: COORD = 0;
        let mut bot_y: COORD = 0;
        let mut width: SIZE = 0;
        let mut next_ch: UniChar = 0 as UniChar;
        let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut num_chars: COUNT = 0;
        num_chars = (*lpText).CharCount;
        if pdelta.is_null() {
            pdelta = &raw mut char_delta_array as *mut BYTE;
            if num_chars as ::core::ffi::c_int > 100 as ::core::ffi::c_int {
                num_chars = 100 as COUNT;
                (*lpText).CharCount = 100 as COUNT;
            }
        }
        top_y = 0 as COORD;
        bot_y = 0 as COORD;
        width = 0 as SIZE;
        pStr = (*lpText).pStr as *const ::core::ffi::c_char;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        loop {
            let c2rust_fresh5 = num_chars;
            num_chars = num_chars.wrapping_sub(1);
            if c2rust_fresh5 == 0 {
                break;
            }
            let mut ch: UniChar = 0;
            let mut last_width: SIZE = 0;
            let mut charFrame: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
            last_width = width;
            ch = next_ch;
            if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                next_ch = getCharFromString(&raw mut pStr);
                if next_ch == '\0' as UniChar {
                    (*lpText).CharCount = ((*lpText).CharCount as ::core::ffi::c_int
                        - num_chars as ::core::ffi::c_int) as COUNT;
                    num_chars = 0 as COUNT;
                }
            }
            if ch == key {
                charFrame = getCharFrame(AltFontPtr, ch);
                swap = (swap as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
                FontPtr = if swap as ::core::ffi::c_int != 0 {
                    AltFontPtr
                } else {
                    _CurFontPtr
                };
            } else {
                FontPtr = if swap as ::core::ffi::c_int != 0 {
                    AltFontPtr
                } else {
                    _CurFontPtr
                };
                charFrame = getCharFrame(FontPtr, ch);
            }
            if !charFrame.is_null() && (*charFrame).disp.width as ::core::ffi::c_int != 0
            {
                let mut y: COORD = 0;
                y = -((*charFrame).HotSpot.y as ::core::ffi::c_int) as COORD;
                if (y as ::core::ffi::c_int) < top_y as ::core::ffi::c_int {
                    top_y = y;
                }
                y = (y as ::core::ffi::c_int
                    + (*charFrame).disp.height as ::core::ffi::c_int) as COORD;
                if y as ::core::ffi::c_int > bot_y as ::core::ffi::c_int {
                    bot_y = y;
                }
                width = (width as ::core::ffi::c_int
                    + ((*charFrame).disp.width as ::core::ffi::c_int
                        + (*FontPtr).CharSpace as ::core::ffi::c_int)) as SIZE;
                if num_chars as ::core::ffi::c_int != 0 && next_ch < 0xffff as UniChar
                    && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                        != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                        & (*FontPtr).KernTab[next_ch as usize] as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int == 0
                {
                    width = (width as ::core::ffi::c_int
                        - (*FontPtr).KernAmount as ::core::ffi::c_int) as SIZE;
                }
            }
            let c2rust_fresh6 = pdelta;
            pdelta = pdelta.offset(1);
            *c2rust_fresh6 = (width as ::core::ffi::c_int
                - last_width as ::core::ffi::c_int) as BYTE;
        }
        if width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && {
                bot_y = (bot_y as ::core::ffi::c_int - top_y as ::core::ffi::c_int)
                    as COORD;
                bot_y as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            }
        {
            if *pdelta.offset(-1isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *pdelta.offset(-1isize) = (*pdelta.offset(-1isize)).wrapping_sub(1);
                width = (width as ::core::ffi::c_int
                    - (*FontPtr).CharSpace as ::core::ffi::c_int) as SIZE;
            }
            if (*lpText).align.0 == TEXT_ALIGN::ALIGN_LEFT.0 {
                (*pRect).corner.x = 0 as COORD;
            } else if (*lpText).align.0 == TEXT_ALIGN::ALIGN_CENTER.0 {
                (*pRect).corner.x = -((width as ::core::ffi::c_int >> resolutionFactor
                    >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            } else {
                (*pRect).corner.x = -(width as ::core::ffi::c_int) as COORD;
            }
            (*pRect).corner.y = top_y;
            (*pRect).extent.width = width as COORD;
            (*pRect).extent.height = bot_y;
            (*pRect).corner.x = ((*pRect).corner.x as ::core::ffi::c_int
                + (*lpText).baseline.x as ::core::ffi::c_int) as COORD;
            (*pRect).corner.y = ((*pRect).corner.y as ::core::ffi::c_int
                + (*lpText).baseline.y as ::core::ffi::c_int) as COORD;
            return BOOLEAN::TRUE;
        }
    }
    (*pRect).corner = (*lpText).baseline;
    (*pRect).extent.width = 0 as COORD;
    (*pRect).extent.height = 0 as COORD;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn _text_blt_alt(
    mut pClipRect: *mut RECT,
    mut TextPtr: *mut TEXT,
    mut ctxOrigin: POINT,
    mut swap: BYTE,
    mut AltFontPtr: FONT,
    mut key: UniChar,
) -> BYTE {
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut num_chars: COUNT = 0;
    let mut next_ch: UniChar = 0;
    let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut backing: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut stock: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut ext: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut mode: DrawMode = (*_pCurContext).Mode;
    let mut leading_step: BYTE = 0;
    FontPtr = _CurFontPtr;
    if FontPtr.is_null() {
        return 0 as BYTE;
    }
    stock = (*_pCurContext).FontBacking;
    if stock.is_null() {
        return 0 as BYTE;
    }
    if !AltFontPtr.is_null() {
        let mut w: SIZE = 0;
        let mut h: SIZE = 0;
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut color: Color = (*_pCurContext).ForeGroundColor;
        w = (*AltFontPtr).disp.width;
        h = (*AltFontPtr).disp.height;
        if w as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || h as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return 0 as BYTE;
        }
        ext = TFB_DrawImage_CreateForScreen(
            w as ::core::ffi::c_int,
            h as ::core::ffi::c_int,
            BOOLEAN::TRUE,
        );
        r.corner = MAKE_HOT_SPOT(0 as COORD, 0 as COORD) as POINT;
        r.extent = MAKE_EXTENT(w, h);
        uqm_lua_checked!(TFB_DrawImage_Rect(
            &raw mut r,
            color,
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            ext,
        ), 0);
    } else {
        return 0 as BYTE
    }
    origin.x = (*pClipRect).corner.x;
    origin.y = (*TextPtr).baseline.y;
    num_chars = (*TextPtr).CharCount;
    if num_chars as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as BYTE;
    }
    leading_step = ((*AltFontPtr).Leading as ::core::ffi::c_int
        - (*_CurFontPtr).Leading as ::core::ffi::c_int) as BYTE;
    pStr = (*TextPtr).pStr as *const ::core::ffi::c_char;
    next_ch = getCharFromString(&raw mut pStr);
    if next_ch == '\0' as UniChar {
        num_chars = 0 as COUNT;
    }
    backing = stock;
    loop {
        let c2rust_fresh4 = num_chars;
        num_chars = num_chars.wrapping_sub(1);
        if c2rust_fresh4 == 0 {
            break;
        }
        let mut ch: UniChar = 0;
        let mut fontChar: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
        ch = next_ch;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        if ch == key {
            fontChar = getCharFrame(AltFontPtr, ch);
            backing = ext;
            swap = (swap as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
            FontPtr = if swap as ::core::ffi::c_int != 0 {
                AltFontPtr
            } else {
                _CurFontPtr
            };
            origin.y = ((*TextPtr).baseline.y as ::core::ffi::c_int
                + leading_step as ::core::ffi::c_int) as COORD;
        } else {
            FontPtr = if swap as ::core::ffi::c_int != 0 {
                AltFontPtr
            } else {
                _CurFontPtr
            };
            fontChar = getCharFrame(FontPtr, ch);
            backing = if swap as ::core::ffi::c_int != 0 { ext } else { stock };
            origin.y = ((*TextPtr).baseline.y as ::core::ffi::c_int
                + if swap as ::core::ffi::c_int != 0 {
                    leading_step as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
        }
        if !fontChar.is_null() && (*fontChar).disp.width as ::core::ffi::c_int != 0 {
            let mut r_0: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            r_0.corner.x = (origin.x as ::core::ffi::c_int
                - (*fontChar).HotSpot.x as ::core::ffi::c_int) as COORD;
            r_0.corner.y = (origin.y as ::core::ffi::c_int
                - (*fontChar).HotSpot.y as ::core::ffi::c_int) as COORD;
            r_0.extent.width = (*fontChar).disp.width;
            r_0.extent.height = (*fontChar).disp.height;
            if BoxIntersect(&raw mut r_0, pClipRect, &raw mut r_0) != 0 {
                uqm_lua_checked!(TFB_Prim_FontChar(origin, fontChar, backing, mode, ctxOrigin), 0);
            }
            origin.x = (origin.x as ::core::ffi::c_int
                + ((*fontChar).disp.width as ::core::ffi::c_int
                    + (*FontPtr).CharSpace as ::core::ffi::c_int)) as COORD;
            if num_chars as ::core::ffi::c_int != 0 && next_ch < 0xffff as UniChar
                && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                && (*FontPtr).KernTab[ch as usize] as ::core::ffi::c_int
                    & (*FontPtr).KernTab[next_ch as usize] as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int == 0
            {
                origin.x = (origin.x as ::core::ffi::c_int
                    - (*FontPtr).KernAmount as ::core::ffi::c_int) as COORD;
            }
        }
    }
    if !ext.is_null() {
        TFB_DrawImage_Delete(ext);
    }
    return swap;
}
#[inline]
unsafe extern "C" fn getCharFrame(
    mut fontPtr: *mut FONT_DESC,
    mut ch: UniChar,
) -> *mut TFB_Char {
    let mut pageStart: UniChar = ch & 0xfffff800 as UniChar;
    let mut charIndex: size_t = 0;
    let mut page: *mut FONT_PAGE = (*fontPtr).fontPages;
    loop {
        if page.is_null() {
            return ::core::ptr::null_mut::<TFB_Char>();
        }
        if (*page).pageStart == pageStart {
            break;
        }
        page = (*page).next as *mut FONT_PAGE;
    }
    charIndex = ch.wrapping_sub((*page).firstChar) as size_t;
    if ch >= (*page).firstChar && charIndex < (*page).numChars
        && !(*(*page).charDesc.offset(charIndex as isize)).data.is_null()
    {
        return (*page).charDesc.offset(charIndex as isize)
    } else {
        return ::core::ptr::null_mut::<TFB_Char>()
    };
}
