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

// Rust translation of src/libs/strings/strings.c.
extern "C" {
    fn utf8StringCountN(
        start: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> size_t;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn StringHashTable_find(
        hashTable: *mut StringHashTable_HashTable,
        key: *const StringHashTable_Key,
    ) -> *mut StringHashTable_Value;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
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
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type RESOURCE = *const ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn AllocStringTable(
    mut num_entries: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> STRING_TABLE {
    let mut strtab: STRING_TABLE = HMalloc(::core::mem::size_of::<STRING_TABLE_DESC>())
        as STRING_TABLE;
    let mut i: ::core::ffi::c_int = 0;
    let mut multiplier: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        multiplier += 1;
    }
    if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        multiplier += 1;
    }
    if flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        multiplier += 1;
    }
    (*strtab).flags = flags as ::core::ffi::c_ushort;
    (*strtab).size = num_entries;
    num_entries *= multiplier;
    (*strtab).strings = HMalloc(
        ::core::mem::size_of::<STRING_TABLE_ENTRY_DESC>()
            .wrapping_mul(num_entries as size_t),
    ) as *mut STRING_TABLE_ENTRY_DESC;
    i = 0 as ::core::ffi::c_int;
    while i < num_entries {
        (*(*strtab).strings.offset(i as isize)).data = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        (*(*strtab).strings.offset(i as isize)).length = 0 as ::core::ffi::c_int;
        (*(*strtab).strings.offset(i as isize)).parent = strtab as *mut string_table;
        (*(*strtab).strings.offset(i as isize)).index = i;
        i += 1;
    }
    (*strtab).nameIndex = ::core::ptr::null_mut::<StringHashTable_HashTable>();
    return strtab;
}
#[no_mangle]
pub unsafe extern "C" fn FreeStringTable(mut strtab: STRING_TABLE) {
    let mut i: ::core::ffi::c_int = 0;
    let mut multiplier: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if strtab.is_null() {
        return;
    }
    if (*strtab).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        multiplier += 1;
    }
    if (*strtab).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        multiplier += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*strtab).size * multiplier {
        if !(*(*strtab).strings.offset(i as isize)).data.is_null() {
            HFree(
                (*(*strtab).strings.offset(i as isize)).data as *mut ::core::ffi::c_void,
            );
        }
        i += 1;
    }
    HFree((*strtab).strings as *mut ::core::ffi::c_void);
    HFree(strtab as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyStringTable(mut StringTable: STRING_TABLE) -> BOOLEAN {
    FreeStringTable(StringTable);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn CaptureStringTable(mut StringTable: STRING_TABLE) -> STRING {
    if !StringTable.is_null() && (*StringTable).size > 0 as ::core::ffi::c_int {
        return (*StringTable).strings;
    }
    return ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseStringTable(mut String: STRING) -> STRING_TABLE {
    let mut StringTable: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    StringTable = GetStringTable(String);
    return StringTable;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringTable(mut String: STRING) -> STRING_TABLE {
    if !String.is_null() && !(*String).parent.is_null() {
        return (*String).parent;
    }
    return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn GetStringTableCount(mut String: STRING) -> COUNT {
    if !String.is_null() && !(*String).parent.is_null() {
        return (*(*String).parent).size as COUNT;
    }
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringTableIndex(mut String: STRING) -> COUNT {
    if !String.is_null() {
        return (*String).index as COUNT;
    }
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn SetAbsStringTableIndex(
    mut String: STRING,
    mut StringTableIndex: COUNT,
) -> STRING {
    let mut StringTablePtr: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    if String.is_null() {
        return ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    }
    StringTablePtr = (*String).parent as STRING_TABLE;
    if StringTablePtr.is_null() {
        String = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    } else {
        StringTableIndex = (StringTableIndex as ::core::ffi::c_int
            % (*StringTablePtr).size) as COUNT;
        String = (*StringTablePtr).strings.offset(StringTableIndex as isize) as STRING;
    }
    return String;
}
#[no_mangle]
pub unsafe extern "C" fn SetRelStringTableIndex(
    mut String: STRING,
    mut StringTableOffs: SIZE,
) -> STRING {
    let mut StringTablePtr: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    if String.is_null() {
        return ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    }
    StringTablePtr = (*String).parent as STRING_TABLE;
    if StringTablePtr.is_null() {
        String = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    } else {
        let mut StringTableIndex: COUNT = 0;
        while (StringTableOffs as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            StringTableOffs = (StringTableOffs as ::core::ffi::c_int
                + (*StringTablePtr).size) as SIZE;
        }
        StringTableIndex = (((*String).index + StringTableOffs as ::core::ffi::c_int)
            % (*StringTablePtr).size) as COUNT;
        String = (*StringTablePtr).strings.offset(StringTableIndex as isize) as STRING;
    }
    return String;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringLength(mut String: STRING) -> COUNT {
    if String.is_null() {
        return 0 as COUNT;
    }
    return utf8StringCountN(
        (*String).data as *const ::core::ffi::c_char,
        (*String).data.offset((*String).length as isize) as *const ::core::ffi::c_char,
    ) as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringLengthBin(mut String: STRING) -> COUNT {
    if String.is_null() {
        return 0 as COUNT;
    }
    return (*String).length as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringName(mut String: STRING) -> STRINGPTR {
    let mut StringTablePtr: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    let mut StringIndex: COUNT = 0;
    if String.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringTablePtr = (*String).parent as STRING_TABLE;
    if StringTablePtr.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringIndex = (*String).index as COUNT;
    if (*StringTablePtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringIndex = (StringIndex as ::core::ffi::c_int + (*StringTablePtr).size) as COUNT;
    String = (*StringTablePtr).strings.offset(StringIndex as isize) as STRING;
    if (*String).length == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*String).data;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringSoundClip(mut String: STRING) -> STRINGPTR {
    let mut StringTablePtr: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    let mut StringIndex: COUNT = 0;
    if String.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringTablePtr = (*String).parent as STRING_TABLE;
    if StringTablePtr.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringIndex = (*String).index as COUNT;
    if (*StringTablePtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringIndex = (StringIndex as ::core::ffi::c_int + (*StringTablePtr).size) as COUNT;
    if (*StringTablePtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        StringIndex = (StringIndex as ::core::ffi::c_int + (*StringTablePtr).size)
            as COUNT;
    }
    String = (*StringTablePtr).strings.offset(StringIndex as isize) as STRING;
    if (*String).length == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*String).data;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringTimeStamp(mut String: STRING) -> STRINGPTR {
    let mut StringTablePtr: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    let mut StringIndex: COUNT = 0;
    if String.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringTablePtr = (*String).parent as STRING_TABLE;
    if StringTablePtr.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringIndex = (*String).index as COUNT;
    if (*StringTablePtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    StringIndex = (StringIndex as ::core::ffi::c_int + (*StringTablePtr).size) as COUNT;
    if (*StringTablePtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        StringIndex = (StringIndex as ::core::ffi::c_int + (*StringTablePtr).size)
            as COUNT;
    }
    if (*StringTablePtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        StringIndex = (StringIndex as ::core::ffi::c_int + (*StringTablePtr).size)
            as COUNT;
    }
    String = (*StringTablePtr).strings.offset(StringIndex as isize) as STRING;
    if (*String).length == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*String).data;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringAddress(mut String: STRING) -> STRINGPTR {
    if String.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*String).data;
}
#[no_mangle]
pub unsafe extern "C" fn GetStringByName(
    mut StringTable: STRING_TABLE,
    mut index: *const ::core::ffi::c_char,
) -> STRING {
    return StringHashTable_find((*StringTable).nameIndex, index);
}
#[no_mangle]
pub unsafe extern "C" fn CheckResString(mut res: RESOURCE) -> BOOLEAN {
    let mut check: BOOLEAN = BOOLEAN::FALSE;
    let mut string: STRING = CaptureStringTable(uqm_lua_checked!(LoadStringTableInstance(res), BOOLEAN(0)));
    if !string.is_null() {
        check = BOOLEAN::TRUE;
    }
    DestroyStringTable(ReleaseStringTable(string));
    string = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    return check;
}
