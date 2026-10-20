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

// Rust translation of src/libs/resource/filecntl.c.
extern "C" {
    pub type uio_Handle;
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn uio_fstat(handle: *mut uio_Handle, statBuf: *mut stat) -> ::core::ffi::c_int;
    fn uio_stat(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        statBuf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn uio_unlink(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fread(
        buf: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_fgetc(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fputc(c: ::core::ffi::c_int, stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fseek(
        stream: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn uio_fwrite(
        buf: *const ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_streamHandle(stream: *mut uio_Stream) -> *mut uio_Handle;
}
pub type size_t = usize;
pub type off_t = ::core::ffi::c_longlong;
pub type time_t = ::core::ffi::c_longlong;
pub type mode_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
pub type ino_t = ::core::ffi::c_ulonglong;
pub type dev_t = ::core::ffi::c_uint;
pub type blksize_t = ::core::ffi::c_int;
pub type blkcnt_t = ::core::ffi::c_int;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub st_ino: ino_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[no_mangle]
pub unsafe extern "C" fn res_OpenResFile(
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> *mut uio_Stream {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut sb: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ino: 0,
    };
    if uqm_lua_checked!(uio_stat(dir, filename, &raw mut sb), ::core::ptr::null_mut()) == 0 as ::core::ffi::c_int
        && sb.st_mode & 0o170000 as mode_t == 0o40000 as mode_t
    {
        return ::core::ptr::with_exposed_provenance_mut::<
            ::core::ffi::c_void,
        >(!(0 as ::core::ffi::c_int) as usize) as *mut uio_Stream;
    }
    fp = uqm_lua_checked!(uio_fopen(dir, filename, mode), ::core::ptr::null_mut());
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn res_CloseResFile(mut fp: *mut uio_Stream) -> BOOLEAN {
    if !fp.is_null() {
        if fp
            != ::core::ptr::with_exposed_provenance_mut::<
                ::core::ffi::c_void,
            >(!(0 as ::core::ffi::c_int) as usize) as *mut uio_Stream
        {
            uqm_lua_checked!(uio_fclose(fp), BOOLEAN(0));
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn DeleteResFile(
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> BOOLEAN {
    return BOOLEAN(
        (uqm_lua_checked!(uio_unlink(dir, filename), BOOLEAN(0)) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ReadResFile(
    mut lpBuf: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut uio_Stream,
) -> size_t {
    let mut retval: ::core::ffi::c_int = 0;
    retval = uqm_lua_checked!(uio_fread(lpBuf, size, count, fp), 0) as ::core::ffi::c_int;
    return retval as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn WriteResFile(
    mut lpBuf: *const ::core::ffi::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut uio_Stream,
) -> size_t {
    let mut retval: ::core::ffi::c_int = 0;
    retval = uqm_lua_checked!(uio_fwrite(lpBuf, size, count, fp), 0) as ::core::ffi::c_int;
    return retval as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn GetResFileChar(mut fp: *mut uio_Stream) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    retval = uqm_lua_checked!(uio_fgetc(fp), 0);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn PutResFileChar(
    mut ch: ::core::ffi::c_char,
    mut fp: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    retval = uqm_lua_checked!(uio_fputc(ch as ::core::ffi::c_int, fp), 0);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn PutResFileNewline(
    mut fp: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    retval = uqm_lua_checked!(PutResFileChar('\n' as ::core::ffi::c_char, fp), 0);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn SeekResFile(
    mut fp: *mut uio_Stream,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    let mut retval: ::core::ffi::c_long = 0;
    retval = uqm_lua_checked!(uio_fseek(fp, offset, whence), 0) as ::core::ffi::c_long;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn TellResFile(mut fp: *mut uio_Stream) -> ::core::ffi::c_long {
    let mut retval: ::core::ffi::c_long = 0;
    retval = uqm_lua_checked!(uio_ftell(fp), 0);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn LengthResFile(mut fp: *mut uio_Stream) -> size_t {
    let mut sb: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ino: 0,
    };
    if fp
        == ::core::ptr::with_exposed_provenance_mut::<
            ::core::ffi::c_void,
        >(!(0 as ::core::ffi::c_int) as usize) as *mut uio_Stream
    {
        return 1 as size_t;
    }
    if uqm_lua_checked!(uio_fstat(uio_streamHandle(fp), &raw mut sb), 0) == -1 as ::core::ffi::c_int {
        return 1 as size_t;
    }
    return sb.st_size as size_t;
}
