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

// Rust translation of src/libs/log/msgbox_stub.c.
extern "C" {
    pub type _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    static mut streamOut: *mut FILE;
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn log_displayBox(
    mut title: *const ::core::ffi::c_char,
    mut isError: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    fprintf(
        streamOut,
        b"Do not know how to display %s box\n\0".as_ptr() as *const ::core::ffi::c_char,
        if isError != 0 {
            b"an error\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"a\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
