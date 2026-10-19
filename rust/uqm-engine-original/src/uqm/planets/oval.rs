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
    fn DrawLine(pLine: *mut LINE);
    fn DrawBatch(
        pBasePrim: *mut PRIMITIVE,
        PrimLinks: PRIM_LINKS,
        BatchFlags: BATCH_FLAGS,
    );
    static mut _pCurContext: CONTEXT;
    fn _clip_line(pClipRect: *const RECT, pLine: *mut BRESENHAM_LINE) -> INTERSECT_CODE;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bresenham_line {
    pub first: POINT,
    pub second: POINT,
    pub abs_delta_x: SIZE,
    pub abs_delta_y: SIZE,
    pub error_term: SIZE,
    pub end_points_exchanged: BOOLEAN,
    pub intersect_code: INTERSECT_CODE,
}
pub type BRESENHAM_LINE = bresenham_line;
#[inline]
unsafe extern "C" fn SetPrimNextLink(mut pPrim: *mut PRIMITIVE, mut Link: COUNT) {
    (*pPrim).Links = ((Link as DWORD) << 16 as ::core::ffi::c_int
        | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
}
#[no_mangle]
pub unsafe extern "C" fn DrawOval(mut pRect: *mut RECT, mut num_off_pixels: BYTE) {
    let mut off: COUNT = 0;
    let mut x: COORD = 0;
    let mut y: COORD = 0;
    let mut A: SIZE = 0;
    let mut B: SIZE = 0;
    let mut Asquared: ::core::ffi::c_long = 0;
    let mut TwoAsquared: ::core::ffi::c_long = 0;
    let mut Bsquared: ::core::ffi::c_long = 0;
    let mut TwoBsquared: ::core::ffi::c_long = 0;
    let mut d: ::core::ffi::c_long = 0;
    let mut dx: ::core::ffi::c_long = 0;
    let mut dy: ::core::ffi::c_long = 0;
    let mut quad_visible: BYTE = 0;
    let mut corners: LINE = LINE {
        first: POINT { x: 0, y: 0 },
        second: POINT { x: 0, y: 0 },
    };
    let mut mp: POINT = POINT { x: 0, y: 0 };
    let mut prim: [PRIMITIVE; 4] = [PRIMITIVE {
        Links: 0,
        Type: 0,
        color: Color { r: 0, g: 0, b: 0, a: 0 },
        Object: PRIM_DESC {
            Point: POINT { x: 0, y: 0 },
        },
    }; 4];
    let mut StartPrim: COUNT = 0;
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut ClipLine: BRESENHAM_LINE = BRESENHAM_LINE {
        first: POINT { x: 0, y: 0 },
        second: POINT { x: 0, y: 0 },
        abs_delta_x: 0,
        abs_delta_y: 0,
        error_term: 0,
        end_points_exchanged: BOOLEAN::FALSE,
        intersect_code: 0,
    };
    ClipRect.corner.y = 0 as COORD;
    ClipRect.corner.x = ClipRect.corner.y;
    corners.first.x = ((*pRect).corner.x as ::core::ffi::c_int
        - ClipRect.corner.x as ::core::ffi::c_int) as COORD;
    corners.first.y = ((*pRect).corner.y as ::core::ffi::c_int
        - ClipRect.corner.y as ::core::ffi::c_int) as COORD;
    corners.second.x = (corners.first.x as ::core::ffi::c_int
        + (*pRect).extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    corners.second.y = (corners.first.y as ::core::ffi::c_int
        + (*pRect).extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    if corners.second.x as ::core::ffi::c_int <= corners.first.x as ::core::ffi::c_int
        || corners.second.y as ::core::ffi::c_int
            <= corners.first.y as ::core::ffi::c_int
    {
        if (corners.second.x as ::core::ffi::c_int)
            < corners.first.x as ::core::ffi::c_int
        {
            corners.second.x = corners.first.x;
        }
        if (corners.second.y as ::core::ffi::c_int)
            < corners.first.y as ::core::ffi::c_int
        {
            corners.second.y = corners.first.y;
        }
        DrawLine(&raw mut corners);
        return;
    }
    ClipRect.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        as COORD;
    ClipRect.extent.height = (ScreenHeight
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        as COORD;
    quad_visible = 0 as BYTE;
    mp.x = (corners.first.x as ::core::ffi::c_int
        + corners.second.x as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    mp.y = (corners.first.y as ::core::ffi::c_int
        + corners.second.y as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    ClipRect.corner.y = 0 as COORD;
    ClipRect.corner.x = ClipRect.corner.y;
    if corners.first.y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.first.y as ::core::ffi::c_int)
            < ClipRect.extent.height as ::core::ffi::c_int
        && corners.second.x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.second.x as ::core::ffi::c_int)
            < ClipRect.extent.width as ::core::ffi::c_int
    {
        quad_visible = (quad_visible as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else {
        ClipLine.first.x = mp.x;
        ClipLine.first.y = corners.first.y;
        ClipLine.second.x = corners.second.x;
        ClipLine.second.y = mp.y;
        if _clip_line(&raw mut ClipRect, &raw mut ClipLine) != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        }
    }
    if corners.first.y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.first.y as ::core::ffi::c_int)
            < ClipRect.extent.height as ::core::ffi::c_int
        && corners.first.x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.first.x as ::core::ffi::c_int)
            < ClipRect.extent.width as ::core::ffi::c_int
    {
        quad_visible = (quad_visible as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else {
        ClipLine.first.x = mp.x;
        ClipLine.first.y = corners.first.y;
        ClipLine.second.x = corners.first.x;
        ClipLine.second.y = mp.y;
        if _clip_line(&raw mut ClipRect, &raw mut ClipLine) != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
        }
    }
    if corners.second.y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.second.y as ::core::ffi::c_int)
            < ClipRect.extent.height as ::core::ffi::c_int
        && corners.first.x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.first.x as ::core::ffi::c_int)
            < ClipRect.extent.width as ::core::ffi::c_int
    {
        quad_visible = (quad_visible as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else {
        ClipLine.first.x = mp.x;
        ClipLine.first.y = corners.second.y;
        ClipLine.second.x = corners.first.x;
        ClipLine.second.y = mp.y;
        if _clip_line(&raw mut ClipRect, &raw mut ClipLine) != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
        }
    }
    if corners.second.y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.second.y as ::core::ffi::c_int)
            < ClipRect.extent.height as ::core::ffi::c_int
        && corners.second.x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (corners.second.x as ::core::ffi::c_int)
            < ClipRect.extent.width as ::core::ffi::c_int
    {
        quad_visible = (quad_visible as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    } else {
        ClipLine.first.x = mp.x;
        ClipLine.first.y = corners.second.y;
        ClipLine.second.x = corners.second.x;
        ClipLine.second.y = mp.y;
        if _clip_line(&raw mut ClipRect, &raw mut ClipLine) != 0 {
            quad_visible = (quad_visible as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        }
    }
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
            prim[x as usize].Type = gfx_object::POINT_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            prim[x as usize].color = (*_pCurContext).ForeGroundColor;
            StartPrim = x as COUNT;
        }
        x += 1;
    }
    A = ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    B = ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as SIZE;
    x = 0 as COORD;
    y = B as COORD;
    Asquared = (A as ::core::ffi::c_long * A as ::core::ffi::c_long)
        << 1 as ::core::ffi::c_int;
    Bsquared = (B as ::core::ffi::c_long * B as ::core::ffi::c_long)
        << 1 as ::core::ffi::c_int;
    loop {
        Asquared >>= 1 as ::core::ffi::c_int;
        Bsquared >>= 1 as ::core::ffi::c_int;
        TwoAsquared = Asquared << 1 as ::core::ffi::c_int;
        dy = TwoAsquared * B as ::core::ffi::c_long;
        if dy / B as ::core::ffi::c_long == TwoAsquared {
            break;
        }
    }
    TwoBsquared = Bsquared << 1 as ::core::ffi::c_int;
    dx = 0 as ::core::ffi::c_long;
    d = Bsquared - (dy >> 1 as ::core::ffi::c_int)
        + (Asquared >> 2 as ::core::ffi::c_int);
    off = 0 as COUNT;
    A = (A as ::core::ffi::c_int + (*pRect).corner.x as ::core::ffi::c_int) as SIZE;
    B = (B as ::core::ffi::c_int + (*pRect).corner.y as ::core::ffi::c_int) as SIZE;
    while dx < dy {
        let c2rust_fresh0 = off;
        off = off.wrapping_sub(1);
        if c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
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
            DrawBatch(
                &raw mut prim as *mut PRIMITIVE,
                StartPrim as PRIM_LINKS,
                0 as BATCH_FLAGS,
            );
            off = num_off_pixels as COUNT;
        }
        if d > 0 as ::core::ffi::c_long {
            y -= 1;
            dy -= TwoAsquared;
            d -= dy;
        }
        x += 1;
        dx += TwoBsquared;
        d += Bsquared + dx;
    }
    d
        += ((Asquared - Bsquared) * 3 as ::core::ffi::c_long >> 1 as ::core::ffi::c_int)
            - (dx + dy) >> 1 as ::core::ffi::c_int;
    while (y as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
        let c2rust_fresh1 = off;
        off = off.wrapping_sub(1);
        if c2rust_fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
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
            DrawBatch(
                &raw mut prim as *mut PRIMITIVE,
                StartPrim as PRIM_LINKS,
                0 as BATCH_FLAGS,
            );
            off = num_off_pixels as COUNT;
        }
        if d < 0 as ::core::ffi::c_long {
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
pub unsafe extern "C" fn DrawFilledOval(mut pRect: *mut RECT) {
    let mut x: COORD = 0;
    let mut y: COORD = 0;
    let mut A: SIZE = 0;
    let mut B: SIZE = 0;
    let mut Asquared: ::core::ffi::c_long = 0;
    let mut TwoAsquared: ::core::ffi::c_long = 0;
    let mut Bsquared: ::core::ffi::c_long = 0;
    let mut TwoBsquared: ::core::ffi::c_long = 0;
    let mut d: ::core::ffi::c_long = 0;
    let mut dx: ::core::ffi::c_long = 0;
    let mut dy: ::core::ffi::c_long = 0;
    let mut corners: LINE = LINE {
        first: POINT { x: 0, y: 0 },
        second: POINT { x: 0, y: 0 },
    };
    let mut prim: [PRIMITIVE; 2] = [PRIMITIVE {
        Links: 0,
        Type: 0,
        color: Color { r: 0, g: 0, b: 0, a: 0 },
        Object: PRIM_DESC {
            Point: POINT { x: 0, y: 0 },
        },
    }; 2];
    let mut StartPrim: COUNT = 0;
    corners.first.x = (*pRect).corner.x;
    corners.first.y = (*pRect).corner.y;
    corners.second.x = (corners.first.x as ::core::ffi::c_int
        + (*pRect).extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    corners.second.y = (corners.first.y as ::core::ffi::c_int
        + (*pRect).extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    if corners.second.x as ::core::ffi::c_int <= corners.first.x as ::core::ffi::c_int
        || corners.second.y as ::core::ffi::c_int
            <= corners.first.y as ::core::ffi::c_int
    {
        if (corners.second.x as ::core::ffi::c_int)
            < corners.first.x as ::core::ffi::c_int
        {
            corners.second.x = corners.first.x;
        }
        if (corners.second.y as ::core::ffi::c_int)
            < corners.first.y as ::core::ffi::c_int
        {
            corners.second.y = corners.first.y;
        }
        DrawLine(&raw mut corners);
        return;
    }
    StartPrim = 0xffff as ::core::ffi::c_int as COUNT;
    x = 0 as COORD;
    while (x as ::core::ffi::c_int) < 4 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
    {
        SetPrimNextLink((&raw mut prim as *mut PRIMITIVE).offset(x as isize), StartPrim);
        prim[x as usize].Type = gfx_object::RECTFILL_PRIM.0 as ::core::ffi::c_int
            as GRAPHICS_PRIM;
        prim[x as usize].color = (*_pCurContext).ForeGroundColor;
        prim[x as usize].Object.Rect.extent.height = 1 as COORD;
        StartPrim = x as COUNT;
        x += 1;
    }
    A = ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    B = ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as SIZE;
    x = 0 as COORD;
    y = B as COORD;
    Asquared = (A as ::core::ffi::c_long * A as ::core::ffi::c_long)
        << 1 as ::core::ffi::c_int;
    Bsquared = (B as ::core::ffi::c_long * B as ::core::ffi::c_long)
        << 1 as ::core::ffi::c_int;
    loop {
        Asquared >>= 1 as ::core::ffi::c_int;
        Bsquared >>= 1 as ::core::ffi::c_int;
        TwoAsquared = Asquared << 1 as ::core::ffi::c_int;
        dy = TwoAsquared * B as ::core::ffi::c_long;
        if dy / B as ::core::ffi::c_long == TwoAsquared {
            break;
        }
    }
    TwoBsquared = Bsquared << 1 as ::core::ffi::c_int;
    dx = 0 as ::core::ffi::c_long;
    d = Bsquared - (dy >> 1 as ::core::ffi::c_int)
        + (Asquared >> 2 as ::core::ffi::c_int);
    A = (A as ::core::ffi::c_int + (*pRect).corner.x as ::core::ffi::c_int) as SIZE;
    B = (B as ::core::ffi::c_int + (*pRect).corner.y as ::core::ffi::c_int) as SIZE;
    while dx < dy {
        if d > 0 as ::core::ffi::c_long {
            prim[1usize].Object.Rect.corner.x = (A as ::core::ffi::c_int
                - x as ::core::ffi::c_int) as COORD;
            prim[0usize].Object.Rect.corner.x = prim[1usize].Object.Rect.corner.x;
            prim[1usize].Object.Rect.extent.width = (((x as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
            prim[0usize].Object.Rect.extent.width = prim[1usize]
                .Object
                .Rect
                .extent
                .width;
            prim[0usize].Object.Rect.corner.y = (B as ::core::ffi::c_int
                - y as ::core::ffi::c_int) as COORD;
            prim[1usize].Object.Rect.corner.y = (B as ::core::ffi::c_int
                + y as ::core::ffi::c_int) as COORD;
            DrawBatch(
                &raw mut prim as *mut PRIMITIVE,
                StartPrim as PRIM_LINKS,
                0 as BATCH_FLAGS,
            );
            y -= 1;
            dy -= TwoAsquared;
            d -= dy;
        }
        x += 1;
        dx += TwoBsquared;
        d += Bsquared + dx;
    }
    d
        += ((Asquared - Bsquared) * 3 as ::core::ffi::c_long >> 1 as ::core::ffi::c_int)
            - (dx + dy) >> 1 as ::core::ffi::c_int;
    while (y as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
        prim[1usize].Object.Rect.corner.x = (A as ::core::ffi::c_int
            - x as ::core::ffi::c_int) as COORD;
        prim[0usize].Object.Rect.corner.x = prim[1usize].Object.Rect.corner.x;
        prim[1usize].Object.Rect.extent.width = (((x as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
        prim[0usize].Object.Rect.extent.width = prim[1usize].Object.Rect.extent.width;
        prim[0usize].Object.Rect.corner.y = (B as ::core::ffi::c_int
            - y as ::core::ffi::c_int) as COORD;
        prim[1usize].Object.Rect.corner.y = (B as ::core::ffi::c_int
            + y as ::core::ffi::c_int) as COORD;
        DrawBatch(
            &raw mut prim as *mut PRIMITIVE,
            StartPrim as PRIM_LINKS,
            0 as BATCH_FLAGS,
        );
        if d < 0 as ::core::ffi::c_long {
            x += 1;
            dx += TwoBsquared;
            d += dx;
        }
        y -= 1;
        dy -= TwoAsquared;
        d += Asquared - dy;
    }
}
