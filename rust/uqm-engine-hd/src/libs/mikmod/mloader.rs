/*	MikMod sound library
	(c) 1998, 1999, 2000, 2001, 2002 Miodrag Vallat and others - see file
	AUTHORS for complete list.

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

  These routines are used to access the available module loaders

==============================================================================*/

// Rust translation of src/libs/mikmod/mloader.c.
extern "C" {
    pub type _IO_FILE;
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
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut MikMod_errno: ::core::ffi::c_int;
    static mut MikMod_critical: BOOL;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    static mut _mm_errorhandler: MikMod_handler_t;
    fn _mm_new_mem_reader(
        buffer: *const ::core::ffi::c_void,
        len: ::core::ffi::c_long,
    ) -> *mut MREADER;
    fn _mm_delete_mem_reader(reader: *mut MREADER);
    fn _mm_new_file_reader(fp: *mut FILE) -> *mut MREADER;
    fn _mm_delete_file_reader(_: *mut MREADER);
    fn _mm_iobase_setcur(_: *mut MREADER);
    fn _mm_iobase_revert(_: *mut MREADER);
    fn _mm_fopen(_: *const CHAR, _: *const CHAR) -> *mut FILE;
    fn _mm_fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn SL_LoadSamples() -> ::core::ffi::c_int;
    fn SL_RegisterSample(
        _: *mut SAMPLE,
        _: ::core::ffi::c_int,
        _: *mut MREADER,
    ) -> *mut SAMPLOAD;
    fn UniInit() -> BOOL;
    fn UniCleanup();
    fn Player_Init(_: *mut MODULE) -> ::core::ffi::c_int;
    fn MD_SampleUnload(_: SWORD);
    fn MikMod_SetNumVoices_internal(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Player_Exit_internal(_: *mut MODULE);
}
pub type size_t = ::core::ffi::c_ulong;
pub type FILE = _IO_FILE;
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
pub type SBYTE = ::core::ffi::c_schar;
pub type UBYTE = ::core::ffi::c_uchar;
pub type SWORD = ::core::ffi::c_short;
pub type UWORD = ::core::ffi::c_ushort;
pub type SLONG = ::core::ffi::c_long;
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
pub type MikMod_handler = unsafe extern "C" fn() -> ();
pub type MikMod_handler_t = Option<MikMod_handler>;
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
pub struct ENVPT {
    pub pos: SWORD,
    pub val: SWORD,
}
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
pub struct MP_CONTROL {
    pub main: MP_CHANNEL,
    pub slave: *mut MP_VOICE,
    pub slavechn: UBYTE,
    pub muted: UBYTE,
    pub ultoffset: UWORD,
    pub anote: UBYTE,
    pub oldnote: UBYTE,
    pub ownper: SWORD,
    pub ownvol: SWORD,
    pub dca: UBYTE,
    pub dct: UBYTE,
    pub row: *mut UBYTE,
    pub retrig: SBYTE,
    pub speed: ULONG,
    pub volume: SWORD,
    pub tmpvolume: SWORD,
    pub tmpperiod: UWORD,
    pub wantedperiod: UWORD,
    pub arpmem: UBYTE,
    pub pansspd: UBYTE,
    pub slidespeed: UWORD,
    pub portspeed: UWORD,
    pub s3mtremor: UBYTE,
    pub s3mtronof: UBYTE,
    pub s3mvolslide: UBYTE,
    pub sliding: SBYTE,
    pub s3mrtgspeed: UBYTE,
    pub s3mrtgslide: UBYTE,
    pub glissando: UBYTE,
    pub wavecontrol: UBYTE,
    pub vibpos: SBYTE,
    pub vibspd: UBYTE,
    pub vibdepth: UBYTE,
    pub trmpos: SBYTE,
    pub trmspd: UBYTE,
    pub trmdepth: UBYTE,
    pub fslideupspd: UBYTE,
    pub fslidednspd: UBYTE,
    pub fportupspd: UBYTE,
    pub fportdnspd: UBYTE,
    pub ffportupspd: UBYTE,
    pub ffportdnspd: UBYTE,
    pub hioffset: ULONG,
    pub soffset: UWORD,
    pub sseffect: UBYTE,
    pub ssdata: UBYTE,
    pub chanvolslide: UBYTE,
    pub panbwave: UBYTE,
    pub panbpos: UBYTE,
    pub panbspd: SBYTE,
    pub panbdepth: UBYTE,
    pub newsamp: UWORD,
    pub voleffect: UBYTE,
    pub voldata: UBYTE,
    pub pat_reppos: SWORD,
    pub pat_repcnt: UWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP_VOICE {
    pub main: MP_CHANNEL,
    pub venv: ENVPR,
    pub penv: ENVPR,
    pub cenv: ENVPR,
    pub avibpos: UWORD,
    pub aswppos: UWORD,
    pub totalvol: ULONG,
    pub mflag: BOOL,
    pub masterchn: SWORD,
    pub masterperiod: UWORD,
    pub master: *mut MP_CONTROL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENVPR {
    pub flg: UBYTE,
    pub pts: UBYTE,
    pub susbeg: UBYTE,
    pub susend: UBYTE,
    pub beg: UBYTE,
    pub end: UBYTE,
    pub p: SWORD,
    pub a: UWORD,
    pub b: UWORD,
    pub env: *mut ENVPT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP_CHANNEL {
    pub i: *mut INSTRUMENT,
    pub s: *mut SAMPLE,
    pub sample: UBYTE,
    pub note: UBYTE,
    pub outvolume: SWORD,
    pub chanvol: SBYTE,
    pub fadevol: UWORD,
    pub panning: SWORD,
    pub kick: UBYTE,
    pub kick_flag: UBYTE,
    pub period: UWORD,
    pub nna: UBYTE,
    pub volflg: UBYTE,
    pub panflg: UBYTE,
    pub pitflg: UBYTE,
    pub keyoff: UBYTE,
    pub handle: SWORD,
    pub notedelay: UBYTE,
    pub start: SLONG,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MLOADER {
    pub next: *mut MLOADER,
    pub r#type: *const CHAR,
    pub version: *const CHAR,
    pub Init: Option<unsafe extern "C" fn() -> BOOL>,
    pub Test: Option<unsafe extern "C" fn() -> BOOL>,
    pub Load: Option<unsafe extern "C" fn(BOOL) -> BOOL>,
    pub Cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub LoadTitle: Option<unsafe extern "C" fn() -> *mut CHAR>,
}
pub type MUNPACKER = Option<
    unsafe extern "C" fn(
        *mut MREADER,
        *mut *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_long,
    ) -> BOOL,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const MD_MUSIC: Self = Self(0);
    pub const MD_SNDFX: Self = Self(1);
}
#[no_mangle]
pub static mut modreader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
#[no_mangle]
pub static mut of: MODULE = MODULE {
    songname: ::core::ptr::null_mut::<CHAR>(),
    modtype: ::core::ptr::null_mut::<CHAR>(),
    comment: ::core::ptr::null_mut::<CHAR>(),
    flags: 0,
    numchn: 0,
    numvoices: 0,
    numpos: 0,
    numpat: 0,
    numins: 0,
    numsmp: 0,
    instruments: ::core::ptr::null_mut::<INSTRUMENT>(),
    samples: ::core::ptr::null_mut::<SAMPLE>(),
    realchn: 0,
    totalchn: 0,
    reppos: 0,
    initspeed: 0,
    inittempo: 0,
    initvolume: 0,
    panning: [0; 64],
    chanvol: [0; 64],
    bpm: 0,
    sngspd: 0,
    volume: 0,
    extspd: 0,
    panflag: 0,
    wrap: 0,
    r#loop: 0,
    fadeout: 0,
    patpos: 0,
    sngpos: 0,
    sngtime: 0,
    relspd: 0,
    numtrk: 0,
    tracks: ::core::ptr::null_mut::<*mut UBYTE>(),
    patterns: ::core::ptr::null_mut::<UWORD>(),
    pattrows: ::core::ptr::null_mut::<UWORD>(),
    positions: ::core::ptr::null_mut::<UWORD>(),
    forbid: 0,
    numrow: 0,
    vbtick: 0,
    sngremainder: 0,
    control: ::core::ptr::null_mut::<MP_CONTROL>(),
    voice: ::core::ptr::null_mut::<MP_VOICE>(),
    globalslide: 0,
    pat_repcrazy: 0,
    patbrk: 0,
    patdly: 0,
    patdly2: 0,
    posjmp: 0,
    bpmlimit: 0,
};
static mut firstloader: *mut MLOADER = ::core::ptr::null_mut::<MLOADER>();
static mut unpackers: [MUNPACKER; 1] = [None];
#[no_mangle]
pub static mut finetune: [UWORD; 16] = [
    8363 as UWORD,
    8413 as UWORD,
    8463 as UWORD,
    8529 as UWORD,
    8581 as UWORD,
    8651 as UWORD,
    8723 as UWORD,
    8757 as UWORD,
    7895 as UWORD,
    7941 as UWORD,
    7985 as UWORD,
    8046 as UWORD,
    8107 as UWORD,
    8169 as UWORD,
    8232 as UWORD,
    8280 as UWORD,
];
#[no_mangle]
pub unsafe extern "C" fn MikMod_InfoLoader() -> *mut CHAR {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: *mut MLOADER = ::core::ptr::null_mut::<MLOADER>();
    let mut list: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    l = firstloader;
    while !l.is_null() {
        len = (len as size_t)
            .wrapping_add(
                ((1 as ::core::ffi::c_int
                    + if !(*l).next.is_null() {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as size_t)
                    .wrapping_add(strlen((*l).version)),
            ) as ::core::ffi::c_int;
        l = (*l).next as *mut MLOADER;
    }
    if len != 0 {
        list = uqm_lua_checked!(MikMod_malloc(
            (len as size_t).wrapping_mul(::core::mem::size_of::<CHAR>() as size_t),
        ), ::core::ptr::null_mut()) as *mut CHAR;
        if !list.is_null() {
            let mut list_end: *mut CHAR = list;
            *list.offset(0isize) = 0 as CHAR;
            l = firstloader;
            while !l.is_null() {
                list_end = list_end
                    .offset(
                        sprintf(
                            list_end,
                            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                            (*l).version,
                            if !(*l).next.is_null() {
                                b"\n\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        ) as isize,
                    );
                l = (*l).next as *mut MLOADER;
            }
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_registerloader(mut ldr: *mut MLOADER) {
    let mut cruise: *mut MLOADER = firstloader;
    if !cruise.is_null() {
        while !(*cruise).next.is_null() {
            cruise = (*cruise).next as *mut MLOADER;
        }
        (*cruise).next = ldr as *mut MLOADER;
    } else {
        firstloader = ldr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_RegisterLoader(mut ldr: *mut MLOADER) {
    if ldr.is_null() || !(*ldr).next.is_null() {
        return;
    }
    _mm_registerloader(ldr);
}
#[no_mangle]
pub unsafe extern "C" fn ReadComment(mut len: UWORD) -> BOOL {
    if len != 0 {
        let mut i: ::core::ffi::c_int = 0;
        of.comment = uqm_lua_checked!(MikMod_malloc(
            (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        ), 0) as *mut CHAR;
        if of.comment.is_null() {
            return 0 as BOOL;
        }
        uqm_lua_checked!((*modreader)
            .Read
            .expect(
                "non-null function pointer",
            )(modreader, of.comment as *mut ::core::ffi::c_void, len as size_t), 0);
        i = 0 as ::core::ffi::c_int;
        while i < len as ::core::ffi::c_int {
            if *of.comment.offset(i as isize) as ::core::ffi::c_int
                == '\r' as ::core::ffi::c_int
            {
                *of.comment.offset(i as isize) = '\n' as CHAR;
            }
            i += 1;
        }
        *of.comment.offset(len as isize) = 0 as CHAR;
    }
    if *of.comment.offset(0isize) == 0 {
        MikMod_free(of.comment as *mut ::core::ffi::c_void);
        of.comment = ::core::ptr::null_mut::<CHAR>();
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn ReadLinedComment(mut len: UWORD, mut linelen: UWORD) -> BOOL {
    let mut buf: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut storage: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut p: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut numlines: size_t = 0;
    let mut line: size_t = 0;
    let mut fpos: size_t = 0;
    let mut cpos: size_t = 0;
    let mut lpos: size_t = 0;
    let mut cnt: size_t = 0;
    if linelen == 0 {
        return 0 as BOOL;
    }
    if len == 0 {
        return 1 as BOOL;
    }
    buf = uqm_lua_checked!(MikMod_malloc(len as size_t), 0) as *mut CHAR;
    if buf.is_null() {
        return 0 as BOOL;
    }
    numlines = ((len as ::core::ffi::c_int + linelen as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) / linelen as ::core::ffi::c_int) as size_t;
    cnt = ((linelen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
        .wrapping_mul(numlines);
    storage = uqm_lua_checked!(MikMod_malloc(cnt.wrapping_add(1 as size_t)), 0) as *mut CHAR;
    if storage.is_null() {
        MikMod_free(buf as *mut ::core::ffi::c_void);
        return 0 as BOOL;
    }
    uqm_lua_checked!((*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(modreader, buf as *mut ::core::ffi::c_void, len as size_t), 0);
    *storage.offset(cnt as isize) = 0 as CHAR;
    line = 0 as size_t;
    fpos = 0 as size_t;
    cpos = 0 as size_t;
    while line < numlines {
        cnt = (len as size_t).wrapping_sub(fpos);
        if cnt > linelen as size_t {
            cnt = linelen as size_t;
        }
        p = storage.offset(cpos as isize);
        memcpy(
            p as *mut ::core::ffi::c_void,
            buf.offset(fpos as isize) as *const ::core::ffi::c_void,
            cnt,
        );
        *p.offset(cnt as isize) = '\r' as CHAR;
        lpos = 0 as size_t;
        while lpos < linelen as size_t {
            match *p as ::core::ffi::c_int {
                0 | 10 | 13 => {
                    *p = ' ' as CHAR;
                }
                _ => {}
            }
            lpos = lpos.wrapping_add(1);
            p = p.offset(1);
        }
        line = line.wrapping_add(1);
        fpos = fpos.wrapping_add(linelen as size_t);
        cpos = cpos
            .wrapping_add(
                (linelen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
            );
    }
    of.comment = storage;
    MikMod_free(buf as *mut ::core::ffi::c_void);
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn AllocPositions(mut total: ::core::ffi::c_int) -> BOOL {
    if total == 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    of.positions = uqm_lua_checked!(MikMod_calloc(
        total as size_t,
        ::core::mem::size_of::<UWORD>() as size_t,
    ), 0) as *mut UWORD;
    if of.positions.is_null() {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn AllocPatterns() -> BOOL {
    let mut s: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut tracks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if of.numpat == 0 || of.numchn == 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    of.patterns = uqm_lua_checked!(MikMod_calloc(
        ((of.numpat as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(of.numchn as size_t),
        ::core::mem::size_of::<UWORD>() as size_t,
    ), 0) as *mut UWORD;
    if of.patterns.is_null() {
        return 0 as BOOL;
    }
    of.pattrows = uqm_lua_checked!(MikMod_calloc(
        (of.numpat as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<UWORD>() as size_t,
    ), 0) as *mut UWORD;
    if of.pattrows.is_null() {
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t <= of.numpat as ::core::ffi::c_int {
        *of.pattrows.offset(t as isize) = 64 as UWORD;
        s = 0 as ::core::ffi::c_int;
        while s < of.numchn as ::core::ffi::c_int {
            let c2rust_fresh0 = tracks;
            tracks += 1;
            *of.patterns.offset((t * of.numchn as ::core::ffi::c_int + s) as isize) = c2rust_fresh0
                as UWORD;
            s += 1;
        }
        t += 1;
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn AllocTracks() -> BOOL {
    if of.numtrk == 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    of.tracks = uqm_lua_checked!(MikMod_calloc(
        of.numtrk as size_t,
        ::core::mem::size_of::<*mut UBYTE>() as size_t,
    ), 0) as *mut *mut UBYTE;
    if of.tracks.is_null() {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn AllocInstruments() -> BOOL {
    let mut t: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if of.numins == 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    of.instruments = uqm_lua_checked!(MikMod_calloc(
        of.numins as size_t,
        ::core::mem::size_of::<INSTRUMENT>() as size_t,
    ), 0) as *mut INSTRUMENT as *mut INSTRUMENT;
    if of.instruments.is_null() {
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < of.numins as ::core::ffi::c_int {
        n = 0 as ::core::ffi::c_int;
        while n < 120 as ::core::ffi::c_int {
            (*of.instruments.offset(t as isize)).samplenote[n as usize] = n as UBYTE;
            (*of.instruments.offset(t as isize)).samplenumber[n as usize] = t as UWORD;
            n += 1;
        }
        (*of.instruments.offset(t as isize)).globvol = 64 as UBYTE;
        t += 1;
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn AllocSamples() -> BOOL {
    let mut u: UWORD = 0;
    if of.numsmp == 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    of.samples = uqm_lua_checked!(MikMod_calloc(
        of.numsmp as size_t,
        ::core::mem::size_of::<SAMPLE>() as size_t,
    ), 0) as *mut SAMPLE as *mut SAMPLE;
    if of.samples.is_null() {
        return 0 as BOOL;
    }
    u = 0 as UWORD;
    while (u as ::core::ffi::c_int) < of.numsmp as ::core::ffi::c_int {
        (*of.samples.offset(u as isize)).panning = 128 as SWORD;
        (*of.samples.offset(u as isize)).handle = -1 as SWORD;
        (*of.samples.offset(u as isize)).globvol = 64 as UBYTE;
        (*of.samples.offset(u as isize)).volume = 64 as UBYTE;
        u = u.wrapping_add(1);
    }
    return 1 as BOOL;
}
unsafe extern "C" fn ML_LoadSamples() -> BOOL {
    let mut s: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut u: ::core::ffi::c_int = 0;
    u = of.numsmp as ::core::ffi::c_int;
    s = of.samples as *mut SAMPLE;
    while u != 0 {
        if (*s).length != 0 {
            uqm_lua_checked!(SL_RegisterSample(
                s,
                C2Rust_Unnamed_0::MD_MUSIC.0 as ::core::ffi::c_int,
                modreader,
            ), 0);
        }
        u -= 1;
        s = s.offset(1);
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn DupStr(
    mut s: *const CHAR,
    mut len: UWORD,
    mut strict: BOOL,
) -> *mut CHAR {
    let mut t: UWORD = 0;
    let mut d: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    while len != 0 {
        if *s.offset((len as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int > 0x20 as ::core::ffi::c_int
        {
            break;
        }
        len = len.wrapping_sub(1);
    }
    if strict != 0 {
        t = 0 as UWORD;
        while (t as ::core::ffi::c_int) < len as ::core::ffi::c_int {
            if *s.offset(t as isize) == 0 {
                break;
            }
            t = t.wrapping_add(1);
        }
        if (t as ::core::ffi::c_int) < len as ::core::ffi::c_int {
            len = t;
        }
    }
    d = uqm_lua_checked!(MikMod_malloc(
        (::core::mem::size_of::<CHAR>() as size_t)
            .wrapping_mul(
                (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
            ),
    ), ::core::ptr::null_mut()) as *mut CHAR;
    if !d.is_null() {
        t = 0 as UWORD;
        while (t as ::core::ffi::c_int) < len as ::core::ffi::c_int {
            *d.offset(t as isize) = (if (*s.offset(t as isize) as ::core::ffi::c_int)
                < 32 as ::core::ffi::c_int
            {
                '.' as ::core::ffi::c_int
            } else {
                *s.offset(t as isize) as ::core::ffi::c_int
            }) as CHAR;
            t = t.wrapping_add(1);
        }
        *d.offset(len as isize) = 0 as CHAR;
    }
    return d;
}
unsafe extern "C" fn ML_XFreeSample(mut s: *mut SAMPLE) {
    if (*s).handle as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        uqm_lua_checked!(MD_SampleUnload((*s).handle), ());
    }
}
unsafe extern "C" fn ML_XFreeInstrument(mut i: *mut INSTRUMENT) {
    MikMod_free((*i).insname as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn ML_FreeEx(mut mf: *mut MODULE) {
    let mut t: UWORD = 0;
    MikMod_free((*mf).songname as *mut ::core::ffi::c_void);
    MikMod_free((*mf).comment as *mut ::core::ffi::c_void);
    MikMod_free((*mf).modtype as *mut ::core::ffi::c_void);
    MikMod_free((*mf).positions as *mut ::core::ffi::c_void);
    MikMod_free((*mf).patterns as *mut ::core::ffi::c_void);
    MikMod_free((*mf).pattrows as *mut ::core::ffi::c_void);
    if !(*mf).tracks.is_null() {
        t = 0 as UWORD;
        while (t as ::core::ffi::c_int) < (*mf).numtrk as ::core::ffi::c_int {
            MikMod_free(*(*mf).tracks.offset(t as isize) as *mut ::core::ffi::c_void);
            t = t.wrapping_add(1);
        }
        MikMod_free((*mf).tracks as *mut ::core::ffi::c_void);
    }
    if !(*mf).instruments.is_null() {
        t = 0 as UWORD;
        while (t as ::core::ffi::c_int) < (*mf).numins as ::core::ffi::c_int {
            ML_XFreeInstrument((*mf).instruments.offset(t as isize));
            t = t.wrapping_add(1);
        }
        MikMod_free((*mf).instruments as *mut ::core::ffi::c_void);
    }
    if !(*mf).samples.is_null() {
        t = 0 as UWORD;
        while (t as ::core::ffi::c_int) < (*mf).numsmp as ::core::ffi::c_int {
            MikMod_free(
                (*(*mf).samples.offset(t as isize)).samplename
                    as *mut ::core::ffi::c_void,
            );
            if (*(*mf).samples.offset(t as isize)).length != 0 {
                uqm_lua_checked!(ML_XFreeSample((*mf).samples.offset(t as isize)), ());
            }
            t = t.wrapping_add(1);
        }
        MikMod_free((*mf).samples as *mut ::core::ffi::c_void);
    }
    memset(
        mf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MODULE>() as size_t,
    );
    if mf != &raw mut of {
        MikMod_free(mf as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn ML_AllocUniMod() -> *mut MODULE {
    return uqm_lua_checked!(MikMod_malloc(::core::mem::size_of::<MODULE>() as size_t), ::core::ptr::null_mut()) as *mut MODULE;
}
unsafe extern "C" fn ML_TryUnpack(
    mut reader: *mut MREADER,
    mut out: *mut *mut ::core::ffi::c_void,
    mut outlen: *mut ::core::ffi::c_long,
) -> BOOL {
    let mut i: ::core::ffi::c_int = 0;
    *out = ::core::ptr::null_mut::<::core::ffi::c_void>();
    *outlen = 0 as ::core::ffi::c_long;
    i = 0 as ::core::ffi::c_int;
    while unpackers[i as usize].is_some() {
        uqm_lua_checked!((*reader)
            .Seek
            .expect(
                "non-null function pointer",
            )(reader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), 0);
        if uqm_lua_checked!(unpackers[i as usize].expect("non-null function pointer")(reader, out, outlen), 0)
            != 0
        {
            return 1 as BOOL;
        }
        i += 1;
    }
    return 0 as BOOL;
}
unsafe extern "C" fn Player_Free_internal(mut mf: *mut MODULE) {
    if !mf.is_null() {
        uqm_lua_checked!(Player_Exit_internal(mf), ());
        uqm_lua_checked!(ML_FreeEx(mf), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_Free(mut mf: *mut MODULE) {
    uqm_lua_checked!(Player_Free_internal(mf), ());
}
unsafe extern "C" fn Player_LoadTitle_internal(mut reader: *mut MREADER) -> *mut CHAR {
    let mut l: *mut MLOADER = ::core::ptr::null_mut::<MLOADER>();
    let mut title: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut unpk: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut newlen: ::core::ffi::c_long = 0;
    modreader = reader;
    MikMod_errno = 0 as ::core::ffi::c_int;
    MikMod_critical = 0 as ::core::ffi::c_int as BOOL;
    uqm_lua_checked!(_mm_iobase_setcur(modreader), ::core::ptr::null_mut());
    if uqm_lua_checked!(ML_TryUnpack(modreader, &raw mut unpk, &raw mut newlen), ::core::ptr::null_mut()) != 0 {
        modreader = uqm_lua_checked!(_mm_new_mem_reader(unpk, newlen), ::core::ptr::null_mut());
        if modreader.is_null() {
            modreader = reader;
            MikMod_free(unpk);
            return ::core::ptr::null_mut::<CHAR>();
        }
    }
    l = firstloader;
    while !l.is_null() {
        uqm_lua_checked!((*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
        if uqm_lua_checked!((*l).Test.expect("non-null function pointer")(), ::core::ptr::null_mut()) != 0 {
            break;
        }
        l = (*l).next as *mut MLOADER;
    }
    if !l.is_null() {
        title = uqm_lua_checked!((*l).LoadTitle.expect("non-null function pointer")(), ::core::ptr::null_mut());
    } else {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        if _mm_errorhandler.is_some() {
            uqm_lua_checked!(_mm_errorhandler.expect("non-null function pointer")(), ::core::ptr::null_mut());
        }
        title = ::core::ptr::null_mut::<CHAR>();
    }
    if modreader != reader {
        _mm_delete_mem_reader(modreader);
        modreader = reader;
        MikMod_free(unpk);
    }
    return title;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadTitleFP(mut fp: *mut FILE) -> *mut CHAR {
    let mut result: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if !fp.is_null()
        && {
            reader = uqm_lua_checked!(_mm_new_file_reader(fp), ::core::ptr::null_mut());
            !reader.is_null()
        }
    {
        result = uqm_lua_checked!(Player_LoadTitle_internal(reader), ::core::ptr::null_mut());
        _mm_delete_file_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadTitleMem(
    mut buffer: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut CHAR {
    let mut result: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if buffer.is_null() || len <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<CHAR>();
    }
    reader = uqm_lua_checked!(_mm_new_mem_reader(
        buffer as *const ::core::ffi::c_void,
        len as ::core::ffi::c_long,
    ), ::core::ptr::null_mut());
    if !reader.is_null() {
        result = uqm_lua_checked!(Player_LoadTitle_internal(reader), ::core::ptr::null_mut());
        _mm_delete_mem_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadTitleGeneric(mut reader: *mut MREADER) -> *mut CHAR {
    let mut result: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    if !reader.is_null() {
        result = uqm_lua_checked!(Player_LoadTitle_internal(reader), ::core::ptr::null_mut());
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadTitle(mut filename: *const CHAR) -> *mut CHAR {
    let mut result: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    fp = uqm_lua_checked!(_mm_fopen(filename, b"rb\0".as_ptr() as *const CHAR), ::core::ptr::null_mut());
    if !fp.is_null() {
        reader = uqm_lua_checked!(_mm_new_file_reader(fp), ::core::ptr::null_mut());
        if !reader.is_null() {
            result = uqm_lua_checked!(Player_LoadTitle_internal(reader), ::core::ptr::null_mut());
            _mm_delete_file_reader(reader);
        }
        _mm_fclose(fp);
    }
    return result;
}
unsafe extern "C" fn Player_LoadGeneric_internal(
    mut reader: *mut MREADER,
    mut maxchan: ::core::ffi::c_int,
    mut curious: BOOL,
) -> *mut MODULE {
    let mut t: ::core::ffi::c_int = 0;
    let mut l: *mut MLOADER = ::core::ptr::null_mut::<MLOADER>();
    let mut ok: BOOL = 0;
    let mut mf: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    let mut unpk: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut newlen: ::core::ffi::c_long = 0;
    modreader = reader;
    MikMod_errno = 0 as ::core::ffi::c_int;
    MikMod_critical = 0 as ::core::ffi::c_int as BOOL;
    uqm_lua_checked!(_mm_iobase_setcur(modreader), ::core::ptr::null_mut());
    if uqm_lua_checked!(ML_TryUnpack(modreader, &raw mut unpk, &raw mut newlen), ::core::ptr::null_mut()) != 0 {
        modreader = uqm_lua_checked!(_mm_new_mem_reader(unpk, newlen), ::core::ptr::null_mut());
        if modreader.is_null() {
            modreader = reader;
            MikMod_free(unpk);
            return ::core::ptr::null_mut::<MODULE>();
        }
    }
    l = firstloader;
    while !l.is_null() {
        uqm_lua_checked!((*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
        if uqm_lua_checked!((*l).Test.expect("non-null function pointer")(), ::core::ptr::null_mut()) != 0 {
            break;
        }
        l = (*l).next as *mut MLOADER;
    }
    if l.is_null() {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        if modreader != reader {
            _mm_delete_mem_reader(modreader);
            modreader = reader;
            MikMod_free(unpk);
        }
        if _mm_errorhandler.is_some() {
            uqm_lua_checked!(_mm_errorhandler.expect("non-null function pointer")(), ::core::ptr::null_mut());
        }
        uqm_lua_checked!((*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
        _mm_iobase_revert(modreader);
        return ::core::ptr::null_mut::<MODULE>();
    }
    if uqm_lua_checked!(UniInit(), ::core::ptr::null_mut()) == 0 {
        if modreader != reader {
            _mm_delete_mem_reader(modreader);
            modreader = reader;
            MikMod_free(unpk);
        }
        if _mm_errorhandler.is_some() {
            uqm_lua_checked!(_mm_errorhandler.expect("non-null function pointer")(), ::core::ptr::null_mut());
        }
        uqm_lua_checked!((*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
        _mm_iobase_revert(modreader);
        return ::core::ptr::null_mut::<MODULE>();
    }
    memset(
        &raw mut of as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MODULE>() as size_t,
    );
    of.bpmlimit = 33 as UWORD;
    of.initvolume = 128 as UBYTE;
    t = 0 as ::core::ffi::c_int;
    while t < 64 as ::core::ffi::c_int {
        of.chanvol[t as usize] = 64 as UBYTE;
        t += 1;
    }
    t = 0 as ::core::ffi::c_int;
    while t < 64 as ::core::ffi::c_int {
        of.panning[t as usize] = (if t + 1 as ::core::ffi::c_int
            & 2 as ::core::ffi::c_int != 0
        {
            255 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as UWORD;
        t += 1;
    }
    if (*l).Init.is_none() || uqm_lua_checked!((*l).Init.expect("non-null function pointer")(), ::core::ptr::null_mut()) != 0 {
        uqm_lua_checked!((*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
        ok = uqm_lua_checked!((*l).Load.expect("non-null function pointer")(curious), ::core::ptr::null_mut());
        if ok != 0 {
            t = 0 as ::core::ffi::c_int;
            while t < of.numsmp as ::core::ffi::c_int {
                if (*of.samples.offset(t as isize)).inflags as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*of.samples.offset(t as isize)).inflags = (*of
                        .samples
                        .offset(t as isize))
                        .flags;
                }
                t += 1;
            }
        }
    } else {
        ok = 0 as ::core::ffi::c_int as BOOL;
    }
    if (*l).Cleanup.is_some() {
        uqm_lua_checked!((*l).Cleanup.expect("non-null function pointer")(), ::core::ptr::null_mut());
    }
    UniCleanup();
    if ok != 0 {
        ok = uqm_lua_checked!(ML_LoadSamples(), ::core::ptr::null_mut());
    }
    if ok != 0 {
        mf = uqm_lua_checked!(ML_AllocUniMod(), ::core::ptr::null_mut());
        ok = !mf.is_null() as ::core::ffi::c_int as BOOL;
    }
    if ok == 0 {
        uqm_lua_checked!(ML_FreeEx(&raw mut of), ::core::ptr::null_mut());
        if modreader != reader {
            _mm_delete_mem_reader(modreader);
            modreader = reader;
            MikMod_free(unpk);
        }
        if _mm_errorhandler.is_some() {
            uqm_lua_checked!(_mm_errorhandler.expect("non-null function pointer")(), ::core::ptr::null_mut());
        }
        uqm_lua_checked!((*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
        _mm_iobase_revert(modreader);
        return ::core::ptr::null_mut::<MODULE>();
    }
    if of.flags as ::core::ffi::c_int & 0x400 as ::core::ffi::c_int == 0 {
        t = 0 as ::core::ffi::c_int;
        while t < of.numchn as ::core::ffi::c_int {
            of.panning[t as usize] = (if t + 1 as ::core::ffi::c_int
                & 2 as ::core::ffi::c_int != 0
            {
                192 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int
            }) as UWORD;
            t += 1;
        }
    }
    memcpy(
        mf as *mut ::core::ffi::c_void,
        &raw mut of as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MODULE>() as size_t,
    );
    if maxchan > 0 as ::core::ffi::c_int {
        if (*mf).flags as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
            && ((*mf).numchn as ::core::ffi::c_int) < maxchan
        {
            maxchan = (*mf).numchn as ::core::ffi::c_int;
        } else if (*mf).numvoices as ::core::ffi::c_int != 0
            && ((*mf).numvoices as ::core::ffi::c_int) < maxchan
        {
            maxchan = (*mf).numvoices as ::core::ffi::c_int;
        }
        if maxchan < (*mf).numchn as ::core::ffi::c_int {
            (*mf).flags = ((*mf).flags as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int)
                as UWORD;
        }
        ok = (uqm_lua_checked!(MikMod_SetNumVoices_internal(maxchan, -1 as ::core::ffi::c_int), ::core::ptr::null_mut()) == 0)
            as ::core::ffi::c_int as BOOL;
    }
    if ok != 0 {
        ok = (uqm_lua_checked!(SL_LoadSamples(), ::core::ptr::null_mut()) == 0) as ::core::ffi::c_int as BOOL;
    }
    if ok != 0 {
        ok = (uqm_lua_checked!(Player_Init(mf), ::core::ptr::null_mut()) == 0) as ::core::ffi::c_int as BOOL;
    }
    if modreader != reader {
        _mm_delete_mem_reader(modreader);
        modreader = reader;
        MikMod_free(unpk);
    }
    _mm_iobase_revert(modreader);
    if ok == 0 {
        uqm_lua_checked!(Player_Free_internal(mf), ::core::ptr::null_mut());
        return ::core::ptr::null_mut::<MODULE>();
    }
    return mf;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadGeneric(
    mut reader: *mut MREADER,
    mut maxchan: ::core::ffi::c_int,
    mut curious: BOOL,
) -> *mut MODULE {
    let mut result: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    result = uqm_lua_checked!(Player_LoadGeneric_internal(reader, maxchan, curious), ::core::ptr::null_mut());
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadMem(
    mut buffer: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut maxchan: ::core::ffi::c_int,
    mut curious: BOOL,
) -> *mut MODULE {
    let mut result: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if buffer.is_null() || len <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<MODULE>();
    }
    reader = uqm_lua_checked!(_mm_new_mem_reader(
        buffer as *const ::core::ffi::c_void,
        len as ::core::ffi::c_long,
    ), ::core::ptr::null_mut());
    if !reader.is_null() {
        result = uqm_lua_checked!(Player_LoadGeneric(reader, maxchan, curious), ::core::ptr::null_mut());
        _mm_delete_mem_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_LoadFP(
    mut fp: *mut FILE,
    mut maxchan: ::core::ffi::c_int,
    mut curious: BOOL,
) -> *mut MODULE {
    let mut result: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    let mut reader: *mut MREADER = ::core::ptr::null_mut::<MREADER>();
    if !fp.is_null()
        && {
            reader = uqm_lua_checked!(_mm_new_file_reader(fp), ::core::ptr::null_mut()) as *mut MREADER;
            !reader.is_null()
        }
    {
        result = uqm_lua_checked!(Player_LoadGeneric(reader, maxchan, curious), ::core::ptr::null_mut());
        _mm_delete_file_reader(reader);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_Load(
    mut filename: *const CHAR,
    mut maxchan: ::core::ffi::c_int,
    mut curious: BOOL,
) -> *mut MODULE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut mf: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    fp = uqm_lua_checked!(_mm_fopen(filename, b"rb\0".as_ptr() as *const CHAR), ::core::ptr::null_mut());
    if !fp.is_null() {
        mf = uqm_lua_checked!(Player_LoadFP(fp, maxchan, curious), ::core::ptr::null_mut());
        _mm_fclose(fp);
    }
    return mf;
}
