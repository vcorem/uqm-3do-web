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

/* DUCK video player
 *
 * Status: fully functional
 */

// Rust translation of src/libs/video/dukvid.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fread(
        buf: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_fseek(
        stream: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut resolutionFactor: ::core::ffi::c_uint;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type size_t = usize;
pub type TFB_VideoDecoder = tfb_videodecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videodecoder {
    pub funcs: *const TFB_VideoDecoderFuncs,
    pub format: *const TFB_PixelFormat,
    pub w: uint32,
    pub h: uint32,
    pub length: ::core::ffi::c_float,
    pub frame_count: uint32,
    pub interframe_wait: uint32,
    pub audio_synced: bool,
    pub callbacks: TFB_VideoCallbacks,
    pub looping: bool,
    pub data: *mut ::core::ffi::c_void,
    pub error: sint32,
    pub pos: ::core::ffi::c_float,
    pub cur_frame: uint32,
    pub dir: *mut uio_DirHandle,
    pub filename: *mut ::core::ffi::c_char,
}
pub type TFB_VideoCallbacks = tfb_videocallbacks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videocallbacks {
    pub BeginFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub EndFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub GetCanvasLine: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> *mut ::core::ffi::c_void,
    >,
    pub GetTicks: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> uint32>,
    pub SetTimer: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> bool>,
}
pub type TFB_PixelFormat = tfb_pixelformat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_pixelformat {
    pub BitsPerPixel: ::core::ffi::c_int,
    pub BytesPerPixel: ::core::ffi::c_int,
    pub Rmask: DWORD,
    pub Gmask: DWORD,
    pub Bmask: DWORD,
    pub Amask: DWORD,
    pub Rshift: DWORD,
    pub Gshift: DWORD,
    pub Bshift: DWORD,
    pub Ashift: DWORD,
    pub Rloss: DWORD,
    pub Gloss: DWORD,
    pub Bloss: DWORD,
    pub Aloss: DWORD,
}
pub type TFB_VideoDecoderFuncs = tfb_videodecoderfunc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videodecoderfunc {
    pub GetName: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub InitModule: Option<unsafe extern "C" fn(::core::ffi::c_int) -> bool>,
    pub TermModule: Option<unsafe extern "C" fn() -> ()>,
    pub GetStructSize: Option<unsafe extern "C" fn() -> uint32>,
    pub GetError: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_int,
    >,
    pub Init: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder, *mut TFB_PixelFormat) -> bool,
    >,
    pub Term: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub Open: Option<
        unsafe extern "C" fn(
            *mut TFB_VideoDecoder,
            *mut uio_DirHandle,
            *const ::core::ffi::c_char,
        ) -> bool,
    >,
    pub Close: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub DecodeNext: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_int,
    >,
    pub SeekFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> uint32>,
    pub SeekTime: Option<
        unsafe extern "C" fn(
            *mut TFB_VideoDecoder,
            ::core::ffi::c_float,
        ) -> ::core::ffi::c_float,
    >,
    pub GetFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> uint32>,
    pub GetTime: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_float,
    >,
}
pub type TFB_DuckVideoDecoder = tfb_duckvideodecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_duckvideodecoder {
    pub decoder: TFB_VideoDecoder,
    pub last_error: sint32,
    pub basedir: *mut uio_DirHandle,
    pub basename: *mut ::core::ffi::c_char,
    pub stream: *mut uio_Stream,
    pub frames: *mut uint32,
    pub cframes: uint32,
    pub iframe: uint32,
    pub version: uint32,
    pub wb: uint32,
    pub hb: uint32,
    pub d: TFB_DuckVideoDeltas,
    pub inbuf: *mut uint8,
    pub decbuf: *mut uint32,
}
pub type TFB_DuckVideoDeltas = tfb_duckvideodeltas;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_duckvideodeltas {
    pub lumas: [[sint32; 16]; 256],
    pub chromas: [[sint32; 16]; 256],
}
pub type TFB_DuckVideoHeader = tfb_duckvideoheader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_duckvideoheader {
    pub version: uint32,
    pub scrn_x_ofs: uint32,
    pub scrn_y_ofs: uint32,
    pub wb: uint16,
    pub hb: uint16,
    pub lumas: [sint16; 8],
    pub chromas: [sint16; 8],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const dukve_None: Self = Self(0);
    pub const dukve_Unknown: Self = Self(-1);
    pub const dukve_BadFile: Self = Self(-2);
    pub const dukve_BadArg: Self = Self(-3);
    pub const dukve_OutOfBuf: Self = Self(-4);
    pub const dukve_EOF: Self = Self(-5);
    pub const dukve_Other: Self = Self(-1000);
}
#[no_mangle]
pub static mut dukv_DecoderVtbl: TFB_VideoDecoderFuncs = tfb_videodecoderfunc {
    GetName: Some(dukv_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        dukv_InitModule as unsafe extern "C" fn(::core::ffi::c_int) -> bool,
    ),
    TermModule: Some(dukv_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(dukv_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        dukv_GetError
            as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(
        dukv_Init
            as unsafe extern "C" fn(*mut TFB_VideoDecoder, *mut TFB_PixelFormat) -> bool,
    ),
    Term: Some(dukv_Term as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()),
    Open: Some(
        dukv_Open
            as unsafe extern "C" fn(
                *mut TFB_VideoDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(dukv_Close as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()),
    DecodeNext: Some(
        dukv_DecodeNext
            as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_int,
    ),
    SeekFrame: Some(
        dukv_SeekFrame as unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> uint32,
    ),
    SeekTime: Some(
        dukv_SeekTime
            as unsafe extern "C" fn(
                *mut TFB_VideoDecoder,
                ::core::ffi::c_float,
            ) -> ::core::ffi::c_float,
    ),
    GetFrame: Some(
        dukv_GetFrame as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> uint32,
    ),
    GetTime: Some(
        dukv_GetTime
            as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_float,
    ),
};
unsafe extern "C" fn dukv_DecodeFrame(
    mut src_p: *mut uint8,
    mut dst_p: *mut uint32,
    mut wb: uint32,
    mut hb: uint32,
    mut deltas: *mut TFB_DuckVideoDeltas,
) {
    let mut iVec: ::core::ffi::c_int = 0;
    let mut iSeq: ::core::ffi::c_int = 0;
    let mut x: uint32 = 0;
    let mut y: uint32 = 0;
    let mut w: sint32 = 0;
    let mut d_p0: *mut uint32 = ::core::ptr::null_mut::<uint32>();
    let mut d_p1: *mut uint32 = ::core::ptr::null_mut::<uint32>();
    let mut i: ::core::ffi::c_int = 0;
    w = wb.wrapping_mul(4 as uint32) as sint32;
    let c2rust_fresh0 = src_p;
    src_p = src_p.offset(1);
    iVec = *c2rust_fresh0 as ::core::ffi::c_int;
    iSeq = 0 as ::core::ffi::c_int;
    y = 0 as uint32;
    while y < hb {
        let mut accum0: sint32 = 0;
        let mut accum1: sint32 = 0;
        let mut corr: sint32 = 0;
        let mut corr0: sint32 = 0;
        let mut corr1: sint32 = 0;
        let mut delta: sint32 = 0;
        let mut pix: [sint32; 4] = [0; 4];
        d_p0 = dst_p
            .offset(y.wrapping_mul(w as uint32).wrapping_mul(2 as uint32) as isize);
        d_p1 = d_p0.offset(w as isize);
        accum0 = 0 as ::core::ffi::c_int as sint32;
        accum1 = 0 as ::core::ffi::c_int as sint32;
        corr0 = 0 as ::core::ffi::c_int as sint32;
        corr1 = 0 as ::core::ffi::c_int as sint32;
        x = 0 as uint32;
        while x < wb {
            if y == 0 as uint32 {
                pix[3usize] = 0 as ::core::ffi::c_int as sint32;
                pix[2usize] = pix[3usize];
                pix[1usize] = pix[2usize];
                pix[0usize] = pix[1usize];
            } else {
                let mut p_p: *mut uint32 = d_p0.offset(-(w as isize));
                pix[0usize] = *p_p.offset(0isize) as sint32;
                pix[1usize] = *p_p.offset(1isize) as sint32;
                pix[2usize] = *p_p.offset(2isize) as sint32;
                pix[3usize] = *p_p.offset(3isize) as sint32;
            }
            let c2rust_fresh1 = iSeq;
            iSeq += 1;
            delta = (*deltas).chromas[iVec as usize][c2rust_fresh1 as usize];
            iSeq += 1;
            accum0 += delta >> 1 as ::core::ffi::c_int;
            if delta & 1 as ::core::ffi::c_int != 0 {
                let c2rust_fresh2 = src_p;
                src_p = src_p.offset(1);
                iVec = *c2rust_fresh2 as ::core::ffi::c_int;
                iSeq = 0 as ::core::ffi::c_int;
            }
            i = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                let c2rust_fresh3 = iSeq;
                iSeq += 1;
                delta = (*deltas).lumas[iVec as usize][c2rust_fresh3 as usize];
                let c2rust_fresh4 = iSeq;
                iSeq += 1;
                corr = (*deltas).lumas[iVec as usize][c2rust_fresh4 as usize];
                accum0 += delta >> 1 as ::core::ffi::c_int;
                corr0 ^= corr;
                pix[i as usize] += accum0;
                pix[i as usize] ^= corr0;
                if delta & 1 as ::core::ffi::c_int != 0 {
                    let c2rust_fresh5 = src_p;
                    src_p = src_p.offset(1);
                    iVec = *c2rust_fresh5 as ::core::ffi::c_int;
                    iSeq = 0 as ::core::ffi::c_int;
                }
                *d_p0 = pix[i as usize] as uint32;
                i += 1;
                d_p0 = d_p0.offset(1);
            }
            i = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                let c2rust_fresh6 = iSeq;
                iSeq += 1;
                delta = (*deltas).lumas[iVec as usize][c2rust_fresh6 as usize];
                let c2rust_fresh7 = iSeq;
                iSeq += 1;
                corr = (*deltas).lumas[iVec as usize][c2rust_fresh7 as usize];
                accum1 += delta >> 1 as ::core::ffi::c_int;
                corr1 ^= corr;
                pix[i as usize] += accum1;
                pix[i as usize] ^= corr1;
                if delta & 1 as ::core::ffi::c_int != 0 {
                    let c2rust_fresh8 = src_p;
                    src_p = src_p.offset(1);
                    iVec = *c2rust_fresh8 as ::core::ffi::c_int;
                    iSeq = 0 as ::core::ffi::c_int;
                }
                *d_p1 = pix[i as usize] as uint32;
                i += 1;
                d_p1 = d_p1.offset(1);
            }
            x = x.wrapping_add(1);
        }
        y = y.wrapping_add(1);
    }
}
unsafe extern "C" fn dukv_DecodeFrameV3(
    mut src_p: *mut uint8,
    mut dst_p: *mut uint32,
    mut wb: uint32,
    mut hb: uint32,
    mut deltas: *mut TFB_DuckVideoDeltas,
) {
    let mut iVec: ::core::ffi::c_int = 0;
    let mut iSeq: ::core::ffi::c_int = 0;
    let mut x: uint32 = 0;
    let mut y: uint32 = 0;
    let mut w: sint32 = 0;
    let mut d_p: *mut uint32 = ::core::ptr::null_mut::<uint32>();
    let mut i: ::core::ffi::c_int = 0;
    let c2rust_fresh9 = src_p;
    src_p = src_p.offset(1);
    iVec = *c2rust_fresh9 as ::core::ffi::c_int;
    iSeq = 0 as ::core::ffi::c_int;
    hb = hb.wrapping_mul(2 as uint32);
    w = wb.wrapping_mul(4 as uint32) as sint32;
    y = 0 as uint32;
    while y < hb {
        let mut accum: sint32 = 0;
        let mut delta: sint32 = 0;
        let mut pix: sint32 = 0;
        d_p = dst_p.offset(y.wrapping_mul(w as uint32) as isize);
        accum = 0 as ::core::ffi::c_int as sint32;
        x = 0 as uint32;
        while x < wb {
            delta = (*deltas).chromas[iVec as usize][iSeq as usize];
            iSeq += 2 as ::core::ffi::c_int;
            accum += delta >> 1 as ::core::ffi::c_int;
            if delta & 1 as ::core::ffi::c_int != 0 {
                let c2rust_fresh10 = src_p;
                src_p = src_p.offset(1);
                iVec = *c2rust_fresh10 as ::core::ffi::c_int;
                iSeq = 0 as ::core::ffi::c_int;
            }
            i = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                if y == 0 as uint32 {
                    pix = 0 as ::core::ffi::c_int as sint32;
                } else {
                    pix = *d_p.offset(-w as isize) as sint32;
                }
                delta = (*deltas).lumas[iVec as usize][iSeq as usize];
                iSeq += 2 as ::core::ffi::c_int;
                accum += delta >> 1 as ::core::ffi::c_int;
                pix += accum;
                if delta & 1 as ::core::ffi::c_int != 0 {
                    let c2rust_fresh11 = src_p;
                    src_p = src_p.offset(1);
                    iVec = *c2rust_fresh11 as ::core::ffi::c_int;
                    iSeq = 0 as ::core::ffi::c_int;
                }
                *d_p = pix as uint32;
                i += 1;
                d_p = d_p.offset(1);
            }
            x = x.wrapping_add(1);
        }
        y = y.wrapping_add(1);
    }
}
unsafe extern "C" fn dukv_OpenStream(mut dukv: *mut TFB_DuckVideoDecoder) -> bool {
    let mut filename: [::core::ffi::c_char; 280] = [0; 280];
    strcat(
        strcpy(&raw mut filename as *mut ::core::ffi::c_char, (*dukv).basename),
        b".duk\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*dukv).stream = uqm_lua_checked!(uio_fopen(
        (*dukv).basedir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    return !(*dukv).stream.is_null();
}
unsafe extern "C" fn dukv_ReadFrames(mut dukv: *mut TFB_DuckVideoDecoder) -> bool {
    let mut filename: [::core::ffi::c_char; 280] = [0; 280];
    let mut i: uint32 = 0;
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    strcat(
        strcpy(&raw mut filename as *mut ::core::ffi::c_char, (*dukv).basename),
        b".frm\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fp = uqm_lua_checked!(uio_fopen(
        (*dukv).basedir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if fp.is_null() {
        return false;
    }
    uqm_lua_checked!(uio_fseek(fp, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int), false);
    (*dukv).cframes = (uqm_lua_checked!(uio_ftell(fp), false) as usize)
        .wrapping_div(::core::mem::size_of::<uint32>()) as uint32;
    uqm_lua_checked!(uio_fseek(fp, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), false);
    (*dukv).frames = HMalloc(
        ((*dukv).cframes as size_t).wrapping_mul(::core::mem::size_of::<uint32>()),
    ) as *mut uint32;
    if uqm_lua_checked!(uio_fread(
        (*dukv).frames as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<uint32>(),
        (*dukv).cframes as size_t,
        fp,
    ), false) != (*dukv).cframes as size_t
    {
        HFree((*dukv).frames as *mut ::core::ffi::c_void);
        (*dukv).frames = ::core::ptr::null_mut::<uint32>();
        return false;
    }
    uqm_lua_checked!(uio_fclose(fp), false);
    i = 0 as uint32;
    while i < (*dukv).cframes {
        *(*dukv).frames.offset(i as isize) = UQM_Swap32(
            *(*dukv).frames.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn dukv_ReadVectors(
    mut dukv: *mut TFB_DuckVideoDecoder,
    mut vectors: *mut uint8,
) -> bool {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut filename: [::core::ffi::c_char; 280] = [0; 280];
    let mut ret: ::core::ffi::c_int = 0;
    strcat(
        strcpy(&raw mut filename as *mut ::core::ffi::c_char, (*dukv).basename),
        b".tbl\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fp = uqm_lua_checked!(uio_fopen(
        (*dukv).basedir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if fp.is_null() {
        return false;
    }
    ret = uqm_lua_checked!(uio_fread(
        vectors as *mut ::core::ffi::c_void,
        0x10 as size_t,
        0x100 as size_t,
        fp,
    ), false) as ::core::ffi::c_int;
    uqm_lua_checked!(uio_fclose(fp), false);
    return ret == 0x100 as ::core::ffi::c_int;
}
unsafe extern "C" fn dukv_ReadHeader(
    mut dukv: *mut TFB_DuckVideoDecoder,
    mut pl: *mut sint32,
    mut pc: *mut sint32,
) -> bool {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut filename: [::core::ffi::c_char; 280] = [0; 280];
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut hdr: TFB_DuckVideoHeader = TFB_DuckVideoHeader {
        version: 0,
        scrn_x_ofs: 0,
        scrn_y_ofs: 0,
        wb: 0,
        hb: 0,
        lumas: [0; 8],
        chromas: [0; 8],
    };
    strcat(
        strcpy(&raw mut filename as *mut ::core::ffi::c_char, (*dukv).basename),
        b".hdr\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fp = uqm_lua_checked!(uio_fopen(
        (*dukv).basedir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if fp.is_null() {
        return false;
    }
    ret = uqm_lua_checked!(uio_fread(
        &raw mut hdr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<TFB_DuckVideoHeader>(),
        1 as size_t,
        fp,
    ), false) as ::core::ffi::c_int;
    uqm_lua_checked!(uio_fclose(fp), false);
    if ret == 0 {
        return false;
    }
    (*dukv).version = UQM_Swap32(hdr.version);
    (*dukv).wb = UQM_Swap16(hdr.wb) as uint32;
    (*dukv).hb = UQM_Swap16(hdr.hb) as uint32;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *pl.offset(i as isize) = UQM_Swap16(hdr.lumas[i as usize] as uint16) as sint16
            as sint32;
        *pc.offset(i as isize) = UQM_Swap16(hdr.chromas[i as usize] as uint16) as sint16
            as sint32;
        i += 1;
    }
    (*dukv).decoder.w = (*dukv).wb.wrapping_mul(4 as uint32);
    (*dukv).decoder.h = (*dukv).hb.wrapping_mul(4 as uint32);
    return true;
}
unsafe extern "C" fn dukv_make_delta(
    mut protos: *mut sint32,
    mut is_chroma: bool,
    mut i1: ::core::ffi::c_int,
    mut i2: ::core::ffi::c_int,
) -> sint32 {
    let mut d1: sint32 = 0;
    let mut d2: sint32 = 0;
    if !is_chroma {
        d1 = ((*protos.offset(i1 as isize) >> 1 as ::core::ffi::c_int)
            * 0x421 as ::core::ffi::c_int) as sint32;
        d2 = ((*protos.offset(i2 as isize) >> 1 as ::core::ffi::c_int)
            * 0x421 as ::core::ffi::c_int) as sint32;
        return (d1 << 16 as ::core::ffi::c_int) + d2 << 1 as ::core::ffi::c_int;
    } else {
        d1 = (*protos.offset(i1 as isize) << 10 as ::core::ffi::c_int)
            + *protos.offset(i2 as isize);
        return (d1 << 16 as ::core::ffi::c_int) + d1 << 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn dukv_make_corr(
    mut protos: *mut sint32,
    mut is_chroma: bool,
    mut i1: ::core::ffi::c_int,
    mut i2: ::core::ffi::c_int,
) -> sint32 {
    let mut d1: sint32 = 0;
    let mut d2: sint32 = 0;
    if !is_chroma {
        d1 = ((*protos.offset(i1 as isize) & 1 as ::core::ffi::c_int)
            << 15 as ::core::ffi::c_int) as sint32;
        d2 = ((*protos.offset(i2 as isize) & 1 as ::core::ffi::c_int)
            << 15 as ::core::ffi::c_int) as sint32;
        return (d1 << 16 as ::core::ffi::c_int) + d2;
    } else {
        return (i1 << 3 as ::core::ffi::c_int) + i2
    };
}
unsafe extern "C" fn dukv_DecodeVector(
    mut vec: *mut uint8,
    mut p: *mut sint32,
    mut is_chroma: bool,
    mut deltas: *mut sint32,
) {
    let mut citems: ::core::ffi::c_int = *vec.offset(0isize) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < citems {
        let mut d: sint32 = dukv_make_delta(
            p,
            is_chroma,
            *vec.offset(1isize) as ::core::ffi::c_int,
            *vec.offset(2isize) as ::core::ffi::c_int,
        );
        if i == citems - 2 as ::core::ffi::c_int {
            d |= 1 as ::core::ffi::c_int;
        }
        *deltas.offset(0isize) = d;
        *deltas.offset(1isize) = dukv_make_corr(
            p,
            is_chroma,
            *vec.offset(1isize) as ::core::ffi::c_int,
            *vec.offset(2isize) as ::core::ffi::c_int,
        );
        i += 2 as ::core::ffi::c_int;
        vec = vec.offset(2 as ::core::ffi::c_int as isize);
        deltas = deltas.offset(2 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn dukv_InitDeltas(
    mut dukv: *mut TFB_DuckVideoDecoder,
    mut vectors: *mut uint8,
    mut pl: *mut sint32,
    mut pc: *mut sint32,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 0x100 as ::core::ffi::c_int {
        let mut vector: *mut uint8 = vectors
            .offset((i * 0x10 as ::core::ffi::c_int) as isize);
        dukv_DecodeVector(
            vector,
            pl,
            false,
            &raw mut *(&raw mut (*dukv).d.lumas as *mut [sint32; 16]).offset(i as isize)
                as *mut sint32,
        );
        dukv_DecodeVector(
            vector,
            pc,
            true,
            &raw mut *(&raw mut (*dukv).d.chromas as *mut [sint32; 16])
                .offset(i as isize) as *mut sint32,
        );
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn dukv_PixelConv(
    mut pix: uint16,
    mut fmt: *const TFB_PixelFormat,
) -> uint32 {
    let mut r: uint32 = 0;
    let mut g: uint32 = 0;
    let mut b: uint32 = 0;
    r = (pix as ::core::ffi::c_int >> 7 as ::core::ffi::c_int
        & 0xf8 as ::core::ffi::c_int) as uint32;
    g = (pix as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 0xf8 as ::core::ffi::c_int) as uint32;
    b = ((pix as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        & 0xf8 as ::core::ffi::c_int) as uint32;
    return r >> (*fmt).Rloss << (*fmt).Rshift | g >> (*fmt).Gloss << (*fmt).Gshift
        | b >> (*fmt).Bloss << (*fmt).Bshift;
}
unsafe extern "C" fn dukv_RenderFrame(mut This: *mut TFB_VideoDecoder) {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    let mut fmt: *const TFB_PixelFormat = (*This).format;
    let mut h: uint32 = 0;
    let mut x: uint32 = 0;
    let mut y: uint32 = 0;
    let mut pair: uint32 = 0;
    let mut dec: *mut uint32 = (*dukv).decbuf;
    let mut bufInc: uint32 = 0 as uint32;
    let mut scale: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    h = (*dukv).decoder.h.wrapping_div(2 as uint32);
    match (*fmt).BytesPerPixel {
        2 => {
            y = 0 as uint32;
            while y < h {
                let mut dst0: *mut uint16 = ::core::ptr::null_mut::<uint16>();
                let mut dst1: *mut uint16 = ::core::ptr::null_mut::<uint16>();
                dst0 = uqm_lua_checked!((*This)
                    .callbacks
                    .GetCanvasLine
                    .expect(
                        "non-null function pointer",
                    )(
                    This,
                    y
                        .wrapping_mul(2 as uint32)
                        .wrapping_add(
                            (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                            } else {
                                C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                            } == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as uint32,
                        ),
                ), ()) as *mut uint16;
                dst1 = uqm_lua_checked!((*This)
                    .callbacks
                    .GetCanvasLine
                    .expect(
                        "non-null function pointer",
                    )(This, y.wrapping_mul(2 as uint32).wrapping_add(1 as uint32)), ())
                    as *mut uint16;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                } else {
                    C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                } != 0 && y.wrapping_rem(scale as uint32) != 0 as uint32
                {
                    dec = dec.offset(-((*dukv).decoder.w as isize));
                }
                x = 0 as uint32;
                while x < (*dukv).decoder.w {
                    if bufInc.wrapping_rem(scale as uint32) == 0 as uint32
                        && if resolutionFactor != 2 as ::core::ffi::c_uint {
                            C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                        } else {
                            C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        dec = dec.offset(1);
                    }
                    pair = *dec;
                    *dst0 = dukv_PixelConv(
                        (pair >> 16 as ::core::ffi::c_int) as uint16,
                        fmt,
                    ) as uint16;
                    *dst1 = dukv_PixelConv((pair & 0xffff as uint32) as uint16, fmt)
                        as uint16;
                    x = x.wrapping_add(1);
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                    } else {
                        C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        dec = dec.offset(1);
                    } else {
                        bufInc = bufInc.wrapping_add(1);
                    };
                    dst0 = dst0.offset(1);
                    dst1 = dst1.offset(1);
                }
                y = y.wrapping_add(1);
            }
        }
        3 => {
            y = 0 as uint32;
            while y < h {
                let mut dst0_0: *mut uint8 = ::core::ptr::null_mut::<uint8>();
                let mut dst1_0: *mut uint8 = ::core::ptr::null_mut::<uint8>();
                dst0_0 = uqm_lua_checked!((*This)
                    .callbacks
                    .GetCanvasLine
                    .expect(
                        "non-null function pointer",
                    )(
                    This,
                    y
                        .wrapping_mul(2 as uint32)
                        .wrapping_add(
                            (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                            } else {
                                C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                            } == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as uint32,
                        ),
                ), ()) as *mut uint8;
                dst1_0 = uqm_lua_checked!((*This)
                    .callbacks
                    .GetCanvasLine
                    .expect(
                        "non-null function pointer",
                    )(This, y.wrapping_mul(2 as uint32).wrapping_add(1 as uint32)), ())
                    as *mut uint8;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                } else {
                    C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                } != 0 && y.wrapping_rem(scale as uint32) != 0 as uint32
                {
                    dec = dec.offset(-((*dukv).decoder.w as isize));
                }
                x = 0 as uint32;
                while x < (*dukv).decoder.w {
                    if bufInc.wrapping_rem(scale as uint32) == 0 as uint32
                        && if resolutionFactor != 2 as ::core::ffi::c_uint {
                            C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                        } else {
                            C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        dec = dec.offset(1);
                    }
                    pair = *dec;
                    *(dst0_0 as *mut uint32) = dukv_PixelConv(
                        (pair >> 16 as ::core::ffi::c_int) as uint16,
                        fmt,
                    );
                    *(dst1_0 as *mut uint32) = dukv_PixelConv(
                        (pair & 0xffff as uint32) as uint16,
                        fmt,
                    );
                    x = x.wrapping_add(1);
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                    } else {
                        C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        dec = dec.offset(1);
                    } else {
                        bufInc = bufInc.wrapping_add(1);
                    };
                    dst0_0 = dst0_0.offset(3 as ::core::ffi::c_int as isize);
                    dst1_0 = dst1_0.offset(3 as ::core::ffi::c_int as isize);
                }
                y = y.wrapping_add(1);
            }
        }
        4 => {
            y = 0 as uint32;
            while y < h {
                let mut dst0_1: *mut uint32 = ::core::ptr::null_mut::<uint32>();
                let mut dst1_1: *mut uint32 = ::core::ptr::null_mut::<uint32>();
                dst0_1 = uqm_lua_checked!((*This)
                    .callbacks
                    .GetCanvasLine
                    .expect(
                        "non-null function pointer",
                    )(This, y.wrapping_mul(2 as uint32)), ()) as *mut uint32;
                dst1_1 = uqm_lua_checked!((*This)
                    .callbacks
                    .GetCanvasLine
                    .expect(
                        "non-null function pointer",
                    )(This, y.wrapping_mul(2 as uint32).wrapping_add(1 as uint32)), ())
                    as *mut uint32;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                } else {
                    C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                } != 0 && y.wrapping_rem(scale as uint32) != 0 as uint32
                {
                    dec = dec
                        .offset(-(((*dukv).decoder.w >> resolutionFactor) as isize));
                }
                x = 0 as uint32;
                while x < (*dukv).decoder.w {
                    if bufInc.wrapping_rem(scale as uint32) == 0 as uint32
                        && if resolutionFactor != 2 as ::core::ffi::c_uint {
                            C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                        } else {
                            C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                        } != 0
                    {
                        dec = dec.offset(1);
                    }
                    pair = *dec;
                    *dst0_1 = dukv_PixelConv(
                        (pair >> 16 as ::core::ffi::c_int) as uint16,
                        fmt,
                    );
                    *dst1_1 = dukv_PixelConv((pair & 0xffff as uint32) as uint16, fmt);
                    x = x.wrapping_add(1);
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                    } else {
                        C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        dec = dec.offset(1);
                    } else {
                        bufInc = bufInc.wrapping_add(1);
                    };
                    dst0_1 = dst0_1.offset(1);
                    dst1_1 = dst1_1.offset(1);
                }
                y = y.wrapping_add(1);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn dukv_GetName() -> *const ::core::ffi::c_char {
    return b"DukVid\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn dukv_InitModule(mut flags: ::core::ffi::c_int) -> bool {
    return true;
}
unsafe extern "C" fn dukv_TermModule() {}
unsafe extern "C" fn dukv_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_DuckVideoDecoder>() as uint32;
}
unsafe extern "C" fn dukv_GetError(
    mut This: *mut TFB_VideoDecoder,
) -> ::core::ffi::c_int {
    return (*This).error;
}
unsafe extern "C" fn dukv_Init(
    mut This: *mut TFB_VideoDecoder,
    mut fmt: *mut TFB_PixelFormat,
) -> bool {
    (*This).format = fmt;
    (*This).audio_synced = true;
    return true;
}
unsafe extern "C" fn dukv_Term(mut This: *mut TFB_VideoDecoder) {
    uqm_lua_checked!(dukv_Close(This), ());
}
unsafe extern "C" fn dukv_Open(
    mut This: *mut TFB_VideoDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    let mut pext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lumas: [sint32; 8] = [0; 8];
    let mut chromas: [sint32; 8] = [0; 8];
    let mut vectors: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    (*dukv).basedir = dir;
    (*dukv).basename = HMalloc(strlen(filename).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    strcpy((*dukv).basename, filename);
    pext = strrchr((*dukv).basename, '.' as ::core::ffi::c_int);
    if !pext.is_null() {
        *pext = 0 as ::core::ffi::c_char;
    }
    vectors = HMalloc(
        (0x10 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int) as size_t,
    ) as *mut uint8;
    if !uqm_lua_checked!(dukv_OpenStream(dukv), false) || !uqm_lua_checked!(dukv_ReadFrames(dukv), false)
        || !uqm_lua_checked!(dukv_ReadHeader(
            dukv,
            &raw mut lumas as *mut sint32,
            &raw mut chromas as *mut sint32,
        ), false) || !uqm_lua_checked!(dukv_ReadVectors(dukv, vectors), false)
    {
        HFree(vectors as *mut ::core::ffi::c_void);
        uqm_lua_checked!(dukv_Close(This), false);
        (*dukv).last_error = C2Rust_Unnamed_0::dukve_BadFile.0 as sint32;
        return false;
    }
    dukv_InitDeltas(
        dukv,
        vectors,
        &raw mut lumas as *mut sint32,
        &raw mut chromas as *mut sint32,
    );
    HFree(vectors as *mut ::core::ffi::c_void);
    (*This).length = (*dukv).cframes as ::core::ffi::c_float / 14.622f32;
    (*This).frame_count = (*dukv).cframes;
    (*This).interframe_wait = (1000.0f64 / 14.622f64) as uint32;
    (*dukv).inbuf = HMalloc(0x8000 as size_t) as *mut uint8;
    (*dukv).decbuf = HMalloc(
        ((*dukv).decoder.w.wrapping_mul((*dukv).decoder.h) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint16>()),
    ) as *mut uint32;
    return true;
}
unsafe extern "C" fn dukv_Close(mut This: *mut TFB_VideoDecoder) {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    if !(*dukv).basename.is_null() {
        HFree((*dukv).basename as *mut ::core::ffi::c_void);
        (*dukv).basename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*dukv).frames.is_null() {
        HFree((*dukv).frames as *mut ::core::ffi::c_void);
        (*dukv).frames = ::core::ptr::null_mut::<uint32>();
    }
    if !(*dukv).stream.is_null() {
        uqm_lua_checked!(uio_fclose((*dukv).stream), ());
        (*dukv).stream = ::core::ptr::null_mut::<uio_Stream>();
    }
    if !(*dukv).inbuf.is_null() {
        HFree((*dukv).inbuf as *mut ::core::ffi::c_void);
        (*dukv).inbuf = ::core::ptr::null_mut::<uint8>();
    }
    if !(*dukv).decbuf.is_null() {
        HFree((*dukv).decbuf as *mut ::core::ffi::c_void);
        (*dukv).decbuf = ::core::ptr::null_mut::<uint32>();
    }
}
unsafe extern "C" fn dukv_DecodeNext(
    mut This: *mut TFB_VideoDecoder,
) -> ::core::ffi::c_int {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    let mut fh: [uint32; 2] = [0; 2];
    let mut vofs: uint32 = 0;
    let mut vsize: uint32 = 0;
    let mut ver: uint16 = 0;
    if (*dukv).stream.is_null() || (*dukv).iframe >= (*dukv).cframes {
        return 0 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_fseek(
        (*dukv).stream,
        *(*dukv).frames.offset((*dukv).iframe as isize) as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    ), 0);
    if uqm_lua_checked!(uio_fread(
        &raw mut fh as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint32; 2]>(),
        1 as size_t,
        (*dukv).stream,
    ), 0) != 1 as size_t
    {
        (*dukv).last_error = C2Rust_Unnamed_0::dukve_EOF.0 as sint32;
        return 0 as ::core::ffi::c_int;
    }
    vofs = UQM_Swap32(fh[0usize]);
    vsize = UQM_Swap32(fh[1usize]);
    if vsize > 0x8000 as ::core::ffi::c_uint {
        (*dukv).last_error = C2Rust_Unnamed_0::dukve_OutOfBuf.0 as sint32;
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_fseek((*dukv).stream, vofs as ::core::ffi::c_long, 1 as ::core::ffi::c_int), 0);
    if uqm_lua_checked!(uio_fread(
        (*dukv).inbuf as *mut ::core::ffi::c_void,
        1 as size_t,
        vsize as size_t,
        (*dukv).stream,
    ), 0) != vsize as size_t
    {
        (*dukv).last_error = C2Rust_Unnamed_0::dukve_EOF.0 as sint32;
        return 0 as ::core::ffi::c_int;
    }
    ver = UQM_Swap16(*((*dukv).inbuf as *mut uint16));
    if ver as ::core::ffi::c_int == 0x300 as ::core::ffi::c_int {
        dukv_DecodeFrameV3(
            (*dukv).inbuf.offset(0x10 as ::core::ffi::c_int as isize),
            (*dukv).decbuf,
            (*dukv).wb,
            (*dukv).hb,
            &raw mut (*dukv).d,
        );
    } else {
        dukv_DecodeFrame(
            (*dukv).inbuf.offset(0x10 as ::core::ffi::c_int as isize),
            (*dukv).decbuf,
            (*dukv).wb,
            (*dukv).hb,
            &raw mut (*dukv).d,
        );
    }
    (*dukv).iframe = (*dukv).iframe.wrapping_add(1);
    uqm_lua_checked!((*This).callbacks.BeginFrame.expect("non-null function pointer")(This), 0);
    uqm_lua_checked!(dukv_RenderFrame(This), 0);
    uqm_lua_checked!((*This).callbacks.EndFrame.expect("non-null function pointer")(This), 0);
    if !(*This).audio_synced {
        uqm_lua_checked!((*This)
            .callbacks
            .SetTimer
            .expect(
                "non-null function pointer",
            )(This, (1000.0f32 / 14.622f32) as uint32), 0);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn dukv_SeekFrame(
    mut This: *mut TFB_VideoDecoder,
    mut frame: uint32,
) -> uint32 {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    if frame > (*dukv).cframes {
        frame = (*dukv).cframes;
    }
    (*dukv).iframe = frame;
    return (*dukv).iframe;
}
unsafe extern "C" fn dukv_SeekTime(
    mut This: *mut TFB_VideoDecoder,
    mut time: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut frame: uint32 = (time * 14.622f32) as uint32;
    return dukv_SeekFrame(This, frame) as ::core::ffi::c_float / 14.622f32;
}
unsafe extern "C" fn dukv_GetFrame(mut This: *mut TFB_VideoDecoder) -> uint32 {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    return (*dukv).iframe;
}
unsafe extern "C" fn dukv_GetTime(
    mut This: *mut TFB_VideoDecoder,
) -> ::core::ffi::c_float {
    let mut dukv: *mut TFB_DuckVideoDecoder = This as *mut TFB_DuckVideoDecoder;
    return (*dukv).iframe as ::core::ffi::c_float / 14.622f32;
}
#[inline]
unsafe extern "C" fn UQM_Swap16(mut D: uint16) -> uint16 {
    return ((D as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | D as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint16;
}
#[inline]
unsafe extern "C" fn UQM_Swap32(mut D: uint32) -> uint32 {
    return D << 24 as ::core::ffi::c_int
        | D << 8 as ::core::ffi::c_int & 0xff0000 as ::core::ffi::c_int as uint32
        | D >> 8 as ::core::ffi::c_int & 0xff00 as uint32
        | D >> 24 as ::core::ffi::c_int;
}
