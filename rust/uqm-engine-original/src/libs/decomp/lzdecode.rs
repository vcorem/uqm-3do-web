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

// Rust translation of src/libs/decomp/lzdecode.c.
extern "C" {
    pub type uio_Stream;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn _update(c: COUNT);
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn GetResFileChar(fp: *mut uio_Stream) -> ::core::ffi::c_int;
    fn PutResFileChar(
        ch: ::core::ffi::c_char,
        fp: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const STREAM_READ: Self = Self(0);
    pub const STREAM_WRITE: Self = Self(1);
}
pub type PLZHCODE_DESC = *mut LZHCODE_DESC;
pub type LZHCODE_DESC = _LZHCODE_DESC;
#[no_mangle]
pub static mut _lpCurCodeDesc: PLZHCODE_DESC = ::core::ptr::null_mut::<LZHCODE_DESC>();
#[no_mangle]
pub static mut _StreamType: STREAM_TYPE = 0;
#[no_mangle]
pub static mut _Stream: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
#[no_mangle]
pub static mut _workbuf: UWORD = 0;
#[no_mangle]
pub static mut _workbuflen: BYTE = 0;
unsafe extern "C" fn GetBit() -> SWORD {
    let mut i: SWORD = 0;
    while _workbuflen as ::core::ffi::c_int <= 8 as ::core::ffi::c_int {
        i = (if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            GetResFileChar(_Stream as *mut uio_Stream)
        } else {
            let c2rust_fresh12 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh12 as ::core::ffi::c_int
        }) as SWORD;
        if (i as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            i = 0 as SWORD;
        }
        _workbuf = (_workbuf as ::core::ffi::c_int
            | (i as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - _workbuflen as ::core::ffi::c_int) as UWORD;
        _workbuflen = (_workbuflen as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as BYTE;
    }
    i = ((_workbuf as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
        >> 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SWORD;
    _workbuf = ((_workbuf as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        & 0xffff as ::core::ffi::c_int) as UWORD;
    _workbuflen = _workbuflen.wrapping_sub(1);
    return i;
}
unsafe extern "C" fn GetBits(mut num_bits: BYTE) -> UWORD {
    let mut i: SWORD = 0;
    while _workbuflen as ::core::ffi::c_int <= 8 as ::core::ffi::c_int {
        i = (if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            GetResFileChar(_Stream as *mut uio_Stream)
        } else {
            let c2rust_fresh11 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh11 as ::core::ffi::c_int
        }) as SWORD;
        if (i as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            i = 0 as SWORD;
        }
        _workbuf = (_workbuf as ::core::ffi::c_int
            | (i as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - _workbuflen as ::core::ffi::c_int) as UWORD;
        _workbuflen = (_workbuflen as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as BYTE;
    }
    i = ((_workbuf as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
        >> 16 as ::core::ffi::c_int - num_bits as ::core::ffi::c_int) as SWORD;
    _workbuf = ((_workbuf as ::core::ffi::c_int) << num_bits as ::core::ffi::c_int
        & 0xffff as ::core::ffi::c_int) as UWORD;
    _workbuflen = (_workbuflen as ::core::ffi::c_int - num_bits as ::core::ffi::c_int)
        as BYTE;
    return i as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn StartHuff() {
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < 256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 16 as ::core::ffi::c_int
    {
        (*_lpCurCodeDesc).freq[i as usize] = 1 as COUNT;
        (*_lpCurCodeDesc).son[i as usize] = (i as ::core::ffi::c_int
            + ((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)) as COUNT;
        (*_lpCurCodeDesc)
            .prnt[(i as ::core::ffi::c_int
            + ((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)) as usize] = i;
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    j = (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as COUNT;
    while j as ::core::ffi::c_int
        <= (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        (*_lpCurCodeDesc).freq[j as usize] = ((*_lpCurCodeDesc).freq[i as usize]
            as ::core::ffi::c_int
            + (*_lpCurCodeDesc)
                .freq[(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int) as COUNT;
        (*_lpCurCodeDesc).son[j as usize] = i;
        (*_lpCurCodeDesc)
            .prnt[(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] = j;
        (*_lpCurCodeDesc).prnt[i as usize] = (*_lpCurCodeDesc)
            .prnt[(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
        i = (i as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT;
        j = j.wrapping_add(1);
    }
    (*_lpCurCodeDesc)
        .freq[((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as usize] = 0xffff as COUNT;
    (*_lpCurCodeDesc)
        .prnt[((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as usize] = 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn copen(
    mut InStream: *mut ::core::ffi::c_void,
    mut SType: STREAM_TYPE,
    mut SMode: STREAM_MODE,
) -> DECODE_REF {
    let mut StreamLength: DWORD = 0;
    _StreamType = SType;
    _Stream = InStream as *mut BYTE;
    if SMode as ::core::ffi::c_int
        == C2Rust_Unnamed_0::STREAM_WRITE.0 as ::core::ffi::c_int
    {
        if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            PutResFileChar(0 as ::core::ffi::c_char, _Stream as *mut uio_Stream);
        } else {
            let c2rust_fresh0 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh0 = 0 as ::core::ffi::c_int as BYTE;
        };
        if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            PutResFileChar(0 as ::core::ffi::c_char, _Stream as *mut uio_Stream);
        } else {
            let c2rust_fresh1 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh1 = 0 as ::core::ffi::c_int as BYTE;
        };
        if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            PutResFileChar(0 as ::core::ffi::c_char, _Stream as *mut uio_Stream);
        } else {
            let c2rust_fresh2 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh2 = 0 as ::core::ffi::c_int as BYTE;
        };
        if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            PutResFileChar(0 as ::core::ffi::c_char, _Stream as *mut uio_Stream);
        } else {
            let c2rust_fresh3 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh3 = 0 as ::core::ffi::c_int as BYTE;
        };
        StreamLength = 0 as DWORD;
    } else {
        let mut lobyte: BYTE = 0;
        let mut hibyte: BYTE = 0;
        let mut loword: UWORD = 0;
        let mut hiword: UWORD = 0;
        lobyte = (if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            GetResFileChar(_Stream as *mut uio_Stream)
        } else {
            let c2rust_fresh4 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh4 as ::core::ffi::c_int
        }) as BYTE;
        hibyte = (if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            GetResFileChar(_Stream as *mut uio_Stream)
        } else {
            let c2rust_fresh5 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh5 as ::core::ffi::c_int
        }) as BYTE;
        loword = (((hibyte as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int | lobyte as ::core::ffi::c_int) as UWORD;
        lobyte = (if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            GetResFileChar(_Stream as *mut uio_Stream)
        } else {
            let c2rust_fresh6 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh6 as ::core::ffi::c_int
        }) as BYTE;
        hibyte = (if _StreamType as ::core::ffi::c_int
            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
        {
            GetResFileChar(_Stream as *mut uio_Stream)
        } else {
            let c2rust_fresh7 = _Stream;
            _Stream = _Stream.offset(1);
            *c2rust_fresh7 as ::core::ffi::c_int
        }) as BYTE;
        hiword = (((hibyte as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int | lobyte as ::core::ffi::c_int) as UWORD;
        StreamLength = (hiword as DWORD) << 16 as ::core::ffi::c_int | loword as DWORD;
    }
    if StreamLength == 0xffffffff as ::core::ffi::c_uint
        || {
            _lpCurCodeDesc = HCalloc(::core::mem::size_of::<LZHCODE_DESC>())
                as PLZHCODE_DESC;
            _lpCurCodeDesc.is_null()
        }
    {
        HFree(_lpCurCodeDesc as *mut ::core::ffi::c_void);
        _lpCurCodeDesc = ::core::ptr::null_mut::<LZHCODE_DESC>();
    } else {
        (*_lpCurCodeDesc).Stream = _Stream as *mut ::core::ffi::c_void;
        (*_lpCurCodeDesc).StreamType = _StreamType;
        (*_lpCurCodeDesc).StreamMode = SMode;
        (*_lpCurCodeDesc).StreamLength = StreamLength;
        (*_lpCurCodeDesc).buf_index = (4096 as ::core::ffi::c_int
            - 16 as ::core::ffi::c_int) as COUNT;
        memset(
            (&raw mut (*_lpCurCodeDesc).text_buf as *mut BYTE).offset(0isize)
                as *mut ::core::ffi::c_void,
            ' ' as ::core::ffi::c_int,
            (4096 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as size_t,
        );
        StartHuff();
    }
    return _lpCurCodeDesc;
}
#[no_mangle]
pub unsafe extern "C" fn cclose(mut lpCodeDesc: PLZHCODE_DESC) -> DWORD {
    _lpCurCodeDesc = lpCodeDesc;
    if !_lpCurCodeDesc.is_null() {
        let mut StreamIndex: DWORD = 0;
        if (*_lpCurCodeDesc).CleanupFunc.is_some() {
            Some((*_lpCurCodeDesc).CleanupFunc.expect("non-null function pointer"))
                .expect("non-null function pointer")();
        }
        StreamIndex = (*lpCodeDesc).StreamIndex;
        HFree(lpCodeDesc as *mut ::core::ffi::c_void);
        _lpCurCodeDesc = ::core::ptr::null_mut::<LZHCODE_DESC>();
        return StreamIndex;
    }
    return 0 as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn cfilelength(
    mut lpCodeDesc: PLZHCODE_DESC,
    mut pfilelen: *mut DWORD,
) {
    if lpCodeDesc.is_null() {
        *pfilelen = 0 as DWORD;
    } else {
        *pfilelen = (*lpCodeDesc).StreamLength;
    };
}
static mut d_code: [BYTE; 256] = [
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
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
    0x9 as BYTE,
    0x9 as BYTE,
    0x9 as BYTE,
    0x9 as BYTE,
    0x9 as BYTE,
    0x9 as BYTE,
    0x9 as BYTE,
    0x9 as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xa as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xb as BYTE,
    0xc as BYTE,
    0xc as BYTE,
    0xc as BYTE,
    0xc as BYTE,
    0xd as BYTE,
    0xd as BYTE,
    0xd as BYTE,
    0xd as BYTE,
    0xe as BYTE,
    0xe as BYTE,
    0xe as BYTE,
    0xe as BYTE,
    0xf as BYTE,
    0xf as BYTE,
    0xf as BYTE,
    0xf as BYTE,
    0x10 as BYTE,
    0x10 as BYTE,
    0x10 as BYTE,
    0x10 as BYTE,
    0x11 as BYTE,
    0x11 as BYTE,
    0x11 as BYTE,
    0x11 as BYTE,
    0x12 as BYTE,
    0x12 as BYTE,
    0x12 as BYTE,
    0x12 as BYTE,
    0x13 as BYTE,
    0x13 as BYTE,
    0x13 as BYTE,
    0x13 as BYTE,
    0x14 as BYTE,
    0x14 as BYTE,
    0x14 as BYTE,
    0x14 as BYTE,
    0x15 as BYTE,
    0x15 as BYTE,
    0x15 as BYTE,
    0x15 as BYTE,
    0x16 as BYTE,
    0x16 as BYTE,
    0x16 as BYTE,
    0x16 as BYTE,
    0x17 as BYTE,
    0x17 as BYTE,
    0x17 as BYTE,
    0x17 as BYTE,
    0x18 as BYTE,
    0x18 as BYTE,
    0x19 as BYTE,
    0x19 as BYTE,
    0x1a as BYTE,
    0x1a as BYTE,
    0x1b as BYTE,
    0x1b as BYTE,
    0x1c as BYTE,
    0x1c as BYTE,
    0x1d as BYTE,
    0x1d as BYTE,
    0x1e as BYTE,
    0x1e as BYTE,
    0x1f as BYTE,
    0x1f as BYTE,
    0x20 as BYTE,
    0x20 as BYTE,
    0x21 as BYTE,
    0x21 as BYTE,
    0x22 as BYTE,
    0x22 as BYTE,
    0x23 as BYTE,
    0x23 as BYTE,
    0x24 as BYTE,
    0x24 as BYTE,
    0x25 as BYTE,
    0x25 as BYTE,
    0x26 as BYTE,
    0x26 as BYTE,
    0x27 as BYTE,
    0x27 as BYTE,
    0x28 as BYTE,
    0x28 as BYTE,
    0x29 as BYTE,
    0x29 as BYTE,
    0x2a as BYTE,
    0x2a as BYTE,
    0x2b as BYTE,
    0x2b as BYTE,
    0x2c as BYTE,
    0x2c as BYTE,
    0x2d as BYTE,
    0x2d as BYTE,
    0x2e as BYTE,
    0x2e as BYTE,
    0x2f as BYTE,
    0x2f as BYTE,
    0x30 as BYTE,
    0x31 as BYTE,
    0x32 as BYTE,
    0x33 as BYTE,
    0x34 as BYTE,
    0x35 as BYTE,
    0x36 as BYTE,
    0x37 as BYTE,
    0x38 as BYTE,
    0x39 as BYTE,
    0x3a as BYTE,
    0x3b as BYTE,
    0x3c as BYTE,
    0x3d as BYTE,
    0x3e as BYTE,
    0x3f as BYTE,
];
static mut d_len: [BYTE; 256] = [
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x1 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x2 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x3 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
    0x4 as BYTE,
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
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
    0x5 as BYTE,
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
    0x6 as BYTE,
    0x6 as BYTE,
    0x6 as BYTE,
    0x6 as BYTE,
];
#[no_mangle]
pub unsafe extern "C" fn cread(
    mut buf: *mut ::core::ffi::c_void,
    mut size: COUNT,
    mut count: COUNT,
    mut lpCodeDesc: PLZHCODE_DESC,
) -> COUNT {
    let mut c: COUNT = 0;
    let mut copy_size: COUNT = 0;
    enum C2Rust_Block {
        S_80,
        ReenterRun,
    }
    let mut c2rust_current_block: C2Rust_Block;
    let mut r: COUNT = 0;
    let mut j: COUNT = 0;
    let mut i: COUNT = 0;
    let mut lpStr: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    _lpCurCodeDesc = lpCodeDesc;
    if _lpCurCodeDesc.is_null() {
        return 0 as COUNT;
    }
    size = (size as ::core::ffi::c_int * count as ::core::ffi::c_int) as COUNT;
    if (*lpCodeDesc).StreamIndex.wrapping_add(size as DWORD) > (*lpCodeDesc).StreamLength
    {
        size = (size as ::core::ffi::c_int / count as ::core::ffi::c_int) as COUNT;
        count = (*lpCodeDesc)
            .StreamLength
            .wrapping_sub((*lpCodeDesc).StreamIndex)
            .wrapping_div(size as DWORD) as COUNT;
        size = (size as ::core::ffi::c_int * count as ::core::ffi::c_int) as COUNT;
    }
    if size as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as COUNT;
    }
    lpStr = buf as *mut BYTE;
    _StreamType = (*lpCodeDesc).StreamType;
    _Stream = (*lpCodeDesc).Stream as *mut BYTE;
    _workbuf = (*lpCodeDesc).workbuf;
    _workbuflen = (*lpCodeDesc).workbuflen;
    (*lpCodeDesc).StreamIndex = (*lpCodeDesc).StreamIndex.wrapping_add(size as DWORD);
    r = (*lpCodeDesc).buf_index;
    j = (*lpCodeDesc).bytes_left;
    if j != 0 {
        (*lpCodeDesc).bytes_left = 0 as COUNT;
        i = (*lpCodeDesc).restart_index;
        c2rust_current_block = C2Rust_Block::ReenterRun;
    } else {
        c2rust_current_block = C2Rust_Block::S_80;
    }
    loop {
        match c2rust_current_block {
            C2Rust_Block::S_80 => {
                c = 0;
                c = (*lpCodeDesc)
                    .son[((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
                while (c as ::core::ffi::c_int)
                    < (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                {
                    c = (*lpCodeDesc)
                        .son[(c as ::core::ffi::c_int + GetBit() as ::core::ffi::c_int)
                        as usize];
                }
                _update(c);
                c = (c as ::core::ffi::c_int
                    - ((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)) as COUNT;
                if (c as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                    size = size.wrapping_sub(1);
                    let c2rust_fresh8 = r;
                    r = r.wrapping_add(1);
                    let c2rust_lvalue_ptr = &raw mut (*lpCodeDesc)
                        .text_buf[(c2rust_fresh8 as ::core::ffi::c_int
                        & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as usize];
                    *c2rust_lvalue_ptr = c as BYTE;
                    let c2rust_fresh9 = lpStr;
                    lpStr = lpStr.offset(1);
                    *c2rust_fresh9 = *c2rust_lvalue_ptr;
                } else {
                    copy_size = 0;
                    while _workbuflen as ::core::ffi::c_int <= 8 as ::core::ffi::c_int {
                        i = (if _StreamType as ::core::ffi::c_int
                            == C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int
                        {
                            GetResFileChar(_Stream as *mut uio_Stream)
                        } else {
                            let c2rust_fresh10 = _Stream;
                            _Stream = _Stream.offset(1);
                            *c2rust_fresh10 as ::core::ffi::c_int
                        }) as COUNT;
                        _workbuf = (_workbuf as ::core::ffi::c_int
                            | (i as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                    - _workbuflen as ::core::ffi::c_int) as UWORD;
                        _workbuflen = (_workbuflen as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int) as BYTE;
                    }
                    i = (_workbuf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                        as BYTE as COUNT;
                    _workbuf = ((_workbuf as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
                        as UWORD;
                    _workbuflen = (_workbuflen as ::core::ffi::c_int
                        - 8 as ::core::ffi::c_int) as BYTE;
                    j = d_len[i as usize] as COUNT;
                    i = ((d_code[i as usize] as UWORD as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int
                        | ((i as ::core::ffi::c_int) << j as ::core::ffi::c_int
                            | GetBits(j as BYTE) as ::core::ffi::c_int)
                            & 0x3f as ::core::ffi::c_int) as COUNT;
                    i = (r as ::core::ffi::c_int - i as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT;
                    j = (c as ::core::ffi::c_int - 255 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as COUNT;
                    c2rust_current_block = C2Rust_Block::ReenterRun;
                    continue;
                }
            }
            C2Rust_Block::ReenterRun => {
                if j as ::core::ffi::c_int > size as ::core::ffi::c_int {
                    (*lpCodeDesc).bytes_left = (j as ::core::ffi::c_int
                        - size as ::core::ffi::c_int) as COUNT;
                    (*lpCodeDesc).restart_index = (i as ::core::ffi::c_int
                        + size as ::core::ffi::c_int) as COUNT;
                    j = size;
                }
                size = (size as ::core::ffi::c_int - j as ::core::ffi::c_int) as COUNT;
                loop {
                    let mut loc_size: COUNT = 0;
                    i = (i as ::core::ffi::c_int
                        & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
                    r = (r as ::core::ffi::c_int
                        & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
                    if (i as ::core::ffi::c_int) < r as ::core::ffi::c_int
                        && i as ::core::ffi::c_int + j as ::core::ffi::c_int
                            > r as ::core::ffi::c_int
                        || i as ::core::ffi::c_int > r as ::core::ffi::c_int
                            && i as ::core::ffi::c_int + j as ::core::ffi::c_int
                                > r as ::core::ffi::c_int + 4096 as ::core::ffi::c_int
                    {
                        copy_size = (r as ::core::ffi::c_int - i as ::core::ffi::c_int
                            & 4096 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as COUNT;
                    } else {
                        copy_size = j;
                        if copy_size as ::core::ffi::c_int > 4096 as ::core::ffi::c_int {
                            copy_size = 4096 as COUNT;
                        }
                    }
                    loc_size = copy_size;
                    if i as ::core::ffi::c_int + loc_size as ::core::ffi::c_int
                        > 4096 as ::core::ffi::c_int
                    {
                        let mut k: COUNT = 0;
                        k = (4096 as ::core::ffi::c_int - i as ::core::ffi::c_int)
                            as COUNT;
                        memcpy(
                            lpStr as *mut ::core::ffi::c_void,
                            (&raw mut (*lpCodeDesc).text_buf as *mut BYTE)
                                .offset(i as isize) as *const ::core::ffi::c_void,
                            k as size_t,
                        );
                        lpStr = lpStr.offset(k as ::core::ffi::c_int as isize);
                        loc_size = (loc_size as ::core::ffi::c_int
                            - k as ::core::ffi::c_int) as COUNT;
                        i = 0 as COUNT;
                    }
                    memcpy(
                        lpStr as *mut ::core::ffi::c_void,
                        (&raw mut (*lpCodeDesc).text_buf as *mut BYTE).offset(i as isize)
                            as *const ::core::ffi::c_void,
                        loc_size as size_t,
                    );
                    lpStr = lpStr.offset(loc_size as ::core::ffi::c_int as isize);
                    i = (i as ::core::ffi::c_int + loc_size as ::core::ffi::c_int)
                        as COUNT;
                    lpStr = lpStr.offset(-(copy_size as ::core::ffi::c_int as isize));
                    loc_size = copy_size;
                    if r as ::core::ffi::c_int + loc_size as ::core::ffi::c_int
                        > 4096 as ::core::ffi::c_int
                    {
                        let mut k_0: COUNT = 0;
                        k_0 = (4096 as ::core::ffi::c_int - r as ::core::ffi::c_int)
                            as COUNT;
                        memcpy(
                            (&raw mut (*lpCodeDesc).text_buf as *mut BYTE)
                                .offset(r as isize) as *mut ::core::ffi::c_void,
                            lpStr as *const ::core::ffi::c_void,
                            k_0 as size_t,
                        );
                        lpStr = lpStr.offset(k_0 as ::core::ffi::c_int as isize);
                        loc_size = (loc_size as ::core::ffi::c_int
                            - k_0 as ::core::ffi::c_int) as COUNT;
                        r = 0 as COUNT;
                    }
                    memcpy(
                        (&raw mut (*lpCodeDesc).text_buf as *mut BYTE).offset(r as isize)
                            as *mut ::core::ffi::c_void,
                        lpStr as *const ::core::ffi::c_void,
                        loc_size as size_t,
                    );
                    lpStr = lpStr.offset(loc_size as ::core::ffi::c_int as isize);
                    r = (r as ::core::ffi::c_int + loc_size as ::core::ffi::c_int)
                        as COUNT;
                    j = (j as ::core::ffi::c_int - copy_size as ::core::ffi::c_int)
                        as COUNT;
                    if j == 0 {
                        break;
                    }
                }
            }
        }
        if size != 0 {
            c2rust_current_block = C2Rust_Block::S_80;
        } else {
            break;
        }
    }
    (*lpCodeDesc).buf_index = r;
    (*lpCodeDesc).Stream = _Stream as *mut ::core::ffi::c_void;
    (*lpCodeDesc).workbuf = _workbuf;
    (*lpCodeDesc).workbuflen = _workbuflen;
    return count;
}
