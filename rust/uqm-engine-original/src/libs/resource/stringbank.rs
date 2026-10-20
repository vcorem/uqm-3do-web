/*  stringbank.c, Copyright (c) 2005 Michael C. Martin */

/*
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope thta it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  Se the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

// Rust translation of src/libs/resource/stringbank.c.
extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stringbank_chunk {
    pub data: [::core::ffi::c_char; 1016],
    pub len: ::core::ffi::c_int,
    pub next: *mut _stringbank_chunk,
}
pub type stringbank = _stringbank_chunk;
unsafe extern "C" fn add_chunk(mut bank: *mut stringbank) -> *mut stringbank {
    let mut n: *mut stringbank = malloc(::core::mem::size_of::<stringbank>())
        as *mut stringbank;
    (*n).len = 0 as ::core::ffi::c_int;
    (*n).next = ::core::ptr::null_mut::<_stringbank_chunk>();
    if !bank.is_null() {
        while !(*bank).next.is_null() {
            bank = (*bank).next as *mut stringbank;
        }
        (*bank).next = n as *mut _stringbank_chunk;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn StringBank_Create() -> *mut stringbank {
    return add_chunk(::core::ptr::null_mut::<stringbank>());
}
#[no_mangle]
pub unsafe extern "C" fn StringBank_Free(mut bank: *mut stringbank) {
    if !bank.is_null() {
        StringBank_Free((*bank).next);
        free(bank as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn StringBank_AddString(
    mut bank: *mut stringbank,
    mut str: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut len: ::core::ffi::c_uint = strlen(str).wrapping_add(1 as size_t)
        as ::core::ffi::c_uint;
    let mut x: *mut stringbank = bank;
    if len as usize
        > 1024usize
            .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>())
            .wrapping_sub(::core::mem::size_of::<::core::ffi::c_int>())
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while !x.is_null() {
        let mut remaining: ::core::ffi::c_uint = 1024usize
            .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>())
            .wrapping_sub(::core::mem::size_of::<::core::ffi::c_int>())
            .wrapping_sub((*x).len as usize) as ::core::ffi::c_uint;
        if len < remaining {
            let mut result: *mut ::core::ffi::c_char = (&raw mut (*x).data
                as *mut ::core::ffi::c_char)
                .offset((*x).len as isize);
            strcpy(result, str);
            (*x).len = ((*x).len as ::core::ffi::c_uint).wrapping_add(len)
                as ::core::ffi::c_int;
            return result;
        }
        x = (*x).next as *mut stringbank;
    }
    x = add_chunk(bank);
    strcpy(&raw mut (*x).data as *mut ::core::ffi::c_char, str);
    (*x).len = ((*x).len as ::core::ffi::c_uint).wrapping_add(len) as ::core::ffi::c_int;
    return &raw mut (*x).data as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn StringBank_AddOrFindString(
    mut bank: *mut stringbank,
    mut str: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut len: ::core::ffi::c_uint = strlen(str).wrapping_add(1 as size_t)
        as ::core::ffi::c_uint;
    let mut x: *mut stringbank = bank;
    if len as usize
        > 1024usize
            .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>())
            .wrapping_sub(::core::mem::size_of::<::core::ffi::c_int>())
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while !x.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*x).len {
            if strcmp(
                (&raw mut (*x).data as *mut ::core::ffi::c_char).offset(i as isize),
                str,
            ) == 0
            {
                return (&raw mut (*x).data as *mut ::core::ffi::c_char)
                    .offset(i as isize);
            }
            while (*x).data[i as usize] != 0 {
                i += 1;
            }
            i += 1;
        }
        x = (*x).next as *mut stringbank;
    }
    return StringBank_AddString(bank, str);
}
static mut buffer: [::core::ffi::c_char; 1016] = [0; 1016];
#[no_mangle]
pub unsafe extern "C" fn StringBank_AddSubstring(
    mut bank: *mut stringbank,
    mut str: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    let mut len: ::core::ffi::c_uint = strlen(str) as ::core::ffi::c_uint;
    if n > len {
        return StringBank_AddString(bank, str);
    }
    if n as usize
        >= 1024usize
            .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>())
            .wrapping_sub(::core::mem::size_of::<::core::ffi::c_int>())
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    strncpy(&raw mut buffer as *mut ::core::ffi::c_char, str, n as size_t);
    buffer[n as usize] = '\0' as ::core::ffi::c_char;
    return StringBank_AddString(bank, &raw mut buffer as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn StringBank_AddOrFindSubstring(
    mut bank: *mut stringbank,
    mut str: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    let mut len: ::core::ffi::c_uint = strlen(str) as ::core::ffi::c_uint;
    if n > len {
        return StringBank_AddOrFindString(bank, str);
    }
    if n as usize
        >= 1024usize
            .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>())
            .wrapping_sub(::core::mem::size_of::<::core::ffi::c_int>())
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    strncpy(&raw mut buffer as *mut ::core::ffi::c_char, str, n as size_t);
    buffer[n as usize] = '\0' as ::core::ffi::c_char;
    return StringBank_AddOrFindString(bank, &raw mut buffer as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SplitString(
    mut s: *const ::core::ffi::c_char,
    mut splitchar: ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut result: *mut *const ::core::ffi::c_char,
    mut bank: *mut stringbank,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut index: *const ::core::ffi::c_char = s;
    i = 0 as ::core::ffi::c_int;
    while i < n - 1 as ::core::ffi::c_int {
        let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut len: ::core::ffi::c_int = 0;
        next = strchr(index, splitchar as ::core::ffi::c_int);
        if next.is_null() {
            break;
        }
        len = next.offset_from(index) as ::core::ffi::c_int;
        *result.offset(i as isize) = StringBank_AddOrFindSubstring(
            bank,
            index,
            len as ::core::ffi::c_uint,
        );
        index = next.offset(1 as ::core::ffi::c_int as isize);
        i += 1;
    }
    *result.offset(i as isize) = StringBank_AddOrFindString(bank, index);
    return i + 1 as ::core::ffi::c_int;
}
