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

/* Wave decoder (.wav adapter)
 * Code is based on Creative's Win32 OpenAL implementation.
 */

// Rust translation of src/libs/sound/decoders/wav.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn crc32b(str: *const ::core::ffi::c_char) -> uint32_t;
}
pub type size_t = usize;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint16 = ::core::ffi::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wave_FormatHeader {
    pub format: uint16,
    pub channels: uint16,
    pub samplesPerSec: uint32,
    pub bytesPerSec: uint32,
    pub blockAlign: uint16,
    pub bitsPerSample: uint16,
}
pub type TFB_WaveSoundDecoder = tfb_wavesounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_wavesounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub last_error: sint32,
    pub fp: *mut uio_Stream,
    pub fmtHdr: wave_FormatHeader,
    pub data_ofs: uint32,
    pub data_size: uint32,
    pub max_pcm: uint32,
    pub cur_pcm: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wave_ChunkHeader {
    pub id: uint32,
    pub size: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wave_FileHeader {
    pub id: uint32,
    pub size: uint32,
    pub r#type: uint32,
}
#[no_mangle]
pub static mut wava_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(wava_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        wava_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(wava_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(wava_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        wava_GetError
            as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(wava_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(wava_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        wava_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(wava_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        wava_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        wava_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(
        wava_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32,
    ),
};
static mut wava_formats: *const TFB_DecoderFormats = ::core::ptr::null::<
    TFB_DecoderFormats,
>();
unsafe extern "C" fn wava_GetName() -> *const ::core::ffi::c_char {
    return b"Wave\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn wava_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    wava_formats = fmts;
    return true;
}
unsafe extern "C" fn wava_TermModule() {}
unsafe extern "C" fn wava_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_WaveSoundDecoder>() as uint32;
}
unsafe extern "C" fn wava_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    let mut wava: *mut TFB_WaveSoundDecoder = This as *mut TFB_WaveSoundDecoder;
    let mut ret: ::core::ffi::c_int = (*wava).last_error;
    (*wava).last_error = 0 as ::core::ffi::c_int as sint32;
    return ret;
}
unsafe extern "C" fn wava_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    (*This).need_swap = (*wava_formats).want_big_endian;
    return true;
}
unsafe extern "C" fn wava_Term(mut This: *mut TFB_SoundDecoder) {
    uqm_lua_checked!(wava_Close(This), ());
}
unsafe extern "C" fn read_le_16(mut fp: *mut uio_Stream, mut v: *mut uint16) -> bool {
    if uqm_lua_checked!(uio_fread(
        v as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<uint16>(),
        1 as size_t,
        fp,
    ), false) == 0
    {
        return false;
    }
    *v = *v;
    return true;
}
unsafe extern "C" fn read_le_32(mut fp: *mut uio_Stream, mut v: *mut uint32) -> bool {
    if uqm_lua_checked!(uio_fread(
        v as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<uint32>(),
        1 as size_t,
        fp,
    ), false) == 0
    {
        return false;
    }
    *v = *v;
    return true;
}
unsafe extern "C" fn wava_readFileHeader(
    mut wava: *mut TFB_WaveSoundDecoder,
    mut hdr: *mut wave_FileHeader,
) -> bool {
    if !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*hdr).id), false)
        || !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*hdr).size), false)
        || !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*hdr).r#type), false)
    {
        (*wava).last_error = *__errno_location() as sint32;
        return false;
    }
    return true;
}
unsafe extern "C" fn wava_readChunkHeader(
    mut wava: *mut TFB_WaveSoundDecoder,
    mut chunk: *mut wave_ChunkHeader,
) -> bool {
    if !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*chunk).id), false)
        || !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*chunk).size), false)
    {
        (*wava).last_error = *__errno_location() as sint32;
        return false;
    }
    return true;
}
unsafe extern "C" fn wava_readFormatHeader(
    mut wava: *mut TFB_WaveSoundDecoder,
    mut fmt: *mut wave_FormatHeader,
) -> bool {
    if !uqm_lua_checked!(read_le_16((*wava).fp, &raw mut (*fmt).format), false)
        || !uqm_lua_checked!(read_le_16((*wava).fp, &raw mut (*fmt).channels), false)
        || !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*fmt).samplesPerSec), false)
        || !uqm_lua_checked!(read_le_32((*wava).fp, &raw mut (*fmt).bytesPerSec), false)
        || !uqm_lua_checked!(read_le_16((*wava).fp, &raw mut (*fmt).blockAlign), false)
        || !uqm_lua_checked!(read_le_16((*wava).fp, &raw mut (*fmt).bitsPerSample), false)
    {
        (*wava).last_error = *__errno_location() as sint32;
        return false;
    }
    return true;
}
unsafe extern "C" fn wava_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    let mut wava: *mut TFB_WaveSoundDecoder = This as *mut TFB_WaveSoundDecoder;
    let mut fileHdr: wave_FileHeader = wave_FileHeader {
        id: 0,
        size: 0,
        r#type: 0,
    };
    let mut chunkHdr: wave_ChunkHeader = wave_ChunkHeader { id: 0, size: 0 };
    let mut dataLeft: ::core::ffi::c_long = 0;
    (*wava).fp = uqm_lua_checked!(uio_fopen(
        dir,
        filename,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if (*wava).fp.is_null() {
        (*wava).last_error = *__errno_location() as sint32;
        return false;
    }
    (*wava).data_size = 0 as uint32;
    (*wava).data_ofs = 0 as uint32;
    if !uqm_lua_checked!(wava_readFileHeader(wava, &raw mut fileHdr), false) {
        (*wava).last_error = *__errno_location() as sint32;
        uqm_lua_checked!(wava_Close(This), false);
        return false;
    }
    if fileHdr.id
        != (('F' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | ('F' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | ('I' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 'R' as ::core::ffi::c_int) as uint32
        || fileHdr.r#type
            != (('E' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                | ('V' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | ('A' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 'W' as ::core::ffi::c_int) as uint32
    {
        log_add(
            log_Level::log_Warning,
            b"wava_Open(): not a wave file, ID 0x%08x, Type 0x%08x\0".as_ptr()
                as *const ::core::ffi::c_char,
            fileHdr.id,
            fileHdr.r#type,
        );
        uqm_lua_checked!(wava_Close(This), false);
        return false;
    }
    dataLeft = (fileHdr.size.wrapping_add(1 as uint32)
        & !(1 as ::core::ffi::c_int) as uint32)
        .wrapping_sub(4 as uint32) as ::core::ffi::c_long;
    while dataLeft > 0 as ::core::ffi::c_long {
        if !uqm_lua_checked!(wava_readChunkHeader(wava, &raw mut chunkHdr), false) {
            uqm_lua_checked!(wava_Close(This), false);
            return false;
        }
        if chunkHdr.id
            == ((' ' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                | ('t' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | ('m' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 'f' as ::core::ffi::c_int) as uint32
        {
            if !uqm_lua_checked!(wava_readFormatHeader(wava, &raw mut (*wava).fmtHdr), false) {
                uqm_lua_checked!(wava_Close(This), false);
                return false;
            }
            uqm_lua_checked!(uio_fseek(
                (*wava).fp,
                chunkHdr.size.wrapping_sub(16 as uint32) as ::core::ffi::c_long,
                1 as ::core::ffi::c_int,
            ), false);
        } else {
            if chunkHdr.id
                == (('a' as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                    | ('t' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                    | ('a' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    | 'd' as ::core::ffi::c_int) as uint32
            {
                (*wava).data_size = chunkHdr.size;
                (*wava).data_ofs = uqm_lua_checked!(uio_ftell((*wava).fp), false) as uint32;
            }
            uqm_lua_checked!(uio_fseek(
                (*wava).fp,
                chunkHdr.size as ::core::ffi::c_long,
                1 as ::core::ffi::c_int,
            ), false);
        }
        uqm_lua_checked!(uio_fseek(
            (*wava).fp,
            (chunkHdr.size & 1 as uint32) as ::core::ffi::c_long,
            1 as ::core::ffi::c_int,
        ), false);
        dataLeft = (dataLeft as ::core::ffi::c_ulong)
            .wrapping_sub(
                (chunkHdr.size.wrapping_add(1 as uint32)
                    & !(1 as ::core::ffi::c_int) as uint32)
                    .wrapping_add(8 as uint32) as ::core::ffi::c_ulong,
            ) as ::core::ffi::c_long;
    }
    if (*wava).data_size == 0 || (*wava).data_ofs == 0 {
        log_add(
            log_Level::log_Warning,
            b"wava_Open(): bad wave file, no DATA chunk found\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(wava_Close(This), false);
        return false;
    }
    if (*wava).fmtHdr.format as ::core::ffi::c_int != 0x1 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"wava_Open(): unsupported format %x\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*wava).fmtHdr.format as ::core::ffi::c_int,
        );
        uqm_lua_checked!(wava_Close(This), false);
        return false;
    }
    if (*wava).fmtHdr.channels as ::core::ffi::c_int != 1 as ::core::ffi::c_int
        && (*wava).fmtHdr.channels as ::core::ffi::c_int != 2 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"wava_Open(): unsupported number of channels %u\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*wava).fmtHdr.channels as ::core::ffi::c_uint,
        );
        uqm_lua_checked!(wava_Close(This), false);
        return false;
    }
    if dataLeft != 0 as ::core::ffi::c_long {
        log_add(
            log_Level::log_Warning,
            b"wava_Open(): bad or unsupported wave file, size in header does not match read chunks\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    (*This).format = if (*wava).fmtHdr.channels as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int
    {
        if (*wava).fmtHdr.bitsPerSample as ::core::ffi::c_int == 8 as ::core::ffi::c_int
        {
            (*wava_formats).mono8
        } else {
            (*wava_formats).mono16
        }
    } else if (*wava).fmtHdr.bitsPerSample as ::core::ffi::c_int
        == 8 as ::core::ffi::c_int
    {
        (*wava_formats).stereo8
    } else {
        (*wava_formats).stereo16
    };
    (*This).frequency = (*wava).fmtHdr.samplesPerSec;
    (*This).filename_hash = crc32b(filename) as uint32;
    uqm_lua_checked!(uio_fseek(
        (*wava).fp,
        (*wava).data_ofs as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    ), false);
    (*wava).max_pcm = (*wava)
        .data_size
        .wrapping_div((*wava).fmtHdr.blockAlign as uint32);
    (*wava).cur_pcm = 0 as uint32;
    (*This).length = (*wava).max_pcm as ::core::ffi::c_float
        / (*wava).fmtHdr.samplesPerSec as ::core::ffi::c_float;
    (*wava).last_error = 0 as ::core::ffi::c_int as sint32;
    return true;
}
unsafe extern "C" fn wava_Close(mut This: *mut TFB_SoundDecoder) {
    let mut wava: *mut TFB_WaveSoundDecoder = This as *mut TFB_WaveSoundDecoder;
    if !(*wava).fp.is_null() {
        uqm_lua_checked!(uio_fclose((*wava).fp), ());
        (*wava).fp = ::core::ptr::null_mut::<uio_Stream>();
    }
}
unsafe extern "C" fn wava_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut wava: *mut TFB_WaveSoundDecoder = This as *mut TFB_WaveSoundDecoder;
    let mut dec_pcm: uint32 = 0;
    dec_pcm = (bufsize / (*wava).fmtHdr.blockAlign as ::core::ffi::c_int) as uint32;
    if dec_pcm > (*wava).max_pcm.wrapping_sub((*wava).cur_pcm) {
        dec_pcm = (*wava).max_pcm.wrapping_sub((*wava).cur_pcm);
    }
    dec_pcm = uqm_lua_checked!(uio_fread(
        buf,
        (*wava).fmtHdr.blockAlign as size_t,
        dec_pcm as size_t,
        (*wava).fp,
    ), 0) as uint32;
    (*wava).cur_pcm = (*wava).cur_pcm.wrapping_add(dec_pcm);
    return dec_pcm.wrapping_mul((*wava).fmtHdr.blockAlign as uint32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn wava_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut wava: *mut TFB_WaveSoundDecoder = This as *mut TFB_WaveSoundDecoder;
    if pcm_pos > (*wava).max_pcm {
        pcm_pos = (*wava).max_pcm;
    }
    (*wava).cur_pcm = pcm_pos;
    uqm_lua_checked!(uio_fseek(
        (*wava).fp,
        (*wava)
            .data_ofs
            .wrapping_add(pcm_pos.wrapping_mul((*wava).fmtHdr.blockAlign as uint32))
            as ::core::ffi::c_long,
        0 as ::core::ffi::c_int,
    ), 0);
    return pcm_pos;
}
unsafe extern "C" fn wava_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    return 0 as uint32;
}
