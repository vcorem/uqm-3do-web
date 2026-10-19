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

// Rust translation of src/libs/sound/sfx.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn uio_fgets(
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        stream: *mut uio_Stream,
    ) -> *mut ::core::ffi::c_char;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn audio_GetError() -> sint32;
    fn audio_Sourcei(srcobj: audio_Object, pname: audio_SourceProp, value: audio_IntVal);
    fn audio_Sourcef(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: ::core::ffi::c_float,
    );
    fn audio_Sourcefv(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: *mut ::core::ffi::c_float,
    );
    fn audio_GetSourcei(
        srcobj: audio_Object,
        pname: audio_SourceProp,
        value: *mut audio_IntVal,
    );
    fn audio_SourcePlay(srcobj: audio_Object);
    fn audio_BufferData(
        bufobj: audio_Object,
        format: uint32,
        data: *mut ::core::ffi::c_void,
        size: uint32,
        freq: uint32,
    );
    fn SoundDecoder_Load(
        dir: *mut uio_DirHandle,
        filename: *mut ::core::ffi::c_char,
        buffer_size: uint32,
        startTime: uint32,
        runTime: sint32,
    ) -> *mut TFB_SoundDecoder;
    fn SoundDecoder_DecodeAll(decoder: *mut TFB_SoundDecoder) -> uint32;
    fn SoundDecoder_Free(decoder: *mut TFB_SoundDecoder);
    static mut optStereoSFX: BOOLEAN;
    static mut contentDir: *mut uio_DirHandle;
    fn AllocStringTable(
        num_entries: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> STRING_TABLE;
    fn FreeStringTable(strtab: STRING_TABLE);
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    static mut soundSource: [TFB_SoundSource; 0];
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut sfxVolumeScale: ::core::ffi::c_float;
    fn StopSource(iSource: ::core::ffi::c_int);
    fn CleanSource(iSource: ::core::ffi::c_int);
    fn TFB_CreateSoundSample(
        _: *mut TFB_SoundDecoder,
        num_buffers: uint32,
        _: *const TFB_SoundCallbacks,
    ) -> *mut TFB_SoundSample;
    fn TFB_DestroySoundSample(_: *mut TFB_SoundSample);
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
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
pub type uio_bool = bool;
pub type uio_uint32 = ::core::ffi::c_uint;
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
pub struct string_table_entry {
    pub data: STRINGPTR,
    pub length: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub parent: *mut string_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_table {
    pub flags: ::core::ffi::c_ushort,
    pub size: ::core::ffi::c_int,
    pub strings: *mut STRING_TABLE_ENTRY_DESC,
    pub nameIndex: *mut StringHashTable_HashTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringHashTable_HashTable {
    pub hashFunction: StringHashTable_HashFunction,
    pub equalFunction: StringHashTable_EqualFunction,
    pub copyFunction: StringHashTable_CopyFunction,
    pub freeKeyFunction: StringHashTable_FreeKeyFunction,
    pub freeValueFunction: StringHashTable_FreeValueFunction,
    pub minFillQuotient: ::core::ffi::c_double,
    pub maxFillQuotient: ::core::ffi::c_double,
    pub minSize: uio_uint32,
    pub maxSize: uio_uint32,
    pub size: uio_uint32,
    pub hashMask: uio_uint32,
    pub entries: *mut *mut StringHashTable_HashEntry,
    pub numEntries: uio_uint32,
    pub numCollisions: uio_uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringHashTable_HashEntry {
    pub hash: uio_uint32,
    pub key: *mut StringHashTable_Key,
    pub value: *mut StringHashTable_Value,
    pub next: *mut StringHashTable_HashEntry,
}
pub type StringHashTable_Value = STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type StringHashTable_Key = ::core::ffi::c_char;
pub type StringHashTable_FreeValueFunction = Option<
    unsafe extern "C" fn(*mut StringHashTable_Value) -> (),
>;
pub type StringHashTable_FreeKeyFunction = Option<
    unsafe extern "C" fn(*mut StringHashTable_Key) -> (),
>;
pub type StringHashTable_CopyFunction = Option<
    unsafe extern "C" fn(*const StringHashTable_Key) -> *mut StringHashTable_Value,
>;
pub type StringHashTable_EqualFunction = Option<
    unsafe extern "C" fn(
        *const StringHashTable_Key,
        *const StringHashTable_Key,
    ) -> uio_bool,
>;
pub type StringHashTable_HashFunction = Option<
    unsafe extern "C" fn(*const StringHashTable_Key) -> uio_uint32,
>;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type SOUND_REF = STRING_TABLE;
pub type SOUND = STRING;
pub type SOUNDPTR = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SoundPosition = soundposition;
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
static mut notPositional: SoundPosition = soundposition {
    positional: BOOLEAN::FALSE,
    x: 0 as ::core::ffi::c_int,
    y: 0 as ::core::ffi::c_int,
};
#[no_mangle]
pub unsafe extern "C" fn PlayChannel(
    mut channel: COUNT,
    mut snd: SOUND,
    mut pos: SoundPosition,
    mut positional_object: *mut ::core::ffi::c_void,
    mut priority: ::core::ffi::c_uchar,
) {
    let mut snd_ptr: SOUNDPTR = GetSoundAddress(snd);
    let mut sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<TFB_SoundSample>();
    StopSource(channel as ::core::ffi::c_int);
    CheckFinishedChannels();
    if snd_ptr.is_null() {
        return;
    }
    sample = *(snd_ptr as *mut *mut TFB_SoundSample);
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize)).sample = sample;
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
        .positional_object = positional_object;
    UpdateSoundPosition(channel, if optStereoSFX.0 != 0 { pos } else { notPositional });
    audio_Sourcei(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
            .handle,
        C2Rust_Unnamed::audio_BUFFER.0 as audio_SourceProp,
        *(*sample).buffer.offset(0isize) as audio_IntVal,
    );
    audio_SourcePlay(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize)).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn StopChannel(
    mut channel: COUNT,
    mut Priority: ::core::ffi::c_uchar,
) {
    StopSource(channel as ::core::ffi::c_int);
}
unsafe extern "C" fn CheckFinishedChannels() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        <= 0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int
    {
        let mut state: audio_IntVal = 0;
        audio_GetSourcei(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize)).handle,
            C2Rust_Unnamed::audio_SOURCE_STATE.0 as audio_SourceProp,
            &raw mut state,
        );
        if state == C2Rust_Unnamed::audio_STOPPED.0 as ::core::ffi::c_int as audio_IntVal
        {
            CleanSource(i);
            audio_GetError();
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ChannelPlaying(mut WhichChannel: COUNT) -> BOOLEAN {
    let mut state: audio_IntVal = 0;
    audio_GetSourcei(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(WhichChannel as isize))
            .handle,
        C2Rust_Unnamed::audio_SOURCE_STATE.0 as audio_SourceProp,
        &raw mut state,
    );
    if state == C2Rust_Unnamed::audio_PLAYING.0 as ::core::ffi::c_int as audio_IntVal {
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn GetPositionalObject(
    mut channel: COUNT,
) -> *mut ::core::ffi::c_void {
    return (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
        .positional_object;
}
#[no_mangle]
pub unsafe extern "C" fn SetPositionalObject(
    mut channel: COUNT,
    mut positional_object: *mut ::core::ffi::c_void,
) {
    (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
        .positional_object = positional_object;
}
#[no_mangle]
pub unsafe extern "C" fn UpdateSoundPosition(
    mut channel: COUNT,
    mut pos: SoundPosition,
) {
    let ATTENUATION: ::core::ffi::c_float = 160.0f32;
    let MIN_DISTANCE: ::core::ffi::c_float = 0.5f32;
    let mut fpos: [::core::ffi::c_float; 3] = [0.; 3];
    if pos.positional.0 != 0 {
        let mut dist: ::core::ffi::c_float = 0.;
        fpos[0usize] = pos.x as ::core::ffi::c_float / ATTENUATION;
        fpos[1usize] = 0.0f32;
        fpos[2usize] = pos.y as ::core::ffi::c_float / ATTENUATION;
        dist = sqrt(
            (fpos[0usize] * fpos[0usize] + fpos[2usize] * fpos[2usize])
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        if dist < MIN_DISTANCE {
            let mut scale: ::core::ffi::c_float = MIN_DISTANCE / dist;
            fpos[0usize] *= scale;
            fpos[2usize] *= scale;
        }
        audio_Sourcefv(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
                .handle,
            C2Rust_Unnamed::audio_POSITION.0 as audio_SourceProp,
            &raw mut fpos as *mut ::core::ffi::c_float,
        );
    } else {
        fpos[1usize] = 0.0f32;
        fpos[0usize] = fpos[1usize];
        fpos[2usize] = -1.0f32;
        audio_Sourcefv(
            (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
                .handle,
            C2Rust_Unnamed::audio_POSITION.0 as audio_SourceProp,
            &raw mut fpos as *mut ::core::ffi::c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetChannelVolume(
    mut channel: COUNT,
    mut volume: COUNT,
    mut priority: BYTE,
) {
    audio_Sourcef(
        (*(&raw mut soundSource as *mut TFB_SoundSource).offset(channel as isize))
            .handle,
        C2Rust_Unnamed::audio_GAIN.0 as audio_SourceProp,
        volume as ::core::ffi::c_int as ::core::ffi::c_float
            / 255 as ::core::ffi::c_int as ::core::ffi::c_float * sfxVolumeScale,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _GetSoundBankData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut snd_ct: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut opos: DWORD = 0;
    let mut CurrentLine: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut filename: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut sndfx: [*mut TFB_SoundSample; 256] = [::core::ptr::null_mut::<
        TFB_SoundSample,
    >(); 256];
    let mut Snd: STRING_TABLE = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    let mut str: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    let mut i: ::core::ffi::c_int = 0;
    opos = uio_ftell(fp) as DWORD;
    let mut s1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut s2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if _cur_resfile_name.is_null()
        || {
            s2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            s1 = strrchr(_cur_resfile_name, '/' as ::core::ffi::c_int);
            s1.is_null() as ::core::ffi::c_int != 0
                && {
                    s2 = strrchr(_cur_resfile_name, '\\' as ::core::ffi::c_int);
                    s2.is_null()
                }
        }
    {
        n = 0 as ::core::ffi::c_int;
    } else {
        if s2 > s1 {
            s1 = s2;
        }
        n = (s1.offset_from(_cur_resfile_name) + 1isize) as ::core::ffi::c_int;
        strncpy(
            &raw mut filename as *mut ::core::ffi::c_char,
            _cur_resfile_name,
            n as size_t,
        );
    }
    snd_ct = 0 as ::core::ffi::c_int;
    while !uio_fgets(
            &raw mut CurrentLine as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null() && snd_ct < 256 as ::core::ffi::c_int
    {
        let mut sample: *mut TFB_SoundSample = ::core::ptr::null_mut::<
            TFB_SoundSample,
        >();
        let mut decoder: *mut TFB_SoundDecoder = ::core::ptr::null_mut::<
            TFB_SoundDecoder,
        >();
        let mut decoded_bytes: uint32 = 0;
        if sscanf(
            &raw mut CurrentLine as *mut ::core::ffi::c_char,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (&raw mut filename as *mut ::core::ffi::c_char).offset(n as isize),
        ) != 1 as ::core::ffi::c_int
        {
            log_add(
                log_Level::log_Warning,
                b"_GetSoundBankData: bad line: '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut CurrentLine as *mut ::core::ffi::c_char,
            );
        } else {
            log_add(
                log_Level::log_Info,
                b"_GetSoundBankData(): loading %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
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
                    b"_GetSoundBankData(): couldn't load %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut filename as *mut ::core::ffi::c_char,
                );
            } else {
                sample = TFB_CreateSoundSample(
                    ::core::ptr::null_mut::<TFB_SoundDecoder>(),
                    1 as uint32,
                    ::core::ptr::null::<TFB_SoundCallbacks>(),
                );
                decoded_bytes = SoundDecoder_DecodeAll(decoder);
                log_add(
                    log_Level::log_Info,
                    b"_GetSoundBankData(): decoded bytes %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    decoded_bytes,
                );
                audio_BufferData(
                    *(*sample).buffer.offset(0isize),
                    (*decoder).format,
                    (*decoder).buffer,
                    decoded_bytes,
                    (*decoder).frequency,
                );
                (*sample).length = (*decoder).length;
                SoundDecoder_Free(decoder);
                sndfx[snd_ct as usize] = sample;
                snd_ct += 1;
            }
        }
    }
    if snd_ct == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    Snd = AllocStringTable(snd_ct, 0 as ::core::ffi::c_int);
    if Snd.is_null() {
        loop {
            let c2rust_fresh0 = snd_ct;
            snd_ct -= 1;
            if c2rust_fresh0 == 0 {
                break;
            }
            TFB_DestroySoundSample(sndfx[snd_ct as usize]);
        }
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    i = 0 as ::core::ffi::c_int;
    str = (*Snd).strings as STRING;
    while i < snd_ct {
        let mut target: *mut *mut TFB_SoundSample = HMalloc(
            ::core::mem::size_of::<*mut TFB_SoundSample>(),
        ) as *mut *mut TFB_SoundSample;
        *target = sndfx[i as usize];
        (*str).data = target as STRINGPTR;
        (*str).length = ::core::mem::size_of::<*mut TFB_SoundSample>()
            as ::core::ffi::c_int;
        i += 1;
        str = str.offset(1);
    }
    return Snd as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _ReleaseSoundBankData(
    mut Snd: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    let mut fxTab: STRING_TABLE = Snd as STRING_TABLE;
    let mut index: ::core::ffi::c_int = 0;
    if fxTab.is_null() {
        return BOOLEAN::FALSE;
    }
    index = 0 as ::core::ffi::c_int;
    while index < (*fxTab).size {
        let mut i: ::core::ffi::c_int = 0;
        let mut sptr: *mut *mut ::core::ffi::c_void = (*(*fxTab)
            .strings
            .offset(index as isize))
            .data as *mut *mut ::core::ffi::c_void;
        let mut sample: *mut TFB_SoundSample = *sptr as *mut TFB_SoundSample;
        i = 0 as ::core::ffi::c_int;
        while i
            < 0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            if (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                .sample == sample
            {
                StopSource(i);
                (*(&raw mut soundSource as *mut TFB_SoundSource).offset(i as isize))
                    .sample = ::core::ptr::null_mut::<TFB_SoundSample>();
            }
            i += 1;
        }
        if !(*sample).decoder.is_null() {
            SoundDecoder_Free((*sample).decoder);
        }
        (*sample).decoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
        TFB_DestroySoundSample(sample);
        index += 1;
    }
    FreeStringTable(fxTab);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn DestroySound(mut target: SOUND_REF) -> BOOLEAN {
    return _ReleaseSoundBankData(target as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn GetSoundAddress(mut sound: SOUND) -> SOUNDPTR {
    return GetStringAddress(sound) as SOUNDPTR;
}
