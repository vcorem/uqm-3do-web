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

// Rust translation of src/libs/uio/utils.c.
extern "C" {
    pub type _IO_FILE;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn time(_: *mut time_t) -> time_t;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn uio_DirHandle_delete(dirHandle: *mut uio_DirHandle);
    fn uio_getMountFileSystemType(mountHandle: *mut uio_MountHandle) -> uio_FileSystemID;
    fn uio_open(
        dir: *mut uio_DirHandle,
        file: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mode: mode_t,
    ) -> *mut uio_Handle;
    fn uio_close(handle: *mut uio_Handle) -> ::core::ffi::c_int;
    fn uio_fstat(handle: *mut uio_Handle, statBuf: *mut stat) -> ::core::ffi::c_int;
    fn uio_mkdir(
        dir: *mut uio_DirHandle,
        name: *const ::core::ffi::c_char,
        mode: mode_t,
    ) -> ::core::ffi::c_int;
    fn uio_read(
        handle: *mut uio_Handle,
        buf: *mut ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_rmdir(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_write(
        handle: *mut uio_Handle,
        buf: *const ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_unlink(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_getFileLocation(
        dir: *mut uio_DirHandle,
        inPath: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mountHandle: *mut *mut uio_MountHandle,
        outPath: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_openDirRelative(
        base: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_DirHandle;
    fn uio_closeDir(dirHandle: *mut uio_DirHandle) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
}
pub type __builtin_va_list = *mut ::core::ffi::c_void;
pub type size_t = usize;
pub type time_t = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type va_list = __builtin_va_list;
pub type __isoc_va_list = __builtin_va_list;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type FILE = _IO_FILE;
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
pub type mode_t = ::core::ffi::c_uint;
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
pub type ino_t = ::core::ffi::c_ulonglong;
pub type blkcnt_t = ::core::ffi::c_int;
pub type blksize_t = ::core::ffi::c_int;
pub type dev_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type uid_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
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
pub type uio_uint8 = ::core::ffi::c_uchar;
pub type uio_uint32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_StdioAccessHandle {
    pub tempRoot: *mut uio_DirHandle,
    pub tempDirName: *mut ::core::ffi::c_char,
    pub tempDir: *mut uio_DirHandle,
    pub fileName: *mut ::core::ffi::c_char,
    pub stdioPath: *mut ::core::ffi::c_char,
}
#[inline]
unsafe extern "C" fn uio_DirHandle_ref(mut dirHandle: *mut uio_DirHandle) {
    (*dirHandle).r#ref += 1;
}
#[inline]
unsafe extern "C" fn uio_DirHandle_unref(mut dirHandle: *mut uio_DirHandle) {
    ((*dirHandle).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"dirHandle->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/iointrn.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int,
                b"uio_DirHandle_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*dirHandle).r#ref -= 1;
    if (*dirHandle).r#ref == 0 as ::core::ffi::c_int {
        uio_DirHandle_delete(dirHandle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_copyFile(
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
    let mut buf: *mut uio_uint8 = ::core::ptr::null_mut::<uio_uint8>();
    let mut bufPtr: *mut uio_uint8 = ::core::ptr::null_mut::<uio_uint8>();
    let mut numInBuf: ssize_t = 0;
    let mut numWritten: ssize_t = 0;
    src = uqm_lua_checked!(uio_open(srcDir, srcName, 0 as ::core::ffi::c_int, 0 as mode_t), 0);
    if src.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(uio_fstat(src, &raw mut sb), 0) == -1 as ::core::ffi::c_int {
        return uqm_lua_checked!(uio_copyError(
            src,
            ::core::ptr::null_mut::<uio_Handle>(),
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<uio_uint8>(),
        ), 0);
    }
    dst = uqm_lua_checked!(uio_open(
        dstDir,
        newName,
        0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int,
        sb.st_mode
            & (0o700 as ::core::ffi::c_int | 0o70 as ::core::ffi::c_int
                | 0o7 as ::core::ffi::c_int) as mode_t,
    ), 0);
    if dst.is_null() {
        return uqm_lua_checked!(uio_copyError(
            src,
            ::core::ptr::null_mut::<uio_Handle>(),
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<uio_uint8>(),
        ), 0);
    }
    buf = malloc(65536 as ::core::ffi::c_int as size_t) as *mut uio_uint8;
    loop {
        numInBuf = uqm_lua_checked!(uio_read(
            src,
            buf as *mut ::core::ffi::c_void,
            65536 as ::core::ffi::c_int as size_t,
        ), 0);
        if numInBuf == -1 as ssize_t {
            if *__errno_location() == 27 as ::core::ffi::c_int {
                continue;
            }
            return uqm_lua_checked!(uio_copyError(src, dst, dstDir, newName, buf), 0);
        } else {
            if numInBuf == 0 as ssize_t {
                break;
            }
            bufPtr = buf;
            loop {
                numWritten = uqm_lua_checked!(uio_write(
                    dst,
                    bufPtr as *const ::core::ffi::c_void,
                    numInBuf as size_t,
                ), 0);
                if numWritten == -1 as ssize_t {
                    if *__errno_location() != 27 as ::core::ffi::c_int {
                        return uqm_lua_checked!(uio_copyError(src, dst, dstDir, newName, buf), 0);
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
    free(buf as *mut ::core::ffi::c_void);
    uqm_lua_checked!(uio_close(src), 0);
    uqm_lua_checked!(uio_close(dst), 0);
    *__errno_location() = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_copyError(
    mut srcHandle: *mut uio_Handle,
    mut dstHandle: *mut uio_Handle,
    mut unlinkHandle: *mut uio_DirHandle,
    mut unlinkPath: *const ::core::ffi::c_char,
    mut buf: *mut uio_uint8,
) -> ::core::ffi::c_int {
    let mut savedErrno: ::core::ffi::c_int = 0;
    savedErrno = *__errno_location();
    fprintf(
        stderr,
        b"Error while copying: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        strerror(*__errno_location()),
    );
    if !srcHandle.is_null() {
        uqm_lua_checked!(uio_close(srcHandle), 0);
    }
    if !dstHandle.is_null() {
        uqm_lua_checked!(uio_close(dstHandle), 0);
    }
    if !unlinkPath.is_null() {
        uqm_lua_checked!(uio_unlink(unlinkHandle, unlinkPath), 0);
    }
    if !buf.is_null() {
        free(buf as *mut ::core::ffi::c_void);
    }
    *__errno_location() = savedErrno;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getStdioAccess(
    mut dir: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut tempDir: *mut uio_DirHandle,
) -> *mut uio_StdioAccessHandle {
    let mut res: ::core::ffi::c_int = 0;
    let mut mountHandle: *mut uio_MountHandle = ::core::ptr::null_mut::<
        uio_MountHandle,
    >();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut newPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tempDirName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut newDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut fsID: uio_FileSystemID = 0;
    res = uqm_lua_checked!(uio_getFileLocation(dir, path, flags, &raw mut mountHandle, &raw mut newPath), ::core::ptr::null_mut());
    if res == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
    }
    fsID = uio_getMountFileSystemType(mountHandle);
    if fsID == 1 as ::core::ffi::c_int {
        return uio_StdioAccessHandle_new(
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            newPath,
        );
    }
    free(newPath as *mut ::core::ffi::c_void);
    let mut dirNum: uio_uint32 = 0;
    let mut i: ::core::ffi::c_int = 0;
    dirNum = time(::core::ptr::null_mut::<time_t>()) as uio_uint32;
    tempDirName = malloc(::core::mem::size_of::<[::core::ffi::c_char; 9]>())
        as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    loop {
        if i >= 16 as ::core::ffi::c_int {
            free(tempDirName as *mut ::core::ffi::c_void);
            *__errno_location() = 51 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
        }
        sprintf(
            tempDirName,
            b"%08lx\0".as_ptr() as *const ::core::ffi::c_char,
            (dirNum as ::core::ffi::c_ulong).wrapping_add(i as ::core::ffi::c_ulong),
        );
        res = uqm_lua_checked!(uio_mkdir(tempDir, tempDirName, 0o700 as mode_t), ::core::ptr::null_mut());
        if res != -1 as ::core::ffi::c_int {
            break;
        }
        let mut savedErrno: ::core::ffi::c_int = 0;
        if *__errno_location() == 20 as ::core::ffi::c_int {
            i += 1;
        } else {
            savedErrno = *__errno_location();
            fprintf(
                stderr,
                b"Error: Could not create temporary dir: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            free(tempDirName as *mut ::core::ffi::c_void);
            *__errno_location() = savedErrno;
            return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
        }
    }
    newDir = uqm_lua_checked!(uio_openDirRelative(tempDir, tempDirName, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
    if newDir.is_null() {
        fprintf(
            stderr,
            b"Error: Could not open temporary dir: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        res = uqm_lua_checked!(uio_rmdir(tempDir, tempDirName), ::core::ptr::null_mut());
        if res == -1 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"Warning: Could not remove temporary dir: %s.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        free(tempDirName as *mut ::core::ffi::c_void);
        *__errno_location() = 29 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
    }
    name = strrchr(path, '/' as ::core::ffi::c_int);
    if name.is_null() {
        name = path;
    }
    res = uqm_lua_checked!(uio_copyFile(dir, path, newDir, name), ::core::ptr::null_mut());
    if res == -1 as ::core::ffi::c_int {
        let mut savedErrno_0: ::core::ffi::c_int = *__errno_location();
        fprintf(
            stderr,
            b"Error: Could not copy file to temporary dir: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        uio_closeDir(newDir);
        free(tempDirName as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno_0;
        return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
    }
    res = uqm_lua_checked!(uio_getFileLocation(
        newDir,
        name,
        flags,
        &raw mut mountHandle,
        &raw mut newPath,
    ), ::core::ptr::null_mut());
    if res == -1 as ::core::ffi::c_int {
        let mut savedErrno_1: ::core::ffi::c_int = *__errno_location();
        fprintf(
            stderr,
            b"Error: uio_getStdioAccess: Could not get location of temporary dir: %s.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        uio_closeDir(newDir);
        free(tempDirName as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno_1;
        return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
    }
    fsID = uio_getMountFileSystemType(mountHandle);
    if fsID != 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Error: uio_getStdioAccess: Temporary file location isn't on a stdio filesystem.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        uio_closeDir(newDir);
        free(tempDirName as *mut ::core::ffi::c_void);
        free(newPath as *mut ::core::ffi::c_void);
        *__errno_location() = 28 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_StdioAccessHandle>();
    }
    uio_DirHandle_ref(tempDir);
    return uio_StdioAccessHandle_new(
        tempDir,
        tempDirName,
        newDir,
        strdup(name),
        newPath,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uio_releaseStdioAccess(mut handle: *mut uio_StdioAccessHandle) {
    if !(*handle).tempDir.is_null() {
        if uqm_lua_checked!(uio_unlink((*handle).tempDir, (*handle).fileName), ()) == -1 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Error: Could not remove temporary file: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        uio_DirHandle_unref((*handle).tempDir);
        (*handle).tempDir = ::core::ptr::null_mut::<uio_DirHandle>();
        if uqm_lua_checked!(uio_rmdir((*handle).tempRoot, (*handle).tempDirName), ())
            == -1 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Error: Could not remove temporary directory: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
    }
    uio_StdioAccessHandle_delete(handle);
}
#[no_mangle]
pub unsafe extern "C" fn uio_StdioAccessHandle_getPath(
    mut handle: *mut uio_StdioAccessHandle,
) -> *const ::core::ffi::c_char {
    return (*handle).stdioPath as *const ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn uio_StdioAccessHandle_new(
    mut tempRoot: *mut uio_DirHandle,
    mut tempDirName: *mut ::core::ffi::c_char,
    mut tempDir: *mut uio_DirHandle,
    mut fileName: *mut ::core::ffi::c_char,
    mut stdioPath: *mut ::core::ffi::c_char,
) -> *mut uio_StdioAccessHandle {
    let mut result: *mut uio_StdioAccessHandle = ::core::ptr::null_mut::<
        uio_StdioAccessHandle,
    >();
    result = uio_StdioAccessHandle_alloc();
    (*result).tempRoot = tempRoot;
    (*result).tempDirName = tempDirName;
    (*result).tempDir = tempDir;
    (*result).fileName = fileName;
    (*result).stdioPath = stdioPath;
    return result;
}
#[inline]
unsafe extern "C" fn uio_StdioAccessHandle_delete(
    mut handle: *mut uio_StdioAccessHandle,
) {
    if !(*handle).tempDir.is_null() {
        uio_DirHandle_unref((*handle).tempDir);
    }
    if !(*handle).fileName.is_null() {
        free((*handle).fileName as *mut ::core::ffi::c_void);
    }
    if !(*handle).tempRoot.is_null() {
        uio_DirHandle_unref((*handle).tempRoot);
    }
    if !(*handle).tempDirName.is_null() {
        free((*handle).tempDirName as *mut ::core::ffi::c_void);
    }
    free((*handle).stdioPath as *mut ::core::ffi::c_void);
    uio_StdioAccessHandle_free(handle);
}
#[inline]
unsafe extern "C" fn uio_StdioAccessHandle_alloc() -> *mut uio_StdioAccessHandle {
    return malloc(::core::mem::size_of::<uio_StdioAccessHandle>())
        as *mut uio_StdioAccessHandle;
}
#[inline]
unsafe extern "C" fn uio_StdioAccessHandle_free(mut handle: *mut uio_StdioAccessHandle) {
    free(handle as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_vasprintf(
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufSize: size_t = 128 as size_t;
    buf = malloc(bufSize) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    loop {
        let mut printResult: ::core::ffi::c_int = vsnprintf(
            buf,
            bufSize,
            format,
            args.as_va_list(),
        );
        if printResult < 0 as ::core::ffi::c_int {
            bufSize = bufSize.wrapping_mul(2 as size_t);
        } else if printResult as ::core::ffi::c_uint as size_t >= bufSize {
            bufSize = (printResult + 1 as ::core::ffi::c_int) as size_t;
        } else {
            if (printResult as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as size_t != bufSize
            {
                let mut newBuf: *mut ::core::ffi::c_char = realloc(
                    buf as *mut ::core::ffi::c_void,
                    (printResult + 1 as ::core::ffi::c_int) as size_t,
                ) as *mut ::core::ffi::c_char;
                if newBuf.is_null() {
                    let mut savedErrno: ::core::ffi::c_int = *__errno_location();
                    free(buf as *mut ::core::ffi::c_void);
                    *__errno_location() = savedErrno;
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                return newBuf;
            }
            return buf;
        }
        let mut newBuf_0: *mut ::core::ffi::c_char = realloc(
            buf as *mut ::core::ffi::c_void,
            bufSize,
        ) as *mut ::core::ffi::c_char;
        if newBuf_0.is_null() {
            let mut savedErrno_0: ::core::ffi::c_int = *__errno_location();
            free(buf as *mut ::core::ffi::c_void);
            *__errno_location() = savedErrno_0;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        buf = newBuf_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_asprintf(
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    let mut args: ::core::ffi::VaListImpl;
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut savedErrno: ::core::ffi::c_int = 0;
    args = c2rust_args.clone();
    result = uio_vasprintf(format, args.as_va_list());
    savedErrno = *__errno_location();
    *__errno_location() = savedErrno;
    return result;
}
