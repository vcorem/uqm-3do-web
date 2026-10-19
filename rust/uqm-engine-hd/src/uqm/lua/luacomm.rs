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

/*
 * This file contains code for using Lua with the game conversations.
 */

// Rust translation of src/uqm/lua/luacomm.c.
extern "C" {
    pub type uio_DirHandle;
    pub type lua_State;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn luaUqm_custom_init(
        luaState: *mut lua_State,
        funs: *const luaUqm_custom_Function,
    ) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_luaL_loadstring"]
    fn luaL_loadstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn luaUqm_prepareEnvironment(luaState: *mut lua_State);
    fn luaUqm_loadScript(
        luaState: *mut lua_State,
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn luaUqm_loadLibs(luaState: *mut lua_State, libs: *const luaL_Reg);
    fn luaUqm_callStackFunction(luaState: *mut lua_State) -> BOOLEAN;
    fn luaUqm_callFunction(
        luaState: *mut lua_State,
        str: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn ReleaseScriptResData(data: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn LoadScriptInstance(res: RESOURCE) -> *mut ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_settop"]
    fn lua_settop(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_type"]
    fn lua_type(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_typename"]
    fn lua_typename(
        L: *mut lua_State,
        tp: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_tolstring"]
    fn lua_tolstring(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        len: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pcallk"]
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: ::core::ffi::c_int,
        nresults: ::core::ffi::c_int,
        errfunc: ::core::ffi::c_int,
        ctx: ::core::ffi::c_int,
        k: lua_CFunction,
    ) -> ::core::ffi::c_int;
    static mut contentDir: *mut uio_DirHandle;
    static mut luaUqm_globalState: *mut lua_State;
    fn luaUqm_comm_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_event_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_log_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_state_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
}
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const ::core::ffi::c_char,
    pub func: lua_CFunction,
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
#[no_mangle]
pub static mut luaUqm_commState: *mut lua_State = ::core::ptr::null_mut::<lua_State>();
static mut commLibs: [luaL_Reg; 5] = [
    luaL_Reg {
        name: b"comm\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_open
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"event\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_event_open
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"log\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_log_open as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"state\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_open
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_init(
    mut customFuncs: *const luaUqm_custom_Function,
    mut scriptRes: RESOURCE,
) -> BOOLEAN {
    (luaUqm_commState.is_null()
        || {
            __assert_fail(
                b"luaUqm_commState == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luacomm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                56 as ::core::ffi::c_int,
                b"luaUqm_comm_init\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    luaUqm_commState = luaUqm_globalState;
    uqm_lua_checked!(luaUqm_prepareEnvironment(luaUqm_commState), BOOLEAN(0));
    uqm_lua_checked!(luaUqm_loadLibs(luaUqm_commState, &raw const commLibs as *const luaL_Reg), BOOLEAN(0));
    if !customFuncs.is_null() {
        uqm_lua_checked!(luaUqm_custom_init(luaUqm_commState, customFuncs), BOOLEAN(0));
        uqm_lua_checked!(lua_settop(luaUqm_commState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), BOOLEAN(0));
    }
    if !scriptRes.is_null() {
        let mut scriptFileName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut loadOk: BOOLEAN = BOOLEAN::FALSE;
        scriptFileName = uqm_lua_checked!(LoadScriptInstance(scriptRes), BOOLEAN(0));
        if scriptFileName.is_null() {
            return BOOLEAN::FALSE;
        }
        loadOk = uqm_lua_checked!(luaUqm_loadScript(luaUqm_commState, contentDir, scriptFileName), BOOLEAN(0));
        ReleaseScriptResData(scriptFileName as *mut ::core::ffi::c_void);
        if loadOk.0 == 0 {
            return BOOLEAN::FALSE;
        }
        uqm_lua_checked!(luaUqm_callStackFunction(luaUqm_commState), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_uninit() {
    (!luaUqm_commState.is_null()
        || {
            __assert_fail(
                b"luaUqm_commState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luacomm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int,
                b"luaUqm_comm_uninit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    luaUqm_commState = ::core::ptr::null_mut::<lua_State>();
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_genericInit() {
    (!luaUqm_commState.is_null()
        || {
            __assert_fail(
                b"luaUqm_commState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luacomm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                b"luaUqm_comm_genericInit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!(luaUqm_callFunction(
        luaUqm_commState,
        b"init\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_genericPost() {
    (!luaUqm_commState.is_null()
        || {
            __assert_fail(
                b"luaUqm_commState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luacomm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_int,
                b"luaUqm_comm_genericPost\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!(luaUqm_callFunction(
        luaUqm_commState,
        b"post\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_genericUninit() {
    (!luaUqm_commState.is_null()
        || {
            __assert_fail(
                b"luaUqm_commState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luacomm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                115 as ::core::ffi::c_int,
                b"luaUqm_comm_genericUninit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!(luaUqm_callFunction(
        luaUqm_commState,
        b"uninit\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    luaUqm_comm_uninit();
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_stringNeedsInterpolate(
    mut str: *const ::core::ffi::c_char,
) -> BOOLEAN {
    return BOOLEAN(
        !strstr(str, b"<%\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn luaUqm_comm_addToBuffer(
    mut buf: *mut *mut ::core::ffi::c_char,
    mut bufLen: *mut size_t,
    mut bufPtr: *mut *mut ::core::ffi::c_char,
    mut add: *const ::core::ffi::c_char,
    mut addLen: size_t,
) {
    let mut bufFill: size_t = (*bufPtr).offset_from(*buf) as size_t;
    let mut newLen: size_t = *bufLen;
    while (bufFill.wrapping_add(addLen) >= newLen) as ::core::ffi::c_int as size_t != 0 {
        newLen = newLen.wrapping_mul(2 as size_t);
    }
    if newLen != *bufLen {
        let mut newBuf: *mut ::core::ffi::c_char = HRealloc(
            *buf as *mut ::core::ffi::c_void,
            newLen,
        ) as *mut ::core::ffi::c_char;
        if newBuf.is_null() {
            log_add(
                log_Level::log_Error,
                b"Error: luaUqm_addToBuffer(): could not allocate memory.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        *bufLen = newLen;
        *buf = newBuf;
        *bufPtr = newBuf.offset(bufFill as isize);
    }
    memcpy(
        *bufPtr as *mut ::core::ffi::c_void,
        add as *const ::core::ffi::c_void,
        addLen,
    );
    *bufPtr = (*bufPtr).offset(addLen as isize);
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_stringInterpolate(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut strPtr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut interI: size_t = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufLen: size_t = 0;
    let mut part: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut partLen: size_t = 0;
    (!luaUqm_commState.is_null()
        || {
            __assert_fail(
                b"luaUqm_commState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luacomm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                175 as ::core::ffi::c_int,
                b"luaUqm_comm_stringInterpolate\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    bufLen = 2048 as size_t;
    buf = HMalloc(bufLen) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    strPtr = str;
    bufPtr = buf;
    interI = 0 as size_t;
    '_out: {
        loop {
            let mut startTag: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut endTag: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut luaStart: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            startTag = strstr(strPtr, b"<%\0".as_ptr() as *const ::core::ffi::c_char);
            if startTag.is_null() {
                break;
            }
            luaStart = startTag.offset(2 as ::core::ffi::c_int as isize);
            luaUqm_comm_addToBuffer(
                &raw mut buf,
                &raw mut bufLen,
                &raw mut bufPtr,
                strPtr,
                startTag.offset_from(strPtr) as size_t,
            );
            endTag = strstr(
                startTag.offset(2 as ::core::ffi::c_int as isize),
                b"%>\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if endTag.is_null() {
                log_add(
                    log_Level::log_Error,
                    b"luaUqm_stringInterpolate(): Unterminated '<%% .. %%>' sequence in string '%s'.\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    str,
                );
                break '_out;
            } else {
                strPtr = endTag.offset(2 as ::core::ffi::c_int as isize);
                interI = interI.wrapping_add(1);
                let mut exprLen: size_t = endTag.offset_from(luaStart) as size_t;
                let mut exprBuf: *mut ::core::ffi::c_char = HMalloc(
                    (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t)
                        .wrapping_add(exprLen),
                ) as *mut ::core::ffi::c_char;
                let mut exprBufPtr: *mut ::core::ffi::c_char = exprBuf;
                strcpy(exprBuf, b"return \0".as_ptr() as *const ::core::ffi::c_char);
                exprBufPtr = exprBufPtr
                    .offset(
                        ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                            .wrapping_sub(1usize) as isize,
                    );
                memcpy(
                    exprBufPtr as *mut ::core::ffi::c_void,
                    luaStart as *const ::core::ffi::c_void,
                    exprLen,
                );
                exprBufPtr = exprBufPtr.offset(exprLen as isize);
                *exprBufPtr = '\0' as ::core::ffi::c_char;
                if uqm_lua_checked!(luaL_loadstring(luaUqm_commState, exprBuf), ::core::ptr::null_mut()) != 0 as ::core::ffi::c_int
                {
                    log_add(
                        log_Level::log_Error,
                        b"luaUqm_stringInterpolate(): lua_loadstring() failed: %s\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        uqm_lua_checked!(lua_tolstring(
                            luaUqm_commState,
                            -1 as ::core::ffi::c_int,
                            ::core::ptr::null_mut::<size_t>(),
                        ), ::core::ptr::null_mut()),
                    );
                    uqm_lua_checked!(lua_settop(
                        luaUqm_commState,
                        -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                    ), ::core::ptr::null_mut());
                } else if uqm_lua_checked!(lua_pcallk(
                    luaUqm_commState,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    None,
                ), ::core::ptr::null_mut()) != 0 as ::core::ffi::c_int
                {
                    log_add(
                        log_Level::log_Error,
                        b"[script] luaUqm_stringInterpolate(): A script error occurred in interpolation %d in string '%s': %s.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        interI,
                        str,
                        uqm_lua_checked!(lua_tolstring(
                            luaUqm_commState,
                            -1 as ::core::ffi::c_int,
                            ::core::ptr::null_mut::<size_t>(),
                        ), ::core::ptr::null_mut()),
                    );
                    uqm_lua_checked!(lua_settop(
                        luaUqm_commState,
                        -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                    ), ::core::ptr::null_mut());
                } else {
                    part = uqm_lua_checked!(lua_tolstring(
                        luaUqm_commState,
                        -1 as ::core::ffi::c_int,
                        &raw mut partLen,
                    ), ::core::ptr::null_mut());
                    if part.is_null() {
                        log_add(
                            log_Level::log_Error,
                            b"[script] luaUqm_stringInterpolate(): Value returned by interpolation %d has type %s, which can not be converted to a string, in string '%s'.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            interI,
                            uqm_lua_checked!(lua_typename(
                                luaUqm_commState,
                                uqm_lua_checked!(lua_type(luaUqm_commState, -1 as ::core::ffi::c_int), ::core::ptr::null_mut()),
                            ), ::core::ptr::null_mut()),
                            str,
                        );
                        uqm_lua_checked!(lua_settop(
                            luaUqm_commState,
                            -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                        ), ::core::ptr::null_mut());
                    } else {
                        luaUqm_comm_addToBuffer(
                            &raw mut buf,
                            &raw mut bufLen,
                            &raw mut bufPtr,
                            part,
                            partLen,
                        );
                        uqm_lua_checked!(lua_settop(
                            luaUqm_commState,
                            -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                        ), ::core::ptr::null_mut());
                    }
                }
                interI = interI.wrapping_add(1);
            }
        }
        luaUqm_comm_addToBuffer(
            &raw mut buf,
            &raw mut bufLen,
            &raw mut bufPtr,
            strPtr,
            strlen(strPtr),
        );
    }
    *bufPtr = '\0' as ::core::ffi::c_char;
    let mut newBuf: *mut ::core::ffi::c_char = HRealloc(
        buf as *mut ::core::ffi::c_void,
        (bufPtr.offset_from(buf) + 1isize) as size_t,
    ) as *mut ::core::ffi::c_char;
    if !newBuf.is_null() {
        buf = newBuf;
    }
    return buf;
}
