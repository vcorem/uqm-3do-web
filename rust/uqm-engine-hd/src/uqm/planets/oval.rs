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

// Rust translation of src/uqm/planets/oval.c.
extern "C" {
    pub type font_desc;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawLine(pLine: *mut LINE, thickness: BYTE);
    fn DrawBatch(
        pBasePrim: *mut PRIMITIVE,
        PrimLinks: PRIM_LINKS,
        BatchFlags: BATCH_FLAGS,
    );
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    static mut _pCurContext: CONTEXT;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut stars_in_space: FRAME;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type sint64 = ::core::ffi::c_longlong;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type SQWORD = sint64;
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
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dextent {
    pub width: SDWORD,
    pub height: SDWORD,
}
pub type DEXTENT = dextent;
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
#[inline]
unsafe extern "C" fn SetPrimNextLink(mut pPrim: *mut PRIMITIVE, mut Link: COUNT) {
    (*pPrim).Links = ((Link as DWORD) << 16 as ::core::ffi::c_int
        | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
}
unsafe extern "C" fn PointOnScreen(mut x: SDWORD, mut y: SDWORD) -> BOOLEAN {
    let mut extend: SDWORD = 0 as SDWORD;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        extend = 5 as ::core::ffi::c_int as SDWORD;
    }
    return BOOLEAN(
        (x + extend >= 0 as ::core::ffi::c_int
            && x
                <= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor)) && y + extend >= 0 as ::core::ffi::c_int
            && y
                <= CanvasHeight
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
}
unsafe extern "C" fn DPointOnScreen(mut p: *mut DPOINT) -> BOOLEAN {
    return BOOLEAN(
        ((*p).x >= 0 as ::core::ffi::c_int
            && (*p).x
                <= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor)) && (*p).y >= 0 as ::core::ffi::c_int
            && (*p).y
                <= CanvasHeight
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
}
unsafe extern "C" fn TruncateDPoint(mut p: *mut DPOINT) {
    if (*p).x > 32767 as ::core::ffi::c_int {
        (*p).x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))) as SDWORD;
    }
    if (*p).x < -32768 as ::core::ffi::c_int {
        (*p).x = 0 as ::core::ffi::c_int as SDWORD;
    }
    if (*p).y > 32767 as ::core::ffi::c_int {
        (*p).y = (CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor)) as SDWORD;
    }
    if (*p).y < -32768 as ::core::ffi::c_int {
        (*p).y = 0 as ::core::ffi::c_int as SDWORD;
    }
}
unsafe extern "C" fn CheckOvalCollision(
    mut p0: *mut DPOINT,
    mut p1: *mut DPOINT,
) -> BYTE {
    let mut mp: DPOINT = DPOINT { x: 0, y: 0 };
    let mut quad_visible: BYTE = 0 as BYTE;
    let mut x0: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut asquared: ::core::ffi::c_double = 0.;
    let mut bsquared: ::core::ffi::c_double = 0.;
    let mut a: ::core::ffi::c_double = 0.;
    let mut b: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut r0: ::core::ffi::c_double = 0.;
    let mut r1: ::core::ffi::c_double = 0.;
    if DPointOnScreen(p0).0 != 0 && DPointOnScreen(p1).0 != 0 {
        quad_visible = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
        return quad_visible;
    }
    x0 = (*p0).x + ((*p1).x - (*p0).x >> 2 as ::core::ffi::c_int);
    x1 = x0 + (x0 - (*p0).x << 1 as ::core::ffi::c_int);
    y0 = (*p0).y + ((*p1).y - (*p0).y >> 2 as ::core::ffi::c_int);
    y1 = y0 + (y0 - (*p0).y << 1 as ::core::ffi::c_int);
    if x0 < 0 as ::core::ffi::c_int && y0 < 0 as ::core::ffi::c_int
        && x1
            > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        && y1
            > CanvasHeight
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
        return quad_visible;
    }
    mp.x = (*p0).x + (*p1).x >> 1 as ::core::ffi::c_int;
    mp.y = (*p0).y + (*p1).y >> 1 as ::core::ffi::c_int;
    a = ((*p1).x - (*p0).x >> 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    b = ((*p1).y - (*p0).y >> 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if b > a {
        let mut p: ::core::ffi::c_double = a;
        a = b;
        b = p;
    }
    asquared = a * a;
    bsquared = b * b;
    if !(mp.x
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) || (*p1).x < 0 as ::core::ffi::c_int
        || (*p0).y
            > CanvasHeight
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
        || mp.y < 0 as ::core::ffi::c_int
        || x1
            > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) && y0 < 0 as ::core::ffi::c_int)
    {
        if PointOnScreen(mp.x, (*p0).y).0 != 0 || PointOnScreen((*p1).x, mp.y).0 != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        } else {
            x = (CanvasWidth - ((64 as SDWORD) << resolutionFactor)
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
                    } << resolutionFactor)) - mp.x) as ::core::ffi::c_double;
            y = (0 as SDWORD - mp.y) as ::core::ffi::c_double;
            r0 = x * x / asquared + y * y / bsquared;
            x = (if 0 as ::core::ffi::c_int > mp.x { 0 as SDWORD } else { mp.x } - mp.x)
                as ::core::ffi::c_double;
            y = (if CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor) < mp.y
            {
                CanvasHeight
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
                    - ((13 as SDWORD) << resolutionFactor)
            } else {
                mp.y
            } - mp.y) as ::core::ffi::c_double;
            r1 = x * x / asquared + y * y / bsquared;
            if r0 >= 1.0f64 && r1 <= 1.0f64 {
                quad_visible = (quad_visible as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
            }
        }
    }
    if !((*p0).x
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) || mp.x < 0 as ::core::ffi::c_int
        || (*p0).y
            > CanvasHeight
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
        || mp.y < 0 as ::core::ffi::c_int
        || x0 < 0 as ::core::ffi::c_int && y0 < 0 as ::core::ffi::c_int)
    {
        if PointOnScreen((*p0).x, mp.y).0 != 0 || PointOnScreen(mp.x, (*p0).y).0 != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
        } else {
            x = (0 as SDWORD - mp.x) as ::core::ffi::c_double;
            y = (0 as SDWORD - mp.y) as ::core::ffi::c_double;
            r0 = x * x / asquared + y * y / bsquared;
            x = (if CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) < mp.x
            {
                CanvasWidth - ((64 as SDWORD) << resolutionFactor)
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
                        } << resolutionFactor))
            } else {
                mp.x
            } - mp.x) as ::core::ffi::c_double;
            y = (if CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor) < mp.y
            {
                CanvasHeight
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
                    - ((13 as SDWORD) << resolutionFactor)
            } else {
                mp.y
            } - mp.y) as ::core::ffi::c_double;
            r1 = x * x / asquared + y * y / bsquared;
            if r0 >= 1.0f64 && r1 <= 1.0f64 {
                quad_visible = (quad_visible as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
            }
        }
    }
    if !((*p0).x
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) || mp.x < 0 as ::core::ffi::c_int
        || mp.y
            > CanvasHeight
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
        || (*p1).y < 0 as ::core::ffi::c_int
        || x0 < 0 as ::core::ffi::c_int
            && y1
                > CanvasHeight
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
    {
        if PointOnScreen((*p0).x, mp.y).0 != 0 || PointOnScreen(mp.x, (*p1).y).0 != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
        } else {
            x = (0 as SDWORD - mp.x) as ::core::ffi::c_double;
            y = (CanvasHeight
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
                - ((13 as SDWORD) << resolutionFactor) - mp.y) as ::core::ffi::c_double;
            r0 = x * x / asquared + y * y / bsquared;
            x = (if CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) < mp.x
            {
                CanvasWidth - ((64 as SDWORD) << resolutionFactor)
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
                        } << resolutionFactor))
            } else {
                mp.x
            } - mp.x) as ::core::ffi::c_double;
            y = (if 0 as ::core::ffi::c_int > mp.y { 0 as SDWORD } else { mp.y } - mp.y)
                as ::core::ffi::c_double;
            r1 = x * x / asquared + y * y / bsquared;
            if r0 >= 1.0f64 && r1 <= 1.0f64 {
                quad_visible = (quad_visible as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
            }
        }
    }
    if !(mp.x
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) || (*p1).x < 0 as ::core::ffi::c_int
        || mp.y
            > CanvasHeight
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
        || (*p1).y < 0 as ::core::ffi::c_int
        || x1
            > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            && y1
                > CanvasHeight
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
    {
        if PointOnScreen(mp.x, (*p1).y).0 != 0 || PointOnScreen((*p1).x, mp.y).0 != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        } else {
            x = (CanvasWidth - ((64 as SDWORD) << resolutionFactor)
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
                    } << resolutionFactor)) - mp.x) as ::core::ffi::c_double;
            y = (CanvasHeight
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
                - ((13 as SDWORD) << resolutionFactor) - mp.y) as ::core::ffi::c_double;
            r0 = x * x / asquared + y * y / bsquared;
            x = (if 0 as ::core::ffi::c_int > mp.x { 0 as SDWORD } else { mp.x } - mp.x)
                as ::core::ffi::c_double;
            y = (if 0 as ::core::ffi::c_int > mp.y { 0 as SDWORD } else { mp.y } - mp.y)
                as ::core::ffi::c_double;
            r1 = x * x / asquared + y * y / bsquared;
            if r0 >= 1.0f64 && r1 <= 1.0f64 {
                quad_visible = (quad_visible as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
            }
        }
    }
    return quad_visible;
}
#[no_mangle]
pub unsafe extern "C" fn DrawOval(
    mut pRect: *mut DRECT,
    mut num_off_pixels: BYTE,
    mut scaled: BOOLEAN,
) {
    let mut off: COUNT = 0;
    let mut x: COORD = 0;
    let mut y: COORD = 0;
    let mut A: SIZE = 0;
    let mut B: SIZE = 0;
    let mut Asquared: SQWORD = 0;
    let mut TwoAsquared: SQWORD = 0;
    let mut Bsquared: SQWORD = 0;
    let mut TwoBsquared: SQWORD = 0;
    let mut d: SQWORD = 0;
    let mut dx: SQWORD = 0;
    let mut dy: SQWORD = 0;
    let mut quad_visible: BYTE = 0;
    let mut prim: [PRIMITIVE; 4] = [PRIMITIVE {
        Links: 0,
        Type: 0,
        color: Color { r: 0, g: 0, b: 0, a: 0 },
        Object: PRIM_DESC {
            Point: POINT { x: 0, y: 0 },
        },
        flags: 0,
    }; 4];
    let mut StartPrim: COUNT = 0;
    let mut p0: DPOINT = DPOINT { x: 0, y: 0 };
    let mut p1: DPOINT = DPOINT { x: 0, y: 0 };
    let mut use_pointprim: BOOLEAN = BOOLEAN(
        (scaled.0 == 0
            && num_off_pixels as ::core::ffi::c_int <= 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut render: BYTE = 0;
    p0.x = (*pRect).corner.x;
    p0.y = (*pRect).corner.y;
    p1.x = (p0.x + (*pRect).extent.width - 1 as ::core::ffi::c_int) as SDWORD;
    p1.y = (p0.y + (*pRect).extent.height - 1 as ::core::ffi::c_int) as SDWORD;
    if p1.x <= p0.x || p1.y <= p0.y {
        let mut corners: LINE = LINE {
            first: POINT { x: 0, y: 0 },
            second: POINT { x: 0, y: 0 },
        };
        if p1.x < p0.x {
            p1.x = p0.x;
        }
        if p1.y < p0.y {
            p1.y = p0.y;
        }
        TruncateDPoint(&raw mut p0);
        TruncateDPoint(&raw mut p1);
        corners.first.x = p0.x as COORD;
        corners.first.y = p0.y as COORD;
        corners.second.x = p1.x as COORD;
        corners.second.y = p1.y as COORD;
        uqm_lua_checked!(DrawLine(&raw mut corners, 1 as BYTE), ());
        return;
    }
    quad_visible = CheckOvalCollision(&raw mut p0, &raw mut p1);
    if quad_visible == 0 {
        return;
    }
    StartPrim = 0xffff as ::core::ffi::c_int as COUNT;
    x = 0 as COORD;
    while (x as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        if quad_visible as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << x as ::core::ffi::c_int != 0
        {
            SetPrimNextLink(
                (&raw mut prim as *mut PRIMITIVE).offset(x as isize),
                StartPrim,
            );
            prim[x as usize].Type = (if use_pointprim.0 != 0 {
                gfx_object::POINT_PRIM.0 as ::core::ffi::c_int
            } else {
                gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
            }) as GRAPHICS_PRIM;
            prim[x as usize].Object.Stamp.frame = DecFrameIndex(stars_in_space);
            prim[x as usize].color = (*_pCurContext).ForeGroundColor;
            prim[x as usize].flags = 0 as BYTE;
            StartPrim = x as COUNT;
        } else {
            prim[x as usize].Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
        }
        x += 1;
    }
    A = ((*pRect).extent.width >> 1 as ::core::ffi::c_int) as SIZE;
    B = ((*pRect).extent.height >> 1 as ::core::ffi::c_int) as SIZE;
    x = 0 as COORD;
    y = B as COORD;
    Asquared = (A as SQWORD * A as SQWORD) << 1 as ::core::ffi::c_int;
    Bsquared = (B as SQWORD * B as SQWORD) << 1 as ::core::ffi::c_int;
    loop {
        Asquared >>= 1 as ::core::ffi::c_int;
        Bsquared >>= 1 as ::core::ffi::c_int;
        TwoAsquared = Asquared << 1 as ::core::ffi::c_int;
        dy = TwoAsquared * B as SQWORD;
        if dy / B as SQWORD == TwoAsquared {
            break;
        }
    }
    TwoBsquared = Bsquared << 1 as ::core::ffi::c_int;
    dx = 0 as SQWORD;
    d = Bsquared - (dy >> 1 as ::core::ffi::c_int)
        + (Asquared >> 2 as ::core::ffi::c_int);
    off = 0 as COUNT;
    A = (A as ::core::ffi::c_int + (*pRect).corner.x as ::core::ffi::c_int) as SIZE;
    B = (B as ::core::ffi::c_int + (*pRect).corner.y as ::core::ffi::c_int) as SIZE;
    if use_pointprim.0 != 0 {
        while dx < dy {
            let c2rust_fresh0 = off;
            off = off.wrapping_sub(1);
            if c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                render = 0 as BYTE;
                prim[3usize].Object.Point.x = (A as ::core::ffi::c_int
                    + x as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Point.x = prim[3usize].Object.Point.x;
                prim[1usize].Object.Point.y = (B as ::core::ffi::c_int
                    - y as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Point.y = prim[1usize].Object.Point.y;
                prim[2usize].Object.Point.x = (A as ::core::ffi::c_int
                    - x as ::core::ffi::c_int) as COORD;
                prim[1usize].Object.Point.x = prim[2usize].Object.Point.x;
                prim[3usize].Object.Point.y = (B as ::core::ffi::c_int
                    + y as ::core::ffi::c_int) as COORD;
                prim[2usize].Object.Point.y = prim[3usize].Object.Point.y;
                if quad_visible as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                {
                    uqm_lua_checked!(DrawBatch(
                        &raw mut prim as *mut PRIMITIVE,
                        StartPrim as PRIM_LINKS,
                        0 as BATCH_FLAGS,
                    ), ());
                } else {
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[0usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[0usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[1usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[1usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[2usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[2usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[3usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[3usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if render as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        uqm_lua_checked!(DrawBatch(
                            &raw mut prim as *mut PRIMITIVE,
                            StartPrim as PRIM_LINKS,
                            0 as BATCH_FLAGS,
                        ), ());
                    }
                }
                off = num_off_pixels as COUNT;
            }
            if d > 0 as SQWORD {
                y -= 1;
                dy -= TwoAsquared;
                d -= dy;
            }
            x += 1;
            dx += TwoBsquared;
            d += Bsquared + dx;
        }
    } else {
        while dx < dy {
            let c2rust_fresh1 = off;
            off = off.wrapping_sub(1);
            if c2rust_fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                render = 0 as BYTE;
                prim[3usize].Object.Stamp.origin.x = (A as ::core::ffi::c_int
                    + x as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Stamp.origin.x = prim[3usize].Object.Stamp.origin.x;
                prim[1usize].Object.Stamp.origin.y = (B as ::core::ffi::c_int
                    - y as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Stamp.origin.y = prim[1usize].Object.Stamp.origin.y;
                prim[2usize].Object.Stamp.origin.x = (A as ::core::ffi::c_int
                    - x as ::core::ffi::c_int) as COORD;
                prim[1usize].Object.Stamp.origin.x = prim[2usize].Object.Stamp.origin.x;
                prim[3usize].Object.Stamp.origin.y = (B as ::core::ffi::c_int
                    + y as ::core::ffi::c_int) as COORD;
                prim[2usize].Object.Stamp.origin.y = prim[3usize].Object.Stamp.origin.y;
                if quad_visible as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                {
                    uqm_lua_checked!(DrawBatch(
                        &raw mut prim as *mut PRIMITIVE,
                        StartPrim as PRIM_LINKS,
                        0 as BATCH_FLAGS,
                    ), ());
                } else {
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[0usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[0usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[1usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[1usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[2usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[2usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[3usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[3usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if render as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        uqm_lua_checked!(DrawBatch(
                            &raw mut prim as *mut PRIMITIVE,
                            StartPrim as PRIM_LINKS,
                            0 as BATCH_FLAGS,
                        ), ());
                    }
                }
                off = num_off_pixels as COUNT;
            }
            if d > 0 as SQWORD {
                y -= 1;
                dy -= TwoAsquared;
                d -= dy;
            }
            x += 1;
            dx += TwoBsquared;
            d += Bsquared + dx;
        }
    }
    d
        += ((Asquared - Bsquared) * 3 as SQWORD >> 1 as ::core::ffi::c_int) - (dx + dy)
            >> 1 as ::core::ffi::c_int;
    if use_pointprim.0 != 0 {
        while (y as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
            let c2rust_fresh2 = off;
            off = off.wrapping_sub(1);
            if c2rust_fresh2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                render = 0 as BYTE;
                prim[3usize].Object.Point.x = (A as ::core::ffi::c_int
                    + x as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Point.x = prim[3usize].Object.Point.x;
                prim[1usize].Object.Point.y = (B as ::core::ffi::c_int
                    - y as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Point.y = prim[1usize].Object.Point.y;
                prim[2usize].Object.Point.x = (A as ::core::ffi::c_int
                    - x as ::core::ffi::c_int) as COORD;
                prim[1usize].Object.Point.x = prim[2usize].Object.Point.x;
                prim[3usize].Object.Point.y = (B as ::core::ffi::c_int
                    + y as ::core::ffi::c_int) as COORD;
                prim[2usize].Object.Point.y = prim[3usize].Object.Point.y;
                if quad_visible as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                {
                    uqm_lua_checked!(DrawBatch(
                        &raw mut prim as *mut PRIMITIVE,
                        StartPrim as PRIM_LINKS,
                        0 as BATCH_FLAGS,
                    ), ());
                } else {
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[0usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[0usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[1usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[1usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[2usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[2usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[3usize].Type = gfx_object::POINT_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[3usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if render as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        uqm_lua_checked!(DrawBatch(
                            &raw mut prim as *mut PRIMITIVE,
                            StartPrim as PRIM_LINKS,
                            0 as BATCH_FLAGS,
                        ), ());
                    }
                }
                off = num_off_pixels as COUNT;
            }
            if d < 0 as SQWORD {
                x += 1;
                dx += TwoBsquared;
                d += dx;
            }
            y -= 1;
            dy -= TwoAsquared;
            d += Asquared - dy;
        }
    } else {
        while (y as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
            let c2rust_fresh3 = off;
            off = off.wrapping_sub(1);
            if c2rust_fresh3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                render = 0 as BYTE;
                prim[3usize].Object.Stamp.origin.x = (A as ::core::ffi::c_int
                    + x as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Stamp.origin.x = prim[3usize].Object.Stamp.origin.x;
                prim[1usize].Object.Stamp.origin.y = (B as ::core::ffi::c_int
                    - y as ::core::ffi::c_int) as COORD;
                prim[0usize].Object.Stamp.origin.y = prim[1usize].Object.Stamp.origin.y;
                prim[2usize].Object.Stamp.origin.x = (A as ::core::ffi::c_int
                    - x as ::core::ffi::c_int) as COORD;
                prim[1usize].Object.Stamp.origin.x = prim[2usize].Object.Stamp.origin.x;
                prim[3usize].Object.Stamp.origin.y = (B as ::core::ffi::c_int
                    + y as ::core::ffi::c_int) as COORD;
                prim[2usize].Object.Stamp.origin.y = prim[3usize].Object.Stamp.origin.y;
                if quad_visible as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                {
                    uqm_lua_checked!(DrawBatch(
                        &raw mut prim as *mut PRIMITIVE,
                        StartPrim as PRIM_LINKS,
                        0 as BATCH_FLAGS,
                    ), ());
                } else {
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[0usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[0usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD - y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[1usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[1usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD - x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[2usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[2usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if quad_visible as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        if PointOnScreen(
                                A as SDWORD + x as SDWORD,
                                B as SDWORD + y as SDWORD,
                            )
                            .0 != 0
                        {
                            prim[3usize].Type = gfx_object::STAMPFILL_PRIM.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                            render = render.wrapping_add(1);
                        } else {
                            prim[3usize].Type = gfx_object::NUM_PRIMS.0
                                as ::core::ffi::c_int as GRAPHICS_PRIM;
                        }
                    }
                    if render as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        uqm_lua_checked!(DrawBatch(
                            &raw mut prim as *mut PRIMITIVE,
                            StartPrim as PRIM_LINKS,
                            0 as BATCH_FLAGS,
                        ), ());
                    }
                }
                off = num_off_pixels as COUNT;
            }
            if d < 0 as SQWORD {
                x += 1;
                dx += TwoBsquared;
                d += dx;
            }
            y -= 1;
            dy -= TwoAsquared;
            d += Asquared - dy;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawFilledOval(mut pRect: *mut DRECT) {
    let mut x: COORD = 0;
    let mut y: COORD = 0;
    let mut A: SIZE = 0;
    let mut B: SIZE = 0;
    let mut Asquared: SQWORD = 0;
    let mut TwoAsquared: SQWORD = 0;
    let mut Bsquared: SQWORD = 0;
    let mut TwoBsquared: SQWORD = 0;
    let mut d: SQWORD = 0;
    let mut dx: SQWORD = 0;
    let mut dy: SQWORD = 0;
    let mut prim: [PRIMITIVE; 2] = [PRIMITIVE {
        Links: 0,
        Type: 0,
        color: Color { r: 0, g: 0, b: 0, a: 0 },
        Object: PRIM_DESC {
            Point: POINT { x: 0, y: 0 },
        },
        flags: 0,
    }; 2];
    let mut StartPrim: COUNT = 0;
    let mut first: POINT = POINT { x: 0, y: 0 };
    let mut second: POINT = POINT { x: 0, y: 0 };
    let mut p0: DPOINT = DPOINT { x: 0, y: 0 };
    let mut p1: DPOINT = DPOINT { x: 0, y: 0 };
    let mut lines_r: COUNT = 0 as COUNT;
    p0.x = (*pRect).corner.x;
    p0.y = (*pRect).corner.y;
    p1.x = (p0.x + (*pRect).extent.width - 1 as ::core::ffi::c_int) as SDWORD;
    p1.y = (p0.y + (*pRect).extent.height - 1 as ::core::ffi::c_int) as SDWORD;
    if p1.x <= p0.x || p1.y <= p0.y {
        let mut corners: LINE = LINE {
            first: POINT { x: 0, y: 0 },
            second: POINT { x: 0, y: 0 },
        };
        if p1.x < p0.x {
            p1.x = p0.x;
        }
        if p1.y < p0.y {
            p1.y = p0.y;
        }
        TruncateDPoint(&raw mut p0);
        TruncateDPoint(&raw mut p1);
        corners.first.x = p0.x as COORD;
        corners.first.y = p0.y as COORD;
        corners.second.x = p1.x as COORD;
        corners.second.y = p1.y as COORD;
        uqm_lua_checked!(DrawLine(&raw mut corners, 1 as BYTE), ());
        return;
    }
    StartPrim = 0xffff as ::core::ffi::c_int as COUNT;
    x = 0 as COORD;
    while (x as ::core::ffi::c_int) < 4 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
    {
        SetPrimNextLink((&raw mut prim as *mut PRIMITIVE).offset(x as isize), StartPrim);
        prim[x as usize].Type = gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
            as GRAPHICS_PRIM;
        prim[x as usize].color = (*_pCurContext).ForeGroundColor;
        StartPrim = x as COUNT;
        x += 1;
    }
    A = ((*pRect).extent.width >> 1 as ::core::ffi::c_int) as SIZE;
    B = ((*pRect).extent.height >> 1 as ::core::ffi::c_int) as SIZE;
    x = 0 as COORD;
    y = B as COORD;
    Asquared = (A as SQWORD * A as SQWORD) << 1 as ::core::ffi::c_int;
    Bsquared = (B as SQWORD * B as SQWORD) << 1 as ::core::ffi::c_int;
    loop {
        Asquared >>= 1 as ::core::ffi::c_int;
        Bsquared >>= 1 as ::core::ffi::c_int;
        TwoAsquared = Asquared << 1 as ::core::ffi::c_int;
        dy = TwoAsquared * B as SQWORD;
        if dy / B as SQWORD == TwoAsquared {
            break;
        }
    }
    TwoBsquared = Bsquared << 1 as ::core::ffi::c_int;
    dx = 0 as SQWORD;
    d = Bsquared - (dy >> 1 as ::core::ffi::c_int)
        + (Asquared >> 2 as ::core::ffi::c_int);
    A = (A as ::core::ffi::c_int + (*pRect).corner.x as ::core::ffi::c_int) as SIZE;
    B = (B as ::core::ffi::c_int + (*pRect).corner.y as ::core::ffi::c_int) as SIZE;
    while dx < dy {
        if d > 0 as SQWORD {
            lines_r = 0 as COUNT;
            first.x = (if A as ::core::ffi::c_int - x as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                A as ::core::ffi::c_int - x as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
            second.x = (if (A as ::core::ffi::c_int - x as ::core::ffi::c_int
                + ((x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int)
                < CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            {
                A as ::core::ffi::c_int - x as ::core::ffi::c_int
                    + ((x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int
            } else {
                CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            }) as COORD;
            second.y = (B as ::core::ffi::c_int - y as ::core::ffi::c_int) as COORD;
            first.y = second.y;
            if B as ::core::ffi::c_int - y as ::core::ffi::c_int
                >= 0 as ::core::ffi::c_int
                && B as ::core::ffi::c_int - y as ::core::ffi::c_int
                    <= CanvasHeight
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
                prim[0usize].Type = gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
                prim[0usize].Object.Line.first = first;
                prim[0usize].Object.Line.second = second;
                lines_r = lines_r.wrapping_add(1);
            } else {
                prim[0usize].Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
            }
            second.y = (B as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
            first.y = second.y;
            if B as ::core::ffi::c_int + y as ::core::ffi::c_int
                >= 0 as ::core::ffi::c_int
                && B as ::core::ffi::c_int + y as ::core::ffi::c_int
                    <= CanvasHeight
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
                prim[1usize].Type = gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
                prim[1usize].Object.Line.first = first;
                prim[1usize].Object.Line.second = second;
                lines_r = lines_r.wrapping_add(1);
            } else {
                prim[1usize].Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
            }
            if lines_r as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DrawBatch(
                    &raw mut prim as *mut PRIMITIVE,
                    StartPrim as PRIM_LINKS,
                    0 as BATCH_FLAGS,
                ), ());
            }
            y -= 1;
            dy -= TwoAsquared;
            d -= dy;
        }
        x += 1;
        dx += TwoBsquared;
        d += Bsquared + dx;
    }
    d
        += ((Asquared - Bsquared) * 3 as SQWORD >> 1 as ::core::ffi::c_int) - (dx + dy)
            >> 1 as ::core::ffi::c_int;
    while (y as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
        lines_r = 0 as COUNT;
        first.x = (if A as ::core::ffi::c_int - x as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            A as ::core::ffi::c_int - x as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
        second.x = (if (A as ::core::ffi::c_int - x as ::core::ffi::c_int
            + ((x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int)
            < CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        {
            A as ::core::ffi::c_int - x as ::core::ffi::c_int
                + ((x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int
        } else {
            CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        }) as COORD;
        second.y = (B as ::core::ffi::c_int - y as ::core::ffi::c_int) as COORD;
        first.y = second.y;
        if B as ::core::ffi::c_int - y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && B as ::core::ffi::c_int - y as ::core::ffi::c_int
                <= CanvasHeight
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
            prim[0usize].Type = gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            prim[0usize].Object.Line.first = first;
            prim[0usize].Object.Line.second = second;
            lines_r = lines_r.wrapping_add(1);
        } else {
            prim[0usize].Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
        }
        second.y = (B as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
        first.y = second.y;
        if B as ::core::ffi::c_int + y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && B as ::core::ffi::c_int + y as ::core::ffi::c_int
                <= CanvasHeight
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
            prim[1usize].Type = gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            prim[1usize].Object.Line.first = first;
            prim[1usize].Object.Line.second = second;
            lines_r = lines_r.wrapping_add(1);
        } else {
            prim[1usize].Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
        }
        if lines_r as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawBatch(
                &raw mut prim as *mut PRIMITIVE,
                StartPrim as PRIM_LINKS,
                0 as BATCH_FLAGS,
            ), ());
        }
        if d < 0 as SQWORD {
            x += 1;
            dx += TwoBsquared;
            d += dx;
        }
        y -= 1;
        dy -= TwoAsquared;
        d += Asquared - dy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawEllipseQuadrants(
    mut cx: ::core::ffi::c_int,
    mut cy: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut shear: ::core::ffi::c_int,
    mut filled: ::core::ffi::c_int,
) {
    let mut p: POINT = POINT { x: 0, y: 0 };
    let mut l: LINE = LINE {
        first: POINT { x: 0, y: 0 },
        second: POINT { x: 0, y: 0 },
    };
    if filled != 0 && y != 0 as ::core::ffi::c_int {
        l.second.x = (cx - x) as COORD;
        l.first.x = l.second.x;
        l.first.y = (cy - shear - y) as COORD;
        l.second.y = (cy - shear + y) as COORD;
        uqm_lua_checked!(DrawLine(&raw mut l, 1 as BYTE), ());
        if x != 0 as ::core::ffi::c_int {
            l.second.x = (cx + x) as COORD;
            l.first.x = l.second.x;
            l.first.y = (cy + shear - y) as COORD;
            l.second.y = (cy + shear + y) as COORD;
            uqm_lua_checked!(DrawLine(&raw mut l, 1 as BYTE), ());
        }
    } else {
        p.x = (cx - x) as COORD;
        p.y = (cy - shear - y) as COORD;
        uqm_lua_checked!(DrawPoint(&raw mut p), ());
        p.y = (cy - shear + y) as COORD;
        uqm_lua_checked!(DrawPoint(&raw mut p), ());
        if x != 0 as ::core::ffi::c_int {
            p.x = (cx + x) as COORD;
            p.y = (cy + shear - y) as COORD;
            uqm_lua_checked!(DrawPoint(&raw mut p), ());
            p.y = (cy + shear + y) as COORD;
            uqm_lua_checked!(DrawPoint(&raw mut p), ());
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawEllipse(
    mut cx: ::core::ffi::c_int,
    mut cy: ::core::ffi::c_int,
    mut rx: ::core::ffi::c_int,
    mut ry: ::core::ffi::c_int,
    mut shear: ::core::ffi::c_int,
    mut filled: ::core::ffi::c_int,
    mut dotted: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = rx;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_int = shear;
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let sRound: ::core::ffi::c_int = if shear < 0 as ::core::ffi::c_int {
        -(rx / 2 as ::core::ffi::c_int)
    } else {
        rx / 2 as ::core::ffi::c_int
    };
    let mut dex: SQWORD = (2 as ::core::ffi::c_long * ry as ::core::ffi::c_long
        * ry as ::core::ffi::c_long) as SQWORD;
    let mut ex: SQWORD = (ry as ::core::ffi::c_long * ry as ::core::ffi::c_long)
        as SQWORD - rx as ::core::ffi::c_long as SQWORD * dex;
    let mut dey: SQWORD = (2 as ::core::ffi::c_long * rx as ::core::ffi::c_long
        * rx as ::core::ffi::c_long) as SQWORD;
    let mut ey: SQWORD = (rx as ::core::ffi::c_long * rx as ::core::ffi::c_long)
        as SQWORD;
    let mut e: SQWORD = ex + ey;
    let mut e2: SQWORD = 0;
    if rx < 0 as ::core::ffi::c_int {
        rx = 0 as ::core::ffi::c_int;
    }
    if ry < 0 as ::core::ffi::c_int {
        ry = 0 as ::core::ffi::c_int;
    }
    if rx == 0 || ry == 0 {
        let mut l: LINE = LINE {
            first: POINT { x: 0, y: 0 },
            second: POINT { x: 0, y: 0 },
        };
        l.first.x = (cx - rx) as COORD;
        l.first.y = (cy - ry - shear) as COORD;
        l.second.x = (cx + rx) as COORD;
        l.second.y = (cy + ry + shear) as COORD;
        uqm_lua_checked!(DrawLine(&raw mut l, 1 as BYTE), ());
        return;
    }
    loop {
        if filled == 0
            && {
                let c2rust_fresh4 = d;
                d -= 1;
                c2rust_fresh4 == 0
            }
        {
            d = dotted;
            s = ((x as SQWORD * shear as SQWORD + sRound as SQWORD) / rx as SQWORD)
                as ::core::ffi::c_int;
            uqm_lua_checked!(DrawEllipseQuadrants(cx, cy, x, y, -s, 0 as ::core::ffi::c_int), ());
        }
        e2 = e * 2 as SQWORD;
        if e2 >= ex {
            if filled != 0 {
                s = ((x as SQWORD * shear as SQWORD + sRound as SQWORD) / rx as SQWORD)
                    as ::core::ffi::c_int;
                uqm_lua_checked!(DrawEllipseQuadrants(cx, cy, x, y, -s, 1 as ::core::ffi::c_int), ());
            }
            x -= 1;
            ex += dex;
            e += ex;
        }
        if e2 <= ey {
            y += 1;
            ey += dey;
            e += ey;
        }
        if x <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    uqm_lua_checked!(DrawEllipseQuadrants(
        cx,
        cy,
        0 as ::core::ffi::c_int,
        ry,
        0 as ::core::ffi::c_int,
        filled,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawRotatedEllipse(
    mut cx: ::core::ffi::c_int,
    mut cy: ::core::ffi::c_int,
    mut rx: ::core::ffi::c_int,
    mut ry: ::core::ffi::c_int,
    mut angle_deg: ::core::ffi::c_int,
    mut filled: ::core::ffi::c_int,
    mut dotted: ::core::ffi::c_int,
) {
    let mut rx2: ::core::ffi::c_double = rx as ::core::ffi::c_double
        * rx as ::core::ffi::c_double;
    let mut ry2: ::core::ffi::c_double = ry as ::core::ffi::c_double
        * ry as ::core::ffi::c_double;
    let mut theta: ::core::ffi::c_double = (angle_deg % 90 as ::core::ffi::c_int)
        as ::core::ffi::c_double * 3.14159265358979323846f64 / 180.0f64;
    let mut st: ::core::ffi::c_double = sin(theta);
    let mut ct: ::core::ffi::c_double = cos(theta);
    let mut xd2: ::core::ffi::c_double = rx2 * ct * ct + ry2 * st * st;
    let mut xd: ::core::ffi::c_double = sqrt(xd2);
    let mut shear: ::core::ffi::c_int = ((rx2 - ry2) * st * ct / xd + 0.5f64)
        as ::core::ffi::c_int;
    rx = (xd + 0.5f64) as ::core::ffi::c_int;
    ry = (sqrt(rx2 * ry2 / xd2) + 0.5f64) as ::core::ffi::c_int;
    uqm_lua_checked!(DrawEllipse(cx, cy, rx, ry, shear, filled, dotted), ());
}
