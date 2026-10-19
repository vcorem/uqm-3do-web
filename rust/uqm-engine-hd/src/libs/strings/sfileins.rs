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

// Rust translation of src/libs/strings/sfileins.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn _GetStringData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
}
pub type size_t = usize;
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
#[no_mangle]
pub unsafe extern "C" fn LoadStringTableFile(
    mut dir: *mut uio_DirHandle,
    mut fileName: *const ::core::ffi::c_char,
) -> STRING_TABLE {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    if !_cur_resfile_name.is_null() {
        return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    }
    fp = uqm_lua_checked!(res_OpenResFile(dir, fileName, b"rb\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut());
    if !fp.is_null() {
        let mut data: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
        _cur_resfile_name = fileName;
        data = uqm_lua_checked!(_GetStringData(fp, uqm_lua_checked!(LengthResFile(fp), ::core::ptr::null_mut()) as DWORD), ::core::ptr::null_mut()) as STRING_TABLE;
        _cur_resfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        uqm_lua_checked!(res_CloseResFile(fp), ::core::ptr::null_mut());
        return data;
    }
    return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
}
