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

// Rust translation of src/libs/math/sqrt.c.
pub type uint32_t = ::core::ffi::c_uint;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type UNICODE = ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn square_root(mut value: DWORD) -> COUNT {
    let mut sig_word: UWORD = 0;
    let mut mask: UWORD = 0;
    let mut result: COUNT = 0;
    let mut shift: COUNT = 0;
    sig_word = (value >> 16 as ::core::ffi::c_int) as UWORD;
    if sig_word as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut mask_squared: DWORD = 0;
        let mut result_shift: DWORD = 0;
        mask = ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as UWORD;
        shift = 31 as COUNT;
        while mask as ::core::ffi::c_int & sig_word as ::core::ffi::c_int == 0 {
            mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as UWORD;
            shift = shift.wrapping_sub(1);
        }
        shift = (shift as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
        mask = ((1 as ::core::ffi::c_int) << shift as ::core::ffi::c_int) as UWORD;
        result = mask as COUNT;
        result_shift = (mask as DWORD) << shift as ::core::ffi::c_int;
        mask_squared = result_shift;
        value = value.wrapping_sub(mask_squared);
        loop {
            mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as UWORD;
            if mask == 0 {
                break;
            }
            let mut remainder: DWORD = 0;
            mask_squared >>= 1 as ::core::ffi::c_int;
            mask_squared >>= 1 as ::core::ffi::c_int;
            remainder = result_shift.wrapping_add(mask_squared);
            if remainder > value {
                result_shift >>= 1 as ::core::ffi::c_int;
            } else {
                value = value.wrapping_sub(remainder);
                result_shift = (result_shift >> 1 as ::core::ffi::c_int)
                    .wrapping_add(mask_squared);
                result = (result as ::core::ffi::c_int | mask as ::core::ffi::c_int)
                    as COUNT;
            }
        }
        return result;
    } else {
        sig_word = value as UWORD;
        if sig_word as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let mut mask_squared_0: UWORD = 0;
            let mut result_shift_0: UWORD = 0;
            mask = ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as UWORD;
            shift = 15 as COUNT;
            while mask as ::core::ffi::c_int & sig_word as ::core::ffi::c_int == 0 {
                mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as UWORD;
                shift = shift.wrapping_sub(1);
            }
            shift = (shift as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
            mask = ((1 as ::core::ffi::c_int) << shift as ::core::ffi::c_int) as UWORD;
            result = mask as COUNT;
            result_shift_0 = ((mask as ::core::ffi::c_int)
                << shift as ::core::ffi::c_int) as UWORD;
            mask_squared_0 = result_shift_0;
            sig_word = (sig_word as ::core::ffi::c_int
                - mask_squared_0 as ::core::ffi::c_int) as UWORD;
            loop {
                mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as UWORD;
                if mask == 0 {
                    break;
                }
                let mut remainder_0: UWORD = 0;
                mask_squared_0 = (mask_squared_0 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as UWORD;
                mask_squared_0 = (mask_squared_0 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as UWORD;
                remainder_0 = (result_shift_0 as ::core::ffi::c_int
                    + mask_squared_0 as ::core::ffi::c_int) as UWORD;
                if remainder_0 as ::core::ffi::c_int > sig_word as ::core::ffi::c_int {
                    result_shift_0 = (result_shift_0 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as UWORD;
                } else {
                    sig_word = (sig_word as ::core::ffi::c_int
                        - remainder_0 as ::core::ffi::c_int) as UWORD;
                    result_shift_0 = ((result_shift_0 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + mask_squared_0 as ::core::ffi::c_int) as UWORD;
                    result = (result as ::core::ffi::c_int | mask as ::core::ffi::c_int)
                        as COUNT;
                }
            }
            return result;
        }
    }
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn crc32b(mut str: *const ::core::ffi::c_char) -> uint32_t {
    let mut byte: ::core::ffi::c_uint = 0;
    let mut crc: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    crc = 0xffffffff as ::core::ffi::c_uint;
    while *str.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        byte = *str.offset(i as isize) as ::core::ffi::c_uint;
        crc = crc ^ byte;
        j = 7 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            mask = -((crc & 1 as ::core::ffi::c_uint) as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
            crc = crc >> 1 as ::core::ffi::c_int
                ^ 0xedb88320 as ::core::ffi::c_uint & mask;
            j -= 1;
        }
        i = i + 1 as ::core::ffi::c_int;
    }
    return !crc;
}
