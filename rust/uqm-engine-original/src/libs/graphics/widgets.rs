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
    pub type frame_desc;
    pub type font_desc;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
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
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
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
    pub const NUM_WIDGET_TYPES: Self = Self(7);
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
    pub maxcolumns: ::core::ffi::c_int,
    pub options: *mut CHOICE_OPTION,
    pub selected: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
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
    pub value: [::core::ffi::c_char; 50],
    pub maxlen: ::core::ffi::c_int,
    pub state: WIDGET_TEXTENTRY_STATE,
    pub cursor_pos: ::core::ffi::c_int,
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
    pub controlname: [[::core::ffi::c_char; 16]; 2],
}
pub type WIDGET_CONTROLENTRY = _widget_controlentry;
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
#[no_mangle]
pub unsafe extern "C" fn DrawShadowedBox(
    mut r: *mut RECT,
    mut bg: Color,
    mut dark: Color,
    mut medium: Color,
) {
    let mut t: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldcolor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    BatchGraphics();
    t.corner.x = ((*r).corner.x as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        as COORD;
    t.corner.y = ((*r).corner.y as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        as COORD;
    t.extent.width = ((*r).extent.width as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
        as COORD;
    t.extent.height = ((*r).extent.height as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int) as COORD;
    oldcolor = SetContextForeGroundColor(dark);
    DrawFilledRectangle(&raw mut t);
    t.corner.x = (t.corner.x as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
    t.corner.y = (t.corner.y as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
    t.extent.width = (t.extent.width as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        as COORD;
    t.extent.height = (t.extent.height as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        as COORD;
    SetContextForeGroundColor(medium);
    DrawFilledRectangle(&raw mut t);
    t.corner.x = (t.corner.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    t.corner.y = (t.corner.y as ::core::ffi::c_int
        + ((*r).extent.height as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    t.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut t);
    t.corner.x = (t.corner.x as ::core::ffi::c_int
        + ((*r).extent.width as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as COORD;
    t.corner.y = (t.corner.y as ::core::ffi::c_int
        - ((*r).extent.height as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as COORD;
    t.extent.width = 1 as COORD;
    DrawFilledRectangle(&raw mut t);
    SetContextForeGroundColor(bg);
    DrawFilledRectangle(r);
    SetContextForeGroundColor(oldcolor);
    UnbatchGraphics();
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
    win_h = (*label).height.expect("non-null function pointer")(label as *mut _widget)
        + 16 as ::core::ffi::c_int;
    win_w = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*label).line_count {
        let mut len: ::core::ffi::c_int = utf8StringCount(
            *(*label).lines.offset(i as isize),
        ) as ::core::ffi::c_int;
        if len > win_w {
            win_w = len;
        }
        i += 1;
    }
    win_w = win_w * 6 as ::core::ffi::c_int + 16 as ::core::ffi::c_int;
    BatchGraphics();
    r.corner.x = (ScreenWidth - win_w >> 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (ScreenHeight - win_h >> 1 as ::core::ffi::c_int) as COORD;
    r.extent.width = win_w as COORD;
    r.extent.height = win_h as COORD;
    DrawShadowedBox(&raw mut r, win_bg_clr, win_dark_clr, win_medium_clr);
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as COORD;
    i = 0 as ::core::ffi::c_int;
    while i < (*label).line_count {
        t.pStr = *(*label).lines.offset(i as isize) as *const UNICODE;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        font_DrawText(&raw mut t);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as COORD;
        i += 1;
    }
    UnbatchGraphics();
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldfg);
    if !windowRect.is_null() {
        (*windowRect).corner.x = (r.corner.x as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        (*windowRect).corner.y = (r.corner.y as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        (*windowRect).extent.width = (r.extent.width as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as COORD;
        (*windowRect).extent.height = (r.extent.height as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as COORD;
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
        colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
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
    r.corner.x = 2 as COORD;
    r.corner.y = 2 as COORD;
    r.extent.width = (ScreenWidth - 4 as ::core::ffi::c_int) as COORD;
    r.extent.height = (ScreenHeight - 4 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + (r.extent.height as ::core::ffi::c_int - 8 as ::core::ffi::c_int
            - 8 as ::core::ffi::c_int * numlines)) as COORD;
    i = 0 as ::core::ffi::c_int;
    while i < numlines {
        t.pStr = *tips.offset(i as isize) as *const UNICODE;
        font_DrawText(&raw mut t);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as COORD;
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
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
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
    let mut widget_index: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut widget_y: ::core::ffi::c_int = 0;
    let mut self_0: *mut WIDGET_MENU_SCREEN = _self as *mut WIDGET_MENU_SCREEN;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    r.corner.x = 2 as COORD;
    r.corner.y = 2 as COORD;
    r.extent.width = (ScreenWidth - 4 as ::core::ffi::c_int) as COORD;
    r.extent.height = (ScreenHeight - 4 as ::core::ffi::c_int) as COORD;
    title = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE);
    default_color = title;
    DrawStamp(&raw mut (*self_0).bgStamp);
    oldtext = SetContextForeGroundColor(title);
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as COORD;
    t.pStr = (*self_0).title as *const UNICODE;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
        as COORD;
    t.pStr = (*self_0).subtitle as *const UNICODE;
    font_DrawText(&raw mut t);
    height = 0 as ::core::ffi::c_int;
    widget_index = 0 as ::core::ffi::c_int;
    while widget_index < (*self_0).num_children {
        let mut child: *mut WIDGET = *(*self_0).child.offset(widget_index as isize);
        height
            += Some((*child).height.expect("non-null function pointer"))
                .expect("non-null function pointer")(child);
        height += 8 as ::core::ffi::c_int;
        widget_index += 1;
    }
    height -= 8 as ::core::ffi::c_int;
    widget_y = ScreenHeight - height >> 1 as ::core::ffi::c_int;
    widget_index = 0 as ::core::ffi::c_int;
    while widget_index < (*self_0).num_children {
        let mut c: *mut WIDGET = *(*self_0).child.offset(widget_index as isize);
        Some((*c).draw.expect("non-null function pointer"))
            .expect("non-null function pointer")(c, 0 as ::core::ffi::c_int, widget_y);
        widget_y
            += Some((*c).height.expect("non-null function pointer"))
                .expect("non-null function pointer")(c) + 8 as ::core::ffi::c_int;
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
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
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
    let mut i: ::core::ffi::c_int = 0;
    let mut home_x: ::core::ffi::c_int = 0;
    let mut home_y: ::core::ffi::c_int = 0;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    default_color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE);
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    font_DrawText(&raw mut t);
    home_x = t.baseline.x as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * (ScreenWidth
                / (((*self_0).maxcolumns + 1 as ::core::ffi::c_int)
                    * 2 as ::core::ffi::c_int));
    home_y = t.baseline.y as ::core::ffi::c_int;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    i = 0 as ::core::ffi::c_int;
    while i < (*self_0).numopts {
        t.baseline.x = (home_x
            + i % 3 as ::core::ffi::c_int
                * (ScreenWidth / ((*self_0).maxcolumns + 1 as ::core::ffi::c_int)))
            as COORD;
        t.baseline.y = (home_y + 8 as ::core::ffi::c_int * (i / 3 as ::core::ffi::c_int))
            as COORD;
        t.pStr = (*(*self_0).options.offset(i as isize)).optname as *const UNICODE;
        if widget_focus == _self && (*self_0).highlighted == i {
            SetContextForeGroundColor(selected);
            Widget_DrawToolTips(
                3 as ::core::ffi::c_int,
                &raw mut (*(*self_0).options.offset(i as isize)).tooltip
                    as *mut *const ::core::ffi::c_char,
            );
        } else if i == (*self_0).selected {
            SetContextForeGroundColor(default_color);
        } else {
            SetContextForeGroundColor(inactive);
        }
        font_DrawText(&raw mut t);
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
    t.baseline.x = 160 as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).name as *const UNICODE;
    if widget_focus == _self {
        Widget_DrawToolTips(
            3 as ::core::ffi::c_int,
            &raw mut (*self_0).tooltip as *mut *const ::core::ffi::c_char,
        );
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(inactive);
    }
    font_DrawText(&raw mut t);
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
        colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
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
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    t.baseline.x = 160 as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    i = 0 as ::core::ffi::c_int;
    while i < (*self_0).line_count {
        t.pStr = *(*self_0).lines.offset(i as isize) as *const UNICODE;
        font_DrawText(&raw mut t);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as COORD;
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
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
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
    let mut tick: ::core::ffi::c_int = (ScreenWidth - x) / 8 as ::core::ffi::c_int;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    default_color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE);
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        Widget_DrawToolTips(
            3 as ::core::ffi::c_int,
            &raw mut (*self_0).tooltip as *mut *const ::core::ffi::c_char,
        );
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    font_DrawText(&raw mut t);
    r.corner.x = (t.baseline.x as ::core::ffi::c_int + 3 as ::core::ffi::c_int * tick)
        as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
    r.extent.height = 2 as COORD;
    r.extent.width = (3 as ::core::ffi::c_int * tick) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.extent.width = 3 as COORD;
    r.extent.height = 8 as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as COORD;
    r.corner.x = (t.baseline.x as ::core::ffi::c_int + 3 as ::core::ffi::c_int * tick
        + 3 as ::core::ffi::c_int * tick * ((*self_0).value - (*self_0).min)
            / ((*self_0).max - (*self_0).min) - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    Some((*self_0).draw_value.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        self_0,
        t.baseline.x as ::core::ffi::c_int + 7 as ::core::ffi::c_int * tick,
        t.baseline.y as ::core::ffi::c_int,
    );
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
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buffer as *mut ::core::ffi::c_char;
    font_DrawText(&raw mut t);
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
    BatchGraphics();
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    font_DrawText(&raw mut t);
    (*self_0).value[(50 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0
        as ::core::ffi::c_char;
    t.baseline.y = y as COORD;
    t.CharCount = utf8StringCount(&raw mut (*self_0).value as *mut ::core::ffi::c_char)
        as COUNT;
    t.pStr = &raw mut (*self_0).value as *mut ::core::ffi::c_char;
    if (*self_0).state.0 & WIDGET_TEXTENTRY_STATE::WTE_EDITING.0 == 0 {
        t.baseline.x = 160 as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        if widget_focus == _self {
            oldtext = SetContextForeGroundColor(selected);
        } else {
            oldtext = SetContextForeGroundColor(inactive);
        }
        font_DrawText(&raw mut t);
    } else {
        let mut i: COUNT = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 50] = [0; 50];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut leading: SIZE = 0;
        t.baseline.x = 90 as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        GetContextFontLeading(&raw mut leading);
        r.corner.x = (t.baseline.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as COORD;
        r.corner.y = (t.baseline.y as ::core::ffi::c_int - leading as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) as COORD;
        r.extent.width = (ScreenWidth - r.corner.x as ::core::ffi::c_int
            - 10 as ::core::ffi::c_int) as COORD;
        r.extent.height = (leading as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as COORD;
        TextRect(&raw mut t, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        oldtext = SetContextForeGroundColor(selected);
        DrawFilledRectangle(&raw mut r);
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
            r.corner.x -= 1;
        }
        if (*self_0).state.0 & WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0 != 0 {
            if (*self_0).cursor_pos == t.CharCount as ::core::ffi::c_int {
                r.extent.width = 1 as COORD;
            } else if (*self_0).cursor_pos + 1 as ::core::ffi::c_int
                == t.CharCount as ::core::ffi::c_int
            {
                r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COORD;
            } else {
                r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COORD;
            }
        } else {
            r.extent.width = 1 as COORD;
        }
        r.corner.x += 1;
        r.corner.y += 1;
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        DrawFilledRectangle(&raw mut r);
        SetContextForeGroundColor(inactive);
        font_DrawText(&raw mut t);
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
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
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
    let mut i: ::core::ffi::c_int = 0;
    let mut home_x: ::core::ffi::c_int = 0;
    let mut home_y: ::core::ffi::c_int = 0;
    if !cur_font.is_null() {
        oldfont = SetContextFont(cur_font);
    }
    default_color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x18 as BYTE, 0x18 as BYTE, 0x1f as BYTE);
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    font_DrawText(&raw mut t);
    home_x = t.baseline.x as ::core::ffi::c_int + ScreenWidth / 2 as ::core::ffi::c_int;
    home_y = t.baseline.y as ::core::ffi::c_int;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        t.baseline.x = (home_x
            + i % 3 as ::core::ffi::c_int * (ScreenWidth / 3 as ::core::ffi::c_int))
            as COORD;
        t.baseline.y = (home_y + 8 as ::core::ffi::c_int * (i / 3 as ::core::ffi::c_int))
            as COORD;
        t.pStr = &raw mut *(&raw mut (*self_0).controlname
            as *mut [::core::ffi::c_char; 16])
            .offset(i as isize) as *mut ::core::ffi::c_char;
        if *t.pStr.offset(0isize) == 0 {
            t.pStr = b"---\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
        }
        if widget_focus == _self && (*self_0).highlighted == i {
            SetContextForeGroundColor(selected);
        } else {
            SetContextForeGroundColor(default_color);
        }
        font_DrawText(&raw mut t);
        i += 1;
    }
    SetContextFontEffect(oldFontEffect);
    if !oldfont.is_null() {
        SetContextFont(oldfont);
    }
    SetContextForeGroundColor(oldtext);
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightChoice(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return ((*(_self as *mut WIDGET_CHOICE)).numopts + 2 as ::core::ffi::c_int)
        / 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightFullScreen(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return ScreenHeight;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightOneLine(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HeightLabel(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_LABEL = _self as *mut WIDGET_LABEL;
    return (*self_0).line_count * 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_WidthFullScreen(
    mut _self: *mut WIDGET,
) -> ::core::ffi::c_int {
    return ScreenWidth;
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
        (*child).receiveFocus.expect("non-null function pointer")(child, event);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    while x != last_x {
        let mut child_0: *mut WIDGET = *(*self_0).child.offset(x as isize);
        if Some((*child_0).receiveFocus.expect("non-null function pointer"))
            .expect("non-null function pointer")(child_0, event) != 0
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
                Some((*self_0).onChange.expect("non-null function pointer"))
                    .expect("non-null function pointer")(self_0, oldval);
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
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        3 => {
            (*self_0).value += (*self_0).step;
            if (*self_0).value > (*self_0).max {
                (*self_0).value = (*self_0).max;
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
            return (*widget_focus)
                .handleEvent
                .expect(
                    "non-null function pointer",
                )(
                widget_focus,
                C2Rust_Unnamed::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int,
            );
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
        if Some((*child).receiveFocus.expect("non-null function pointer"))
            .expect("non-null function pointer")(child, event) != 0
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
        return Some((*self_0).handleEventSelect.expect("non-null function pointer"))
            .expect("non-null function pointer")(self_0);
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Widget_HandleEventControlEntry(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_CONTROLENTRY = _self as *mut WIDGET_CONTROLENTRY;
    if event == C2Rust_Unnamed::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        if (*self_0).onChange.is_some() {
            (*self_0).onChange.expect("non-null function pointer")(self_0);
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_DELETE.0 as ::core::ffi::c_int {
        if (*self_0).onDelete.is_some() {
            (*self_0).onDelete.expect("non-null function pointer")(self_0);
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
    }
    if event == C2Rust_Unnamed::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int
        || event == C2Rust_Unnamed::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int
    {
        (*self_0).highlighted = 1 as ::core::ffi::c_int - (*self_0).highlighted;
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
        if Some((*widget).handleEvent.expect("non-null function pointer"))
            .expect("non-null function pointer")(widget, event) != 0
        {
            return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        }
        widget = (*widget).parent as *mut WIDGET;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
