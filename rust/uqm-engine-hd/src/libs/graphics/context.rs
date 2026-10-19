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

// Rust translation of src/libs/graphics/context.c.
extern "C" {
    pub type NativePalette;
    fn BoxIntersect(
        pr1: *mut RECT,
        pr2: *mut RECT,
        printer: *mut RECT,
    ) -> INTERSECT_CODE;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn LoadDisplayPixmap(area: *const RECT, frame: FRAME) -> DRAWABLE;
    fn GetContextFontDispHeight(pheight: *mut SIZE) -> BOOLEAN;
    fn GetContextFontDispWidth(pwidth: *mut SIZE) -> BOOLEAN;
    fn CopyFrameRect(Frame: FRAME, area: *const RECT) -> DRAWABLE;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    static mut _CurFramePtr: FRAME;
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
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type SBYTE = sint8;
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
pub type DRAWABLE_DESC = drawable_desc;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub type GRAPHICS_STATUS = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_colormap {
    pub index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut tfb_colormap,
    pub palette: *mut NativePalette,
}
pub type TFB_ColorMap = tfb_colormap;
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
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn extentsEqual(mut e1: EXTENT, mut e2: EXTENT) -> bool {
    return e1.width as ::core::ffi::c_int == e2.width as ::core::ffi::c_int
        && e1.height as ::core::ffi::c_int == e2.height as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn rectsEqual(mut r1: RECT, mut r2: RECT) -> bool {
    return pointsEqual(r1.corner, r2.corner) as ::core::ffi::c_int != 0
        && extentsEqual(r1.extent, r2.extent) as ::core::ffi::c_int != 0;
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
#[no_mangle]
pub static mut _GraphicsStatusFlags: GRAPHICS_STATUS = 0;
#[no_mangle]
pub static mut _pCurContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut firstContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut contextEnd: *mut CONTEXT = unsafe {
    &raw const firstContext as *mut CONTEXT
};
#[no_mangle]
pub static mut _locPrim: PRIMITIVE = PRIMITIVE {
    Links: 0,
    Type: 0,
    color: Color { r: 0, g: 0, b: 0, a: 0 },
    Object: PRIM_DESC {
        Point: POINT { x: 0, y: 0 },
    },
    flags: 0,
};
#[no_mangle]
pub static mut _CurFontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub unsafe extern "C" fn SetContext(mut Context: CONTEXT) -> CONTEXT {
    let mut LastContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    LastContext = _pCurContext;
    if Context != LastContext {
        if !LastContext.is_null() {
            (*_pCurContext).Flags = ((*_pCurContext).Flags as ::core::ffi::c_int
                & !((((((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as GRAPHICS_STATUS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as GRAPHICS_STATUS as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                    as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)) as UWORD;
            (*_pCurContext).Flags = ((*_pCurContext).Flags as ::core::ffi::c_int
                | ((((_GraphicsStatusFlags as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                        as GRAPHICS_STATUS as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as GRAPHICS_STATUS as ::core::ffi::c_int)) as BYTE
                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                    as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)) as UWORD;
            _GraphicsStatusFlags = (_GraphicsStatusFlags as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as GRAPHICS_STATUS as ::core::ffi::c_int)) as GRAPHICS_STATUS;
        }
        _pCurContext = Context;
        if !_pCurContext.is_null() {
            _GraphicsStatusFlags = (_GraphicsStatusFlags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as GRAPHICS_STATUS as ::core::ffi::c_int) as GRAPHICS_STATUS;
            _GraphicsStatusFlags = (_GraphicsStatusFlags as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as GRAPHICS_STATUS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as GRAPHICS_STATUS as ::core::ffi::c_int)) as GRAPHICS_STATUS;
            _GraphicsStatusFlags = (_GraphicsStatusFlags as ::core::ffi::c_int
                | ((*_pCurContext).Flags as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                as GRAPHICS_STATUS;
            _locPrim.color = (*_pCurContext).ForeGroundColor;
            _CurFramePtr = (*_pCurContext).ForeGroundFrame;
            _CurFontPtr = (*_pCurContext).Font;
        }
    }
    return LastContext;
}
#[no_mangle]
pub unsafe extern "C" fn CreateContextAux(
    mut name: *const ::core::ffi::c_char,
) -> CONTEXT {
    let mut NewContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    NewContext = HCalloc(::core::mem::size_of::<CONTEXT_DESC>()) as CONTEXT;
    if !NewContext.is_null() {
        (*NewContext).name = name;
        (*NewContext).next = ::core::ptr::null_mut::<CONTEXT_DESC>();
        *contextEnd = NewContext;
        contextEnd = &raw mut (*NewContext).next;
        (*NewContext).Mode = MAKE_DRAW_MODE(DrawKind::DRAW_DEFAULT, 255 as SWORD);
        (*NewContext).ForeGroundColor = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        (*NewContext).BackGroundColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    return NewContext;
}
unsafe extern "C" fn FindContextPtr(mut context: CONTEXT) -> *mut CONTEXT {
    let mut ptr: *mut CONTEXT = ::core::ptr::null_mut::<CONTEXT>();
    ptr = &raw mut firstContext;
    while !(*ptr).is_null() {
        if *ptr == context {
            break;
        }
        ptr = &raw mut (**ptr).next;
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyContext(mut ContextRef: CONTEXT) -> BOOLEAN {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if ContextRef.is_null() {
        return BOOLEAN::FALSE;
    }
    if !_pCurContext.is_null() && _pCurContext == ContextRef {
        SetContext(::core::ptr::null_mut::<CONTEXT_DESC>());
    }
    let mut contextPtr: *mut CONTEXT = FindContextPtr(ContextRef);
    if contextEnd == &raw mut (*ContextRef).next {
        contextEnd = contextPtr;
    }
    *contextPtr = (*ContextRef).next;
    img = (*ContextRef).FontBacking;
    if !img.is_null() {
        TFB_DrawImage_Delete(img);
    }
    HFree(ContextRef as *mut ::core::ffi::c_void);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn SetContextForeGroundColor(mut color: Color) -> Color {
    let mut oldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    }
    oldColor = (*_pCurContext).ForeGroundColor;
    if !sameColor(oldColor, color) {
        (*_pCurContext).ForeGroundColor = color;
        if (*_pCurContext).BackingFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
        {
            (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as FBK_FLAGS;
        }
    }
    _locPrim.color = color;
    return oldColor;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextForeGroundColor() -> Color {
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    }
    return (*_pCurContext).ForeGroundColor;
}
#[no_mangle]
pub unsafe extern "C" fn SetContextBackGroundColor(mut color: Color) -> Color {
    let mut oldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    oldColor = (*_pCurContext).BackGroundColor;
    if !sameColor(oldColor, color) {
        (*_pCurContext).BackGroundColor = color;
    }
    return oldColor;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextBackGroundColor() -> Color {
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    return (*_pCurContext).BackGroundColor;
}
#[no_mangle]
pub unsafe extern "C" fn SetContextDrawMode(mut mode: DrawMode) -> DrawMode {
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return MAKE_DRAW_MODE(DrawKind::DRAW_DEFAULT, 255 as SWORD);
    }
    oldMode = (*_pCurContext).Mode;
    (*_pCurContext).Mode = mode;
    return oldMode;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextDrawMode() -> DrawMode {
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return MAKE_DRAW_MODE(DrawKind::DRAW_DEFAULT, 255 as SWORD);
    }
    return (*_pCurContext).Mode;
}
#[inline]
unsafe extern "C" fn _get_context_fg_rect() -> RECT {
    let mut r: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: 0 as COORD,
            height: 0 as COORD,
        },
    };
    if !_CurFramePtr.is_null() {
        r.extent = (*_CurFramePtr).Bounds;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn SetContextClipRect(mut lpRect: *mut RECT) -> BOOLEAN {
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return BOOLEAN::FALSE;
    }
    if !lpRect.is_null() {
        if rectsEqual(*lpRect, _get_context_fg_rect()) {
            (*_pCurContext).ClipRect.extent.width = 0 as COORD;
        } else {
            (*_pCurContext).ClipRect = *lpRect;
        }
    } else {
        (*_pCurContext).ClipRect.extent.width = 0 as COORD;
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextClipRect(mut lpRect: *mut RECT) -> BOOLEAN {
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return BOOLEAN::FALSE;
    }
    *lpRect = (*_pCurContext).ClipRect;
    if (*_pCurContext).ClipRect.extent.width == 0 {
        *lpRect = _get_context_fg_rect();
    }
    return BOOLEAN(
        ((*_pCurContext).ClipRect.extent.width as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetContextOrigin(mut orgOffset: POINT) -> POINT {
    return SetFrameHot(_CurFramePtr, orgOffset);
}
#[no_mangle]
pub unsafe extern "C" fn SetContextFontEffect(mut EffectFrame: FRAME) -> FRAME {
    let mut LastEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
    {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    LastEffect = (*_pCurContext).FontEffect;
    if EffectFrame != LastEffect {
        (*_pCurContext).FontEffect = EffectFrame;
        (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
            as FBK_FLAGS;
        if !EffectFrame.is_null() {
            (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as FBK_FLAGS;
        } else {
            (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as FBK_FLAGS;
        }
    }
    return LastEffect;
}
#[no_mangle]
pub unsafe extern "C" fn FixContextFontEffect() {
    let mut w: SIZE = 0;
    let mut h: SIZE = 0;
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
        || !(*_pCurContext).FontBacking.is_null()
            && (*_pCurContext).BackingFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        return;
    }
    if GetContextFontDispHeight(&raw mut h).0 == 0
        || GetContextFontDispWidth(&raw mut w).0 == 0
    {
        return;
    }
    img = (*_pCurContext).FontBacking;
    if !img.is_null() {
        TFB_DrawScreen_DeleteImage(img);
    }
    img = TFB_DrawImage_CreateForScreen(
        w as ::core::ffi::c_int,
        h as ::core::ffi::c_int,
        BOOLEAN::TRUE,
    );
    if (*_pCurContext).BackingFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        let mut EffectFrame: FRAME = (*_pCurContext).FontEffect;
        uqm_lua_checked!(TFB_DrawImage_Image(
            (*EffectFrame).image,
            -((*EffectFrame).HotSpot.x as ::core::ffi::c_int),
            -((*EffectFrame).HotSpot.y as ::core::ffi::c_int),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<TFB_ColorMap>(),
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            img,
        ), ());
    } else {
        let mut r: RECT = rect {
            corner: point {
                x: 0 as COORD,
                y: 0 as COORD,
            },
            extent: extent { width: w, height: h },
        };
        let mut color: Color = (*_pCurContext).ForeGroundColor;
        uqm_lua_checked!(TFB_DrawImage_Rect(
            &raw mut r,
            color,
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            img,
        ), ());
    }
    (*_pCurContext).FontBacking = img;
    (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)) as FBK_FLAGS;
}
#[no_mangle]
pub unsafe extern "C" fn CopyContextRect(mut area: *const RECT) -> DRAWABLE {
    let mut clipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut fgRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0 || _CurFramePtr.is_null()
    {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    fgRect = _get_context_fg_rect();
    GetContextClipRect(&raw mut clipRect);
    r = clipRect;
    if !area.is_null() {
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + (*area).corner.x as ::core::ffi::c_int) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + (*area).corner.y as ::core::ffi::c_int) as COORD;
        r.extent = (*area).extent;
    }
    if BoxIntersect(&raw mut r, &raw mut fgRect, &raw mut r) == 0 {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        return LoadDisplayPixmap(&raw mut r, ::core::ptr::null_mut::<FRAME_DESC>())
    } else {
        return CopyFrameRect(_CurFramePtr, &raw mut r)
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetContextName(
    mut context: CONTEXT,
) -> *const ::core::ffi::c_char {
    return (*context).name;
}
#[no_mangle]
pub unsafe extern "C" fn GetFirstContext() -> CONTEXT {
    return firstContext;
}
#[no_mangle]
pub unsafe extern "C" fn GetNextContext(mut context: CONTEXT) -> CONTEXT {
    return (*context).next;
}
