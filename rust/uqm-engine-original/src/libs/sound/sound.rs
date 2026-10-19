//Copyright Paul Reiche, Fred Ford. 1992-2002

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

// Rust translation of src/libs/sound/sound.c.
extern "C" {
    pub type uio_DirHandle;
    fn audio_Sourcef(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: ::core::ffi::c_float,
    );
    fn audio_GetSourcei(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: *mut audio_IntVal,
    );
    fn audio_SourceRewind(srcobj: audio_Object);
    fn audio_SourceStop(srcobj: audio_Object);
    fn audio_SourceUnqueueBuffers(
        srcobj: audio_Object,
        n: uint32,
        pbufobj: *mut audio_Object,
    );
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn ChannelPlaying(Channel: COUNT) -> BOOLEAN;
    fn SetMusicVolume(Volume: COUNT);
    static mut QuitPosted: ::core::ffi::c_int;
    fn PlayingStream(source: uint32) -> BOOLEAN;
    fn SetMusicStreamFade(howLong: sint32, endVolume: ::core::ffi::c_int) -> bool;
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type audio_Object = uintptr_t;
pub type audio_IntVal = intptr_t;
pub type audio_SourceProp = sint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub type size_t = usize;
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
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
pub type Mutex = *mut ::core::ffi::c_void;
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
pub static mut musicVolume: ::core::ffi::c_int = 160 as ::core::ffi::c_int;
#[no_mangle]
pub static mut musicVolumeScale: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut sfxVolumeScale: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut speechVolumeScale: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut soundSource: [TFB_SoundSource; 7] = [TFB_SoundSource {
    sample: ::core::ptr::null_mut::<TFB_SoundSample>(),
    handle: 0,
    stream_should_be_playing: false,
    stream_mutex: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    start_time: 0,
    pause_time: 0,
    positional_object: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    last_q_buf: 0,
    sbuffer: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    sbuf_size: 0,
    sbuf_tail: 0,
    sbuf_head: 0,
    sbuf_lasttime: 0,
}; 7];
#[no_mangle]
pub unsafe extern "C" fn StopSound() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        <= 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int
    {
        StopSource(i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CleanSource(mut iSource: ::core::ffi::c_int) {
    let mut processed: audio_IntVal = 0;
    soundSource[iSource as usize].positional_object = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    audio_GetSourcei(
        soundSource[iSource as usize].handle,
        C2Rust_Unnamed::audio_BUFFERS_PROCESSED.0 as audio_SourceProp,
        &raw mut processed,
    );
    if processed != 0 as audio_IntVal {
        let mut stack_bufs: [audio_Object; 64] = [0; 64];
        let mut bufs: *mut audio_Object = ::core::ptr::null_mut::<audio_Object>();
        if processed > 64 as audio_IntVal {
            bufs = HMalloc(
                ::core::mem::size_of::<audio_Object>().wrapping_mul(processed as size_t),
            ) as *mut audio_Object;
        } else {
            bufs = &raw mut stack_bufs as *mut audio_Object;
        }
        audio_SourceUnqueueBuffers(
            soundSource[iSource as usize].handle,
            processed as uint32,
            bufs,
        );
        if processed > 64 as audio_IntVal {
            HFree(bufs as *mut ::core::ffi::c_void);
        }
    }
    audio_SourceRewind(soundSource[iSource as usize].handle);
}
#[no_mangle]
pub unsafe extern "C" fn StopSource(mut iSource: ::core::ffi::c_int) {
    audio_SourceStop(soundSource[iSource as usize].handle);
    CleanSource(iSource);
}
#[no_mangle]
pub unsafe extern "C" fn SoundPlaying() -> BOOLEAN {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
    {
        let mut sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<
            TFB_SoundSample,
        >();
        sample = soundSource[i as usize].sample;
        if !sample.is_null() && !(*sample).decoder.is_null() {
            let mut result: BOOLEAN = BOOLEAN::FALSE;
            LockMutex(soundSource[i as usize].stream_mutex);
            result = PlayingStream(i as uint32);
            UnlockMutex(soundSource[i as usize].stream_mutex);
            if result.0 != 0 {
                return BOOLEAN::TRUE;
            }
        } else {
            let mut state: audio_IntVal = 0;
            audio_GetSourcei(
                soundSource[i as usize].handle,
                C2Rust_Unnamed::audio_SOURCE_STATE.0 as audio_SourceProp,
                &raw mut state,
            );
            if state
                == C2Rust_Unnamed::audio_PLAYING.0 as ::core::ffi::c_int as audio_IntVal
            {
                return BOOLEAN::TRUE;
            }
        }
        i += 1;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn WaitForSoundEnd(mut Channel: COUNT) {
    while if Channel as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        SoundPlaying().0
    } else {
        ChannelPlaying(Channel).0
    } != 0
    {
        SleepThread(
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimePeriod,
        );
        if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitSound(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> BOOLEAN {
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn UninitSound() {}
#[no_mangle]
pub unsafe extern "C" fn SetSFXVolume(mut volume: ::core::ffi::c_float) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        <= 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int
    {
        audio_Sourcef(
            soundSource[i as usize].handle,
            C2Rust_Unnamed::audio_GAIN.0 as audio_SourceProp,
            volume,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetSpeechVolume(mut volume: ::core::ffi::c_float) {
    audio_Sourcef(
        soundSource[(0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize]
            .handle,
        C2Rust_Unnamed::audio_GAIN.0 as audio_SourceProp,
        volume,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FadeMusic(mut end_vol: BYTE, mut TimeInterval: SIZE) -> DWORD {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
        TimeInterval = 0 as SIZE;
    }
    if (TimeInterval as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        TimeInterval = 0 as SIZE;
    }
    if !SetMusicStreamFade(TimeInterval as sint32, end_vol as ::core::ffi::c_int) {
        SetMusicVolume(end_vol as COUNT);
        return GetTimeCounter();
    } else {
        return GetTimeCounter()
            .wrapping_add(TimeInterval as DWORD)
            .wrapping_add(1 as DWORD)
    };
}
