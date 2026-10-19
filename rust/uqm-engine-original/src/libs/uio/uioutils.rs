/*
 * Copyright (C) 2003  Serge van den Boom
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * Nota bene: later versions of the GNU General Public License do not apply
 * to this program.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

// Rust translation of src/libs/uio/uioutils.c.
extern "C" {
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn uio_skipUNCServerShare(inPath: *const ::core::ffi::c_char) -> size_t;
}
pub type time_t = ::core::ffi::c_longlong;
pub type size_t = ::core::ffi::c_ulong;
pub type uio_uint16 = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn strcata(
    mut first: *const ::core::ffi::c_char,
    mut second: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut resPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut firstLen: size_t = 0;
    let mut secondLen: size_t = 0;
    firstLen = strlen(first) as size_t;
    secondLen = strlen(second) as size_t;
    result = malloc(firstLen.wrapping_add(secondLen).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    resPtr = result;
    memcpy(
        resPtr as *mut ::core::ffi::c_void,
        first as *const ::core::ffi::c_void,
        firstLen,
    );
    resPtr = resPtr.offset(firstLen as isize);
    memcpy(
        resPtr as *mut ::core::ffi::c_void,
        second as *const ::core::ffi::c_void,
        secondLen,
    );
    resPtr = resPtr.offset(secondLen as isize);
    *resPtr = '\0' as ::core::ffi::c_char;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn insertArray(
    mut array: *const ::core::ffi::c_void,
    mut oldNumElements: size_t,
    mut insertPos: ::core::ffi::c_int,
    mut element: *const ::core::ffi::c_void,
    mut elementSize: size_t,
) -> *mut ::core::ffi::c_void {
    let mut newArray: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut newArrayPtr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut arrayPtr: *const ::core::ffi::c_void = ::core::ptr::null::<
        ::core::ffi::c_void,
    >();
    let mut preInsertSize: size_t = 0;
    newArray = malloc(
        oldNumElements.wrapping_add(1 as size_t).wrapping_mul(elementSize),
    );
    preInsertSize = (insertPos as size_t).wrapping_mul(elementSize);
    memcpy(newArray, array, preInsertSize);
    newArrayPtr = (newArray as *mut ::core::ffi::c_char).offset(preInsertSize as isize)
        as *mut ::core::ffi::c_void;
    arrayPtr = (array as *const ::core::ffi::c_char).offset(preInsertSize as isize)
        as *const ::core::ffi::c_void;
    memcpy(newArrayPtr, element, elementSize);
    newArrayPtr = (newArrayPtr as *mut ::core::ffi::c_char).offset(elementSize as isize)
        as *mut ::core::ffi::c_void;
    memcpy(
        newArrayPtr,
        arrayPtr,
        oldNumElements.wrapping_sub(insertPos as size_t).wrapping_mul(elementSize),
    );
    return newArray;
}
#[no_mangle]
pub unsafe extern "C" fn insertArrayPointer(
    mut array: *mut *const ::core::ffi::c_void,
    mut oldNumElements: size_t,
    mut insertPos: ::core::ffi::c_int,
    mut element: *const ::core::ffi::c_void,
) -> *mut *mut ::core::ffi::c_void {
    let mut newArray: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    let mut newArrayPtr: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    let mut arrayPtr: *mut *const ::core::ffi::c_void = ::core::ptr::null_mut::<
        *const ::core::ffi::c_void,
    >();
    let mut preInsertSize: size_t = 0;
    newArray = malloc(
        oldNumElements
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    preInsertSize = (insertPos as usize)
        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>()) as size_t;
    memcpy(
        newArray as *mut ::core::ffi::c_void,
        array as *const ::core::ffi::c_void,
        preInsertSize,
    );
    newArrayPtr = newArray.offset(insertPos as isize);
    arrayPtr = array.offset(insertPos as isize);
    *newArrayPtr = unconst(element);
    newArrayPtr = newArrayPtr.offset(1);
    memcpy(
        newArrayPtr as *mut ::core::ffi::c_void,
        arrayPtr as *const ::core::ffi::c_void,
        oldNumElements
            .wrapping_sub(insertPos as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    );
    return newArray;
}
#[no_mangle]
pub unsafe extern "C" fn excludeArray(
    mut array: *const ::core::ffi::c_void,
    mut oldNumElements: size_t,
    mut startPos: ::core::ffi::c_int,
    mut numExclude: ::core::ffi::c_int,
    mut elementSize: size_t,
) -> *mut ::core::ffi::c_void {
    let mut newArray: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut newArrayPtr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut arrayPtr: *const ::core::ffi::c_void = ::core::ptr::null::<
        ::core::ffi::c_void,
    >();
    let mut preExcludeSize: size_t = 0;
    newArray = malloc(
        oldNumElements.wrapping_sub(numExclude as size_t).wrapping_mul(elementSize),
    );
    preExcludeSize = (startPos as size_t).wrapping_mul(elementSize);
    memcpy(newArray, array, preExcludeSize);
    newArrayPtr = (newArray as *mut ::core::ffi::c_char).offset(preExcludeSize as isize)
        as *mut ::core::ffi::c_void;
    arrayPtr = (array as *const ::core::ffi::c_char)
        .offset(
            ((startPos + numExclude) as usize)
                .wrapping_mul(::core::mem::size_of::<size_t>()) as isize,
        ) as *const ::core::ffi::c_void;
    memcpy(
        newArrayPtr,
        arrayPtr,
        oldNumElements
            .wrapping_sub(startPos as size_t)
            .wrapping_sub(numExclude as size_t)
            .wrapping_mul(elementSize),
    );
    return newArray;
}
#[no_mangle]
pub unsafe extern "C" fn excludeArrayPointer(
    mut array: *mut *const ::core::ffi::c_void,
    mut oldNumElements: size_t,
    mut startPos: ::core::ffi::c_int,
    mut numExclude: ::core::ffi::c_int,
) -> *mut *mut ::core::ffi::c_void {
    let mut newArray: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    newArray = malloc(
        oldNumElements
            .wrapping_sub(numExclude as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    memcpy(
        newArray as *mut ::core::ffi::c_void,
        array as *const ::core::ffi::c_void,
        (startPos as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    );
    memcpy(
        newArray.offset(startPos as isize) as *mut ::core::ffi::c_void,
        array.offset((startPos + numExclude) as isize) as *const ::core::ffi::c_void,
        oldNumElements
            .wrapping_sub(startPos as size_t)
            .wrapping_sub(numExclude as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
    );
    return newArray;
}
#[no_mangle]
pub unsafe extern "C" fn dosToUnixTime(
    mut date: uio_uint16,
    mut tm: uio_uint16,
) -> time_t {
    let mut year: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hours: ::core::ffi::c_int = 0;
    let mut minutes: ::core::ffi::c_int = 0;
    let mut seconds: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_long = 0;
    static mut daysUntilMonth: [::core::ffi::c_int; 16] = [
        0 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int,
        90 as ::core::ffi::c_int,
        120 as ::core::ffi::c_int,
        151 as ::core::ffi::c_int,
        181 as ::core::ffi::c_int,
        212 as ::core::ffi::c_int,
        243 as ::core::ffi::c_int,
        273 as ::core::ffi::c_int,
        304 as ::core::ffi::c_int,
        334 as ::core::ffi::c_int,
        334 as ::core::ffi::c_int,
        334 as ::core::ffi::c_int,
        334 as ::core::ffi::c_int,
        334 as ::core::ffi::c_int,
    ];
    year = date as ::core::ffi::c_int >> 9 as ::core::ffi::c_int;
    month = (date as ::core::ffi::c_int >> 5 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
    day = date as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        & 0x1f as ::core::ffi::c_int;
    hours = tm as ::core::ffi::c_int >> 11 as ::core::ffi::c_int;
    minutes = tm as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
        & 0x3f as ::core::ffi::c_int;
    seconds = (tm as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
        * 2 as ::core::ffi::c_int;
    result = (year * 365 as ::core::ffi::c_int + daysUntilMonth[month as usize] + day)
        as ::core::ffi::c_long;
    if year % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && month <= 2 as ::core::ffi::c_int
    {
        result += (year / 4 as ::core::ffi::c_int) as ::core::ffi::c_long;
    } else {
        result
            += (1 as ::core::ffi::c_int + year / 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long;
    }
    result
        += (365 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) as ::core::ffi::c_long;
    result = result * 24 as ::core::ffi::c_long + hours as ::core::ffi::c_long;
    result = result * 60 as ::core::ffi::c_long + minutes as ::core::ffi::c_long;
    result = result * 60 as ::core::ffi::c_long + seconds as ::core::ffi::c_long;
    return result as time_t;
}
#[no_mangle]
pub unsafe extern "C" fn dosToUnixPath(
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut srcPtr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dstPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut skip: size_t = 0;
    result = malloc(strlen(path).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    srcPtr = path;
    dstPtr = result;
    skip = uio_skipUNCServerShare(srcPtr);
    if skip != 0 as size_t {
        let mut slash: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        memcpy(
            dstPtr as *mut ::core::ffi::c_void,
            srcPtr as *const ::core::ffi::c_void,
            skip,
        );
        slash = memchr(
            srcPtr.offset(2 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            '/' as ::core::ffi::c_int,
            skip.wrapping_sub(2 as size_t),
        ) as *mut ::core::ffi::c_char;
        if !slash.is_null() {
            *slash = '\\' as ::core::ffi::c_char;
        }
        srcPtr = srcPtr.offset(skip as isize);
        dstPtr = dstPtr.offset(skip as isize);
    }
    while *srcPtr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if *srcPtr as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            *dstPtr = '/' as ::core::ffi::c_char;
        } else {
            *dstPtr = *srcPtr;
        }
        srcPtr = srcPtr.offset(1);
        dstPtr = dstPtr.offset(1);
    }
    *dstPtr = '\0' as ::core::ffi::c_char;
    return result;
}
#[inline]
unsafe extern "C" fn unconst(
    mut arg: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut u: C2Rust_Unnamed = C2Rust_Unnamed {
        c: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    u.cc = arg;
    return u.c;
}
