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

/* OpenAL audio driver
 */

// Rust translation of src/libs/sound/openal/audiodrv_openal.c.
extern "C" {
    pub type uio_DirHandle;
    pub type ALCcontext_struct;
    pub type ALCdevice_struct;
    fn SoundDecoder_Init(
        flags: ::core::ffi::c_int,
        formats: *mut TFB_DecoderFormats,
    ) -> sint32;
    fn SoundDecoder_Uninit();
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn HFree(p: *mut ::core::ffi::c_void);
    fn alIsBuffer(buffer: ALuint) -> ALboolean;
    fn alIsSource(source: ALuint) -> ALboolean;
    fn alGetError() -> ALenum;
    fn alGetString(param: ALenum) -> *const ALchar;
    fn alBufferData(
        buffer: ALuint,
        format: ALenum,
        data: *const ::core::ffi::c_void,
        size: ALsizei,
        freq: ALsizei,
    );
    fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
    fn alDeleteSources(n: ALsizei, sources: *const ALuint);
    fn alDistanceModel(distanceModel: ALenum);
    fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
    fn alGenSources(n: ALsizei, sources: *mut ALuint);
    fn alGetBufferi(buffer: ALuint, param: ALenum, value: *mut ALint);
    fn alGetSourcef(source: ALuint, param: ALenum, value: *mut ALfloat);
    fn alGetSourcei(source: ALuint, param: ALenum, value: *mut ALint);
    fn alListenerfv(param: ALenum, values: *const ALfloat);
    fn alSourcePause(source: ALuint);
    fn alSourcePlay(source: ALuint);
    fn alSourceQueueBuffers(source: ALuint, nb: ALsizei, buffers: *const ALuint);
    fn alSourceRewind(source: ALuint);
    fn alSourceStop(source: ALuint);
    fn alSourceUnqueueBuffers(source: ALuint, nb: ALsizei, buffers: *mut ALuint);
    fn alSourcef(source: ALuint, param: ALenum, value: ALfloat);
    fn alSourcefv(source: ALuint, param: ALenum, values: *const ALfloat);
    fn alSourcei(source: ALuint, param: ALenum, value: ALint);
    fn InitStreamDecoder() -> ::core::ffi::c_int;
    fn UninitStreamDecoder();
    fn StopStream(source: uint32);
    static mut soundSource: [TFB_SoundSource; 0];
    fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;
    fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
    fn alcCreateContext(
        device: *mut ALCdevice,
        attrlist: *const ALCint,
    ) -> *mut ALCcontext;
    fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
    fn alcGetError(device: *mut ALCdevice) -> ALCenum;
    fn alcGetString(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar;
    fn alcDestroyContext(context: *mut ALCcontext);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
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
pub type ALboolean = ::core::ffi::c_char;
pub type ALchar = ::core::ffi::c_char;
pub type ALfloat = ::core::ffi::c_float;
pub type ALenum = ::core::ffi::c_int;
pub type ALint = ::core::ffi::c_int;
pub type ALsizei = ::core::ffi::c_int;
pub type ALuint = ::core::ffi::c_uint;
pub type ALvoid = ();
pub type ALCboolean = ::core::ffi::c_char;
pub type ALCchar = ::core::ffi::c_char;
pub type ALCenum = ::core::ffi::c_int;
pub type ALCint = ::core::ffi::c_int;
pub type ALCcontext = ALCcontext_struct;
pub type ALCdevice = ALCdevice_struct;
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
#[no_mangle]
pub static mut alcContext: *mut ALCcontext = ::core::ptr::null_mut::<ALCcontext>();
#[no_mangle]
pub static mut alcDevice: *mut ALCdevice = ::core::ptr::null_mut::<ALCdevice>();
#[no_mangle]
pub static mut defaultPos: [ALfloat; 3] = [0.0f32, 0.0f32, -1.0f32];
#[no_mangle]
pub static mut listenerPos: [ALfloat; 3] = [0.0f32, 0.0f32, 0.0f32];
#[no_mangle]
pub static mut listenerVel: [ALfloat; 3] = [0.0f32, 0.0f32, 0.0f32];
#[no_mangle]
pub static mut listenerOri: [ALfloat; 6] = [
    0.0f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    1.0f32,
    0.0f32,
];
static mut openAL_Driver: audio_Driver = audio_Driver {
    Uninitialize: Some(openAL_Uninit as unsafe extern "C" fn() -> ()),
    GetError: Some(openAL_GetError as unsafe extern "C" fn() -> sint32),
    driverID: C2Rust_Unnamed::audio_DRIVER_OPENAL.0 as sint32,
    EnumLookup: [
        0 as sint32,
        0xa001 as sint32,
        0xa002 as sint32,
        0xa003 as sint32,
        0xa004 as sint32,
        0xa005 as sint32,
        C2Rust_Unnamed_0::audio_DRIVER_FAILURE.0 as sint32,
        0x1004 as sint32,
        0x1007 as sint32,
        0x1009 as sint32,
        0x100a as sint32,
        0x1010 as sint32,
        0x1015 as sint32,
        0x1016 as sint32,
        0x1011 as sint32,
        0x1014 as sint32,
        0x1012 as sint32,
        0x1013 as sint32,
        0x2001 as sint32,
        0x2002 as sint32,
        0x2003 as sint32,
        0x2004 as sint32,
        0x1101 as sint32,
        0x1103 as sint32,
        0x1100 as sint32,
        0x1102 as sint32,
    ],
    GenSources: Some(
        openAL_GenSources as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    DeleteSources: Some(
        openAL_DeleteSources as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    IsSource: Some(openAL_IsSource as unsafe extern "C" fn(audio_Object) -> bool),
    Sourcei: Some(
        openAL_Sourcei
            as unsafe extern "C" fn(audio_Object, audio_SourceProp, audio_IntVal) -> (),
    ),
    Sourcef: Some(
        openAL_Sourcef
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                ::core::ffi::c_float,
            ) -> (),
    ),
    Sourcefv: Some(
        openAL_Sourcefv
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                *mut ::core::ffi::c_float,
            ) -> (),
    ),
    GetSourcei: Some(
        openAL_GetSourcei
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                *mut audio_IntVal,
            ) -> (),
    ),
    GetSourcef: Some(
        openAL_GetSourcef
            as unsafe extern "C" fn(
                audio_Object,
                audio_SourceProp,
                *mut ::core::ffi::c_float,
            ) -> (),
    ),
    SourceRewind: Some(openAL_SourceRewind as unsafe extern "C" fn(audio_Object) -> ()),
    SourcePlay: Some(openAL_SourcePlay as unsafe extern "C" fn(audio_Object) -> ()),
    SourcePause: Some(openAL_SourcePause as unsafe extern "C" fn(audio_Object) -> ()),
    SourceStop: Some(openAL_SourceStop as unsafe extern "C" fn(audio_Object) -> ()),
    SourceQueueBuffers: Some(
        openAL_SourceQueueBuffers
            as unsafe extern "C" fn(audio_Object, uint32, *mut audio_Object) -> (),
    ),
    SourceUnqueueBuffers: Some(
        openAL_SourceUnqueueBuffers
            as unsafe extern "C" fn(audio_Object, uint32, *mut audio_Object) -> (),
    ),
    GenBuffers: Some(
        openAL_GenBuffers as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    DeleteBuffers: Some(
        openAL_DeleteBuffers as unsafe extern "C" fn(uint32, *mut audio_Object) -> (),
    ),
    IsBuffer: Some(openAL_IsBuffer as unsafe extern "C" fn(audio_Object) -> bool),
    GetBufferi: Some(
        openAL_GetBufferi
            as unsafe extern "C" fn(
                audio_Object,
                audio_BufferProp,
                *mut audio_IntVal,
            ) -> (),
    ),
    BufferData: Some(
        openAL_BufferData
            as unsafe extern "C" fn(
                audio_Object,
                uint32,
                *mut ::core::ffi::c_void,
                uint32,
                uint32,
            ) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn openAL_Init(
    mut driver: *mut audio_Driver,
    mut flags: sint32,
) -> sint32 {
    let mut i: ::core::ffi::c_int = 0;
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
        b"Initializing OpenAL.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    alcDevice = alcOpenDevice(::core::ptr::null::<ALCchar>());
    if alcDevice.is_null() {
        log_add(
            log_Level::log_Error,
            b"Couldn't initialize OpenAL: %d\0".as_ptr() as *const ::core::ffi::c_char,
            alcGetError(::core::ptr::null_mut::<ALCdevice>()),
        );
        return -1 as sint32;
    }
    *driver = openAL_Driver;
    alcContext = alcCreateContext(alcDevice, ::core::ptr::null::<ALCint>());
    if alcContext.is_null() {
        log_add(
            log_Level::log_Error,
            b"Couldn't create OpenAL context: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            alcGetError(alcDevice),
        );
        alcCloseDevice(alcDevice);
        alcDevice = ::core::ptr::null_mut::<ALCdevice>();
        return -1 as sint32;
    }
    alcMakeContextCurrent(alcContext);
    log_add(
        log_Level::log_Info,
        b"OpenAL initialized.\n    version:     %s\n    vendor:      %s\n    renderer:    %s\n    device:      %s\0"
            .as_ptr() as *const ::core::ffi::c_char,
        alGetString(0xb002 as ALenum),
        alGetString(0xb001 as ALenum),
        alGetString(0xb003 as ALenum),
        alcGetString(alcDevice, 0x1004 as ALCenum),
    );
    log_add(
        log_Level::log_Info,
        b"Initializing sound decoders.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if SoundDecoder_Init(flags, &raw mut formats) != 0 {
        log_add(
            log_Level::log_Error,
            b"Sound decoders initialization failed.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        alcMakeContextCurrent(::core::ptr::null_mut::<ALCcontext>());
        alcDestroyContext(alcContext);
        alcContext = ::core::ptr::null_mut::<ALCcontext>();
        alcCloseDevice(alcDevice);
        alcDevice = ::core::ptr::null_mut::<ALCdevice>();
        return -1 as sint32;
    }
    log_add(
        log_Level::log_Error,
        b"Sound decoders initialized.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    alListenerfv(0x1004 as ALenum, &raw mut listenerPos as *mut ALfloat);
    alListenerfv(0x1006 as ALenum, &raw mut listenerVel as *mut ALfloat);
    alListenerfv(0x100f as ALenum, &raw mut listenerOri as *mut ALfloat);
    i = 0 as ::core::ffi::c_int;
    while i
        < 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
    {
        let mut zero: [::core::ffi::c_float; 3] = [0.0f32, 0.0f32, 0.0f32];
        alGenSources(
            1 as ALsizei,
            &raw mut (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .handle as *mut ALuint,
        );
        alSourcei(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize)).handle
                as ALuint,
            0x1007 as ALenum,
            0 as ALint,
        );
        alSourcefv(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize)).handle
                as ALuint,
            0x1004 as ALenum,
            &raw mut defaultPos as *mut ALfloat,
        );
        alSourcefv(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize)).handle
                as ALuint,
            0x1006 as ALenum,
            &raw mut zero as *mut ::core::ffi::c_float,
        );
        alSourcefv(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize)).handle
                as ALuint,
            0x1005 as ALenum,
            &raw mut zero as *mut ::core::ffi::c_float,
        );
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
            .stream_mutex = CreateMutex_Core(
            b"OpenAL stream mutex\0".as_ptr() as *const ::core::ffi::c_char,
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
        SoundDecoder_Uninit();
        alcMakeContextCurrent(::core::ptr::null_mut::<ALCcontext>());
        alcDestroyContext(alcContext);
        alcContext = ::core::ptr::null_mut::<ALCcontext>();
        alcCloseDevice(alcDevice);
        alcDevice = ::core::ptr::null_mut::<ALCdevice>();
        return -1 as sint32;
    }
    alDistanceModel(0xd001 as ALenum);
    return 0 as sint32;
}
#[no_mangle]
pub unsafe extern "C" fn openAL_Uninit() {
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
            StopStream(i as uint32);
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
        i += 1;
    }
    alcMakeContextCurrent(::core::ptr::null_mut::<ALCcontext>());
    alcDestroyContext(alcContext);
    alcContext = ::core::ptr::null_mut::<ALCcontext>();
    alcCloseDevice(alcDevice);
    alcDevice = ::core::ptr::null_mut::<ALCdevice>();
    SoundDecoder_Uninit();
}
#[no_mangle]
pub unsafe extern "C" fn openAL_GetError() -> sint32 {
    let mut value: ALint = alGetError();
    match value {
        0 => return C2Rust_Unnamed_0::audio_NO_ERROR.0 as sint32,
        0xa001 => return C2Rust_Unnamed_0::audio_INVALID_NAME.0 as sint32,
        0xa002 => return C2Rust_Unnamed_0::audio_INVALID_ENUM.0 as sint32,
        0xa003 => return C2Rust_Unnamed_0::audio_INVALID_VALUE.0 as sint32,
        0xa004 => return C2Rust_Unnamed_0::audio_INVALID_OPERATION.0 as sint32,
        0xa005 => return C2Rust_Unnamed_0::audio_OUT_OF_MEMORY.0 as sint32,
        _ => {
            log_add(
                log_Level::log_Debug,
                b"openAL_GetError: unknown value %x\0".as_ptr()
                    as *const ::core::ffi::c_char,
                value,
            );
            return C2Rust_Unnamed_0::audio_DRIVER_FAILURE.0 as sint32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn openAL_GenSources(
    mut n: uint32,
    mut psrcobj: *mut audio_Object,
) {
    alGenSources(n as ALsizei, psrcobj as *mut ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_DeleteSources(
    mut n: uint32,
    mut psrcobj: *mut audio_Object,
) {
    alDeleteSources(n as ALsizei, psrcobj as *mut ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_IsSource(mut srcobj: audio_Object) -> bool {
    return alIsSource(srcobj as ALuint) != 0;
}
#[no_mangle]
pub unsafe extern "C" fn openAL_Sourcei(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: audio_IntVal,
) {
    alSourcei(srcobj as ALuint, pname, value as ALint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_Sourcef(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: ::core::ffi::c_float,
) {
    alSourcef(srcobj as ALuint, pname, value);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_Sourcefv(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    alSourcefv(srcobj as ALuint, pname, value);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_GetSourcei(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut audio_IntVal,
) {
    alGetSourcei(srcobj as ALuint, pname, value as *mut ALint);
    if pname == 0x1010 as ::core::ffi::c_int {
        match *value {
            0x1011 => {
                *value = C2Rust_Unnamed_0::audio_INITIAL.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            0x1014 => {
                *value = C2Rust_Unnamed_0::audio_STOPPED.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            0x1012 => {
                *value = C2Rust_Unnamed_0::audio_PLAYING.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            0x1013 => {
                *value = C2Rust_Unnamed_0::audio_PAUSED.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
            _ => {
                log_add(
                    log_Level::log_Debug,
                    b"openAL_GetSourcei(): unknown value %x\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *value,
                );
                *value = C2Rust_Unnamed_0::audio_DRIVER_FAILURE.0 as ::core::ffi::c_int
                    as audio_IntVal;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn openAL_GetSourcef(
    mut srcobj: audio_Object,
    pname: audio_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    alGetSourcef(srcobj as ALuint, pname, value);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_SourceRewind(mut srcobj: audio_Object) {
    alSourceRewind(srcobj as ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_SourcePlay(mut srcobj: audio_Object) {
    alSourcePlay(srcobj as ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_SourcePause(mut srcobj: audio_Object) {
    alSourcePause(srcobj as ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_SourceStop(mut srcobj: audio_Object) {
    alSourceStop(srcobj as ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_SourceQueueBuffers(
    mut srcobj: audio_Object,
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    alSourceQueueBuffers(srcobj as ALuint, n as ALsizei, pbufobj as *mut ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_SourceUnqueueBuffers(
    mut srcobj: audio_Object,
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    alSourceUnqueueBuffers(srcobj as ALuint, n as ALsizei, pbufobj as *mut ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_GenBuffers(
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    alGenBuffers(n as ALsizei, pbufobj as *mut ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_DeleteBuffers(
    mut n: uint32,
    mut pbufobj: *mut audio_Object,
) {
    alDeleteBuffers(n as ALsizei, pbufobj as *mut ALuint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_IsBuffer(mut bufobj: audio_Object) -> bool {
    return alIsBuffer(bufobj as ALuint) != 0;
}
#[no_mangle]
pub unsafe extern "C" fn openAL_GetBufferi(
    mut bufobj: audio_Object,
    pname: audio_BufferProp,
    mut value: *mut audio_IntVal,
) {
    alGetBufferi(bufobj as ALuint, pname, value as *mut ALint);
}
#[no_mangle]
pub unsafe extern "C" fn openAL_BufferData(
    mut bufobj: audio_Object,
    mut format: uint32,
    mut data: *mut ::core::ffi::c_void,
    mut size: uint32,
    mut freq: uint32,
) {
    alBufferData(
        bufobj as ALuint,
        format as ALenum,
        data,
        size as ALsizei,
        freq as ALsizei,
    );
}
