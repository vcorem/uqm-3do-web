/*
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

// Rust translation of src/libs/strings/stringhashtable.c.
extern "C" {
    pub type string_table_entry;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uio_bool = bool;
pub type uio_uint32 = ::core::ffi::c_uint;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type StringHashTable_Key = ::core::ffi::c_char;
pub type StringHashTable_Value = STRING_TABLE_ENTRY_DESC;
pub type StringHashTable_HashFunction = Option<
    unsafe extern "C" fn(*const StringHashTable_Key) -> uio_uint32,
>;
pub type StringHashTable_EqualFunction = Option<
    unsafe extern "C" fn(
        *const StringHashTable_Key,
        *const StringHashTable_Key,
    ) -> uio_bool,
>;
pub type StringHashTable_CopyFunction = Option<
    unsafe extern "C" fn(*const StringHashTable_Key) -> *mut StringHashTable_Value,
>;
pub type StringHashTable_FreeKeyFunction = Option<
    unsafe extern "C" fn(*mut StringHashTable_Key) -> (),
>;
pub type StringHashTable_FreeValueFunction = Option<
    unsafe extern "C" fn(*mut StringHashTable_Value) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringHashTable_Iterator {
    pub hashTable: *const StringHashTable_HashTable,
    pub bucketNr: uio_uint32,
    pub entry: *mut StringHashTable_HashEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
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
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_newHashTable(
    mut hashFunction: StringHashTable_HashFunction,
    mut equalFunction: StringHashTable_EqualFunction,
    mut copyFunction: StringHashTable_CopyFunction,
    mut freeKeyFunction: StringHashTable_FreeKeyFunction,
    mut freeValueFunction: StringHashTable_FreeValueFunction,
    mut initialSize: uio_uint32,
    mut minFillQuotient: ::core::ffi::c_double,
    mut maxFillQuotient: ::core::ffi::c_double,
) -> *mut StringHashTable_HashTable {
    let mut hashTable: *mut StringHashTable_HashTable = ::core::ptr::null_mut::<
        StringHashTable_HashTable,
    >();
    (maxFillQuotient >= minFillQuotient
        || {
            __assert_fail(
                b"maxFillQuotient >= minFillQuotient\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/uio/hashtable.c\0".as_ptr() as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_int,
                b"StringHashTable_newHashTable\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hashTable = StringHashTable_allocHashTable();
    (*hashTable).hashFunction = hashFunction;
    (*hashTable).equalFunction = equalFunction;
    (*hashTable).copyFunction = copyFunction;
    (*hashTable).freeKeyFunction = freeKeyFunction;
    (*hashTable).freeValueFunction = freeValueFunction;
    (*hashTable).minFillQuotient = minFillQuotient;
    (*hashTable).maxFillQuotient = maxFillQuotient;
    StringHashTable_setup(hashTable, initialSize);
    return hashTable;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_add(
    mut hashTable: *mut StringHashTable_HashTable,
    mut key: *const StringHashTable_Key,
    mut value: *mut StringHashTable_Value,
) -> uio_bool {
    let mut hash: uio_uint32 = 0;
    let mut entry: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    hash = StringHashTable_hash(hashTable, key);
    entry = *(*hashTable).entries.offset((hash & (*hashTable).hashMask) as isize)
        as *mut StringHashTable_HashEntry;
    while !entry.is_null() {
        if StringHashTable_equal(hashTable, key, (*entry).key) {
            return false;
        }
        entry = (*entry).next as *mut StringHashTable_HashEntry;
    }
    if !(*(*hashTable).entries.offset((hash & (*hashTable).hashMask) as isize)).is_null()
    {
        (*hashTable).numCollisions = (*hashTable).numCollisions.wrapping_add(1);
    }
    *(*hashTable).entries.offset((hash & (*hashTable).hashMask) as isize) = StringHashTable_newHashEntry(
        hash,
        StringHashTable_copy(hashTable, key),
        value,
        *(*hashTable).entries.offset((hash & (*hashTable).hashMask) as isize),
    );
    (*hashTable).numEntries = (*hashTable).numEntries.wrapping_add(1);
    if (*hashTable).numEntries > (*hashTable).maxSize {
        StringHashTable_resize(hashTable);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_remove(
    mut hashTable: *mut StringHashTable_HashTable,
    mut key: *const StringHashTable_Key,
) -> uio_bool {
    let mut hash: uio_uint32 = 0;
    let mut entry: *mut *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        *mut StringHashTable_HashEntry,
    >();
    let mut next: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    hash = StringHashTable_hash(hashTable, key);
    entry = (*hashTable).entries.offset((hash & (*hashTable).hashMask) as isize)
        as *mut *mut StringHashTable_HashEntry;
    loop {
        if (*entry).is_null() {
            return false;
        }
        if StringHashTable_equal(hashTable, key, (**entry).key) {
            break;
        }
        entry = &raw mut (**entry).next as *mut *mut StringHashTable_HashEntry;
    }
    next = (**entry).next as *mut StringHashTable_HashEntry;
    StringHashTable_freeHashEntry(*entry);
    *entry = next;
    (*hashTable).numEntries = (*hashTable).numEntries.wrapping_sub(1);
    if (*hashTable).numEntries < (*hashTable).minSize {
        StringHashTable_resize(hashTable);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_find(
    mut hashTable: *mut StringHashTable_HashTable,
    mut key: *const StringHashTable_Key,
) -> *mut StringHashTable_Value {
    let mut hash: uio_uint32 = 0;
    let mut entry: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    hash = StringHashTable_hash(hashTable, key);
    entry = *(*hashTable).entries.offset((hash & (*hashTable).hashMask) as isize)
        as *mut StringHashTable_HashEntry;
    while !entry.is_null() {
        if StringHashTable_equal(hashTable, key, (*entry).key) {
            return (*entry).value;
        }
        entry = (*entry).next as *mut StringHashTable_HashEntry;
    }
    return ::core::ptr::null_mut::<StringHashTable_Value>();
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_count(
    mut hashTable: *const StringHashTable_HashTable,
) -> uio_uint32 {
    return (*hashTable).numEntries;
}
unsafe extern "C" fn StringHashTable_setup(
    mut hashTable: *mut StringHashTable_HashTable,
    mut initialSize: uio_uint32,
) {
    if initialSize < 4 as uio_uint32 {
        initialSize = 4 as uio_uint32;
    }
    (*hashTable).size = nextPower2(
        ceil(initialSize as ::core::ffi::c_double / (*hashTable).maxFillQuotient)
            as uio_uint32,
    );
    (*hashTable).hashMask = (*hashTable).size.wrapping_sub(1 as uio_uint32);
    (*hashTable).minSize = ceil(
        ((*hashTable).size >> 1 as ::core::ffi::c_int) as ::core::ffi::c_double
            * (*hashTable).minFillQuotient,
    ) as uio_uint32;
    (*hashTable).maxSize = floor(
        (*hashTable).size as ::core::ffi::c_double * (*hashTable).maxFillQuotient,
    ) as uio_uint32;
    (*hashTable).entries = calloc(
        (*hashTable).size as size_t,
        ::core::mem::size_of::<*mut StringHashTable_HashEntry>(),
    ) as *mut *mut StringHashTable_HashEntry;
    (*hashTable).numEntries = 0 as uio_uint32;
    (*hashTable).numCollisions = 0 as uio_uint32;
}
unsafe extern "C" fn StringHashTable_resize(
    mut hashTable: *mut StringHashTable_HashTable,
) {
    let mut oldEntries: *mut *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        *mut StringHashTable_HashEntry,
    >();
    let mut entry: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    let mut next: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    let mut newLocation: *mut *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        *mut StringHashTable_HashEntry,
    >();
    let mut oldNumEntries: uio_uint32 = 0;
    let mut i: uio_uint32 = 0;
    oldNumEntries = (*hashTable).numEntries;
    oldEntries = (*hashTable).entries;
    StringHashTable_setup(hashTable, (*hashTable).numEntries);
    (*hashTable).numEntries = oldNumEntries;
    i = 0 as uio_uint32;
    while oldNumEntries > 0 as uio_uint32 {
        entry = *oldEntries.offset(i as isize);
        while !entry.is_null() {
            next = (*entry).next;
            newLocation = (*hashTable)
                .entries
                .offset(((*entry).hash & (*hashTable).hashMask) as isize);
            if !(*newLocation).is_null() {
                (*hashTable).numCollisions = (*hashTable).numCollisions.wrapping_add(1);
            }
            (*entry).next = *newLocation;
            *newLocation = entry;
            oldNumEntries = oldNumEntries.wrapping_sub(1);
            entry = next;
        }
        i = i.wrapping_add(1);
    }
    free(oldEntries as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn nextPower2(mut x: uio_uint32) -> uio_uint32 {
    x = x.wrapping_sub(1);
    x |= x >> 1 as ::core::ffi::c_int;
    x |= x >> 2 as ::core::ffi::c_int;
    x |= x >> 4 as ::core::ffi::c_int;
    x |= x >> 8 as ::core::ffi::c_int;
    x |= x >> 16 as ::core::ffi::c_int;
    return x.wrapping_add(1 as uio_uint32);
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_getIterator(
    mut hashTable: *const StringHashTable_HashTable,
) -> *mut StringHashTable_Iterator {
    let mut iterator: *mut StringHashTable_Iterator = ::core::ptr::null_mut::<
        StringHashTable_Iterator,
    >();
    let mut i: uio_uint32 = 0;
    iterator = malloc(::core::mem::size_of::<StringHashTable_Iterator>())
        as *mut StringHashTable_Iterator;
    (*iterator).hashTable = hashTable;
    i = 0 as uio_uint32;
    while i < (*(*iterator).hashTable).size {
        if !(*(*(*iterator).hashTable).entries.offset(i as isize)).is_null() {
            (*iterator).bucketNr = i;
            (*iterator).entry = *(*(*iterator).hashTable).entries.offset(i as isize);
            return iterator;
        }
        i = i.wrapping_add(1);
    }
    (*iterator).bucketNr = i;
    (*iterator).entry = ::core::ptr::null_mut::<StringHashTable_HashEntry>();
    return iterator;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_iteratorDone(
    mut iterator: *const StringHashTable_Iterator,
) -> ::core::ffi::c_int {
    return ((*iterator).bucketNr >= (*(*iterator).hashTable).size) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_iteratorKey(
    mut iterator: *mut StringHashTable_Iterator,
) -> *mut StringHashTable_Key {
    return (*(*iterator).entry).key;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_iteratorValue(
    mut iterator: *mut StringHashTable_Iterator,
) -> *mut StringHashTable_Value {
    return (*(*iterator).entry).value;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_iteratorNext(
    mut iterator: *mut StringHashTable_Iterator,
) -> *mut StringHashTable_Iterator {
    let mut i: uio_uint32 = 0;
    (*iterator).entry = (*(*iterator).entry).next;
    if !(*iterator).entry.is_null() {
        return iterator;
    }
    i = (*iterator).bucketNr.wrapping_add(1 as uio_uint32);
    while i < (*(*iterator).hashTable).size {
        if !(*(*(*iterator).hashTable).entries.offset(i as isize)).is_null() {
            (*iterator).bucketNr = i;
            (*iterator).entry = *(*(*iterator).hashTable).entries.offset(i as isize);
            return iterator;
        }
        i = i.wrapping_add(1);
    }
    (*iterator).bucketNr = i;
    (*iterator).entry = ::core::ptr::null_mut::<StringHashTable_HashEntry>();
    return iterator;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_freeIterator(
    mut iterator: *mut StringHashTable_Iterator,
) {
    free(iterator as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn StringHashTable_allocHashTable() -> *mut StringHashTable_HashTable {
    return malloc(::core::mem::size_of::<StringHashTable_HashTable>())
        as *mut StringHashTable_HashTable;
}
#[inline]
unsafe extern "C" fn StringHashTable_newHashEntry(
    mut hash: uio_uint32,
    mut key: *mut StringHashTable_Key,
    mut value: *mut StringHashTable_Value,
    mut next: *mut StringHashTable_HashEntry,
) -> *mut StringHashTable_HashEntry {
    let mut result: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    result = StringHashTable_allocHashEntry();
    (*result).hash = hash;
    (*result).key = key;
    (*result).value = value;
    (*result).next = next;
    return result;
}
#[inline]
unsafe extern "C" fn StringHashTable_allocHashEntry() -> *mut StringHashTable_HashEntry {
    return malloc(::core::mem::size_of::<StringHashTable_HashEntry>())
        as *mut StringHashTable_HashEntry;
}
#[no_mangle]
pub unsafe extern "C" fn StringHashTable_deleteHashTable(
    mut hashTable: *mut StringHashTable_HashTable,
) {
    let mut i: uio_uint32 = 0;
    let mut entry: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    let mut next: *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        StringHashTable_HashEntry,
    >();
    let mut bucketPtr: *mut *mut StringHashTable_HashEntry = ::core::ptr::null_mut::<
        *mut StringHashTable_HashEntry,
    >();
    i = (*hashTable).numEntries;
    bucketPtr = (*hashTable).entries;
    while i > 0 as uio_uint32 {
        entry = *bucketPtr;
        while !entry.is_null() {
            next = (*entry).next;
            StringHashTable_freeHashEntry(entry);
            entry = next;
            i = i.wrapping_sub(1);
        }
        bucketPtr = bucketPtr.offset(1);
    }
    free((*hashTable).entries as *mut ::core::ffi::c_void);
    free(hashTable as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn StringHashTable_freeHashEntry(
    mut entry: *mut StringHashTable_HashEntry,
) {
    free(entry as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn StringHashTable_hash(
    mut hashTable: *mut StringHashTable_HashTable,
    mut key: *const ::core::ffi::c_char,
) -> uio_uint32 {
    let mut hash: uio_uint32 = 0;
    hash = 0 as uio_uint32;
    while *key as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        hash = hash << 4 as ::core::ffi::c_int ^ hash >> 28 as ::core::ffi::c_int
            ^ *key as uio_uint32;
        key = key.offset(1);
    }
    return hash ^ hash >> 10 as ::core::ffi::c_int ^ hash >> 20 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn StringHashTable_equal(
    mut hashTable: *mut StringHashTable_HashTable,
    mut key1: *const ::core::ffi::c_char,
    mut key2: *const ::core::ffi::c_char,
) -> uio_bool {
    return strcmp(key1, key2) == 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn StringHashTable_copy(
    mut hashTable: *mut StringHashTable_HashTable,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return unconst(key as *const ::core::ffi::c_void) as *mut ::core::ffi::c_char;
}
