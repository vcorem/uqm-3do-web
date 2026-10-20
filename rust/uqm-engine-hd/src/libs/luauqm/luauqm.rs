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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/libs/luauqm/luauqm.c.
extern "C" {
    pub type lua_State;
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    #[link_name = "uqm_lua_guard_lua_gettop"]
    fn lua_gettop(L: *mut lua_State) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_settop"]
    fn lua_settop(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_type"]
    fn lua_type(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_typename"]
    fn lua_typename(
        L: *mut lua_State,
        tp: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_tonumberx"]
    fn lua_tonumberx(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        isnum: *mut ::core::ffi::c_int,
    ) -> lua_Number;
    #[link_name = "uqm_lua_guard_lua_toboolean"]
    fn lua_toboolean(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_tolstring"]
    fn lua_tolstring(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        len: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pushinteger"]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    #[link_name = "uqm_lua_guard_lua_pushstring"]
    fn lua_pushstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pushcclosure"]
    fn lua_pushcclosure(L: *mut lua_State, r#fn: lua_CFunction, n: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_getglobal"]
    fn lua_getglobal(L: *mut lua_State, var: *const ::core::ffi::c_char);
    #[link_name = "uqm_lua_guard_lua_rawgeti"]
    fn lua_rawgeti(L: *mut lua_State, idx: ::core::ffi::c_int, n: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_createtable"]
    fn lua_createtable(
        L: *mut lua_State,
        narr: ::core::ffi::c_int,
        nrec: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_lua_setfield"]
    fn lua_setfield(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        k: *const ::core::ffi::c_char,
    );
    #[link_name = "uqm_lua_guard_lua_rawseti"]
    fn lua_rawseti(L: *mut lua_State, idx: ::core::ffi::c_int, n: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_callk"]
    fn lua_callk(
        L: *mut lua_State,
        nargs: ::core::ffi::c_int,
        nresults: ::core::ffi::c_int,
        ctx: ::core::ffi::c_int,
        k: lua_CFunction,
    );
    #[link_name = "uqm_lua_guard_lua_pcallk"]
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: ::core::ffi::c_int,
        nresults: ::core::ffi::c_int,
        errfunc: ::core::ffi::c_int,
        ctx: ::core::ffi::c_int,
        k: lua_CFunction,
    ) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_load"]
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut ::core::ffi::c_void,
        chunkname: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn luaopen_base(L: *mut lua_State) -> ::core::ffi::c_int;
    fn luaopen_table(L: *mut lua_State) -> ::core::ffi::c_int;
    fn luaopen_string(L: *mut lua_State) -> ::core::ffi::c_int;
    fn luaopen_bit32(L: *mut lua_State) -> ::core::ffi::c_int;
    fn luaopen_math(L: *mut lua_State) -> ::core::ffi::c_int;
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
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fread(
        buf: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn InstallScriptResType() -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
pub type ptrdiff_t = isize;
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
pub type lua_Reader = Option<
    unsafe extern "C" fn(
        *mut lua_State,
        *mut ::core::ffi::c_void,
        *mut size_t,
    ) -> *const ::core::ffi::c_char,
>;
pub type lua_Number = ::core::ffi::c_double;
pub type lua_Integer = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const ::core::ffi::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_EnumValue {
    pub name: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_ReaderState {
    pub fileName: *const ::core::ffi::c_char,
    pub r#in: *mut uio_Stream,
    pub buf: *mut ::core::ffi::c_char,
}
static mut safeLibs: [luaL_Reg; 6] = [
    luaL_Reg {
        name: b"\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaopen_base as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"bit32\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaopen_bit32 as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"math\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaopen_math as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"string\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaopen_string as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"table\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaopen_table as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
#[no_mangle]
pub unsafe extern "C" fn luaUqm_loadLib(
    mut luaState: *mut lua_State,
    mut lib: *const luaL_Reg,
) {
    uqm_lua_checked!(lua_rawgeti(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ), ());
    uqm_lua_checked!(lua_pushcclosure(luaState, (*lib).func, 0 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_pushstring(luaState, (*lib).name), ());
    uqm_lua_checked!(lua_callk(
        luaState,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        None,
    ), ());
    uqm_lua_checked!(lua_setfield(luaState, -2 as ::core::ffi::c_int, (*lib).name), ());
    uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_loadLibs(
    mut luaState: *mut lua_State,
    mut libs: *const luaL_Reg,
) {
    while (*libs).func.is_some() {
        uqm_lua_checked!(luaUqm_loadLib(luaState, libs), ());
        libs = libs.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_loadSafeDefaultLibs(mut luaState: *mut lua_State) {
    uqm_lua_checked!(luaUqm_loadLibs(luaState, &raw const safeLibs as *const luaL_Reg), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_init() {
    InstallScriptResType();
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_uninit() {}
unsafe extern "C" fn luaUqm_reader(
    mut luaState: *mut lua_State,
    mut data: *mut ::core::ffi::c_void,
    mut size: *mut size_t,
) -> *const ::core::ffi::c_char {
    let mut readerState: *mut luaUqm_ReaderState = data as *mut luaUqm_ReaderState;
    let mut numRead: size_t = uqm_lua_checked!(uio_fread(
        (*readerState).buf as *mut ::core::ffi::c_void,
        1 as size_t,
        0x10000 as size_t,
        (*readerState).r#in,
    ), ::core::ptr::null());
    if numRead == -1 as ::core::ffi::c_int as size_t {
        log_add(
            log_Level::log_Error,
            b"luaUqm_loadScript(): Read error readin script file '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*readerState).fileName,
        );
        *size = 0 as size_t;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    *size = numRead;
    return (*readerState).buf;
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_prepareEnvironment(mut luaState: *mut lua_State) {
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_rawseti(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ), ());
    uqm_lua_checked!(luaUqm_loadSafeDefaultLibs(luaState), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_loadScript(
    mut luaState: *mut lua_State,
    mut dir: *mut uio_DirHandle,
    mut fileName: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut r#in: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut readerState: luaUqm_ReaderState = luaUqm_ReaderState {
        fileName: ::core::ptr::null::<::core::ffi::c_char>(),
        r#in: ::core::ptr::null_mut::<uio_Stream>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    log_add(
        log_Level::log_Debug,
        b"Loading script '%s'.\0".as_ptr() as *const ::core::ffi::c_char,
        fileName,
    );
    r#in = uqm_lua_checked!(uio_fopen(dir, fileName, b"rt\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    if r#in.is_null() {
        log_add(
            log_Level::log_Error,
            b"luaUqm_loadScript(): Unable to open script file '%s' for reading.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            fileName,
        );
    } else {
        buf = malloc(0x10000 as size_t) as *mut ::core::ffi::c_char;
        if !buf.is_null() {
            readerState.fileName = fileName;
            readerState.r#in = r#in;
            readerState.buf = buf;
            if uqm_lua_checked!(lua_load(
                luaState,
                Some(
                    luaUqm_reader
                        as unsafe extern "C" fn(
                            *mut lua_State,
                            *mut ::core::ffi::c_void,
                            *mut size_t,
                        ) -> *const ::core::ffi::c_char,
                ),
                &raw mut readerState as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ), BOOLEAN(0)) != 0 as ::core::ffi::c_int
            {
                log_add(
                    log_Level::log_Error,
                    b"luaUqm_loadScript(): lua_load() failed: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    uqm_lua_checked!(lua_tolstring(
                        luaState,
                        -1 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<size_t>(),
                    ), BOOLEAN(0)),
                );
                uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), BOOLEAN(0));
            } else {
                free(buf as *mut ::core::ffi::c_void);
                uqm_lua_checked!(uio_fclose(r#in), BOOLEAN(0));
                return BOOLEAN::TRUE;
            }
        }
    }
    if !buf.is_null() {
        free(buf as *mut ::core::ffi::c_void);
    }
    if !r#in.is_null() {
        uqm_lua_checked!(uio_fclose(r#in), BOOLEAN(0));
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_runScript(
    mut luaState: *mut lua_State,
    mut dir: *mut uio_DirHandle,
    mut fileName: *const ::core::ffi::c_char,
) -> BOOLEAN {
    if uqm_lua_checked!(luaUqm_loadScript(luaState, dir, fileName), BOOLEAN(0)).0 == 0 {
        return BOOLEAN::FALSE;
    }
    return uqm_lua_checked!(luaUqm_callStackFunction(luaState), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_runLuaDir(
    mut luaState: *mut lua_State,
    mut dirHandle: *mut uio_DirHandle,
    mut luaDirName: *const ::core::ffi::c_char,
) {
    let mut luaDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut luaFiles: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    let mut fileI: ::core::ffi::c_int = 0;
    luaDir = uqm_lua_checked!(uio_openDirRelative(dirHandle, luaDirName, 0 as ::core::ffi::c_int), ());
    if luaDir.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: Could not open Lua script directory '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            luaDirName,
        );
    } else {
        luaFiles = uqm_lua_checked!(uio_getDirList(
            luaDir,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b".lua\0".as_ptr() as *const ::core::ffi::c_char,
            match_MatchType::match_MATCH_SUFFIX,
        ), ());
        if luaFiles.is_null() {
            log_add(
                log_Level::log_Warning,
                b"Warning: Could not read Lua script directory '%s'.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                luaDirName,
            );
        } else {
            log_add(
                log_Level::log_Debug,
                b"Script directory '%s': loading %d file(s).\0".as_ptr()
                    as *const ::core::ffi::c_char,
                luaDirName,
                (*luaFiles).numNames,
            );
            fileI = 0 as ::core::ffi::c_int;
            while fileI < (*luaFiles).numNames {
                let mut fileName: *const ::core::ffi::c_char = *(*luaFiles)
                    .names
                    .offset(fileI as isize);
                uqm_lua_checked!(luaUqm_runScript(luaState, luaDir, fileName), ());
                fileI += 1;
            }
            uio_DirList_free(luaFiles);
            uio_closeDir(luaDir);
            return;
        }
    }
    if !luaFiles.is_null() {
        uio_DirList_free(luaFiles);
    }
    if !luaDir.is_null() {
        uio_closeDir(luaDir);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_callStackFunction(
    mut luaState: *mut lua_State,
) -> BOOLEAN {
    if uqm_lua_checked!(lua_pcallk(
        luaState,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        None,
    ), BOOLEAN(0)) != 0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"[script] A script error occurred in luaUqm_callStackFunction(): %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaState,
                -1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), BOOLEAN(0)),
        );
        uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_callFunction(
    mut luaState: *mut lua_State,
    mut str: *const ::core::ffi::c_char,
) -> BOOLEAN {
    uqm_lua_checked!(lua_getglobal(luaState, str), BOOLEAN(0));
    if !(uqm_lua_checked!(lua_type(luaState, -1 as ::core::ffi::c_int), BOOLEAN(0)) == 6 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    return uqm_lua_checked!(luaUqm_callStackFunction(luaState), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_makeEnum(
    mut luaState: *mut lua_State,
    mut enumVals: *const luaUqm_EnumValue,
) {
    let mut enumPtr: *const luaUqm_EnumValue = ::core::ptr::null::<luaUqm_EnumValue>();
    let mut enumCount: size_t = 0 as size_t;
    enumPtr = enumVals;
    while !(*enumPtr).name.is_null() {
        enumCount = enumCount.wrapping_add(1);
        enumPtr = enumPtr.offset(1);
    }
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, enumCount as ::core::ffi::c_int), ());
    enumPtr = enumVals;
    while !(*enumPtr).name.is_null() {
        uqm_lua_checked!(lua_pushinteger(luaState, (*enumPtr).value as lua_Integer), ());
        uqm_lua_checked!(lua_setfield(luaState, -2 as ::core::ffi::c_int, (*enumPtr).name), ());
        enumPtr = enumPtr.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_dumpStack(mut luaState: *mut lua_State) {
    let mut top: ::core::ffi::c_int = uqm_lua_checked!(lua_gettop(luaState), ());
    let mut stackI: ::core::ffi::c_int = 0;
    stackI = 1 as ::core::ffi::c_int;
    while stackI <= top {
        let mut r#type: ::core::ffi::c_int = uqm_lua_checked!(lua_type(luaState, stackI), ());
        let mut typeName: *const ::core::ffi::c_char = uqm_lua_checked!(lua_typename(luaState, r#type), ());
        log_add(
            log_Level::log_Debug,
            b"[%d] (%s)\0".as_ptr() as *const ::core::ffi::c_char,
            stackI,
            typeName,
        );
        match r#type {
            1 => {
                log_add(
                    log_Level::log_Debug,
                    b"    %s\0".as_ptr() as *const ::core::ffi::c_char,
                    if uqm_lua_checked!(lua_toboolean(luaState, stackI), ()) != 0 {
                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"false\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
            }
            3 => {
                log_add(
                    log_Level::log_Debug,
                    b"    %g\0".as_ptr() as *const ::core::ffi::c_char,
                    uqm_lua_checked!(lua_tonumberx(
                        luaState,
                        stackI,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ), ()),
                );
            }
            4 => {
                log_add(
                    log_Level::log_Debug,
                    b"    \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                    uqm_lua_checked!(lua_tolstring(luaState, stackI, ::core::ptr::null_mut::<size_t>()), ()),
                );
            }
            -1 | 0 | 2 | 5 | 6 | 7 | 8 | _ => {}
        }
        stackI += 1;
    }
}
