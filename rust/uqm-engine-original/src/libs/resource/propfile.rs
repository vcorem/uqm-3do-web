/*  propfile.c, Copyright (c) 2008 Michael C. Martin */

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

// Rust translation of src/libs/resource/propfile.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn ReadResFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type PROPERTY_HANDLER = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *const ::core::ffi::c_char) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[no_mangle]
pub unsafe extern "C" fn PropFile_from_string(
    mut d: *mut ::core::ffi::c_char,
    mut handler: PROPERTY_HANDLER,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut len: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    len = strlen(d) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        let mut key_start: ::core::ffi::c_int = 0;
        let mut key_end: ::core::ffi::c_int = 0;
        let mut value_start: ::core::ffi::c_int = 0;
        let mut value_end: ::core::ffi::c_int = 0;
        while i < len && isspace(*d.offset(i as isize) as ::core::ffi::c_int) != 0 {
            i += 1;
        }
        if i >= len {
            break;
        }
        if *d.offset(i as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
            while i < len
                && *d.offset(i as isize) as ::core::ffi::c_int
                    != '\n' as ::core::ffi::c_int
            {
                i += 1;
            }
            if i >= len {
                break;
            }
        } else {
            key_start = i;
            while i < len
                && *d.offset(i as isize) as ::core::ffi::c_int
                    != '=' as ::core::ffi::c_int
                && *d.offset(i as isize) as ::core::ffi::c_int
                    != '\n' as ::core::ffi::c_int
                && *d.offset(i as isize) as ::core::ffi::c_int
                    != '#' as ::core::ffi::c_int
            {
                i += 1;
            }
            if i >= len {
                log_add(
                    log_Level::log_Warning,
                    b"Warning: Bare keyword at EOF\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else if *d.offset(i as isize) as ::core::ffi::c_int
                != '=' as ::core::ffi::c_int
            {
                log_add(
                    log_Level::log_Warning,
                    b"Warning: Key without value\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                while i < len
                    && *d.offset(i as isize) as ::core::ffi::c_int
                        != '\n' as ::core::ffi::c_int
                {
                    i += 1;
                }
                if i >= len {
                    break;
                }
            } else {
                key_end = i;
                while key_end > key_start
                    && isspace(
                        *d.offset((key_end - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                    ) != 0
                {
                    key_end -= 1;
                }
                i += 1;
                while i < len
                    && *d.offset(i as isize) as ::core::ffi::c_int
                        != '#' as ::core::ffi::c_int
                    && *d.offset(i as isize) as ::core::ffi::c_int
                        != '\n' as ::core::ffi::c_int
                    && isspace(*d.offset(i as isize) as ::core::ffi::c_int) != 0
                {
                    i += 1;
                }
                value_start = i;
                while i < len
                    && *d.offset(i as isize) as ::core::ffi::c_int
                        != '#' as ::core::ffi::c_int
                    && *d.offset(i as isize) as ::core::ffi::c_int
                        != '\n' as ::core::ffi::c_int
                {
                    i += 1;
                }
                value_end = i;
                while value_end > value_start
                    && isspace(
                        *d.offset((value_end - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                    ) != 0
                {
                    value_end -= 1;
                }
                while i < len
                    && *d.offset(i as isize) as ::core::ffi::c_int
                        != '\n' as ::core::ffi::c_int
                {
                    i += 1;
                }
                i += 1;
                *d.offset(key_end as isize) = '\0' as ::core::ffi::c_char;
                *d.offset(value_end as isize) = '\0' as ::core::ffi::c_char;
                if !prefix.is_null() {
                    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
                    snprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        255 as size_t,
                        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                        prefix,
                        d.offset(key_start as isize),
                    );
                    buf[255usize] = 0 as ::core::ffi::c_char;
                    handler
                        .expect(
                            "non-null function pointer",
                        )(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        d.offset(value_start as isize),
                    );
                } else {
                    handler
                        .expect(
                            "non-null function pointer",
                        )(d.offset(key_start as isize), d.offset(value_start as isize));
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PropFile_from_file(
    mut f: *mut uio_Stream,
    mut handler: PROPERTY_HANDLER,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut flen: size_t = 0;
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    flen = LengthResFile(f);
    data = malloc(flen.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if data.is_null() {
        return;
    }
    flen = ReadResFile(data as *mut ::core::ffi::c_void, 1 as size_t, flen, f);
    *data.offset(flen as isize) = '\0' as ::core::ffi::c_char;
    PropFile_from_string(data, handler, prefix);
    free(data as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn PropFile_from_filename(
    mut path: *mut uio_DirHandle,
    mut fname: *const ::core::ffi::c_char,
    mut handler: PROPERTY_HANDLER,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut f: *mut uio_Stream = res_OpenResFile(
        path,
        fname,
        b"rt\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if f.is_null() {
        return;
    }
    PropFile_from_file(f, handler, prefix);
    res_CloseResFile(f);
}
