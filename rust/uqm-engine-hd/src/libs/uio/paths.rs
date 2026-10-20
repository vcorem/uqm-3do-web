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

// Rust translation of src/libs/uio/paths.c.
extern "C" {
    pub type _IO_FILE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
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
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PathComp {
    pub name: *mut ::core::ffi::c_char,
    pub nameLen: size_t,
    pub next: *mut uio_PathComp,
    pub up: *mut uio_PathComp,
}
pub type uio_bool = bool;
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn isPathDelimiter(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c == '/' as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getFirstPathComponent(
    mut dir: *const ::core::ffi::c_char,
    mut dirEnd: *const ::core::ffi::c_char,
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    (dir <= dirEnd
        || {
            __assert_fail(
                b"dir <= dirEnd\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/paths.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                40 as ::core::ffi::c_int,
                b"getFirstPathComponent\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    *startComp = dir;
    if *startComp == dirEnd {
        *endComp = *startComp;
        return;
    }
    *endComp = memchr(
        *startComp as *const ::core::ffi::c_void,
        '/' as ::core::ffi::c_int,
        dirEnd.offset_from(*startComp) as size_t,
    ) as *const ::core::ffi::c_char;
    if (*endComp).is_null() {
        *endComp = dirEnd;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getFirstPath0Component(
    mut dir: *const ::core::ffi::c_char,
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    *startComp = dir;
    if **startComp as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *endComp = *startComp;
        return;
    }
    *endComp = strchr(*startComp, '/' as ::core::ffi::c_int);
    if (*endComp).is_null() {
        *endComp = (*startComp).offset(strlen(*startComp) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getNextPathComponent(
    mut dirEnd: *const ::core::ffi::c_char,
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    (*endComp <= dirEnd
        || {
            __assert_fail(
                b"*endComp <= dirEnd\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/paths.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                77 as ::core::ffi::c_int,
                b"getNextPathComponent\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if *endComp == dirEnd {
        *startComp = *endComp;
        return;
    }
    (**endComp as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || {
            __assert_fail(
                b"**endComp == '/'\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/paths.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int,
                b"getNextPathComponent\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    *startComp = (*endComp).offset(1 as ::core::ffi::c_int as isize);
    *endComp = memchr(
        *startComp as *const ::core::ffi::c_void,
        '/' as ::core::ffi::c_int,
        dirEnd.offset_from(*startComp) as size_t,
    ) as *const ::core::ffi::c_char;
    if (*endComp).is_null() {
        *endComp = dirEnd;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getNextPath0Component(
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    if **endComp as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *startComp = *endComp;
        return;
    }
    (**endComp as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || {
            __assert_fail(
                b"**endComp == '/'\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/paths.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                b"getNextPath0Component\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    *startComp = (*endComp).offset(1 as ::core::ffi::c_int as isize);
    *endComp = strchr(*startComp, '/' as ::core::ffi::c_int);
    if (*endComp).is_null() {
        *endComp = (*startComp).offset(strlen(*startComp) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getLastPathComponent(
    mut dir: *const ::core::ffi::c_char,
    mut endDir: *const ::core::ffi::c_char,
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    *endComp = endDir;
    *startComp = *endComp;
    while (*startComp).offset(-(1 as ::core::ffi::c_int as isize)) >= dir
        && *(*startComp).offset(-(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        *startComp = (*startComp).offset(-1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getLastPath0Component(
    mut dir: *const ::core::ffi::c_char,
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    *endComp = dir.offset(strlen(dir) as isize);
    *startComp = *endComp;
    while (*startComp).offset(-(1 as ::core::ffi::c_int as isize)) >= dir
        && *(*startComp).offset(-(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        *startComp = (*startComp).offset(-1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getPreviousPathComponent(
    mut dir: *const ::core::ffi::c_char,
    mut startComp: *mut *const ::core::ffi::c_char,
    mut endComp: *mut *const ::core::ffi::c_char,
) {
    if *startComp == dir {
        *endComp = *startComp;
        return;
    }
    *endComp = (*startComp).offset(-(1 as ::core::ffi::c_int as isize));
    *startComp = *endComp;
    while (*startComp).offset(-(1 as ::core::ffi::c_int as isize)) >= dir
        && *(*startComp).offset(-(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        *startComp = (*startComp).offset(-1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn joinPaths(
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
    if *first.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return strdup(second);
    }
    firstLen = strlen(first);
    if *first.offset(firstLen.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        == '/' as ::core::ffi::c_int
    {
        firstLen = firstLen.wrapping_sub(1);
    }
    secondLen = strlen(second);
    result = malloc(firstLen.wrapping_add(secondLen).wrapping_add(2 as size_t))
        as *mut ::core::ffi::c_char;
    resPtr = result;
    memcpy(
        resPtr as *mut ::core::ffi::c_void,
        first as *const ::core::ffi::c_void,
        firstLen,
    );
    resPtr = resPtr.offset(firstLen as isize);
    *resPtr = '/' as ::core::ffi::c_char;
    resPtr = resPtr.offset(1);
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
pub unsafe extern "C" fn joinPathsAbsolute(
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
    if *first.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        secondLen = strlen(second);
        result = malloc(secondLen.wrapping_add(2 as size_t)) as *mut ::core::ffi::c_char;
        *result.offset(0isize) = '/' as ::core::ffi::c_char;
        memcpy(
            result.offset(1isize) as *mut ::core::ffi::c_void,
            second as *const ::core::ffi::c_void,
            secondLen,
        );
        *result.offset(secondLen.wrapping_add(1 as size_t) as isize) = '\0'
            as ::core::ffi::c_char;
        return result;
    }
    firstLen = strlen(first);
    if *first.offset(firstLen.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        == '/' as ::core::ffi::c_int
    {
        firstLen = firstLen.wrapping_sub(1);
    }
    secondLen = strlen(second);
    result = malloc(firstLen.wrapping_add(secondLen).wrapping_add(3 as size_t))
        as *mut ::core::ffi::c_char;
    resPtr = result;
    *resPtr = '/' as ::core::ffi::c_char;
    resPtr = resPtr.offset(1);
    memcpy(
        resPtr as *mut ::core::ffi::c_void,
        first as *const ::core::ffi::c_void,
        firstLen,
    );
    resPtr = resPtr.offset(firstLen as isize);
    *resPtr = '/' as ::core::ffi::c_char;
    resPtr = resPtr.offset(1);
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
pub unsafe extern "C" fn validPathName(
    mut path: *const ::core::ffi::c_char,
    mut len: size_t,
) -> uio_bool {
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    pathEnd = path.offset(len as isize);
    getFirstPathComponent(path, pathEnd, &raw mut start, &raw mut end);
    while start < pathEnd {
        if end == start
            || end.offset_from(start) == 1isize
                && *start.offset(0isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
            || end.offset_from(start) == 2isize
                && *start.offset(0isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                && *start.offset(1isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
        {
            return false;
        }
        getNextPathComponent(pathEnd, &raw mut start, &raw mut end);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn uio_skipUNCServerShare(
    mut inPath: *const ::core::ffi::c_char,
) -> size_t {
    let mut path: *const ::core::ffi::c_char = inPath;
    if *path.offset(0isize) as ::core::ffi::c_int != '\\' as ::core::ffi::c_int
        || *path.offset(1isize) as ::core::ffi::c_int != '\\' as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int as size_t;
    }
    path = path.offset(2 as ::core::ffi::c_int as isize);
    while *path as ::core::ffi::c_int != '\\' as ::core::ffi::c_int
        && *path as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        if *path as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int as size_t;
        }
        path = path.offset(1);
    }
    path = path.offset(1);
    while *path as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *path as ::core::ffi::c_int != '\\' as ::core::ffi::c_int
        && *path as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        path = path.offset(1);
    }
    return path.offset_from(inPath) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getUNCServerShare(
    mut inPath: *const ::core::ffi::c_char,
    mut outPath: *mut *mut ::core::ffi::c_char,
    mut outLen: *mut size_t,
) -> size_t {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut server: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut serverEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut share: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut shareEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut nameEnd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut nameLen: size_t = 0;
    ptr = inPath;
    '_noMatch: {
        if !(*ptr.offset(0isize) as ::core::ffi::c_int != '\\' as ::core::ffi::c_int
            || *ptr.offset(1isize) as ::core::ffi::c_int != '\\' as ::core::ffi::c_int)
        {
            server = ptr.offset(2 as ::core::ffi::c_int as isize);
            serverEnd = server;
            loop {
                if *serverEnd as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    break '_noMatch;
                }
                if isPathDelimiter(*serverEnd as ::core::ffi::c_int) != 0 {
                    break;
                }
                serverEnd = serverEnd.offset(1);
            }
            if serverEnd != server {
                share = serverEnd.offset(1 as ::core::ffi::c_int as isize);
                shareEnd = share;
                while *shareEnd as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    if isPathDelimiter(*shareEnd as ::core::ffi::c_int) != 0 {
                        break;
                    }
                    serverEnd = serverEnd.offset(1);
                }
                ptr = shareEnd;
                while isPathDelimiter(*ptr as ::core::ffi::c_int) != 0 {
                    ptr = ptr.offset(1);
                }
                nameLen = (serverEnd.offset_from(server) + shareEnd.offset_from(share)
                    + 3isize) as size_t;
                name = malloc(nameLen.wrapping_add(1 as size_t))
                    as *mut ::core::ffi::c_char;
                nameEnd = name;
                let c2rust_fresh0 = nameEnd;
                nameEnd = nameEnd.offset(1);
                *c2rust_fresh0 = '\\' as ::core::ffi::c_char;
                let c2rust_fresh1 = nameEnd;
                nameEnd = nameEnd.offset(1);
                *c2rust_fresh1 = '\\' as ::core::ffi::c_char;
                memcpy(
                    nameEnd as *mut ::core::ffi::c_void,
                    server as *const ::core::ffi::c_void,
                    serverEnd.offset_from(server) as size_t,
                );
                let c2rust_fresh2 = nameEnd;
                nameEnd = nameEnd.offset(1);
                *c2rust_fresh2 = '\\' as ::core::ffi::c_char;
                memcpy(
                    nameEnd as *mut ::core::ffi::c_void,
                    share as *const ::core::ffi::c_void,
                    shareEnd.offset_from(share) as size_t,
                );
                *nameEnd = '\0' as ::core::ffi::c_char;
                *outPath = name;
                if !outLen.is_null() {
                    *outLen = nameLen;
                }
                return ptr.offset_from(inPath) as size_t;
            }
        }
    }
    *outPath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !outLen.is_null() {
        *outLen = 0 as size_t;
    }
    return 0 as ::core::ffi::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn decomposePath(
    mut path: *const ::core::ffi::c_char,
    mut pathComp: *mut *mut uio_PathComp,
    mut isAbsolute: *mut uio_bool,
) -> ::core::ffi::c_int {
    let mut result: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut last: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut endResult: *mut *mut uio_PathComp = &raw mut result;
    let mut absolute: uio_bool = false;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *__errno_location() = 44 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    last = ::core::ptr::null_mut::<uio_PathComp>();
    if isPathDelimiter(*path as ::core::ffi::c_int) != 0 {
        absolute = true;
        loop {
            path = path.offset(1);
            if isPathDelimiter(*path as ::core::ffi::c_int) == 0 {
                break;
            }
        }
    }
    while *path as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = path;
        while *path as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && isPathDelimiter(*path as ::core::ffi::c_int) == 0
        {
            path = path.offset(1);
        }
        name = uio_memdup0(path, path.offset_from(start) as size_t);
        *endResult = uio_PathComp_new(name, path.offset_from(start) as size_t, last);
        last = *endResult;
        endResult = &raw mut (*last).next as *mut *mut uio_PathComp;
        while isPathDelimiter(*path as ::core::ffi::c_int) != 0 {
            path = path.offset(1);
        }
    }
    *endResult = ::core::ptr::null_mut::<uio_PathComp>();
    *pathComp = result;
    if !isAbsolute.is_null() {
        *isAbsolute = absolute;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn composePath(
    mut pathComp: *const uio_PathComp,
    mut absolute: uio_bool,
    mut path: *mut *mut ::core::ffi::c_char,
    mut pathLen: *mut size_t,
) {
    let mut len: size_t = 0;
    let mut ptr: *const uio_PathComp = ::core::ptr::null::<uio_PathComp>();
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pathPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    (!pathComp.is_null()
        || {
            __assert_fail(
                b"pathComp != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/paths.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                444 as ::core::ffi::c_int,
                b"composePath\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    len = 0 as size_t;
    if absolute {
        len = len.wrapping_add(1);
    }
    ptr = pathComp;
    while !ptr.is_null() {
        len = len.wrapping_add((*ptr).nameLen);
        ptr = (*ptr).next;
    }
    result = malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    pathPtr = result;
    ptr = pathComp;
    if absolute {
        let c2rust_fresh3 = pathPtr;
        pathPtr = pathPtr.offset(1);
        *c2rust_fresh3 = '/' as ::core::ffi::c_char;
    }
    loop {
        memcpy(
            pathPtr as *mut ::core::ffi::c_void,
            (*ptr).name as *const ::core::ffi::c_void,
            (*ptr).nameLen,
        );
        pathPtr = pathPtr.offset((*ptr).nameLen as isize);
        ptr = (*ptr).next;
        if ptr.is_null() {
            break;
        }
        let c2rust_fresh4 = pathPtr;
        pathPtr = pathPtr.offset(1);
        *c2rust_fresh4 = '/' as ::core::ffi::c_char;
    }
    *path = result;
    *pathLen = len;
}
#[inline]
unsafe extern "C" fn uio_PathComp_alloc() -> *mut uio_PathComp {
    let mut result: *mut uio_PathComp = malloc(::core::mem::size_of::<uio_PathComp>())
        as *mut uio_PathComp;
    return result;
}
#[inline]
unsafe extern "C" fn uio_PathComp_free(mut pathComp: *mut uio_PathComp) {
    free(pathComp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_PathComp_new(
    mut name: *mut ::core::ffi::c_char,
    mut nameLen: size_t,
    mut upComp: *mut uio_PathComp,
) -> *mut uio_PathComp {
    let mut result: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    result = uio_PathComp_alloc();
    (*result).name = name;
    (*result).nameLen = nameLen;
    (*result).up = upComp as *mut uio_PathComp;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_PathComp_delete(mut pathComp: *mut uio_PathComp) {
    let mut next: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    while !pathComp.is_null() {
        next = (*pathComp).next as *mut uio_PathComp;
        free((*pathComp).name as *mut ::core::ffi::c_void);
        uio_PathComp_free(pathComp);
        pathComp = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_countPathComps(
    mut comp: *const uio_PathComp,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    count = 0 as ::core::ffi::c_int;
    while !comp.is_null() {
        count += 1;
        comp = (*comp).next;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn uio_lastPathComp(
    mut comp: *mut uio_PathComp,
) -> *mut uio_PathComp {
    if comp.is_null() {
        return ::core::ptr::null_mut::<uio_PathComp>();
    }
    while !(*comp).next.is_null() {
        comp = (*comp).next as *mut uio_PathComp;
    }
    return comp;
}
#[no_mangle]
pub unsafe extern "C" fn uio_makePathComps(
    mut path: *const ::core::ffi::c_char,
    mut upComp: *mut uio_PathComp,
) -> *mut uio_PathComp {
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut compPtr: *mut *mut uio_PathComp = ::core::ptr::null_mut::<
        *mut uio_PathComp,
    >();
    compPtr = &raw mut result;
    getFirstPath0Component(path, &raw mut start, &raw mut end);
    while *start as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        str = malloc((end.offset_from(start) + 1isize) as size_t)
            as *mut ::core::ffi::c_char;
        memcpy(
            str as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            end.offset_from(start) as size_t,
        );
        *str.offset(end.offset_from(start) as isize) = '\0' as ::core::ffi::c_char;
        *compPtr = uio_PathComp_new(str, end.offset_from(start) as size_t, upComp);
        upComp = *compPtr;
        compPtr = &raw mut (**compPtr).next as *mut *mut uio_PathComp;
        getNextPath0Component(&raw mut start, &raw mut end);
    }
    *compPtr = ::core::ptr::null_mut::<uio_PathComp>();
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_printPathComp(
    mut outStream: *mut FILE,
    mut comp: *const uio_PathComp,
) {
    fprintf(outStream, b"%s\0".as_ptr() as *const ::core::ffi::c_char, (*comp).name);
}
#[no_mangle]
pub unsafe extern "C" fn uio_printPathToComp(
    mut outStream: *mut FILE,
    mut comp: *const uio_PathComp,
) {
    if comp.is_null() {
        return;
    }
    uio_printPathToComp(outStream, (*comp).up);
    fprintf(outStream, b"/\0".as_ptr() as *const ::core::ffi::c_char);
    uio_printPathComp(outStream, comp);
}
#[inline]
unsafe extern "C" fn uio_memdup0(
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut dst: *mut ::core::ffi::c_char = malloc(len.wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    memcpy(dst as *mut ::core::ffi::c_void, src as *const ::core::ffi::c_void, len);
    *dst.offset(len as isize) = '\0' as ::core::ffi::c_char;
    return dst;
}
