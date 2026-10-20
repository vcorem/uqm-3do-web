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

// auxiliary functions for use by io.c

// Rust translation of src/libs/uio/ioaux.c.
extern "C" {
    pub type _IO_FILE;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn uio_getPDirEntryHandle(
        dirHandle: *const uio_PDirHandle,
        name: *const ::core::ffi::c_char,
    ) -> *mut uio_PDirEntryHandle;
    fn uio_PDirEntryHandle_delete(pDirEntryHandle: *mut uio_PDirEntryHandle);
    fn uio_PDirHandle_delete(pDirHandle: *mut uio_PDirHandle);
    fn uio_findMountTree(
        top: *mut uio_MountTree,
        path: *const ::core::ffi::c_char,
        resTree: *mut *mut uio_MountTree,
        pPath: *mut *const ::core::ffi::c_char,
    );
    fn uio_mountTreeItemRestPath(
        item: *const uio_MountTreeItem,
        endComp: *mut uio_PathComp,
        path: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn uio_mountTreeCountPLocs(tree: *const uio_MountTree) -> ::core::ffi::c_int;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn getFirstPathComponent(
        dir: *const ::core::ffi::c_char,
        dirEnd: *const ::core::ffi::c_char,
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn getNextPathComponent(
        dirEnd: *const ::core::ffi::c_char,
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn getLastPathComponent(
        dir: *const ::core::ffi::c_char,
        dirEnd: *const ::core::ffi::c_char,
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn getLastPath0Component(
        dir: *const ::core::ffi::c_char,
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn getPreviousPathComponent(
        dir: *const ::core::ffi::c_char,
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn joinPathsAbsolute(
        first: *const ::core::ffi::c_char,
        second: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type mode_t = ::core::ffi::c_uint;
pub type off_t = ::core::ffi::c_longlong;
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
pub type ssize_t = isize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type time_t = ::core::ffi::c_longlong;
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
pub type FILE = _IO_FILE;
pub type uio_bool = bool;
#[inline]
unsafe extern "C" fn uio_PDirEntryHandle_isDir(
    mut handle: *const uio_PDirEntryHandle,
) -> uio_bool {
    return (*handle).flags & 0x1 as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn uio_PDirEntryHandle_unref(mut handle: *mut uio_PDirEntryHandle) {
    ((*handle).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"handle->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/iointrn.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_int,
                b"uio_PDirEntryHandle_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*handle).r#ref -= 1;
    if (*handle).r#ref == 0 as ::core::ffi::c_int {
        uio_PDirEntryHandle_delete(handle);
    }
}
#[inline]
unsafe extern "C" fn uio_PDirHandle_ref(mut handle: *mut uio_PDirHandle) {
    (*handle).r#ref += 1;
}
#[inline]
unsafe extern "C" fn uio_PDirHandle_unref(mut handle: *mut uio_PDirHandle) {
    ((*handle).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"handle->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/iointrn.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                175 as ::core::ffi::c_int,
                b"uio_PDirHandle_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*handle).r#ref -= 1;
    if (*handle).r#ref == 0 as ::core::ffi::c_int {
        uio_PDirHandle_delete(handle);
    }
}
#[inline]
unsafe extern "C" fn uio_mountInfoIsReadOnly(
    mut mountInfo: *mut uio_MountInfo,
) -> uio_bool {
    return (*mountInfo).flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_walkPhysicalPath(
    mut startPDirHandle: *mut uio_PDirHandle,
    mut path: *const ::core::ffi::c_char,
    mut pathLen: size_t,
    mut endPDirHandle: *mut *mut uio_PDirHandle,
    mut pathRest: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut partStart: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut partEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut tempBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut entry: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
        uio_PDirEntryHandle,
    >();
    let mut retVal: ::core::ffi::c_int = 0;
    uio_PDirHandle_ref(startPDirHandle);
    pDirHandle = startPDirHandle;
    tempBuf = malloc(strlen(path).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    pathEnd = path.offset(pathLen as isize);
    getFirstPathComponent(path, pathEnd, &raw mut partStart, &raw mut partEnd);
    loop {
        if partStart == pathEnd {
            retVal = 0 as ::core::ffi::c_int;
            break;
        } else {
            memcpy(
                tempBuf as *mut ::core::ffi::c_void,
                partStart as *const ::core::ffi::c_void,
                partEnd.offset_from(partStart) as size_t,
            );
            *tempBuf.offset(partEnd.offset_from(partStart) as isize) = '\0'
                as ::core::ffi::c_char;
            entry = uio_getPDirEntryHandle(pDirHandle, tempBuf);
            if entry.is_null() {
                retVal = 44 as ::core::ffi::c_int;
                break;
            } else if !uio_PDirEntryHandle_isDir(entry) {
                uio_PDirEntryHandle_unref(entry);
                retVal = 54 as ::core::ffi::c_int;
                break;
            } else {
                uio_PDirHandle_unref(pDirHandle);
                pDirHandle = entry as *mut uio_PDirHandle;
                getNextPathComponent(pathEnd, &raw mut partStart, &raw mut partEnd);
            }
        }
    }
    free(tempBuf as *mut ::core::ffi::c_void);
    *pathRest = partStart;
    *endPDirHandle = pDirHandle;
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn uio_makePath(
    mut pDirHandle: *mut uio_PDirHandle,
    mut path: *const ::core::ffi::c_char,
    mut pathLen: size_t,
    mut mode: mode_t,
) -> *mut uio_PDirHandle {
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mkdirFun: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            mode_t,
        ) -> *mut uio_PDirHandle,
    > = None;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut newPDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    mkdirFun = (*(*(*pDirHandle).pRoot).handler).mkdir;
    if mkdirFun.is_none() {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_PDirHandle>();
    }
    pathEnd = path.offset(pathLen as isize);
    buf = malloc(pathLen.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    uio_walkPhysicalPath(pDirHandle, path, pathLen, &raw mut pDirHandle, &raw mut rest);
    getFirstPathComponent(rest, pathEnd, &raw mut start, &raw mut end);
    while start < pathEnd {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            end.offset_from(start) as size_t,
        );
        *buf.offset(end.offset_from(start) as isize) = '\0' as ::core::ffi::c_char;
        newPDirHandle = mkdirFun
            .expect("non-null function pointer")(pDirHandle, buf, mode);
        if newPDirHandle.is_null() {
            let mut savedErrno: ::core::ffi::c_int = *__errno_location();
            uio_PDirHandle_unref(pDirHandle);
            *__errno_location() = savedErrno;
            free(buf as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<uio_PDirHandle>();
        }
        uio_PDirHandle_unref(pDirHandle);
        pDirHandle = newPDirHandle;
        getNextPathComponent(pathEnd, &raw mut start, &raw mut end);
    }
    free(buf as *mut ::core::ffi::c_void);
    return pDirHandle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_copyFilePhysical(
    mut fromDir: *mut uio_PDirHandle,
    mut fromName: *const ::core::ffi::c_char,
    mut toDir: *mut uio_PDirHandle,
    mut toName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fromHandler: *mut uio_FileSystemHandler = ::core::ptr::null_mut::<
        uio_FileSystemHandler,
    >();
    let mut toHandler: *mut uio_FileSystemHandler = ::core::ptr::null_mut::<
        uio_FileSystemHandler,
    >();
    let mut fromHandle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let mut toHandle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let mut statBuf: stat = stat {
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
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut numInBuf: ssize_t = 0;
    let mut numWritten: ssize_t = 0;
    fromHandler = (*(*fromDir).pRoot).handler as *mut uio_FileSystemHandler;
    toHandler = (*(*toDir).pRoot).handler as *mut uio_FileSystemHandler;
    if (*toHandler).write.is_none() || (*fromHandler).fstat.is_none()
        || (*toHandler).unlink.is_none()
    {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    fromHandle = (*fromHandler)
        .open
        .expect(
            "non-null function pointer",
        )(fromDir, fromName, 0 as ::core::ffi::c_int, 0 as mode_t);
    if fromHandle.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*fromHandler)
        .fstat
        .expect("non-null function pointer")(fromHandle, &raw mut statBuf)
        == -1 as ::core::ffi::c_int
    {
        return copyError(
            *__errno_location(),
            fromHandler,
            fromHandle,
            toHandler,
            ::core::ptr::null_mut::<uio_Handle>(),
            ::core::ptr::null_mut::<uio_PDirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    toHandle = (*toHandler)
        .open
        .expect(
            "non-null function pointer",
        )(
        toDir,
        toName,
        0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int,
        statBuf.st_mode
            & (0o700 as ::core::ffi::c_int | 0o70 as ::core::ffi::c_int
                | 0o7 as ::core::ffi::c_int) as mode_t,
    );
    if toHandle.is_null() {
        return copyError(
            *__errno_location(),
            fromHandler,
            fromHandle,
            toHandler,
            ::core::ptr::null_mut::<uio_Handle>(),
            ::core::ptr::null_mut::<uio_PDirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    buf = malloc(0x10000 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_char;
    loop {
        numInBuf = (*fromHandler)
            .read
            .expect(
                "non-null function pointer",
            )(
            fromHandle,
            buf as *mut ::core::ffi::c_void,
            0x10000 as ::core::ffi::c_int as size_t,
        );
        if numInBuf == -1 as ssize_t {
            if *__errno_location() == 27 as ::core::ffi::c_int {
                continue;
            }
            return copyError(
                *__errno_location(),
                fromHandler,
                fromHandle,
                toHandler,
                toHandle,
                toDir,
                toName,
                buf,
            );
        } else {
            if numInBuf == 0 as ssize_t {
                break;
            }
            bufPtr = buf;
            loop {
                numWritten = (*toHandler)
                    .write
                    .expect(
                        "non-null function pointer",
                    )(
                    toHandle,
                    bufPtr as *const ::core::ffi::c_void,
                    numInBuf as size_t,
                );
                if numWritten == -1 as ssize_t {
                    if *__errno_location() != 27 as ::core::ffi::c_int {
                        return copyError(
                            *__errno_location(),
                            fromHandler,
                            fromHandle,
                            toHandler,
                            toHandle,
                            toDir,
                            toName,
                            buf,
                        );
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
    (*toHandler).close.expect("non-null function pointer")(toHandle);
    (*fromHandler).close.expect("non-null function pointer")(fromHandle);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn copyError(
    mut error: ::core::ffi::c_int,
    mut fromHandler: *mut uio_FileSystemHandler,
    mut fromHandle: *mut uio_Handle,
    mut toHandler: *mut uio_FileSystemHandler,
    mut toHandle: *mut uio_Handle,
    mut toDir: *mut uio_PDirHandle,
    mut toName: *const ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    fprintf(
        stderr,
        b"Error while copying: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        strerror(error),
    );
    if !fromHandle.is_null() {
        (*fromHandler).close.expect("non-null function pointer")(fromHandle);
    }
    if !toHandle.is_null() {
        (*toHandler).close.expect("non-null function pointer")(toHandle);
    }
    if !toName.is_null() {
        (*toHandler).unlink.expect("non-null function pointer")(toDir, toName);
    }
    if !buf.is_null() {
        free(buf as *mut ::core::ffi::c_void);
    }
    *__errno_location() = error;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getPhysicalAccess(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut extraFlags: ::core::ffi::c_int,
    mut mountInfoReadPtr: *mut *mut uio_MountInfo,
    mut readPDirHandlePtr: *mut *mut uio_PDirHandle,
    mut readPRootPathPtr: *mut *mut ::core::ffi::c_char,
    mut mountInfoWritePtr: *mut *mut uio_MountInfo,
    mut writePDirHandlePtr: *mut *mut uio_PDirHandle,
    mut writePRootPathPtr: *mut *mut ::core::ffi::c_char,
    mut restPtr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fullPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pRootPath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut readPRootPath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut writePRootPath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut readRest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut tree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    let mut readItem: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    let mut writeItem: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    let mut readPDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut writePDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut retVal: ::core::ffi::c_int = 0;
    let mut entryExists: uio_bool = false;
    if uio_resolvePath(dirHandle, path, strlen(path), &raw mut fullPath) == -1 as ssize_t
    {
        return -1 as ::core::ffi::c_int;
    }
    uio_findMountTree(
        (*(*dirHandle).repository).mountTree,
        fullPath,
        &raw mut tree,
        &raw mut rest,
    );
    readItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    readPDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    readPRootPath = ::core::ptr::null::<::core::ffi::c_char>();
    writeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    writePDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    writePRootPath = ::core::ptr::null::<::core::ffi::c_char>();
    readRest = ::core::ptr::null::<::core::ffi::c_char>();
    entryExists = false;
    item = (*tree).pLocs as *mut uio_MountTreeItem;
    's_216: while !item.is_null() {
        pRootPath = uio_mountTreeItemRestPath(item, (*tree).lastComp, fullPath);
        retVal = uio_walkPhysicalPath(
            (*(*item).mountInfo).pDirHandle,
            pRootPath,
            strlen(pRootPath),
            &raw mut pDirHandle,
            &raw mut rest,
        );
        if retVal == 0 as ::core::ffi::c_int {
            free(fullPath as *mut ::core::ffi::c_void);
            uio_PDirHandle_unref(pDirHandle);
            if !readPDirHandle.is_null() {
                uio_PDirHandle_unref(readPDirHandle);
            }
            *__errno_location() = 31 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        if writeItem.is_null() && !uio_mountInfoIsReadOnly((*item).mountInfo) {
            writeItem = item;
        }
        's_61: {
            if strchr(rest, '/' as ::core::ffi::c_int).is_null() {
                let mut entry: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
                    uio_PDirEntryHandle,
                >();
                entry = uio_getPDirEntryHandle(pDirHandle, rest);
                if !entry.is_null() {
                    uio_PDirEntryHandle_unref(entry);
                    readItem = item;
                    if !readPDirHandle.is_null() {
                        uio_PDirHandle_unref(readPDirHandle);
                    }
                    readPDirHandle = pDirHandle;
                    readPRootPath = pRootPath;
                    readRest = rest;
                    entryExists = true;
                    break 's_216;
                } else if flags
                    & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
                    != 0 as ::core::ffi::c_int && readItem.is_null()
                {
                    readItem = item;
                    (readPDirHandle.is_null()
                        || {
                            __assert_fail(
                                b"readPDirHandle == NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"src/libs/uio/ioaux.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                436 as ::core::ffi::c_int,
                                b"uio_getPhysicalAccess\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            0 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    readPDirHandle = pDirHandle;
                    readPRootPath = pRootPath;
                    readRest = rest;
                    break 's_61;
                }
            } else if retVal == 54 as ::core::ffi::c_int {
                free(fullPath as *mut ::core::ffi::c_void);
                uio_PDirHandle_unref(pDirHandle);
                if !readPDirHandle.is_null() {
                    uio_PDirHandle_unref(readPDirHandle);
                }
                *__errno_location() = 54 as ::core::ffi::c_int;
                return -1 as ::core::ffi::c_int;
            }
            uio_PDirHandle_unref(pDirHandle);
        }
        item = (*item).next as *mut uio_MountTreeItem;
    }
    if readItem.is_null() {
        free(fullPath as *mut ::core::ffi::c_void);
        *__errno_location() = 44 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if flags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        *mountInfoReadPtr = (*readItem).mountInfo as *mut uio_MountInfo;
        *readPDirHandlePtr = readPDirHandle;
        if !readPRootPathPtr.is_null() {
            *readPRootPathPtr = joinPathsAbsolute(
                (*(*readItem).mountInfo).dirName,
                readPRootPath,
            );
        }
        *restPtr = strdup(readRest);
        free(fullPath as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    } else if entryExists {
        if flags & 0o100 as ::core::ffi::c_int != 0
            && flags & 0o200 as ::core::ffi::c_int != 0
        {
            uio_PDirHandle_unref(readPDirHandle);
            free(fullPath as *mut ::core::ffi::c_void);
            *__errno_location() = 20 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
    } else if flags & 0o100 as ::core::ffi::c_int != 0 {} else {
        uio_PDirHandle_unref(readPDirHandle);
        free(fullPath as *mut ::core::ffi::c_void);
        *__errno_location() = 44 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if writeItem == readItem {
        *mountInfoReadPtr = (*readItem).mountInfo as *mut uio_MountInfo;
        *readPDirHandlePtr = readPDirHandle;
        if !readPRootPathPtr.is_null() {
            *readPRootPathPtr = joinPathsAbsolute(
                (*(*readItem).mountInfo).dirName,
                readPRootPath,
            );
        }
        *mountInfoWritePtr = (*writeItem).mountInfo as *mut uio_MountInfo;
        uio_PDirHandle_ref(readPDirHandle);
        *writePDirHandlePtr = readPDirHandle;
        if !writePRootPathPtr.is_null() {
            *writePRootPathPtr = joinPathsAbsolute(
                (*(*writeItem).mountInfo).dirName,
                writePRootPath,
            );
        }
        if !restPtr.is_null() {
            *restPtr = strdup(readRest);
        }
        free(fullPath as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    if writeItem.is_null() {
        free(fullPath as *mut ::core::ffi::c_void);
        uio_PDirHandle_unref(readPDirHandle);
        *__errno_location() = 63 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    pRootPath = uio_mountTreeItemRestPath(writeItem, (*tree).lastComp, fullPath);
    rest = strrchr(pRootPath, '/' as ::core::ffi::c_int);
    if rest.is_null() {
        rest = pRootPath;
        uio_PDirHandle_ref((*(*writeItem).mountInfo).pDirHandle);
        writePDirHandle = (*(*writeItem).mountInfo).pDirHandle;
    } else {
        if extraFlags & 1 as ::core::ffi::c_int != 0 {
            uio_PDirHandle_unref(readPDirHandle);
            free(fullPath as *mut ::core::ffi::c_void);
            *__errno_location() = 44 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        writePDirHandle = uio_makePath(
            (*(*writeItem).mountInfo).pDirHandle,
            pRootPath,
            rest.offset_from(pRootPath) as size_t,
            0o777 as mode_t,
        );
        if writePDirHandle.is_null() {
            let mut savedErrno: ::core::ffi::c_int = 0;
            if *__errno_location() == 52 as ::core::ffi::c_int {
                *__errno_location() = 29 as ::core::ffi::c_int;
            }
            savedErrno = *__errno_location();
            uio_PDirHandle_unref(readPDirHandle);
            free(fullPath as *mut ::core::ffi::c_void);
            *__errno_location() = savedErrno;
            return -1 as ::core::ffi::c_int;
        }
        rest = rest.offset(1);
    }
    if !entryExists {
        readItem = writeItem;
        uio_PDirHandle_ref(writePDirHandle);
        uio_PDirHandle_unref(readPDirHandle);
        readPDirHandle = writePDirHandle;
    }
    *mountInfoReadPtr = (*readItem).mountInfo as *mut uio_MountInfo;
    *readPDirHandlePtr = readPDirHandle;
    if !readPRootPathPtr.is_null() {
        *readPRootPathPtr = joinPathsAbsolute(
            (*(*readItem).mountInfo).dirName,
            readPRootPath,
        );
    }
    *mountInfoWritePtr = (*writeItem).mountInfo as *mut uio_MountInfo;
    *writePDirHandlePtr = writePDirHandle;
    if !writePRootPathPtr.is_null() {
        *writePRootPathPtr = joinPathsAbsolute(
            (*(*writeItem).mountInfo).dirName,
            writePRootPath,
        );
    }
    if !restPtr.is_null() {
        *restPtr = strdup(rest);
    }
    free(fullPath as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getPathPhysicalDirs(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut pathLen: size_t,
    mut resPDirHandles: *mut *mut *mut uio_PDirHandle,
    mut resNumPDirHandles: *mut ::core::ffi::c_int,
    mut resItems: *mut *mut *mut uio_MountTreeItem,
) -> ::core::ffi::c_int {
    let mut pDirHandles: *mut *mut uio_PDirHandle = ::core::ptr::null_mut::<
        *mut uio_PDirHandle,
    >();
    let mut fullPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    let mut items: *mut *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        *mut uio_MountTreeItem,
    >();
    let mut pRootPath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut numPDirHandles: ::core::ffi::c_int = 0;
    let mut pDirI: ::core::ffi::c_int = 0;
    if uio_resolvePath(dirHandle, path, pathLen, &raw mut fullPath) == -1 as ssize_t {
        return -1 as ::core::ffi::c_int;
    }
    uio_findMountTree(
        (*(*dirHandle).repository).mountTree,
        fullPath,
        &raw mut tree,
        &raw mut rest,
    );
    numPDirHandles = uio_mountTreeCountPLocs(tree);
    pDirHandles = malloc(
        (numPDirHandles as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut uio_PDirHandle>()),
    ) as *mut *mut uio_PDirHandle;
    if !resItems.is_null() {
        items = malloc(
            (numPDirHandles as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut uio_MountTreeItem>()),
        ) as *mut *mut uio_MountTreeItem;
    } else {
        items = ::core::ptr::null_mut::<*mut uio_MountTreeItem>();
    }
    pDirI = 0 as ::core::ffi::c_int;
    item = (*tree).pLocs as *mut uio_MountTreeItem;
    while !item.is_null() {
        let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
            uio_PDirHandle,
        >();
        pRootPath = uio_mountTreeItemRestPath(item, (*tree).lastComp, fullPath);
        match uio_walkPhysicalPath(
            (*(*item).mountInfo).pDirHandle,
            pRootPath,
            strlen(pRootPath),
            &raw mut pDirHandle,
            &raw mut rest,
        ) {
            0 => {
                *pDirHandles.offset(pDirI as isize) = pDirHandle;
                if !resItems.is_null() {
                    *items.offset(pDirI as isize) = item;
                }
                pDirI += 1;
            }
            44 => {
                uio_PDirHandle_unref(pDirHandle);
            }
            54 => {
                uio_PDirHandle_unref(pDirHandle);
                break;
            }
            _ => {
                (false
                    || {
                        __assert_fail(
                            b"false\0".as_ptr() as *const ::core::ffi::c_char,
                            b"src/libs/uio/ioaux.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            683 as ::core::ffi::c_int,
                            b"uio_getPathPhysicalDirs\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        0 as ::core::ffi::c_int != 0
                    }) as ::core::ffi::c_int;
                uio_PDirHandle_unref(pDirHandle);
            }
        }
        item = (*item).next as *mut uio_MountTreeItem;
    }
    numPDirHandles = pDirI;
    free(fullPath as *mut ::core::ffi::c_void);
    *resPDirHandles = realloc(
        pDirHandles as *mut ::core::ffi::c_void,
        (numPDirHandles as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut uio_PDirHandle>()),
    ) as *mut *mut uio_PDirHandle;
    if !resItems.is_null() {
        *resItems = realloc(
            items as *mut ::core::ffi::c_void,
            (numPDirHandles as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut uio_MountTreeItem>()),
        ) as *mut *mut uio_MountTreeItem;
    }
    *resNumPDirHandles = numPDirHandles;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_verifyPath(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut resolvedPath: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut retVal: ::core::ffi::c_int = 0;
    if uio_resolvePath(dirHandle, path, strlen(path), resolvedPath) == -1 as ssize_t {
        return -1 as ::core::ffi::c_int;
    }
    uio_findMountTree(
        (*(*dirHandle).repository).mountTree,
        *resolvedPath,
        &raw mut tree,
        &raw mut rest,
    );
    if *rest.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    item = (*tree).pLocs as *mut uio_MountTreeItem;
    while !item.is_null() {
        let mut pRootPath: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
            uio_PDirHandle,
        >();
        pRootPath = uio_mountTreeItemRestPath(item, (*tree).lastComp, *resolvedPath);
        retVal = uio_walkPhysicalPath(
            (*(*item).mountInfo).pDirHandle,
            pRootPath,
            strlen(pRootPath),
            &raw mut pDirHandle,
            &raw mut rest,
        );
        uio_PDirHandle_unref(pDirHandle);
        match retVal {
            0 => return 0 as ::core::ffi::c_int,
            54 => {
                free(*resolvedPath as *mut ::core::ffi::c_void);
                *__errno_location() = 54 as ::core::ffi::c_int;
                return -1 as ::core::ffi::c_int;
            }
            44 => {
                item = (*item).next as *mut uio_MountTreeItem;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Warning: Unknown error from uio_walkPhysicalPath: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerror(retVal),
                );
                free(*resolvedPath as *mut ::core::ffi::c_void);
                *__errno_location() = retVal;
                return -1 as ::core::ffi::c_int;
            }
        }
    }
    free(*resolvedPath as *mut ::core::ffi::c_void);
    *__errno_location() = 44 as ::core::ffi::c_int;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_resolvePath(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut pathLen: size_t,
    mut destPath: *mut *mut ::core::ffi::c_char,
) -> ssize_t {
    let mut len: size_t = 0;
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut numUp: ::core::ffi::c_int = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut endBufPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut absolute: uio_bool = false;
    absolute = *path.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int;
    pathEnd = path.offset(pathLen as isize);
    len = 0 as size_t;
    numUp = 0 as ::core::ffi::c_int;
    getLastPathComponent(path, pathEnd, &raw mut start, &raw mut end);
    while end > path {
        's_28: {
            if *start.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
                if start.offset(1 as ::core::ffi::c_int as isize) == end {
                    break 's_28;
                } else if *start.offset(1isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                    && start.offset(2 as ::core::ffi::c_int as isize) == end
                {
                    numUp += 1;
                    break 's_28;
                }
            }
            if numUp > 0 as ::core::ffi::c_int {
                numUp -= 1;
            } else {
                len = len.wrapping_add((end.offset_from(start) + 1isize) as size_t);
            }
        }
        getPreviousPathComponent(path, &raw mut start, &raw mut end);
    }
    len = len
        .wrapping_add((*dirHandle).rootEnd.offset_from((*dirHandle).path) as size_t);
    if !absolute {
        getLastPath0Component((*dirHandle).rootEnd, &raw mut start, &raw mut end);
        while end > (*dirHandle).rootEnd as *const ::core::ffi::c_char {
            if numUp > 0 as ::core::ffi::c_int {
                numUp -= 1;
            } else {
                len = len.wrapping_add((end.offset_from(start) + 1isize) as size_t);
            }
            getPreviousPathComponent((*dirHandle).rootEnd, &raw mut start, &raw mut end);
        }
    }
    if numUp > 0 as ::core::ffi::c_int {
        *__errno_location() = 44 as ::core::ffi::c_int;
        return -1 as ssize_t;
    }
    if len == 0 as size_t {
        *destPath = malloc(1 as size_t) as *mut ::core::ffi::c_char;
        *(*destPath).offset(0isize) = '\0' as ::core::ffi::c_char;
        return 0 as ssize_t;
    }
    buffer = malloc(len) as *mut ::core::ffi::c_char;
    endBufPtr = buffer.offset(len as isize).offset(-(1 as ::core::ffi::c_int as isize));
    *endBufPtr = '\0' as ::core::ffi::c_char;
    numUp = 0 as ::core::ffi::c_int;
    getLastPathComponent(path, pathEnd, &raw mut start, &raw mut end);
    while end > path {
        's_159: {
            if *start.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
                if start.offset(1 as ::core::ffi::c_int as isize) == end {
                    break 's_159;
                } else if *start.offset(1isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                    && start.offset(2 as ::core::ffi::c_int as isize) == end
                {
                    numUp += 1;
                    break 's_159;
                }
            }
            if numUp > 0 as ::core::ffi::c_int {
                numUp -= 1;
            } else {
                endBufPtr = endBufPtr.offset(-(end.offset_from(start) as isize));
                memcpy(
                    endBufPtr as *mut ::core::ffi::c_void,
                    start as *const ::core::ffi::c_void,
                    end.offset_from(start) as size_t,
                );
                if endBufPtr != buffer {
                    endBufPtr = endBufPtr.offset(-1);
                    *endBufPtr = '/' as ::core::ffi::c_char;
                } else {
                    *destPath = buffer;
                    return len as ssize_t;
                }
            }
        }
        getPreviousPathComponent(path, &raw mut start, &raw mut end);
    }
    endBufPtr = endBufPtr
        .offset(-((*dirHandle).rootEnd.offset_from((*dirHandle).path) as isize));
    memcpy(
        endBufPtr as *mut ::core::ffi::c_void,
        (*dirHandle).path as *const ::core::ffi::c_void,
        (*dirHandle).rootEnd.offset_from((*dirHandle).path) as size_t,
    );
    if !absolute {
        getLastPath0Component((*dirHandle).rootEnd, &raw mut start, &raw mut end);
        while end > (*dirHandle).rootEnd as *const ::core::ffi::c_char {
            if numUp > 0 as ::core::ffi::c_int {
                numUp -= 1;
            } else {
                endBufPtr = endBufPtr.offset(-(end.offset_from(start) as isize));
                memcpy(
                    endBufPtr as *mut ::core::ffi::c_void,
                    start as *const ::core::ffi::c_void,
                    end.offset_from(start) as size_t,
                );
                if endBufPtr == buffer {
                    break;
                }
                endBufPtr = endBufPtr.offset(-1);
                *endBufPtr = '/' as ::core::ffi::c_char;
            }
            getPreviousPathComponent((*dirHandle).rootEnd, &raw mut start, &raw mut end);
        }
    }
    *destPath = buffer;
    return len as ssize_t;
}
