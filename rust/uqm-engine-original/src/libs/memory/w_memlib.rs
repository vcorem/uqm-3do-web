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

// Rust translation of src/libs/memory/w_memlib.c.
extern "C" {
    pub type _IO_FILE;
    static stderr: *mut FILE;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type FILE = _IO_FILE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
#[no_mangle]
pub unsafe extern "C" fn mem_init() -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn mem_uninit() -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn HMalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = malloc(size);
    if p.is_null() && size > 0 as size_t {
        log_add(
            log_Level::log_Fatal,
            b"HMalloc() FATAL: out of memory.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        explode();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn HFree(mut p: *mut ::core::ffi::c_void) {
    free(p);
}
#[no_mangle]
pub unsafe extern "C" fn HCalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    p = HMalloc(size);
    memset(p, 0 as ::core::ffi::c_int, size);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn HRealloc(
    mut p: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    p = realloc(p, size);
    if p.is_null() && size > 0 as size_t {
        log_add(
            log_Level::log_Fatal,
            b"HRealloc() FATAL: out of memory.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        explode();
    }
    return p;
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
