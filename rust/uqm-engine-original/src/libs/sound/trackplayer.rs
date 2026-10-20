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

// Rust translation of src/libs/sound/trackplayer.c.
extern "C" {
    pub type uio_DirHandle;
    fn audio_GetSourcei(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: *mut audio_IntVal,
    );
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn GetTimeCounter() -> TimeCount;
    fn SoundDecoder_Load(
        dir: *mut uio_DirHandle,
        filename: *mut ::core::ffi::c_char,
        buffer_size: uint32,
        startTime: uint32,
        runTime: sint32,
    ) -> *mut TFB_SoundDecoder;
    fn SoundDecoder_DecodeAll(decoder: *mut TFB_SoundDecoder) -> uint32;
    fn SoundDecoder_Seek(decoder: *mut TFB_SoundDecoder, msecs: uint32);
    fn SoundDecoder_Rewind(decoder: *mut TFB_SoundDecoder);
    fn SoundDecoder_Free(decoder: *mut TFB_SoundDecoder);
    fn SoundDecoder_GetName(
        decoder: *mut TFB_SoundDecoder,
    ) -> *const ::core::ffi::c_char;
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn TFB_CreateSoundSample(
        _: *mut TFB_SoundDecoder,
        num_buffers: uint32,
        _: *const TFB_SoundCallbacks,
    ) -> *mut TFB_SoundSample;
    fn TFB_DestroySoundSample(_: *mut TFB_SoundSample);
    fn TFB_ClearBufferTag(_: *mut TFB_SoundTag);
    fn TFB_TagBuffer(
        _: *mut TFB_SoundSample,
        buffer: audio_Object,
        data: intptr_t,
    ) -> bool;
    fn PlayStream(
        sample: *mut TFB_SoundSample,
        source: uint32,
        looping: bool,
        scope: bool,
        rewind: bool,
    );
    fn StopStream(source: uint32);
    fn PauseStream(source: uint32);
    fn ResumeStream(source: uint32);
    fn PlayingStream(source: uint32) -> BOOLEAN;
    static mut soundSource: [TFB_SoundSource; 0];
    static mut contentDir: *mut uio_DirHandle;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn ispunct(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtoul(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint16 = ::core::ffi::c_ushort;
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
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type TimeCount = DWORD;
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
pub type TFB_SoundSource = tfb_soundsource;
pub type CallbackArg = *mut ::core::ffi::c_void;
pub type CallbackFunction = Option<unsafe extern "C" fn(CallbackArg) -> ()>;
pub type TFB_SoundChunk = tfb_soundchunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_soundchunk {
    pub decoder: *mut TFB_SoundDecoder,
    pub start_time: ::core::ffi::c_float,
    pub tag_me: ::core::ffi::c_int,
    pub track_num: uint32,
    pub text: *mut UNICODE,
    pub callback: CallbackFunction,
    pub next: *mut tfb_soundchunk,
}
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
pub type SUBTITLE_REF = *mut tfb_soundchunk;
static mut track_count: ::core::ffi::c_int = 0;
static mut no_page_break: ::core::ffi::c_int = 0;
static mut sound_sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<
    TFB_SoundSample,
>();
static mut tracks_length: uint32 = 0;
static mut chunks_head: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
static mut chunks_tail: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
static mut last_sub: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
static mut cur_chunk: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
static mut cur_sub_chunk: *mut TFB_SoundChunk = ::core::ptr::null_mut::<
    TFB_SoundChunk,
>();
static mut trackCBs: TFB_SoundCallbacks = tfb_soundcallbacks {
    OnStartStream: Some(
        OnStreamStart as unsafe extern "C" fn(*mut TFB_SoundSample) -> bool,
    ),
    OnEndChunk: Some(
        OnChunkEnd as unsafe extern "C" fn(*mut TFB_SoundSample, audio_Object) -> bool,
    ),
    OnEndStream: Some(OnStreamEnd as unsafe extern "C" fn(*mut TFB_SoundSample) -> ()),
    OnTaggedBuffer: Some(
        OnBufferTag
            as unsafe extern "C" fn(*mut TFB_SoundSample, *mut TFB_SoundTag) -> (),
    ),
    OnQueueBuffer: None,
};
#[inline]
unsafe extern "C" fn chunk_end_time(mut chunk: *mut TFB_SoundChunk) -> sint32 {
    return (((*chunk).start_time + (*(*chunk).decoder).length)
        * 840 as ::core::ffi::c_int as ::core::ffi::c_float) as sint32;
}
#[inline]
unsafe extern "C" fn tracks_end_time() -> sint32 {
    return chunk_end_time(chunks_tail);
}
#[no_mangle]
pub unsafe extern "C" fn JumpTrack() {
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    seek_track(
        ::core::ptr::read_volatile::<uint32>(&raw const tracks_length)
            .wrapping_add(1 as uint32) as sint32,
    );
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PlayTrack() {
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    ::core::ptr::write_volatile(&raw mut tracks_length, tracks_end_time() as uint32);
    cur_chunk = chunks_head;
    PlayStream(
        sound_sample,
        (0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as uint32,
        false,
        true,
        true,
    );
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PauseTrack() {
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    PauseStream(
        (0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as uint32,
    );
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ResumeTrack() {
    let mut state: audio_IntVal = 0;
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    if cur_chunk.is_null() {
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        return;
    }
    audio_GetSourcei(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .handle,
        C2Rust_Unnamed::audio_SOURCE_STATE.0 as audio_SourceProp,
        &raw mut state,
    );
    if state == C2Rust_Unnamed::audio_PAUSED.0 as ::core::ffi::c_int as audio_IntVal {
        ResumeStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
        );
    }
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PlayingTrack() -> COUNT {
    let mut result: COUNT = 0 as COUNT;
    if sound_sample.is_null() {
        return 0 as COUNT;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    if !cur_chunk.is_null() {
        result = (*cur_chunk).track_num.wrapping_add(1 as uint32) as COUNT;
    }
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn StopTrack() {
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    StopStream(
        (0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as uint32,
    );
    track_count = 0 as ::core::ffi::c_int;
    ::core::ptr::write_volatile(&raw mut tracks_length, 0 as uint32);
    cur_chunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    cur_sub_chunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    if !chunks_head.is_null() {
        chunks_tail = ::core::ptr::null_mut::<TFB_SoundChunk>();
        destroy_SoundChunk_list(chunks_head);
        chunks_head = ::core::ptr::null_mut::<TFB_SoundChunk>();
        last_sub = ::core::ptr::null_mut::<TFB_SoundChunk>();
    }
    if !sound_sample.is_null() {
        (*sound_sample).decoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
        TFB_DestroySoundSample(sound_sample);
        sound_sample = ::core::ptr::null_mut::<TFB_SoundSample>();
    }
}
unsafe extern "C" fn DoTrackTag(mut chunk: *mut TFB_SoundChunk) {
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    if (*chunk).callback.is_some() {
        (*chunk)
            .callback
            .expect(
                "non-null function pointer",
            )(::core::ptr::null_mut::<::core::ffi::c_void>());
    }
    cur_sub_chunk = chunk;
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
unsafe extern "C" fn OnStreamStart(mut sample: *mut TFB_SoundSample) -> bool {
    if sample != sound_sample {
        return false;
    }
    if cur_chunk.is_null() {
        return false;
    }
    (*sample).decoder = (*cur_chunk).decoder;
    (*sample).offset = ((*cur_chunk).start_time
        * 840 as ::core::ffi::c_int as ::core::ffi::c_float) as sint32;
    if (*cur_chunk).tag_me != 0 {
        DoTrackTag(cur_chunk);
    }
    return true;
}
unsafe extern "C" fn OnChunkEnd(
    mut sample: *mut TFB_SoundSample,
    mut buffer: audio_Object,
) -> bool {
    if sample != sound_sample {
        return false;
    }
    if cur_chunk.is_null() || (*cur_chunk).next.is_null() {
        return false;
    }
    cur_chunk = (*cur_chunk).next as *mut TFB_SoundChunk;
    (*sample).decoder = (*cur_chunk).decoder;
    SoundDecoder_Rewind((*sample).decoder);
    log_add(
        log_Level::log_Info,
        b"Switching to stream %s at pos %d\0".as_ptr() as *const ::core::ffi::c_char,
        (*(*sample).decoder).filename,
        (*(*sample).decoder).start_sample,
    );
    if (*cur_chunk).tag_me != 0 {
        TFB_TagBuffer(sample, buffer, cur_chunk.expose_provenance() as intptr_t);
    }
    return true;
}
unsafe extern "C" fn OnStreamEnd(mut sample: *mut TFB_SoundSample) {
    if sample != sound_sample {
        return;
    }
    cur_chunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    cur_sub_chunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
}
unsafe extern "C" fn OnBufferTag(
    mut sample: *mut TFB_SoundSample,
    mut tag: *mut TFB_SoundTag,
) {
    let mut chunk: *mut TFB_SoundChunk = ::core::ptr::with_exposed_provenance_mut::<
        TFB_SoundChunk,
    >((*tag).data as usize);
    (::core::mem::size_of::<intptr_t>() >= ::core::mem::size_of::<*mut TFB_SoundChunk>()
        || {
            __assert_fail(
                b"sizeof (tag->data) >= sizeof (chunk)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/sound/trackplayer.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int,
                b"OnBufferTag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if sample != sound_sample {
        return;
    }
    TFB_ClearBufferTag(tag);
    DoTrackTag(chunk);
}
unsafe extern "C" fn GetTimeStamps(
    mut TimeStamps: *mut UNICODE,
    mut time_stamps: *mut sint32,
) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *TimeStamps as ::core::ffi::c_int != 0
        && {
            pos = strcspn(TimeStamps, b",\r\n\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            pos != 0
        }
    {
        let mut valStr: [UNICODE; 32] = [0; 32];
        let mut val: uint32 = 0;
        memcpy(
            &raw mut valStr as *mut UNICODE as *mut ::core::ffi::c_void,
            TimeStamps as *const ::core::ffi::c_void,
            pos as size_t,
        );
        valStr[pos as usize] = '\0' as UNICODE;
        val = strtoul(
            &raw mut valStr as *mut UNICODE,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as uint32;
        if val != 0 {
            *time_stamps = val as sint32;
            num += 1;
            time_stamps = time_stamps.offset(1);
        }
        TimeStamps = TimeStamps.offset(pos as isize);
        TimeStamps = TimeStamps
            .offset(
                strspn(TimeStamps, b",\r\n\0".as_ptr() as *const ::core::ffi::c_char)
                    as isize,
            );
    }
    return num;
}
unsafe extern "C" fn SplitSubPages(
    mut text: *mut UNICODE,
    mut pages: *mut *mut UNICODE,
    mut timestamp: *mut sint32,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lead_ellips: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut page: COUNT = 0;
    page = 0 as COUNT;
    while (page as ::core::ffi::c_int) < size
        && *text as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut aft_ellips: ::core::ffi::c_int = 0;
        let mut pos: ::core::ffi::c_int = 0;
        pos = strcspn(text, b"\r\n\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        aft_ellips = 3 as ::core::ffi::c_int
            * (*text.offset(pos as isize) as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int && pos > 0 as ::core::ffi::c_int
                && ispunct(
                    *text.offset((pos - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                ) == 0
                && isspace(
                    *text.offset((pos - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                ) == 0) as ::core::ffi::c_int;
        *pages.offset(page as isize) = HMalloc(
            ::core::mem::size_of::<UNICODE>()
                .wrapping_mul(
                    (lead_ellips + pos + aft_ellips + 1 as ::core::ffi::c_int) as size_t,
                ),
        ) as *mut UNICODE;
        if lead_ellips != 0 {
            strcpy(
                *pages.offset(page as isize),
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        memcpy(
            (*pages.offset(page as isize)).offset(lead_ellips as isize)
                as *mut ::core::ffi::c_void,
            text as *const ::core::ffi::c_void,
            pos as size_t,
        );
        *(*pages.offset(page as isize)).offset((lead_ellips + pos) as isize) = '\0'
            as UNICODE;
        if aft_ellips != 0 {
            strcpy(
                (*pages.offset(page as isize))
                    .offset(lead_ellips as isize)
                    .offset(pos as isize),
                b"...\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        *timestamp.offset(page as isize) = (pos * 80 as ::core::ffi::c_int) as sint32;
        if *timestamp.offset(page as isize) < 1000 as ::core::ffi::c_int {
            *timestamp.offset(page as isize) = 1000 as ::core::ffi::c_int as sint32;
        }
        lead_ellips = if aft_ellips != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        text = text.offset(pos as isize);
        text = text
            .offset(
                strspn(text, b"\r\n\0".as_ptr() as *const ::core::ffi::c_char) as isize,
            );
        page = page.wrapping_add(1);
    }
    return page as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SpliceMultiTrack(
    mut TrackNames: *mut *mut UNICODE,
    mut TrackText: *mut UNICODE,
) {
    let mut track_decs: [*mut TFB_SoundDecoder; 21] = [::core::ptr::null_mut::<
        TFB_SoundDecoder,
    >(); 21];
    let mut tracks: ::core::ffi::c_int = 0;
    let mut slen1: ::core::ffi::c_int = 0;
    let mut slen2: ::core::ffi::c_int = 0;
    if TrackText.is_null() {
        log_add(
            log_Level::log_Debug,
            b"SpliceMultiTrack(): no track text\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if sound_sample.is_null() || chunks_tail.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SpliceMultiTrack(): Cannot be called before SpliceTrack()\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    log_add(
        log_Level::log_Info,
        b"SpliceMultiTrack(): loading...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    tracks = 0 as ::core::ffi::c_int;
    while !(*TrackNames).is_null() && tracks < 20 as ::core::ffi::c_int {
        track_decs[tracks as usize] = SoundDecoder_Load(
            contentDir,
            *TrackNames,
            32768 as uint32,
            0 as uint32,
            -3 as sint32 * 80 as sint32,
        );
        if !track_decs[tracks as usize].is_null() {
            log_add(
                log_Level::log_Info,
                b"  track: %s, decoder: %s, rate %d format %x\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *TrackNames,
                SoundDecoder_GetName(track_decs[tracks as usize]),
                (*track_decs[tracks as usize]).frequency,
                (*track_decs[tracks as usize]).format,
            );
            SoundDecoder_DecodeAll(track_decs[tracks as usize]);
            (*chunks_tail).next = create_SoundChunk(
                track_decs[tracks as usize],
                (*sound_sample).length,
            ) as *mut tfb_soundchunk;
            chunks_tail = (*chunks_tail).next as *mut TFB_SoundChunk;
            (*chunks_tail).track_num = (track_count - 1 as ::core::ffi::c_int) as uint32;
            (*sound_sample).length += (*track_decs[tracks as usize]).length;
        } else {
            log_add(
                log_Level::log_Warning,
                b"SpliceMultiTrack(): couldn't load %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *TrackNames,
            );
            tracks -= 1;
        }
        TrackNames = TrackNames.offset(1);
        tracks += 1;
    }
    track_decs[tracks as usize] = ::core::ptr::null_mut::<TFB_SoundDecoder>();
    if tracks == 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"SpliceMultiTrack(): no tracks loaded\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    slen1 = strlen((*last_sub).text) as ::core::ffi::c_int;
    slen2 = strlen(TrackText) as ::core::ffi::c_int;
    (*last_sub).text = HRealloc(
        (*last_sub).text as *mut ::core::ffi::c_void,
        (slen1 + slen2 + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut UNICODE;
    strcpy((*last_sub).text.offset(slen1 as isize), TrackText);
    no_page_break = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SpliceTrack(
    mut TrackName: *mut UNICODE,
    mut TrackText: *mut UNICODE,
    mut TimeStamp: *mut UNICODE,
    mut cb: CallbackFunction,
) {
    static mut last_track_name: [UNICODE; 128] = unsafe {
        ::core::mem::transmute::<
            [u8; 128],
            [UNICODE; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        )
    };
    static mut dec_offset: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut pages: [*mut UNICODE; 50] = [::core::ptr::null_mut::<UNICODE>(); 50];
    let mut time_stamps: [sint32; 50] = [0; 50];
    let mut num_pages: ::core::ffi::c_int = 0;
    let mut page: ::core::ffi::c_int = 0;
    if TrackText.is_null() {
        return;
    }
    if TrackName.is_null() {
        let mut slen1: ::core::ffi::c_int = 0;
        let mut slen2: ::core::ffi::c_int = 0;
        if track_count == 0 as ::core::ffi::c_int {
            log_add(
                log_Level::log_Warning,
                b"SpliceTrack(): Tried to append a subtitle, but no current track\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if last_sub.is_null() || (*last_sub).text.is_null() {
            log_add(
                log_Level::log_Warning,
                b"SpliceTrack(): Tried to append a subtitle to a NULL string\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        num_pages = SplitSubPages(
            TrackText,
            &raw mut pages as *mut *mut UNICODE,
            &raw mut time_stamps as *mut sint32,
            50 as ::core::ffi::c_int,
        );
        if num_pages == 0 as ::core::ffi::c_int {
            log_add(
                log_Level::log_Warning,
                b"SpliceTrack(): Failed to parse subtitles\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        time_stamps[(num_pages - 1 as ::core::ffi::c_int) as usize] = -time_stamps[(num_pages
            - 1 as ::core::ffi::c_int) as usize];
        slen1 = strlen((*last_sub).text) as ::core::ffi::c_int;
        slen2 = strlen(pages[0usize]) as ::core::ffi::c_int;
        (*last_sub).text = HRealloc(
            (*last_sub).text as *mut ::core::ffi::c_void,
            (slen1 + slen2 + 1 as ::core::ffi::c_int) as size_t,
        ) as *mut UNICODE;
        strcpy((*last_sub).text.offset(slen1 as isize), pages[0usize]);
        HFree(pages[0usize] as *mut ::core::ffi::c_void);
        page = 1 as ::core::ffi::c_int;
        while page < num_pages {
            let mut next_sub: *mut TFB_SoundChunk = find_next_page(last_sub);
            if !next_sub.is_null() {
                (*next_sub).text = pages[page as usize];
                last_sub = next_sub;
            } else {
                let mut decoder: *mut TFB_SoundDecoder = SoundDecoder_Load(
                    contentDir,
                    &raw mut last_track_name as *mut ::core::ffi::c_char,
                    4096 as uint32,
                    dec_offset as uint32,
                    time_stamps[page as usize],
                );
                if decoder.is_null() {
                    log_add(
                        log_Level::log_Warning,
                        b"SpliceTrack(): couldn't load %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        TrackName,
                    );
                    break;
                } else {
                    dec_offset = dec_offset
                        .wrapping_add(
                            ((*decoder).length
                                * 1000 as ::core::ffi::c_int as ::core::ffi::c_float)
                                as ::core::ffi::c_ulong,
                        );
                    (*chunks_tail).next = create_SoundChunk(
                        decoder,
                        (*sound_sample).length,
                    ) as *mut tfb_soundchunk;
                    chunks_tail = (*chunks_tail).next as *mut TFB_SoundChunk;
                    (*chunks_tail).tag_me = 1 as ::core::ffi::c_int;
                    (*chunks_tail).track_num = (track_count - 1 as ::core::ffi::c_int)
                        as uint32;
                    (*chunks_tail).text = pages[page as usize];
                    (*chunks_tail).callback = cb;
                    last_sub = chunks_tail;
                    (*sound_sample).length += (*decoder).length;
                }
            }
            page += 1;
        }
    } else {
        let mut num_timestamps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        utf8StringCopy(
            &raw mut last_track_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 128]>(),
            TrackName,
        );
        num_pages = SplitSubPages(
            TrackText,
            &raw mut pages as *mut *mut UNICODE,
            &raw mut time_stamps as *mut sint32,
            50 as ::core::ffi::c_int,
        );
        if num_pages == 0 as ::core::ffi::c_int {
            log_add(
                log_Level::log_Warning,
                b"SpliceTrack(): Failed to parse sutitles\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        time_stamps[(num_pages - 1 as ::core::ffi::c_int) as usize] = -time_stamps[(num_pages
            - 1 as ::core::ffi::c_int) as usize];
        if no_page_break != 0 && track_count != 0 {
            let mut slen1_0: ::core::ffi::c_int = 0;
            let mut slen2_0: ::core::ffi::c_int = 0;
            slen1_0 = strlen((*last_sub).text) as ::core::ffi::c_int;
            slen2_0 = strlen(pages[0usize]) as ::core::ffi::c_int;
            (*last_sub).text = HRealloc(
                (*last_sub).text as *mut ::core::ffi::c_void,
                (slen1_0 + slen2_0 + 1 as ::core::ffi::c_int) as size_t,
            ) as *mut UNICODE;
            strcpy((*last_sub).text.offset(slen1_0 as isize), pages[0usize]);
            HFree(pages[0usize] as *mut ::core::ffi::c_void);
        } else {
            track_count += 1;
        }
        log_add(
            log_Level::log_Info,
            b"SpliceTrack(): loading %s\0".as_ptr() as *const ::core::ffi::c_char,
            TrackName,
        );
        if !TimeStamp.is_null() {
            num_timestamps = GetTimeStamps(
                TimeStamp,
                &raw mut time_stamps as *mut sint32,
            ) + 1 as ::core::ffi::c_int;
            if num_timestamps < num_pages {
                log_add(
                    log_Level::log_Warning,
                    b"SpliceTrack(): number of timestamps doesn't match number of pages!\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else if num_timestamps > num_pages {
                time_stamps[(num_timestamps - 1 as ::core::ffi::c_int) as usize] = -100000
                    as ::core::ffi::c_int as sint32;
            }
        } else {
            num_timestamps = num_pages;
        }
        dec_offset = 0 as ::core::ffi::c_ulong;
        page = 0 as ::core::ffi::c_int;
        while page < num_timestamps {
            let mut decoder_0: *mut TFB_SoundDecoder = SoundDecoder_Load(
                contentDir,
                TrackName,
                4096 as uint32,
                dec_offset as uint32,
                time_stamps[page as usize],
            );
            if decoder_0.is_null() {
                log_add(
                    log_Level::log_Warning,
                    b"SpliceTrack(): couldn't load %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    TrackName,
                );
                break;
            } else {
                if sound_sample.is_null() {
                    sound_sample = TFB_CreateSoundSample(
                        ::core::ptr::null_mut::<TFB_SoundDecoder>(),
                        8 as uint32,
                        &raw mut trackCBs,
                    );
                    chunks_head = create_SoundChunk(decoder_0, 0.0f32);
                    chunks_tail = chunks_head;
                } else {
                    (*chunks_tail).next = create_SoundChunk(
                        decoder_0,
                        (*sound_sample).length,
                    ) as *mut tfb_soundchunk;
                    chunks_tail = (*chunks_tail).next as *mut TFB_SoundChunk;
                }
                dec_offset = dec_offset
                    .wrapping_add(
                        ((*decoder_0).length
                            * 1000 as ::core::ffi::c_int as ::core::ffi::c_float)
                            as ::core::ffi::c_ulong,
                    );
                (*sound_sample).length += (*decoder_0).length;
                (*chunks_tail).track_num = (track_count - 1 as ::core::ffi::c_int)
                    as uint32;
                if no_page_break == 0 {
                    (*chunks_tail).tag_me = 1 as ::core::ffi::c_int;
                    if page < num_pages {
                        (*chunks_tail).text = pages[page as usize];
                        last_sub = chunks_tail;
                    }
                    (*chunks_tail).callback = cb;
                }
                no_page_break = 0 as ::core::ffi::c_int;
                page += 1;
            }
        }
    };
}
unsafe extern "C" fn seek_track(mut offset: sint32) {
    let mut cur: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    let mut last_tag: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    if sound_sample.is_null() {
        return;
    }
    if offset < 0 as ::core::ffi::c_int {
        offset = 0 as ::core::ffi::c_int as sint32;
    } else if offset as uint32
        > ::core::ptr::read_volatile::<uint32>(&raw const tracks_length)
    {
        offset = ::core::ptr::read_volatile::<uint32>(&raw const tracks_length)
            .wrapping_add(1 as uint32) as sint32;
    }
    (*(&raw mut soundSource as *mut TFB_SoundSource)
        .offset(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ))
        .start_time = GetTimeCounter().wrapping_sub(offset as TimeCount) as sint32;
    cur = chunks_head;
    while !cur.is_null() && offset >= chunk_end_time(cur) {
        if (*cur).tag_me != 0 {
            last_tag = cur;
        }
        cur = (*cur).next as *mut TFB_SoundChunk;
    }
    if !cur.is_null() {
        cur_chunk = cur;
        SoundDecoder_Seek(
            (*cur).decoder,
            ((offset as ::core::ffi::c_float
                / 840 as ::core::ffi::c_int as ::core::ffi::c_float - (*cur).start_time)
                * 1000 as ::core::ffi::c_int as ::core::ffi::c_float) as uint32,
        );
        (*sound_sample).decoder = (*cur).decoder;
        if (*cur).tag_me != 0 {
            last_tag = cur;
        }
        if !last_tag.is_null() {
            DoTrackTag(last_tag);
        }
    } else {
        StopStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
        );
        cur_chunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
        cur_sub_chunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    };
}
unsafe extern "C" fn get_current_track_pos() -> sint32 {
    let mut start_time: sint32 = (*(&raw mut soundSource as *mut TFB_SoundSource)
        .offset(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ))
        .start_time;
    let mut pos: sint32 = GetTimeCounter().wrapping_sub(start_time as TimeCount)
        as sint32;
    if pos < 0 as ::core::ffi::c_int {
        pos = 0 as ::core::ffi::c_int as sint32;
    } else if pos as uint32
        > ::core::ptr::read_volatile::<uint32>(&raw const tracks_length)
    {
        pos = ::core::ptr::read_volatile::<uint32>(&raw const tracks_length) as sint32;
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn FastReverse_Smooth() {
    let mut offset: sint32 = 0;
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    offset = get_current_track_pos();
    offset -= 300 as ::core::ffi::c_int;
    seek_track(offset);
    if PlayingStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
        )
        .0 == 0
    {
        PlayStream(
            sound_sample,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
            false,
            true,
            false,
        );
    }
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FastForward_Smooth() {
    let mut offset: sint32 = 0;
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    offset = get_current_track_pos();
    offset += 300 as ::core::ffi::c_int;
    seek_track(offset);
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FastReverse_Page() {
    let mut prev: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    prev = find_prev_page(cur_sub_chunk);
    if !prev.is_null() {
        cur_chunk = prev;
        cur_sub_chunk = prev;
        PlayStream(
            sound_sample,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
            false,
            true,
            true,
        );
    }
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FastForward_Page() {
    let mut next: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    if sound_sample.is_null() {
        return;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    next = find_next_page(cur_sub_chunk);
    if !next.is_null() {
        cur_chunk = next;
        cur_sub_chunk = next;
        PlayStream(
            sound_sample,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
            false,
            true,
            true,
        );
    } else {
        seek_track(
            ::core::ptr::read_volatile::<uint32>(&raw const tracks_length)
                .wrapping_add(1 as uint32) as sint32,
        );
    }
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetTrackPosition(
    mut in_units: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut offset: uint32 = 0;
    let mut length: uint32 = ::core::ptr::read_volatile::<
        uint32,
    >(&raw const tracks_length);
    if sound_sample.is_null() || length == 0 as uint32 {
        return 0 as ::core::ffi::c_int;
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    offset = get_current_track_pos() as uint32;
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    return (in_units as uint32).wrapping_mul(offset).wrapping_div(length)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn create_SoundChunk(
    mut decoder: *mut TFB_SoundDecoder,
    mut start_time: ::core::ffi::c_float,
) -> *mut TFB_SoundChunk {
    let mut chunk: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    chunk = HCalloc(::core::mem::size_of::<TFB_SoundChunk>()) as *mut TFB_SoundChunk;
    (*chunk).decoder = decoder;
    (*chunk).start_time = start_time;
    return chunk;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_SoundChunk_list(mut chunk: *mut TFB_SoundChunk) {
    let mut next: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    while !chunk.is_null() {
        next = (*chunk).next as *mut TFB_SoundChunk;
        if !(*chunk).decoder.is_null() {
            SoundDecoder_Free((*chunk).decoder);
        }
        HFree((*chunk).text as *mut ::core::ffi::c_void);
        HFree(chunk as *mut ::core::ffi::c_void);
        chunk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_next_page(
    mut cur: *mut TFB_SoundChunk,
) -> *mut TFB_SoundChunk {
    if cur.is_null() {
        return ::core::ptr::null_mut::<TFB_SoundChunk>();
    }
    cur = (*cur).next as *mut TFB_SoundChunk;
    while !cur.is_null() && (*cur).tag_me == 0 {
        cur = (*cur).next as *mut TFB_SoundChunk;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn find_prev_page(
    mut cur: *mut TFB_SoundChunk,
) -> *mut TFB_SoundChunk {
    let mut prev: *mut TFB_SoundChunk = ::core::ptr::null_mut::<TFB_SoundChunk>();
    let mut last_valid: *mut TFB_SoundChunk = chunks_head;
    if cur == chunks_head {
        return cur;
    }
    prev = chunks_head;
    while !prev.is_null() && prev != cur {
        if (*prev).tag_me != 0 {
            last_valid = prev;
        }
        prev = (*prev).next as *mut TFB_SoundChunk;
    }
    return last_valid;
}
#[no_mangle]
pub unsafe extern "C" fn GetFirstTrackSubtitle() -> SUBTITLE_REF {
    return chunks_head;
}
#[no_mangle]
pub unsafe extern "C" fn GetNextTrackSubtitle(
    mut LastRef: SUBTITLE_REF,
) -> SUBTITLE_REF {
    if LastRef.is_null() {
        return ::core::ptr::null_mut::<tfb_soundchunk>();
    }
    return find_next_page(LastRef);
}
#[no_mangle]
pub unsafe extern "C" fn GetTrackSubtitleText(
    mut SubRef: SUBTITLE_REF,
) -> *const UNICODE {
    if SubRef.is_null() {
        return ::core::ptr::null::<UNICODE>();
    }
    return (*SubRef).text;
}
#[no_mangle]
pub unsafe extern "C" fn GetTrackSubtitle() -> *const UNICODE {
    let mut cur_sub: *const UNICODE = ::core::ptr::null::<UNICODE>();
    if sound_sample.is_null() {
        return ::core::ptr::null::<UNICODE>();
    }
    LockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    if !cur_sub_chunk.is_null() {
        cur_sub = (*cur_sub_chunk).text;
    }
    UnlockMutex(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ))
            .stream_mutex,
    );
    return cur_sub;
}
