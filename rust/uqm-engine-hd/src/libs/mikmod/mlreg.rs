/*	MikMod sound library
	(c) 1998, 1999 Miodrag Vallat and others - see file AUTHORS for
	complete list.

	This library is free software; you can redistribute it and/or modify
	it under the terms of the GNU Library General Public License as
	published by the Free Software Foundation; either version 2 of
	the License, or (at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU Library General Public License for more details.

	You should have received a copy of the GNU Library General Public
	License along with this library; if not, write to the Free Software
	Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA
	02111-1307, USA.
*/

/*==============================================================================

  $Id$

  Routine for registering all loaders in libmikmod for the current platform.

==============================================================================*/

// Rust translation of src/libs/mikmod/mlreg.c.
extern "C" {
    fn _mm_registerloader(_: *mut MLOADER);
    static mut load_it: MLOADER;
    static mut load_mod: MLOADER;
    static mut load_stm: MLOADER;
    static mut load_s3m: MLOADER;
    static mut load_xm: MLOADER;
}
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MLOADER {
    pub next: *mut MLOADER,
    pub r#type: *const CHAR,
    pub version: *const CHAR,
    pub Init: Option<unsafe extern "C" fn() -> BOOL>,
    pub Test: Option<unsafe extern "C" fn() -> BOOL>,
    pub Load: Option<unsafe extern "C" fn(BOOL) -> BOOL>,
    pub Cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub LoadTitle: Option<unsafe extern "C" fn() -> *mut CHAR>,
}
unsafe extern "C" fn MikMod_RegisterAllLoaders_internal() {
    _mm_registerloader(&raw mut load_it);
    _mm_registerloader(&raw mut load_mod);
    _mm_registerloader(&raw mut load_s3m);
    _mm_registerloader(&raw mut load_stm);
    _mm_registerloader(&raw mut load_xm);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_RegisterAllLoaders() {
    MikMod_RegisterAllLoaders_internal();
}
