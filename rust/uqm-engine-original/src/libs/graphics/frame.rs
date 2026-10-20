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

// Rust translation of src/libs/graphics/frame.c.
extern "C" {
    fn BoxIntersect(
        pr1: *mut RECT,
        pr2: *mut RECT,
        printer: *mut RECT,
    ) -> INTERSECT_CODE;
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    static mut _pCurContext: CONTEXT;
    static mut _locPrim: PRIMITIVE;
    static mut _GraphicsStatusFlags: GRAPHICS_STATUS;
    static mut _CurFramePtr: FRAME;
    fn _text_blt(pClipRect: *mut RECT, TextPtr: *mut TEXT, ctxOrigin: POINT);
    fn TFB_Prim_Line(_: *mut LINE, _: Color, _: DrawMode, ctxOrigin: POINT);
    fn TFB_Prim_Point(_: *mut POINT, _: Color, _: DrawMode, ctxOrigin: POINT);
    fn TFB_Prim_Rect(_: *mut RECT, _: Color, _: DrawMode, ctxOrigin: POINT);
    fn TFB_Prim_FillRect(_: *mut RECT, _: Color, _: DrawMode, ctxOrigin: POINT);
    fn TFB_Prim_Stamp(_: *mut STAMP, _: DrawMode, ctxOrigin: POINT);
    fn TFB_Prim_StampFill(_: *mut STAMP, _: Color, _: DrawMode, ctxOrigin: POINT);
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
    pub Leading: UWORD,
    pub LeadingWidth: UWORD,
    pub fontPages: *mut FONT_PAGE,
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
pub type INTERSECT_CODE = BYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_DEFAULT: Self = Self(0);
}
pub type GRAPHICS_STATUS = BYTE;
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
#[no_mangle]
pub unsafe extern "C" fn MAKE_HOT_SPOT(mut x: COORD, mut y: COORD) -> HOT_SPOT {
    let mut hs: HOT_SPOT = HOT_SPOT { x: 0, y: 0 };
    hs.x = x;
    hs.y = y;
    return hs;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextValidRect(
    mut pValidRect: *mut RECT,
    mut origin: *mut POINT,
) -> BOOLEAN {
    let mut tempRect: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut tempPt: POINT = HOT_SPOT { x: 0, y: 0 };
    if pValidRect.is_null() {
        pValidRect = &raw mut tempRect;
    }
    if origin.is_null() {
        origin = &raw mut tempPt;
    }
    (*pValidRect).corner.x = 0 as COORD;
    (*pValidRect).corner.y = 0 as COORD;
    (*pValidRect).extent = (*_CurFramePtr).Bounds;
    *origin = (*_CurFramePtr).HotSpot as POINT;
    if (*_pCurContext).ClipRect.extent.width != 0 {
        if BoxIntersect(&raw mut (*_pCurContext).ClipRect, pValidRect, pValidRect) == 0 {
            return BOOLEAN::FALSE;
        }
        (*origin).x = ((*origin).x as ::core::ffi::c_int
            + (*_pCurContext).ClipRect.corner.x as ::core::ffi::c_int) as COORD;
        (*origin).y = ((*origin).y as ::core::ffi::c_int
            + (*_pCurContext).ClipRect.corner.y as ::core::ffi::c_int) as COORD;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn ClearBackGround(mut pClipRect: *mut RECT) {
    let mut clearRect: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut color: Color = (*_pCurContext).BackGroundColor;
    clearRect.corner.x = 0 as COORD;
    clearRect.corner.y = 0 as COORD;
    clearRect.extent = (*pClipRect).extent;
    TFB_Prim_FillRect(
        &raw mut clearRect,
        color,
        MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
        (*pClipRect).corner,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DrawBatch(
    mut lpBasePrim: *mut PRIMITIVE,
    mut PrimLinks: PRIM_LINKS,
    mut BatchFlags: BATCH_FLAGS,
) {
    let mut ValidRect: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(&raw mut ValidRect, &raw mut origin).0 != 0
    {
        let mut CurIndex: COUNT = 0;
        let mut lpPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
        let mut mode: DrawMode = (*_pCurContext).Mode;
        BatchGraphics();
        if BatchFlags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BATCH_FLAGS
                as ::core::ffi::c_int != 0
        {
            ClearBackGround(&raw mut ValidRect);
        }
        CurIndex = PrimLinks as UWORD as COUNT;
        while CurIndex as ::core::ffi::c_int
            != 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
        {
            let mut PrimType: GRAPHICS_PRIM = 0;
            let mut lpWorkPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
            let mut ClipRect: RECT = RECT {
                corner: HOT_SPOT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
            lpPrim = lpBasePrim.offset(CurIndex as isize);
            PrimType = (*lpPrim).Type;
            if (PrimType as ::core::ffi::c_int)
                < gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
            {
                lpWorkPrim = lpPrim;
                match PrimType as ::core::ffi::c_int {
                    0 => {
                        color = (*lpWorkPrim).color;
                        TFB_Prim_Point(
                            &raw mut (*lpWorkPrim).Object.Point,
                            color,
                            mode,
                            origin,
                        );
                    }
                    1 => {
                        TFB_Prim_Stamp(
                            &raw mut (*lpWorkPrim).Object.Stamp,
                            mode,
                            origin,
                        );
                    }
                    2 => {
                        color = (*lpWorkPrim).color;
                        TFB_Prim_StampFill(
                            &raw mut (*lpWorkPrim).Object.Stamp,
                            color,
                            mode,
                            origin,
                        );
                    }
                    3 => {
                        color = (*lpWorkPrim).color;
                        TFB_Prim_Line(
                            &raw mut (*lpWorkPrim).Object.Line,
                            color,
                            mode,
                            origin,
                        );
                    }
                    4 => {
                        if TextRect(
                                &raw mut (*lpWorkPrim).Object.Text,
                                &raw mut ClipRect,
                                ::core::ptr::null_mut::<BYTE>(),
                            )
                            .0 != 0
                        {
                            _text_blt(
                                &raw mut ClipRect,
                                &raw mut (*lpWorkPrim).Object.Text,
                                origin,
                            );
                        }
                    }
                    5 => {
                        color = (*lpWorkPrim).color;
                        TFB_Prim_Rect(
                            &raw mut (*lpWorkPrim).Object.Rect,
                            color,
                            mode,
                            origin,
                        );
                    }
                    6 => {
                        color = (*lpWorkPrim).color;
                        TFB_Prim_FillRect(
                            &raw mut (*lpWorkPrim).Object.Rect,
                            color,
                            mode,
                            origin,
                        );
                    }
                    _ => {}
                }
            }
            CurIndex = ((*lpPrim).Links >> 16 as ::core::ffi::c_int) as UWORD as COUNT;
        }
        UnbatchGraphics();
    }
}
#[no_mangle]
pub unsafe extern "C" fn ClearDrawable() {
    let mut ValidRect: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(&raw mut ValidRect, ::core::ptr::null_mut::<POINT>()).0
            != 0
    {
        ClearBackGround(&raw mut ValidRect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawPoint(mut lpPoint: *mut POINT) {
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 != 0
    {
        let mut color: Color = _locPrim.color;
        let mut mode: DrawMode = (*_pCurContext).Mode;
        TFB_Prim_Point(lpPoint, color, mode, origin);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawRectangle(mut lpRect: *mut RECT) {
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 != 0
    {
        let mut color: Color = _locPrim.color;
        let mut mode: DrawMode = (*_pCurContext).Mode;
        TFB_Prim_Rect(lpRect, color, mode, origin);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawFilledRectangle(mut lpRect: *mut RECT) {
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 != 0
    {
        let mut color: Color = _locPrim.color;
        let mut mode: DrawMode = (*_pCurContext).Mode;
        TFB_Prim_FillRect(lpRect, color, mode, origin);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawLine(mut lpLine: *mut LINE) {
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 != 0
    {
        let mut color: Color = _locPrim.color;
        let mut mode: DrawMode = (*_pCurContext).Mode;
        TFB_Prim_Line(lpLine, color, mode, origin);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawStamp(mut stmp: *mut STAMP) {
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 != 0
    {
        let mut mode: DrawMode = (*_pCurContext).Mode;
        TFB_Prim_Stamp(stmp, mode, origin);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawFilledStamp(mut stmp: *mut STAMP) {
    let mut origin: POINT = HOT_SPOT { x: 0, y: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        && GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 != 0
    {
        let mut color: Color = _locPrim.color;
        let mut mode: DrawMode = (*_pCurContext).Mode;
        TFB_Prim_StampFill(stmp, color, mode, origin);
    }
}
