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

// Rust translation of src/libs/uio/uiostream.c.
extern "C" {
    pub type _IO_FILE;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn uio_open(
        dir: *mut uio_DirHandle,
        file: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mode: mode_t,
    ) -> *mut uio_Handle;
    fn uio_close(handle: *mut uio_Handle) -> ::core::ffi::c_int;
    fn uio_read(
        handle: *mut uio_Handle,
        buf: *mut ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_lseek(
        handle: *mut uio_Handle,
        offset: off_t,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_write(
        handle: *mut uio_Handle,
        buf: *const ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
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
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn emscripten_asm_const_int_sync_on_main_thread(
        code: *const ::core::ffi::c_char,
        arg_sigs: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn uio_vasprintf(
        format: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
}
pub type __builtin_va_list = *mut ::core::ffi::c_void;
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type time_t = ::core::ffi::c_longlong;
pub type mode_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
pub type ino_t = ::core::ffi::c_ulonglong;
pub type dev_t = ::core::ffi::c_uint;
pub type blksize_t = ::core::ffi::c_int;
pub type blkcnt_t = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PDirEntryHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PRoot {
    pub mountRef: ::core::ffi::c_int,
    pub handleRef: ::core::ffi::c_int,
    pub rootDir: *mut uio_PDirHandle,
    pub handler: *mut uio_FileSystemHandler,
    pub flags: ::core::ffi::c_int,
    pub handle: *mut uio_Handle,
    pub extra: uio_PRootExtra,
}
pub type uio_PRootExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_Handle {
    pub r#ref: ::core::ffi::c_int,
    pub root: *mut uio_PRoot,
    pub native: uio_NativeHandle,
    pub openFlags: ::core::ffi::c_int,
}
pub type uio_NativeHandle = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_FileSystemHandler {
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub unInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut uio_PRoot, ::core::ffi::c_int) -> ()>,
    pub mount: Option<
        unsafe extern "C" fn(*mut uio_Handle, ::core::ffi::c_int) -> *mut uio_PRoot,
    >,
    pub umount: Option<unsafe extern "C" fn(*mut uio_PRoot) -> ::core::ffi::c_int>,
    pub access: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(*mut uio_Handle) -> ()>,
    pub fstat: Option<
        unsafe extern "C" fn(*mut uio_Handle, *mut stat) -> ::core::ffi::c_int,
    >,
    pub stat: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            *mut stat,
        ) -> ::core::ffi::c_int,
    >,
    pub mkdir: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            mode_t,
        ) -> *mut uio_PDirHandle,
    >,
    pub open: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            mode_t,
        ) -> *mut uio_Handle,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            *mut uio_Handle,
            *mut ::core::ffi::c_void,
            size_t,
        ) -> ssize_t,
    >,
    pub rename: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub rmdir: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub seek: Option<
        unsafe extern "C" fn(*mut uio_Handle, off_t, ::core::ffi::c_int) -> off_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            *mut uio_Handle,
            *const ::core::ffi::c_void,
            size_t,
        ) -> ssize_t,
    >,
    pub unlink: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub openEntries: Option<
        unsafe extern "C" fn(*mut uio_PDirHandle) -> uio_NativeEntriesContext,
    >,
    pub readEntries: Option<
        unsafe extern "C" fn(
            *mut uio_NativeEntriesContext,
            *mut ::core::ffi::c_char,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub closeEntries: Option<unsafe extern "C" fn(uio_NativeEntriesContext) -> ()>,
    pub getPDirEntryHandle: Option<
        unsafe extern "C" fn(
            *const uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> *mut uio_PDirEntryHandle,
    >,
    pub deletePRootExtra: Option<unsafe extern "C" fn(uio_PRootExtra) -> ()>,
    pub deletePDirHandleExtra: Option<unsafe extern "C" fn(uio_PDirHandleExtra) -> ()>,
    pub deletePFileHandleExtra: Option<unsafe extern "C" fn(uio_PFileHandleExtra) -> ()>,
}
pub type uio_PFileHandleExtra = *mut ::core::ffi::c_void;
pub type uio_PDirHandleExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PDirHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
    pub extra: uio_PDirHandleExtra,
}
pub type uio_NativeEntriesContext = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_DirHandle {
    pub r#ref: ::core::ffi::c_int,
    pub repository: *mut uio_Repository,
    pub path: *mut ::core::ffi::c_char,
    pub rootEnd: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_Repository {
    pub r#ref: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub numMounts: ::core::ffi::c_int,
    pub mounts: *mut *mut uio_MountInfo,
    pub mountTree: *mut uio_MountTree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_MountTree {
    pub subTrees: *mut uio_MountTree,
    pub pLocs: *mut uio_MountTreeItem,
    pub upTree: *mut uio_MountTree,
    pub comps: *mut uio_PathComp,
    pub lastComp: *mut uio_PathComp,
    pub next: *mut uio_MountTree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PathComp {
    pub name: *mut ::core::ffi::c_char,
    pub nameLen: size_t,
    pub next: *mut uio_PathComp,
    pub up: *mut uio_PathComp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_MountTreeItem {
    pub mountInfo: *mut uio_MountInfo,
    pub depth: ::core::ffi::c_int,
    pub next: *mut uio_MountTreeItem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_MountInfo {
    pub flags: ::core::ffi::c_int,
    pub fsID: uio_FileSystemID,
    pub dirName: *mut ::core::ffi::c_char,
    pub pDirHandle: *mut uio_PDirHandle,
    pub mountTree: *mut uio_MountTree,
    pub autoMount: *mut *mut uio_AutoMount,
    pub mountHandle: *mut uio_MountHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_MountHandle {
    pub repository: *mut uio_Repository,
    pub mountInfo: *mut uio_MountInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_AutoMount {
    pub pattern: *const ::core::ffi::c_char,
    pub matchType: match_MatchType,
    pub fileSystemID: uio_FileSystemID,
    pub mountFlags: ::core::ffi::c_int,
}
pub type uio_FileSystemID = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct match_MatchType(pub ::core::ffi::c_uint);
impl match_MatchType {
    pub const match_MATCH_LITERAL: Self = Self(0);
    pub const match_MATCH_PREFIX: Self = Self(1);
    pub const match_MATCH_SUFFIX: Self = Self(2);
    pub const match_MATCH_SUBSTRING: Self = Self(3);
    pub const match_MATCH_REGEX: Self = Self(4);
}
pub type va_list = __builtin_va_list;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_Stream {
    pub buf: *mut ::core::ffi::c_char,
    pub dataStart: *mut ::core::ffi::c_char,
    pub dataEnd: *mut ::core::ffi::c_char,
    pub bufEnd: *mut ::core::ffi::c_char,
    pub handle: *mut uio_Handle,
    pub status: ::core::ffi::c_int,
    pub operation: uio_StreamOperation,
    pub openFlags: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uio_StreamOperation(pub ::core::ffi::c_uint);
impl uio_StreamOperation {
    pub const uio_StreamOperation_none: Self = Self(0);
    pub const uio_StreamOperation_read: Self = Self(1);
    pub const uio_StreamOperation_write: Self = Self(2);
}
#[no_mangle]
pub unsafe extern "C" fn uio_fopen(
    mut dir: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> *mut uio_Stream {
    let mut openFlags: ::core::ffi::c_int = 0;
    let mut handle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut i: ::core::ffi::c_int = 0;
    's_42: {
        match *mode as ::core::ffi::c_int {
            114 => {
                openFlags = 0 as ::core::ffi::c_int;
                break 's_42;
            }
            119 => {
                openFlags = 0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
                    | 0o1000 as ::core::ffi::c_int;
                break 's_42;
            }
            97 => {
                openFlags = 0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
                    | 0o2000 as ::core::ffi::c_int;
            }
            _ => {}
        }
        *__errno_location() = 28 as ::core::ffi::c_int;
        fprintf(
            stderr,
            b"Invalid mode string in call to uio_fopen().\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<uio_Stream>();
    }
    mode = mode.offset(1);
    i = 2 as ::core::ffi::c_int;
    loop {
        let c2rust_fresh0 = i;
        i -= 1;
        if !(c2rust_fresh0 != 0
            && *mode as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
        {
            break;
        }
        match *mode as ::core::ffi::c_int {
            98 | 116 => {}
            43 => {
                openFlags = openFlags
                    & !(0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
                    | 0o2 as ::core::ffi::c_int;
            }
            _ => {
                i = 0 as ::core::ffi::c_int;
            }
        }
        mode = mode.offset(1);
    }
    handle = uio_open(
        dir,
        path,
        openFlags,
        (0o400 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int
            | 0o40 as ::core::ffi::c_int | 0o20 as ::core::ffi::c_int
            | 0o4 as ::core::ffi::c_int | 0o2 as ::core::ffi::c_int) as mode_t,
    );
    if handle.is_null() {
        return ::core::ptr::null_mut::<uio_Stream>();
    }
    stream = uio_Stream_new(handle, openFlags);
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fclose(mut stream: *mut uio_Stream) -> ::core::ffi::c_int {
    if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_write.0 {
        uio_Stream_flushWriteBuffer(stream);
        let mut c2rust_lvalue: [::core::ffi::c_char; 1] = ['\0' as ::core::ffi::c_char];
        emscripten_asm_const_int_sync_on_main_thread(
            ({
                extern "C" { static uqm_em_asm_0: [::core::ffi::c_char; 14]; }
            &raw const uqm_em_asm_0 as *const ::core::ffi::c_char
            }),
            &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
        );
    }
    uio_close((*stream).handle);
    uio_Stream_delete(stream);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fread(
    mut buf: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut uio_Stream,
) -> size_t {
    let mut bytesToRead: size_t = 0;
    let mut bytesRead: size_t = 0;
    bytesToRead = size.wrapping_mul(nmemb);
    bytesRead = 0 as size_t;
    uio_assertReadSanity(stream);
    (*stream).operation = uio_StreamOperation::uio_StreamOperation_read;
    if (*stream).dataEnd > (*stream).dataStart {
        let mut numRead: size_t = 0;
        numRead = minu(
            (*stream).dataEnd.offset_from((*stream).dataStart) as ::core::ffi::c_uint,
            bytesToRead as ::core::ffi::c_uint,
        ) as size_t;
        memcpy(buf, (*stream).dataStart as *const ::core::ffi::c_void, numRead);
        buf = (buf as *mut ::core::ffi::c_char).offset(numRead as isize)
            as *mut ::core::ffi::c_void;
        (*stream).dataStart = (*stream).dataStart.offset(numRead as isize);
        bytesToRead = bytesToRead.wrapping_sub(numRead);
        bytesRead = bytesRead.wrapping_add(numRead);
    }
    if bytesToRead == 0 as size_t {
        return nmemb;
    }
    let mut numRead_0: ssize_t = 0;
    numRead_0 = uio_read((*stream).handle, buf, bytesToRead);
    if numRead_0 == -1 as ssize_t {
        (*stream).status = 2 as ::core::ffi::c_int;
    } else {
        bytesRead = bytesRead.wrapping_add(numRead_0 as size_t);
        if (numRead_0 as size_t) < bytesToRead {
            (*stream).status = 1 as ::core::ffi::c_int;
            (*stream).operation = uio_StreamOperation::uio_StreamOperation_none;
        }
    }
    if bytesToRead == 0 as size_t {
        return nmemb;
    }
    return bytesRead.wrapping_div(size);
}
#[no_mangle]
pub unsafe extern "C" fn uio_fgets(
    mut s: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut stream: *mut uio_Stream,
) -> *mut ::core::ffi::c_char {
    let mut orgSize: ::core::ffi::c_int = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    uio_assertReadSanity(stream);
    (*stream).operation = uio_StreamOperation::uio_StreamOperation_read;
    size -= 1;
    orgSize = size;
    buf = s;
    while size > 0 as ::core::ffi::c_int {
        let mut maxRead: size_t = 0;
        let mut newLinePos: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if (*stream).dataStart == (*stream).dataEnd {
            if uio_Stream_fillReadBuffer(stream) == -1 as ::core::ffi::c_int {
                (*stream).status = 2 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            if (*stream).dataStart == (*stream).dataEnd {
                (*stream).status = 1 as ::core::ffi::c_int;
                (*stream).operation = uio_StreamOperation::uio_StreamOperation_none;
                if size == orgSize {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                break;
            }
        }
        maxRead = minu(
            (*stream).dataEnd.offset_from((*stream).dataStart) as ::core::ffi::c_uint,
            size as ::core::ffi::c_uint,
        ) as size_t;
        newLinePos = memchr(
            (*stream).dataStart as *const ::core::ffi::c_void,
            '\n' as ::core::ffi::c_int,
            maxRead,
        ) as *const ::core::ffi::c_char;
        if !newLinePos.is_null() {
            maxRead = newLinePos
                .offset(1 as ::core::ffi::c_int as isize)
                .offset_from((*stream).dataStart) as size_t;
            memcpy(
                buf as *mut ::core::ffi::c_void,
                (*stream).dataStart as *const ::core::ffi::c_void,
                maxRead,
            );
            (*stream).dataStart = (*stream).dataStart.offset(maxRead as isize);
            *buf.offset(maxRead as isize) = '\0' as ::core::ffi::c_char;
            return buf;
        }
        memcpy(
            buf as *mut ::core::ffi::c_void,
            (*stream).dataStart as *const ::core::ffi::c_void,
            maxRead,
        );
        (*stream).dataStart = (*stream).dataStart.offset(maxRead as isize);
        buf = buf.offset(maxRead as isize);
        size = (size as size_t).wrapping_sub(maxRead) as ::core::ffi::c_int;
    }
    *buf = '\0' as ::core::ffi::c_char;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fgetc(mut stream: *mut uio_Stream) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    uio_assertReadSanity(stream);
    (*stream).operation = uio_StreamOperation::uio_StreamOperation_read;
    if (*stream).dataStart == (*stream).dataEnd {
        if uio_Stream_fillReadBuffer(stream) == -1 as ::core::ffi::c_int {
            (*stream).status = 2 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        if (*stream).dataStart == (*stream).dataEnd {
            (*stream).status = 1 as ::core::ffi::c_int;
            (*stream).operation = uio_StreamOperation::uio_StreamOperation_none;
            return -1 as ::core::ffi::c_int;
        }
    }
    result = *((*stream).dataStart as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int;
    (*stream).dataStart = (*stream).dataStart.offset(1);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_ungetc(
    mut c: ::core::ffi::c_int,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    ((*stream).openFlags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        != 0o1 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"(stream->openFlags & O_ACCMODE) != O_WRONLY\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int,
                b"uio_ungetc\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (c >= 0 as ::core::ffi::c_int && c <= 255 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"c >= 0 && c <= 255\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                272 as ::core::ffi::c_int,
                b"uio_ungetc\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_vfprintf(
    mut stream: *mut uio_Stream,
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut putResult: ::core::ffi::c_int = 0;
    let mut savedErrno: ::core::ffi::c_int = 0;
    buf = uio_vasprintf(format, args.as_va_list());
    if buf.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    putResult = uio_fputs(buf, stream);
    savedErrno = *__errno_location();
    free(buf as *mut ::core::ffi::c_void);
    *__errno_location() = savedErrno;
    return putResult;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fprintf(
    mut stream: *mut uio_Stream,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    let mut args: ::core::ffi::VaListImpl;
    let mut result: ::core::ffi::c_int = 0;
    args = c2rust_args.clone();
    result = uio_vfprintf(stream, format, args.as_va_list());
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fputc(
    mut c: ::core::ffi::c_int,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    ((*stream).openFlags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"(stream->openFlags & O_ACCMODE) != O_RDONLY\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                321 as ::core::ffi::c_int,
                b"uio_fputc\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (c >= 0 as ::core::ffi::c_int && c <= 255 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"c >= 0 && c <= 255\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                322 as ::core::ffi::c_int,
                b"uio_fputc\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uio_assertWriteSanity(stream);
    (*stream).operation = uio_StreamOperation::uio_StreamOperation_write;
    if (*stream).dataEnd == (*stream).bufEnd {
        if uio_Stream_flushWriteBuffer(stream) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
    }
    *((*stream).dataEnd as *mut ::core::ffi::c_uchar) = c as ::core::ffi::c_uchar;
    (*stream).dataEnd = (*stream).dataEnd.offset(1);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fputs(
    mut s: *const ::core::ffi::c_char,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = uio_fwrite(s as *const ::core::ffi::c_void, strlen(s), 1 as size_t, stream)
        as ::core::ffi::c_int;
    if result != 1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fseek(
    mut stream: *mut uio_Stream,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut newPos: ::core::ffi::c_int = 0;
    if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_read.0 {
        uio_Stream_discardReadBuffer(stream);
    } else if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_write.0 {
        if uio_Stream_flushWriteBuffer(stream) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
    }
    ((*stream).dataStart == (*stream).buf
        || {
            __assert_fail(
                b"stream->dataStart == stream->buf\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                363 as ::core::ffi::c_int,
                b"uio_fseek\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    ((*stream).dataEnd == (*stream).buf
        || {
            __assert_fail(
                b"stream->dataEnd == stream->buf\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                364 as ::core::ffi::c_int,
                b"uio_fseek\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*stream).operation = uio_StreamOperation::uio_StreamOperation_none;
    newPos = uio_lseek((*stream).handle, offset as off_t, whence);
    if newPos == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    (*stream).status = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_ftell(mut stream: *mut uio_Stream) -> ::core::ffi::c_long {
    let mut newPos: off_t = 0;
    newPos = uio_lseek((*stream).handle, 0 as off_t, 1 as ::core::ffi::c_int) as off_t;
    if newPos == -1 as ::core::ffi::c_int as off_t {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_long;
    }
    if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_write.0 {
        newPos += (*stream).dataEnd.offset_from((*stream).dataStart) as off_t;
    } else if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_read.0 {
        newPos -= (*stream).dataEnd.offset_from((*stream).dataStart) as off_t;
    }
    return newPos as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fwrite(
    mut buf: *const ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut uio_Stream,
) -> size_t {
    let mut bytesToWrite: ssize_t = 0;
    let mut bytesWritten: ssize_t = 0;
    uio_assertWriteSanity(stream);
    (*stream).operation = uio_StreamOperation::uio_StreamOperation_write;
    bytesToWrite = size.wrapping_mul(nmemb) as ssize_t;
    if bytesToWrite < (*stream).bufEnd.offset_from((*stream).dataEnd) {
        memcpy(
            (*stream).dataEnd as *mut ::core::ffi::c_void,
            buf,
            bytesToWrite as size_t,
        );
        (*stream).dataEnd = (*stream).dataEnd.offset(bytesToWrite as isize);
        return nmemb;
    }
    if uio_Stream_flushWriteBuffer(stream) == -1 as ::core::ffi::c_int {
        return 0 as size_t;
    }
    if bytesToWrite < (*stream).bufEnd.offset_from((*stream).dataEnd) {
        memcpy(
            (*stream).dataEnd as *mut ::core::ffi::c_void,
            buf,
            bytesToWrite as size_t,
        );
        (*stream).dataEnd = (*stream).dataEnd.offset(bytesToWrite as isize);
        return nmemb;
    }
    bytesWritten = uio_write((*stream).handle, buf, bytesToWrite as size_t);
    if bytesWritten != bytesToWrite {
        (*stream).status = 2 as ::core::ffi::c_int;
        if bytesWritten == -1 as ssize_t {
            return 0 as size_t;
        }
    }
    if bytesWritten == bytesToWrite {
        return nmemb;
    }
    return (bytesWritten as size_t).wrapping_div(size);
}
#[no_mangle]
pub unsafe extern "C" fn uio_fflush(mut stream: *mut uio_Stream) -> ::core::ffi::c_int {
    (!stream.is_null()
        || {
            __assert_fail(
                b"stream != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                456 as ::core::ffi::c_int,
                b"uio_fflush\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_write.0 {
        if uio_Stream_flushWriteBuffer(stream) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        (*stream).operation = uio_StreamOperation::uio_StreamOperation_none;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_feof(mut stream: *mut uio_Stream) -> ::core::ffi::c_int {
    return ((*stream).status == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_ferror(mut stream: *mut uio_Stream) -> ::core::ffi::c_int {
    return ((*stream).status == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_clearerr(mut stream: *mut uio_Stream) {
    (*stream).status = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_streamHandle(
    mut stream: *mut uio_Stream,
) -> *mut uio_Handle {
    return (*stream).handle;
}
unsafe extern "C" fn uio_assertReadSanity(mut stream: *mut uio_Stream) {
    ((*stream).openFlags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        != 0o1 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"(stream->openFlags & O_ACCMODE) != O_WRONLY\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                493 as ::core::ffi::c_int,
                b"uio_assertReadSanity\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_write.0 {
        fprintf(
            stderr,
            b"Error: Reading on a file directly after writing, without an intervening call to fflush() or a file positioning function.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    }
}
unsafe extern "C" fn uio_assertWriteSanity(mut stream: *mut uio_Stream) {
    ((*stream).openFlags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"(stream->openFlags & O_ACCMODE) != O_RDONLY\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                513 as ::core::ffi::c_int,
                b"uio_assertWriteSanity\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_read.0 {
        fprintf(
            stderr,
            b"Error: Writing on a file directly after reading, without an intervening call to a file positioning function.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    }
    ((*stream).dataStart == (*stream).buf
        || {
            __assert_fail(
                b"stream->dataStart == stream->buf\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                527 as ::core::ffi::c_int,
                b"uio_assertWriteSanity\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_Stream_flushWriteBuffer(
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut bytesWritten: ssize_t = 0;
    ((*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_write.0
        || {
            __assert_fail(
                b"stream->operation == uio_StreamOperation_write\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                535 as ::core::ffi::c_int,
                b"uio_Stream_flushWriteBuffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    bytesWritten = uio_write(
        (*stream).handle,
        (*stream).dataStart as *const ::core::ffi::c_void,
        (*stream).dataEnd.offset_from((*stream).dataStart) as size_t,
    );
    if bytesWritten != (*stream).dataEnd.offset_from((*stream).dataStart) {
        (*stream).status = 2 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    ((*stream).dataStart == (*stream).buf
        || {
            __assert_fail(
                b"stream->dataStart == stream->buf\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                543 as ::core::ffi::c_int,
                b"uio_Stream_flushWriteBuffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*stream).dataEnd = (*stream).buf;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_Stream_discardReadBuffer(mut stream: *mut uio_Stream) {
    ((*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_read.0
        || {
            __assert_fail(
                b"stream->operation == uio_StreamOperation_read\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                551 as ::core::ffi::c_int,
                b"uio_Stream_discardReadBuffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*stream).dataStart = (*stream).buf;
    (*stream).dataEnd = (*stream).buf;
}
unsafe extern "C" fn uio_Stream_fillReadBuffer(
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut numRead: ssize_t = 0;
    ((*stream).operation.0 == uio_StreamOperation::uio_StreamOperation_read.0
        || {
            __assert_fail(
                b"stream->operation == uio_StreamOperation_read\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/uiostream.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                561 as ::core::ffi::c_int,
                b"uio_Stream_fillReadBuffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    numRead = uio_read(
        (*stream).handle,
        (*stream).buf as *mut ::core::ffi::c_void,
        1024 as size_t,
    );
    if numRead == -1 as ssize_t {
        return -1 as ::core::ffi::c_int;
    }
    (*stream).dataStart = (*stream).buf;
    (*stream).dataEnd = (*stream).buf.offset(numRead as isize);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn uio_Stream_new(
    mut handle: *mut uio_Handle,
    mut openFlags: ::core::ffi::c_int,
) -> *mut uio_Stream {
    let mut result: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    result = uio_Stream_alloc();
    (*result).handle = handle;
    (*result).openFlags = openFlags;
    (*result).status = 0 as ::core::ffi::c_int;
    (*result).operation = uio_StreamOperation::uio_StreamOperation_none;
    (*result).buf = malloc(1024 as size_t) as *mut ::core::ffi::c_char;
    (*result).dataStart = (*result).buf;
    (*result).dataEnd = (*result).buf;
    (*result).bufEnd = (*result).buf.offset(1024 as ::core::ffi::c_int as isize);
    return result;
}
#[inline]
unsafe extern "C" fn uio_Stream_alloc() -> *mut uio_Stream {
    let mut result: *mut uio_Stream = malloc(::core::mem::size_of::<uio_Stream>())
        as *mut uio_Stream;
    return result;
}
#[inline]
unsafe extern "C" fn uio_Stream_delete(mut stream: *mut uio_Stream) {
    free((*stream).buf as *mut ::core::ffi::c_void);
    uio_Stream_free(stream);
}
#[inline]
unsafe extern "C" fn uio_Stream_free(mut stream: *mut uio_Stream) {
    free(stream as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn minu(
    mut i1: ::core::ffi::c_uint,
    mut i2: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return if i1 <= i2 { i1 } else { i2 };
}
