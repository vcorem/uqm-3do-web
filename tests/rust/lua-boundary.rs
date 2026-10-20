#![no_std]
#![allow(non_snake_case, dead_code, unused_unsafe)]
#[macro_use]
#[path = "../../rust/uqm-lua/guard.rs"]
mod uqm_lua_guard;
use core::ffi::{c_char, c_int, c_void};
use core::ptr::{null, null_mut};
type State = c_void;
type Callback = Option<unsafe extern "C" fn(*mut State) -> c_int>;
type Reader = Option<unsafe extern "C" fn(*mut State, *mut c_void, *mut usize) -> *const c_char>;
#[repr(C)]
struct Reg {
    name: *const c_char,
    function: Callback,
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { abort() }
}
extern "C" {
    fn abort() -> !;
    fn test_rust_enter();
    fn test_rust_leave();
    fn test_operation_completed();
    fn test_arm_oom();
    fn test_disarm_oom();
    fn test_big_string() -> *const c_char;
}
struct ReturnMarker;
impl ReturnMarker {
    unsafe fn new() -> Self {
        test_rust_enter();
        Self
    }
}
impl Drop for ReturnMarker {
    fn drop(&mut self) {
        unsafe {
            test_rust_leave();
        }
    }
}

extern "C" {
    fn uqm_lua_guard_lua_absindex(a0: *mut State, a1: c_int) -> c_int;
    fn uqm_lua_guard_lua_gettop(a0: *mut State) -> c_int;
    fn uqm_lua_guard_lua_settop(a0: *mut State, a1: c_int) -> ();
    fn uqm_lua_guard_lua_pushvalue(a0: *mut State, a1: c_int) -> ();
    fn uqm_lua_guard_lua_replace(a0: *mut State, a1: c_int) -> ();
    fn uqm_lua_guard_lua_type(a0: *mut State, a1: c_int) -> c_int;
    fn uqm_lua_guard_lua_typename(a0: *mut State, a1: c_int) -> *const c_char;
    fn uqm_lua_guard_lua_tonumberx(a0: *mut State, a1: c_int, a2: *mut c_int) -> f64;
    fn uqm_lua_guard_lua_tointegerx(a0: *mut State, a1: c_int, a2: *mut c_int) -> isize;
    fn uqm_lua_guard_lua_toboolean(a0: *mut State, a1: c_int) -> c_int;
    fn uqm_lua_guard_lua_tolstring(a0: *mut State, a1: c_int, a2: *mut usize) -> *const c_char;
    fn uqm_lua_guard_lua_topointer(a0: *mut State, a1: c_int) -> *const c_void;
    fn uqm_lua_guard_lua_pushnil(a0: *mut State) -> ();
    fn uqm_lua_guard_lua_pushinteger(a0: *mut State, a1: isize) -> ();
    fn uqm_lua_guard_lua_pushstring(a0: *mut State, a1: *const c_char) -> *const c_char;
    fn uqm_lua_guard_lua_pushboolean(a0: *mut State, a1: c_int) -> ();
    fn uqm_lua_guard_lua_pushlightuserdata(a0: *mut State, a1: *mut c_void) -> ();
    fn uqm_lua_guard_lua_getglobal(a0: *mut State, a1: *const c_char) -> ();
    fn uqm_lua_guard_lua_gettable(a0: *mut State, a1: c_int) -> ();
    fn uqm_lua_guard_lua_getfield(a0: *mut State, a1: c_int, a2: *const c_char) -> ();
    fn uqm_lua_guard_lua_rawgeti(a0: *mut State, a1: c_int, a2: c_int) -> ();
    fn uqm_lua_guard_lua_createtable(a0: *mut State, a1: c_int, a2: c_int) -> ();
    fn uqm_lua_guard_lua_settable(a0: *mut State, a1: c_int) -> ();
    fn uqm_lua_guard_lua_setfield(a0: *mut State, a1: c_int, a2: *const c_char) -> ();
    fn uqm_lua_guard_lua_rawseti(a0: *mut State, a1: c_int, a2: c_int) -> ();
    fn uqm_lua_guard_luaL_checkinteger(a0: *mut State, a1: c_int) -> isize;
    fn uqm_lua_guard_luaL_checklstring(a0: *mut State, a1: c_int, a2: *mut usize) -> *const c_char;
    fn uqm_lua_guard_luaL_checktype(a0: *mut State, a1: c_int, a2: c_int) -> ();
    fn uqm_lua_guard_luaL_checkoption(
        a0: *mut State,
        a1: c_int,
        a2: *const c_char,
        a3: *const *const c_char,
    ) -> c_int;
    fn uqm_lua_guard_luaL_loadstring(a0: *mut State, a1: *const c_char) -> c_int;
    fn uqm_lua_guard_lua_pushcclosure(a0: *mut State, a1: Callback, a2: c_int) -> ();
    fn uqm_lua_guard_luaL_setfuncs(a0: *mut State, a1: *const Reg, a2: c_int) -> ();
    fn uqm_lua_guard_lua_callk(a0: *mut State, a1: c_int, a2: c_int, a3: c_int, a4: Callback)
        -> ();
    fn uqm_lua_guard_lua_pcallk(
        a0: *mut State,
        a1: c_int,
        a2: c_int,
        a3: c_int,
        a4: c_int,
        a5: Callback,
    ) -> c_int;
    fn uqm_lua_guard_lua_load(
        a0: *mut State,
        a1: Reader,
        a2: *mut c_void,
        a3: *const c_char,
        a4: *const c_char,
    ) -> c_int;
    fn uqm_lua_guard_luaL_error(a0: *mut State, a1: *const c_char, ...) -> c_int;
    fn uqm_lua_guard_lua_close(a0: *mut State) -> ();
}
macro_rules! check {
    ($call:expr) => {
        uqm_lua_checked!($call, 0)
    };
}
unsafe fn integer(L: *mut State, index: c_int) -> isize {
    let _marker = ReturnMarker::new();
    uqm_lua_checked!(uqm_lua_guard_luaL_checkinteger(L, index), 0)
}
unsafe extern "C" fn nested(L: *mut State) -> c_int {
    let _marker = ReturnMarker::new();
    let v = check!(integer(L, 1));
    check!(uqm_lua_guard_lua_pushinteger(L, v + 1));
    test_operation_completed();
    1
}
unsafe extern "C" fn upvalue(L: *mut State) -> c_int {
    let _marker = ReturnMarker::new();
    let n = check!(uqm_lua_guard_luaL_checkinteger(L, 1));
    let add = check!(uqm_lua_guard_lua_tointegerx(L, -1001001, null_mut()));
    check!(uqm_lua_guard_lua_pushinteger(L, n + add));
    1
}
unsafe extern "C" fn reader(L: *mut State, data: *mut c_void, size: *mut usize) -> *const c_char {
    let _marker = ReturnMarker::new();
    let read = &mut *(data as *mut c_int);
    if *read != 0 {
        *size = 0;
        return null();
    }
    *read = 1;
    let v = uqm_lua_checked!(uqm_lua_guard_luaL_checkinteger(L, 1), null());
    if v == 0 {
        *size = 8;
        b"return 7".as_ptr().cast()
    } else {
        *size = 7;
        b"return ".as_ptr().cast()
    }
}
#[no_mangle]
pub unsafe extern "C" fn rust_lua_exercise(L: *mut State) -> c_int {
    let _marker = ReturnMarker::new();
    let mode = check!(uqm_lua_guard_lua_tointegerx(L, -1001001, null_mut()));
    let results = match mode {
        0 => {
            let v = check!(integer(L, 1));
            check!(uqm_lua_guard_lua_pushinteger(L, v + 5));
            1
        }
        1 => {
            let mut len = 0;
            let p = check!(uqm_lua_guard_luaL_checklstring(L, 1, &mut len));
            check!(uqm_lua_guard_lua_pushstring(L, p));
            check!(uqm_lua_guard_lua_pushinteger(L, len as isize));
            2
        }
        2 => {
            check!(uqm_lua_guard_luaL_checktype(L, 1, 5));
            check!(uqm_lua_guard_lua_getfield(L, 1, b"value\0".as_ptr().cast()));
            let mut valid = 0;
            let v = check!(uqm_lua_guard_lua_tonumberx(L, -1, &mut valid));
            check!(uqm_lua_guard_lua_pushinteger(L, v as isize));
            check!(uqm_lua_guard_lua_pushboolean(L, valid));
            2
        }
        3 => {
            let options = [b"red\0".as_ptr().cast(), b"blue\0".as_ptr().cast(), null()];
            let n = check!(uqm_lua_guard_luaL_checkoption(
                L,
                1,
                b"blue\0".as_ptr().cast(),
                options.as_ptr()
            ));
            check!(uqm_lua_guard_lua_pushinteger(L, n as isize));
            1
        }
        4 => {
            check!(uqm_lua_guard_luaL_error(
                L,
                b"custom %s %d\0".as_ptr().cast(),
                b"failure\0".as_ptr().cast::<c_char>(),
                17 as c_int
            ));
            0
        }
        5 => {
            check!(uqm_lua_guard_lua_createtable(L, 2, 2));
            let table = check!(uqm_lua_guard_lua_absindex(L, -1));
            check!(uqm_lua_guard_lua_pushinteger(L, 42));
            check!(uqm_lua_guard_lua_setfield(
                L,
                table,
                b"key\0".as_ptr().cast()
            ));
            check!(uqm_lua_guard_lua_pushstring(L, b"seven\0".as_ptr().cast()));
            check!(uqm_lua_guard_lua_rawseti(L, table, 7));
            check!(uqm_lua_guard_lua_pushstring(L, b"key\0".as_ptr().cast()));
            check!(uqm_lua_guard_lua_gettable(L, table));
            check!(uqm_lua_guard_lua_rawgeti(L, table, 7));
            check!(uqm_lua_guard_lua_getglobal(L, b"type\0".as_ptr().cast()));
            check!(uqm_lua_guard_lua_pushvalue(L, table));
            check!(uqm_lua_guard_lua_callk(L, 1, 1, 0, None));
            3
        }
        6 => {
            check!(uqm_lua_guard_lua_pushvalue(L, 1));
            check!(uqm_lua_guard_lua_callk(L, 0, 1, 0, None));
            1
        }
        7 => {
            check!(uqm_lua_guard_lua_pushvalue(L, 1));
            let status = check!(uqm_lua_guard_lua_pcallk(L, 0, 1, 0, 0, None));
            check!(uqm_lua_guard_lua_pushinteger(L, status as isize));
            2
        }
        8 => {
            let mut len = 0;
            let p = check!(uqm_lua_guard_lua_tolstring(L, 1, &mut len));
            check!(uqm_lua_guard_lua_pushstring(L, p));
            check!(uqm_lua_guard_lua_pushinteger(L, len as isize));
            2
        }
        9 => {
            check!(uqm_lua_guard_lua_pushlightuserdata(L, L));
            let p = check!(uqm_lua_guard_lua_topointer(L, -1));
            check!(uqm_lua_guard_lua_pushboolean(L, (p == L) as c_int));
            let v = check!(uqm_lua_guard_lua_toboolean(L, -1));
            check!(uqm_lua_guard_lua_pushnil(L));
            check!(uqm_lua_guard_lua_replace(L, -2));
            check!(uqm_lua_guard_lua_settop(L, -2));
            check!(uqm_lua_guard_lua_pushinteger(L, v as isize));
            let ty = check!(uqm_lua_guard_lua_type(L, -1));
            let name = check!(uqm_lua_guard_lua_typename(L, ty));
            check!(uqm_lua_guard_lua_pushstring(L, name));
            2
        }
        10 => {
            check!(uqm_lua_guard_lua_createtable(L, 0, 1));
            let regs = [
                Reg {
                    name: b"inc\0".as_ptr().cast(),
                    function: Some(nested),
                },
                Reg {
                    name: null(),
                    function: None,
                },
            ];
            check!(uqm_lua_guard_luaL_setfuncs(L, regs.as_ptr(), 0));
            check!(uqm_lua_guard_lua_getfield(L, -1, b"inc\0".as_ptr().cast()));
            check!(uqm_lua_guard_lua_pushvalue(L, 1));
            check!(uqm_lua_guard_lua_callk(L, 1, 1, 0, None));
            1
        }
        11 => {
            let mut read = 0;
            let status = check!(uqm_lua_guard_lua_load(
                L,
                Some(reader),
                (&mut read as *mut c_int).cast(),
                b"reader\0".as_ptr().cast(),
                null()
            ));
            check!(uqm_lua_guard_lua_pushinteger(L, status as isize));
            2
        }
        12 => {
            test_arm_oom();
            check!(uqm_lua_guard_lua_createtable(L, 100000, 0));
            test_disarm_oom();
            1
        }
        13 => {
            let p = test_big_string();
            test_arm_oom();
            check!(uqm_lua_guard_lua_pushstring(L, p));
            test_disarm_oom();
            1
        }
        14 => {
            test_arm_oom();
            check!(uqm_lua_guard_lua_tolstring(L, 1, null_mut()));
            test_disarm_oom();
            1
        }
        15 => {
            let status = check!(uqm_lua_guard_luaL_loadstring(
                L,
                b"return (\0".as_ptr().cast()
            ));
            check!(uqm_lua_guard_lua_pushinteger(L, status as isize));
            2
        }
        16 => {
            check!(uqm_lua_guard_lua_pushstring(L, b"key\0".as_ptr().cast()));
            check!(uqm_lua_guard_lua_pushinteger(L, 9));
            check!(uqm_lua_guard_lua_settable(L, 1));
            0
        }
        17 => {
            let count = check!(uqm_lua_guard_lua_gettop(L));
            check!(uqm_lua_guard_lua_pushinteger(L, count as isize));
            1
        }
        18 => {
            let regs = [
                Reg {
                    name: b"sum\0".as_ptr().cast(),
                    function: Some(upvalue),
                },
                Reg {
                    name: b"sum_again\0".as_ptr().cast(),
                    function: Some(upvalue),
                },
                Reg {
                    name: null(),
                    function: None,
                },
            ];
            check!(uqm_lua_guard_lua_createtable(L, 0, 2));
            check!(uqm_lua_guard_lua_pushinteger(L, 43));
            check!(uqm_lua_guard_luaL_setfuncs(L, regs.as_ptr(), 1));
            check!(uqm_lua_guard_lua_getfield(
                L,
                -1,
                b"sum_again\0".as_ptr().cast()
            ));
            check!(uqm_lua_guard_lua_pushvalue(L, 1));
            check!(uqm_lua_guard_lua_callk(L, 1, 1, 0, None));
            1
        }
        19 => {
            test_arm_oom();
            check!(uqm_lua_guard_lua_pushcclosure(L, Some(nested), 0));
            test_disarm_oom();
            1
        }
        20 => {
            test_arm_oom();
            check!(uqm_lua_guard_luaL_error(
                L,
                b"error message with new allocation %d\0".as_ptr().cast(),
                918273 as c_int
            ));
            0
        }
        21 => {
            test_arm_oom();
            check!(uqm_lua_guard_luaL_checkinteger(L, 1));
            test_disarm_oom();
            0
        }
        _ => abort(),
    };
    test_operation_completed();
    results
}
