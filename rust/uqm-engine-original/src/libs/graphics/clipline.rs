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

// Rust translation of src/libs/graphics/clipline.c.
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type SWORD = sint16;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
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
#[no_mangle]
pub unsafe extern "C" fn _clip_line(
    mut pClipRect: *const RECT,
    mut pLine: *mut BRESENHAM_LINE,
) -> INTERSECT_CODE {
    let mut p: COORD = 0;
    let mut x0: COORD = 0;
    let mut y0: COORD = 0;
    let mut xmin: COORD = 0;
    let mut ymin: COORD = 0;
    let mut xmax: COORD = 0;
    let mut ymax: COORD = 0;
    let mut abs_delta_x: SIZE = 0;
    let mut abs_delta_y: SIZE = 0;
    let mut intersect_code: INTERSECT_CODE = 0;
    xmin = (*pClipRect).corner.x;
    ymin = (*pClipRect).corner.y;
    xmax = ((*pClipRect).corner.x as ::core::ffi::c_int
        + (*pClipRect).extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    ymax = ((*pClipRect).corner.y as ::core::ffi::c_int
        + (*pClipRect).extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    if (*pLine).first.x as ::core::ffi::c_int <= (*pLine).second.x as ::core::ffi::c_int
    {
        (*pLine).end_points_exchanged = BOOLEAN::FALSE;
    } else {
        p = (*pLine).first.x;
        (*pLine).first.x = (*pLine).second.x;
        (*pLine).second.x = p;
        p = (*pLine).first.y;
        (*pLine).first.y = (*pLine).second.y;
        (*pLine).second.y = p;
        (*pLine).end_points_exchanged = BOOLEAN::TRUE;
    }
    if (*pLine).first.x as ::core::ffi::c_int > xmax as ::core::ffi::c_int
        || ((*pLine).second.x as ::core::ffi::c_int) < xmin as ::core::ffi::c_int
        || (*pLine).first.y as ::core::ffi::c_int > ymax as ::core::ffi::c_int
            && (*pLine).second.y as ::core::ffi::c_int > ymax as ::core::ffi::c_int
        || ((*pLine).first.y as ::core::ffi::c_int) < ymin as ::core::ffi::c_int
            && ((*pLine).second.y as ::core::ffi::c_int) < ymin as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int as INTERSECT_CODE;
    }
    intersect_code = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
        as INTERSECT_CODE;
    y0 = 0 as COORD;
    x0 = y0;
    abs_delta_x = (((*pLine).second.x as ::core::ffi::c_int
        - (*pLine).first.x as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
    abs_delta_y = (((*pLine).second.y as ::core::ffi::c_int
        - (*pLine).first.y as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
    (*pLine).abs_delta_x = abs_delta_x;
    (*pLine).abs_delta_y = abs_delta_y;
    if abs_delta_y as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if ((*pLine).first.x as ::core::ffi::c_int) < xmin as ::core::ffi::c_int {
            (*pLine).first.x = xmin;
            intersect_code = (intersect_code as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
        }
        if (*pLine).second.x as ::core::ffi::c_int > xmax as ::core::ffi::c_int {
            (*pLine).second.x = xmax;
            intersect_code = (intersect_code as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
        }
    } else if abs_delta_x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (abs_delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            p = (*pLine).first.y;
            (*pLine).first.y = (*pLine).second.y;
            (*pLine).second.y = p;
            abs_delta_y = -(abs_delta_y as ::core::ffi::c_int) as SIZE;
            (*pLine).abs_delta_y = abs_delta_y;
        }
        if ((*pLine).first.y as ::core::ffi::c_int) < ymin as ::core::ffi::c_int {
            (*pLine).first.y = ymin;
            intersect_code = (intersect_code as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
        }
        if (*pLine).second.y as ::core::ffi::c_int > ymax as ::core::ffi::c_int {
            (*pLine).second.y = ymax;
            intersect_code = (intersect_code as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
        }
    } else {
        let mut x1: COORD = 0;
        let mut y1: COORD = 0;
        p = (*pLine).first.x;
        x1 = ((*pLine).second.x as ::core::ffi::c_int - p as ::core::ffi::c_int)
            as COORD;
        xmin = (xmin as ::core::ffi::c_int - p as ::core::ffi::c_int) as COORD;
        xmax = (xmax as ::core::ffi::c_int - p as ::core::ffi::c_int) as COORD;
        p = (*pLine).first.y;
        if abs_delta_y as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            y1 = ((*pLine).second.y as ::core::ffi::c_int - p as ::core::ffi::c_int)
                as COORD;
            ymin = (ymin as ::core::ffi::c_int - p as ::core::ffi::c_int) as COORD;
            ymax = (ymax as ::core::ffi::c_int - p as ::core::ffi::c_int) as COORD;
        } else {
            y1 = (p as ::core::ffi::c_int - (*pLine).second.y as ::core::ffi::c_int)
                as COORD;
            ymin = (p as ::core::ffi::c_int - ymin as ::core::ffi::c_int) as COORD;
            ymax = (p as ::core::ffi::c_int - ymax as ::core::ffi::c_int) as COORD;
            p = ymin;
            ymin = ymax;
            ymax = p;
            abs_delta_y = -(abs_delta_y as ::core::ffi::c_int) as SIZE;
        }
        if abs_delta_x as ::core::ffi::c_int > abs_delta_y as ::core::ffi::c_int {
            let mut half_dx: SIZE = 0;
            half_dx = (abs_delta_x as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as SIZE;
            if (x0 as ::core::ffi::c_int) < xmin as ::core::ffi::c_int {
                x0 = xmin;
                y0 = ((abs_delta_y as ::core::ffi::c_long * x0 as ::core::ffi::c_long
                    + half_dx as ::core::ffi::c_long)
                    / abs_delta_x as ::core::ffi::c_long) as COORD;
                if y0 as ::core::ffi::c_int > ymax as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
            }
            if x1 as ::core::ffi::c_int > xmax as ::core::ffi::c_int {
                x1 = xmax;
                y1 = ((abs_delta_y as ::core::ffi::c_long * x1 as ::core::ffi::c_long
                    + half_dx as ::core::ffi::c_long)
                    / abs_delta_x as ::core::ffi::c_long) as COORD;
                if (y1 as ::core::ffi::c_int) < ymin as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
            }
            if (y0 as ::core::ffi::c_int) < ymin as ::core::ffi::c_int {
                y0 = ymin;
                x0 = ((abs_delta_x as ::core::ffi::c_long * y0 as ::core::ffi::c_long
                    - half_dx as ::core::ffi::c_long
                    + (abs_delta_y as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) / abs_delta_y as ::core::ffi::c_long)
                    as COORD;
                if x0 as ::core::ffi::c_int > xmax as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
                intersect_code = (intersect_code as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
            if y1 as ::core::ffi::c_int > ymax as ::core::ffi::c_int {
                y1 = ymax;
                x1 = (((abs_delta_x as ::core::ffi::c_long
                    * (y1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long - half_dx as ::core::ffi::c_long
                    + (abs_delta_y as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) / abs_delta_y as ::core::ffi::c_long)
                    as COORD as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
                if (x1 as ::core::ffi::c_int) < xmin as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
                intersect_code = (intersect_code as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
        } else {
            let mut half_dy: SIZE = 0;
            half_dy = (abs_delta_y as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as SIZE;
            if (y0 as ::core::ffi::c_int) < ymin as ::core::ffi::c_int {
                y0 = ymin;
                x0 = ((abs_delta_x as ::core::ffi::c_long * y0 as ::core::ffi::c_long
                    + half_dy as ::core::ffi::c_long)
                    / abs_delta_y as ::core::ffi::c_long) as COORD;
                if x0 as ::core::ffi::c_int > xmax as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
            }
            if y1 as ::core::ffi::c_int > ymax as ::core::ffi::c_int {
                y1 = ymax;
                x1 = ((abs_delta_x as ::core::ffi::c_long * y1 as ::core::ffi::c_long
                    + half_dy as ::core::ffi::c_long)
                    / abs_delta_y as ::core::ffi::c_long) as COORD;
                if (x1 as ::core::ffi::c_int) < xmin as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
            }
            if (x0 as ::core::ffi::c_int) < xmin as ::core::ffi::c_int {
                x0 = xmin;
                y0 = ((abs_delta_y as ::core::ffi::c_long * x0 as ::core::ffi::c_long
                    - half_dy as ::core::ffi::c_long
                    + (abs_delta_x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) / abs_delta_x as ::core::ffi::c_long)
                    as COORD;
                if y0 as ::core::ffi::c_int > ymax as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
                intersect_code = (intersect_code as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
            if x1 as ::core::ffi::c_int > xmax as ::core::ffi::c_int {
                x1 = xmax;
                y1 = (((abs_delta_y as ::core::ffi::c_long
                    * (x1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long - half_dy as ::core::ffi::c_long
                    + (abs_delta_x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) / abs_delta_x as ::core::ffi::c_long)
                    as COORD as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
                if (y1 as ::core::ffi::c_int) < ymin as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int as INTERSECT_CODE;
                }
                intersect_code = (intersect_code as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int) as INTERSECT_CODE;
                intersect_code = (intersect_code as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
        }
        (*pLine).second.x = ((*pLine).first.x as ::core::ffi::c_int
            + x1 as ::core::ffi::c_int) as COORD;
        (*pLine).first.x = ((*pLine).first.x as ::core::ffi::c_int
            + x0 as ::core::ffi::c_int) as COORD;
        if (*pLine).abs_delta_y as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*pLine).second.y = ((*pLine).first.y as ::core::ffi::c_int
                + y1 as ::core::ffi::c_int) as COORD;
            (*pLine).first.y = ((*pLine).first.y as ::core::ffi::c_int
                + y0 as ::core::ffi::c_int) as COORD;
        } else {
            let mut y_code: INTERSECT_CODE = 0;
            (*pLine).second.y = ((*pLine).first.y as ::core::ffi::c_int
                - y1 as ::core::ffi::c_int) as COORD;
            (*pLine).first.y = ((*pLine).first.y as ::core::ffi::c_int
                - y0 as ::core::ffi::c_int) as COORD;
            y_code = (intersect_code as ::core::ffi::c_int
                & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as INTERSECT_CODE as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            if y_code as ::core::ffi::c_int != 0
                && y_code as ::core::ffi::c_int
                    != ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as INTERSECT_CODE as ::core::ffi::c_int
            {
                intersect_code = (intersect_code as ::core::ffi::c_int
                    ^ (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as INTERSECT_CODE as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as INTERSECT_CODE as ::core::ffi::c_int)) as INTERSECT_CODE;
            }
        }
    }
    if intersect_code as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as INTERSECT_CODE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as INTERSECT_CODE
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as INTERSECT_CODE
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as INTERSECT_CODE
                as ::core::ffi::c_int) as INTERSECT_CODE as ::core::ffi::c_int == 0
    {
        if abs_delta_x as ::core::ffi::c_int > abs_delta_y as ::core::ffi::c_int {
            (*pLine).error_term = -((abs_delta_x as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as SIZE;
        } else {
            (*pLine).error_term = -((abs_delta_y as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as SIZE;
        }
    } else {
        intersect_code = (intersect_code as ::core::ffi::c_int
            & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as INTERSECT_CODE
                as ::core::ffi::c_int)) as INTERSECT_CODE;
        if abs_delta_x as ::core::ffi::c_int > abs_delta_y as ::core::ffi::c_int {
            (*pLine).error_term = ((x0 as ::core::ffi::c_long
                * abs_delta_y as ::core::ffi::c_long
                - y0 as ::core::ffi::c_long * abs_delta_x as ::core::ffi::c_long) as SIZE
                as ::core::ffi::c_int
                - (abs_delta_x as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as SIZE;
        } else {
            (*pLine).error_term = ((y0 as ::core::ffi::c_long
                * abs_delta_x as ::core::ffi::c_long
                - x0 as ::core::ffi::c_long * abs_delta_y as ::core::ffi::c_long) as SIZE
                as ::core::ffi::c_int
                - (abs_delta_y as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as SIZE;
        }
    }
    (*pLine).intersect_code = intersect_code;
    return (*pLine).intersect_code;
}
