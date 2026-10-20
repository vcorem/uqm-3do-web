/*	MikMod sound library
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

  $Id$

  Routines for loading samples. The sample loader utilizes the routines
  provided by the "registered" sample loader.

==============================================================================*/

// Rust translation of src/libs/mikmod/sloader.c.
extern "C" {
    static mut _mm_errorhandler: MikMod_handler_t;
    fn _mm_read_I_UWORD(_: *mut MREADER) -> UWORD;
    fn _mm_read_M_SWORDS(_: *mut SWORD, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn _mm_read_I_SWORDS(_: *mut SWORD, _: ::core::ffi::c_int, _: *mut MREADER) -> BOOL;
    fn MD_SampleLoad(_: *mut SAMPLOAD, _: ::core::ffi::c_int) -> SWORD;
    fn MD_SampleSpace(_: ::core::ffi::c_int) -> ULONG;
    fn MD_SampleLength(_: ::core::ffi::c_int, _: *mut SAMPLE) -> ULONG;
    static mut MikMod_errno: ::core::ffi::c_int;
    static mut MikMod_critical: BOOL;
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const MD_MUSIC: Self = Self(0);
    pub const MD_SNDFX: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ITPACK {
    pub bits: UWORD,
    pub bufbits: UWORD,
    pub last: SWORD,
    pub buf: UBYTE,
}
static mut sl_rlength: ::core::ffi::c_int = 0;
static mut sl_old: SWORD = 0;
static mut sl_buffer: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
static mut musiclist: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
static mut sndfxlist: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
#[no_mangle]
pub unsafe extern "C" fn SL_Init(mut s: *mut SAMPLOAD) -> BOOL {
    if sl_buffer.is_null() {
        sl_buffer = uqm_lua_checked!(MikMod_malloc(
            (2048 as size_t).wrapping_mul(::core::mem::size_of::<SWORD>()),
        ), 0) as *mut SWORD;
        if sl_buffer.is_null() {
            return 0 as BOOL;
        }
    }
    sl_rlength = (*s).length as ::core::ffi::c_int;
    if (*s).infmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        sl_rlength >>= 1 as ::core::ffi::c_int;
    }
    sl_old = 0 as SWORD;
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn SL_Exit(mut s: *mut SAMPLOAD) {
    if sl_rlength > 0 as ::core::ffi::c_int {
        uqm_lua_checked!((*(*s).reader)
            .Seek
            .expect(
                "non-null function pointer",
            )((*s).reader, sl_rlength as ::core::ffi::c_long, 1 as ::core::ffi::c_int), ());
    }
    MikMod_free(sl_buffer as *mut ::core::ffi::c_void);
    sl_buffer = ::core::ptr::null_mut::<SWORD>();
}
unsafe extern "C" fn read_itcompr8(
    mut status: *mut ITPACK,
    mut reader: *mut MREADER,
    mut out: *mut SWORD,
    mut count: UWORD,
    mut incnt: *mut UWORD,
) -> ::core::ffi::c_int {
    let mut dest: *mut SWORD = out;
    let mut end: *mut SWORD = out.offset(count as ::core::ffi::c_int as isize);
    let mut x: UWORD = 0;
    let mut y: UWORD = 0;
    let mut needbits: UWORD = 0;
    let mut havebits: UWORD = 0;
    let mut new_count: UWORD = 0 as UWORD;
    let mut bits: UWORD = (*status).bits;
    let mut bufbits: UWORD = (*status).bufbits;
    let mut last: SBYTE = (*status).last as SBYTE;
    let mut buf: UBYTE = (*status).buf;
    while dest < end {
        needbits = (if new_count as ::core::ffi::c_int != 0 {
            3 as ::core::ffi::c_int
        } else {
            bits as ::core::ffi::c_int
        }) as UWORD;
        havebits = 0 as UWORD;
        x = havebits;
        while needbits != 0 {
            if bufbits == 0 {
                let c2rust_fresh6 = *incnt;
                *incnt = (*incnt).wrapping_sub(1);
                if c2rust_fresh6 != 0 {
                    buf = uqm_lua_checked!((*reader).Get.expect("non-null function pointer")(reader), 0)
                        as UBYTE;
                } else {
                    buf = 0 as UBYTE;
                }
                bufbits = 8 as UWORD;
            }
            y = (if (needbits as ::core::ffi::c_int) < bufbits as ::core::ffi::c_int {
                needbits as ::core::ffi::c_int
            } else {
                bufbits as ::core::ffi::c_int
            }) as UWORD;
            x = (x as ::core::ffi::c_int
                | (buf as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << y as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) << havebits as ::core::ffi::c_int)
                as UWORD;
            buf = (buf as ::core::ffi::c_int >> y as ::core::ffi::c_int) as UBYTE;
            bufbits = (bufbits as ::core::ffi::c_int - y as ::core::ffi::c_int) as UWORD;
            needbits = (needbits as ::core::ffi::c_int - y as ::core::ffi::c_int)
                as UWORD;
            havebits = (havebits as ::core::ffi::c_int + y as ::core::ffi::c_int)
                as UWORD;
        }
        if new_count != 0 {
            new_count = 0 as UWORD;
            x = x.wrapping_add(1);
            if x as ::core::ffi::c_int >= bits as ::core::ffi::c_int {
                x = x.wrapping_add(1);
            }
            bits = x;
        } else {
            if (bits as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
                if x as ::core::ffi::c_int
                    == (1 as ::core::ffi::c_int)
                        << bits as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    new_count = 1 as UWORD;
                    continue;
                }
            } else if (bits as ::core::ffi::c_int) < 9 as ::core::ffi::c_int {
                y = ((0xff as ::core::ffi::c_int
                    >> 9 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                    - 4 as ::core::ffi::c_int) as UWORD;
                if x as ::core::ffi::c_int > y as ::core::ffi::c_int
                    && x as ::core::ffi::c_int
                        <= y as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                {
                    x = (x as ::core::ffi::c_int - y as ::core::ffi::c_int) as UWORD;
                    if x as ::core::ffi::c_int >= bits as ::core::ffi::c_int {
                        x = x.wrapping_add(1);
                    }
                    bits = x;
                    continue;
                }
            } else if (bits as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                if x as ::core::ffi::c_int >= 0x100 as ::core::ffi::c_int {
                    bits = (x as ::core::ffi::c_int - 0x100 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as UWORD;
                    continue;
                }
            } else {
                MikMod_errno = C2Rust_Unnamed::MMERR_ITPACK_INVALID_DATA.0
                    as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            if (bits as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                x = (((x as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int - bits as ::core::ffi::c_int) as SBYTE
                    as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int - bits as ::core::ffi::c_int) as UWORD;
            }
            last = (last as ::core::ffi::c_int + x as ::core::ffi::c_int) as SBYTE;
            let c2rust_fresh7 = dest;
            dest = dest.offset(1);
            *c2rust_fresh7 = ((last as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as SWORD;
        }
    }
    (*status).bits = bits;
    (*status).bufbits = bufbits;
    (*status).last = last as SWORD;
    (*status).buf = buf;
    return dest.offset_from(out) as ::core::ffi::c_int;
}
unsafe extern "C" fn read_itcompr16(
    mut status: *mut ITPACK,
    mut reader: *mut MREADER,
    mut out: *mut SWORD,
    mut count: UWORD,
    mut incnt: *mut UWORD,
) -> ::core::ffi::c_int {
    let mut dest: *mut SWORD = out;
    let mut end: *mut SWORD = out.offset(count as ::core::ffi::c_int as isize);
    let mut x: SLONG = 0;
    let mut y: SLONG = 0;
    let mut needbits: SLONG = 0;
    let mut havebits: SLONG = 0;
    let mut new_count: SLONG = 0 as SLONG;
    let mut bits: UWORD = (*status).bits;
    let mut bufbits: UWORD = (*status).bufbits;
    let mut last: SWORD = (*status).last;
    let mut buf: UBYTE = (*status).buf;
    while dest < end {
        needbits = (if new_count != 0 {
            4 as ::core::ffi::c_int
        } else {
            bits as ::core::ffi::c_int
        }) as SLONG;
        havebits = 0 as SLONG;
        x = havebits;
        while needbits != 0 {
            if bufbits == 0 {
                let c2rust_fresh8 = *incnt;
                *incnt = (*incnt).wrapping_sub(1);
                if c2rust_fresh8 != 0 {
                    buf = uqm_lua_checked!((*reader).Get.expect("non-null function pointer")(reader), 0)
                        as UBYTE;
                } else {
                    buf = 0 as UBYTE;
                }
                bufbits = 8 as UWORD;
            }
            y = if needbits < bufbits as SLONG { needbits } else { bufbits as SLONG };
            x
                |= ((buf as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << y) - 1 as ::core::ffi::c_int)
                    << havebits) as SLONG;
            buf = (buf as ::core::ffi::c_int >> y) as UBYTE;
            bufbits = (bufbits as ::core::ffi::c_int - y as UWORD as ::core::ffi::c_int)
                as UWORD;
            needbits -= y as UWORD as SLONG;
            havebits += y as UWORD as SLONG;
        }
        if new_count != 0 {
            new_count = 0 as SLONG;
            x += 1;
            if x >= bits as SLONG {
                x += 1;
            }
            bits = x as UWORD;
        } else {
            if (bits as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
                if x
                    == ((1 as ::core::ffi::c_int)
                        << bits as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SLONG
                {
                    new_count = 1 as SLONG;
                    continue;
                }
            } else if (bits as ::core::ffi::c_int) < 17 as ::core::ffi::c_int {
                y = ((0xffff as ::core::ffi::c_int
                    >> 17 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                    - 8 as ::core::ffi::c_int) as SLONG;
                if x > y && x <= y + 16 as SLONG {
                    x -= y;
                    if x >= bits as SLONG {
                        x += 1;
                    }
                    bits = x as UWORD;
                    continue;
                }
            } else if (bits as ::core::ffi::c_int) < 18 as ::core::ffi::c_int {
                if x >= 0x10000 as SLONG {
                    bits = (x - 0x10000 as SLONG + 1 as SLONG) as UWORD;
                    continue;
                }
            } else {
                MikMod_errno = C2Rust_Unnamed::MMERR_ITPACK_INVALID_DATA.0
                    as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            if (bits as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                x = ((x << 16 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                    as SWORD as ::core::ffi::c_int
                    >> 16 as ::core::ffi::c_int - bits as ::core::ffi::c_int) as SLONG;
            }
            last = (last as SLONG + x) as SWORD;
            let c2rust_fresh9 = dest;
            dest = dest.offset(1);
            *c2rust_fresh9 = last;
        }
    }
    (*status).bits = bits;
    (*status).bufbits = bufbits;
    (*status).last = last;
    (*status).buf = buf;
    return dest.offset_from(out) as ::core::ffi::c_int;
}
unsafe extern "C" fn SL_LoadInternal(
    mut buffer: *mut ::core::ffi::c_void,
    mut infmt: UWORD,
    mut outfmt: UWORD,
    mut scalefactor: ::core::ffi::c_int,
    mut length: ULONG,
    mut reader: *mut MREADER,
    mut dither: BOOL,
) -> ::core::ffi::c_int {
    let mut bptr: *mut SBYTE = buffer as *mut SBYTE;
    let mut wptr: *mut SWORD = buffer as *mut SWORD;
    let mut stodo: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 0;
    let mut c_block: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut status: ITPACK = ITPACK {
        bits: 0,
        bufbits: 0,
        last: 0,
        buf: 0,
    };
    let mut incnt: UWORD = 0 as UWORD;
    status.buf = 0 as UBYTE;
    status.last = 0 as SWORD;
    status.bufbits = 0 as UWORD;
    status.bits = 0 as UWORD;
    while length != 0 {
        stodo = (if length < 2048 as ULONG { length } else { 2048 as ULONG })
            as ::core::ffi::c_int;
        if infmt as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
            sl_rlength = 0 as ::core::ffi::c_int;
            if c_block == 0 {
                status.bits = (if infmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                    != 0
                {
                    17 as ::core::ffi::c_int
                } else {
                    9 as ::core::ffi::c_int
                }) as UWORD;
                status.bufbits = 0 as UWORD;
                status.last = status.bufbits as SWORD;
                incnt = uqm_lua_checked!(_mm_read_I_UWORD(reader), 0);
                c_block = if infmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                {
                    0x4000 as ::core::ffi::c_int
                } else {
                    0x8000 as ::core::ffi::c_int
                };
                if infmt as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                    sl_old = 0 as SWORD;
                }
            }
            if infmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                result = uqm_lua_checked!(read_itcompr16(
                    &raw mut status,
                    reader,
                    sl_buffer,
                    stodo as UWORD,
                    &raw mut incnt,
                ), 0);
                if result == 0 {
                    return 1 as ::core::ffi::c_int;
                }
            } else {
                result = uqm_lua_checked!(read_itcompr8(
                    &raw mut status,
                    reader,
                    sl_buffer,
                    stodo as UWORD,
                    &raw mut incnt,
                ), 0);
                if result == 0 {
                    return 1 as ::core::ffi::c_int;
                }
            }
            if result != stodo {
                MikMod_errno = C2Rust_Unnamed::MMERR_ITPACK_INVALID_DATA.0
                    as ::core::ffi::c_int;
                return 1 as ::core::ffi::c_int;
            }
            c_block -= stodo;
        } else {
            if infmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                if uqm_lua_checked!((*reader).Eof.expect("non-null function pointer")(reader), 0) != 0 {
                    MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_STREAM.0
                        as ::core::ffi::c_int;
                    return 1 as ::core::ffi::c_int;
                }
                if infmt as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
                    uqm_lua_checked!(_mm_read_M_SWORDS(sl_buffer, stodo, reader), 0);
                } else {
                    uqm_lua_checked!(_mm_read_I_SWORDS(sl_buffer, stodo, reader), 0);
                }
            } else {
                let mut src: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
                let mut dest: *mut SWORD = ::core::ptr::null_mut::<SWORD>();
                if uqm_lua_checked!((*reader).Eof.expect("non-null function pointer")(reader), 0) != 0 {
                    MikMod_errno = C2Rust_Unnamed::MMERR_NOT_A_STREAM.0
                        as ::core::ffi::c_int;
                    return 1 as ::core::ffi::c_int;
                }
                uqm_lua_checked!((*reader)
                    .Read
                    .expect(
                        "non-null function pointer",
                    )(
                    reader,
                    sl_buffer as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<SBYTE>().wrapping_mul(stodo as size_t),
                ), 0);
                src = sl_buffer as *mut SBYTE;
                dest = sl_buffer;
                src = src.offset(stodo as isize);
                dest = dest.offset(stodo as isize);
                t = 0 as ::core::ffi::c_int;
                while t < stodo {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = ((*src as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        as SWORD;
                    t += 1;
                }
            }
            sl_rlength -= stodo;
        }
        if infmt as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
            t = 0 as ::core::ffi::c_int;
            while t < stodo {
                *sl_buffer.offset(t as isize) = (*sl_buffer.offset(t as isize)
                    as ::core::ffi::c_int + sl_old as ::core::ffi::c_int) as SWORD;
                sl_old = *sl_buffer.offset(t as isize);
                t += 1;
            }
        }
        if (infmt as ::core::ffi::c_int ^ outfmt as ::core::ffi::c_int)
            & 0x4 as ::core::ffi::c_int != 0
        {
            t = 0 as ::core::ffi::c_int;
            while t < stodo {
                *sl_buffer.offset(t as isize) = (*sl_buffer.offset(t as isize)
                    as ::core::ffi::c_int ^ 0x8000 as ::core::ffi::c_int) as SWORD;
                t += 1;
            }
        }
        if scalefactor != 0 {
            let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut scaleval: SLONG = 0;
            t = 0 as ::core::ffi::c_int;
            while t < stodo && length != 0 {
                scaleval = 0 as SLONG;
                u = scalefactor;
                while u != 0 && t < stodo {
                    scaleval += *sl_buffer.offset(t as isize) as SLONG;
                    u -= 1;
                    t += 1;
                }
                let c2rust_fresh0 = idx;
                idx += 1;
                *sl_buffer.offset(c2rust_fresh0 as isize) = (scaleval
                    / (scalefactor - u) as SLONG) as UWORD as SWORD;
                length = length.wrapping_sub(1);
            }
            stodo = idx;
        } else {
            length = length.wrapping_sub(stodo as ULONG);
        }
        if dither != 0 {
            if infmt as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0
                && outfmt as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0
            {
                let mut avgval: SLONG = 0;
                let mut idx_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                t = 0 as ::core::ffi::c_int;
                while t < stodo && length != 0 {
                    let c2rust_fresh1 = t;
                    t += 1;
                    avgval = *sl_buffer.offset(c2rust_fresh1 as isize) as SLONG;
                    let c2rust_fresh2 = t;
                    t += 1;
                    avgval += *sl_buffer.offset(c2rust_fresh2 as isize) as SLONG;
                    let c2rust_fresh3 = idx_0;
                    idx_0 += 1;
                    *sl_buffer.offset(c2rust_fresh3 as isize) = (avgval
                        >> 1 as ::core::ffi::c_int) as SWORD;
                    length = length.wrapping_sub(2 as ULONG);
                }
                stodo = idx_0;
            }
        }
        if outfmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
            t = 0 as ::core::ffi::c_int;
            while t < stodo {
                let c2rust_fresh4 = wptr;
                wptr = wptr.offset(1);
                *c2rust_fresh4 = *sl_buffer.offset(t as isize);
                t += 1;
            }
        } else {
            t = 0 as ::core::ffi::c_int;
            while t < stodo {
                let c2rust_fresh5 = bptr;
                bptr = bptr.offset(1);
                *c2rust_fresh5 = (*sl_buffer.offset(t as isize) as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int) as SBYTE;
                t += 1;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SL_Load(
    mut buffer: *mut ::core::ffi::c_void,
    mut smp: *mut SAMPLOAD,
    mut length: ULONG,
) -> ::core::ffi::c_int {
    return uqm_lua_checked!(SL_LoadInternal(
        buffer,
        (*smp).infmt,
        (*smp).outfmt,
        (*smp).scalefactor,
        length,
        (*smp).reader,
        0 as BOOL,
    ), 0);
}
#[no_mangle]
pub unsafe extern "C" fn SL_RegisterSample(
    mut s: *mut SAMPLE,
    mut r#type: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> *mut SAMPLOAD {
    let mut news: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
    let mut samplist: *mut *mut SAMPLOAD = ::core::ptr::null_mut::<*mut SAMPLOAD>();
    let mut cruise: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
    if r#type == C2Rust_Unnamed_0::MD_MUSIC.0 as ::core::ffi::c_int {
        samplist = &raw mut musiclist;
        cruise = musiclist;
    } else if r#type == C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int {
        samplist = &raw mut sndfxlist;
        cruise = sndfxlist;
    } else {
        return ::core::ptr::null_mut::<SAMPLOAD>()
    }
    news = uqm_lua_checked!(MikMod_malloc(::core::mem::size_of::<SAMPLOAD>()), ::core::ptr::null_mut()) as *mut SAMPLOAD;
    if news.is_null() {
        return ::core::ptr::null_mut::<SAMPLOAD>();
    }
    if !cruise.is_null() {
        while !(*cruise).next.is_null() {
            cruise = (*cruise).next as *mut SAMPLOAD;
        }
        (*cruise).next = news as *mut SAMPLOAD;
    } else {
        *samplist = news;
    }
    (*news).infmt = ((*s).flags as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
        as UWORD;
    (*news).outfmt = (*news).infmt;
    (*news).reader = reader;
    (*news).sample = s;
    (*news).length = (*s).length;
    (*news).loopstart = (*s).loopstart;
    (*news).loopend = (*s).loopend;
    return news;
}
unsafe extern "C" fn FreeSampleList(mut s: *mut SAMPLOAD) {
    let mut old: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
    while !s.is_null() {
        old = s;
        s = (*s).next as *mut SAMPLOAD;
        MikMod_free(old as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn SampleTotal(
    mut samplist: *mut SAMPLOAD,
    mut r#type: ::core::ffi::c_int,
) -> ULONG {
    let mut total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !samplist.is_null() {
        (*(*samplist).sample).flags = ((*(*samplist).sample).flags as ::core::ffi::c_int
            & !(0x3f as ::core::ffi::c_int) | (*samplist).outfmt as ::core::ffi::c_int)
            as UWORD;
        total = (total as ULONG)
            .wrapping_add(uqm_lua_checked!(MD_SampleLength(r#type, (*samplist).sample), 0))
            as ::core::ffi::c_int;
        samplist = (*samplist).next as *mut SAMPLOAD;
    }
    return total as ULONG;
}
unsafe extern "C" fn RealSpeed(mut s: *mut SAMPLOAD) -> ULONG {
    return (*(*s).sample)
        .speed
        .wrapping_div(
            (if (*s).scalefactor != 0 {
                (*s).scalefactor
            } else {
                1 as ::core::ffi::c_int
            }) as ULONG,
        );
}
unsafe extern "C" fn DitherSamples(
    mut samplist: *mut SAMPLOAD,
    mut r#type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c2smp: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
    let mut maxsize: ULONG = 0;
    let mut speed: ULONG = 0;
    let mut s: *mut SAMPLOAD = ::core::ptr::null_mut::<SAMPLOAD>();
    if samplist.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    maxsize = uqm_lua_checked!(MD_SampleSpace(r#type), 0).wrapping_mul(1024 as ULONG);
    if maxsize != 0 as ULONG {
        while uqm_lua_checked!(SampleTotal(samplist, r#type), 0) > maxsize {
            s = samplist;
            while !s.is_null() {
                if (*s).outfmt as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                    SL_Sample16to8(s);
                    break;
                } else {
                    s = (*s).next as *mut SAMPLOAD;
                }
            }
            if s.is_null() {
                s = samplist;
                speed = 0 as ULONG;
                while !s.is_null() {
                    if (*(*s).sample).length != 0 && RealSpeed(s) > speed {
                        speed = RealSpeed(s);
                        c2smp = s;
                    }
                    s = (*s).next as *mut SAMPLOAD;
                }
                if !c2smp.is_null() {
                    SL_HalveSample(c2smp, 2 as ::core::ffi::c_int);
                }
            }
        }
    }
    s = samplist;
    while !s.is_null() {
        if (*(*s).sample).length != 0 {
            if (*(*s).sample).seekpos != 0 {
                uqm_lua_checked!((*(*s).reader)
                    .Seek
                    .expect(
                        "non-null function pointer",
                    )(
                    (*s).reader,
                    (*(*s).sample).seekpos as ::core::ffi::c_long,
                    0 as ::core::ffi::c_int,
                ), 0);
            }
            (*(*s).sample).handle = uqm_lua_checked!(MD_SampleLoad(s, r#type), 0);
            (*(*s).sample).flags = ((*(*s).sample).flags as ::core::ffi::c_int
                & !(0x3f as ::core::ffi::c_int) | (*s).outfmt as ::core::ffi::c_int)
                as UWORD;
            if ((*(*s).sample).handle as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                FreeSampleList(samplist);
                if _mm_errorhandler.is_some() {
                    uqm_lua_checked!(_mm_errorhandler.expect("non-null function pointer")(), 0);
                }
                return 1 as ::core::ffi::c_int;
            }
        }
        s = (*s).next as *mut SAMPLOAD;
    }
    FreeSampleList(samplist);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SL_LoadSamples() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    MikMod_critical = 0 as ::core::ffi::c_int as BOOL;
    if musiclist.is_null() && sndfxlist.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    rc = (uqm_lua_checked!(DitherSamples(musiclist, C2Rust_Unnamed_0::MD_MUSIC.0 as ::core::ffi::c_int), 0)
        != 0
        || uqm_lua_checked!(DitherSamples(sndfxlist, C2Rust_Unnamed_0::MD_SNDFX.0 as ::core::ffi::c_int), 0)
            != 0) as ::core::ffi::c_int;
    sndfxlist = ::core::ptr::null_mut::<SAMPLOAD>();
    musiclist = sndfxlist;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn SL_Sample16to8(mut s: *mut SAMPLOAD) {
    (*s).outfmt = ((*s).outfmt as ::core::ffi::c_int & !(0x1 as ::core::ffi::c_int))
        as UWORD;
    (*(*s).sample).flags = ((*(*s).sample).flags as ::core::ffi::c_int
        & !(0x3f as ::core::ffi::c_int) | (*s).outfmt as ::core::ffi::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn SL_Sample8to16(mut s: *mut SAMPLOAD) {
    (*s).outfmt = ((*s).outfmt as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
        as UWORD;
    (*(*s).sample).flags = ((*(*s).sample).flags as ::core::ffi::c_int
        & !(0x3f as ::core::ffi::c_int) | (*s).outfmt as ::core::ffi::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn SL_SampleSigned(mut s: *mut SAMPLOAD) {
    (*s).outfmt = ((*s).outfmt as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        as UWORD;
    (*(*s).sample).flags = ((*(*s).sample).flags as ::core::ffi::c_int
        & !(0x3f as ::core::ffi::c_int) | (*s).outfmt as ::core::ffi::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn SL_SampleUnsigned(mut s: *mut SAMPLOAD) {
    (*s).outfmt = ((*s).outfmt as ::core::ffi::c_int & !(0x4 as ::core::ffi::c_int))
        as UWORD;
    (*(*s).sample).flags = ((*(*s).sample).flags as ::core::ffi::c_int
        & !(0x3f as ::core::ffi::c_int) | (*s).outfmt as ::core::ffi::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn SL_HalveSample(
    mut s: *mut SAMPLOAD,
    mut factor: ::core::ffi::c_int,
) {
    (*s).scalefactor = if factor > 0 as ::core::ffi::c_int {
        factor
    } else {
        2 as ::core::ffi::c_int
    };
    (*(*s).sample).divfactor = (*s).scalefactor as UBYTE;
    (*(*s).sample).length = (*s).length.wrapping_div((*s).scalefactor as ULONG);
    (*(*s).sample).loopstart = (*s).loopstart.wrapping_div((*s).scalefactor as ULONG);
    (*(*s).sample).loopend = (*s).loopend.wrapping_div((*s).scalefactor as ULONG);
}
