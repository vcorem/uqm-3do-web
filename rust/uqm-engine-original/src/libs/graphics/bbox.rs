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

// Rust translation of src/libs/graphics/bbox.c.
extern "C" {
    fn TFB_DrawCanvas_GetExtent(canvas: TFB_Canvas, size: *mut EXTENT);
}
pub type sint16 = ::core::ffi::c_short;
pub type SWORD = sint16;
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
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
pub type TFB_Canvas = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_BoundingBox {
    pub valid: ::core::ffi::c_int,
    pub region: RECT,
    pub clip: RECT,
}
#[no_mangle]
pub static mut TFB_BBox: TFB_BoundingBox = TFB_BoundingBox {
    valid: 0,
    region: RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    },
    clip: RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    },
};
#[no_mangle]
pub static mut maxWidth: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut maxHeight: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_Init(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    maxWidth = width;
    maxHeight = height;
    TFB_BBox.clip.extent.width = width as COORD;
    TFB_BBox.clip.extent.height = height as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_Reset() {
    TFB_BBox.valid = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_SetClipRect(mut r: *const RECT) {
    if r.is_null() {
        TFB_BBox.clip.corner.x = 0 as COORD;
        TFB_BBox.clip.corner.y = 0 as COORD;
        TFB_BBox.clip.extent.width = maxWidth as COORD;
        TFB_BBox.clip.extent.height = maxHeight as COORD;
        return;
    }
    TFB_BBox.clip = *r;
    if (TFB_BBox.clip.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        TFB_BBox.clip.corner.x = 0 as COORD;
    }
    if (TFB_BBox.clip.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        TFB_BBox.clip.corner.y = 0 as COORD;
    }
    if TFB_BBox.clip.corner.x as ::core::ffi::c_int
        + TFB_BBox.clip.extent.width as ::core::ffi::c_int > maxWidth
    {
        TFB_BBox.clip.extent.width = (maxWidth
            - TFB_BBox.clip.corner.x as ::core::ffi::c_int) as COORD;
    }
    if TFB_BBox.clip.corner.y as ::core::ffi::c_int
        + TFB_BBox.clip.extent.height as ::core::ffi::c_int > maxHeight
    {
        TFB_BBox.clip.extent.height = (maxHeight
            - TFB_BBox.clip.corner.y as ::core::ffi::c_int) as COORD;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_RegisterPoint(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut x1: ::core::ffi::c_int = TFB_BBox.clip.corner.x as ::core::ffi::c_int;
    let mut y1: ::core::ffi::c_int = TFB_BBox.clip.corner.y as ::core::ffi::c_int;
    let mut x2: ::core::ffi::c_int = TFB_BBox.clip.corner.x as ::core::ffi::c_int
        + TFB_BBox.clip.extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    let mut y2: ::core::ffi::c_int = TFB_BBox.clip.corner.y as ::core::ffi::c_int
        + TFB_BBox.clip.extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    if x < x1 {
        x = x1;
    }
    if x >= x2 {
        x = x2;
    }
    if y < y1 {
        y = y1;
    }
    if y >= y2 {
        y = y2;
    }
    if TFB_BBox.valid == 0 {
        TFB_BBox.valid = 1 as ::core::ffi::c_int;
        TFB_BBox.region.corner.x = x as COORD;
        TFB_BBox.region.corner.y = y as COORD;
        TFB_BBox.region.extent.width = 1 as COORD;
        TFB_BBox.region.extent.height = 1 as COORD;
        return;
    }
    x1 = TFB_BBox.region.corner.x as ::core::ffi::c_int;
    y1 = TFB_BBox.region.corner.y as ::core::ffi::c_int;
    x2 = TFB_BBox.region.corner.x as ::core::ffi::c_int
        + TFB_BBox.region.extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    y2 = TFB_BBox.region.corner.y as ::core::ffi::c_int
        + TFB_BBox.region.extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    if x < x1 {
        TFB_BBox.region.corner.x = x as COORD;
        TFB_BBox.region.extent.width = (TFB_BBox.region.extent.width
            as ::core::ffi::c_int + (x1 - x)) as COORD;
    }
    if y < y1 {
        TFB_BBox.region.corner.y = y as COORD;
        TFB_BBox.region.extent.height = (TFB_BBox.region.extent.height
            as ::core::ffi::c_int + (y1 - y)) as COORD;
    }
    if x > x2 {
        TFB_BBox.region.extent.width = (TFB_BBox.region.extent.width
            as ::core::ffi::c_int + (x - x2)) as COORD;
    }
    if y > y2 {
        TFB_BBox.region.extent.height = (TFB_BBox.region.extent.height
            as ::core::ffi::c_int + (y - y2)) as COORD;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_RegisterRect(mut r: *const RECT) {
    TFB_BBox_RegisterPoint(
        (*r).corner.x as ::core::ffi::c_int,
        (*r).corner.y as ::core::ffi::c_int,
    );
    TFB_BBox_RegisterPoint(
        (*r).corner.x as ::core::ffi::c_int + (*r).extent.width as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        (*r).corner.y as ::core::ffi::c_int + (*r).extent.height as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_RegisterCanvas(
    mut c: TFB_Canvas,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.corner.x = x as COORD;
    r.corner.y = y as COORD;
    TFB_DrawCanvas_GetExtent(c, &raw mut r.extent);
    TFB_BBox_RegisterRect(&raw mut r);
}
