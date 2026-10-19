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

  Fasttracker (XM) module loader

==============================================================================*/

// Rust translation of src/libs/mikmod/load_xm.c.
extern "C" {
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    static mut MikMod_errno: ::core::ffi::c_int;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_realloc(
        _: *mut ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn MikMod_strdup(_: *const CHAR) -> *mut CHAR;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    fn _mm_read_string(_: *mut CHAR, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn _mm_read_I_UWORD(_: *mut MREADER) -> UWORD;
    fn _mm_read_I_ULONG(_: *mut MREADER) -> ULONG;
    fn _mm_read_I_UWORDS(_: *mut UWORD, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
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
    fn AllocInstruments() -> BOOL;
    fn AllocSamples() -> BOOL;
    fn DupStr(_: *const CHAR, _: UWORD, _: BOOL) -> *mut CHAR;
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
pub struct XMHEADER {
    pub id: [CHAR; 17],
    pub songname: [CHAR; 21],
    pub trackername: [CHAR; 20],
    pub version: UWORD,
    pub headersize: ULONG,
    pub songlength: UWORD,
    pub restart: UWORD,
    pub numchn: UWORD,
    pub numpat: UWORD,
    pub numins: UWORD,
    pub flags: UWORD,
    pub tempo: UWORD,
    pub bpm: UWORD,
    pub orders: [UBYTE; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMWAVHEADER {
    pub length: ULONG,
    pub loopstart: ULONG,
    pub looplength: ULONG,
    pub volume: UBYTE,
    pub finetune: SBYTE,
    pub r#type: UBYTE,
    pub panning: UBYTE,
    pub relnote: SBYTE,
    pub reserved: UBYTE,
    pub samplename: [CHAR; 22],
    pub vibtype: UBYTE,
    pub vibsweep: UBYTE,
    pub vibdepth: UBYTE,
    pub vibrate: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMINSTHEADER {
    pub size: ULONG,
    pub name: [CHAR; 22],
    pub r#type: UBYTE,
    pub numsmp: UWORD,
    pub ssize: ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMPATCHHEADER {
    pub what: [UBYTE; 96],
    pub volenv: [UWORD; 24],
    pub panenv: [UWORD; 24],
    pub volpts: UBYTE,
    pub panpts: UBYTE,
    pub volsus: UBYTE,
    pub volbeg: UBYTE,
    pub volend: UBYTE,
    pub pansus: UBYTE,
    pub panbeg: UBYTE,
    pub panend: UBYTE,
    pub volflg: UBYTE,
    pub panflg: UBYTE,
    pub vibflg: UBYTE,
    pub vibsweep: UBYTE,
    pub vibdepth: UBYTE,
    pub vibrate: UBYTE,
    pub volfade: UWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMNOTE {
    pub note: UBYTE,
    pub ins: UBYTE,
    pub vol: UBYTE,
    pub eff: UBYTE,
    pub dat: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMPATHEADER {
    pub size: ULONG,
    pub packing: UBYTE,
    pub numrows: UWORD,
    pub packsize: SWORD,
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
static mut xmpat: *mut XMNOTE = ::core::ptr::null_mut::<XMNOTE>();
static mut mh: *mut XMHEADER = ::core::ptr::null_mut::<XMHEADER>();
static mut nextwav: *mut ULONG = ::core::ptr::null_mut::<ULONG>();
static mut s: *mut XMWAVHEADER = ::core::ptr::null_mut::<XMWAVHEADER>();
static mut wh: *mut XMWAVHEADER = ::core::ptr::null_mut::<XMWAVHEADER>();
unsafe extern "C" fn XM_Test() -> BOOL {
    let mut id: [UBYTE; 38] = [0; 38];
    if (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(modreader, &raw mut id as *mut UBYTE as *mut ::core::ffi::c_void, 38 as size_t)
        == 0
    {
        return 0 as BOOL;
    }
    if memcmp(
        &raw mut id as *mut UBYTE as *const ::core::ffi::c_void,
        b"Extended Module: \0".as_ptr() as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        17 as size_t,
    ) != 0
    {
        return 0 as BOOL;
    }
    if id[37usize] as ::core::ffi::c_int == 0x1a as ::core::ffi::c_int {
        return 1 as BOOL;
    }
    return 0 as BOOL;
}
unsafe extern "C" fn XM_Init() -> BOOL {
    mh = MikMod_malloc(::core::mem::size_of::<XMHEADER>()) as *mut XMHEADER;
    if mh.is_null() {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn XM_Cleanup() {
    MikMod_free(mh as *mut ::core::ffi::c_void);
    mh = ::core::ptr::null_mut::<XMHEADER>();
}
unsafe extern "C" fn XM_ReadNote(mut n: *mut XMNOTE) -> ::core::ffi::c_int {
    let mut cmp: UBYTE = 0;
    let mut result: UBYTE = 1 as UBYTE;
    memset(
        n as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<XMNOTE>(),
    );
    cmp = (*modreader).Get.expect("non-null function pointer")(modreader) as UBYTE;
    if cmp as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        if cmp as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            result = result.wrapping_add(1);
            (*n).note = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE;
        }
        if cmp as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
            result = result.wrapping_add(1);
            (*n).ins = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE;
        }
        if cmp as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
            result = result.wrapping_add(1);
            (*n).vol = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE;
        }
        if cmp as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
            result = result.wrapping_add(1);
            (*n).eff = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE;
        }
        if cmp as ::core::ffi::c_int & 16 as ::core::ffi::c_int != 0 {
            result = result.wrapping_add(1);
            (*n).dat = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE;
        }
    } else {
        (*n).note = cmp;
        (*n).ins = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        (*n).vol = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        (*n).eff = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        (*n).dat = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        result = (result as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as UBYTE;
    }
    return result as ::core::ffi::c_int;
}
unsafe extern "C" fn XM_Convert(
    mut xmtrack: *mut XMNOTE,
    mut rows: UWORD,
) -> *mut UBYTE {
    let mut t: ::core::ffi::c_int = 0;
    let mut note: UBYTE = 0;
    let mut ins: UBYTE = 0;
    let mut vol: UBYTE = 0;
    let mut eff: UBYTE = 0;
    let mut dat: UBYTE = 0;
    UniReset();
    t = 0 as ::core::ffi::c_int;
    while t < rows as ::core::ffi::c_int {
        note = (*xmtrack).note;
        ins = (*xmtrack).ins;
        vol = (*xmtrack).vol;
        eff = (*xmtrack).eff;
        dat = (*xmtrack).dat;
        if note != 0 {
            if note as ::core::ffi::c_int
                > 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
            {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_KEYFADE.0 as ::core::ffi::c_int as UWORD,
                    0 as UWORD,
                );
            } else {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_NOTE.0 as ::core::ffi::c_int as UWORD,
                    (note as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD,
                );
            }
        }
        if ins != 0 {
            UniEffect(
                C2Rust_Unnamed_0::UNI_INSTRUMENT.0 as ::core::ffi::c_int as UWORD,
                (ins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD,
            );
        }
        match vol as ::core::ffi::c_int >> 4 as ::core::ffi::c_int {
            0x6 => {
                if vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_XMEFFECTA.0 as ::core::ffi::c_int as UWORD,
                        (vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UWORD,
                    );
                }
            }
            0x7 => {
                if vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_XMEFFECTA.0 as ::core::ffi::c_int as UWORD,
                        ((vol as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UWORD,
                    );
                }
            }
            0x8 => {
                UniPTEffect(
                    0xe as UBYTE,
                    (0xb0 as ::core::ffi::c_int
                        | vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE,
                );
            }
            0x9 => {
                UniPTEffect(
                    0xe as UBYTE,
                    (0xa0 as ::core::ffi::c_int
                        | vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE,
                );
            }
            0xa => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECT4.0 as ::core::ffi::c_int as UWORD,
                    ((vol as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UWORD,
                );
            }
            0xb => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECT4.0 as ::core::ffi::c_int as UWORD,
                    (vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UWORD,
                );
            }
            0xc => {
                UniPTEffect(
                    0x8 as UBYTE,
                    ((vol as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UBYTE,
                );
            }
            0xd => {
                if vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_XMEFFECTP.0 as ::core::ffi::c_int as UWORD,
                        (vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UWORD,
                    );
                }
            }
            0xe => {
                if vol as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
                    UniEffect(
                        C2Rust_Unnamed_0::UNI_XMEFFECTP.0 as ::core::ffi::c_int as UWORD,
                        ((vol as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UWORD,
                    );
                }
            }
            0xf => {
                UniPTEffect(
                    0x3 as UBYTE,
                    ((vol as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UBYTE,
                );
            }
            _ => {
                if vol as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && vol as ::core::ffi::c_int <= 0x50 as ::core::ffi::c_int
                {
                    UniPTEffect(
                        0xc as UBYTE,
                        (vol as ::core::ffi::c_int - 0x10 as ::core::ffi::c_int) as UBYTE,
                    );
                }
            }
        }
        match eff as ::core::ffi::c_int {
            0x4 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECT4.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            0x6 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECT6.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            0xa => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECTA.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            0xe => {
                match dat as ::core::ffi::c_int >> 4 as ::core::ffi::c_int {
                    0x1 => {
                        UniEffect(
                            C2Rust_Unnamed_0::UNI_XMEFFECTE1.0 as ::core::ffi::c_int
                                as UWORD,
                            (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UWORD,
                        );
                    }
                    0x2 => {
                        UniEffect(
                            C2Rust_Unnamed_0::UNI_XMEFFECTE2.0 as ::core::ffi::c_int
                                as UWORD,
                            (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UWORD,
                        );
                    }
                    0xa => {
                        UniEffect(
                            C2Rust_Unnamed_0::UNI_XMEFFECTEA.0 as ::core::ffi::c_int
                                as UWORD,
                            (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UWORD,
                        );
                    }
                    0xb => {
                        UniEffect(
                            C2Rust_Unnamed_0::UNI_XMEFFECTEB.0 as ::core::ffi::c_int
                                as UWORD,
                            (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UWORD,
                        );
                    }
                    _ => {
                        UniPTEffect(eff, dat);
                    }
                }
            }
            16 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECTG.0 as ::core::ffi::c_int as UWORD,
                    (if dat as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
                        128 as ::core::ffi::c_int
                    } else {
                        (dat as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    }) as UWORD,
                );
            }
            17 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECTH.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            20 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_KEYFADE.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            21 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECTL.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            25 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_XMEFFECTP.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            27 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_S3MEFFECTQ.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            29 => {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_S3MEFFECTI.0 as ::core::ffi::c_int as UWORD,
                    dat as UWORD,
                );
            }
            33 => {
                match dat as ::core::ffi::c_int >> 4 as ::core::ffi::c_int {
                    1 => {
                        UniEffect(
                            C2Rust_Unnamed_0::UNI_XMEFFECTX1.0 as ::core::ffi::c_int
                                as UWORD,
                            (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UWORD,
                        );
                    }
                    2 => {
                        UniEffect(
                            C2Rust_Unnamed_0::UNI_XMEFFECTX2.0 as ::core::ffi::c_int
                                as UWORD,
                            (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UWORD,
                        );
                    }
                    _ => {}
                }
            }
            _ => {
                if eff as ::core::ffi::c_int <= 0xf as ::core::ffi::c_int {
                    if eff as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
                        if (dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                            >> 4 as ::core::ffi::c_int <= 9 as ::core::ffi::c_int
                            && dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                                <= 9 as ::core::ffi::c_int
                        {
                            dat = (((dat as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int) >> 4 as ::core::ffi::c_int)
                                * 10 as ::core::ffi::c_int
                                + (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                                as UBYTE;
                        }
                    }
                    UniPTEffect(eff, dat);
                }
            }
        }
        UniNewline();
        xmtrack = xmtrack.offset(1);
        t += 1;
    }
    return UniDup();
}
unsafe extern "C" fn LoadPatterns(mut dummypat: BOOL) -> BOOL {
    let mut t: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut numtrk: ::core::ffi::c_int = 0;
    if AllocTracks() == 0 {
        return 0 as BOOL;
    }
    if AllocPatterns() == 0 {
        return 0 as BOOL;
    }
    numtrk = 0 as ::core::ffi::c_int;
    t = 0 as ::core::ffi::c_int;
    while t < (*mh).numpat as ::core::ffi::c_int {
        let mut ph: XMPATHEADER = XMPATHEADER {
            size: 0,
            packing: 0,
            numrows: 0,
            packsize: 0,
        };
        ph.size = _mm_read_I_ULONG(modreader);
        if ph.size
            < (if (*mh).version as ::core::ffi::c_int == 0x102 as ::core::ffi::c_int {
                8 as ::core::ffi::c_int
            } else {
                9 as ::core::ffi::c_int
            }) as ULONG
        {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        ph.packing = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        if ph.packing != 0 {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        if (*mh).version as ::core::ffi::c_int == 0x102 as ::core::ffi::c_int {
            ph.numrows = ((*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as UWORD;
        } else {
            ph.numrows = _mm_read_I_UWORD(modreader);
        }
        ph.packsize = _mm_read_I_UWORD(modreader) as SWORD;
        ph.size = ph
            .size
            .wrapping_sub(
                (if (*mh).version as ::core::ffi::c_int == 0x102 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    9 as ::core::ffi::c_int
                }) as ULONG,
            );
        if ph.size != 0 {
            (*modreader)
                .Seek
                .expect(
                    "non-null function pointer",
                )(modreader, ph.size as ::core::ffi::c_long, 1 as ::core::ffi::c_int);
        }
        *of.pattrows.offset(t as isize) = ph.numrows;
        if ph.numrows != 0 {
            xmpat = MikMod_calloc(
                (ph.numrows as ::core::ffi::c_int * of.numchn as ::core::ffi::c_int)
                    as size_t,
                ::core::mem::size_of::<XMNOTE>(),
            ) as *mut XMNOTE;
            if xmpat.is_null() {
                return 0 as BOOL;
            }
            if ph.packsize != 0 {
                u = 0 as ::core::ffi::c_int;
                while u < ph.numrows as ::core::ffi::c_int {
                    v = 0 as ::core::ffi::c_int;
                    while v < of.numchn as ::core::ffi::c_int {
                        if ph.packsize == 0 {
                            break;
                        }
                        ph.packsize = (ph.packsize as ::core::ffi::c_int
                            - XM_ReadNote(
                                xmpat
                                    .offset((v * ph.numrows as ::core::ffi::c_int + u) as isize),
                            )) as SWORD;
                        if (ph.packsize as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                        {
                            MikMod_free(xmpat as *mut ::core::ffi::c_void);
                            xmpat = ::core::ptr::null_mut::<XMNOTE>();
                            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0
                                as ::core::ffi::c_int;
                            return 0 as BOOL;
                        }
                        v += 1;
                    }
                    u += 1;
                }
            }
            if ph.packsize != 0 {
                (*modreader)
                    .Seek
                    .expect(
                        "non-null function pointer",
                    )(
                    modreader,
                    ph.packsize as ::core::ffi::c_long,
                    1 as ::core::ffi::c_int,
                );
            }
            if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
                MikMod_free(xmpat as *mut ::core::ffi::c_void);
                xmpat = ::core::ptr::null_mut::<XMNOTE>();
                MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0
                    as ::core::ffi::c_int;
                return 0 as BOOL;
            }
            v = 0 as ::core::ffi::c_int;
            while v < of.numchn as ::core::ffi::c_int {
                let c2rust_fresh2 = numtrk;
                numtrk += 1;
                *of.tracks.offset(c2rust_fresh2 as isize) = XM_Convert(
                    xmpat.offset((v * ph.numrows as ::core::ffi::c_int) as isize),
                    ph.numrows,
                );
                v += 1;
            }
            MikMod_free(xmpat as *mut ::core::ffi::c_void);
            xmpat = ::core::ptr::null_mut::<XMNOTE>();
        } else {
            v = 0 as ::core::ffi::c_int;
            while v < of.numchn as ::core::ffi::c_int {
                let c2rust_fresh3 = numtrk;
                numtrk += 1;
                *of.tracks.offset(c2rust_fresh3 as isize) = XM_Convert(
                    ::core::ptr::null_mut::<XMNOTE>(),
                    ph.numrows,
                );
                v += 1;
            }
        }
        t += 1;
    }
    if dummypat != 0 {
        *of.pattrows.offset(t as isize) = 64 as UWORD;
        xmpat = MikMod_calloc(
            (64 as ::core::ffi::c_int * of.numchn as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<XMNOTE>(),
        ) as *mut XMNOTE;
        if xmpat.is_null() {
            return 0 as BOOL;
        }
        v = 0 as ::core::ffi::c_int;
        while v < of.numchn as ::core::ffi::c_int {
            let c2rust_fresh4 = numtrk;
            numtrk += 1;
            *of.tracks.offset(c2rust_fresh4 as isize) = XM_Convert(
                xmpat.offset((v * 64 as ::core::ffi::c_int) as isize),
                64 as UWORD,
            );
            v += 1;
        }
        MikMod_free(xmpat as *mut ::core::ffi::c_void);
        xmpat = ::core::ptr::null_mut::<XMNOTE>();
    }
    return 1 as BOOL;
}
unsafe extern "C" fn FixEnvelope(mut cur: *mut ENVPT, mut pts: ::core::ffi::c_int) {
    let mut u: ::core::ffi::c_int = 0;
    let mut old: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_int = 0;
    let mut prev: *mut ENVPT = ::core::ptr::null_mut::<ENVPT>();
    let c2rust_fresh1 = cur;
    cur = cur.offset(1);
    prev = c2rust_fresh1;
    old = (*prev).pos as ::core::ffi::c_int;
    u = 1 as ::core::ffi::c_int;
    while u < pts {
        if ((*cur).pos as ::core::ffi::c_int) < (*prev).pos as ::core::ffi::c_int {
            if ((*cur).pos as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
                if (*cur).pos as ::core::ffi::c_int > old {
                    tmp = (*cur).pos as ::core::ffi::c_int
                        + ((*prev).pos as ::core::ffi::c_int - old);
                } else {
                    tmp = (*cur).pos as ::core::ffi::c_int
                        | (*prev).pos as ::core::ffi::c_int + 0x100 as ::core::ffi::c_int
                            & 0xff00 as ::core::ffi::c_int;
                }
                old = (*cur).pos as ::core::ffi::c_int;
                (*cur).pos = tmp as SWORD;
            } else {
                old = (*cur).pos as ::core::ffi::c_int;
            }
        } else {
            old = (*cur).pos as ::core::ffi::c_int;
        }
        u += 1;
        prev = prev.offset(1);
        cur = cur.offset(1);
    }
}
unsafe extern "C" fn LoadInstruments() -> BOOL {
    let mut filend: ::core::ffi::c_long = 0;
    let mut ck: ::core::ffi::c_long = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut d: *mut INSTRUMENT = ::core::ptr::null_mut::<INSTRUMENT>();
    let mut next: ULONG = 0 as ULONG;
    let mut wavcnt: UWORD = 0 as UWORD;
    ck = (*modreader).Tell.expect("non-null function pointer")(modreader);
    (*modreader)
        .Seek
        .expect(
            "non-null function pointer",
        )(modreader, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int);
    filend = (*modreader).Tell.expect("non-null function pointer")(modreader);
    (*modreader)
        .Seek
        .expect("non-null function pointer")(modreader, ck, 0 as ::core::ffi::c_int);
    if AllocInstruments() == 0 {
        return 0 as BOOL;
    }
    d = of.instruments as *mut INSTRUMENT;
    t = 0 as ::core::ffi::c_int;
    while t < of.numins as ::core::ffi::c_int {
        let mut ih: XMINSTHEADER = XMINSTHEADER {
            size: 0,
            name: [0; 22],
            r#type: 0,
            numsmp: 0,
            ssize: 0,
        };
        let mut headend: ::core::ffi::c_long = 0;
        memset(
            &raw mut (*d).samplenumber as *mut UWORD as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            (120 as size_t).wrapping_mul(::core::mem::size_of::<UWORD>()),
        );
        headend = (*modreader).Tell.expect("non-null function pointer")(modreader);
        ih.size = _mm_read_I_ULONG(modreader);
        headend = (headend as ULONG).wrapping_add(ih.size) as ::core::ffi::c_long;
        ck = (*modreader).Tell.expect("non-null function pointer")(modreader);
        if headend < 0 as ::core::ffi::c_long || filend < headend || headend < ck {
            break;
        }
        _mm_read_string(
            &raw mut ih.name as *mut CHAR,
            22 as ::core::ffi::c_int,
            modreader,
        );
        ih.r#type = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        ih.numsmp = _mm_read_I_UWORD(modreader);
        (*d).insname = DupStr(&raw mut ih.name as *mut CHAR, 22 as UWORD, 1 as BOOL);
        if ih.size as SWORD as ::core::ffi::c_int > 29 as ::core::ffi::c_int {
            ih.ssize = _mm_read_I_ULONG(modreader);
            if ih.numsmp as SWORD as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && ih.numsmp as ::core::ffi::c_int
                    <= 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
            {
                let mut pth: XMPATCHHEADER = XMPATCHHEADER {
                    what: [0; 96],
                    volenv: [0; 24],
                    panenv: [0; 24],
                    volpts: 0,
                    panpts: 0,
                    volsus: 0,
                    volbeg: 0,
                    volend: 0,
                    pansus: 0,
                    panbeg: 0,
                    panend: 0,
                    volflg: 0,
                    panflg: 0,
                    vibflg: 0,
                    vibsweep: 0,
                    vibdepth: 0,
                    vibrate: 0,
                    volfade: 0,
                };
                let mut p: ::core::ffi::c_int = 0;
                (*modreader)
                    .Read
                    .expect(
                        "non-null function pointer",
                    )(
                    modreader,
                    &raw mut pth.what as *mut UBYTE as *mut ::core::ffi::c_void,
                    (8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as size_t,
                );
                _mm_read_I_UWORDS(
                    &raw mut pth.volenv as *mut UWORD,
                    12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
                    modreader,
                );
                _mm_read_I_UWORDS(
                    &raw mut pth.panenv as *mut UWORD,
                    12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
                    modreader,
                );
                pth.volpts = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.panpts = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.volsus = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.volbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.volend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.pansus = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.panbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.panend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.volflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.panflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.vibflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.vibsweep = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.vibdepth = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.vibrate = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                pth.volfade = _mm_read_I_UWORD(modreader);
                if headend
                    >= (*modreader).Tell.expect("non-null function pointer")(modreader)
                {
                    u = (headend
                        - (*modreader)
                            .Tell
                            .expect("non-null function pointer")(modreader))
                        as ::core::ffi::c_int;
                    while u != 0 {
                        (*modreader).Get.expect("non-null function pointer")(modreader);
                        u -= 1;
                    }
                }
                if pth.volpts as ::core::ffi::c_int
                    > 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int
                {
                    pth.volpts = (12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int) as UBYTE;
                }
                if pth.panpts as ::core::ffi::c_int
                    > 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int
                {
                    pth.panpts = (12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int) as UBYTE;
                }
                if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0
                    || pth.volpts as ::core::ffi::c_int
                        > 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int
                    || pth.panpts as ::core::ffi::c_int
                        > 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int
                {
                    MikMod_free(nextwav as *mut ::core::ffi::c_void);
                    nextwav = ::core::ptr::null_mut::<ULONG>();
                    MikMod_free(wh as *mut ::core::ffi::c_void);
                    wh = ::core::ptr::null_mut::<XMWAVHEADER>();
                    MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0
                        as ::core::ffi::c_int;
                    return 0 as BOOL;
                }
                u = 0 as ::core::ffi::c_int;
                while u < 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int {
                    (*d).samplenumber[u as usize] = (pth.what[u as usize]
                        as ::core::ffi::c_int + of.numsmp as ::core::ffi::c_int)
                        as UWORD;
                    u += 1;
                }
                (*d).volfade = pth.volfade;
                u = 0 as ::core::ffi::c_int;
                while u
                    < 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    (*d).volenv[u as usize].pos = pth
                        .volenv[(u << 1 as ::core::ffi::c_int) as usize] as SWORD;
                    (*d).volenv[u as usize].val = pth
                        .volenv[((u << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int) as usize] as SWORD;
                    u += 1;
                }
                if pth.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if pth.volflg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                if pth.volflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 4 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).volsusend = pth.volsus;
                (*d).volsusbeg = (*d).volsusend;
                (*d).volbeg = pth.volbeg;
                (*d).volend = pth.volend;
                (*d).volpts = pth.volpts;
                p = 0 as ::core::ffi::c_int;
                while p
                    < 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int
                {
                    (*d).volenv[p as usize].val = (((*d).volenv[p as usize].val
                        as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SWORD;
                    p += 1;
                }
                if (*d).volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                    && ((*d).volpts as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        & !(1 as ::core::ffi::c_int)) as UBYTE;
                }
                u = 0 as ::core::ffi::c_int;
                while u
                    < 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    (*d).panenv[u as usize].pos = pth
                        .panenv[(u << 1 as ::core::ffi::c_int) as usize] as SWORD;
                    (*d).panenv[u as usize].val = pth
                        .panenv[((u << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int) as usize] as SWORD;
                    u += 1;
                }
                if pth.panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if pth.panflg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                if pth.panflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        | 4 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).pansusend = pth.pansus;
                (*d).pansusbeg = (*d).pansusend;
                (*d).panbeg = pth.panbeg;
                (*d).panend = pth.panend;
                (*d).panpts = pth.panpts;
                p = 0 as ::core::ffi::c_int;
                while p
                    < 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int
                {
                    (*d).panenv[p as usize].val = (((*d).panenv[p as usize].val
                        as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SWORD;
                    p += 1;
                }
                if (*d).panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                    && ((*d).panpts as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        & !(1 as ::core::ffi::c_int)) as UBYTE;
                }
                if (*d).volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    FixEnvelope(
                        &raw mut (*d).volenv as *mut ENVPT,
                        (*d).volpts as ::core::ffi::c_int,
                    );
                }
                if (*d).panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    FixEnvelope(
                        &raw mut (*d).panenv as *mut ENVPT,
                        (*d).panpts as ::core::ffi::c_int,
                    );
                }
                if (*mh).version as ::core::ffi::c_int > 0x103 as ::core::ffi::c_int {
                    next = 0 as ULONG;
                }
                u = 0 as ::core::ffi::c_int;
                while u < ih.numsmp as ::core::ffi::c_int {
                    if (*modreader).Tell.expect("non-null function pointer")(modreader)
                        + 40 as ::core::ffi::c_long > filend
                    {
                        MikMod_free(nextwav as *mut ::core::ffi::c_void);
                        MikMod_free(wh as *mut ::core::ffi::c_void);
                        nextwav = ::core::ptr::null_mut::<ULONG>();
                        wh = ::core::ptr::null_mut::<XMWAVHEADER>();
                        MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0
                            as ::core::ffi::c_int;
                        return 0 as BOOL;
                    }
                    if of.numsmp as ::core::ffi::c_int + u
                        == wavcnt as ::core::ffi::c_int
                    {
                        wavcnt = (wavcnt as ::core::ffi::c_int
                            + 64 as ::core::ffi::c_int) as UWORD;
                        nextwav = MikMod_realloc(
                            nextwav as *mut ::core::ffi::c_void,
                            (wavcnt as size_t)
                                .wrapping_mul(::core::mem::size_of::<ULONG>()),
                        ) as *mut ULONG;
                        if nextwav.is_null() {
                            MikMod_free(wh as *mut ::core::ffi::c_void);
                            wh = ::core::ptr::null_mut::<XMWAVHEADER>();
                            MikMod_errno = C2Rust_Unnamed::MMERR_OUT_OF_MEMORY.0
                                as ::core::ffi::c_int;
                            return 0 as BOOL;
                        }
                        wh = MikMod_realloc(
                            wh as *mut ::core::ffi::c_void,
                            (wavcnt as size_t)
                                .wrapping_mul(::core::mem::size_of::<XMWAVHEADER>()),
                        ) as *mut XMWAVHEADER;
                        if wh.is_null() {
                            MikMod_free(nextwav as *mut ::core::ffi::c_void);
                            nextwav = ::core::ptr::null_mut::<ULONG>();
                            MikMod_errno = C2Rust_Unnamed::MMERR_OUT_OF_MEMORY.0
                                as ::core::ffi::c_int;
                            return 0 as BOOL;
                        }
                        s = wh
                            .offset(
                                (wavcnt as ::core::ffi::c_int - 64 as ::core::ffi::c_int)
                                    as isize,
                            );
                    }
                    (*s).length = _mm_read_I_ULONG(modreader);
                    (*s).loopstart = _mm_read_I_ULONG(modreader);
                    (*s).looplength = _mm_read_I_ULONG(modreader);
                    (*s).volume = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    (*s).finetune = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as SBYTE;
                    (*s).r#type = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    (*s).panning = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    (*s).relnote = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as SBYTE;
                    (*s).vibtype = pth.vibflg;
                    (*s).vibsweep = pth.vibsweep;
                    (*s).vibdepth = (pth.vibdepth as ::core::ffi::c_int
                        * 4 as ::core::ffi::c_int) as UBYTE;
                    (*s).vibrate = pth.vibrate;
                    (*s).reserved = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    _mm_read_string(
                        &raw mut (*s).samplename as *mut CHAR,
                        22 as ::core::ffi::c_int,
                        modreader,
                    );
                    *nextwav.offset((of.numsmp as ::core::ffi::c_int + u) as isize) = next;
                    next = next.wrapping_add((*s).length);
                    u += 1;
                    s = s.offset(1);
                }
                if (*mh).version as ::core::ffi::c_int > 0x103 as ::core::ffi::c_int {
                    u = 0 as ::core::ffi::c_int;
                    while u < ih.numsmp as ::core::ffi::c_int {
                        let c2rust_fresh0 = of.numsmp;
                        of.numsmp = of.numsmp.wrapping_add(1);
                        let c2rust_lvalue_ptr = &raw mut *nextwav
                            .offset(c2rust_fresh0 as isize);
                        *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr)
                            .wrapping_add(
                                (*modreader)
                                    .Tell
                                    .expect("non-null function pointer")(modreader) as ULONG,
                            );
                        u += 1;
                    }
                    (*modreader)
                        .Seek
                        .expect(
                            "non-null function pointer",
                        )(
                        modreader,
                        next as ::core::ffi::c_long,
                        1 as ::core::ffi::c_int,
                    );
                } else {
                    of.numsmp = (of.numsmp as ::core::ffi::c_int
                        + ih.numsmp as ::core::ffi::c_int) as UWORD;
                }
            } else {
                ck = (*modreader).Tell.expect("non-null function pointer")(modreader);
                if headend < 0 as ::core::ffi::c_long || filend < headend || headend < ck
                {
                    break;
                }
                u = (headend
                    - (*modreader).Tell.expect("non-null function pointer")(modreader))
                    as ::core::ffi::c_int;
                while u != 0 {
                    (*modreader).Get.expect("non-null function pointer")(modreader);
                    u -= 1;
                }
                if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0
                    && (((*mh).version as ::core::ffi::c_int)
                        < 0x104 as ::core::ffi::c_int
                        || t < of.numins as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                {
                    MikMod_free(nextwav as *mut ::core::ffi::c_void);
                    MikMod_free(wh as *mut ::core::ffi::c_void);
                    nextwav = ::core::ptr::null_mut::<ULONG>();
                    wh = ::core::ptr::null_mut::<XMWAVHEADER>();
                    MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0
                        as ::core::ffi::c_int;
                    return 0 as BOOL;
                }
            }
        }
        t += 1;
        d = d.offset(1);
    }
    if of.numsmp == 0 {
        MikMod_free(nextwav as *mut ::core::ffi::c_void);
        nextwav = ::core::ptr::null_mut::<ULONG>();
        MikMod_free(wh as *mut ::core::ffi::c_void);
        wh = ::core::ptr::null_mut::<XMWAVHEADER>();
        MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn XM_Load(mut curious: BOOL) -> BOOL {
    let mut d: *mut INSTRUMENT = ::core::ptr::null_mut::<INSTRUMENT>();
    let mut q: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    let mut t: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut dummypat: BOOL = 0 as BOOL;
    let mut tracker: [::core::ffi::c_char; 21] = [0; 21];
    let mut modtype: [::core::ffi::c_char; 60] = [0; 60];
    _mm_read_string(&raw mut (*mh).id as *mut CHAR, 17 as ::core::ffi::c_int, modreader);
    _mm_read_string(
        &raw mut (*mh).songname as *mut CHAR,
        21 as ::core::ffi::c_int,
        modreader,
    );
    _mm_read_string(
        &raw mut (*mh).trackername as *mut CHAR,
        20 as ::core::ffi::c_int,
        modreader,
    );
    (*mh).version = _mm_read_I_UWORD(modreader);
    if !(((*mh).version as ::core::ffi::c_int) < 0x102 as ::core::ffi::c_int
        || (*mh).version as ::core::ffi::c_int > 0x104 as ::core::ffi::c_int)
    {
        (*mh).headersize = _mm_read_I_ULONG(modreader);
        (*mh).songlength = _mm_read_I_UWORD(modreader);
        (*mh).restart = _mm_read_I_UWORD(modreader);
        (*mh).numchn = _mm_read_I_UWORD(modreader);
        (*mh).numpat = _mm_read_I_UWORD(modreader);
        (*mh).numins = _mm_read_I_UWORD(modreader);
        (*mh).flags = _mm_read_I_UWORD(modreader);
        (*mh).tempo = _mm_read_I_UWORD(modreader);
        (*mh).bpm = _mm_read_I_UWORD(modreader);
        if (*mh).numchn as ::core::ffi::c_int <= 64 as ::core::ffi::c_int {
            if !((*mh).tempo as ::core::ffi::c_int > 32 as ::core::ffi::c_int
                || ((*mh).bpm as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                || (*mh).bpm as ::core::ffi::c_int > 255 as ::core::ffi::c_int)
            {
                if !((*mh).songlength as ::core::ffi::c_int > 256 as ::core::ffi::c_int
                    || (*mh).headersize < 20 as ULONG
                    || (*mh).headersize
                        > (20 as ::core::ffi::c_int + 256 as ::core::ffi::c_int)
                            as ULONG)
                {
                    if !((*mh).numpat as ::core::ffi::c_int > 256 as ::core::ffi::c_int
                        || (*mh).numins as ::core::ffi::c_int > 255 as ::core::ffi::c_int
                        || (*mh).restart as ::core::ffi::c_int
                            > 255 as ::core::ffi::c_int)
                    {
                        (*modreader)
                            .Read
                            .expect(
                                "non-null function pointer",
                            )(
                            modreader,
                            &raw mut (*mh).orders as *mut UBYTE
                                as *mut ::core::ffi::c_void,
                            (*mh).songlength as size_t,
                        );
                        if (*modreader)
                            .Seek
                            .expect(
                                "non-null function pointer",
                            )(
                            modreader,
                            (*mh).headersize.wrapping_add(60 as ULONG)
                                as ::core::ffi::c_long,
                            0 as ::core::ffi::c_int,
                        ) != 0
                            || (*modreader)
                                .Eof
                                .expect("non-null function pointer")(modreader) != 0
                        {
                            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0
                                as ::core::ffi::c_int;
                            return 0 as BOOL;
                        } else {
                            of.initspeed = (*mh).tempo as UBYTE;
                            of.inittempo = (*mh).bpm;
                            strncpy(
                                &raw mut tracker as *mut ::core::ffi::c_char,
                                &raw mut (*mh).trackername as *mut CHAR,
                                20 as size_t,
                            );
                            tracker[20usize] = 0 as ::core::ffi::c_char;
                            t = 20 as ::core::ffi::c_int;
                            while t >= 0 as ::core::ffi::c_int
                                && tracker[t as usize] as ::core::ffi::c_int
                                    <= ' ' as ::core::ffi::c_int
                            {
                                tracker[t as usize] = 0 as ::core::ffi::c_char;
                                t -= 1;
                            }
                            if tracker[0usize] == 0 {
                                strcpy(
                                    &raw mut tracker as *mut ::core::ffi::c_char,
                                    b"Unknown tracker\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                            sprintf(
                                &raw mut modtype as *mut ::core::ffi::c_char,
                                b"%s (XM format %d.%02d)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut tracker as *mut ::core::ffi::c_char,
                                (*mh).version as ::core::ffi::c_int
                                    >> 8 as ::core::ffi::c_int,
                                (*mh).version as ::core::ffi::c_int
                                    & 0xff as ::core::ffi::c_int,
                            );
                            of.modtype = MikMod_strdup(
                                &raw mut modtype as *mut ::core::ffi::c_char,
                            );
                            of.numchn = (*mh).numchn as UBYTE;
                            of.numpat = (*mh).numpat;
                            of.numtrk = (of.numpat as ::core::ffi::c_int
                                * of.numchn as ::core::ffi::c_int) as UWORD;
                            of.songname = DupStr(
                                &raw mut (*mh).songname as *mut CHAR,
                                20 as UWORD,
                                1 as BOOL,
                            );
                            of.numpos = (*mh).songlength;
                            of.reppos = (if ((*mh).restart as ::core::ffi::c_int)
                                < (*mh).songlength as ::core::ffi::c_int
                            {
                                (*mh).restart as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as UWORD;
                            of.numins = (*mh).numins;
                            of.flags = (of.flags as ::core::ffi::c_int
                                | (0x1 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int
                                    | 0x80 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int
                                    | 0x400 as ::core::ffi::c_int)) as UWORD;
                            if (*mh).flags as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int != 0
                            {
                                of.flags = (of.flags as ::core::ffi::c_int
                                    | 0x2 as ::core::ffi::c_int) as UWORD;
                            }
                            of.bpmlimit = 32 as UWORD;
                            memset(
                                &raw mut of.chanvol as *mut UBYTE
                                    as *mut ::core::ffi::c_void,
                                64 as ::core::ffi::c_int,
                                of.numchn as size_t,
                            );
                            if AllocPositions(
                                of.numpos as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                            ) == 0
                            {
                                return 0 as BOOL;
                            }
                            t = 0 as ::core::ffi::c_int;
                            while t < of.numpos as ::core::ffi::c_int {
                                *of.positions.offset(t as isize) = (*mh).orders[t as usize]
                                    as UWORD;
                                t += 1;
                            }
                            t = 0 as ::core::ffi::c_int;
                            while t < of.numpos as ::core::ffi::c_int {
                                if *of.positions.offset(t as isize) as ::core::ffi::c_int
                                    >= of.numpat as ::core::ffi::c_int
                                {
                                    *of.positions.offset(t as isize) = of.numpat;
                                    dummypat = 1 as ::core::ffi::c_int as BOOL;
                                }
                                t += 1;
                            }
                            if dummypat != 0 {
                                of.numpat = of.numpat.wrapping_add(1);
                                of.numtrk = (of.numtrk as ::core::ffi::c_int
                                    + of.numchn as ::core::ffi::c_int) as UWORD;
                            }
                            if ((*mh).version as ::core::ffi::c_int)
                                < 0x104 as ::core::ffi::c_int
                            {
                                if LoadInstruments() == 0 {
                                    return 0 as BOOL;
                                }
                                if LoadPatterns(dummypat) == 0 {
                                    return 0 as BOOL;
                                }
                                t = 0 as ::core::ffi::c_int;
                                while t < of.numsmp as ::core::ffi::c_int {
                                    *nextwav.offset(t as isize) = (*nextwav.offset(t as isize))
                                        .wrapping_add(
                                            (*modreader)
                                                .Tell
                                                .expect("non-null function pointer")(modreader) as ULONG,
                                        );
                                    t += 1;
                                }
                            } else {
                                if LoadPatterns(dummypat) == 0 {
                                    return 0 as BOOL;
                                }
                                if LoadInstruments() == 0 {
                                    return 0 as BOOL;
                                }
                            }
                            if AllocSamples() == 0 {
                                MikMod_free(nextwav as *mut ::core::ffi::c_void);
                                MikMod_free(wh as *mut ::core::ffi::c_void);
                                nextwav = ::core::ptr::null_mut::<ULONG>();
                                wh = ::core::ptr::null_mut::<XMWAVHEADER>();
                                return 0 as BOOL;
                            }
                            q = of.samples as *mut SAMPLE;
                            s = wh;
                            u = 0 as ::core::ffi::c_int;
                            while u < of.numsmp as ::core::ffi::c_int {
                                (*q).samplename = DupStr(
                                    &raw mut (*s).samplename as *mut CHAR,
                                    22 as UWORD,
                                    1 as BOOL,
                                );
                                (*q).length = (*s).length;
                                (*q).loopstart = (*s).loopstart;
                                (*q).loopend = (*s).loopstart.wrapping_add((*s).looplength);
                                (*q).volume = (*s).volume;
                                (*q).speed = ((*s).finetune as ::core::ffi::c_int
                                    + 128 as ::core::ffi::c_int) as ULONG;
                                (*q).panning = (*s).panning as SWORD;
                                (*q).seekpos = *nextwav.offset(u as isize);
                                (*q).vibtype = (*s).vibtype;
                                (*q).vibsweep = (*s).vibsweep;
                                (*q).vibdepth = (*s).vibdepth;
                                (*q).vibrate = (*s).vibrate;
                                if (*s).r#type as ::core::ffi::c_int
                                    & 0x10 as ::core::ffi::c_int != 0
                                {
                                    (*q).length >>= 1 as ::core::ffi::c_int;
                                    (*q).loopstart >>= 1 as ::core::ffi::c_int;
                                    (*q).loopend >>= 1 as ::core::ffi::c_int;
                                }
                                (*q).flags = ((*q).flags as ::core::ffi::c_int
                                    | (0x1000 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int
                                        | 0x4 as ::core::ffi::c_int)) as UWORD;
                                if (*s).r#type as ::core::ffi::c_int
                                    & 0x3 as ::core::ffi::c_int != 0
                                {
                                    (*q).flags = ((*q).flags as ::core::ffi::c_int
                                        | 0x100 as ::core::ffi::c_int) as UWORD;
                                }
                                if (*s).r#type as ::core::ffi::c_int
                                    & 0x2 as ::core::ffi::c_int != 0
                                {
                                    (*q).flags = ((*q).flags as ::core::ffi::c_int
                                        | 0x200 as ::core::ffi::c_int) as UWORD;
                                }
                                if (*s).r#type as ::core::ffi::c_int
                                    & 0x10 as ::core::ffi::c_int != 0
                                {
                                    (*q).flags = ((*q).flags as ::core::ffi::c_int
                                        | 0x1 as ::core::ffi::c_int) as UWORD;
                                }
                                u += 1;
                                q = q.offset(1);
                                s = s.offset(1);
                            }
                            d = of.instruments as *mut INSTRUMENT;
                            s = wh;
                            u = 0 as ::core::ffi::c_int;
                            while u < of.numins as ::core::ffi::c_int {
                                t = 0 as ::core::ffi::c_int;
                                while t < 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                                {
                                    if (*d).samplenumber[t as usize] as ::core::ffi::c_int
                                        >= of.numsmp as ::core::ffi::c_int
                                    {
                                        (*d).samplenote[t as usize] = 255 as UBYTE;
                                    } else {
                                        let mut note: ::core::ffi::c_int = t
                                            + (*s.offset((*d).samplenumber[t as usize] as isize))
                                                .relnote as ::core::ffi::c_int;
                                        (*d).samplenote[t as usize] = (if note
                                            < 0 as ::core::ffi::c_int
                                        {
                                            0 as ::core::ffi::c_int
                                        } else {
                                            note
                                        }) as UBYTE;
                                    }
                                    t += 1;
                                }
                                u += 1;
                                d = d.offset(1);
                            }
                            MikMod_free(wh as *mut ::core::ffi::c_void);
                            MikMod_free(nextwav as *mut ::core::ffi::c_void);
                            wh = ::core::ptr::null_mut::<XMWAVHEADER>();
                            nextwav = ::core::ptr::null_mut::<ULONG>();
                            return 1 as BOOL;
                        }
                    }
                }
            }
        }
    }
    MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
    return 0 as BOOL;
}
unsafe extern "C" fn XM_LoadTitle() -> *mut CHAR {
    let mut str: [CHAR; 21] = [0; 21];
    (*modreader)
        .Seek
        .expect(
            "non-null function pointer",
        )(modreader, 17 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    if (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(modreader, &raw mut str as *mut CHAR as *mut ::core::ffi::c_void, 21 as size_t)
        == 0
    {
        return ::core::ptr::null_mut::<CHAR>();
    }
    return DupStr(&raw mut str as *mut CHAR, 21 as UWORD, 1 as BOOL);
}
#[no_mangle]
pub static mut load_xm: MLOADER = MLOADER {
    next: ::core::ptr::null_mut::<MLOADER>(),
    r#type: b"XM\0".as_ptr() as *const CHAR,
    version: b"XM (FastTracker 2)\0".as_ptr() as *const CHAR,
    Init: Some(XM_Init as unsafe extern "C" fn() -> BOOL),
    Test: Some(XM_Test as unsafe extern "C" fn() -> BOOL),
    Load: Some(XM_Load as unsafe extern "C" fn(BOOL) -> BOOL),
    Cleanup: Some(XM_Cleanup as unsafe extern "C" fn() -> ()),
    LoadTitle: Some(XM_LoadTitle as unsafe extern "C" fn() -> *mut CHAR),
};
