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

// Rust translation of src/uqm/supermelee/netplay/netoptions.c.
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayPeerOptions {
    pub isServer: bool,
    pub host: *const ::core::ffi::c_char,
    pub port: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayOptions {
    pub metaServer: *const ::core::ffi::c_char,
    pub metaPort: *const ::core::ffi::c_char,
    pub peer: [NetplayPeerOptions; 2],
    pub inputDelay: size_t,
}
#[no_mangle]
pub static mut netplayOptions: NetplayOptions = NetplayOptions {
    metaServer: b"uqm.stack.nl\0".as_ptr() as *const ::core::ffi::c_char,
    metaPort: b"21836\0".as_ptr() as *const ::core::ffi::c_char,
    peer: [
        NetplayPeerOptions {
            isServer: true,
            host: b"localhost\0".as_ptr() as *const ::core::ffi::c_char,
            port: b"21837\0".as_ptr() as *const ::core::ffi::c_char,
        },
        NetplayPeerOptions {
            isServer: true,
            host: b"localhost\0".as_ptr() as *const ::core::ffi::c_char,
            port: b"21837\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ],
    inputDelay: 2 as size_t,
};
#[no_mangle]
pub static mut webNetplayRole: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
