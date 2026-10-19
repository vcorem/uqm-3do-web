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

  Common source parts between the two software mixers.
  This file is probably the ugliest part of libmikmod...

==============================================================================*/

// Rust translation of src/libs/mikmod/virtch_common.c.
extern "C" {
    fn VC1_Init() -> ::core::ffi::c_int;
    fn VC2_Init() -> ::core::ffi::c_int;
    fn VC1_PlayStart() -> ::core::ffi::c_int;
    fn VC2_PlayStart() -> ::core::ffi::c_int;
    fn VC1_PlayStop();
    fn VC2_PlayStop();
    fn VC1_SetNumVoices() -> ::core::ffi::c_int;
    fn VC2_SetNumVoices() -> ::core::ffi::c_int;
    static mut md_mode: UWORD;
    fn VC1_SilenceBytes(_: *mut SBYTE, _: ULONG) -> ULONG;
    fn VC2_SilenceBytes(_: *mut SBYTE, _: ULONG) -> ULONG;
    fn VC1_WriteBytes(_: *mut SBYTE, _: ULONG) -> ULONG;
    fn VC2_WriteBytes(_: *mut SBYTE, _: ULONG) -> ULONG;
    fn VC1_Exit();
    fn VC2_Exit();
    fn VC1_VoiceGetVolume(_: UBYTE) -> UWORD;
    fn VC2_VoiceGetVolume(_: UBYTE) -> UWORD;
    fn VC1_VoiceGetPanning(_: UBYTE) -> ULONG;
    fn VC2_VoiceGetPanning(_: UBYTE) -> ULONG;
    fn VC1_VoiceSetFrequency(_: UBYTE, _: ULONG);
    fn VC2_VoiceSetFrequency(_: UBYTE, _: ULONG);
    fn VC1_VoiceGetFrequency(_: UBYTE) -> ULONG;
    fn VC2_VoiceGetFrequency(_: UBYTE) -> ULONG;
    fn VC1_VoicePlay(
        _: UBYTE,
        _: SWORD,
        _: ULONG,
        _: ULONG,
        _: ULONG,
        _: ULONG,
        _: UWORD,
    );
    fn VC2_VoicePlay(
        _: UBYTE,
        _: SWORD,
        _: ULONG,
        _: ULONG,
        _: ULONG,
        _: ULONG,
        _: UWORD,
    );
    fn VC1_VoiceStop(_: UBYTE);
    fn VC2_VoiceStop(_: UBYTE);
    fn VC1_VoiceStopped(_: UBYTE) -> BOOL;
    fn VC2_VoiceStopped(_: UBYTE) -> BOOL;
    fn VC1_VoiceGetPosition(_: UBYTE) -> SLONG;
    fn VC2_VoiceGetPosition(_: UBYTE) -> SLONG;
    fn VC1_VoiceSetVolume(_: UBYTE, _: UWORD);
    fn VC2_VoiceSetVolume(_: UBYTE, _: UWORD);
    fn VC1_VoiceSetPanning(_: UBYTE, _: ULONG);
    fn VC2_VoiceSetPanning(_: UBYTE, _: ULONG);
    fn VC1_SampleUnload(_: SWORD);
    fn VC2_SampleUnload(_: SWORD);
    fn VC1_SampleLoad(_: *mut SAMPLOAD, _: ::core::ffi::c_int) -> SWORD;
    fn VC2_SampleLoad(_: *mut SAMPLOAD, _: ::core::ffi::c_int) -> SWORD;
    fn VC1_SampleSpace(_: ::core::ffi::c_int) -> ULONG;
    fn VC2_SampleSpace(_: ::core::ffi::c_int) -> ULONG;
    fn VC1_SampleLength(_: ::core::ffi::c_int, _: *mut SAMPLE) -> ULONG;
    fn VC2_SampleLength(_: ::core::ffi::c_int, _: *mut SAMPLE) -> ULONG;
    fn VC1_VoiceRealVolume(_: UBYTE) -> ULONG;
    fn VC2_VoiceRealVolume(_: UBYTE) -> ULONG;
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
static mut VC_Init_ptr: Option<unsafe extern "C" fn() -> ::core::ffi::c_int> = Some(
    VC1_Init as unsafe extern "C" fn() -> ::core::ffi::c_int,
);
static mut VC_Exit_ptr: Option<unsafe extern "C" fn() -> ()> = Some(
    VC1_Exit as unsafe extern "C" fn() -> (),
);
static mut VC_SetNumVoices_ptr: Option<unsafe extern "C" fn() -> ::core::ffi::c_int> = None;
static mut VC_SampleSpace_ptr: Option<
    unsafe extern "C" fn(::core::ffi::c_int) -> ULONG,
> = None;
static mut VC_SampleLength_ptr: Option<
    unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG,
> = None;
static mut VC_PlayStart_ptr: Option<unsafe extern "C" fn() -> ::core::ffi::c_int> = None;
static mut VC_PlayStop_ptr: Option<unsafe extern "C" fn() -> ()> = None;
static mut VC_SampleLoad_ptr: Option<
    unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD,
> = None;
static mut VC_SampleUnload_ptr: Option<unsafe extern "C" fn(SWORD) -> ()> = None;
static mut VC_WriteBytes_ptr: Option<unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG> = None;
static mut VC_SilenceBytes_ptr: Option<
    unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG,
> = None;
static mut VC_VoiceSetVolume_ptr: Option<unsafe extern "C" fn(UBYTE, UWORD) -> ()> = None;
static mut VC_VoiceGetVolume_ptr: Option<unsafe extern "C" fn(UBYTE) -> UWORD> = None;
static mut VC_VoiceSetFrequency_ptr: Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()> = None;
static mut VC_VoiceGetFrequency_ptr: Option<unsafe extern "C" fn(UBYTE) -> ULONG> = None;
static mut VC_VoiceSetPanning_ptr: Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()> = None;
static mut VC_VoiceGetPanning_ptr: Option<unsafe extern "C" fn(UBYTE) -> ULONG> = None;
static mut VC_VoicePlay_ptr: Option<
    unsafe extern "C" fn(UBYTE, SWORD, ULONG, ULONG, ULONG, ULONG, UWORD) -> (),
> = None;
static mut VC_VoiceStop_ptr: Option<unsafe extern "C" fn(UBYTE) -> ()> = None;
static mut VC_VoiceStopped_ptr: Option<unsafe extern "C" fn(UBYTE) -> BOOL> = None;
static mut VC_VoiceGetPosition_ptr: Option<unsafe extern "C" fn(UBYTE) -> SLONG> = None;
static mut VC_VoiceRealVolume_ptr: Option<unsafe extern "C" fn(UBYTE) -> ULONG> = None;
#[no_mangle]
pub unsafe extern "C" fn VC_Init() -> ::core::ffi::c_int {
    return uqm_lua_checked!(VC_Init_ptr.expect("non-null function pointer")(), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_Exit() {
    uqm_lua_checked!(VC_Exit_ptr.expect("non-null function pointer")(), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_SetNumVoices() -> ::core::ffi::c_int {
    return uqm_lua_checked!(VC_SetNumVoices_ptr.expect("non-null function pointer")(), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_SampleSpace(mut a: ::core::ffi::c_int) -> ULONG {
    return uqm_lua_checked!(VC_SampleSpace_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_SampleLength(
    mut a: ::core::ffi::c_int,
    mut b: *mut SAMPLE,
) -> ULONG {
    return uqm_lua_checked!(VC_SampleLength_ptr.expect("non-null function pointer")(a, b), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_PlayStart() -> ::core::ffi::c_int {
    return uqm_lua_checked!(VC_PlayStart_ptr.expect("non-null function pointer")(), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_PlayStop() {
    uqm_lua_checked!(VC_PlayStop_ptr.expect("non-null function pointer")(), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_SampleLoad(
    mut a: *mut SAMPLOAD,
    mut b: ::core::ffi::c_int,
) -> SWORD {
    return uqm_lua_checked!(VC_SampleLoad_ptr.expect("non-null function pointer")(a, b), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_SampleUnload(mut a: SWORD) {
    uqm_lua_checked!(VC_SampleUnload_ptr.expect("non-null function pointer")(a), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_WriteBytes(mut a: *mut SBYTE, mut b: ULONG) -> ULONG {
    return uqm_lua_checked!(VC_WriteBytes_ptr.expect("non-null function pointer")(a, b), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_SilenceBytes(mut a: *mut SBYTE, mut b: ULONG) -> ULONG {
    return uqm_lua_checked!(VC_SilenceBytes_ptr.expect("non-null function pointer")(a, b), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceSetVolume(mut a: UBYTE, mut b: UWORD) {
    uqm_lua_checked!(VC_VoiceSetVolume_ptr.expect("non-null function pointer")(a, b), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceGetVolume(mut a: UBYTE) -> UWORD {
    return uqm_lua_checked!(VC_VoiceGetVolume_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceSetFrequency(mut a: UBYTE, mut b: ULONG) {
    uqm_lua_checked!(VC_VoiceSetFrequency_ptr.expect("non-null function pointer")(a, b), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceGetFrequency(mut a: UBYTE) -> ULONG {
    return uqm_lua_checked!(VC_VoiceGetFrequency_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceSetPanning(mut a: UBYTE, mut b: ULONG) {
    uqm_lua_checked!(VC_VoiceSetPanning_ptr.expect("non-null function pointer")(a, b), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceGetPanning(mut a: UBYTE) -> ULONG {
    return uqm_lua_checked!(VC_VoiceGetPanning_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoicePlay(
    mut a: UBYTE,
    mut b: SWORD,
    mut c: ULONG,
    mut d: ULONG,
    mut e: ULONG,
    mut f: ULONG,
    mut g: UWORD,
) {
    uqm_lua_checked!(VC_VoicePlay_ptr.expect("non-null function pointer")(a, b, c, d, e, f, g), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceStop(mut a: UBYTE) {
    uqm_lua_checked!(VC_VoiceStop_ptr.expect("non-null function pointer")(a), ());
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceStopped(mut a: UBYTE) -> BOOL {
    return uqm_lua_checked!(VC_VoiceStopped_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceGetPosition(mut a: UBYTE) -> SLONG {
    return uqm_lua_checked!(VC_VoiceGetPosition_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_VoiceRealVolume(mut a: UBYTE) -> ULONG {
    return uqm_lua_checked!(VC_VoiceRealVolume_ptr.expect("non-null function pointer")(a), 0);
}
#[no_mangle]
pub unsafe extern "C" fn VC_SetupPointers() {
    if md_mode as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
        VC_Init_ptr = Some(VC2_Init as unsafe extern "C" fn() -> ::core::ffi::c_int)
            as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
        VC_Exit_ptr = Some(VC2_Exit as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
        VC_SetNumVoices_ptr = Some(
            VC2_SetNumVoices as unsafe extern "C" fn() -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
        VC_SampleSpace_ptr = Some(
            VC2_SampleSpace as unsafe extern "C" fn(::core::ffi::c_int) -> ULONG,
        ) as Option<unsafe extern "C" fn(::core::ffi::c_int) -> ULONG>;
        VC_SampleLength_ptr = Some(
            VC2_SampleLength
                as unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG,
        ) as Option<unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG>;
        VC_PlayStart_ptr = Some(
            VC2_PlayStart as unsafe extern "C" fn() -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
        VC_PlayStop_ptr = Some(VC2_PlayStop as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
        VC_SampleLoad_ptr = Some(
            VC2_SampleLoad
                as unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD,
        ) as Option<unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD>;
        VC_SampleUnload_ptr = Some(VC2_SampleUnload as unsafe extern "C" fn(SWORD) -> ())
            as Option<unsafe extern "C" fn(SWORD) -> ()>;
        VC_WriteBytes_ptr = Some(
            VC2_WriteBytes as unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG,
        ) as Option<unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG>;
        VC_SilenceBytes_ptr = Some(
            VC2_SilenceBytes as unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG,
        ) as Option<unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG>;
        VC_VoiceSetVolume_ptr = Some(
            VC2_VoiceSetVolume as unsafe extern "C" fn(UBYTE, UWORD) -> (),
        ) as Option<unsafe extern "C" fn(UBYTE, UWORD) -> ()>;
        VC_VoiceGetVolume_ptr = Some(
            VC2_VoiceGetVolume as unsafe extern "C" fn(UBYTE) -> UWORD,
        ) as Option<unsafe extern "C" fn(UBYTE) -> UWORD>;
        VC_VoiceSetFrequency_ptr = Some(
            VC2_VoiceSetFrequency as unsafe extern "C" fn(UBYTE, ULONG) -> (),
        ) as Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()>;
        VC_VoiceGetFrequency_ptr = Some(
            VC2_VoiceGetFrequency as unsafe extern "C" fn(UBYTE) -> ULONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> ULONG>;
        VC_VoiceSetPanning_ptr = Some(
            VC2_VoiceSetPanning as unsafe extern "C" fn(UBYTE, ULONG) -> (),
        ) as Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()>;
        VC_VoiceGetPanning_ptr = Some(
            VC2_VoiceGetPanning as unsafe extern "C" fn(UBYTE) -> ULONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> ULONG>;
        VC_VoicePlay_ptr = Some(
            VC2_VoicePlay
                as unsafe extern "C" fn(
                    UBYTE,
                    SWORD,
                    ULONG,
                    ULONG,
                    ULONG,
                    ULONG,
                    UWORD,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    UBYTE,
                    SWORD,
                    ULONG,
                    ULONG,
                    ULONG,
                    ULONG,
                    UWORD,
                ) -> (),
            >;
        VC_VoiceStop_ptr = Some(VC2_VoiceStop as unsafe extern "C" fn(UBYTE) -> ())
            as Option<unsafe extern "C" fn(UBYTE) -> ()>;
        VC_VoiceStopped_ptr = Some(
            VC2_VoiceStopped as unsafe extern "C" fn(UBYTE) -> BOOL,
        ) as Option<unsafe extern "C" fn(UBYTE) -> BOOL>;
        VC_VoiceGetPosition_ptr = Some(
            VC2_VoiceGetPosition as unsafe extern "C" fn(UBYTE) -> SLONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> SLONG>;
        VC_VoiceRealVolume_ptr = Some(
            VC2_VoiceRealVolume as unsafe extern "C" fn(UBYTE) -> ULONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> ULONG>;
    } else {
        VC_Init_ptr = Some(VC1_Init as unsafe extern "C" fn() -> ::core::ffi::c_int)
            as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
        VC_Exit_ptr = Some(VC1_Exit as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
        VC_SetNumVoices_ptr = Some(
            VC1_SetNumVoices as unsafe extern "C" fn() -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
        VC_SampleSpace_ptr = Some(
            VC1_SampleSpace as unsafe extern "C" fn(::core::ffi::c_int) -> ULONG,
        ) as Option<unsafe extern "C" fn(::core::ffi::c_int) -> ULONG>;
        VC_SampleLength_ptr = Some(
            VC1_SampleLength
                as unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG,
        ) as Option<unsafe extern "C" fn(::core::ffi::c_int, *mut SAMPLE) -> ULONG>;
        VC_PlayStart_ptr = Some(
            VC1_PlayStart as unsafe extern "C" fn() -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
        VC_PlayStop_ptr = Some(VC1_PlayStop as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
        VC_SampleLoad_ptr = Some(
            VC1_SampleLoad
                as unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD,
        ) as Option<unsafe extern "C" fn(*mut SAMPLOAD, ::core::ffi::c_int) -> SWORD>;
        VC_SampleUnload_ptr = Some(VC1_SampleUnload as unsafe extern "C" fn(SWORD) -> ())
            as Option<unsafe extern "C" fn(SWORD) -> ()>;
        VC_WriteBytes_ptr = Some(
            VC1_WriteBytes as unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG,
        ) as Option<unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG>;
        VC_SilenceBytes_ptr = Some(
            VC1_SilenceBytes as unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG,
        ) as Option<unsafe extern "C" fn(*mut SBYTE, ULONG) -> ULONG>;
        VC_VoiceSetVolume_ptr = Some(
            VC1_VoiceSetVolume as unsafe extern "C" fn(UBYTE, UWORD) -> (),
        ) as Option<unsafe extern "C" fn(UBYTE, UWORD) -> ()>;
        VC_VoiceGetVolume_ptr = Some(
            VC1_VoiceGetVolume as unsafe extern "C" fn(UBYTE) -> UWORD,
        ) as Option<unsafe extern "C" fn(UBYTE) -> UWORD>;
        VC_VoiceSetFrequency_ptr = Some(
            VC1_VoiceSetFrequency as unsafe extern "C" fn(UBYTE, ULONG) -> (),
        ) as Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()>;
        VC_VoiceGetFrequency_ptr = Some(
            VC1_VoiceGetFrequency as unsafe extern "C" fn(UBYTE) -> ULONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> ULONG>;
        VC_VoiceSetPanning_ptr = Some(
            VC1_VoiceSetPanning as unsafe extern "C" fn(UBYTE, ULONG) -> (),
        ) as Option<unsafe extern "C" fn(UBYTE, ULONG) -> ()>;
        VC_VoiceGetPanning_ptr = Some(
            VC1_VoiceGetPanning as unsafe extern "C" fn(UBYTE) -> ULONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> ULONG>;
        VC_VoicePlay_ptr = Some(
            VC1_VoicePlay
                as unsafe extern "C" fn(
                    UBYTE,
                    SWORD,
                    ULONG,
                    ULONG,
                    ULONG,
                    ULONG,
                    UWORD,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    UBYTE,
                    SWORD,
                    ULONG,
                    ULONG,
                    ULONG,
                    ULONG,
                    UWORD,
                ) -> (),
            >;
        VC_VoiceStop_ptr = Some(VC1_VoiceStop as unsafe extern "C" fn(UBYTE) -> ())
            as Option<unsafe extern "C" fn(UBYTE) -> ()>;
        VC_VoiceStopped_ptr = Some(
            VC1_VoiceStopped as unsafe extern "C" fn(UBYTE) -> BOOL,
        ) as Option<unsafe extern "C" fn(UBYTE) -> BOOL>;
        VC_VoiceGetPosition_ptr = Some(
            VC1_VoiceGetPosition as unsafe extern "C" fn(UBYTE) -> SLONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> SLONG>;
        VC_VoiceRealVolume_ptr = Some(
            VC1_VoiceRealVolume as unsafe extern "C" fn(UBYTE) -> ULONG,
        ) as Option<unsafe extern "C" fn(UBYTE) -> ULONG>;
    };
}
