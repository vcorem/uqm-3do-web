//Copyright Paul Reiche, Fred Ford. 1992-2002

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

 /*
 * LZHUF.C English version 1.0
 * Based on Japanese version 29-NOV-1988
 * LZSS coded by Haruhiko OKUMURA
 * Adaptive Huffman Coding coded by Haruyasu YOSHIZAKI
 * Edited and translated to English by Kenji RIKITAKE
 */

// Rust translation of src/libs/decomp/lzencode.c.
extern "C" {
    pub type uio_Stream;
    fn _update(c: COUNT);
    static mut _lpCurCodeDesc: PLZHCODE_DESC;
    static mut _StreamType: STREAM_TYPE;
    static mut _Stream: *mut BYTE;
    static mut _workbuf: UWORD;
    static mut _workbuflen: BYTE;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn PutResFileChar(
        ch: ::core::ffi::c_char,
        fp: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn SeekResFile(
        fp: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type PVOIDFUNC = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LZHCODE_DESC {
    pub buf_index: COUNT,
    pub restart_index: COUNT,
    pub bytes_left: COUNT,
    pub text_buf: [BYTE; 4111],
    pub freq: [COUNT; 540],
    pub prnt: [COUNT; 809],
    pub son: [COUNT; 539],
    pub workbuf: UWORD,
    pub workbuflen: BYTE,
    pub StreamType: STREAM_TYPE,
    pub Stream: *mut ::core::ffi::c_void,
    pub StreamIndex: DWORD,
    pub StreamLength: DWORD,
    pub StreamMode: STREAM_MODE,
    pub CleanupFunc: PVOIDFUNC,
}
pub type STREAM_MODE = BYTE;
pub type STREAM_TYPE = BYTE;
pub type DECODE_REF = *mut _LZHCODE_DESC;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const FILE_STREAM: Self = Self(0);
    pub const MEMORY_STREAM: Self = Self(1);
}
pub type PLZHCODE_DESC = *mut LZHCODE_DESC;
pub type LZHCODE_DESC = _LZHCODE_DESC;
static mut match_position: UWORD = 0;
static mut match_length: UWORD = 0;
static mut lson: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
static mut rson: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
static mut dad: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
static mut encode_arrays: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
unsafe extern "C" fn InitTree() -> BOOLEAN {
    encode_arrays = HCalloc(
        ((4096 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + (4096 as ::core::ffi::c_int + 257 as ::core::ffi::c_int)
            + (4096 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<SWORD>()),
    ) as *mut SWORD;
    if encode_arrays.is_null() {
        HFree(encode_arrays as *mut ::core::ffi::c_void);
        encode_arrays = ::core::ptr::null_mut::<SWORD>();
        return BOOLEAN::FALSE;
    } else {
        let mut i: SWORD = 0;
        lson = encode_arrays;
        rson = lson
            .offset((4096 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        dad = rson
            .offset((4096 as ::core::ffi::c_int + 257 as ::core::ffi::c_int) as isize);
        i = (4096 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SWORD;
        while i as ::core::ffi::c_int
            <= 4096 as ::core::ffi::c_int + 256 as ::core::ffi::c_int
        {
            *rson.offset(i as isize) = 4096 as SWORD;
            i += 1;
        }
        i = 0 as SWORD;
        while (i as ::core::ffi::c_int) < 4096 as ::core::ffi::c_int {
            *dad.offset(i as isize) = 4096 as SWORD;
            i += 1;
        }
        return BOOLEAN::TRUE;
    };
}
unsafe extern "C" fn InsertNode(mut r: SWORD) {
    let mut p: SWORD = 0;
    let mut cmp: SWORD = 0;
    let mut lpBuf: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    cmp = 1 as SWORD;
    lpBuf = &raw mut (*_lpCurCodeDesc).text_buf as *mut BYTE;
    p = (4096 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        + *lpBuf.offset(r as isize) as ::core::ffi::c_int) as SWORD;
    *lson.offset(r as isize) = 4096 as SWORD;
    *rson.offset(r as isize) = *lson.offset(r as isize);
    match_length = 0 as UWORD;
    loop {
        let mut i: UWORD = 0;
        if cmp as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            if *rson.offset(p as isize) as ::core::ffi::c_int
                != 4096 as ::core::ffi::c_int
            {
                p = *rson.offset(p as isize);
            } else {
                *rson.offset(p as isize) = r;
                *dad.offset(r as isize) = p;
                return;
            }
        } else if *lson.offset(p as isize) as ::core::ffi::c_int
            != 4096 as ::core::ffi::c_int
        {
            p = *lson.offset(p as isize);
        } else {
            *lson.offset(p as isize) = r;
            *dad.offset(r as isize) = p;
            return;
        }
        i = 16 as UWORD;
        let mut _r: SWORD = 0;
        let mut _p: SWORD = 0;
        _r = r;
        _p = p;
        loop {
            i = i.wrapping_sub(1);
            if !(i as ::core::ffi::c_int != 0
                && {
                    _r += 1;
                    _p += 1;
                    cmp = (*lpBuf.offset(_r as isize) as ::core::ffi::c_int
                        - *lpBuf.offset(_p as isize) as ::core::ffi::c_int) as SWORD;
                    cmp as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                })
            {
                break;
            }
        }
        i = (16 as ::core::ffi::c_int - i as ::core::ffi::c_int) as UWORD;
        if i as ::core::ffi::c_int <= 2 as ::core::ffi::c_int {
            continue;
        }
        if i as ::core::ffi::c_int > match_length as ::core::ffi::c_int {
            match_position = ((r as ::core::ffi::c_int - p as ::core::ffi::c_int
                & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as UWORD;
            match_length = i;
            if match_length as ::core::ffi::c_int >= 16 as ::core::ffi::c_int {
                break;
            }
        } else if i as ::core::ffi::c_int == match_length as ::core::ffi::c_int {
            i = ((r as ::core::ffi::c_int - p as ::core::ffi::c_int
                & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as UWORD;
            if (i as ::core::ffi::c_int) < match_position as ::core::ffi::c_int {
                match_position = i;
            }
        }
    }
    *dad.offset(r as isize) = *dad.offset(p as isize);
    *lson.offset(r as isize) = *lson.offset(p as isize);
    *rson.offset(r as isize) = *rson.offset(p as isize);
    *dad.offset(*lson.offset(p as isize) as isize) = r;
    *dad.offset(*rson.offset(p as isize) as isize) = r;
    if *rson.offset(*dad.offset(p as isize) as isize) as ::core::ffi::c_int
        == p as ::core::ffi::c_int
    {
        *rson.offset(*dad.offset(p as isize) as isize) = r;
    } else {
        *lson.offset(*dad.offset(p as isize) as isize) = r;
    }
    *dad.offset(p as isize) = 4096 as SWORD;
}
unsafe extern "C" fn DeleteNode(mut p: SWORD) {
    let mut q: SWORD = 0;
    if *dad.offset(p as isize) as ::core::ffi::c_int == 4096 as ::core::ffi::c_int {
        return;
    }
    if *rson.offset(p as isize) as ::core::ffi::c_int == 4096 as ::core::ffi::c_int {
        q = *lson.offset(p as isize);
    } else if *lson.offset(p as isize) as ::core::ffi::c_int
        == 4096 as ::core::ffi::c_int
    {
        q = *rson.offset(p as isize);
    } else {
        q = *lson.offset(p as isize);
        if *rson.offset(q as isize) as ::core::ffi::c_int != 4096 as ::core::ffi::c_int {
            loop {
                q = *rson.offset(q as isize);
                if *rson.offset(q as isize) as ::core::ffi::c_int
                    == 4096 as ::core::ffi::c_int
                {
                    break;
                }
            }
            *rson.offset(*dad.offset(q as isize) as isize) = *lson.offset(q as isize);
            *dad.offset(*lson.offset(q as isize) as isize) = *dad.offset(q as isize);
            *lson.offset(q as isize) = *lson.offset(p as isize);
            *dad.offset(*lson.offset(p as isize) as isize) = q;
        }
        *rson.offset(q as isize) = *rson.offset(p as isize);
        *dad.offset(*rson.offset(p as isize) as isize) = q;
    }
    *dad.offset(q as isize) = *dad.offset(p as isize);
    if *rson.offset(*dad.offset(p as isize) as isize) as ::core::ffi::c_int
        == p as ::core::ffi::c_int
    {
        *rson.offset(*dad.offset(p as isize) as isize) = q;
    } else {
        *lson.offset(*dad.offset(p as isize) as isize) = q;
    }
    *dad.offset(p as isize) = 4096 as SWORD;
}
unsafe extern "C" fn Putcode(mut l: SWORD, mut c: UWORD) {
    _workbuf = (_workbuf as ::core::ffi::c_int
        | c as ::core::ffi::c_int >> _workbuflen as ::core::ffi::c_int) as UWORD;
    _workbuflen = (_workbuflen as ::core::ffi::c_int + l as ::core::ffi::c_int) as BYTE;
    if _workbuflen as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
        if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            PutResFileChar(
                (_workbuf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_char,
                _Stream as *mut uio_Stream,
            );
        } else {
            let c2rust_fresh2 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh2 = (_workbuf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as BYTE;
        };
        (*_lpCurCodeDesc).StreamIndex = (*_lpCurCodeDesc).StreamIndex.wrapping_add(1);
        _workbuflen = (_workbuflen as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
            as BYTE;
        if _workbuflen as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
            if _StreamType as ::core::ffi::c_int
                == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
            {
                PutResFileChar(
                    _workbuf as BYTE as ::core::ffi::c_char,
                    _Stream as *mut uio_Stream,
                );
            } else {
                let c2rust_fresh3 = _Stream;
                _Stream = _Stream.offset(1);
                *c2rust_fresh3 = _workbuf as BYTE;
            };
            (*_lpCurCodeDesc).StreamIndex = (*_lpCurCodeDesc)
                .StreamIndex
                .wrapping_add(1);
            _workbuflen = (_workbuflen as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                as BYTE;
            _workbuf = ((c as ::core::ffi::c_int)
                << l as ::core::ffi::c_int - _workbuflen as ::core::ffi::c_int) as UWORD;
        } else {
            _workbuf = ((_workbuf as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as UWORD;
        }
        _workbuf = (_workbuf as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
            as UWORD;
    }
}
unsafe extern "C" fn EncodeChar(mut c: UWORD) {
    let mut i: UWORD = 0;
    let mut j: SWORD = 0;
    let mut k: SWORD = 0;
    i = 0 as UWORD;
    j = 0 as SWORD;
    k = (*_lpCurCodeDesc)
        .prnt[(c as ::core::ffi::c_int
        + ((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)) as usize] as SWORD;
    loop {
        i = (i as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as UWORD;
        if k as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            i = (i as ::core::ffi::c_int + 0x8000 as ::core::ffi::c_int) as UWORD;
        }
        j += 1;
        k = (*_lpCurCodeDesc).prnt[k as usize] as SWORD;
        if k as ::core::ffi::c_int
            == (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            break;
        }
    }
    Putcode(j, i);
    _update(
        (c as ::core::ffi::c_int
            + ((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)) as COUNT,
    );
}
unsafe extern "C" fn EncodePosition(mut c: UWORD) {
    let mut i: UWORD = 0;
    static mut p_len: [BYTE; 64] = [
        0x3 as BYTE,
        0x4 as BYTE,
        0x4 as BYTE,
        0x4 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x5 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x6 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x7 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
        0x8 as BYTE,
    ];
    static mut p_code: [BYTE; 64] = [
        0 as BYTE,
        0x20 as BYTE,
        0x30 as BYTE,
        0x40 as BYTE,
        0x50 as BYTE,
        0x58 as BYTE,
        0x60 as BYTE,
        0x68 as BYTE,
        0x70 as BYTE,
        0x78 as BYTE,
        0x80 as BYTE,
        0x88 as BYTE,
        0x90 as BYTE,
        0x94 as BYTE,
        0x98 as BYTE,
        0x9c as BYTE,
        0xa0 as BYTE,
        0xa4 as BYTE,
        0xa8 as BYTE,
        0xac as BYTE,
        0xb0 as BYTE,
        0xb4 as BYTE,
        0xb8 as BYTE,
        0xbc as BYTE,
        0xc0 as BYTE,
        0xc2 as BYTE,
        0xc4 as BYTE,
        0xc6 as BYTE,
        0xc8 as BYTE,
        0xca as BYTE,
        0xcc as BYTE,
        0xce as BYTE,
        0xd0 as BYTE,
        0xd2 as BYTE,
        0xd4 as BYTE,
        0xd6 as BYTE,
        0xd8 as BYTE,
        0xda as BYTE,
        0xdc as BYTE,
        0xde as BYTE,
        0xe0 as BYTE,
        0xe2 as BYTE,
        0xe4 as BYTE,
        0xe6 as BYTE,
        0xe8 as BYTE,
        0xea as BYTE,
        0xec as BYTE,
        0xee as BYTE,
        0xf0 as BYTE,
        0xf1 as BYTE,
        0xf2 as BYTE,
        0xf3 as BYTE,
        0xf4 as BYTE,
        0xf5 as BYTE,
        0xf6 as BYTE,
        0xf7 as BYTE,
        0xf8 as BYTE,
        0xf9 as BYTE,
        0xfa as BYTE,
        0xfb as BYTE,
        0xfc as BYTE,
        0xfd as BYTE,
        0xfe as BYTE,
        0xff as BYTE,
    ];
    i = (c as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as UWORD;
    Putcode(
        p_len[i as usize] as SWORD,
        ((p_code[i as usize] as UWORD as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as UWORD,
    );
    Putcode(
        6 as SWORD,
        ((c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int) as UWORD,
    );
}
unsafe extern "C" fn UninitTree() {
    if _workbuflen != 0 {
        if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            PutResFileChar(
                (_workbuf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_char,
                _Stream as *mut uio_Stream,
            );
        } else {
            let c2rust_fresh9 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh9 = (_workbuf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as BYTE;
        };
        (*_lpCurCodeDesc).StreamIndex = (*_lpCurCodeDesc).StreamIndex.wrapping_add(1);
    }
    HFree(encode_arrays as *mut ::core::ffi::c_void);
    encode_arrays = ::core::ptr::null_mut::<SWORD>();
    lson = ::core::ptr::null_mut::<SWORD>();
    rson = ::core::ptr::null_mut::<SWORD>();
    dad = ::core::ptr::null_mut::<SWORD>();
}
unsafe extern "C" fn _encode_cleanup() {
    let mut r: UWORD = 0;
    let mut s: UWORD = 0;
    let mut last_match_length: UWORD = 0;
    let mut len: UWORD = 0;
    _StreamType = (*_lpCurCodeDesc).StreamType;
    _Stream = (*_lpCurCodeDesc).Stream as *mut BYTE;
    _workbuf = (*_lpCurCodeDesc).workbuf;
    _workbuflen = (*_lpCurCodeDesc).workbuflen;
    r = (*_lpCurCodeDesc).buf_index as UWORD;
    s = (*_lpCurCodeDesc).restart_index as UWORD;
    last_match_length = (*_lpCurCodeDesc).bytes_left as UWORD;
    if (*_lpCurCodeDesc).StreamLength >= 16 as DWORD {
        len = 16 as UWORD;
    } else {
        let mut i: UWORD = 0;
        i = 1 as UWORD;
        while i as ::core::ffi::c_int <= 16 as ::core::ffi::c_int {
            InsertNode((r as ::core::ffi::c_int - i as ::core::ffi::c_int) as SWORD);
            i = i.wrapping_add(1);
        }
        InsertNode(r as SWORD);
        len = (*_lpCurCodeDesc).StreamLength as UWORD;
    }
    loop {
        loop {
            let c2rust_fresh4 = last_match_length;
            last_match_length = last_match_length.wrapping_sub(1);
            if c2rust_fresh4 == 0 {
                break;
            }
            DeleteNode(s as SWORD);
            len = len.wrapping_sub(1);
            if len as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut lobyte: BYTE = 0;
                let mut hibyte: BYTE = 0;
                let mut loword: UWORD = 0;
                let mut hiword: UWORD = 0;
                UninitTree();
                (*_lpCurCodeDesc).StreamIndex = (*_lpCurCodeDesc)
                    .StreamIndex
                    .wrapping_add(4 as DWORD);
                if (*_lpCurCodeDesc).StreamType as ::core::ffi::c_int
                    == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
                {
                    SeekResFile(
                        _Stream as *mut uio_Stream,
                        -((*_lpCurCodeDesc).StreamIndex as ::core::ffi::c_int)
                            as ::core::ffi::c_long,
                        1 as ::core::ffi::c_int,
                    );
                } else {
                    _Stream = _Stream.offset(-((*_lpCurCodeDesc).StreamIndex as isize));
                }
                loword = (*_lpCurCodeDesc).StreamLength as UWORD;
                lobyte = loword as BYTE;
                hibyte = (loword as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                    as BYTE;
                if _StreamType as ::core::ffi::c_int
                    == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
                {
                    PutResFileChar(
                        lobyte as ::core::ffi::c_char,
                        _Stream as *mut uio_Stream,
                    );
                } else {
                    let c2rust_fresh5 = _Stream;
                    _Stream = _Stream.offset(1);
                    *c2rust_fresh5 = lobyte;
                };
                if _StreamType as ::core::ffi::c_int
                    == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
                {
                    PutResFileChar(
                        hibyte as ::core::ffi::c_char,
                        _Stream as *mut uio_Stream,
                    );
                } else {
                    let c2rust_fresh6 = _Stream;
                    _Stream = _Stream.offset(1);
                    *c2rust_fresh6 = hibyte;
                };
                hiword = ((*_lpCurCodeDesc).StreamLength >> 16 as ::core::ffi::c_int)
                    as UWORD;
                lobyte = hiword as BYTE;
                hibyte = (hiword as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                    as BYTE;
                if _StreamType as ::core::ffi::c_int
                    == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
                {
                    PutResFileChar(
                        lobyte as ::core::ffi::c_char,
                        _Stream as *mut uio_Stream,
                    );
                } else {
                    let c2rust_fresh7 = _Stream;
                    _Stream = _Stream.offset(1);
                    *c2rust_fresh7 = lobyte;
                };
                if _StreamType as ::core::ffi::c_int
                    == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
                {
                    PutResFileChar(
                        hibyte as ::core::ffi::c_char,
                        _Stream as *mut uio_Stream,
                    );
                } else {
                    let c2rust_fresh8 = _Stream;
                    _Stream = _Stream.offset(1);
                    *c2rust_fresh8 = hibyte;
                };
                return;
            }
            s = (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD;
            r = (r as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD;
            InsertNode(r as SWORD);
        }
        if match_length as ::core::ffi::c_int > len as ::core::ffi::c_int {
            match_length = len;
        }
        if match_length as ::core::ffi::c_int <= 2 as ::core::ffi::c_int {
            match_length = 1 as UWORD;
            EncodeChar((*_lpCurCodeDesc).text_buf[r as usize] as UWORD);
        } else {
            EncodeChar(
                (255 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    + match_length as ::core::ffi::c_int) as UWORD,
            );
            EncodePosition(match_position);
        }
        last_match_length = match_length;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cwrite(
    mut buf: *const ::core::ffi::c_void,
    mut size: COUNT,
    mut count: COUNT,
    mut lpCodeDesc: PLZHCODE_DESC,
) -> COUNT {
    enum C2Rust_Block {
        S_131,
        EncodeExit,
        EncodeRestart,
    }
    let mut c2rust_current_block: C2Rust_Block;
    let mut r: UWORD = 0;
    let mut s: UWORD = 0;
    let mut last_match_length: UWORD = 0;
    let mut lpBuf: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut lpStr: *const BYTE = ::core::ptr::null::<BYTE>();
    _lpCurCodeDesc = lpCodeDesc;
    if _lpCurCodeDesc.is_null()
        || {
            size = (size as ::core::ffi::c_int * count as ::core::ffi::c_int) as COUNT;
            size as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        }
    {
        return 0 as COUNT;
    }
    _StreamType = (*lpCodeDesc).StreamType;
    _Stream = (*lpCodeDesc).Stream as *mut BYTE;
    _workbuf = (*lpCodeDesc).workbuf;
    _workbuflen = (*lpCodeDesc).workbuflen;
    lpStr = buf as *const BYTE;
    lpBuf = &raw mut (*lpCodeDesc).text_buf as *mut BYTE;
    r = (*lpCodeDesc).buf_index as UWORD;
    s = (*lpCodeDesc).restart_index as UWORD;
    last_match_length = (*lpCodeDesc).bytes_left as UWORD;
    if last_match_length != 0 {
        (*lpCodeDesc).StreamLength = (*lpCodeDesc)
            .StreamLength
            .wrapping_add(size as DWORD);
        c2rust_current_block = C2Rust_Block::EncodeRestart;
    } else if (*lpCodeDesc).StreamLength < 16 as DWORD {
        let mut i: UWORD = 0;
        i = (*lpCodeDesc).StreamLength as UWORD;
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if InitTree().0 == 0 {
                return 0 as COUNT;
            }
            (*_lpCurCodeDesc).StreamIndex = 0 as DWORD;
            (*lpCodeDesc).CleanupFunc = Some(
                _encode_cleanup as unsafe extern "C" fn() -> (),
            ) as PVOIDFUNC;
        }
        (*lpCodeDesc).StreamLength = (*lpCodeDesc)
            .StreamLength
            .wrapping_add(size as DWORD);
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            && size as ::core::ffi::c_int != 0
        {
            let c2rust_fresh0 = lpStr;
            lpStr = lpStr.offset(1);
            *lpBuf
                .offset((r as ::core::ffi::c_int + i as ::core::ffi::c_int) as isize) = *c2rust_fresh0;
            i = i.wrapping_add(1);
            size = size.wrapping_sub(1);
        }
        if (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            c2rust_current_block = C2Rust_Block::EncodeExit;
        } else {
            i = 1 as UWORD;
            while i as ::core::ffi::c_int <= 16 as ::core::ffi::c_int {
                InsertNode((r as ::core::ffi::c_int - i as ::core::ffi::c_int) as SWORD);
                i = i.wrapping_add(1);
            }
            InsertNode(r as SWORD);
            if size as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                c2rust_current_block = C2Rust_Block::EncodeExit;
            } else {
                c2rust_current_block = C2Rust_Block::S_131;
            }
        }
    } else {
        (*lpCodeDesc).StreamLength = (*lpCodeDesc)
            .StreamLength
            .wrapping_add(size as DWORD);
        c2rust_current_block = C2Rust_Block::S_131;
    }
    loop {
        match c2rust_current_block {
            C2Rust_Block::S_131 => {
                if match_length as ::core::ffi::c_int > 16 as ::core::ffi::c_int {
                    match_length = 16 as UWORD;
                }
                if match_length as ::core::ffi::c_int <= 2 as ::core::ffi::c_int {
                    match_length = 1 as UWORD;
                    EncodeChar(*lpBuf.offset(r as isize) as UWORD);
                } else {
                    EncodeChar(
                        (255 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + match_length as ::core::ffi::c_int) as UWORD,
                    );
                    EncodePosition(match_position);
                }
                last_match_length = match_length;
                c2rust_current_block = C2Rust_Block::EncodeRestart;
            }
            C2Rust_Block::EncodeRestart => {
                while last_match_length as ::core::ffi::c_int != 0
                    && size as ::core::ffi::c_int != 0
                {
                    let mut c: BYTE = 0;
                    size = size.wrapping_sub(1);
                    last_match_length = last_match_length.wrapping_sub(1);
                    DeleteNode(s as SWORD);
                    let c2rust_fresh1 = lpStr;
                    lpStr = lpStr.offset(1);
                    c = *c2rust_fresh1;
                    *lpBuf.offset(s as isize) = c;
                    if (s as ::core::ffi::c_int)
                        < 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    {
                        *lpBuf
                            .offset(
                                (s as ::core::ffi::c_int + 4096 as ::core::ffi::c_int)
                                    as isize,
                            ) = c;
                    }
                    s = (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD;
                    r = (r as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD;
                    InsertNode(r as SWORD);
                }
                if last_match_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    c2rust_current_block = C2Rust_Block::S_131;
                } else {
                    c2rust_current_block = C2Rust_Block::EncodeExit;
                }
            }
            C2Rust_Block::EncodeExit => {
                (*lpCodeDesc).buf_index = r as COUNT;
                break;
            }
        }
    }
    (*lpCodeDesc).restart_index = s as COUNT;
    (*lpCodeDesc).bytes_left = last_match_length as COUNT;
    (*lpCodeDesc).Stream = _Stream as *mut ::core::ffi::c_void;
    (*lpCodeDesc).workbuf = _workbuf;
    (*lpCodeDesc).workbuflen = _workbuflen;
    return count;
}
