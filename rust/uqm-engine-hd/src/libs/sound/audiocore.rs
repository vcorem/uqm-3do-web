/*
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

/* Audio Core API (derived from OpenAL)
 */

// Rust translation of src/libs/sound/audiocore.c.
extern "C" {
    fn exit(_: ::core::ffi::c_int) -> !;
    fn openAL_Init(driver: *mut audio_Driver, flags: sint32) -> sint32;
    fn mixSDL_Init(driver: *mut audio_Driver, flags: sint32) -> sint32;
    fn noSound_Init(driver: *mut audio_Driver, flags: sint32) -> sint32;
    static mut musicVolume: ::core::ffi::c_int;
    static mut sfxVolumeScale: ::core::ffi::c_float;
    static mut speechVolumeScale: ::core::ffi::c_float;
    fn SetSFXVolume(volume: ::core::ffi::c_float);
    fn SetSpeechVolume(volume: ::core::ffi::c_float);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn SetMusicVolume(Volume: COUNT);
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const audio_DRIVER_MIXSDL: Self = Self(0);
    pub const audio_DRIVER_NOSOUND: Self = Self(1);
    pub const audio_DRIVER_OPENAL: Self = Self(2);
}
pub type audio_Object = uintptr_t;
pub type audio_IntVal = intptr_t;
pub type audio_SourceProp = sint32;
pub type audio_BufferProp = sint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const audio_NO_ERROR: Self = Self(0);
    pub const audio_INVALID_NAME: Self = Self(1);
    pub const audio_INVALID_ENUM: Self = Self(2);
    pub const audio_INVALID_VALUE: Self = Self(3);
    pub const audio_INVALID_OPERATION: Self = Self(4);
    pub const audio_OUT_OF_MEMORY: Self = Self(5);
    pub const audio_DRIVER_FAILURE: Self = Self(6);
    pub const audio_POSITION: Self = Self(7);
    pub const audio_LOOPING: Self = Self(8);
    pub const audio_BUFFER: Self = Self(9);
    pub const audio_GAIN: Self = Self(10);
    pub const audio_SOURCE_STATE: Self = Self(11);
    pub const audio_BUFFERS_QUEUED: Self = Self(12);
    pub const audio_BUFFERS_PROCESSED: Self = Self(13);
    pub const audio_INITIAL: Self = Self(14);
    pub const audio_STOPPED: Self = Self(15);
    pub const audio_PLAYING: Self = Self(16);
    pub const audio_PAUSED: Self = Self(17);
    pub const audio_FREQUENCY: Self = Self(18);
    pub const audio_BITS: Self = Self(19);
    pub const audio_CHANNELS: Self = Self(20);
    pub const audio_SIZE: Self = Self(21);
    pub const audio_FORMAT_MONO16: Self = Self(22);
    pub const audio_FORMAT_STEREO16: Self = Self(23);
    pub const audio_FORMAT_MONO8: Self = Self(24);
    pub const audio_FORMAT_STEREO8: Self = Self(25);
    pub const audio_ENUM_SIZE: Self = Self(26);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_Driver {
    pub Uninitialize: Option<unsafe extern "C" fn() -> ()>,
    pub GetError: Option<unsafe extern "C" fn() -> sint32>,
    pub driverID: sint32,
    pub EnumLookup: [sint32; 26],
    pub GenSources: Option<unsafe extern "C" fn(uint32, *mut audio_Object) -> ()>,
    pub DeleteSources: Option<unsafe extern "C" fn(uint32, *mut audio_Object) -> ()>,
    pub IsSource: Option<unsafe extern "C" fn(audio_Object) -> bool>,
    pub Sourcei: Option<
        unsafe extern "C" fn(audio_Object, audio_SourceProp, audio_IntVal) -> (),
    >,
    pub Sourcef: Option<
        unsafe extern "C" fn(audio_Object, audio_SourceProp, ::core::ffi::c_float) -> (),
    >,
    pub Sourcefv: Option<
        unsafe extern "C" fn(
            audio_Object,
            audio_SourceProp,
            *mut ::core::ffi::c_float,
        ) -> (),
    >,
    pub GetSourcei: Option<
        unsafe extern "C" fn(audio_Object, audio_SourceProp, *mut audio_IntVal) -> (),
    >,
    pub GetSourcef: Option<
        unsafe extern "C" fn(
            audio_Object,
            audio_SourceProp,
            *mut ::core::ffi::c_float,
        ) -> (),
    >,
    pub SourceRewind: Option<unsafe extern "C" fn(audio_Object) -> ()>,
    pub SourcePlay: Option<unsafe extern "C" fn(audio_Object) -> ()>,
    pub SourcePause: Option<unsafe extern "C" fn(audio_Object) -> ()>,
    pub SourceStop: Option<unsafe extern "C" fn(audio_Object) -> ()>,
    pub SourceQueueBuffers: Option<
        unsafe extern "C" fn(audio_Object, uint32, *mut audio_Object) -> (),
    >,
    pub SourceUnqueueBuffers: Option<
        unsafe extern "C" fn(audio_Object, uint32, *mut audio_Object) -> (),
    >,
    pub GenBuffers: Option<unsafe extern "C" fn(uint32, *mut audio_Object) -> ()>,
    pub DeleteBuffers: Option<unsafe extern "C" fn(uint32, *mut audio_Object) -> ()>,
    pub IsBuffer: Option<unsafe extern "C" fn(audio_Object) -> bool>,
    pub GetBufferi: Option<
        unsafe extern "C" fn(audio_Object, audio_BufferProp, *mut audio_IntVal) -> (),
    >,
    pub BufferData: Option<
        unsafe extern "C" fn(
            audio_Object,
            uint32,
            *mut ::core::ffi::c_void,
            uint32,
            uint32,
        ) -> (),
    >,
}
pub type COUNT = UWORD;
pub type UWORD = uint16;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
static mut audiodrv: audio_Driver = audio_Driver {
    Uninitialize: None,
    GetError: None,
    driverID: 0,
    EnumLookup: [0; 26],
    GenSources: None,
    DeleteSources: None,
    IsSource: None,
    Sourcei: None,
    Sourcef: None,
    Sourcefv: None,
    GetSourcei: None,
    GetSourcef: None,
    SourceRewind: None,
    SourcePlay: None,
    SourcePause: None,
    SourceStop: None,
    SourceQueueBuffers: None,
    SourceUnqueueBuffers: None,
    GenBuffers: None,
    DeleteBuffers: None,
    IsBuffer: None,
    GetBufferi: None,
    BufferData: None,
};
#[no_mangle]
pub static mut soundflags: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut snddriver: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut audio_inited: bool = false;
#[no_mangle]
pub unsafe extern "C" fn initAudio(mut driver: sint32, mut flags: sint32) -> sint32 {
    let mut ret: sint32 = 0;
    if driver == C2Rust_Unnamed::audio_DRIVER_MIXSDL.0 as ::core::ffi::c_int {
        ret = uqm_lua_checked!(mixSDL_Init(&raw mut audiodrv, flags), 0);
    } else if driver == C2Rust_Unnamed::audio_DRIVER_OPENAL.0 as ::core::ffi::c_int {
        ret = uqm_lua_checked!(openAL_Init(&raw mut audiodrv, flags), 0);
    } else {
        ret = uqm_lua_checked!(noSound_Init(&raw mut audiodrv, flags), 0);
    }
    if ret != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Fatal,
            b"Sound driver initialization failed.\nThis may happen when a soundcard is not present or not available.\nNOTICE: Try running UQM with '--sound=none' option\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    uqm_lua_checked!(SetSFXVolume(sfxVolumeScale), 0);
    uqm_lua_checked!(SetSpeechVolume(speechVolumeScale), 0);
    uqm_lua_checked!(SetMusicVolume(musicVolume as COUNT), 0);
    ::core::ptr::write_volatile(&raw mut audio_inited, true);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn unInitAudio() {
    if !::core::ptr::read_volatile::<bool>(&raw const audio_inited) {
        return;
    }
    ::core::ptr::write_volatile(&raw mut audio_inited, false);
    uqm_lua_checked!(audiodrv.Uninitialize.expect("non-null function pointer")(), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_GetError() -> sint32 {
    return uqm_lua_checked!(audiodrv.GetError.expect("non-null function pointer")(), 0);
}
#[no_mangle]
pub unsafe extern "C" fn audio_GenSources(
    mut n: uint32,
    mut psrcobj: *mut audio_Object,
) {
    uqm_lua_checked!(audiodrv.GenSources.expect("non-null function pointer")(n, psrcobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_DeleteSources(
    mut n: uint32,
    mut psrcobj: *mut audio_Object,
) {
    uqm_lua_checked!(audiodrv.DeleteSources.expect("non-null function pointer")(n, psrcobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_IsSource(mut srcobj: audio_Object) -> bool {
    return uqm_lua_checked!(audiodrv.IsSource.expect("non-null function pointer")(srcobj), false);
}
#[no_mangle]
pub unsafe extern "C" fn audio_Sourcei(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: audio_IntVal,
) {
    uqm_lua_checked!(audiodrv
        .Sourcei
        .expect(
            "non-null function pointer",
        )(srcobj, audiodrv.EnumLookup[pname as usize], value), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_Sourcef(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: ::core::ffi::c_float,
) {
    uqm_lua_checked!(audiodrv
        .Sourcef
        .expect(
            "non-null function pointer",
        )(srcobj, audiodrv.EnumLookup[pname as usize], value), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_Sourcefv(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    uqm_lua_checked!(audiodrv
        .Sourcefv
        .expect(
            "non-null function pointer",
        )(srcobj, audiodrv.EnumLookup[pname as usize], value), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_GetSourcei(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut audio_IntVal,
) {
    uqm_lua_checked!(audiodrv
        .GetSourcei
        .expect(
            "non-null function pointer",
        )(srcobj, audiodrv.EnumLookup[pname as usize], value), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_GetSourcef(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    uqm_lua_checked!(audiodrv
        .GetSourcef
        .expect(
            "non-null function pointer",
        )(srcobj, audiodrv.EnumLookup[pname as usize], value), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_SourceRewind(mut srcobj: audio_Object) {
    uqm_lua_checked!(audiodrv.SourceRewind.expect("non-null function pointer")(srcobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_SourcePlay(mut srcobj: audio_Object) {
    uqm_lua_checked!(audiodrv.SourcePlay.expect("non-null function pointer")(srcobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_SourcePause(mut srcobj: audio_Object) {
    uqm_lua_checked!(audiodrv.SourcePause.expect("non-null function pointer")(srcobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_SourceStop(mut srcobj: audio_Object) {
    uqm_lua_checked!(audiodrv.SourceStop.expect("non-null function pointer")(srcobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_SourceQueueBuffers(
    mut srcobj: audio_Object,
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    uqm_lua_checked!(audiodrv.SourceQueueBuffers.expect("non-null function pointer")(srcobj, n, pbufobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_SourceUnqueueBuffers(
    mut srcobj: audio_Object,
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    uqm_lua_checked!(audiodrv
        .SourceUnqueueBuffers
        .expect("non-null function pointer")(srcobj, n, pbufobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_GenBuffers(
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    uqm_lua_checked!(audiodrv.GenBuffers.expect("non-null function pointer")(n, pbufobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_DeleteBuffers(
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    uqm_lua_checked!(audiodrv.DeleteBuffers.expect("non-null function pointer")(n, pbufobj), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_IsBuffer(mut bufobj: audio_Object) -> bool {
    return uqm_lua_checked!(audiodrv.IsBuffer.expect("non-null function pointer")(bufobj), false);
}
#[no_mangle]
pub unsafe extern "C" fn audio_GetBufferi(
    mut bufobj: audio_Object,
    pname: audio_BufferProp,
    mut value: *mut audio_IntVal,
) {
    uqm_lua_checked!(audiodrv
        .GetBufferi
        .expect(
            "non-null function pointer",
        )(bufobj, audiodrv.EnumLookup[pname as usize], value), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_BufferData(
    mut bufobj: audio_Object,
    mut format: uint32,
    mut data: *mut ::core::ffi::c_void,
    mut size: uint32,
    mut freq: uint32,
) {
    uqm_lua_checked!(audiodrv
        .BufferData
        .expect(
            "non-null function pointer",
        )(bufobj, audiodrv.EnumLookup[format as usize] as uint32, data, size, freq), ());
}
#[no_mangle]
pub unsafe extern "C" fn audio_GetFormatInfo(
    mut format: uint32,
    mut channels: *mut ::core::ffi::c_int,
    mut sample_size: *mut ::core::ffi::c_int,
) -> bool {
    match format {
        24 => {
            *channels = 1 as ::core::ffi::c_int;
            *sample_size = ::core::mem::size_of::<uint8>() as ::core::ffi::c_int;
            return true;
        }
        25 => {
            *channels = 2 as ::core::ffi::c_int;
            *sample_size = ::core::mem::size_of::<uint8>() as ::core::ffi::c_int;
            return true;
        }
        22 => {
            *channels = 1 as ::core::ffi::c_int;
            *sample_size = ::core::mem::size_of::<sint16>() as ::core::ffi::c_int;
            return true;
        }
        23 => {
            *channels = 2 as ::core::ffi::c_int;
            *sample_size = ::core::mem::size_of::<sint16>() as ::core::ffi::c_int;
            return true;
        }
        _ => {}
    }
    return false;
}
