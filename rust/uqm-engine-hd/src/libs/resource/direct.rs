//Copyright Paul Reiche, Fred Ford. 1992-2002

/*
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

// Rust translation of src/libs/resource/direct.c.
extern "C" {
    pub type uio_DirHandle;
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
    fn uio_stat(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        statBuf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn uio_openDirRelative(
        base: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_DirHandle;
    fn uio_closeDir(dirHandle: *mut uio_DirHandle) -> ::core::ffi::c_int;
    fn uio_getDirList(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        matchType: match_MatchType,
    ) -> *mut uio_DirList;
    fn uio_DirList_free(dirList: *mut uio_DirList);
    fn AllocStringTable(
        num_entries: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> STRING_TABLE;
    fn FreeStringTable(strtab: STRING_TABLE);
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type off_t = ::core::ffi::c_longlong;
pub type uint16 = ::core::ffi::c_ushort;
pub type UWORD = uint16;
pub type COUNT = UWORD;
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
pub struct uio_DirList {
    pub names: *mut *const ::core::ffi::c_char,
    pub numNames: ::core::ffi::c_int,
}
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
pub type uio_bool = bool;
pub type uio_uint32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_table_entry {
    pub data: STRINGPTR,
    pub length: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub parent: *mut string_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_table {
    pub flags: ::core::ffi::c_ushort,
    pub size: ::core::ffi::c_int,
    pub strings: *mut STRING_TABLE_ENTRY_DESC,
    pub nameIndex: *mut StringHashTable_HashTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringHashTable_HashTable {
    pub hashFunction: StringHashTable_HashFunction,
    pub equalFunction: StringHashTable_EqualFunction,
    pub copyFunction: StringHashTable_CopyFunction,
    pub freeKeyFunction: StringHashTable_FreeKeyFunction,
    pub freeValueFunction: StringHashTable_FreeValueFunction,
    pub minFillQuotient: ::core::ffi::c_double,
    pub maxFillQuotient: ::core::ffi::c_double,
    pub minSize: uio_uint32,
    pub maxSize: uio_uint32,
    pub size: uio_uint32,
    pub hashMask: uio_uint32,
    pub entries: *mut *mut StringHashTable_HashEntry,
    pub numEntries: uio_uint32,
    pub numCollisions: uio_uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringHashTable_HashEntry {
    pub hash: uio_uint32,
    pub key: *mut StringHashTable_Key,
    pub value: *mut StringHashTable_Value,
    pub next: *mut StringHashTable_HashEntry,
}
pub type StringHashTable_Value = STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type StringHashTable_Key = ::core::ffi::c_char;
pub type StringHashTable_FreeValueFunction = Option<
    unsafe extern "C" fn(*mut StringHashTable_Value) -> (),
>;
pub type StringHashTable_FreeKeyFunction = Option<
    unsafe extern "C" fn(*mut StringHashTable_Key) -> (),
>;
pub type StringHashTable_CopyFunction = Option<
    unsafe extern "C" fn(*const StringHashTable_Key) -> *mut StringHashTable_Value,
>;
pub type StringHashTable_EqualFunction = Option<
    unsafe extern "C" fn(
        *const StringHashTable_Key,
        *const StringHashTable_Key,
    ) -> uio_bool,
>;
pub type StringHashTable_HashFunction = Option<
    unsafe extern "C" fn(*const StringHashTable_Key) -> uio_uint32,
>;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type DIRENTRY_REF = STRING_TABLE;
#[no_mangle]
pub unsafe extern "C" fn LoadDirEntryTable(
    mut dirHandle: *mut uio_DirHandle,
    mut path: *const ::core::ffi::c_char,
    mut pattern: *const ::core::ffi::c_char,
    mut matchType: match_MatchType,
) -> DIRENTRY_REF {
    let mut dirList: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    let mut num_entries: COUNT = 0;
    let mut i: COUNT = 0;
    let mut dir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut StringTable: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    let mut lpST: *mut STRING_TABLE_DESC = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    let mut lpLastString: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    dir = uqm_lua_checked!(uio_openDirRelative(dirHandle, path, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
    (!dir.is_null()
        || {
            __assert_fail(
                b"dir != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/resource/direct.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_int,
                b"LoadDirEntryTable\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    dirList = uqm_lua_checked!(uio_getDirList(
        dir,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        pattern,
        matchType,
    ), ::core::ptr::null_mut());
    (!dirList.is_null()
        || {
            __assert_fail(
                b"dirList != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/resource/direct.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                40 as ::core::ffi::c_int,
                b"LoadDirEntryTable\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    num_entries = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*dirList).numNames {
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
        if *(*(*dirList).names.offset(i as isize)).offset(0isize) as ::core::ffi::c_int
            == '.' as ::core::ffi::c_int
        {
            *(*dirList).names.offset(i as isize) = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
        } else if uqm_lua_checked!(uio_stat(dir, *(*dirList).names.offset(i as isize), &raw mut sb), ::core::ptr::null_mut())
            == -1 as ::core::ffi::c_int
        {
            *(*dirList).names.offset(i as isize) = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
        } else if !(sb.st_mode & 0o170000 as mode_t == 0o100000 as mode_t) {
            *(*dirList).names.offset(i as isize) = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
        } else {
            num_entries = num_entries.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    uio_closeDir(dir);
    if num_entries as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uio_DirList_free(dirList);
        return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    }
    StringTable = AllocStringTable(
        num_entries as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    lpST = StringTable as *mut STRING_TABLE_DESC;
    if lpST.is_null() {
        FreeStringTable(StringTable);
        uio_DirList_free(dirList);
        return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    }
    (*lpST).size = num_entries as ::core::ffi::c_int;
    lpLastString = (*lpST).strings as STRING;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*dirList).numNames {
        let mut size: ::core::ffi::c_int = 0;
        let mut target: STRINGPTR = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*(*dirList).names.offset(i as isize)).is_null() {
            size = strlen(*(*dirList).names.offset(i as isize)).wrapping_add(1 as size_t)
                as ::core::ffi::c_int;
            target = HMalloc(size as size_t) as STRINGPTR;
            memcpy(
                target as *mut ::core::ffi::c_void,
                *(*dirList).names.offset(i as isize) as *const ::core::ffi::c_void,
                size as size_t,
            );
            (*lpLastString).data = target;
            (*lpLastString).length = size;
            lpLastString = lpLastString.offset(1);
        }
        i = i.wrapping_add(1);
    }
    uio_DirList_free(dirList);
    return StringTable;
}
