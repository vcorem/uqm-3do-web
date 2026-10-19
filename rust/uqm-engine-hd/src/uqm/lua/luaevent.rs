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
 * This file contains code for using Lua for game event scripts.
 */

// Rust translation of src/uqm/lua/luaevent.c.
extern "C" {
    pub type uio_DirHandle;
    pub type lua_State;
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
    fn luaUqm_prepareEnvironment(luaState: *mut lua_State);
    fn luaUqm_loadScript(
        luaState: *mut lua_State,
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn luaUqm_loadLibs(luaState: *mut lua_State, libs: *const luaL_Reg);
    fn luaUqm_callStackFunction(luaState: *mut lua_State) -> BOOLEAN;
    fn ReleaseScriptResData(data: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn LoadScriptInstance(res: RESOURCE) -> *mut ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_settop"]
    fn lua_settop(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_replace"]
    fn lua_replace(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_type"]
    fn lua_type(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_pushstring"]
    fn lua_pushstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_gettable"]
    fn lua_gettable(L: *mut lua_State, idx: ::core::ffi::c_int);
    static mut contentDir: *mut uio_DirHandle;
    static mut luaUqm_globalState: *mut lua_State;
    fn luaUqm_getEventTable(luaState: *mut lua_State);
    fn luaUqm_event_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_log_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_state_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
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
pub static mut luaUqm_eventState: *mut lua_State = ::core::ptr::null_mut::<lua_State>();
static mut eventLibs: [luaL_Reg; 4] = [
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
pub unsafe extern "C" fn luaUqm_event_init(
    mut customFuncs: *const luaUqm_custom_Function,
    mut scriptRes: RESOURCE,
) -> BOOLEAN {
    let mut scriptFileName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut loadOk: BOOLEAN = BOOLEAN::FALSE;
    (luaUqm_eventState.is_null()
        || {
            __assert_fail(
                b"luaUqm_eventState == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luaevent.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_int,
                b"luaUqm_event_init\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    luaUqm_eventState = luaUqm_globalState;
    uqm_lua_checked!(luaUqm_prepareEnvironment(luaUqm_eventState), BOOLEAN(0));
    uqm_lua_checked!(luaUqm_loadLibs(luaUqm_eventState, &raw const eventLibs as *const luaL_Reg), BOOLEAN(0));
    uqm_lua_checked!(luaUqm_custom_init(luaUqm_eventState, customFuncs), BOOLEAN(0));
    uqm_lua_checked!(lua_settop(luaUqm_eventState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), BOOLEAN(0));
    scriptFileName = uqm_lua_checked!(LoadScriptInstance(scriptRes), BOOLEAN(0));
    if scriptFileName.is_null() {
        return BOOLEAN::FALSE;
    }
    loadOk = uqm_lua_checked!(luaUqm_loadScript(luaUqm_eventState, contentDir, scriptFileName), BOOLEAN(0));
    ReleaseScriptResData(scriptFileName as *mut ::core::ffi::c_void);
    if loadOk.0 == 0 {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(luaUqm_callStackFunction(luaUqm_eventState), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_event_uninit() {
    (!luaUqm_eventState.is_null()
        || {
            __assert_fail(
                b"luaUqm_eventState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luaevent.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_int,
                b"luaUqm_event_uninit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    luaUqm_eventState = ::core::ptr::null_mut::<lua_State>();
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_event_callEvent(
    mut eventIdStr: *const ::core::ffi::c_char,
) {
    (!luaUqm_eventState.is_null()
        || {
            __assert_fail(
                b"luaUqm_eventState != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/lua/luaevent.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_int,
                b"luaUqm_event_callEvent\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    uqm_lua_checked!(luaUqm_getEventTable(luaUqm_eventState), ());
    uqm_lua_checked!(lua_pushstring(luaUqm_eventState, eventIdStr), ());
    uqm_lua_checked!(lua_gettable(luaUqm_eventState, -2 as ::core::ffi::c_int), ());
    if uqm_lua_checked!(lua_type(luaUqm_eventState, -1 as ::core::ffi::c_int), ()) == 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"[script] Warning: luaUqm_event_callEvent(): Event '%s' is not registered.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            eventIdStr,
        );
        uqm_lua_checked!(lua_settop(
            luaUqm_eventState,
            -2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        ), ());
        return;
    }
    uqm_lua_checked!(lua_replace(luaUqm_eventState, -2 as ::core::ffi::c_int), ());
    uqm_lua_checked!(luaUqm_callStackFunction(luaUqm_eventState), ());
}
