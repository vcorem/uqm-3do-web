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

/*
 * This file makes use of zlib (http://www.gzip.org/zlib/)
 *
 * References:
 * The .zip format description from PKWare:
 *   http://www.pkware.com/products/enterprise/white_papers/appnote.html
 * The .zip format description from InfoZip:
 *   ftp://ftp.info-zip.org/pub/infozip/doc/appnote-011203-iz.zip
 */

// Rust translation of src/libs/uio/zip/zip.c.
extern "C" {
    pub type _IO_FILE;
    pub type uio_FileBlock;
    pub type internal_state;
    pub type uio_GPDirEntries_Iterator;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn uio_GPDir_openEntries(
        pDirHandle: *mut uio_PDirHandle,
    ) -> uio_NativeEntriesContext;
    fn uio_GPDir_readEntries(
        iterator: *mut uio_NativeEntriesContext,
        buf: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn uio_GPDir_closeEntries(iterator: uio_NativeEntriesContext);
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
    fn uio_GPDir_getGPDirEntry(
        gPDir: *mut uio_GPDir,
        name: *const ::core::ffi::c_char,
    ) -> *mut uio_GPDirEntry;
    fn uio_GPDir_getPDirEntryHandle(
        pDirHandle: *const uio_PDirHandle,
        name: *const ::core::ffi::c_char,
    ) -> *mut uio_PDirEntryHandle;
    fn uio_walkGPPath(
        startGPDir: *mut uio_GPDir,
        path: *const ::core::ffi::c_char,
        pathLen: size_t,
        endGPDir: *mut *mut uio_GPDir,
        pathRest: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_GPDirHandle_delete(gPDirHandle: *mut uio_GPDirHandle);
    fn uio_GPFileHandle_delete(gPFileHandle: *mut uio_GPFileHandle);
    fn uio_GPRoot_delete(gPRoot: *mut uio_GPRoot);
    fn uio_GPFile_new(
        pRoot: *mut uio_PRoot,
        extra: uio_GPFileExtra,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_GPFile;
    fn uio_GPFile_delete(gPFile: *mut uio_GPFile);
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
    fn uio_fstat(handle: *mut uio_Handle, statBuf: *mut stat) -> ::core::ffi::c_int;
    fn inflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn uio_openFileBlock(handle: *mut uio_Handle) -> *mut uio_FileBlock;
    fn uio_openFileBlock2(
        handle: *mut uio_Handle,
        offset: off_t,
        size: size_t,
    ) -> *mut uio_FileBlock;
    fn uio_accessFileBlock(
        block: *mut uio_FileBlock,
        offset: off_t,
        length: size_t,
        buffer: *mut *mut ::core::ffi::c_char,
    ) -> ssize_t;
    fn uio_copyFileBlock(
        block: *mut uio_FileBlock,
        offset: off_t,
        buffer: *mut ::core::ffi::c_char,
        length: size_t,
    ) -> ::core::ffi::c_int;
    fn uio_closeFileBlock(block: *mut uio_FileBlock) -> ::core::ffi::c_int;
    fn uio_setFileBlockUsageHint(
        block: *mut uio_FileBlock,
        usage: ::core::ffi::c_int,
        readAheadBufSize: size_t,
    );
    fn dosToUnixTime(date: uio_uint16, tm: uio_uint16) -> time_t;
    fn uio_getFileSystemHandler(id: uio_FileSystemID) -> *mut uio_FileSystemHandler;
    fn uio_Handle_new(
        root: *mut uio_PRoot,
        native: uio_NativeHandle,
        openFlags: ::core::ffi::c_int,
    ) -> *mut uio_Handle;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn uio_PRoot_getRootDirHandle(pRoot: *mut uio_PRoot) -> *mut uio_PDirHandle;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type FILE = _IO_FILE;
pub type mode_t = ::core::ffi::c_uint;
pub type time_t = ::core::ffi::c_longlong;
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
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
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
pub struct zip_Handle {
    pub file: *mut uio_GPFile,
    pub zipStream: z_stream,
    pub fileBlock: *mut uio_FileBlock,
    pub uncompressedOffset: off_t,
    pub compressedOffset: off_t,
}
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = ::core::ffi::c_ulong;
pub type voidpf = *mut ::core::ffi::c_void;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type uInt = ::core::ffi::c_uint;
pub type Bytef = Byte;
pub type Byte = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPFile {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub extra: uio_GPFileExtra,
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
pub type uio_PRootExtra = *mut uio_GPRoot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPRoot {
    pub flags: ::core::ffi::c_int,
    pub ops: *mut uio_GPRoot_Operations,
    pub extra: uio_GPRootExtra,
}
pub type uio_GPRootExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPRoot_Operations {
    pub fillGPDir: Option<unsafe extern "C" fn(*mut uio_GPDir) -> ()>,
    pub deleteGPRootExtra: Option<unsafe extern "C" fn(uio_GPRootExtra) -> ()>,
    pub deleteGPDirExtra: Option<unsafe extern "C" fn(uio_GPDirExtra) -> ()>,
    pub deleteGPFileExtra: Option<unsafe extern "C" fn(uio_GPFileExtra) -> ()>,
}
pub type uio_GPFileExtra = *mut zip_GPFileData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_GPFileData {
    pub compressedSize: off_t,
    pub uncompressedSize: off_t,
    pub compressionFlags: uio_uint16,
    pub compressionMethod: uio_uint16,
    pub headerOffset: off_t,
    pub fileOffset: off_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub mode: mode_t,
    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,
}
pub type uio_uint16 = ::core::ffi::c_ushort;
pub type uio_GPDirExtra = *mut zip_GPFileData;
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
pub struct uio_Handle {
    pub r#ref: ::core::ffi::c_int,
    pub root: *mut uio_PRoot,
    pub native: uio_NativeHandle,
    pub openFlags: ::core::ffi::c_int,
}
pub type uio_NativeHandle = *mut zip_Handle;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_GPDirEntry {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub extra: *mut ::core::ffi::c_void,
}
pub type uio_uint8 = ::core::ffi::c_uchar;
pub type uio_sint32 = ::core::ffi::c_int;
pub type uio_FileSystemID = ::core::ffi::c_int;
pub type z_streamp = *mut z_stream;
pub type zip_GPDirData = zip_GPFileData;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zip_OSType(pub ::core::ffi::c_uint);
impl zip_OSType {
    pub const zip_OSType_FAT: Self = Self(0);
    pub const zip_OSType_Amiga: Self = Self(1);
    pub const zip_OSType_OpenVMS: Self = Self(2);
    pub const zip_OSType_UNIX: Self = Self(3);
    pub const zip_OSType_VMCMS: Self = Self(4);
    pub const zip_OSType_AtariST: Self = Self(5);
    pub const zip_OSType_HPFS: Self = Self(6);
    pub const zip_OSType_HFS: Self = Self(7);
    pub const zip_OSType_ZSystem: Self = Self(8);
    pub const zip_OSType_CPM: Self = Self(9);
    pub const zip_OSType_TOPS20: Self = Self(10);
    pub const zip_OSType_NTFS: Self = Self(11);
    pub const zip_OSType_QDOS: Self = Self(12);
    pub const zip_OSType_Acorn: Self = Self(13);
    pub const zip_OSType_VFAT: Self = Self(14);
    pub const zip_OSType_MVS: Self = Self(15);
    pub const zip_OSType_BeOS: Self = Self(16);
    pub const zip_OSType_Tandem: Self = Self(17);
    pub const zip_numOSTypes: Self = Self(18);
}
pub type zip_readFunctionType = Option<
    unsafe extern "C" fn(*mut uio_Handle, *mut ::core::ffi::c_void, size_t) -> ssize_t,
>;
pub type zip_seekFunctionType = Option<
    unsafe extern "C" fn(*mut uio_Handle, off_t) -> off_t,
>;
#[inline]
unsafe extern "C" fn uio_GPDirEntry_isDir(
    mut gPDirEntry: *mut uio_GPDirEntry,
) -> uio_bool {
    return (*gPDirEntry).flags & 0x1 as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn uio_GPFile_ref(mut gPFile: *mut uio_GPFile) {
    (*gPFile).r#ref += 1;
}
#[inline]
unsafe extern "C" fn uio_GPFile_unref(mut gPFile: *mut uio_GPFile) {
    ((*gPFile).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"gPFile->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/zip/../gphys.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                302 as ::core::ffi::c_int,
                b"uio_GPFile_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*gPFile).r#ref -= 1;
    if (*gPFile).r#ref == 0 as ::core::ffi::c_int {
        uio_GPFile_delete(gPFile);
    }
}
#[no_mangle]
pub static mut zip_fileSystemHandler: uio_FileSystemHandler = uio_FileSystemHandler {
    init: None,
    unInit: None,
    cleanup: None,
    mount: Some(
        zip_mount
            as unsafe extern "C" fn(
                *mut uio_Handle,
                ::core::ffi::c_int,
            ) -> *mut uio_PRoot,
    ),
    umount: Some(
        uio_GPRoot_umount as unsafe extern "C" fn(*mut uio_PRoot) -> ::core::ffi::c_int,
    ),
    access: Some(
        zip_access
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    close: Some(zip_close as unsafe extern "C" fn(*mut uio_Handle) -> ()),
    fstat: Some(
        zip_fstat
            as unsafe extern "C" fn(*mut uio_Handle, *mut stat) -> ::core::ffi::c_int,
    ),
    stat: Some(
        zip_stat
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                *mut stat,
            ) -> ::core::ffi::c_int,
    ),
    mkdir: None,
    open: Some(
        zip_open
            as unsafe extern "C" fn(
                *mut uio_PDirHandle,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                mode_t,
            ) -> *mut uio_Handle,
    ),
    read: Some(
        zip_read
            as unsafe extern "C" fn(
                *mut uio_Handle,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> ssize_t,
    ),
    rename: None,
    rmdir: None,
    seek: Some(
        zip_seek
            as unsafe extern "C" fn(*mut uio_Handle, off_t, ::core::ffi::c_int) -> off_t,
    ),
    write: None,
    unlink: None,
    openEntries: Some(
        uio_GPDir_openEntries
            as unsafe extern "C" fn(*mut uio_PDirHandle) -> uio_NativeEntriesContext,
    ),
    readEntries: Some(
        uio_GPDir_readEntries
            as unsafe extern "C" fn(
                *mut uio_NativeEntriesContext,
                *mut ::core::ffi::c_char,
                size_t,
            ) -> ::core::ffi::c_int,
    ),
    closeEntries: Some(
        uio_GPDir_closeEntries as unsafe extern "C" fn(uio_NativeEntriesContext) -> (),
    ),
    getPDirEntryHandle: Some(
        uio_GPDir_getPDirEntryHandle
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
pub static mut zip_GPRootOperations: uio_GPRoot_Operations = uio_GPRoot_Operations {
    fillGPDir: None,
    deleteGPRootExtra: None,
    deleteGPDirExtra: Some(
        zip_GPDirData_delete as unsafe extern "C" fn(*mut zip_GPDirData) -> (),
    ),
    deleteGPFileExtra: Some(
        zip_GPFileData_delete as unsafe extern "C" fn(*mut zip_GPFileData) -> (),
    ),
};
static mut zip_compressionMethodSupported: [uio_bool; 11] = [
    true, false, false, false, false, false, false, false, true, false, false,
];
#[no_mangle]
pub static mut zip_readMethods: [zip_readFunctionType; 11] = [
    Some(
        zip_readStored
            as unsafe extern "C" fn(
                *mut uio_Handle,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> ssize_t,
    ),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(
        zip_readDeflated
            as unsafe extern "C" fn(
                *mut uio_Handle,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> ssize_t,
    ),
    None,
    None,
];
#[no_mangle]
pub static mut zip_seekMethods: [zip_seekFunctionType; 11] = [
    Some(zip_seekStored as unsafe extern "C" fn(*mut uio_Handle, off_t) -> off_t),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(zip_seekDeflated as unsafe extern "C" fn(*mut uio_Handle, off_t) -> off_t),
    None,
    None,
];
#[no_mangle]
pub unsafe extern "C" fn zip_close(mut handle: *mut uio_Handle) {
    let mut zip_handle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    zip_handle = (*handle).native as *mut zip_Handle;
    uio_GPFile_unref((*zip_handle).file);
    zip_unInitZipStream(&raw mut (*zip_handle).zipStream);
    uio_closeFileBlock((*zip_handle).fileBlock);
    free(zip_handle as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn zip_fillStat(
    mut statBuf: *mut stat,
    mut gPFileData: *const zip_GPFileData,
) {
    memset(
        statBuf as *mut ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<stat>(),
    );
    (*statBuf).st_size = (*gPFileData).uncompressedSize;
    (*statBuf).st_uid = (*gPFileData).uid;
    (*statBuf).st_gid = (*gPFileData).gid;
    (*statBuf).st_mode = (*gPFileData).mode;
    (*statBuf).st_atim.tv_sec = (*gPFileData).atime;
    (*statBuf).st_mtim.tv_sec = (*gPFileData).mtime;
    (*statBuf).st_ctim.tv_sec = (*gPFileData).ctime;
}
#[no_mangle]
pub unsafe extern "C" fn zip_access(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *__errno_location() = 52 as ::core::ffi::c_int;
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zip_fstat(
    mut handle: *mut uio_Handle,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    if (*(*(*(*handle).native).file).extra).fileOffset == -1 as off_t {
        if zip_updateFileDataFromLocalHeader(
            (*(*handle).root).handle,
            (*(*(*handle).native).file).extra,
        ) == -1 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    zip_fillStat(statBuf, (*(*(*handle).native).file).extra as *const zip_GPFileData);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zip_stat(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
    mut statBuf: *mut stat,
) -> ::core::ffi::c_int {
    let mut entry: *mut uio_GPDirEntry = ::core::ptr::null_mut::<uio_GPDirEntry>();
    if *name.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        && *name.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        entry = (*pDirHandle).extra as *mut uio_GPDirEntry;
    } else {
        entry = uio_GPDir_getGPDirEntry((*pDirHandle).extra, name);
        if entry.is_null() {
            *__errno_location() = 44 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
    }
    if uio_GPDirEntry_isDir(entry) as ::core::ffi::c_int != 0 && (*entry).extra.is_null()
    {
        memset(
            statBuf as *mut ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            ::core::mem::size_of::<stat>(),
        );
        (*statBuf).st_mode = (0o40000 as ::core::ffi::c_int | 0o400 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o40 as ::core::ffi::c_int | 0o20 as ::core::ffi::c_int
            | 0o1 as ::core::ffi::c_int | 0o4 as ::core::ffi::c_int
            | 0o2 as ::core::ffi::c_int | 0o1 as ::core::ffi::c_int) as mode_t;
        (*statBuf).st_uid = 0 as uid_t;
        (*statBuf).st_gid = 0 as gid_t;
        return 0 as ::core::ffi::c_int;
    }
    zip_fillStat(statBuf, (*entry).extra as *mut zip_GPFileData);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zip_open(
    mut pDirHandle: *mut uio_PDirHandle,
    mut name: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mode: mode_t,
) -> *mut uio_Handle {
    let mut handle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    let mut gPFile: *mut uio_GPFile = ::core::ptr::null_mut::<uio_GPFile>();
    if flags & (0o3 as ::core::ffi::c_int | 0o10000000 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        *__errno_location() = 2 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    gPFile = uio_GPDir_getGPDirEntry((*pDirHandle).extra, name) as *mut uio_GPFile;
    if gPFile.is_null() {
        *__errno_location() = 44 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    if (*(*gPFile).extra).fileOffset == -1 as off_t {
        if zip_updateFileDataFromLocalHeader(
            (*(*pDirHandle).pRoot).handle,
            (*gPFile).extra,
        ) == -1 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<uio_Handle>();
        }
    }
    handle = malloc(::core::mem::size_of::<zip_Handle>()) as *mut zip_Handle;
    uio_GPFile_ref(gPFile);
    (*handle).file = gPFile;
    (*handle).fileBlock = uio_openFileBlock2(
        (*(*pDirHandle).pRoot).handle,
        (*(*gPFile).extra).fileOffset,
        (*(*gPFile).extra).compressedSize as size_t,
    );
    if (*handle).fileBlock.is_null() {
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    if zip_initZipStream(&raw mut (*handle).zipStream) == -1 as ::core::ffi::c_int {
        uio_GPFile_unref(gPFile);
        uio_closeFileBlock((*handle).fileBlock);
        return ::core::ptr::null_mut::<uio_Handle>();
    }
    (*handle).compressedOffset = 0 as off_t;
    (*handle).uncompressedOffset = 0 as off_t;
    return uio_Handle_new((*pDirHandle).pRoot, handle, flags);
}
#[no_mangle]
pub unsafe extern "C" fn zip_read(
    mut handle: *mut uio_Handle,
    mut buf: *mut ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut result: ssize_t = 0;
    result = zip_readMethods[(*(*(*(*handle).native).file).extra).compressionMethod
            as usize]
        .expect("non-null function pointer")(handle, buf, count);
    return result;
}
unsafe extern "C" fn zip_readStored(
    mut handle: *mut uio_Handle,
    mut buf: *mut ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut numBytes: ::core::ffi::c_int = 0;
    let mut zipHandle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    zipHandle = (*handle).native as *mut zip_Handle;
    numBytes = uio_copyFileBlock(
        (*zipHandle).fileBlock,
        (*zipHandle).uncompressedOffset,
        buf as *mut ::core::ffi::c_char,
        count,
    );
    if numBytes == -1 as ::core::ffi::c_int {
        return -1 as ssize_t;
    }
    (*zipHandle).uncompressedOffset += numBytes as off_t;
    (*zipHandle).compressedOffset += numBytes as off_t;
    return numBytes as ssize_t;
}
unsafe extern "C" fn zip_readDeflated(
    mut handle: *mut uio_Handle,
    mut buf: *mut ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut zipHandle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    let mut inflateResult: ::core::ffi::c_int = 0;
    zipHandle = (*handle).native as *mut zip_Handle;
    if count as off_t
        > (*(*(*zipHandle).file).extra).uncompressedSize
            - (*zipHandle).zipStream.total_out as off_t
    {
        count = ((*(*(*zipHandle).file).extra).uncompressedSize
            - (*zipHandle).zipStream.total_out as off_t) as size_t;
    }
    (*zipHandle).zipStream.next_out = buf as *mut Bytef;
    (*zipHandle).zipStream.avail_out = count as uInt;
    while (*zipHandle).zipStream.avail_out > 0 as uInt {
        if (*zipHandle).zipStream.avail_in == 0 as uInt {
            let mut numBytes: ssize_t = 0;
            numBytes = uio_accessFileBlock(
                (*zipHandle).fileBlock,
                (*zipHandle).compressedOffset,
                0x10000 as ::core::ffi::c_int as size_t,
                &raw mut (*zipHandle).zipStream.next_in as *mut *mut ::core::ffi::c_char,
            );
            if numBytes == -1 as ssize_t {
                return -1 as ssize_t;
            }
            (*zipHandle).zipStream.avail_in = numBytes as uInt;
            (*zipHandle).compressedOffset += numBytes as off_t;
        }
        inflateResult = inflate(
            &raw mut (*zipHandle).zipStream,
            2 as ::core::ffi::c_int,
        );
        (*zipHandle).uncompressedOffset = (*zipHandle).zipStream.total_out as off_t;
        if inflateResult == 1 as ::core::ffi::c_int {
            break;
        }
        if inflateResult != 0 as ::core::ffi::c_int {
            match inflateResult {
                -6 => {
                    fprintf(
                        stderr,
                        b"Error: Incompatible version problem for  decompression.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                2 => {
                    fprintf(
                        stderr,
                        b"Error: Decompressing requires preset dictionary.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                -3 => {
                    fprintf(
                        stderr,
                        b"Error: Compressed file is corrupted.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                -2 => {
                    fprintf(
                        stderr,
                        b"Fatal: internal error using zlib.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                }
                -4 => {
                    fprintf(
                        stderr,
                        b"Error: Not enough memory available while decompressing.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                -5 => {
                    fprintf(
                        stderr,
                        b"Error: When decompressing: premature end of input file.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    *__errno_location() = 29 as ::core::ffi::c_int;
                    return -1 as ssize_t;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Fatal: unknown error from inflate().\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                }
            }
            if !(*zipHandle).zipStream.msg.is_null() {
                fprintf(
                    stderr,
                    b"ZLib reports: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*zipHandle).zipStream.msg,
                );
            }
            *__errno_location() = 29 as ::core::ffi::c_int;
            return -1 as ssize_t;
        }
    }
    return count.wrapping_sub((*zipHandle).zipStream.avail_out as size_t) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn zip_seek(
    mut handle: *mut uio_Handle,
    mut offset: off_t,
    mut whence: ::core::ffi::c_int,
) -> off_t {
    let mut zipHandle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    zipHandle = (*handle).native as *mut zip_Handle;
    (whence == 0 as ::core::ffi::c_int || whence == 1 as ::core::ffi::c_int
        || whence == 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"whence == SEEK_SET || whence == SEEK_CUR || whence == SEEK_END\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/zip/zip.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int,
                b"zip_seek\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    match whence {
        1 => {
            offset += (*zipHandle).uncompressedOffset;
        }
        2 => {
            offset += (*(*(*zipHandle).file).extra).uncompressedSize;
        }
        0 | _ => {}
    }
    if offset < 0 as off_t {
        offset = 0 as off_t;
    } else if offset > (*(*(*zipHandle).file).extra).uncompressedSize {
        offset = (*(*(*zipHandle).file).extra).uncompressedSize;
    }
    return zip_seekMethods[(*(*(*(*handle).native).file).extra).compressionMethod
            as usize]
        .expect("non-null function pointer")(handle, offset);
}
unsafe extern "C" fn zip_seekStored(
    mut handle: *mut uio_Handle,
    mut offset: off_t,
) -> off_t {
    let mut zipHandle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    zipHandle = (*handle).native as *mut zip_Handle;
    if offset > (*(*(*zipHandle).file).extra).uncompressedSize {
        offset = (*(*(*zipHandle).file).extra).uncompressedSize;
    }
    (*zipHandle).compressedOffset = offset;
    (*zipHandle).uncompressedOffset = offset;
    return offset;
}
unsafe extern "C" fn zip_seekDeflated(
    mut handle: *mut uio_Handle,
    mut offset: off_t,
) -> off_t {
    let mut zipHandle: *mut zip_Handle = ::core::ptr::null_mut::<zip_Handle>();
    zipHandle = (*handle).native as *mut zip_Handle;
    if offset < (*zipHandle).uncompressedOffset {
        if zip_reInitZipStream(&raw mut (*zipHandle).zipStream)
            == -1 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Fatal: Could not reinitialise zip stream: %s.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            abort();
        }
        (*zipHandle).compressedOffset = 0 as off_t;
        (*zipHandle).uncompressedOffset = 0 as off_t;
    }
    if offset == (*zipHandle).uncompressedOffset {
        return offset;
    }
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut numRead: ssize_t = 0;
    let mut toRead: size_t = 0;
    buffer = malloc(0x10000 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_char;
    toRead = (offset - (*zipHandle).uncompressedOffset) as size_t;
    while toRead > 0 as size_t {
        numRead = zip_read(
            handle,
            buffer as *mut ::core::ffi::c_void,
            if toRead < 0x10000 as ::core::ffi::c_int as size_t {
                toRead
            } else {
                0x10000 as ::core::ffi::c_int as size_t
            },
        );
        if numRead == -1 as ssize_t {
            fprintf(
                stderr,
                b"Warning: Could not read zipped file: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            break;
        } else {
            toRead = toRead.wrapping_sub(numRead as size_t);
        }
    }
    free(buffer as *mut ::core::ffi::c_void);
    return (*zipHandle).uncompressedOffset;
}
#[no_mangle]
pub unsafe extern "C" fn zip_mount(
    mut handle: *mut uio_Handle,
    mut flags: ::core::ffi::c_int,
) -> *mut uio_PRoot {
    let mut result: *mut uio_PRoot = ::core::ptr::null_mut::<uio_PRoot>();
    let mut rootDirHandle: *mut uio_PDirHandle = ::core::ptr::null_mut::<
        uio_PDirHandle,
    >();
    if flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
    {
        *__errno_location() = 2 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<uio_PRoot>();
    }
    uio_Handle_ref(handle);
    result = uio_GPRoot_makePRoot(
        uio_getFileSystemHandler(2 as uio_FileSystemID),
        flags,
        &raw mut zip_GPRootOperations,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0x4000 as ::core::ffi::c_int,
        handle,
        ::core::ptr::null_mut::<zip_GPFileData>(),
        0x1000 as ::core::ffi::c_int,
    );
    rootDirHandle = uio_PRoot_getRootDirHandle(result);
    if zip_fillDirStructure((*rootDirHandle).extra, handle) == -1 as ::core::ffi::c_int {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        fprintf(
            stderr,
            b"Error: failed to read the zip directory structure - %s.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        uio_GPRoot_umount(result);
        *__errno_location() = savedErrno;
        return ::core::ptr::null_mut::<uio_PRoot>();
    }
    return result;
}
unsafe extern "C" fn zip_fillDirStructure(
    mut top: *mut uio_GPDir,
    mut handle: *mut uio_Handle,
) -> ::core::ffi::c_int {
    return zip_fillDirStructureCentral(top, handle);
}
unsafe extern "C" fn zip_fillDirStructureCentral(
    mut top: *mut uio_GPDir,
    mut handle: *mut uio_Handle,
) -> ::core::ffi::c_int {
    let mut fileBlock: *mut uio_FileBlock = ::core::ptr::null_mut::<uio_FileBlock>();
    let mut pos: off_t = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut numBytes: ssize_t = 0;
    let mut numEntries: uio_uint16 = 0;
    let mut eocdr: off_t = 0;
    let mut startCentralDir: off_t = 0;
    fileBlock = uio_openFileBlock(handle);
    '_err: {
        if !fileBlock.is_null() {
            eocdr = zip_findEndOfCentralDirectoryRecord(handle, fileBlock);
            if eocdr != -1 as off_t {
                numBytes = uio_accessFileBlock(
                    fileBlock,
                    eocdr,
                    22 as size_t,
                    &raw mut buf,
                );
                if numBytes != -1 as ssize_t {
                    if numBytes != 22 as ssize_t {
                        *__errno_location() = 29 as ::core::ffi::c_int;
                    } else {
                        numEntries = makeUInt16(
                            *buf.offset(10isize) as uio_uint8,
                            *buf.offset(11isize) as uio_uint8,
                        );
                        if numEntries as ::core::ffi::c_int
                            == 0xffff as ::core::ffi::c_int
                        {
                            fprintf(
                                stderr,
                                b"Error: Zip64 .zip files are not supported.\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            *__errno_location() = 52 as ::core::ffi::c_int;
                        } else {
                            startCentralDir = makeUInt32(
                                *buf.offset(16isize) as uio_uint8,
                                *buf.offset(17isize) as uio_uint8,
                                *buf.offset(18isize) as uio_uint8,
                                *buf.offset(19isize) as uio_uint8,
                            ) as off_t;
                            uio_setFileBlockUsageHint(
                                fileBlock,
                                1 as ::core::ffi::c_int,
                                0x10000 as ::core::ffi::c_int as size_t,
                            );
                            pos = startCentralDir;
                            loop {
                                let c2rust_fresh0 = numEntries;
                                numEntries = numEntries.wrapping_sub(1);
                                if c2rust_fresh0 == 0 {
                                    break;
                                }
                                if zip_fillDirStructureCentralProcessEntry(
                                    top,
                                    fileBlock,
                                    &raw mut pos,
                                ) == -1 as ::core::ffi::c_int
                                {
                                    break '_err;
                                }
                            }
                            uio_closeFileBlock(fileBlock);
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    let mut savedErrno: ::core::ffi::c_int = *__errno_location();
    if !fileBlock.is_null() {
        uio_closeFileBlock(fileBlock);
    }
    *__errno_location() = savedErrno;
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_fillDirStructureCentralProcessEntry(
    mut topGPDir: *mut uio_GPDir,
    mut fileBlock: *mut uio_FileBlock,
    mut pos: *mut off_t,
) -> ::core::ffi::c_int {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut gPFileData: *mut zip_GPFileData = ::core::ptr::null_mut::<zip_GPFileData>();
    let mut numBytes: ssize_t = 0;
    let mut signature: uio_uint32 = 0;
    let mut lastModTime: uio_uint16 = 0;
    let mut lastModDate: uio_uint16 = 0;
    let mut crc: uio_uint32 = 0;
    let mut fileNameLength: uio_uint16 = 0;
    let mut extraFieldLength: uio_uint16 = 0;
    let mut fileCommentLength: uio_uint16 = 0;
    let mut fileName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut creatorOS: zip_OSType = zip_OSType::zip_OSType_FAT;
    let mut nextEntryOffset: off_t = 0;
    numBytes = uio_accessFileBlock(fileBlock, *pos, 46 as size_t, &raw mut buf);
    if numBytes != 46 as ssize_t {
        return zip_badFile(
            ::core::ptr::null_mut::<zip_GPFileData>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    signature = makeUInt32(
        *buf.offset(0isize) as uio_uint8,
        *buf.offset(1isize) as uio_uint8,
        *buf.offset(2isize) as uio_uint8,
        *buf.offset(3isize) as uio_uint8,
    );
    if signature != 0x2014b50 as ::core::ffi::c_int as uio_uint32 {
        fprintf(
            stderr,
            b"Error: Premature end of central directory.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        *__errno_location() = 29 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    gPFileData = zip_GPFileData_new();
    creatorOS = zip_OSType(*buf.offset(5isize) as ::core::ffi::c_uint);
    (*gPFileData).compressionFlags = makeUInt16(
        *buf.offset(8isize) as uio_uint8,
        *buf.offset(9isize) as uio_uint8,
    );
    (*gPFileData).compressionMethod = makeUInt16(
        *buf.offset(10isize) as uio_uint8,
        *buf.offset(11isize) as uio_uint8,
    );
    lastModTime = makeUInt16(
        *buf.offset(12isize) as uio_uint8,
        *buf.offset(13isize) as uio_uint8,
    );
    lastModDate = makeUInt16(
        *buf.offset(14isize) as uio_uint8,
        *buf.offset(15isize) as uio_uint8,
    );
    (*gPFileData).atime = 0 as ::core::ffi::c_int as time_t;
    (*gPFileData).mtime = dosToUnixTime(lastModDate, lastModTime);
    (*gPFileData).ctime = 0 as ::core::ffi::c_int as time_t;
    crc = makeUInt32(
        *buf.offset(16isize) as uio_uint8,
        *buf.offset(17isize) as uio_uint8,
        *buf.offset(18isize) as uio_uint8,
        *buf.offset(19isize) as uio_uint8,
    );
    (*gPFileData).compressedSize = makeUInt32(
        *buf.offset(20isize) as uio_uint8,
        *buf.offset(21isize) as uio_uint8,
        *buf.offset(22isize) as uio_uint8,
        *buf.offset(23isize) as uio_uint8,
    ) as off_t;
    (*gPFileData).uncompressedSize = makeUInt32(
        *buf.offset(24isize) as uio_uint8,
        *buf.offset(25isize) as uio_uint8,
        *buf.offset(26isize) as uio_uint8,
        *buf.offset(27isize) as uio_uint8,
    ) as off_t;
    fileNameLength = makeUInt16(
        *buf.offset(28isize) as uio_uint8,
        *buf.offset(29isize) as uio_uint8,
    );
    extraFieldLength = makeUInt16(
        *buf.offset(30isize) as uio_uint8,
        *buf.offset(31isize) as uio_uint8,
    );
    fileCommentLength = makeUInt16(
        *buf.offset(32isize) as uio_uint8,
        *buf.offset(33isize) as uio_uint8,
    );
    (*gPFileData).uid = 0 as uid_t;
    (*gPFileData).gid = 0 as gid_t;
    (*gPFileData).mode = zip_makeFileMode(
        creatorOS,
        makeUInt32(
            *buf.offset(38isize) as uio_uint8,
            *buf.offset(39isize) as uio_uint8,
            *buf.offset(40isize) as uio_uint8,
            *buf.offset(41isize) as uio_uint8,
        ),
    );
    (*gPFileData).headerOffset = makeSInt32(
        *buf.offset(42isize) as uio_uint8,
        *buf.offset(43isize) as uio_uint8,
        *buf.offset(44isize) as uio_uint8,
        *buf.offset(45isize) as uio_uint8,
    ) as off_t;
    (*gPFileData).fileOffset = -1 as ::core::ffi::c_int as off_t;
    *pos += 46 as off_t;
    nextEntryOffset = *pos + fileNameLength as off_t + extraFieldLength as off_t
        + fileCommentLength as off_t;
    numBytes = uio_accessFileBlock(
        fileBlock,
        *pos,
        fileNameLength as size_t,
        &raw mut buf,
    );
    if numBytes != fileNameLength as ssize_t {
        return zip_badFile(gPFileData, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    fileName = malloc(
        (fileNameLength as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut ::core::ffi::c_char;
    memcpy(
        fileName as *mut ::core::ffi::c_void,
        buf as *const ::core::ffi::c_void,
        fileNameLength as size_t,
    );
    *fileName.offset(fileNameLength as isize) = '\0' as ::core::ffi::c_char;
    *pos += fileNameLength as off_t;
    if (*gPFileData).compressionMethod as ::core::ffi::c_int >= 11 as ::core::ffi::c_int
        || !zip_compressionMethodSupported[(*gPFileData).compressionMethod as usize]
    {
        fprintf(
            stderr,
            b"Warning: File '%s' is compressed with unsupported method %d - skipped.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            fileName,
            (*gPFileData).compressionMethod as ::core::ffi::c_int,
        );
        *pos = nextEntryOffset;
        zip_GPFileData_delete(gPFileData);
        return 0 as ::core::ffi::c_int;
    }
    if (*gPFileData).compressedSize == 0xffffffff as ::core::ffi::c_uint as off_t
        || (*gPFileData).uncompressedSize == 0xffffffff as ::core::ffi::c_uint as off_t
        || (*gPFileData).headerOffset < 0 as off_t
    {
        fprintf(
            stderr,
            b"Warning: Skipping Zip64 file '%s'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            fileName,
        );
        *pos = nextEntryOffset;
        zip_GPFileData_delete(gPFileData);
        return 0 as ::core::ffi::c_int;
    }
    if isBitSet((*gPFileData).compressionFlags as uio_uint32, 0 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"Warning: Skipping encrypted file '%s'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            fileName,
        );
        *pos = nextEntryOffset;
        zip_GPFileData_delete(gPFileData);
        return 0 as ::core::ffi::c_int;
    }
    match zip_fillDirStructureProcessExtraFields(
        fileBlock,
        extraFieldLength as off_t,
        gPFileData,
        fileName,
        *pos,
        true,
    ) {
        1 => {
            *pos = nextEntryOffset;
            zip_GPFileData_delete(gPFileData);
            free(fileName as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
        -1 => return zip_badFile(gPFileData, fileName),
        0 | _ => {}
    }
    *pos += extraFieldLength as off_t;
    if (*gPFileData).mode & 0o170000 as mode_t == 0o100000 as mode_t {
        if zip_foundFile(topGPDir, fileName, gPFileData) == -1 as ::core::ffi::c_int {
            if *__errno_location() == 31 as ::core::ffi::c_int {
                zip_GPFileData_delete(gPFileData);
                free(fileName as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
            return zip_badFile(gPFileData, fileName);
        }
    } else if (*gPFileData).mode & 0o170000 as mode_t == 0o40000 as mode_t {
        if *fileName
            .offset(
                (fileNameLength as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        {
            *fileName
                .offset(
                    (fileNameLength as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as isize,
                ) = '\0' as ::core::ffi::c_char;
        }
        if zip_foundDir(topGPDir, fileName, gPFileData) == -1 as ::core::ffi::c_int {
            if *__errno_location() == 31 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Warning: file '%s' already exists as a dir - skipped.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fileName,
                );
                zip_GPFileData_delete(gPFileData);
                free(fileName as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
            return zip_badFile(gPFileData, fileName);
        }
    } else {
        fprintf(
            stderr,
            b"Warning: '%s' is not a regular file, nor a directory - skipped.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            fileName,
        );
        zip_GPFileData_delete(gPFileData);
        free(fileName as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    free(fileName as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_findEndOfCentralDirectoryRecord(
    mut handle: *mut uio_Handle,
    mut fileBlock: *mut uio_FileBlock,
) -> off_t {
    let mut fileSize: off_t = 0;
    let mut endPos: off_t = 0;
    let mut startPos: off_t = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufLen: ssize_t = 0;
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
    if uio_fstat(handle, &raw mut statBuf) == -1 as ::core::ffi::c_int {
        return -1 as off_t;
    }
    fileSize = statBuf.st_size;
    startPos = fileSize - 0xffff as off_t - 22 as off_t;
    if startPos < 0 as off_t {
        startPos = 0 as off_t;
    }
    endPos = fileSize - 22 as off_t;
    bufLen = uio_accessFileBlock(
        fileBlock,
        startPos,
        (endPos - startPos + 4 as off_t) as size_t,
        &raw mut buf,
    );
    if bufLen == -1 as ssize_t {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        fprintf(
            stderr,
            b"Error: Read error while searching for 'end-of-central-directory record'.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        *__errno_location() = savedErrno;
        return -1 as off_t;
    }
    if bufLen as off_t != endPos - startPos + 4 as off_t {
        fprintf(
            stderr,
            b"Error: Read error while searching for 'end-of-central-directory record'.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        *__errno_location() = 29 as ::core::ffi::c_int;
        return -1 as off_t;
    }
    bufPtr = buf.offset((endPos - startPos) as isize);
    loop {
        if bufPtr < buf {
            fprintf(
                stderr,
                b"Error: Zip file corrupt; could not find 'end-of-central-directory record'.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            *__errno_location() = 29 as ::core::ffi::c_int;
            return -1 as off_t;
        }
        if *bufPtr.offset(0isize) as ::core::ffi::c_int == 0x50 as ::core::ffi::c_int
            && *bufPtr.offset(1isize) as ::core::ffi::c_int == 0x4b as ::core::ffi::c_int
            && *bufPtr.offset(2isize) as ::core::ffi::c_int == 0x5 as ::core::ffi::c_int
            && *bufPtr.offset(3isize) as ::core::ffi::c_int == 0x6 as ::core::ffi::c_int
        {
            break;
        }
        bufPtr = bufPtr.offset(-1);
    }
    return startPos + bufPtr.offset_from(buf) as off_t;
}
unsafe extern "C" fn zip_makeFileMode(
    mut creatorOS: zip_OSType,
    mut modeBytes: uio_uint32,
) -> mode_t {
    match creatorOS {
        zip_OSType::zip_OSType_FAT
        | zip_OSType::zip_OSType_NTFS
        | zip_OSType::zip_OSType_VFAT => {
            let mut mode: mode_t = 0;
            if modeBytes == 0 as uio_uint32 {
                return (0o100000 as ::core::ffi::c_int | 0o400 as ::core::ffi::c_int
                    | 0o200 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int
                    | 0o20 as ::core::ffi::c_int | 0o4 as ::core::ffi::c_int
                    | 0o2 as ::core::ffi::c_int) as mode_t;
            }
            if modeBytes & 0x10 as uio_uint32 != 0 {
                mode = (0o40000 as ::core::ffi::c_int | 0o400 as ::core::ffi::c_int
                    | 0o100 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int
                    | 0o10 as ::core::ffi::c_int | 0o4 as ::core::ffi::c_int
                    | 0o1 as ::core::ffi::c_int) as mode_t;
            } else {
                mode = (0o100000 as ::core::ffi::c_int | 0o400 as ::core::ffi::c_int
                    | 0o40 as ::core::ffi::c_int | 0o4 as ::core::ffi::c_int) as mode_t;
            }
            if modeBytes & 0x1 as uio_uint32 != 0 {
                return mode
            } else {
                return mode | 0o200 as mode_t | 0o20 as mode_t | 0o2 as mode_t
            }
        }
        zip_OSType::zip_OSType_UNIX => return modeBytes >> 16 as ::core::ffi::c_int,
        _ => {
            fprintf(
                stderr,
                b"Warning: file created by unknown OS.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return (0o100000 as ::core::ffi::c_int | 0o400 as ::core::ffi::c_int
                | 0o200 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int
                | 0o4 as ::core::ffi::c_int) as mode_t;
        }
    };
}
unsafe extern "C" fn zip_updatePFileDataFromLocalFileHeader(
    mut gPFileData: *mut zip_GPFileData,
    mut fileBlock: *mut uio_FileBlock,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut numBytes: ssize_t = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut signature: uio_uint32 = 0;
    let mut fileNameLength: uio_uint16 = 0;
    let mut extraFieldLength: uio_uint16 = 0;
    numBytes = uio_accessFileBlock(fileBlock, pos as off_t, 30 as size_t, &raw mut buf);
    if numBytes != 30 as ssize_t {
        *__errno_location() = 29 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    signature = makeUInt32(
        *buf.offset(0isize) as uio_uint8,
        *buf.offset(1isize) as uio_uint8,
        *buf.offset(2isize) as uio_uint8,
        *buf.offset(3isize) as uio_uint8,
    );
    if signature != 0x4034b50 as ::core::ffi::c_int as uio_uint32 {
        *__errno_location() = 29 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    fileNameLength = makeUInt16(
        *buf.offset(26isize) as uio_uint8,
        *buf.offset(27isize) as uio_uint8,
    );
    extraFieldLength = makeUInt16(
        *buf.offset(28isize) as uio_uint8,
        *buf.offset(29isize) as uio_uint8,
    );
    pos += 30 as ::core::ffi::c_int + fileNameLength as ::core::ffi::c_int;
    match zip_fillDirStructureProcessExtraFields(
        fileBlock,
        extraFieldLength as off_t,
        gPFileData,
        b"<unknown>\0".as_ptr() as *const ::core::ffi::c_char,
        pos as off_t,
        false,
    ) {
        1 => {
            fprintf(
                stderr,
                b"Warning: according to the central directory of a zip file, some file inside is acceptable, but according to the local header it isn't.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            *__errno_location() = 29 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        -1 => {
            *__errno_location() = 29 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        0 | _ => {}
    }
    pos += extraFieldLength as ::core::ffi::c_int;
    (*gPFileData).fileOffset = pos as off_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zip_updateFileDataFromLocalHeader(
    mut handle: *mut uio_Handle,
    mut gPFileData: *mut zip_GPFileData,
) -> ::core::ffi::c_int {
    let mut fileBlock: *mut uio_FileBlock = ::core::ptr::null_mut::<uio_FileBlock>();
    fileBlock = uio_openFileBlock(handle);
    if fileBlock.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if zip_updatePFileDataFromLocalFileHeader(
        gPFileData,
        fileBlock,
        (*gPFileData).headerOffset as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int
    {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        uio_closeFileBlock(fileBlock);
        *__errno_location() = savedErrno;
        return -1 as ::core::ffi::c_int;
    }
    if (*gPFileData).ctime == 0 as ::core::ffi::c_int as time_t {
        (*gPFileData).ctime = (*gPFileData).mtime;
    }
    if (*gPFileData).atime == 0 as ::core::ffi::c_int as time_t {
        (*gPFileData).atime = (*gPFileData).mtime;
    }
    uio_closeFileBlock(fileBlock);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_fillDirStructureProcessExtraFields(
    mut fileBlock: *mut uio_FileBlock,
    mut extraFieldLength: off_t,
    mut gPFileData: *mut zip_GPFileData,
    mut fileName: *const ::core::ffi::c_char,
    mut pos: off_t,
    mut central: uio_bool,
) -> ::core::ffi::c_int {
    let mut posEnd: off_t = 0;
    let mut headerID: uio_uint16 = 0;
    let mut dataSize: ssize_t = 0;
    let mut numBytes: ssize_t = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    posEnd = pos + extraFieldLength;
    while pos < posEnd {
        numBytes = uio_accessFileBlock(fileBlock, pos, 4 as size_t, &raw mut buf);
        if numBytes != 4 as ssize_t {
            return -1 as ::core::ffi::c_int;
        }
        headerID = makeUInt16(
            *buf.offset(0isize) as uio_uint8,
            *buf.offset(1isize) as uio_uint8,
        );
        dataSize = makeUInt16(
            *buf.offset(2isize) as uio_uint8,
            *buf.offset(3isize) as uio_uint8,
        ) as ssize_t;
        pos += 4 as off_t;
        numBytes = uio_accessFileBlock(fileBlock, pos, dataSize as size_t, &raw mut buf);
        if numBytes != dataSize {
            return -1 as ::core::ffi::c_int;
        }
        match headerID as ::core::ffi::c_int {
            0xd | 0x5855 => {
                (*gPFileData).atime = makeUInt32(
                    *buf.offset(0isize) as uio_uint8,
                    *buf.offset(1isize) as uio_uint8,
                    *buf.offset(2isize) as uio_uint8,
                    *buf.offset(3isize) as uio_uint8,
                ) as time_t;
                (*gPFileData).mtime = makeUInt32(
                    *buf.offset(4isize) as uio_uint8,
                    *buf.offset(5isize) as uio_uint8,
                    *buf.offset(6isize) as uio_uint8,
                    *buf.offset(7isize) as uio_uint8,
                ) as time_t;
                if !central {
                    if dataSize > 8 as ssize_t {
                        (*gPFileData).uid = makeUInt16(
                            *buf.offset(8isize) as uio_uint8,
                            *buf.offset(9isize) as uio_uint8,
                        ) as uid_t;
                        (*gPFileData).gid = makeUInt16(
                            *buf.offset(10isize) as uio_uint8,
                            *buf.offset(11isize) as uio_uint8,
                        ) as uid_t as gid_t;
                    }
                }
            }
            0x5455 => {
                let mut flags: uio_uint8 = 0;
                let mut bufPtr: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                flags = *buf.offset(0isize) as uio_uint8;
                bufPtr = buf.offset(1 as ::core::ffi::c_int as isize);
                if isBitSet(flags as uio_uint32, 0 as ::core::ffi::c_int) {
                    (*gPFileData).mtime = makeUInt32(
                        *bufPtr.offset(0isize) as uio_uint8,
                        *bufPtr.offset(1isize) as uio_uint8,
                        *bufPtr.offset(2isize) as uio_uint8,
                        *bufPtr.offset(3isize) as uio_uint8,
                    ) as time_t;
                    bufPtr = bufPtr.offset(4 as ::core::ffi::c_int as isize);
                }
                if !central {
                    if isBitSet(flags as uio_uint32, 1 as ::core::ffi::c_int) {
                        (*gPFileData).atime = makeUInt32(
                            *bufPtr.offset(0isize) as uio_uint8,
                            *bufPtr.offset(1isize) as uio_uint8,
                            *bufPtr.offset(2isize) as uio_uint8,
                            *bufPtr.offset(3isize) as uio_uint8,
                        ) as time_t;
                        bufPtr = bufPtr.offset(4 as ::core::ffi::c_int as isize);
                    }
                }
            }
            0x7855 => {
                if !central {
                    (*gPFileData).uid = makeUInt16(
                        *buf.offset(0isize) as uio_uint8,
                        *buf.offset(1isize) as uio_uint8,
                    ) as uid_t;
                    (*gPFileData).gid = makeUInt16(
                        *buf.offset(2isize) as uio_uint8,
                        *buf.offset(3isize) as uio_uint8,
                    ) as uid_t as gid_t;
                }
            }
            0x756e => {
                let mut mode: mode_t = 0;
                mode = makeUInt16(
                    *buf.offset(4isize) as uio_uint8,
                    *buf.offset(5isize) as uio_uint8,
                ) as mode_t;
                if !(mode & 0o170000 as mode_t == 0o100000 as mode_t)
                    && !(mode & 0o170000 as mode_t == 0o40000 as mode_t)
                {
                    fprintf(
                        stderr,
                        b"Warning: Skipping '%s'; not a regular file, nor a directory.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fileName,
                    );
                    return 1 as ::core::ffi::c_int;
                }
                (*gPFileData).uid = makeUInt16(
                    *buf.offset(10isize) as uio_uint8,
                    *buf.offset(11isize) as uio_uint8,
                ) as uid_t;
                (*gPFileData).gid = makeUInt16(
                    *buf.offset(12isize) as uio_uint8,
                    *buf.offset(13isize) as uio_uint8,
                ) as uid_t as gid_t;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Debug: Extra field 0x%04x unsupported, used for file '%s' - ignored.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    headerID as ::core::ffi::c_int,
                    fileName,
                );
            }
        }
        pos += dataSize as off_t;
    }
    if pos != posEnd {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_badFile(
    mut gPFileData: *mut zip_GPFileData,
    mut fileName: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    fprintf(
        stderr,
        b"Error: Bad file format for .zip file.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !gPFileData.is_null() {
        zip_GPFileData_delete(gPFileData);
    }
    if !fileName.is_null() {
        free(fileName as *mut ::core::ffi::c_void);
    }
    *__errno_location() = 28 as ::core::ffi::c_int;
    return -1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zip_foundFile(
    mut gPDir: *mut uio_GPDir,
    mut path: *const ::core::ffi::c_char,
    mut gPFileData: *mut zip_GPFileData,
) -> ::core::ffi::c_int {
    let mut file: *mut uio_GPFile = ::core::ptr::null_mut::<uio_GPFile>();
    let mut pathLen: size_t = 0;
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if *path.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        path = path.offset(1);
    }
    pathLen = strlen(path);
    if *path.offset(pathLen.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        == '/' as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Warning: '%s' is not a valid file name - skipped.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            path,
        );
        *__errno_location() = 31 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    pathEnd = path.offset(pathLen as isize);
    match uio_walkGPPath(gPDir, path, pathLen, &raw mut gPDir, &raw mut rest) {
        0 => {
            fprintf(
                stderr,
                b"Warning: '%s' already exists as a dir - skipped.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
            );
            *__errno_location() = 31 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        54 => {
            fprintf(
                stderr,
                b"Warning: A component to '%s' is not a directory - file skipped.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                path,
            );
            *__errno_location() = 54 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        44 | _ => {}
    }
    buf = malloc(pathLen.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    getFirstPathComponent(rest, pathEnd, &raw mut start, &raw mut end);
    loop {
        let mut newGPDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
        if end == start
            || end.offset_from(start) == 1isize
                && *start.offset(0isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
            || end.offset_from(start) == 2isize
                && *start.offset(0isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                && *start.offset(1isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Warning: file '%s' has an invalid path - skipped.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
            );
            free(buf as *mut ::core::ffi::c_void);
            *__errno_location() = 28 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        if end == pathEnd {
            rest = start;
            break;
        } else {
            memcpy(
                buf as *mut ::core::ffi::c_void,
                start as *const ::core::ffi::c_void,
                end.offset_from(start) as size_t,
            );
            *buf.offset(end.offset_from(start) as isize) = '\0' as ::core::ffi::c_char;
            newGPDir = uio_GPDir_prepareSubDir(gPDir, buf);
            (*newGPDir).flags |= 0x1000 as ::core::ffi::c_int;
            uio_GPDir_commitSubDir(gPDir, buf, newGPDir);
            gPDir = newGPDir;
            getNextPathComponent(pathEnd, &raw mut start, &raw mut end);
        }
    }
    free(buf as *mut ::core::ffi::c_void);
    file = uio_GPFile_new(
        (*gPDir).pRoot,
        gPFileData,
        uio_gPFileFlagsFromPRootFlags((*(*gPDir).pRoot).flags),
    );
    uio_GPDir_addFile(gPDir, rest, file);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn zip_foundDir(
    mut gPDir: *mut uio_GPDir,
    mut path: *const ::core::ffi::c_char,
    mut gPDirData: *mut zip_GPDirData,
) -> ::core::ffi::c_int {
    let mut pathLen: size_t = 0;
    let mut pathEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if *path.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        path = path.offset(1);
    }
    pathLen = strlen(path);
    pathEnd = path.offset(pathLen as isize);
    match uio_walkGPPath(gPDir, path, pathLen, &raw mut gPDir, &raw mut rest) {
        0 => {
            if !(*gPDir).extra.is_null() {
                fprintf(
                    stderr,
                    b"Warning: '%s' is present more than once in the zip file. The last entry will be used.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    path,
                );
                zip_GPDirData_delete((*gPDir).extra);
            }
            (*gPDir).extra = gPDirData as uio_GPDirExtra;
            return 0 as ::core::ffi::c_int;
        }
        54 => {
            fprintf(
                stderr,
                b"Warning: A component of '%s' is not a directory - file skipped.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                path,
            );
            *__errno_location() = 54 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        44 | _ => {}
    }
    buf = malloc(pathLen.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    getFirstPathComponent(rest, pathEnd, &raw mut start, &raw mut end);
    while start < pathEnd {
        let mut newGPDir: *mut uio_GPDir = ::core::ptr::null_mut::<uio_GPDir>();
        if end == start
            || end.offset_from(start) == 1isize
                && *start.offset(0isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
            || end.offset_from(start) == 2isize
                && *start.offset(0isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                && *start.offset(1isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"Warning: directory '%s' has an invalid path - skipped.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
            );
            free(buf as *mut ::core::ffi::c_void);
            *__errno_location() = 28 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            buf as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            end.offset_from(start) as size_t,
        );
        *buf.offset(end.offset_from(start) as isize) = '\0' as ::core::ffi::c_char;
        newGPDir = uio_GPDir_prepareSubDir(gPDir, buf);
        (*newGPDir).flags |= 0x1000 as ::core::ffi::c_int;
        uio_GPDir_commitSubDir(gPDir, buf, newGPDir);
        gPDir = newGPDir;
        getNextPathComponent(pathEnd, &raw mut start, &raw mut end);
    }
    (*gPDir).extra = gPDirData as uio_GPDirExtra;
    free(buf as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_initZipStream(
    mut zipStream: *mut z_stream,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    (*zipStream).next_in = ::core::ptr::null_mut::<Bytef>();
    (*zipStream).avail_in = 0 as uInt;
    (*zipStream).zalloc = Some(
        zip_alloc as unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf,
    ) as alloc_func;
    (*zipStream).zfree = Some(zip_free as unsafe extern "C" fn(voidpf, voidpf) -> ())
        as free_func;
    (*zipStream).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>() as voidpf;
    retVal = inflateInit2_(
        zipStream,
        -15 as ::core::ffi::c_int,
        b"1.2.13\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
    );
    if retVal != 0 as ::core::ffi::c_int {
        match retVal {
            -4 => {
                fprintf(
                    stderr,
                    b"Error: Not enough memory available for  decompression.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            -6 => {
                fprintf(
                    stderr,
                    b"Error: Incompatible version problem for  decompression.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"Fatal: unknown error from inflateInit().\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
        }
        if !(*zipStream).msg.is_null() {
            fprintf(
                stderr,
                b"ZLib reports: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*zipStream).msg,
            );
        }
        *__errno_location() = 29 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_unInitZipStream(
    mut zipStream: *mut z_stream,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    retVal = inflateEnd(zipStream);
    if retVal != 0 as ::core::ffi::c_int {
        match retVal {
            -2 => {
                fprintf(
                    stderr,
                    b"Fatal: internal error using zlib.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
            _ => {
                fprintf(
                    stderr,
                    b"Fatal: unknown error from inflateEnd().\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_reInitZipStream(
    mut zipStream: *mut z_stream,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    (*zipStream).next_in = ::core::ptr::null_mut::<Bytef>();
    (*zipStream).avail_in = 0 as uInt;
    retVal = inflateReset(zipStream);
    if retVal != 0 as ::core::ffi::c_int {
        match retVal {
            -2 => {
                fprintf(
                    stderr,
                    b"Fatal: internal error using zlib.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
            _ => {
                fprintf(
                    stderr,
                    b"Fatal: unknown error from inflateInit().\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_alloc(
    mut opaque: voidpf,
    mut items: uInt,
    mut size: uInt,
) -> voidpf {
    return calloc(items as size_t, size as size_t);
}
unsafe extern "C" fn zip_free(mut opaque: voidpf, mut address: voidpf) {
    free(address);
}
#[inline]
unsafe extern "C" fn zip_GPFileData_new() -> *mut zip_GPFileData {
    return zip_GPFileData_alloc();
}
#[inline]
unsafe extern "C" fn zip_GPFileData_delete(mut gPFileData: *mut zip_GPFileData) {
    zip_GPFileData_free(gPFileData);
}
#[inline]
unsafe extern "C" fn zip_GPFileData_alloc() -> *mut zip_GPFileData {
    let mut result: *mut zip_GPFileData = malloc(
        ::core::mem::size_of::<zip_GPFileData>(),
    ) as *mut zip_GPFileData;
    return result;
}
#[inline]
unsafe extern "C" fn zip_GPFileData_free(mut gPFileData: *mut zip_GPFileData) {
    free(gPFileData as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn zip_GPDirData_delete(mut gPDirData: *mut zip_GPDirData) {
    zip_GPDirData_free(gPDirData);
}
#[inline]
unsafe extern "C" fn zip_GPDirData_free(mut gPDirData: *mut zip_GPDirData) {
    free(gPDirData as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn makeUInt32(
    mut byte1: uio_uint8,
    mut byte2: uio_uint8,
    mut byte3: uio_uint8,
    mut byte4: uio_uint8,
) -> uio_uint32 {
    return (byte1 as ::core::ffi::c_int
        | (byte2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (byte3 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
        | (byte4 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uio_uint32;
}
#[inline]
unsafe extern "C" fn makeUInt16(
    mut byte1: uio_uint8,
    mut byte2: uio_uint8,
) -> uio_uint16 {
    return (byte1 as ::core::ffi::c_int
        | (byte2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uio_uint16;
}
#[inline]
unsafe extern "C" fn makeSInt32(
    mut byte1: uio_uint8,
    mut byte2: uio_uint8,
    mut byte3: uio_uint8,
    mut byte4: uio_uint8,
) -> uio_sint32 {
    return byte1 as uio_sint32 | (byte2 as uio_sint32) << 8 as ::core::ffi::c_int
        | (byte3 as uio_sint32) << 16 as ::core::ffi::c_int
        | (byte4 as uio_sint32) << 24 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn isBitSet(
    mut bitField: uio_uint32,
    mut bit: ::core::ffi::c_int,
) -> uio_bool {
    return bitField >> bit & 1 as uio_uint32 == 1 as uio_uint32;
}
#[inline]
unsafe extern "C" fn uio_Handle_ref(mut handle: *mut uio_Handle) {
    (*handle).r#ref += 1;
}
