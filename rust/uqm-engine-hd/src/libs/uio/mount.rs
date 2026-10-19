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

// Rust translation of src/libs/uio/mount.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn uio_makeRootMountTree() -> *mut uio_MountTree;
    fn uio_MountTree_delete(tree: *mut uio_MountTree);
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn insertArrayPointer(
        array: *mut *const ::core::ffi::c_void,
        oldNumElements: size_t,
        insertPos: ::core::ffi::c_int,
        element: *const ::core::ffi::c_void,
    ) -> *mut *mut ::core::ffi::c_void;
    fn excludeArrayPointer(
        array: *mut *const ::core::ffi::c_void,
        oldNumElements: size_t,
        startPos: ::core::ffi::c_int,
        numExclude: ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_void;
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type off_t = ::core::ffi::c_longlong;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uio_MountLocation(pub ::core::ffi::c_uint);
impl uio_MountLocation {
    pub const uio_MOUNT_BOTTOM: Self = Self(0);
    pub const uio_MOUNT_TOP: Self = Self(4);
    pub const uio_MOUNT_BELOW: Self = Self(8);
    pub const uio_MOUNT_ABOVE: Self = Self(12);
}
#[no_mangle]
pub unsafe extern "C" fn uio_repositoryAddMount(
    mut repository: *mut uio_Repository,
    mut mountInfo: *mut uio_MountInfo,
    mut location: uio_MountLocation,
    mut relative: *mut uio_MountInfo,
) {
    match location {
        uio_MountLocation::uio_MOUNT_TOP => {
            let mut newMounts: *mut *mut uio_MountInfo = ::core::ptr::null_mut::<
                *mut uio_MountInfo,
            >();
            newMounts = malloc(
                (((*repository).numMounts + 2 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut uio_MountInfo>() as size_t),
            ) as *mut *mut uio_MountInfo;
            *newMounts.offset(0isize) = mountInfo;
            memcpy(
                newMounts.offset(1isize) as *mut ::core::ffi::c_void,
                (*repository).mounts as *const ::core::ffi::c_void,
                (((*repository).numMounts + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut uio_MountInfo>() as size_t),
            );
            free((*repository).mounts as *mut ::core::ffi::c_void);
            (*repository).mounts = newMounts as *mut *mut uio_MountInfo;
            (*repository).numMounts += 1;
        }
        uio_MountLocation::uio_MOUNT_BOTTOM => {
            (*repository).mounts = realloc(
                (*repository).mounts as *mut ::core::ffi::c_void,
                (((*repository).numMounts + 2 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut uio_MountInfo>() as size_t),
            ) as *mut *mut uio_MountInfo;
            *(*repository).mounts.offset((*repository).numMounts as isize) = mountInfo
                as *mut uio_MountInfo;
            (*repository).numMounts += 1;
        }
        uio_MountLocation::uio_MOUNT_ABOVE => {
            let mut i: ::core::ffi::c_int = 0;
            let mut newMounts_0: *mut *mut uio_MountInfo = ::core::ptr::null_mut::<
                *mut uio_MountInfo,
            >();
            i = 0 as ::core::ffi::c_int;
            while *(*repository).mounts.offset(i as isize) != relative {
                i += 1;
            }
            newMounts_0 = insertArrayPointer(
                (*repository).mounts as *mut *const ::core::ffi::c_void,
                ((*repository).numMounts + 1 as ::core::ffi::c_int) as size_t,
                i,
                mountInfo as *mut ::core::ffi::c_void,
            ) as *mut *mut uio_MountInfo;
            free((*repository).mounts as *mut ::core::ffi::c_void);
            (*repository).mounts = newMounts_0 as *mut *mut uio_MountInfo;
            (*repository).numMounts += 1;
        }
        uio_MountLocation::uio_MOUNT_BELOW => {
            let mut i_0: ::core::ffi::c_int = 0;
            let mut newMounts_1: *mut *mut uio_MountInfo = ::core::ptr::null_mut::<
                *mut uio_MountInfo,
            >();
            i_0 = 0 as ::core::ffi::c_int;
            while *(*repository).mounts.offset(i_0 as isize) != relative {
                i_0 += 1;
            }
            i_0 += 1;
            newMounts_1 = insertArrayPointer(
                (*repository).mounts as *mut *const ::core::ffi::c_void,
                ((*repository).numMounts + 1 as ::core::ffi::c_int) as size_t,
                i_0,
                mountInfo as *mut ::core::ffi::c_void,
            ) as *mut *mut uio_MountInfo;
            free((*repository).mounts as *mut ::core::ffi::c_void);
            (*repository).mounts = newMounts_1 as *mut *mut uio_MountInfo;
            (*repository).numMounts += 1;
        }
        _ => {
            (false
                || {
                    __assert_fail(
                        b"false\0".as_ptr() as *const ::core::ffi::c_char,
                        b"src/libs/uio/mount.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        96 as ::core::ffi::c_int,
                        b"uio_repositoryAddMount\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_repositoryRemoveMount(
    mut repository: *mut uio_Repository,
    mut mountInfo: *mut uio_MountInfo,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut newMounts: *mut *mut uio_MountInfo = ::core::ptr::null_mut::<
        *mut uio_MountInfo,
    >();
    i = 0 as ::core::ffi::c_int;
    while *(*repository).mounts.offset(i as isize) != mountInfo {
        i += 1;
    }
    newMounts = excludeArrayPointer(
        (*repository).mounts as *mut *const ::core::ffi::c_void,
        ((*repository).numMounts + 1 as ::core::ffi::c_int) as size_t,
        i,
        1 as ::core::ffi::c_int,
    ) as *mut *mut uio_MountInfo;
    free((*repository).mounts as *mut ::core::ffi::c_void);
    (*repository).mounts = newMounts as *mut *mut uio_MountInfo;
    (*repository).numMounts -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn uio_Repository_new(
    mut flags: ::core::ffi::c_int,
) -> *mut uio_Repository {
    let mut result: *mut uio_Repository = ::core::ptr::null_mut::<uio_Repository>();
    result = uio_Repository_alloc();
    (*result).r#ref = 1 as ::core::ffi::c_int;
    (*result).flags = flags;
    (*result).numMounts = 0 as ::core::ffi::c_int;
    (*result).mounts = malloc(
        (1 as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut uio_MountInfo>() as size_t),
    ) as *mut *mut uio_MountInfo;
    *(*result).mounts.offset(0isize) = ::core::ptr::null_mut::<uio_MountInfo>();
    (*result).mountTree = uio_makeRootMountTree() as *mut uio_MountTree;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_Repository_unref(mut repository: *mut uio_Repository) {
    ((*repository).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"repository->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/mount.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int,
                b"uio_Repository_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*repository).r#ref -= 1;
    if (*repository).r#ref == 0 as ::core::ffi::c_int {
        uio_Repository_delete(repository);
    }
}
unsafe extern "C" fn uio_Repository_alloc() -> *mut uio_Repository {
    let mut result: *mut uio_Repository = malloc(
        ::core::mem::size_of::<uio_Repository>() as size_t,
    ) as *mut uio_Repository;
    return result;
}
unsafe extern "C" fn uio_Repository_delete(mut repository: *mut uio_Repository) {
    ((*repository).numMounts == 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"repository->numMounts == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/mount.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int,
                b"uio_Repository_delete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    free((*repository).mounts as *mut ::core::ffi::c_void);
    uio_MountTree_delete((*repository).mountTree);
    uio_Repository_free(repository);
}
unsafe extern "C" fn uio_Repository_free(mut repository: *mut uio_Repository) {
    free(repository as *mut ::core::ffi::c_void);
}
