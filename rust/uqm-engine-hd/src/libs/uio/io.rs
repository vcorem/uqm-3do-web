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

// Rust translation of src/libs/uio/io.c.
extern "C" {
    pub type _IO_FILE;
    pub type match_MatchContext;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static stderr: *mut FILE;
    fn fwrite(
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn uio_PRoot_getRootDirHandle(pRoot: *mut uio_PRoot) -> *mut uio_PDirHandle;
    fn uio_PRoot_refHandle(pRoot: *mut uio_PRoot);
    fn uio_PRoot_unrefHandle(pRoot: *mut uio_PRoot);
    fn uio_PRoot_refMount(pRoot: *mut uio_PRoot);
    fn uio_PRoot_unrefMount(pRoot: *mut uio_PRoot);
    fn uio_registerDefaultFileSystems();
    fn uio_unRegisterDefaultFileSystems();
    fn uio_getFileSystemHandler(id: uio_FileSystemID) -> *mut uio_FileSystemHandler;
    fn match_prepareContext(
        pattern: *const ::core::ffi::c_char,
        contextPtr: *mut *mut match_MatchContext,
        r#type: match_MatchType,
    ) -> match_Result;
    fn match_matchPattern(
        context: *mut match_MatchContext,
        string: *const ::core::ffi::c_char,
    ) -> match_Result;
    fn match_errorString(
        context: *mut match_MatchContext,
        result: match_Result,
    ) -> *const ::core::ffi::c_char;
    fn match_freeContext(context: *mut match_MatchContext);
    fn validPathName(path: *const ::core::ffi::c_char, len: size_t) -> uio_bool;
    fn uio_Repository_new(flags: ::core::ffi::c_int) -> *mut uio_Repository;
    fn uio_Repository_unref(repository: *mut uio_Repository);
    fn uio_repositoryAddMount(
        repository: *mut uio_Repository,
        mountInfo: *mut uio_MountInfo,
        location: uio_MountLocation,
        relative: *mut uio_MountInfo,
    );
    fn uio_repositoryRemoveMount(
        repository: *mut uio_Repository,
        mountInfo: *mut uio_MountInfo,
    );
    fn uio_walkPhysicalPath(
        startPDirHandle: *mut uio_PDirHandle,
        path: *const ::core::ffi::c_char,
        pathLen: size_t,
        endPDirHandle: *mut *mut uio_PDirHandle,
        pathRest: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_copyFilePhysical(
        fromDir: *mut uio_PDirHandle,
        fromName: *const ::core::ffi::c_char,
        toDir: *mut uio_PDirHandle,
        toName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_getPhysicalAccess(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        extraFlags: ::core::ffi::c_int,
        mountInfoReadPtr: *mut *mut uio_MountInfo,
        readPDirHandlePtr: *mut *mut uio_PDirHandle,
        readPRootPathPtr: *mut *mut ::core::ffi::c_char,
        mountInfoWritePtr: *mut *mut uio_MountInfo,
        writePDirHandlePtr: *mut *mut uio_PDirHandle,
        writePRootPathPtr: *mut *mut ::core::ffi::c_char,
        restPtr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_getPathPhysicalDirs(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        pathLen: size_t,
        resPDirHandles: *mut *mut *mut uio_PDirHandle,
        resNumPDirHandles: *mut ::core::ffi::c_int,
        resItems: *mut *mut *mut uio_MountTreeItem,
    ) -> ::core::ffi::c_int;
    fn uio_verifyPath(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        resolvedPath: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_mountTreeAddMountInfo(
        repository: *mut uio_Repository,
        mountTree: *mut uio_MountTree,
        mountInfo: *mut uio_MountInfo,
        path: *const ::core::ffi::c_char,
        location: uio_MountLocation,
        relative: *const uio_MountInfo,
    ) -> *mut uio_MountTree;
    fn uio_mountTreeRemoveMountInfo(
        repository: *mut uio_Repository,
        mountTree: *mut uio_MountTree,
        mountInfo: *mut uio_MountInfo,
    );
    fn uio_MountInfo_new(
        fsID: uio_FileSystemID,
        mountTree: *mut uio_MountTree,
        pDirHandle: *mut uio_PDirHandle,
        dirName: *mut ::core::ffi::c_char,
        autoMount: *mut *mut uio_AutoMount,
        mountHandle: *mut uio_MountHandle,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_MountInfo;
    fn uio_MountInfo_delete(mountInfo: *mut uio_MountInfo);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type mode_t = ::core::ffi::c_uint;
pub type off_t = ::core::ffi::c_longlong;
pub type ssize_t = isize;
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
pub struct uio_PFileHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
    pub extra: uio_PFileHandleExtra,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_EntriesContext {
    pub pRoot: *mut uio_PRoot,
    pub native: uio_NativeEntriesContext,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_DirList {
    pub names: *mut *const ::core::ffi::c_char,
    pub numNames: ::core::ffi::c_int,
    pub buffer: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uio_MountLocation(pub ::core::ffi::c_uint);
impl uio_MountLocation {
    pub const uio_MOUNT_BOTTOM: Self = Self(0);
    pub const uio_MOUNT_TOP: Self = Self(4);
    pub const uio_MOUNT_BELOW: Self = Self(8);
    pub const uio_MOUNT_ABOVE: Self = Self(12);
}
pub type match_Result = ::core::ffi::c_int;
pub type uio_bool = bool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_DirBufferLink {
    pub buffer: *mut ::core::ffi::c_char,
    pub numEntries: ::core::ffi::c_int,
    pub next: *mut uio_DirBufferLink,
}
#[inline]
unsafe extern "C" fn uio_Handle_unref(mut handle: *mut uio_Handle) {
    ((*handle).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"handle->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/iointrn.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                130 as ::core::ffi::c_int,
                b"uio_Handle_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*handle).r#ref -= 1;
    if (*handle).r#ref == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(uio_Handle_delete(handle), ());
    }
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
        uqm_lua_checked!(uio_PDirEntryHandle_delete(handle), ());
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
        uqm_lua_checked!(uio_PDirHandle_delete(handle), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_init() {
    uqm_lua_checked!(uio_registerDefaultFileSystems(), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_unInit() {
    uqm_lua_checked!(uio_unRegisterDefaultFileSystems(), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_openRepository(
    mut flags: ::core::ffi::c_int,
) -> *mut uio_Repository {
    return uio_Repository_new(flags);
}
#[no_mangle]
pub unsafe extern "C" fn uio_closeRepository(mut repository: *mut uio_Repository) {
    uqm_lua_checked!(uio_unmountAllDirs(repository), ());
    uio_Repository_unref(repository);
}
#[no_mangle]
pub unsafe extern "C" fn uio_mountDir(
    mut destRep: *mut uio_Repository,
    mut mountPoint: *const ::core::ffi::c_char,
    mut fsType: uio_FileSystemID,
    mut sourceDir: *mut uio_DirHandle,
    mut sourcePath: *const ::core::ffi::c_char,
    mut inPath: *const ::core::ffi::c_char,
    mut autoMount: *mut *mut uio_AutoMount,
    mut flags: ::core::ffi::c_int,
    mut relative: *mut uio_MountHandle,
) -> *mut uio_MountHandle {
    let mut pRoot: *mut uio_PRoot = ::core::ptr::null_mut::<uio_PRoot>();
    let mut handle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let mut handler: *mut uio_FileSystemHandler = ::core::ptr::null_mut::<
        uio_FileSystemHandler,
    >();
    let mut relativeInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    match flags & (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int {
        4 | 0 => {
            if !relative.is_null() {
                *__errno_location() = 28 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<uio_MountHandle>();
            }
            relativeInfo = ::core::ptr::null_mut::<uio_MountInfo>();
        }
        8 | 12 => {
            if relative.is_null() {
                *__errno_location() = 28 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<uio_MountHandle>();
            }
            relativeInfo = (*relative).mountInfo as *mut uio_MountInfo;
        }
        _ => {
            abort();
        }
    }
    if *mountPoint.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        mountPoint = mountPoint.offset(1);
    }
    if !validPathName(mountPoint, strlen(mountPoint)) {
        *__errno_location() = 28 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    if sourceDir.is_null() {
        if !sourcePath.is_null() {
            *__errno_location() = 28 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<uio_MountHandle>();
        }
        handle = ::core::ptr::null_mut::<uio_Handle>();
    } else {
        if sourcePath.is_null() {
            *__errno_location() = 28 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<uio_MountHandle>();
        }
        log_add(
            log_Level::log_Info,
            b"uio_open %s\0".as_ptr() as *const ::core::ffi::c_char,
            sourcePath,
        );
        handle = uqm_lua_checked!(uio_open(
            sourceDir,
            sourcePath,
            if flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                == (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                0o2 as ::core::ffi::c_int
            },
            0 as mode_t,
        ), ::core::ptr::null_mut());
        if handle.is_null() {
            log_add(
                log_Level::log_Info,
                b"uio_open failed for %s\0".as_ptr() as *const ::core::ffi::c_char,
                sourcePath,
            );
            return ::core::ptr::null_mut::<uio_MountHandle>();
        }
    }
    handler = uio_getFileSystemHandler(fsType);
    log_add(
        log_Level::log_Info,
        b"uio_getFileSystemHandler %p\0".as_ptr() as *const ::core::ffi::c_char,
        handler,
    );
    if handler.is_null() {
        if !handle.is_null() {
            uqm_lua_checked!(uio_close(handle), ::core::ptr::null_mut());
        }
        *__errno_location() = 43 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    ((*handler).mount.is_some()
        || {
            __assert_fail(
                b"handler->mount != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                221 as ::core::ffi::c_int,
                b"uio_mountDir\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    pRoot = uqm_lua_checked!((*handler).mount.expect("non-null function pointer")(handle, flags), ::core::ptr::null_mut())
        as *mut uio_PRoot;
    if pRoot.is_null() {
        let mut savedErrno: ::core::ffi::c_int = 0;
        savedErrno = *__errno_location();
        if !handle.is_null() {
            uqm_lua_checked!(uio_close(handle), ::core::ptr::null_mut());
        }
        *__errno_location() = savedErrno;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    if !handle.is_null() {
        uqm_lua_checked!(uio_close(handle), ::core::ptr::null_mut());
    }
    let mut endDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut endInPath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut dirName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut mountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut mountTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut pRootHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    if *inPath.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        inPath = inPath.offset(1);
    }
    pRootHandle = uio_PRoot_getRootDirHandle(pRoot);
    uqm_lua_checked!(uio_walkPhysicalPath(
        pRootHandle,
        inPath,
        strlen(inPath),
        &raw mut endDirHandle,
        &raw mut endInPath,
    ), ::core::ptr::null_mut());
    if *endInPath as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        uqm_lua_checked!(uio_PDirHandle_unref(endDirHandle), ::core::ptr::null_mut());
        uqm_lua_checked!(uio_PRoot_unrefMount(pRoot), ::core::ptr::null_mut());
        *__errno_location() = 44 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    dirName = malloc((endInPath.offset_from(inPath) + 1isize) as size_t)
        as *mut ::core::ffi::c_char;
    memcpy(
        dirName as *mut ::core::ffi::c_void,
        inPath as *const ::core::ffi::c_void,
        endInPath.offset_from(inPath) as size_t,
    );
    *dirName.offset(endInPath.offset_from(inPath) as isize) = '\0'
        as ::core::ffi::c_char;
    mountInfo = uio_MountInfo_new(
        fsType,
        ::core::ptr::null_mut::<uio_MountTree>(),
        endDirHandle,
        dirName,
        autoMount,
        ::core::ptr::null_mut::<uio_MountHandle>(),
        flags,
    );
    uio_repositoryAddMount(
        destRep,
        mountInfo,
        uqm_lua_checked!(uio_MountLocation(
            (flags & (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ), ::core::ptr::null_mut()),
        relativeInfo,
    );
    mountTree = uio_mountTreeAddMountInfo(
        destRep,
        (*destRep).mountTree,
        mountInfo,
        mountPoint,
        uqm_lua_checked!(uio_MountLocation(
            (flags & (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ), ::core::ptr::null_mut()),
        relativeInfo,
    );
    (*mountInfo).mountTree = mountTree;
    (*mountInfo).mountHandle = uio_MountHandle_new(destRep, mountInfo);
    return (*mountInfo).mountHandle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_transplantDir(
    mut mountPoint: *const ::core::ffi::c_char,
    mut sourceDir: *mut uio_DirHandle,
    mut flags: ::core::ffi::c_int,
    mut relative: *mut uio_MountHandle,
) -> *mut uio_MountHandle {
    let mut relativeInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut numPDirHandles: ::core::ffi::c_int = 0;
    let mut pDirHandles: *mut *mut uio_PDirHandle = ::core::ptr::null_mut::<
        *mut uio_PDirHandle,
    >();
    let mut treeItems: *mut *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        *mut uio_MountTreeItem,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut handle: *mut uio_MountHandle = ::core::ptr::null_mut::<uio_MountHandle>();
    if flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
    {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    match flags & (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int {
        4 | 0 => {
            if !relative.is_null() {
                *__errno_location() = 28 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<uio_MountHandle>();
            }
            relativeInfo = ::core::ptr::null_mut::<uio_MountInfo>();
        }
        8 | 12 => {
            if relative.is_null() {
                *__errno_location() = 28 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<uio_MountHandle>();
            }
            relativeInfo = (*relative).mountInfo as *mut uio_MountInfo;
        }
        _ => {
            abort();
        }
    }
    if *mountPoint.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        mountPoint = mountPoint.offset(1);
    }
    if !validPathName(mountPoint, strlen(mountPoint)) {
        *__errno_location() = 28 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    if uqm_lua_checked!(uio_getPathPhysicalDirs(
        sourceDir,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        0 as size_t,
        &raw mut pDirHandles,
        &raw mut numPDirHandles,
        &raw mut treeItems,
    ), ::core::ptr::null_mut()) == -1 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    if numPDirHandles == 0 as ::core::ffi::c_int {
        *__errno_location() = 44 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_MountHandle>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < numPDirHandles {
        let mut pDirHandle: *mut uio_PDirHandle = *pDirHandles.offset(i as isize);
        let mut oldMountInfo: *mut uio_MountInfo = (**treeItems.offset(i as isize))
            .mountInfo;
        let mut rep: *mut uio_Repository = (*(*oldMountInfo).mountHandle).repository;
        let mut mountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
        let mut mountTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
        if !uio_mountInfoIsReadOnly(oldMountInfo) {
            i += 1;
        } else {
            mountInfo = uio_MountInfo_new(
                (*oldMountInfo).fsID,
                ::core::ptr::null_mut::<uio_MountTree>(),
                pDirHandle,
                strdup(b"\0".as_ptr() as *const ::core::ffi::c_char),
                (*oldMountInfo).autoMount,
                ::core::ptr::null_mut::<uio_MountHandle>(),
                flags,
            );
            uio_PDirHandle_ref(pDirHandle);
            uio_PRoot_refMount((*pDirHandle).pRoot);
            uio_repositoryAddMount(
                rep,
                mountInfo,
                uqm_lua_checked!(uio_MountLocation(
                    (flags & (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ), ::core::ptr::null_mut()),
                relativeInfo,
            );
            mountTree = uio_mountTreeAddMountInfo(
                rep,
                (*rep).mountTree,
                mountInfo,
                mountPoint,
                uqm_lua_checked!(uio_MountLocation(
                    (flags & (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ), ::core::ptr::null_mut()),
                relativeInfo,
            );
            (*mountInfo).mountTree = mountTree;
            (*mountInfo).mountHandle = uio_MountHandle_new(rep, mountInfo);
            handle = (*mountInfo).mountHandle;
            break;
        }
    }
    uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), ::core::ptr::null_mut());
    free(treeItems as *mut ::core::ffi::c_void);
    if handle.is_null() {
        *__errno_location() = 44 as ::core::ffi::c_int;
    }
    return handle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_unmountDir(
    mut mountHandle: *mut uio_MountHandle,
) -> ::core::ffi::c_int {
    let mut pRoot: *mut uio_PRoot = ::core::ptr::null_mut::<uio_PRoot>();
    pRoot = (*(*(*mountHandle).mountInfo).pDirHandle).pRoot;
    if (*pRoot).mountRef == 1 as ::core::ffi::c_int
        && (*pRoot).handleRef > 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Warning: File system to be unmounted still has file descriptors open. The file system will not be deallocated until these are all closed.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    uio_mountTreeRemoveMountInfo(
        (*mountHandle).repository,
        (*(*mountHandle).mountInfo).mountTree,
        (*mountHandle).mountInfo,
    );
    uio_repositoryRemoveMount((*mountHandle).repository, (*mountHandle).mountInfo);
    uqm_lua_checked!(uio_MountInfo_delete((*mountHandle).mountInfo), 0);
    uio_MountHandle_delete(mountHandle);
    uqm_lua_checked!(uio_PRoot_unrefMount(pRoot), 0);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_unmountAllDirs(
    mut repository: *mut uio_Repository,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = (*repository).numMounts;
    loop {
        let c2rust_fresh0 = i;
        i -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        uqm_lua_checked!(uio_unmountDir((**(*repository).mounts.offset(i as isize)).mountHandle), 0);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getMountFileSystemType(
    mut mountHandle: *mut uio_MountHandle,
) -> uio_FileSystemID {
    return (*(*mountHandle).mountInfo).fsID;
}
#[no_mangle]
pub unsafe extern "C" fn uio_close(mut handle: *mut uio_Handle) -> ::core::ffi::c_int {
    uqm_lua_checked!(uio_Handle_unref(handle), 0);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_rename(
    mut oldDir: *mut uio_DirHandle,
    mut oldPath: *const ::core::ffi::c_char,
    mut newDir: *mut uio_DirHandle,
    mut newPath: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut oldPReadDir: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut newPReadDir: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut newPWriteDir: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut oldReadMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<
        uio_MountInfo,
    >();
    let mut newReadMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<
        uio_MountInfo,
    >();
    let mut newWriteMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<
        uio_MountInfo,
    >();
    let mut oldName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut newName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut retVal: ::core::ffi::c_int = 0;
    if uqm_lua_checked!(uio_getPhysicalAccess(
        oldDir,
        oldPath,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut oldReadMountInfo,
        &raw mut oldPReadDir,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*mut uio_MountInfo>(),
        ::core::ptr::null_mut::<*mut uio_PDirHandle>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut oldName,
    ), 0) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(uio_getPhysicalAccess(
        newDir,
        newPath,
        0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        &raw mut newReadMountInfo,
        &raw mut newPReadDir,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut newWriteMountInfo,
        &raw mut newPWriteDir,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut newName,
    ), 0) == -1 as ::core::ffi::c_int
    {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        uqm_lua_checked!(uio_PDirHandle_unref(oldPReadDir), 0);
        free(oldName as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    if oldReadMountInfo != newWriteMountInfo {
        uqm_lua_checked!(uio_PDirHandle_unref(oldPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPWriteDir), 0);
        free(oldName as *mut ::core::ffi::c_void);
        free(newName as *mut ::core::ffi::c_void);
        *__errno_location() = 75 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if uio_mountInfoIsReadOnly(oldReadMountInfo) {
        uqm_lua_checked!(uio_PDirHandle_unref(oldPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPWriteDir), 0);
        free(oldName as *mut ::core::ffi::c_void);
        free(newName as *mut ::core::ffi::c_void);
        *__errno_location() = 69 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if (*(*(*(*oldReadMountInfo).pDirHandle).pRoot).handler).rename.is_none() {
        uqm_lua_checked!(uio_PDirHandle_unref(oldPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPWriteDir), 0);
        free(oldName as *mut ::core::ffi::c_void);
        free(newName as *mut ::core::ffi::c_void);
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    retVal = uqm_lua_checked!((*(*(*(*oldReadMountInfo).pDirHandle).pRoot).handler)
        .rename
        .expect(
            "non-null function pointer",
        )(oldPReadDir, oldName, newPWriteDir, newName), 0);
    if retVal == -1 as ::core::ffi::c_int {
        let mut savedErrno_0: ::core::ffi::c_int = *__errno_location();
        uqm_lua_checked!(uio_PDirHandle_unref(oldPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPReadDir), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(newPWriteDir), 0);
        free(oldName as *mut ::core::ffi::c_void);
        free(newName as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno_0;
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_PDirHandle_unref(oldPReadDir), 0);
    uqm_lua_checked!(uio_PDirHandle_unref(newPReadDir), 0);
    uqm_lua_checked!(uio_PDirHandle_unref(newPWriteDir), 0);
    free(oldName as *mut ::core::ffi::c_void);
    free(newName as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_access(
    mut dir: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *__errno_location() = 52 as ::core::ffi::c_int;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_fstat(
    mut handle: *mut uio_Handle,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    if (*(*(*handle).root).handler).fstat.is_none() {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    return uqm_lua_checked!((*(*(*handle).root).handler)
        .fstat
        .expect("non-null function pointer")(handle, statBuf), 0);
}
#[no_mangle]
pub unsafe extern "C" fn uio_stat(
    mut dir: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    let mut pReadDir: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut readMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = 0;
    if uqm_lua_checked!(uio_getPhysicalAccess(
        dir,
        path,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut readMountInfo,
        &raw mut pReadDir,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*mut uio_MountInfo>(),
        ::core::ptr::null_mut::<*mut uio_PDirHandle>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut name,
    ), 0) == -1 as ::core::ffi::c_int
    {
        if uqm_lua_checked!(uio_statDir(dir, path, statBuf), 0) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    if (*(*(*pReadDir).pRoot).handler).stat.is_none() {
        uqm_lua_checked!(uio_PDirHandle_unref(pReadDir), 0);
        free(name as *mut ::core::ffi::c_void);
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    result = uqm_lua_checked!((*(*(*pReadDir).pRoot).handler)
        .stat
        .expect("non-null function pointer")(pReadDir, name, statBuf), 0);
    if result == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        uqm_lua_checked!(uio_PDirHandle_unref(pReadDir), 0);
        free(name as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_PDirHandle_unref(pReadDir), 0);
    free(name as *mut ::core::ffi::c_void);
    return result;
}
unsafe extern "C" fn uio_statDir(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    let mut numPDirHandles: ::core::ffi::c_int = 0;
    let mut pDirHandles: *mut *mut uio_PDirHandle = ::core::ptr::null_mut::<
        *mut uio_PDirHandle,
    >();
    if uqm_lua_checked!(uio_getPathPhysicalDirs(
        dirHandle,
        path,
        strlen(path),
        &raw mut pDirHandles,
        &raw mut numPDirHandles,
        ::core::ptr::null_mut::<*mut *mut uio_MountTreeItem>(),
    ), 0) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if numPDirHandles == 0 as ::core::ffi::c_int {
        *__errno_location() = 44 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(uio_statOneDir(*pDirHandles.offset(0isize), statBuf), 0) == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), 0);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_statOneDir(
    mut pDirHandle: *mut uio_PDirHandle,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    if (*(*(*pDirHandle).pRoot).handler).stat.is_none() {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    return uqm_lua_checked!((*(*(*pDirHandle).pRoot).handler)
        .stat
        .expect(
            "non-null function pointer",
        )(pDirHandle, b".\0".as_ptr() as *const ::core::ffi::c_char, statBuf), 0);
}
#[no_mangle]
pub unsafe extern "C" fn uio_mkdir(
    mut dir: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    let mut pReadDir: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut pWriteDir: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut readMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut writeMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<
        uio_MountInfo,
    >();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut newDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    if uqm_lua_checked!(uio_getPhysicalAccess(
        dir,
        path,
        0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut readMountInfo,
        &raw mut pReadDir,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut writeMountInfo,
        &raw mut pWriteDir,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut name,
    ), 0) == -1 as ::core::ffi::c_int
    {
        if *__errno_location() == 31 as ::core::ffi::c_int {
            *__errno_location() = 20 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_PDirHandle_unref(pReadDir), 0);
    if (*(*(*pWriteDir).pRoot).handler).mkdir.is_none() {
        free(name as *mut ::core::ffi::c_void);
        uqm_lua_checked!(uio_PDirHandle_unref(pWriteDir), 0);
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    newDirHandle = uqm_lua_checked!((*(*(*pWriteDir).pRoot).handler)
        .mkdir
        .expect("non-null function pointer")(pWriteDir, name, mode), 0);
    if newDirHandle.is_null() {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(name as *mut ::core::ffi::c_void);
        uqm_lua_checked!(uio_PDirHandle_unref(pWriteDir), 0);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_PDirHandle_unref(pWriteDir), 0);
    uqm_lua_checked!(uio_PDirHandle_unref(newDirHandle), 0);
    free(name as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_open(
    mut dir: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mode: mode_t,
) -> *mut uio_Handle {
    let mut readPDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut writePDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut readMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut writeMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<
        uio_MountInfo,
    >();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut handle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    if uqm_lua_checked!(uio_getPhysicalAccess(
        dir,
        path,
        flags,
        0 as ::core::ffi::c_int,
        &raw mut readMountInfo,
        &raw mut readPDirHandle,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut writeMountInfo,
        &raw mut writePDirHandle,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut name,
    ), ::core::ptr::null_mut()) == -1 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    if flags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        pDirHandle = readPDirHandle;
    } else if readPDirHandle == writePDirHandle {
        uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), ::core::ptr::null_mut());
        pDirHandle = readPDirHandle;
    } else {
        let mut entry: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
            uio_PDirEntryHandle,
        >();
        entry = uqm_lua_checked!(uio_getPDirEntryHandle(readPDirHandle, name), ::core::ptr::null_mut());
        if !entry.is_null() {
            uqm_lua_checked!(uio_PDirEntryHandle_unref(entry), ::core::ptr::null_mut());
            if flags & 0o100 as ::core::ffi::c_int == 0o100 as ::core::ffi::c_int
                && flags & 0o200 as ::core::ffi::c_int == 0o200 as ::core::ffi::c_int
            {
                free(name as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), ::core::ptr::null_mut());
                uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), ::core::ptr::null_mut());
                *__errno_location() = 20 as ::core::ffi::c_int;
                log_add(
                    log_Level::log_Info,
                    b"uio_open: O_CREAT | O_EXCL: file already exists '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    name,
                );
                return ::core::ptr::null_mut::<uio_Handle>();
            }
            if flags & 0o1000 as ::core::ffi::c_int == 0o1000 as ::core::ffi::c_int {
                flags &= !(0o1000 as ::core::ffi::c_int);
            } else if uqm_lua_checked!(uio_copyFilePhysical(readPDirHandle, name, writePDirHandle, name), ::core::ptr::null_mut())
                == -1 as ::core::ffi::c_int
            {
                let mut savedErrno: ::core::ffi::c_int = *__errno_location();
                free(name as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), ::core::ptr::null_mut());
                uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), ::core::ptr::null_mut());
                *__errno_location() = savedErrno;
                log_add(
                    log_Level::log_Info,
                    b"uio_open: uio_copyFilePhysical failed '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    name,
                );
                return ::core::ptr::null_mut::<uio_Handle>();
            }
        } else if flags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
            || flags & 0o100 as ::core::ffi::c_int != 0o100 as ::core::ffi::c_int
        {
            free(name as *mut ::core::ffi::c_void);
            uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), ::core::ptr::null_mut());
            uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), ::core::ptr::null_mut());
            *__errno_location() = 44 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<uio_Handle>();
        }
        uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), ::core::ptr::null_mut());
        pDirHandle = writePDirHandle;
    }
    handle = uqm_lua_checked!((*(*(*pDirHandle).pRoot).handler)
        .open
        .expect("non-null function pointer")(pDirHandle, name, flags, mode), ::core::ptr::null_mut());
    if handle.is_null() {
        let mut savedErrno_0: ::core::ffi::c_int = *__errno_location();
        log_add(
            log_Level::log_Info,
            b"uio_open: open file failed '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        free(name as *mut ::core::ffi::c_void);
        uqm_lua_checked!(uio_PDirHandle_unref(pDirHandle), ::core::ptr::null_mut());
        *__errno_location() = savedErrno_0;
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    free(name as *mut ::core::ffi::c_void);
    uqm_lua_checked!(uio_PDirHandle_unref(pDirHandle), ::core::ptr::null_mut());
    return handle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_openDir(
    mut repository: *mut uio_Repository,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_DirHandle {
    let mut dirHandle: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let rootStr: *const ::core::ffi::c_char = b"\0".as_ptr()
        as *const ::core::ffi::c_char;
    dirHandle = uio_DirHandle_new(
        repository,
        unconst(rootStr as *const ::core::ffi::c_void) as *mut ::core::ffi::c_char,
        unconst(rootStr as *const ::core::ffi::c_void) as *mut ::core::ffi::c_char,
    );
    if uqm_lua_checked!(uio_verifyPath(dirHandle, path, &raw mut (*dirHandle).path), ::core::ptr::null_mut())
        == -1 as ::core::ffi::c_int
    {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        uio_DirHandle_free(dirHandle);
        *__errno_location() = savedErrno;
        return ::core::ptr::null_mut::<uio_DirHandle>();
    }
    (*dirHandle).rootEnd = (*dirHandle).path;
    if flags & 1 as ::core::ffi::c_int != 0 {
        (*dirHandle).rootEnd = (*dirHandle)
            .rootEnd
            .offset(strlen((*dirHandle).path) as isize);
    }
    return dirHandle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_openDirRelative(
    mut base: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_DirHandle {
    let mut dirHandle: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut newPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if uqm_lua_checked!(uio_verifyPath(base, path, &raw mut newPath), ::core::ptr::null_mut()) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<uio_DirHandle>();
    }
    if flags & 1 as ::core::ffi::c_int != 0 {
        dirHandle = uio_DirHandle_new(
            (*base).repository,
            newPath,
            newPath.offset(strlen(newPath) as isize),
        );
    } else {
        dirHandle = uio_DirHandle_new(
            (*base).repository,
            newPath,
            newPath.offset((*base).rootEnd.offset_from((*base).path) as isize),
        );
    }
    return dirHandle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_closeDir(
    mut dirHandle: *mut uio_DirHandle,
) -> ::core::ffi::c_int {
    uio_DirHandle_unref(dirHandle);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_read(
    mut handle: *mut uio_Handle,
    mut buf: *mut ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    return uqm_lua_checked!((*(*(*handle).root).handler)
        .read
        .expect("non-null function pointer")(handle, buf, count), 0);
}
#[no_mangle]
pub unsafe extern "C" fn uio_rmdir(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut numPDirHandles: ::core::ffi::c_int = 0;
    let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut pDirHandles: *mut *mut uio_PDirHandle = ::core::ptr::null_mut::<
        *mut uio_PDirHandle,
    >();
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut entry: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
        uio_PDirEntryHandle,
    >();
    let mut items: *mut *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        *mut uio_MountTreeItem,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut numDeleted: ::core::ffi::c_int = 0;
    pathEnd = strrchr(path, '/' as ::core::ffi::c_int);
    if pathEnd.is_null() {
        pathEnd = path;
        name = path;
    } else {
        name = pathEnd.offset(1 as ::core::ffi::c_int as isize);
    }
    if uqm_lua_checked!(uio_getPathPhysicalDirs(
        dirHandle,
        path,
        pathEnd.offset_from(path) as size_t,
        &raw mut pDirHandles,
        &raw mut numPDirHandles,
        &raw mut items,
    ), 0) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    entry = ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    '_err: {
        if *name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            *__errno_location() = 44 as ::core::ffi::c_int;
        } else {
            numDeleted = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < numPDirHandles {
                pDirHandle = *pDirHandles.offset(i as isize);
                entry = uqm_lua_checked!(uio_getPDirEntryHandle(pDirHandle, name), 0);
                if !entry.is_null() {
                    if !uio_PDirEntryHandle_isDir(entry) {
                        *__errno_location() = 54 as ::core::ffi::c_int;
                        break '_err;
                    } else if uio_mountInfoIsReadOnly(
                        (**items.offset(i as isize)).mountInfo,
                    ) {
                        *__errno_location() = 69 as ::core::ffi::c_int;
                        break '_err;
                    } else if (*(*(*pDirHandle).pRoot).handler).rmdir.is_none() {
                        *__errno_location() = 52 as ::core::ffi::c_int;
                        break '_err;
                    } else {
                        if uqm_lua_checked!((*(*(*pDirHandle).pRoot).handler)
                            .rmdir
                            .expect("non-null function pointer")(pDirHandle, name), 0)
                            == -1 as ::core::ffi::c_int
                        {
                            break '_err;
                        }
                        numDeleted += 1;
                        uqm_lua_checked!(uio_PDirEntryHandle_unref(entry), 0);
                    }
                }
                i += 1;
            }
            entry = ::core::ptr::null_mut::<uio_PDirEntryHandle>();
            if numDeleted == 0 as ::core::ffi::c_int {
                *__errno_location() = 44 as ::core::ffi::c_int;
            } else {
                uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), 0);
                free(items as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    let mut savedErrno: ::core::ffi::c_int = *__errno_location();
    uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), 0);
    free(items as *mut ::core::ffi::c_void);
    if !entry.is_null() {
        uqm_lua_checked!(uio_PDirEntryHandle_unref(entry), 0);
    }
    *__errno_location() = savedErrno;
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_PDirHandles_delete(
    mut pDirHandles: *mut *mut uio_PDirHandle,
    mut numPDirHandles: ::core::ffi::c_int,
) {
    loop {
        let c2rust_fresh1 = numPDirHandles;
        numPDirHandles -= 1;
        if c2rust_fresh1 == 0 {
            break;
        }
        uqm_lua_checked!(uio_PDirHandle_unref(*pDirHandles.offset(numPDirHandles as isize)), ());
    }
    free(pDirHandles as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_lseek(
    mut handle: *mut uio_Handle,
    mut offset: off_t,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*(*(*handle).root).handler).seek.is_none() {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    return uqm_lua_checked!((*(*(*handle).root).handler)
        .seek
        .expect("non-null function pointer")(handle, offset, whence), 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_write(
    mut handle: *mut uio_Handle,
    mut buf: *const ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    if (*(*(*handle).root).handler).write.is_none() {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ssize_t;
    }
    return uqm_lua_checked!((*(*(*handle).root).handler)
        .write
        .expect("non-null function pointer")(handle, buf, count), 0);
}
#[no_mangle]
pub unsafe extern "C" fn uio_unlink(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut numPDirHandles: ::core::ffi::c_int = 0;
    let mut pDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    let mut pDirHandles: *mut *mut uio_PDirHandle = ::core::ptr::null_mut::<
        *mut uio_PDirHandle,
    >();
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut entry: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
        uio_PDirEntryHandle,
    >();
    let mut items: *mut *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        *mut uio_MountTreeItem,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut numDeleted: ::core::ffi::c_int = 0;
    pathEnd = strrchr(path, '/' as ::core::ffi::c_int);
    if pathEnd.is_null() {
        pathEnd = path;
        name = path;
    } else {
        name = pathEnd.offset(1 as ::core::ffi::c_int as isize);
    }
    if uqm_lua_checked!(uio_getPathPhysicalDirs(
        dirHandle,
        path,
        pathEnd.offset_from(path) as size_t,
        &raw mut pDirHandles,
        &raw mut numPDirHandles,
        &raw mut items,
    ), 0) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    entry = ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    '_err: {
        if *name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            *__errno_location() = 44 as ::core::ffi::c_int;
        } else {
            numDeleted = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < numPDirHandles {
                pDirHandle = *pDirHandles.offset(i as isize);
                entry = uqm_lua_checked!(uio_getPDirEntryHandle(pDirHandle, name), 0);
                if !entry.is_null() {
                    if uio_PDirEntryHandle_isDir(entry) {
                        *__errno_location() = 31 as ::core::ffi::c_int;
                        break '_err;
                    } else if uio_mountInfoIsReadOnly(
                        (**items.offset(i as isize)).mountInfo,
                    ) {
                        *__errno_location() = 69 as ::core::ffi::c_int;
                        break '_err;
                    } else if (*(*(*pDirHandle).pRoot).handler).unlink.is_none() {
                        *__errno_location() = 52 as ::core::ffi::c_int;
                        break '_err;
                    } else {
                        if uqm_lua_checked!((*(*(*pDirHandle).pRoot).handler)
                            .unlink
                            .expect("non-null function pointer")(pDirHandle, name), 0)
                            == -1 as ::core::ffi::c_int
                        {
                            break '_err;
                        }
                        numDeleted += 1;
                        uqm_lua_checked!(uio_PDirEntryHandle_unref(entry), 0);
                    }
                }
                i += 1;
            }
            entry = ::core::ptr::null_mut::<uio_PDirEntryHandle>();
            if numDeleted == 0 as ::core::ffi::c_int {
                *__errno_location() = 44 as ::core::ffi::c_int;
            } else {
                uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), 0);
                free(items as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    let mut savedErrno: ::core::ffi::c_int = *__errno_location();
    uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), 0);
    free(items as *mut ::core::ffi::c_void);
    if !entry.is_null() {
        uqm_lua_checked!(uio_PDirEntryHandle_unref(entry), 0);
    }
    *__errno_location() = savedErrno;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getFileLocation(
    mut dir: *mut uio_DirHandle,
    mut inPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mountHandle: *mut *mut uio_MountHandle,
    mut outPath: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut readPDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut writePDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut readMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut writeMountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<
        uio_MountInfo,
    >();
    let mut mountInfo: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut readPRootPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut writePRootPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pRootPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if uqm_lua_checked!(uio_getPhysicalAccess(
        dir,
        inPath,
        flags,
        0 as ::core::ffi::c_int,
        &raw mut readMountInfo,
        &raw mut readPDirHandle,
        &raw mut readPRootPath,
        &raw mut writeMountInfo,
        &raw mut writePDirHandle,
        &raw mut writePRootPath,
        &raw mut name,
    ), 0) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if flags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), 0);
        pRootPath = readPRootPath;
        mountInfo = readMountInfo;
    } else if readPDirHandle == writePDirHandle {
        uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), 0);
        pRootPath = readPRootPath;
        mountInfo = readMountInfo;
        free(writePRootPath as *mut ::core::ffi::c_void);
    } else {
        let mut entry: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
            uio_PDirEntryHandle,
        >();
        entry = uqm_lua_checked!(uio_getPDirEntryHandle(readPDirHandle, name), 0);
        if !entry.is_null() {
            uqm_lua_checked!(uio_PDirEntryHandle_unref(entry), 0);
            if flags & 0o100 as ::core::ffi::c_int == 0o100 as ::core::ffi::c_int
                && flags & 0o200 as ::core::ffi::c_int == 0o200 as ::core::ffi::c_int
            {
                free(name as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), 0);
                free(readPRootPath as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), 0);
                free(writePRootPath as *mut ::core::ffi::c_void);
                *__errno_location() = 20 as ::core::ffi::c_int;
                return -1 as ::core::ffi::c_int;
            }
            if flags & 0o1000 as ::core::ffi::c_int == 0o1000 as ::core::ffi::c_int {
                flags &= !(0o1000 as ::core::ffi::c_int);
            } else if uqm_lua_checked!(uio_copyFilePhysical(readPDirHandle, name, writePDirHandle, name), 0)
                == -1 as ::core::ffi::c_int
            {
                let mut savedErrno: ::core::ffi::c_int = *__errno_location();
                free(name as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), 0);
                free(readPRootPath as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), 0);
                free(writePRootPath as *mut ::core::ffi::c_void);
                *__errno_location() = savedErrno;
                return -1 as ::core::ffi::c_int;
            }
        } else if flags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
            || flags & 0o100 as ::core::ffi::c_int != 0o100 as ::core::ffi::c_int
        {
            free(name as *mut ::core::ffi::c_void);
            uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), 0);
            free(readPRootPath as *mut ::core::ffi::c_void);
            uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), 0);
            free(writePRootPath as *mut ::core::ffi::c_void);
            *__errno_location() = 44 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        uqm_lua_checked!(uio_PDirHandle_unref(readPDirHandle), 0);
        uqm_lua_checked!(uio_PDirHandle_unref(writePDirHandle), 0);
        pRootPath = writePRootPath;
        mountInfo = writeMountInfo;
        free(readPRootPath as *mut ::core::ffi::c_void);
    }
    free(name as *mut ::core::ffi::c_void);
    *mountHandle = (*mountInfo).mountHandle;
    *outPath = pRootPath;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getDirList(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut pattern: *const ::core::ffi::c_char,
    mut matchType: match_MatchType,
) -> *mut uio_DirList {
    let mut numPDirHandles: ::core::ffi::c_int = 0;
    let mut pDirHandles: *mut *mut uio_PDirHandle = ::core::ptr::null_mut::<
        *mut uio_PDirHandle,
    >();
    let mut result: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    if uqm_lua_checked!(uio_getPathPhysicalDirs(
        dirHandle,
        path,
        strlen(path),
        &raw mut pDirHandles,
        &raw mut numPDirHandles,
        ::core::ptr::null_mut::<*mut *mut uio_MountTreeItem>(),
    ), ::core::ptr::null_mut()) == -1 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<uio_DirList>();
    }
    if numPDirHandles == 0 as ::core::ffi::c_int {
        (pDirHandles.is_null()
            || {
                __assert_fail(
                    b"pDirHandles == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/libs/uio/io.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1298 as ::core::ffi::c_int,
                    b"uio_getDirList\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        return uio_DirList_new(
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    result = uqm_lua_checked!(uio_getDirListMulti(pDirHandles, numPDirHandles, pattern, matchType), ::core::ptr::null_mut());
    let mut savedErrno: ::core::ffi::c_int = 0;
    savedErrno = *__errno_location();
    uqm_lua_checked!(uio_PDirHandles_delete(pDirHandles, numPDirHandles), ::core::ptr::null_mut());
    *__errno_location() = savedErrno;
    return result;
}
unsafe extern "C" fn uio_filterDoubleNames(
    mut names: *const *const ::core::ffi::c_char,
    mut numNames: ::core::ffi::c_int,
    mut newNames: *mut *const ::core::ffi::c_char,
    mut numNewNames: *mut ::core::ffi::c_int,
) {
    let mut endNames: *const *const ::core::ffi::c_char = ::core::ptr::null::<
        *const ::core::ffi::c_char,
    >();
    let mut prevName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut newNum: ::core::ffi::c_int = 0;
    if numNames == 0 as ::core::ffi::c_int {
        *numNewNames = 0 as ::core::ffi::c_int;
        return;
    }
    endNames = names.offset(numNames as isize);
    prevName = *names;
    *newNames = *names;
    newNames = newNames.offset(1);
    names = names.offset(1);
    newNum = 1 as ::core::ffi::c_int;
    while names < endNames {
        if strcmp(prevName, *names) != 0 as ::core::ffi::c_int {
            *newNames = *names;
            newNum += 1;
            prevName = *names;
            newNames = newNames.offset(1);
        }
        names = names.offset(1);
    }
    *numNewNames = newNum;
}
unsafe extern "C" fn uio_getDirListMulti(
    mut pDirHandles: *mut *mut uio_PDirHandle,
    mut numPDirHandles: ::core::ffi::c_int,
    mut pattern: *const ::core::ffi::c_char,
    mut matchType: match_MatchType,
) -> *mut uio_DirList {
    let mut pDirI: ::core::ffi::c_int = 0;
    let mut links: *mut *mut uio_DirBufferLink = ::core::ptr::null_mut::<
        *mut uio_DirBufferLink,
    >();
    let mut linkPtr: *mut uio_DirBufferLink = ::core::ptr::null_mut::<
        uio_DirBufferLink,
    >();
    let mut numNames: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut totalNumNames: ::core::ffi::c_int = 0;
    let mut bigNameBuffer: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
        *const ::core::ffi::c_char,
    >();
    let mut destPtr: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
        *const ::core::ffi::c_char,
    >();
    let mut result: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    let mut matchResult: match_Result = 0;
    let mut matchContext: *mut match_MatchContext = ::core::ptr::null_mut::<
        match_MatchContext,
    >();
    matchResult = match_prepareContext(pattern, &raw mut matchContext, matchType);
    if matchResult != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Error compiling match function: %s.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            match_errorString(matchContext, matchResult),
        );
        match_freeContext(matchContext);
        *__errno_location() = 29 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_DirList>();
    }
    totalNumNames = 0 as ::core::ffi::c_int;
    links = malloc(
        (numPDirHandles as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut uio_DirBufferLink>()),
    ) as *mut *mut uio_DirBufferLink;
    numNames = malloc(
        (numPDirHandles as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
    ) as *mut ::core::ffi::c_int;
    pDirI = 0 as ::core::ffi::c_int;
    while pDirI < numPDirHandles {
        uqm_lua_checked!(uio_collectDirEntries(
            *pDirHandles.offset(pDirI as isize),
            links.offset(pDirI as isize),
            numNames.offset(pDirI as isize),
        ), ::core::ptr::null_mut());
        totalNumNames += *numNames.offset(pDirI as isize);
        pDirI += 1;
    }
    bigNameBuffer = malloc(
        (totalNumNames as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut uio_DirBufferLink>()),
    ) as *mut *const ::core::ffi::c_char;
    destPtr = bigNameBuffer;
    totalNumNames = 0 as ::core::ffi::c_int;
    pDirI = 0 as ::core::ffi::c_int;
    while pDirI < numPDirHandles {
        linkPtr = *links.offset(pDirI as isize);
        while !linkPtr.is_null() {
            let mut numNewNames: ::core::ffi::c_int = 0;
            uio_filterNames(
                (*linkPtr).buffer as *const *const ::core::ffi::c_char,
                (*linkPtr).numEntries,
                destPtr,
                &raw mut numNewNames,
                matchContext,
            );
            totalNumNames += numNewNames;
            destPtr = destPtr.offset(numNewNames as isize);
            linkPtr = (*linkPtr).next as *mut uio_DirBufferLink;
        }
        pDirI += 1;
    }
    match_freeContext(matchContext);
    qsort(
        bigNameBuffer as *mut ::core::ffi::c_void,
        totalNumNames as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const *const ::core::ffi::c_char,
                    *const *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        >(
            Some(
                strPtrCmp
                    as unsafe extern "C" fn(
                        *const *const ::core::ffi::c_char,
                        *const *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    if numPDirHandles > 1 as ::core::ffi::c_int {
        uio_filterDoubleNames(
            bigNameBuffer,
            totalNumNames,
            bigNameBuffer,
            &raw mut totalNumNames,
        );
    }
    bigNameBuffer = realloc(
        bigNameBuffer as *mut ::core::ffi::c_void,
        (totalNumNames as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    ) as *mut *const ::core::ffi::c_char;
    result = uio_makeDirList(bigNameBuffer, bigNameBuffer, totalNumNames);
    pDirI = 0 as ::core::ffi::c_int;
    while pDirI < numPDirHandles {
        uio_DirBufferChain_free(*links.offset(pDirI as isize));
        pDirI += 1;
    }
    free(links as *mut ::core::ffi::c_void);
    free(numNames as *mut ::core::ffi::c_void);
    return result;
}
unsafe extern "C" fn uio_makeDirList(
    mut newNames: *mut *const ::core::ffi::c_char,
    mut names: *const *const ::core::ffi::c_char,
    mut numNames: ::core::ffi::c_int,
) -> *mut uio_DirList {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut totLen: size_t = 0;
    let mut bufPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    if newNames.is_null() {
        newNames = malloc(
            (numNames as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        ) as *mut *const ::core::ffi::c_char;
    }
    totLen = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < numNames {
        totLen = totLen.wrapping_add(strlen(*names.offset(i as isize)));
        i += 1;
    }
    totLen = totLen.wrapping_add(numNames as size_t);
    result = uio_DirList_new(
        newNames,
        numNames,
        malloc(totLen) as *mut ::core::ffi::c_char,
    );
    bufPtr = (*result).buffer;
    i = 0 as ::core::ffi::c_int;
    while i < numNames {
        len = strlen(*names.offset(i as isize)).wrapping_add(1 as size_t);
        memcpy(
            bufPtr as *mut ::core::ffi::c_void,
            *names.offset(i as isize) as *const ::core::ffi::c_void,
            len,
        );
        *newNames.offset(i as isize) = bufPtr;
        bufPtr = bufPtr.offset(len as isize);
        i += 1;
    }
    return result;
}
unsafe extern "C" fn uio_collectDirEntries(
    mut pDirHandle: *mut uio_PDirHandle,
    mut linkPtr: *mut *mut uio_DirBufferLink,
    mut numEntries: *mut ::core::ffi::c_int,
) {
    let mut entriesContext: *mut uio_EntriesContext = ::core::ptr::null_mut::<
        uio_EntriesContext,
    >();
    let mut linkEndPtr: *mut *mut uio_DirBufferLink = ::core::ptr::null_mut::<
        *mut uio_DirBufferLink,
    >();
    let mut numRead: ::core::ffi::c_int = 0;
    let mut totalEntries: ::core::ffi::c_int = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    entriesContext = uqm_lua_checked!(uio_openEntriesPhysical(pDirHandle), ());
    if entriesContext.is_null() {
        fprintf(
            stderr,
            b"Error: uio_openEntriesPhysical() failed: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        *linkPtr = ::core::ptr::null_mut::<uio_DirBufferLink>();
        *numEntries = 0 as ::core::ffi::c_int;
        return;
    }
    linkEndPtr = linkPtr;
    totalEntries = 0 as ::core::ffi::c_int;
    loop {
        *linkEndPtr = malloc(::core::mem::size_of::<uio_DirBufferLink>())
            as *mut uio_DirBufferLink;
        buffer = malloc(2048 as size_t) as *mut ::core::ffi::c_char;
        (**linkEndPtr).buffer = buffer;
        numRead = uqm_lua_checked!(uio_readEntriesPhysical(entriesContext, buffer, 2048 as size_t), ());
        if numRead == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"Warning: uio_DIR_BUFFER_SIZE is too small to hold a certain large entry on its own!\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            uio_DirBufferLink_free(*linkEndPtr);
            break;
        } else {
            totalEntries += numRead;
            (**linkEndPtr).numEntries = numRead;
            if (*(buffer as *mut *mut ::core::ffi::c_char)
                .offset((numRead - 1 as ::core::ffi::c_int) as isize))
                .is_null()
            {
                (**linkEndPtr).numEntries -= 1;
                totalEntries -= 1;
                linkEndPtr = &raw mut (**linkEndPtr).next as *mut *mut uio_DirBufferLink;
                break;
            } else {
                linkEndPtr = &raw mut (**linkEndPtr).next as *mut *mut uio_DirBufferLink;
            }
        }
    }
    *linkEndPtr = ::core::ptr::null_mut::<uio_DirBufferLink>();
    uqm_lua_checked!(uio_closeEntriesPhysical(entriesContext), ());
    *numEntries = totalEntries;
}
unsafe extern "C" fn uio_filterNames(
    mut names: *const *const ::core::ffi::c_char,
    mut numNames: ::core::ffi::c_int,
    mut newNames: *mut *const ::core::ffi::c_char,
    mut numNewNames: *mut ::core::ffi::c_int,
    mut matchContext: *mut match_MatchContext,
) {
    let mut newNum: ::core::ffi::c_int = 0;
    let mut namesEnd: *const *const ::core::ffi::c_char = ::core::ptr::null::<
        *const ::core::ffi::c_char,
    >();
    let mut matchResult: match_Result = 0;
    newNum = 0 as ::core::ffi::c_int;
    namesEnd = names.offset(numNames as isize);
    while names < namesEnd {
        matchResult = match_matchPattern(matchContext, *names);
        if matchResult == 1 as ::core::ffi::c_int {
            *newNames = *names;
            newNames = newNames.offset(1);
            newNum += 1;
        } else if matchResult != 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"Error trying to match pattern: %s.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                match_errorString(matchContext, matchResult),
            );
        }
        names = names.offset(1);
    }
    *numNewNames = newNum;
}
unsafe extern "C" fn strPtrCmp(
    mut ptr1: *const *const ::core::ffi::c_char,
    mut ptr2: *const *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strcmp(*ptr1, *ptr2);
}
unsafe extern "C" fn uio_openEntriesPhysical(
    mut dirHandle: *mut uio_PDirHandle,
) -> *mut uio_EntriesContext {
    let mut native: *mut uio_NativeEntriesContext = ::core::ptr::null_mut::<
        uio_NativeEntriesContext,
    >();
    let mut pRoot: *mut uio_PRoot = ::core::ptr::null_mut::<uio_PRoot>();
    pRoot = (*dirHandle).pRoot;
    ((*(*pRoot).handler).openEntries.is_some()
        || {
            __assert_fail(
                b"pRoot->handler->openEntries != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1555 as ::core::ffi::c_int,
                b"uio_openEntriesPhysical\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    native = uqm_lua_checked!((*(*pRoot).handler)
        .openEntries
        .expect("non-null function pointer")(dirHandle), ::core::ptr::null_mut()) as *mut uio_NativeEntriesContext;
    if native.is_null() {
        return ::core::ptr::null_mut::<uio_EntriesContext>();
    }
    uio_PRoot_refHandle(pRoot);
    return uio_EntriesContext_new(pRoot, native);
}
unsafe extern "C" fn uio_readEntriesPhysical(
    mut iterator: *mut uio_EntriesContext,
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    ((*(*(*iterator).pRoot).handler).readEntries.is_some()
        || {
            __assert_fail(
                b"iterator->pRoot->handler->readEntries != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1565 as ::core::ffi::c_int,
                b"uio_readEntriesPhysical\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return uqm_lua_checked!((*(*(*iterator).pRoot).handler)
        .readEntries
        .expect("non-null function pointer")(&raw mut (*iterator).native, buf, len), 0);
}
unsafe extern "C" fn uio_closeEntriesPhysical(mut iterator: *mut uio_EntriesContext) {
    ((*(*(*iterator).pRoot).handler).closeEntries.is_some()
        || {
            __assert_fail(
                b"iterator->pRoot->handler->closeEntries != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1571 as ::core::ffi::c_int,
                b"uio_closeEntriesPhysical\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*(*iterator).pRoot).handler)
        .closeEntries
        .expect("non-null function pointer")((*iterator).native), ());
    uqm_lua_checked!(uio_PRoot_unrefHandle((*iterator).pRoot), ());
    uio_EntriesContext_delete(iterator);
}
unsafe extern "C" fn uio_EntriesContext_new(
    mut pRoot: *mut uio_PRoot,
    mut native: *mut uio_NativeEntriesContext,
) -> *mut uio_EntriesContext {
    let mut result: *mut uio_EntriesContext = ::core::ptr::null_mut::<
        uio_EntriesContext,
    >();
    result = uio_EntriesContext_alloc();
    (*result).pRoot = pRoot;
    (*result).native = native as uio_NativeEntriesContext;
    return result;
}
#[inline]
unsafe extern "C" fn uio_EntriesContext_alloc() -> *mut uio_EntriesContext {
    return malloc(::core::mem::size_of::<uio_EntriesContext>())
        as *mut uio_EntriesContext;
}
#[inline]
unsafe extern "C" fn uio_EntriesContext_delete(
    mut entriesContext: *mut uio_EntriesContext,
) {
    uio_EntriesContext_free(entriesContext);
}
#[inline]
unsafe extern "C" fn uio_EntriesContext_free(
    mut entriesContext: *mut uio_EntriesContext,
) {
    free(entriesContext as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn uio_DirBufferLink_free(mut dirBufferLink: *mut uio_DirBufferLink) {
    free((*dirBufferLink).buffer as *mut ::core::ffi::c_void);
    free(dirBufferLink as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn uio_DirBufferChain_free(mut dirBufferLink: *mut uio_DirBufferLink) {
    let mut next: *mut uio_DirBufferLink = ::core::ptr::null_mut::<uio_DirBufferLink>();
    while !dirBufferLink.is_null() {
        next = (*dirBufferLink).next as *mut uio_DirBufferLink;
        uio_DirBufferLink_free(dirBufferLink);
        dirBufferLink = next;
    }
}
unsafe extern "C" fn uio_DirList_new(
    mut names: *mut *const ::core::ffi::c_char,
    mut numNames: ::core::ffi::c_int,
    mut buffer: *mut ::core::ffi::c_char,
) -> *mut uio_DirList {
    let mut result: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    result = uio_DirList_alloc();
    (*result).names = names;
    (*result).numNames = numNames;
    (*result).buffer = buffer;
    return result;
}
unsafe extern "C" fn uio_DirList_alloc() -> *mut uio_DirList {
    return malloc(::core::mem::size_of::<uio_DirList>()) as *mut uio_DirList;
}
#[no_mangle]
pub unsafe extern "C" fn uio_DirList_free(mut dirList: *mut uio_DirList) {
    if !(*dirList).buffer.is_null() {
        free((*dirList).buffer as *mut ::core::ffi::c_void);
    }
    if !(*dirList).names.is_null() {
        free((*dirList).names as *mut ::core::ffi::c_void);
    }
    free(dirList as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_getPDirEntryHandle(
    mut dirHandle: *const uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
) -> *mut uio_PDirEntryHandle {
    (!(*(*dirHandle).pRoot).handler.is_null()
        || {
            __assert_fail(
                b"dirHandle->pRoot->handler != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1651 as ::core::ffi::c_int,
                b"uio_getPDirEntryHandle\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return uqm_lua_checked!((*(*(*dirHandle).pRoot).handler)
        .getPDirEntryHandle
        .expect("non-null function pointer")(dirHandle, name), ::core::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn uio_PDirHandle_deletePDirHandleExtra(
    mut pDirHandle: *mut uio_PDirHandle,
) {
    if (*pDirHandle).extra.is_null() {
        return;
    }
    ((*(*(*pDirHandle).pRoot).handler).deletePDirHandleExtra.is_some()
        || {
            __assert_fail(
                b"pDirHandle->pRoot->handler->deletePDirHandleExtra != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1659 as ::core::ffi::c_int,
                b"uio_PDirHandle_deletePDirHandleExtra\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*(*pDirHandle).pRoot).handler)
        .deletePDirHandleExtra
        .expect("non-null function pointer")((*pDirHandle).extra), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_PFileHandle_deletePFileHandleExtra(
    mut pFileHandle: *mut uio_PFileHandle,
) {
    if (*pFileHandle).extra.is_null() {
        return;
    }
    ((*(*(*pFileHandle).pRoot).handler).deletePFileHandleExtra.is_some()
        || {
            __assert_fail(
                b"pFileHandle->pRoot->handler->deletePFileHandleExtra != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/io.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1667 as ::core::ffi::c_int,
                b"uio_PFileHandle_deletePFileHandleExtra\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*(*pFileHandle).pRoot).handler)
        .deletePFileHandleExtra
        .expect("non-null function pointer")((*pFileHandle).extra), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_PDirHandle_new(
    mut pRoot: *mut uio_PRoot,
    mut extra: uio_PDirHandleExtra,
) -> *mut uio_PDirHandle {
    let mut result: *mut uio_PDirHandle = ::core::ptr::null_mut::<uio_PDirHandle>();
    result = uio_PDirHandle_alloc();
    (*result).flags = 0x1 as ::core::ffi::c_int;
    (*result).r#ref = 1 as ::core::ffi::c_int;
    (*result).pRoot = pRoot;
    (*result).extra = extra;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_PDirEntryHandle_delete(
    mut pDirEntryHandle: *mut uio_PDirEntryHandle,
) {
    if uio_PDirEntryHandle_isDir(pDirEntryHandle) {
        uqm_lua_checked!(uio_PDirHandle_delete(pDirEntryHandle as *mut uio_PDirHandle), ());
    } else {
        uqm_lua_checked!(uio_PFileHandle_delete(pDirEntryHandle as *mut uio_PFileHandle), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_PDirHandle_delete(mut pDirHandle: *mut uio_PDirHandle) {
    uqm_lua_checked!(uio_PDirHandle_deletePDirHandleExtra(pDirHandle), ());
    uio_PDirHandle_free(pDirHandle);
}
#[inline]
unsafe extern "C" fn uio_PDirHandle_alloc() -> *mut uio_PDirHandle {
    let mut result: *mut uio_PDirHandle = malloc(
        ::core::mem::size_of::<uio_PDirHandle>(),
    ) as *mut uio_PDirHandle;
    return result;
}
#[inline]
unsafe extern "C" fn uio_PDirHandle_free(mut pDirHandle: *mut uio_PDirHandle) {
    free(pDirHandle as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_PFileHandle_new(
    mut pRoot: *mut uio_PRoot,
    mut extra: uio_PFileHandleExtra,
) -> *mut uio_PFileHandle {
    let mut result: *mut uio_PFileHandle = ::core::ptr::null_mut::<uio_PFileHandle>();
    result = uio_PFileHandle_alloc();
    (*result).flags = 0 as ::core::ffi::c_int;
    (*result).r#ref = 1 as ::core::ffi::c_int;
    (*result).pRoot = pRoot;
    (*result).extra = extra;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_PFileHandle_delete(mut pFileHandle: *mut uio_PFileHandle) {
    uqm_lua_checked!(uio_PFileHandle_deletePFileHandleExtra(pFileHandle), ());
    uio_PFileHandle_free(pFileHandle);
}
#[inline]
unsafe extern "C" fn uio_PFileHandle_alloc() -> *mut uio_PFileHandle {
    let mut result: *mut uio_PFileHandle = malloc(
        ::core::mem::size_of::<uio_PFileHandle>(),
    ) as *mut uio_PFileHandle;
    return result;
}
#[inline]
unsafe extern "C" fn uio_PFileHandle_free(mut pFileHandle: *mut uio_PFileHandle) {
    free(pFileHandle as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_Handle_new(
    mut root: *mut uio_PRoot,
    mut native: uio_NativeHandle,
    mut openFlags: ::core::ffi::c_int,
) -> *mut uio_Handle {
    let mut handle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    handle = uio_Handle_alloc();
    (*handle).r#ref = 1 as ::core::ffi::c_int;
    uio_PRoot_refHandle(root);
    (*handle).root = root as *mut uio_PRoot;
    (*handle).native = native;
    (*handle).openFlags = openFlags;
    return handle;
}
#[no_mangle]
pub unsafe extern "C" fn uio_Handle_delete(mut handle: *mut uio_Handle) {
    uqm_lua_checked!((*(*(*handle).root).handler).close.expect("non-null function pointer")(handle), ());
    uqm_lua_checked!(uio_PRoot_unrefHandle((*handle).root), ());
    uio_Handle_free(handle);
}
#[inline]
unsafe extern "C" fn uio_Handle_alloc() -> *mut uio_Handle {
    let mut result: *mut uio_Handle = malloc(::core::mem::size_of::<uio_Handle>())
        as *mut uio_Handle;
    return result;
}
#[inline]
unsafe extern "C" fn uio_Handle_free(mut handle: *mut uio_Handle) {
    free(handle as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn uio_DirHandle_new(
    mut repository: *mut uio_Repository,
    mut path: *mut ::core::ffi::c_char,
    mut rootEnd: *mut ::core::ffi::c_char,
) -> *mut uio_DirHandle {
    let mut result: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    result = uio_DirHandle_alloc();
    (*result).r#ref = 1 as ::core::ffi::c_int;
    (*result).repository = repository as *mut uio_Repository;
    (*result).path = path;
    (*result).rootEnd = rootEnd;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_DirHandle_delete(mut dirHandle: *mut uio_DirHandle) {
    free((*dirHandle).path as *mut ::core::ffi::c_void);
    uio_DirHandle_free(dirHandle);
}
#[inline]
unsafe extern "C" fn uio_DirHandle_alloc() -> *mut uio_DirHandle {
    let mut result: *mut uio_DirHandle = malloc(::core::mem::size_of::<uio_DirHandle>())
        as *mut uio_DirHandle;
    return result;
}
#[inline]
unsafe extern "C" fn uio_DirHandle_free(mut dirHandle: *mut uio_DirHandle) {
    free(dirHandle as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_DirHandle_print(
    mut dirHandle: *const uio_DirHandle,
    mut out: *mut FILE,
) {
    fprintf(out, b"[\0".as_ptr() as *const ::core::ffi::c_char);
    fwrite(
        (*dirHandle).path as *const ::core::ffi::c_void,
        (*dirHandle).path.offset_from((*dirHandle).rootEnd) as size_t,
        1 as size_t,
        out,
    );
    fprintf(out, b"]%s\0".as_ptr() as *const ::core::ffi::c_char, (*dirHandle).rootEnd);
}
unsafe extern "C" fn uio_MountHandle_new(
    mut repository: *mut uio_Repository,
    mut mountInfo: *mut uio_MountInfo,
) -> *mut uio_MountHandle {
    let mut result: *mut uio_MountHandle = ::core::ptr::null_mut::<uio_MountHandle>();
    result = uio_MountHandle_alloc();
    (*result).repository = repository as *mut uio_Repository;
    (*result).mountInfo = mountInfo as *mut uio_MountInfo;
    return result;
}
#[inline]
unsafe extern "C" fn uio_MountHandle_delete(mut mountHandle: *mut uio_MountHandle) {
    uio_MountHandle_free(mountHandle);
}
#[inline]
unsafe extern "C" fn uio_MountHandle_alloc() -> *mut uio_MountHandle {
    let mut result: *mut uio_MountHandle = malloc(
        ::core::mem::size_of::<uio_MountHandle>(),
    ) as *mut uio_MountHandle;
    return result;
}
#[inline]
unsafe extern "C" fn uio_MountHandle_free(mut mountHandle: *mut uio_MountHandle) {
    free(mountHandle as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Get_Basename(
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_uchar {
    let mut last_slash: *const ::core::ffi::c_char = strrchr(
        path,
        '/' as ::core::ffi::c_int,
    );
    let mut last_backslash: *const ::core::ffi::c_char = strrchr(
        path,
        '\\' as ::core::ffi::c_int,
    );
    let mut last_separator: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if !last_slash.is_null() && !last_backslash.is_null() {
        last_separator = if last_slash > last_backslash {
            last_slash
        } else {
            last_backslash
        };
    } else if !last_slash.is_null() {
        last_separator = last_slash;
    } else if !last_backslash.is_null() {
        last_separator = last_backslash;
    }
    if !last_separator.is_null() {
        return last_separator.offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_uchar;
    }
    return path as *mut ::core::ffi::c_uchar;
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
#[inline]
unsafe extern "C" fn uio_mountInfoIsReadOnly(
    mut mountInfo: *mut uio_MountInfo,
) -> uio_bool {
    return (*mountInfo).flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
