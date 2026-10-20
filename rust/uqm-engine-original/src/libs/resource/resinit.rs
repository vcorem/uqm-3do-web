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

// Rust translation of src/libs/resource/resinit.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn WriteResFile(
        lpBuf: *const ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn PutResFileChar(
        ch: ::core::ffi::c_char,
        fp: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn PutResFileNewline(fp: *mut uio_Stream) -> ::core::ffi::c_int;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn InstallStringTableResType() -> BOOLEAN;
    fn InstallAudioResTypes() -> BOOLEAN;
    fn InstallVideoResType() -> BOOLEAN;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn PropFile_from_filename(
        path: *mut uio_DirHandle,
        fname: *const ::core::ffi::c_char,
        handler: PROPERTY_HANDLER,
        prefix: *const ::core::ffi::c_char,
    );
    fn lookupResourceDesc(idx: RESOURCE_INDEX, res: RESOURCE) -> *mut ResourceDesc;
    fn InstallGraphicResTypes() -> BOOLEAN;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn InstallCodeResType() -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
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
pub struct resource_index_desc {
    pub map: *mut CharHashTable_HashTable,
    pub numRes: size_t,
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
pub type CharHashTable_HashFunction = Option<
    unsafe extern "C" fn(*const CharHashTable_Key) -> uio_uint32,
>;
pub type RESOURCE_INDEX_DESC = resource_index_desc;
pub type RESOURCE_INDEX = *mut RESOURCE_INDEX_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
pub type ResourceDesc = resource_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_desc {
    pub res_id: RESOURCE,
    pub fname: *mut ::core::ffi::c_char,
    pub vtable: *mut ResourceHandlers,
    pub resdata: RESOURCE_DATA,
    pub refcount: ::core::ffi::c_uint,
}
pub type ResourceHandlers = resource_handlers;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_handlers {
    pub resType: *const ::core::ffi::c_char,
    pub loadFun: Option<ResourceLoadFun>,
    pub freeFun: Option<ResourceFreeFun>,
    pub toString: Option<ResourceStringFun>,
}
pub type PROPERTY_HANDLER = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *const ::core::ffi::c_char) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharHashTable_Iterator {
    pub hashTable: *const CharHashTable_HashTable,
    pub bucketNr: uio_uint32,
    pub entry: *mut CharHashTable_HashEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
unsafe extern "C" fn allocResourceIndex() -> RESOURCE_INDEX {
    let mut ndx: RESOURCE_INDEX = HMalloc(
        ::core::mem::size_of::<RESOURCE_INDEX_DESC>() as size_t,
    ) as RESOURCE_INDEX;
    (*ndx).map = CharHashTable_newHashTable(
        None,
        None,
        None,
        None,
        None,
        0 as uio_uint32,
        0.85f64,
        0.9f64,
    );
    return ndx;
}
unsafe extern "C" fn freeResourceIndex(mut h: RESOURCE_INDEX) {
    if !h.is_null() {
        CharHashTable_deleteHashTable((*h).map);
        HFree(h as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn newResourceDesc(
    mut res_id: *const ::core::ffi::c_char,
    mut resval: *const ::core::ffi::c_char,
) -> *mut ResourceDesc {
    let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut pathlen: ::core::ffi::c_int = 0;
    let mut vtable: *mut ResourceHandlers = ::core::ptr::null_mut::<ResourceHandlers>();
    let mut result: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    let mut handlerdesc: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut typestr: [::core::ffi::c_char; 32] = [0; 32];
    path = strchr(resval, ':' as ::core::ffi::c_int);
    if path.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Could not find type information for resource '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            res_id,
        );
        strncpy(
            &raw mut typestr as *mut ::core::ffi::c_char,
            b"sys.UNKNOWNRES\0".as_ptr() as *const ::core::ffi::c_char,
            32 as size_t,
        );
        path = resval;
    } else {
        let mut n: ::core::ffi::c_int = path.offset_from(resval) as ::core::ffi::c_int;
        if n >= 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int {
            n = 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        }
        strncpy(
            &raw mut typestr as *mut ::core::ffi::c_char,
            b"sys.\0".as_ptr() as *const ::core::ffi::c_char,
            32 as size_t,
        );
        strncat(
            (&raw mut typestr as *mut ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize),
            resval,
            n as size_t,
        );
        typestr[(n + 4 as ::core::ffi::c_int) as usize] = '\0' as ::core::ffi::c_char;
        path = path.offset(1);
    }
    pathlen = strlen(path) as ::core::ffi::c_int;
    handlerdesc = lookupResourceDesc(
        idx,
        &raw mut typestr as *mut ::core::ffi::c_char as RESOURCE,
    );
    if handlerdesc.is_null() {
        path = resval;
        log_add(
            log_Level::log_Warning,
            b"Illegal type '%s' for resource '%s'; treating as UNKNOWNRES\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut typestr as *mut ::core::ffi::c_char,
            res_id,
        );
        handlerdesc = lookupResourceDesc(idx, b"sys.UNKNOWNRES\0".as_ptr() as RESOURCE);
    }
    vtable = (*handlerdesc).resdata.ptr as *mut ResourceHandlers;
    if (*vtable).loadFun.is_none() {
        log_add(
            log_Level::log_Warning,
            b"Warning: Unable to load '%s'; no handler for type %s defined.\0".as_ptr()
                as *const ::core::ffi::c_char,
            res_id,
            &raw mut typestr as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<ResourceDesc>();
    }
    result = HMalloc(::core::mem::size_of::<ResourceDesc>() as size_t)
        as *mut ResourceDesc;
    if result.is_null() {
        return ::core::ptr::null_mut::<ResourceDesc>();
    }
    (*result).fname = HMalloc((pathlen + 1 as ::core::ffi::c_int) as size_t)
        as *mut ::core::ffi::c_char;
    strncpy((*result).fname, path, pathlen as size_t);
    *(*result).fname.offset(pathlen as isize) = '\0' as ::core::ffi::c_char;
    (*result).vtable = vtable;
    (*result).refcount = 0 as ::core::ffi::c_uint;
    if (*vtable).freeFun.is_none() {
        (*vtable)
            .loadFun
            .expect(
                "non-null function pointer",
            )((*result).fname, &raw mut (*result).resdata);
    } else {
        (*result).resdata.ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return result;
}
unsafe extern "C" fn process_resource_desc(
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    let mut map: *mut CharHashTable_HashTable = (*_get_current_index_header()).map;
    let mut newDesc: *mut ResourceDesc = newResourceDesc(key, value);
    if !newDesc.is_null() {
        if !CharHashTable_add(map, key, newDesc as *mut ::core::ffi::c_void) {
            res_Remove(key);
            CharHashTable_add(map, key, newDesc as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn UseDescriptorAsRes(
    mut descriptor: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).str = descriptor;
}
unsafe extern "C" fn DescriptorToInt(
    mut descriptor: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).num = atoi(descriptor) as DWORD;
}
unsafe extern "C" fn DescriptorToBoolean(
    mut descriptor: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    if strcasecmp(descriptor, b"true\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        (*resdata).num = BOOLEAN::TRUE.0 as ::core::ffi::c_int as DWORD;
    } else {
        (*resdata).num = BOOLEAN::FALSE.0 as ::core::ffi::c_int as DWORD;
    };
}
#[inline]
unsafe extern "C" fn skipWhiteSpace(mut start: *const ::core::ffi::c_char) -> size_t {
    let mut ptr: *const ::core::ffi::c_char = start;
    while isspace(*ptr as ::core::ffi::c_int) != 0 {
        ptr = ptr.offset(1);
    }
    return ptr.offset_from(start) as size_t;
}
unsafe extern "C" fn DescriptorToColor(
    mut descriptor: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    let mut bytesParsed: ::core::ffi::c_int = 0;
    let mut componentBits: ::core::ffi::c_int = 0;
    let mut maxComponentValue: ::core::ffi::c_int = 0;
    let mut componentCount: size_t = 0;
    let mut compI: size_t = 0;
    let mut comps: [::core::ffi::c_int; 4] = [0; 4];
    descriptor = descriptor.offset(skipWhiteSpace(descriptor) as isize);
    if sscanf(
        descriptor,
        b"rgb ( %i , %i , %i ) %n\0".as_ptr() as *const ::core::ffi::c_char,
        (&raw mut comps as *mut ::core::ffi::c_int).offset(0isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(1isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(2isize),
        &raw mut bytesParsed,
    ) >= 3 as ::core::ffi::c_int
    {
        componentBits = 8 as ::core::ffi::c_int;
        componentCount = 3 as size_t;
        comps[3usize] = 0xff as ::core::ffi::c_int;
    } else if sscanf(
        descriptor,
        b"rgba ( %i , %i , %i , %i ) %n\0".as_ptr() as *const ::core::ffi::c_char,
        (&raw mut comps as *mut ::core::ffi::c_int).offset(0isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(1isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(2isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(3isize),
        &raw mut bytesParsed,
    ) >= 4 as ::core::ffi::c_int
    {
        componentBits = 8 as ::core::ffi::c_int;
        componentCount = 4 as size_t;
    } else if sscanf(
        descriptor,
        b"rgb15 ( %i , %i , %i ) %n\0".as_ptr() as *const ::core::ffi::c_char,
        (&raw mut comps as *mut ::core::ffi::c_int).offset(0isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(1isize),
        (&raw mut comps as *mut ::core::ffi::c_int).offset(2isize),
        &raw mut bytesParsed,
    ) >= 3 as ::core::ffi::c_int
    {
        componentBits = 5 as ::core::ffi::c_int;
        componentCount = 3 as size_t;
        comps[3usize] = 0xff as ::core::ffi::c_int;
    } else {
        log_add(
            log_Level::log_Error,
            b"Invalid color description string for resource.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        (*resdata).num = 0 as DWORD;
        return;
    }
    if *descriptor.offset(bytesParsed as isize) as ::core::ffi::c_int
        != '\0' as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"Junk after color resource string.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    maxComponentValue = ((1 as ::core::ffi::c_int) << componentBits)
        - 1 as ::core::ffi::c_int;
    compI = 0 as size_t;
    while compI < componentCount {
        if comps[compI as usize] < 0 as ::core::ffi::c_int {
            comps[compI as usize] = 0 as ::core::ffi::c_int;
            log_add(
                log_Level::log_Warning,
                b"Color component value too small; value clipped.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if comps[compI as usize] as ::core::ffi::c_long
            > maxComponentValue as ::core::ffi::c_long
        {
            comps[compI as usize] = maxComponentValue;
            log_add(
                log_Level::log_Warning,
                b"Color component value too large; value clipped.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        compI = compI.wrapping_add(1);
    }
    if componentBits == 5 as ::core::ffi::c_int {
        (*resdata).num = ((comps[0usize] << 3 as ::core::ffi::c_int
            | comps[0usize] >> 2 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | (comps[1usize] << 3 as ::core::ffi::c_int
                | comps[1usize] >> 2 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (comps[2usize] << 3 as ::core::ffi::c_int
                | comps[2usize] >> 2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | comps[3usize]) as DWORD;
    } else {
        (*resdata).num = (comps[0usize] << 24 as ::core::ffi::c_int
            | comps[1usize] << 16 as ::core::ffi::c_int
            | comps[2usize] << 8 as ::core::ffi::c_int | comps[3usize]) as DWORD;
    };
}
unsafe extern "C" fn RawDescriptor(
    mut resdata: *mut RESOURCE_DATA,
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) {
    snprintf(
        buf,
        size as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*resdata).str,
    );
}
unsafe extern "C" fn IntToString(
    mut resdata: *mut RESOURCE_DATA,
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) {
    snprintf(
        buf,
        size as size_t,
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        (*resdata).num,
    );
}
unsafe extern "C" fn BooleanToString(
    mut resdata: *mut RESOURCE_DATA,
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) {
    snprintf(
        buf,
        size as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if (*resdata).num != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn ColorToString(
    mut resdata: *mut RESOURCE_DATA,
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) {
    if (*resdata).num & 0xff as DWORD == 0xff as DWORD {
        snprintf(
            buf,
            size as size_t,
            b"rgb(0x%02x, 0x%02x, 0x%02x)\0".as_ptr() as *const ::core::ffi::c_char,
            (*resdata).num >> 24 as ::core::ffi::c_int,
            (*resdata).num >> 16 as ::core::ffi::c_int & 0xff as DWORD,
            (*resdata).num >> 8 as ::core::ffi::c_int & 0xff as DWORD,
        );
    } else {
        snprintf(
            buf,
            size as size_t,
            b"rgba(0x%02x, 0x%02x, 0x%02x, 0x%02x)\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*resdata).num >> 24 as ::core::ffi::c_int,
            (*resdata).num >> 16 as ::core::ffi::c_int & 0xff as DWORD,
            (*resdata).num >> 8 as ::core::ffi::c_int & 0xff as DWORD,
            (*resdata).num & 0xff as DWORD,
        );
    };
}
static mut curResourceIndex: RESOURCE_INDEX = ::core::ptr::null_mut::<
    RESOURCE_INDEX_DESC,
>();
#[no_mangle]
pub unsafe extern "C" fn _set_current_index_header(
    mut newResourceIndex: RESOURCE_INDEX,
) {
    curResourceIndex = newResourceIndex;
}
#[no_mangle]
pub unsafe extern "C" fn InitResourceSystem() -> RESOURCE_INDEX {
    let mut ndx: RESOURCE_INDEX = ::core::ptr::null_mut::<RESOURCE_INDEX_DESC>();
    if !curResourceIndex.is_null() {
        return curResourceIndex;
    }
    ndx = allocResourceIndex();
    _set_current_index_header(ndx);
    InstallResTypeVectors(
        b"UNKNOWNRES\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            UseDescriptorAsRes
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        None,
        None,
    );
    InstallResTypeVectors(
        b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            UseDescriptorAsRes
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        None,
        Some(
            RawDescriptor
                as unsafe extern "C" fn(
                    *mut RESOURCE_DATA,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_uint,
                ) -> (),
        ),
    );
    InstallResTypeVectors(
        b"INT32\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            DescriptorToInt
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        None,
        Some(
            IntToString
                as unsafe extern "C" fn(
                    *mut RESOURCE_DATA,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_uint,
                ) -> (),
        ),
    );
    InstallResTypeVectors(
        b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            DescriptorToBoolean
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        None,
        Some(
            BooleanToString
                as unsafe extern "C" fn(
                    *mut RESOURCE_DATA,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_uint,
                ) -> (),
        ),
    );
    InstallResTypeVectors(
        b"COLOR\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            DescriptorToColor
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        None,
        Some(
            ColorToString
                as unsafe extern "C" fn(
                    *mut RESOURCE_DATA,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_uint,
                ) -> (),
        ),
    );
    InstallGraphicResTypes();
    InstallStringTableResType();
    InstallAudioResTypes();
    InstallVideoResType();
    InstallCodeResType();
    return ndx;
}
#[no_mangle]
pub unsafe extern "C" fn _get_current_index_header() -> RESOURCE_INDEX {
    if curResourceIndex.is_null() {
        InitResourceSystem();
    }
    return curResourceIndex;
}
#[no_mangle]
pub unsafe extern "C" fn LoadResourceIndex(
    mut dir: *mut uio_DirHandle,
    mut rmpfile: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) {
    PropFile_from_filename(
        dir,
        rmpfile,
        Some(
            process_resource_desc
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
        prefix,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SaveResourceIndex(
    mut dir: *mut uio_DirHandle,
    mut rmpfile: *const ::core::ffi::c_char,
    mut root: *const ::core::ffi::c_char,
    mut strip_root: BOOLEAN,
) {
    let mut f: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut it: *mut CharHashTable_Iterator = ::core::ptr::null_mut::<
        CharHashTable_Iterator,
    >();
    let mut prefix_len: ::core::ffi::c_uint = 0;
    f = res_OpenResFile(dir, rmpfile, b"wb\0".as_ptr() as *const ::core::ffi::c_char);
    if f.is_null() {
        return;
    }
    prefix_len = (if !root.is_null() { strlen(root) } else { 0 as size_t })
        as ::core::ffi::c_uint;
    it = CharHashTable_getIterator((*_get_current_index_header()).map);
    while CharHashTable_iteratorDone(it) == 0 {
        let mut key: *mut ::core::ffi::c_char = CharHashTable_iteratorKey(it);
        if root.is_null() || strncmp(root, key, prefix_len as size_t) == 0 {
            let mut value: *mut ResourceDesc = CharHashTable_iteratorValue(it)
                as *mut ResourceDesc;
            if value.is_null() {
                log_add(
                    log_Level::log_Warning,
                    b"Resource %s had no value\0".as_ptr() as *const ::core::ffi::c_char,
                    key,
                );
            } else if (*value).vtable.is_null() {
                log_add(
                    log_Level::log_Warning,
                    b"Resource %s had no type\0".as_ptr() as *const ::core::ffi::c_char,
                    key,
                );
            } else if (*(*value).vtable).toString.is_some() {
                let mut buf: [::core::ffi::c_char; 256] = [0; 256];
                (*(*value).vtable)
                    .toString
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut (*value).resdata,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    256 as ::core::ffi::c_uint,
                );
                buf[255usize] = 0 as ::core::ffi::c_char;
                if !root.is_null() && strip_root.0 != 0 {
                    WriteResFile(
                        key.offset(prefix_len as isize) as *const ::core::ffi::c_void,
                        1 as size_t,
                        strlen(key).wrapping_sub(prefix_len as size_t),
                        f,
                    );
                } else {
                    WriteResFile(
                        key as *const ::core::ffi::c_void,
                        1 as size_t,
                        strlen(key),
                        f,
                    );
                }
                PutResFileChar(' ' as ::core::ffi::c_char, f);
                PutResFileChar('=' as ::core::ffi::c_char, f);
                PutResFileChar(' ' as ::core::ffi::c_char, f);
                WriteResFile(
                    (*(*value).vtable).resType as *const ::core::ffi::c_void,
                    1 as size_t,
                    strlen((*(*value).vtable).resType),
                    f,
                );
                PutResFileChar(':' as ::core::ffi::c_char, f);
                WriteResFile(
                    &raw mut buf as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    1 as size_t,
                    strlen(&raw mut buf as *mut ::core::ffi::c_char),
                    f,
                );
                PutResFileNewline(f);
            }
        }
        it = CharHashTable_iteratorNext(it);
    }
    res_CloseResFile(f);
    CharHashTable_freeIterator(it);
}
#[no_mangle]
pub unsafe extern "C" fn UninitResourceSystem() {
    freeResourceIndex(_get_current_index_header());
    _set_current_index_header(::core::ptr::null_mut::<RESOURCE_INDEX_DESC>());
}
#[no_mangle]
pub unsafe extern "C" fn InstallResTypeVectors(
    mut resType: *const ::core::ffi::c_char,
    mut loadFun: Option<ResourceLoadFun>,
    mut freeFun: Option<ResourceFreeFun>,
    mut stringFun: Option<ResourceStringFun>,
) -> BOOLEAN {
    let mut handlers: *mut ResourceHandlers = ::core::ptr::null_mut::<
        ResourceHandlers,
    >();
    let mut result: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    let mut key: [::core::ffi::c_char; 32] = [0; 32];
    let mut typelen: ::core::ffi::c_int = 0;
    let mut map: *mut CharHashTable_HashTable = ::core::ptr::null_mut::<
        CharHashTable_HashTable,
    >();
    snprintf(
        &raw mut key as *mut ::core::ffi::c_char,
        32 as size_t,
        b"sys.%s\0".as_ptr() as *const ::core::ffi::c_char,
        resType,
    );
    key[(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = '\0'
        as ::core::ffi::c_char;
    typelen = strlen(resType) as ::core::ffi::c_int;
    handlers = HMalloc(::core::mem::size_of::<ResourceHandlers>() as size_t)
        as *mut ResourceHandlers;
    if handlers.is_null() {
        return BOOLEAN::FALSE;
    }
    (*handlers).loadFun = loadFun;
    (*handlers).freeFun = freeFun;
    (*handlers).toString = stringFun;
    (*handlers).resType = resType;
    result = HMalloc(::core::mem::size_of::<ResourceDesc>() as size_t)
        as *mut ResourceDesc;
    if result.is_null() {
        return BOOLEAN::FALSE;
    }
    (*result).fname = HMalloc(strlen(resType).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    strncpy((*result).fname, resType, typelen as size_t);
    *(*result).fname.offset(typelen as isize) = '\0' as ::core::ffi::c_char;
    (*result).vtable = ::core::ptr::null_mut::<ResourceHandlers>();
    (*result).resdata.ptr = handlers as *mut ::core::ffi::c_void;
    map = (*_get_current_index_header()).map;
    return BOOLEAN(
        (CharHashTable_add(
            map,
            &raw mut key as *mut ::core::ffi::c_char,
            result as *mut ::core::ffi::c_void,
        ) as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_IsString(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    return BOOLEAN(
        (!desc.is_null()
            && strcmp(
                (*(*desc).vtable).resType,
                b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_GetString(
    mut key: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    if desc.is_null() || (*desc).resdata.str.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return (*desc).resdata.str;
}
#[no_mangle]
pub unsafe extern "C" fn res_PutString(
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    let mut srclen: ::core::ffi::c_int = 0;
    let mut dstlen: ::core::ffi::c_int = 0;
    if desc.is_null() || (*desc).resdata.str.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        process_resource_desc(
            key,
            b"STRING:undefined\0".as_ptr() as *const ::core::ffi::c_char,
        );
        desc = lookupResourceDesc(idx, key);
    }
    srclen = strlen(value) as ::core::ffi::c_int;
    dstlen = strlen((*desc).fname) as ::core::ffi::c_int;
    if srclen > dstlen {
        let mut newValue: *mut ::core::ffi::c_char = HMalloc(
            (srclen + 1 as ::core::ffi::c_int) as size_t,
        ) as *mut ::core::ffi::c_char;
        let mut oldValue: *mut ::core::ffi::c_char = (*desc).fname;
        log_add(
            log_Level::log_Warning,
            b"Reallocating string space for '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            key,
        );
        strncpy(newValue, value, (srclen + 1 as ::core::ffi::c_int) as size_t);
        (*desc).resdata.str = newValue;
        (*desc).fname = newValue;
        HFree(oldValue as *mut ::core::ffi::c_void);
    } else {
        strncpy((*desc).fname, value, (dstlen + 1 as ::core::ffi::c_int) as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn res_IsInteger(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    return BOOLEAN(
        (!desc.is_null()
            && strcmp(
                (*(*desc).vtable).resType,
                b"INT32\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_GetInteger(
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    if desc.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"INT32\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return (*desc).resdata.num as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn res_PutInteger(
    mut key: *const ::core::ffi::c_char,
    mut value: ::core::ffi::c_int,
) {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    if desc.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"INT32\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        process_resource_desc(key, b"INT32:0\0".as_ptr() as *const ::core::ffi::c_char);
        desc = lookupResourceDesc(idx, key);
    }
    (*desc).resdata.num = value as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn res_IsBoolean(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    return BOOLEAN(
        (!desc.is_null()
            && strcmp(
                (*(*desc).vtable).resType,
                b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_GetBoolean(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    if desc.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(
        (if (*desc).resdata.num != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_PutBoolean(
    mut key: *const ::core::ffi::c_char,
    mut value: BOOLEAN,
) {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    if desc.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        process_resource_desc(
            key,
            b"BOOLEAN:false\0".as_ptr() as *const ::core::ffi::c_char,
        );
        desc = lookupResourceDesc(idx, key);
    }
    (*desc).resdata.num = value.0;
}
#[no_mangle]
pub unsafe extern "C" fn res_IsColor(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    return BOOLEAN(
        (!desc.is_null()
            && strcmp(
                (*(*desc).vtable).resType,
                b"COLOR\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_GetColor(mut key: *const ::core::ffi::c_char) -> Color {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    let mut num: DWORD = 0;
    if desc.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"COLOR\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        return buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    num = (*desc).resdata.num;
    return buildColorRgba(
        (num >> 24 as ::core::ffi::c_int) as BYTE,
        (num >> 16 as ::core::ffi::c_int & 0xff as DWORD) as BYTE,
        ((*desc).resdata.num >> 8 as ::core::ffi::c_int & 0xff as DWORD) as BYTE,
        (num & 0xff as DWORD) as BYTE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_PutColor(
    mut key: *const ::core::ffi::c_char,
    mut value: Color,
) {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    let mut desc: *mut ResourceDesc = lookupResourceDesc(idx, key);
    if desc.is_null()
        || strcmp(
            (*(*desc).vtable).resType,
            b"COLOR\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        process_resource_desc(
            key,
            b"COLOR:rgb(0, 0, 0)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        desc = lookupResourceDesc(idx, key);
    }
    (*desc).resdata.num = ((value.r as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
        | (value.g as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
        | (value.b as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | value.a as ::core::ffi::c_int) as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn res_HasKey(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut idx: RESOURCE_INDEX = _get_current_index_header();
    return BOOLEAN(
        !lookupResourceDesc(idx, key).is_null() as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_Remove(mut key: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut map: *mut CharHashTable_HashTable = (*_get_current_index_header()).map;
    let mut oldDesc: *mut ResourceDesc = CharHashTable_find(map, key)
        as *mut ResourceDesc;
    if !oldDesc.is_null() {
        if !(*oldDesc).resdata.ptr.is_null() {
            if (*oldDesc).refcount > 0 as ::core::ffi::c_uint {
                log_add(
                    log_Level::log_Warning,
                    b"WARNING: Replacing '%s' while it is live\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    key,
                );
            }
            if !(*oldDesc).vtable.is_null() && (*(*oldDesc).vtable).freeFun.is_some() {
                (*(*oldDesc).vtable)
                    .freeFun
                    .expect("non-null function pointer")((*oldDesc).resdata.ptr);
            }
        }
        HFree((*oldDesc).fname as *mut ::core::ffi::c_void);
        HFree(oldDesc as *mut ::core::ffi::c_void);
    }
    return BOOLEAN(CharHashTable_remove(map, key) as ::core::ffi::c_uint);
}
#[inline]
unsafe extern "C" fn buildColorRgba(
    mut r: BYTE,
    mut g: BYTE,
    mut b: BYTE,
    mut a: BYTE,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = r;
    c.g = g;
    c.b = b;
    c.a = a;
    return c;
}
