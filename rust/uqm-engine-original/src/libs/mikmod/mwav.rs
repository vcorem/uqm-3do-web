/*	MikMod sound library
	(c) 2004, Raphael Assenat
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

  WAV sample loader

==============================================================================*/

/*
   FIXME: Stereo .WAV files are not yet supported as samples.
*/

// Rust translation of src/libs/mikmod/mwav.c.
extern "C" {
    pub type _IO_FILE;
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    static mut MikMod_errno: ::core::ffi::c_int;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    static mut md_mode: UWORD;
    fn _mm_new_mem_reader(
        buffer: *const ::core::ffi::c_void,
        len: ::core::ffi::c_long,
    ) -> *mut MREADER;
    fn _mm_delete_mem_reader(reader: *mut MREADER);
    fn _mm_new_file_reader(fp: *mut FILE) -> *mut MREADER;
    fn _mm_delete_file_reader(_: *mut MREADER);
    fn _mm_fopen(_: *const CHAR, _: *const CHAR) -> *mut FILE;
    fn _mm_fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn _mm_read_string(_: *mut CHAR, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn _mm_read_I_UWORD(_: *mut MREADER) -> UWORD;
    fn _mm_read_I_ULONG(_: *mut MREADER) -> ULONG;
    fn SL_LoadSamples() -> ::core::ffi::c_int;
    fn SL_RegisterSample(
        _: *mut SAMPLE,
        _: ::core::ffi::c_int,
        _: *mut MREADER,
    ) -> *mut SAMPLOAD;
    fn MD_SampleUnload(_: SWORD);
}
pub type size_t = ::core::ffi::c_ulong;
pub type FILE = _IO_FILE;
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
pub type UBYTE = ::core::ffi::c_uchar;
pub type SWORD = ::core::ffi::c_short;
pub type UWORD = ::core::ffi::c_ushort;
pub type ULONG = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const MMERR_OPENING_FILE: Self = Self(1);
    pub const MMERR_OUT_OF_MEMORY: Self = Self(2);
    pub const MMERR_DYNAMIC_LINKING: Self = Self(3);
    pub const MMERR_SAMPLE_TOO_BIG: Self = Self(4);
    pub const MMERR_OUT_OF_HANDLES: Self = Self(5);
    pub const MMERR_UNKNOWN_WAVE_TYPE: Self = Self(6);
    pub const MMERR_LOADING_PATTERN: Self = Self(7);
    pub const MMERR_LOADING_TRACK: Self = Self(8);
    pub const MMERR_LOADING_HEADER: Self = Self(9);
    pub const MMERR_LOADING_SAMPLEINFO: Self = Self(10);
    pub const MMERR_NOT_A_MODULE: Self = Self(11);
    pub const MMERR_NOT_A_STREAM: Self = Self(12);
    pub const MMERR_MED_SYNTHSAMPLES: Self = Self(13);
    pub const MMERR_ITPACK_INVALID_DATA: Self = Self(14);
    pub const MMERR_DETECTING_DEVICE: Self = Self(15);
    pub const MMERR_INVALID_DEVICE: Self = Self(16);
    pub const MMERR_INITIALIZING_MIXER: Self = Self(17);
    pub const MMERR_OPENING_AUDIO: Self = Self(18);
    pub const MMERR_8BIT_ONLY: Self = Self(19);
    pub const MMERR_16BIT_ONLY: Self = Self(20);
    pub const MMERR_STEREO_ONLY: Self = Self(21);
    pub const MMERR_ULAW: Self = Self(22);
    pub const MMERR_NON_BLOCK: Self = Self(23);
    pub const MMERR_AF_AUDIO_PORT: Self = Self(24);
    pub const MMERR_AIX_CONFIG_INIT: Self = Self(25);
    pub const MMERR_AIX_CONFIG_CONTROL: Self = Self(26);
    pub const MMERR_AIX_CONFIG_START: Self = Self(27);
    pub const MMERR_GUS_SETTINGS: Self = Self(28);
    pub const MMERR_GUS_RESET: Self = Self(29);
    pub const MMERR_GUS_TIMER: Self = Self(30);
    pub const MMERR_HP_SETSAMPLESIZE: Self = Self(31);
    pub const MMERR_HP_SETSPEED: Self = Self(32);
    pub const MMERR_HP_CHANNELS: Self = Self(33);
    pub const MMERR_HP_AUDIO_OUTPUT: Self = Self(34);
    pub const MMERR_HP_AUDIO_DESC: Self = Self(35);
    pub const MMERR_HP_BUFFERSIZE: Self = Self(36);
    pub const MMERR_OSS_SETFRAGMENT: Self = Self(37);
    pub const MMERR_OSS_SETSAMPLESIZE: Self = Self(38);
    pub const MMERR_OSS_SETSTEREO: Self = Self(39);
    pub const MMERR_OSS_SETSPEED: Self = Self(40);
    pub const MMERR_SGI_SPEED: Self = Self(41);
    pub const MMERR_SGI_16BIT: Self = Self(42);
    pub const MMERR_SGI_8BIT: Self = Self(43);
    pub const MMERR_SGI_STEREO: Self = Self(44);
    pub const MMERR_SGI_MONO: Self = Self(45);
    pub const MMERR_SUN_INIT: Self = Self(46);
    pub const MMERR_OS2_MIXSETUP: Self = Self(47);
    pub const MMERR_OS2_SEMAPHORE: Self = Self(48);
    pub const MMERR_OS2_TIMER: Self = Self(49);
    pub const MMERR_OS2_THREAD: Self = Self(50);
    pub const MMERR_DS_PRIORITY: Self = Self(51);
    pub const MMERR_DS_BUFFER: Self = Self(52);
    pub const MMERR_DS_FORMAT: Self = Self(53);
    pub const MMERR_DS_NOTIFY: Self = Self(54);
    pub const MMERR_DS_EVENT: Self = Self(55);
    pub const MMERR_DS_THREAD: Self = Self(56);
    pub const MMERR_DS_UPDATE: Self = Self(57);
    pub const MMERR_WINMM_HANDLE: Self = Self(58);
    pub const MMERR_WINMM_ALLOCATED: Self = Self(59);
    pub const MMERR_WINMM_DEVICEID: Self = Self(60);
    pub const MMERR_WINMM_FORMAT: Self = Self(61);
    pub const MMERR_WINMM_UNKNOWN: Self = Self(62);
    pub const MMERR_MAC_SPEED: Self = Self(63);
    pub const MMERR_MAC_START: Self = Self(64);
    pub const MMERR_OSX_UNKNOWN_DEVICE: Self = Self(65);
    pub const MMERR_OSX_BAD_PROPERTY: Self = Self(66);
    pub const MMERR_OSX_UNSUPPORTED_FORMAT: Self = Self(67);
    pub const MMERR_OSX_SET_STEREO: Self = Self(68);
    pub const MMERR_OSX_BUFFER_ALLOC: Self = Self(69);
    pub const MMERR_OSX_ADD_IO_PROC: Self = Self(70);
    pub const MMERR_OSX_DEVICE_START: Self = Self(71);
    pub const MMERR_OSX_PTHREAD: Self = Self(72);
    pub const MMERR_DOSWSS_STARTDMA: Self = Self(73);
    pub const MMERR_DOSSB_STARTDMA: Self = Self(74);
    pub const MMERR_NO_FLOAT32: Self = Self(75);
    pub const MMERR_OPENAL_CREATECTX: Self = Self(76);
    pub const MMERR_OPENAL_CTXCURRENT: Self = Self(77);
    pub const MMERR_OPENAL_GENBUFFERS: Self = Self(78);
    pub const MMERR_OPENAL_GENSOURCES: Self = Self(79);
    pub const MMERR_OPENAL_SOURCE: Self = Self(80);
    pub const MMERR_OPENAL_QUEUEBUFFERS: Self = Self(81);
    pub const MMERR_OPENAL_UNQUEUEBUFFERS: Self = Self(82);
    pub const MMERR_OPENAL_BUFFERDATA: Self = Self(83);
    pub const MMERR_OPENAL_GETSOURCE: Self = Self(84);
    pub const MMERR_OPENAL_SOURCEPLAY: Self = Self(85);
    pub const MMERR_OPENAL_SOURCESTOP: Self = Self(86);
    pub const MMERR_ALSA_NOCONFIG: Self = Self(87);
    pub const MMERR_ALSA_SETPARAMS: Self = Self(88);
    pub const MMERR_ALSA_SETFORMAT: Self = Self(89);
    pub const MMERR_ALSA_SETRATE: Self = Self(90);
    pub const MMERR_ALSA_SETCHANNELS: Self = Self(91);
    pub const MMERR_ALSA_BUFFERSIZE: Self = Self(92);
    pub const MMERR_ALSA_PCM_START: Self = Self(93);
    pub const MMERR_ALSA_PCM_WRITE: Self = Self(94);
    pub const MMERR_ALSA_PCM_RECOVER: Self = Self(95);
    pub const MMERR_SNDIO_SETPARAMS: Self = Self(96);
    pub const MMERR_SNDIO_BADPARAMS: Self = Self(97);
    pub const MMERR_MAX: Self = Self(98);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAMPLE {
    pub panning: SWORD,
    pub speed: ULONG,
    pub volume: UBYTE,
    pub inflags: UWORD,
    pub flags: UWORD,
    pub length: ULONG,
    pub loopstart: ULONG,
    pub loopend: ULONG,
    pub susbegin: ULONG,
    pub susend: ULONG,
    pub globvol: UBYTE,
    pub vibflags: UBYTE,
    pub vibtype: UBYTE,
    pub vibsweep: UBYTE,
    pub vibdepth: UBYTE,
    pub vibrate: UBYTE,
    pub samplename: *mut CHAR,
    pub avibpos: UWORD,
    pub divfactor: UBYTE,
    pub seekpos: ULONG,
    pub handle: SWORD,
    pub onfree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAMPLOAD {
    pub next: *mut SAMPLOAD,
    pub length: ULONG,
    pub loopstart: ULONG,
    pub loopend: ULONG,
    pub infmt: UWORD,
    pub outfmt: UWORD,
    pub scalefactor: ::core::ffi::c_int,
    pub sample: *mut SAMPLE,
    pub reader: *mut MREADER,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MREADER {
    pub Seek: Option<
        unsafe extern "C" fn(
            *mut MREADER,
            ::core::ffi::c_long,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub Tell: Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long>,
    pub Read: Option<
        unsafe extern "C" fn(*mut MREADER, *mut ::core::ffi::c_void, size_t) -> BOOL,
    >,
    pub Get: Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int>,
    pub Eof: Option<unsafe extern "C" fn(*mut MREADER) -> BOOL>,
    pub iobase: ::core::ffi::c_long,
    pub prev_iobase: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WAV {
    pub rID: [CHAR; 4],
    pub rLen: ULONG,
    pub wID: [CHAR; 4],
    pub fID: [CHAR; 4],
    pub fLen: ULONG,
    pub wFormatTag: UWORD,
    pub nChannels: UWORD,
    pub nSamplesPerSec: ULONG,
    pub nAvgBytesPerSec: ULONG,
    pub nBlockAlign: UWORD,
    pub nFormatSpecific: UWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const MD_MUSIC: Self = Self(0);
    pub const MD_SNDFX: Self = Self(1);
}
unsafe extern "C" fn isWaveFile(mut reader: *mut MREADER) -> BOOL {
    let mut wh: WAV = WAV {
        rID: [0; 4],
        rLen: 0,
        wID: [0; 4],
        fID: [0; 4],
        fLen: 0,
        wFormatTag: 0,
        nChannels: 0,
        nSamplesPerSec: 0,
        nAvgBytesPerSec: 0,
        nBlockAlign: 0,
        nFormatSpecific: 0,
    };
    (*reader)
        .Seek
        .expect(
            "non-null function pointer",
        )(reader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    _mm_read_string(&raw mut wh.rID as *mut CHAR, 4 as ::core::ffi::c_int, reader);
    wh.rLen = _mm_read_I_ULONG(reader);
    _mm_read_string(&raw mut wh.wID as *mut CHAR, 4 as ::core::ffi::c_int, reader);
    if (*reader).Eof.expect("non-null function pointer")(reader) != 0
        || memcmp(
            &raw mut wh.rID as *mut CHAR as *const ::core::ffi::c_void,
            b"RIFF\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0
        || memcmp(
            &raw mut wh.wID as *mut CHAR as *const ::core::ffi::c_void,
            b"WAVE\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0
    {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn Sample_LoadGeneric_internal_wav(
    mut reader: *mut MREADER,
) -> *mut SAMPLE {
    let mut si: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut wh: WAV = WAV {
        rID: [0; 4],
        rLen: 0,
        wID: [0; 4],
        fID: [0; 4],
        fLen: 0,
        wFormatTag: 0,
        nChannels: 0,
        nSamplesPerSec: 0,
        nAvgBytesPerSec: 0,
        nBlockAlign: 0,
        nFormatSpecific: 0,
    };
    let mut have_fmt: BOOL = 0 as BOOL;
    (*reader)
        .Seek
        .expect(
            "non-null function pointer",
        )(reader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    _mm_read_string(&raw mut wh.rID as *mut CHAR, 4 as ::core::ffi::c_int, reader);
    wh.rLen = _mm_read_I_ULONG(reader);
    _mm_read_string(&raw mut wh.wID as *mut CHAR, 4 as ::core::ffi::c_int, reader);
    if (*reader).Eof.expect("non-null function pointer")(reader) != 0
        || memcmp(
            &raw mut wh.rID as *mut CHAR as *const ::core::ffi::c_void,
            b"RIFF\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0
        || memcmp(
            &raw mut wh.wID as *mut CHAR as *const ::core::ffi::c_void,
            b"WAVE\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0
    {
        MikMod_errno = C2Rust_Unnamed::MMERR_UNKNOWN_WAVE_TYPE.0 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<SAMPLE>();
    }
    loop {
        let mut dID: [CHAR; 4] = [0; 4];
        let mut len: ULONG = 0;
        let mut start: ULONG = 0;
        _mm_read_string(&raw mut dID as *mut CHAR, 4 as ::core::ffi::c_int, reader);
        len = _mm_read_I_ULONG(reader);
        if (*reader).Eof.expect("non-null function pointer")(reader) != 0 {
            MikMod_errno = C2Rust_Unnamed::MMERR_UNKNOWN_WAVE_TYPE.0
                as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<SAMPLE>();
        }
        start = (*reader).Tell.expect("non-null function pointer")(reader) as ULONG;
        if memcmp(
            &raw mut dID as *mut CHAR as *const ::core::ffi::c_void,
            b"fmt \0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            wh.wFormatTag = _mm_read_I_UWORD(reader);
            wh.nChannels = _mm_read_I_UWORD(reader);
            wh.nSamplesPerSec = _mm_read_I_ULONG(reader);
            wh.nAvgBytesPerSec = _mm_read_I_ULONG(reader);
            wh.nBlockAlign = _mm_read_I_UWORD(reader);
            wh.nFormatSpecific = _mm_read_I_UWORD(reader);
            if have_fmt != 0
                || wh.nChannels as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            {
                MikMod_errno = C2Rust_Unnamed::MMERR_UNKNOWN_WAVE_TYPE.0
                    as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<SAMPLE>();
            }
            have_fmt = 1 as ::core::ffi::c_int as BOOL;
        } else if memcmp(
            &raw mut dID as *mut CHAR as *const ::core::ffi::c_void,
            b"data\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if have_fmt == 0 {
                MikMod_errno = C2Rust_Unnamed::MMERR_UNKNOWN_WAVE_TYPE.0
                    as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<SAMPLE>();
            }
            si = MikMod_malloc(::core::mem::size_of::<SAMPLE>() as size_t)
                as *mut SAMPLE;
            if si.is_null() {
                return ::core::ptr::null_mut::<SAMPLE>();
            }
            (*si).speed = wh.nSamplesPerSec.wrapping_div(wh.nChannels as ULONG);
            (*si).volume = 64 as UBYTE;
            (*si).length = len;
            if wh.nBlockAlign as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                (*si).flags = (0x1 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                    as UWORD;
                (*si).length >>= 1 as ::core::ffi::c_int;
            }
            (*si).inflags = (*si).flags;
            SL_RegisterSample(
                si,
                C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int,
                reader,
            );
            SL_LoadSamples();
            break;
        }
        (*reader)
            .Seek
            .expect(
                "non-null function pointer",
            )(
            reader,
            start.wrapping_add(len) as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        if (*reader).Eof.expect("non-null function pointer")(reader) != 0 {
            break;
        }
    }
    return si;
}
unsafe extern "C" fn Sample_LoadRawGeneric_internal(
    mut reader: *mut MREADER,
    mut rate: ULONG,
    mut channel: ULONG,
    mut flags: ULONG,
) -> *mut SAMPLE {
    let mut si: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut len: ::core::ffi::c_long = 0;
    let mut samp_size: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    si = MikMod_malloc(::core::mem::size_of::<SAMPLE>() as size_t) as *mut SAMPLE;
    if si.is_null() {
        return ::core::ptr::null_mut::<SAMPLE>();
    }
    (*reader)
        .Seek
        .expect(
            "non-null function pointer",
        )(reader, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int);
    len = (*reader).Tell.expect("non-null function pointer")(reader);
    (*si).panning = 128 as SWORD;
    (*si).speed = rate.wrapping_div(1 as ULONG);
    (*si).volume = 64 as UBYTE;
    (*si).length = len as ULONG;
    (*si).loopstart = 0 as ULONG;
    (*si).loopend = len as ULONG;
    (*si).susbegin = 0 as ULONG;
    (*si).susend = 0 as ULONG;
    (*si).flags = flags as UWORD;
    (*si).inflags = (*si).flags;
    if (*si).flags as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        (*si).length >>= 1 as ::core::ffi::c_int;
        (*si).loopstart >>= 1 as ::core::ffi::c_int;
        (*si).loopend >>= 1 as ::core::ffi::c_int;
        samp_size = 2 as ::core::ffi::c_int;
    }
    if (*si).flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut channel_data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut num_samp: ::core::ffi::c_int = (*si)
            .length
            .wrapping_div(samp_size as ULONG)
            .wrapping_div(2 as ULONG) as ::core::ffi::c_int;
        let mut chn_reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
        data = MikMod_malloc((*si).length) as *mut ::core::ffi::c_char;
        if data.is_null() {
            MikMod_free(si as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<SAMPLE>();
        }
        channel_data = MikMod_malloc((*si).length.wrapping_div(2 as size_t))
            as *mut ::core::ffi::c_char;
        if channel_data.is_null() {
            MikMod_free(data as *mut ::core::ffi::c_void);
            MikMod_free(si as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<SAMPLE>();
        }
        (*reader)
            .Seek
            .expect(
                "non-null function pointer",
            )(reader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
        (*reader)
            .Read
            .expect(
                "non-null function pointer",
            )(reader, data as *mut ::core::ffi::c_void, (*si).length);
        extract_channel(
            data,
            channel_data,
            2 as ::core::ffi::c_int,
            num_samp,
            samp_size,
            channel as ::core::ffi::c_int,
        );
        chn_reader = _mm_new_mem_reader(
            channel_data as *const ::core::ffi::c_void,
            (num_samp * samp_size) as ::core::ffi::c_long,
        );
        if chn_reader.is_null() {
            MikMod_free(channel_data as *mut ::core::ffi::c_void);
            MikMod_free(data as *mut ::core::ffi::c_void);
            MikMod_free(si as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<SAMPLE>();
        }
        (*si).loopstart = 0 as ULONG;
        (*si).length = num_samp as ULONG;
        (*si).loopend = num_samp as ULONG;
        SL_RegisterSample(
            si,
            C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int,
            chn_reader,
        );
        SL_LoadSamples();
        _mm_delete_mem_reader(chn_reader);
        MikMod_free(channel_data as *mut ::core::ffi::c_void);
        MikMod_free(data as *mut ::core::ffi::c_void);
        return si;
    }
    (*reader)
        .Seek
        .expect(
            "non-null function pointer",
        )(reader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    SL_RegisterSample(si, C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int, reader);
    SL_LoadSamples();
    return si;
}
unsafe extern "C" fn Sample_LoadGeneric_internal(
    mut reader: *mut MREADER,
    mut options: *const ::core::ffi::c_char,
) -> *mut SAMPLE {
    if isWaveFile(reader) != 0 {
        return Sample_LoadGeneric_internal_wav(reader);
    }
    return ::core::ptr::null_mut::<SAMPLE>();
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadRawGeneric(
    mut reader: *mut MREADER,
    mut rate: ULONG,
    mut channel: ULONG,
    mut flags: ULONG,
) -> *mut SAMPLE {
    let mut result: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    result = Sample_LoadRawGeneric_internal(reader, rate, channel, flags);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadRawMem(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut rate: ULONG,
    mut channel: ULONG,
    mut flags: ULONG,
) -> *mut SAMPLE {
    let mut result: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if buf.is_null() || len <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<SAMPLE>();
    }
    reader = _mm_new_mem_reader(
        buf as *const ::core::ffi::c_void,
        len as ::core::ffi::c_long,
    );
    if !reader.is_null() {
        result = Sample_LoadRawGeneric(reader, rate, channel, flags);
        _mm_delete_mem_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadRawFP(
    mut fp: *mut FILE,
    mut rate: ULONG,
    mut channel: ULONG,
    mut flags: ULONG,
) -> *mut SAMPLE {
    let mut result: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if !fp.is_null()
        && {
            reader = _mm_new_file_reader(fp);
            !reader.is_null()
        }
    {
        result = Sample_LoadRawGeneric(reader, rate, channel, flags);
        _mm_delete_file_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadRaw(
    mut filename: *const CHAR,
    mut rate: ULONG,
    mut channel: ULONG,
    mut flags: ULONG,
) -> *mut SAMPLE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut si: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<SAMPLE>();
    }
    fp = _mm_fopen(filename, b"rb\0".as_ptr() as *const CHAR);
    if !fp.is_null() {
        si = Sample_LoadRawFP(fp, rate, channel, flags);
        _mm_fclose(fp);
    }
    return si;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadGeneric(mut reader: *mut MREADER) -> *mut SAMPLE {
    let mut result: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    result = Sample_LoadGeneric_internal(
        reader,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadMem(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut SAMPLE {
    let mut result: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if buf.is_null() || len <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<SAMPLE>();
    }
    reader = _mm_new_mem_reader(
        buf as *const ::core::ffi::c_void,
        len as ::core::ffi::c_long,
    );
    if !reader.is_null() {
        result = Sample_LoadGeneric(reader);
        _mm_delete_mem_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_LoadFP(mut fp: *mut FILE) -> *mut SAMPLE {
    let mut result: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if !fp.is_null()
        && {
            reader = _mm_new_file_reader(fp);
            !reader.is_null()
        }
    {
        result = Sample_LoadGeneric(reader);
        _mm_delete_file_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_Load(mut filename: *const CHAR) -> *mut SAMPLE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut si: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<SAMPLE>();
    }
    fp = _mm_fopen(filename, b"rb\0".as_ptr() as *const CHAR);
    if !fp.is_null() {
        si = Sample_LoadFP(fp);
        _mm_fclose(fp);
    }
    return si;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_Free(mut si: *mut SAMPLE) {
    if !si.is_null() {
        if (*si).onfree.is_some() {
            (*si).onfree.expect("non-null function pointer")((*si).ctx);
        }
        MD_SampleUnload((*si).handle);
        MikMod_free(si as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sample_Free_internal(mut si: *mut SAMPLE) {
    Sample_Free(si);
}
unsafe extern "C" fn extract_channel(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut num_chan: ::core::ffi::c_int,
    mut num_samples: ::core::ffi::c_int,
    mut samp_size: ::core::ffi::c_int,
    mut channel: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    src = src.offset((channel * samp_size) as isize);
    loop {
        let c2rust_fresh0 = num_samples;
        num_samples -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        i = 0 as ::core::ffi::c_int;
        while i < samp_size {
            *dst.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
        src = src.offset((samp_size * num_chan) as isize);
        dst = dst.offset(samp_size as isize);
    };
}
