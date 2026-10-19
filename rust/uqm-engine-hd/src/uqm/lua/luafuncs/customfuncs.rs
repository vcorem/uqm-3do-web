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

// Rust translation of src/uqm/lua/luafuncs/customfuncs.c.
extern "C" {
    pub type lua_State;
    #[link_name = "uqm_lua_guard_lua_gettop"]
    fn lua_gettop(L: *mut lua_State) -> ::core::ffi::c_int;
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
    #[link_name = "uqm_lua_guard_lua_topointer"]
    fn lua_topointer(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    #[link_name = "uqm_lua_guard_lua_pushinteger"]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    #[link_name = "uqm_lua_guard_lua_pushcclosure"]
    fn lua_pushcclosure(L: *mut lua_State, r#fn: lua_CFunction, n: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_pushlightuserdata"]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut ::core::ffi::c_void);
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
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
}
pub type ptrdiff_t = isize;
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
pub type lua_Integer = ptrdiff_t;
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
unsafe extern "C" fn luaFunctionWrapper(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut arg: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 0;
    let mut isNum: ::core::ffi::c_int = 0;
    let mut fun: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
    > = ::core::mem::transmute::<
        *const ::core::ffi::c_void,
        Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    >(
        uqm_lua_checked!(lua_topointer(
            luaState,
            -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
        ), 0),
    );
    if uqm_lua_checked!(lua_gettop(luaState), 0) == 0 as ::core::ffi::c_int {
        arg = 0 as ::core::ffi::c_int;
    } else {
        arg = uqm_lua_checked!(lua_tointegerx(luaState, -1 as ::core::ffi::c_int, &raw mut isNum), 0)
            as ::core::ffi::c_int;
        if isNum == 0 {
            log_add(
                log_Level::log_Error,
                b"[script] Warning: luaFunctionWrapper(): Invalid type of argument to custom function (%s).\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                uqm_lua_checked!(lua_typename(luaState, uqm_lua_checked!(lua_type(luaState, -1 as ::core::ffi::c_int), 0)), 0),
            );
        }
    }
    result = uqm_lua_checked!(Some(fun.expect("non-null function pointer"))
        .expect("non-null function pointer")(arg), 0);
    uqm_lua_checked!(lua_pushinteger(luaState, result as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_custom_addFunction(
    mut luaState: *mut lua_State,
    mut fun: *const luaUqm_custom_Function,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushlightuserdata(
        luaState,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
            *mut ::core::ffi::c_void,
        >((*fun).fun),
    ), 0);
    uqm_lua_checked!(lua_pushcclosure(
        luaState,
        Some(
            luaFunctionWrapper
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
        1 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(luaState, -2 as ::core::ffi::c_int, (*fun).name), 0);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaUqm_custom_init(
    mut luaState: *mut lua_State,
    mut funs: *const luaUqm_custom_Function,
) -> ::core::ffi::c_int {
    let mut funCount: size_t = 0 as size_t;
    let mut ptr: *const luaUqm_custom_Function = ::core::ptr::null::<
        luaUqm_custom_Function,
    >();
    ptr = funs;
    while !(*ptr).name.is_null() {
        funCount = funCount.wrapping_add(1);
        ptr = ptr.offset(1);
    }
    uqm_lua_checked!(lua_rawgeti(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, funCount as ::core::ffi::c_int), 0);
    ptr = funs;
    while !(*ptr).name.is_null() {
        uqm_lua_checked!(luaUqm_custom_addFunction(luaState, ptr), 0);
        ptr = ptr.offset(1);
    }
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"custom\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
