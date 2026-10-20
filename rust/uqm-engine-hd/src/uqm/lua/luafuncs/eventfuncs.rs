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

// Rust translation of src/uqm/lua/luafuncs/eventfuncs.c.
extern "C" {
    pub type lua_State;
    #[link_name = "uqm_lua_guard_lua_absindex"]
    fn lua_absindex(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_settop"]
    fn lua_settop(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_pushvalue"]
    fn lua_pushvalue(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_type"]
    fn lua_type(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_tolstring"]
    fn lua_tolstring(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        len: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pushnil"]
    fn lua_pushnil(L: *mut lua_State);
    #[link_name = "uqm_lua_guard_lua_pushinteger"]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    #[link_name = "uqm_lua_guard_lua_gettable"]
    fn lua_gettable(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_createtable"]
    fn lua_createtable(
        L: *mut lua_State,
        narr: ::core::ffi::c_int,
        nrec: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_lua_settable"]
    fn lua_settable(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_luaL_checklstring"]
    fn luaL_checklstring(
        L: *mut lua_State,
        numArg: ::core::ffi::c_int,
        l: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_luaL_checkinteger"]
    fn luaL_checkinteger(L: *mut lua_State, numArg: ::core::ffi::c_int) -> lua_Integer;
    #[link_name = "uqm_lua_guard_luaL_checktype"]
    fn luaL_checktype(
        L: *mut lua_State,
        narg: ::core::ffi::c_int,
        t: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_luaL_setfuncs"]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: ::core::ffi::c_int);
    fn eventIdStrToNum(eventIdStr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn luaUqm_getEventTable(luaState: *mut lua_State);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn AddEvent(
        r#type: EVENT_TYPE,
        month_index: COUNT,
        day_index: COUNT,
        year_index: COUNT,
        func_index: BYTE,
    ) -> HEVENT;
}
pub type size_t = ::core::ffi::c_ulong;
pub type ptrdiff_t = isize;
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
pub type lua_Integer = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const ::core::ffi::c_char,
    pub func: lua_CFunction,
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
}
pub type HEVENT = HLINK;
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
static mut eventFuncs: [luaL_Reg; 5] = [
    luaL_Reg {
        name: b"addAbsolute\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_event_addAbsolute
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"addRelative\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_event_addRelative
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"register\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_event_register
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"unregister\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_event_unregister
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
#[no_mangle]
pub unsafe extern "C" fn luaUqm_event_open(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 5]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const eventFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn isEventRegistered(
    mut luaState: *mut lua_State,
    mut argn: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    argn = uqm_lua_checked!(lua_absindex(luaState, argn), BOOLEAN(0));
    uqm_lua_checked!(luaUqm_getEventTable(luaState), BOOLEAN(0));
    uqm_lua_checked!(lua_pushvalue(luaState, argn), BOOLEAN(0));
    uqm_lua_checked!(lua_gettable(luaState, -2 as ::core::ffi::c_int), BOOLEAN(0));
    result = BOOLEAN(
        !(uqm_lua_checked!(lua_type(luaState, -1 as ::core::ffi::c_int), BOOLEAN(0)) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(lua_settop(luaState, -2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), BOOLEAN(0));
    return result;
}
unsafe extern "C" fn addEvent(
    mut luaState: *mut lua_State,
    mut r#type: EVENT_TYPE,
) -> ::core::ffi::c_int {
    let mut year: ::core::ffi::c_int = uqm_lua_checked!(luaL_checkinteger(
        luaState,
        1 as ::core::ffi::c_int,
    ), 0) as ::core::ffi::c_int;
    let mut month: ::core::ffi::c_int = uqm_lua_checked!(luaL_checkinteger(
        luaState,
        2 as ::core::ffi::c_int,
    ), 0) as ::core::ffi::c_int;
    let mut day: ::core::ffi::c_int = uqm_lua_checked!(luaL_checkinteger(
        luaState,
        3 as ::core::ffi::c_int,
    ), 0) as ::core::ffi::c_int;
    let mut eventIdStr: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        4 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut eventNum: ::core::ffi::c_int = 0;
    let mut event: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if uqm_lua_checked!(isEventRegistered(luaState, 4 as ::core::ffi::c_int), 0).0 == 0 {
        log_add(
            log_Level::log_Warning,
            b"[script] event.%s(): Event '%s' is not registered.\0".as_ptr()
                as *const ::core::ffi::c_char,
            if r#type.0 == EVENT_TYPE::RELATIVE_EVENT.0 {
                b"addRelative\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"addAbsolute\0".as_ptr() as *const ::core::ffi::c_char
            },
            uqm_lua_checked!(lua_tolstring(
                luaState,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), 0),
        );
        uqm_lua_checked!(lua_pushinteger(luaState, -1 as lua_Integer), 0);
        return 1 as ::core::ffi::c_int;
    }
    eventNum = eventIdStrToNum(eventIdStr);
    if eventNum == -1 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"[script] event.%s(): Event '%s' is not known. It must currently be one of the hard-coded strings.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            if r#type.0 == EVENT_TYPE::RELATIVE_EVENT.0 {
                b"addRelative\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"addAbsolute\0".as_ptr() as *const ::core::ffi::c_char
            },
            uqm_lua_checked!(lua_tolstring(
                luaState,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), 0),
        );
        uqm_lua_checked!(lua_pushinteger(luaState, -1 as lua_Integer), 0);
        return 1 as ::core::ffi::c_int;
    }
    event = uqm_lua_checked!(AddEvent(
        r#type,
        month as COUNT,
        day as COUNT,
        year as COUNT,
        eventNum as BYTE,
    ), 0);
    uqm_lua_checked!(lua_pushinteger(
        luaState,
        (if event.is_null() {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as lua_Integer,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_event_addAbsolute(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(addEvent(luaState, EVENT_TYPE::ABSOLUTE_EVENT), 0);
}
unsafe extern "C" fn luaUqm_event_addRelative(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(addEvent(luaState, EVENT_TYPE::RELATIVE_EVENT), 0);
}
unsafe extern "C" fn luaUqm_event_register(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    uqm_lua_checked!(luaL_checktype(luaState, 2 as ::core::ffi::c_int, 6 as ::core::ffi::c_int), 0);
    if uqm_lua_checked!(isEventRegistered(luaState, 1 as ::core::ffi::c_int), 0).0 != 0 {
        log_add(
            log_Level::log_Warning,
            b"[script] event.register(): Event '%s' is already registered.\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaState,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), 0),
        );
        return 0 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(luaUqm_getEventTable(luaState), 0);
    uqm_lua_checked!(lua_pushvalue(luaState, 1 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_pushvalue(luaState, 2 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_settable(luaState, -3 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_event_unregister(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if uqm_lua_checked!(isEventRegistered(luaState, 1 as ::core::ffi::c_int), 0).0 == 0 {
        log_add(
            log_Level::log_Warning,
            b"[script] event.unregister(): Event '%s' was not registered.\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaState,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), 0),
        );
        return 0 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(luaUqm_getEventTable(luaState), 0);
    uqm_lua_checked!(lua_pushvalue(luaState, 1 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_pushnil(luaState), 0);
    uqm_lua_checked!(lua_settable(luaState, -3 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), 0);
    return 0 as ::core::ffi::c_int;
}
