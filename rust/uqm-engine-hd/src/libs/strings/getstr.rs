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

// Rust translation of src/libs/strings/getstr.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fgets(
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        stream: *mut uio_Stream,
    ) -> *mut ::core::ffi::c_char;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn AllocStringTable(
        num_entries: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> STRING_TABLE;
    fn StringHashTable_newHashTable(
        hashFunction: StringHashTable_HashFunction,
        equalFunction: StringHashTable_EqualFunction,
        copyFunction: StringHashTable_CopyFunction,
        freeKeyFunction: StringHashTable_FreeKeyFunction,
        freeValueFunction: StringHashTable_FreeValueFunction,
        initialSize: uio_uint32,
        minFillQuotient: ::core::ffi::c_double,
        maxFillQuotient: ::core::ffi::c_double,
    ) -> *mut StringHashTable_HashTable;
    fn StringHashTable_add(
        hashTable: *mut StringHashTable_HashTable,
        key: *const StringHashTable_Key,
        value: *mut StringHashTable_Value,
    ) -> uio_bool;
    fn StringHashTable_deleteHashTable(hashTable: *mut StringHashTable_HashTable);
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn GetResourceData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut contentDir: *mut uio_DirHandle;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union RESOURCE_DATA {
    pub num: DWORD,
    pub ptr: *mut ::core::ffi::c_void,
    pub str: *const ::core::ffi::c_char,
}
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
unsafe extern "C" fn dword_convert(mut dword_array: *mut DWORD, mut num_dwords: COUNT) {
    let mut p: *mut BYTE = dword_array as *mut BYTE;
    loop {
        let c2rust_fresh0 = dword_array;
        dword_array = dword_array.offset(1);
        *c2rust_fresh0 = ((((*p.offset(0isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
            | *p.offset(1isize) as ::core::ffi::c_int) as DWORD)
            << 16 as ::core::ffi::c_int
            | (((*p.offset(2isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as UWORD as ::core::ffi::c_int | *p.offset(3isize) as ::core::ffi::c_int)
                as UWORD as DWORD;
        p = p.offset(4 as ::core::ffi::c_int as isize);
        num_dwords = num_dwords.wrapping_sub(1);
        if num_dwords == 0 {
            break;
        }
    };
}
unsafe extern "C" fn set_strtab_entry(
    mut strtab: *mut STRING_TABLE_DESC,
    mut index: ::core::ffi::c_int,
    mut value: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> STRING {
    let mut str: STRING = (*strtab).strings.offset(index as isize);
    if !(*str).data.is_null() {
        HFree((*str).data as *mut ::core::ffi::c_void);
        (*str).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*str).length = 0 as ::core::ffi::c_int;
    }
    if len != 0 {
        (*str).data = HMalloc(len as size_t) as STRINGPTR;
        (*str).length = len;
        memcpy(
            (*str).data as *mut ::core::ffi::c_void,
            value as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
    return str;
}
unsafe extern "C" fn copy_strings_to_strtab(
    mut strtab: *mut STRING_TABLE_DESC,
    mut firstIndex: size_t,
    mut count: size_t,
    mut data: *const ::core::ffi::c_char,
    mut lens: *const DWORD,
) {
    let mut stringI: size_t = 0;
    let mut off: *const ::core::ffi::c_char = data;
    stringI = 0 as size_t;
    while stringI < count {
        set_strtab_entry(
            strtab,
            firstIndex.wrapping_add(stringI) as ::core::ffi::c_int,
            off,
            *lens.offset(stringI as isize) as ::core::ffi::c_int,
        );
        off = off.offset(*lens.offset(stringI as isize) as isize);
        stringI = stringI.wrapping_add(1);
    }
}
unsafe extern "C" fn ensureBufSize(
    mut buf: *mut *mut ::core::ffi::c_char,
    mut curSize: *mut size_t,
    mut minSize: size_t,
    mut increment: size_t,
) -> BOOLEAN {
    let mut newBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut newSize: size_t = 0;
    if minSize <= *curSize {
        return BOOLEAN::TRUE;
    }
    newSize = minSize
        .wrapping_add(increment.wrapping_sub(1 as size_t))
        .wrapping_div(increment)
        .wrapping_mul(increment);
    newBuf = HRealloc(*buf as *mut ::core::ffi::c_void, newSize)
        as *mut ::core::ffi::c_char;
    if newBuf.is_null() {
        return BOOLEAN::FALSE;
    }
    *buf = newBuf;
    *curSize = newSize;
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn _GetConversationData(
    mut path: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    let mut dataLen: ::core::ffi::c_ulong = 0;
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut stringI: ::core::ffi::c_int = 0;
    let mut path_len: ::core::ffi::c_int = 0;
    let mut opos: DWORD = 0;
    let mut namedata: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut nlen: [DWORD; 2048] = [0; 2048];
    let mut NameOffs: DWORD = 0;
    let mut tot_name_size: size_t = 0;
    let mut strdata: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut slen: [DWORD; 2048] = [0; 2048];
    let mut StringOffs: DWORD = 0;
    let mut tot_string_size: size_t = 0;
    let mut clipdata: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut clen: [DWORD; 2048] = [0; 2048];
    let mut ClipOffs: DWORD = 0;
    let mut tot_clip_size: size_t = 0;
    let mut ts_data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tslen: [DWORD; 2048] = [0; 2048];
    let mut TSOffs: DWORD = 0;
    let mut tot_ts_size: size_t = 0 as size_t;
    let mut CurrentLine: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut paths: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut clip_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ts_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut timestamp_fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut nameHashTable: *mut StringHashTable_HashTable = ::core::ptr::null_mut::<
        StringHashTable_HashTable,
    >();
    strncpy(&raw mut paths as *mut ::core::ffi::c_char, path, 1023 as size_t);
    paths[1023usize] = '\0' as ::core::ffi::c_char;
    clip_path = strchr(
        &raw mut paths as *mut ::core::ffi::c_char,
        ':' as ::core::ffi::c_int,
    );
    if clip_path.is_null() {
        ts_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        *clip_path = '\0' as ::core::ffi::c_char;
        clip_path = clip_path.offset(1);
        ts_path = strchr(clip_path, ':' as ::core::ffi::c_int);
        if !ts_path.is_null() {
            *ts_path = '\0' as ::core::ffi::c_char;
            ts_path = ts_path.offset(1);
        }
    }
    fp = uqm_lua_checked!(res_OpenResFile(
        contentDir,
        &raw mut paths as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    if fp.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: Can't open '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut paths as *mut ::core::ffi::c_char,
        );
        (*resdata).ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
        return;
    }
    dataLen = uqm_lua_checked!(LengthResFile(fp), ()) as ::core::ffi::c_ulong;
    log_add(
        log_Level::log_Info,
        b"\t'%s' -- conversation phrases -- %lu bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut paths as *mut ::core::ffi::c_char,
        dataLen,
    );
    if !clip_path.is_null() {
        log_add(
            log_Level::log_Info,
            b"\t'%s' -- voice clip directory\0".as_ptr() as *const ::core::ffi::c_char,
            clip_path,
        );
    } else {
        log_add(
            log_Level::log_Info,
            b"\tNo associated voice clips\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !ts_path.is_null() {
        log_add(
            log_Level::log_Info,
            b"\t'%s' -- timestamps\0".as_ptr() as *const ::core::ffi::c_char,
            ts_path,
        );
    } else {
        log_add(
            log_Level::log_Info,
            b"\tNo associated timestamp file\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    '_err: {
        if dataLen == 0 as ::core::ffi::c_ulong {
            log_add(
                log_Level::log_Warning,
                b"Warning: Trying to load empty file '%s'.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
            );
        } else {
            tot_string_size = 4096 as size_t;
            strdata = HMalloc(tot_string_size) as *mut ::core::ffi::c_char;
            if !strdata.is_null() {
                tot_name_size = 4096 as size_t;
                namedata = HMalloc(tot_name_size) as *mut ::core::ffi::c_char;
                if !namedata.is_null() {
                    tot_clip_size = 4096 as size_t;
                    clipdata = HMalloc(tot_clip_size) as *mut ::core::ffi::c_char;
                    if !clipdata.is_null() {
                        ts_data = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        nameHashTable = StringHashTable_newHashTable(
                            None,
                            None,
                            None,
                            None,
                            None,
                            0 as uio_uint32,
                            0.85f64,
                            0.9f64,
                        );
                        if !nameHashTable.is_null() {
                            path_len = (if !clip_path.is_null() {
                                strlen(clip_path)
                            } else {
                                0 as size_t
                            }) as ::core::ffi::c_int;
                            if !ts_path.is_null() {
                                timestamp_fp = uqm_lua_checked!(uio_fopen(
                                    contentDir,
                                    ts_path,
                                    b"rb\0".as_ptr() as *const ::core::ffi::c_char,
                                ), ());
                                if !timestamp_fp.is_null() {
                                    tot_ts_size = 4096 as size_t;
                                    ts_data = HMalloc(tot_ts_size) as *mut ::core::ffi::c_char;
                                    if ts_data.is_null() {
                                        break '_err;
                                    }
                                }
                            }
                            opos = uqm_lua_checked!(uio_ftell(fp), ()) as DWORD;
                            stringI = -1 as ::core::ffi::c_int;
                            NameOffs = 0 as DWORD;
                            StringOffs = 0 as DWORD;
                            ClipOffs = 0 as DWORD;
                            TSOffs = 0 as DWORD;
                            loop {
                                let mut l: ::core::ffi::c_int = 0;
                                if uqm_lua_checked!(uio_fgets(
                                        &raw mut CurrentLine as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                                            as ::core::ffi::c_int,
                                        fp,
                                    ), ())
                                    .is_null()
                                {
                                    break;
                                }
                                if stringI
                                    >= 2048 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                {
                                    break;
                                }
                                if CurrentLine[0usize] as ::core::ffi::c_int
                                    == '#' as ::core::ffi::c_int
                                {
                                    let mut CopyLine: [::core::ffi::c_char; 1024] = [0; 1024];
                                    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                        ::core::ffi::c_char,
                                    >();
                                    let mut ts: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                        ::core::ffi::c_char,
                                    >();
                                    strcpy(
                                        &raw mut CopyLine as *mut ::core::ffi::c_char,
                                        &raw mut CurrentLine as *mut ::core::ffi::c_char,
                                    );
                                    name = strtok(
                                        (&raw mut CopyLine as *mut ::core::ffi::c_char)
                                            .offset(1isize),
                                        b"()\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    if !name.is_null() {
                                        if stringI >= 0 as ::core::ffi::c_int {
                                            while slen[stringI as usize] > 1 as DWORD
                                                && (*strdata
                                                    .offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                                                    as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                                                    || *strdata
                                                        .offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                                                        as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
                                            {
                                                slen[stringI as usize] = slen[stringI as usize]
                                                    .wrapping_sub(1);
                                                StringOffs = StringOffs.wrapping_sub(1);
                                                *strdata
                                                    .offset(StringOffs.wrapping_sub(1 as DWORD) as isize) = '\0'
                                                    as ::core::ffi::c_char;
                                            }
                                        }
                                        stringI += 1;
                                        slen[stringI as usize] = 0 as DWORD;
                                        l = strlen(name).wrapping_add(1 as size_t)
                                            as ::core::ffi::c_int;
                                        if ensureBufSize(
                                                &raw mut namedata,
                                                &raw mut tot_name_size,
                                                NameOffs.wrapping_add(l as DWORD) as size_t,
                                                4096 as size_t,
                                            )
                                            .0 == 0
                                        {
                                            break '_err;
                                        }
                                        strcpy(namedata.offset(NameOffs as isize), name);
                                        NameOffs = NameOffs.wrapping_add(l as DWORD);
                                        nlen[stringI as usize] = l as DWORD;
                                        if !timestamp_fp.is_null() {
                                            let mut TimeStampLine: [::core::ffi::c_char; 1024] = [0; 1024];
                                            let mut tsptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                ::core::ffi::c_char,
                                            >();
                                            let mut ts_ok: BOOLEAN = BOOLEAN::FALSE;
                                            uqm_lua_checked!(uio_fgets(
                                                &raw mut TimeStampLine as *mut ::core::ffi::c_char,
                                                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                                                    as ::core::ffi::c_int,
                                                timestamp_fp,
                                            ), ());
                                            if TimeStampLine[0usize] as ::core::ffi::c_int
                                                == '#' as ::core::ffi::c_int
                                            {
                                                tslen[stringI as usize] = 0 as DWORD;
                                                tsptr = strstr(
                                                    &raw mut TimeStampLine as *mut ::core::ffi::c_char,
                                                    name,
                                                );
                                                if !tsptr.is_null() {
                                                    tsptr = tsptr
                                                        .offset(strlen(name).wrapping_add(1 as size_t) as isize);
                                                    ts_ok = BOOLEAN::TRUE;
                                                    while strcspn(
                                                        tsptr,
                                                        b" \t\r\n\0".as_ptr() as *const ::core::ffi::c_char,
                                                    ) == 0 && *tsptr as ::core::ffi::c_int != 0
                                                    {
                                                        tsptr = tsptr.offset(1);
                                                    }
                                                    if *tsptr != 0 {
                                                        l = strlen(tsptr).wrapping_add(1 as size_t)
                                                            as ::core::ffi::c_int;
                                                        if ensureBufSize(
                                                                &raw mut ts_data,
                                                                &raw mut tot_ts_size,
                                                                TSOffs.wrapping_add(l as DWORD) as size_t,
                                                                4096 as size_t,
                                                            )
                                                            .0 == 0
                                                        {
                                                            break '_err;
                                                        }
                                                        strcpy(ts_data.offset(TSOffs as isize), tsptr);
                                                        TSOffs = TSOffs.wrapping_add(l as DWORD);
                                                        tslen[stringI as usize] = l as DWORD;
                                                    }
                                                }
                                            }
                                            if ts_ok.0 == 0 {
                                                log_add(
                                                    log_Level::log_Warning,
                                                    b"Invalid timestamp data for '%s'.  Disabling timestamps\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    name,
                                                );
                                                HFree(ts_data as *mut ::core::ffi::c_void);
                                                ts_data = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                uqm_lua_checked!(uio_fclose(timestamp_fp), ());
                                                timestamp_fp = ::core::ptr::null_mut::<uio_Stream>();
                                                TSOffs = 0 as DWORD;
                                            }
                                        }
                                        clen[stringI as usize] = 0 as DWORD;
                                        ts = strtok(
                                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                            b" \t\r\n)\0".as_ptr() as *const ::core::ffi::c_char,
                                        );
                                        if !ts.is_null() {
                                            l = (path_len as size_t)
                                                .wrapping_add(strlen(ts))
                                                .wrapping_add(1 as size_t) as ::core::ffi::c_int;
                                            if ensureBufSize(
                                                    &raw mut clipdata,
                                                    &raw mut tot_clip_size,
                                                    ClipOffs.wrapping_add(l as DWORD) as size_t,
                                                    4096 as size_t,
                                                )
                                                .0 == 0
                                            {
                                                break '_err;
                                            }
                                            if !clip_path.is_null() {
                                                strcpy(clipdata.offset(ClipOffs as isize), clip_path);
                                            }
                                            strcpy(
                                                clipdata
                                                    .offset(ClipOffs.wrapping_add(path_len as DWORD) as isize),
                                                ts,
                                            );
                                            ClipOffs = ClipOffs.wrapping_add(l as DWORD);
                                            clen[stringI as usize] = l as DWORD;
                                        }
                                    }
                                } else if stringI >= 0 as ::core::ffi::c_int {
                                    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                        ::core::ffi::c_char,
                                    >();
                                    l = strlen(&raw mut CurrentLine as *mut ::core::ffi::c_char)
                                        .wrapping_add(1 as size_t) as ::core::ffi::c_int;
                                    if ensureBufSize(
                                            &raw mut strdata,
                                            &raw mut tot_string_size,
                                            StringOffs.wrapping_add(l as DWORD) as size_t,
                                            4096 as size_t,
                                        )
                                        .0 == 0
                                    {
                                        break '_err;
                                    }
                                    if slen[stringI as usize] != 0 {
                                        slen[stringI as usize] = slen[stringI as usize]
                                            .wrapping_sub(1);
                                        StringOffs = StringOffs.wrapping_sub(1);
                                    }
                                    s = strdata.offset(StringOffs as isize);
                                    slen[stringI as usize] = slen[stringI as usize]
                                        .wrapping_add(l as DWORD);
                                    StringOffs = StringOffs.wrapping_add(l as DWORD);
                                    strcpy(s, &raw mut CurrentLine as *mut ::core::ffi::c_char);
                                }
                                if uqm_lua_checked!(uio_ftell(fp), ()) as ::core::ffi::c_int
                                    - opos as ::core::ffi::c_int
                                    >= dataLen as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if stringI >= 0 as ::core::ffi::c_int {
                                while slen[stringI as usize] > 1 as DWORD
                                    && (*strdata
                                        .offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                                        as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                                        || *strdata
                                            .offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                                            as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
                                {
                                    slen[stringI as usize] = slen[stringI as usize]
                                        .wrapping_sub(1);
                                    StringOffs = StringOffs.wrapping_sub(1);
                                    *strdata
                                        .offset(StringOffs.wrapping_sub(1 as DWORD) as isize) = '\0'
                                        as ::core::ffi::c_char;
                                }
                            }
                            if !timestamp_fp.is_null() {
                                uqm_lua_checked!(uio_fclose(timestamp_fp), ());
                            }
                            result = ::core::ptr::null_mut::<::core::ffi::c_void>();
                            stringI += 1;
                            if stringI != 0 {
                                let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut stringCount: ::core::ffi::c_int = stringI;
                                if ClipOffs != 0 {
                                    flags
                                        |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
                                }
                                if TSOffs != 0 {
                                    flags
                                        |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
                                }
                                flags
                                    |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
                                result = AllocStringTable(stringCount, flags)
                                    as *mut ::core::ffi::c_void;
                                if !result.is_null() {
                                    let mut lpST: *mut STRING_TABLE_DESC = result
                                        as *mut STRING_TABLE_DESC;
                                    let mut str: STRING = ::core::ptr::null_mut::<
                                        STRING_TABLE_ENTRY_DESC,
                                    >();
                                    stringI = 0 as ::core::ffi::c_int;
                                    copy_strings_to_strtab(
                                        lpST,
                                        stringI as size_t,
                                        stringCount as size_t,
                                        strdata,
                                        &raw mut slen as *mut DWORD,
                                    );
                                    stringI += stringCount;
                                    copy_strings_to_strtab(
                                        lpST,
                                        stringI as size_t,
                                        stringCount as size_t,
                                        namedata,
                                        &raw mut nlen as *mut DWORD,
                                    );
                                    stringI += stringCount;
                                    if (*lpST).flags as ::core::ffi::c_int
                                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                                    {
                                        copy_strings_to_strtab(
                                            lpST,
                                            stringI as size_t,
                                            stringCount as size_t,
                                            clipdata,
                                            &raw mut clen as *mut DWORD,
                                        );
                                        stringI += stringCount;
                                    }
                                    if (*lpST).flags as ::core::ffi::c_int
                                        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                                    {
                                        copy_strings_to_strtab(
                                            lpST,
                                            stringI as size_t,
                                            stringCount as size_t,
                                            ts_data,
                                            &raw mut tslen as *mut DWORD,
                                        );
                                    }
                                    str = (*lpST).strings.offset(stringCount as isize)
                                        as STRING;
                                    stringI = 0 as ::core::ffi::c_int;
                                    while stringI < stringCount {
                                        StringHashTable_add(
                                            nameHashTable,
                                            (*str.offset(stringI as isize)).data
                                                as *const StringHashTable_Key,
                                            str.offset(stringI as isize),
                                        );
                                        stringI += 1;
                                    }
                                    (*lpST).nameIndex = nameHashTable;
                                }
                            }
                            HFree(strdata as *mut ::core::ffi::c_void);
                            if !clipdata.is_null() {
                                HFree(clipdata as *mut ::core::ffi::c_void);
                            }
                            if !ts_data.is_null() {
                                HFree(ts_data as *mut ::core::ffi::c_void);
                            }
                            (*resdata).ptr = result;
                            return;
                        }
                    }
                }
            }
        }
    }
    if !nameHashTable.is_null() {
        StringHashTable_deleteHashTable(nameHashTable);
    }
    if !ts_data.is_null() {
        HFree(ts_data as *mut ::core::ffi::c_void);
    }
    if !clipdata.is_null() {
        HFree(clipdata as *mut ::core::ffi::c_void);
    }
    if !strdata.is_null() {
        HFree(strdata as *mut ::core::ffi::c_void);
    }
    uqm_lua_checked!(res_CloseResFile(fp), ());
    (*resdata).ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn _GetStringData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut stringI: ::core::ffi::c_int = 0;
    let mut opos: DWORD = 0;
    let mut slen: [DWORD; 2048] = [0; 2048];
    let mut StringOffs: DWORD = 0;
    let mut tot_string_size: size_t = 0;
    let mut CurrentLine: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut strdata: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    tot_string_size = 4096 as size_t;
    strdata = HMalloc(tot_string_size) as *mut ::core::ffi::c_char;
    '_err: {
        if !strdata.is_null() {
            opos = uqm_lua_checked!(uio_ftell(fp), ::core::ptr::null_mut()) as DWORD;
            stringI = -1 as ::core::ffi::c_int;
            StringOffs = 0 as DWORD;
            loop {
                let mut l: ::core::ffi::c_int = 0;
                if uqm_lua_checked!(uio_fgets(
                        &raw mut CurrentLine as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                            as ::core::ffi::c_int,
                        fp,
                    ), ::core::ptr::null_mut())
                    .is_null()
                {
                    break;
                } else {
                    if stringI >= 2048 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                        break;
                    }
                    if CurrentLine[0usize] as ::core::ffi::c_int
                        == '#' as ::core::ffi::c_int
                    {
                        let mut CopyLine: [::core::ffi::c_char; 1024] = [0; 1024];
                        let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        strcpy(
                            &raw mut CopyLine as *mut ::core::ffi::c_char,
                            &raw mut CurrentLine as *mut ::core::ffi::c_char,
                        );
                        s = strtok(
                            (&raw mut CopyLine as *mut ::core::ffi::c_char)
                                .offset(1isize),
                            b"()\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !s.is_null() {
                            if stringI >= 0 as ::core::ffi::c_int {
                                while slen[stringI as usize] > 1 as DWORD
                                    && (*strdata
                                        .offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                                        as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                                        || *strdata
                                            .offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                                            as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
                                {
                                    slen[stringI as usize] = slen[stringI as usize]
                                        .wrapping_sub(1);
                                    StringOffs = StringOffs.wrapping_sub(1);
                                    *strdata
                                        .offset(StringOffs.wrapping_sub(1 as DWORD) as isize) = '\0'
                                        as ::core::ffi::c_char;
                                }
                            }
                            stringI += 1;
                            slen[stringI as usize] = 0 as DWORD;
                        }
                    } else if stringI >= 0 as ::core::ffi::c_int {
                        let mut s_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        l = strlen(&raw mut CurrentLine as *mut ::core::ffi::c_char)
                            .wrapping_add(1 as size_t) as ::core::ffi::c_int;
                        if ensureBufSize(
                                &raw mut strdata,
                                &raw mut tot_string_size,
                                StringOffs.wrapping_add(l as DWORD) as size_t,
                                4096 as size_t,
                            )
                            .0 == 0
                        {
                            break '_err;
                        }
                        if slen[stringI as usize] != 0 {
                            slen[stringI as usize] = slen[stringI as usize]
                                .wrapping_sub(1);
                            StringOffs = StringOffs.wrapping_sub(1);
                        }
                        s_0 = strdata.offset(StringOffs as isize);
                        slen[stringI as usize] = slen[stringI as usize]
                            .wrapping_add(l as DWORD);
                        StringOffs = StringOffs.wrapping_add(l as DWORD);
                        strcpy(s_0, &raw mut CurrentLine as *mut ::core::ffi::c_char);
                    }
                    if uqm_lua_checked!(uio_ftell(fp), ::core::ptr::null_mut()) as ::core::ffi::c_int - opos as ::core::ffi::c_int
                        >= length as ::core::ffi::c_int
                    {
                        break;
                    }
                }
            }
            if stringI >= 0 as ::core::ffi::c_int {
                while slen[stringI as usize] > 1 as DWORD
                    && (*strdata.offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                        as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                        || *strdata.offset(StringOffs.wrapping_sub(2 as DWORD) as isize)
                            as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
                {
                    slen[stringI as usize] = slen[stringI as usize].wrapping_sub(1);
                    StringOffs = StringOffs.wrapping_sub(1);
                    *strdata.offset(StringOffs.wrapping_sub(1 as DWORD) as isize) = '\0'
                        as ::core::ffi::c_char;
                }
            }
            result = ::core::ptr::null_mut::<::core::ffi::c_void>();
            stringI += 1;
            if stringI != 0 {
                let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut stringCount: ::core::ffi::c_int = stringI;
                result = AllocStringTable(stringI, flags) as *mut ::core::ffi::c_void;
                if !result.is_null() {
                    let mut lpST: *mut STRING_TABLE_DESC = result
                        as *mut STRING_TABLE_DESC;
                    copy_strings_to_strtab(
                        lpST,
                        0 as size_t,
                        stringCount as size_t,
                        strdata,
                        &raw mut slen as *mut DWORD,
                    );
                }
            }
            HFree(strdata as *mut ::core::ffi::c_void);
            return result;
        }
    }
    if !strdata.is_null() {
        HFree(strdata as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn _GetBinaryTableData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    result = uqm_lua_checked!(GetResourceData(fp, length), ::core::ptr::null_mut());
    if !result.is_null() {
        let mut fileData: *mut DWORD = ::core::ptr::null_mut::<DWORD>();
        let mut lpST: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
        fileData = result as *mut DWORD;
        dword_convert(fileData, 1 as COUNT);
        lpST = AllocStringTable(
            *fileData.offset(0isize) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if !lpST.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut size: ::core::ffi::c_int = 0;
            let mut stringptr: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
            size = (*lpST).size;
            dword_convert(
                fileData.offset(1 as ::core::ffi::c_int as isize),
                (size + 1 as ::core::ffi::c_int) as COUNT,
            );
            stringptr = fileData
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(size as isize)
                .offset(*fileData.offset(1isize) as isize) as *mut BYTE;
            i = 0 as ::core::ffi::c_int;
            while i < size {
                set_strtab_entry(
                    lpST,
                    i,
                    stringptr as *mut ::core::ffi::c_char,
                    *fileData.offset((2 as ::core::ffi::c_int + i) as isize)
                        as ::core::ffi::c_int,
                );
                stringptr = stringptr
                    .offset(
                        *fileData.offset((2 as ::core::ffi::c_int + i) as isize) as isize,
                    );
                i += 1;
            }
        }
        HFree(result);
        result = lpST as *mut ::core::ffi::c_void;
    }
    return result;
}
