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

// Rust translation of src/uqm/lua/luafuncs/logfuncs.c.
extern "C" {
    pub type lua_State;
    #[link_name = "uqm_lua_guard_lua_tolstring"]
    fn lua_tolstring(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        len: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_createtable"]
    fn lua_createtable(
        L: *mut lua_State,
        narr: ::core::ffi::c_int,
        nrec: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_luaL_setfuncs"]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: ::core::ffi::c_int);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
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
static mut logFuncs: [luaL_Reg; 6] = [
    luaL_Reg {
        name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_log_debug
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"error\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_log_error
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"fatal\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_log_fatal
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"info\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_log_info as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"warn\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_log_warn as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
#[no_mangle]
pub unsafe extern "C" fn luaUqm_log_open(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 6]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const logFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn logHelper(
    mut luaState: *mut lua_State,
    mut level: log_Level,
) -> ::core::ffi::c_int {
    let mut str: *const ::core::ffi::c_char = uqm_lua_checked!(lua_tolstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    log_add(level, b"Lua: %s\0".as_ptr() as *const ::core::ffi::c_char, str);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_log_debug(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(logHelper(luaState, log_Level::log_Debug), 0);
}
unsafe extern "C" fn luaUqm_log_error(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(logHelper(luaState, log_Level::log_Error), 0);
}
unsafe extern "C" fn luaUqm_log_fatal(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(logHelper(luaState, log_Level::log_Fatal), 0);
}
unsafe extern "C" fn luaUqm_log_info(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(logHelper(luaState, log_Level::log_Info), 0);
}
unsafe extern "C" fn luaUqm_log_warn(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(logHelper(luaState, log_Level::log_Warning), 0);
}
