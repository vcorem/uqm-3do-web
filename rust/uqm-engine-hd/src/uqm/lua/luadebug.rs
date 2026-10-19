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

//#include <stdlib.h>
//#include <lauxlib.h>

// Rust translation of src/uqm/lua/luadebug.c.
extern "C" {
    pub type lua_State;
    pub type _IO_FILE;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
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
    static stdin: *mut FILE;
    static stdout: *mut FILE;
    static stderr: *mut FILE;
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn clearerr(_: *mut FILE);
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_luaL_loadstring"]
    fn luaL_loadstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn luaUqm_prepareEnvironment(luaState: *mut lua_State);
    fn luaUqm_loadLibs(luaState: *mut lua_State, libs: *const luaL_Reg);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn luaUqm_comm_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_event_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_log_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_state_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    static mut luaUqm_globalState: *mut lua_State;
}
pub type __builtin_va_list = *mut ::core::ffi::c_void;
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const ::core::ffi::c_char,
    pub func: lua_CFunction,
}
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_DebugContext {
    pub r#in: *mut FILE,
    pub out: *mut FILE,
    pub err: *mut FILE,
    pub debugState: *mut lua_State,
}
pub type va_list = __builtin_va_list;
pub type __isoc_va_list = __builtin_va_list;
#[no_mangle]
pub static mut luaUqm_debugState: *mut lua_State = ::core::ptr::null_mut::<lua_State>();
static mut debugLibs: [luaL_Reg; 5] = [
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
pub unsafe extern "C" fn luaUqm_debug_init() {
    (luaUqm_debugState.is_null()
        || {
            __assert_fail(
                b"luaUqm_debugState == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luadebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_int,
                b"luaUqm_debug_init\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    luaUqm_debugState = luaUqm_globalState;
    uqm_lua_checked!(luaUqm_prepareEnvironment(luaUqm_debugState), ());
    uqm_lua_checked!(luaUqm_loadLibs(luaUqm_debugState, &raw const debugLibs as *const luaL_Reg), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_debug_uninit() {
    (!luaUqm_debugState.is_null()
        || {
            __assert_fail(
                b"luaUqm_debugState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luadebug.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                81 as ::core::ffi::c_int,
                b"luaUqm_debug_uninit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    luaUqm_debugState = ::core::ptr::null_mut::<lua_State>();
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_debug_run() {
    uqm_lua_checked!(luaUqm_debug_init(), ());
    uqm_lua_checked!(luaUqm_debug_interactive(stdin, stdout, stderr), ());
    luaUqm_debug_uninit();
}
unsafe extern "C" fn luaUqm_debug_interactive(
    mut r#in: *mut FILE,
    mut out: *mut FILE,
    mut err: *mut FILE,
) {
    let mut lineBuf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut lineLen: size_t = 0;
    let mut debugContext: luaUqm_DebugContext = luaUqm_DebugContext {
        r#in: ::core::ptr::null_mut::<FILE>(),
        out: ::core::ptr::null_mut::<FILE>(),
        err: ::core::ptr::null_mut::<FILE>(),
        debugState: ::core::ptr::null_mut::<lua_State>(),
    };
    memset(
        &raw mut debugContext as *mut ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<luaUqm_DebugContext>() as size_t,
    );
    debugContext.r#in = r#in;
    debugContext.out = out;
    debugContext.err = err;
    debugContext.debugState = luaUqm_debugState;
    loop {
        fprintf(out, b"> \0".as_ptr() as *const ::core::ffi::c_char);
        if fgets(
                &raw mut lineBuf as *mut ::core::ffi::c_char,
                2048 as ::core::ffi::c_int,
                r#in,
            )
            .is_null()
        {
            if feof(r#in) != 0 {
                break;
            }
            clearerr(r#in);
        } else {
            lineLen = strlen(&raw mut lineBuf as *mut ::core::ffi::c_char) as size_t;
            if lineBuf[lineLen.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                != '\n' as ::core::ffi::c_int
                && lineBuf[lineLen.wrapping_sub(1 as size_t) as usize]
                    as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
            {
                fprintf(
                    err,
                    b"Error: Too long command line.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                uqm_lua_checked!(luaUqm_debug_runLine(
                    &raw mut lineBuf as *mut ::core::ffi::c_char,
                    Some(
                        luaUqm_debug_outputCallback
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                                ...
                            ) -> (),
                    ),
                    Some(
                        luaUqm_debug_errorCallback
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                                ...
                            ) -> (),
                    ),
                    &raw mut debugContext as *mut ::core::ffi::c_void,
                ), ());
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_debug_runLine(
    mut exprBuf: *const ::core::ffi::c_char,
    mut outputCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ...
        ) -> (),
    >,
    mut errorCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ...
        ) -> (),
    >,
    mut extra: *mut ::core::ffi::c_void,
) {
    let mut resultType: ::core::ffi::c_int = 0;
    let mut resultStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut resultTypeStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if uqm_lua_checked!(luaL_loadstring(luaUqm_debugState, exprBuf), ()) != 0 as ::core::ffi::c_int {
        uqm_lua_checked!(errorCallback
            .expect(
                "non-null function pointer",
            )(
            extra,
            b"Syntax error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaUqm_debugState,
                -1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), ()),
        ), ());
        uqm_lua_checked!(lua_settop(
            luaUqm_debugState,
            -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        ), ());
        return;
    }
    if uqm_lua_checked!(lua_pcallk(
        luaUqm_debugState,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        None,
    ), ()) != 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(errorCallback
            .expect(
                "non-null function pointer",
            )(
            extra,
            b"Runtime error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaUqm_debugState,
                -1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), ()),
        ), ());
        uqm_lua_checked!(lua_settop(
            luaUqm_debugState,
            -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        ), ());
        return;
    }
    resultType = uqm_lua_checked!(lua_type(luaUqm_debugState, -1 as ::core::ffi::c_int), ());
    resultTypeStr = uqm_lua_checked!(lua_typename(luaUqm_debugState, resultType), ());
    resultStr = uqm_lua_checked!(lua_tolstring(
        luaUqm_debugState,
        -1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), ());
    if resultStr.is_null() {
        uqm_lua_checked!(outputCallback
            .expect(
                "non-null function pointer",
            )(extra, b"(%s)\n\0".as_ptr() as *const ::core::ffi::c_char, resultTypeStr), ());
    } else {
        uqm_lua_checked!(outputCallback
            .expect(
                "non-null function pointer",
            )(
            extra,
            b"(%s) %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            resultTypeStr,
            resultStr,
        ), ());
    }
    uqm_lua_checked!(lua_settop(luaUqm_debugState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
}
unsafe extern "C" fn luaUqm_debug_outputCallback(
    mut extra: *mut ::core::ffi::c_void,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut args: ::core::ffi::VaListImpl;
    let mut debugContext: *mut luaUqm_DebugContext = extra as *mut luaUqm_DebugContext;
    args = c2rust_args.clone();
    vfprintf((*debugContext).out, format, args.as_va_list());
}
unsafe extern "C" fn luaUqm_debug_errorCallback(
    mut extra: *mut ::core::ffi::c_void,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut args: ::core::ffi::VaListImpl;
    let mut debugContext: *mut luaUqm_DebugContext = extra as *mut luaUqm_DebugContext;
    args = c2rust_args.clone();
    vfprintf((*debugContext).err, format, args.as_va_list());
}
