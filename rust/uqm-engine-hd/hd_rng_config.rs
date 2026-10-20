///Copyright Paul Reiche, Fred Ford. 1992-2002

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

 /****************************************************************************
* FILE: random.c
* DESC: a library of random number generators for general purpose use.
*
* References:
* "Random Number Generators: Good ones are hard to find" S.K.Park & K.W.Miller
* Communications of the ACM, Vol31 Number 10, October 1988, Pp 1192-1201
*
* HISTORY: Created 1/23/1989
* LAST CHANGED:
*
* Copyright (c) 1989, Robert Leyland and Fred Ford
****************************************************************************/

/* ----------------------------INCLUDES----------------------------------- */

// SPDX-License-Identifier: GPL-2.0-or-later
// SeedA from src/libs/math/random.h and printBits from random.c.
// Access follows the same engine synchronization as the original macros.
type size_t = core::ffi::c_ulong;
extern "C" {
    fn printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn puts(text: *const core::ffi::c_char) -> core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uqm_hd_seed_a() -> u32 {
    let seed = crate::src::uqm::globdata::GlobData.SIS_state.Seed;
    if crate::src::options::optSeedType == 1 && (3..=2_147_483_644).contains(&seed) {
        seed as u32
    } else {
        16_807
    }
}
#[no_mangle]
pub unsafe extern "C" fn printBits(size: size_t, ptr: *const ::core::ffi::c_void) {
    let mut b: *mut ::core::ffi::c_uchar = ptr as *mut ::core::ffi::c_uchar;
    let mut byte: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = size.wrapping_sub(1 as size_t) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        j = 7 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            byte = (*b.offset(i as isize) as ::core::ffi::c_int >> j
                & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            printf(
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                byte as ::core::ffi::c_int,
            );
            j -= 1;
        }
        i -= 1;
    }
    puts(b"\0".as_ptr() as *const ::core::ffi::c_char);
}
