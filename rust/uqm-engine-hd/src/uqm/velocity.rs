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

// Rust translation of src/uqm/velocity.c.
extern "C" {
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
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
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
#[no_mangle]
pub unsafe extern "C" fn GetCurrentVelocityComponents(
    mut velocityptr: *mut VELOCITY_DESC,
    mut pdx: *mut SIZE,
    mut pdy: *mut SIZE,
) {
    *pdx = ((((*velocityptr).vector.width as ::core::ffi::c_int)
        << 5 as ::core::ffi::c_int)
        + ((*velocityptr).fract.width as ::core::ffi::c_int
            - ((*velocityptr).incr.width as UWORD as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as SIZE as ::core::ffi::c_int))
        as SIZE;
    *pdy = ((((*velocityptr).vector.height as ::core::ffi::c_int)
        << 5 as ::core::ffi::c_int)
        + ((*velocityptr).fract.height as ::core::ffi::c_int
            - ((*velocityptr).incr.height as UWORD as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as SIZE as ::core::ffi::c_int))
        as SIZE;
}
#[no_mangle]
pub unsafe extern "C" fn GetCurrentVelocityComponentsSdword(
    mut velocityptr: *mut VELOCITY_DESC,
    mut pdx: *mut SDWORD,
    mut pdy: *mut SDWORD,
) {
    *pdx = (((*velocityptr).vector.width as SDWORD) << 5 as ::core::ffi::c_int)
        + ((*velocityptr).fract.width as SDWORD
            - ((*velocityptr).incr.width as UWORD as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as SDWORD);
    *pdy = (((*velocityptr).vector.height as SDWORD) << 5 as ::core::ffi::c_int)
        + ((*velocityptr).fract.height as SDWORD
            - ((*velocityptr).incr.height as UWORD as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as SDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn GetNextVelocityComponents(
    mut velocityptr: *mut VELOCITY_DESC,
    mut pdx: *mut SIZE,
    mut pdy: *mut SIZE,
    mut num_frames: COUNT,
) {
    let mut e: COUNT = 0;
    e = ((*velocityptr).error.width as COUNT as ::core::ffi::c_int
        + (*velocityptr).fract.width as COUNT as ::core::ffi::c_int
            * num_frames as ::core::ffi::c_int) as COUNT;
    *pdx = ((*velocityptr).vector.width as ::core::ffi::c_int
        * num_frames as ::core::ffi::c_int
        + (*velocityptr).incr.width as UWORD as BYTE as SBYTE as SIZE
            as ::core::ffi::c_int * (e as ::core::ffi::c_int >> 5 as ::core::ffi::c_int))
        as SIZE;
    (*velocityptr).error.width = (e as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COORD;
    e = ((*velocityptr).error.height as COUNT as ::core::ffi::c_int
        + (*velocityptr).fract.height as COUNT as ::core::ffi::c_int
            * num_frames as ::core::ffi::c_int) as COUNT;
    *pdy = ((*velocityptr).vector.height as ::core::ffi::c_int
        * num_frames as ::core::ffi::c_int
        + (*velocityptr).incr.height as UWORD as BYTE as SBYTE as SIZE
            as ::core::ffi::c_int * (e as ::core::ffi::c_int >> 5 as ::core::ffi::c_int))
        as SIZE;
    (*velocityptr).error.height = (e as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn GetNextVelocityComponentsSdword(
    mut velocityptr: *mut VELOCITY_DESC,
    mut pdx: *mut SDWORD,
    mut pdy: *mut SDWORD,
    mut num_frames: DWORD,
) {
    let mut e: DWORD = 0;
    e = ((*velocityptr).error.width as DWORD)
        .wrapping_add(((*velocityptr).fract.width as DWORD).wrapping_mul(num_frames));
    *pdx = ((*velocityptr).vector.width as SDWORD as DWORD)
        .wrapping_mul(num_frames)
        .wrapping_add(
            ((*velocityptr).incr.width as UWORD as BYTE as SBYTE as SDWORD as DWORD)
                .wrapping_mul(e >> 5 as ::core::ffi::c_int),
        ) as SDWORD;
    (*velocityptr).error.width = (e
        & (((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD) as COUNT as COORD;
    e = ((*velocityptr).error.height as DWORD)
        .wrapping_add(((*velocityptr).fract.height as DWORD).wrapping_mul(num_frames));
    *pdy = ((*velocityptr).vector.height as SDWORD as DWORD)
        .wrapping_mul(num_frames)
        .wrapping_add(
            ((*velocityptr).incr.height as UWORD as BYTE as SBYTE as SDWORD as DWORD)
                .wrapping_mul(e >> 5 as ::core::ffi::c_int),
        ) as SDWORD;
    (*velocityptr).error.height = (e
        & (((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD) as COUNT as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn SetVelocityVector(
    mut velocityptr: *mut VELOCITY_DESC,
    mut magnitude: SDWORD,
    mut facing: COUNT,
) {
    let mut angle: COUNT = 0;
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    (*velocityptr).TravelAngle = (((facing as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD)
        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
    angle = (*velocityptr).TravelAngle;
    magnitude = magnitude << 5 as ::core::ffi::c_int;
    dx = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            (angle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long * magnitude as ::core::ffi::c_long
        >> 14 as ::core::ffi::c_int) as SDWORD as SIZE;
    dy = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            (angle as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long * magnitude as ::core::ffi::c_long
        >> 14 as ::core::ffi::c_int) as SDWORD as SIZE;
    if dx as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        (*velocityptr).vector.width = (dx as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int) as COORD;
        (*velocityptr).incr.width = (((0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
    } else {
        dx = -(dx as ::core::ffi::c_int) as SIZE;
        (*velocityptr).vector.width = -(dx as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int) as COORD;
        (*velocityptr).incr.width = (((((dx as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | 0xff as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
    }
    if dy as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        (*velocityptr).vector.height = (dy as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int) as COORD;
        (*velocityptr).incr.height = (((0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
    } else {
        dy = -(dy as ::core::ffi::c_int) as SIZE;
        (*velocityptr).vector.height = -(dy as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int) as COORD;
        (*velocityptr).incr.height = (((((dy as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | 0xff as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
    }
    (*velocityptr).fract.width = (dx as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COORD;
    (*velocityptr).fract.height = (dy as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COORD;
    (*velocityptr).error.height = 0 as COORD;
    (*velocityptr).error.width = (*velocityptr).error.height;
}
#[no_mangle]
pub unsafe extern "C" fn SetVelocityComponents(
    mut velocityptr: *mut VELOCITY_DESC,
    mut dx: SDWORD,
    mut dy: SDWORD,
) {
    let mut angle: COUNT = 0;
    angle = ARCTAN(dx, dy);
    if angle as ::core::ffi::c_int
        == (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
    {
        memset(
            velocityptr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>() as size_t,
        );
    } else {
        if dx >= 0 as ::core::ffi::c_int {
            (*velocityptr).vector.width = (dx >> 5 as ::core::ffi::c_int) as COORD;
            (*velocityptr).incr.width = (((0 as ::core::ffi::c_int as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
        } else {
            dx = -dx;
            (*velocityptr).vector.width = -(dx >> 5 as ::core::ffi::c_int) as COORD;
            (*velocityptr).incr.width = (((((dx
                & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | 0xff as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
        }
        if dy >= 0 as ::core::ffi::c_int {
            (*velocityptr).vector.height = (dy >> 5 as ::core::ffi::c_int) as COORD;
            (*velocityptr).incr.height = (((0 as ::core::ffi::c_int as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
        } else {
            dy = -dy;
            (*velocityptr).vector.height = -(dy >> 5 as ::core::ffi::c_int) as COORD;
            (*velocityptr).incr.height = (((((dy
                & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | 0xff as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as COORD;
        }
        (*velocityptr).fract.width = (dx
            & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COORD;
        (*velocityptr).fract.height = (dy
            & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COORD;
        (*velocityptr).error.height = 0 as COORD;
        (*velocityptr).error.width = (*velocityptr).error.height;
    }
    (*velocityptr).TravelAngle = angle;
}
#[no_mangle]
pub unsafe extern "C" fn DeltaVelocityComponents(
    mut velocityptr: *mut VELOCITY_DESC,
    mut dx: SDWORD,
    mut dy: SDWORD,
) {
    dx
        += (((*velocityptr).vector.width as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int)
            + ((*velocityptr).fract.width as ::core::ffi::c_int
                - ((*velocityptr).incr.width as UWORD as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as BYTE as SIZE as ::core::ffi::c_int);
    dy
        += (((*velocityptr).vector.height as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int)
            + ((*velocityptr).fract.height as ::core::ffi::c_int
                - ((*velocityptr).incr.height as UWORD as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as BYTE as SIZE as ::core::ffi::c_int);
    SetVelocityComponents(velocityptr, dx, dy);
}
