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

// Rust translation of src/libs/uio/mounttree.c.
extern "C" {
    pub type _IO_FILE;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn uio_PDirHandle_delete(pDirHandle: *mut uio_PDirHandle);
    fn uio_getFileSystemInfo(id: uio_FileSystemID) -> *mut uio_FileSystemInfo;
    fn getFirstPath0Component(
        dir: *const ::core::ffi::c_char,
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn getNextPath0Component(
        startComp: *mut *const ::core::ffi::c_char,
        endComp: *mut *const ::core::ffi::c_char,
    );
    fn uio_PathComp_delete(pathComp: *mut uio_PathComp);
    fn uio_countPathComps(comp: *const uio_PathComp) -> ::core::ffi::c_int;
    fn uio_lastPathComp(comp: *mut uio_PathComp) -> *mut uio_PathComp;
    fn uio_makePathComps(
        path: *const ::core::ffi::c_char,
        upComp: *mut uio_PathComp,
    ) -> *mut uio_PathComp;
    fn uio_printPathToComp(outStream: *mut FILE, comp: *const uio_PathComp);
}
pub type size_t = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_FileSystemInfo {
    pub r#ref: ::core::ffi::c_int,
    pub id: uio_FileSystemID,
    pub name: *mut ::core::ffi::c_char,
    pub handler: *mut uio_FileSystemHandler,
    pub next: *mut uio_FileSystemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
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
pub unsafe extern "C" fn uio_makeRootMountTree() -> *mut uio_MountTree {
    return uio_MountTree_new(
        ::core::ptr::null_mut::<uio_MountTree>(),
        ::core::ptr::null_mut::<uio_MountTreeItem>(),
        ::core::ptr::null_mut::<uio_MountTree>(),
        ::core::ptr::null_mut::<uio_PathComp>(),
        ::core::ptr::null_mut::<uio_PathComp>(),
        ::core::ptr::null_mut::<uio_MountTree>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn uio_mountTreeAddMountInfo(
    mut repository: *mut uio_Repository,
    mut mountTree: *mut uio_MountTree,
    mut mountInfo: *mut uio_MountInfo,
    mut path: *const ::core::ffi::c_char,
    mut location: uio_MountLocation,
    mut relative: *const uio_MountInfo,
) -> *mut uio_MountTree {
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    getFirstPath0Component(path, &raw mut start, &raw mut end);
    return uio_mountTreeAddMountInfoRecTree(
        repository,
        mountTree,
        mountInfo,
        start,
        end,
        ::core::ptr::null_mut::<uio_PathComp>(),
        location,
        relative,
    );
}
unsafe extern "C" fn uio_mountTreeAddMountInfoRecTree(
    mut repository: *mut uio_Repository,
    mut tree: *mut uio_MountTree,
    mut mountInfo: *mut uio_MountInfo,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut upComp: *mut uio_PathComp,
    mut location: uio_MountLocation,
    mut relative: *const uio_MountInfo,
) -> *mut uio_MountTree {
    let mut sub: *mut *mut uio_MountTree = ::core::ptr::null_mut::<*mut uio_MountTree>();
    if *start as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        uio_mountTreeAddMountInfoLocAll(
            repository,
            tree,
            mountInfo,
            0 as ::core::ffi::c_int,
            location,
            relative,
        );
        return tree;
    }
    sub = &raw mut (*tree).subTrees as *mut *mut uio_MountTree;
    while !(*sub).is_null() {
        let mut resTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
        resTree = uio_mountTreeAddMountInfoRecTreeSub(
            repository,
            sub,
            mountInfo,
            start,
            end,
            location,
            relative,
        );
        if !resTree.is_null() {
            return resTree;
        }
        sub = &raw mut (**sub).next as *mut *mut uio_MountTree;
    }
    return uio_mountTreeAddNewSubTree(
        repository,
        tree,
        start,
        mountInfo,
        upComp,
        location,
        relative,
    );
}
#[inline]
unsafe extern "C" fn uio_mountTreeAddMountInfoRecTreeSub(
    mut repository: *mut uio_Repository,
    mut tree: *mut *mut uio_MountTree,
    mut mountInfo: *mut uio_MountInfo,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut location: uio_MountLocation,
    mut relative: *const uio_MountInfo,
) -> *mut uio_MountTree {
    let mut comp: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut lastComp: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut depth: ::core::ffi::c_int = 0;
    comp = (**tree).comps as *mut uio_PathComp;
    if strncmp((*comp).name, start, end.offset_from(start) as size_t)
        != 0 as ::core::ffi::c_int
        || *(*comp).name.offset(end.offset_from(start) as isize) as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<uio_MountTree>();
    }
    depth = 1 as ::core::ffi::c_int;
    loop {
        getNextPath0Component(&raw mut start, &raw mut end);
        lastComp = comp;
        comp = (*comp).next as *mut uio_PathComp;
        if comp.is_null() {
            break;
        }
        if *start as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            let mut newTree: *mut uio_MountTree = ::core::ptr::null_mut::<
                uio_MountTree,
            >();
            newTree = uio_splitMountTree(tree, lastComp, depth);
            uio_mountTreeAddMountInfoLocAll(
                repository,
                newTree,
                mountInfo,
                0 as ::core::ffi::c_int,
                location,
                relative,
            );
            return newTree;
        }
        if strncmp((*comp).name, start, end.offset_from(start) as size_t)
            != 0 as ::core::ffi::c_int
            || *(*comp).name.offset(end.offset_from(start) as isize)
                as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            let mut newTree_0: *mut uio_MountTree = ::core::ptr::null_mut::<
                uio_MountTree,
            >();
            newTree_0 = uio_splitMountTree(tree, lastComp, depth);
            return uio_mountTreeAddNewSubTree(
                repository,
                newTree_0,
                start,
                mountInfo,
                lastComp,
                location,
                relative,
            );
        }
        getNextPath0Component(&raw mut start, &raw mut end);
        depth += 1;
    }
    return uio_mountTreeAddMountInfoRecTree(
        repository,
        *tree,
        mountInfo,
        start,
        end,
        lastComp,
        location,
        relative,
    );
}
unsafe extern "C" fn uio_mountTreeAddMountInfoLocAll(
    mut repository: *mut uio_Repository,
    mut tree: *mut uio_MountTree,
    mut mountInfo: *mut uio_MountInfo,
    mut depth: ::core::ffi::c_int,
    mut location: uio_MountLocation,
    mut relative: *const uio_MountInfo,
) {
    let mut newPLoc: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    let mut subTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut compCount: ::core::ffi::c_int = 0;
    newPLoc = uio_MountTree_newItem(
        mountInfo,
        depth,
        ::core::ptr::null_mut::<uio_MountTreeItem>(),
    );
    uio_addMountTreeItem(
        repository,
        &raw mut (*tree).pLocs,
        newPLoc,
        location,
        relative,
    );
    subTree = (*tree).subTrees as *mut uio_MountTree;
    while !subTree.is_null() {
        compCount = uio_countPathComps((*subTree).comps);
        uio_mountTreeAddMountInfoLocAll(
            repository,
            subTree,
            mountInfo,
            depth + compCount,
            location,
            relative,
        );
        subTree = (*subTree).next as *mut uio_MountTree;
    }
}
unsafe extern "C" fn uio_addMountTreeItem(
    mut repository: *mut uio_Repository,
    mut pLocs: *mut *mut uio_MountTreeItem,
    mut item: *mut uio_MountTreeItem,
    mut location: uio_MountLocation,
    mut relative: *const uio_MountInfo,
) {
    match location {
        uio_MountLocation::uio_MOUNT_TOP => {
            (*item).next = *pLocs as *mut uio_MountTreeItem;
            *pLocs = item;
        }
        uio_MountLocation::uio_MOUNT_BOTTOM => {
            while !(*pLocs).is_null() {
                pLocs = &raw mut (**pLocs).next as *mut *mut uio_MountTreeItem;
            }
            (*item).next = ::core::ptr::null_mut::<uio_MountTreeItem>();
            *pLocs = item;
        }
        uio_MountLocation::uio_MOUNT_ABOVE => {
            let mut mountInfo: *mut *mut uio_MountInfo = ::core::ptr::null_mut::<
                *mut uio_MountInfo,
            >();
            mountInfo = (*repository).mounts as *mut *mut uio_MountInfo;
            while *mountInfo != relative as *mut uio_MountInfo {
                (!(*mountInfo).is_null()
                    || {
                        __assert_fail(
                            b"*mountInfo != NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"src/libs/uio/mounttree.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            251 as ::core::ffi::c_int,
                            b"uio_addMountTreeItem\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        0 as ::core::ffi::c_int != 0
                    }) as ::core::ffi::c_int;
                if (**pLocs).mountInfo == *mountInfo {
                    pLocs = &raw mut (**pLocs).next as *mut *mut uio_MountTreeItem;
                }
                mountInfo = mountInfo.offset(1);
            }
            (*item).next = *pLocs as *mut uio_MountTreeItem;
            *pLocs = item;
        }
        uio_MountLocation::uio_MOUNT_BELOW => {
            let mut mountInfo_0: *mut *mut uio_MountInfo = ::core::ptr::null_mut::<
                *mut uio_MountInfo,
            >();
            mountInfo_0 = (*repository).mounts as *mut *mut uio_MountInfo;
            while *mountInfo_0 != relative as *mut uio_MountInfo {
                (!(*mountInfo_0).is_null()
                    || {
                        __assert_fail(
                            b"*mountInfo != NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"src/libs/uio/mounttree.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            264 as ::core::ffi::c_int,
                            b"uio_addMountTreeItem\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        0 as ::core::ffi::c_int != 0
                    }) as ::core::ffi::c_int;
                if (**pLocs).mountInfo == *mountInfo_0 {
                    pLocs = &raw mut (**pLocs).next as *mut *mut uio_MountTreeItem;
                }
                mountInfo_0 = mountInfo_0.offset(1);
            }
            (*item).next = (**pLocs).next;
            (**pLocs).next = item as *mut uio_MountTreeItem;
        }
        _ => {
            (false
                || {
                    __assert_fail(
                        b"false\0".as_ptr() as *const ::core::ffi::c_char,
                        b"src/libs/uio/mounttree.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        274 as ::core::ffi::c_int,
                        b"uio_addMountTreeItem\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn uio_copyMountTreeItems(
    mut item: *mut uio_MountTreeItem,
    mut extraDepth: ::core::ffi::c_int,
) -> *mut uio_MountTreeItem {
    let mut result: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    let mut resPtr: *mut *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        *mut uio_MountTreeItem,
    >();
    let mut newItem: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    resPtr = &raw mut result;
    while !item.is_null() {
        newItem = uio_MountTree_newItem(
            (*item).mountInfo,
            (*item).depth + extraDepth,
            ::core::ptr::null_mut::<uio_MountTreeItem>(),
        );
        *resPtr = newItem;
        resPtr = &raw mut (*newItem).next as *mut *mut uio_MountTreeItem;
        item = (*item).next as *mut uio_MountTreeItem;
    }
    *resPtr = ::core::ptr::null_mut::<uio_MountTreeItem>();
    return result;
}
unsafe extern "C" fn uio_mountTreeAddNewSubTree(
    mut repository: *mut uio_Repository,
    mut tree: *mut uio_MountTree,
    mut path: *const ::core::ffi::c_char,
    mut mountInfo: *mut uio_MountInfo,
    mut upComp: *mut uio_PathComp,
    mut location: uio_MountLocation,
    mut relative: *const uio_MountInfo,
) -> *mut uio_MountTree {
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    let mut items: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    let mut newTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut compList: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut lastComp: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    let mut compCount: ::core::ffi::c_int = 0;
    compList = uio_makePathComps(path, upComp);
    compCount = uio_countPathComps(compList);
    lastComp = uio_lastPathComp(compList);
    item = uio_MountTree_newItem(
        mountInfo,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<uio_MountTreeItem>(),
    );
    (*item).next = ::core::ptr::null_mut::<uio_MountTreeItem>();
    items = uio_copyMountTreeItems((*tree).pLocs, compCount);
    uio_addMountTreeItem(repository, &raw mut items, item, location, relative);
    newTree = uio_MountTree_new(
        ::core::ptr::null_mut::<uio_MountTree>(),
        items,
        tree,
        compList,
        lastComp,
        ::core::ptr::null_mut::<uio_MountTree>(),
    );
    uio_mountTreeAddSub(tree, newTree);
    return newTree;
}
unsafe extern "C" fn uio_mountTreeAddSub(
    mut tree: *mut uio_MountTree,
    mut sub: *mut uio_MountTree,
) {
    let mut subPtr: *mut *mut uio_MountTree = ::core::ptr::null_mut::<
        *mut uio_MountTree,
    >();
    subPtr = &raw mut (*tree).subTrees as *mut *mut uio_MountTree;
    while !(*subPtr).is_null() {
        subPtr = &raw mut (**subPtr).next as *mut *mut uio_MountTree;
    }
    *subPtr = sub;
}
unsafe extern "C" fn uio_splitMountTree(
    mut tree: *mut *mut uio_MountTree,
    mut lastComp: *mut uio_PathComp,
    mut depth: ::core::ffi::c_int,
) -> *mut uio_MountTree {
    let mut newTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut items: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    items = uio_copyMountTreeItems((*(**tree).upTree).pLocs, depth);
    newTree = uio_MountTree_new(
        *tree,
        items,
        (**tree).upTree,
        (**tree).comps,
        lastComp,
        ::core::ptr::null_mut::<uio_MountTree>(),
    );
    (**tree).upTree = newTree as *mut uio_MountTree;
    (**tree).comps = (*lastComp).next;
    (*lastComp).next = ::core::ptr::null_mut::<uio_PathComp>();
    *tree = newTree;
    return newTree;
}
#[no_mangle]
pub unsafe extern "C" fn uio_mountTreeRemoveMountInfo(
    mut repository: *mut uio_Repository,
    mut mountTree: *mut uio_MountTree,
    mut mountInfo: *mut uio_MountInfo,
) {
    let mut subTreePtr: *mut *mut uio_MountTree = ::core::ptr::null_mut::<
        *mut uio_MountTree,
    >();
    let mut upTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    if (*mountTree).upTree.is_null() || !(*mountTree).subTrees.is_null()
        || uio_mountTreeCountPLocs(mountTree)
            != uio_mountTreeCountPLocs((*mountTree).upTree) + 1 as ::core::ffi::c_int
    {
        uio_mountTreeRemoveMountInfoRec(mountTree, mountInfo);
        return;
    }
    subTreePtr = &raw mut (*(*mountTree).upTree).subTrees as *mut *mut uio_MountTree;
    loop {
        (!(*subTreePtr).is_null()
            || {
                __assert_fail(
                    b"*subTreePtr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/libs/uio/mounttree.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    392 as ::core::ffi::c_int,
                    b"uio_mountTreeRemoveMountInfo\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if *subTreePtr == mountTree {
            break;
        }
        subTreePtr = &raw mut (**subTreePtr).next as *mut *mut uio_MountTree;
    }
    *subTreePtr = (*mountTree).next as *mut uio_MountTree;
    upTree = (*mountTree).upTree as *mut uio_MountTree;
    uio_MountTree_delete(mountTree);
    if (*upTree).subTrees.is_null() || !(*(*upTree).subTrees).next.is_null()
        || uio_mountTreeCountPLocs(upTree) != uio_mountTreeCountPLocs((*upTree).subTrees)
    {
        return;
    }
    (!(*(*upTree).subTrees).lastComp.is_null()
        || {
            __assert_fail(
                b"upTree->subTrees->lastComp != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/mounttree.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                420 as ::core::ffi::c_int,
                b"uio_mountTreeRemoveMountInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*(*(*upTree).subTrees).lastComp).next = (*(*upTree).subTrees).comps;
    (*(*upTree).subTrees).lastComp = (*upTree).lastComp;
    (*(*upTree).subTrees).comps = (*upTree).comps;
    if (*upTree).upTree.is_null() {
        ((*repository).mountTree == upTree
            || {
                __assert_fail(
                    b"repository->mountTree == upTree\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/libs/uio/mounttree.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    427 as ::core::ffi::c_int,
                    b"uio_mountTreeRemoveMountInfo\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        (*repository).mountTree = (*upTree).subTrees;
    } else {
        let mut next: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
        subTreePtr = &raw mut (*(*upTree).upTree).subTrees as *mut *mut uio_MountTree;
        loop {
            (!(*subTreePtr).is_null()
                || {
                    __assert_fail(
                        b"*subTreePtr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"src/libs/uio/mounttree.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        434 as ::core::ffi::c_int,
                        b"uio_mountTreeRemoveMountInfo\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            if *subTreePtr == upTree {
                break;
            }
            subTreePtr = &raw mut (**subTreePtr).next as *mut *mut uio_MountTree;
        }
        next = (**subTreePtr).next as *mut uio_MountTree;
        *subTreePtr = (*upTree).subTrees as *mut uio_MountTree;
        (*(*upTree).subTrees).next = next as *mut uio_MountTree;
    }
    (*upTree).subTrees = ::core::ptr::null_mut::<uio_MountTree>();
    (*upTree).comps = ::core::ptr::null_mut::<uio_PathComp>();
    uio_MountTree_delete(upTree);
}
unsafe extern "C" fn uio_mountTreeRemoveMountInfoRec(
    mut mountTree: *mut uio_MountTree,
    mut mountInfo: *mut uio_MountInfo,
) {
    let mut subTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut itemPtr: *mut *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        *mut uio_MountTreeItem,
    >();
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    subTree = (*mountTree).subTrees as *mut uio_MountTree;
    while !subTree.is_null() {
        uio_mountTreeRemoveMountInfoRec(subTree, mountInfo);
        subTree = (*subTree).next as *mut uio_MountTree;
    }
    itemPtr = &raw mut (*mountTree).pLocs as *mut *mut uio_MountTreeItem;
    loop {
        (!(*itemPtr).is_null()
            || {
                __assert_fail(
                    b"*itemPtr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/libs/uio/mounttree.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    466 as ::core::ffi::c_int,
                    b"uio_mountTreeRemoveMountInfoRec\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if (**itemPtr).mountInfo == mountInfo {
            break;
        }
        itemPtr = &raw mut (**itemPtr).next as *mut *mut uio_MountTreeItem;
    }
    item = *itemPtr;
    *itemPtr = (*item).next as *mut uio_MountTreeItem;
    uio_MountTreeItem_delete(item);
}
#[no_mangle]
pub unsafe extern "C" fn uio_mountTreeCountPLocs(
    mut tree: *const uio_MountTree,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    count = 0 as ::core::ffi::c_int;
    item = (*tree).pLocs as *mut uio_MountTreeItem;
    while !item.is_null() {
        count += 1;
        item = (*item).next as *mut uio_MountTreeItem;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn uio_findMountTree(
    mut top: *mut uio_MountTree,
    mut path: *const ::core::ffi::c_char,
    mut resTree: *mut *mut uio_MountTree,
    mut pPath: *mut *const ::core::ffi::c_char,
) {
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pathFromTree: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut tree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut sub: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut comp: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    getFirstPath0Component(path, &raw mut start, &raw mut end);
    tree = top;
    loop {
        if *start as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            *resTree = tree;
            *pPath = start;
            return;
        }
        pathFromTree = start;
        sub = (*tree).subTrees as *mut uio_MountTree;
        loop {
            if sub.is_null() {
                *resTree = tree;
                *pPath = pathFromTree;
                return;
            }
            comp = (*sub).comps as *mut uio_PathComp;
            if strncmp((*comp).name, start, end.offset_from(start) as size_t)
                == 0 as ::core::ffi::c_int
                && *(*comp).name.offset(end.offset_from(start) as isize)
                    as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            {
                break;
            }
            sub = (*sub).next as *mut uio_MountTree;
        }
        loop {
            getNextPath0Component(&raw mut start, &raw mut end);
            comp = (*comp).next as *mut uio_PathComp;
            if comp.is_null() {
                break;
            }
            if *start as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || strncmp((*comp).name, start, end.offset_from(start) as size_t)
                    != 0 as ::core::ffi::c_int
                || *(*comp).name.offset(end.offset_from(start) as isize)
                    as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                *resTree = tree;
                *pPath = pathFromTree;
                return;
            }
        }
        tree = sub;
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_mountTreeItemRestPath(
    mut item: *const uio_MountTreeItem,
    mut endComp: *mut uio_PathComp,
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut pathPtr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    i = (*item).depth;
    loop {
        let c2rust_fresh0 = i;
        i -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        endComp = (*endComp).up as *mut uio_PathComp;
    }
    pathPtr = path;
    if !endComp.is_null() {
        loop {
            pathPtr = pathPtr.offset((*endComp).nameLen as isize);
            endComp = (*endComp).up as *mut uio_PathComp;
            if endComp.is_null() {
                break;
            }
            pathPtr = pathPtr.offset(1);
        }
    }
    if *path as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        pathPtr = pathPtr.offset(1);
    }
    if *pathPtr as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        pathPtr = pathPtr.offset(1);
    }
    return unconst(pathPtr as *const ::core::ffi::c_void) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn uio_printMountTree(
    mut outStream: *mut FILE,
    mut tree: *const uio_MountTree,
    mut indent: ::core::ffi::c_int,
) {
    let mut sub: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut comp: *mut uio_PathComp = ::core::ptr::null_mut::<uio_PathComp>();
    fprintf(outStream, b"(\0".as_ptr() as *const ::core::ffi::c_char);
    uio_printMountTreeItems(outStream, (*tree).pLocs);
    fprintf(outStream, b")\n\0".as_ptr() as *const ::core::ffi::c_char);
    sub = (*tree).subTrees as *mut uio_MountTree;
    while !sub.is_null() {
        let mut newIndent: ::core::ffi::c_int = 0;
        newIndent = indent;
        fprintf(
            outStream,
            b"%*s\0".as_ptr() as *const ::core::ffi::c_char,
            indent,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        comp = (*sub).comps as *mut uio_PathComp;
        while !comp.is_null() {
            fprintf(
                outStream,
                b"/%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*comp).name,
            );
            newIndent = (newIndent as size_t)
                .wrapping_add((1 as size_t).wrapping_add((*comp).nameLen))
                as ::core::ffi::c_int;
            comp = (*comp).next as *mut uio_PathComp;
        }
        fprintf(outStream, b" \0".as_ptr() as *const ::core::ffi::c_char);
        newIndent += 1 as ::core::ffi::c_int;
        uio_printMountTree(outStream, sub, newIndent);
        sub = (*sub).next as *mut uio_MountTree;
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_printMountTreeItem(
    mut outStream: *mut FILE,
    mut item: *const uio_MountTreeItem,
) {
    uio_printMountInfo(outStream, (*item).mountInfo);
    fprintf(outStream, b":%d\0".as_ptr() as *const ::core::ffi::c_char, (*item).depth);
}
#[no_mangle]
pub unsafe extern "C" fn uio_printMountTreeItems(
    mut outStream: *mut FILE,
    mut item: *const uio_MountTreeItem,
) {
    if item.is_null() {
        return;
    }
    loop {
        uio_printMountTreeItem(outStream, item);
        item = (*item).next;
        if item.is_null() {
            break;
        }
        fprintf(outStream, b", \0".as_ptr() as *const ::core::ffi::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_printPathToMountTree(
    mut outStream: *mut FILE,
    mut tree: *const uio_MountTree,
) {
    if (*tree).upTree.is_null() {
        fprintf(outStream, b"/\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        uio_printPathToComp(outStream, (*tree).lastComp);
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_printMountInfo(
    mut outStream: *mut FILE,
    mut mountInfo: *const uio_MountInfo,
) {
    let mut fsInfo: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        uio_FileSystemInfo,
    >();
    fsInfo = uio_getFileSystemInfo((*mountInfo).fsID);
    fprintf(
        outStream,
        b"%s:/%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*fsInfo).name,
        (*mountInfo).dirName,
    );
}
unsafe extern "C" fn uio_printMount(
    mut outStream: *mut FILE,
    mut mountInfo: *const uio_MountInfo,
) {
    let mut fsInfo: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        uio_FileSystemInfo,
    >();
    fsInfo = uio_getFileSystemInfo((*mountInfo).fsID);
    fprintf(
        outStream,
        b"???:%s on \0".as_ptr() as *const ::core::ffi::c_char,
        (*mountInfo).dirName,
    );
    uio_printPathToMountTree(outStream, (*mountInfo).mountTree);
    fprintf(
        outStream,
        b" type %s (\0".as_ptr() as *const ::core::ffi::c_char,
        (*fsInfo).name,
    );
    if (*mountInfo).flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        fprintf(outStream, b"ro\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(outStream, b"rw\0".as_ptr() as *const ::core::ffi::c_char);
    }
    fprintf(outStream, b")\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn uio_printMounts(
    mut outStream: *mut FILE,
    mut repository: *const uio_Repository,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*repository).numMounts {
        uio_printMount(outStream, *(*repository).mounts.offset(i as isize));
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn uio_MountTree_new(
    mut subTrees: *mut uio_MountTree,
    mut pLocs: *mut uio_MountTreeItem,
    mut upTree: *mut uio_MountTree,
    mut comps: *mut uio_PathComp,
    mut lastComp: *mut uio_PathComp,
    mut next: *mut uio_MountTree,
) -> *mut uio_MountTree {
    let mut result: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    result = uio_MountTree_alloc();
    (*result).subTrees = subTrees as *mut uio_MountTree;
    (*result).pLocs = pLocs as *mut uio_MountTreeItem;
    (*result).upTree = upTree as *mut uio_MountTree;
    (*result).comps = comps as *mut uio_PathComp;
    (*result).lastComp = lastComp as *mut uio_PathComp;
    (*result).next = next as *mut uio_MountTree;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_MountTree_delete(mut tree: *mut uio_MountTree) {
    let mut subTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut nextTree: *mut uio_MountTree = ::core::ptr::null_mut::<uio_MountTree>();
    let mut item: *mut uio_MountTreeItem = ::core::ptr::null_mut::<uio_MountTreeItem>();
    let mut nextItem: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    subTree = (*tree).subTrees as *mut uio_MountTree;
    while !subTree.is_null() {
        nextTree = (*subTree).next as *mut uio_MountTree;
        uio_MountTree_delete(subTree);
        subTree = nextTree;
    }
    item = (*tree).pLocs as *mut uio_MountTreeItem;
    while !item.is_null() {
        nextItem = (*item).next as *mut uio_MountTreeItem;
        uio_MountTreeItem_delete(item);
        item = nextItem;
    }
    if !(*tree).comps.is_null() {
        uio_PathComp_delete((*tree).comps);
    }
    uio_MountTree_free(tree);
}
#[inline]
unsafe extern "C" fn uio_MountTree_alloc() -> *mut uio_MountTree {
    let mut result: *mut uio_MountTree = malloc(
        ::core::mem::size_of::<uio_MountTree>() as size_t,
    ) as *mut uio_MountTree;
    return result;
}
#[inline]
unsafe extern "C" fn uio_MountTree_free(mut mountTree: *mut uio_MountTree) {
    free(mountTree as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn uio_MountTree_newItem(
    mut mountInfo: *mut uio_MountInfo,
    mut depth: ::core::ffi::c_int,
    mut next: *mut uio_MountTreeItem,
) -> *mut uio_MountTreeItem {
    let mut result: *mut uio_MountTreeItem = ::core::ptr::null_mut::<
        uio_MountTreeItem,
    >();
    result = uio_MountTreeItem_alloc();
    (*result).mountInfo = mountInfo as *mut uio_MountInfo;
    (*result).depth = depth;
    (*result).next = next as *mut uio_MountTreeItem;
    return result;
}
#[inline]
unsafe extern "C" fn uio_MountTreeItem_delete(mut item: *mut uio_MountTreeItem) {
    uio_MountTreeItem_free(item);
}
#[inline]
unsafe extern "C" fn uio_MountTreeItem_alloc() -> *mut uio_MountTreeItem {
    let mut result: *mut uio_MountTreeItem = malloc(
        ::core::mem::size_of::<uio_MountTreeItem>() as size_t,
    ) as *mut uio_MountTreeItem;
    return result;
}
#[inline]
unsafe extern "C" fn uio_MountTreeItem_free(mut mountTreeItem: *mut uio_MountTreeItem) {
    free(mountTreeItem as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_MountInfo_new(
    mut fsID: uio_FileSystemID,
    mut mountTree: *mut uio_MountTree,
    mut pDirHandle: *mut uio_PDirHandle,
    mut dirName: *mut ::core::ffi::c_char,
    mut autoMount: *mut *mut uio_AutoMount,
    mut mountHandle: *mut uio_MountHandle,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_MountInfo {
    let mut result: *mut uio_MountInfo = ::core::ptr::null_mut::<uio_MountInfo>();
    result = uio_MountInfo_alloc();
    (*result).fsID = fsID;
    (*result).mountTree = mountTree;
    (*result).pDirHandle = pDirHandle;
    (*result).dirName = dirName;
    (*result).autoMount = autoMount;
    (*result).mountHandle = mountHandle;
    (*result).flags = flags;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_MountInfo_delete(mut mountInfo: *mut uio_MountInfo) {
    free((*mountInfo).dirName as *mut ::core::ffi::c_void);
    uqm_lua_checked!(uio_PDirHandle_unref((*mountInfo).pDirHandle), ());
    uio_MountInfo_free(mountInfo);
}
#[inline]
unsafe extern "C" fn uio_MountInfo_alloc() -> *mut uio_MountInfo {
    let mut result: *mut uio_MountInfo = malloc(
        ::core::mem::size_of::<uio_MountInfo>() as size_t,
    ) as *mut uio_MountInfo;
    return result;
}
#[inline]
unsafe extern "C" fn uio_MountInfo_free(mut mountInfo: *mut uio_MountInfo) {
    free(mountInfo as *mut ::core::ffi::c_void);
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
