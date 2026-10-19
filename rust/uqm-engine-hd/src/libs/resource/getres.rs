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

// Rust translation of src/libs/resource/getres.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn CharHashTable_find(
        hashTable: *mut CharHashTable_HashTable,
        key: *const CharHashTable_Key,
    ) -> *mut ::core::ffi::c_void;
    static mut contentDir: *mut uio_DirHandle;
    fn _get_current_index_header() -> RESOURCE_INDEX;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
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
pub type ResourceLoadFileFun = unsafe extern "C" fn(
    *mut uio_Stream,
    DWORD,
) -> *mut ::core::ffi::c_void;
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
#[no_mangle]
pub static mut _cur_resfile_name: *const ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>();
#[no_mangle]
pub unsafe extern "C" fn lookupResourceDesc(
    mut idx: RESOURCE_INDEX,
    mut res: RESOURCE,
) -> *mut ResourceDesc {
    return CharHashTable_find((*idx).map, res) as *mut ResourceDesc;
}
#[no_mangle]
pub unsafe extern "C" fn loadResourceDesc(mut desc: *mut ResourceDesc) {
    uqm_lua_checked!((*(*desc).vtable)
        .loadFun
        .expect("non-null function pointer")((*desc).fname, &raw mut (*desc).resdata), ());
}
#[no_mangle]
pub unsafe extern "C" fn LoadResourceFromPath(
    mut path: *const ::core::ffi::c_char,
    mut loadFun: Option<ResourceLoadFileFun>,
) -> *mut ::core::ffi::c_void {
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut dataLen: ::core::ffi::c_ulong = 0;
    let mut resdata: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    stream = uqm_lua_checked!(res_OpenResFile(
        contentDir,
        path,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), ::core::ptr::null_mut());
    if stream.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: Can't open '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            path,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    dataLen = uqm_lua_checked!(LengthResFile(stream), ::core::ptr::null_mut()) as ::core::ffi::c_ulong;
    log_add(
        log_Level::log_Info,
        b"\t'%s' -- %lu bytes\0".as_ptr() as *const ::core::ffi::c_char,
        path,
        dataLen,
    );
    if dataLen == 0 as ::core::ffi::c_ulong {
        log_add(
            log_Level::log_Warning,
            b"Warning: Trying to load empty file '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            path,
        );
        uqm_lua_checked!(res_CloseResFile(stream), ::core::ptr::null_mut());
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        _cur_resfile_name = path;
        resdata = uqm_lua_checked!(Some(loadFun.expect("non-null function pointer"))
            .expect("non-null function pointer")(stream, dataLen as DWORD), ::core::ptr::null_mut());
        _cur_resfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        uqm_lua_checked!(res_CloseResFile(stream), ::core::ptr::null_mut());
        return resdata;
    };
}
#[no_mangle]
pub unsafe extern "C" fn res_GetResourceType(
    mut res: RESOURCE,
) -> *const ::core::ffi::c_char {
    let mut resourceIndex: RESOURCE_INDEX = ::core::ptr::null_mut::<
        RESOURCE_INDEX_DESC,
    >();
    let mut desc: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    if res.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Trying to get type of null resource\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    resourceIndex = _get_current_index_header();
    desc = lookupResourceDesc(resourceIndex, res);
    if desc.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Trying to get type of undefined resource '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            res,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(*desc).vtable).resType;
}
#[no_mangle]
pub unsafe extern "C" fn res_GetResource(mut res: RESOURCE) -> *mut ::core::ffi::c_void {
    let mut resourceIndex: RESOURCE_INDEX = ::core::ptr::null_mut::<
        RESOURCE_INDEX_DESC,
    >();
    let mut desc: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    if res.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Trying to get null resource\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    resourceIndex = _get_current_index_header();
    desc = lookupResourceDesc(resourceIndex, res);
    if desc.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Trying to get undefined resource '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            res,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*desc).resdata.ptr.is_null() {
        uqm_lua_checked!(loadResourceDesc(desc), ::core::ptr::null_mut());
    }
    if !(*desc).resdata.ptr.is_null() {
        (*desc).refcount = (*desc).refcount.wrapping_add(1);
    }
    return (*desc).resdata.ptr;
}
#[no_mangle]
pub unsafe extern "C" fn res_GetIntResource(mut res: RESOURCE) -> DWORD {
    let mut resourceIndex: RESOURCE_INDEX = ::core::ptr::null_mut::<
        RESOURCE_INDEX_DESC,
    >();
    let mut desc: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    if res.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Trying to get null resource\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as DWORD;
    }
    resourceIndex = _get_current_index_header();
    desc = lookupResourceDesc(resourceIndex, res);
    if desc.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Trying to get undefined resource '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            res,
        );
        return 0 as DWORD;
    }
    return (*desc).resdata.num;
}
#[no_mangle]
pub unsafe extern "C" fn res_GetBooleanResource(mut res: RESOURCE) -> BOOLEAN {
    return BOOLEAN(
        (res_GetIntResource(res) != 0 as DWORD) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn res_FreeResource(mut res: RESOURCE) {
    let mut desc: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    let mut freeFun: Option<ResourceFreeFun> = None;
    desc = lookupResourceDesc(_get_current_index_header(), res);
    if desc.is_null() {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to free an unrecognised resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*desc).refcount > 0 as ::core::ffi::c_uint {
        (*desc).refcount = (*desc).refcount.wrapping_sub(1);
    } else {
        log_add(
            log_Level::log_Debug,
            b"Warning: freeing an unreferenced resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if (*desc).refcount > 0 as ::core::ffi::c_uint {
        return;
    }
    freeFun = (*(*desc).vtable).freeFun;
    if freeFun.is_none() {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to free a non-heap resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*desc).resdata.ptr.is_null() {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to free not loaded resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    uqm_lua_checked!(Some(freeFun.expect("non-null function pointer"))
        .expect("non-null function pointer")((*desc).resdata.ptr), ());
    (*desc).resdata.ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn res_DetachResource(
    mut res: RESOURCE,
) -> *mut ::core::ffi::c_void {
    let mut desc: *mut ResourceDesc = ::core::ptr::null_mut::<ResourceDesc>();
    let mut freeFun: Option<ResourceFreeFun> = None;
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    desc = lookupResourceDesc(_get_current_index_header(), res);
    if desc.is_null() {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to detach from an unrecognised resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    freeFun = (*(*desc).vtable).freeFun;
    if freeFun.is_none() {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to detach from a non-heap resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*desc).resdata.ptr.is_null() {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to detach from a not loaded resource.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*desc).refcount > 1 as ::core::ffi::c_uint {
        log_add(
            log_Level::log_Debug,
            b"Warning: trying to detach a resource referenced %u times\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*desc).refcount,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    result = (*desc).resdata.ptr;
    (*desc).resdata.ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*desc).refcount = 0 as ::core::ffi::c_uint;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn FreeResourceData(
    mut data: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    HFree(data);
    return BOOLEAN::TRUE;
}
