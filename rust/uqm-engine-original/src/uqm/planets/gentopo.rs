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

// See doc/devel/planettopo for details.

// Rust translation of src/uqm/planets/gentopo.c.
extern "C" {
    pub type RandomContext;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    static mut SysGenRNG: *mut RandomContext;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub x_top: COORD,
    pub x_bot: COORD,
    pub x_incr: SIZE,
    pub delta_x: SIZE,
    pub error_term: SIZE,
}
#[no_mangle]
pub unsafe extern "C" fn DeltaTopography(
    mut num_iterations: COUNT,
    mut DepthArray: *mut SBYTE,
    mut pRect: *mut RECT,
    mut depth_delta: SIZE,
) {
    let mut width: SIZE = 0;
    let mut height: SIZE = 0;
    let mut delta_y: SIZE = 0;
    let mut LineDDA0: C2Rust_Unnamed = C2Rust_Unnamed {
        x_top: 0,
        x_bot: 0,
        x_incr: 0,
        delta_x: 0,
        error_term: 0,
    };
    let mut LineDDA1: C2Rust_Unnamed = C2Rust_Unnamed {
        x_top: 0,
        x_bot: 0,
        x_incr: 0,
        delta_x: 0,
        error_term: 0,
    };
    width = (*pRect).extent.width as SIZE;
    height = (*pRect).extent.height as SIZE;
    delta_y = ((height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as SIZE;
    loop {
        let mut d: SIZE = 0;
        let mut h: COUNT = 0;
        let mut w1: COUNT = 0;
        let mut w2: COUNT = 0;
        let mut rand_val: DWORD = 0;
        let mut lpDst: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
        if RandomContext_Random(SysGenRNG) & 1 as DWORD == 0 as DWORD {
            depth_delta = -(depth_delta as ::core::ffi::c_int) as SIZE;
        }
        rand_val = RandomContext_Random(SysGenRNG);
        w1 = rand_val as UWORD as COUNT;
        w2 = (rand_val >> 16 as ::core::ffi::c_int) as UWORD as COUNT;
        LineDDA0.x_top = (w1 as BYTE as ::core::ffi::c_int % width as ::core::ffi::c_int)
            as COORD;
        LineDDA0.x_bot = ((w1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int % width as ::core::ffi::c_int) as COORD;
        LineDDA0.delta_x = ((LineDDA0.x_bot as ::core::ffi::c_int
            - LineDDA0.x_top as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
        if LineDDA0.delta_x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            LineDDA0.x_incr = 1 as SIZE;
        } else {
            LineDDA0.x_incr = -1 as SIZE;
            LineDDA0.delta_x = -(LineDDA0.delta_x as ::core::ffi::c_int) as SIZE;
        }
        if LineDDA0.delta_x as ::core::ffi::c_int > delta_y as ::core::ffi::c_int {
            LineDDA0.error_term = -(LineDDA0.delta_x as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
        } else {
            LineDDA0.error_term = -(delta_y as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
        }
        LineDDA1.x_top = (w2 as BYTE as ::core::ffi::c_int
            % (width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + LineDDA0.x_top as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
        LineDDA1.x_bot = ((w2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            % (width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + LineDDA0.x_bot as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
        LineDDA1.delta_x = ((LineDDA1.x_bot as ::core::ffi::c_int
            - LineDDA1.x_top as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
        if LineDDA1.delta_x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            LineDDA1.x_incr = 1 as SIZE;
        } else {
            LineDDA1.x_incr = -1 as SIZE;
            LineDDA1.delta_x = -(LineDDA1.delta_x as ::core::ffi::c_int) as SIZE;
        }
        if LineDDA1.delta_x as ::core::ffi::c_int > delta_y as ::core::ffi::c_int {
            LineDDA1.error_term = -(LineDDA1.delta_x as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
        } else {
            LineDDA1.error_term = -(delta_y as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
        }
        lpDst = DepthArray.offset(LineDDA0.x_top as isize);
        h = height as COUNT;
        loop {
            let mut w: COUNT = 0;
            w1 = (LineDDA1.x_top as ::core::ffi::c_int
                - LineDDA0.x_top as ::core::ffi::c_int) as COUNT;
            w2 = (width as ::core::ffi::c_int - w1 as ::core::ffi::c_int) as COUNT;
            if LineDDA0.x_top as ::core::ffi::c_int + w1 as ::core::ffi::c_int
                > width as ::core::ffi::c_int
            {
                w = (width as ::core::ffi::c_int - LineDDA0.x_top as ::core::ffi::c_int)
                    as COUNT;
            } else {
                w = w1;
                LineDDA0.x_top = (LineDDA0.x_top as ::core::ffi::c_int
                    + w1 as ::core::ffi::c_int) as COORD;
            }
            w1 = (w1 as ::core::ffi::c_int - w as ::core::ffi::c_int) as COUNT;
            loop {
                let c2rust_fresh0 = w;
                w = w.wrapping_sub(1);
                if c2rust_fresh0 == 0 {
                    break;
                }
                d = (*lpDst as ::core::ffi::c_int + depth_delta as ::core::ffi::c_int)
                    as SIZE;
                if d as ::core::ffi::c_int >= -128 as ::core::ffi::c_int
                    && d as ::core::ffi::c_int <= 127 as ::core::ffi::c_int
                {
                    *lpDst = d as SBYTE;
                }
                lpDst = lpDst.offset(1);
            }
            if w1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if LineDDA0.x_top as ::core::ffi::c_int == width as ::core::ffi::c_int {
                    LineDDA0.x_top = 0 as COORD;
                    lpDst = lpDst.offset(-(width as ::core::ffi::c_int as isize));
                }
            } else {
                LineDDA0.x_top = w1 as COORD;
                lpDst = lpDst.offset(-(width as ::core::ffi::c_int as isize));
                loop {
                    d = (*lpDst as ::core::ffi::c_int
                        + depth_delta as ::core::ffi::c_int) as SIZE;
                    if d as ::core::ffi::c_int >= -128 as ::core::ffi::c_int
                        && d as ::core::ffi::c_int <= 127 as ::core::ffi::c_int
                    {
                        *lpDst = d as SBYTE;
                    }
                    lpDst = lpDst.offset(1);
                    w1 = w1.wrapping_sub(1);
                    if w1 == 0 {
                        break;
                    }
                }
            }
            if LineDDA0.x_top as ::core::ffi::c_int + w2 as ::core::ffi::c_int
                > width as ::core::ffi::c_int
            {
                w = (width as ::core::ffi::c_int - LineDDA0.x_top as ::core::ffi::c_int)
                    as COUNT;
            } else {
                w = w2;
                LineDDA0.x_top = (LineDDA0.x_top as ::core::ffi::c_int
                    + w2 as ::core::ffi::c_int) as COORD;
            }
            w2 = (w2 as ::core::ffi::c_int - w as ::core::ffi::c_int) as COUNT;
            loop {
                let c2rust_fresh1 = w;
                w = w.wrapping_sub(1);
                if c2rust_fresh1 == 0 {
                    break;
                }
                d = (*lpDst as ::core::ffi::c_int - depth_delta as ::core::ffi::c_int)
                    as SIZE;
                if d as ::core::ffi::c_int >= -128 as ::core::ffi::c_int
                    && d as ::core::ffi::c_int <= 127 as ::core::ffi::c_int
                {
                    *lpDst = d as SBYTE;
                }
                lpDst = lpDst.offset(1);
            }
            if w2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if LineDDA0.x_top as ::core::ffi::c_int == width as ::core::ffi::c_int {
                    LineDDA0.x_top = 0 as COORD;
                    lpDst = lpDst.offset(-(width as ::core::ffi::c_int as isize));
                }
            } else {
                LineDDA0.x_top = w2 as COORD;
                lpDst = lpDst.offset(-(width as ::core::ffi::c_int as isize));
                loop {
                    d = (*lpDst as ::core::ffi::c_int
                        - depth_delta as ::core::ffi::c_int) as SIZE;
                    if d as ::core::ffi::c_int >= -128 as ::core::ffi::c_int
                        && d as ::core::ffi::c_int <= 127 as ::core::ffi::c_int
                    {
                        *lpDst = d as SBYTE;
                    }
                    lpDst = lpDst.offset(1);
                    w2 = w2.wrapping_sub(1);
                    if w2 == 0 {
                        break;
                    }
                }
            }
            lpDst = lpDst.offset((*pRect).extent.width as ::core::ffi::c_int as isize);
            if delta_y as ::core::ffi::c_int >= LineDDA0.delta_x as ::core::ffi::c_int {
                LineDDA0.error_term = (LineDDA0.error_term as ::core::ffi::c_int
                    + LineDDA0.delta_x as ::core::ffi::c_int) as SIZE;
                if LineDDA0.error_term as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                    lpDst = lpDst.offset(LineDDA0.x_incr as ::core::ffi::c_int as isize);
                    LineDDA0.x_top = (LineDDA0.x_top as ::core::ffi::c_int
                        + LineDDA0.x_incr as ::core::ffi::c_int) as COORD;
                    LineDDA0.error_term = (LineDDA0.error_term as ::core::ffi::c_int
                        - delta_y as ::core::ffi::c_int) as SIZE;
                }
            } else {
                loop {
                    lpDst = lpDst.offset(LineDDA0.x_incr as ::core::ffi::c_int as isize);
                    LineDDA0.x_top = (LineDDA0.x_top as ::core::ffi::c_int
                        + LineDDA0.x_incr as ::core::ffi::c_int) as COORD;
                    LineDDA0.error_term = (LineDDA0.error_term as ::core::ffi::c_int
                        + delta_y as ::core::ffi::c_int) as SIZE;
                    if (LineDDA0.error_term as ::core::ffi::c_int)
                        >= 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                LineDDA0.error_term = (LineDDA0.error_term as ::core::ffi::c_int
                    - LineDDA0.delta_x as ::core::ffi::c_int) as SIZE;
            }
            if delta_y as ::core::ffi::c_int >= LineDDA1.delta_x as ::core::ffi::c_int {
                LineDDA1.error_term = (LineDDA1.error_term as ::core::ffi::c_int
                    + LineDDA1.delta_x as ::core::ffi::c_int) as SIZE;
                if LineDDA1.error_term as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                    LineDDA1.x_top = (LineDDA1.x_top as ::core::ffi::c_int
                        + LineDDA1.x_incr as ::core::ffi::c_int) as COORD;
                    LineDDA1.error_term = (LineDDA1.error_term as ::core::ffi::c_int
                        - delta_y as ::core::ffi::c_int) as SIZE;
                }
            } else {
                loop {
                    LineDDA1.x_top = (LineDDA1.x_top as ::core::ffi::c_int
                        + LineDDA1.x_incr as ::core::ffi::c_int) as COORD;
                    LineDDA1.error_term = (LineDDA1.error_term as ::core::ffi::c_int
                        + delta_y as ::core::ffi::c_int) as SIZE;
                    if (LineDDA1.error_term as ::core::ffi::c_int)
                        >= 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                LineDDA1.error_term = (LineDDA1.error_term as ::core::ffi::c_int
                    - LineDDA1.delta_x as ::core::ffi::c_int) as SIZE;
            }
            h = h.wrapping_sub(1);
            if h == 0 {
                break;
            }
        }
        num_iterations = num_iterations.wrapping_sub(1);
        if num_iterations == 0 {
            break;
        }
    };
}
