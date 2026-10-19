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

  Error handling functions.
  Register an error handler with _mm_RegisterErrorHandler() and you're all set.

==============================================================================*/

/*

	The global variables _mm_errno, and _mm_critical are set before the error
	handler in called.  See below for the values of these variables.

*/

// Rust translation of src/libs/mikmod/mmerror.c.
pub type BOOL = ::core::ffi::c_int;
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
static mut _mm_errmsg: [*const ::core::ffi::c_char; 99] = [
    b"No error\0".as_ptr() as *const ::core::ffi::c_char,
    b"Could not open requested file\0".as_ptr() as *const ::core::ffi::c_char,
    b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
    b"Dynamic linking failed\0".as_ptr() as *const ::core::ffi::c_char,
    b"Out of memory to load sample\0".as_ptr() as *const ::core::ffi::c_char,
    b"Out of sample handles to load sample\0".as_ptr() as *const ::core::ffi::c_char,
    b"Sample format not recognized\0".as_ptr() as *const ::core::ffi::c_char,
    b"Failure loading module pattern\0".as_ptr() as *const ::core::ffi::c_char,
    b"Failure loading module track\0".as_ptr() as *const ::core::ffi::c_char,
    b"Failure loading module header\0".as_ptr() as *const ::core::ffi::c_char,
    b"Failure loading sampleinfo\0".as_ptr() as *const ::core::ffi::c_char,
    b"Module format not recognized\0".as_ptr() as *const ::core::ffi::c_char,
    b"Module sample format not recognized\0".as_ptr() as *const ::core::ffi::c_char,
    b"Synthsounds not supported in MED files\0".as_ptr() as *const ::core::ffi::c_char,
    b"Compressed sample is invalid\0".as_ptr() as *const ::core::ffi::c_char,
    b"Sound device not detected\0".as_ptr() as *const ::core::ffi::c_char,
    b"Device number out of range\0".as_ptr() as *const ::core::ffi::c_char,
    b"Software mixer failure\0".as_ptr() as *const ::core::ffi::c_char,
    b"Could not open sound device\0".as_ptr() as *const ::core::ffi::c_char,
    b"This driver supports 8 bit linear output only\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"This driver supports 16 bit linear output only\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"This driver supports stereo output only\0".as_ptr() as *const ::core::ffi::c_char,
    b"This driver supports uLaw output (8 bit mono, 8 kHz) only\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"Unable to set non-blocking mode for audio device\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"This driver doesn't support 32 bit float output\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid error code\0".as_ptr() as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn MikMod_strerror(
    mut code: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if code < 0 as ::core::ffi::c_int
        || code > C2Rust_Unnamed::MMERR_MAX.0 as ::core::ffi::c_int
    {
        code = C2Rust_Unnamed::MMERR_MAX.0 as ::core::ffi::c_int;
    }
    return _mm_errmsg[code as usize];
}
#[no_mangle]
pub static mut _mm_errorhandler: MikMod_handler_t = None;
#[no_mangle]
pub static mut MikMod_errno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut MikMod_critical: BOOL = 0 as BOOL;
unsafe extern "C" fn _mm_registererrorhandler(
    mut proc: MikMod_handler_t,
) -> MikMod_handler_t {
    let mut oldproc: MikMod_handler_t = _mm_errorhandler;
    _mm_errorhandler = proc;
    return oldproc;
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_RegisterErrorHandler(
    mut proc: MikMod_handler_t,
) -> MikMod_handler_t {
    let mut result: MikMod_handler_t = None;
    result = _mm_registererrorhandler(proc);
    return result;
}
