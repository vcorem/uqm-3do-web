/*	MikMod sound library
	(c) 1998, 1999, 2000, 2001 Miodrag Vallat and others - see file AUTHORS
	for complete list.

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

  All routines dealing with the manipulation of UNITRK streams

==============================================================================*/

// Rust translation of src/libs/mikmod/munitrk.c.
extern "C" {
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_realloc(
        _: *mut ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type BOOL = ::core::ffi::c_int;
pub type UBYTE = ::core::ffi::c_uchar;
pub type UWORD = ::core::ffi::c_ushort;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const UNI_NOTE: Self = Self(1);
    pub const UNI_INSTRUMENT: Self = Self(2);
    pub const UNI_PTEFFECT0: Self = Self(3);
    pub const UNI_PTEFFECT1: Self = Self(4);
    pub const UNI_PTEFFECT2: Self = Self(5);
    pub const UNI_PTEFFECT3: Self = Self(6);
    pub const UNI_PTEFFECT4: Self = Self(7);
    pub const UNI_PTEFFECT5: Self = Self(8);
    pub const UNI_PTEFFECT6: Self = Self(9);
    pub const UNI_PTEFFECT7: Self = Self(10);
    pub const UNI_PTEFFECT8: Self = Self(11);
    pub const UNI_PTEFFECT9: Self = Self(12);
    pub const UNI_PTEFFECTA: Self = Self(13);
    pub const UNI_PTEFFECTB: Self = Self(14);
    pub const UNI_PTEFFECTC: Self = Self(15);
    pub const UNI_PTEFFECTD: Self = Self(16);
    pub const UNI_PTEFFECTE: Self = Self(17);
    pub const UNI_PTEFFECTF: Self = Self(18);
    pub const UNI_S3MEFFECTA: Self = Self(19);
    pub const UNI_S3MEFFECTD: Self = Self(20);
    pub const UNI_S3MEFFECTE: Self = Self(21);
    pub const UNI_S3MEFFECTF: Self = Self(22);
    pub const UNI_S3MEFFECTI: Self = Self(23);
    pub const UNI_S3MEFFECTQ: Self = Self(24);
    pub const UNI_S3MEFFECTR: Self = Self(25);
    pub const UNI_S3MEFFECTT: Self = Self(26);
    pub const UNI_S3MEFFECTU: Self = Self(27);
    pub const UNI_KEYOFF: Self = Self(28);
    pub const UNI_KEYFADE: Self = Self(29);
    pub const UNI_VOLEFFECTS: Self = Self(30);
    pub const UNI_XMEFFECT4: Self = Self(31);
    pub const UNI_XMEFFECT6: Self = Self(32);
    pub const UNI_XMEFFECTA: Self = Self(33);
    pub const UNI_XMEFFECTE1: Self = Self(34);
    pub const UNI_XMEFFECTE2: Self = Self(35);
    pub const UNI_XMEFFECTEA: Self = Self(36);
    pub const UNI_XMEFFECTEB: Self = Self(37);
    pub const UNI_XMEFFECTG: Self = Self(38);
    pub const UNI_XMEFFECTH: Self = Self(39);
    pub const UNI_XMEFFECTL: Self = Self(40);
    pub const UNI_XMEFFECTP: Self = Self(41);
    pub const UNI_XMEFFECTX1: Self = Self(42);
    pub const UNI_XMEFFECTX2: Self = Self(43);
    pub const UNI_ITEFFECTG: Self = Self(44);
    pub const UNI_ITEFFECTH: Self = Self(45);
    pub const UNI_ITEFFECTI: Self = Self(46);
    pub const UNI_ITEFFECTM: Self = Self(47);
    pub const UNI_ITEFFECTN: Self = Self(48);
    pub const UNI_ITEFFECTP: Self = Self(49);
    pub const UNI_ITEFFECTT: Self = Self(50);
    pub const UNI_ITEFFECTU: Self = Self(51);
    pub const UNI_ITEFFECTW: Self = Self(52);
    pub const UNI_ITEFFECTY: Self = Self(53);
    pub const UNI_ITEFFECTZ: Self = Self(54);
    pub const UNI_ITEFFECTS0: Self = Self(55);
    pub const UNI_ULTEFFECT9: Self = Self(56);
    pub const UNI_MEDSPEED: Self = Self(57);
    pub const UNI_MEDEFFECTF1: Self = Self(58);
    pub const UNI_MEDEFFECTF2: Self = Self(59);
    pub const UNI_MEDEFFECTF3: Self = Self(60);
    pub const UNI_OKTARP: Self = Self(61);
    pub const UNI_LAST: Self = Self(62);
}
#[no_mangle]
pub static mut unioperands: [UWORD; 62] = [
    0 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    0 as UWORD,
    1 as UWORD,
    2 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    1 as UWORD,
    2 as UWORD,
    1 as UWORD,
    2 as UWORD,
    2 as UWORD,
    0 as UWORD,
    0 as UWORD,
    0 as UWORD,
    2 as UWORD,
];
static mut rowstart: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
static mut rowend: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
static mut rowpc: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
static mut lastbyte: UBYTE = 0;
#[no_mangle]
pub unsafe extern "C" fn UniSetRow(mut t: *mut UBYTE) {
    rowstart = t;
    rowpc = rowstart;
    rowend = if !t.is_null() {
        let c2rust_fresh0 = rowpc;
        rowpc = rowpc.offset(1);
        rowstart
            .offset(
                (*c2rust_fresh0 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                    as isize,
            )
    } else {
        t
    };
}
#[no_mangle]
pub unsafe extern "C" fn UniGetByte() -> UBYTE {
    lastbyte = (if rowpc < rowend {
        let c2rust_fresh1 = rowpc;
        rowpc = rowpc.offset(1);
        *c2rust_fresh1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as UBYTE;
    return lastbyte;
}
#[no_mangle]
pub unsafe extern "C" fn UniGetWord() -> UWORD {
    return ((UniGetByte() as UWORD as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | UniGetByte() as ::core::ffi::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn UniSkipOpcode() {
    if (lastbyte as ::core::ffi::c_int)
        < C2Rust_Unnamed::UNI_LAST.0 as ::core::ffi::c_int
    {
        let mut t: UWORD = unioperands[lastbyte as usize];
        loop {
            let c2rust_fresh2 = t;
            t = t.wrapping_sub(1);
            if c2rust_fresh2 == 0 {
                break;
            }
            UniGetByte();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UniFindRow(mut t: *mut UBYTE, mut row: UWORD) -> *mut UBYTE {
    let mut c: UBYTE = 0;
    let mut l: UBYTE = 0;
    if !t.is_null() {
        loop {
            c = *t;
            if c == 0 {
                return ::core::ptr::null_mut::<UBYTE>();
            }
            l = ((c as ::core::ffi::c_int >> 5 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as UBYTE;
            if l as ::core::ffi::c_int > row as ::core::ffi::c_int {
                break;
            }
            row = (row as ::core::ffi::c_int - l as ::core::ffi::c_int) as UWORD;
            t = t
                .offset((c as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as isize);
        }
    }
    return t;
}
static mut unibuf: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
static mut unimax: UWORD = 0;
static mut unipc: UWORD = 0;
static mut unitt: UWORD = 0;
static mut lastp: UWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn UniReset() {
    unitt = 0 as UWORD;
    unipc = 1 as UWORD;
    lastp = 0 as UWORD;
    *unibuf.offset(0isize) = 0 as UBYTE;
}
unsafe extern "C" fn UniExpand(mut wanted: ::core::ffi::c_int) -> BOOL {
    if unipc as ::core::ffi::c_int + wanted >= unimax as ::core::ffi::c_int {
        let mut newbuf: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
        newbuf = MikMod_realloc(
            unibuf as *mut ::core::ffi::c_void,
            ((unimax as ::core::ffi::c_int + 128 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<UBYTE>()),
        ) as *mut UBYTE;
        if !newbuf.is_null() {
            unibuf = newbuf;
            unimax = (unimax as ::core::ffi::c_int + 128 as ::core::ffi::c_int) as UWORD;
            return 1 as BOOL;
        } else {
            return 0 as BOOL
        }
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn UniWriteByte(mut data: UBYTE) {
    if UniExpand(1 as ::core::ffi::c_int) != 0 {
        let c2rust_fresh3 = unipc;
        unipc = unipc.wrapping_add(1);
        *unibuf.offset(c2rust_fresh3 as isize) = data;
    }
}
#[no_mangle]
pub unsafe extern "C" fn UniWriteWord(mut data: UWORD) {
    if UniExpand(2 as ::core::ffi::c_int) != 0 {
        let c2rust_fresh4 = unipc;
        unipc = unipc.wrapping_add(1);
        *unibuf.offset(c2rust_fresh4 as isize) = (data as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as UBYTE;
        let c2rust_fresh5 = unipc;
        unipc = unipc.wrapping_add(1);
        *unibuf.offset(c2rust_fresh5 as isize) = (data as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as UBYTE;
    }
}
unsafe extern "C" fn MyCmp(
    mut a: *const UBYTE,
    mut b: *const UBYTE,
    mut l: UWORD,
) -> BOOL {
    let mut t: UWORD = 0;
    t = 0 as UWORD;
    while (t as ::core::ffi::c_int) < l as ::core::ffi::c_int {
        let c2rust_fresh7 = a;
        a = a.offset(1);
        let c2rust_fresh8 = b;
        b = b.offset(1);
        if *c2rust_fresh7 as ::core::ffi::c_int != *c2rust_fresh8 as ::core::ffi::c_int {
            return 0 as BOOL;
        }
        t = t.wrapping_add(1);
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn UniNewline() {
    let mut n: UWORD = 0;
    let mut l: UWORD = 0;
    let mut len: UWORD = 0;
    n = ((*unibuf.offset(lastp as isize) as ::core::ffi::c_int
        >> 5 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as UWORD;
    l = (*unibuf.offset(lastp as isize) as ::core::ffi::c_int
        & 0x1f as ::core::ffi::c_int) as UWORD;
    len = (unipc as ::core::ffi::c_int - unitt as ::core::ffi::c_int) as UWORD;
    if (n as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
        && len as ::core::ffi::c_int == l as ::core::ffi::c_int
        && MyCmp(
            unibuf
                .offset(
                    (lastp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ),
            unibuf
                .offset(
                    (unitt as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ),
            (len as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD,
        ) != 0
    {
        *unibuf.offset(lastp as isize) = (*unibuf.offset(lastp as isize)
            as ::core::ffi::c_int + 0x20 as ::core::ffi::c_int) as UBYTE;
        unipc = (unitt as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as UWORD;
    } else if UniExpand(unitt as ::core::ffi::c_int - unipc as ::core::ffi::c_int) != 0 {
        *unibuf.offset(unitt as isize) = len as UBYTE;
        lastp = unitt;
        let c2rust_fresh6 = unipc;
        unipc = unipc.wrapping_add(1);
        unitt = c2rust_fresh6;
    }
}
#[no_mangle]
pub unsafe extern "C" fn UniDup() -> *mut UBYTE {
    let mut d: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if UniExpand(unipc as ::core::ffi::c_int - unitt as ::core::ffi::c_int) == 0 {
        return ::core::ptr::null_mut::<UBYTE>();
    }
    *unibuf.offset(unitt as isize) = 0 as UBYTE;
    d = uqm_lua_checked!(MikMod_malloc(unipc as size_t), ::core::ptr::null_mut());
    if d.is_null() {
        return ::core::ptr::null_mut::<UBYTE>();
    }
    memcpy(d, unibuf as *const ::core::ffi::c_void, unipc as size_t);
    return d as *mut UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn UniInit() -> BOOL {
    unimax = 128 as UWORD;
    unibuf = uqm_lua_checked!(MikMod_malloc(
        (unimax as size_t).wrapping_mul(::core::mem::size_of::<UBYTE>()),
    ), 0) as *mut UBYTE;
    if unibuf.is_null() {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn UniCleanup() {
    MikMod_free(unibuf as *mut ::core::ffi::c_void);
    unibuf = ::core::ptr::null_mut::<UBYTE>();
}
