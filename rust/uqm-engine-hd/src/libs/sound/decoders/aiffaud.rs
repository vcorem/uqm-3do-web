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

/* Portions (C) Serge van den Boom (svdb at stack.nl) */
/* Portions (C) Alex Volkov (codepro at usa.net) */

/* AIFF decoder (.aif)
 *
 * Doesn't work on *all* aiff files in general, only 8/16 PCM and
 * 16-bit AIFF-C SDX2-compressed.
 */

// Rust translation of src/libs/sound/decoders/aiffaud.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
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
    fn crc32b(str: *const ::core::ffi::c_char) -> uint32_t;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
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
    pub numpos: uint16,
    pub filename_hash: uint32,
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
pub type TFB_AiffSoundDecoder = tfb_wavesounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_wavesounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub last_error: sint32,
    pub fp: *mut uio_Stream,
    pub fmtHdr: aiff_ExtCommonChunk,
    pub comp_type: aiff_CompressionType,
    pub bits_per_sample: ::core::ffi::c_uint,
    pub block_align: ::core::ffi::c_uint,
    pub file_block: ::core::ffi::c_uint,
    pub data_ofs: uint32,
    pub data_size: uint32,
    pub max_pcm: uint32,
    pub cur_pcm: uint32,
    pub prev_val: [sint32; 4],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct aiff_CompressionType(pub ::core::ffi::c_uint);
impl aiff_CompressionType {
    pub const aifc_None: Self = Self(0);
    pub const aifc_Sdx2: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aiff_ExtCommonChunk {
    pub channels: uint16,
    pub sampleFrames: uint32,
    pub sampleSize: uint16,
    pub sampleRate: sint32,
    pub extTypeID: aiff_ID,
    pub extName: [::core::ffi::c_char; 32],
}
pub type aiff_ID = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aiff_FileHeader {
    pub chunk: aiff_ChunkHeader,
    pub r#type: aiff_ID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aiff_ChunkHeader {
    pub id: aiff_ID,
    pub size: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aiff_SoundDataChunk {
    pub offset: uint32,
    pub blockSize: uint32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const aifae_None: Self = Self(0);
    pub const aifae_Unknown: Self = Self(-1);
    pub const aifae_BadFile: Self = Self(-2);
    pub const aifae_BadArg: Self = Self(-3);
    pub const aifae_Other: Self = Self(-1000);
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
pub static mut aifa_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(aifa_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        aifa_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(aifa_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(aifa_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        aifa_GetError
            as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(aifa_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(aifa_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        aifa_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(aifa_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        aifa_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        aifa_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(
        aifa_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32,
    ),
};
static mut aifa_formats: *const TFB_DecoderFormats = ::core::ptr::null::<
    TFB_DecoderFormats,
>();
unsafe extern "C" fn aifa_GetName() -> *const ::core::ffi::c_char {
    return b"AIFF\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn aifa_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    aifa_formats = fmts;
    return true;
}
unsafe extern "C" fn aifa_TermModule() {}
unsafe extern "C" fn aifa_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_AiffSoundDecoder>() as uint32;
}
unsafe extern "C" fn aifa_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    let mut aifa: *mut TFB_AiffSoundDecoder = This as *mut TFB_AiffSoundDecoder;
    let mut ret: ::core::ffi::c_int = (*aifa).last_error;
    (*aifa).last_error = 0 as ::core::ffi::c_int as sint32;
    return ret;
}
unsafe extern "C" fn aifa_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    (*This).need_swap = !(*aifa_formats).want_big_endian;
    return true;
}
unsafe extern "C" fn aifa_Term(mut This: *mut TFB_SoundDecoder) {
    uqm_lua_checked!(aifa_Close(This), ());
}
unsafe extern "C" fn read_be_16(mut fp: *mut uio_Stream, mut v: *mut uint16) -> bool {
    if uqm_lua_checked!(uio_fread(
        v as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<uint16>(),
        1 as size_t,
        fp,
    ), false) == 0
    {
        return false;
    }
    *v = UQM_Swap16(*v);
    return true;
}
unsafe extern "C" fn read_be_32(mut fp: *mut uio_Stream, mut v: *mut uint32) -> bool {
    if uqm_lua_checked!(uio_fread(
        v as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<uint32>(),
        1 as size_t,
        fp,
    ), false) == 0
    {
        return false;
    }
    *v = UQM_Swap32(*v);
    return true;
}
unsafe extern "C" fn read_be_f80(mut fp: *mut uio_Stream, mut v: *mut sint32) -> bool {
    let mut sign: ::core::ffi::c_int = 0;
    let mut exp: ::core::ffi::c_int = 0;
    let mut shift: ::core::ffi::c_int = 0;
    let mut se: uint16 = 0;
    let mut mant: uint32 = 0;
    let mut mant_low: uint32 = 0;
    if !uqm_lua_checked!(read_be_16(fp, &raw mut se), false) || !uqm_lua_checked!(read_be_32(fp, &raw mut mant), false)
        || !uqm_lua_checked!(read_be_32(fp, &raw mut mant_low), false)
    {
        return false;
    }
    sign = se as ::core::ffi::c_int >> 15 as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int;
    exp = se as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
    mant >>= 1 as ::core::ffi::c_int;
    exp
        -= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
    shift = exp - 31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    if shift > 0 as ::core::ffi::c_int {
        mant = 0x7fffffff as ::core::ffi::c_int as uint32;
    } else if shift < 0 as ::core::ffi::c_int {
        mant >>= -shift;
    }
    *v = if sign != 0 { -(mant as sint32) } else { mant as sint32 };
    return true;
}
unsafe extern "C" fn aifa_readFileHeader(
    mut aifa: *mut TFB_AiffSoundDecoder,
    mut hdr: *mut aiff_FileHeader,
) -> bool {
    if !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*hdr).chunk.id), false)
        || !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*hdr).chunk.size), false)
        || !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*hdr).r#type), false)
    {
        (*aifa).last_error = *__errno_location() as sint32;
        return false;
    }
    return true;
}
unsafe extern "C" fn aifa_readChunkHeader(
    mut aifa: *mut TFB_AiffSoundDecoder,
    mut hdr: *mut aiff_ChunkHeader,
) -> bool {
    if !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*hdr).id), false)
        || !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*hdr).size), false)
    {
        (*aifa).last_error = *__errno_location() as sint32;
        return false;
    }
    return true;
}
unsafe extern "C" fn aifa_readCommonChunk(
    mut aifa: *mut TFB_AiffSoundDecoder,
    mut size: uint32,
    mut fmt: *mut aiff_ExtCommonChunk,
) -> ::core::ffi::c_int {
    let mut bytes: ::core::ffi::c_int = 0;
    memset(
        fmt as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<aiff_ExtCommonChunk>(),
    );
    if size
        < (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            + 10 as ::core::ffi::c_int) as uint32
    {
        (*aifa).last_error = C2Rust_Unnamed::aifae_BadFile.0 as sint32;
        return 0 as ::core::ffi::c_int;
    }
    if !uqm_lua_checked!(read_be_16((*aifa).fp, &raw mut (*fmt).channels), 0)
        || !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*fmt).sampleFrames), 0)
        || !uqm_lua_checked!(read_be_16((*aifa).fp, &raw mut (*fmt).sampleSize), 0)
        || !uqm_lua_checked!(read_be_f80((*aifa).fp, &raw mut (*fmt).sampleRate), 0)
    {
        (*aifa).last_error = *__errno_location() as sint32;
        return 0 as ::core::ffi::c_int;
    }
    bytes = 2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        + 10 as ::core::ffi::c_int;
    if size
        >= (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            + 10 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32
    {
        if !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*fmt).extTypeID), 0) {
            (*aifa).last_error = *__errno_location() as sint32;
            return 0 as ::core::ffi::c_int;
        }
        bytes = (bytes as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<aiff_ID>() as ::core::ffi::c_ulong)
            as ::core::ffi::c_int;
    }
    return bytes;
}
unsafe extern "C" fn aifa_readSoundDataChunk(
    mut aifa: *mut TFB_AiffSoundDecoder,
    mut data: *mut aiff_SoundDataChunk,
) -> bool {
    if !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*data).offset), false)
        || !uqm_lua_checked!(read_be_32((*aifa).fp, &raw mut (*data).blockSize), false)
    {
        (*aifa).last_error = *__errno_location() as sint32;
        return false;
    }
    return true;
}
unsafe extern "C" fn aifa_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    let mut aifa: *mut TFB_AiffSoundDecoder = This as *mut TFB_AiffSoundDecoder;
    let mut fileHdr: aiff_FileHeader = aiff_FileHeader {
        chunk: aiff_ChunkHeader { id: 0, size: 0 },
        r#type: 0,
    };
    let mut chunkHdr: aiff_ChunkHeader = aiff_ChunkHeader { id: 0, size: 0 };
    let mut remSize: sint32 = 0;
    (*aifa).fp = uqm_lua_checked!(uio_fopen(
        dir,
        filename,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if (*aifa).fp.is_null() {
        (*aifa).last_error = *__errno_location() as sint32;
        return false;
    }
    (*aifa).data_size = 0 as uint32;
    (*aifa).max_pcm = 0 as uint32;
    (*aifa).data_ofs = 0 as uint32;
    memset(
        &raw mut (*aifa).fmtHdr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<aiff_ExtCommonChunk>(),
    );
    memset(
        &raw mut (*aifa).prev_val as *mut sint32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[sint32; 4]>(),
    );
    if !uqm_lua_checked!(aifa_readFileHeader(aifa, &raw mut fileHdr), false) {
        (*aifa).last_error = *__errno_location() as sint32;
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    if fileHdr.chunk.id
        != (('F' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | ('O' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | ('R' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 'M' as ::core::ffi::c_int) as aiff_ID
    {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): not an aiff file, ID 0x%08x\0".as_ptr()
                as *const ::core::ffi::c_char,
            fileHdr.chunk.id,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    if fileHdr.r#type
        != (('A' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | ('I' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | ('F' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 'F' as ::core::ffi::c_int) as aiff_ID
        && fileHdr.r#type
            != (('A' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                | ('I' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | ('F' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 'C' as ::core::ffi::c_int) as aiff_ID
    {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): unsupported aiff file, Type 0x%08x\0".as_ptr()
                as *const ::core::ffi::c_char,
            fileHdr.r#type,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    remSize = (fileHdr.chunk.size as usize)
        .wrapping_sub(::core::mem::size_of::<aiff_ID>()) as sint32;
    while remSize > 0 as ::core::ffi::c_int {
        if !uqm_lua_checked!(aifa_readChunkHeader(aifa, &raw mut chunkHdr), false) {
            uqm_lua_checked!(aifa_Close(This), false);
            return false;
        }
        if chunkHdr.id
            == (('C' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                | ('O' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | ('M' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 'M' as ::core::ffi::c_int) as aiff_ID
        {
            let mut read: ::core::ffi::c_int = uqm_lua_checked!(aifa_readCommonChunk(
                aifa,
                chunkHdr.size,
                &raw mut (*aifa).fmtHdr,
            ), false);
            if read == 0 {
                uqm_lua_checked!(aifa_Close(This), false);
                return false;
            }
            uqm_lua_checked!(uio_fseek(
                (*aifa).fp,
                chunkHdr.size.wrapping_sub(read as uint32) as ::core::ffi::c_long,
                1 as ::core::ffi::c_int,
            ), false);
        } else if chunkHdr.id
            == (('S' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                | ('S' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | ('N' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 'D' as ::core::ffi::c_int) as aiff_ID
        {
            let mut data: aiff_SoundDataChunk = aiff_SoundDataChunk {
                offset: 0,
                blockSize: 0,
            };
            if !uqm_lua_checked!(aifa_readSoundDataChunk(aifa, &raw mut data), false) {
                uqm_lua_checked!(aifa_Close(This), false);
                return false;
            }
            (*aifa).data_ofs = (uqm_lua_checked!(uio_ftell((*aifa).fp), false) as ::core::ffi::c_ulong)
                .wrapping_add(data.offset as ::core::ffi::c_ulong) as uint32;
            uqm_lua_checked!(uio_fseek(
                (*aifa).fp,
                chunkHdr
                    .size
                    .wrapping_sub(
                        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32,
                    ) as ::core::ffi::c_long,
                1 as ::core::ffi::c_int,
            ), false);
        } else {
            uqm_lua_checked!(uio_fseek(
                (*aifa).fp,
                chunkHdr.size as ::core::ffi::c_long,
                1 as ::core::ffi::c_int,
            ), false);
        }
        uqm_lua_checked!(uio_fseek(
            (*aifa).fp,
            (chunkHdr.size & 1 as uint32) as ::core::ffi::c_long,
            1 as ::core::ffi::c_int,
        ), false);
        remSize = (remSize as uint32)
            .wrapping_sub(
                (chunkHdr.size.wrapping_add(1 as uint32)
                    & !(1 as ::core::ffi::c_int) as uint32)
                    .wrapping_add(
                        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32,
                    ),
            ) as sint32;
    }
    if (*aifa).fmtHdr.sampleFrames == 0 as uint32 {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): aiff file has no sound data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    (*aifa).bits_per_sample = ((*aifa).fmtHdr.sampleSize as ::core::ffi::c_int
        + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    if (*aifa).bits_per_sample == 0 as ::core::ffi::c_uint
        || (*aifa).bits_per_sample > 16 as ::core::ffi::c_uint
    {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): unsupported sample size %u\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*aifa).bits_per_sample,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    if (*aifa).fmtHdr.channels as ::core::ffi::c_int != 1 as ::core::ffi::c_int
        && (*aifa).fmtHdr.channels as ::core::ffi::c_int != 2 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): unsupported number of channels %u\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*aifa).fmtHdr.channels as ::core::ffi::c_uint,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    if (*aifa).fmtHdr.sampleRate < 300 as ::core::ffi::c_int
        || (*aifa).fmtHdr.sampleRate > 128000 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): unsupported sampling rate %ld\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*aifa).fmtHdr.sampleRate as ::core::ffi::c_long,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    (*aifa).block_align = (*aifa)
        .bits_per_sample
        .wrapping_div(8 as ::core::ffi::c_uint)
        .wrapping_mul((*aifa).fmtHdr.channels as ::core::ffi::c_uint);
    (*aifa).file_block = (*aifa).block_align;
    if (*aifa).data_ofs == 0 {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): bad aiff file, no SSND chunk found\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    if fileHdr.r#type
        == (('A' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | ('I' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | ('F' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 'F' as ::core::ffi::c_int) as aiff_ID
    {
        if (*aifa).fmtHdr.extTypeID != 0 as aiff_ID {
            log_add(
                log_Level::log_Warning,
                b"aifa_Open(): unsupported extension 0x%08x\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*aifa).fmtHdr.extTypeID,
            );
            uqm_lua_checked!(aifa_Close(This), false);
            return false;
        }
        (*aifa).comp_type = aiff_CompressionType::aifc_None;
    } else if fileHdr.r#type
        == (('A' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | ('I' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | ('F' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 'C' as ::core::ffi::c_int) as aiff_ID
    {
        if (*aifa).fmtHdr.extTypeID
            != (('S' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                | ('D' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | ('X' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | '2' as ::core::ffi::c_int) as aiff_ID
        {
            log_add(
                log_Level::log_Warning,
                b"aifa_Open(): unsupported compression 0x%08x\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*aifa).fmtHdr.extTypeID,
            );
            uqm_lua_checked!(aifa_Close(This), false);
            return false;
        }
        (*aifa).comp_type = aiff_CompressionType::aifc_Sdx2;
        (*aifa).file_block = (*aifa).file_block.wrapping_div(2 as ::core::ffi::c_uint);
        ((*aifa).fmtHdr.channels as ::core::ffi::c_int <= 4 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"aifa->fmtHdr.channels <= MAX_CHANNELS\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/libs/sound/decoders/aiffaud.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    498 as ::core::ffi::c_int,
                    b"aifa_Open\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        (*This).need_swap = (*aifa_formats).big_endian as ::core::ffi::c_int
            != (*aifa_formats).want_big_endian as ::core::ffi::c_int;
    }
    (*aifa).data_size = (*aifa).fmtHdr.sampleFrames.wrapping_mul((*aifa).file_block)
        as uint32;
    if (*aifa).comp_type.0 == aiff_CompressionType::aifc_Sdx2.0
        && (*aifa).bits_per_sample != 16 as ::core::ffi::c_uint
    {
        log_add(
            log_Level::log_Warning,
            b"aifa_Open(): unsupported sample size %u for SDX2\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*aifa).fmtHdr.sampleSize as ::core::ffi::c_uint,
        );
        uqm_lua_checked!(aifa_Close(This), false);
        return false;
    }
    (*This).format = if (*aifa).fmtHdr.channels as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int
    {
        if (*aifa).bits_per_sample == 8 as ::core::ffi::c_uint {
            (*aifa_formats).mono8
        } else {
            (*aifa_formats).mono16
        }
    } else if (*aifa).bits_per_sample == 8 as ::core::ffi::c_uint {
        (*aifa_formats).stereo8
    } else {
        (*aifa_formats).stereo16
    };
    (*This).frequency = (*aifa).fmtHdr.sampleRate as uint32;
    uqm_lua_checked!(uio_fseek(
        (*aifa).fp,
        (*aifa).data_ofs as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    ), false);
    (*aifa).max_pcm = (*aifa).fmtHdr.sampleFrames;
    (*aifa).cur_pcm = 0 as uint32;
    (*This).length = (*aifa).max_pcm as ::core::ffi::c_float
        / (*aifa).fmtHdr.sampleRate as ::core::ffi::c_float;
    (*This).filename_hash = crc32b(filename) as uint32;
    (*aifa).last_error = 0 as ::core::ffi::c_int as sint32;
    return true;
}
unsafe extern "C" fn aifa_Close(mut This: *mut TFB_SoundDecoder) {
    let mut aifa: *mut TFB_AiffSoundDecoder = This as *mut TFB_AiffSoundDecoder;
    if !(*aifa).fp.is_null() {
        uqm_lua_checked!(uio_fclose((*aifa).fp), ());
        (*aifa).fp = ::core::ptr::null_mut::<uio_Stream>();
    }
}
unsafe extern "C" fn aifa_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut aifa: *mut TFB_AiffSoundDecoder = This as *mut TFB_AiffSoundDecoder;
    match (*aifa).comp_type {
        aiff_CompressionType::aifc_None => return uqm_lua_checked!(aifa_DecodePCM(aifa, buf, bufsize), 0),
        aiff_CompressionType::aifc_Sdx2 => return uqm_lua_checked!(aifa_DecodeSDX2(aifa, buf, bufsize), 0),
        _ => {
            (false
                && !(b"Unknown comp_type\0".as_ptr() as *const ::core::ffi::c_char)
                    .is_null()
                || {
                    __assert_fail(
                        b"false && \"Unknown comp_type\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/libs/sound/decoders/aiffaud.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        558 as ::core::ffi::c_int,
                        b"aifa_Decode\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn aifa_DecodePCM(
    mut aifa: *mut TFB_AiffSoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut dec_pcm: uint32 = 0;
    let mut size: uint32 = 0;
    dec_pcm = (bufsize as ::core::ffi::c_uint).wrapping_div((*aifa).block_align)
        as uint32;
    if dec_pcm > (*aifa).max_pcm.wrapping_sub((*aifa).cur_pcm) {
        dec_pcm = (*aifa).max_pcm.wrapping_sub((*aifa).cur_pcm);
    }
    dec_pcm = uqm_lua_checked!(uio_fread(buf, (*aifa).file_block as size_t, dec_pcm as size_t, (*aifa).fp), 0)
        as uint32;
    (*aifa).cur_pcm = (*aifa).cur_pcm.wrapping_add(dec_pcm);
    size = dec_pcm.wrapping_mul((*aifa).block_align) as uint32;
    if (*aifa).bits_per_sample == 8 as ::core::ffi::c_uint {
        let mut ptr: *mut uint8 = buf as *mut uint8;
        let mut left: uint32 = 0;
        left = size;
        while left > 0 as uint32 {
            *ptr = (*ptr as ::core::ffi::c_int + 128 as ::core::ffi::c_int) as uint8;
            left = left.wrapping_sub(1);
            ptr = ptr.offset(1);
        }
    }
    return size as ::core::ffi::c_int;
}
unsafe extern "C" fn aifa_DecodeSDX2(
    mut aifa: *mut TFB_AiffSoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut dec_pcm: uint32 = 0;
    let mut src: *mut sint8 = ::core::ptr::null_mut::<sint8>();
    let mut dst: *mut sint16 = buf as *mut sint16;
    let mut left: uint32 = 0;
    dec_pcm = (bufsize as ::core::ffi::c_uint).wrapping_div((*aifa).block_align)
        as uint32;
    if dec_pcm > (*aifa).max_pcm.wrapping_sub((*aifa).cur_pcm) {
        dec_pcm = (*aifa).max_pcm.wrapping_sub((*aifa).cur_pcm);
    }
    src = (buf as *mut sint8)
        .offset(bufsize as isize)
        .offset(-(dec_pcm.wrapping_mul((*aifa).file_block) as isize));
    dec_pcm = uqm_lua_checked!(uio_fread(
        src as *mut ::core::ffi::c_void,
        (*aifa).file_block as size_t,
        dec_pcm as size_t,
        (*aifa).fp,
    ), 0) as uint32;
    (*aifa).cur_pcm = (*aifa).cur_pcm.wrapping_add(dec_pcm);
    left = dec_pcm;
    while left > 0 as uint32 {
        let mut i: ::core::ffi::c_int = 0;
        let mut prev: *mut sint32 = &raw mut (*aifa).prev_val as *mut sint32;
        i = (*aifa).fmtHdr.channels as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            let mut v: sint32 = *src as sint32 * abs(*src as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if *src as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                v += *prev;
            }
            if v > 32767 as ::core::ffi::c_int {
                v = 32767 as ::core::ffi::c_int as sint32;
            } else if v < -32768 as ::core::ffi::c_int {
                v = -32768 as ::core::ffi::c_int as sint32;
            }
            *prev = v;
            *dst = v as sint16;
            i -= 1;
            prev = prev.offset(1);
            src = src.offset(1);
            dst = dst.offset(1);
        }
        left = left.wrapping_sub(1);
    }
    return dec_pcm.wrapping_mul((*aifa).block_align) as ::core::ffi::c_int;
}
unsafe extern "C" fn aifa_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut aifa: *mut TFB_AiffSoundDecoder = This as *mut TFB_AiffSoundDecoder;
    if pcm_pos > (*aifa).max_pcm {
        pcm_pos = (*aifa).max_pcm;
    }
    (*aifa).cur_pcm = pcm_pos;
    uqm_lua_checked!(uio_fseek(
        (*aifa).fp,
        (*aifa).data_ofs.wrapping_add(pcm_pos.wrapping_mul((*aifa).file_block))
            as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    ), 0);
    memset(
        &raw mut (*aifa).prev_val as *mut sint32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[sint32; 4]>(),
    );
    return pcm_pos;
}
unsafe extern "C" fn aifa_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    return 0 as uint32;
}
