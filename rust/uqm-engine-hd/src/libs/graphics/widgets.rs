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

// Rust translation of src/libs/graphics/widgets.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
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
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut optWindowType: DWORD;
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
        CreateCorners: BOOLEAN,
        CornerColor: Color,
    );
    static mut GameStrings: STRING;
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const WIDGET_EVENT_UP: Self = Self(0);
    pub const WIDGET_EVENT_DOWN: Self = Self(1);
    pub const WIDGET_EVENT_LEFT: Self = Self(2);
    pub const WIDGET_EVENT_RIGHT: Self = Self(3);
    pub const WIDGET_EVENT_SELECT: Self = Self(4);
    pub const WIDGET_EVENT_CANCEL: Self = Self(5);
    pub const WIDGET_EVENT_DELETE: Self = Self(6);
    pub const NUM_WIDGET_EVENTS: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WIDGET_TYPE(pub ::core::ffi::c_uint);
impl WIDGET_TYPE {
    pub const WIDGET_TYPE_MENU_SCREEN: Self = Self(0);
    pub const WIDGET_TYPE_CHOICE: Self = Self(1);
    pub const WIDGET_TYPE_BUTTON: Self = Self(2);
    pub const WIDGET_TYPE_LABEL: Self = Self(3);
    pub const WIDGET_TYPE_SLIDER: Self = Self(4);
    pub const WIDGET_TYPE_TEXTENTRY: Self = Self(5);
    pub const WIDGET_TYPE_CONTROLENTRY: Self = Self(6);
    pub const WIDGET_TYPE_MENUCONTROLENTRY: Self = Self(7);
    pub const NUM_WIDGET_TYPES: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
}
pub type WIDGET = _widget;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_menu_screen {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub title: *const ::core::ffi::c_char,
    pub subtitle: *const ::core::ffi::c_char,
    pub bgStamp: STAMP,
    pub num_children: ::core::ffi::c_int,
    pub child: *mut *mut _widget,
    pub highlighted: ::core::ffi::c_int,
}
pub type WIDGET_MENU_SCREEN = _widget_menu_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHOICE_OPTION {
    pub optname: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_choice {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub category: *const ::core::ffi::c_char,
    pub numopts: ::core::ffi::c_int,
    pub options: *mut CHOICE_OPTION,
    pub selected: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub choice_num: ::core::ffi::c_int,
    pub onChange: Option<
        unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> (),
    >,
}
pub type WIDGET_CHOICE = _widget_choice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_button {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub name: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type WIDGET_BUTTON = _widget_button;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_label {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub line_count: ::core::ffi::c_int,
    pub lines: *mut *const ::core::ffi::c_char,
}
pub type WIDGET_LABEL = _widget_label;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_slider {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub draw_value: Option<
        unsafe extern "C" fn(
            *mut _widget_slider,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub min: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub step: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
    pub category: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_slider) -> ()>,
}
pub type WIDGET_SLIDER = _widget_slider;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WIDGET_TEXTENTRY_STATE(pub ::core::ffi::c_uint);
impl WIDGET_TEXTENTRY_STATE {
    pub const WTE_NORMAL: Self = Self(0);
    pub const WTE_EDITING: Self = Self(1);
    pub const WTE_BLOCKCUR: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_textentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub handleEventSelect: Option<
        unsafe extern "C" fn(*mut _widget_textentry) -> ::core::ffi::c_int,
    >,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_textentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub value: [::core::ffi::c_char; 31],
    pub maxlen: ::core::ffi::c_int,
    pub state: WIDGET_TEXTENTRY_STATE,
    pub cursor_pos: ::core::ffi::c_int,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type WIDGET_TEXTENTRY = _widget_textentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_controlentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>,
    pub onDelete: Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub controlindex: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub controlname: [[::core::ffi::c_char; 16]; 4],
    pub current_page: ::core::ffi::c_int,
    pub num_pages: ::core::ffi::c_int,
}
pub type WIDGET_CONTROLENTRY = _widget_controlentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_menucontrolentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_menucontrolentry) -> ()>,
    pub onDelete: Option<unsafe extern "C" fn(*mut _widget_menucontrolentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub controlindex: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub controlname: [[::core::ffi::c_char; 16]; 6],
    pub current_page: ::core::ffi::c_int,
    pub num_pages: ::core::ffi::c_int,
}
pub type WIDGET_MENUCONTROLENTRY = _widget_menucontrolentry;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[no_mangle]
pub static mut widget_focus: *mut WIDGET = ::core::ptr::null_mut::<WIDGET>();
static mut win_bg_clr: Color = Color {
    r: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    a: 0xff as BYTE,
};
static mut win_medium_clr: Color = Color {
    r: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    b: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    a: 0xff as BYTE,
};
static mut win_dark_clr: Color = Color {
    r: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
    a: 0xff as BYTE,
};
static mut cur_font: FONT = ::core::ptr::null_mut::<FONT_DESC>();
static mut offset_t: COUNT = 0 as COUNT;
static mut offset_b: COUNT = 0;
static mut arrow_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub unsafe extern "C" fn ResetOffset() {
    offset_t = 0 as COUNT;
    offset_b = (if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        10 as ::core::ffi::c_int
    } else {
        13 as ::core::ffi::c_int
    }) as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn LoadArrows() {
    if arrow_frame.is_null() || optRequiresReload.0 != 0 {
        arrow_frame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.setupmenuarrows\0".as_ptr() as RESOURCE), ())
                as DRAWABLE,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseArrows() {
    if !arrow_frame.is_null() {
        DestroyDrawable(ReleaseDrawable(arrow_frame));
        arrow_frame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawShadowedBox(
    mut r: *mut RECT,
    mut bg: Color,
    mut dark: Color,
    mut medium: Color,
) {
    uqm_lua_checked!(DrawStarConBox(
        r,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
        dark,
        medium,
        BOOLEAN::TRUE,
        bg,
        BOOLEAN::TRUE,
        buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawLabelAsWindow(
    mut label: *mut WIDGET_LABEL,
    mut windowRect: *mut RECT,
) {
    let mut oldfg: Color = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut win_w: ::core::ffi::c_int = 0;
    let mut win_h: ::core::ffi::c_int = 0;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    win_h = uqm_lua_checked!((*label).height.expect("non-null function pointer")(label as *mut _widget), ())
        + ((16 as ::core::ffi::c_int) << resolutionFactor);
    win_w = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*label).line_count {
        let mut len: ::core::ffi::c_int = utf8StringCount(
            *(*label).lines.offset(i as isize),
        ) as ::core::ffi::c_int;
        if len > win_w {
            win_w = len << resolutionFactor;
        }
        i += 1;
    }
    win_w = win_w * 6 as ::core::ffi::c_int
        + ((16 as ::core::ffi::c_int) << resolutionFactor);
    BatchGraphics();
    r.corner.x = ((((CanvasWidth >> resolutionFactor) - (win_w >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = ((((CanvasHeight >> resolutionFactor) - (win_h >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (win_w + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (win_h + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawShadowedBox(&raw mut r, win_bg_clr, win_dark_clr, win_medium_clr), ());
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((r.extent.width as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((16 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    i = 0 as ::core::ffi::c_int;
    while i < (*label).line_count {
        t.pStr = *(*label).lines.offset(i as isize) as *const UNICODE;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i += 1;
    }
    UnbatchGraphics();
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldfg);
    if !windowRect.is_null() {
        (*windowRect).corner.x = r.corner.x;
        (*windowRect).corner.y = r.corner.y;
        (*windowRect).extent.width = r.extent.width;
        (*windowRect).extent.height = r.extent.height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Widget_SetWindowColors(
    mut bg: Color,
    mut dark: Color,
    mut medium: Color,
) {
    win_bg_clr = bg;
    win_dark_clr = dark;
    win_medium_clr = medium;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_SetFont(mut newFont: FONT) -> FONT {
    let mut oldFont: FONT = cur_font;
    cur_font = newFont;
    return oldFont;
}
unsafe extern "C" fn Widget_DrawToolTips(
    mut numlines: ::core::ffi::c_int,
    mut tips: *mut *const ::core::ffi::c_char,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut oldtext: Color = SetContextForeGroundColor(
        colorFromRgb15(0xd2 as BYTE, 0xb4 as BYTE, 0x8c as BYTE),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut amperBang: *const UNICODE = b"&!\0".as_ptr() as *const UNICODE;
    let abSize: size_t = strlen(amperBang);
    let mut warning: BOOLEAN = BOOLEAN::FALSE;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((8 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (CanvasWidth - ((4 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    r.extent.height = (CanvasHeight + ((3 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + (r.extent.height as ::core::ffi::c_int
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor) * numlines)) as COORD;
    i = 0 as ::core::ffi::c_int;
    while i < numlines {
        t.pStr = *tips.offset(i as isize) as *const UNICODE;
        warning = BOOLEAN(
            (strncmp(amperBang, t.pStr, abSize) == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if warning.0 != 0 {
            t.pStr = t.pStr.offset(abSize as isize);
            SetContextForeGroundColor(
                buildColorRgba(0xb1 as BYTE, 0x27 as BYTE, 0x47 as BYTE, 0xff as BYTE),
            );
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        if warning.0 != 0 {
            SetContextForeGroundColor(
                colorFromRgb15(0xd2 as BYTE, 0xb4 as BYTE, 0x8c as BYTE),
            );
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i += 1;
    }
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawMenuScreen(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut title: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut widget_index: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut widget_y: ::core::ffi::c_int = 0;
    let mut on_screen: ::core::ffi::c_int = 0;
    let mut self_0: *mut WIDGET_MENU_SCREEN = _self as *mut WIDGET_MENU_SCREEN;
    if offset_b == 0 {
        offset_b = (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            10 as ::core::ffi::c_int
        } else {
            13 as ::core::ffi::c_int
        }) as COUNT;
    }
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (CanvasWidth - ((4 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    r.extent.height = (CanvasHeight - ((4 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    title = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    uqm_lua_checked!(DrawStamp(&raw mut (*self_0).bgStamp), ());
    oldtext = SetContextForeGroundColor(title);
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = (*self_0).title as *const UNICODE;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = (*self_0).subtitle as *const UNICODE;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    height = 0 as ::core::ffi::c_int;
    widget_index = 0 as ::core::ffi::c_int;
    while widget_index < (*self_0).num_children {
        let mut child: *mut WIDGET = *(*self_0).child.offset(widget_index as isize);
        if widget_index
            <= if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                10 as ::core::ffi::c_int
            } else {
                13 as ::core::ffi::c_int
            }
        {
            height
                += uqm_lua_checked!(Some((*child).height.expect("non-null function pointer"))
                    .expect("non-null function pointer")(child), ());
            height += (5 as ::core::ffi::c_int) << resolutionFactor;
        }
        widget_index += 1;
    }
    height -= (8 as ::core::ffi::c_int) << resolutionFactor;
    widget_y = CanvasHeight - height >> 1 as ::core::ffi::c_int;
    if (*self_0).highlighted > offset_b as ::core::ffi::c_int - 3 as ::core::ffi::c_int
        && (offset_b as ::core::ffi::c_int)
            < (*self_0).num_children - 1 as ::core::ffi::c_int
    {
        if (*self_0).highlighted == (*self_0).num_children - 1 as ::core::ffi::c_int
            && offset_t as ::core::ffi::c_int
                != (*self_0).highlighted
                    - (if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        10 as ::core::ffi::c_int
                    } else {
                        13 as ::core::ffi::c_int
                    } - 3 as ::core::ffi::c_int)
        {
            offset_t = (offset_t as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as COUNT;
            offset_b = (offset_b as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as COUNT;
        } else {
            offset_b = ((*self_0).highlighted + 3 as ::core::ffi::c_int) as COUNT;
            if offset_b as ::core::ffi::c_int
                > (*self_0).num_children - 1 as ::core::ffi::c_int
            {
                offset_b = ((*self_0).num_children - 1 as ::core::ffi::c_int) as COUNT;
                offset_t = (offset_b as ::core::ffi::c_int
                    - if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        10 as ::core::ffi::c_int
                    } else {
                        13 as ::core::ffi::c_int
                    }) as COUNT;
            } else {
                offset_t = ((*self_0).highlighted
                    - (if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        10 as ::core::ffi::c_int
                    } else {
                        13 as ::core::ffi::c_int
                    } - 3 as ::core::ffi::c_int)) as COUNT;
            }
        }
    }
    if (*self_0).highlighted < offset_t as ::core::ffi::c_int + 3 as ::core::ffi::c_int
        && offset_t as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        if (*self_0).highlighted < offset_t as ::core::ffi::c_int
            && offset_t as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            offset_t = (offset_t as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT;
            offset_b = (offset_b as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT;
        } else {
            offset_t = ((*self_0).highlighted - 3 as ::core::ffi::c_int) as COUNT;
            if offset_t as ::core::ffi::c_int > 65500 as ::core::ffi::c_int {
                offset_t = 0 as COUNT;
                offset_b = (offset_t as ::core::ffi::c_int
                    + if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        10 as ::core::ffi::c_int
                    } else {
                        13 as ::core::ffi::c_int
                    }) as COUNT;
            } else {
                offset_b = ((*self_0).highlighted
                    + (if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        10 as ::core::ffi::c_int
                    } else {
                        13 as ::core::ffi::c_int
                    } - 3 as ::core::ffi::c_int)) as COUNT;
            }
        }
    }
    if (*self_0).num_children
        > if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            10 as ::core::ffi::c_int
        } else {
            13 as ::core::ffi::c_int
        }
    {
        let mut arr: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        arr.origin.x = ((309 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        if offset_t as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            arr.frame = SetAbsFrameIndex(arrow_frame, 0 as COUNT);
            arr.origin.y = (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                10 as ::core::ffi::c_int
            } else {
                11 as ::core::ffi::c_int
            } << resolutionFactor) as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut arr), ());
        }
        if offset_b as ::core::ffi::c_int
            != (*self_0).num_children - 1 as ::core::ffi::c_int
        {
            arr.frame = SetAbsFrameIndex(arrow_frame, 1 as COUNT);
            arr.origin.y = (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                178 as ::core::ffi::c_int
            } else {
                218 as ::core::ffi::c_int
            } << resolutionFactor) as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut arr), ());
        }
    }
    on_screen = (*self_0).num_children;
    if (offset_b as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        < (*self_0).num_children
    {
        on_screen = offset_b as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    widget_index = offset_t as ::core::ffi::c_int;
    while widget_index < on_screen {
        let mut c: *mut WIDGET = *(*self_0).child.offset(widget_index as isize);
        uqm_lua_checked!(Some((*c).draw.expect("non-null function pointer"))
            .expect("non-null function pointer")(c, 0 as ::core::ffi::c_int, widget_y), ());
        widget_y
            += uqm_lua_checked!(Some((*c).height.expect("non-null function pointer"))
                .expect("non-null function pointer")(c), ())
                + ((5 as ::core::ffi::c_int) << resolutionFactor);
        widget_index += 1;
    }
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawChoice(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_CHOICE = _self as *mut WIDGET_CHOICE;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut default_color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut enabled: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut disabled: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut canbe: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    default_color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    enabled = colorFromRgb15(0 as BYTE, 0x18 as BYTE, 0 as BYTE);
    disabled = colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0xf as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    canbe = buildColorRgba(0xdf as BYTE, 0x6a as BYTE, 0x13 as BYTE, 0xff as BYTE);
    t.baseline.x = (x
        + (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
            - 7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.x = (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
        + 7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    i = 0 as ::core::ffi::c_int;
    while i < (*self_0).numopts {
        if i == (*self_0).selected && widget_focus != _self {
            t.pStr = (*(*self_0).options.offset(i as isize)).optname as *const UNICODE;
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
        }
        if widget_focus == _self && (*self_0).highlighted == i {
            if i == (*self_0).selected {
                SetContextForeGroundColor(selected);
            } else {
                SetContextForeGroundColor(canbe);
            }
            uqm_lua_checked!(Widget_DrawToolTips(
                3 as ::core::ffi::c_int,
                &raw mut (*(*self_0).options.offset(i as isize)).tooltip
                    as *mut *const ::core::ffi::c_char,
            ), ());
            t.pStr = (*(*self_0).options.offset(i as isize)).optname as *const UNICODE;
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
            let mut arr: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            arr.origin.x = (uqm_lua_checked!(font_GetTextRect(&raw mut t), ()).corner.x as ::core::ffi::c_int
                - ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            arr.origin.y = t.baseline.y;
            arr.frame = SetAbsFrameIndex(arrow_frame, 2 as COUNT);
            uqm_lua_checked!(DrawStamp(&raw mut arr), ());
            arr.origin.x = (uqm_lua_checked!(font_GetTextRect(&raw mut t), ()).corner.x as ::core::ffi::c_int
                + uqm_lua_checked!(font_GetTextRect(&raw mut t), ()).extent.width as ::core::ffi::c_int
                + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            arr.frame = SetAbsFrameIndex(arrow_frame, 3 as COUNT);
            uqm_lua_checked!(DrawStamp(&raw mut arr), ());
            let mut d: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut c: COUNT = 0;
            d.extent.width = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            d.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            d.corner.x = (t.baseline.x as ::core::ffi::c_int
                - ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            d.corner.y = (t.baseline.y as ::core::ffi::c_int
                + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            c = 0 as COUNT;
            while (c as ::core::ffi::c_int) < (*self_0).numopts {
                d.corner.x = (d.corner.x as ::core::ffi::c_int
                    + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                if c as ::core::ffi::c_int == (*self_0).highlighted {
                    SetContextForeGroundColor(enabled);
                } else {
                    SetContextForeGroundColor(disabled);
                }
                uqm_lua_checked!(DrawFilledRectangle(&raw mut d), ());
                c = c.wrapping_add(1);
            }
        }
        i += 1;
    }
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawButton(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_BUTTON = _self as *mut WIDGET_BUTTON;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE);
    t.baseline.x = (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).name as *const UNICODE;
    if widget_focus == _self {
        uqm_lua_checked!(Widget_DrawToolTips(
            3 as ::core::ffi::c_int,
            &raw mut (*self_0).tooltip as *mut *const ::core::ffi::c_char,
        ), ());
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(inactive);
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawLabel(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_LABEL = _self as *mut WIDGET_LABEL;
    let mut oldtext: Color = SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0x77 as BYTE, 0x77 as BYTE, 0xff as BYTE),
    );
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut amperScore: *const UNICODE = b"&_\0".as_ptr() as *const UNICODE;
    let mut amperBang: *const UNICODE = b"&!\0".as_ptr() as *const UNICODE;
    let asSize: size_t = strlen(amperScore);
    let mut underline: BOOLEAN = BOOLEAN::FALSE;
    let mut warning: BOOLEAN = BOOLEAN::FALSE;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    t.baseline.x = (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    i = 0 as ::core::ffi::c_int;
    while i < (*self_0).line_count {
        t.pStr = *(*self_0).lines.offset(i as isize) as *const UNICODE;
        underline = BOOLEAN(
            (strncmp(amperScore, t.pStr, asSize) == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        warning = BOOLEAN(
            (strncmp(amperBang, t.pStr, asSize) == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if underline.0 != 0 {
            t.pStr = t.pStr.offset(asSize as isize);
        }
        if warning.0 != 0 {
            t.pStr = t.pStr.offset(asSize as isize);
            SetContextForeGroundColor(
                buildColorRgba(0xb1 as BYTE, 0x27 as BYTE, 0x47 as BYTE, 0xff as BYTE),
            );
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if warning.0 != 0 {
            SetContextForeGroundColor(
                buildColorRgba(0 as BYTE, 0x77 as BYTE, 0x77 as BYTE, 0xff as BYTE),
            );
        }
        if underline.0 != 0 {
            r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
        i += 1;
    }
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawSlider(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_SLIDER = _self as *mut WIDGET_SLIDER;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut default_color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
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
    let mut tick: ::core::ffi::c_int = (CanvasWidth - x) / 8 as ::core::ffi::c_int;
    let mut slider_width: ::core::ffi::c_int = 0;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    default_color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    t.baseline.x = (x
        + (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
            - 7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        uqm_lua_checked!(Widget_DrawToolTips(
            3 as ::core::ffi::c_int,
            &raw mut (*self_0).tooltip as *mut *const ::core::ffi::c_char,
        ), ());
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    r.corner.x = (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
        + 7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (2 as ::core::ffi::c_int * tick) as COORD;
    slider_width = r.corner.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.extent.width = ((3 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((8 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int
        - ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.x = ((((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
        + 7 as ::core::ffi::c_int) << resolutionFactor)
        + 2 as ::core::ffi::c_int * tick * ((*self_0).value - (*self_0).min)
            / ((*self_0).max - (*self_0).min)
        - ((3 as ::core::ffi::c_int) << resolutionFactor >> 1 as ::core::ffi::c_int))
        as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    uqm_lua_checked!(Some((*self_0).draw_value.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(self_0, slider_width, t.baseline.y as ::core::ffi::c_int), ());
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_Slider_DrawValue(
    mut self_0: *mut WIDGET_SLIDER,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buffer: [::core::ffi::c_char; 16] = [0; 16];
    sprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        (*self_0).value,
    );
    t.baseline.x = (x + ((14 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buffer as *mut ::core::ffi::c_char;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawTextEntry(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_TEXTENTRY = _self as *mut WIDGET_TEXTENTRY;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut default_color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut editing: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    default_color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE);
    editing = buildColorRgba(0 as BYTE, 0x77 as BYTE, 0x77 as BYTE, 0xff as BYTE);
    BatchGraphics();
    t.baseline.x = (x
        + (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
            - 7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        uqm_lua_checked!(Widget_DrawToolTips(
            3 as ::core::ffi::c_int,
            &raw mut (*self_0).tooltip as *mut *const ::core::ffi::c_char,
        ), ());
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        - t.baseline.x as ::core::ffi::c_int) as COORD;
    (*self_0)
        .value[(31 as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as usize] = 0
        as ::core::ffi::c_char;
    t.baseline.y = y as COORD;
    t.CharCount = utf8StringCount(&raw mut (*self_0).value as *mut ::core::ffi::c_char)
        as COUNT;
    t.pStr = &raw mut (*self_0).value as *mut ::core::ffi::c_char;
    if (*self_0).state.0 & WIDGET_TEXTENTRY_STATE::WTE_EDITING.0 == 0 {
        t.baseline.x = (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        if widget_focus == _self {
            oldtext = SetContextForeGroundColor(selected);
        } else {
            oldtext = SetContextForeGroundColor(inactive);
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    } else {
        let mut i: COUNT = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 31] = [0; 31];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut leading: SIZE = 0;
        t.baseline.x = (((CanvasWidth >> resolutionFactor) / 2 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        GetContextFontLeading(&raw mut leading);
        r.corner.x = (t.baseline.x as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (t.baseline.y as ::core::ffi::c_int - leading as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (CanvasWidth - r.corner.x as ::core::ffi::c_int
            - ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = (leading as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        TextRect(&raw mut t, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        oldtext = SetContextForeGroundColor(selected);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        pchar_deltas = &raw mut char_deltas as *mut BYTE;
        i = (*self_0).cursor_pos as COUNT;
        while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let c2rust_fresh0 = pchar_deltas;
            pchar_deltas = pchar_deltas.offset(1);
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + *c2rust_fresh0 as SIZE as ::core::ffi::c_int) as COORD;
            i = i.wrapping_sub(1);
        }
        if (*self_0).cursor_pos < t.CharCount as ::core::ffi::c_int {
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        if (*self_0).state.0 & WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0 != 0 {
            r.corner.y = r.corner.y;
            r.extent.height = r.extent.height;
            if (*self_0).cursor_pos == t.CharCount as ::core::ffi::c_int {
                r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    }) as COORD;
            } else if (*self_0).cursor_pos + 1 as ::core::ffi::c_int
                == t.CharCount as ::core::ffi::c_int
            {
                r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    }) as COORD;
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            } else {
                r.extent.width = *pchar_deltas as SIZE as COORD;
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            }
        } else {
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            r.extent.height = (r.extent.height as ::core::ffi::c_int
                - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            if (*self_0).cursor_pos == t.CharCount as ::core::ffi::c_int {
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    }) as COORD;
            }
        }
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextForeGroundColor(editing);
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
    UnbatchGraphics();
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawControlEntry(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_CONTROLENTRY = _self as *mut WIDGET_CONTROLENTRY;
    let mut oldcolor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
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
    let mut i: ::core::ffi::c_int = 0;
    let mut home_x: ::core::ffi::c_int = 0;
    let mut start_slot: ::core::ffi::c_int = 0;
    let mut col_size: [::core::ffi::c_int; 2] = [0 as ::core::ffi::c_int; 2];
    let mut num_pages: ::core::ffi::c_int = (*self_0).num_pages;
    let mut offset: ::core::ffi::c_int = (5 as ::core::ffi::c_int) << resolutionFactor;
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    oldFontEffect = SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    oldcolor = SetContextForeGroundColor(selected);
    t.baseline.x = (x + CanvasWidth / 5 as ::core::ffi::c_int + offset) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus != _self {
        SetContextForeGroundColor(
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
        );
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    home_x = ((t.baseline.x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) - offset;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    start_slot = (*self_0).current_page * 2 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let mut slot: ::core::ffi::c_int = start_slot + i;
        t.baseline.x = (home_x
            + i % 3 as ::core::ffi::c_int * (CanvasWidth / 3 as ::core::ffi::c_int))
            as COORD;
        t.pStr = &raw mut *(&raw mut (*self_0).controlname
            as *mut [::core::ffi::c_char; 16])
            .offset(slot as isize) as *mut ::core::ffi::c_char;
        if widget_focus == _self && (*self_0).highlighted == i {
            SetContextForeGroundColor(selected);
        } else if *t.pStr.offset(0isize) == 0 {
            SetContextForeGroundColor(
                buildColorRgba(0xb1 as BYTE, 0x27 as BYTE, 0x47 as BYTE, 0xff as BYTE),
            );
        } else {
            SetContextForeGroundColor(
                colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE),
            );
        }
        if *t.pStr.offset(0isize) == 0 {
            t.pStr = b"---\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        col_size[i as usize] = t.baseline.x as ::core::ffi::c_int;
        i += 1;
    }
    if widget_focus == _self && num_pages > 1 as ::core::ffi::c_int {
        let mut arrow: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        let mut arrow_buffer: ::core::ffi::c_int = (38 as ::core::ffi::c_int)
            << resolutionFactor;
        GetFrameRect(SetAbsFrameIndex(arrow_frame, 2 as COUNT), &raw mut r);
        if (*self_0).current_page > 0 as ::core::ffi::c_int {
            arrow.frame = SetAbsFrameIndex(arrow_frame, 2 as COUNT);
            arrow.origin.x = (col_size[0usize] - arrow_buffer
                - r.extent.width as ::core::ffi::c_int) as COORD;
            arrow.origin.y = y as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut arrow), ());
        }
        if (*self_0).current_page < num_pages - 1 as ::core::ffi::c_int {
            arrow.frame = SetAbsFrameIndex(arrow_frame, 3 as COUNT);
            arrow.origin.x = (col_size[1usize] + arrow_buffer
                + r.extent.width as ::core::ffi::c_int) as COORD;
            arrow.origin.y = y as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut arrow), ());
        }
    }
    if widget_focus == _self && num_pages > 1 as ::core::ffi::c_int {
        let mut rect_mid: ::core::ffi::c_int = 0;
        let mut rect_width: ::core::ffi::c_int = 0;
        let mut col_mid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rect_gap: ::core::ffi::c_int = (4 as ::core::ffi::c_int)
            << resolutionFactor;
        let mut leading: SIZE = 0;
        let mut height: ::core::ffi::c_int = 0;
        let mut corrected_y: ::core::ffi::c_int = 0;
        let mut index: ::core::ffi::c_int = 0;
        let mut current: *mut WIDGET_CONTROLENTRY = ::core::ptr::null_mut::<
            WIDGET_CONTROLENTRY,
        >();
        let mut buf: [::core::ffi::c_char; 12] = [0; 12];
        r.extent.width = ((16 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.x = 0 as COORD;
        rect_width = r.extent.width as ::core::ffi::c_int * num_pages
            + rect_gap * 2 as ::core::ffi::c_int;
        rect_mid = rect_width >> 1 as ::core::ffi::c_int;
        if col_size[0usize] > 0 as ::core::ffi::c_int
            && col_size[1usize] > 0 as ::core::ffi::c_int
        {
            col_mid = (col_size[1usize] - col_size[0usize] >> 1 as ::core::ffi::c_int)
                + col_size[0usize];
            r.corner.x = (col_mid - rect_mid) as COORD;
        }
        GetContextFontLeading(&raw mut leading);
        current = _self as *mut WIDGET_CONTROLENTRY;
        index = (*current).controlindex;
        height = uqm_lua_checked!(Some((*_self).height.expect("non-null function pointer"))
            .expect("non-null function pointer")(_self), ());
        corrected_y = leading as ::core::ffi::c_int
            + index * (height + ((5 as ::core::ffi::c_int) << resolutionFactor));
        r.corner.y = (t.baseline.y as ::core::ffi::c_int
            - r.extent.height as ::core::ffi::c_int - corrected_y) as COORD;
        i = 0 as ::core::ffi::c_int;
        while i < num_pages {
            if i > 0 as ::core::ffi::c_int {
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    + (r.extent.width as ::core::ffi::c_int + rect_gap)) as COORD;
            }
            SetContextForeGroundColor(
                if i == (*self_0).current_page {
                    colorFromRgb15(0 as BYTE, 0x18 as BYTE, 0 as BYTE)
                } else {
                    colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0xf as BYTE)
                },
            );
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            i += 1;
        }
        SetContextForeGroundColor(
            colorFromRgb15(0xd2 as BYTE, 0xb4 as BYTE, 0x8c as BYTE),
        );
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 12]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                        + 37 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            (*self_0).current_page + 1 as ::core::ffi::c_int,
            num_pages,
        );
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            - (height >> 1 as ::core::ffi::c_int)) as COORD;
        t.baseline.x = col_mid as COORD;
        t.pStr = &raw mut buf as *mut ::core::ffi::c_char;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldcolor);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_DrawMenuControlEntry(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_MENUCONTROLENTRY = _self as *mut WIDGET_MENUCONTROLENTRY;
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut oldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
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
    let mut i: ::core::ffi::c_int = 0;
    let mut home_x: ::core::ffi::c_int = 0;
    let mut start_slot: ::core::ffi::c_int = 0;
    let mut col_size: [::core::ffi::c_int; 2] = [0 as ::core::ffi::c_int; 2];
    let mut num_pages: ::core::ffi::c_int = (*self_0).num_pages;
    let mut offset: ::core::ffi::c_int = (22 as ::core::ffi::c_int) << resolutionFactor;
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    if !cur_font.is_null() {
        OldFont = SetContextFont(cur_font);
    }
    oldFontEffect = SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    OldColor = SetContextForeGroundColor(selected);
    t.baseline.x = (x + CanvasWidth / 5 as ::core::ffi::c_int + offset) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus != _self {
        SetContextForeGroundColor(
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
        );
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    home_x = ((t.baseline.x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) - offset;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    start_slot = (*self_0).current_page * 2 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let mut slot: ::core::ffi::c_int = start_slot + i;
        t.baseline.x = (home_x
            + i % 3 as ::core::ffi::c_int * (CanvasWidth / 3 as ::core::ffi::c_int))
            as COORD;
        t.pStr = &raw mut *(&raw mut (*self_0).controlname
            as *mut [::core::ffi::c_char; 16])
            .offset(slot as isize) as *mut ::core::ffi::c_char;
        if widget_focus == _self && (*self_0).highlighted == i {
            let mut tips: [*const ::core::ffi::c_char; 3] = [
                b"Press Return to edit\0".as_ptr() as *const ::core::ffi::c_char,
                b"or Delete to remove\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ];
            uqm_lua_checked!(Widget_DrawToolTips(
                2 as ::core::ffi::c_int,
                &raw mut tips as *mut *const ::core::ffi::c_char,
            ), ());
            SetContextForeGroundColor(selected);
        } else if *t.pStr.offset(0isize) == 0 {
            SetContextForeGroundColor(
                buildColorRgba(0xb1 as BYTE, 0x27 as BYTE, 0x47 as BYTE, 0xff as BYTE),
            );
        } else {
            SetContextForeGroundColor(
                colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE),
            );
        }
        if *t.pStr.offset(0isize) == 0 {
            t.pStr = b"---\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        col_size[i as usize] = t.baseline.x as ::core::ffi::c_int;
        i += 1;
    }
    if widget_focus == _self && num_pages > 1 as ::core::ffi::c_int {
        let mut arrow: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        let mut arrow_buffer: ::core::ffi::c_int = (38 as ::core::ffi::c_int)
            << resolutionFactor;
        GetFrameRect(SetAbsFrameIndex(arrow_frame, 2 as COUNT), &raw mut r);
        if (*self_0).current_page > 0 as ::core::ffi::c_int {
            arrow.frame = SetAbsFrameIndex(arrow_frame, 2 as COUNT);
            arrow.origin.x = (col_size[0usize] - arrow_buffer
                - r.extent.width as ::core::ffi::c_int) as COORD;
            arrow.origin.y = y as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut arrow), ());
        }
        if (*self_0).current_page < num_pages - 1 as ::core::ffi::c_int {
            arrow.frame = SetAbsFrameIndex(arrow_frame, 3 as COUNT);
            arrow.origin.x = (col_size[1usize] + arrow_buffer
                + r.extent.width as ::core::ffi::c_int) as COORD;
            arrow.origin.y = y as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut arrow), ());
        }
    }
    if widget_focus == _self && num_pages > 1 as ::core::ffi::c_int {
        let mut rect_mid: ::core::ffi::c_int = 0;
        let mut rect_width: ::core::ffi::c_int = 0;
        let mut col_mid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rect_gap: ::core::ffi::c_int = (4 as ::core::ffi::c_int)
            << resolutionFactor;
        let mut leading: SIZE = 0;
        let mut height: ::core::ffi::c_int = 0;
        let mut corrected_y: ::core::ffi::c_int = 0;
        let mut index: ::core::ffi::c_int = 0;
        let mut current: *mut WIDGET_MENUCONTROLENTRY = ::core::ptr::null_mut::<
            WIDGET_MENUCONTROLENTRY,
        >();
        let mut buf: [::core::ffi::c_char; 12] = [0; 12];
        r.extent.width = ((16 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.x = 0 as COORD;
        rect_width = r.extent.width as ::core::ffi::c_int * num_pages
            + rect_gap * 2 as ::core::ffi::c_int;
        rect_mid = rect_width >> 1 as ::core::ffi::c_int;
        if col_size[0usize] > 0 as ::core::ffi::c_int
            && col_size[1usize] > 0 as ::core::ffi::c_int
        {
            col_mid = (col_size[1usize] - col_size[0usize] >> 1 as ::core::ffi::c_int)
                + col_size[0usize];
            r.corner.x = (col_mid - rect_mid) as COORD;
        }
        GetContextFontLeading(&raw mut leading);
        current = _self as *mut WIDGET_MENUCONTROLENTRY;
        index = (*current).controlindex;
        height = uqm_lua_checked!(Some((*_self).height.expect("non-null function pointer"))
            .expect("non-null function pointer")(_self), ());
        index = if index > 9 as ::core::ffi::c_int {
            10 as ::core::ffi::c_int
        } else {
            index
        };
        corrected_y = leading as ::core::ffi::c_int
            + index * (height + ((5 as ::core::ffi::c_int) << resolutionFactor));
        r.corner.y = (t.baseline.y as ::core::ffi::c_int
            - r.extent.height as ::core::ffi::c_int - corrected_y) as COORD;
        if (r.corner.y as ::core::ffi::c_int)
            < (22 as ::core::ffi::c_int) << resolutionFactor
        {
            r.corner.y = ((22 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        }
        i = 0 as ::core::ffi::c_int;
        while i < num_pages {
            if i > 0 as ::core::ffi::c_int {
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    + (r.extent.width as ::core::ffi::c_int + rect_gap)) as COORD;
            }
            SetContextForeGroundColor(
                if i == (*self_0).current_page {
                    colorFromRgb15(0 as BYTE, 0x18 as BYTE, 0 as BYTE)
                } else {
                    colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0xf as BYTE)
                },
            );
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            i += 1;
        }
        SetContextForeGroundColor(
            colorFromRgb15(0xd2 as BYTE, 0xb4 as BYTE, 0x8c as BYTE),
        );
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 12]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                        + 37 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            (*self_0).current_page + 1 as ::core::ffi::c_int,
            num_pages,
        );
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            - (height >> 1 as ::core::ffi::c_int)) as COORD;
        t.baseline.x = col_mid as COORD;
        t.pStr = &raw mut buf as *mut ::core::ffi::c_char;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
    SetContextForeGroundColor(OldColor);
    SetContextFontEffect(oldFontEffect);
    if !OldFont.is_null() {
        SetContextFont(OldFont);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightChoice(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return (8 as ::core::ffi::c_int) << resolutionFactor;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightFullScreen(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return CanvasHeight;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightOneLine(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return (8 as ::core::ffi::c_int) << resolutionFactor;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightLabel(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_LABEL = _self as *mut WIDGET_LABEL;
    return (*self_0).line_count * ((8 as ::core::ffi::c_int) << resolutionFactor);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_WidthFullScreen(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return CanvasWidth;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_ReceiveFocusSimple(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    widget_focus = _self;
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_ReceiveFocusChoice(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_CHOICE = _self as *mut WIDGET_CHOICE;
    widget_focus = _self;
    (*self_0).highlighted = (*self_0).selected;
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_ReceiveFocusControlEntry(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_CONTROLENTRY = _self as *mut WIDGET_CONTROLENTRY;
    let mut oldval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*widget_focus).tag.0 == WIDGET_TYPE::WIDGET_TYPE_CONTROLENTRY.0 {
        oldval = (*(widget_focus as *mut WIDGET_CONTROLENTRY)).highlighted;
    }
    widget_focus = _self;
    (*self_0).highlighted = oldval;
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_ReceiveFocusMenuControlEntry(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_MENUCONTROLENTRY = _self as *mut WIDGET_MENUCONTROLENTRY;
    let mut oldval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*widget_focus).tag.0 == WIDGET_TYPE::WIDGET_TYPE_MENUCONTROLENTRY.0 {
        oldval = (*(widget_focus as *mut WIDGET_MENUCONTROLENTRY)).highlighted;
    }
    widget_focus = _self;
    (*self_0).highlighted = oldval;
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_ReceiveFocusMenuScreen(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_MENU_SCREEN = _self as *mut WIDGET_MENU_SCREEN;
    let mut x: ::core::ffi::c_int = 0;
    let mut last_x: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < (*self_0).num_children {
        (**(*self_0).child.offset(x as isize)).parent = _self as *mut _widget;
        x += 1;
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_UP.0 as ::core::ffi::c_int {
        x = (*self_0).num_children - 1 as ::core::ffi::c_int;
        dx = -1 as ::core::ffi::c_int;
        last_x = -1 as ::core::ffi::c_int;
    } else if event == C2Rust_Unnamed::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        dx = 1 as ::core::ffi::c_int;
        last_x = (*self_0).num_children;
    } else {
        let mut child: *mut WIDGET = *(*self_0)
            .child
            .offset((*self_0).highlighted as isize);
        uqm_lua_checked!((*child).receiveFocus.expect("non-null function pointer")(child, event), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    while x != last_x {
        let mut child_0: *mut WIDGET = *(*self_0).child.offset(x as isize);
        if uqm_lua_checked!(Some((*child_0).receiveFocus.expect("non-null function pointer"))
            .expect("non-null function pointer")(child_0, event), 0) != 0
        {
            (*self_0).highlighted = x;
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        x += dx;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_ReceiveFocusRefuseFocus(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventIgnoreAll(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventChoice(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_CHOICE = _self as *mut WIDGET_CHOICE;
    match event {
        2 => {
            (*self_0).highlighted -= 1 as ::core::ffi::c_int;
            if (*self_0).highlighted < 0 as ::core::ffi::c_int {
                (*self_0).highlighted = (*self_0).numopts - 1 as ::core::ffi::c_int;
            }
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        3 => {
            (*self_0).highlighted += 1 as ::core::ffi::c_int;
            if (*self_0).highlighted >= (*self_0).numopts {
                (*self_0).highlighted = 0 as ::core::ffi::c_int;
            }
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        4 => {
            let mut oldval: ::core::ffi::c_int = (*self_0).selected;
            (*self_0).selected = (*self_0).highlighted;
            if (*self_0).onChange.is_some() {
                uqm_lua_checked!(Some((*self_0).onChange.expect("non-null function pointer"))
                    .expect("non-null function pointer")(self_0, oldval), 0);
            }
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        _ => return BOOLEAN::FALSE.0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventSlider(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_SLIDER = _self as *mut WIDGET_SLIDER;
    match event {
        2 => {
            (*self_0).value -= (*self_0).step;
            if (*self_0).value < (*self_0).min {
                (*self_0).value = (*self_0).min;
            }
            if (*self_0).onChange.is_some() {
                uqm_lua_checked!(Some((*self_0).onChange.expect("non-null function pointer"))
                    .expect("non-null function pointer")(self_0), 0);
            }
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        3 => {
            (*self_0).value += (*self_0).step;
            if (*self_0).value > (*self_0).max {
                (*self_0).value = (*self_0).max;
            }
            if (*self_0).onChange.is_some() {
                uqm_lua_checked!(Some((*self_0).onChange.expect("non-null function pointer"))
                    .expect("non-null function pointer")(self_0), 0);
            }
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        _ => return BOOLEAN::FALSE.0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventMenuScreen(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_MENU_SCREEN = _self as *mut WIDGET_MENU_SCREEN;
    let mut x: ::core::ffi::c_int = 0;
    let mut last_x: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    match event {
        0 => {
            dx = -1 as ::core::ffi::c_int;
        }
        1 => {
            dx = 1 as ::core::ffi::c_int;
        }
        5 => {
            (*self_0).highlighted = (*self_0).num_children - 1 as ::core::ffi::c_int;
            widget_focus = *(*self_0).child.offset((*self_0).highlighted as isize)
                as *mut WIDGET;
            return uqm_lua_checked!((*widget_focus)
                .handleEvent
                .expect(
                    "non-null function pointer",
                )(
                widget_focus,
                C2Rust_Unnamed::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int,
            ), 0);
        }
        _ => return BOOLEAN::FALSE.0 as ::core::ffi::c_int,
    }
    last_x = (*self_0).highlighted;
    x = (*self_0).highlighted + dx;
    while x != last_x {
        let mut child: *mut WIDGET = ::core::ptr::null_mut::<WIDGET>();
        if x == -1 as ::core::ffi::c_int {
            x = (*self_0).num_children - 1 as ::core::ffi::c_int;
        }
        if x == (*self_0).num_children {
            x = 0 as ::core::ffi::c_int;
        }
        child = *(*self_0).child.offset(x as isize) as *mut WIDGET;
        if uqm_lua_checked!(Some((*child).receiveFocus.expect("non-null function pointer"))
            .expect("non-null function pointer")(child, event), 0) != 0
        {
            (*self_0).highlighted = x;
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        x += dx;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventTextEntry(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_TEXTENTRY = _self as *mut WIDGET_TEXTENTRY;
    if event == C2Rust_Unnamed::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        if (*self_0).handleEventSelect.is_none() {
            return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
        }
        return uqm_lua_checked!(Some((*self_0).handleEventSelect.expect("non-null function pointer"))
            .expect("non-null function pointer")(self_0), 0);
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn SyncControlPages(
    mut self_0: *mut WIDGET_CONTROLENTRY,
    mut new_page: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut parent: *mut WIDGET_MENU_SCREEN = ::core::ptr::null_mut::<
        WIDGET_MENU_SCREEN,
    >();
    let mut child: *mut WIDGET_CONTROLENTRY = ::core::ptr::null_mut::<
        WIDGET_CONTROLENTRY,
    >();
    parent = (*self_0).parent as *mut WIDGET_MENU_SCREEN;
    i = 0 as ::core::ffi::c_int;
    while i < (*parent).num_children {
        if (**(*parent).child.offset(i as isize)).tag.0
            == WIDGET_TYPE::WIDGET_TYPE_CONTROLENTRY.0
        {
            child = *(*parent).child.offset(i as isize) as *mut WIDGET_CONTROLENTRY;
            (*child).current_page = new_page;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventControlEntry(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_page: ::core::ffi::c_int = 0;
    let mut num_pages: ::core::ffi::c_int = 0;
    let mut new_page: ::core::ffi::c_int = 0;
    let mut self_0: *mut WIDGET_CONTROLENTRY = _self as *mut WIDGET_CONTROLENTRY;
    current_page = (*self_0).current_page;
    num_pages = (*self_0).num_pages;
    if event == C2Rust_Unnamed::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        if (*self_0).onChange.is_some() {
            uqm_lua_checked!((*self_0).onChange.expect("non-null function pointer")(self_0), 0);
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_DELETE.0 as ::core::ffi::c_int {
        if (*self_0).onDelete.is_some() {
            uqm_lua_checked!((*self_0).onDelete.expect("non-null function pointer")(self_0), 0);
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int {
        if (*self_0).highlighted < 1 as ::core::ffi::c_int {
            (*self_0).highlighted += 1;
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        new_page = if current_page < num_pages - 1 as ::core::ffi::c_int {
            current_page + 1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        SyncControlPages(self_0, new_page);
        (*self_0).highlighted = 0 as ::core::ffi::c_int;
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int {
        if (*self_0).highlighted > 0 as ::core::ffi::c_int {
            (*self_0).highlighted -= 1;
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        new_page = if current_page > 0 as ::core::ffi::c_int {
            current_page - 1 as ::core::ffi::c_int
        } else {
            num_pages - 1 as ::core::ffi::c_int
        };
        SyncControlPages(self_0, new_page);
        (*self_0).highlighted = if new_page == num_pages - 1 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn SyncMenuControlPages(
    mut self_0: *mut WIDGET_MENUCONTROLENTRY,
    mut new_page: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut parent: *mut WIDGET_MENU_SCREEN = ::core::ptr::null_mut::<
        WIDGET_MENU_SCREEN,
    >();
    let mut child: *mut WIDGET_MENUCONTROLENTRY = ::core::ptr::null_mut::<
        WIDGET_MENUCONTROLENTRY,
    >();
    parent = (*self_0).parent as *mut WIDGET_MENU_SCREEN;
    i = 0 as ::core::ffi::c_int;
    while i < (*parent).num_children {
        if (**(*parent).child.offset(i as isize)).tag.0
            == WIDGET_TYPE::WIDGET_TYPE_MENUCONTROLENTRY.0
        {
            child = *(*parent).child.offset(i as isize) as *mut WIDGET_MENUCONTROLENTRY;
            (*child).current_page = new_page;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventMenuControlEntry(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_MENUCONTROLENTRY = _self as *mut WIDGET_MENUCONTROLENTRY;
    if event == C2Rust_Unnamed::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        if (*self_0).onChange.is_some() {
            uqm_lua_checked!((*self_0).onChange.expect("non-null function pointer")(self_0), 0);
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_DELETE.0 as ::core::ffi::c_int {
        if (*self_0).onDelete.is_some() {
            uqm_lua_checked!((*self_0).onDelete.expect("non-null function pointer")(self_0), 0);
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int {
        if (*self_0).highlighted < 1 as ::core::ffi::c_int {
            (*self_0).highlighted += 1;
        } else {
            let mut new_page: ::core::ffi::c_int = 0;
            if (*self_0).current_page < (*self_0).num_pages - 1 as ::core::ffi::c_int {
                new_page = (*self_0).current_page + 1 as ::core::ffi::c_int;
            } else {
                new_page = 0 as ::core::ffi::c_int;
            }
            SyncMenuControlPages(self_0, new_page);
            (*self_0).highlighted = 0 as ::core::ffi::c_int;
        }
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int {
        if (*self_0).highlighted > 0 as ::core::ffi::c_int {
            (*self_0).highlighted -= 1;
        } else {
            let mut new_page_0: ::core::ffi::c_int = 0;
            if (*self_0).current_page > 0 as ::core::ffi::c_int {
                new_page_0 = (*self_0).current_page - 1 as ::core::ffi::c_int;
            } else {
                new_page_0 = (*self_0).num_pages - 1 as ::core::ffi::c_int;
            }
            SyncMenuControlPages(self_0, new_page_0);
            (*self_0).highlighted = 1 as ::core::ffi::c_int;
        }
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_Event(
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut widget: *mut WIDGET = widget_focus;
    while !widget.is_null() {
        if uqm_lua_checked!(Some((*widget).handleEvent.expect("non-null function pointer"))
            .expect("non-null function pointer")(widget, event), 0) != 0
        {
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        widget = (*widget).parent as *mut WIDGET;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
