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

  Driver for no output

==============================================================================*/

/*

	Written by Jean-Paul Mikkers <mikmak@via.nl>

*/

// Rust translation of src/libs/mikmod/drv_nos.c.
extern "C" {
    fn MikMod_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
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
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
pub type SBYTE = ::core::ffi::c_schar;
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
static mut zerobuf: *mut SBYTE = ::core::ptr::null_mut::<SBYTE>();
unsafe extern "C" fn NS_IsThere() -> BOOL {
    return 1 as BOOL;
}
unsafe extern "C" fn NS_Init() -> ::core::ffi::c_int {
    zerobuf = MikMod_malloc(32768 as size_t) as *mut SBYTE;
    return VC_Init();
}
unsafe extern "C" fn NS_Exit() {
    VC_Exit();
    MikMod_free(zerobuf as *mut ::core::ffi::c_void);
    zerobuf = ::core::ptr::null_mut::<SBYTE>();
}
unsafe extern "C" fn NS_Update() {
    if !zerobuf.is_null() {
        VC_WriteBytes(zerobuf, 32768 as ULONG);
    }
}
#[no_mangle]
pub static mut drv_nos: MDRIVER = MDRIVER {
    next: ::core::ptr::null_mut::<MDRIVER>(),
    Name: b"No Sound\0".as_ptr() as *const CHAR,
    Version: b"Nosound Driver v3.0\0".as_ptr() as *const CHAR,
    HardVoiceLimit: 255 as UBYTE,
    SoftVoiceLimit: 255 as UBYTE,
    Alias: b"nosound\0".as_ptr() as *const CHAR,
    CmdLineHelp: ::core::ptr::null::<CHAR>(),
    CommandLine: None,
    IsPresent: Some(NS_IsThere as unsafe extern "C" fn() -> BOOL),
    SampleLoad: Some(
        VC_SampleLoad as unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD,
    ),
    SampleUnload: Some(VC_SampleUnload as unsafe extern "C" fn(SWORD) -> ()),
    FreeSampleSpace: Some(
        VC_SampleSpace as unsafe extern "C" fn(::core::ffi::c_int) -> ULONG,
    ),
    RealSampleLength: Some(
        VC_SampleLength as unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG,
    ),
    Init: Some(NS_Init as unsafe extern "C" fn() -> ::core::ffi::c_int),
    Exit: Some(NS_Exit as unsafe extern "C" fn() -> ()),
    Reset: None,
    SetNumVoices: Some(VC_SetNumVoices as unsafe extern "C" fn() -> ::core::ffi::c_int),
    PlayStart: Some(VC_PlayStart as unsafe extern "C" fn() -> ::core::ffi::c_int),
    PlayStop: Some(VC_PlayStop as unsafe extern "C" fn() -> ()),
    Update: Some(NS_Update as unsafe extern "C" fn() -> ()),
    Pause: None,
    VoiceSetVolume: Some(VC_VoiceSetVolume as unsafe extern "C" fn(UBYTE, UWORD) -> ()),
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
    VoiceGetPanning: Some(VC_VoiceGetPanning as unsafe extern "C" fn(UBYTE) -> ULONG),
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
    VoiceGetPosition: Some(VC_VoiceGetPosition as unsafe extern "C" fn(UBYTE) -> SLONG),
    VoiceRealVolume: Some(VC_VoiceRealVolume as unsafe extern "C" fn(UBYTE) -> ULONG),
};
