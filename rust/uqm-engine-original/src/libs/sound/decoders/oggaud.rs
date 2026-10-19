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

/* Ogg Vorbis decoder (.ogg adapter)
 */

// Rust translation of src/libs/sound/decoders/oggaud.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
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
    fn ov_clear(vf: *mut OggVorbis_File) -> ::core::ffi::c_int;
    fn ov_open_callbacks(
        datasource: *mut ::core::ffi::c_void,
        vf: *mut OggVorbis_File,
        initial: *const ::core::ffi::c_char,
        ibytes: ::core::ffi::c_long,
        callbacks: ov_callbacks,
    ) -> ::core::ffi::c_int;
    fn ov_time_total(
        vf: *mut OggVorbis_File,
        i: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn ov_pcm_seek(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> ::core::ffi::c_int;
    fn ov_pcm_tell(vf: *mut OggVorbis_File) -> ogg_int64_t;
    fn ov_info(vf: *mut OggVorbis_File, link: ::core::ffi::c_int) -> *mut vorbis_info;
    fn ov_read(
        vf: *mut OggVorbis_File,
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        bigendianp: ::core::ffi::c_int,
        word: ::core::ffi::c_int,
        sgned: ::core::ffi::c_int,
        bitstream: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
pub type size_t = usize;
pub type int64_t = ::core::ffi::c_longlong;
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
pub type ogg_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oggpack_buffer {
    pub endbyte: ::core::ffi::c_long,
    pub endbit: ::core::ffi::c_int,
    pub buffer: *mut ::core::ffi::c_uchar,
    pub ptr: *mut ::core::ffi::c_uchar,
    pub storage: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_stream_state {
    pub body_data: *mut ::core::ffi::c_uchar,
    pub body_storage: ::core::ffi::c_long,
    pub body_fill: ::core::ffi::c_long,
    pub body_returned: ::core::ffi::c_long,
    pub lacing_vals: *mut ::core::ffi::c_int,
    pub granule_vals: *mut ogg_int64_t,
    pub lacing_storage: ::core::ffi::c_long,
    pub lacing_fill: ::core::ffi::c_long,
    pub lacing_packet: ::core::ffi::c_long,
    pub lacing_returned: ::core::ffi::c_long,
    pub header: [::core::ffi::c_uchar; 282],
    pub header_fill: ::core::ffi::c_int,
    pub e_o_s: ::core::ffi::c_int,
    pub b_o_s: ::core::ffi::c_int,
    pub serialno: ::core::ffi::c_long,
    pub pageno: ::core::ffi::c_long,
    pub packetno: ogg_int64_t,
    pub granulepos: ogg_int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_sync_state {
    pub data: *mut ::core::ffi::c_uchar,
    pub storage: ::core::ffi::c_int,
    pub fill: ::core::ffi::c_int,
    pub returned: ::core::ffi::c_int,
    pub unsynced: ::core::ffi::c_int,
    pub headerbytes: ::core::ffi::c_int,
    pub bodybytes: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_info {
    pub version: ::core::ffi::c_int,
    pub channels: ::core::ffi::c_int,
    pub rate: ::core::ffi::c_long,
    pub bitrate_upper: ::core::ffi::c_long,
    pub bitrate_nominal: ::core::ffi::c_long,
    pub bitrate_lower: ::core::ffi::c_long,
    pub bitrate_window: ::core::ffi::c_long,
    pub codec_setup: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_dsp_state {
    pub analysisp: ::core::ffi::c_int,
    pub vi: *mut vorbis_info,
    pub pcm: *mut *mut ::core::ffi::c_float,
    pub pcmret: *mut *mut ::core::ffi::c_float,
    pub pcm_storage: ::core::ffi::c_int,
    pub pcm_current: ::core::ffi::c_int,
    pub pcm_returned: ::core::ffi::c_int,
    pub preextrapolate: ::core::ffi::c_int,
    pub eofflag: ::core::ffi::c_int,
    pub lW: ::core::ffi::c_long,
    pub W: ::core::ffi::c_long,
    pub nW: ::core::ffi::c_long,
    pub centerW: ::core::ffi::c_long,
    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,
    pub glue_bits: ogg_int64_t,
    pub time_bits: ogg_int64_t,
    pub floor_bits: ogg_int64_t,
    pub res_bits: ogg_int64_t,
    pub backend_state: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_block {
    pub pcm: *mut *mut ::core::ffi::c_float,
    pub opb: oggpack_buffer,
    pub lW: ::core::ffi::c_long,
    pub W: ::core::ffi::c_long,
    pub nW: ::core::ffi::c_long,
    pub pcmend: ::core::ffi::c_int,
    pub mode: ::core::ffi::c_int,
    pub eofflag: ::core::ffi::c_int,
    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,
    pub vd: *mut vorbis_dsp_state,
    pub localstore: *mut ::core::ffi::c_void,
    pub localtop: ::core::ffi::c_long,
    pub localalloc: ::core::ffi::c_long,
    pub totaluse: ::core::ffi::c_long,
    pub reap: *mut alloc_chain,
    pub glue_bits: ::core::ffi::c_long,
    pub time_bits: ::core::ffi::c_long,
    pub floor_bits: ::core::ffi::c_long,
    pub res_bits: ::core::ffi::c_long,
    pub internal: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_chain {
    pub ptr: *mut ::core::ffi::c_void,
    pub next: *mut alloc_chain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_comment {
    pub user_comments: *mut *mut ::core::ffi::c_char,
    pub comment_lengths: *mut ::core::ffi::c_int,
    pub comments: ::core::ffi::c_int,
    pub vendor: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ov_callbacks {
    pub read_func: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            size_t,
            size_t,
            *mut ::core::ffi::c_void,
        ) -> size_t,
    >,
    pub seek_func: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ogg_int64_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub close_func: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub tell_func: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_long,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OggVorbis_File {
    pub datasource: *mut ::core::ffi::c_void,
    pub seekable: ::core::ffi::c_int,
    pub offset: ogg_int64_t,
    pub end: ogg_int64_t,
    pub oy: ogg_sync_state,
    pub links: ::core::ffi::c_int,
    pub offsets: *mut ogg_int64_t,
    pub dataoffsets: *mut ogg_int64_t,
    pub serialnos: *mut ::core::ffi::c_long,
    pub pcmlengths: *mut ogg_int64_t,
    pub vi: *mut vorbis_info,
    pub vc: *mut vorbis_comment,
    pub pcm_offset: ogg_int64_t,
    pub ready_state: ::core::ffi::c_int,
    pub current_serialno: ::core::ffi::c_long,
    pub current_link: ::core::ffi::c_int,
    pub bittrack: ::core::ffi::c_double,
    pub samptrack: ::core::ffi::c_double,
    pub os: ogg_stream_state,
    pub vd: vorbis_dsp_state,
    pub vb: vorbis_block,
    pub callbacks: ov_callbacks,
}
pub type TFB_OggSoundDecoder = tfb_oggsounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_oggsounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub last_error: sint32,
    pub vf: OggVorbis_File,
}
#[no_mangle]
pub static mut ova_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(ova_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        ova_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(ova_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(ova_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        ova_GetError as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(ova_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(ova_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        ova_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(ova_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        ova_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        ova_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(ova_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32),
};
static mut ova_formats: *const TFB_DecoderFormats = ::core::ptr::null::<
    TFB_DecoderFormats,
>();
unsafe extern "C" fn ogg_read(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut datasource: *mut ::core::ffi::c_void,
) -> size_t {
    return uio_fread(ptr, size, nmemb, datasource as *mut uio_Stream);
}
unsafe extern "C" fn ogg_seek(
    mut datasource: *mut ::core::ffi::c_void,
    mut offset: ogg_int64_t,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut off: ::core::ffi::c_long = offset as ::core::ffi::c_long;
    return uio_fseek(datasource as *mut uio_Stream, off, whence);
}
unsafe extern "C" fn ogg_close(
    mut datasource: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return uio_fclose(datasource as *mut uio_Stream);
}
unsafe extern "C" fn ogg_tell(
    mut datasource: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_long {
    return uio_ftell(datasource as *mut uio_Stream);
}
static mut ogg_callbacks: ov_callbacks = ov_callbacks {
    read_func: Some(
        ogg_read
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                size_t,
                size_t,
                *mut ::core::ffi::c_void,
            ) -> size_t,
    ),
    seek_func: Some(
        ogg_seek
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ogg_int64_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    close_func: Some(
        ogg_close as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ),
    tell_func: Some(
        ogg_tell as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_long,
    ),
};
unsafe extern "C" fn ova_GetName() -> *const ::core::ffi::c_char {
    return b"Ogg Vorbis\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn ova_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    ova_formats = fmts;
    return true;
}
unsafe extern "C" fn ova_TermModule() {}
unsafe extern "C" fn ova_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_OggSoundDecoder>() as uint32;
}
unsafe extern "C" fn ova_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    let mut ova: *mut TFB_OggSoundDecoder = This as *mut TFB_OggSoundDecoder;
    let mut ret: ::core::ffi::c_int = (*ova).last_error;
    (*ova).last_error = 0 as ::core::ffi::c_int as sint32;
    return ret;
}
unsafe extern "C" fn ova_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    (*This).need_swap = false;
    return true;
}
unsafe extern "C" fn ova_Term(mut This: *mut TFB_SoundDecoder) {
    ova_Close(This);
}
unsafe extern "C" fn ova_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    let mut ova: *mut TFB_OggSoundDecoder = This as *mut TFB_OggSoundDecoder;
    let mut rc: ::core::ffi::c_int = 0;
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut vinfo: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
    fp = uio_fopen(dir, filename, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if fp.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ova_Open(): could not open %s\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        return false;
    }
    rc = ov_open_callbacks(
        fp as *mut ::core::ffi::c_void,
        &raw mut (*ova).vf,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_long,
        ogg_callbacks,
    );
    if rc != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"ova_Open(): ov_open_callbacks failed for %s, error code %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            filename,
            rc,
        );
        uio_fclose(fp);
        return false;
    }
    vinfo = ov_info(&raw mut (*ova).vf, -1 as ::core::ffi::c_int);
    if vinfo.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ova_Open(): failed to retrieve ogg bitstream info for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            filename,
        );
        ov_clear(&raw mut (*ova).vf);
        return false;
    }
    (*This).frequency = (*vinfo).rate as uint32;
    (*This).length = ov_time_total(&raw mut (*ova).vf, -1 as ::core::ffi::c_int)
        as ::core::ffi::c_float;
    if (*vinfo).channels == 1 as ::core::ffi::c_int {
        (*This).format = (*ova_formats).mono16;
    } else {
        (*This).format = (*ova_formats).stereo16;
    }
    (*ova).last_error = 0 as ::core::ffi::c_int as sint32;
    return true;
}
unsafe extern "C" fn ova_Close(mut This: *mut TFB_SoundDecoder) {
    let mut ova: *mut TFB_OggSoundDecoder = This as *mut TFB_OggSoundDecoder;
    ov_clear(&raw mut (*ova).vf);
}
unsafe extern "C" fn ova_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut ova: *mut TFB_OggSoundDecoder = This as *mut TFB_OggSoundDecoder;
    let mut rc: ::core::ffi::c_long = 0;
    let mut bitstream: ::core::ffi::c_int = 0;
    rc = ov_read(
        &raw mut (*ova).vf,
        buf as *mut ::core::ffi::c_char,
        bufsize,
        (*ova_formats).want_big_endian as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        &raw mut bitstream,
    );
    if rc < 0 as ::core::ffi::c_long {
        (*ova).last_error = rc as sint32;
    } else {
        (*ova).last_error = 0 as ::core::ffi::c_int as sint32;
    }
    return rc as ::core::ffi::c_int;
}
unsafe extern "C" fn ova_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut ova: *mut TFB_OggSoundDecoder = This as *mut TFB_OggSoundDecoder;
    let mut ret: ::core::ffi::c_int = 0;
    ret = ov_pcm_seek(&raw mut (*ova).vf, pcm_pos as ogg_int64_t);
    if ret != 0 as ::core::ffi::c_int {
        (*ova).last_error = ret as sint32;
        return ov_pcm_tell(&raw mut (*ova).vf) as uint32;
    } else {
        return pcm_pos
    };
}
unsafe extern "C" fn ova_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    let mut ova: *mut TFB_OggSoundDecoder = This as *mut TFB_OggSoundDecoder;
    return (*ova).vf.os.pageno as uint32;
}
