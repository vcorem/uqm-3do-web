/*
 *  Copyright 2012  Serge van den Boom <svdb@stack.nl>
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

// Rust translation of src/libs/callback/async.c.
extern "C" {
    fn Alarm_processOne() -> bool;
    fn Alarm_timeBeforeNextMs() -> uint32;
    fn Callback_process();
    fn Callback_haveMore() -> bool;
}
pub type uint32 = ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn Async_process() {
    uqm_lua_checked!(Callback_process(), ());
    loop {
        if !uqm_lua_checked!(Alarm_processOne(), ()) {
            return;
        }
        uqm_lua_checked!(Callback_process(), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn Async_timeBeforeNextMs() -> uint32 {
    if Callback_haveMore() {
        return 1 as uint32;
    }
    return Alarm_timeBeforeNextMs();
}
