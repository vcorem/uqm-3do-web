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

// The GPDir structures and functions are used for caching only.

// Rust translation of src/libs/uio/stdio/stdio.c.
extern "C" {
    pub type __dirstream;
    pub type _IO_FILE;
    fn closedir(_: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(_: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir_r(_: *mut DIR, _: *mut dirent, _: *mut *mut dirent) -> ::core::ffi::c_int;
    fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lseek(_: ::core::ffi::c_int, _: off_t, _: ::core::ffi::c_int) -> off_t;
    fn read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, _: size_t) -> ssize_t;
    fn write(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;
    fn unlink(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rmdir(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn access(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    fn fstat(_: ::core::ffi::c_int, _: *mut stat) -> ::core::ffi::c_int;
    fn mkdir(_: *const ::core::ffi::c_char, _: mode_t) -> ::core::ffi::c_int;
    static stderr: *mut FILE;
    fn rename(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn joinPaths(
        first: *const ::core::ffi::c_char,
        second: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn open(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn uio_getFileSystemHandler(id: uio_FileSystemID) -> *mut uio_FileSystemHandler;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn uio_Handle_new(
        root: *mut uio_PRoot,
        native: uio_NativeHandle,
        openFlags: ::core::ffi::c_int,
    ) -> *mut uio_Handle;
    fn uio_PDirHandle_new(
        pRoot: *mut uio_PRoot,
        extra: uio_PDirHandleExtra,
    ) -> *mut uio_PDirHandle;
    fn uio_PFileHandle_new(
        pRoot: *mut uio_PRoot,
        extra: uio_PFileHandleExtra,
    ) -> *mut uio_PFileHandle;
    fn uio_gPFileFlagsFromPRootFlags(flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uio_GPRoot_makePRoot(
        handler: *mut uio_FileSystemHandler,
        pRootFlags: ::core::ffi::c_int,
        ops: *mut uio_GPRoot_Operations,
        gPRootExtra: uio_GPRootExtra,
        gPRootFlags: ::core::ffi::c_int,
        handle: *mut uio_Handle,
        gPDirExtra: uio_GPDirExtra,
        gPDirFlags: ::core::ffi::c_int,
    ) -> *mut uio_PRoot;
    fn uio_GPRoot_umount(pRoot: *mut uio_PRoot) -> ::core::ffi::c_int;
    fn uio_GPDir_prepareSubDir(
        gPDir: *mut uio_GPDir,
        dirName: *const ::core::ffi::c_char,
    ) -> *mut uio_GPDir;
    fn uio_GPDir_commitSubDir(
        gPDir: *mut uio_GPDir,
        dirName: *const ::core::ffi::c_char,
        subDir: *mut uio_GPDir,
    );
    fn uio_GPDir_addFile(
        gPDir: *mut uio_GPDir,
        fileName: *const ::core::ffi::c_char,
        file: *mut uio_GPFile,
    );
    fn uio_GPDir_removeFile(gPDir: *mut uio_GPDir, fileName: *const ::core::ffi::c_char);
    fn uio_GPDir_removeSubDir(
        gPDir: *mut uio_GPDir,
        dirName: *const ::core::ffi::c_char,
    );
    fn uio_GPDir_setComplete(gPDir: *mut uio_GPDir, flag: uio_bool);
    fn uio_GPDir_getGPDirEntry(
        gPDir: *mut uio_GPDir,
        name: *const ::core::ffi::c_char,
    ) -> *mut uio_GPDirEntry;
    fn uio_GPDir_getPDirEntryHandle(
        pDirHandle: *const uio_PDirHandle,
        name: *const ::core::ffi::c_char,
    ) -> *mut uio_PDirEntryHandle;
    fn uio_GPDirHandle_delete(gPDirHandle: *mut uio_GPDirHandle);
    fn uio_GPFileHandle_delete(gPFileHandle: *mut uio_GPFileHandle);
    fn uio_GPRoot_delete(gPRoot: *mut uio_GPRoot);
    fn uio_GPDir_delete(gPDir: *mut uio_GPDir);
    fn uio_GPFile_new(
        pRoot: *mut uio_PRoot,
        extra: uio_GPFileExtra,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_GPFile;
    fn CharHashTable_add(
        hashTable: *mut CharHashTable_HashTable,
        key: *const CharHashTable_Key,
        value: *mut ::core::ffi::c_void,
    ) -> uio_bool;
    fn CharHashTable_remove(
        hashTable: *mut CharHashTable_HashTable,
        key: *const CharHashTable_Key,
    ) -> uio_bool;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stdio_Handle {
    pub fd: ::core::ffi::c_int,
}
pub type uio_NativeHandle = *mut stdio_Handle;
pub type uio_GPRootExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stdio_GPDirData {
    pub name: *mut ::core::ffi::c_char,
    pub cachedPath: *mut ::core::ffi::c_char,
    pub upDir: *mut uio_GPDir,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPDir {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub extra: uio_GPDirExtra,
    pub pRoot: *mut uio_PRoot,
    pub entries: *mut uio_GPDirEntries,
}
pub type uio_GPDirEntries = CharHashTable_HashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharHashTable_HashTable {
    pub hashFunction: CharHashTable_HashFunction,
    pub equalFunction: CharHashTable_EqualFunction,
    pub copyFunction: CharHashTable_CopyFunction,
    pub freeKeyFunction: CharHashTable_FreeKeyFunction,
    pub freeValueFunction: CharHashTable_FreeValueFunction,
    pub minFillQuotient: ::core::ffi::c_double,
    pub maxFillQuotient: ::core::ffi::c_double,
    pub minSize: uio_uint32,
    pub maxSize: uio_uint32,
    pub size: uio_uint32,
    pub hashMask: uio_uint32,
    pub entries: *mut *mut CharHashTable_HashEntry,
    pub numEntries: uio_uint32,
    pub numCollisions: uio_uint32,
}
pub type uio_uint32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharHashTable_HashEntry {
    pub hash: uio_uint32,
    pub key: *mut CharHashTable_Key,
    pub value: *mut ::core::ffi::c_void,
    pub next: *mut CharHashTable_HashEntry,
}
pub type CharHashTable_Value = ();
pub type CharHashTable_Key = ::core::ffi::c_char;
pub type CharHashTable_FreeValueFunction = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type CharHashTable_FreeKeyFunction = Option<
    unsafe extern "C" fn(*mut CharHashTable_Key) -> (),
>;
pub type CharHashTable_CopyFunction = Option<
    unsafe extern "C" fn(*const CharHashTable_Key) -> *mut ::core::ffi::c_void,
>;
pub type CharHashTable_EqualFunction = Option<
    unsafe extern "C" fn(*const CharHashTable_Key, *const CharHashTable_Key) -> uio_bool,
>;
pub type uio_bool = bool;
pub type CharHashTable_HashFunction = Option<
    unsafe extern "C" fn(*const CharHashTable_Key) -> uio_uint32,
>;
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
pub type uio_PRootExtra = *mut uio_GPRoot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPRoot {
    pub flags: ::core::ffi::c_int,
    pub ops: *mut uio_GPRoot_Operations,
    pub extra: uio_GPRootExtra,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPRoot_Operations {
    pub fillGPDir: Option<unsafe extern "C" fn(*mut uio_GPDir) -> ()>,
    pub deleteGPRootExtra: Option<unsafe extern "C" fn(uio_GPRootExtra) -> ()>,
    pub deleteGPDirExtra: Option<unsafe extern "C" fn(uio_GPDirExtra) -> ()>,
    pub deleteGPFileExtra: Option<unsafe extern "C" fn(uio_GPFileExtra) -> ()>,
}
pub type uio_GPFileExtra = *mut ::core::ffi::c_void;
pub type uio_GPDirExtra = *mut stdio_GPDirData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_Handle {
    pub r#ref: ::core::ffi::c_int,
    pub root: *mut uio_PRoot,
    pub native: uio_NativeHandle,
    pub openFlags: ::core::ffi::c_int,
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
pub type uio_PFileHandleExtra = *mut uio_GPFileHandle;
pub type uio_GPFileHandle = uio_GPFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPFile {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub extra: uio_GPFileExtra,
    pub pRoot: *mut uio_PRoot,
}
pub type uio_PDirHandleExtra = *mut uio_GPDirHandle;
pub type uio_GPDirHandle = uio_GPDir;
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
pub struct uio_PDirEntryHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
}
pub type uio_NativeEntriesContext = *mut stdio_EntriesIterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stdio_EntriesIterator {
    pub dirHandle: *mut DIR,
    pub entry: *mut dirent,
    pub direntBuffer: *mut dirent,
    pub status: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type off_t = ::core::ffi::c_longlong;
pub type ino_t = ::core::ffi::c_ulonglong;
pub type DIR = __dirstream;
pub type size_t = usize;
pub type ssize_t = isize;
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
pub struct uio_GPDirEntry {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub extra: *mut ::core::ffi::c_void,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PFileHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
    pub extra: uio_PFileHandleExtra,
}
pub type uio_FileSystemID = ::core::ffi::c_int;
#[no_mangle]
pub static mut stdio_fileSystemHandler: uio_FileSystemHandler = uio_FileSystemHandler {
    init: None,
    unInit: None,
    cleanup: None,
    mount: Some(
        stdio_mount
            as unsafe extern "C" fn(
                *mut uio_Handle,
                ::core::ffi::c_int,
            ) -> *mut uio_PRoot,
    ),
    umount: Some(
        uio_GPRoot_umount as unsafe extern "C" fn(*mut uio_PRoot) -> ::core::ffi::c_int,
    ),
    access: Some(
        stdio_access
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    close: Some(stdio_close as unsafe extern "C" fn(*mut uio_Handle) -> ()),
    fstat: Some(
        stdio_fstat
            as unsafe extern "C" fn(*mut uio_Handle, *mut stat) -> ::core::ffi::c_int,
    ),
    stat: Some(
        stdio_stat
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                *mut stat,
            ) -> ::core::ffi::c_int,
    ),
    mkdir: Some(
        stdio_mkdir
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                mode_t,
            ) -> *mut uio_PDirHandle,
    ),
    open: Some(
        stdio_open
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                mode_t,
            ) -> *mut uio_Handle,
    ),
    read: Some(
        stdio_read
            as unsafe extern "C" fn(
                *mut uio_Handle,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> ssize_t,
    ),
    rename: Some(
        stdio_rename
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
    ),
    rmdir: Some(
        stdio_rmdir
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
    ),
    seek: Some(
        stdio_seek
            as unsafe extern "C" fn(*mut uio_Handle, off_t, ::core::ffi::c_int) -> off_t,
    ),
    write: Some(
        stdio_write
            as unsafe extern "C" fn(
                *mut uio_Handle,
                *const ::core::ffi::c_void,
                size_t,
            ) -> ssize_t,
    ),
    unlink: Some(
        stdio_unlink
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
    ),
    openEntries: Some(
        stdio_openEntries
            as unsafe extern "C" fn(*mut uio_PDirHandle) -> *mut stdio_EntriesIterator,
    ),
    readEntries: Some(
        stdio_readEntries
            as unsafe extern "C" fn(
                *mut *mut stdio_EntriesIterator,
                *mut ::core::ffi::c_char,
                size_t,
            ) -> ::core::ffi::c_int,
    ),
    closeEntries: Some(
        stdio_closeEntries as unsafe extern "C" fn(*mut stdio_EntriesIterator) -> (),
    ),
    getPDirEntryHandle: Some(
        stdio_getPDirEntryHandle
            as unsafe extern "C" fn(
                *const uio_PDirHandle,
                *const ::core::ffi::c_char,
            ) -> *mut uio_PDirEntryHandle,
    ),
    deletePRootExtra: Some(
        uio_GPRoot_delete as unsafe extern "C" fn(*mut uio_GPRoot) -> (),
    ),
    deletePDirHandleExtra: Some(
        uio_GPDirHandle_delete as unsafe extern "C" fn(*mut uio_GPDirHandle) -> (),
    ),
    deletePFileHandleExtra: Some(
        uio_GPFileHandle_delete as unsafe extern "C" fn(*mut uio_GPFileHandle) -> (),
    ),
};
#[no_mangle]
pub static mut stdio_GPRootOperations: uio_GPRoot_Operations = uio_GPRoot_Operations {
    fillGPDir: None,
    deleteGPRootExtra: None,
    deleteGPDirExtra: Some(
        stdio_GPDirData_delete as unsafe extern "C" fn(*mut stdio_GPDirData) -> (),
    ),
    deleteGPFileExtra: None,
};
#[no_mangle]
pub unsafe extern "C" fn stdio_close(mut handle: *mut uio_Handle) {
    let mut fd: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 0;
    fd = (*(*handle).native).fd;
    free((*handle).native as *mut ::core::ffi::c_void);
    loop {
        result = close(fd);
        if result == 0 as ::core::ffi::c_int {
            break;
        }
        if *__errno_location() == 27 as ::core::ffi::c_int {
            continue;
        }
        fprintf(
            stderr,
            b"Warning: Error while closing socket: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        break;
    };
}
#[no_mangle]
pub unsafe extern "C" fn stdio_access(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = 0;
    path = joinPaths(stdio_getPath((*pDirHandle).extra), name);
    if path.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    result = access(path, mode);
    if result == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    free(path as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_fstat(
    mut handle: *mut uio_Handle,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    return fstat((*(*handle).native).fd, statBuf);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_stat(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = 0;
    path = joinPaths(stdio_getPath((*pDirHandle).extra), name);
    if path.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    result = stat(path, statBuf);
    if result == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    free(path as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_mkdir(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> *mut uio_PDirHandle {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut newGPDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    path = joinPaths(stdio_getPath((*pDirHandle).extra), name);
    if path.is_null() {
        return ::core::ptr::null_mut::<uio_PDirHandle>();
    }
    if mkdir(path, mode) == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return ::core::ptr::null_mut::<uio_PDirHandle>();
    }
    free(path as *mut ::core::ffi::c_void);
    newGPDir = stdio_addDir((*pDirHandle).extra, name);
    uio_GPDir_ref(newGPDir);
    return uio_PDirHandle_new((*pDirHandle).pRoot, newGPDir);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_open(
    mut pDirHandle: *mut uio_PDirHandle,
    mut file: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mode: mode_t,
) -> *mut uio_Handle {
    let mut handle: *mut stdio_Handle = ::core::ptr::null_mut::<stdio_Handle>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut fd: ::core::ffi::c_int = 0;
    path = joinPaths(stdio_getPath((*pDirHandle).extra), file);
    if path.is_null() {
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    fd = open(path, flags, mode);
    if fd == -1 as ::core::ffi::c_int {
        let mut save_errno: ::core::ffi::c_int = 0;
        save_errno = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = save_errno;
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    free(path as *mut ::core::ffi::c_void);
    handle = malloc(::core::mem::size_of::<stdio_Handle>()) as *mut stdio_Handle;
    (*handle).fd = fd;
    return uio_Handle_new((*pDirHandle).pRoot, handle, flags);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_read(
    mut handle: *mut uio_Handle,
    mut buf: *mut ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    return read((*(*handle).native).fd, buf, count);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_rename(
    mut oldPDirHandle: *mut uio_PDirHandle,
    mut oldName: *const ::core::ffi::c_char,
    mut newPDirHandle: *mut uio_PDirHandle,
    mut newName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut newPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut oldPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = 0;
    oldPath = joinPaths(stdio_getPath((*oldPDirHandle).extra), oldName);
    if oldPath.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    newPath = joinPaths(stdio_getPath((*newPDirHandle).extra), newName);
    if newPath.is_null() {
        free(oldPath as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    result = rename(oldPath, newPath);
    if result == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(oldPath as *mut ::core::ffi::c_void);
        free(newPath as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    free(oldPath as *mut ::core::ffi::c_void);
    free(newPath as *mut ::core::ffi::c_void);
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    entry = uio_GPDir_getGPDirEntry((*oldPDirHandle).extra, oldName);
    if !entry.is_null() {
        CharHashTable_remove((*(*oldPDirHandle).extra).entries, oldName);
        CharHashTable_add(
            (*(*newPDirHandle).extra).entries,
            newName,
            entry as *mut ::core::ffi::c_void,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_rmdir(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = 0;
    path = joinPaths(stdio_getPath((*pDirHandle).extra), name);
    if path.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    result = rmdir(path);
    if result == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    free(path as *mut ::core::ffi::c_void);
    uio_GPDir_removeSubDir((*pDirHandle).extra, name);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_seek(
    mut handle: *mut uio_Handle,
    mut offset: off_t,
    mut whence: ::core::ffi::c_int,
) -> off_t {
    return lseek((*(*handle).native).fd, offset, whence);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_write(
    mut handle: *mut uio_Handle,
    mut buf: *const ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    return write((*(*handle).native).fd, buf, count);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_unlink(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut result: ::core::ffi::c_int = 0;
    path = joinPaths(stdio_getPath((*pDirHandle).extra), name);
    if path.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    result = unlink(path);
    if result == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    free(path as *mut ::core::ffi::c_void);
    uio_GPDir_removeFile((*pDirHandle).extra, name);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_getPDirEntryHandle(
    mut pDirHandle: *const uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
) -> *mut uio_PDirEntryHandle {
    let mut result: *mut uio_PDirEntryHandle = ::core::ptr::null_mut::<
        uio_PDirEntryHandle,
    >();
    let mut pathUpTo: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
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
    result = uio_GPDir_getPDirEntryHandle(pDirHandle, name);
    if !result.is_null() {
        return result;
    }
    pathUpTo = stdio_getPath((*pDirHandle).extra);
    if pathUpTo.is_null() {
        return ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    }
    path = joinPaths(pathUpTo, name);
    if path.is_null() {
        return ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    }
    if stat(path, &raw mut statBuf) == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        free(path as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
        return ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    }
    free(path as *mut ::core::ffi::c_void);
    if statBuf.st_mode & 0o170000 as mode_t == 0o100000 as mode_t {
        let mut gPFile: *mut uio_GPFile = ::core::ptr::null_mut::<uio_GPFile>();
        gPFile = stdio_addFile((*pDirHandle).extra, name);
        uio_GPFile_ref(gPFile);
        return uio_PFileHandle_new((*pDirHandle).pRoot, gPFile)
            as *mut uio_PDirEntryHandle;
    } else if statBuf.st_mode & 0o170000 as mode_t == 0o40000 as mode_t {
        let mut gPDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
        gPDir = stdio_addDir((*pDirHandle).extra, name);
        uio_GPDir_ref(gPDir);
        return uio_PDirHandle_new((*pDirHandle).pRoot, gPDir)
            as *mut uio_PDirEntryHandle;
    } else {
        fprintf(
            stderr,
            b"Warning: Attempt to access '%s' from '%s', which is not a regular file, nor a directory.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            name,
            pathUpTo,
        );
        return ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn stdio_mount(
    mut handle: *mut uio_Handle,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_PRoot {
    let mut result: *mut uio_PRoot = ::core::ptr::null_mut::<uio_PRoot>();
    let mut extra: *mut stdio_GPDirData = ::core::ptr::null_mut::<stdio_GPDirData>();
    (handle.is_null()
        || {
            __assert_fail(
                b"handle == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/stdio/stdio.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_int,
                b"stdio_mount\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    extra = stdio_GPDirData_new(
        strdup(b"\0".as_ptr() as *const ::core::ffi::c_char),
        strdup(b"/\0".as_ptr() as *const ::core::ffi::c_char),
        ::core::ptr::null_mut::<uio_GPDir>(),
    );
    result = uio_GPRoot_makePRoot(
        uio_getFileSystemHandler(1 as uio_FileSystemID),
        flags,
        &raw mut stdio_GPRootOperations,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0x4000 as ::core::ffi::c_int,
        handle,
        extra,
        0 as ::core::ffi::c_int,
    );
    uio_GPDir_setComplete((*(*result).rootDir).extra, true);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_openEntries(
    mut pDirHandle: *mut uio_PDirHandle,
) -> *mut stdio_EntriesIterator {
    let mut dirPath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut dirHandle: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut result: *mut stdio_EntriesIterator = ::core::ptr::null_mut::<
        stdio_EntriesIterator,
    >();
    dirPath = stdio_getPath((*pDirHandle).extra);
    if dirPath.is_null() {
        return ::core::ptr::null_mut::<stdio_EntriesIterator>();
    }
    dirHandle = opendir(dirPath);
    if dirHandle.is_null() {
        return ::core::ptr::null_mut::<stdio_EntriesIterator>();
    }
    result = stdio_EntriesIterator_new(dirHandle);
    (*result).status = readdir_r(
        dirHandle,
        (*result).direntBuffer,
        &raw mut (*result).entry,
    );
    if (*result).status != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Warning: readdir_r() failed: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror((*result).status),
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_readEntries(
    mut iteratorPtr: *mut *mut stdio_EntriesIterator,
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut start: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    let mut num: ::core::ffi::c_int = 0;
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut nameLen: size_t = 0;
    let mut iterator: *mut stdio_EntriesIterator = ::core::ptr::null_mut::<
        stdio_EntriesIterator,
    >();
    iterator = *iteratorPtr;
    start = buf as *mut *mut ::core::ffi::c_char;
    end = buf.offset(len as isize);
    num = 0 as ::core::ffi::c_int;
    while (*iterator).status == 0 as ::core::ffi::c_int && !(*iterator).entry.is_null() {
        name = &raw mut (*(*iterator).entry).d_name as *mut ::core::ffi::c_char;
        if !(*name.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            && (*name.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || *name.offset(1isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                    && *name.offset(2isize) as ::core::ffi::c_int
                        == '\0' as ::core::ffi::c_int))
        {
            nameLen = strlen(name).wrapping_add(1 as size_t);
            if ::core::mem::size_of::<*mut ::core::ffi::c_char>().wrapping_add(nameLen)
                > end.offset_from(start as *mut ::core::ffi::c_char) as size_t
            {
                return num;
            }
            end = end.offset(-(nameLen as isize));
            memcpy(
                end as *mut ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                nameLen,
            );
            *start = end;
            start = start.offset(1);
            num += 1;
        }
        (*iterator).status = readdir_r(
            (*iterator).dirHandle,
            (*iterator).direntBuffer,
            &raw mut (*iterator).entry,
        );
    }
    if (*iterator).status != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Warning: readdir_r() failed: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror((*iterator).status),
        );
    }
    if ::core::mem::size_of::<*mut ::core::ffi::c_char>()
        > end.offset_from(start as *mut ::core::ffi::c_char) as size_t
    {
        return num;
    }
    *start = ::core::ptr::null_mut::<::core::ffi::c_char>();
    num += 1;
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_closeEntries(mut iterator: *mut stdio_EntriesIterator) {
    closedir((*iterator).dirHandle);
    stdio_EntriesIterator_delete(iterator);
}
#[no_mangle]
pub unsafe extern "C" fn stdio_EntriesIterator_new(
    mut dirHandle: *mut DIR,
) -> *mut stdio_EntriesIterator {
    let mut result: *mut stdio_EntriesIterator = ::core::ptr::null_mut::<
        stdio_EntriesIterator,
    >();
    let mut bufferSize: size_t = 0;
    result = stdio_EntriesIterator_alloc();
    (*result).dirHandle = dirHandle;
    bufferSize = ::core::mem::size_of::<dirent>()
        .wrapping_sub(::core::mem::size_of::<[::core::ffi::c_char; 256]>())
        .wrapping_add((255 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize)
        as size_t;
    (*result).direntBuffer = malloc(bufferSize) as *mut dirent;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn stdio_EntriesIterator_delete(
    mut iterator: *mut stdio_EntriesIterator,
) {
    free((*iterator).direntBuffer as *mut ::core::ffi::c_void);
    stdio_EntriesIterator_free(iterator);
}
#[inline]
unsafe extern "C" fn stdio_EntriesIterator_alloc() -> *mut stdio_EntriesIterator {
    return malloc(::core::mem::size_of::<stdio_EntriesIterator>())
        as *mut stdio_EntriesIterator;
}
#[inline]
unsafe extern "C" fn stdio_EntriesIterator_free(
    mut iterator: *mut stdio_EntriesIterator,
) {
    free(iterator as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn stdio_addFile(
    mut gPDir: *mut uio_GPDir,
    mut fileName: *const ::core::ffi::c_char,
) -> *mut uio_GPFile {
    let mut file: *mut uio_GPFile = ::core::ptr::null_mut::<uio_GPFile>();
    file = uio_GPFile_new(
        (*gPDir).pRoot,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        uio_gPFileFlagsFromPRootFlags((*(*gPDir).pRoot).flags),
    );
    uio_GPDir_addFile(gPDir, fileName, file);
    return file;
}
#[inline]
unsafe extern "C" fn stdio_addDir(
    mut gPDir: *mut uio_GPDir,
    mut dirName: *const ::core::ffi::c_char,
) -> *mut uio_GPDir {
    let mut subDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    subDir = uio_GPDir_prepareSubDir(gPDir, dirName);
    if (*subDir).extra.is_null() {
        uio_GPDir_ref(gPDir);
        (*subDir).extra = stdio_GPDirData_new(
            strdup(dirName),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            gPDir,
        ) as uio_GPDirExtra;
        uio_GPDir_setComplete(subDir, true);
    }
    uio_GPDir_commitSubDir(gPDir, dirName, subDir);
    return subDir;
}
unsafe extern "C" fn stdio_getPath(
    mut gPDir: *mut uio_GPDir,
) -> *mut ::core::ffi::c_char {
    if (*(*gPDir).extra).cachedPath.is_null() {
        let mut upPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut upPathLen: size_t = 0;
        let mut nameLen: size_t = 0;
        if (*(*gPDir).extra).upDir.is_null() {
            (*(*gPDir).extra).cachedPath = malloc(2 as size_t)
                as *mut ::core::ffi::c_char;
            *(*(*gPDir).extra).cachedPath.offset(0isize) = '/' as ::core::ffi::c_char;
            *(*(*gPDir).extra).cachedPath.offset(1isize) = '\0' as ::core::ffi::c_char;
            return (*(*gPDir).extra).cachedPath;
        }
        upPath = stdio_getPath((*(*gPDir).extra).upDir);
        if upPath.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        upPathLen = strlen(upPath);
        if *upPath.offset(upPathLen.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        {
            upPathLen = upPathLen.wrapping_sub(1);
        }
        nameLen = strlen((*(*gPDir).extra).name);
        if upPathLen.wrapping_add(nameLen).wrapping_add(1 as size_t) >= 4096 as size_t {
            *__errno_location() = 37 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*(*gPDir).extra).cachedPath = malloc(
            upPathLen.wrapping_add(nameLen).wrapping_add(2 as size_t),
        ) as *mut ::core::ffi::c_char;
        memcpy(
            (*(*gPDir).extra).cachedPath as *mut ::core::ffi::c_void,
            upPath as *const ::core::ffi::c_void,
            upPathLen,
        );
        *(*(*gPDir).extra).cachedPath.offset(upPathLen as isize) = '/'
            as ::core::ffi::c_char;
        memcpy(
            (*(*gPDir).extra)
                .cachedPath
                .offset(upPathLen as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*(*gPDir).extra).name as *const ::core::ffi::c_void,
            nameLen,
        );
        *(*(*gPDir).extra)
            .cachedPath
            .offset(
                upPathLen.wrapping_add(nameLen).wrapping_add(1 as size_t) as isize,
            ) = '\0' as ::core::ffi::c_char;
    }
    return (*(*gPDir).extra).cachedPath;
}
unsafe extern "C" fn stdio_GPDirData_new(
    mut name: *mut ::core::ffi::c_char,
    mut cachedPath: *mut ::core::ffi::c_char,
    mut upDir: *mut uio_GPDir,
) -> *mut stdio_GPDirData {
    let mut result: *mut stdio_GPDirData = ::core::ptr::null_mut::<stdio_GPDirData>();
    result = stdio_GPDirData_alloc();
    (*result).name = name;
    (*result).cachedPath = cachedPath;
    (*result).upDir = upDir;
    return result;
}
unsafe extern "C" fn stdio_GPDirData_delete(mut gPDirData: *mut stdio_GPDirData) {
    if !(*gPDirData).upDir.is_null() {
        uio_GPDir_unref((*gPDirData).upDir);
    }
    stdio_GPDirData_free(gPDirData);
}
#[inline]
unsafe extern "C" fn stdio_GPDirData_alloc() -> *mut stdio_GPDirData {
    let mut result: *mut stdio_GPDirData = ::core::ptr::null_mut::<stdio_GPDirData>();
    result = malloc(::core::mem::size_of::<stdio_GPDirData>()) as *mut stdio_GPDirData;
    return result;
}
#[inline]
unsafe extern "C" fn stdio_GPDirData_free(mut gPDirData: *mut stdio_GPDirData) {
    free((*gPDirData).name as *mut ::core::ffi::c_void);
    if !(*gPDirData).cachedPath.is_null() {
        free((*gPDirData).cachedPath as *mut ::core::ffi::c_void);
    }
    free(gPDirData as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn uio_GPDir_ref(mut gPDir: *mut uio_GPDir) {
    (*gPDir).r#ref += 1;
}
#[inline]
unsafe extern "C" fn uio_GPDir_unref(mut gPDir: *mut uio_GPDir) {
    ((*gPDir).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"gPDir->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/stdio/./../gphys.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int,
                b"uio_GPDir_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*gPDir).r#ref -= 1;
    if (*gPDir).r#ref == 0 as ::core::ffi::c_int {
        uio_GPDir_delete(gPDir);
    }
}
#[inline]
unsafe extern "C" fn uio_GPFile_ref(mut gPFile: *mut uio_GPFile) {
    (*gPFile).r#ref += 1;
}
