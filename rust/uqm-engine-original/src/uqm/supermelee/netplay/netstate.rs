/*
 *  Copyright 2006  Serge van den Boom <svdb@stack.nl>
 *
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

// Rust translation of src/uqm/supermelee/netplay/netstate.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnectionStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
}
pub type NetConnectionStateData_ReleaseFunction = Option<
    unsafe extern "C" fn(*mut NetConnectionStateData) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetStateData {
    pub name: *const ::core::ffi::c_char,
}
#[no_mangle]
pub static mut netStateData: [NetStateData; 10] = [
    NetStateData {
        name: b"unconnected\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"connecting\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"init\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"inSetup\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"preBattle\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"interBattle\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"selectShip\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"inBattle\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"endingBattle\0".as_ptr() as *const ::core::ffi::c_char,
    },
    NetStateData {
        name: b"endingBattle2\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
#[no_mangle]
pub unsafe extern "C" fn NetConnectionStateData_release(
    mut stateData: *mut NetConnectionStateData,
) {
    ((*stateData).releaseFunction.is_some()
        || {
            __assert_fail(
                b"stateData->releaseFunction != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/netstate.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_int,
                b"NetConnectionStateData_release\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*stateData).releaseFunction.expect("non-null function pointer")(stateData);
}
