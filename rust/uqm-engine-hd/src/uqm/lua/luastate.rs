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
 * This file contains code for connecting storing the game state in Lua.
 */

// Rust translation of src/uqm/lua/luastate.c.
extern "C" {
    pub type lua_State;
    #[link_name = "uqm_lua_guard_lua_close"]
    fn lua_close(L: *mut lua_State);
    #[link_name = "uqm_lua_guard_lua_absindex"]
    fn lua_absindex(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_settop"]
    fn lua_settop(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_pushvalue"]
    fn lua_pushvalue(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_replace"]
    fn lua_replace(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_type"]
    fn lua_type(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_typename"]
    fn lua_typename(
        L: *mut lua_State,
        tp: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_tointegerx"]
    fn lua_tointegerx(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        isnum: *mut ::core::ffi::c_int,
    ) -> lua_Integer;
    #[link_name = "uqm_lua_guard_lua_pushinteger"]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    #[link_name = "uqm_lua_guard_lua_pushstring"]
    fn lua_pushstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_gettable"]
    fn lua_gettable(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_getfield"]
    fn lua_getfield(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        k: *const ::core::ffi::c_char,
    );
    #[link_name = "uqm_lua_guard_lua_createtable"]
    fn lua_createtable(
        L: *mut lua_State,
        narr: ::core::ffi::c_int,
        nrec: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_lua_settable"]
    fn lua_settable(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_luaL_error"]
    fn luaL_error(
        L: *mut lua_State,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_luaL_newstate"]
    fn luaL_newstate() -> *mut lua_State;
    fn luaUqm_runInitScripts();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type ptrdiff_t = isize;
pub type lua_Integer = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta_types {
    pub _int: ::core::ffi::c_int,
    pub _bool: BOOLEAN,
    pub _str: *mut UNICODE,
    pub _char: ::core::ffi::c_char,
}
pub type DELTA_TYPES = delta_types;
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
pub static mut luaUqm_globalState: *mut lua_State = ::core::ptr::null_mut::<lua_State>();
static mut statePropRegistryKey: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<
        [u8; 27],
        [::core::ffi::c_char; 27],
    >(*b"uqm_state_prop_registryKey\0")
};
static mut eventRegistryKey: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<
        [u8; 22],
        [::core::ffi::c_char; 22],
    >(*b"uqm_event_registryKey\0")
};
#[no_mangle]
pub static mut luaUqm_delta: DELTA_TYPES = DELTA_TYPES {
    _int: 0,
    _bool: BOOLEAN::FALSE,
    _str: ::core::ptr::null_mut::<UNICODE>(),
    _char: 0,
};
#[no_mangle]
pub unsafe extern "C" fn luaUqm_initState() {
    if !luaUqm_globalState.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Lua state multiply uninitialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(luaUqm_uninitState(), ());
    }
    luaUqm_globalState = uqm_lua_checked!(luaL_newstate(), ());
    uqm_lua_checked!(luaUqm_initStatePropertyTable(luaUqm_globalState), ());
    uqm_lua_checked!(luaUqm_initEventTable(luaUqm_globalState), ());
    uqm_lua_checked!(luaUqm_runInitScripts(), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_uninitState() {
    if !luaUqm_globalState.is_null() {
        uqm_lua_checked!(lua_close(luaUqm_globalState), ());
        luaUqm_globalState = ::core::ptr::null_mut::<lua_State>();
    } else {
        log_add(
            log_Level::log_Warning,
            b"Lua state multiply uninitialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_reinitState() {
    if luaUqm_globalState.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Lua state reinitialized while NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        uqm_lua_checked!(luaUqm_uninitState(), ());
    }
    uqm_lua_checked!(luaUqm_initState(), ());
}
unsafe extern "C" fn luaUqm_initStatePropertyTable(mut luaState: *mut lua_State) {
    uqm_lua_checked!(lua_pushstring(
        luaState,
        &raw const statePropRegistryKey as *const ::core::ffi::c_char,
    ), ());
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_settable(luaState, -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_checkPropValueType(
    mut luaState: *mut lua_State,
    mut funName: *const ::core::ffi::c_char,
    mut nameIndex: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r#type: ::core::ffi::c_int = uqm_lua_checked!(lua_type(luaState, nameIndex), 0);
    match r#type {
        3 | 1 | 4 => return 0 as ::core::ffi::c_int,
        _ => {
            let mut typeName: *const ::core::ffi::c_char = uqm_lua_checked!(lua_typename(
                luaState,
                nameIndex,
            ), 0);
            return uqm_lua_checked!(luaL_error(
                luaState,
                b"Property value has an invalid type, in parameter to %s() (%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                funName,
                typeName,
            ), 0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_setProp(
    mut luaState: *mut lua_State,
    mut nameIndex: ::core::ffi::c_int,
    mut valueIndex: ::core::ffi::c_int,
) {
    nameIndex = uqm_lua_checked!(lua_absindex(luaState, nameIndex), ());
    valueIndex = uqm_lua_checked!(lua_absindex(luaState, valueIndex), ());
    uqm_lua_checked!(lua_getfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const statePropRegistryKey as *const ::core::ffi::c_char,
    ), ());
    uqm_lua_checked!(lua_pushvalue(luaState, nameIndex), ());
    uqm_lua_checked!(lua_pushvalue(luaState, valueIndex), ());
    uqm_lua_checked!(lua_settable(luaState, -3 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_getProp(
    mut luaState: *mut lua_State,
    mut nameIndex: ::core::ffi::c_int,
) {
    nameIndex = uqm_lua_checked!(lua_absindex(luaState, nameIndex), ());
    uqm_lua_checked!(lua_getfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const statePropRegistryKey as *const ::core::ffi::c_char,
    ), ());
    uqm_lua_checked!(lua_pushvalue(luaState, nameIndex), ());
    uqm_lua_checked!(lua_gettable(luaState, -2 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_replace(luaState, -2 as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn setGameStateUint(
    mut name: *const ::core::ffi::c_char,
    mut val: DWORD,
) {
    uqm_lua_checked!(lua_pushstring(luaUqm_globalState, name), ());
    uqm_lua_checked!(lua_pushinteger(luaUqm_globalState, val as lua_Integer), ());
    uqm_lua_checked!(luaUqm_setProp(
        luaUqm_globalState,
        -2 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
    ), ());
    uqm_lua_checked!(lua_settop(luaUqm_globalState, -2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn getGameStateUint(
    mut name: *const ::core::ffi::c_char,
) -> DWORD {
    let mut result: DWORD = 0;
    let mut resultType: ::core::ffi::c_int = 0;
    uqm_lua_checked!(lua_pushstring(luaUqm_globalState, name), 0);
    uqm_lua_checked!(luaUqm_getProp(luaUqm_globalState, -1 as ::core::ffi::c_int), 0);
    resultType = uqm_lua_checked!(lua_type(luaUqm_globalState, -1 as ::core::ffi::c_int), 0);
    match resultType {
        0 => {
            uqm_lua_checked!(lua_settop(
                luaUqm_globalState,
                -2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            ), 0);
            return 0 as DWORD;
        }
        3 => {}
        _ => {
            log_add(
                log_Level::log_Error,
                b"Warning: getGameState(): property '%s' has a non-number value (%s).\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                name,
                uqm_lua_checked!(lua_typename(luaUqm_globalState, -1 as ::core::ffi::c_int), 0),
            );
            uqm_lua_checked!(lua_settop(
                luaUqm_globalState,
                -2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            ), 0);
            return 0 as DWORD;
        }
    }
    result = uqm_lua_checked!(lua_tointegerx(
        luaUqm_globalState,
        -1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ), 0) as DWORD;
    uqm_lua_checked!(lua_settop(luaUqm_globalState, -2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), 0);
    return result;
}
unsafe extern "C" fn luaUqm_initEventTable(mut luaState: *mut lua_State) {
    uqm_lua_checked!(lua_pushstring(luaState, &raw const eventRegistryKey as *const ::core::ffi::c_char), ());
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_settable(luaState, -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int), ());
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_getEventTable(mut luaState: *mut lua_State) {
    uqm_lua_checked!(lua_getfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const eventRegistryKey as *const ::core::ffi::c_char,
    ), ());
}
