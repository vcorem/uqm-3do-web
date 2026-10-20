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

  Screamtracker 2 (STM) module loader

==============================================================================*/

// Rust translation of src/libs/mikmod/load_stm.c.
extern "C" {
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    static mut MikMod_errno: ::core::ffi::c_int;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_strdup(_: *const CHAR) -> *mut CHAR;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    fn _mm_read_string(_: *mut CHAR, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn _mm_read_I_UWORD(_: *mut MREADER) -> UWORD;
    fn _mm_read_I_ULONG(_: *mut MREADER) -> ULONG;
    fn UniReset();
    fn UniNewline();
    fn UniDup() -> *mut UBYTE;
    fn UniEffect(_: UWORD, _: UWORD);
    fn UniPTEffect(_: UBYTE, _: UBYTE);
    static mut modreader: *mut MREADER;
    static mut of: MODULE;
    fn AllocPositions(_: ::core::ffi::c_int) -> BOOL;
    fn AllocPatterns() -> BOOL;
    fn AllocTracks() -> BOOL;
    fn AllocSamples() -> BOOL;
    fn DupStr(_: *const CHAR, _: UWORD, _: BOOL) -> *mut CHAR;
    static mut STM_Signatures: [*const CHAR; 3];
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STMNOTE {
    pub note: UBYTE,
    pub insvol: UBYTE,
    pub volcmd: UBYTE,
    pub cmdinf: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STMHEADER {
    pub songname: [CHAR; 20],
    pub trackername: [CHAR; 8],
    pub unused: UBYTE,
    pub filetype: UBYTE,
    pub ver_major: UBYTE,
    pub ver_minor: UBYTE,
    pub inittempo: UBYTE,
    pub numpat: UBYTE,
    pub globalvol: UBYTE,
    pub reserved: [UBYTE; 13],
    pub sample: [STMSAMPLE; 31],
    pub patorder: [UBYTE; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STMSAMPLE {
    pub filename: [CHAR; 12],
    pub unused: UBYTE,
    pub instdisk: UBYTE,
    pub reserved: UWORD,
    pub length: UWORD,
    pub loopbeg: UWORD,
    pub loopend: UWORD,
    pub volume: UBYTE,
    pub reserved2: UBYTE,
    pub c2spd: UWORD,
    pub reserved3: ULONG,
    pub isa: UWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const UNI_NOTE: Self = Self(1);
    pub const UNI_INSTRUMENT: Self = Self(2);
    pub const UNI_PTEFFECT0: Self = Self(3);
    pub const UNI_PTEFFECT1: Self = Self(4);
    pub const UNI_PTEFFECT2: Self = Self(5);
    pub const UNI_PTEFFECT3: Self = Self(6);
    pub const UNI_PTEFFECT4: Self = Self(7);
    pub const UNI_PTEFFECT5: Self = Self(8);
    pub const UNI_PTEFFECT6: Self = Self(9);
    pub const UNI_PTEFFECT7: Self = Self(10);
    pub const UNI_PTEFFECT8: Self = Self(11);
    pub const UNI_PTEFFECT9: Self = Self(12);
    pub const UNI_PTEFFECTA: Self = Self(13);
    pub const UNI_PTEFFECTB: Self = Self(14);
    pub const UNI_PTEFFECTC: Self = Self(15);
    pub const UNI_PTEFFECTD: Self = Self(16);
    pub const UNI_PTEFFECTE: Self = Self(17);
    pub const UNI_PTEFFECTF: Self = Self(18);
    pub const UNI_S3MEFFECTA: Self = Self(19);
    pub const UNI_S3MEFFECTD: Self = Self(20);
    pub const UNI_S3MEFFECTE: Self = Self(21);
    pub const UNI_S3MEFFECTF: Self = Self(22);
    pub const UNI_S3MEFFECTI: Self = Self(23);
    pub const UNI_S3MEFFECTQ: Self = Self(24);
    pub const UNI_S3MEFFECTR: Self = Self(25);
    pub const UNI_S3MEFFECTT: Self = Self(26);
    pub const UNI_S3MEFFECTU: Self = Self(27);
    pub const UNI_KEYOFF: Self = Self(28);
    pub const UNI_KEYFADE: Self = Self(29);
    pub const UNI_VOLEFFECTS: Self = Self(30);
    pub const UNI_XMEFFECT4: Self = Self(31);
    pub const UNI_XMEFFECT6: Self = Self(32);
    pub const UNI_XMEFFECTA: Self = Self(33);
    pub const UNI_XMEFFECTE1: Self = Self(34);
    pub const UNI_XMEFFECTE2: Self = Self(35);
    pub const UNI_XMEFFECTEA: Self = Self(36);
    pub const UNI_XMEFFECTEB: Self = Self(37);
    pub const UNI_XMEFFECTG: Self = Self(38);
    pub const UNI_XMEFFECTH: Self = Self(39);
    pub const UNI_XMEFFECTL: Self = Self(40);
    pub const UNI_XMEFFECTP: Self = Self(41);
    pub const UNI_XMEFFECTX1: Self = Self(42);
    pub const UNI_XMEFFECTX2: Self = Self(43);
    pub const UNI_ITEFFECTG: Self = Self(44);
    pub const UNI_ITEFFECTH: Self = Self(45);
    pub const UNI_ITEFFECTI: Self = Self(46);
    pub const UNI_ITEFFECTM: Self = Self(47);
    pub const UNI_ITEFFECTN: Self = Self(48);
    pub const UNI_ITEFFECTP: Self = Self(49);
    pub const UNI_ITEFFECTT: Self = Self(50);
    pub const UNI_ITEFFECTU: Self = Self(51);
    pub const UNI_ITEFFECTW: Self = Self(52);
    pub const UNI_ITEFFECTY: Self = Self(53);
    pub const UNI_ITEFFECTZ: Self = Self(54);
    pub const UNI_ITEFFECTS0: Self = Self(55);
    pub const UNI_ULTEFFECT9: Self = Self(56);
    pub const UNI_MEDSPEED: Self = Self(57);
    pub const UNI_MEDEFFECTF1: Self = Self(58);
    pub const UNI_MEDEFFECTF2: Self = Self(59);
    pub const UNI_MEDEFFECTF3: Self = Self(60);
    pub const UNI_OKTARP: Self = Self(61);
    pub const UNI_LAST: Self = Self(62);
}
static mut stmbuf: *mut STMNOTE = ::core::ptr::null_mut::<STMNOTE>();
static mut mh: *mut STMHEADER = ::core::ptr::null_mut::<STMHEADER>();
static mut STM_Version: [*const CHAR; 3] = [
    b"Screamtracker 2\0".as_ptr() as *const CHAR,
    b"Converted by MOD2STM (STM format)\0".as_ptr() as *const CHAR,
    b"Wuzamod (STM format)\0".as_ptr() as *const CHAR,
];
unsafe extern "C" fn STM_Test() -> BOOL {
    let mut str: [UBYTE; 44] = [0; 44];
    let mut t: ::core::ffi::c_int = 0;
    uqm_lua_checked!((*modreader)
        .Seek
        .expect(
            "non-null function pointer",
        )(modreader, 20 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), 0);
    uqm_lua_checked!((*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut str as *mut UBYTE as *mut ::core::ffi::c_void,
        44 as size_t,
    ), 0);
    if str[9usize] as ::core::ffi::c_int != 2 as ::core::ffi::c_int {
        return 0 as BOOL;
    }
    if memcmp(
        (&raw mut str as *mut UBYTE).offset(40 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        b"SCRM\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < 3 as ::core::ffi::c_int {
        if memcmp(
            &raw mut str as *mut UBYTE as *const ::core::ffi::c_void,
            STM_Signatures[t as usize] as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0
        {
            return 1 as BOOL;
        }
        t += 1;
    }
    return 0 as BOOL;
}
unsafe extern "C" fn STM_Init() -> BOOL {
    mh = uqm_lua_checked!(MikMod_malloc(::core::mem::size_of::<STMHEADER>()), 0) as *mut STMHEADER;
    if mh.is_null() {
        return 0 as BOOL;
    }
    stmbuf = uqm_lua_checked!(MikMod_calloc(
        (64 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint) as size_t,
        ::core::mem::size_of::<STMNOTE>(),
    ), 0) as *mut STMNOTE;
    if stmbuf.is_null() {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn STM_Cleanup() {
    MikMod_free(mh as *mut ::core::ffi::c_void);
    MikMod_free(stmbuf as *mut ::core::ffi::c_void);
    mh = ::core::ptr::null_mut::<STMHEADER>();
    stmbuf = ::core::ptr::null_mut::<STMNOTE>();
}
unsafe extern "C" fn STM_ConvertNote(mut n: *mut STMNOTE) {
    let mut note: UBYTE = 0;
    let mut ins: UBYTE = 0;
    let mut vol: UBYTE = 0;
    let mut cmd: UBYTE = 0;
    let mut inf: UBYTE = 0;
    note = (*n).note;
    ins = ((*n).insvol as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as UBYTE;
    vol = (((*n).insvol as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
        + (((*n).volcmd as ::core::ffi::c_int & 0x70 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)) as UBYTE;
    cmd = ((*n).volcmd as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as UBYTE;
    inf = (*n).cmdinf;
    if ins as ::core::ffi::c_int != 0
        && (ins as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
    {
        UniEffect(
            C2Rust_Unnamed_0::UNI_INSTRUMENT.0 as ::core::ffi::c_int as UWORD,
            (ins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD,
        );
    }
    if note as ::core::ffi::c_int == 254 as ::core::ffi::c_int
        || note as ::core::ffi::c_int == 252 as ::core::ffi::c_int
    {
        UniPTEffect(0xc as UBYTE, 0 as UBYTE);
        (*n).volcmd = ((*n).volcmd as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int)
            as UBYTE;
    } else if (note as ::core::ffi::c_int) < 251 as ::core::ffi::c_int {
        UniEffect(
            C2Rust_Unnamed_0::UNI_NOTE.0 as ::core::ffi::c_int as UWORD,
            (((note as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int) * 12 as ::core::ffi::c_int
                + (note as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)) as UWORD,
        );
    }
    if (*n).volcmd as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0
        && (vol as ::core::ffi::c_int) < 65 as ::core::ffi::c_int
    {
        UniPTEffect(0xc as UBYTE, vol);
    }
    if cmd as ::core::ffi::c_int != 255 as ::core::ffi::c_int {
        's_170: {
            match cmd as ::core::ffi::c_int {
                1 => {
                    UniPTEffect(
                        0xf as UBYTE,
                        (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE,
                    );
                    break 's_170;
                }
                2 => {
                    UniPTEffect(0xb as UBYTE, inf);
                    break 's_170;
                }
                3 => {
                    UniPTEffect(
                        0xd as UBYTE,
                        (((inf as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                            >> 4 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int
                            + (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as UBYTE,
                    );
                    break 's_170;
                }
                4 => {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_S3MEFFECTD.0 as ::core::ffi::c_int
                            as UWORD,
                        inf as UWORD,
                    );
                    break 's_170;
                }
                5 => {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_S3MEFFECTE.0 as ::core::ffi::c_int
                            as UWORD,
                        inf as UWORD,
                    );
                    break 's_170;
                }
                6 => {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_S3MEFFECTF.0 as ::core::ffi::c_int
                            as UWORD,
                        inf as UWORD,
                    );
                    break 's_170;
                }
                7 => {
                    UniPTEffect(0x3 as UBYTE, inf);
                    break 's_170;
                }
                8 => {
                    UniPTEffect(0x4 as UBYTE, inf);
                    break 's_170;
                }
                9 => {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_S3MEFFECTI.0 as ::core::ffi::c_int
                            as UWORD,
                        inf as UWORD,
                    );
                    break 's_170;
                }
                0 => {
                    if inf == 0 {
                        break 's_170;
                    }
                }
                0xa => {}
                0xb => {
                    UniPTEffect(0x4 as UBYTE, 0 as UBYTE);
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_S3MEFFECTD.0 as ::core::ffi::c_int
                            as UWORD,
                        inf as UWORD,
                    );
                    break 's_170;
                }
                0xc => {
                    UniPTEffect(0x3 as UBYTE, 0 as UBYTE);
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_S3MEFFECTD.0 as ::core::ffi::c_int
                            as UWORD,
                        inf as UWORD,
                    );
                    break 's_170;
                }
                0x18 => {
                    UniPTEffect(0x8 as UBYTE, inf);
                    of.flags = (of.flags as ::core::ffi::c_int
                        | 0x400 as ::core::ffi::c_int) as UWORD;
                    break 's_170;
                }
                _ => {
                    break 's_170;
                }
            }
            UniPTEffect(0 as UBYTE, inf);
        }
    }
}
unsafe extern "C" fn STM_ConvertTrack(mut n: *mut STMNOTE) -> *mut UBYTE {
    let mut t: ::core::ffi::c_int = 0;
    UniReset();
    t = 0 as ::core::ffi::c_int;
    while t < 64 as ::core::ffi::c_int {
        STM_ConvertNote(n);
        UniNewline();
        n = n.offset(of.numchn as ::core::ffi::c_int as isize);
        t += 1;
    }
    return uqm_lua_checked!(UniDup(), ::core::ptr::null_mut());
}
unsafe extern "C" fn STM_LoadPatterns() -> BOOL {
    let mut t: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut tracks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if uqm_lua_checked!(AllocPatterns(), 0) == 0 {
        return 0 as BOOL;
    }
    if uqm_lua_checked!(AllocTracks(), 0) == 0 {
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < of.numpat as ::core::ffi::c_int {
        s = 0 as ::core::ffi::c_int;
        while (s as ::core::ffi::c_uint)
            < (64 as ::core::ffi::c_uint).wrapping_mul(of.numchn as ::core::ffi::c_uint)
        {
            (*stmbuf.offset(s as isize)).note = uqm_lua_checked!((*modreader)
                .Get
                .expect("non-null function pointer")(modreader), 0) as UBYTE;
            (*stmbuf.offset(s as isize)).insvol = uqm_lua_checked!((*modreader)
                .Get
                .expect("non-null function pointer")(modreader), 0) as UBYTE;
            (*stmbuf.offset(s as isize)).volcmd = uqm_lua_checked!((*modreader)
                .Get
                .expect("non-null function pointer")(modreader), 0) as UBYTE;
            (*stmbuf.offset(s as isize)).cmdinf = uqm_lua_checked!((*modreader)
                .Get
                .expect("non-null function pointer")(modreader), 0) as UBYTE;
            s += 1;
        }
        if uqm_lua_checked!((*modreader).Eof.expect("non-null function pointer")(modreader), 0) != 0 {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        s = 0 as ::core::ffi::c_int;
        while s < of.numchn as ::core::ffi::c_int {
            let c2rust_fresh0 = tracks;
            tracks += 1;
            let c2rust_lvalue_ptr = &raw mut *of.tracks.offset(c2rust_fresh0 as isize);
            *c2rust_lvalue_ptr = uqm_lua_checked!(STM_ConvertTrack(stmbuf.offset(s as isize)), 0);
            if (*c2rust_lvalue_ptr).is_null() {
                return 0 as BOOL;
            }
            s += 1;
        }
        t += 1;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn STM_Load(mut curious: BOOL) -> BOOL {
    let mut t: ::core::ffi::c_int = 0;
    let mut MikMod_ISA: ULONG = 0;
    let mut q: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    uqm_lua_checked!(_mm_read_string(
        &raw mut (*mh).songname as *mut CHAR,
        20 as ::core::ffi::c_int,
        modreader,
    ), 0);
    uqm_lua_checked!(_mm_read_string(
        &raw mut (*mh).trackername as *mut CHAR,
        8 as ::core::ffi::c_int,
        modreader,
    ), 0);
    (*mh).unused = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    (*mh).filetype = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    (*mh).ver_major = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    (*mh).ver_minor = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    (*mh).inittempo = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    if (*mh).inittempo == 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    (*mh).numpat = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    (*mh).globalvol = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
        as UBYTE;
    uqm_lua_checked!((*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut (*mh).reserved as *mut UBYTE as *mut ::core::ffi::c_void,
        13 as size_t,
    ), 0);
    if (*mh).numpat as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < 31 as ::core::ffi::c_int {
        let mut s: *mut STMSAMPLE = (&raw mut (*mh).sample as *mut STMSAMPLE)
            .offset(t as isize);
        uqm_lua_checked!(_mm_read_string(
            &raw mut (*s).filename as *mut CHAR,
            12 as ::core::ffi::c_int,
            modreader,
        ), 0);
        (*s).unused = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
            as UBYTE;
        (*s).instdisk = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
            as UBYTE;
        (*s).reserved = uqm_lua_checked!(_mm_read_I_UWORD(modreader), 0);
        (*s).length = uqm_lua_checked!(_mm_read_I_UWORD(modreader), 0);
        (*s).loopbeg = uqm_lua_checked!(_mm_read_I_UWORD(modreader), 0);
        (*s).loopend = uqm_lua_checked!(_mm_read_I_UWORD(modreader), 0);
        (*s).volume = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
            as UBYTE;
        (*s).reserved2 = uqm_lua_checked!((*modreader).Get.expect("non-null function pointer")(modreader), 0)
            as UBYTE;
        (*s).c2spd = uqm_lua_checked!(_mm_read_I_UWORD(modreader), 0);
        (*s).reserved3 = uqm_lua_checked!(_mm_read_I_ULONG(modreader), 0);
        (*s).isa = uqm_lua_checked!(_mm_read_I_UWORD(modreader), 0);
        t += 1;
    }
    uqm_lua_checked!((*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut (*mh).patorder as *mut UBYTE as *mut ::core::ffi::c_void,
        128 as size_t,
    ), 0);
    if uqm_lua_checked!((*modreader).Eof.expect("non-null function pointer")(modreader), 0) != 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < 3 as ::core::ffi::c_int {
        if memcmp(
            &raw mut (*mh).trackername as *mut CHAR as *const ::core::ffi::c_void,
            STM_Signatures[t as usize] as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0
        {
            break;
        }
        t += 1;
    }
    of.modtype = uqm_lua_checked!(MikMod_strdup(STM_Version[t as usize]), 0);
    of.songname = uqm_lua_checked!(DupStr(&raw mut (*mh).songname as *mut CHAR, 20 as UWORD, 1 as BOOL), 0);
    of.numpat = (*mh).numpat as UWORD;
    of.inittempo = 125 as UWORD;
    of.initspeed = ((*mh).inittempo as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        as UBYTE;
    of.numchn = 4 as UBYTE;
    of.reppos = 0 as UWORD;
    of.flags = (of.flags as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as UWORD;
    of.bpmlimit = 32 as UWORD;
    t = 0 as ::core::ffi::c_int;
    if uqm_lua_checked!(AllocPositions(0x80 as ::core::ffi::c_int), 0) == 0 {
        return 0 as BOOL;
    }
    while (*mh).patorder[t as usize] as ::core::ffi::c_int <= 99 as ::core::ffi::c_int
        && ((*mh).patorder[t as usize] as ::core::ffi::c_int)
            < (*mh).numpat as ::core::ffi::c_int
    {
        *of.positions.offset(t as isize) = (*mh).patorder[t as usize] as UWORD;
        t += 1;
        if t == 0x80 as ::core::ffi::c_int {
            MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
            return 0 as BOOL;
        }
    }
    if (*mh).patorder[t as usize] as ::core::ffi::c_int <= 99 as ::core::ffi::c_int {
        t += 1;
    }
    of.numpos = t as UWORD;
    of.numtrk = (of.numpat as ::core::ffi::c_int * of.numchn as ::core::ffi::c_int)
        as UWORD;
    of.numsmp = 31 as UWORD;
    of.numins = of.numsmp;
    if uqm_lua_checked!(AllocSamples(), 0) == 0 {
        return 0 as BOOL;
    }
    if uqm_lua_checked!(STM_LoadPatterns(), 0) == 0 {
        return 0 as BOOL;
    }
    MikMod_ISA = uqm_lua_checked!((*modreader).Tell.expect("non-null function pointer")(modreader), 0)
        as ULONG;
    MikMod_ISA = MikMod_ISA.wrapping_add(15 as ULONG) & 0xfffffff0 as ULONG;
    q = of.samples as *mut SAMPLE;
    t = 0 as ::core::ffi::c_int;
    while t < of.numsmp as ::core::ffi::c_int {
        (*q).samplename = uqm_lua_checked!(DupStr(
            &raw mut (*(&raw mut (*mh).sample as *mut STMSAMPLE).offset(t as isize))
                .filename as *mut CHAR,
            12 as UWORD,
            1 as BOOL,
        ), 0);
        (*q).speed = ((*mh).sample[t as usize].c2spd as ::core::ffi::c_int
            * 8363 as ::core::ffi::c_int / 8448 as ::core::ffi::c_int) as ULONG;
        (*q).volume = (*mh).sample[t as usize].volume;
        (*q).length = (*mh).sample[t as usize].length as ULONG;
        if (*q).length == 1 as ULONG {
            (*q).length = 0 as ULONG;
        }
        (*q).loopstart = (*mh).sample[t as usize].loopbeg as ULONG;
        (*q).loopend = (*mh).sample[t as usize].loopend as ULONG;
        (*q).seekpos = MikMod_ISA;
        MikMod_ISA = MikMod_ISA.wrapping_add((*q).length);
        MikMod_ISA = MikMod_ISA.wrapping_add(15 as ULONG) & 0xfffffff0 as ULONG;
        (*q).flags = 0x4 as UWORD;
        if (*q).loopend != 0 && (*q).loopend != 0xffff as ULONG {
            (*q).flags = ((*q).flags as ::core::ffi::c_int | 0x100 as ::core::ffi::c_int)
                as UWORD;
        }
        t += 1;
        q = q.offset(1);
    }
    return 1 as BOOL;
}
unsafe extern "C" fn STM_LoadTitle() -> *mut CHAR {
    let mut s: [CHAR; 20] = [0; 20];
    uqm_lua_checked!((*modreader)
        .Seek
        .expect(
            "non-null function pointer",
        )(modreader, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
    if uqm_lua_checked!((*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(modreader, &raw mut s as *mut CHAR as *mut ::core::ffi::c_void, 20 as size_t), ::core::ptr::null_mut())
        == 0
    {
        return ::core::ptr::null_mut::<CHAR>();
    }
    return uqm_lua_checked!(DupStr(&raw mut s as *mut CHAR, 20 as UWORD, 1 as BOOL), ::core::ptr::null_mut());
}
#[no_mangle]
pub static mut load_stm: MLOADER = MLOADER {
    next: ::core::ptr::null_mut::<MLOADER>(),
    r#type: b"STM\0".as_ptr() as *const CHAR,
    version: b"STM (Scream Tracker)\0".as_ptr() as *const CHAR,
    Init: Some(STM_Init as unsafe extern "C" fn() -> BOOL),
    Test: Some(STM_Test as unsafe extern "C" fn() -> BOOL),
    Load: Some(STM_Load as unsafe extern "C" fn(BOOL) -> BOOL),
    Cleanup: Some(STM_Cleanup as unsafe extern "C" fn() -> ()),
    LoadTitle: Some(STM_LoadTitle as unsafe extern "C" fn() -> *mut CHAR),
};
