/*	MikMod sound library
	(c) 1998, 1999, 2000 Miodrag Vallat and others - see file AUTHORS for
	complete list.

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

  High-quality sample mixing routines, using a 32 bits mixing buffer,
  interpolation, and sample smoothing to improve sound quality and remove
  clicks.

==============================================================================*/

/*

  Future Additions:
	Low-Pass filter to remove annoying staticy buzz.

*/

// Rust translation of src/libs/mikmod/virtch2.c.
extern "C" {
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut MikMod_errno: ::core::ffi::c_int;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    static mut md_reverb: UBYTE;
    static mut md_mixfreq: UWORD;
    static mut md_mode: UWORD;
    fn SL_Sample8to16(_: *mut SAMPLOAD);
    fn SL_SampleSigned(_: *mut SAMPLOAD);
    fn SL_Load(
        _: *mut ::core::ffi::c_void,
        _: *mut SAMPLOAD,
        _: ULONG,
    ) -> ::core::ffi::c_int;
    static mut md_bpm: UWORD;
    static mut md_softchn: UBYTE;
    static mut md_player: Option<unsafe extern "C" fn() -> ()>;
    fn VC_SetupPointers();
    fn VC1_Init() -> ::core::ffi::c_int;
    static mut vc_callback: MikMod_callback_t;
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type MikMod_callback_t = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_uchar, size_t) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const MD_HARDWARE: Self = Self(0);
    pub const MD_SOFTWARE: Self = Self(1);
}
pub type SLONGLONG = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VINFO {
    pub kick: UBYTE,
    pub active: UBYTE,
    pub flags: UWORD,
    pub handle: SWORD,
    pub start: ULONG,
    pub size: ULONG,
    pub reppos: ULONG,
    pub repend: ULONG,
    pub frq: ULONG,
    pub vol: ::core::ffi::c_int,
    pub pan: ::core::ffi::c_int,
    pub click: ::core::ffi::c_int,
    pub rampvol: ::core::ffi::c_int,
    pub lastvalL: SLONG,
    pub lastvalR: SLONG,
    pub lvolsel: ::core::ffi::c_int,
    pub rvolsel: ::core::ffi::c_int,
    pub oldlvol: ::core::ffi::c_int,
    pub oldrvol: ::core::ffi::c_int,
    pub current: SLONGLONG,
    pub increment: SLONGLONG,
}
static mut Samples: *mut *mut SWORD = ::core::ptr::null_mut::<*mut SWORD>();
static mut vnf: *mut VINFO = ::core::ptr::null_mut::<VINFO>();
static mut vinf: *mut VINFO = ::core::ptr::null_mut::<VINFO>();
static mut vc_memory: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
static mut tickleft: ::core::ffi::c_long = 0;
static mut samplesthatfit: ::core::ffi::c_long = 0;
static mut vc_softchn: ::core::ffi::c_int = 0;
static mut idxsize: SLONGLONG = 0;
static mut idxlpos: SLONGLONG = 0;
static mut idxlend: SLONGLONG = 0;
static mut vc_tickbuf: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut vc_mode: UWORD = 0;
static mut RVc7: ::core::ffi::c_int = 0;
static mut RVc8: ::core::ffi::c_int = 0;
static mut RVc6: ::core::ffi::c_int = 0;
static mut RVc5: ::core::ffi::c_int = 0;
static mut RVc3: ::core::ffi::c_int = 0;
static mut RVc2: ::core::ffi::c_int = 0;
static mut RVc1: ::core::ffi::c_int = 0;
static mut RVc4: ::core::ffi::c_int = 0;
static mut RVRindex: ULONG = 0;
static mut RVbufL6: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL7: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL8: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL4: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL5: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL3: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL2: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufL1: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR3: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR1: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR5: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR6: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR8: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR7: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR4: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
static mut RVbufR2: *mut SLONG = ::core::ptr::null_mut::<SLONG>();
unsafe extern "C" fn Mix32MonoNormal(
    srce: *const SWORD,
    mut dest: *mut SLONG,
    mut idx: SLONG,
    mut increment: SLONG,
    mut todo: SLONG,
) -> SLONG {
    let mut sample: SWORD = 0 as SWORD;
    let mut i: SLONG = 0;
    let mut f: SLONG = 0;
    loop {
        let c2rust_fresh32 = todo;
        todo -= 1;
        if c2rust_fresh32 == 0 {
            break;
        }
        i = idx >> 28 as ::core::ffi::c_int;
        f = (idx as ::core::ffi::c_ulong
            & ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong)) as SLONG;
        sample = ((*srce.offset(i as isize) as ::core::ffi::c_ulong)
            .wrapping_mul(
                ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_sub(f as ::core::ffi::c_ulong),
            ) as SLONG + *srce.offset((i + 1 as SLONG) as isize) as SLONG * f
            >> 28 as ::core::ffi::c_int) as SWORD;
        idx += increment;
        if (*vnf).rampvol != 0 {
            let c2rust_fresh33 = dest;
            dest = dest.offset(1);
            *c2rust_fresh33
                += (((*vnf).oldlvol * (*vnf).rampvol) as SLONG
                    + (*vnf).lvolsel as SLONG
                        * (((1 as SLONG)
                            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            - (*vnf).rampvol as SLONG)) * sample as SLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            (*vnf).rampvol -= 1;
        } else if (*vnf).click != 0 {
            let c2rust_fresh34 = dest;
            dest = dest.offset(1);
            *c2rust_fresh34
                += (*vnf).lvolsel as SLONG
                    * (((1 as SLONG)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).click as SLONG) * sample as SLONG
                    + (*vnf).lastvalL * (*vnf).click as SLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            (*vnf).click -= 1;
        } else {
            let c2rust_fresh35 = dest;
            dest = dest.offset(1);
            *c2rust_fresh35 += ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
        }
    }
    (*vnf).lastvalL = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    return idx;
}
unsafe extern "C" fn Mix32StereoNormal(
    srce: *const SWORD,
    mut dest: *mut SLONG,
    mut idx: SLONG,
    mut increment: SLONG,
    mut todo: ULONG,
) -> SLONG {
    let mut sample: SWORD = 0 as SWORD;
    let mut i: SLONG = 0;
    let mut f: SLONG = 0;
    loop {
        let c2rust_fresh36 = todo;
        todo = todo.wrapping_sub(1);
        if c2rust_fresh36 == 0 {
            break;
        }
        i = idx >> 28 as ::core::ffi::c_int;
        f = (idx as ::core::ffi::c_ulong
            & ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong)) as SLONG;
        sample = ((*srce.offset(i as isize) as SLONG as ::core::ffi::c_ulong)
            .wrapping_mul(
                ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_sub(f as ::core::ffi::c_ulong),
            )
            .wrapping_add(
                (*srce.offset((i + 1 as SLONG) as isize) as SLONG * f)
                    as ::core::ffi::c_ulong,
            ) >> 28 as ::core::ffi::c_int) as SWORD;
        idx += increment;
        if (*vnf).rampvol != 0 {
            let c2rust_fresh37 = dest;
            dest = dest.offset(1);
            *c2rust_fresh37
                += ((*vnf).oldlvol as SLONG * (*vnf).rampvol as SLONG
                    + (*vnf).lvolsel as SLONG
                        * (((1 as SLONG)
                            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            - (*vnf).rampvol as SLONG)) * sample as SLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            let c2rust_fresh38 = dest;
            dest = dest.offset(1);
            *c2rust_fresh38
                += ((*vnf).oldrvol as SLONG * (*vnf).rampvol as SLONG
                    + (*vnf).rvolsel as SLONG
                        * (((1 as SLONG)
                            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            - (*vnf).rampvol as SLONG)) * sample as SLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            (*vnf).rampvol -= 1;
        } else if (*vnf).click != 0 {
            let c2rust_fresh39 = dest;
            dest = dest.offset(1);
            *c2rust_fresh39
                += (*vnf).lvolsel as ::core::ffi::c_long
                    * (((1 as ::core::ffi::c_long)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).click as ::core::ffi::c_long) * sample as SLONG
                    + (*vnf).lastvalL * (*vnf).click as SLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            let c2rust_fresh40 = dest;
            dest = dest.offset(1);
            *c2rust_fresh40
                += (*vnf).rvolsel as SLONG
                    * (((1 as SLONG)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).click as SLONG) * sample as SLONG
                    + (*vnf).lastvalR * (*vnf).click as SLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            (*vnf).click -= 1;
        } else {
            let c2rust_fresh41 = dest;
            dest = dest.offset(1);
            *c2rust_fresh41 += ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh42 = dest;
            dest = dest.offset(1);
            *c2rust_fresh42 += ((*vnf).rvolsel * sample as ::core::ffi::c_int) as SLONG;
        }
    }
    (*vnf).lastvalL = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    (*vnf).lastvalR = ((*vnf).rvolsel * sample as ::core::ffi::c_int) as SLONG;
    return idx;
}
unsafe extern "C" fn Mix32StereoSurround(
    srce: *const SWORD,
    mut dest: *mut SLONG,
    mut idx: SLONG,
    mut increment: SLONG,
    mut todo: ULONG,
) -> SLONG {
    let mut sample: SWORD = 0 as SWORD;
    let mut whoop: ::core::ffi::c_long = 0;
    let mut i: SLONG = 0;
    let mut f: SLONG = 0;
    loop {
        let c2rust_fresh43 = todo;
        todo = todo.wrapping_sub(1);
        if c2rust_fresh43 == 0 {
            break;
        }
        i = idx >> 28 as ::core::ffi::c_int;
        f = (idx as ::core::ffi::c_ulong
            & ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong)) as SLONG;
        sample = ((*srce.offset(i as isize) as SLONG as ::core::ffi::c_ulong)
            .wrapping_mul(
                ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_sub(f as ::core::ffi::c_ulong),
            )
            .wrapping_add(
                (*srce.offset((i + 1 as SLONG) as isize) as SLONG * f)
                    as ::core::ffi::c_ulong,
            ) >> 28 as ::core::ffi::c_int) as SWORD;
        idx += increment;
        if (*vnf).rampvol != 0 {
            whoop = (((*vnf).oldlvol * (*vnf).rampvol) as SLONG
                + (*vnf).lvolsel as SLONG
                    * (((1 as SLONG)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).rampvol as SLONG)) * sample as SLONG
                >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            let c2rust_fresh44 = dest;
            dest = dest.offset(1);
            *c2rust_fresh44 += whoop;
            let c2rust_fresh45 = dest;
            dest = dest.offset(1);
            *c2rust_fresh45 -= whoop;
            (*vnf).rampvol -= 1;
        } else if (*vnf).click != 0 {
            whoop = (*vnf).lvolsel as SLONG
                * (((1 as SLONG) << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    - (*vnf).click as SLONG) * sample as SLONG
                + (*vnf).lastvalL * (*vnf).click as SLONG
                >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
            let c2rust_fresh46 = dest;
            dest = dest.offset(1);
            *c2rust_fresh46 += whoop;
            let c2rust_fresh47 = dest;
            dest = dest.offset(1);
            *c2rust_fresh47 -= whoop;
            (*vnf).click -= 1;
        } else {
            let c2rust_fresh48 = dest;
            dest = dest.offset(1);
            *c2rust_fresh48 += ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh49 = dest;
            dest = dest.offset(1);
            *c2rust_fresh49 -= ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
        }
    }
    (*vnf).lastvalL = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    (*vnf).lastvalR = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    return idx;
}
unsafe extern "C" fn MixMonoNormal(
    srce: *const SWORD,
    mut dest: *mut SLONG,
    mut idx: SLONGLONG,
    mut increment: SLONGLONG,
    mut todo: SLONG,
) -> SLONGLONG {
    let mut sample: SWORD = 0 as SWORD;
    let mut i: SLONGLONG = 0;
    let mut f: SLONGLONG = 0;
    loop {
        let c2rust_fresh50 = todo;
        todo -= 1;
        if c2rust_fresh50 == 0 {
            break;
        }
        i = idx >> 28 as ::core::ffi::c_int;
        f = idx
            & ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong) as SLONGLONG;
        sample = (*srce.offset(i as isize) as SLONGLONG
            * (((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong) as SLONGLONG - f)
            + *srce.offset((i + 1 as SLONGLONG) as isize) as SLONGLONG * f
            >> 28 as ::core::ffi::c_int) as SWORD;
        idx += increment;
        if (*vnf).rampvol != 0 {
            let c2rust_fresh51 = dest;
            dest = dest.offset(1);
            *c2rust_fresh51
                += ((((*vnf).oldlvol * (*vnf).rampvol) as SLONGLONG
                    + ((*vnf).lvolsel as ::core::ffi::c_long
                        * (((1 as ::core::ffi::c_long)
                            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            - (*vnf).rampvol as ::core::ffi::c_long)) as SLONGLONG)
                    * sample as SLONGLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            (*vnf).rampvol -= 1;
        } else if (*vnf).click != 0 {
            let c2rust_fresh52 = dest;
            dest = dest.offset(1);
            *c2rust_fresh52
                += ((*vnf).lvolsel as SLONGLONG
                    * (((1 as ::core::ffi::c_long)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).click as ::core::ffi::c_long) as SLONGLONG
                    * sample as SLONGLONG
                    + ((*vnf).lastvalL * (*vnf).click as SLONG) as SLONGLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            (*vnf).click -= 1;
        } else {
            let c2rust_fresh53 = dest;
            dest = dest.offset(1);
            *c2rust_fresh53 += ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
        }
    }
    (*vnf).lastvalL = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    return idx;
}
unsafe extern "C" fn MixStereoNormal(
    srce: *const SWORD,
    mut dest: *mut SLONG,
    mut idx: SLONGLONG,
    mut increment: SLONGLONG,
    mut todo: ULONG,
) -> SLONGLONG {
    let mut sample: SWORD = 0 as SWORD;
    let mut i: SLONGLONG = 0;
    let mut f: SLONGLONG = 0;
    loop {
        let c2rust_fresh54 = todo;
        todo = todo.wrapping_sub(1);
        if c2rust_fresh54 == 0 {
            break;
        }
        i = idx >> 28 as ::core::ffi::c_int;
        f = idx
            & ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong) as SLONGLONG;
        sample = (*srce.offset(i as isize) as SLONGLONG
            * (((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong) as SLONGLONG - f)
            + *srce.offset((i + 1 as SLONGLONG) as isize) as SLONGLONG * f
            >> 28 as ::core::ffi::c_int) as SWORD;
        idx += increment;
        if (*vnf).rampvol != 0 {
            let c2rust_fresh55 = dest;
            dest = dest.offset(1);
            *c2rust_fresh55
                += (((*vnf).oldlvol as SLONGLONG * (*vnf).rampvol as SLONGLONG
                    + ((*vnf).lvolsel as ::core::ffi::c_long
                        * (((1 as ::core::ffi::c_long)
                            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            - (*vnf).rampvol as ::core::ffi::c_long)) as SLONGLONG)
                    * sample as SLONGLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            let c2rust_fresh56 = dest;
            dest = dest.offset(1);
            *c2rust_fresh56
                += (((*vnf).oldrvol as SLONGLONG * (*vnf).rampvol as SLONGLONG
                    + ((*vnf).rvolsel as ::core::ffi::c_long
                        * (((1 as ::core::ffi::c_long)
                            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            - (*vnf).rampvol as ::core::ffi::c_long)) as SLONGLONG)
                    * sample as SLONGLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            (*vnf).rampvol -= 1;
        } else if (*vnf).click != 0 {
            let c2rust_fresh57 = dest;
            dest = dest.offset(1);
            *c2rust_fresh57
                += (((*vnf).lvolsel as ::core::ffi::c_long
                    * (((1 as ::core::ffi::c_long)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).click as ::core::ffi::c_long)) as SLONGLONG
                    * sample as SLONGLONG
                    + ((*vnf).lastvalL * (*vnf).click as SLONG) as SLONGLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            let c2rust_fresh58 = dest;
            dest = dest.offset(1);
            *c2rust_fresh58
                += ((*vnf).rvolsel as SLONGLONG
                    * (((1 as ::core::ffi::c_long)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).click as ::core::ffi::c_long) as SLONGLONG
                    * sample as SLONGLONG
                    + ((*vnf).lastvalR * (*vnf).click as SLONG) as SLONGLONG
                    >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            (*vnf).click -= 1;
        } else {
            let c2rust_fresh59 = dest;
            dest = dest.offset(1);
            *c2rust_fresh59 += ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh60 = dest;
            dest = dest.offset(1);
            *c2rust_fresh60 += ((*vnf).rvolsel * sample as ::core::ffi::c_int) as SLONG;
        }
    }
    (*vnf).lastvalL = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    (*vnf).lastvalR = ((*vnf).rvolsel * sample as ::core::ffi::c_int) as SLONG;
    return idx;
}
unsafe extern "C" fn MixStereoSurround(
    mut srce: *const SWORD,
    mut dest: *mut SLONG,
    mut idx: SLONGLONG,
    mut increment: SLONGLONG,
    mut todo: ULONG,
) -> SLONGLONG {
    let mut sample: SWORD = 0 as SWORD;
    let mut whoop: ::core::ffi::c_long = 0;
    let mut i: SLONGLONG = 0;
    let mut f: SLONGLONG = 0;
    loop {
        let c2rust_fresh61 = todo;
        todo = todo.wrapping_sub(1);
        if c2rust_fresh61 == 0 {
            break;
        }
        i = idx >> 28 as ::core::ffi::c_int;
        f = idx
            & ((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong) as SLONGLONG;
        sample = (*srce.offset(i as isize) as SLONGLONG
            * (((1 as ::core::ffi::c_ulong) << 28 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong) as SLONGLONG - f)
            + *srce.offset((i + 1 as SLONGLONG) as isize) as SLONGLONG * f
            >> 28 as ::core::ffi::c_int) as SWORD;
        idx += increment;
        if (*vnf).rampvol != 0 {
            whoop = ((((*vnf).oldlvol * (*vnf).rampvol) as SLONGLONG
                + ((*vnf).lvolsel as ::core::ffi::c_long
                    * (((1 as ::core::ffi::c_long)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        - (*vnf).rampvol as ::core::ffi::c_long)) as SLONGLONG)
                * sample as SLONGLONG
                >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as ::core::ffi::c_long;
            let c2rust_fresh62 = dest;
            dest = dest.offset(1);
            *c2rust_fresh62 += whoop;
            let c2rust_fresh63 = dest;
            dest = dest.offset(1);
            *c2rust_fresh63 -= whoop;
            (*vnf).rampvol -= 1;
        } else if (*vnf).click != 0 {
            whoop = ((*vnf).lvolsel as SLONGLONG
                * (((1 as ::core::ffi::c_long)
                    << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    - (*vnf).click as ::core::ffi::c_long) as SLONGLONG
                * sample as SLONGLONG
                + ((*vnf).lastvalL * (*vnf).click as SLONG) as SLONGLONG
                >> 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as ::core::ffi::c_long;
            let c2rust_fresh64 = dest;
            dest = dest.offset(1);
            *c2rust_fresh64 += whoop;
            let c2rust_fresh65 = dest;
            dest = dest.offset(1);
            *c2rust_fresh65 -= whoop;
            (*vnf).click -= 1;
        } else {
            let c2rust_fresh66 = dest;
            dest = dest.offset(1);
            *c2rust_fresh66 += ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh67 = dest;
            dest = dest.offset(1);
            *c2rust_fresh67 -= ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
        }
    }
    (*vnf).lastvalL = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    (*vnf).lastvalR = ((*vnf).lvolsel * sample as ::core::ffi::c_int) as SLONG;
    return idx;
}
static mut Mix32toFP: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_float, *const SLONG, SLONG) -> (),
> = None;
static mut Mix32to16: Option<
    unsafe extern "C" fn(*mut SWORD, *const SLONG, SLONG) -> (),
> = None;
static mut Mix32to8: Option<
    unsafe extern "C" fn(*mut SBYTE, *const SLONG, SLONG) -> (),
> = None;
static mut MixReverb: Option<unsafe extern "C" fn(*mut SLONG, SLONG) -> ()> = None;
unsafe extern "C" fn MixReverb_Normal(mut srce: *mut SLONG, mut count: SLONG) {
    let mut speedup: SLONG = 0;
    let mut ReverbPct: ::core::ffi::c_int = 0;
    let mut loc1: ::core::ffi::c_uint = 0;
    let mut loc2: ::core::ffi::c_uint = 0;
    let mut loc3: ::core::ffi::c_uint = 0;
    let mut loc4: ::core::ffi::c_uint = 0;
    let mut loc5: ::core::ffi::c_uint = 0;
    let mut loc6: ::core::ffi::c_uint = 0;
    let mut loc7: ::core::ffi::c_uint = 0;
    let mut loc8: ::core::ffi::c_uint = 0;
    ReverbPct = 58 as ::core::ffi::c_int
        + md_reverb as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
    loc1 = RVRindex.wrapping_rem(RVc1 as ULONG) as ::core::ffi::c_uint;
    loc2 = RVRindex.wrapping_rem(RVc2 as ULONG) as ::core::ffi::c_uint;
    loc3 = RVRindex.wrapping_rem(RVc3 as ULONG) as ::core::ffi::c_uint;
    loc4 = RVRindex.wrapping_rem(RVc4 as ULONG) as ::core::ffi::c_uint;
    loc5 = RVRindex.wrapping_rem(RVc5 as ULONG) as ::core::ffi::c_uint;
    loc6 = RVRindex.wrapping_rem(RVc6 as ULONG) as ::core::ffi::c_uint;
    loc7 = RVRindex.wrapping_rem(RVc7 as ULONG) as ::core::ffi::c_uint;
    loc8 = RVRindex.wrapping_rem(RVc8 as ULONG) as ::core::ffi::c_uint;
    loop {
        let c2rust_fresh0 = count;
        count -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        speedup = *srce >> 3 as ::core::ffi::c_int;
        *RVbufL1.offset(loc1 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL1.offset(loc1 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL2.offset(loc2 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL2.offset(loc2 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL3.offset(loc3 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL3.offset(loc3 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL4.offset(loc4 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL4.offset(loc4 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL5.offset(loc5 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL5.offset(loc5 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL6.offset(loc6 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL6.offset(loc6 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL7.offset(loc7 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL7.offset(loc7 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL8.offset(loc8 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL8.offset(loc8 as isize)
                >> 7 as ::core::ffi::c_int);
        RVRindex = RVRindex.wrapping_add(1);
        loc1 = RVRindex.wrapping_rem(RVc1 as ULONG) as ::core::ffi::c_uint;
        loc2 = RVRindex.wrapping_rem(RVc2 as ULONG) as ::core::ffi::c_uint;
        loc3 = RVRindex.wrapping_rem(RVc3 as ULONG) as ::core::ffi::c_uint;
        loc4 = RVRindex.wrapping_rem(RVc4 as ULONG) as ::core::ffi::c_uint;
        loc5 = RVRindex.wrapping_rem(RVc5 as ULONG) as ::core::ffi::c_uint;
        loc6 = RVRindex.wrapping_rem(RVc6 as ULONG) as ::core::ffi::c_uint;
        loc7 = RVRindex.wrapping_rem(RVc7 as ULONG) as ::core::ffi::c_uint;
        loc8 = RVRindex.wrapping_rem(RVc8 as ULONG) as ::core::ffi::c_uint;
        let c2rust_fresh1 = srce;
        srce = srce.offset(1);
        *c2rust_fresh1
            += *RVbufL1.offset(loc1 as isize) - *RVbufL2.offset(loc2 as isize)
                + *RVbufL3.offset(loc3 as isize) - *RVbufL4.offset(loc4 as isize)
                + *RVbufL5.offset(loc5 as isize) - *RVbufL6.offset(loc6 as isize)
                + *RVbufL7.offset(loc7 as isize) - *RVbufL8.offset(loc8 as isize);
    };
}
unsafe extern "C" fn MixReverb_Stereo(mut srce: *mut SLONG, mut count: SLONG) {
    let mut speedup: SLONG = 0;
    let mut ReverbPct: ::core::ffi::c_int = 0;
    let mut loc1: ::core::ffi::c_uint = 0;
    let mut loc2: ::core::ffi::c_uint = 0;
    let mut loc3: ::core::ffi::c_uint = 0;
    let mut loc4: ::core::ffi::c_uint = 0;
    let mut loc5: ::core::ffi::c_uint = 0;
    let mut loc6: ::core::ffi::c_uint = 0;
    let mut loc7: ::core::ffi::c_uint = 0;
    let mut loc8: ::core::ffi::c_uint = 0;
    ReverbPct = 58 as ::core::ffi::c_int
        + md_reverb as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
    loc1 = RVRindex.wrapping_rem(RVc1 as ULONG) as ::core::ffi::c_uint;
    loc2 = RVRindex.wrapping_rem(RVc2 as ULONG) as ::core::ffi::c_uint;
    loc3 = RVRindex.wrapping_rem(RVc3 as ULONG) as ::core::ffi::c_uint;
    loc4 = RVRindex.wrapping_rem(RVc4 as ULONG) as ::core::ffi::c_uint;
    loc5 = RVRindex.wrapping_rem(RVc5 as ULONG) as ::core::ffi::c_uint;
    loc6 = RVRindex.wrapping_rem(RVc6 as ULONG) as ::core::ffi::c_uint;
    loc7 = RVRindex.wrapping_rem(RVc7 as ULONG) as ::core::ffi::c_uint;
    loc8 = RVRindex.wrapping_rem(RVc8 as ULONG) as ::core::ffi::c_uint;
    loop {
        let c2rust_fresh11 = count;
        count -= 1;
        if c2rust_fresh11 == 0 {
            break;
        }
        speedup = *srce >> 3 as ::core::ffi::c_int;
        *RVbufL1.offset(loc1 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL1.offset(loc1 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL2.offset(loc2 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL2.offset(loc2 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL3.offset(loc3 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL3.offset(loc3 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL4.offset(loc4 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL4.offset(loc4 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL5.offset(loc5 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL5.offset(loc5 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL6.offset(loc6 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL6.offset(loc6 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL7.offset(loc7 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL7.offset(loc7 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufL8.offset(loc8 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufL8.offset(loc8 as isize)
                >> 7 as ::core::ffi::c_int);
        speedup = *srce.offset(1isize) >> 3 as ::core::ffi::c_int;
        *RVbufR1.offset(loc1 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR1.offset(loc1 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR2.offset(loc2 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR2.offset(loc2 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR3.offset(loc3 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR3.offset(loc3 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR4.offset(loc4 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR4.offset(loc4 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR5.offset(loc5 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR5.offset(loc5 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR6.offset(loc6 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR6.offset(loc6 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR7.offset(loc7 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR7.offset(loc7 as isize)
                >> 7 as ::core::ffi::c_int);
        *RVbufR8.offset(loc8 as isize) = speedup
            + (ReverbPct as SLONG * *RVbufR8.offset(loc8 as isize)
                >> 7 as ::core::ffi::c_int);
        RVRindex = RVRindex.wrapping_add(1);
        loc1 = RVRindex.wrapping_rem(RVc1 as ULONG) as ::core::ffi::c_uint;
        loc2 = RVRindex.wrapping_rem(RVc2 as ULONG) as ::core::ffi::c_uint;
        loc3 = RVRindex.wrapping_rem(RVc3 as ULONG) as ::core::ffi::c_uint;
        loc4 = RVRindex.wrapping_rem(RVc4 as ULONG) as ::core::ffi::c_uint;
        loc5 = RVRindex.wrapping_rem(RVc5 as ULONG) as ::core::ffi::c_uint;
        loc6 = RVRindex.wrapping_rem(RVc6 as ULONG) as ::core::ffi::c_uint;
        loc7 = RVRindex.wrapping_rem(RVc7 as ULONG) as ::core::ffi::c_uint;
        loc8 = RVRindex.wrapping_rem(RVc8 as ULONG) as ::core::ffi::c_uint;
        let c2rust_fresh12 = srce;
        srce = srce.offset(1);
        *c2rust_fresh12
            += *RVbufL1.offset(loc1 as isize) - *RVbufL2.offset(loc2 as isize)
                + *RVbufL3.offset(loc3 as isize) - *RVbufL4.offset(loc4 as isize)
                + *RVbufL5.offset(loc5 as isize) - *RVbufL6.offset(loc6 as isize)
                + *RVbufL7.offset(loc7 as isize) - *RVbufL8.offset(loc8 as isize);
        let c2rust_fresh13 = srce;
        srce = srce.offset(1);
        *c2rust_fresh13
            += *RVbufR1.offset(loc1 as isize) - *RVbufR2.offset(loc2 as isize)
                + *RVbufR3.offset(loc3 as isize) - *RVbufR4.offset(loc4 as isize)
                + *RVbufR5.offset(loc5 as isize) - *RVbufR6.offset(loc6 as isize)
                + *RVbufR7.offset(loc7 as isize) - *RVbufR8.offset(loc8 as isize);
    };
}
static mut MixLowPass: Option<unsafe extern "C" fn(*mut SLONG, SLONG) -> ()> = None;
static mut nRightNR: ::core::ffi::c_int = 0;
static mut nLeftNR: ::core::ffi::c_int = 0;
unsafe extern "C" fn MixLowPass_Stereo(mut srce: *mut SLONG, mut count: SLONG) {
    let mut n1: ::core::ffi::c_int = nLeftNR;
    let mut n2: ::core::ffi::c_int = nRightNR;
    let mut pnr: *mut SLONG = srce;
    let mut nr: ::core::ffi::c_int = count as ::core::ffi::c_int;
    while nr != 0 {
        let mut vnr: ::core::ffi::c_int = (*pnr.offset(0isize)
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        *pnr.offset(0isize) = (vnr + n1) as SLONG;
        n1 = vnr;
        vnr = (*pnr.offset(1isize) >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        *pnr.offset(1isize) = (vnr + n2) as SLONG;
        n2 = vnr;
        pnr = pnr.offset(2 as ::core::ffi::c_int as isize);
        nr -= 1;
    }
    nLeftNR = n1;
    nRightNR = n2;
}
unsafe extern "C" fn MixLowPass_Normal(mut srce: *mut SLONG, mut count: SLONG) {
    let mut n1: ::core::ffi::c_int = nLeftNR;
    let mut pnr: *mut SLONG = srce;
    let mut nr: ::core::ffi::c_int = count as ::core::ffi::c_int;
    while nr != 0 {
        let mut vnr: ::core::ffi::c_int = (*pnr.offset(0isize)
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        *pnr.offset(0isize) = (vnr + n1) as SLONG;
        n1 = vnr;
        pnr = pnr.offset(1);
        nr -= 1;
    }
    nLeftNR = n1;
}
unsafe extern "C" fn Mix32ToFP_Normal(
    mut dste: *mut ::core::ffi::c_float,
    mut srce: *const SLONG,
    mut count: SLONG,
) {
    let mut x1: ::core::ffi::c_float = 0.;
    let mut x2: ::core::ffi::c_float = 0.;
    let mut tmpx: ::core::ffi::c_float = 0.;
    let mut i: ::core::ffi::c_int = 0;
    count = (count as ::core::ffi::c_ulong)
        .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as SLONG;
    while count != 0 {
        tmpx = 0.0f32;
        i = ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_div(2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        while i != 0 {
            let c2rust_fresh8 = srce;
            srce = srce.offset(1);
            x1 = *c2rust_fresh8 as ::core::ffi::c_float
                * (1.0f32 / 32768.0f32
                    / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_float * 1.0f32));
            let c2rust_fresh9 = srce;
            srce = srce.offset(1);
            x2 = *c2rust_fresh9 as ::core::ffi::c_float
                * (1.0f32 / 32768.0f32
                    / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_float * 1.0f32));
            x1 = if x1 > 1.0f32 { 1.0f32 } else if x1 < -1.0f32 { -1.0f32 } else { x1 };
            x2 = if x2 > 1.0f32 { 1.0f32 } else if x2 < -1.0f32 { -1.0f32 } else { x2 };
            tmpx += x1 + x2;
            i -= 1;
        }
        let c2rust_fresh10 = dste;
        dste = dste.offset(1);
        *c2rust_fresh10 = tmpx
            * (1.0f32
                / ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_float);
        count -= 1;
    }
}
unsafe extern "C" fn Mix32ToFP_Stereo(
    mut dste: *mut ::core::ffi::c_float,
    mut srce: *const SLONG,
    mut count: SLONG,
) {
    let mut x1: ::core::ffi::c_float = 0.;
    let mut x2: ::core::ffi::c_float = 0.;
    let mut x3: ::core::ffi::c_float = 0.;
    let mut x4: ::core::ffi::c_float = 0.;
    let mut tmpx: ::core::ffi::c_float = 0.;
    let mut tmpy: ::core::ffi::c_float = 0.;
    let mut i: ::core::ffi::c_int = 0;
    count = (count as ::core::ffi::c_ulong)
        .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as SLONG;
    while count != 0 {
        tmpy = 0.0f32;
        tmpx = tmpy;
        i = ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_div(2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        while i != 0 {
            let c2rust_fresh26 = srce;
            srce = srce.offset(1);
            x1 = *c2rust_fresh26 as ::core::ffi::c_float
                * (1.0f32 / 32768.0f32
                    / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_float * 1.0f32));
            let c2rust_fresh27 = srce;
            srce = srce.offset(1);
            x2 = *c2rust_fresh27 as ::core::ffi::c_float
                * (1.0f32 / 32768.0f32
                    / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_float * 1.0f32));
            let c2rust_fresh28 = srce;
            srce = srce.offset(1);
            x3 = *c2rust_fresh28 as ::core::ffi::c_float
                * (1.0f32 / 32768.0f32
                    / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_float * 1.0f32));
            let c2rust_fresh29 = srce;
            srce = srce.offset(1);
            x4 = *c2rust_fresh29 as ::core::ffi::c_float
                * (1.0f32 / 32768.0f32
                    / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                        as ::core::ffi::c_float * 1.0f32));
            x1 = if x1 > 1.0f32 { 1.0f32 } else if x1 < -1.0f32 { -1.0f32 } else { x1 };
            x2 = if x2 > 1.0f32 { 1.0f32 } else if x2 < -1.0f32 { -1.0f32 } else { x2 };
            x3 = if x3 > 1.0f32 { 1.0f32 } else if x3 < -1.0f32 { -1.0f32 } else { x3 };
            x4 = if x4 > 1.0f32 { 1.0f32 } else if x4 < -1.0f32 { -1.0f32 } else { x4 };
            tmpx += x1 + x3;
            tmpy += x2 + x4;
            i -= 1;
        }
        let c2rust_fresh30 = dste;
        dste = dste.offset(1);
        *c2rust_fresh30 = tmpx
            * (1.0f32
                / ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_float);
        let c2rust_fresh31 = dste;
        dste = dste.offset(1);
        *c2rust_fresh31 = tmpy
            * (1.0f32
                / ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_float);
        count -= 1;
    }
}
unsafe extern "C" fn Mix32To16_Normal(
    mut dste: *mut SWORD,
    mut srce: *const SLONG,
    mut count: SLONG,
) {
    let mut x1: SLONG = 0;
    let mut x2: SLONG = 0;
    let mut tmpx: SLONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    count = (count as ::core::ffi::c_ulong)
        .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as SLONG;
    while count != 0 {
        tmpx = 0 as SLONG;
        i = ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_div(2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        while i != 0 {
            let c2rust_fresh5 = srce;
            srce = srce.offset(1);
            x1 = *c2rust_fresh5
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 1 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh6 = srce;
            srce = srce.offset(1);
            x2 = *c2rust_fresh6
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 1 as ::core::ffi::c_int) as SLONG;
            x1 = if x1 >= 32768 as SLONG {
                (32768 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x1 < -32768 as SLONG {
                -32768 as SLONG
            } else {
                x1
            };
            x2 = if x2 >= 32768 as SLONG {
                (32768 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x2 < -32768 as SLONG {
                -32768 as SLONG
            } else {
                x2
            };
            tmpx += x1 + x2;
            i -= 1;
        }
        let c2rust_fresh7 = dste;
        dste = dste.offset(1);
        *c2rust_fresh7 = (tmpx as ::core::ffi::c_ulong)
            .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            as SWORD;
        count -= 1;
    }
}
unsafe extern "C" fn Mix32To16_Stereo(
    mut dste: *mut SWORD,
    mut srce: *const SLONG,
    mut count: SLONG,
) {
    let mut x1: SLONG = 0;
    let mut x2: SLONG = 0;
    let mut x3: SLONG = 0;
    let mut x4: SLONG = 0;
    let mut tmpx: SLONG = 0;
    let mut tmpy: SLONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    count = (count as ::core::ffi::c_ulong)
        .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as SLONG;
    while count != 0 {
        tmpy = 0 as SLONG;
        tmpx = tmpy;
        i = ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_div(2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        while i != 0 {
            let c2rust_fresh20 = srce;
            srce = srce.offset(1);
            x1 = *c2rust_fresh20
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 1 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh21 = srce;
            srce = srce.offset(1);
            x2 = *c2rust_fresh21
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 1 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh22 = srce;
            srce = srce.offset(1);
            x3 = *c2rust_fresh22
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 1 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh23 = srce;
            srce = srce.offset(1);
            x4 = *c2rust_fresh23
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 1 as ::core::ffi::c_int) as SLONG;
            x1 = if x1 >= 32768 as SLONG {
                (32768 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x1 < -32768 as SLONG {
                -32768 as SLONG
            } else {
                x1
            };
            x2 = if x2 >= 32768 as SLONG {
                (32768 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x2 < -32768 as SLONG {
                -32768 as SLONG
            } else {
                x2
            };
            x3 = if x3 >= 32768 as SLONG {
                (32768 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x3 < -32768 as SLONG {
                -32768 as SLONG
            } else {
                x3
            };
            x4 = if x4 >= 32768 as SLONG {
                (32768 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x4 < -32768 as SLONG {
                -32768 as SLONG
            } else {
                x4
            };
            tmpx += x1 + x3;
            tmpy += x2 + x4;
            i -= 1;
        }
        let c2rust_fresh24 = dste;
        dste = dste.offset(1);
        *c2rust_fresh24 = (tmpx as ::core::ffi::c_ulong)
            .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            as SWORD;
        let c2rust_fresh25 = dste;
        dste = dste.offset(1);
        *c2rust_fresh25 = (tmpy as ::core::ffi::c_ulong)
            .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            as SWORD;
        count -= 1;
    }
}
unsafe extern "C" fn Mix32To8_Normal(
    mut dste: *mut SBYTE,
    mut srce: *const SLONG,
    mut count: SLONG,
) {
    let mut x1: SLONG = 0;
    let mut x2: SLONG = 0;
    let mut tmpx: SLONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    count = (count as ::core::ffi::c_ulong)
        .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as SLONG;
    while count != 0 {
        tmpx = 0 as SLONG;
        i = ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_div(2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        while i != 0 {
            let c2rust_fresh2 = srce;
            srce = srce.offset(1);
            x1 = *c2rust_fresh2
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 256 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh3 = srce;
            srce = srce.offset(1);
            x2 = *c2rust_fresh3
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 256 as ::core::ffi::c_int) as SLONG;
            x1 = if x1 >= 128 as SLONG {
                (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x1 < -128 as SLONG {
                -128 as SLONG
            } else {
                x1
            };
            x2 = if x2 >= 128 as SLONG {
                (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x2 < -128 as SLONG {
                -128 as SLONG
            } else {
                x2
            };
            tmpx += x1 + x2;
            i -= 1;
        }
        let c2rust_fresh4 = dste;
        dste = dste.offset(1);
        *c2rust_fresh4 = (tmpx as ::core::ffi::c_ulong)
            .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_add(128 as ::core::ffi::c_ulong) as SBYTE;
        count -= 1;
    }
}
unsafe extern "C" fn Mix32To8_Stereo(
    mut dste: *mut SBYTE,
    mut srce: *const SLONG,
    mut count: SLONG,
) {
    let mut x1: SLONG = 0;
    let mut x2: SLONG = 0;
    let mut x3: SLONG = 0;
    let mut x4: SLONG = 0;
    let mut tmpx: SLONG = 0;
    let mut tmpy: SLONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    count = (count as ::core::ffi::c_ulong)
        .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as SLONG;
    while count != 0 {
        tmpy = 0 as SLONG;
        tmpx = tmpy;
        i = ((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_div(2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        while i != 0 {
            let c2rust_fresh14 = srce;
            srce = srce.offset(1);
            x1 = *c2rust_fresh14
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 256 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh15 = srce;
            srce = srce.offset(1);
            x2 = *c2rust_fresh15
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 256 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh16 = srce;
            srce = srce.offset(1);
            x3 = *c2rust_fresh16
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 256 as ::core::ffi::c_int) as SLONG;
            let c2rust_fresh17 = srce;
            srce = srce.offset(1);
            x4 = *c2rust_fresh17
                / (((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                    * 256 as ::core::ffi::c_int) as SLONG;
            x1 = if x1 >= 128 as SLONG {
                (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x1 < -128 as SLONG {
                -128 as SLONG
            } else {
                x1
            };
            x2 = if x2 >= 128 as SLONG {
                (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x2 < -128 as SLONG {
                -128 as SLONG
            } else {
                x2
            };
            x3 = if x3 >= 128 as SLONG {
                (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x3 < -128 as SLONG {
                -128 as SLONG
            } else {
                x3
            };
            x4 = if x4 >= 128 as SLONG {
                (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
            } else if x4 < -128 as SLONG {
                -128 as SLONG
            } else {
                x4
            };
            tmpx += x1 + x3;
            tmpy += x2 + x4;
            i -= 1;
        }
        let c2rust_fresh18 = dste;
        dste = dste.offset(1);
        *c2rust_fresh18 = (tmpx as ::core::ffi::c_ulong)
            .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_add(128 as ::core::ffi::c_ulong) as SBYTE;
        let c2rust_fresh19 = dste;
        dste = dste.offset(1);
        *c2rust_fresh19 = (tmpy as ::core::ffi::c_ulong)
            .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
            .wrapping_add(128 as ::core::ffi::c_ulong) as SBYTE;
        count -= 1;
    }
}
unsafe extern "C" fn AddChannel(mut ptr: *mut SLONG, mut todo: SLONG) {
    let mut end: SLONGLONG = 0;
    let mut done: SLONGLONG = 0;
    let mut s: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
    s = *Samples.offset((*vnf).handle as isize);
    if s.is_null() {
        (*vnf).active = 0 as UBYTE;
        (*vnf).current = (*vnf).active as SLONGLONG;
        (*vnf).lastvalR = 0 as SLONG;
        (*vnf).lastvalL = (*vnf).lastvalR;
        return;
    }
    while todo > 0 as SLONG {
        let mut endpos: SLONGLONG = 0;
        if (*vnf).flags as ::core::ffi::c_int & 0x400 as ::core::ffi::c_int != 0 {
            if (*vnf).flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0
                && (*vnf).current < idxlpos
            {
                if (*vnf).flags as ::core::ffi::c_int & 0x200 as ::core::ffi::c_int != 0
                {
                    (*vnf).current = idxlpos + (idxlpos - (*vnf).current);
                    (*vnf).flags = ((*vnf).flags as ::core::ffi::c_int
                        & !(0x400 as ::core::ffi::c_int)) as UWORD;
                    (*vnf).increment = -(*vnf).increment;
                } else {
                    (*vnf).current = idxlend - (idxlpos - (*vnf).current);
                }
            } else if (*vnf).current < 0 as SLONGLONG {
                (*vnf).active = 0 as UBYTE;
                (*vnf).current = (*vnf).active as SLONGLONG;
                break;
            }
        } else if (*vnf).flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0
            && (*vnf).current >= idxlend
        {
            if (*vnf).flags as ::core::ffi::c_int & 0x200 as ::core::ffi::c_int != 0 {
                (*vnf).flags = ((*vnf).flags as ::core::ffi::c_int
                    | 0x400 as ::core::ffi::c_int) as UWORD;
                (*vnf).increment = -(*vnf).increment;
                (*vnf).current = idxlend - ((*vnf).current - idxlend);
            } else {
                (*vnf).current = idxlpos + ((*vnf).current - idxlend);
            }
        } else if (*vnf).current >= idxsize {
            (*vnf).active = 0 as UBYTE;
            (*vnf).current = (*vnf).active as SLONGLONG;
            break;
        }
        end = if (*vnf).flags as ::core::ffi::c_int & 0x400 as ::core::ffi::c_int != 0 {
            if (*vnf).flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0 {
                idxlpos
            } else {
                0 as SLONGLONG
            }
        } else if (*vnf).flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0 {
            idxlend
        } else {
            idxsize
        };
        if end == (*vnf).current || (*vnf).increment == 0 {
            done = 0 as SLONGLONG;
        } else {
            done = if ((end - (*vnf).current) / (*vnf).increment + 1 as SLONGLONG)
                < todo as SLONGLONG
            {
                (end - (*vnf).current) / (*vnf).increment + 1 as SLONGLONG
            } else {
                todo as SLONGLONG
            };
            if done < 0 as SLONGLONG {
                done = 0 as SLONGLONG;
            }
        }
        if done == 0 {
            (*vnf).active = 0 as UBYTE;
            break;
        } else {
            endpos = (*vnf).current + done * (*vnf).increment;
            if (*vnf).vol != 0 || (*vnf).rampvol != 0 {
                if (*vnf).current < 0x7fffffff as SLONGLONG
                    && endpos < 0x7fffffff as SLONGLONG
                {
                    if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
                        if (*vnf).pan == 512 as ::core::ffi::c_int
                            && vc_mode as ::core::ffi::c_int
                                & 0x100 as ::core::ffi::c_int != 0
                        {
                            (*vnf).current = Mix32StereoSurround(
                                s,
                                ptr,
                                (*vnf).current as SLONG,
                                (*vnf).increment as SLONG,
                                done as ULONG,
                            ) as SLONGLONG;
                        } else {
                            (*vnf).current = Mix32StereoNormal(
                                s,
                                ptr,
                                (*vnf).current as SLONG,
                                (*vnf).increment as SLONG,
                                done as ULONG,
                            ) as SLONGLONG;
                        }
                    } else {
                        (*vnf).current = Mix32MonoNormal(
                            s,
                            ptr,
                            (*vnf).current as SLONG,
                            (*vnf).increment as SLONG,
                            done as SLONG,
                        ) as SLONGLONG;
                    }
                } else if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0
                {
                    if (*vnf).pan == 512 as ::core::ffi::c_int
                        && vc_mode as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int
                            != 0
                    {
                        (*vnf).current = MixStereoSurround(
                            s,
                            ptr,
                            (*vnf).current,
                            (*vnf).increment,
                            done as ULONG,
                        );
                    } else {
                        (*vnf).current = MixStereoNormal(
                            s,
                            ptr,
                            (*vnf).current,
                            (*vnf).increment,
                            done as ULONG,
                        );
                    }
                } else {
                    (*vnf).current = MixMonoNormal(
                        s,
                        ptr,
                        (*vnf).current,
                        (*vnf).increment,
                        done as SLONG,
                    );
                }
            } else {
                (*vnf).lastvalR = 0 as SLONG;
                (*vnf).lastvalL = (*vnf).lastvalR;
                (*vnf).current = endpos;
            }
            todo = (todo as SLONGLONG - done) as SLONG;
            ptr = ptr
                .offset(
                    (if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
                        done << 1 as ::core::ffi::c_int
                    } else {
                        done
                    }) as isize,
                );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VC2_WriteSamples(mut buf: *mut SBYTE, mut todo: ULONG) {
    let mut left: ::core::ffi::c_int = 0;
    let mut portion: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buffer: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
    let mut t: ::core::ffi::c_int = 0;
    let mut pan: ::core::ffi::c_int = 0;
    let mut vol: ::core::ffi::c_int = 0;
    todo = todo.wrapping_mul((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int);
    while todo != 0 {
        if tickleft == 0 {
            if vc_mode as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
                md_player.expect("non-null function pointer")();
            }
            tickleft = ((md_mixfreq as ::core::ffi::c_long * 125 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong)
                .wrapping_mul((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
                .wrapping_div(
                    (md_bpm as ::core::ffi::c_long * 50 as ::core::ffi::c_long)
                        as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_long;
            tickleft = (tickleft as ::core::ffi::c_ulong
                & !((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)) as ::core::ffi::c_long;
        }
        left = (if (tickleft as ULONG) < todo { tickleft as ULONG } else { todo })
            as ::core::ffi::c_int;
        buffer = buf;
        tickleft -= left as ::core::ffi::c_long;
        todo = todo.wrapping_sub(left as ULONG);
        buf = buf
            .offset(
                samples2bytes(left as ULONG)
                    .wrapping_div((1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int)
                    as isize,
            );
        while left != 0 {
            portion = (if (left as ::core::ffi::c_long) < samplesthatfit {
                left as ::core::ffi::c_long
            } else {
                samplesthatfit
            }) as ::core::ffi::c_int;
            memset(
                vc_tickbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (portion
                    << if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    }) as size_t,
            );
            t = 0 as ::core::ffi::c_int;
            while t < vc_softchn {
                vnf = vinf.offset(t as isize);
                if (*vnf).kick != 0 {
                    (*vnf).current = ((*vnf).start as SLONGLONG)
                        << 28 as ::core::ffi::c_int;
                    (*vnf).kick = 0 as UBYTE;
                    (*vnf).active = 1 as UBYTE;
                    (*vnf).click = ((1 as ::core::ffi::c_long)
                        << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    (*vnf).rampvol = 0 as ::core::ffi::c_int;
                }
                if (*vnf).frq == 0 {
                    (*vnf).active = 0 as UBYTE;
                }
                if (*vnf).active != 0 {
                    (*vnf).increment = (((*vnf).frq as SLONGLONG)
                        << 28 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                        / md_mixfreq as SLONGLONG;
                    if (*vnf).flags as ::core::ffi::c_int & 0x400 as ::core::ffi::c_int
                        != 0
                    {
                        (*vnf).increment = -(*vnf).increment;
                    }
                    vol = (*vnf).vol;
                    pan = (*vnf).pan;
                    (*vnf).oldlvol = (*vnf).lvolsel;
                    (*vnf).oldrvol = (*vnf).rvolsel;
                    if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
                        if pan != 512 as ::core::ffi::c_int {
                            (*vnf).lvolsel = vol * (255 as ::core::ffi::c_int - pan)
                                >> 8 as ::core::ffi::c_int;
                            (*vnf).rvolsel = vol * pan >> 8 as ::core::ffi::c_int;
                        } else {
                            (*vnf).rvolsel = (vol as ::core::ffi::c_long
                                * 256 as ::core::ffi::c_long / 480 as ::core::ffi::c_long)
                                as ::core::ffi::c_int;
                            (*vnf).lvolsel = (*vnf).rvolsel;
                        }
                    } else {
                        (*vnf).lvolsel = vol;
                    }
                    idxsize = if (*vnf).size != 0 {
                        (((*vnf).size as SLONGLONG) << 28 as ::core::ffi::c_int)
                            - 1 as SLONGLONG
                    } else {
                        0 as SLONGLONG
                    };
                    idxlend = if (*vnf).repend != 0 {
                        (((*vnf).repend as SLONGLONG) << 28 as ::core::ffi::c_int)
                            - 1 as SLONGLONG
                    } else {
                        0 as SLONGLONG
                    };
                    idxlpos = ((*vnf).reppos as SLONGLONG) << 28 as ::core::ffi::c_int;
                    AddChannel(vc_tickbuf, portion as SLONG);
                }
                t += 1;
            }
            if md_mode as ::core::ffi::c_int & 0x1000 as ::core::ffi::c_int != 0 {
                MixLowPass
                    .expect("non-null function pointer")(vc_tickbuf, portion as SLONG);
            }
            if md_reverb != 0 {
                if md_reverb as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
                    md_reverb = 15 as UBYTE;
                }
                MixReverb
                    .expect("non-null function pointer")(vc_tickbuf, portion as SLONG);
            }
            if vc_callback.is_some() {
                vc_callback
                    .expect(
                        "non-null function pointer",
                    )(vc_tickbuf as *mut ::core::ffi::c_uchar, portion as size_t);
            }
            if vc_mode as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                Mix32toFP
                    .expect(
                        "non-null function pointer",
                    )(buffer as *mut ::core::ffi::c_float, vc_tickbuf, portion as SLONG);
            } else if vc_mode as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                Mix32to16
                    .expect(
                        "non-null function pointer",
                    )(buffer as *mut SWORD, vc_tickbuf, portion as SLONG);
            } else {
                Mix32to8
                    .expect(
                        "non-null function pointer",
                    )(buffer, vc_tickbuf, portion as SLONG);
            }
            buffer = buffer
                .offset(
                    samples2bytes(portion as ULONG)
                        .wrapping_div(
                            (1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int,
                        ) as isize,
                );
            left -= portion;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VC2_Init() -> ::core::ffi::c_int {
    VC_SetupPointers();
    if md_mode as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int == 0 {
        return VC1_Init();
    }
    Samples = MikMod_malloc(
        (384 as size_t).wrapping_mul(::core::mem::size_of::<*mut SWORD>() as size_t),
    ) as *mut *mut SWORD;
    if Samples.is_null() {
        MikMod_errno = C2Rust_Unnamed::MMERR_INITIALIZING_MIXER.0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    if vc_tickbuf.is_null() {
        vc_tickbuf = MikMod_malloc(
            ((8192 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<SLONG>() as size_t),
        ) as *mut SLONG;
        if vc_tickbuf.is_null() {
            MikMod_errno = C2Rust_Unnamed::MMERR_INITIALIZING_MIXER.0
                as ::core::ffi::c_int;
            return 1 as ::core::ffi::c_int;
        }
    }
    if md_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        Mix32toFP = Some(
            Mix32ToFP_Stereo
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_float,
                    *const SLONG,
                    SLONG,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_float,
                    *const SLONG,
                    SLONG,
                ) -> (),
            >;
        Mix32to16 = Some(
            Mix32To16_Stereo
                as unsafe extern "C" fn(*mut SWORD, *const SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SWORD, *const SLONG, SLONG) -> ()>;
        Mix32to8 = Some(
            Mix32To8_Stereo
                as unsafe extern "C" fn(*mut SBYTE, *const SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SBYTE, *const SLONG, SLONG) -> ()>;
        MixReverb = Some(
            MixReverb_Stereo as unsafe extern "C" fn(*mut SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SLONG, SLONG) -> ()>;
        MixLowPass = Some(
            MixLowPass_Stereo as unsafe extern "C" fn(*mut SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SLONG, SLONG) -> ()>;
    } else {
        Mix32toFP = Some(
            Mix32ToFP_Normal
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_float,
                    *const SLONG,
                    SLONG,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_float,
                    *const SLONG,
                    SLONG,
                ) -> (),
            >;
        Mix32to16 = Some(
            Mix32To16_Normal
                as unsafe extern "C" fn(*mut SWORD, *const SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SWORD, *const SLONG, SLONG) -> ()>;
        Mix32to8 = Some(
            Mix32To8_Normal
                as unsafe extern "C" fn(*mut SBYTE, *const SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SBYTE, *const SLONG, SLONG) -> ()>;
        MixReverb = Some(
            MixReverb_Normal as unsafe extern "C" fn(*mut SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SLONG, SLONG) -> ()>;
        MixLowPass = Some(
            MixLowPass_Normal as unsafe extern "C" fn(*mut SLONG, SLONG) -> (),
        ) as Option<unsafe extern "C" fn(*mut SLONG, SLONG) -> ()>;
    }
    md_mode = (md_mode as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int) as UWORD;
    vc_mode = md_mode;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_PlayStart() -> ::core::ffi::c_int {
    md_mode = (md_mode as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int) as UWORD;
    samplesthatfit = 8192 as ::core::ffi::c_long;
    if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        samplesthatfit >>= 1 as ::core::ffi::c_int;
    }
    tickleft = 0 as ::core::ffi::c_long;
    RVc1 = (5000 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc2 = (5078 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc3 = (5313 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc4 = (5703 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc5 = (6250 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc6 = (6953 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc7 = (7813 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVc8 = (8828 as ::core::ffi::c_long * md_mixfreq as ::core::ffi::c_long
        / (11000 as ::core::ffi::c_long * 10 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    RVbufL1 = MikMod_calloc(
        (RVc1 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL1.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL2 = MikMod_calloc(
        (RVc2 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL3 = MikMod_calloc(
        (RVc3 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL3.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL4 = MikMod_calloc(
        (RVc4 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL4.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL5 = MikMod_calloc(
        (RVc5 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL5.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL6 = MikMod_calloc(
        (RVc6 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL6.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL7 = MikMod_calloc(
        (RVc7 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL7.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    RVbufL8 = MikMod_calloc(
        (RVc8 + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<SLONG>() as size_t,
    ) as *mut SLONG;
    if RVbufL8.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        RVbufR1 = MikMod_calloc(
            (RVc1 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR1.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR2 = MikMod_calloc(
            (RVc2 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR2.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR3 = MikMod_calloc(
            (RVc3 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR3.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR4 = MikMod_calloc(
            (RVc4 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR4.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR5 = MikMod_calloc(
            (RVc5 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR5.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR6 = MikMod_calloc(
            (RVc6 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR6.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR7 = MikMod_calloc(
            (RVc7 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR7.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        RVbufR8 = MikMod_calloc(
            (RVc8 + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<SLONG>() as size_t,
        ) as *mut SLONG;
        if RVbufR8.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    RVRindex = 0 as ULONG;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_PlayStop() {
    MikMod_free(RVbufL1 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL2 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL3 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL4 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL5 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL6 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL7 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufL8 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR1 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR2 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR3 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR4 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR5 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR6 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR7 as *mut ::core::ffi::c_void);
    MikMod_free(RVbufR8 as *mut ::core::ffi::c_void);
    RVbufL8 = ::core::ptr::null_mut::<SLONG>();
    RVbufL7 = RVbufL8;
    RVbufL6 = RVbufL7;
    RVbufL5 = RVbufL6;
    RVbufL4 = RVbufL5;
    RVbufL3 = RVbufL4;
    RVbufL2 = RVbufL3;
    RVbufL1 = RVbufL2;
    RVbufR8 = ::core::ptr::null_mut::<SLONG>();
    RVbufR7 = RVbufR8;
    RVbufR6 = RVbufR7;
    RVbufR5 = RVbufR6;
    RVbufR4 = RVbufR5;
    RVbufR3 = RVbufR4;
    RVbufR2 = RVbufR3;
    RVbufR1 = RVbufR2;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_SetNumVoices() -> ::core::ffi::c_int {
    let mut t: ::core::ffi::c_int = 0;
    md_mode = (md_mode as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int) as UWORD;
    vc_softchn = md_softchn as ::core::ffi::c_int;
    if vc_softchn == 0 {
        return 0 as ::core::ffi::c_int;
    }
    MikMod_free(vinf as *mut ::core::ffi::c_void);
    vinf = MikMod_calloc(vc_softchn as size_t, ::core::mem::size_of::<VINFO>() as size_t)
        as *mut VINFO;
    if vinf.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    t = 0 as ::core::ffi::c_int;
    while t < vc_softchn {
        (*vinf.offset(t as isize)).frq = 10000 as ULONG;
        (*vinf.offset(t as isize)).pan = if t & 1 as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            255 as ::core::ffi::c_int
        };
        t += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn samples2bytes(mut samples: ULONG) -> ULONG {
    if vc_mode as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
        samples <<= 2 as ::core::ffi::c_int;
    } else if vc_mode as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        samples <<= 1 as ::core::ffi::c_int;
    }
    if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        samples <<= 1 as ::core::ffi::c_int;
    }
    return samples;
}
unsafe extern "C" fn bytes2samples(mut bytes: ULONG) -> ULONG {
    if vc_mode as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
        bytes >>= 2 as ::core::ffi::c_int;
    } else if vc_mode as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        bytes >>= 1 as ::core::ffi::c_int;
    }
    if vc_mode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        bytes >>= 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_SilenceBytes(
    mut buf: *mut SBYTE,
    mut todo: ULONG,
) -> ULONG {
    todo = samples2bytes(bytes2samples(todo));
    if vc_mode as ::core::ffi::c_int
        & (0x1 as ::core::ffi::c_int | 0x20 as ::core::ffi::c_int) != 0
    {
        memset(buf as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, todo);
    } else {
        memset(buf as *mut ::core::ffi::c_void, 0x80 as ::core::ffi::c_int, todo);
    }
    return todo;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_WriteBytes(mut buf: *mut SBYTE, mut todo: ULONG) -> ULONG {
    if vc_softchn == 0 {
        return VC2_SilenceBytes(buf, todo);
    }
    todo = bytes2samples(todo);
    VC2_WriteSamples(buf, todo);
    return samples2bytes(todo);
}
#[no_mangle]
pub unsafe extern "C" fn VC2_Exit() {
    MikMod_free(vinf as *mut ::core::ffi::c_void);
    MikMod_free(vc_tickbuf as *mut ::core::ffi::c_void);
    MikMod_free(Samples as *mut ::core::ffi::c_void);
    vc_tickbuf = ::core::ptr::null_mut::<SLONG>();
    vinf = ::core::ptr::null_mut::<VINFO>();
    Samples = ::core::ptr::null_mut::<*mut SWORD>();
    VC_SetupPointers();
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceGetVolume(mut voice: UBYTE) -> UWORD {
    return (*vinf.offset(voice as isize)).vol as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceGetPanning(mut voice: UBYTE) -> ULONG {
    return (*vinf.offset(voice as isize)).pan as ULONG;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceSetFrequency(mut voice: UBYTE, mut frq: ULONG) {
    (*vinf.offset(voice as isize)).frq = frq;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceGetFrequency(mut voice: UBYTE) -> ULONG {
    return (*vinf.offset(voice as isize)).frq;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoicePlay(
    mut voice: UBYTE,
    mut handle: SWORD,
    mut start: ULONG,
    mut size: ULONG,
    mut reppos: ULONG,
    mut repend: ULONG,
    mut flags: UWORD,
) {
    (*vinf.offset(voice as isize)).flags = flags;
    (*vinf.offset(voice as isize)).handle = handle;
    (*vinf.offset(voice as isize)).start = start;
    (*vinf.offset(voice as isize)).size = size;
    (*vinf.offset(voice as isize)).reppos = reppos;
    (*vinf.offset(voice as isize)).repend = repend;
    (*vinf.offset(voice as isize)).kick = 1 as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceStop(mut voice: UBYTE) {
    (*vinf.offset(voice as isize)).active = 0 as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceStopped(mut voice: UBYTE) -> BOOL {
    return ((*vinf.offset(voice as isize)).active as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceGetPosition(mut voice: UBYTE) -> SLONG {
    return ((*vinf.offset(voice as isize)).current >> 28 as ::core::ffi::c_int) as SLONG;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceSetVolume(mut voice: UBYTE, mut vol: UWORD) {
    if abs((*vinf.offset(voice as isize)).vol - vol as ::core::ffi::c_int)
        > 32 as ::core::ffi::c_int
    {
        (*vinf.offset(voice as isize)).rampvol = ((1 as ::core::ffi::c_long)
            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    (*vinf.offset(voice as isize)).vol = vol as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceSetPanning(mut voice: UBYTE, mut pan: ULONG) {
    if abs((*vinf.offset(voice as isize)).pan - pan as ::core::ffi::c_int)
        > 48 as ::core::ffi::c_int
    {
        (*vinf.offset(voice as isize)).rampvol = ((1 as ::core::ffi::c_long)
            << 6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    (*vinf.offset(voice as isize)).pan = pan as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_SampleUnload(mut handle: SWORD) {
    if !Samples.is_null() && (handle as ::core::ffi::c_int) < 384 as ::core::ffi::c_int {
        MikMod_free(*Samples.offset(handle as isize) as *mut ::core::ffi::c_void);
        *Samples.offset(handle as isize) = ::core::ptr::null_mut::<SWORD>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn VC2_SampleLoad(
    mut sload: *mut SAMPLOAD,
    mut r#type: ::core::ffi::c_int,
) -> SWORD {
    let mut s: *mut SAMPLE = (*sload).sample;
    let mut handle: ::core::ffi::c_int = 0;
    let mut t: ULONG = 0;
    let mut length: ULONG = 0;
    let mut loopstart: ULONG = 0;
    let mut loopend: ULONG = 0;
    let mut looplen: ULONG = 0;
    if r#type == C2Rust_Unnamed_0::MD_HARDWARE.0 as ::core::ffi::c_int {
        return -1 as SWORD;
    }
    if (*s).length > 0x10000000 as ULONG {
        MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_STREAM.0 as ::core::ffi::c_int;
        return -1 as SWORD;
    }
    handle = 0 as ::core::ffi::c_int;
    while handle < 384 as ::core::ffi::c_int {
        if (*Samples.offset(handle as isize)).is_null() {
            break;
        }
        handle += 1;
    }
    if handle == 384 as ::core::ffi::c_int {
        MikMod_errno = C2Rust_Unnamed::MMERR_OUT_OF_HANDLES.0 as ::core::ffi::c_int;
        return -1 as SWORD;
    }
    if (*s).loopend > (*s).length {
        (*s).loopend = (*s).length;
    }
    if (*s).loopstart >= (*s).loopend {
        (*s).flags = ((*s).flags as ::core::ffi::c_int & !(0x100 as ::core::ffi::c_int))
            as UWORD;
    }
    length = (*s).length;
    loopstart = (*s).loopstart;
    loopend = (*s).loopend;
    SL_SampleSigned(sload);
    SL_Sample8to16(sload);
    *Samples.offset(handle as isize) = MikMod_malloc(
        length.wrapping_add(20 as size_t) << 1 as ::core::ffi::c_int,
    ) as *mut SWORD;
    if (*Samples.offset(handle as isize)).is_null() {
        MikMod_errno = C2Rust_Unnamed::MMERR_SAMPLE_TOO_BIG.0 as ::core::ffi::c_int;
        return -1 as SWORD;
    }
    if SL_Load(
        *Samples.offset(handle as isize) as *mut ::core::ffi::c_void,
        sload,
        length,
    ) != 0
    {
        MikMod_free(*Samples.offset(handle as isize) as *mut ::core::ffi::c_void);
        *Samples.offset(handle as isize) = ::core::ptr::null_mut::<SWORD>();
        return -1 as SWORD;
    }
    if (*s).flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0 {
        looplen = loopend.wrapping_sub(loopstart);
        if (*s).flags as ::core::ffi::c_int & 0x200 as ::core::ffi::c_int != 0 {
            t = 0 as ULONG;
            while t < 16 as ULONG && t < looplen {
                *(*Samples.offset(handle as isize))
                    .offset(loopend.wrapping_add(t) as isize) = *(*Samples
                    .offset(handle as isize))
                    .offset(loopend.wrapping_sub(t).wrapping_sub(1 as ULONG) as isize);
                t = t.wrapping_add(1);
            }
        } else {
            t = 0 as ULONG;
            while t < 16 as ULONG && t < looplen {
                *(*Samples.offset(handle as isize))
                    .offset(loopend.wrapping_add(t) as isize) = *(*Samples
                    .offset(handle as isize))
                    .offset(t.wrapping_add(loopstart) as isize);
                t = t.wrapping_add(1);
            }
        }
    } else {
        t = 0 as ULONG;
        while t < 16 as ULONG {
            *(*Samples.offset(handle as isize))
                .offset(t.wrapping_add(length) as isize) = 0 as SWORD;
            t = t.wrapping_add(1);
        }
    }
    return handle as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_SampleSpace(mut r#type: ::core::ffi::c_int) -> ULONG {
    return vc_memory as ULONG;
}
#[no_mangle]
pub unsafe extern "C" fn VC2_SampleLength(
    mut r#type: ::core::ffi::c_int,
    mut s: *mut SAMPLE,
) -> ULONG {
    if s.is_null() {
        return 0 as ULONG;
    }
    return (*s)
        .length
        .wrapping_mul(
            (if (*s).flags as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ULONG,
        )
        .wrapping_add(16 as ULONG);
}
#[no_mangle]
pub unsafe extern "C" fn VC2_VoiceRealVolume(mut voice: UBYTE) -> ULONG {
    let mut i: ULONG = 0;
    let mut s: ULONG = 0;
    let mut size: ULONG = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut smp: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
    let mut t: SLONG = 0;
    t = ((*vinf.offset(voice as isize)).current >> 28 as ::core::ffi::c_int) as SLONG;
    if (*vinf.offset(voice as isize)).active == 0 {
        return 0 as ULONG;
    }
    s = (*vinf.offset(voice as isize)).handle as ULONG;
    size = (*vinf.offset(voice as isize)).size;
    i = 64 as ULONG;
    t -= 64 as SLONG;
    k = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    if i > size {
        i = size;
    }
    if t < 0 as SLONG {
        t = 0 as SLONG;
    }
    if (t as ULONG).wrapping_add(i) > size {
        t = size.wrapping_sub(i) as SLONG;
    }
    i &= !(1 as ::core::ffi::c_int) as ULONG;
    smp = (*Samples.offset(s as isize)).offset(t as isize);
    while i != 0 {
        if k < *smp as ::core::ffi::c_int {
            k = *smp as ::core::ffi::c_int;
        }
        if j > *smp as ::core::ffi::c_int {
            j = *smp as ::core::ffi::c_int;
        }
        i = i.wrapping_sub(1);
        smp = smp.offset(1);
    }
    return abs(k - j) as ULONG;
}
