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

// Rust translation of src/libs/math/random.c.
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
static mut seed: DWORD = 12345 as DWORD;
#[no_mangle]
pub unsafe extern "C" fn TFB_Random() -> DWORD {
    seed = (16807 as ::core::ffi::c_ulong)
        .wrapping_mul(
            (seed as ::core::ffi::c_ulong).wrapping_rem(127773 as ::core::ffi::c_ulong),
        )
        .wrapping_sub(
            (2836 as ::core::ffi::c_ulong)
                .wrapping_mul(
                    (seed as ::core::ffi::c_ulong)
                        .wrapping_div(127773 as ::core::ffi::c_ulong),
                ),
        ) as DWORD;
    if seed as ::core::ffi::c_ulong > 2147483647 as ::core::ffi::c_ulong {
        seed = (seed as ::core::ffi::c_ulong)
            .wrapping_sub(2147483647 as ::core::ffi::c_ulong) as DWORD;
        return seed;
    } else if seed != 0 {
        return seed
    } else {
        seed = 1 as DWORD;
        return seed;
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SeedRandom(mut new_seed: DWORD) -> DWORD {
    let mut old_seed: DWORD = 0;
    if new_seed as ::core::ffi::c_ulong == 0 as ::core::ffi::c_ulong {
        new_seed = 1 as DWORD;
    } else if new_seed as ::core::ffi::c_ulong > 2147483647 as ::core::ffi::c_ulong {
        new_seed = (new_seed as ::core::ffi::c_ulong)
            .wrapping_sub(2147483647 as ::core::ffi::c_ulong) as DWORD;
    }
    old_seed = seed;
    seed = new_seed;
    return old_seed;
}
