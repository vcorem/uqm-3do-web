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

// Rust translation of src/libs/uio/gphys.c.
extern "C" {
    pub type _IO_FILE;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn CharHashTable_newHashTable(
        hashFunction: CharHashTable_HashFunction,
        equalFunction: CharHashTable_EqualFunction,
        copyFunction: CharHashTable_CopyFunction,
        freeKeyFunction: CharHashTable_FreeKeyFunction,
        freeValueFunction: CharHashTable_FreeValueFunction,
        initialSize: uio_uint32,
        minFillQuotient: ::core::ffi::c_double,
        maxFillQuotient: ::core::ffi::c_double,
    ) -> *mut CharHashTable_HashTable;
    fn CharHashTable_add(
        hashTable: *mut CharHashTable_HashTable,
        key: *const CharHashTable_Key,
        value: *mut ::core::ffi::c_void,
    ) -> uio_bool;
    fn CharHashTable_remove(
        hashTable: *mut CharHashTable_HashTable,
        key: *const CharHashTable_Key,
    ) -> uio_bool;
    fn CharHashTable_find(
        hashTable: *mut CharHashTable_HashTable,
        key: *const CharHashTable_Key,
    ) -> *mut ::core::ffi::c_void;
    fn CharHashTable_count(hashTable: *const CharHashTable_HashTable) -> uio_uint32;
    fn CharHashTable_deleteHashTable(hashTable: *mut CharHashTable_HashTable);
    fn CharHashTable_getIterator(
        hashTable: *const CharHashTable_HashTable,
    ) -> *mut CharHashTable_Iterator;
    fn CharHashTable_iteratorDone(
        iterator: *const CharHashTable_Iterator,
    ) -> ::core::ffi::c_int;
    fn CharHashTable_iteratorKey(
        iterator: *mut CharHashTable_Iterator,
    ) -> *mut CharHashTable_Key;
    fn CharHashTable_iteratorValue(
        iterator: *mut CharHashTable_Iterator,
    ) -> *mut ::core::ffi::c_void;
    fn CharHashTable_iteratorNext(
        iterator: *mut CharHashTable_Iterator,
    ) -> *mut CharHashTable_Iterator;
    fn CharHashTable_freeIterator(iterator: *mut CharHashTable_Iterator);
    fn uio_PDirHandle_new(
        pRoot: *mut uio_PRoot,
        extra: uio_PDirHandleExtra,
    ) -> *mut uio_PDirHandle;
    fn uio_PFileHandle_new(
        pRoot: *mut uio_PRoot,
        extra: uio_PFileHandleExtra,
    ) -> *mut uio_PFileHandle;
    fn uio_PDirHandle_delete(pDirHandle: *mut uio_PDirHandle);
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
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn uio_PRoot_new(
        topDirHandle: *mut uio_PDirHandle,
        handler: *mut uio_FileSystemHandler,
        handle: *mut uio_Handle,
        extra: uio_PRootExtra,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_PRoot;
    fn uio_PRoot_getRootDirHandle(pRoot: *mut uio_PRoot) -> *mut uio_PDirHandle;
}
pub type uio_NativeHandle = *mut ::core::ffi::c_void;
pub type uio_GPRootExtra = *mut ::core::ffi::c_void;
pub type uio_GPDirExtra = *mut ::core::ffi::c_void;
pub type uio_GPFileExtra = *mut ::core::ffi::c_void;
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type FILE = _IO_FILE;
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
pub type uio_GPDirEntries = CharHashTable_HashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPDirEntry {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub extra: *mut ::core::ffi::c_void,
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
pub type uio_NativeEntriesContext = *mut uio_GPDirEntries_Iterator;
pub type uio_GPDirEntries_Iterator = CharHashTable_Iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharHashTable_Iterator {
    pub hashTable: *const CharHashTable_HashTable,
    pub bucketNr: uio_uint32,
    pub entry: *mut CharHashTable_HashEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PFileHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
    pub extra: uio_PFileHandleExtra,
}
#[inline]
unsafe extern "C" fn uio_GPDirEntry_isDir(
    mut gPDirEntry: *mut uio_GPDirEntry,
) -> uio_bool {
    return (*gPDirEntry).flags & 0x1 as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn uio_GPDirEntry_ref(mut gPDirEntry: *mut uio_GPDirEntry) {
    (*gPDirEntry).r#ref += 1;
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
                b"src/libs/uio/gphys.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int,
                b"uio_GPDir_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*gPDir).r#ref -= 1;
    if (*gPDir).r#ref == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(uio_GPDir_delete(gPDir), ());
    }
}
#[inline]
unsafe extern "C" fn uio_GPFile_unref(mut gPFile: *mut uio_GPFile) {
    ((*gPFile).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"gPFile->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                302 as ::core::ffi::c_int,
                b"uio_GPFile_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*gPFile).r#ref -= 1;
    if (*gPFile).r#ref == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(uio_GPFile_delete(gPFile), ());
    }
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
pub unsafe extern "C" fn uio_GPDir_prepareSubDir(
    mut gPDir: *mut uio_GPDir,
    mut dirName: *const ::core::ffi::c_char,
) -> *mut uio_GPDir {
    let mut subDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    entry = CharHashTable_find((*gPDir).entries, dirName) as *mut uio_GPDirEntry;
    if !entry.is_null() {
        if uio_GPDirEntry_isDir(entry) {
            uio_GPDir_ref(entry as *mut uio_GPDir);
            return entry as *mut uio_GPDir;
        } else {
            fprintf(
                stderr,
                b"Directory %s shadows file with the same name from the same filesystem.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                dirName,
            );
        }
    }
    subDir = uio_GPDir_new(
        (*gPDir).pRoot,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0x2000 as ::core::ffi::c_int,
    );
    return subDir;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_commitSubDir(
    mut gPDir: *mut uio_GPDir,
    mut dirName: *const ::core::ffi::c_char,
    mut subDir: *mut uio_GPDir,
) {
    if (*subDir).flags & 0x2000 as ::core::ffi::c_int != 0 {
        CharHashTable_add((*gPDir).entries, dirName, subDir as *mut ::core::ffi::c_void);
        (*subDir).flags &= !(0x2000 as ::core::ffi::c_int);
        if (*subDir).flags & 0x4000 as ::core::ffi::c_int == 0 {
            uqm_lua_checked!(uio_GPDir_unref(subDir), ());
        }
    } else {
        uqm_lua_checked!(uio_GPDir_unref(subDir), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_addFile(
    mut gPDir: *mut uio_GPDir,
    mut fileName: *const ::core::ffi::c_char,
    mut file: *mut uio_GPFile,
) {
    CharHashTable_add(
        (*gPDir).entries,
        fileName,
        file as *mut uio_GPDirEntry as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_removeFile(
    mut gPDir: *mut uio_GPDir,
    mut fileName: *const ::core::ffi::c_char,
) {
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    let mut file: *mut uio_GPFile = ::core::ptr::null_mut::<uio_GPFile>();
    let mut retVal: uio_bool = false;
    entry = CharHashTable_find((*gPDir).entries, fileName) as *mut uio_GPDirEntry;
    if entry.is_null() {
        return;
    }
    (!uio_GPDirEntry_isDir(entry)
        || {
            __assert_fail(
                b"!uio_GPDirEntry_isDir(entry)\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                126 as ::core::ffi::c_int,
                b"uio_GPDir_removeFile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    file = entry as *mut uio_GPFile;
    uqm_lua_checked!(uio_GPFile_unref(file), ());
    retVal = CharHashTable_remove((*gPDir).entries, fileName);
    (retVal as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"retVal\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                130 as ::core::ffi::c_int,
                b"uio_GPDir_removeFile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_removeSubDir(
    mut gPDir: *mut uio_GPDir,
    mut dirName: *const ::core::ffi::c_char,
) {
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    let mut subDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    let mut retVal: uio_bool = false;
    entry = CharHashTable_find((*gPDir).entries, dirName) as *mut uio_GPDirEntry;
    if entry.is_null() {
        return;
    }
    (uio_GPDirEntry_isDir(entry) as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"uio_GPDirEntry_isDir(entry)\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int,
                b"uio_GPDir_removeSubDir\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    subDir = entry as *mut uio_GPDir;
    if (*subDir).flags & 0x4000 as ::core::ffi::c_int != 0 {
        uqm_lua_checked!(uio_GPDir_unref(subDir), ());
    }
    retVal = CharHashTable_remove((*gPDir).entries, dirName);
    (retVal as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"retVal\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int,
                b"uio_GPDir_removeSubDir\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_setComplete(
    mut gPDir: *mut uio_GPDir,
    mut flag: uio_bool,
) {
    if flag {
        (*gPDir).flags |= 0x1000 as ::core::ffi::c_int;
    } else {
        (*gPDir).flags &= !(0x1000 as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_entryCount(
    mut gPDir: *const uio_GPDir,
) -> ::core::ffi::c_int {
    return CharHashTable_count((*gPDir).entries) as ::core::ffi::c_int;
}
unsafe extern "C" fn uio_GPDir_access(mut gPDir: *mut uio_GPDir) {
    if (*gPDir).flags & 0x1000 as ::core::ffi::c_int == 0 {
        uqm_lua_checked!(uio_GPDir_fill(gPDir), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_getGPDirEntry(
    mut gPDir: *mut uio_GPDir,
    mut name: *const ::core::ffi::c_char,
) -> *mut uio_GPDirEntry {
    uqm_lua_checked!(uio_GPDir_access(gPDir), ::core::ptr::null_mut());
    return CharHashTable_find((*gPDir).entries, name) as *mut uio_GPDirEntry;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_getPDirEntryHandle(
    mut pDirHandle: *const uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
) -> *mut uio_PDirEntryHandle {
    let mut gPDirEntry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    gPDirEntry = uqm_lua_checked!(uio_GPDir_getGPDirEntry((*pDirHandle).extra, name), ::core::ptr::null_mut());
    if gPDirEntry.is_null() {
        return ::core::ptr::null_mut::<uio_PDirEntryHandle>();
    }
    uio_GPDirEntry_ref(gPDirEntry);
    if uio_GPDirEntry_isDir(gPDirEntry) {
        return uio_PDirHandle_new((*pDirHandle).pRoot, gPDirEntry as uio_PDirHandleExtra)
            as *mut uio_PDirEntryHandle
    } else {
        return uio_PFileHandle_new(
            (*pDirHandle).pRoot,
            gPDirEntry as uio_PFileHandleExtra,
        ) as *mut uio_PDirEntryHandle
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_walkGPPath(
    mut startGPDir: *mut uio_GPDir,
    mut path: *const ::core::ffi::c_char,
    mut pathLen: size_t,
    mut endGPDir: *mut *mut uio_GPDir,
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
    let mut gPDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    let mut retVal: ::core::ffi::c_int = 0;
    gPDir = startGPDir;
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
            entry = uqm_lua_checked!(uio_GPDir_getGPDirEntry(gPDir, tempBuf), 0);
            if entry.is_null() {
                retVal = 44 as ::core::ffi::c_int;
                break;
            } else if !uio_GPDirEntry_isDir(entry) {
                retVal = 54 as ::core::ffi::c_int;
                break;
            } else {
                gPDir = entry as *mut uio_GPDir;
                getNextPathComponent(pathEnd, &raw mut partStart, &raw mut partEnd);
            }
        }
    }
    free(tempBuf as *mut ::core::ffi::c_void);
    *pathRest = partStart;
    *endGPDir = gPDir;
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_openEntries(
    mut pDirHandle: *mut uio_PDirHandle,
) -> uio_NativeEntriesContext {
    uqm_lua_checked!(uio_GPDir_access((*pDirHandle).extra), ::core::ptr::null_mut());
    return CharHashTable_getIterator(
        (*(*pDirHandle).extra).entries as *const CharHashTable_HashTable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_readEntries(
    mut iterator: *mut *mut uio_GPDirEntries_Iterator,
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
    start = buf as *mut *mut ::core::ffi::c_char;
    end = buf.offset(len as isize);
    num = 0 as ::core::ffi::c_int;
    while CharHashTable_iteratorDone(*iterator as *const CharHashTable_Iterator) == 0 {
        name = CharHashTable_iteratorKey(*iterator);
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
        *iterator = CharHashTable_iteratorNext(*iterator);
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
pub unsafe extern "C" fn uio_GPDir_closeEntries(
    mut iterator: *mut uio_GPDirEntries_Iterator,
) {
    CharHashTable_freeIterator(iterator);
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_fill(mut gPDir: *mut uio_GPDir) {
    if (*gPDir).flags & 0x1000 as ::core::ffi::c_int != 0
        && (*gPDir).flags & 0x2 as ::core::ffi::c_int == 0
    {
        return;
    }
    ((*(*(*(*gPDir).pRoot).extra).ops).fillGPDir.is_some()
        || {
            __assert_fail(
                b"gPDir->pRoot->extra->ops->fillGPDir != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                323 as ::core::ffi::c_int,
                b"uio_GPDir_fill\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*(*(*gPDir).pRoot).extra).ops)
        .fillGPDir
        .expect("non-null function pointer")(gPDir), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPRoot_deleteGPRootExtra(mut gPRoot: *mut uio_GPRoot) {
    if (*gPRoot).extra.is_null() {
        return;
    }
    ((*(*gPRoot).ops).deleteGPRootExtra.is_some()
        || {
            __assert_fail(
                b"gPRoot->ops->deleteGPRootExtra != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int,
                b"uio_GPRoot_deleteGPRootExtra\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*gPRoot).ops)
        .deleteGPRootExtra
        .expect("non-null function pointer")((*gPRoot).extra), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_deleteGPDirExtra(mut gPDir: *mut uio_GPDir) {
    if (*gPDir).extra.is_null() {
        return;
    }
    ((*(*(*(*gPDir).pRoot).extra).ops).deleteGPDirExtra.is_some()
        || {
            __assert_fail(
                b"gPDir->pRoot->extra->ops->deleteGPDirExtra != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                339 as ::core::ffi::c_int,
                b"uio_GPDir_deleteGPDirExtra\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*(*(*gPDir).pRoot).extra).ops)
        .deleteGPDirExtra
        .expect("non-null function pointer")((*gPDir).extra), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPFile_deleteGPFileExtra(mut gPFile: *mut uio_GPFile) {
    if (*gPFile).extra.is_null() {
        return;
    }
    ((*(*(*(*gPFile).pRoot).extra).ops).deleteGPFileExtra.is_some()
        || {
            __assert_fail(
                b"gPFile->pRoot->extra->ops->deleteGPFileExtra != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                347 as ::core::ffi::c_int,
                b"uio_GPFile_deleteGPFileExtra\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!((*(*(*(*gPFile).pRoot).extra).ops)
        .deleteGPFileExtra
        .expect("non-null function pointer")((*gPFile).extra), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_gPDirFlagsFromPRootFlags(
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut newFlags: ::core::ffi::c_int = 0;
    newFlags = 0 as ::core::ffi::c_int;
    if flags & 0x2 as ::core::ffi::c_int != 0 {
        newFlags |= 0x2 as ::core::ffi::c_int;
    }
    return newFlags;
}
#[no_mangle]
pub unsafe extern "C" fn uio_gPFileFlagsFromPRootFlags(
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut newFlags: ::core::ffi::c_int = 0;
    newFlags = 0 as ::core::ffi::c_int;
    if flags & 0x2 as ::core::ffi::c_int != 0 {
        newFlags |= 0x2 as ::core::ffi::c_int;
    }
    return newFlags;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDirHandle_delete(mut gPDirHandle: *mut uio_GPDirHandle) {
    uqm_lua_checked!(uio_GPDir_unref(gPDirHandle), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPFileHandle_delete(
    mut gPFileHandle: *mut uio_GPFileHandle,
) {
    uqm_lua_checked!(uio_GPFile_unref(gPFileHandle), ());
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDirEntry_delete(mut gPDirEntry: *mut uio_GPDirEntry) {
    if uio_GPDirEntry_isDir(gPDirEntry) {
        uqm_lua_checked!(uio_GPDir_delete(gPDirEntry as *mut uio_GPDir), ());
    } else {
        uqm_lua_checked!(uio_GPFile_delete(gPDirEntry as *mut uio_GPFile), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_new(
    mut pRoot: *mut uio_PRoot,
    mut extra: uio_GPDirExtra,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_GPDir {
    let mut gPDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    gPDir = uio_GPDir_alloc();
    (*gPDir).r#ref = 1 as ::core::ffi::c_int;
    (*gPDir).pRoot = pRoot;
    (*gPDir).entries = CharHashTable_newHashTable(
        None,
        None,
        None,
        None,
        None,
        0 as uio_uint32,
        0.85f64,
        0.9f64,
    );
    (*gPDir).extra = extra;
    flags |= uio_gPDirFlagsFromPRootFlags((*(*gPDir).pRoot).flags);
    if (*(*pRoot).extra).flags & 0x4000 as ::core::ffi::c_int != 0 {
        flags |= 0x4000 as ::core::ffi::c_int;
    }
    (*gPDir).flags = flags | 0x1 as ::core::ffi::c_int;
    return gPDir;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_delete(mut gPDir: *mut uio_GPDir) {
    ((*gPDir).r#ref == 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"gPDir->ref == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                439 as ::core::ffi::c_int,
                b"uio_GPDir_delete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    CharHashTable_deleteHashTable((*gPDir).entries);
    uqm_lua_checked!(uio_GPDir_deleteGPDirExtra(gPDir), ());
    uio_GPDir_free(gPDir);
}
unsafe extern "C" fn uio_GPDir_deepPersistentUnref(mut gPDir: *mut uio_GPDir) {
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    let mut iterator: *mut uio_GPDirEntries_Iterator = ::core::ptr::null_mut::<
        uio_GPDirEntries_Iterator,
    >();
    iterator = CharHashTable_getIterator(
        (*gPDir).entries as *const CharHashTable_HashTable,
    );
    while CharHashTable_iteratorDone(iterator as *const CharHashTable_Iterator) == 0 {
        entry = CharHashTable_iteratorValue(iterator) as *mut uio_GPDirEntry;
        if uio_GPDirEntry_isDir(entry) {
            uqm_lua_checked!(uio_GPDir_deepPersistentUnref(entry as *mut uio_GPDir), ());
        } else {
            uqm_lua_checked!(uio_GPFile_unref(entry as *mut uio_GPFile), ());
        }
        iterator = CharHashTable_iteratorNext(iterator);
    }
    CharHashTable_freeIterator(iterator);
    if (*gPDir).flags & 0x4000 as ::core::ffi::c_int != 0 {
        uqm_lua_checked!(uio_GPDir_unref(gPDir), ());
    } else {
        (*gPDir).flags &= !(0x1000 as ::core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C" fn uio_GPDir_alloc() -> *mut uio_GPDir {
    let mut result: *mut uio_GPDir = malloc(::core::mem::size_of::<uio_GPDir>())
        as *mut uio_GPDir;
    return result;
}
#[inline]
unsafe extern "C" fn uio_GPDir_free(mut gPDir: *mut uio_GPDir) {
    free(gPDir as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPFile_new(
    mut pRoot: *mut uio_PRoot,
    mut extra: uio_GPFileExtra,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_GPFile {
    let mut gPFile: *mut uio_GPFile = ::core::ptr::null_mut::<uio_GPFile>();
    gPFile = uio_GPFile_alloc();
    (*gPFile).r#ref = 1 as ::core::ffi::c_int;
    (*gPFile).pRoot = pRoot;
    (*gPFile).extra = extra;
    (*gPFile).flags = flags;
    return gPFile;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPFile_delete(mut gPFile: *mut uio_GPFile) {
    ((*gPFile).r#ref == 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"gPFile->ref == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/gphys.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                500 as ::core::ffi::c_int,
                b"uio_GPFile_delete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!(uio_GPFile_deleteGPFileExtra(gPFile), ());
    uio_GPFile_free(gPFile);
}
#[inline]
unsafe extern "C" fn uio_GPFile_alloc() -> *mut uio_GPFile {
    let mut result: *mut uio_GPFile = malloc(::core::mem::size_of::<uio_GPFile>())
        as *mut uio_GPFile;
    return result;
}
#[inline]
unsafe extern "C" fn uio_GPFile_free(mut gPFile: *mut uio_GPFile) {
    free(gPFile as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPRoot_makePRoot(
    mut handler: *mut uio_FileSystemHandler,
    mut pRootFlags: ::core::ffi::c_int,
    mut ops: *mut uio_GPRoot_Operations,
    mut gPRootExtra: uio_GPRootExtra,
    mut gPRootFlags: ::core::ffi::c_int,
    mut handle: *mut uio_Handle,
    mut gPDirExtra: uio_GPDirExtra,
    mut gPDirFlags: ::core::ffi::c_int,
) -> *mut uio_PRoot {
    let mut result: *mut uio_PRoot = ::core::ptr::null_mut::<uio_PRoot>();
    let mut gPTopDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    let mut gPRoot: *mut uio_GPRoot = ::core::ptr::null_mut::<uio_GPRoot>();
    gPRoot = uio_GPRoot_new(ops, gPRootExtra, gPRootFlags);
    result = uio_PRoot_new(
        ::core::ptr::null_mut::<uio_PDirHandle>(),
        handler,
        handle,
        gPRoot,
        pRootFlags,
    );
    gPTopDir = uio_GPDir_new(result, gPDirExtra, gPDirFlags);
    if (*gPRoot).flags & 0x4000 as ::core::ffi::c_int != 0 {
        uio_GPDir_ref(gPTopDir);
    }
    (*result).rootDir = uio_GPDir_makePDirHandle(gPTopDir) as *mut uio_PDirHandle;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPRoot_umount(
    mut pRoot: *mut uio_PRoot,
) -> ::core::ffi::c_int {
    let mut topDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    let mut topDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
    topDirHandle = uio_PRoot_getRootDirHandle(pRoot);
    topDir = (*topDirHandle).extra as *mut uio_GPDir;
    if (*(*pRoot).extra).flags & 0x4000 as ::core::ffi::c_int != 0 {
        uqm_lua_checked!(uio_GPDir_deepPersistentUnref(topDir), 0);
    }
    uqm_lua_checked!(uio_PDirHandle_unref(topDirHandle), 0);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPRoot_new(
    mut ops: *mut uio_GPRoot_Operations,
    mut extra: uio_GPRootExtra,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_GPRoot {
    let mut result: *mut uio_GPRoot = ::core::ptr::null_mut::<uio_GPRoot>();
    result = uio_GPRoot_alloc();
    (*result).ops = ops;
    (*result).extra = extra;
    (*result).flags = flags;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPRoot_delete(mut gPRoot: *mut uio_GPRoot) {
    uqm_lua_checked!(uio_GPRoot_deleteGPRootExtra(gPRoot), ());
    uio_GPRoot_free(gPRoot);
}
unsafe extern "C" fn uio_GPRoot_alloc() -> *mut uio_GPRoot {
    let mut result: *mut uio_GPRoot = malloc(::core::mem::size_of::<uio_GPRoot>())
        as *mut uio_GPRoot;
    return result;
}
unsafe extern "C" fn uio_GPRoot_free(mut gPRoot: *mut uio_GPRoot) {
    free(gPRoot as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_makePDirHandle(
    mut gPDir: *mut uio_GPDir,
) -> *mut uio_PDirHandle {
    return uio_PDirHandle_new((*gPDir).pRoot, gPDir);
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDirEntry_print(
    mut outStream: *mut FILE,
    mut gPDirEntry: *mut uio_GPDirEntry,
) {
    if uio_GPDirEntry_isDir(gPDirEntry) {
        uio_GPDir_print(outStream, gPDirEntry as *mut uio_GPDir);
    } else {
        uio_GPFile_print(outStream, gPDirEntry as *mut uio_GPFile);
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_GPDir_print(
    mut outStream: *mut FILE,
    mut gPDir: *mut uio_GPDir,
) {}
#[no_mangle]
pub unsafe extern "C" fn uio_GPFile_print(
    mut outStream: *mut FILE,
    mut gPFile: *mut uio_GPFile,
) {}
