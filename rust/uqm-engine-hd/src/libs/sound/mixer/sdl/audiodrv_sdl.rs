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

/* SDL audio driver
 */

// Rust translation of src/libs/sound/mixer/sdl/audiodrv_sdl.c.
extern "C" {
    pub type uio_DirHandle;
    fn audio_GenSources(n: uint32, psrcobj: *mut audio_Object);
    fn SoundDecoder_Init(
        flags: ::core::ffi::c_int,
        formats: *mut TFB_DecoderFormats,
    ) -> sint32;
    fn SoundDecoder_Uninit();
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn HFree(p: *mut ::core::ffi::c_void);
    static mut soundSource: [TFB_SoundSource; 0];
    fn InitStreamDecoder() -> ::core::ffi::c_int;
    fn UninitStreamDecoder();
    fn StopStream(source: uint32);
    fn mixer_GetError() -> uint32;
    fn mixer_Init(
        frequency: uint32,
        format: uint32,
        quality: mixer_Quality,
        flags: mixer_Flags,
    ) -> bool;
    fn mixer_Uninit();
    fn mixer_MixChannels(
        userdata: *mut ::core::ffi::c_void,
        stream: *mut uint8,
        len: sint32,
    );
    fn mixer_GenSources(n: uint32, psrcobj: *mut mixer_Object);
    fn mixer_DeleteSources(n: uint32, psrcobj: *mut mixer_Object);
    fn mixer_IsSource(srcobj: mixer_Object) -> bool;
    fn mixer_Sourcei(srcobj: mixer_Object, pname: mixer_SourceProp, value: mixer_IntVal);
    fn mixer_Sourcef(
        srcobj: mixer_Object,
        pname: mixer_SourceProp,
        value: ::core::ffi::c_float,
    );
    fn mixer_Sourcefv(
        srcobj: mixer_Object,
        pname: mixer_SourceProp,
        value: *mut ::core::ffi::c_float,
    );
    fn mixer_GetSourcei(
        srcobj: mixer_Object,
        pname: mixer_SourceProp,
        value: *mut mixer_IntVal,
    );
    fn mixer_GetSourcef(
        srcobj: mixer_Object,
        pname: mixer_SourceProp,
        value: *mut ::core::ffi::c_float,
    );
    fn mixer_SourceRewind(srcobj: mixer_Object);
    fn mixer_SourcePlay(srcobj: mixer_Object);
    fn mixer_SourcePause(srcobj: mixer_Object);
    fn mixer_SourceStop(srcobj: mixer_Object);
    fn mixer_SourceQueueBuffers(
        srcobj: mixer_Object,
        n: uint32,
        pbufobj: *mut mixer_Object,
    );
    fn mixer_SourceUnqueueBuffers(
        srcobj: mixer_Object,
        n: uint32,
        pbufobj: *mut mixer_Object,
    );
    fn mixer_GenBuffers(n: uint32, pbufobj: *mut mixer_Object);
    fn mixer_DeleteBuffers(n: uint32, pbufobj: *mut mixer_Object);
    fn mixer_IsBuffer(bufobj: mixer_Object) -> bool;
    fn mixer_GetBufferi(
        bufobj: mixer_Object,
        pname: mixer_BufferProp,
        value: *mut mixer_IntVal,
    );
    fn mixer_BufferData(
        bufobj: mixer_Object,
        format: uint32,
        data: *mut ::core::ffi::c_void,
        size: uint32,
        freq: uint32,
    );
    fn SDL_GetError() -> *const ::core::ffi::c_char;
    fn SDL_GetCurrentAudioDriver() -> *const ::core::ffi::c_char;
    fn SDL_OpenAudioDevice(
        device: *const ::core::ffi::c_char,
        iscapture: ::core::ffi::c_int,
        desired: *const SDL_AudioSpec,
        obtained: *mut SDL_AudioSpec,
        allowed_changes: ::core::ffi::c_int,
    ) -> SDL_AudioDeviceID;
    fn SDL_PauseAudioDevice(dev_0: SDL_AudioDeviceID, pause_on: ::core::ffi::c_int);
    fn SDL_CloseAudioDevice(dev_0: SDL_AudioDeviceID);
    fn SDL_InitSubSystem(flags: Uint32) -> ::core::ffi::c_int;
    fn SDL_QuitSubSystem(flags: Uint32);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_decoderformats {
    pub big_endian: bool,
    pub want_big_endian: bool,
    pub mono8: uint32,
    pub stereo8: uint32,
    pub mono16: uint32,
    pub stereo16: uint32,
}
pub type TFB_DecoderFormats = tfb_decoderformats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_sounddecoder {
    pub funcs: *const TFB_SoundDecoderFuncs,
    pub format: uint32,
    pub frequency: uint32,
    pub length: ::core::ffi::c_float,
    pub is_null: bool,
    pub need_swap: bool,
    pub buffer: *mut ::core::ffi::c_void,
    pub buffer_size: uint32,
    pub error: sint32,
    pub bytes_per_samp: uint32,
    pub looping: bool,
    pub dir: *mut uio_DirHandle,
    pub filename: *mut ::core::ffi::c_char,
    pub pos: uint32,
    pub start_sample: uint32,
    pub end_sample: uint32,
    pub numpos: uint16,
    pub filename_hash: uint32,
}
pub type TFB_SoundDecoderFuncs = tfb_sounddecoderfunc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_sounddecoderfunc {
    pub GetName: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub InitModule: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *const TFB_DecoderFormats) -> bool,
    >,
    pub TermModule: Option<unsafe extern "C" fn() -> ()>,
    pub GetStructSize: Option<unsafe extern "C" fn() -> uint32>,
    pub GetError: Option<
        unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    >,
    pub Init: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool>,
    pub Term: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()>,
    pub Open: Option<
        unsafe extern "C" fn(
            *mut TFB_SoundDecoder,
            *mut uio_DirHandle,
            *const ::core::ffi::c_char,
        ) -> bool,
    >,
    pub Close: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()>,
    pub Decode: Option<
        unsafe extern "C" fn(
            *mut TFB_SoundDecoder,
            *mut ::core::ffi::c_void,
            sint32,
        ) -> ::core::ffi::c_int,
    >,
    pub Seek: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32>,
    pub GetFrame: Option<unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32>,
}
pub type TFB_SoundDecoder = tfb_sounddecoder;
pub type DWORD = uint32;
pub type Mutex = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_soundsample {
    pub decoder: *mut TFB_SoundDecoder,
    pub length: ::core::ffi::c_float,
    pub buffer: *mut audio_Object,
    pub num_buffers: uint32,
    pub buffer_tag: *mut TFB_SoundTag,
    pub offset: sint32,
    pub data: *mut ::core::ffi::c_void,
    pub callbacks: TFB_SoundCallbacks,
}
pub type TFB_SoundCallbacks = tfb_soundcallbacks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_soundcallbacks {
    pub OnStartStream: Option<unsafe extern "C" fn(*mut TFB_SoundSample) -> bool>,
    pub OnEndChunk: Option<
        unsafe extern "C" fn(*mut TFB_SoundSample, audio_Object) -> bool,
    >,
    pub OnEndStream: Option<unsafe extern "C" fn(*mut TFB_SoundSample) -> ()>,
    pub OnTaggedBuffer: Option<
        unsafe extern "C" fn(*mut TFB_SoundSample, *mut TFB_SoundTag) -> (),
    >,
    pub OnQueueBuffer: Option<
        unsafe extern "C" fn(*mut TFB_SoundSample, audio_Object) -> (),
    >,
}
pub type TFB_SoundSample = tfb_soundsample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_SoundTag {
    pub in_use: ::core::ffi::c_int,
    pub buf_name: audio_Object,
    pub data: intptr_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const MIX_NO_ERROR: Self = Self(0);
    pub const MIX_INVALID_NAME: Self = Self(40961);
    pub const MIX_INVALID_ENUM: Self = Self(40962);
    pub const MIX_INVALID_VALUE: Self = Self(40963);
    pub const MIX_INVALID_OPERATION: Self = Self(40964);
    pub const MIX_OUT_OF_MEMORY: Self = Self(40965);
    pub const MIX_DRIVER_FAILURE: Self = Self(41217);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mixer_SourceProp(pub ::core::ffi::c_uint);
impl mixer_SourceProp {
    pub const MIX_POSITION: Self = Self(4100);
    pub const MIX_LOOPING: Self = Self(4103);
    pub const MIX_BUFFER: Self = Self(4105);
    pub const MIX_GAIN: Self = Self(4106);
    pub const MIX_SOURCE_STATE: Self = Self(4112);
    pub const MIX_BUFFERS_QUEUED: Self = Self(4117);
    pub const MIX_BUFFERS_PROCESSED: Self = Self(4118);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const MIX_INITIAL: Self = Self(0);
    pub const MIX_STOPPED: Self = Self(1);
    pub const MIX_PLAYING: Self = Self(2);
    pub const MIX_PAUSED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mixer_BufferProp(pub ::core::ffi::c_uint);
impl mixer_BufferProp {
    pub const MIX_FREQUENCY: Self = Self(8193);
    pub const MIX_BITS: Self = Self(8194);
    pub const MIX_CHANNELS: Self = Self(8195);
    pub const MIX_SIZE: Self = Self(8196);
    pub const MIX_DATA: Self = Self(8197);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const MIX_FORMAT_MONO8: Self = Self(1507585);
    pub const MIX_FORMAT_STEREO8: Self = Self(1507841);
    pub const MIX_FORMAT_MONO16: Self = Self(1507586);
    pub const MIX_FORMAT_STEREO16: Self = Self(1507842);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mixer_Quality(pub ::core::ffi::c_uint);
impl mixer_Quality {
    pub const MIX_QUALITY_LOW: Self = Self(0);
    pub const MIX_QUALITY_MEDIUM: Self = Self(1);
    pub const MIX_QUALITY_HIGH: Self = Self(2);
    pub const MIX_QUALITY_DEFAULT: Self = Self(1);
    pub const MIX_QUALITY_COUNT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mixer_Flags(pub ::core::ffi::c_uint);
impl mixer_Flags {
    pub const MIX_NOFLAGS: Self = Self(0);
    pub const MIX_FAKE_DATA: Self = Self(1);
}
pub type mixer_Object = intptr_t;
pub type mixer_IntVal = intptr_t;
pub type Uint8 = uint8_t;
pub type Uint16 = uint16_t;
pub type Uint32 = uint32_t;
pub type SDL_AudioFormat = Uint16;
pub type SDL_AudioCallback = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut Uint8, ::core::ffi::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioSpec {
    pub freq: ::core::ffi::c_int,
    pub format: SDL_AudioFormat,
    pub channels: Uint8,
    pub silence: Uint8,
    pub samples: Uint16,
    pub padding: Uint16,
    pub size: Uint32,
    pub callback: SDL_AudioCallback,
    pub userdata: *mut ::core::ffi::c_void,
}
pub type SDL_AudioDeviceID = Uint32;
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
pub type TFB_SoundSource = tfb_soundsource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_soundsource {
    pub sample: *mut TFB_SoundSample,
    pub handle: audio_Object,
    pub stream_should_be_playing: bool,
    pub stream_mutex: Mutex,
    pub start_time: sint32,
    pub pause_time: uint32,
    pub positional_object: *mut ::core::ffi::c_void,
    pub last_q_buf: audio_Object,
    pub sbuffer: *mut ::core::ffi::c_void,
    pub sbuf_size: uint32,
    pub sbuf_tail: uint32,
    pub sbuf_head: uint32,
    pub sbuf_lasttime: uint32,
}
static mut dev: SDL_AudioDeviceID = 0;
static mut mixSDL_Driver: audio_Driver = audio_Driver {
    Uninitialize: Some(mixSDL_Uninit as unsafe extern "C" fn() -> ()),
    GetError: Some(mixSDL_GetError as unsafe extern "C" fn() -> sint32),
    driverID: C2Rust_Unnamed::audio_DRIVER_MIXSDL.0 as sint32,
    EnumLookup: [
        C2Rust_Unnamed_2::MIX_NO_ERROR.0 as sint32,
        C2Rust_Unnamed_2::MIX_INVALID_NAME.0 as sint32,
        C2Rust_Unnamed_2::MIX_INVALID_ENUM.0 as sint32,
        C2Rust_Unnamed_2::MIX_INVALID_VALUE.0 as sint32,
        C2Rust_Unnamed_2::MIX_INVALID_OPERATION.0 as sint32,
        C2Rust_Unnamed_2::MIX_OUT_OF_MEMORY.0 as sint32,
        C2Rust_Unnamed_2::MIX_DRIVER_FAILURE.0 as sint32,
        mixer_SourceProp::MIX_POSITION.0 as sint32,
        mixer_SourceProp::MIX_LOOPING.0 as sint32,
        mixer_SourceProp::MIX_BUFFER.0 as sint32,
        mixer_SourceProp::MIX_GAIN.0 as sint32,
        mixer_SourceProp::MIX_SOURCE_STATE.0 as sint32,
        mixer_SourceProp::MIX_BUFFERS_QUEUED.0 as sint32,
        mixer_SourceProp::MIX_BUFFERS_PROCESSED.0 as sint32,
        C2Rust_Unnamed_3::MIX_INITIAL.0 as sint32,
        C2Rust_Unnamed_3::MIX_STOPPED.0 as sint32,
        C2Rust_Unnamed_3::MIX_PLAYING.0 as sint32,
        C2Rust_Unnamed_3::MIX_PAUSED.0 as sint32,
        mixer_BufferProp::MIX_FREQUENCY.0 as sint32,
        mixer_BufferProp::MIX_BITS.0 as sint32,
        mixer_BufferProp::MIX_CHANNELS.0 as sint32,
        mixer_BufferProp::MIX_SIZE.0 as sint32,
        C2Rust_Unnamed_4::MIX_FORMAT_MONO16.0 as sint32,
        C2Rust_Unnamed_4::MIX_FORMAT_STEREO16.0 as sint32,
        C2Rust_Unnamed_4::MIX_FORMAT_MONO8.0 as sint32,
        C2Rust_Unnamed_4::MIX_FORMAT_STEREO8.0 as sint32,
    ],
    GenSources: Some(
        mixSDL_GenSources as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    DeleteSources: Some(
        mixSDL_DeleteSources as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    IsSource: Some(mixSDL_IsSource as unsafe extern "C" fn(audio_Object) -> bool),
    Sourcei: Some(
        mixSDL_Sourcei
            as unsafe extern "C" fn(audio_Object, audio_SourceProp, audio_IntVal) -> (),
    ),
    Sourcef: Some(
        mixSDL_Sourcef
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                ::core::ffi::c_float,
            ) -> (),
    ),
    Sourcefv: Some(
        mixSDL_Sourcefv
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                *mut ::core::ffi::c_float,
            ) -> (),
    ),
    GetSourcei: Some(
        mixSDL_GetSourcei
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                *mut audio_IntVal,
            ) -> (),
    ),
    GetSourcef: Some(
        mixSDL_GetSourcef
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                *mut ::core::ffi::c_float,
            ) -> (),
    ),
    SourceRewind: Some(mixSDL_SourceRewind as unsafe extern "C" fn(audio_Object) -> ()),
    SourcePlay: Some(mixSDL_SourcePlay as unsafe extern "C" fn(audio_Object) -> ()),
    SourcePause: Some(mixSDL_SourcePause as unsafe extern "C" fn(audio_Object) -> ()),
    SourceStop: Some(mixSDL_SourceStop as unsafe extern "C" fn(audio_Object) -> ()),
    SourceQueueBuffers: Some(
        mixSDL_SourceQueueBuffers
            as unsafe extern "C" fn(audio_Object, uint32, *mut audio_Object) -> (),
    ),
    SourceUnqueueBuffers: Some(
        mixSDL_SourceUnqueueBuffers
            as unsafe extern "C" fn(audio_Object, uint32, *mut audio_Object) -> (),
    ),
    GenBuffers: Some(
        mixSDL_GenBuffers as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    DeleteBuffers: Some(
        mixSDL_DeleteBuffers as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    IsBuffer: Some(mixSDL_IsBuffer as unsafe extern "C" fn(audio_Object) -> bool),
    GetBufferi: Some(
        mixSDL_GetBufferi
            as unsafe extern "C" fn(
                audio_Object,
                audio_BufferProp,
                *mut audio_IntVal,
            ) -> (),
    ),
    BufferData: Some(
        mixSDL_BufferData
            as unsafe extern "C" fn(
                audio_Object,
                uint32,
                *mut ::core::ffi::c_void,
                uint32,
                uint32,
            ) -> (),
    ),
};
unsafe extern "C" fn mixSDL_ConvertObjectArrayToMixerObjects(
    mut n: uint32,
    mut arr: *mut audio_Object,
) {
    let mut i: uint32 = 0;
    if ::core::mem::size_of::<audio_Object>() == ::core::mem::size_of::<mixer_Object>() {
        return;
    }
    i = 0 as uint32;
    while i < n {
        *(arr as *mut mixer_Object).offset(i as isize) = *arr.offset(i as isize)
            as mixer_Object;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn mixSDL_ConvertObjectArrayFromMixerObjects(
    mut n: uint32,
    mut arr: *mut audio_Object,
) {
    let mut i: uint32 = 0;
    if ::core::mem::size_of::<audio_Object>() == ::core::mem::size_of::<mixer_Object>() {
        return;
    }
    i = n;
    loop {
        let c2rust_fresh0 = i;
        i = i.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        *arr.offset(i as isize) = *(arr as *mut mixer_Object).offset(i as isize)
            as audio_Object;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_Init(
    mut driver: *mut audio_Driver,
    mut flags: sint32,
) -> sint32 {
    let mut i: ::core::ffi::c_int = 0;
    let mut desired: SDL_AudioSpec = SDL_AudioSpec {
        freq: 0,
        format: 0,
        channels: 0,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut obtained: SDL_AudioSpec = SDL_AudioSpec {
        freq: 0,
        format: 0,
        channels: 0,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut quality: mixer_Quality = mixer_Quality::MIX_QUALITY_LOW;
    let mut formats: TFB_DecoderFormats = tfb_decoderformats {
        big_endian: false,
        want_big_endian: false,
        mono8: C2Rust_Unnamed_0::audio_FORMAT_MONO8.0 as ::core::ffi::c_int as uint32,
        stereo8: C2Rust_Unnamed_0::audio_FORMAT_STEREO8.0 as ::core::ffi::c_int
            as uint32,
        mono16: C2Rust_Unnamed_0::audio_FORMAT_MONO16.0 as ::core::ffi::c_int as uint32,
        stereo16: C2Rust_Unnamed_0::audio_FORMAT_STEREO16.0 as ::core::ffi::c_int
            as uint32,
    };
    log_add(
        log_Level::log_Info,
        b"Initializing SDL audio subsystem.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if SDL_InitSubSystem(0x10 as Uint32) == -1 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"Couldn't initialize audio subsystem: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        return -1 as sint32;
    }
    log_add(
        log_Level::log_Info,
        b"SDL audio subsystem initialized.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        quality = mixer_Quality::MIX_QUALITY_HIGH;
        desired.freq = 44100 as ::core::ffi::c_int;
        desired.samples = 4096 as Uint16;
    } else if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        quality = mixer_Quality::MIX_QUALITY_LOW;
        desired.freq = 22050 as ::core::ffi::c_int;
        desired.samples = 2048 as Uint16;
    } else {
        quality = mixer_Quality::MIX_QUALITY_DEFAULT;
        desired.freq = 44100 as ::core::ffi::c_int;
        desired.samples = 4096 as Uint16;
    }
    desired.format = 0x8010 as SDL_AudioFormat;
    desired.channels = 2 as Uint8;
    desired.callback = Some(
        audioCallback
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut Uint8,
                ::core::ffi::c_int,
            ) -> (),
    ) as SDL_AudioCallback;
    log_add(
        log_Level::log_Info,
        b"Opening SDL audio device.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    dev = SDL_OpenAudioDevice(
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        &raw mut desired,
        &raw mut obtained,
        0x1 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int,
    );
    if dev != 0 as SDL_AudioDeviceID
        && obtained.channels as ::core::ffi::c_int != 1 as ::core::ffi::c_int
        && obtained.channels as ::core::ffi::c_int != 2 as ::core::ffi::c_int
    {
        dev = SDL_OpenAudioDevice(
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            &raw mut desired,
            &raw mut obtained,
            0x1 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int,
        );
    }
    if dev == 0 as SDL_AudioDeviceID {
        log_add(
            log_Level::log_Error,
            b"Unable to open audio device: %s\0".as_ptr() as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        SDL_QuitSubSystem(0x10 as Uint32);
        return -1 as sint32;
    }
    if obtained.format as ::core::ffi::c_int != desired.format as ::core::ffi::c_int
        || obtained.channels as ::core::ffi::c_int != 1 as ::core::ffi::c_int
            && obtained.channels as ::core::ffi::c_int != 2 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"Unable to obtain desired audio format.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        SDL_CloseAudioDevice(dev);
        SDL_QuitSubSystem(0x10 as Uint32);
        return -1 as sint32;
    }
    let mut devicename: *const ::core::ffi::c_char = SDL_GetCurrentAudioDriver();
    log_add(
        log_Level::log_Info,
        b"    using %s at %d Hz 16 bit %s, %d samples audio buffer\0".as_ptr()
            as *const ::core::ffi::c_char,
        devicename,
        obtained.freq,
        if obtained.channels as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            b"stereo\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"mono\0".as_ptr() as *const ::core::ffi::c_char
        },
        obtained.samples as ::core::ffi::c_int,
    );
    log_add(
        log_Level::log_Info,
        b"Initializing mixer.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mixer_Init(
        obtained.freq as uint32,
        (0x170000 as ::core::ffi::c_int
            | 2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
            | (obtained.channels as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as uint32,
        quality,
        uqm_lua_checked!(mixer_Flags(0 as ::core::ffi::c_int as ::core::ffi::c_uint), 0),
    ) {
        log_add(
            log_Level::log_Error,
            b"Mixer initialization failed: %x\0".as_ptr() as *const ::core::ffi::c_char,
            mixer_GetError(),
        );
        SDL_CloseAudioDevice(dev);
        SDL_QuitSubSystem(0x10 as Uint32);
        return -1 as sint32;
    }
    log_add(
        log_Level::log_Info,
        b"Mixer initialized.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_Info,
        b"Initializing sound decoders.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if uqm_lua_checked!(SoundDecoder_Init(flags, &raw mut formats), 0) != 0 {
        log_add(
            log_Level::log_Error,
            b"Sound decoders initialization failed.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        SDL_CloseAudioDevice(dev);
        mixer_Uninit();
        SDL_QuitSubSystem(0x10 as Uint32);
        return -1 as sint32;
    }
    log_add(
        log_Level::log_Info,
        b"Sound decoders initialized.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    *driver = mixSDL_Driver;
    i = 0 as ::core::ffi::c_int;
    while i
        < 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
    {
        uqm_lua_checked!(audio_GenSources(
            1 as uint32,
            &raw mut (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .handle,
        ), 0);
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
            .stream_mutex = CreateMutex_Core(
            b"MixSDL stream mutex\0".as_ptr() as *const ::core::ffi::c_char,
            C2Rust_Unnamed_1::SYNC_CLASS_AUDIO.0 as ::core::ffi::c_int as DWORD,
        );
        i += 1;
    }
    if InitStreamDecoder() != 0 {
        log_add(
            log_Level::log_Error,
            b"Stream decoder initialization failed.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        SDL_CloseAudioDevice(dev);
        uqm_lua_checked!(SoundDecoder_Uninit(), 0);
        mixer_Uninit();
        SDL_QuitSubSystem(0x10 as Uint32);
        return -1 as sint32;
    }
    SDL_PauseAudioDevice(dev, 0 as ::core::ffi::c_int);
    return 0 as sint32;
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_Uninit() {
    let mut i: ::core::ffi::c_int = 0;
    UninitStreamDecoder();
    i = 0 as ::core::ffi::c_int;
    while i
        < 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
    {
        if !(*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
            .sample
            .is_null()
            && !(*(*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .sample)
                .decoder
                .is_null()
        {
            uqm_lua_checked!(StopStream(i as uint32), ());
        }
        if !(*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
            .sbuffer
            .is_null()
        {
            let mut sbuffer: *mut ::core::ffi::c_void = (*(&raw mut soundSource
                as *mut TFB_SoundSource)
                .offset(i as isize))
                .sbuffer;
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .sbuffer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            HFree(sbuffer);
        }
        DestroyMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .stream_mutex,
        );
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
            .stream_mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
        mixSDL_DeleteSources(
            1 as uint32,
            &raw mut (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .handle,
        );
        i += 1;
    }
    SDL_CloseAudioDevice(dev);
    mixer_Uninit();
    uqm_lua_checked!(SoundDecoder_Uninit(), ());
    SDL_QuitSubSystem(0x10 as Uint32);
}
unsafe extern "C" fn audioCallback(
    mut userdata: *mut ::core::ffi::c_void,
    mut stream: *mut Uint8,
    mut len: ::core::ffi::c_int,
) {
    uqm_lua_checked!(mixer_MixChannels(userdata, stream, len), ());
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_GetError() -> sint32 {
    let mut value: sint32 = mixer_GetError() as sint32;
    match value {
        0 => return C2Rust_Unnamed_0::audio_NO_ERROR.0 as sint32,
        40961 => return C2Rust_Unnamed_0::audio_INVALID_NAME.0 as sint32,
        40962 => return C2Rust_Unnamed_0::audio_INVALID_ENUM.0 as sint32,
        40963 => return C2Rust_Unnamed_0::audio_INVALID_VALUE.0 as sint32,
        40964 => return C2Rust_Unnamed_0::audio_INVALID_OPERATION.0 as sint32,
        40965 => return C2Rust_Unnamed_0::audio_OUT_OF_MEMORY.0 as sint32,
        _ => {
            log_add(
                log_Level::log_Debug,
                b"mixSDL_GetError: unknown value %x\0".as_ptr()
                    as *const ::core::ffi::c_char,
                value,
            );
            return C2Rust_Unnamed_0::audio_DRIVER_FAILURE.0 as sint32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_GenSources(
    mut n: uint32,
    mut psrcobj: *mut audio_Object,
) {
    mixer_GenSources(n, psrcobj as *mut mixer_Object);
    mixSDL_ConvertObjectArrayFromMixerObjects(n, psrcobj);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_DeleteSources(
    mut n: uint32,
    mut psrcobj: *mut audio_Object,
) {
    mixSDL_ConvertObjectArrayToMixerObjects(n, psrcobj);
    mixer_DeleteSources(n, psrcobj as *mut mixer_Object);
    mixSDL_ConvertObjectArrayFromMixerObjects(n, psrcobj);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_IsSource(mut srcobj: audio_Object) -> bool {
    return mixer_IsSource(srcobj as mixer_Object);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_Sourcei(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: audio_IntVal,
) {
    mixer_Sourcei(
        srcobj as mixer_Object,
        uqm_lua_checked!(mixer_SourceProp(pname as ::core::ffi::c_uint), ()),
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_Sourcef(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: ::core::ffi::c_float,
) {
    mixer_Sourcef(
        srcobj as mixer_Object,
        uqm_lua_checked!(mixer_SourceProp(pname as ::core::ffi::c_uint), ()),
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_Sourcefv(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    mixer_Sourcefv(
        srcobj as mixer_Object,
        uqm_lua_checked!(mixer_SourceProp(pname as ::core::ffi::c_uint), ()),
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_GetSourcei(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut audio_IntVal,
) {
    let mut temp: mixer_IntVal = *value;
    mixer_GetSourcei(
        srcobj as mixer_Object,
        uqm_lua_checked!(mixer_SourceProp(pname as ::core::ffi::c_uint), ()),
        &raw mut temp,
    );
    *value = temp as audio_IntVal;
    if pname == mixer_SourceProp::MIX_SOURCE_STATE.0 as ::core::ffi::c_int {
        match *value {
            0 => {
                *value = C2Rust_Unnamed_0::audio_INITIAL.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            1 => {
                *value = C2Rust_Unnamed_0::audio_STOPPED.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            2 => {
                *value = C2Rust_Unnamed_0::audio_PLAYING.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            3 => {
                *value = C2Rust_Unnamed_0::audio_PAUSED.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            _ => {
                *value = C2Rust_Unnamed_0::audio_DRIVER_FAILURE.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_GetSourcef(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    mixer_GetSourcef(
        srcobj as mixer_Object,
        uqm_lua_checked!(mixer_SourceProp(pname as ::core::ffi::c_uint), ()),
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_SourceRewind(mut srcobj: audio_Object) {
    mixer_SourceRewind(srcobj as mixer_Object);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_SourcePlay(mut srcobj: audio_Object) {
    mixer_SourcePlay(srcobj as mixer_Object);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_SourcePause(mut srcobj: audio_Object) {
    mixer_SourcePause(srcobj as mixer_Object);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_SourceStop(mut srcobj: audio_Object) {
    mixer_SourceStop(srcobj as mixer_Object);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_SourceQueueBuffers(
    mut srcobj: audio_Object,
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    mixSDL_ConvertObjectArrayToMixerObjects(n, pbufobj);
    mixer_SourceQueueBuffers(srcobj as mixer_Object, n, pbufobj as *mut mixer_Object);
    mixSDL_ConvertObjectArrayFromMixerObjects(n, pbufobj);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_SourceUnqueueBuffers(
    mut srcobj: audio_Object,
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    mixSDL_ConvertObjectArrayToMixerObjects(n, pbufobj);
    mixer_SourceUnqueueBuffers(srcobj as mixer_Object, n, pbufobj as *mut mixer_Object);
    mixSDL_ConvertObjectArrayFromMixerObjects(n, pbufobj);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_GenBuffers(
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    mixer_GenBuffers(n, pbufobj as *mut mixer_Object);
    mixSDL_ConvertObjectArrayFromMixerObjects(n, pbufobj);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_DeleteBuffers(
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    mixSDL_ConvertObjectArrayToMixerObjects(n, pbufobj);
    mixer_DeleteBuffers(n, pbufobj as *mut mixer_Object);
    mixSDL_ConvertObjectArrayFromMixerObjects(n, pbufobj);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_IsBuffer(mut bufobj: audio_Object) -> bool {
    return mixer_IsBuffer(bufobj as mixer_Object);
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_GetBufferi(
    mut bufobj: audio_Object,
    pname: audio_BufferProp,
    mut value: *mut audio_IntVal,
) {
    let mut temp: mixer_IntVal = *value;
    mixer_GetBufferi(
        bufobj as mixer_Object,
        uqm_lua_checked!(mixer_BufferProp(pname as ::core::ffi::c_uint), ()),
        &raw mut temp,
    );
    *value = temp as audio_IntVal;
}
#[no_mangle]
pub unsafe extern "C" fn mixSDL_BufferData(
    mut bufobj: audio_Object,
    mut format: uint32,
    mut data: *mut ::core::ffi::c_void,
    mut size: uint32,
    mut freq: uint32,
) {
    uqm_lua_checked!(mixer_BufferData(bufobj as mixer_Object, format, data, size, freq), ());
}
