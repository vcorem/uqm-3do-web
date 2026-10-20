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

// Rust translation of src/libs/graphics/intersec.c.
extern "C" {
    fn TFB_DrawImage_Intersect(
        img1: *mut TFB_Image,
        img1org: POINT,
        img2: *mut TFB_Image,
        img2org: POINT,
        interRect: *const RECT,
    ) -> BOOLEAN;
    fn BoxIntersect(
        pr1: *mut RECT,
        pr2: *mut RECT,
        printer: *mut RECT,
    ) -> INTERSECT_CODE;
    static mut _GraphicsStatusFlags: GRAPHICS_STATUS;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type TFB_Canvas = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type INTERSECT_CODE = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGE_BOX {
    pub Box: RECT,
    pub FramePtr: FRAME,
}
pub type GRAPHICS_STATUS = BYTE;
#[inline]
unsafe extern "C" fn images_intersect(
    mut box1: *mut IMAGE_BOX,
    mut box2: *mut IMAGE_BOX,
    mut rect: *const RECT,
) -> BOOLEAN {
    return TFB_DrawImage_Intersect(
        (*(*box1).FramePtr).image,
        (*box1).Box.corner,
        (*(*box2).FramePtr).image,
        (*box2).Box.corner,
        rect,
    );
}
unsafe extern "C" fn frame_intersect(
    mut pControl0: *mut INTERSECT_CONTROL,
    mut pr0: *mut RECT,
    mut pControl1: *mut INTERSECT_CONTROL,
    mut pr1: *mut RECT,
    mut t0: TIME_VALUE,
    mut t1: TIME_VALUE,
) -> TIME_VALUE {
    enum C2Rust_Block {
        CheckFirstIntersection,
        S_323,
    }
    let mut c2rust_current_block: C2Rust_Block;
    let mut time_error0: SIZE = 0;
    let mut time_error1: SIZE = 0;
    let mut cycle0: SIZE = 0;
    let mut cycle1: SIZE = 0;
    let mut dx_0: SIZE = 0;
    let mut dy_0: SIZE = 0;
    let mut dx_1: SIZE = 0;
    let mut dy_1: SIZE = 0;
    let mut xincr0: SIZE = 0;
    let mut yincr0: SIZE = 0;
    let mut xincr1: SIZE = 0;
    let mut yincr1: SIZE = 0;
    let mut xerror0: SIZE = 0;
    let mut xerror1: SIZE = 0;
    let mut yerror0: SIZE = 0;
    let mut yerror1: SIZE = 0;
    let mut r_intersect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut IB0: IMAGE_BOX = IMAGE_BOX {
        Box: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        FramePtr: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut IB1: IMAGE_BOX = IMAGE_BOX {
        Box: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        FramePtr: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut check0: BOOLEAN = BOOLEAN::FALSE;
    let mut check1: BOOLEAN = BOOLEAN::FALSE;
    IB0.FramePtr = (*pControl0).IntersectStamp.frame;
    IB0.Box.corner = (*pr0).corner;
    IB0.Box.extent.width = (*IB0.FramePtr).Bounds.width;
    IB0.Box.extent.height = (*IB0.FramePtr).Bounds.height;
    IB1.FramePtr = (*pControl1).IntersectStamp.frame;
    IB1.Box.corner = (*pr1).corner;
    IB1.Box.extent.width = (*IB1.FramePtr).Bounds.width;
    IB1.Box.extent.height = (*IB1.FramePtr).Bounds.height;
    dx_0 = (*pr0).extent.width as SIZE;
    dy_0 = (*pr0).extent.height as SIZE;
    if dx_0 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        xincr0 = 1 as SIZE;
    } else {
        xincr0 = -1 as SIZE;
        dx_0 = -(dx_0 as ::core::ffi::c_int) as SIZE;
    }
    if dy_0 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        yincr0 = 1 as SIZE;
    } else {
        yincr0 = -1 as SIZE;
        dy_0 = -(dy_0 as ::core::ffi::c_int) as SIZE;
    }
    if dx_0 as ::core::ffi::c_int >= dy_0 as ::core::ffi::c_int {
        cycle0 = dx_0;
    } else {
        cycle0 = dy_0;
    }
    yerror0 = cycle0;
    xerror0 = yerror0;
    dx_1 = (*pr1).extent.width as SIZE;
    dy_1 = (*pr1).extent.height as SIZE;
    if dx_1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        xincr1 = 1 as SIZE;
    } else {
        xincr1 = -1 as SIZE;
        dx_1 = -(dx_1 as ::core::ffi::c_int) as SIZE;
    }
    if dy_1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        yincr1 = 1 as SIZE;
    } else {
        yincr1 = -1 as SIZE;
        dy_1 = -(dy_1 as ::core::ffi::c_int) as SIZE;
    }
    if dx_1 as ::core::ffi::c_int >= dy_1 as ::core::ffi::c_int {
        cycle1 = dx_1;
    } else {
        cycle1 = dy_1;
    }
    yerror1 = cycle1;
    xerror1 = yerror1;
    check1 = BOOLEAN::FALSE;
    check0 = check1;
    '_CheckFirstIntersection: {
        if t0 as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
            time_error1 = 0 as SIZE;
            time_error0 = time_error1;
            if t0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                t0 = t0.wrapping_add(1);
                c2rust_current_block = C2Rust_Block::CheckFirstIntersection;
                break '_CheckFirstIntersection;
            }
        } else {
            let mut delta: SIZE = 0;
            let mut start: COUNT = 0;
            let mut error: ::core::ffi::c_long = 0;
            start = (cycle0 as COUNT as ::core::ffi::c_int
                * (t0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT
                    as ::core::ffi::c_int) as COUNT;
            time_error0 = (start as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as SIZE;
            start = (start as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int) as COUNT;
            if start as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                error = xerror0 as ::core::ffi::c_long
                    - dx_0 as ::core::ffi::c_long * start as ::core::ffi::c_long;
                if error > 0 as ::core::ffi::c_long {
                    xerror0 = error as SIZE;
                } else {
                    delta = (-((error / cycle0 as ::core::ffi::c_long) as SIZE
                        as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as SIZE;
                    IB0.Box.corner.x = (IB0.Box.corner.x as ::core::ffi::c_int
                        + xincr0 as ::core::ffi::c_int * delta as ::core::ffi::c_int)
                        as COORD;
                    xerror0 = (error
                        + cycle0 as ::core::ffi::c_long * delta as ::core::ffi::c_long)
                        as SIZE;
                }
                error = yerror0 as ::core::ffi::c_long
                    - dy_0 as ::core::ffi::c_long * start as ::core::ffi::c_long;
                if error > 0 as ::core::ffi::c_long {
                    yerror0 = error as SIZE;
                } else {
                    delta = (-((error / cycle0 as ::core::ffi::c_long) as SIZE
                        as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as SIZE;
                    IB0.Box.corner.y = (IB0.Box.corner.y as ::core::ffi::c_int
                        + yincr0 as ::core::ffi::c_int * delta as ::core::ffi::c_int)
                        as COORD;
                    yerror0 = (error
                        + cycle0 as ::core::ffi::c_long * delta as ::core::ffi::c_long)
                        as SIZE;
                }
                (*pr0).corner = IB0.Box.corner;
            }
            start = (cycle1 as COUNT as ::core::ffi::c_int
                * (t0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT
                    as ::core::ffi::c_int) as COUNT;
            time_error1 = (start as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as SIZE;
            start = (start as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int) as COUNT;
            if start as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                error = xerror1 as ::core::ffi::c_long
                    - dx_1 as ::core::ffi::c_long * start as ::core::ffi::c_long;
                if error > 0 as ::core::ffi::c_long {
                    xerror1 = error as SIZE;
                } else {
                    delta = (-((error / cycle1 as ::core::ffi::c_long) as SIZE
                        as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as SIZE;
                    IB1.Box.corner.x = (IB1.Box.corner.x as ::core::ffi::c_int
                        + xincr1 as ::core::ffi::c_int * delta as ::core::ffi::c_int)
                        as COORD;
                    xerror1 = (error
                        + cycle1 as ::core::ffi::c_long * delta as ::core::ffi::c_long)
                        as SIZE;
                }
                error = yerror1 as ::core::ffi::c_long
                    - dy_1 as ::core::ffi::c_long * start as ::core::ffi::c_long;
                if error > 0 as ::core::ffi::c_long {
                    yerror1 = error as SIZE;
                } else {
                    delta = (-((error / cycle1 as ::core::ffi::c_long) as SIZE
                        as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as SIZE;
                    IB1.Box.corner.y = (IB1.Box.corner.y as ::core::ffi::c_int
                        + yincr1 as ::core::ffi::c_int * delta as ::core::ffi::c_int)
                        as COORD;
                    yerror1 = (error
                        + cycle1 as ::core::ffi::c_long * delta as ::core::ffi::c_long)
                        as SIZE;
                }
                (*pr1).corner = IB1.Box.corner;
            }
        }
        (*pControl1).last_time_val = t0;
        (*pControl0).last_time_val = (*pControl1).last_time_val;
        c2rust_current_block = C2Rust_Block::S_323;
    }
    loop {
        match c2rust_current_block {
            C2Rust_Block::S_323 => {
                t0 = t0.wrapping_add(1);
                time_error0 = (time_error0 as ::core::ffi::c_int
                    + cycle0 as ::core::ffi::c_int) as SIZE;
                if time_error0 as ::core::ffi::c_int
                    >= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                {
                    xerror0 = (xerror0 as ::core::ffi::c_int
                        - dx_0 as ::core::ffi::c_int) as SIZE;
                    if xerror0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        IB0.Box.corner.x = (IB0.Box.corner.x as ::core::ffi::c_int
                            + xincr0 as ::core::ffi::c_int) as COORD;
                        xerror0 = (xerror0 as ::core::ffi::c_int
                            + cycle0 as ::core::ffi::c_int) as SIZE;
                    }
                    yerror0 = (yerror0 as ::core::ffi::c_int
                        - dy_0 as ::core::ffi::c_int) as SIZE;
                    if yerror0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        IB0.Box.corner.y = (IB0.Box.corner.y as ::core::ffi::c_int
                            + yincr0 as ::core::ffi::c_int) as COORD;
                        yerror0 = (yerror0 as ::core::ffi::c_int
                            + cycle0 as ::core::ffi::c_int) as SIZE;
                    }
                    check0 = BOOLEAN::TRUE;
                    time_error0 = (time_error0 as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                        as SIZE;
                }
                time_error1 = (time_error1 as ::core::ffi::c_int
                    + cycle1 as ::core::ffi::c_int) as SIZE;
                if time_error1 as ::core::ffi::c_int
                    >= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                {
                    xerror1 = (xerror1 as ::core::ffi::c_int
                        - dx_1 as ::core::ffi::c_int) as SIZE;
                    if xerror1 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        IB1.Box.corner.x = (IB1.Box.corner.x as ::core::ffi::c_int
                            + xincr1 as ::core::ffi::c_int) as COORD;
                        xerror1 = (xerror1 as ::core::ffi::c_int
                            + cycle1 as ::core::ffi::c_int) as SIZE;
                    }
                    yerror1 = (yerror1 as ::core::ffi::c_int
                        - dy_1 as ::core::ffi::c_int) as SIZE;
                    if yerror1 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        IB1.Box.corner.y = (IB1.Box.corner.y as ::core::ffi::c_int
                            + yincr1 as ::core::ffi::c_int) as COORD;
                        yerror1 = (yerror1 as ::core::ffi::c_int
                            + cycle1 as ::core::ffi::c_int) as SIZE;
                    }
                    check1 = BOOLEAN::TRUE;
                    time_error1 = (time_error1 as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                        as SIZE;
                }
                if check0.0 != 0 || check1.0 != 0 {
                    c2rust_current_block = C2Rust_Block::CheckFirstIntersection;
                    continue;
                }
            }
            C2Rust_Block::CheckFirstIntersection => {
                if BoxIntersect(&raw mut IB0.Box, &raw mut IB1.Box, &raw mut r_intersect)
                    as ::core::ffi::c_int != 0
                    && images_intersect(&raw mut IB0, &raw mut IB1, &raw mut r_intersect)
                        .0 != 0
                {
                    return t0;
                }
                if check0.0 != 0 {
                    (*pr0).corner = IB0.Box.corner;
                    (*pControl0).last_time_val = t0;
                    check0 = BOOLEAN::FALSE;
                }
                if check1.0 != 0 {
                    (*pr1).corner = IB1.Box.corner;
                    (*pControl1).last_time_val = t0;
                    check1 = BOOLEAN::FALSE;
                }
            }
        }
        if t0 as ::core::ffi::c_int <= t1 as ::core::ffi::c_int {
            c2rust_current_block = C2Rust_Block::S_323;
        } else {
            break;
        }
    }
    return 0 as ::core::ffi::c_int as TIME_VALUE;
}
#[no_mangle]
pub unsafe extern "C" fn DrawablesIntersect(
    mut pControl0: *mut INTERSECT_CONTROL,
    mut pControl1: *mut INTERSECT_CONTROL,
    mut max_time_val: TIME_VALUE,
) -> TIME_VALUE {
    let mut dy: SIZE = 0;
    let mut time_y_0: SIZE = 0;
    let mut time_y_1: SIZE = 0;
    let mut r0: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut r1: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut FramePtr0: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut FramePtr1: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int == 0
        || max_time_val as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int as TIME_VALUE
    } else if max_time_val as ::core::ffi::c_int
        > ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int
    {
        max_time_val = (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as TIME_VALUE;
    }
    (*pControl1).last_time_val = 0 as TIME_VALUE;
    (*pControl0).last_time_val = (*pControl1).last_time_val;
    r0.corner = (*pControl0).IntersectStamp.origin;
    r1.corner = (*pControl1).IntersectStamp.origin;
    r0.extent.width = ((*pControl0).EndPoint.x as ::core::ffi::c_int
        - r0.corner.x as ::core::ffi::c_int) as COORD;
    r0.extent.height = ((*pControl0).EndPoint.y as ::core::ffi::c_int
        - r0.corner.y as ::core::ffi::c_int) as COORD;
    r1.extent.width = ((*pControl1).EndPoint.x as ::core::ffi::c_int
        - r1.corner.x as ::core::ffi::c_int) as COORD;
    r1.extent.height = ((*pControl1).EndPoint.y as ::core::ffi::c_int
        - r1.corner.y as ::core::ffi::c_int) as COORD;
    FramePtr0 = (*pControl0).IntersectStamp.frame;
    if FramePtr0.is_null() {
        return 0 as TIME_VALUE;
    }
    r0.corner.x = (r0.corner.x as ::core::ffi::c_int
        - (*FramePtr0).HotSpot.x as ::core::ffi::c_int) as COORD;
    r0.corner.y = (r0.corner.y as ::core::ffi::c_int
        - (*FramePtr0).HotSpot.y as ::core::ffi::c_int) as COORD;
    FramePtr1 = (*pControl1).IntersectStamp.frame;
    if FramePtr1.is_null() {
        return 0 as TIME_VALUE;
    }
    r1.corner.x = (r1.corner.x as ::core::ffi::c_int
        - (*FramePtr1).HotSpot.x as ::core::ffi::c_int) as COORD;
    r1.corner.y = (r1.corner.y as ::core::ffi::c_int
        - (*FramePtr1).HotSpot.y as ::core::ffi::c_int) as COORD;
    dy = (r1.corner.y as ::core::ffi::c_int - r0.corner.y as ::core::ffi::c_int) as SIZE;
    time_y_0 = (dy as ::core::ffi::c_int
        - (*FramePtr0).Bounds.height as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as SIZE;
    time_y_1 = (dy as ::core::ffi::c_int
        + (*FramePtr1).Bounds.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as SIZE;
    dy = (r0.extent.height as ::core::ffi::c_int
        - r1.extent.height as ::core::ffi::c_int) as SIZE;
    if time_y_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
        && time_y_1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        || time_y_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && dy as ::core::ffi::c_int >= time_y_0 as ::core::ffi::c_int
        || (time_y_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && dy as ::core::ffi::c_int <= time_y_1 as ::core::ffi::c_int
    {
        let mut dx: SIZE = 0;
        let mut time_x_0: SIZE = 0;
        let mut time_x_1: SIZE = 0;
        dx = (r1.corner.x as ::core::ffi::c_int - r0.corner.x as ::core::ffi::c_int)
            as SIZE;
        time_x_0 = (dx as ::core::ffi::c_int
            - (*FramePtr0).Bounds.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as SIZE;
        time_x_1 = (dx as ::core::ffi::c_int
            + (*FramePtr1).Bounds.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as SIZE;
        dx = (r0.extent.width as ::core::ffi::c_int
            - r1.extent.width as ::core::ffi::c_int) as SIZE;
        if time_x_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
            && time_x_1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            || time_x_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && dx as ::core::ffi::c_int >= time_x_0 as ::core::ffi::c_int
            || (time_x_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && dx as ::core::ffi::c_int <= time_x_1 as ::core::ffi::c_int
        {
            let mut intersect_time: TIME_VALUE = 0;
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                time_y_1 = 0 as SIZE;
                time_y_0 = time_y_1;
            } else {
                let mut t: SIZE = 0;
                let mut time_beg: ::core::ffi::c_long = 0;
                let mut time_end: ::core::ffi::c_long = 0;
                let mut fract: ::core::ffi::c_long = 0;
                if (time_y_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    t = time_y_0;
                    time_y_0 = -(time_y_1 as ::core::ffi::c_int) as SIZE;
                    time_y_1 = -(t as ::core::ffi::c_int) as SIZE;
                } else if time_y_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                    if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        time_y_1 = -(time_y_0 as ::core::ffi::c_int) as SIZE;
                    }
                    time_y_0 = 0 as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < time_y_1 as ::core::ffi::c_int {
                    time_y_1 = dy;
                }
                time_y_0 -= 1;
                time_y_1 += 1;
                if (time_x_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    t = time_x_0;
                    time_x_0 = -(time_x_1 as ::core::ffi::c_int) as SIZE;
                    time_x_1 = -(t as ::core::ffi::c_int) as SIZE;
                } else if time_x_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                    if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        time_x_1 = -(time_x_0 as ::core::ffi::c_int) as SIZE;
                    }
                    time_x_0 = 0 as SIZE;
                }
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dx as ::core::ffi::c_int) < time_x_1 as ::core::ffi::c_int {
                    time_x_1 = dx;
                }
                time_x_0 -= 1;
                time_x_1 += 1;
                if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    time_beg = time_y_0 as ::core::ffi::c_long;
                    time_end = time_y_1 as ::core::ffi::c_long;
                    fract = dy as ::core::ffi::c_long;
                } else if dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    time_beg = time_x_0 as ::core::ffi::c_long;
                    time_end = time_x_1 as ::core::ffi::c_long;
                    fract = dx as ::core::ffi::c_long;
                } else {
                    let mut time_x: ::core::ffi::c_long = 0;
                    let mut time_y: ::core::ffi::c_long = 0;
                    time_x = time_x_0 as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                    time_y = time_y_0 as ::core::ffi::c_long * dx as ::core::ffi::c_long;
                    time_beg = if time_x < time_y { time_y } else { time_x };
                    time_x = time_x_1 as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                    time_y = time_y_1 as ::core::ffi::c_long * dx as ::core::ffi::c_long;
                    time_end = if time_x > time_y { time_y } else { time_x };
                    fract = dx as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                }
                time_beg <<= 8 as ::core::ffi::c_int;
                if time_beg < fract {
                    time_y_0 = 0 as SIZE;
                } else {
                    time_y_0 = (time_beg / fract) as SIZE;
                }
                if time_end >= fract
                    || {
                        time_end <<= 8 as ::core::ffi::c_int;
                        time_end >= fract * max_time_val as ::core::ffi::c_long
                    }
                {
                    time_y_1 = (max_time_val as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as SIZE;
                } else {
                    time_y_1 = (((time_end + fract - 1 as ::core::ffi::c_long) / fract)
                        as SIZE as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
                }
            }
            if time_y_0 as ::core::ffi::c_int <= time_y_1 as ::core::ffi::c_int
                && {
                    intersect_time = frame_intersect(
                        pControl0,
                        &raw mut r0,
                        pControl1,
                        &raw mut r1,
                        time_y_0 as TIME_VALUE,
                        time_y_1 as TIME_VALUE,
                    );
                    intersect_time as ::core::ffi::c_int != 0
                }
            {
                FramePtr0 = (*pControl0).IntersectStamp.frame;
                (*pControl0).EndPoint.x = (r0.corner.x as ::core::ffi::c_int
                    + (*FramePtr0).HotSpot.x as ::core::ffi::c_int) as COORD;
                (*pControl0).EndPoint.y = (r0.corner.y as ::core::ffi::c_int
                    + (*FramePtr0).HotSpot.y as ::core::ffi::c_int) as COORD;
                FramePtr1 = (*pControl1).IntersectStamp.frame;
                (*pControl1).EndPoint.x = (r1.corner.x as ::core::ffi::c_int
                    + (*FramePtr1).HotSpot.x as ::core::ffi::c_int) as COORD;
                (*pControl1).EndPoint.y = (r1.corner.y as ::core::ffi::c_int
                    + (*FramePtr1).HotSpot.y as ::core::ffi::c_int) as COORD;
                return intersect_time;
            }
        }
    }
    return 0 as ::core::ffi::c_int as TIME_VALUE;
}
