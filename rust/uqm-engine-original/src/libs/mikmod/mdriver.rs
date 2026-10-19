/*	MikMod sound library
	(c) 1998-2014 Miodrag Vallat and others - see file AUTHORS
	for a complete list.

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

  These routines are used to access the available soundcard drivers.

==============================================================================*/

// Rust translation of src/libs/mikmod/mdriver.c.
extern "C" {
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut pf: *mut MODULE;
    static mut _mm_errorhandler: MikMod_handler_t;
    fn SL_Init(_: *mut SAMPLOAD) -> BOOL;
    fn SL_Exit(_: *mut SAMPLOAD);
    fn Player_HandleTick();
    static mut MikMod_errno: ::core::ffi::c_int;
    static mut MikMod_critical: BOOL;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    static mut drv_nos: MDRIVER;
    fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    fn getuid() -> uid_t;
    fn geteuid() -> uid_t;
    fn getgid() -> gid_t;
    fn setuid(_: uid_t) -> ::core::ffi::c_int;
    fn getpwnam(_: *const ::core::ffi::c_char) -> *mut passwd;
}
pub type size_t = ::core::ffi::c_ulong;
pub type off_t = ::core::ffi::c_longlong;
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
pub type MikMod_player_t = Option<unsafe extern "C" fn() -> ()>;
pub type MikMod_callback_t = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_uchar, size_t) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const MD_MUSIC: Self = Self(0);
    pub const MD_SNDFX: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const MD_HARDWARE: Self = Self(0);
    pub const MD_SOFTWARE: Self = Self(1);
}
pub type mode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub st_ino: ino_t,
}
pub type ino_t = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type time_t = ::core::ffi::c_longlong;
pub type blkcnt_t = ::core::ffi::c_int;
pub type blksize_t = ::core::ffi::c_int;
pub type dev_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type uid_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
#[no_mangle]
pub static mut md_driver: *mut MDRIVER = ::core::ptr::null_mut::<MDRIVER>();
#[no_mangle]
pub static mut md_device: UWORD = 0 as UWORD;
#[no_mangle]
pub static mut md_mixfreq: UWORD = 44100 as UWORD;
#[no_mangle]
pub static mut md_mode: UWORD = (0x2 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int
    | 0x100 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int
    | 0x4 as ::core::ffi::c_int) as UWORD;
#[no_mangle]
pub static mut md_pansep: UBYTE = 128 as UBYTE;
#[no_mangle]
pub static mut md_reverb: UBYTE = 0 as UBYTE;
#[no_mangle]
pub static mut md_volume: UBYTE = 128 as UBYTE;
#[no_mangle]
pub static mut md_musicvolume: UBYTE = 128 as UBYTE;
#[no_mangle]
pub static mut md_sndfxvolume: UBYTE = 128 as UBYTE;
#[no_mangle]
pub static mut md_bpm: UWORD = 125 as UWORD;
#[no_mangle]
pub static mut md_sfxchn: UBYTE = 0 as UBYTE;
#[no_mangle]
pub static mut md_numchn: UBYTE = 0 as UBYTE;
#[no_mangle]
pub static mut md_sngchn: UBYTE = 0 as UBYTE;
#[no_mangle]
pub static mut md_softchn: UBYTE = 0 as UBYTE;
#[no_mangle]
pub static mut md_hardchn: UBYTE = 0 as UBYTE;
#[no_mangle]
pub static mut md_player: Option<unsafe extern "C" fn() -> ()> = Some(
    Player_HandleTick as unsafe extern "C" fn() -> (),
);
#[no_mangle]
pub static mut vc_callback: MikMod_callback_t = None;
static mut firstdriver: *mut MDRIVER = ::core::ptr::null_mut::<MDRIVER>();
static mut isplaying: BOOL = 0 as BOOL;
static mut initialized: BOOL = 0 as BOOL;
static mut sfxinfo: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
static mut sfxpool: ::core::ffi::c_int = 0;
static mut md_sample: *mut *mut SAMPLE = ::core::ptr::null_mut::<*mut SAMPLE>();
static mut olddevice: SWORD = -1 as SWORD;
unsafe extern "C" fn LimitHardVoices(mut limit: ::core::ffi::c_int) {
    let mut t: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
        && md_sfxchn as ::core::ffi::c_int > limit
    {
        md_sfxchn = limit as UBYTE;
    }
    if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
        && md_sngchn as ::core::ffi::c_int > limit
    {
        md_sngchn = limit as UBYTE;
    }
    if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 {
        md_hardchn = md_sfxchn;
    } else {
        md_hardchn = 0 as UBYTE;
    }
    if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 {
        md_hardchn = (md_hardchn as ::core::ffi::c_int + md_sngchn as ::core::ffi::c_int)
            as UBYTE;
    }
    while md_hardchn as ::core::ffi::c_int > limit {
        t += 1;
        if t & 1 as ::core::ffi::c_int != 0 {
            if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                && md_sfxchn as ::core::ffi::c_int > 4 as ::core::ffi::c_int
            {
                md_sfxchn = md_sfxchn.wrapping_sub(1);
            }
        } else if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
            && md_sngchn as ::core::ffi::c_int > 8 as ::core::ffi::c_int
        {
            md_sngchn = md_sngchn.wrapping_sub(1);
        }
        if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 {
            md_hardchn = md_sfxchn;
        } else {
            md_hardchn = 0 as UBYTE;
        }
        if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 {
            md_hardchn = (md_hardchn as ::core::ffi::c_int
                + md_sngchn as ::core::ffi::c_int) as UBYTE;
        }
    }
    md_numchn = (md_hardchn as ::core::ffi::c_int + md_softchn as ::core::ffi::c_int)
        as UBYTE;
}
unsafe extern "C" fn LimitSoftVoices(mut limit: ::core::ffi::c_int) {
    let mut t: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
        && md_sfxchn as ::core::ffi::c_int > limit
    {
        md_sfxchn = limit as UBYTE;
    }
    if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
        && md_sngchn as ::core::ffi::c_int > limit
    {
        md_sngchn = limit as UBYTE;
    }
    if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 {
        md_softchn = md_sfxchn;
    } else {
        md_softchn = 0 as UBYTE;
    }
    if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
        md_softchn = (md_softchn as ::core::ffi::c_int + md_sngchn as ::core::ffi::c_int)
            as UBYTE;
    }
    while md_softchn as ::core::ffi::c_int > limit {
        t += 1;
        if t & 1 as ::core::ffi::c_int != 0 {
            if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                && md_sfxchn as ::core::ffi::c_int > 4 as ::core::ffi::c_int
            {
                md_sfxchn = md_sfxchn.wrapping_sub(1);
            }
        } else if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
            && md_sngchn as ::core::ffi::c_int > 8 as ::core::ffi::c_int
        {
            md_sngchn = md_sngchn.wrapping_sub(1);
        }
        if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0 {
            md_softchn = md_sfxchn;
        } else {
            md_softchn = 0 as UBYTE;
        }
        if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0 {
            md_softchn = (md_softchn as ::core::ffi::c_int
                + md_sngchn as ::core::ffi::c_int) as UBYTE;
        }
    }
    md_numchn = (md_hardchn as ::core::ffi::c_int + md_softchn as ::core::ffi::c_int)
        as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn MD_SampleSpace(mut r#type: ::core::ffi::c_int) -> ULONG {
    if r#type == C2Rust_Unnamed_0::MD_MUSIC.0 as ::core::ffi::c_int {
        r#type = if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
            C2Rust_Unnamed_1::MD_SOFTWARE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_1::MD_HARDWARE.0 as ::core::ffi::c_int
        };
    } else if r#type == C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int {
        r#type = if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 {
            C2Rust_Unnamed_1::MD_SOFTWARE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_1::MD_HARDWARE.0 as ::core::ffi::c_int
        };
    }
    return (*md_driver).FreeSampleSpace.expect("non-null function pointer")(r#type);
}
#[no_mangle]
pub unsafe extern "C" fn MD_SampleLength(
    mut r#type: ::core::ffi::c_int,
    mut s: *mut SAMPLE,
) -> ULONG {
    if r#type == C2Rust_Unnamed_0::MD_MUSIC.0 as ::core::ffi::c_int {
        r#type = if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
            C2Rust_Unnamed_1::MD_SOFTWARE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_1::MD_HARDWARE.0 as ::core::ffi::c_int
        };
    } else if r#type == C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int {
        r#type = if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 {
            C2Rust_Unnamed_1::MD_SOFTWARE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_1::MD_HARDWARE.0 as ::core::ffi::c_int
        };
    }
    return (*md_driver).RealSampleLength.expect("non-null function pointer")(r#type, s);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_InfoDriver() -> *mut CHAR {
    let mut t: ::core::ffi::c_int = 0;
    let mut len: size_t = 0 as size_t;
    let mut l: *mut MDRIVER = ::core::ptr::null_mut::<MDRIVER>();
    let mut list: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    l = firstdriver;
    while !l.is_null() {
        len = len
            .wrapping_add(
                ((4 as ::core::ffi::c_int
                    + if !(*l).next.is_null() {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as size_t)
                    .wrapping_add(strlen((*l).Version)) as size_t,
            );
        l = (*l).next as *mut MDRIVER;
    }
    if len != 0 {
        list = MikMod_malloc(len.wrapping_mul(::core::mem::size_of::<CHAR>() as size_t))
            as *mut CHAR;
        if !list.is_null() {
            let mut list_end: *mut CHAR = list;
            *list.offset(0isize) = 0 as CHAR;
            t = 1 as ::core::ffi::c_int;
            l = firstdriver;
            while !l.is_null() {
                list_end = list_end
                    .offset(
                        sprintf(
                            list_end,
                            b"%2d %s%s\0".as_ptr() as *const ::core::ffi::c_char,
                            t,
                            (*l).Version,
                            if !(*l).next.is_null() {
                                b"\n\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        ) as isize,
                    );
                l = (*l).next as *mut MDRIVER;
                t += 1;
            }
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_registerdriver(mut drv: *mut MDRIVER) {
    let mut cruise: *mut MDRIVER = firstdriver;
    if !(*drv).Name.is_null() && !(*drv).Version.is_null() {
        if !cruise.is_null() {
            if cruise == drv {
                return;
            }
            while !(*cruise).next.is_null() {
                cruise = (*cruise).next as *mut MDRIVER;
                if cruise == drv {
                    return;
                }
            }
            (*cruise).next = drv;
        } else {
            firstdriver = drv as *mut MDRIVER;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_RegisterDriver(mut drv: *mut MDRIVER) {
    if drv.is_null() || !(*drv).next.is_null() || (*drv).Name.is_null() {
        return;
    }
    _mm_registerdriver(drv);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_DriverFromAlias(
    mut alias: *const CHAR,
) -> ::core::ffi::c_int {
    let mut rank: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut cruise: *mut MDRIVER = ::core::ptr::null_mut::<MDRIVER>();
    cruise = firstdriver;
    while !cruise.is_null() {
        if !(*cruise).Alias.is_null() {
            if strcasecmp(alias, (*cruise).Alias) == 0 {
                break;
            }
            rank += 1;
        }
        cruise = (*cruise).next as *mut MDRIVER;
    }
    if cruise.is_null() {
        rank = 0 as ::core::ffi::c_int;
    }
    return rank;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_DriverByOrdinal(
    mut ordinal: ::core::ffi::c_int,
) -> *mut MDRIVER {
    let mut cruise: *mut MDRIVER = ::core::ptr::null_mut::<MDRIVER>();
    if ordinal == 0 {
        return ::core::ptr::null_mut::<MDRIVER>();
    }
    cruise = firstdriver;
    while !cruise.is_null()
        && {
            ordinal -= 1;
            ordinal != 0
        }
    {
        cruise = (*cruise).next as *mut MDRIVER;
    }
    return cruise;
}
#[no_mangle]
pub unsafe extern "C" fn MD_SampleLoad(
    mut s: *mut SAMPLOAD,
    mut r#type: ::core::ffi::c_int,
) -> SWORD {
    let mut result: SWORD = 0;
    if r#type == C2Rust_Unnamed_0::MD_MUSIC.0 as ::core::ffi::c_int {
        r#type = if md_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
            C2Rust_Unnamed_1::MD_SOFTWARE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_1::MD_HARDWARE.0 as ::core::ffi::c_int
        };
    } else if r#type == C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int {
        r#type = if md_mode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 {
            C2Rust_Unnamed_1::MD_SOFTWARE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_1::MD_HARDWARE.0 as ::core::ffi::c_int
        };
    }
    SL_Init(s);
    result = (*md_driver).SampleLoad.expect("non-null function pointer")(s, r#type);
    SL_Exit(s);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MD_SampleUnload(mut handle: SWORD) {
    (*md_driver).SampleUnload.expect("non-null function pointer")(handle);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_RegisterPlayer(
    mut player: MikMod_player_t,
) -> MikMod_player_t {
    let mut result: MikMod_player_t = None;
    result = md_player as MikMod_player_t;
    md_player = player as Option<unsafe extern "C" fn() -> ()>;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Update() {
    if ::core::ptr::read_volatile::<BOOL>(&raw const isplaying) != 0 {
        if pf.is_null() || (*pf).forbid == 0 {
            (*md_driver).Update.expect("non-null function pointer")();
        } else if (*md_driver).Pause.is_some() {
            (*md_driver).Pause.expect("non-null function pointer")();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Voice_SetVolume_internal(mut voice: SBYTE, mut vol: UWORD) {
    let mut tmp: ULONG = 0;
    if (voice as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || voice as ::core::ffi::c_int >= md_numchn as ::core::ffi::c_int
    {
        return;
    }
    if md_musicvolume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        md_musicvolume = 128 as UBYTE;
    }
    if md_sndfxvolume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        md_sndfxvolume = 128 as UBYTE;
    }
    if md_volume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        md_volume = 128 as UBYTE;
    }
    tmp = (vol as ULONG)
        .wrapping_mul(md_volume as ULONG)
        .wrapping_mul(
            if (voice as ::core::ffi::c_int) < md_sngchn as ::core::ffi::c_int {
                md_musicvolume as ULONG
            } else {
                md_sndfxvolume as ULONG
            },
        );
    (*md_driver)
        .VoiceSetVolume
        .expect(
            "non-null function pointer",
        )(voice as UBYTE, tmp.wrapping_div(16384 as ::core::ffi::c_ulong) as UWORD);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_SetVolume(mut voice: SBYTE, mut vol: UWORD) {
    Voice_SetVolume_internal(voice, vol);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_GetVolume(mut voice: SBYTE) -> UWORD {
    let mut result: UWORD = 0 as UWORD;
    if voice as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (voice as ::core::ffi::c_int) < md_numchn as ::core::ffi::c_int
    {
        result = (*md_driver)
            .VoiceGetVolume
            .expect("non-null function pointer")(voice as UBYTE);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Voice_SetFrequency_internal(mut voice: SBYTE, mut frq: ULONG) {
    if (voice as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || voice as ::core::ffi::c_int >= md_numchn as ::core::ffi::c_int
    {
        return;
    }
    if !(*md_sample.offset(voice as isize)).is_null()
        && (**md_sample.offset(voice as isize)).divfactor as ::core::ffi::c_int != 0
    {
        frq = frq.wrapping_div((**md_sample.offset(voice as isize)).divfactor as ULONG);
    }
    (*md_driver)
        .VoiceSetFrequency
        .expect("non-null function pointer")(voice as UBYTE, frq);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_SetFrequency(mut voice: SBYTE, mut frq: ULONG) {
    Voice_SetFrequency_internal(voice, frq);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_GetFrequency(mut voice: SBYTE) -> ULONG {
    let mut result: ULONG = 0 as ULONG;
    if voice as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (voice as ::core::ffi::c_int) < md_numchn as ::core::ffi::c_int
    {
        result = (*md_driver)
            .VoiceGetFrequency
            .expect("non-null function pointer")(voice as UBYTE);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Voice_SetPanning_internal(mut voice: SBYTE, mut pan: ULONG) {
    if (voice as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || voice as ::core::ffi::c_int >= md_numchn as ::core::ffi::c_int
    {
        return;
    }
    if pan != 512 as ULONG {
        if md_pansep as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
            md_pansep = 128 as UBYTE;
        }
        if md_mode as ::core::ffi::c_int & 0x400 as ::core::ffi::c_int != 0 {
            pan = (255 as ULONG).wrapping_sub(pan);
        }
        pan = (pan.wrapping_sub(128 as ULONG) as SWORD as ::core::ffi::c_int
            * md_pansep as ::core::ffi::c_int / 128 as ::core::ffi::c_int
            + 128 as ::core::ffi::c_int) as ULONG;
    }
    (*md_driver)
        .VoiceSetPanning
        .expect("non-null function pointer")(voice as UBYTE, pan);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_SetPanning(mut voice: SBYTE, mut pan: ULONG) {
    Voice_SetPanning_internal(voice, pan);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_GetPanning(mut voice: SBYTE) -> ULONG {
    let mut result: ULONG = 128 as ULONG;
    if voice as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (voice as ::core::ffi::c_int) < md_numchn as ::core::ffi::c_int
    {
        result = (*md_driver)
            .VoiceGetPanning
            .expect("non-null function pointer")(voice as UBYTE);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Voice_Play_internal(
    mut voice: SBYTE,
    mut s: *mut SAMPLE,
    mut start: ULONG,
) {
    let mut repend: ULONG = 0;
    if (voice as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || voice as ::core::ffi::c_int >= md_numchn as ::core::ffi::c_int
    {
        return;
    }
    *md_sample.offset(voice as isize) = s;
    repend = (*s).loopend;
    if (*s).flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0 {
        if repend > (*s).length {
            repend = (*s).length;
        }
    }
    (*md_driver)
        .VoicePlay
        .expect(
            "non-null function pointer",
        )(
        voice as UBYTE,
        (*s).handle,
        start,
        (*s).length,
        (*s).loopstart,
        repend,
        (*s).flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Voice_Play(
    mut voice: SBYTE,
    mut s: *mut SAMPLE,
    mut start: ULONG,
) {
    if start > (*s).length {
        return;
    }
    Voice_Play_internal(voice, s, start);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_Stop_internal(mut voice: SBYTE) {
    if (voice as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || voice as ::core::ffi::c_int >= md_numchn as ::core::ffi::c_int
    {
        return;
    }
    if voice as ::core::ffi::c_int >= md_sngchn as ::core::ffi::c_int {
        *sfxinfo
            .offset(
                (voice as ::core::ffi::c_int - md_sngchn as ::core::ffi::c_int) as isize,
            ) = 0 as UBYTE;
    }
    (*md_driver).VoiceStop.expect("non-null function pointer")(voice as UBYTE);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_Stop(mut voice: SBYTE) {
    Voice_Stop_internal(voice);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_Stopped_internal(mut voice: SBYTE) -> BOOL {
    if (voice as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || voice as ::core::ffi::c_int >= md_numchn as ::core::ffi::c_int
    {
        return 0 as BOOL;
    }
    return (*md_driver).VoiceStopped.expect("non-null function pointer")(voice as UBYTE);
}
#[no_mangle]
pub unsafe extern "C" fn Voice_Stopped(mut voice: SBYTE) -> BOOL {
    let mut result: BOOL = 0;
    result = Voice_Stopped_internal(voice);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Voice_GetPosition(mut voice: SBYTE) -> SLONG {
    let mut result: SLONG = 0 as SLONG;
    if voice as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (voice as ::core::ffi::c_int) < md_numchn as ::core::ffi::c_int
    {
        if (*md_driver).VoiceGetPosition.is_some() {
            result = (*md_driver)
                .VoiceGetPosition
                .expect("non-null function pointer")(voice as UBYTE);
        } else {
            result = -1 as SLONG;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Voice_RealVolume(mut voice: SBYTE) -> ULONG {
    let mut result: ULONG = 0 as ULONG;
    if voice as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (voice as ::core::ffi::c_int) < md_numchn as ::core::ffi::c_int
        && (*md_driver).VoiceRealVolume.is_some()
    {
        result = (*md_driver)
            .VoiceRealVolume
            .expect("non-null function pointer")(voice as UBYTE);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn VC_SetCallback(mut callback: MikMod_callback_t) {
    vc_callback = callback;
}
unsafe extern "C" fn _mm_init(mut cmdline: *const CHAR) -> ::core::ffi::c_int {
    let mut t: UWORD = 0;
    MikMod_critical = 1 as ::core::ffi::c_int as BOOL;
    if md_device == 0 {
        cmdline = ::core::ptr::null::<CHAR>();
        t = 1 as UWORD;
        md_driver = firstdriver;
        while !md_driver.is_null() {
            if (*md_driver).IsPresent.expect("non-null function pointer")() != 0 {
                break;
            }
            md_driver = (*md_driver).next as *mut MDRIVER;
            t = t.wrapping_add(1);
        }
        if md_driver.is_null() {
            MikMod_errno = C2Rust_Unnamed::MMERR_DETECTING_DEVICE.0
                as ::core::ffi::c_int;
            if _mm_errorhandler.is_some() {
                _mm_errorhandler.expect("non-null function pointer")();
            }
            md_driver = &raw mut drv_nos as *mut MDRIVER;
            return 1 as ::core::ffi::c_int;
        }
        md_device = t;
    } else {
        t = 1 as UWORD;
        md_driver = firstdriver;
        while !md_driver.is_null()
            && t as ::core::ffi::c_int != md_device as ::core::ffi::c_int
        {
            t = t.wrapping_add(1);
            md_driver = (*md_driver).next as *mut MDRIVER;
        }
        if md_driver.is_null() {
            MikMod_errno = C2Rust_Unnamed::MMERR_INVALID_DEVICE.0 as ::core::ffi::c_int;
            if _mm_errorhandler.is_some() {
                _mm_errorhandler.expect("non-null function pointer")();
            }
            md_driver = &raw mut drv_nos as *mut MDRIVER;
            return 1 as ::core::ffi::c_int;
        }
        if !cmdline.is_null() && (*md_driver).CommandLine.is_some() {
            (*md_driver).CommandLine.expect("non-null function pointer")(cmdline);
        }
        if (*md_driver).IsPresent.expect("non-null function pointer")() == 0 {
            MikMod_errno = C2Rust_Unnamed::MMERR_DETECTING_DEVICE.0
                as ::core::ffi::c_int;
            if _mm_errorhandler.is_some() {
                _mm_errorhandler.expect("non-null function pointer")();
            }
            md_driver = &raw mut drv_nos as *mut MDRIVER;
            return 1 as ::core::ffi::c_int;
        }
    }
    olddevice = md_device as SWORD;
    if (*md_driver).Init.expect("non-null function pointer")() != 0 {
        MikMod_Exit_internal();
        if _mm_errorhandler.is_some() {
            _mm_errorhandler.expect("non-null function pointer")();
        }
        return 1 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut initialized, 1 as ::core::ffi::c_int as BOOL);
    MikMod_critical = 0 as ::core::ffi::c_int as BOOL;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Init(mut cmdline: *const CHAR) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = _mm_init(cmdline);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Exit_internal() {
    MikMod_DisableOutput_internal();
    (*md_driver).Exit.expect("non-null function pointer")();
    md_sngchn = 0 as UBYTE;
    md_sfxchn = md_sngchn;
    md_numchn = md_sfxchn;
    md_driver = &raw mut drv_nos as *mut MDRIVER;
    MikMod_free(sfxinfo as *mut ::core::ffi::c_void);
    MikMod_free(md_sample as *mut ::core::ffi::c_void);
    md_sample = ::core::ptr::null_mut::<*mut SAMPLE>();
    sfxinfo = ::core::ptr::null_mut::<UBYTE>();
    ::core::ptr::write_volatile(&raw mut initialized, 0 as ::core::ffi::c_int as BOOL);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Exit() {
    MikMod_Exit_internal();
}
unsafe extern "C" fn _mm_reset(mut cmdline: *const CHAR) -> ::core::ffi::c_int {
    let mut wasplaying: BOOL = 0 as BOOL;
    if ::core::ptr::read_volatile::<BOOL>(&raw const initialized) == 0 {
        return _mm_init(cmdline);
    }
    if ::core::ptr::read_volatile::<BOOL>(&raw const isplaying) != 0 {
        wasplaying = 1 as ::core::ffi::c_int as BOOL;
        (*md_driver).PlayStop.expect("non-null function pointer")();
    }
    if (*md_driver).Reset.is_none()
        || md_device as ::core::ffi::c_int != olddevice as ::core::ffi::c_int
    {
        (*md_driver).Exit.expect("non-null function pointer")();
        if _mm_init(cmdline) != 0 {
            MikMod_Exit_internal();
            if MikMod_errno != 0 {
                if _mm_errorhandler.is_some() {
                    _mm_errorhandler.expect("non-null function pointer")();
                }
            }
            return 1 as ::core::ffi::c_int;
        }
    } else if (*md_driver).Reset.expect("non-null function pointer")() != 0 {
        MikMod_Exit_internal();
        if MikMod_errno != 0 {
            if _mm_errorhandler.is_some() {
                _mm_errorhandler.expect("non-null function pointer")();
            }
        }
        return 1 as ::core::ffi::c_int;
    }
    if wasplaying != 0 {
        return (*md_driver).PlayStart.expect("non-null function pointer")();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Reset(mut cmdline: *const CHAR) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = _mm_reset(cmdline);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_SetNumVoices_internal(
    mut music: ::core::ffi::c_int,
    mut sfx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut resume: BOOL = 0 as BOOL;
    let mut t: ::core::ffi::c_int = 0;
    let mut oldchn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if music == 0 && sfx == 0 {
        return 1 as ::core::ffi::c_int;
    }
    MikMod_critical = 1 as ::core::ffi::c_int as BOOL;
    if ::core::ptr::read_volatile::<BOOL>(&raw const isplaying) != 0 {
        MikMod_DisableOutput_internal();
        oldchn = md_numchn as ::core::ffi::c_int;
        resume = 1 as ::core::ffi::c_int as BOOL;
    }
    MikMod_free(sfxinfo as *mut ::core::ffi::c_void);
    MikMod_free(md_sample as *mut ::core::ffi::c_void);
    md_sample = ::core::ptr::null_mut::<*mut SAMPLE>();
    sfxinfo = ::core::ptr::null_mut::<UBYTE>();
    if music != -1 as ::core::ffi::c_int {
        md_sngchn = music as UBYTE;
    }
    if sfx != -1 as ::core::ffi::c_int {
        md_sfxchn = sfx as UBYTE;
    }
    md_numchn = (md_sngchn as ::core::ffi::c_int + md_sfxchn as ::core::ffi::c_int)
        as UBYTE;
    LimitHardVoices((*md_driver).HardVoiceLimit as ::core::ffi::c_int);
    LimitSoftVoices((*md_driver).SoftVoiceLimit as ::core::ffi::c_int);
    if (*md_driver).SetNumVoices.expect("non-null function pointer")() != 0 {
        MikMod_Exit_internal();
        if MikMod_errno != 0 {
            if _mm_errorhandler.is_some() {
                _mm_errorhandler.expect("non-null function pointer")();
            }
        }
        md_sngchn = 0 as UBYTE;
        md_sfxchn = md_sngchn;
        md_hardchn = md_sfxchn;
        md_softchn = md_hardchn;
        md_numchn = md_softchn;
        return 1 as ::core::ffi::c_int;
    }
    if md_sngchn as ::core::ffi::c_int + md_sfxchn as ::core::ffi::c_int != 0 {
        md_sample = MikMod_calloc(
            (md_sngchn as ::core::ffi::c_int + md_sfxchn as ::core::ffi::c_int)
                as size_t,
            ::core::mem::size_of::<*mut SAMPLE>() as size_t,
        ) as *mut *mut SAMPLE;
    }
    if md_sfxchn != 0 {
        sfxinfo = MikMod_calloc(
            md_sfxchn as size_t,
            ::core::mem::size_of::<UBYTE>() as size_t,
        ) as *mut UBYTE;
    }
    t = oldchn;
    while t < md_numchn as ::core::ffi::c_int {
        Voice_Stop_internal(t as SBYTE);
        t += 1;
    }
    sfxpool = 0 as ::core::ffi::c_int;
    if resume != 0 {
        MikMod_EnableOutput_internal();
    }
    MikMod_critical = 0 as ::core::ffi::c_int as BOOL;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_SetNumVoices(
    mut music: ::core::ffi::c_int,
    mut sfx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = MikMod_SetNumVoices_internal(music, sfx);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_EnableOutput_internal() -> ::core::ffi::c_int {
    MikMod_critical = 1 as ::core::ffi::c_int as BOOL;
    if ::core::ptr::read_volatile::<BOOL>(&raw const isplaying) == 0 {
        if (*md_driver).PlayStart.expect("non-null function pointer")() != 0 {
            return 1 as ::core::ffi::c_int;
        }
        ::core::ptr::write_volatile(&raw mut isplaying, 1 as ::core::ffi::c_int as BOOL);
    }
    MikMod_critical = 0 as ::core::ffi::c_int as BOOL;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_EnableOutput() -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = MikMod_EnableOutput_internal();
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_DisableOutput_internal() {
    if ::core::ptr::read_volatile::<BOOL>(&raw const isplaying) != 0
        && !md_driver.is_null()
    {
        ::core::ptr::write_volatile(&raw mut isplaying, 0 as ::core::ffi::c_int as BOOL);
        (*md_driver).PlayStop.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_DisableOutput() {
    MikMod_DisableOutput_internal();
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Active_internal() -> BOOL {
    return ::core::ptr::read_volatile::<BOOL>(&raw const isplaying);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Active() -> BOOL {
    let mut result: BOOL = 0;
    result = MikMod_Active_internal();
    return result;
}
unsafe extern "C" fn Sample_Play_internal(
    mut s: *mut SAMPLE,
    mut start: ULONG,
    mut flags: UBYTE,
) -> SBYTE {
    let mut orig: ::core::ffi::c_int = sfxpool;
    let mut c: ::core::ffi::c_int = 0;
    if md_sfxchn == 0 {
        return -1 as SBYTE;
    }
    if (*s).volume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
        (*s).volume = 64 as UBYTE;
    }
    loop {
        if *sfxinfo.offset(sfxpool as isize) as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int != 0
        {
            c = sfxpool + md_sngchn as ::core::ffi::c_int;
            if (*md_driver).VoiceStopped.expect("non-null function pointer")(c as UBYTE)
                != 0
            {
                *sfxinfo.offset(sfxpool as isize) = flags;
                Voice_Play_internal(c as SBYTE, s, start);
                (*md_driver)
                    .VoiceSetVolume
                    .expect(
                        "non-null function pointer",
                    )(
                    c as UBYTE,
                    (((*s).volume as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as UWORD,
                );
                Voice_SetPanning_internal(c as SBYTE, (*s).panning as ULONG);
                (*md_driver)
                    .VoiceSetFrequency
                    .expect("non-null function pointer")(c as UBYTE, (*s).speed);
                sfxpool += 1;
                if sfxpool >= md_sfxchn as ::core::ffi::c_int {
                    sfxpool = 0 as ::core::ffi::c_int;
                }
                return c as SBYTE;
            }
        } else {
            *sfxinfo.offset(sfxpool as isize) = flags;
            c = sfxpool + md_sngchn as ::core::ffi::c_int;
            Voice_Play_internal(c as SBYTE, s, start);
            (*md_driver)
                .VoiceSetVolume
                .expect(
                    "non-null function pointer",
                )(
                c as UBYTE,
                (((*s).volume as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UWORD,
            );
            Voice_SetPanning_internal(c as SBYTE, (*s).panning as ULONG);
            (*md_driver)
                .VoiceSetFrequency
                .expect("non-null function pointer")(c as UBYTE, (*s).speed);
            sfxpool += 1;
            if sfxpool >= md_sfxchn as ::core::ffi::c_int {
                sfxpool = 0 as ::core::ffi::c_int;
            }
            return c as SBYTE;
        }
        sfxpool += 1;
        if sfxpool >= md_sfxchn as ::core::ffi::c_int {
            sfxpool = 0 as ::core::ffi::c_int;
        }
        if sfxpool == orig {
            break;
        }
    }
    return -1 as SBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn Sample_Play(
    mut s: *mut SAMPLE,
    mut start: ULONG,
    mut flags: UBYTE,
) -> SBYTE {
    let mut result: SBYTE = 0;
    result = Sample_Play_internal(s, start, flags);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_GetVersion() -> ::core::ffi::c_long {
    return (3 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int
        | (3 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int
        | 11 as ::core::ffi::c_long;
}
#[no_mangle]
pub static mut _mm_mutex_vars: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub static mut _mm_mutex_lists: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn MikMod_InitThreads() -> BOOL {
    static mut firstcall: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    static mut result: BOOL = 0 as BOOL;
    if firstcall != 0 {
        firstcall = 0 as ::core::ffi::c_int;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Unlock() {}
#[no_mangle]
pub unsafe extern "C" fn MikMod_Lock() {}
#[no_mangle]
pub unsafe extern "C" fn MD_GetAtom(
    mut atomname: *const CHAR,
    mut cmdline: *const CHAR,
    mut implicit: BOOL,
) -> *mut CHAR {
    let mut ret: *mut CHAR = ::core::ptr::null_mut::<CHAR>();
    if !cmdline.is_null() {
        let mut buf: *const CHAR = strstr(cmdline, atomname);
        if !buf.is_null()
            && (buf == cmdline
                || *buf.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == ',' as ::core::ffi::c_int)
        {
            let mut ptr: *const CHAR = buf.offset(strlen(atomname) as isize);
            if *ptr as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                ptr = ptr.offset(1);
                buf = ptr;
                while *ptr as ::core::ffi::c_int != 0
                    && *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1);
                }
                ret = MikMod_malloc(
                    (ptr.offset(1 as ::core::ffi::c_int as isize).offset_from(buf)
                        as size_t)
                        .wrapping_mul(::core::mem::size_of::<CHAR>() as size_t),
                ) as *mut CHAR;
                if !ret.is_null() {
                    strncpy(ret, buf, ptr.offset_from(buf) as size_t);
                }
            } else if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || *ptr == 0
            {
                if implicit != 0 {
                    ret = MikMod_malloc(
                        (ptr.offset(1 as ::core::ffi::c_int as isize).offset_from(buf)
                            as size_t)
                            .wrapping_mul(::core::mem::size_of::<CHAR>() as size_t),
                    ) as *mut CHAR;
                    if !ret.is_null() {
                        strncpy(ret, buf, ptr.offset_from(buf) as size_t);
                    }
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn MD_Access(mut filename: *const CHAR) -> BOOL {
    let mut buf: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ino: 0,
    };
    if stat(filename, &raw mut buf) == 0 {
        if !(buf.st_mode & 0o170000 as mode_t == 0o100000 as mode_t) {
            return 0 as BOOL;
        }
        if buf.st_nlink > 1 as nlink_t {
            return 0 as BOOL;
        }
        if getuid() == buf.st_uid {
            if buf.st_mode & 0o200 as mode_t == 0 {
                return 0 as BOOL;
            }
        } else if getgid() == buf.st_gid {
            if buf.st_mode & 0o20 as mode_t == 0 {
                return 0 as BOOL;
            }
        } else if buf.st_mode & 0o2 as mode_t == 0 {
            return 0 as BOOL
        }
    }
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn MD_DropPrivileges() -> ::core::ffi::c_int {
    if geteuid() == 0 {
        if getuid() != 0 {
            if setuid(getuid()) != 0 {
                return 1 as ::core::ffi::c_int;
            }
        } else {
            let mut nobody: *mut passwd = ::core::ptr::null_mut::<passwd>();
            let mut uid: ::core::ffi::c_int = 0;
            nobody = getpwnam(b"nobody\0".as_ptr() as *const ::core::ffi::c_char);
            if nobody.is_null() {
                return 1 as ::core::ffi::c_int;
            }
            uid = (*nobody).pw_uid as ::core::ffi::c_int;
            if uid == 0 {
                return 1 as ::core::ffi::c_int;
            }
            if setuid(uid as uid_t) != 0 {
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
