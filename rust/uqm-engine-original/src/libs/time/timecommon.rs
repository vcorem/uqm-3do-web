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
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

/* By Serge van den Boom, 2002-09-12
 */

// Rust translation of src/libs/time/timecommon.c.
extern "C" {
    fn SDLWrapper_GetTimeCounter() -> Uint32;
}
pub type uint32_t = ::core::ffi::c_uint;
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
pub type TimeCount = DWORD;
pub type Uint32 = uint32_t;
#[no_mangle]
pub unsafe extern "C" fn InitTimeSystem() {}
#[no_mangle]
pub unsafe extern "C" fn UnInitTimeSystem() {}
#[no_mangle]
pub unsafe extern "C" fn GetTimeCounter() -> TimeCount {
    return SDLWrapper_GetTimeCounter();
}
