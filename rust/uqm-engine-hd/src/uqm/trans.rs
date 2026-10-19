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

// Rust translation of src/uqm/trans.c.
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[no_mangle]
pub static mut sinetab: [SDWORD; 64] = [
    -((1.000000f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.995185f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.980785f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.956940f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.923880f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.881921f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.831470f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.773010f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.707107f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.634393f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.555570f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.471397f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.382683f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.290285f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.195090f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.098017f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    (0.000000f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.098017f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.195090f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.290285f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.382683f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.471397f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.555570f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.634393f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.707107f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.773010f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.831470f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.881921f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.923880f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.956940f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.980785f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.995185f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (1.000000f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.995185f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.980785f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.956940f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.923880f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.881921f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.831470f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.773010f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.707107f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.634393f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.555570f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.471397f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.382683f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.290285f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.195090f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.098017f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    (0.000000f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD,
    -((0.098017f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.195090f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.290285f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.382683f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.471397f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.555570f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.634393f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.707107f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.773010f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.831470f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.881921f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.923880f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.956940f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.980785f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
    -((0.995185f64
        * ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as SIZE as SDWORD),
];
#[no_mangle]
pub unsafe extern "C" fn ARCTAN(mut delta_x: SDWORD, mut delta_y: SDWORD) -> COUNT {
    let mut v1: SDWORD = 0;
    let mut v2: SDWORD = 0;
    static mut atantab: [COUNT; 33] = [
        0 as COUNT,
        0 as COUNT,
        1 as COUNT,
        1 as COUNT,
        1 as COUNT,
        2 as COUNT,
        2 as COUNT,
        2 as COUNT,
        2 as COUNT,
        3 as COUNT,
        3 as COUNT,
        3 as COUNT,
        4 as COUNT,
        4 as COUNT,
        4 as COUNT,
        4 as COUNT,
        5 as COUNT,
        5 as COUNT,
        5 as COUNT,
        5 as COUNT,
        6 as COUNT,
        6 as COUNT,
        6 as COUNT,
        6 as COUNT,
        7 as COUNT,
        7 as COUNT,
        7 as COUNT,
        7 as COUNT,
        7 as COUNT,
        7 as COUNT,
        8 as COUNT,
        8 as COUNT,
        8 as COUNT,
    ];
    v1 = delta_x;
    v2 = delta_y;
    if v1 == 0 as ::core::ffi::c_int && v2 == 0 as ::core::ffi::c_int {
        return ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as COUNT;
    }
    if v1 < 0 as ::core::ffi::c_int {
        v1 = -v1;
    }
    if v2 < 0 as ::core::ffi::c_int {
        v2 = -v2;
    }
    if v1 > v2 {
        v1 = (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            - atantab[((v2 as DWORD)
                << 6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                .wrapping_add((v1 >> 1 as ::core::ffi::c_int) as DWORD)
                .wrapping_div(v1 as DWORD) as usize] as ::core::ffi::c_int) as SDWORD;
    } else {
        v1 = atantab[((v1 as DWORD) << 6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            .wrapping_add((v2 >> 1 as ::core::ffi::c_int) as DWORD)
            .wrapping_div(v2 as DWORD) as usize] as SDWORD;
    }
    if delta_x < 0 as ::core::ffi::c_int {
        v1 = ((1 as SDWORD) << 6 as ::core::ffi::c_int) - v1;
    }
    if delta_y > 0 as ::core::ffi::c_int {
        v1 = ((1 as SDWORD) << 6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) - v1;
    }
    return (v1
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD as COUNT;
}
