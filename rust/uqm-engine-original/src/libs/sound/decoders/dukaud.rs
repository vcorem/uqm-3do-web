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

/* .duk sound track decoder
 */

// Rust translation of src/libs/sound/decoders/dukaud.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
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
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_decoderformats {
    pub big_endian: bool,
    pub want_big_endian: bool,
    pub mono8: uint32,
    pub stereo8: uint32,
    pub mono16: uint32,
    pub stereo16: uint32,
}
pub type TFB_DecoderFormats = tfb_decoderformats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_sounddecoder {
    pub funcs: *const TFB_SoundDecoderFuncs,
    pub format: uint32,
    pub frequency: uint32,
    pub length: ::core::ffi::c_float,
    pub is_null: bool,
    pub need_swap: bool,
    pub buffer: *mut ::core::ffi::c_void,
    pub buffer_size: uint32,
    pub error: sint32,
    pub bytes_per_samp: uint32,
    pub looping: bool,
    pub dir: *mut uio_DirHandle,
    pub filename: *mut ::core::ffi::c_char,
    pub pos: uint32,
    pub start_sample: uint32,
    pub end_sample: uint32,
}
pub type TFB_SoundDecoderFuncs = tfb_sounddecoderfunc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_sounddecoderfunc {
    pub GetName: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub InitModule: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *const TFB_DecoderFormats) -> bool,
    >,
    pub TermModule: Option<unsafe extern "C" fn() -> ()>,
    pub GetStructSize: Option<unsafe extern "C" fn() -> uint32>,
    pub GetError: Option<
        unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    >,
    pub Init: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool>,
    pub Term: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()>,
    pub Open: Option<
        unsafe extern "C" fn(
            *mut TFB_SoundDecoder,
            *mut uio_DirHandle,
            *const ::core::ffi::c_char,
        ) -> bool,
    >,
    pub Close: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()>,
    pub Decode: Option<
        unsafe extern "C" fn(
            *mut TFB_SoundDecoder,
            *mut ::core::ffi::c_void,
            sint32,
        ) -> ::core::ffi::c_int,
    >,
    pub Seek: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32>,
    pub GetFrame: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32>,
}
pub type TFB_SoundDecoder = tfb_sounddecoder;
pub type TFB_DuckSoundDecoder = tfb_ducksounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_ducksounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub iframe: uint32,
    pub cframes: uint32,
    pub channels: uint32,
    pub pcm_frame: uint32,
    pub last_error: sint32,
    pub duk: *mut uio_Stream,
    pub frames: *mut uint32,
    pub data: *mut ::core::ffi::c_void,
    pub maxdata: uint32,
    pub cbdata: uint32,
    pub dataofs: uint32,
    pub predictors: [sint32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DukAud_AudSubframe {
    pub magic: uint16,
    pub numsamples: uint16,
    pub tag: uint16,
    pub indices: [uint16; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DukAud_FrameHeader {
    pub audsize: uint32,
    pub vidsize: uint32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const dukae_None: Self = Self(0);
    pub const dukae_Unknown: Self = Self(-1);
    pub const dukae_BadFile: Self = Self(-2);
    pub const dukae_BadArg: Self = Self(-3);
    pub const dukae_Other: Self = Self(-1000);
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
#[no_mangle]
pub static mut duka_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(duka_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        duka_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(duka_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(duka_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        duka_GetError
            as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(duka_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(duka_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        duka_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(duka_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        duka_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        duka_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(
        duka_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32,
    ),
};
static mut duka_formats: *const TFB_DecoderFormats = ::core::ptr::null::<
    TFB_DecoderFormats,
>();
unsafe extern "C" fn duka_readAudFrameHeader(
    mut duka: *mut TFB_DuckSoundDecoder,
    mut iframe: uint32,
    mut aud: *mut DukAud_AudSubframe,
) -> sint32 {
    let mut hdr: DukAud_FrameHeader = DukAud_FrameHeader {
        audsize: 0,
        vidsize: 0,
    };
    uio_fseek(
        (*duka).duk,
        *(*duka).frames.offset(iframe as isize) as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    );
    if uio_fread(
        &raw mut hdr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<DukAud_FrameHeader>(),
        1 as size_t,
        (*duka).duk,
    ) != 1 as size_t
    {
        (*duka).last_error = *__errno_location() as sint32;
        return C2Rust_Unnamed::dukae_BadFile.0;
    }
    hdr.audsize = UQM_Swap32(hdr.audsize);
    if uio_fread(
        aud as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<DukAud_AudSubframe>(),
        1 as size_t,
        (*duka).duk,
    ) != 1 as size_t
    {
        (*duka).last_error = *__errno_location() as sint32;
        return C2Rust_Unnamed::dukae_BadFile.0;
    }
    (*aud).magic = UQM_Swap16((*aud).magic);
    if (*aud).magic as ::core::ffi::c_int != 0xf77f as ::core::ffi::c_int {
        (*duka).last_error = C2Rust_Unnamed::dukae_BadFile.0 as sint32;
        return (*duka).last_error;
    }
    (*aud).numsamples = UQM_Swap16((*aud).numsamples);
    (*aud).tag = UQM_Swap16((*aud).tag);
    (*aud).indices[0usize] = UQM_Swap16((*aud).indices[0usize]);
    (*aud).indices[1usize] = UQM_Swap16((*aud).indices[1usize]);
    return 0 as sint32;
}
static mut adpcm_step: [::core::ffi::c_int; 89] = [
    0x7 as ::core::ffi::c_int,
    0x8 as ::core::ffi::c_int,
    0x9 as ::core::ffi::c_int,
    0xa as ::core::ffi::c_int,
    0xb as ::core::ffi::c_int,
    0xc as ::core::ffi::c_int,
    0xd as ::core::ffi::c_int,
    0xf as ::core::ffi::c_int,
    0x10 as ::core::ffi::c_int,
    0x12 as ::core::ffi::c_int,
    0x13 as ::core::ffi::c_int,
    0x15 as ::core::ffi::c_int,
    0x17 as ::core::ffi::c_int,
    0x1a as ::core::ffi::c_int,
    0x1c as ::core::ffi::c_int,
    0x1f as ::core::ffi::c_int,
    0x22 as ::core::ffi::c_int,
    0x26 as ::core::ffi::c_int,
    0x29 as ::core::ffi::c_int,
    0x2e as ::core::ffi::c_int,
    0x32 as ::core::ffi::c_int,
    0x37 as ::core::ffi::c_int,
    0x3d as ::core::ffi::c_int,
    0x43 as ::core::ffi::c_int,
    0x4a as ::core::ffi::c_int,
    0x51 as ::core::ffi::c_int,
    0x59 as ::core::ffi::c_int,
    0x62 as ::core::ffi::c_int,
    0x6c as ::core::ffi::c_int,
    0x76 as ::core::ffi::c_int,
    0x82 as ::core::ffi::c_int,
    0x8f as ::core::ffi::c_int,
    0x9e as ::core::ffi::c_int,
    0xad as ::core::ffi::c_int,
    0xbf as ::core::ffi::c_int,
    0xd2 as ::core::ffi::c_int,
    0xe7 as ::core::ffi::c_int,
    0xfe as ::core::ffi::c_int,
    0x117 as ::core::ffi::c_int,
    0x133 as ::core::ffi::c_int,
    0x152 as ::core::ffi::c_int,
    0x174 as ::core::ffi::c_int,
    0x199 as ::core::ffi::c_int,
    0x1c2 as ::core::ffi::c_int,
    0x1ef as ::core::ffi::c_int,
    0x220 as ::core::ffi::c_int,
    0x256 as ::core::ffi::c_int,
    0x292 as ::core::ffi::c_int,
    0x2d4 as ::core::ffi::c_int,
    0x31d as ::core::ffi::c_int,
    0x36c as ::core::ffi::c_int,
    0x3c4 as ::core::ffi::c_int,
    0x424 as ::core::ffi::c_int,
    0x48e as ::core::ffi::c_int,
    0x503 as ::core::ffi::c_int,
    0x583 as ::core::ffi::c_int,
    0x610 as ::core::ffi::c_int,
    0x6ac as ::core::ffi::c_int,
    0x756 as ::core::ffi::c_int,
    0x812 as ::core::ffi::c_int,
    0x8e1 as ::core::ffi::c_int,
    0x9c4 as ::core::ffi::c_int,
    0xabe as ::core::ffi::c_int,
    0xbd1 as ::core::ffi::c_int,
    0xcff as ::core::ffi::c_int,
    0xe4c as ::core::ffi::c_int,
    0xfba as ::core::ffi::c_int,
    0x114d as ::core::ffi::c_int,
    0x1308 as ::core::ffi::c_int,
    0x14ef as ::core::ffi::c_int,
    0x1707 as ::core::ffi::c_int,
    0x1954 as ::core::ffi::c_int,
    0x1bdd as ::core::ffi::c_int,
    0x1ea6 as ::core::ffi::c_int,
    0x21b7 as ::core::ffi::c_int,
    0x2516 as ::core::ffi::c_int,
    0x28cb as ::core::ffi::c_int,
    0x2cdf as ::core::ffi::c_int,
    0x315c as ::core::ffi::c_int,
    0x364c as ::core::ffi::c_int,
    0x3bba as ::core::ffi::c_int,
    0x41b2 as ::core::ffi::c_int,
    0x4844 as ::core::ffi::c_int,
    0x4f7e as ::core::ffi::c_int,
    0x5771 as ::core::ffi::c_int,
    0x6030 as ::core::ffi::c_int,
    0x69ce as ::core::ffi::c_int,
    0x7463 as ::core::ffi::c_int,
    0x7fff as ::core::ffi::c_int,
];
static mut adpcm_index: [::core::ffi::c_int; 16] = [
    -1 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    -1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
];
unsafe extern "C" fn decode_nibbles(
    mut output: *mut sint16,
    mut output_size: sint32,
    mut channels: sint32,
    mut predictors: *mut sint32,
    mut indices: *mut uint16,
) {
    let mut step: [sint32; 2] = [0; 2];
    let mut index: [sint32; 2] = [0; 2];
    let mut diff: sint32 = 0;
    let mut i: sint32 = 0;
    let mut sign: ::core::ffi::c_int = 0;
    let mut delta: sint32 = 0;
    let mut channel_number: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    channels -= 1 as ::core::ffi::c_int;
    index[0usize] = *indices.offset(0isize) as sint32;
    index[1usize] = *indices.offset(1isize) as sint32;
    step[0usize] = adpcm_step[index[0usize] as usize] as sint32;
    step[1usize] = adpcm_step[index[1usize] as usize] as sint32;
    i = 0 as ::core::ffi::c_int as sint32;
    while i < output_size {
        delta = *output.offset(i as isize) as sint32;
        index[channel_number as usize] += adpcm_index[delta as usize];
        if index[channel_number as usize] < 0 as ::core::ffi::c_int {
            index[channel_number as usize] = 0 as ::core::ffi::c_int as sint32;
        } else if index[channel_number as usize] > 88 as ::core::ffi::c_int {
            index[channel_number as usize] = 88 as ::core::ffi::c_int as sint32;
        }
        sign = delta & 8 as ::core::ffi::c_int;
        delta = (delta & 7 as ::core::ffi::c_int) as sint32;
        diff = ((delta << 1 as ::core::ffi::c_int) + 1 as sint32)
            * step[channel_number as usize] >> 3 as ::core::ffi::c_int;
        if sign != 0 {
            *predictors.offset(channel_number as isize) -= diff;
        } else {
            *predictors.offset(channel_number as isize) += diff;
        }
        if *predictors.offset(channel_number as isize) < -32768 as ::core::ffi::c_int {
            *predictors.offset(channel_number as isize) = -32768 as ::core::ffi::c_int
                as sint32;
        } else if *predictors.offset(channel_number as isize)
            > 32767 as ::core::ffi::c_int
        {
            *predictors.offset(channel_number as isize) = 32767 as ::core::ffi::c_int
                as sint32;
        }
        *output.offset(i as isize) = *predictors.offset(channel_number as isize)
            as sint16;
        step[channel_number as usize] = adpcm_step[index[channel_number as usize]
            as usize] as sint32;
        channel_number ^= channels as ::core::ffi::c_int;
        i += 1;
    }
}
unsafe extern "C" fn duka_decodeFrame(
    mut duka: *mut TFB_DuckSoundDecoder,
    mut header: *mut DukAud_AudSubframe,
    mut input: *mut uint8,
) -> sint32 {
    let mut inend: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut output: *mut sint16 = ::core::ptr::null_mut::<sint16>();
    let mut outptr: *mut sint16 = ::core::ptr::null_mut::<sint16>();
    let mut outputsize: sint32 = 0;
    outputsize = (((*header).numsamples as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        as usize)
        .wrapping_mul(::core::mem::size_of::<sint16>()) as sint32;
    output = ((*duka).data as *mut uint8).offset((*duka).cbdata as isize) as *mut sint16;
    outptr = output;
    inend = input.offset((*header).numsamples as ::core::ffi::c_int as isize);
    while input < inend {
        let c2rust_fresh0 = outptr;
        outptr = outptr.offset(1);
        *c2rust_fresh0 = (*input as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as sint16;
        let c2rust_fresh1 = outptr;
        outptr = outptr.offset(1);
        *c2rust_fresh1 = (*input as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
            as sint16;
        input = input.offset(1);
    }
    decode_nibbles(
        output,
        (*header).numsamples as sint32 * 2 as sint32,
        (*duka).channels as sint32,
        &raw mut (*duka).predictors as *mut sint32,
        &raw mut (*header).indices as *mut uint16,
    );
    (*duka).cbdata = (*duka).cbdata.wrapping_add(outputsize as uint32);
    return outputsize;
}
unsafe extern "C" fn duka_readNextFrame(mut duka: *mut TFB_DuckSoundDecoder) -> sint32 {
    let mut hdr: DukAud_FrameHeader = DukAud_FrameHeader {
        audsize: 0,
        vidsize: 0,
    };
    let mut aud: *mut DukAud_AudSubframe = ::core::ptr::null_mut::<DukAud_AudSubframe>();
    let mut p: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    uio_fseek(
        (*duka).duk,
        *(*duka).frames.offset((*duka).iframe as isize) as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    );
    if uio_fread(
        &raw mut hdr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<DukAud_FrameHeader>(),
        1 as size_t,
        (*duka).duk,
    ) != 1 as size_t
    {
        (*duka).last_error = *__errno_location() as sint32;
        return C2Rust_Unnamed::dukae_BadFile.0;
    }
    hdr.audsize = UQM_Swap32(hdr.audsize);
    p = ((*duka).data as *mut uint8)
        .offset((*duka).maxdata as isize)
        .offset(
            -((hdr.audsize.wrapping_add(7 as uint32)
                & -8 as ::core::ffi::c_int as uint32) as isize),
        );
    if uio_fread(
        p as *mut ::core::ffi::c_void,
        1 as size_t,
        hdr.audsize as size_t,
        (*duka).duk,
    ) != hdr.audsize as size_t
    {
        (*duka).last_error = *__errno_location() as sint32;
        return C2Rust_Unnamed::dukae_BadFile.0;
    }
    aud = p as *mut DukAud_AudSubframe;
    p = p.offset(::core::mem::size_of::<DukAud_AudSubframe>() as isize);
    (*aud).magic = UQM_Swap16((*aud).magic);
    if (*aud).magic as ::core::ffi::c_int != 0xf77f as ::core::ffi::c_int {
        (*duka).last_error = C2Rust_Unnamed::dukae_BadFile.0 as sint32;
        return (*duka).last_error;
    }
    (*aud).numsamples = UQM_Swap16((*aud).numsamples);
    (*aud).tag = UQM_Swap16((*aud).tag);
    (*aud).indices[0usize] = UQM_Swap16((*aud).indices[0usize]);
    (*aud).indices[1usize] = UQM_Swap16((*aud).indices[1usize]);
    (*duka).iframe = (*duka).iframe.wrapping_add(1);
    return duka_decodeFrame(duka, aud, p);
}
unsafe extern "C" fn duka_stuffBuffer(
    mut duka: *mut TFB_DuckSoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> sint32 {
    let mut dataleft: sint32 = 0;
    dataleft = (*duka).cbdata.wrapping_sub((*duka).dataofs) as sint32;
    if dataleft > 0 as ::core::ffi::c_int {
        if dataleft > bufsize {
            dataleft = (bufsize & -4 as ::core::ffi::c_int) as sint32;
        }
        memcpy(
            buf,
            ((*duka).data as *mut uint8).offset((*duka).dataofs as isize)
                as *const ::core::ffi::c_void,
            dataleft as size_t,
        );
        (*duka).dataofs = (*duka).dataofs.wrapping_add(dataleft as uint32);
    }
    if (*duka).cbdata > 0 as uint32 && (*duka).dataofs >= (*duka).cbdata {
        (*duka).dataofs = 0 as uint32;
        (*duka).cbdata = (*duka).dataofs;
    }
    return dataleft;
}
unsafe extern "C" fn duka_GetName() -> *const ::core::ffi::c_char {
    return b"DukAud\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn duka_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    duka_formats = fmts;
    return true;
}
unsafe extern "C" fn duka_TermModule() {}
unsafe extern "C" fn duka_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_DuckSoundDecoder>() as uint32;
}
unsafe extern "C" fn duka_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    let mut duka: *mut TFB_DuckSoundDecoder = This as *mut TFB_DuckSoundDecoder;
    let mut ret: ::core::ffi::c_int = (*duka).last_error;
    (*duka).last_error = C2Rust_Unnamed::dukae_None.0 as sint32;
    return ret;
}
unsafe extern "C" fn duka_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    (*This).need_swap = (*duka_formats).big_endian as ::core::ffi::c_int
        != (*duka_formats).want_big_endian as ::core::ffi::c_int;
    return true;
}
unsafe extern "C" fn duka_Term(mut This: *mut TFB_SoundDecoder) {
    duka_Close(This);
}
unsafe extern "C" fn duka_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut file: *const ::core::ffi::c_char,
) -> bool {
    let mut duka: *mut TFB_DuckSoundDecoder = This as *mut TFB_DuckSoundDecoder;
    let mut duk: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut frm: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut aud: DukAud_AudSubframe = DukAud_AudSubframe {
        magic: 0,
        numsamples: 0,
        tag: 0,
        indices: [0; 2],
    };
    let mut filename: [::core::ffi::c_char; 256] = [0; 256];
    let mut filelen: uint32 = 0;
    let mut cread: size_t = 0;
    let mut i: uint32 = 0;
    filelen = strlen(file) as uint32;
    if filelen as usize
        > ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(1usize)
    {
        return false;
    }
    strcpy(&raw mut filename as *mut ::core::ffi::c_char, file);
    duk = uio_fopen(
        dir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if duk.is_null() {
        (*duka).last_error = *__errno_location() as sint32;
        return false;
    }
    strcpy(
        (&raw mut filename as *mut ::core::ffi::c_char)
            .offset(filelen as isize)
            .offset(-(3 as ::core::ffi::c_int as isize)),
        b"frm\0".as_ptr() as *const ::core::ffi::c_char,
    );
    frm = uio_fopen(
        dir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if frm.is_null() {
        (*duka).last_error = *__errno_location() as sint32;
        uio_fclose(duk);
        return false;
    }
    (*duka).duk = duk;
    uio_fseek(frm, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int);
    (*duka).cframes = (uio_ftell(frm) as usize)
        .wrapping_div(::core::mem::size_of::<uint32>()) as uint32;
    uio_fseek(frm, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    if (*duka).cframes == 0 {
        (*duka).last_error = C2Rust_Unnamed::dukae_BadFile.0 as sint32;
        uio_fclose(frm);
        duka_Close(This);
        return false;
    }
    (*duka).frames = HMalloc(
        ((*duka).cframes as size_t).wrapping_mul(::core::mem::size_of::<uint32>()),
    ) as *mut uint32;
    cread = uio_fread(
        (*duka).frames as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<uint32>(),
        (*duka).cframes as size_t,
        frm,
    );
    uio_fclose(frm);
    if cread != (*duka).cframes as size_t {
        (*duka).last_error = C2Rust_Unnamed::dukae_BadFile.0 as sint32;
        duka_Close(This);
        return false;
    }
    i = 0 as uint32;
    while i < (*duka).cframes {
        *(*duka).frames.offset(i as isize) = UQM_Swap32(
            *(*duka).frames.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
    if duka_readAudFrameHeader(duka, 0 as uint32, &raw mut aud) < 0 as ::core::ffi::c_int
    {
        duka_Close(This);
        return false;
    }
    (*This).frequency = 22050 as uint32;
    (*This).format = (*duka_formats).stereo16;
    (*duka).channels = 2 as uint32;
    (*duka).pcm_frame = aud.numsamples as uint32;
    (*duka).data = HMalloc(0x8000 as size_t);
    (*duka).maxdata = 0x8000 as uint32;
    (*This).length = (*duka).cframes as ::core::ffi::c_float / 14.622f32;
    (*duka).last_error = 0 as ::core::ffi::c_int as sint32;
    return true;
}
unsafe extern "C" fn duka_Close(mut This: *mut TFB_SoundDecoder) {
    let mut duka: *mut TFB_DuckSoundDecoder = This as *mut TFB_DuckSoundDecoder;
    if !(*duka).data.is_null() {
        HFree((*duka).data);
        (*duka).data = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !(*duka).frames.is_null() {
        HFree((*duka).frames as *mut ::core::ffi::c_void);
        (*duka).frames = ::core::ptr::null_mut::<uint32>();
    }
    if !(*duka).duk.is_null() {
        uio_fclose((*duka).duk);
        (*duka).duk = ::core::ptr::null_mut::<uio_Stream>();
    }
    (*duka).last_error = 0 as ::core::ffi::c_int as sint32;
}
unsafe extern "C" fn duka_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut duka: *mut TFB_DuckSoundDecoder = This as *mut TFB_DuckSoundDecoder;
    let mut stuffed: sint32 = 0;
    let mut total: sint32 = 0 as sint32;
    if bufsize <= 0 as ::core::ffi::c_int {
        (*duka).last_error = C2Rust_Unnamed::dukae_BadArg.0 as sint32;
        return (*duka).last_error;
    }
    loop {
        stuffed = duka_stuffBuffer(duka, buf, bufsize);
        buf = (buf as *mut uint8).offset(stuffed as isize) as *mut ::core::ffi::c_void;
        bufsize -= stuffed;
        total += stuffed;
        if bufsize > 0 as ::core::ffi::c_int && (*duka).iframe < (*duka).cframes {
            stuffed = duka_readNextFrame(duka);
            if stuffed <= 0 as ::core::ffi::c_int {
                return stuffed;
            }
        }
        if !(bufsize > 0 as ::core::ffi::c_int && (*duka).iframe < (*duka).cframes) {
            break;
        }
    }
    return total;
}
unsafe extern "C" fn duka_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut duka: *mut TFB_DuckSoundDecoder = This as *mut TFB_DuckSoundDecoder;
    let mut iframe: uint32 = 0;
    iframe = pcm_pos.wrapping_div((*duka).pcm_frame);
    if iframe < (*duka).cframes {
        (*duka).iframe = iframe;
        (*duka).cbdata = 0 as uint32;
        (*duka).dataofs = 0 as uint32;
        (*duka).predictors[0usize] = 0 as ::core::ffi::c_int as sint32;
        (*duka).predictors[1usize] = 0 as ::core::ffi::c_int as sint32;
    }
    return (*duka).iframe.wrapping_mul((*duka).pcm_frame);
}
unsafe extern "C" fn duka_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    let mut duka: *mut TFB_DuckSoundDecoder = This as *mut TFB_DuckSoundDecoder;
    return if (*duka).dataofs == (*duka).cbdata {
        (*duka).iframe
    } else {
        (*duka).iframe.wrapping_sub(1 as uint32)
    };
}
