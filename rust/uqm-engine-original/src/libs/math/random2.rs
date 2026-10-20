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

// This file contains variants of the random functions in random.c
// that store the state of the RNG in a context, allowing for multiple
// independant RNGs to be used simultaneously.
// The RNG behavior itself is the same.

// Rust translation of src/libs/math/random2.c.
extern "C" {
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
}
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RandomContext {
    pub seed: DWORD,
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn RandomContext_New() -> *mut RandomContext {
    let mut result: *mut RandomContext = HMalloc(::core::mem::size_of::<RandomContext>())
        as *mut RandomContext;
    (*result).seed = 12345 as DWORD;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Delete(mut context: *mut RandomContext) {
    HFree(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Copy(
    mut source: *const RandomContext,
) -> *mut RandomContext {
    let mut result: *mut RandomContext = HMalloc(::core::mem::size_of::<RandomContext>())
        as *mut RandomContext;
    *result = *source;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Random(mut context: *mut RandomContext) -> DWORD {
    (*context).seed = (16807 as DWORD)
        .wrapping_mul(
            (*context).seed.wrapping_rem(127773 as ::core::ffi::c_int as DWORD),
        )
        .wrapping_sub(
            (2836 as DWORD)
                .wrapping_mul(
                    (*context).seed.wrapping_div(127773 as ::core::ffi::c_int as DWORD),
                ),
        );
    if (*context).seed > 2147483647 as ::core::ffi::c_int as DWORD {
        (*context).seed = (*context)
            .seed
            .wrapping_sub(2147483647 as ::core::ffi::c_int as DWORD);
    } else if (*context).seed == 0 as DWORD {
        (*context).seed = 1 as DWORD;
    }
    return (*context).seed;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_SeedRandom(
    mut context: *mut RandomContext,
    mut new_seed: DWORD,
) -> DWORD {
    let mut old_seed: DWORD = 0;
    if new_seed == 0 as DWORD {
        new_seed = 1 as DWORD;
    } else if new_seed > 2147483647 as ::core::ffi::c_int as DWORD {
        new_seed = new_seed.wrapping_sub(2147483647 as ::core::ffi::c_int as DWORD);
    }
    old_seed = (*context).seed;
    (*context).seed = new_seed;
    return old_seed;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_GetSeed(
    mut context: *mut RandomContext,
) -> DWORD {
    return (*context).seed;
}
