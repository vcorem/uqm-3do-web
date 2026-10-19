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

// Rust translation of src/libs/uio/defaultfs.c.
extern "C" {
    static mut stdio_fileSystemHandler: uio_FileSystemHandler;
    static mut zip_fileSystemHandler: uio_FileSystemHandler;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_DefaultFileSystemSetup {
    pub id: uio_FileSystemID,
    pub name: *const ::core::ffi::c_char,
    pub handler: *mut uio_FileSystemHandler,
}
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
pub type uio_PRootExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PDirHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
    pub extra: uio_PDirHandleExtra,
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
pub struct uio_PDirEntryHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
}
pub type uio_NativeEntriesContext = *mut ::core::ffi::c_void;
pub type size_t = ::core::ffi::c_ulong;
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
pub type uio_FileSystemID = ::core::ffi::c_int;
#[no_mangle]
pub static mut defaultFileSystems: [uio_DefaultFileSystemSetup; 2] = unsafe {
    [
        uio_DefaultFileSystemSetup {
            id: 1 as uio_FileSystemID,
            name: b"stdio\0".as_ptr() as *const ::core::ffi::c_char,
            handler: &raw const stdio_fileSystemHandler as *mut uio_FileSystemHandler,
        },
        uio_DefaultFileSystemSetup {
            id: 2 as uio_FileSystemID,
            name: b"zip\0".as_ptr() as *const ::core::ffi::c_char,
            handler: &raw const zip_fileSystemHandler as *mut uio_FileSystemHandler,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn uio_numDefaultFileSystems() -> ::core::ffi::c_int {
    return ::core::mem::size_of::<[uio_DefaultFileSystemSetup; 2]>()
        .wrapping_div(::core::mem::size_of::<uio_DefaultFileSystemSetup>())
        as ::core::ffi::c_int;
}
