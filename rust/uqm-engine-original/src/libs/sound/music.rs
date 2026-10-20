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

// Rust translation of src/libs/sound/music.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn fileExists2(
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> bool;
    static mut contentDir: *mut uio_DirHandle;
    fn audio_Sourcef(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: ::core::ffi::c_float,
    );
    fn SoundDecoder_Load(
        dir: *mut uio_DirHandle,
        filename: *mut ::core::ffi::c_char,
        buffer_size: uint32,
        startTime: uint32,
        runTime: sint32,
    ) -> *mut TFB_SoundDecoder;
    fn SoundDecoder_Free(decoder: *mut TFB_SoundDecoder);
    fn SoundDecoder_GetName(
        decoder: *mut TFB_SoundDecoder,
    ) -> *const ::core::ffi::c_char;
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    static mut soundSource: [TFB_SoundSource; 0];
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
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
    fn SeekStream(source: uint32, pos: uint32);
    fn PlayingStream(source: uint32) -> BOOLEAN;
    static mut musicVolume: ::core::ffi::c_int;
    static mut musicVolumeScale: ::core::ffi::c_float;
    fn TFB_CreateSoundSample(
        _: *mut TFB_SoundDecoder,
        num_buffers: uint32,
        _: *const TFB_SoundCallbacks,
    ) -> *mut TFB_SoundSample;
    fn TFB_DestroySoundSample(_: *mut TFB_SoundSample);
}
pub type size_t = ::core::ffi::c_ulong;
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type audio_Object = uintptr_t;
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
static mut curMusicRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
static mut curSpeechRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
#[no_mangle]
pub unsafe extern "C" fn PLRPlaySong(
    mut MusicRef: MUSIC_REF,
    mut Continuous: BOOLEAN,
    mut Priority: BYTE,
) {
    let mut pmus: *mut *mut TFB_SoundSample = MusicRef;
    if !pmus.is_null() {
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        PlayStream(
            *pmus,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            Continuous.0 != 0,
            true,
            true,
        );
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        curMusicRef = MusicRef;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PLRStop(mut MusicRef: MUSIC_REF) {
    if MusicRef == curMusicRef
        || MusicRef
            == ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize)
    {
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        StopStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        );
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        curMusicRef = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn PLRPlaying(mut MusicRef: MUSIC_REF) -> BOOLEAN {
    if !curMusicRef.is_null()
        && (MusicRef == curMusicRef
            || MusicRef
                == ::core::ptr::with_exposed_provenance_mut::<
                    *mut TFB_SoundSample,
                >(!(0 as ::core::ffi::c_int) as usize))
    {
        let mut playing: BOOLEAN = BOOLEAN::FALSE;
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        playing = PlayingStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        );
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        return playing;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn PLRSeek(mut MusicRef: MUSIC_REF, mut pos: DWORD) {
    if MusicRef == curMusicRef
        || MusicRef
            == ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize)
    {
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        SeekStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            pos,
        );
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn PLRPause(mut MusicRef: MUSIC_REF) {
    if MusicRef == curMusicRef
        || MusicRef
            == ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize)
    {
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        PauseStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        );
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn PLRResume(mut MusicRef: MUSIC_REF) {
    if MusicRef == curMusicRef
        || MusicRef
            == ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize)
    {
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        ResumeStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        );
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_PlaySpeech(mut SpeechRef: MUSIC_REF) {
    let mut pmus: *mut *mut TFB_SoundSample = SpeechRef;
    if !pmus.is_null() {
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
        PlayStream(
            *pmus,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
            false,
            false,
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
        curSpeechRef = SpeechRef;
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_StopSpeech() {
    if curSpeechRef.is_null() {
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
    StopStream(
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
    curSpeechRef = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
}
#[no_mangle]
pub unsafe extern "C" fn DestroyMusic(mut MusicRef: MUSIC_REF) -> BOOLEAN {
    return _ReleaseMusicData(MusicRef as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SetMusicVolume(mut Volume: COUNT) {
    let mut f: ::core::ffi::c_float = Volume as ::core::ffi::c_int
        as ::core::ffi::c_float / 255 as ::core::ffi::c_int as ::core::ffi::c_float
        * musicVolumeScale;
    musicVolume = Volume as ::core::ffi::c_int;
    audio_Sourcef(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .handle,
        C2Rust_Unnamed::audio_GAIN.0 as audio_SourceProp,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CheckMusicResName(
    mut fileName: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !fileExists2(contentDir, fileName) {
        log_add(
            log_Level::log_Warning,
            b"Requested track '%s' not found.\0".as_ptr() as *const ::core::ffi::c_char,
            fileName,
        );
    }
    return fileName;
}
#[no_mangle]
pub unsafe extern "C" fn _GetMusicData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut h: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    let mut sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<TFB_SoundSample>();
    let mut decoder: *mut TFB_SoundDecoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
    let mut filename: [::core::ffi::c_char; 256] = [0; 256];
    if _cur_resfile_name.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    strncpy(
        &raw mut filename as *mut ::core::ffi::c_char,
        _cur_resfile_name,
        (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t)
            .wrapping_sub(1 as size_t),
    );
    filename[::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        .wrapping_sub(1usize)] = '\0' as ::core::ffi::c_char;
    CheckMusicResName(&raw mut filename as *mut ::core::ffi::c_char);
    log_add(
        log_Level::log_Info,
        b"_GetMusicData(): loading %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut filename as *mut ::core::ffi::c_char,
    );
    decoder = SoundDecoder_Load(
        contentDir,
        &raw mut filename as *mut ::core::ffi::c_char,
        4096 as uint32,
        0 as uint32,
        0 as sint32,
    );
    if decoder.is_null() {
        log_add(
            log_Level::log_Warning,
            b"_GetMusicData(): couldn't load %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    h = HMalloc(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
        as MUSIC_REF;
    if h.is_null() {
        SoundDecoder_Free(decoder);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    sample = TFB_CreateSoundSample(
        decoder,
        64 as uint32,
        ::core::ptr::null::<TFB_SoundCallbacks>(),
    );
    *h = sample;
    log_add(
        log_Level::log_Info,
        b"    decoder: %s, rate %d format %x\0".as_ptr() as *const ::core::ffi::c_char,
        SoundDecoder_GetName((*sample).decoder),
        (*(*sample).decoder).frequency,
        (*(*sample).decoder).format,
    );
    return h as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _ReleaseMusicData(
    mut data: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    let mut pmus: *mut *mut TFB_SoundSample = data as *mut *mut TFB_SoundSample;
    let mut sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<TFB_SoundSample>();
    if pmus.is_null() {
        return BOOLEAN::FALSE;
    }
    sample = *pmus;
    (!sample.is_null()
        || {
            __assert_fail(
                b"sample != 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/sound/music.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                214 as ::core::ffi::c_int,
                b"_ReleaseMusicData\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !(*sample).decoder.is_null() {
        let mut decoder: *mut TFB_SoundDecoder = (*sample).decoder;
        LockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        if (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .sample == sample
        {
            StopStream(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            );
        }
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        (*sample).decoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
        SoundDecoder_Free(decoder);
    }
    TFB_DestroySoundSample(sample);
    HFree(data);
    return BOOLEAN::TRUE;
}
