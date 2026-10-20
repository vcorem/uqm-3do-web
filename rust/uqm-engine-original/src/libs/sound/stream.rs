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

// Rust translation of src/libs/sound/stream.c.
extern "C" {
    pub type uio_DirHandle;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn audio_GetError() -> sint32;
    fn audio_Sourcei(srcobj: audio_Object, pname: audio_SourceProp, value: audio_IntVal);
    fn audio_GetSourcei(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: *mut audio_IntVal,
    );
    fn audio_SourcePlay(srcobj: audio_Object);
    fn audio_SourcePause(srcobj: audio_Object);
    fn audio_SourceQueueBuffers(
        srcobj: audio_Object,
        n: uint32,
        pbufobj: *mut audio_Object,
    );
    fn audio_SourceUnqueueBuffers(
        srcobj: audio_Object,
        n: uint32,
        pbufobj: *mut audio_Object,
    );
    fn audio_GenBuffers(n: uint32, pbufobj: *mut audio_Object);
    fn audio_DeleteBuffers(n: uint32, pbufobj: *mut audio_Object);
    fn audio_GetBufferi(
        bufobj: audio_Object,
        pname: audio_BufferProp,
        value: *mut audio_IntVal,
    );
    fn audio_BufferData(
        bufobj: audio_Object,
        format: uint32,
        data: *mut ::core::ffi::c_void,
        size: uint32,
        freq: uint32,
    );
    fn audio_GetFormatInfo(
        format: uint32,
        channels: *mut ::core::ffi::c_int,
        sample_size: *mut ::core::ffi::c_int,
    ) -> bool;
    fn SoundDecoder_Decode(decoder: *mut TFB_SoundDecoder) -> uint32;
    fn SoundDecoder_GetTime(decoder: *mut TFB_SoundDecoder) -> ::core::ffi::c_float;
    fn SoundDecoder_Seek(decoder: *mut TFB_SoundDecoder, msecs: uint32);
    fn SoundDecoder_Rewind(decoder: *mut TFB_SoundDecoder);
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn HibernateThread(timePeriod: TimePeriod);
    fn TaskSwitch();
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn GetTimeCounter() -> TimeCount;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn SetMusicVolume(Volume: COUNT);
    static mut soundSource: [TFB_SoundSource; 0];
    static mut musicVolume: ::core::ffi::c_int;
    fn StopSource(iSource: ::core::ffi::c_int);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn AssignTask(
        task_func: ThreadFunction,
        Stacksize: SDWORD,
        name: *const ::core::ffi::c_char,
    ) -> Task;
    fn Task_ReadState(task: Task, state_mask: DWORD) -> DWORD;
    fn FinishTask(task: Task);
    fn ConcludeTask(task: Task);
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type audio_BufferProp = sint32;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SOUNDDECODER_OK: Self = Self(0);
    pub const SOUNDDECODER_ERROR: Self = Self(1);
    pub const SOUNDDECODER_EOF: Self = Self(2);
}
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
pub type ThreadFunction = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type Thread = *mut ::core::ffi::c_void;
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
pub type Task = *mut taskstruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taskstruct {
    pub state_mutex: Mutex,
    pub state: DWORD,
    pub name: *const ::core::ffi::c_char,
    pub thread: Thread,
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
static mut decoderTask: Task = ::core::ptr::null_mut::<taskstruct>();
static mut musicFadeStartTime: TimeCount = 0;
static mut musicFadeInterval: sint32 = 0;
static mut musicFadeStartVolume: ::core::ffi::c_int = 0;
static mut musicFadeDelta: ::core::ffi::c_int = 0;
static mut fade_mutex: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn PlayStream(
    mut sample: *mut TFB_SoundSample,
    mut source: uint32,
    mut looping: bool,
    mut scope: bool,
    mut rewind: bool,
) {
    let mut i: uint32 = 0;
    let mut offset: sint32 = 0;
    let mut decoder: *mut TFB_SoundDecoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
    if sample.is_null() {
        return;
    }
    StopStream(source);
    if (*sample).callbacks.OnStartStream.is_some()
        && !(*sample).callbacks.OnStartStream.expect("non-null function pointer")(sample)
    {
        return;
    }
    if !(*sample).buffer_tag.is_null() {
        memset(
            (*sample).buffer_tag as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*sample).num_buffers as size_t)
                .wrapping_mul(::core::mem::size_of::<TFB_SoundTag>() as size_t),
        );
    }
    decoder = (*sample).decoder;
    offset = (*sample).offset;
    if rewind {
        SoundDecoder_Rewind(decoder);
    } else {
        offset
            += (SoundDecoder_GetTime(decoder)
                * 840 as ::core::ffi::c_int as ::core::ffi::c_float) as sint32;
    }
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize)).sample = sample;
    (*decoder).looping = looping;
    audio_Sourcei(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize)).handle,
        C2Rust_Unnamed::audio_LOOPING.0 as audio_SourceProp,
        0 as audio_IntVal,
    );
    if scope {
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
            .sbuf_size = (*sample)
            .num_buffers
            .wrapping_mul((*decoder).buffer_size)
            .wrapping_add(256 as uint32);
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
            .sbuffer = HCalloc(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
                .sbuf_size as size_t,
        );
    }
    i = 0 as uint32;
    while i < (*sample).num_buffers {
        let mut decoded_bytes: uint32 = 0;
        decoded_bytes = SoundDecoder_Decode(decoder);
        if decoded_bytes == 0 as uint32 {
            break;
        }
        audio_BufferData(
            *(*sample).buffer.offset(i as isize),
            (*decoder).format,
            (*decoder).buffer,
            decoded_bytes,
            (*decoder).frequency,
        );
        audio_SourceQueueBuffers(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
                .handle,
            1 as uint32,
            (*sample).buffer.offset(i as isize),
        );
        if (*sample).callbacks.OnQueueBuffer.is_some() {
            (*sample)
                .callbacks
                .OnQueueBuffer
                .expect(
                    "non-null function pointer",
                )(sample, *(*sample).buffer.offset(i as isize));
        }
        if scope {
            add_scope_data(
                (&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize),
                decoded_bytes,
            );
        }
        if (*decoder).error != C2Rust_Unnamed_0::SOUNDDECODER_OK.0 as ::core::ffi::c_int
        {
            if (*decoder).error
                != C2Rust_Unnamed_0::SOUNDDECODER_EOF.0 as ::core::ffi::c_int
                || (*sample).callbacks.OnEndChunk.is_none()
                || !(*sample)
                    .callbacks
                    .OnEndChunk
                    .expect(
                        "non-null function pointer",
                    )(sample, *(*sample).buffer.offset(i as isize))
            {
                break;
            }
            decoder = (*sample).decoder;
        }
        i = i.wrapping_add(1);
    }
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .sbuf_lasttime = GetTimeCounter() as uint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .start_time = GetTimeCounter().wrapping_sub(offset as TimeCount) as sint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .pause_time = 0 as uint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .stream_should_be_playing = BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0;
    audio_SourcePlay(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize)).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn StopStream(mut source: uint32) {
    StopSource(source as ::core::ffi::c_int);
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .stream_should_be_playing = BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize)).sample = ::core::ptr::null_mut::<
        TFB_SoundSample,
    >();
    if !(*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .sbuffer
        .is_null()
    {
        let mut sbuffer: *mut ::core::ffi::c_void = (*(&raw mut soundSource
            as *mut TFB_SoundSource)
            .offset(source as isize))
            .sbuffer;
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
            .sbuffer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        HFree(sbuffer);
    }
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .sbuf_size = 0 as uint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .sbuf_head = 0 as uint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .sbuf_tail = 0 as uint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .pause_time = 0 as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn PauseStream(mut source: uint32) {
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .stream_should_be_playing = BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0;
    if (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .pause_time == 0
    {
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
            .pause_time = GetTimeCounter() as uint32;
    }
    audio_SourcePause(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize)).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ResumeStream(mut source: uint32) {
    if (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .pause_time != 0
    {
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
            .start_time = ((*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(source as isize))
            .start_time as uint32)
            .wrapping_add(
                GetTimeCounter()
                    .wrapping_sub(
                        (*(&raw mut soundSource as *mut TFB_SoundSource)
                            .offset(source as isize))
                            .pause_time,
                    ),
            ) as sint32;
    }
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .pause_time = 0 as uint32;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .stream_should_be_playing = BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0;
    audio_SourcePlay(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize)).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SeekStream(mut source: uint32, mut pos: uint32) {
    let mut sample: *mut TFB_SoundSample = (*(&raw mut soundSource
        as *mut TFB_SoundSource)
        .offset(source as isize))
        .sample;
    let mut looping: bool = false;
    let mut scope: bool = false;
    if sample.is_null() {
        return;
    }
    looping = (*(*sample).decoder).looping;
    scope = !(*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
        .sbuffer
        .is_null();
    StopSource(source as ::core::ffi::c_int);
    SoundDecoder_Seek((*sample).decoder, pos);
    PlayStream(sample, source, looping, scope, false);
}
#[no_mangle]
pub unsafe extern "C" fn PlayingStream(mut source: uint32) -> BOOLEAN {
    return BOOLEAN(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(source as isize))
            .stream_should_be_playing as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_CreateSoundSample(
    mut decoder: *mut TFB_SoundDecoder,
    mut num_buffers: uint32,
    mut pcbs: *const TFB_SoundCallbacks,
) -> *mut TFB_SoundSample {
    let mut sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<TFB_SoundSample>();
    sample = HCalloc(::core::mem::size_of::<TFB_SoundSample>() as size_t)
        as *mut TFB_SoundSample;
    (*sample).decoder = decoder;
    (*sample).num_buffers = num_buffers;
    (*sample).buffer = HCalloc(
        (::core::mem::size_of::<audio_Object>() as size_t)
            .wrapping_mul(num_buffers as size_t),
    ) as *mut audio_Object;
    audio_GenBuffers(num_buffers, (*sample).buffer);
    if !pcbs.is_null() {
        (*sample).callbacks = *pcbs;
    }
    return sample;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DestroySoundSample(mut sample: *mut TFB_SoundSample) {
    if !(*sample).buffer.is_null() {
        audio_DeleteBuffers((*sample).num_buffers, (*sample).buffer);
        HFree((*sample).buffer as *mut ::core::ffi::c_void);
    }
    HFree((*sample).buffer_tag as *mut ::core::ffi::c_void);
    HFree(sample as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetSoundSampleData(
    mut sample: *mut TFB_SoundSample,
    mut data: *mut ::core::ffi::c_void,
) {
    (*sample).data = data;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetSoundSampleData(
    mut sample: *mut TFB_SoundSample,
) -> *mut ::core::ffi::c_void {
    return (*sample).data;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetSoundSampleCallbacks(
    mut sample: *mut TFB_SoundSample,
    mut pcbs: *const TFB_SoundCallbacks,
) {
    if !pcbs.is_null() {
        (*sample).callbacks = *pcbs;
    } else {
        memset(
            &raw mut (*sample).callbacks as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TFB_SoundCallbacks>() as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetSoundSampleDecoder(
    mut sample: *mut TFB_SoundSample,
) -> *mut TFB_SoundDecoder {
    return (*sample).decoder;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_FindTaggedBuffer(
    mut sample: *mut TFB_SoundSample,
    mut buffer: audio_Object,
) -> *mut TFB_SoundTag {
    let mut buf_num: uint32 = 0;
    if (*sample).buffer_tag.is_null() {
        return ::core::ptr::null_mut::<TFB_SoundTag>();
    }
    buf_num = 0 as uint32;
    while buf_num < (*sample).num_buffers
        && ((*(*sample).buffer_tag.offset(buf_num as isize)).in_use == 0
            || (*(*sample).buffer_tag.offset(buf_num as isize)).buf_name != buffer)
    {
        buf_num = buf_num.wrapping_add(1);
    }
    return if buf_num < (*sample).num_buffers {
        (*sample).buffer_tag.offset(buf_num as isize)
    } else {
        ::core::ptr::null_mut::<TFB_SoundTag>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_TagBuffer(
    mut sample: *mut TFB_SoundSample,
    mut buffer: audio_Object,
    mut data: intptr_t,
) -> bool {
    let mut buf_num: uint32 = 0;
    if (*sample).buffer_tag.is_null() {
        (*sample).buffer_tag = HCalloc(
            (::core::mem::size_of::<TFB_SoundTag>() as size_t)
                .wrapping_mul((*sample).num_buffers as size_t),
        ) as *mut TFB_SoundTag;
    }
    buf_num = 0 as uint32;
    while buf_num < (*sample).num_buffers
        && (*(*sample).buffer_tag.offset(buf_num as isize)).in_use != 0
        && (*(*sample).buffer_tag.offset(buf_num as isize)).buf_name != buffer
    {
        buf_num = buf_num.wrapping_add(1);
    }
    if buf_num >= (*sample).num_buffers {
        return false;
    }
    (*(*sample).buffer_tag.offset(buf_num as isize)).in_use = 1 as ::core::ffi::c_int;
    (*(*sample).buffer_tag.offset(buf_num as isize)).buf_name = buffer;
    (*(*sample).buffer_tag.offset(buf_num as isize)).data = data;
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_ClearBufferTag(mut ptag: *mut TFB_SoundTag) {
    (*ptag).in_use = 0 as ::core::ffi::c_int;
    (*ptag).buf_name = 0 as audio_Object;
}
unsafe extern "C" fn remove_scope_data(
    mut source: *mut TFB_SoundSource,
    mut buffer: audio_Object,
) {
    let mut buf_size: audio_IntVal = 0;
    audio_GetBufferi(
        buffer,
        C2Rust_Unnamed::audio_SIZE.0 as audio_BufferProp,
        &raw mut buf_size,
    );
    (*source).sbuf_head = ((*source).sbuf_head as ::core::ffi::c_ulong)
        .wrapping_add(buf_size as ::core::ffi::c_ulong) as uint32;
    (*source).sbuf_head = (*source).sbuf_head.wrapping_rem((*source).sbuf_size);
    (*source).sbuf_lasttime = GetTimeCounter() as uint32;
}
unsafe extern "C" fn add_scope_data(
    mut source: *mut TFB_SoundSource,
    mut bytes: uint32,
) {
    let mut sbuffer: *mut uint8 = (*source).sbuffer as *mut uint8;
    let mut dec_buf: *mut uint8 = (*(*(*source).sample).decoder).buffer as *mut uint8;
    let mut tail_bytes: uint32 = 0;
    let mut wrap_bytes: uint32 = 0;
    if (*source).sbuf_tail.wrapping_add(bytes) > (*source).sbuf_size {
        tail_bytes = (*source).sbuf_size.wrapping_sub((*source).sbuf_tail);
        wrap_bytes = bytes.wrapping_sub(tail_bytes);
    } else {
        tail_bytes = bytes;
        wrap_bytes = 0 as uint32;
    }
    if tail_bytes != 0 {
        memcpy(
            sbuffer.offset((*source).sbuf_tail as isize) as *mut ::core::ffi::c_void,
            dec_buf as *const ::core::ffi::c_void,
            tail_bytes as size_t,
        );
        (*source).sbuf_tail = (*source).sbuf_tail.wrapping_add(tail_bytes);
    }
    if wrap_bytes != 0 {
        memcpy(
            sbuffer as *mut ::core::ffi::c_void,
            dec_buf.offset(tail_bytes as isize) as *const ::core::ffi::c_void,
            wrap_bytes as size_t,
        );
        (*source).sbuf_tail = wrap_bytes;
    }
}
unsafe extern "C" fn process_stream(mut source: *mut TFB_SoundSource) {
    let mut sample: *mut TFB_SoundSample = (*source).sample;
    let mut decoder: *mut TFB_SoundDecoder = (*sample).decoder;
    let mut end_chunk_failed: bool = false;
    let mut processed: audio_IntVal = 0;
    let mut queued: audio_IntVal = 0;
    audio_GetSourcei(
        (*source).handle,
        C2Rust_Unnamed::audio_BUFFERS_PROCESSED.0 as audio_SourceProp,
        &raw mut processed,
    );
    audio_GetSourcei(
        (*source).handle,
        C2Rust_Unnamed::audio_BUFFERS_QUEUED.0 as audio_SourceProp,
        &raw mut queued,
    );
    if processed == 0 as audio_IntVal {
        let mut state: audio_IntVal = 0;
        audio_GetSourcei(
            (*source).handle,
            C2Rust_Unnamed::audio_SOURCE_STATE.0 as audio_SourceProp,
            &raw mut state,
        );
        if state != C2Rust_Unnamed::audio_PLAYING.0 as ::core::ffi::c_int as audio_IntVal
        {
            if queued == 0 as audio_IntVal
                && (*decoder).error
                    == C2Rust_Unnamed_0::SOUNDDECODER_EOF.0 as ::core::ffi::c_int
            {
                log_add(
                    log_Level::log_Info,
                    b"StreamDecoderTaskFunc(): finished playing %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*decoder).filename,
                );
                (*source).stream_should_be_playing = BOOLEAN::FALSE.0
                    as ::core::ffi::c_int != 0;
                if (*sample).callbacks.OnEndStream.is_some() {
                    (*sample)
                        .callbacks
                        .OnEndStream
                        .expect("non-null function pointer")(sample);
                }
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"StreamDecoderTaskFunc(): buffer underrun playing %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*decoder).filename,
                );
                audio_SourcePlay((*source).handle);
            }
        }
    }
    while processed > 0 as audio_IntVal {
        let mut error: uint32 = 0;
        let mut buffer: audio_Object = 0;
        let mut decoded_bytes: uint32 = 0;
        audio_GetError();
        audio_SourceUnqueueBuffers((*source).handle, 1 as uint32, &raw mut buffer);
        error = audio_GetError() as uint32;
        if error != C2Rust_Unnamed::audio_NO_ERROR.0 as ::core::ffi::c_int as uint32 {
            log_add(
                log_Level::log_Warning,
                b"StreamDecoderTaskFunc(): error after audio_SourceUnqueueBuffers: %x, file %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error,
                (*decoder).filename,
            );
            break;
        } else {
            if (*sample).callbacks.OnTaggedBuffer.is_some() {
                let mut tag: *mut TFB_SoundTag = TFB_FindTaggedBuffer(sample, buffer);
                if !tag.is_null() {
                    (*sample)
                        .callbacks
                        .OnTaggedBuffer
                        .expect("non-null function pointer")(sample, tag);
                }
            }
            if !(*source).sbuffer.is_null() {
                remove_scope_data(source, buffer);
            }
            's_77: {
                if (*decoder).error
                    != C2Rust_Unnamed_0::SOUNDDECODER_OK.0 as ::core::ffi::c_int
                {
                    if (*decoder).error
                        == C2Rust_Unnamed_0::SOUNDDECODER_EOF.0 as ::core::ffi::c_int
                    {
                        if end_chunk_failed {
                            break 's_77;
                        } else if (*sample).callbacks.OnEndChunk.is_none()
                            || !(*sample)
                                .callbacks
                                .OnEndChunk
                                .expect(
                                    "non-null function pointer",
                                )(sample, (*source).last_q_buf)
                        {
                            end_chunk_failed = true;
                            break 's_77;
                        } else {
                            decoder = (*sample).decoder;
                        }
                    } else {
                        break 's_77;
                    }
                }
                decoded_bytes = SoundDecoder_Decode(decoder);
                if (*decoder).error
                    == C2Rust_Unnamed_0::SOUNDDECODER_ERROR.0 as ::core::ffi::c_int
                {
                    log_add(
                        log_Level::log_Warning,
                        b"StreamDecoderTaskFunc(): SoundDecoder_Decode error %d, file %s\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*decoder).error,
                        (*decoder).filename,
                    );
                    (*source).stream_should_be_playing = BOOLEAN::FALSE.0
                        as ::core::ffi::c_int != 0;
                } else if decoded_bytes != 0 as uint32 {
                    audio_BufferData(
                        buffer,
                        (*decoder).format,
                        (*decoder).buffer,
                        decoded_bytes,
                        (*decoder).frequency,
                    );
                    error = audio_GetError() as uint32;
                    if error
                        != C2Rust_Unnamed::audio_NO_ERROR.0 as ::core::ffi::c_int
                            as uint32
                    {
                        log_add(
                            log_Level::log_Warning,
                            b"StreamDecoderTaskFunc(): error after audio_BufferData: %x, file %s, decoded %d\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            error,
                            (*decoder).filename,
                            decoded_bytes,
                        );
                    } else {
                        audio_SourceQueueBuffers(
                            (*source).handle,
                            1 as uint32,
                            &raw mut buffer,
                        );
                        error = audio_GetError() as uint32;
                        if error
                            != C2Rust_Unnamed::audio_NO_ERROR.0 as ::core::ffi::c_int
                                as uint32
                        {
                            log_add(
                                log_Level::log_Warning,
                                b"StreamDecoderTaskFunc(): error after audio_SourceQueueBuffers: %x, file %s, decoded %d\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                error,
                                (*decoder).filename,
                                decoded_bytes,
                            );
                        } else {
                            (*source).last_q_buf = buffer;
                            if (*sample).callbacks.OnQueueBuffer.is_some() {
                                (*sample)
                                    .callbacks
                                    .OnQueueBuffer
                                    .expect("non-null function pointer")(sample, buffer);
                            }
                            if !(*source).sbuffer.is_null() {
                                add_scope_data(source, decoded_bytes);
                            }
                        }
                    }
                }
            }
            processed -= 1;
        }
    }
}
unsafe extern "C" fn processMusicFade() {
    let mut Now: TimeCount = 0;
    let mut elapsed: sint32 = 0;
    let mut newVolume: ::core::ffi::c_int = 0;
    LockMutex(fade_mutex);
    if musicFadeInterval == 0 {
        UnlockMutex(fade_mutex);
        return;
    }
    Now = GetTimeCounter();
    elapsed = Now.wrapping_sub(musicFadeStartTime) as sint32;
    if elapsed > musicFadeInterval {
        elapsed = musicFadeInterval;
    }
    newVolume = (musicFadeStartVolume as ::core::ffi::c_long
        + musicFadeDelta as ::core::ffi::c_long * elapsed as ::core::ffi::c_long
            / musicFadeInterval as ::core::ffi::c_long) as ::core::ffi::c_int;
    SetMusicVolume(newVolume as COUNT);
    if elapsed >= musicFadeInterval {
        musicFadeInterval = 0 as ::core::ffi::c_int as sint32;
    }
    UnlockMutex(fade_mutex);
}
unsafe extern "C" fn StreamDecoderTaskFunc(
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut task: Task = data as Task;
    let mut active_streams: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    while Task_ReadState(task, 2 as DWORD) == 0 {
        active_streams = 0 as ::core::ffi::c_int;
        processMusicFade();
        i = 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        while i
            < 0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            let mut source: *mut TFB_SoundSource = (&raw mut soundSource
                as *mut TFB_SoundSource)
                .offset(i as isize);
            LockMutex((*source).stream_mutex);
            if (*source).sample.is_null() || (*(*source).sample).decoder.is_null()
                || !(*source).stream_should_be_playing
                || (*(*(*source).sample).decoder).error
                    == C2Rust_Unnamed_0::SOUNDDECODER_ERROR.0 as ::core::ffi::c_int
            {
                UnlockMutex((*source).stream_mutex);
            } else {
                process_stream(source);
                active_streams += 1;
                UnlockMutex((*source).stream_mutex);
            }
            i += 1;
        }
        if active_streams == 0 as ::core::ffi::c_int {
            HibernateThread(
                (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as TimePeriod,
            );
        } else {
            TaskSwitch();
        }
    }
    FinishTask(task);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn readSoundSample(
    mut ptr: *mut ::core::ffi::c_void,
    mut sample_size: ::core::ffi::c_int,
) -> sint32 {
    if sample_size as usize == ::core::mem::size_of::<uint8>() {
        return (*(ptr as *mut uint8) as sint32 - 128 as sint32)
            << 8 as ::core::ffi::c_int
    } else {
        return *(ptr as *mut sint16) as sint32
    };
}
#[no_mangle]
pub unsafe extern "C" fn GraphForegroundStream(
    mut data: *mut uint8,
    mut width: sint32,
    mut height: sint32,
    mut wantSpeech: bool,
) -> ::core::ffi::c_int {
    let mut source_num: ::core::ffi::c_int = 0;
    let mut source: *mut TFB_SoundSource = ::core::ptr::null_mut::<TFB_SoundSource>();
    let mut decoder: *mut TFB_SoundDecoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
    let mut channels: ::core::ffi::c_int = 0;
    let mut sample_size: ::core::ffi::c_int = 0;
    let mut full_sample: ::core::ffi::c_int = 0;
    let mut step: ::core::ffi::c_int = 0;
    let mut played_time: ::core::ffi::c_long = 0;
    let mut delta: ::core::ffi::c_long = 0;
    let mut sbuffer: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut pos: ::core::ffi::c_ulong = 0;
    let mut scale: ::core::ffi::c_int = 0;
    let mut i: sint32 = 0;
    static mut page_sum: ::core::ffi::c_int = 28000 as ::core::ffi::c_int
        * 16 as ::core::ffi::c_int;
    static mut pages: [::core::ffi::c_int; 16] = [
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
        28000 as ::core::ffi::c_int,
    ];
    static mut page_head: ::core::ffi::c_int = 0;
    static mut frame_sum: ::core::ffi::c_int = 0;
    static mut frames: ::core::ffi::c_int = 0;
    static mut avg_amp: ::core::ffi::c_int = 28000 as ::core::ffi::c_int;
    let mut target_amp: ::core::ffi::c_int = 0;
    let mut max_a: ::core::ffi::c_int = 0;
    let mut energy: ::core::ffi::c_long = 0;
    source_num = 0 as ::core::ffi::c_int
        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    source = (&raw mut soundSource as *mut TFB_SoundSource).offset(source_num as isize);
    LockMutex((*source).stream_mutex);
    if wantSpeech as ::core::ffi::c_int != 0
        && ((*source).sample.is_null() || (*(*source).sample).decoder.is_null()
            || !(*(*(*source).sample).decoder).is_null)
    {
        step = 1 as ::core::ffi::c_int;
    } else {
        UnlockMutex((*source).stream_mutex);
        source_num = 0 as ::core::ffi::c_int
            + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        source = (&raw mut soundSource as *mut TFB_SoundSource)
            .offset(source_num as isize);
        LockMutex((*source).stream_mutex);
        step = 4 as ::core::ffi::c_int;
    }
    if PlayingStream(source_num as uint32).0 == 0 || (*source).sample.is_null()
        || (*(*source).sample).decoder.is_null() || (*source).sbuffer.is_null()
        || (*source).sbuf_size == 0 as uint32
    {
        UnlockMutex((*source).stream_mutex);
        return 0 as ::core::ffi::c_int;
    }
    decoder = (*(*source).sample).decoder;
    if !audio_GetFormatInfo((*decoder).format, &raw mut channels, &raw mut sample_size) {
        UnlockMutex((*source).stream_mutex);
        log_add(
            log_Level::log_Debug,
            b"GraphForegroundStream(): uknown format %u\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*decoder).format,
        );
        return 0 as ::core::ffi::c_int;
    }
    full_sample = channels * sample_size;
    played_time = GetTimeCounter().wrapping_sub((*source).sbuf_lasttime)
        as ::core::ffi::c_long;
    delta = (played_time as ::core::ffi::c_ulong)
        .wrapping_mul((*decoder).frequency as ::core::ffi::c_ulong)
        .wrapping_mul(full_sample as ::core::ffi::c_ulong)
        .wrapping_div(840 as ::core::ffi::c_ulong) as ::core::ffi::c_long;
    delta = delta & !(full_sample - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if delta < 0 as ::core::ffi::c_long {
        log_add(
            log_Level::log_Debug,
            b"GraphForegroundStream(): something is messed with timing, delta %ld\0"
                .as_ptr() as *const ::core::ffi::c_char,
            delta,
        );
        delta = 0 as ::core::ffi::c_long;
    } else if delta > (*source).sbuf_size as ::core::ffi::c_long {
        delta = 0 as ::core::ffi::c_long;
    }
    step = (*decoder)
        .frequency
        .wrapping_mul(step as uint32)
        .wrapping_div(11025 as uint32) as ::core::ffi::c_int;
    if step == 0 as ::core::ffi::c_int {
        step = 1 as ::core::ffi::c_int;
    }
    step *= full_sample;
    sbuffer = (*source).sbuffer as *mut uint8;
    pos = ((*source).sbuf_head as ::core::ffi::c_ulong)
        .wrapping_add(delta as ::core::ffi::c_ulong);
    target_amp = (height >> 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    scale = avg_amp / target_amp;
    max_a = 0 as ::core::ffi::c_int;
    energy = 0 as ::core::ffi::c_long;
    i = 0 as ::core::ffi::c_int as sint32;
    while i < width {
        let mut s: sint32 = 0;
        let mut t: ::core::ffi::c_int = 0;
        pos = pos.wrapping_rem((*source).sbuf_size as ::core::ffi::c_ulong);
        s = readSoundSample(
            sbuffer.offset(pos as isize) as *mut ::core::ffi::c_void,
            sample_size,
        );
        if channels > 1 as ::core::ffi::c_int {
            s
                += readSoundSample(
                    sbuffer.offset(pos as isize).offset(sample_size as isize)
                        as *mut ::core::ffi::c_void,
                    sample_size,
                );
        }
        energy += (s * s / 0x10000 as ::core::ffi::c_int) as ::core::ffi::c_long;
        t = abs(s);
        if t > max_a {
            max_a = t;
        }
        s = s / scale + (height >> 1 as ::core::ffi::c_int);
        if s < 0 as ::core::ffi::c_int {
            s = 0 as ::core::ffi::c_int as sint32;
        } else if s > height - 1 as ::core::ffi::c_int {
            s = (height - 1 as ::core::ffi::c_int) as sint32;
        }
        *data.offset(i as isize) = s as uint8;
        i += 1;
        pos = pos.wrapping_add(step as ::core::ffi::c_ulong);
    }
    energy /= width as ::core::ffi::c_long;
    if energy > 100 as ::core::ffi::c_long {
        frame_sum += max_a;
        frames += 1;
        if frames == 8 as ::core::ffi::c_int {
            frame_sum /= 8 as ::core::ffi::c_int;
            page_sum -= pages[page_head as usize];
            page_sum += frame_sum;
            pages[page_head as usize] = frame_sum;
            page_head = (page_head + 1 as ::core::ffi::c_int) % 16 as ::core::ffi::c_int;
            frame_sum = 0 as ::core::ffi::c_int;
            frames = 0 as ::core::ffi::c_int;
            avg_amp = page_sum / 16 as ::core::ffi::c_int;
        }
    }
    UnlockMutex((*source).stream_mutex);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetMusicStreamFade(
    mut howLong: sint32,
    mut endVolume: ::core::ffi::c_int,
) -> bool {
    let mut ret: bool = true;
    LockMutex(fade_mutex);
    if howLong < 0 as ::core::ffi::c_int {
        howLong = 0 as ::core::ffi::c_int as sint32;
    }
    musicFadeStartTime = GetTimeCounter();
    musicFadeInterval = howLong;
    musicFadeStartVolume = musicVolume;
    musicFadeDelta = endVolume - musicFadeStartVolume;
    if musicFadeInterval == 0 {
        ret = false;
    }
    UnlockMutex(fade_mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn InitStreamDecoder() -> ::core::ffi::c_int {
    fade_mutex = CreateMutex_Core(
        b"Stream fade mutex\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_1::SYNC_CLASS_AUDIO.0 as ::core::ffi::c_int as DWORD,
    );
    if fade_mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    decoderTask = AssignTask(
        Some(
            StreamDecoderTaskFunc
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        1024 as SDWORD,
        b"audio stream decoder\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if decoderTask.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UninitStreamDecoder() {
    if !decoderTask.is_null() {
        ConcludeTask(decoderTask);
        decoderTask = ::core::ptr::null_mut::<taskstruct>();
    }
    if !fade_mutex.is_null() {
        DestroyMutex(fade_mutex);
        fade_mutex = ::core::ptr::null_mut::<::core::ffi::c_void>() as Mutex;
    }
}
