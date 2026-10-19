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

  Impulse tracker (IT) module loader

==============================================================================*/

// Rust translation of src/libs/mikmod/load_it.c.
extern "C" {
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
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    fn _mm_read_I_UWORDS(_: *mut UWORD, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn _mm_read_I_ULONGS(_: *mut ULONG, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn UniReset();
    fn UniWriteByte(_: UBYTE);
    fn UniNewline();
    fn UniDup() -> *mut UBYTE;
    fn UniEffect(_: UWORD, _: UWORD);
    fn UniPTEffect(_: UBYTE, _: UBYTE);
    fn UniVolEffect(_: UWORD, _: UBYTE);
    static mut modreader: *mut MREADER;
    static mut of: MODULE;
    static mut remap: [SBYTE; 64];
    static mut poslookup: *mut UBYTE;
    static mut poslookupcnt: UWORD;
    static mut origpositions: *mut UWORD;
    static mut filters: BOOL;
    static mut activemacro: UBYTE;
    static mut filtermacros: [UBYTE; 16];
    static mut filtersettings: [FILTER; 256];
    static mut noteindex: *mut ::core::ffi::c_int;
    fn ReadComment(_: UWORD) -> BOOL;
    fn AllocPositions(_: ::core::ffi::c_int) -> BOOL;
    fn AllocPatterns() -> BOOL;
    fn AllocTracks() -> BOOL;
    fn AllocInstruments() -> BOOL;
    fn AllocSamples() -> BOOL;
    fn DupStr(_: *const CHAR, _: UWORD, _: BOOL) -> *mut CHAR;
    fn AllocLinear() -> *mut ::core::ffi::c_int;
    fn FreeLinear();
    fn speed_to_finetune(_: ULONG, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn S3MIT_ProcessCmd(_: UBYTE, _: UBYTE, _: ::core::ffi::c_uint);
    fn S3MIT_CreateOrders(_: BOOL);
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
pub struct ITNOTE {
    pub note: UBYTE,
    pub ins: UBYTE,
    pub volpan: UBYTE,
    pub cmd: UBYTE,
    pub inf: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ITHEADER {
    pub songname: [CHAR; 26],
    pub blank01: [UBYTE; 2],
    pub ordnum: UWORD,
    pub insnum: UWORD,
    pub smpnum: UWORD,
    pub patnum: UWORD,
    pub cwt: UWORD,
    pub cmwt: UWORD,
    pub flags: UWORD,
    pub special: UWORD,
    pub globvol: UBYTE,
    pub mixvol: UBYTE,
    pub initspeed: UBYTE,
    pub inittempo: UBYTE,
    pub pansep: UBYTE,
    pub zerobyte: UBYTE,
    pub msglength: UWORD,
    pub msgoffset: ULONG,
    pub blank02: [UBYTE; 4],
    pub pantable: [UBYTE; 64],
    pub voltable: [UBYTE; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ITINSTHEADER {
    pub size: ULONG,
    pub filename: [CHAR; 12],
    pub zerobyte: UBYTE,
    pub volflg: UBYTE,
    pub volpts: UBYTE,
    pub volbeg: UBYTE,
    pub volend: UBYTE,
    pub volsusbeg: UBYTE,
    pub volsusend: UBYTE,
    pub panflg: UBYTE,
    pub panpts: UBYTE,
    pub panbeg: UBYTE,
    pub panend: UBYTE,
    pub pansusbeg: UBYTE,
    pub pansusend: UBYTE,
    pub pitflg: UBYTE,
    pub pitpts: UBYTE,
    pub pitbeg: UBYTE,
    pub pitend: UBYTE,
    pub pitsusbeg: UBYTE,
    pub pitsusend: UBYTE,
    pub blank: UWORD,
    pub globvol: UBYTE,
    pub chanpan: UBYTE,
    pub fadeout: UWORD,
    pub dnc: UBYTE,
    pub dca: UBYTE,
    pub dct: UBYTE,
    pub nna: UBYTE,
    pub trkvers: UWORD,
    pub ppsep: UBYTE,
    pub ppcenter: UBYTE,
    pub rvolvar: UBYTE,
    pub rpanvar: UBYTE,
    pub numsmp: UWORD,
    pub name: [CHAR; 26],
    pub blank01: [UBYTE; 6],
    pub samptable: [UWORD; 120],
    pub volenv: [UBYTE; 200],
    pub oldvoltick: [UBYTE; 25],
    pub volnode: [UBYTE; 25],
    pub voltick: [UWORD; 25],
    pub pannode: [SBYTE; 25],
    pub pantick: [UWORD; 25],
    pub pitnode: [SBYTE; 25],
    pub pittick: [UWORD; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ITSAMPLE {
    pub filename: [CHAR; 12],
    pub zerobyte: UBYTE,
    pub globvol: UBYTE,
    pub flag: UBYTE,
    pub volume: UBYTE,
    pub panning: UBYTE,
    pub sampname: [CHAR; 28],
    pub convert: UWORD,
    pub length: ULONG,
    pub loopbeg: ULONG,
    pub loopend: ULONG,
    pub c5spd: ULONG,
    pub susbegin: ULONG,
    pub susend: ULONG,
    pub sampoffset: ULONG,
    pub vibspeed: UBYTE,
    pub vibdepth: UBYTE,
    pub vibrate: UBYTE,
    pub vibwave: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER {
    pub filter: UBYTE,
    pub inf: UBYTE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const VOL_VOLUME: Self = Self(1);
    pub const VOL_PANNING: Self = Self(2);
    pub const VOL_VOLSLIDE: Self = Self(3);
    pub const VOL_PITCHSLIDEDN: Self = Self(4);
    pub const VOL_PITCHSLIDEUP: Self = Self(5);
    pub const VOL_PORTAMENTO: Self = Self(6);
    pub const VOL_VIBRATO: Self = Self(7);
}
static mut paraptr: *mut ULONG = ::core::ptr::null_mut::<ULONG>();
static mut mh: *mut ITHEADER = ::core::ptr::null_mut::<ITHEADER>();
static mut itpat: *mut ITNOTE = ::core::ptr::null_mut::<ITNOTE>();
static mut mask: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
static mut last: *mut ITNOTE = ::core::ptr::null_mut::<ITNOTE>();
static mut numtrk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut old_effect: ::core::ffi::c_uint = 0;
static mut IT_Version: [*const CHAR; 6] = [
    b"ImpulseTracker  .  \0".as_ptr() as *const CHAR,
    b"Compressed ImpulseTracker  .  \0".as_ptr() as *const CHAR,
    b"ImpulseTracker 2.14p3\0".as_ptr() as *const CHAR,
    b"Compressed ImpulseTracker 2.14p3\0".as_ptr() as *const CHAR,
    b"ImpulseTracker 2.14p4\0".as_ptr() as *const CHAR,
    b"Compressed ImpulseTracker 2.14p4\0".as_ptr() as *const CHAR,
];
static mut portatable: [UBYTE; 10] = [
    0 as UBYTE,
    1 as UBYTE,
    4 as UBYTE,
    8 as UBYTE,
    16 as UBYTE,
    32 as UBYTE,
    64 as UBYTE,
    96 as UBYTE,
    128 as UBYTE,
    255 as UBYTE,
];
unsafe extern "C" fn IT_Test() -> BOOL {
    let mut id: [UBYTE; 4] = [0; 4];
    if (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(modreader, &raw mut id as *mut UBYTE as *mut ::core::ffi::c_void, 4 as size_t)
        == 0
    {
        return 0 as BOOL;
    }
    if memcmp(
        &raw mut id as *mut UBYTE as *const ::core::ffi::c_void,
        b"IMPM\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return 1 as BOOL;
    }
    return 0 as BOOL;
}
unsafe extern "C" fn IT_Init() -> BOOL {
    mh = MikMod_malloc(::core::mem::size_of::<ITHEADER>()) as *mut ITHEADER;
    if mh.is_null() {
        return 0 as BOOL;
    }
    poslookup = MikMod_malloc(
        (256 as size_t).wrapping_mul(::core::mem::size_of::<UBYTE>()),
    ) as *mut UBYTE;
    if poslookup.is_null() {
        return 0 as BOOL;
    }
    itpat = MikMod_malloc(
        ((200 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<ITNOTE>()),
    ) as *mut ITNOTE;
    if itpat.is_null() {
        return 0 as BOOL;
    }
    mask = MikMod_malloc((64 as size_t).wrapping_mul(::core::mem::size_of::<UBYTE>()))
        as *mut UBYTE;
    if mask.is_null() {
        return 0 as BOOL;
    }
    last = MikMod_malloc((64 as size_t).wrapping_mul(::core::mem::size_of::<ITNOTE>()))
        as *mut ITNOTE;
    if last.is_null() {
        return 0 as BOOL;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn IT_Cleanup() {
    FreeLinear();
    MikMod_free(mh as *mut ::core::ffi::c_void);
    MikMod_free(poslookup as *mut ::core::ffi::c_void);
    MikMod_free(itpat as *mut ::core::ffi::c_void);
    MikMod_free(mask as *mut ::core::ffi::c_void);
    MikMod_free(last as *mut ::core::ffi::c_void);
    MikMod_free(paraptr as *mut ::core::ffi::c_void);
    MikMod_free(origpositions as *mut ::core::ffi::c_void);
    mh = ::core::ptr::null_mut::<ITHEADER>();
    poslookup = ::core::ptr::null_mut::<UBYTE>();
    itpat = ::core::ptr::null_mut::<ITNOTE>();
    mask = ::core::ptr::null_mut::<UBYTE>();
    last = ::core::ptr::null_mut::<ITNOTE>();
    paraptr = ::core::ptr::null_mut::<ULONG>();
    origpositions = ::core::ptr::null_mut::<UWORD>();
}
unsafe extern "C" fn IT_GetNumChannels(mut patrows: UWORD) -> BOOL {
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut flag: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    loop {
        if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        flag = (*modreader).Get.expect("non-null function pointer")(modreader) as UBYTE
            as ::core::ffi::c_int;
        if flag == 0 {
            row += 1;
        } else {
            ch = flag - 1 as ::core::ffi::c_int & 63 as ::core::ffi::c_int;
            remap[ch as usize] = 0 as SBYTE;
            if flag & 128 as ::core::ffi::c_int != 0 {
                *mask.offset(ch as isize) = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
            }
            if *mask.offset(ch as isize) as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                != 0
            {
                (*modreader).Get.expect("non-null function pointer")(modreader);
            }
            if *mask.offset(ch as isize) as ::core::ffi::c_int & 2 as ::core::ffi::c_int
                != 0
            {
                (*modreader).Get.expect("non-null function pointer")(modreader);
            }
            if *mask.offset(ch as isize) as ::core::ffi::c_int & 4 as ::core::ffi::c_int
                != 0
            {
                (*modreader).Get.expect("non-null function pointer")(modreader);
            }
            if *mask.offset(ch as isize) as ::core::ffi::c_int & 8 as ::core::ffi::c_int
                != 0
            {
                (*modreader).Get.expect("non-null function pointer")(modreader);
                (*modreader).Get.expect("non-null function pointer")(modreader);
            }
        }
        if row >= patrows as ::core::ffi::c_int {
            break;
        }
    }
    return 1 as BOOL;
}
unsafe extern "C" fn IT_ConvertTrack(
    mut tr: *mut ITNOTE,
    mut numrows: UWORD,
) -> *mut UBYTE {
    let mut t: ::core::ffi::c_int = 0;
    let mut note: UBYTE = 0;
    let mut ins: UBYTE = 0;
    let mut volpan: UBYTE = 0;
    UniReset();
    t = 0 as ::core::ffi::c_int;
    while t < numrows as ::core::ffi::c_int {
        note = (*tr.offset((t * of.numchn as ::core::ffi::c_int) as isize)).note;
        ins = (*tr.offset((t * of.numchn as ::core::ffi::c_int) as isize)).ins;
        volpan = (*tr.offset((t * of.numchn as ::core::ffi::c_int) as isize)).volpan;
        if note as ::core::ffi::c_int != 255 as ::core::ffi::c_int {
            if note as ::core::ffi::c_int == 253 as ::core::ffi::c_int {
                UniWriteByte(
                    C2Rust_Unnamed_0::UNI_KEYOFF.0 as ::core::ffi::c_int as UBYTE,
                );
            } else if note as ::core::ffi::c_int == 254 as ::core::ffi::c_int {
                UniPTEffect(0xc as UBYTE, -1 as ::core::ffi::c_int as UBYTE);
                volpan = 255 as UBYTE;
            } else {
                UniEffect(
                    C2Rust_Unnamed_0::UNI_NOTE.0 as ::core::ffi::c_int as UWORD,
                    note as UWORD,
                );
            }
        }
        if ins as ::core::ffi::c_int != 0
            && (ins as ::core::ffi::c_int) < 100 as ::core::ffi::c_int
        {
            UniEffect(
                C2Rust_Unnamed_0::UNI_INSTRUMENT.0 as ::core::ffi::c_int as UWORD,
                (ins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD,
            );
        } else if ins as ::core::ffi::c_int == 253 as ::core::ffi::c_int {
            UniWriteByte(C2Rust_Unnamed_0::UNI_KEYOFF.0 as ::core::ffi::c_int as UBYTE);
        } else if ins as ::core::ffi::c_int != 255 as ::core::ffi::c_int {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<UBYTE>();
        }
        if volpan as ::core::ffi::c_int <= 64 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLUME.0 as ::core::ffi::c_int as UWORD,
                volpan,
            );
        } else if volpan as ::core::ffi::c_int == 65 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLSLIDE.0 as ::core::ffi::c_int as UWORD,
                0 as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 74 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLSLIDE.0 as ::core::ffi::c_int as UWORD,
                (0xf as ::core::ffi::c_int
                    + ((volpan as ::core::ffi::c_int - 65 as ::core::ffi::c_int)
                        << 4 as ::core::ffi::c_int)) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int == 75 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLSLIDE.0 as ::core::ffi::c_int as UWORD,
                0 as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 84 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLSLIDE.0 as ::core::ffi::c_int as UWORD,
                (0xf0 as ::core::ffi::c_int
                    + (volpan as ::core::ffi::c_int - 75 as ::core::ffi::c_int)) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 94 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLSLIDE.0 as ::core::ffi::c_int as UWORD,
                ((volpan as ::core::ffi::c_int - 85 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 104 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VOLSLIDE.0 as ::core::ffi::c_int as UWORD,
                (volpan as ::core::ffi::c_int - 95 as ::core::ffi::c_int) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 114 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_PITCHSLIDEDN.0 as ::core::ffi::c_int as UWORD,
                (volpan as ::core::ffi::c_int - 105 as ::core::ffi::c_int) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 124 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_PITCHSLIDEUP.0 as ::core::ffi::c_int as UWORD,
                (volpan as ::core::ffi::c_int - 115 as ::core::ffi::c_int) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 127 as ::core::ffi::c_int {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<UBYTE>();
        } else if volpan as ::core::ffi::c_int <= 192 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_PANNING.0 as ::core::ffi::c_int as UWORD,
                (if volpan as ::core::ffi::c_int - 128 as ::core::ffi::c_int
                    == 64 as ::core::ffi::c_int
                {
                    255 as ::core::ffi::c_int
                } else {
                    (volpan as ::core::ffi::c_int - 128 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
                }) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int <= 202 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_PORTAMENTO.0 as ::core::ffi::c_int as UWORD,
                portatable[(volpan as ::core::ffi::c_int - 193 as ::core::ffi::c_int)
                    as usize],
            );
        } else if volpan as ::core::ffi::c_int <= 212 as ::core::ffi::c_int {
            UniVolEffect(
                C2Rust_Unnamed_1::VOL_VIBRATO.0 as ::core::ffi::c_int as UWORD,
                (volpan as ::core::ffi::c_int - 203 as ::core::ffi::c_int) as UBYTE,
            );
        } else if volpan as ::core::ffi::c_int != 239 as ::core::ffi::c_int
            && volpan as ::core::ffi::c_int != 255 as ::core::ffi::c_int
        {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<UBYTE>();
        }
        S3MIT_ProcessCmd(
            (*tr.offset((t * of.numchn as ::core::ffi::c_int) as isize)).cmd,
            (*tr.offset((t * of.numchn as ::core::ffi::c_int) as isize)).inf,
            old_effect | 2 as ::core::ffi::c_uint,
        );
        UniNewline();
        t += 1;
    }
    return UniDup();
}
unsafe extern "C" fn IT_ReadPattern(mut patrows: UWORD) -> BOOL {
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut flag: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut blah: ::core::ffi::c_uint = 0;
    let mut itt: *mut ITNOTE = itpat;
    let mut dummy: ITNOTE = ITNOTE {
        note: 0,
        ins: 0,
        volpan: 0,
        cmd: 0,
        inf: 0,
    };
    let mut n: *mut ITNOTE = ::core::ptr::null_mut::<ITNOTE>();
    let mut l: *mut ITNOTE = ::core::ptr::null_mut::<ITNOTE>();
    let mut ite: *mut ITNOTE = itpat
        .offset(
            (200 as ::core::ffi::c_int * 64 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as isize,
        );
    let mut m: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
    memset(
        itt as *mut ::core::ffi::c_void,
        255 as ::core::ffi::c_int,
        ((200 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<ITNOTE>()),
    );
    loop {
        if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0 as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        flag = (*modreader).Get.expect("non-null function pointer")(modreader) as UBYTE
            as ::core::ffi::c_int;
        if flag == 0 {
            itt = itt.offset(of.numchn as isize);
            row += 1;
        } else {
            ch = remap[(flag - 1 as ::core::ffi::c_int & 63 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
            if ch != -1 as ::core::ffi::c_int {
                n = itt.offset(ch as isize);
                l = last.offset(ch as isize);
                m = mask.offset(ch as isize);
                if n > ite {
                    MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0
                        as ::core::ffi::c_int;
                    return 0 as BOOL;
                }
            } else {
                l = &raw mut dummy;
                n = l;
                blah = 0 as ::core::ffi::c_uint;
                m = &raw mut blah as *mut UBYTE;
            }
            if flag & 128 as ::core::ffi::c_int != 0 {
                *m = (*modreader).Get.expect("non-null function pointer")(modreader)
                    as UBYTE;
            }
            if *m as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                (*n).note = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                (*l).note = (*n).note;
                if (*l).note as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
                    (*n).note = 253 as UBYTE;
                    (*l).note = (*n).note;
                }
            }
            if *m as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                (*n).ins = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                (*l).ins = (*n).ins;
            }
            if *m as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                (*n).volpan = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                (*l).volpan = (*n).volpan;
            }
            if *m as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
                (*n).cmd = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                (*l).cmd = (*n).cmd;
                (*n).inf = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                (*l).inf = (*n).inf;
            }
            if *m as ::core::ffi::c_int & 16 as ::core::ffi::c_int != 0 {
                (*n).note = (*l).note;
            }
            if *m as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                (*n).ins = (*l).ins;
            }
            if *m as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                (*n).volpan = (*l).volpan;
            }
            if *m as ::core::ffi::c_int & 128 as ::core::ffi::c_int != 0 {
                (*n).cmd = (*l).cmd;
                (*n).inf = (*l).inf;
            }
        }
        if row >= patrows as ::core::ffi::c_int {
            break;
        }
    }
    blah = 0 as ::core::ffi::c_uint;
    while blah < of.numchn as ::core::ffi::c_uint {
        let c2rust_fresh2 = numtrk;
        numtrk += 1;
        let c2rust_lvalue_ptr = &raw mut *of.tracks.offset(c2rust_fresh2 as isize);
        *c2rust_lvalue_ptr = IT_ConvertTrack(itpat.offset(blah as isize), patrows);
        if (*c2rust_lvalue_ptr).is_null() {
            return 0 as BOOL;
        }
        blah = blah.wrapping_add(1);
    }
    return 1 as BOOL;
}
unsafe extern "C" fn LoadMidiString(mut r: *mut MREADER, mut dest: *mut CHAR) {
    let mut curp: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    let mut lastp: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    (*r)
        .Read
        .expect(
            "non-null function pointer",
        )(r, dest as *mut ::core::ffi::c_void, 32 as size_t);
    lastp = dest;
    curp = lastp;
    while *lastp != 0 {
        if mik_isalnum(*lastp as ::core::ffi::c_int) != 0 {
            let c2rust_fresh3 = curp;
            curp = curp.offset(1);
            *c2rust_fresh3 = mik_toupper(*lastp as ::core::ffi::c_int) as CHAR;
        }
        lastp = lastp.offset(1);
    }
    *curp = 0 as CHAR;
}
unsafe extern "C" fn IT_LoadMidiConfiguration(mut r: *mut MREADER) {
    let mut i: ::core::ffi::c_int = 0;
    memset(
        &raw mut filtermacros as *mut UBYTE as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[UBYTE; 16]>(),
    );
    memset(
        &raw mut filtersettings as *mut FILTER as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[FILTER; 256]>(),
    );
    if !r.is_null() {
        let mut dat: UWORD = 0;
        let mut midiline: [CHAR; 33] = [0; 33];
        dat = _mm_read_I_UWORD(r);
        (*r)
            .Seek
            .expect(
                "non-null function pointer",
            )(
            r,
            (8 as ::core::ffi::c_int * dat as ::core::ffi::c_int
                + 0x120 as ::core::ffi::c_int) as ::core::ffi::c_long,
            1 as ::core::ffi::c_int,
        );
        i = 0 as ::core::ffi::c_int;
        while i < 0x10 as ::core::ffi::c_int {
            LoadMidiString(r, &raw mut midiline as *mut CHAR);
            if strncmp(
                &raw mut midiline as *mut CHAR,
                b"F0F00\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0
                && (midiline[5usize] as ::core::ffi::c_int == '0' as ::core::ffi::c_int
                    || midiline[5usize] as ::core::ffi::c_int
                        == '1' as ::core::ffi::c_int)
            {
                filtermacros[i as usize] = (midiline[5usize] as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as UBYTE;
            }
            i += 1;
        }
        i = 0x80 as ::core::ffi::c_int;
        while i < 0x100 as ::core::ffi::c_int {
            LoadMidiString(r, &raw mut midiline as *mut CHAR);
            if strncmp(
                &raw mut midiline as *mut CHAR,
                b"F0F00\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0
                && (midiline[5usize] as ::core::ffi::c_int == '0' as ::core::ffi::c_int
                    || midiline[5usize] as ::core::ffi::c_int
                        == '1' as ::core::ffi::c_int)
            {
                filtersettings[i as usize].filter = (midiline[5usize]
                    as ::core::ffi::c_int - '0' as ::core::ffi::c_int
                    | 0x80 as ::core::ffi::c_int) as UBYTE;
                dat = (if midiline[6usize] as ::core::ffi::c_int != 0 {
                    midiline[6usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as UWORD;
                if midiline[7usize] != 0 {
                    dat = ((dat as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | midiline[7usize] as ::core::ffi::c_int
                            - '0' as ::core::ffi::c_int) as UWORD;
                }
                filtersettings[i as usize].inf = dat as UBYTE;
            }
            i += 1;
        }
    } else {
        filtermacros[0usize] = 0x80 as UBYTE;
        i = 0x80 as ::core::ffi::c_int;
        while i < 0x90 as ::core::ffi::c_int {
            filtersettings[i as usize].filter = 0x81 as UBYTE;
            filtersettings[i as usize].inf = ((i & 0x7f as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int) as UBYTE;
            i += 1;
        }
    }
    activemacro = 0 as UBYTE;
    i = 0 as ::core::ffi::c_int;
    while i < 0x80 as ::core::ffi::c_int {
        filtersettings[i as usize].filter = filtermacros[0usize];
        filtersettings[i as usize].inf = i as UBYTE;
        i += 1;
    }
}
unsafe extern "C" fn IT_Load(mut curious: BOOL) -> BOOL {
    let mut t: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut d: *mut INSTRUMENT = ::core::ptr::null_mut::<INSTRUMENT>();
    let mut q: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    numtrk = 0 as ::core::ffi::c_int;
    filters = 0 as ::core::ffi::c_int as BOOL;
    _mm_read_I_ULONG(modreader);
    _mm_read_string(
        &raw mut (*mh).songname as *mut CHAR,
        26 as ::core::ffi::c_int,
        modreader,
    );
    (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut (*mh).blank01 as *mut UBYTE as *mut ::core::ffi::c_void,
        2 as size_t,
    );
    (*mh).ordnum = _mm_read_I_UWORD(modreader);
    (*mh).insnum = _mm_read_I_UWORD(modreader);
    (*mh).smpnum = _mm_read_I_UWORD(modreader);
    (*mh).patnum = _mm_read_I_UWORD(modreader);
    (*mh).cwt = _mm_read_I_UWORD(modreader);
    (*mh).cmwt = _mm_read_I_UWORD(modreader);
    (*mh).flags = _mm_read_I_UWORD(modreader);
    (*mh).special = _mm_read_I_UWORD(modreader);
    (*mh).globvol = (*modreader).Get.expect("non-null function pointer")(modreader)
        as UBYTE;
    (*mh).mixvol = (*modreader).Get.expect("non-null function pointer")(modreader)
        as UBYTE;
    (*mh).initspeed = (*modreader).Get.expect("non-null function pointer")(modreader)
        as UBYTE;
    (*mh).inittempo = (*modreader).Get.expect("non-null function pointer")(modreader)
        as UBYTE;
    (*mh).pansep = (*modreader).Get.expect("non-null function pointer")(modreader)
        as UBYTE;
    (*mh).zerobyte = (*modreader).Get.expect("non-null function pointer")(modreader)
        as UBYTE;
    (*mh).msglength = _mm_read_I_UWORD(modreader);
    (*mh).msgoffset = _mm_read_I_ULONG(modreader);
    (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut (*mh).blank02 as *mut UBYTE as *mut ::core::ffi::c_void,
        4 as size_t,
    );
    (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut (*mh).pantable as *mut UBYTE as *mut ::core::ffi::c_void,
        64 as size_t,
    );
    (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(
        modreader,
        &raw mut (*mh).voltable as *mut UBYTE as *mut ::core::ffi::c_void,
        64 as size_t,
    );
    if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    if (*mh).ordnum as ::core::ffi::c_int > 256 as ::core::ffi::c_int
        || (*mh).insnum as ::core::ffi::c_int > 255 as ::core::ffi::c_int
        || (*mh).smpnum as ::core::ffi::c_int > 255 as ::core::ffi::c_int
        || (*mh).patnum as ::core::ffi::c_int > 255 as ::core::ffi::c_int
    {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_MODULE.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    of.songname = DupStr(&raw mut (*mh).songname as *mut CHAR, 26 as UWORD, 0 as BOOL);
    of.reppos = 0 as UWORD;
    of.numpat = (*mh).patnum;
    of.numins = (*mh).insnum;
    of.numsmp = (*mh).smpnum;
    of.initspeed = (*mh).initspeed;
    of.inittempo = (*mh).inittempo as UWORD;
    of.initvolume = (*mh).globvol;
    of.flags = (of.flags as ::core::ffi::c_int
        | (0x20 as ::core::ffi::c_int | 0x100 as ::core::ffi::c_int)) as UWORD;
    if (*mh).flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
        of.flags = (of.flags as ::core::ffi::c_int | 0x400 as ::core::ffi::c_int)
            as UWORD;
    }
    of.bpmlimit = 32 as UWORD;
    if (*mh).songname[25usize] != 0 {
        of.numvoices = (1 as ::core::ffi::c_int
            + (*mh).songname[25usize] as ::core::ffi::c_int) as UBYTE;
    }
    if (*mh).cwt as ::core::ffi::c_int <= 0x219 as ::core::ffi::c_int
        && (*mh).cwt as ::core::ffi::c_int >= 0x217 as ::core::ffi::c_int
    {
        of.modtype = MikMod_strdup(
            IT_Version[(if ((*mh).cmwt as ::core::ffi::c_int)
                < 0x214 as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                5 as ::core::ffi::c_int
            }) as usize],
        );
    } else if (*mh).cwt as ::core::ffi::c_int >= 0x215 as ::core::ffi::c_int {
        of.modtype = MikMod_strdup(
            IT_Version[(if ((*mh).cmwt as ::core::ffi::c_int)
                < 0x214 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            }) as usize],
        );
    } else {
        of.modtype = MikMod_strdup(
            IT_Version[(if ((*mh).cmwt as ::core::ffi::c_int)
                < 0x214 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as usize],
        );
        *of
            .modtype
            .offset(
                (if ((*mh).cmwt as ::core::ffi::c_int) < 0x214 as ::core::ffi::c_int {
                    15 as ::core::ffi::c_int
                } else {
                    26 as ::core::ffi::c_int
                }) as isize,
            ) = (((*mh).cwt as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
            + '0' as ::core::ffi::c_int) as CHAR;
        *of
            .modtype
            .offset(
                (if ((*mh).cmwt as ::core::ffi::c_int) < 0x214 as ::core::ffi::c_int {
                    17 as ::core::ffi::c_int
                } else {
                    28 as ::core::ffi::c_int
                }) as isize,
            ) = (((*mh).cwt as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int) + '0' as ::core::ffi::c_int) as CHAR;
        *of
            .modtype
            .offset(
                (if ((*mh).cmwt as ::core::ffi::c_int) < 0x214 as ::core::ffi::c_int {
                    18 as ::core::ffi::c_int
                } else {
                    29 as ::core::ffi::c_int
                }) as isize,
            ) = (((*mh).cwt as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
            + '0' as ::core::ffi::c_int) as CHAR;
    }
    if (*mh).flags as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
        of.flags = (of.flags as ::core::ffi::c_int
            | (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)) as UWORD;
    }
    if (*mh).cwt as ::core::ffi::c_int >= 0x106 as ::core::ffi::c_int
        && (*mh).flags as ::core::ffi::c_int & 16 as ::core::ffi::c_int != 0
    {
        old_effect = 1 as ::core::ffi::c_uint;
    } else {
        old_effect = 0 as ::core::ffi::c_uint;
    }
    if (*mh).flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
        t = 0 as ::core::ffi::c_int;
        while t < 64 as ::core::ffi::c_int {
            (*mh).pantable[t as usize] = ((*mh).pantable[t as usize]
                as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as UBYTE;
            if ((*mh).pantable[t as usize] as ::core::ffi::c_int)
                < 64 as ::core::ffi::c_int
            {
                of.panning[t as usize] = (((*mh).pantable[t as usize]
                    as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UWORD;
            } else if (*mh).pantable[t as usize] as ::core::ffi::c_int
                == 64 as ::core::ffi::c_int
            {
                of.panning[t as usize] = 255 as UWORD;
            } else if (*mh).pantable[t as usize] as ::core::ffi::c_int
                == 100 as ::core::ffi::c_int
            {
                of.panning[t as usize] = 512 as UWORD;
            } else if (*mh).pantable[t as usize] as ::core::ffi::c_int
                == 127 as ::core::ffi::c_int
            {
                of.panning[t as usize] = 128 as UWORD;
            } else {
                MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0
                    as ::core::ffi::c_int;
                return 0 as BOOL;
            }
            t += 1;
        }
    } else {
        t = 0 as ::core::ffi::c_int;
        while t < 64 as ::core::ffi::c_int {
            of.panning[t as usize] = 128 as UWORD;
            t += 1;
        }
    }
    memcpy(
        &raw mut of.chanvol as *mut UBYTE as *mut ::core::ffi::c_void,
        &raw mut (*mh).voltable as *mut UBYTE as *const ::core::ffi::c_void,
        64 as size_t,
    );
    if AllocPositions((*mh).ordnum as ::core::ffi::c_int) == 0 {
        return 0 as BOOL;
    }
    origpositions = MikMod_calloc(
        (*mh).ordnum as size_t,
        ::core::mem::size_of::<UWORD>(),
    ) as *mut UWORD;
    if origpositions.is_null() {
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < (*mh).ordnum as ::core::ffi::c_int {
        *origpositions.offset(t as isize) = (*modreader)
            .Get
            .expect("non-null function pointer")(modreader) as UBYTE as UWORD;
        if *origpositions.offset(t as isize) as ::core::ffi::c_int
            > (*mh).patnum as ::core::ffi::c_int
            && (*origpositions.offset(t as isize) as ::core::ffi::c_int)
                < 254 as ::core::ffi::c_int
        {
            *origpositions.offset(t as isize) = 255 as UWORD;
        }
        t += 1;
    }
    if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    poslookupcnt = (*mh).ordnum;
    S3MIT_CreateOrders(curious);
    paraptr = MikMod_malloc(
        (((*mh).insnum as ::core::ffi::c_int + (*mh).smpnum as ::core::ffi::c_int
            + of.numpat as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<ULONG>()),
    ) as *mut ULONG;
    if paraptr.is_null() {
        return 0 as BOOL;
    }
    _mm_read_I_ULONGS(
        paraptr,
        (*mh).insnum as ::core::ffi::c_int + (*mh).smpnum as ::core::ffi::c_int
            + of.numpat as ::core::ffi::c_int,
        modreader,
    );
    if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
        MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0 as ::core::ffi::c_int;
        return 0 as BOOL;
    }
    if (*mh).cmwt as ::core::ffi::c_int >= 0x216 as ::core::ffi::c_int {
        if (*mh).special as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
            IT_LoadMidiConfiguration(modreader);
            if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
                MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_HEADER.0
                    as ::core::ffi::c_int;
                return 0 as BOOL;
            }
        } else {
            IT_LoadMidiConfiguration(::core::ptr::null_mut::<MREADER>());
        }
        filters = 1 as ::core::ffi::c_int as BOOL;
    }
    if (*mh).special as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
        && (*mh).cwt as ::core::ffi::c_int >= 0x104 as ::core::ffi::c_int
        && (*mh).msglength as ::core::ffi::c_int != 0
    {
        (*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(
            modreader,
            (*mh).msgoffset as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        if ReadComment((*mh).msglength) == 0 {
            return 0 as BOOL;
        }
    }
    if (*mh).flags as ::core::ffi::c_int & 4 as ::core::ffi::c_int == 0 {
        of.numins = of.numsmp;
    }
    if AllocSamples() == 0 {
        return 0 as BOOL;
    }
    if AllocLinear().is_null() {
        return 0 as BOOL;
    }
    q = of.samples as *mut SAMPLE;
    t = 0 as ::core::ffi::c_int;
    while t < (*mh).smpnum as ::core::ffi::c_int {
        let mut s: ITSAMPLE = ITSAMPLE {
            filename: [0; 12],
            zerobyte: 0,
            globvol: 0,
            flag: 0,
            volume: 0,
            panning: 0,
            sampname: [0; 28],
            convert: 0,
            length: 0,
            loopbeg: 0,
            loopend: 0,
            c5spd: 0,
            susbegin: 0,
            susend: 0,
            sampoffset: 0,
            vibspeed: 0,
            vibdepth: 0,
            vibrate: 0,
            vibwave: 0,
        };
        (*modreader)
            .Seek
            .expect(
                "non-null function pointer",
            )(
            modreader,
            (*paraptr.offset(((*mh).insnum as ::core::ffi::c_int + t) as isize))
                .wrapping_add(4 as ULONG) as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        _mm_read_string(
            &raw mut s.filename as *mut CHAR,
            12 as ::core::ffi::c_int,
            modreader,
        );
        s.zerobyte = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.globvol = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.flag = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.volume = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        _mm_read_string(
            &raw mut s.sampname as *mut CHAR,
            26 as ::core::ffi::c_int,
            modreader,
        );
        s.convert = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE as UWORD;
        s.panning = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.length = _mm_read_I_ULONG(modreader);
        s.loopbeg = _mm_read_I_ULONG(modreader);
        s.loopend = _mm_read_I_ULONG(modreader);
        s.c5spd = _mm_read_I_ULONG(modreader);
        s.susbegin = _mm_read_I_ULONG(modreader);
        s.susend = _mm_read_I_ULONG(modreader);
        s.sampoffset = _mm_read_I_ULONG(modreader);
        s.vibspeed = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.vibdepth = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.vibrate = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        s.vibwave = (*modreader).Get.expect("non-null function pointer")(modreader)
            as UBYTE;
        if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0
            || s.c5spd > 0x7ffff as ULONG || s.length > 0xfffffff as ::core::ffi::c_ulong
        {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0
                as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        if s.flag as ::core::ffi::c_int & 16 as ::core::ffi::c_int != 0
            && (s.loopbeg > 0xfffffff as ::core::ffi::c_ulong
                || s.loopend > 0xfffffff as ::core::ffi::c_ulong)
        {
            MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0
                as ::core::ffi::c_int;
            return 0 as BOOL;
        }
        (*q).samplename = DupStr(
            &raw mut s.sampname as *mut CHAR,
            26 as UWORD,
            0 as BOOL,
        );
        (*q).speed = s.c5spd.wrapping_div(2 as ULONG);
        (*q).panning = (if s.panning as ::core::ffi::c_int & 127 as ::core::ffi::c_int
            == 64 as ::core::ffi::c_int
        {
            255 as ::core::ffi::c_int
        } else {
            (s.panning as ::core::ffi::c_int & 127 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int
        }) as SWORD;
        (*q).length = s.length;
        (*q).loopstart = s.loopbeg;
        (*q).loopend = s.loopend;
        (*q).volume = s.volume;
        (*q).globvol = s.globvol;
        (*q).seekpos = s.sampoffset;
        if of.flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
            (*q).speed = speed_to_finetune(s.c5spd, t) as ULONG;
        }
        if s.panning as ::core::ffi::c_int & 128 as ::core::ffi::c_int != 0 {
            (*q).flags = ((*q).flags as ::core::ffi::c_int
                | 0x1000 as ::core::ffi::c_int) as UWORD;
        }
        if s.vibrate != 0 {
            (*q).vibflags = ((*q).vibflags as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int) as UBYTE;
            (*q).vibtype = s.vibwave;
            (*q).vibsweep = (s.vibrate as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as UBYTE;
            (*q).vibdepth = s.vibdepth;
            (*q).vibrate = s.vibspeed;
        }
        if s.flag as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
            (*q).flags = ((*q).flags as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                as UWORD;
        }
        if s.flag as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0
            && (*mh).cwt as ::core::ffi::c_int >= 0x214 as ::core::ffi::c_int
        {
            (*q).flags = ((*q).flags as ::core::ffi::c_int | 0x20 as ::core::ffi::c_int)
                as UWORD;
        }
        if s.flag as ::core::ffi::c_int & 16 as ::core::ffi::c_int != 0 {
            (*q).flags = ((*q).flags as ::core::ffi::c_int | 0x100 as ::core::ffi::c_int)
                as UWORD;
        }
        if s.flag as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
            (*q).flags = ((*q).flags as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int)
                as UWORD;
        }
        if (*mh).cwt as ::core::ffi::c_int >= 0x200 as ::core::ffi::c_int {
            if s.convert as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                (*q).flags = ((*q).flags as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int) as UWORD;
            }
            if s.convert as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                (*q).flags = ((*q).flags as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int) as UWORD;
            }
        }
        q = q.offset(1);
        t += 1;
    }
    if (*mh).flags as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
        if AllocInstruments() == 0 {
            return 0 as BOOL;
        }
        d = of.instruments as *mut INSTRUMENT;
        of.flags = (of.flags as ::core::ffi::c_int
            | (0x8 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)) as UWORD;
        t = 0 as ::core::ffi::c_int;
        while t < (*mh).insnum as ::core::ffi::c_int {
            let mut ih: ITINSTHEADER = ITINSTHEADER {
                size: 0,
                filename: [0; 12],
                zerobyte: 0,
                volflg: 0,
                volpts: 0,
                volbeg: 0,
                volend: 0,
                volsusbeg: 0,
                volsusend: 0,
                panflg: 0,
                panpts: 0,
                panbeg: 0,
                panend: 0,
                pansusbeg: 0,
                pansusend: 0,
                pitflg: 0,
                pitpts: 0,
                pitbeg: 0,
                pitend: 0,
                pitsusbeg: 0,
                pitsusend: 0,
                blank: 0,
                globvol: 0,
                chanpan: 0,
                fadeout: 0,
                dnc: 0,
                dca: 0,
                dct: 0,
                nna: 0,
                trkvers: 0,
                ppsep: 0,
                ppcenter: 0,
                rvolvar: 0,
                rpanvar: 0,
                numsmp: 0,
                name: [0; 26],
                blank01: [0; 6],
                samptable: [0; 120],
                volenv: [0; 200],
                oldvoltick: [0; 25],
                volnode: [0; 25],
                voltick: [0; 25],
                pannode: [0; 25],
                pantick: [0; 25],
                pitnode: [0; 25],
                pittick: [0; 25],
            };
            (*modreader)
                .Seek
                .expect(
                    "non-null function pointer",
                )(
                modreader,
                (*paraptr.offset(t as isize)).wrapping_add(4 as ULONG)
                    as ::core::ffi::c_long,
                0 as ::core::ffi::c_int,
            );
            _mm_read_string(
                &raw mut ih.filename as *mut CHAR,
                12 as ::core::ffi::c_int,
                modreader,
            );
            ih.zerobyte = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE;
            if ((*mh).cwt as ::core::ffi::c_int) < 0x200 as ::core::ffi::c_int {
                ih.volflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volsusbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volsusend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                _mm_read_I_UWORD(modreader);
                ih.fadeout = _mm_read_I_UWORD(modreader);
                ih.nna = (*modreader).Get.expect("non-null function pointer")(modreader)
                    as UBYTE;
                ih.dnc = (*modreader).Get.expect("non-null function pointer")(modreader)
                    as UBYTE;
            } else {
                ih.nna = (*modreader).Get.expect("non-null function pointer")(modreader)
                    as UBYTE;
                ih.dct = (*modreader).Get.expect("non-null function pointer")(modreader)
                    as UBYTE;
                ih.dca = (*modreader).Get.expect("non-null function pointer")(modreader)
                    as UBYTE;
                ih.fadeout = _mm_read_I_UWORD(modreader);
                ih.ppsep = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.ppcenter = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.globvol = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.chanpan = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.rvolvar = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.rpanvar = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
            }
            ih.trkvers = _mm_read_I_UWORD(modreader);
            ih.numsmp = (*modreader).Get.expect("non-null function pointer")(modreader)
                as UBYTE as UWORD;
            (*modreader).Get.expect("non-null function pointer")(modreader);
            _mm_read_string(
                &raw mut ih.name as *mut CHAR,
                26 as ::core::ffi::c_int,
                modreader,
            );
            (*modreader)
                .Read
                .expect(
                    "non-null function pointer",
                )(
                modreader,
                &raw mut ih.blank01 as *mut UBYTE as *mut ::core::ffi::c_void,
                6 as size_t,
            );
            _mm_read_I_UWORDS(
                &raw mut ih.samptable as *mut UWORD,
                120 as ::core::ffi::c_int,
                modreader,
            );
            if ((*mh).cwt as ::core::ffi::c_int) < 0x200 as ::core::ffi::c_int {
                (*modreader)
                    .Read
                    .expect(
                        "non-null function pointer",
                    )(
                    modreader,
                    &raw mut ih.volenv as *mut UBYTE as *mut ::core::ffi::c_void,
                    200 as size_t,
                );
                lp = 0 as ::core::ffi::c_int;
                while lp < 25 as ::core::ffi::c_int {
                    ih.oldvoltick[lp as usize] = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    ih.volnode[lp as usize] = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    lp += 1;
                }
            } else {
                ih.volflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volpts = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                if ih.volpts as ::core::ffi::c_int > 25 as ::core::ffi::c_int {
                    ih.volpts = 25 as UBYTE;
                }
                ih.volbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volsusbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.volsusend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                lp = 0 as ::core::ffi::c_int;
                while lp < 25 as ::core::ffi::c_int {
                    ih.volnode[lp as usize] = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as UBYTE;
                    ih.voltick[lp as usize] = _mm_read_I_UWORD(modreader);
                    lp += 1;
                }
                (*modreader).Get.expect("non-null function pointer")(modreader);
                ih.panflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.panpts = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                if ih.panpts as ::core::ffi::c_int > 25 as ::core::ffi::c_int {
                    ih.panpts = 25 as UBYTE;
                }
                ih.panbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.panend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.pansusbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.pansusend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                lp = 0 as ::core::ffi::c_int;
                while lp < 25 as ::core::ffi::c_int {
                    ih.pannode[lp as usize] = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as SBYTE;
                    ih.pantick[lp as usize] = _mm_read_I_UWORD(modreader);
                    lp += 1;
                }
                (*modreader).Get.expect("non-null function pointer")(modreader);
                ih.pitflg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.pitpts = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                if ih.pitpts as ::core::ffi::c_int > 25 as ::core::ffi::c_int {
                    ih.pitpts = 25 as UBYTE;
                }
                ih.pitbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.pitend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.pitsusbeg = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                ih.pitsusend = (*modreader)
                    .Get
                    .expect("non-null function pointer")(modreader) as UBYTE;
                lp = 0 as ::core::ffi::c_int;
                while lp < 25 as ::core::ffi::c_int {
                    ih.pitnode[lp as usize] = (*modreader)
                        .Get
                        .expect("non-null function pointer")(modreader) as SBYTE;
                    ih.pittick[lp as usize] = _mm_read_I_UWORD(modreader);
                    lp += 1;
                }
                (*modreader).Get.expect("non-null function pointer")(modreader);
            }
            if (*modreader).Eof.expect("non-null function pointer")(modreader) != 0 {
                MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_SAMPLEINFO.0
                    as ::core::ffi::c_int;
                return 0 as BOOL;
            }
            (*d).volflg = ((*d).volflg as ::core::ffi::c_int | 8 as ::core::ffi::c_int)
                as UBYTE;
            (*d).insname = DupStr(&raw mut ih.name as *mut CHAR, 26 as UWORD, 0 as BOOL);
            (*d).nnatype = (ih.nna as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                as UBYTE;
            if ((*mh).cwt as ::core::ffi::c_int) < 0x200 as ::core::ffi::c_int {
                (*d).volfade = ((ih.fadeout as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as UWORD;
                if ih.dnc != 0 {
                    (*d).dct = 1 as UBYTE;
                    (*d).dca = 0 as UBYTE;
                }
                if ih.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.volflg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 4 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.volflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).volbeg = ih.volbeg;
                (*d).volend = ih.volend;
                (*d).volsusbeg = ih.volsusbeg;
                (*d).volsusend = ih.volsusend;
                if ih.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    u = 0 as ::core::ffi::c_int;
                    while u < 25 as ::core::ffi::c_int {
                        if ih.oldvoltick[(*d).volpts as usize] as ::core::ffi::c_int
                            == 0xff as ::core::ffi::c_int
                        {
                            break;
                        }
                        (*d).volenv[(*d).volpts as usize].val = ((ih
                            .volnode[(*d).volpts as usize] as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) as SWORD;
                        (*d).volenv[(*d).volpts as usize].pos = ih
                            .oldvoltick[(*d).volpts as usize] as SWORD;
                        (*d).volpts = (*d).volpts.wrapping_add(1);
                        u += 1;
                    }
                }
            } else {
                (*d).panning = (if ih.chanpan as ::core::ffi::c_int
                    & 127 as ::core::ffi::c_int == 64 as ::core::ffi::c_int
                {
                    255 as ::core::ffi::c_int
                } else {
                    (ih.chanpan as ::core::ffi::c_int & 127 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
                }) as SWORD;
                if ih.chanpan as ::core::ffi::c_int & 128 as ::core::ffi::c_int == 0 {
                    (*d).flags = ((*d).flags as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.ppsep as ::core::ffi::c_int & 128 as ::core::ffi::c_int == 0 {
                    (*d).pitpansep = ((ih.ppsep as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) as UBYTE;
                    (*d).pitpancenter = ih.ppcenter;
                    (*d).flags = ((*d).flags as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).globvol = (ih.globvol as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as UBYTE;
                (*d).volfade = ((ih.fadeout as ::core::ffi::c_int)
                    << 5 as ::core::ffi::c_int) as UWORD;
                (*d).dct = ih.dct;
                (*d).dca = ih.dca;
                if (*mh).cwt as ::core::ffi::c_int >= 0x204 as ::core::ffi::c_int {
                    (*d).rvolvar = ih.rvolvar;
                    (*d).rpanvar = ih.rpanvar;
                }
                if ih.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.volflg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 4 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.volflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).volpts = ih.volpts;
                (*d).volbeg = ih.volbeg;
                (*d).volend = ih.volend;
                (*d).volsusbeg = ih.volsusbeg;
                (*d).volsusend = ih.volsusend;
                u = 0 as ::core::ffi::c_int;
                while u < ih.volpts as ::core::ffi::c_int {
                    (*d).volenv[u as usize].pos = ih.voltick[u as usize] as SWORD;
                    u += 1;
                }
                if (*d).volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                    && ((*d).volpts as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                {
                    (*d).volflg = ((*d).volflg as ::core::ffi::c_int
                        & !(1 as ::core::ffi::c_int)) as UBYTE;
                }
                u = 0 as ::core::ffi::c_int;
                while u < ih.volpts as ::core::ffi::c_int {
                    (*d).volenv[u as usize].val = ((ih.volnode[u as usize]
                        as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SWORD;
                    u += 1;
                }
                if ih.panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.panflg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        | 4 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.panflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).panpts = ih.panpts;
                (*d).panbeg = ih.panbeg;
                (*d).panend = ih.panend;
                (*d).pansusbeg = ih.pansusbeg;
                (*d).pansusend = ih.pansusend;
                u = 0 as ::core::ffi::c_int;
                while u < ih.panpts as ::core::ffi::c_int {
                    (*d).panenv[u as usize].pos = ih.pantick[u as usize] as SWORD;
                    u += 1;
                }
                if (*d).panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                    && ((*d).panpts as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                {
                    (*d).panflg = ((*d).panflg as ::core::ffi::c_int
                        & !(1 as ::core::ffi::c_int)) as UBYTE;
                }
                u = 0 as ::core::ffi::c_int;
                while u < ih.panpts as ::core::ffi::c_int {
                    (*d).panenv[u as usize].val = (if ih.pannode[u as usize]
                        as ::core::ffi::c_int == 32 as ::core::ffi::c_int
                    {
                        255 as ::core::ffi::c_int
                    } else {
                        (ih.pannode[u as usize] as ::core::ffi::c_int
                            + 32 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    }) as SWORD;
                    u += 1;
                }
                if ih.pitflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    (*d).pitflg = ((*d).pitflg as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.pitflg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    (*d).pitflg = ((*d).pitflg as ::core::ffi::c_int
                        | 4 as ::core::ffi::c_int) as UBYTE;
                }
                if ih.pitflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                    (*d).pitflg = ((*d).pitflg as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                }
                (*d).pitpts = ih.pitpts;
                (*d).pitbeg = ih.pitbeg;
                (*d).pitend = ih.pitend;
                (*d).pitsusbeg = ih.pitsusbeg;
                (*d).pitsusend = ih.pitsusend;
                u = 0 as ::core::ffi::c_int;
                while u < ih.pitpts as ::core::ffi::c_int {
                    (*d).pitenv[u as usize].pos = ih.pittick[u as usize] as SWORD;
                    u += 1;
                }
                if (*d).pitflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                    && ((*d).pitpts as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                {
                    (*d).pitflg = ((*d).pitflg as ::core::ffi::c_int
                        & !(1 as ::core::ffi::c_int)) as UBYTE;
                }
                u = 0 as ::core::ffi::c_int;
                while u < ih.pitpts as ::core::ffi::c_int {
                    (*d).pitenv[u as usize].val = (ih.pitnode[u as usize]
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as SWORD;
                    u += 1;
                }
                if ih.pitflg as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    (*d).pitflg = ((*d).pitflg as ::core::ffi::c_int
                        & !(1 as ::core::ffi::c_int)) as UBYTE;
                    ih.pitend = 0 as UBYTE;
                    ih.pitbeg = ih.pitend;
                    ih.pitpts = ih.pitbeg;
                }
            }
            u = 0 as ::core::ffi::c_int;
            while u < 120 as ::core::ffi::c_int {
                (*d).samplenote[u as usize] = (ih.samptable[u as usize]
                    as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as UBYTE;
                (*d).samplenumber[u as usize] = (if ih.samptable[u as usize]
                    as ::core::ffi::c_int >> 8 as ::core::ffi::c_int != 0
                {
                    (ih.samptable[u as usize] as ::core::ffi::c_int
                        >> 8 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
                } else {
                    0xffff as ::core::ffi::c_int
                }) as UWORD;
                if (*d).samplenumber[u as usize] as ::core::ffi::c_int
                    >= of.numsmp as ::core::ffi::c_int
                {
                    (*d).samplenote[u as usize] = 255 as UBYTE;
                } else if of.flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0
                {
                    let mut note: ::core::ffi::c_int = (*d).samplenote[u as usize]
                        as ::core::ffi::c_int
                        + *noteindex.offset((*d).samplenumber[u as usize] as isize);
                    (*d).samplenote[u as usize] = (if note < 0 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else if note > 255 as ::core::ffi::c_int {
                        255 as ::core::ffi::c_int
                    } else {
                        note
                    }) as UBYTE;
                }
                u += 1;
            }
            d = d.offset(1);
            t += 1;
        }
    } else if of.flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        if AllocInstruments() == 0 {
            return 0 as BOOL;
        }
        d = of.instruments as *mut INSTRUMENT;
        of.flags = (of.flags as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int) as UWORD;
        t = 0 as ::core::ffi::c_int;
        while t < (*mh).smpnum as ::core::ffi::c_int {
            u = 0 as ::core::ffi::c_int;
            while u < 120 as ::core::ffi::c_int {
                if (*d).samplenumber[u as usize] as ::core::ffi::c_int
                    >= of.numsmp as ::core::ffi::c_int
                {
                    (*d).samplenote[u as usize] = 255 as UBYTE;
                } else {
                    let mut note_0: ::core::ffi::c_int = (*d).samplenote[u as usize]
                        as ::core::ffi::c_int
                        + *noteindex.offset((*d).samplenumber[u as usize] as isize);
                    (*d).samplenote[u as usize] = (if note_0 < 0 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else if note_0 > 255 as ::core::ffi::c_int {
                        255 as ::core::ffi::c_int
                    } else {
                        note_0
                    }) as UBYTE;
                }
                u += 1;
            }
            t += 1;
            d = d.offset(1);
        }
    }
    of.numchn = 0 as UBYTE;
    memset(
        &raw mut remap as *mut SBYTE as *mut ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        (64 as size_t).wrapping_mul(::core::mem::size_of::<UBYTE>()),
    );
    t = 0 as ::core::ffi::c_int;
    while t < of.numpat as ::core::ffi::c_int {
        let mut packlen: UWORD = 0;
        if *paraptr
            .offset(
                ((*mh).insnum as ::core::ffi::c_int + (*mh).smpnum as ::core::ffi::c_int
                    + t) as isize,
            ) != 0
        {
            (*modreader)
                .Seek
                .expect(
                    "non-null function pointer",
                )(
                modreader,
                *paraptr
                    .offset(
                        ((*mh).insnum as ::core::ffi::c_int
                            + (*mh).smpnum as ::core::ffi::c_int + t) as isize,
                    ) as ::core::ffi::c_long,
                0 as ::core::ffi::c_int,
            );
            _mm_read_I_UWORD(modreader);
            packlen = _mm_read_I_UWORD(modreader);
            if packlen as ::core::ffi::c_int > 256 as ::core::ffi::c_int {
                MikMod_errno = C2Rust_Unnamed::MMERR_LOADING_PATTERN.0
                    as ::core::ffi::c_int;
                return 0 as BOOL;
            }
            _mm_read_I_ULONG(modreader);
            if IT_GetNumChannels(packlen) == 0 {
                return 0 as BOOL;
            }
        }
        t += 1;
    }
    t = 0 as ::core::ffi::c_int;
    while t < 64 as ::core::ffi::c_int {
        if remap[t as usize] == 0 {
            let c2rust_fresh0 = of.numchn;
            of.numchn = of.numchn.wrapping_add(1);
            remap[t as usize] = c2rust_fresh0 as SBYTE;
        }
        t += 1;
    }
    of.numtrk = (of.numpat as ::core::ffi::c_int * of.numchn as ::core::ffi::c_int)
        as UWORD;
    if of.numvoices != 0 {
        if (of.numvoices as ::core::ffi::c_int) < of.numchn as ::core::ffi::c_int {
            of.numvoices = of.numchn;
        }
    }
    if AllocPatterns() == 0 {
        return 0 as BOOL;
    }
    if AllocTracks() == 0 {
        return 0 as BOOL;
    }
    t = 0 as ::core::ffi::c_int;
    while t < of.numpat as ::core::ffi::c_int {
        let mut packlen_0: UWORD = 0;
        if *paraptr
            .offset(
                ((*mh).insnum as ::core::ffi::c_int + (*mh).smpnum as ::core::ffi::c_int
                    + t) as isize,
            ) == 0
        {
            *of.pattrows.offset(t as isize) = 64 as UWORD;
            u = 0 as ::core::ffi::c_int;
            while u < of.numchn as ::core::ffi::c_int {
                let mut k: ::core::ffi::c_int = 0;
                UniReset();
                k = 0 as ::core::ffi::c_int;
                while k < 64 as ::core::ffi::c_int {
                    UniNewline();
                    k += 1;
                }
                let c2rust_fresh1 = numtrk;
                numtrk += 1;
                *of.tracks.offset(c2rust_fresh1 as isize) = UniDup();
                u += 1;
            }
        } else {
            (*modreader)
                .Seek
                .expect(
                    "non-null function pointer",
                )(
                modreader,
                *paraptr
                    .offset(
                        ((*mh).insnum as ::core::ffi::c_int
                            + (*mh).smpnum as ::core::ffi::c_int + t) as isize,
                    ) as ::core::ffi::c_long,
                0 as ::core::ffi::c_int,
            );
            packlen_0 = _mm_read_I_UWORD(modreader);
            *of.pattrows.offset(t as isize) = _mm_read_I_UWORD(modreader);
            _mm_read_I_ULONG(modreader);
            if IT_ReadPattern(*of.pattrows.offset(t as isize)) == 0 {
                return 0 as BOOL;
            }
        }
        t += 1;
    }
    return 1 as BOOL;
}
unsafe extern "C" fn IT_LoadTitle() -> *mut CHAR {
    let mut s: [CHAR; 26] = [0; 26];
    (*modreader)
        .Seek
        .expect(
            "non-null function pointer",
        )(modreader, 4 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    if (*modreader)
        .Read
        .expect(
            "non-null function pointer",
        )(modreader, &raw mut s as *mut CHAR as *mut ::core::ffi::c_void, 26 as size_t)
        == 0
    {
        return ::core::ptr::null_mut::<CHAR>();
    }
    return DupStr(&raw mut s as *mut CHAR, 26 as UWORD, 0 as BOOL);
}
#[no_mangle]
pub static mut load_it: MLOADER = MLOADER {
    next: ::core::ptr::null_mut::<MLOADER>(),
    r#type: b"IT\0".as_ptr() as *const CHAR,
    version: b"IT (Impulse Tracker)\0".as_ptr() as *const CHAR,
    Init: Some(IT_Init as unsafe extern "C" fn() -> BOOL),
    Test: Some(IT_Test as unsafe extern "C" fn() -> BOOL),
    Load: Some(IT_Load as unsafe extern "C" fn(BOOL) -> BOOL),
    Cleanup: Some(IT_Cleanup as unsafe extern "C" fn() -> ()),
    LoadTitle: Some(IT_LoadTitle as unsafe extern "C" fn() -> *mut CHAR),
};
#[inline]
unsafe extern "C" fn mik_islower(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c >= 'a' as ::core::ffi::c_int && c <= 'z' as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn mik_isupper(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c >= 'A' as ::core::ffi::c_int && c <= 'Z' as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn mik_isalpha(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (mik_islower(c) != 0 || mik_isupper(c) != 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn mik_isdigit(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c >= '0' as ::core::ffi::c_int && c <= '9' as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn mik_isalnum(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (mik_isalpha(c) != 0 || mik_isdigit(c) != 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn mik_toupper(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if mik_islower(c) != 0 {
        c & !('a' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int)
    } else {
        c
    };
}
