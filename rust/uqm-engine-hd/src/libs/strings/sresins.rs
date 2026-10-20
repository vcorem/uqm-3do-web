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

// Rust translation of src/libs/strings/sresins.c.
extern "C" {
    pub type uio_Stream;
    fn _GetStringData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _GetBinaryTableData(
        fp: *mut uio_Stream,
        length: DWORD,
    ) -> *mut ::core::ffi::c_void;
    fn _GetConversationData(
        path: *const ::core::ffi::c_char,
        resdata: *mut RESOURCE_DATA,
    );
    fn LoadResourceFromPath(
        pathname: *const ::core::ffi::c_char,
        r#fn: Option<ResourceLoadFileFun>,
    ) -> *mut ::core::ffi::c_void;
    fn InstallResTypeVectors(
        res_type: *const ::core::ffi::c_char,
        loadFun: Option<ResourceLoadFun>,
        freeFun: Option<ResourceFreeFun>,
        stringFun: Option<ResourceStringFun>,
    ) -> BOOLEAN;
    fn res_GetResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn res_DetachResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn FreeResourceData(_: *mut ::core::ffi::c_void) -> BOOLEAN;
}
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
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub union RESOURCE_DATA {
    pub num: DWORD,
    pub ptr: *mut ::core::ffi::c_void,
    pub str: *const ::core::ffi::c_char,
}
pub type ResourceLoadFun = unsafe extern "C" fn(
    *const ::core::ffi::c_char,
    *mut RESOURCE_DATA,
) -> ();
pub type ResourceFreeFun = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN;
pub type ResourceStringFun = unsafe extern "C" fn(
    *mut RESOURCE_DATA,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uint,
) -> ();
pub type ResourceLoadFileFun = unsafe extern "C" fn(
    *mut uio_Stream,
    DWORD,
) -> *mut ::core::ffi::c_void;
unsafe extern "C" fn GetStringTableFileData(
    mut pathname: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).ptr = uqm_lua_checked!(LoadResourceFromPath(
        pathname,
        Some(
            _GetStringData
                as unsafe extern "C" fn(
                    *mut uio_Stream,
                    DWORD,
                ) -> *mut ::core::ffi::c_void,
        ),
    ), ());
}
unsafe extern "C" fn GetBinaryTableFileData(
    mut pathname: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).ptr = uqm_lua_checked!(LoadResourceFromPath(
        pathname,
        Some(
            _GetBinaryTableData
                as unsafe extern "C" fn(
                    *mut uio_Stream,
                    DWORD,
                ) -> *mut ::core::ffi::c_void,
        ),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn InstallStringTableResType() -> BOOLEAN {
    InstallResTypeVectors(
        b"STRTAB\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetStringTableFileData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            FreeResourceData as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    InstallResTypeVectors(
        b"BINTAB\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetBinaryTableFileData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            FreeResourceData as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    InstallResTypeVectors(
        b"CONVERSATION\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            _GetConversationData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            FreeResourceData as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadStringTableInstance(mut res: RESOURCE) -> STRING_TABLE {
    let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    data = uqm_lua_checked!(res_GetResource(res), ::core::ptr::null_mut());
    if !data.is_null() {
        res_DetachResource(res);
    }
    return data as STRING_TABLE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadStringInstance(
    mut res: RESOURCE,
) -> *const ::core::ffi::c_char {
    let mut data: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    data = uqm_lua_checked!(res_GetResource(res), ::core::ptr::null()) as *const ::core::ffi::c_char;
    if !data.is_null() {
        res_DetachResource(res);
    }
    return data;
}
