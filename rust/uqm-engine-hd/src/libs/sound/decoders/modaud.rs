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

/* MikMod decoder (.mod adapter)
 */

// Rust translation of src/libs/sound/decoders/modaud.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type MP_VOICE;
    pub type MP_CONTROL;
    pub type SAMPLOAD;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn uio_fgetc(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fseek(
        stream: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn uio_feof(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn crc32b(str: *const ::core::ffi::c_char) -> uint32_t;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut MikMod_errno: ::core::ffi::c_int;
    fn MikMod_strerror(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn MikMod_RegisterDriver(_: *mut MDRIVER);
    fn MikMod_Init(_: *const CHAR) -> ::core::ffi::c_int;
    fn MikMod_Exit();
    fn MikMod_Update();
    fn MikMod_RegisterAllLoaders();
    fn Player_LoadGeneric(
        _: *mut MREADER,
        _: ::core::ffi::c_int,
        _: BOOL,
    ) -> *mut MODULE;
    fn Player_Free(_: *mut MODULE);
    fn Player_Start(_: *mut MODULE);
    fn Player_Active() -> BOOL;
    fn Player_SetPosition(_: UWORD);
    static mut md_reverb: UBYTE;
    static mut md_pansep: UBYTE;
    static mut md_mixfreq: UWORD;
    static mut md_mode: UWORD;
    fn VC_Init() -> ::core::ffi::c_int;
    fn VC_Exit();
    fn VC_SetNumVoices() -> ::core::ffi::c_int;
    fn VC_SampleSpace(_: ::core::ffi::c_int) -> ULONG;
    fn VC_SampleLength(_: ::core::ffi::c_int, _: *mut SAMPLE) -> ULONG;
    fn VC_PlayStart() -> ::core::ffi::c_int;
    fn VC_PlayStop();
    fn VC_SampleLoad(_: *mut SAMPLOAD, _: ::core::ffi::c_int) -> SWORD;
    fn VC_SampleUnload(_: SWORD);
    fn VC_WriteBytes(_: *mut SBYTE, _: ULONG) -> ULONG;
    fn VC_VoiceSetVolume(_: UBYTE, _: UWORD);
    fn VC_VoiceGetVolume(_: UBYTE) -> UWORD;
    fn VC_VoiceSetFrequency(_: UBYTE, _: ULONG);
    fn VC_VoiceGetFrequency(_: UBYTE) -> ULONG;
    fn VC_VoiceSetPanning(_: UBYTE, _: ULONG);
    fn VC_VoiceGetPanning(_: UBYTE) -> ULONG;
    fn VC_VoicePlay(
        _: UBYTE,
        _: SWORD,
        _: ULONG,
        _: ULONG,
        _: ULONG,
        _: ULONG,
        _: UWORD,
    );
    fn VC_VoiceStop(_: UBYTE);
    fn VC_VoiceStopped(_: UBYTE) -> BOOL;
    fn VC_VoiceGetPosition(_: UBYTE) -> SLONG;
    fn VC_VoiceRealVolume(_: UBYTE) -> ULONG;
}
pub type size_t = usize;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint16 = ::core::ffi::c_ushort;
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
pub type SWORD = ::core::ffi::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MODULE {
    pub songname: *mut CHAR,
    pub modtype: *mut CHAR,
    pub comment: *mut CHAR,
    pub flags: UWORD,
    pub numchn: UBYTE,
    pub numvoices: UBYTE,
    pub numpos: UWORD,
    pub numpat: UWORD,
    pub numins: UWORD,
    pub numsmp: UWORD,
    pub instruments: *mut INSTRUMENT,
    pub samples: *mut SAMPLE,
    pub realchn: UBYTE,
    pub totalchn: UBYTE,
    pub reppos: UWORD,
    pub initspeed: UBYTE,
    pub inittempo: UWORD,
    pub initvolume: UBYTE,
    pub panning: [UWORD; 64],
    pub chanvol: [UBYTE; 64],
    pub bpm: UWORD,
    pub sngspd: UWORD,
    pub volume: SWORD,
    pub extspd: BOOL,
    pub panflag: BOOL,
    pub wrap: BOOL,
    pub r#loop: BOOL,
    pub fadeout: BOOL,
    pub patpos: UWORD,
    pub sngpos: SWORD,
    pub sngtime: ULONG,
    pub relspd: SWORD,
    pub numtrk: UWORD,
    pub tracks: *mut *mut UBYTE,
    pub patterns: *mut UWORD,
    pub pattrows: *mut UWORD,
    pub positions: *mut UWORD,
    pub forbid: BOOL,
    pub numrow: UWORD,
    pub vbtick: UWORD,
    pub sngremainder: UWORD,
    pub control: *mut MP_CONTROL,
    pub voice: *mut MP_VOICE,
    pub globalslide: UBYTE,
    pub pat_repcrazy: UBYTE,
    pub patbrk: UWORD,
    pub patdly: UBYTE,
    pub patdly2: UBYTE,
    pub posjmp: SWORD,
    pub bpmlimit: UWORD,
}
pub type UWORD = ::core::ffi::c_ushort;
pub type UBYTE = ::core::ffi::c_uchar;
pub type BOOL = ::core::ffi::c_int;
pub type ULONG = ::core::ffi::c_ulong;
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
pub type CHAR = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INSTRUMENT {
    pub insname: *mut CHAR,
    pub flags: UBYTE,
    pub samplenumber: [UWORD; 120],
    pub samplenote: [UBYTE; 120],
    pub nnatype: UBYTE,
    pub dca: UBYTE,
    pub dct: UBYTE,
    pub globvol: UBYTE,
    pub volfade: UWORD,
    pub panning: SWORD,
    pub pitpansep: UBYTE,
    pub pitpancenter: UBYTE,
    pub rvolvar: UBYTE,
    pub rpanvar: UBYTE,
    pub volflg: UBYTE,
    pub volpts: UBYTE,
    pub volsusbeg: UBYTE,
    pub volsusend: UBYTE,
    pub volbeg: UBYTE,
    pub volend: UBYTE,
    pub volenv: [ENVPT; 32],
    pub panflg: UBYTE,
    pub panpts: UBYTE,
    pub pansusbeg: UBYTE,
    pub pansusend: UBYTE,
    pub panbeg: UBYTE,
    pub panend: UBYTE,
    pub panenv: [ENVPT; 32],
    pub pitflg: UBYTE,
    pub pitpts: UBYTE,
    pub pitsusbeg: UBYTE,
    pub pitsusend: UBYTE,
    pub pitbeg: UBYTE,
    pub pitend: UBYTE,
    pub pitenv: [ENVPT; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENVPT {
    pub pos: SWORD,
    pub val: SWORD,
}
pub type TFB_ModSoundDecoder = tfb_modsounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_modsounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub last_error: sint32,
    pub module: *mut MODULE,
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
pub struct MUIOREADER {
    pub core: MREADER,
    pub file: *mut uio_Stream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDRIVER {
    pub next: *mut MDRIVER,
    pub Name: *const CHAR,
    pub Version: *const CHAR,
    pub HardVoiceLimit: UBYTE,
    pub SoftVoiceLimit: UBYTE,
    pub Alias: *const CHAR,
    pub CmdLineHelp: *const CHAR,
    pub CommandLine: Option<unsafe extern "C" fn(*const CHAR) -> ()>,
    pub IsPresent: Option<unsafe extern "C" fn() -> BOOL>,
    pub SampleLoad: Option<
        unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD,
    >,
    pub SampleUnload: Option<unsafe extern "C" fn(SWORD) -> ()>,
    pub FreeSampleSpace: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ULONG>,
    pub RealSampleLength: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG,
    >,
    pub Init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub Exit: Option<unsafe extern "C" fn() -> ()>,
    pub Reset: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub SetNumVoices: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub PlayStart: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub PlayStop: Option<unsafe extern "C" fn() -> ()>,
    pub Update: Option<unsafe extern "C" fn() -> ()>,
    pub Pause: Option<unsafe extern "C" fn() -> ()>,
    pub VoiceSetVolume: Option<unsafe extern "C" fn(UBYTE, UWORD) -> ()>,
    pub VoiceGetVolume: Option<unsafe extern "C" fn(UBYTE) -> UWORD>,
    pub VoiceSetFrequency: Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()>,
    pub VoiceGetFrequency: Option<unsafe extern "C" fn(UBYTE) -> ULONG>,
    pub VoiceSetPanning: Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()>,
    pub VoiceGetPanning: Option<unsafe extern "C" fn(UBYTE) -> ULONG>,
    pub VoicePlay: Option<
        unsafe extern "C" fn(UBYTE, SWORD, ULONG, ULONG, ULONG, ULONG, UWORD) -> (),
    >,
    pub VoiceStop: Option<unsafe extern "C" fn(UBYTE) -> ()>,
    pub VoiceStopped: Option<unsafe extern "C" fn(UBYTE) -> BOOL>,
    pub VoiceGetPosition: Option<unsafe extern "C" fn(UBYTE) -> SLONG>,
    pub VoiceRealVolume: Option<unsafe extern "C" fn(UBYTE) -> ULONG>,
}
pub type SLONG = ::core::ffi::c_long;
pub type SBYTE = ::core::ffi::c_schar;
#[no_mangle]
pub static mut moda_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(moda_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        moda_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(moda_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(moda_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        moda_GetError
            as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(moda_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(moda_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        moda_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(moda_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        moda_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        moda_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(
        moda_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32,
    ),
};
static mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
static mut bufsize: ULONG = 0;
static mut written: ULONG = 0;
unsafe extern "C" fn moda_mmout_SetOutputBuffer(
    mut buf: *mut ::core::ffi::c_void,
    mut size: ULONG,
) -> *mut ULONG {
    buffer = buf;
    bufsize = size;
    written = 0 as ULONG;
    return &raw mut written;
}
unsafe extern "C" fn moda_mmout_IsThere() -> BOOL {
    return 1 as BOOL;
}
unsafe extern "C" fn moda_mmout_Init() -> BOOL {
    md_mode = (md_mode as ::core::ffi::c_int
        | (0x8 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)) as UWORD;
    return uqm_lua_checked!(VC_Init(), 0);
}
unsafe extern "C" fn moda_mmout_Exit() {
    uqm_lua_checked!(VC_Exit(), ());
}
unsafe extern "C" fn moda_mmout_Update() {
    written = 0 as ULONG;
    if buffer.is_null() || bufsize == 0 as ULONG {
        return;
    }
    written = uqm_lua_checked!(VC_WriteBytes(buffer as *mut SBYTE, bufsize), ());
}
unsafe extern "C" fn moda_mmout_Reset() -> BOOL {
    return 0 as BOOL;
}
static mut MDRIVER_name: [::core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"Mem Buffer\0")
};
static mut MDRIVER_version: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<
        [u8; 23],
        [::core::ffi::c_char; 23],
    >(*b"Mem Buffer driver v1.1\0")
};
static mut MDRIVER_alias: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"membuf\0")
};
static mut moda_mmout_drv: MDRIVER = unsafe {
    MDRIVER {
        next: ::core::ptr::null_mut::<MDRIVER>(),
        Name: &raw const MDRIVER_name as *mut ::core::ffi::c_char,
        Version: &raw const MDRIVER_version as *mut ::core::ffi::c_char,
        HardVoiceLimit: 0 as UBYTE,
        SoftVoiceLimit: 255 as UBYTE,
        Alias: &raw const MDRIVER_alias as *mut ::core::ffi::c_char,
        CmdLineHelp: ::core::ptr::null::<CHAR>(),
        CommandLine: None,
        IsPresent: Some(moda_mmout_IsThere as unsafe extern "C" fn() -> BOOL),
        SampleLoad: Some(
            VC_SampleLoad
                as unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD,
        ),
        SampleUnload: Some(VC_SampleUnload as unsafe extern "C" fn(SWORD) -> ()),
        FreeSampleSpace: Some(
            VC_SampleSpace as unsafe extern "C" fn(::core::ffi::c_int) -> ULONG,
        ),
        RealSampleLength: Some(
            VC_SampleLength
                as unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG,
        ),
        Init: Some(moda_mmout_Init as unsafe extern "C" fn() -> BOOL),
        Exit: Some(moda_mmout_Exit as unsafe extern "C" fn() -> ()),
        Reset: Some(moda_mmout_Reset as unsafe extern "C" fn() -> BOOL),
        SetNumVoices: Some(
            VC_SetNumVoices as unsafe extern "C" fn() -> ::core::ffi::c_int,
        ),
        PlayStart: Some(VC_PlayStart as unsafe extern "C" fn() -> ::core::ffi::c_int),
        PlayStop: Some(VC_PlayStop as unsafe extern "C" fn() -> ()),
        Update: Some(moda_mmout_Update as unsafe extern "C" fn() -> ()),
        Pause: None,
        VoiceSetVolume: Some(
            VC_VoiceSetVolume as unsafe extern "C" fn(UBYTE, UWORD) -> (),
        ),
        VoiceGetVolume: Some(VC_VoiceGetVolume as unsafe extern "C" fn(UBYTE) -> UWORD),
        VoiceSetFrequency: Some(
            VC_VoiceSetFrequency as unsafe extern "C" fn(UBYTE, ULONG) -> (),
        ),
        VoiceGetFrequency: Some(
            VC_VoiceGetFrequency as unsafe extern "C" fn(UBYTE) -> ULONG,
        ),
        VoiceSetPanning: Some(
            VC_VoiceSetPanning as unsafe extern "C" fn(UBYTE, ULONG) -> (),
        ),
        VoiceGetPanning: Some(
            VC_VoiceGetPanning as unsafe extern "C" fn(UBYTE) -> ULONG,
        ),
        VoicePlay: Some(
            VC_VoicePlay
                as unsafe extern "C" fn(
                    UBYTE,
                    SWORD,
                    ULONG,
                    ULONG,
                    ULONG,
                    ULONG,
                    UWORD,
                ) -> (),
        ),
        VoiceStop: Some(VC_VoiceStop as unsafe extern "C" fn(UBYTE) -> ()),
        VoiceStopped: Some(VC_VoiceStopped as unsafe extern "C" fn(UBYTE) -> BOOL),
        VoiceGetPosition: Some(
            VC_VoiceGetPosition as unsafe extern "C" fn(UBYTE) -> SLONG,
        ),
        VoiceRealVolume: Some(VC_VoiceRealVolume as unsafe extern "C" fn(UBYTE) -> ULONG),
    }
};
static mut moda_formats: *const TFB_DecoderFormats = ::core::ptr::null::<
    TFB_DecoderFormats,
>();
unsafe extern "C" fn moda_uioReader_Eof(mut reader: *mut MREADER) -> BOOL {
    return uio_feof((*(reader as *mut MUIOREADER)).file);
}
unsafe extern "C" fn moda_uioReader_Read(
    mut reader: *mut MREADER,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> BOOL {
    return uqm_lua_checked!(uio_fread(ptr, size, 1 as size_t, (*(reader as *mut MUIOREADER)).file), 0)
        as BOOL;
}
unsafe extern "C" fn moda_uioReader_Get(mut reader: *mut MREADER) -> ::core::ffi::c_int {
    return uqm_lua_checked!(uio_fgetc((*(reader as *mut MUIOREADER)).file), 0);
}
unsafe extern "C" fn moda_uioReader_Seek(
    mut reader: *mut MREADER,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> BOOL {
    return uqm_lua_checked!(uio_fseek((*(reader as *mut MUIOREADER)).file, offset, whence), 0);
}
unsafe extern "C" fn moda_uioReader_Tell(
    mut reader: *mut MREADER,
) -> ::core::ffi::c_long {
    return uqm_lua_checked!(uio_ftell((*(reader as *mut MUIOREADER)).file), 0);
}
unsafe extern "C" fn moda_new_uioReader(mut fp: *mut uio_Stream) -> *mut MREADER {
    let mut reader: *mut MUIOREADER = HMalloc(::core::mem::size_of::<MUIOREADER>())
        as *mut MUIOREADER;
    if !reader.is_null() {
        (*reader).core.Eof = Some(
            moda_uioReader_Eof as unsafe extern "C" fn(*mut MREADER) -> BOOL,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> BOOL>;
        (*reader).core.Read = Some(
            moda_uioReader_Read
                as unsafe extern "C" fn(
                    *mut MREADER,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MREADER,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
            >;
        (*reader).core.Get = Some(
            moda_uioReader_Get
                as unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int>;
        (*reader).core.Seek = Some(
            moda_uioReader_Seek
                as unsafe extern "C" fn(
                    *mut MREADER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> BOOL,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MREADER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*reader).core.Tell = Some(
            moda_uioReader_Tell
                as unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long>;
        (*reader).file = fp;
    }
    return reader as *mut MREADER;
}
unsafe extern "C" fn moda_delete_uioReader(mut reader: *mut MREADER) {
    if !reader.is_null() {
        HFree(reader as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn moda_GetName() -> *const ::core::ffi::c_char {
    return b"MikMod\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn moda_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    MikMod_RegisterDriver(&raw mut moda_mmout_drv);
    MikMod_RegisterAllLoaders();
    if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        md_mode = (0x10 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int
            | 0x1 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int
            | 0x100 as ::core::ffi::c_int) as UWORD;
        md_mixfreq = 44100 as UWORD;
        md_reverb = 1 as UBYTE;
    } else if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        md_mode = (0x8 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int
            | 0x1 as ::core::ffi::c_int) as UWORD;
        md_mixfreq = 22050 as UWORD;
        md_reverb = 0 as UBYTE;
    } else {
        md_mode = (0x8 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int
            | 0x1 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int) as UWORD;
        md_mixfreq = 44100 as UWORD;
        md_reverb = 0 as UBYTE;
    }
    md_pansep = 64 as UBYTE;
    if uqm_lua_checked!(MikMod_Init(::core::ptr::null::<CHAR>()), false) != 0 {
        log_add(
            log_Level::log_Error,
            b"MikMod_Init() failed, %s\0".as_ptr() as *const ::core::ffi::c_char,
            MikMod_strerror(MikMod_errno),
        );
        return false;
    }
    moda_formats = fmts;
    return true;
}
unsafe extern "C" fn moda_TermModule() {
    uqm_lua_checked!(MikMod_Exit(), ());
}
unsafe extern "C" fn moda_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_ModSoundDecoder>() as uint32;
}
unsafe extern "C" fn moda_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    let mut moda: *mut TFB_ModSoundDecoder = This as *mut TFB_ModSoundDecoder;
    let mut ret: ::core::ffi::c_int = (*moda).last_error;
    (*moda).last_error = 0 as ::core::ffi::c_int as sint32;
    return ret;
}
unsafe extern "C" fn moda_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    (*This).need_swap = (*moda_formats).big_endian as ::core::ffi::c_int
        != (*moda_formats).want_big_endian as ::core::ffi::c_int;
    return true;
}
unsafe extern "C" fn moda_Term(mut This: *mut TFB_SoundDecoder) {
    uqm_lua_checked!(moda_Close(This), ());
}
unsafe extern "C" fn moda_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    let mut moda: *mut TFB_ModSoundDecoder = This as *mut TFB_ModSoundDecoder;
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    let mut r#mod: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    fp = uqm_lua_checked!(uio_fopen(dir, filename, b"rb\0".as_ptr() as *const ::core::ffi::c_char), false);
    if fp.is_null() {
        (*moda).last_error = *__errno_location() as sint32;
        return false;
    }
    reader = moda_new_uioReader(fp);
    if reader.is_null() {
        (*moda).last_error = -1 as ::core::ffi::c_int as sint32;
        uqm_lua_checked!(uio_fclose(fp), false);
        return false;
    }
    r#mod = uqm_lua_checked!(Player_LoadGeneric(reader, 8 as ::core::ffi::c_int, 0 as BOOL), false);
    moda_delete_uioReader(reader);
    uqm_lua_checked!(uio_fclose(fp), false);
    if r#mod.is_null() {
        log_add(
            log_Level::log_Warning,
            b"moda_Open(): could not load %s\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        return false;
    }
    (*moda).module = r#mod;
    (*r#mod).extspd = 1 as ::core::ffi::c_int as BOOL;
    (*r#mod).panflag = 1 as ::core::ffi::c_int as BOOL;
    (*r#mod).wrap = 0 as ::core::ffi::c_int as BOOL;
    (*r#mod).r#loop = 1 as ::core::ffi::c_int as BOOL;
    (*This).format = (*moda_formats).stereo16;
    (*This).frequency = md_mixfreq as uint32;
    (*This).length = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*This).numpos = (*r#mod).numpos as uint16;
    (*This).filename_hash = crc32b(filename) as uint32;
    (*moda).last_error = 0 as ::core::ffi::c_int as sint32;
    return true;
}
unsafe extern "C" fn moda_Close(mut This: *mut TFB_SoundDecoder) {
    let mut moda: *mut TFB_ModSoundDecoder = This as *mut TFB_ModSoundDecoder;
    if !(*moda).module.is_null() {
        uqm_lua_checked!(Player_Free((*moda).module), ());
        (*moda).module = ::core::ptr::null_mut::<MODULE>();
    }
}
unsafe extern "C" fn moda_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize_0: sint32,
) -> ::core::ffi::c_int {
    let mut moda: *mut TFB_ModSoundDecoder = This as *mut TFB_ModSoundDecoder;
    let mut poutsize: *mut ULONG = ::core::ptr::null_mut::<ULONG>();
    uqm_lua_checked!(Player_Start((*moda).module), 0);
    if Player_Active() == 0 {
        return 0 as ::core::ffi::c_int;
    }
    poutsize = moda_mmout_SetOutputBuffer(buf, bufsize_0 as ULONG) as *mut ULONG;
    uqm_lua_checked!(MikMod_Update(), 0);
    return ::core::ptr::read_volatile::<ULONG>(poutsize) as ::core::ffi::c_int;
}
unsafe extern "C" fn moda_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut moda: *mut TFB_ModSoundDecoder = This as *mut TFB_ModSoundDecoder;
    uqm_lua_checked!(Player_Start((*moda).module), 0);
    uqm_lua_checked!(Player_SetPosition(pcm_pos as UWORD), 0);
    return 0 as uint32;
}
unsafe extern "C" fn moda_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    let mut moda: *mut TFB_ModSoundDecoder = This as *mut TFB_ModSoundDecoder;
    return (*(*moda).module).sngpos as uint32;
}
