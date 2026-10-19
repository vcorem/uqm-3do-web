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

// Rust translation of src/libs/graphics/boxint.c.
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type SWORD = sint16;
pub type SIZE = SWORD;
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type INTERSECT_CODE = BYTE;
#[no_mangle]
pub unsafe extern "C" fn BoxIntersect(
    mut pr1: *mut RECT,
    mut pr2: *mut RECT,
    mut pinter: *mut RECT,
) -> INTERSECT_CODE {
    let mut intersect_code: INTERSECT_CODE = 0;
    let mut x1: COORD = 0;
    let mut w1: SIZE = 0;
    let mut w2: SIZE = 0;
    let mut delta: SIZE = 0;
    intersect_code = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
        as INTERSECT_CODE;
    x1 = ((*pr1).corner.x as ::core::ffi::c_int - (*pr2).corner.x as ::core::ffi::c_int)
        as COORD;
    w1 = (*pr1).extent.width as SIZE;
    w2 = (*pr2).extent.width as SIZE;
    delta = (w2 as ::core::ffi::c_int - x1 as ::core::ffi::c_int) as SIZE;
    if delta as ::core::ffi::c_int <= w1 as ::core::ffi::c_int {
        if delta as ::core::ffi::c_int != w1 as ::core::ffi::c_int {
            w1 = delta;
            intersect_code = (intersect_code as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
        }
        intersect_code = (intersect_code as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as INTERSECT_CODE
                as ::core::ffi::c_int) as INTERSECT_CODE;
    }
    if x1 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        if (x1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            w1 = (w1 as ::core::ffi::c_int + x1 as ::core::ffi::c_int) as SIZE;
            x1 = 0 as COORD;
            intersect_code = (intersect_code as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
        }
        intersect_code = (intersect_code as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as INTERSECT_CODE
                as ::core::ffi::c_int) as INTERSECT_CODE;
    }
    if w1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut y1: COORD = 0;
        let mut h1: SIZE = 0;
        y1 = ((*pr1).corner.y as ::core::ffi::c_int
            - (*pr2).corner.y as ::core::ffi::c_int) as COORD;
        h1 = (*pr1).extent.height as SIZE;
        w2 = (*pr2).extent.height as SIZE;
        delta = (w2 as ::core::ffi::c_int - y1 as ::core::ffi::c_int) as SIZE;
        if delta as ::core::ffi::c_int <= h1 as ::core::ffi::c_int {
            if delta as ::core::ffi::c_int != h1 as ::core::ffi::c_int {
                h1 = delta;
                intersect_code = (intersect_code as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
            intersect_code = (intersect_code as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
        }
        if y1 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            if (y1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                h1 = (h1 as ::core::ffi::c_int + y1 as ::core::ffi::c_int) as SIZE;
                y1 = 0 as COORD;
                intersect_code = (intersect_code as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
            intersect_code = (intersect_code as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
        }
        if h1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*pinter).corner.x = (x1 as ::core::ffi::c_int
                + (*pr2).corner.x as ::core::ffi::c_int) as COORD;
            (*pinter).corner.y = (y1 as ::core::ffi::c_int
                + (*pr2).corner.y as ::core::ffi::c_int) as COORD;
            (*pinter).extent.width = w1 as COORD;
            (*pinter).extent.height = h1 as COORD;
            return intersect_code;
        }
    }
    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
}
#[no_mangle]
pub unsafe extern "C" fn BoxUnion(
    mut pr1: *mut RECT,
    mut pr2: *mut RECT,
    mut punion: *mut RECT,
) {
    (*punion).corner.x = (if (*pr1).corner.x as ::core::ffi::c_int
        <= (*pr2).corner.x as ::core::ffi::c_int
    {
        (*pr1).corner.x as ::core::ffi::c_int
    } else {
        (*pr2).corner.x as ::core::ffi::c_int
    }) as COORD;
    (*punion).corner.y = (if (*pr1).corner.y as ::core::ffi::c_int
        <= (*pr2).corner.y as ::core::ffi::c_int
    {
        (*pr1).corner.y as ::core::ffi::c_int
    } else {
        (*pr2).corner.y as ::core::ffi::c_int
    }) as COORD;
    (*punion).extent.width = (if (*pr1).corner.x as ::core::ffi::c_int
        + (*pr1).extent.width as ::core::ffi::c_int
        >= (*pr2).corner.x as ::core::ffi::c_int
            + (*pr2).extent.width as ::core::ffi::c_int
    {
        (*pr1).corner.x as ::core::ffi::c_int + (*pr1).extent.width as ::core::ffi::c_int
    } else {
        (*pr2).corner.x as ::core::ffi::c_int + (*pr2).extent.width as ::core::ffi::c_int
    } - (*punion).corner.x as ::core::ffi::c_int) as COORD;
    (*punion).extent.height = (if (*pr1).corner.y as ::core::ffi::c_int
        + (*pr1).extent.height as ::core::ffi::c_int
        >= (*pr2).corner.y as ::core::ffi::c_int
            + (*pr2).extent.height as ::core::ffi::c_int
    {
        (*pr1).corner.y as ::core::ffi::c_int
            + (*pr1).extent.height as ::core::ffi::c_int
    } else {
        (*pr2).corner.y as ::core::ffi::c_int
            + (*pr2).extent.height as ::core::ffi::c_int
    } - (*punion).corner.y as ::core::ffi::c_int) as COORD;
}
