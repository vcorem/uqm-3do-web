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

// Rust translation of src/libs/resource/loadres.c.
extern "C" {
    pub type uio_Stream;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn ReadResFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn FreeResourceData(_: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
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
pub unsafe extern "C" fn GetResourceData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut compLen: DWORD = 0;
    if uqm_lua_checked!(ReadResFile(
        &raw mut compLen as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<DWORD>() as size_t,
        1 as size_t,
        fp,
    ), ::core::ptr::null_mut()) != 1 as size_t
    {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if compLen != !(0 as ::core::ffi::c_int as DWORD) {
        log_add(
            log_Level::log_Warning,
            b"LZ-compressed binary data not supported\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    length = (length as ::core::ffi::c_ulong)
        .wrapping_sub(::core::mem::size_of::<DWORD>() as ::core::ffi::c_ulong) as DWORD;
    result = HMalloc(length as size_t);
    if result.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if uqm_lua_checked!(ReadResFile(result, 1 as size_t, length as size_t, fp), ::core::ptr::null_mut()) != length as size_t {
        FreeResourceData(result);
        result = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return result;
}
