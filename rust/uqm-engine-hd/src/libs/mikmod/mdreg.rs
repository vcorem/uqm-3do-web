/*	MikMod sound library
	(c) 1998, 1999 Miodrag Vallat and others - see file AUTHORS for
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

  Routine for registering all drivers in libmikmod for the current platform.

==============================================================================*/

// Rust translation of src/libs/mikmod/mdreg.c.
extern "C" {
    static mut drv_nos: MDRIVER;
    fn _mm_registerdriver(_: *mut MDRIVER);
}
pub type size_t = usize;
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
pub type UBYTE = ::core::ffi::c_uchar;
pub type SWORD = ::core::ffi::c_short;
pub type UWORD = ::core::ffi::c_ushort;
pub type SLONG = ::core::ffi::c_long;
pub type ULONG = ::core::ffi::c_ulong;
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
unsafe extern "C" fn _mm_registeralldrivers() {
    _mm_registerdriver(&raw mut drv_nos);
}
#[no_mangle]
pub unsafe extern "C" fn MikMod_RegisterAllDrivers() {
    _mm_registeralldrivers();
}
