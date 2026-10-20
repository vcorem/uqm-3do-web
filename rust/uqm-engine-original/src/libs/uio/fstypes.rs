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

// Rust translation of src/libs/uio/fstypes.c.
extern "C" {
    pub type _IO_FILE;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    static defaultFileSystems: [uio_DefaultFileSystemSetup; 0];
    fn uio_numDefaultFileSystems() -> ::core::ffi::c_int;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
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
pub type uio_FileSystemID = ::core::ffi::c_int;
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
pub struct uio_DefaultFileSystemSetup {
    pub id: uio_FileSystemID,
    pub name: *const ::core::ffi::c_char,
    pub handler: *mut uio_FileSystemHandler,
}
pub type uio_bool = bool;
#[no_mangle]
pub static mut uio_fileSystems: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
    uio_FileSystemInfo,
>();
#[no_mangle]
pub unsafe extern "C" fn uio_registerDefaultFileSystems() {
    let mut i: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0;
    let mut registerResult: uio_FileSystemID = 0;
    num = uio_numDefaultFileSystems();
    i = 0 as ::core::ffi::c_int;
    while i < num {
        registerResult = uio_registerFileSystem(
            (*(&raw const defaultFileSystems as *const uio_DefaultFileSystemSetup)
                .offset(i as isize))
                .id,
            (*(&raw const defaultFileSystems as *const uio_DefaultFileSystemSetup)
                .offset(i as isize))
                .name,
            (*(&raw const defaultFileSystems as *const uio_DefaultFileSystemSetup)
                .offset(i as isize))
                .handler,
        );
        match registerResult {
            0 => {
                fprintf(
                    stderr,
                    b"Warning: Default file system '%s' is already registered.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*(&raw const defaultFileSystems
                        as *const uio_DefaultFileSystemSetup)
                        .offset(i as isize))
                        .name,
                );
            }
            -1 => {
                fprintf(
                    stderr,
                    b"Error: Could not register '%s' file \nsystem: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(&raw const defaultFileSystems
                        as *const uio_DefaultFileSystemSetup)
                        .offset(i as isize))
                        .name,
                    strerror(*__errno_location()),
                );
            }
            _ => {
                (registerResult
                    == (*(&raw const defaultFileSystems
                        as *const uio_DefaultFileSystemSetup)
                        .offset(i as isize))
                        .id
                    || {
                        __assert_fail(
                            b"registerResult == defaultFileSystems[i].id\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"src/libs/uio/fstypes.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            74 as ::core::ffi::c_int,
                            b"uio_registerDefaultFileSystems\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        0 as ::core::ffi::c_int != 0
                    }) as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_unRegisterDefaultFileSystems() {
    let mut i: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0;
    num = uio_numDefaultFileSystems();
    i = 0 as ::core::ffi::c_int;
    while i < num {
        if uio_unRegisterFileSystem(
            (*(&raw const defaultFileSystems as *const uio_DefaultFileSystemSetup)
                .offset(i as isize))
                .id,
        ) == -1 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Could not unregister '%s' file system: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*(&raw const defaultFileSystems as *const uio_DefaultFileSystemSetup)
                    .offset(i as isize))
                    .name,
                strerror(*__errno_location()),
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_registerFileSystem(
    mut wantedID: uio_FileSystemID,
    mut name: *const ::core::ffi::c_char,
    mut handler: *mut uio_FileSystemHandler,
) -> uio_FileSystemID {
    let mut ptr: *mut *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        *mut uio_FileSystemInfo,
    >();
    if !uio_validFileSystemHandler(handler) {
        *__errno_location() = 28 as ::core::ffi::c_int;
        return -1 as uio_FileSystemID;
    }
    if wantedID == 0 as ::core::ffi::c_int {
        ptr = &raw mut uio_fileSystems;
        while !(*ptr).is_null() {
            if (**ptr).id >= 0x10 as ::core::ffi::c_int {
                break;
            }
            ptr = &raw mut (**ptr).next as *mut *mut uio_FileSystemInfo;
        }
        wantedID = 0x10 as ::core::ffi::c_int as uio_FileSystemID;
        while !(*ptr).is_null() {
            if (**ptr).id != wantedID {
                break;
            }
            wantedID += 1;
            ptr = &raw mut (**ptr).next as *mut *mut uio_FileSystemInfo;
        }
    } else {
        ptr = &raw mut uio_fileSystems;
        while !(*ptr).is_null() {
            if (**ptr).id <= wantedID {
                if (**ptr).id == wantedID {
                    return 0 as uio_FileSystemID;
                }
                break;
            } else {
                ptr = &raw mut (**ptr).next as *mut *mut uio_FileSystemInfo;
            }
        }
    }
    if (*handler).init.is_some()
        && (*handler).init.expect("non-null function pointer")()
            == -1 as ::core::ffi::c_int
    {
        return -1 as uio_FileSystemID;
    }
    let mut newInfo: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        uio_FileSystemInfo,
    >();
    newInfo = uio_FileSystemInfo_new(wantedID, handler, strdup(name));
    (*newInfo).next = *ptr as *mut uio_FileSystemInfo;
    *ptr = newInfo;
    return wantedID;
}
#[no_mangle]
pub unsafe extern "C" fn uio_unRegisterFileSystem(
    mut id: uio_FileSystemID,
) -> ::core::ffi::c_int {
    let mut ptr: *mut *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        *mut uio_FileSystemInfo,
    >();
    let mut temp: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        uio_FileSystemInfo,
    >();
    ptr = uio_getFileSystemInfoPtr(id);
    if ptr.is_null() {
        *__errno_location() = 28 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if (**ptr).r#ref > 1 as ::core::ffi::c_int {
        *__errno_location() = 10 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if (*(**ptr).handler).unInit.is_some()
        && (*(**ptr).handler).unInit.expect("non-null function pointer")()
            == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    temp = *ptr;
    *ptr = (**ptr).next as *mut uio_FileSystemInfo;
    free((*temp).name as *mut ::core::ffi::c_void);
    uio_FileSystemInfo_free(temp);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_validFileSystemHandler(
    mut handler: *mut uio_FileSystemHandler,
) -> uio_bool {
    if (*handler).mount.is_none() || (*handler).umount.is_none()
        || (*handler).open.is_none() || (*handler).close.is_none()
        || (*handler).read.is_none() || (*handler).openEntries.is_none()
        || (*handler).readEntries.is_none() || (*handler).closeEntries.is_none()
    {
        fprintf(
            stderr,
            b"Invalid file system handler.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return false;
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn uio_getFileSystemHandler(
    mut id: uio_FileSystemID,
) -> *mut uio_FileSystemHandler {
    let mut ptr: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<uio_FileSystemInfo>();
    ptr = uio_fileSystems;
    while !ptr.is_null() {
        if (*ptr).id == id {
            return (*ptr).handler;
        }
        ptr = (*ptr).next as *mut uio_FileSystemInfo;
    }
    return ::core::ptr::null_mut::<uio_FileSystemHandler>();
}
#[no_mangle]
pub unsafe extern "C" fn uio_getFileSystemInfo(
    mut id: uio_FileSystemID,
) -> *mut uio_FileSystemInfo {
    let mut ptr: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<uio_FileSystemInfo>();
    ptr = uio_fileSystems;
    while !ptr.is_null() {
        if (*ptr).id == id {
            return ptr;
        }
        ptr = (*ptr).next as *mut uio_FileSystemInfo;
    }
    return ::core::ptr::null_mut::<uio_FileSystemInfo>();
}
unsafe extern "C" fn uio_getFileSystemInfoPtr(
    mut id: uio_FileSystemID,
) -> *mut *mut uio_FileSystemInfo {
    let mut ptr: *mut *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        *mut uio_FileSystemInfo,
    >();
    ptr = &raw mut uio_fileSystems;
    while !(*ptr).is_null() {
        if (**ptr).id == id {
            return ptr;
        }
        ptr = &raw mut (**ptr).next as *mut *mut uio_FileSystemInfo;
    }
    return ::core::ptr::null_mut::<*mut uio_FileSystemInfo>();
}
unsafe extern "C" fn uio_FileSystemInfo_new(
    mut id: uio_FileSystemID,
    mut handler: *mut uio_FileSystemHandler,
    mut name: *mut ::core::ffi::c_char,
) -> *mut uio_FileSystemInfo {
    let mut result: *mut uio_FileSystemInfo = ::core::ptr::null_mut::<
        uio_FileSystemInfo,
    >();
    result = uio_FileSystemInfo_alloc();
    (*result).id = id;
    (*result).handler = handler;
    (*result).name = name;
    (*result).r#ref = 1 as ::core::ffi::c_int;
    return result;
}
#[inline]
unsafe extern "C" fn uio_FileSystemInfo_alloc() -> *mut uio_FileSystemInfo {
    let mut result: *mut uio_FileSystemInfo = malloc(
        ::core::mem::size_of::<uio_FileSystemInfo>(),
    ) as *mut uio_FileSystemInfo;
    return result;
}
#[inline]
unsafe extern "C" fn uio_FileSystemInfo_free(
    mut fileSystemInfo: *mut uio_FileSystemInfo,
) {
    free(fileSystemInfo as *mut ::core::ffi::c_void);
}
