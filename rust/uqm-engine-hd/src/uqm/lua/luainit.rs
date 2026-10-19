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
 * This file contains code for running scripts when a new game is started
 * or an old game is loaded.
 */

// Rust translation of src/uqm/lua/luainit.c.
extern "C" {
    pub type uio_DirHandle;
    pub type lua_State;
    fn res_GetString(key: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn luaUqm_prepareEnvironment(luaState: *mut lua_State);
    fn luaUqm_runLuaDir(
        luaState: *mut lua_State,
        dirHandle: *mut uio_DirHandle,
        luaDirName: *const ::core::ffi::c_char,
    );
    fn luaUqm_loadLibs(luaState: *mut lua_State, libs: *const luaL_Reg);
    static mut contentDir: *mut uio_DirHandle;
    static mut luaUqm_globalState: *mut lua_State;
    fn luaUqm_event_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_log_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn luaUqm_state_open(luaState: *mut lua_State) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
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
static mut initLibs: [luaL_Reg; 4] = [
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
pub unsafe extern "C" fn luaUqm_runInitScripts() {
    let mut scriptDir: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    uqm_lua_checked!(luaUqm_prepareEnvironment(luaUqm_globalState), ());
    uqm_lua_checked!(luaUqm_loadLibs(luaUqm_globalState, &raw const initLibs as *const luaL_Reg), ());
    scriptDir = res_GetString(
        b"script.dir.initgame\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if scriptDir.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Location of game initialisation scripts ('%s') was not specified.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"script.dir.initgame\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        uqm_lua_checked!(luaUqm_runLuaDir(luaUqm_globalState, contentDir, scriptDir), ());
    };
}
