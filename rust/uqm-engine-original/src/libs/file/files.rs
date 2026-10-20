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

// Contains code handling files

// Rust translation of src/libs/file/files.c.
extern "C" {
    pub type uio_Handle;
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn access(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_open(
        dir: *mut uio_DirHandle,
        file: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mode: mode_t,
    ) -> *mut uio_Handle;
    fn uio_close(handle: *mut uio_Handle) -> ::core::ffi::c_int;
    fn uio_fstat(handle: *mut uio_Handle, statBuf: *mut stat) -> ::core::ffi::c_int;
    fn uio_read(
        handle: *mut uio_Handle,
        buf: *mut ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_write(
        handle: *mut uio_Handle,
        buf: *const ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
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
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
pub type time_t = ::core::ffi::c_longlong;
pub type ssize_t = isize;
pub type mode_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
pub type off_t = ::core::ffi::c_longlong;
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
pub type uint8 = ::core::ffi::c_uchar;
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
pub unsafe extern "C" fn fileExists(mut name: *const ::core::ffi::c_char) -> bool {
    return access(name, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fileExists2(
    mut dir: *mut uio_DirHandle,
    mut fileName: *const ::core::ffi::c_char,
) -> bool {
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    stream = uio_fopen(dir, fileName, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if stream.is_null() {
        return false;
    }
    uio_fclose(stream);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn copyFile(
    mut srcDir: *mut uio_DirHandle,
    mut srcName: *const ::core::ffi::c_char,
    mut dstDir: *mut uio_DirHandle,
    mut newName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut src: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let mut dst: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
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
    let mut buf: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut bufPtr: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut numInBuf: ssize_t = 0;
    let mut numWritten: ssize_t = 0;
    src = uio_open(srcDir, srcName, 0 as ::core::ffi::c_int, 0 as mode_t);
    if src.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if uio_fstat(src, &raw mut sb) == -1 as ::core::ffi::c_int {
        return copyError(
            src,
            ::core::ptr::null_mut::<uio_Handle>(),
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<uint8>(),
        );
    }
    dst = uio_open(
        dstDir,
        newName,
        0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int,
        sb.st_mode
            & (0o700 as ::core::ffi::c_int | 0o70 as ::core::ffi::c_int
                | 0o7 as ::core::ffi::c_int) as mode_t,
    );
    if dst.is_null() {
        return copyError(
            src,
            ::core::ptr::null_mut::<uio_Handle>(),
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<uint8>(),
        );
    }
    buf = HMalloc(65536 as size_t) as *mut uint8;
    loop {
        numInBuf = uio_read(src, buf as *mut ::core::ffi::c_void, 65536 as size_t);
        if numInBuf == -1 as ssize_t {
            if *__errno_location() == 27 as ::core::ffi::c_int {
                continue;
            }
            return copyError(src, dst, dstDir, newName, buf);
        } else {
            if numInBuf == 0 as ssize_t {
                break;
            }
            bufPtr = buf;
            loop {
                numWritten = uio_write(
                    dst,
                    bufPtr as *const ::core::ffi::c_void,
                    numInBuf as size_t,
                );
                if numWritten == -1 as ssize_t {
                    if *__errno_location() != 27 as ::core::ffi::c_int {
                        return copyError(src, dst, dstDir, newName, buf);
                    }
                } else {
                    numInBuf -= numWritten;
                    bufPtr = bufPtr.offset(numWritten as isize);
                }
                if numInBuf <= 0 as ssize_t {
                    break;
                }
            }
        }
    }
    HFree(buf as *mut ::core::ffi::c_void);
    uio_close(src);
    uio_close(dst);
    *__errno_location() = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn copyError(
    mut srcHandle: *mut uio_Handle,
    mut dstHandle: *mut uio_Handle,
    mut unlinkHandle: *mut uio_DirHandle,
    mut unlinkPath: *const ::core::ffi::c_char,
    mut buf: *mut uint8,
) -> ::core::ffi::c_int {
    let mut savedErrno: ::core::ffi::c_int = 0;
    savedErrno = *__errno_location();
    log_add(
        log_Level::log_Debug,
        b"Error while copying: %s\0".as_ptr() as *const ::core::ffi::c_char,
        strerror(*__errno_location()),
    );
    if !srcHandle.is_null() {
        uio_close(srcHandle);
    }
    if !dstHandle.is_null() {
        uio_close(dstHandle);
    }
    if !unlinkPath.is_null() {
        uio_unlink(unlinkHandle, unlinkPath);
    }
    if !buf.is_null() {
        HFree(buf as *mut ::core::ffi::c_void);
    }
    *__errno_location() = savedErrno;
    return -1 as ::core::ffi::c_int;
}
