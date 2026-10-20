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

  MOD format period table.  Used by both the MOD and M15 (15-inst mod) Loaders.

==============================================================================*/

// Rust translation of src/libs/mikmod/npertab.c.
pub type UWORD = ::core::ffi::c_ushort;
#[no_mangle]
pub static mut npertab: [UWORD; 84] = [
    0x6b0 as UWORD,
    0x650 as UWORD,
    0x5f4 as UWORD,
    0x5a0 as UWORD,
    0x54c as UWORD,
    0x500 as UWORD,
    0x4b8 as UWORD,
    0x474 as UWORD,
    0x434 as UWORD,
    0x3f8 as UWORD,
    0x3c0 as UWORD,
    0x38a as UWORD,
    0x358 as UWORD,
    0x328 as UWORD,
    0x2fa as UWORD,
    0x2d0 as UWORD,
    0x2a6 as UWORD,
    0x280 as UWORD,
    0x25c as UWORD,
    0x23a as UWORD,
    0x21a as UWORD,
    0x1fc as UWORD,
    0x1e0 as UWORD,
    0x1c5 as UWORD,
    0x1ac as UWORD,
    0x194 as UWORD,
    0x17d as UWORD,
    0x168 as UWORD,
    0x153 as UWORD,
    0x140 as UWORD,
    0x12e as UWORD,
    0x11d as UWORD,
    0x10d as UWORD,
    0xfe as UWORD,
    0xf0 as UWORD,
    0xe2 as UWORD,
    0xd6 as UWORD,
    0xca as UWORD,
    0xbe as UWORD,
    0xb4 as UWORD,
    0xaa as UWORD,
    0xa0 as UWORD,
    0x97 as UWORD,
    0x8f as UWORD,
    0x87 as UWORD,
    0x7f as UWORD,
    0x78 as UWORD,
    0x71 as UWORD,
    0x6b as UWORD,
    0x65 as UWORD,
    0x5f as UWORD,
    0x5a as UWORD,
    0x55 as UWORD,
    0x50 as UWORD,
    0x4b as UWORD,
    0x47 as UWORD,
    0x43 as UWORD,
    0x3f as UWORD,
    0x3c as UWORD,
    0x38 as UWORD,
    0x35 as UWORD,
    0x32 as UWORD,
    0x2f as UWORD,
    0x2d as UWORD,
    0x2a as UWORD,
    0x28 as UWORD,
    0x25 as UWORD,
    0x23 as UWORD,
    0x21 as UWORD,
    0x1f as UWORD,
    0x1e as UWORD,
    0x1c as UWORD,
    0x1b as UWORD,
    0x19 as UWORD,
    0x18 as UWORD,
    0x16 as UWORD,
    0x15 as UWORD,
    0x14 as UWORD,
    0x13 as UWORD,
    0x12 as UWORD,
    0x11 as UWORD,
    0x10 as UWORD,
    0xf as UWORD,
    0xe as UWORD,
];
