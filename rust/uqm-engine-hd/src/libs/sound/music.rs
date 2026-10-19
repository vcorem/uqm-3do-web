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
    pub type frame_desc;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    ) -> !;
    fn fileExists2(
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> bool;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    fn GetTimeCounter() -> TimeCount;
    static mut optMusicResume: ::core::ffi::c_int;
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
    static mut soundSource: [TFB_SoundSource; 0];
    static mut musicVolume: ::core::ffi::c_int;
    static mut musicVolumeScale: ::core::ffi::c_float;
    fn TFB_CreateSoundSample(
        _: *mut TFB_SoundDecoder,
        num_buffers: uint32,
        _: *const TFB_SoundCallbacks,
    ) -> *mut TFB_SoundSample;
    fn TFB_DestroySoundSample(_: *mut TFB_SoundSample);
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
    fn GetStreamFrame(source: uint32) -> uint32;
    fn GetNumTrackerPos(source: uint32) -> uint16;
    fn IsTracker(source: uint32) -> BOOLEAN;
    fn GetStreamLength(source: uint32) -> ::core::ffi::c_float;
    fn GetStreamTime(source: uint32) -> DWORD;
    fn PlayingStream(source: uint32) -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut GlobData: GLOBDATA;
}
pub type size_t = ::core::ffi::c_ulong;
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type LINE = line;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
pub type TEXT = text;
pub type GRAPHICS_PRIM = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type PRIM_LINKS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
    pub flags: BYTE,
}
pub type TimeCount = DWORD;
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
pub type ACTIVITY = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAME_STATE {
    pub glob_flags: BYTE,
    pub CrewCost: BYTE,
    pub FuelCost: BYTE,
    pub ModuleCost: [BYTE; 20],
    pub ElementWorth: [BYTE; 8],
    pub DisplayArray: *mut PRIMITIVE,
    pub CurrentActivity: ACTIVITY,
    pub GameClock: CLOCK_STATE,
    pub autopilot: POINT,
    pub ip_location: POINT,
    pub ShipStamp: STAMP,
    pub ShipFacing: UWORD,
    pub ip_planet: BYTE,
    pub in_orbit: BYTE,
    pub velocity: VELOCITY_DESC,
    pub BattleGroupRef: DWORD,
    pub avail_race_q: QUEUE,
    pub npc_built_ship_q: QUEUE,
    pub ip_group_q: QUEUE,
    pub encounter_q: QUEUE,
    pub built_ship_q: QUEUE,
    pub stowed_ship_q: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIS_STATE {
    pub log_x: SDWORD,
    pub log_y: SDWORD,
    pub ResUnits: DWORD,
    pub FuelOnBoard: DWORD,
    pub CrewEnlisted: COUNT,
    pub TotalElementMass: COUNT,
    pub TotalBioMass: COUNT,
    pub ModuleSlots: [BYTE; 16],
    pub DriveSlots: [BYTE; 11],
    pub JetSlots: [BYTE; 8],
    pub NumLanders: BYTE,
    pub ElementAmounts: [COUNT; 8],
    pub ShipName: [UNICODE; 32],
    pub CommanderName: [UNICODE; 32],
    pub PlanetName: [UNICODE; 32],
    pub Difficulty: BYTE,
    pub Extended: BYTE,
    pub Nomad: BYTE,
    pub ShipSeed: BYTE,
    pub Seed: SDWORD,
    pub SaveVersion: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct music_position {
    pub filename_hash: DWORD,
    pub position: DWORD,
    pub last_played: DWORD,
}
pub type MUSIC_POSITION = music_position;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SUPER_MELEE: Self = Self(0);
    pub const IN_LAST_BATTLE: Self = Self(1);
    pub const IN_ENCOUNTER: Self = Self(2);
    pub const IN_HYPERSPACE: Self = Self(3);
    pub const IN_INTERPLANETARY: Self = Self(4);
    pub const WON_LAST_BATTLE: Self = Self(5);
    pub const IN_QUASISPACE: Self = Self(6);
    pub const IN_PLANET_ORBIT: Self = Self(7);
    pub const IN_STARBASE: Self = Self(8);
    pub const CHECK_PAUSE: Self = Self(256);
    pub const IN_BATTLE: Self = Self(512);
    pub const START_ENCOUNTER: Self = Self(1024);
    pub const START_INTERPLANETARY: Self = Self(2048);
    pub const CHECK_LOAD: Self = Self(4096);
    pub const CHECK_RESTART: Self = Self(8192);
    pub const CHECK_ABORT: Self = Self(16384);
}
static mut curMusicRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
static mut curSpeechRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
static mut resumeMusicArray: [MUSIC_POSITION; 4096] = [MUSIC_POSITION {
    filename_hash: 0,
    position: 0,
    last_played: 0,
}; 4096];
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
        uqm_lua_checked!(PlayStream(
            *pmus,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            Continuous.0 != 0,
            true,
            true,
        ), ());
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
        uqm_lua_checked!(StopStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        ), ());
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
        uqm_lua_checked!(SeekStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            pos,
        ), ());
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
        uqm_lua_checked!(PauseStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        ), ());
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
unsafe extern "C" fn get_current_music_pos(mut MusicRef: MUSIC_REF) -> DWORD {
    let mut pos: DWORD = 0 as DWORD;
    let mut length: ::core::ffi::c_float = 0.0f32;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return 0 as DWORD;
    }
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
        if uqm_lua_checked!(IsTracker(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            ), 0)
            .0 != 0
        {
            length = GetNumTrackerPos(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            ) as ::core::ffi::c_float;
            pos = uqm_lua_checked!(GetStreamFrame(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            ), 0) as DWORD;
        } else {
            length = GetStreamLength(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            );
            pos = GetStreamTime(
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
    } else {
        return 0 as DWORD
    }
    if pos > length as DWORD {
        pos = 0 as DWORD;
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn PLRGetPos() -> DWORD {
    return if !curMusicRef.is_null() {
        uqm_lua_checked!(get_current_music_pos(curMusicRef), 0)
    } else {
        0 as DWORD
    };
}
unsafe extern "C" fn get_current_music_filename(
    mut MusicRef: MUSIC_REF,
) -> *mut ::core::ffi::c_char {
    let mut filename: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
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
        filename = (*(*(*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .sample)
            .decoder)
            .filename as *mut UNICODE;
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        return filename;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn PLRGetFilename() -> *mut UNICODE {
    return if !curMusicRef.is_null() {
        get_current_music_filename(curMusicRef)
    } else {
        ::core::ptr::null_mut::<UNICODE>()
    };
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
        uqm_lua_checked!(ResumeStream(
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
        ), ());
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
unsafe extern "C" fn get_current_music_filename_hash(mut MusicRef: MUSIC_REF) -> uint32 {
    let mut filename_hash: uint32 = 0;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return 0 as uint32;
    }
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
        filename_hash = (*(*(*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .sample)
            .decoder)
            .filename_hash;
        UnlockMutex(
            (*(&raw mut soundSource as *mut TFB_SoundSource)
                .offset(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .stream_mutex,
        );
        return filename_hash;
    } else {
        return 0 as uint32
    };
}
#[no_mangle]
pub unsafe extern "C" fn PLRGetFilenameHash() -> uint32 {
    return if !curMusicRef.is_null() {
        get_current_music_filename_hash(curMusicRef)
    } else {
        0 as uint32
    };
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
        uqm_lua_checked!(PlayStream(
            *pmus,
            (0 as ::core::ffi::c_int
                + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32,
            false,
            false,
            true,
        ), ());
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
    uqm_lua_checked!(StopStream(
        (0 as ::core::ffi::c_int + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as uint32,
    ), ());
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
    return uqm_lua_checked!(_ReleaseMusicData(MusicRef as *mut ::core::ffi::c_void), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn SetMusicVolume(mut Volume: COUNT) {
    let mut f: ::core::ffi::c_float = Volume as ::core::ffi::c_int
        as ::core::ffi::c_float / 255 as ::core::ffi::c_int as ::core::ffi::c_float
        * musicVolumeScale;
    musicVolume = Volume as ::core::ffi::c_int;
    uqm_lua_checked!(audio_Sourcef(
        (*(&raw mut soundSource as *mut TFB_SoundSource)
            .offset(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .handle,
        C2Rust_Unnamed::audio_GAIN.0 as audio_SourceProp,
        f,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn CheckMusicResName(
    mut fileName: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !uqm_lua_checked!(fileExists2(contentDir, fileName), ::core::ptr::null_mut()) {
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
    uqm_lua_checked!(CheckMusicResName(&raw mut filename as *mut ::core::ffi::c_char), ::core::ptr::null_mut());
    log_add(
        log_Level::log_Info,
        b"_GetMusicData(): loading %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut filename as *mut ::core::ffi::c_char,
    );
    decoder = uqm_lua_checked!(SoundDecoder_Load(
        contentDir,
        &raw mut filename as *mut ::core::ffi::c_char,
        4096 as uint32,
        0 as uint32,
        0 as sint32,
    ), ::core::ptr::null_mut());
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
        uqm_lua_checked!(SoundDecoder_Free(decoder), ::core::ptr::null_mut());
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    sample = uqm_lua_checked!(TFB_CreateSoundSample(
        decoder,
        64 as uint32,
        ::core::ptr::null::<TFB_SoundCallbacks>(),
    ), ::core::ptr::null_mut());
    *h = sample;
    log_add(
        log_Level::log_Info,
        b"    decoder: %s, rate %d format %x\0".as_ptr() as *const ::core::ffi::c_char,
        uqm_lua_checked!(SoundDecoder_GetName((*sample).decoder), ::core::ptr::null_mut()),
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
                314 as ::core::ffi::c_int,
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
            uqm_lua_checked!(StopStream(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            ), BOOLEAN(0));
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
        uqm_lua_checked!(SoundDecoder_Free(decoder), BOOLEAN(0));
    }
    uqm_lua_checked!(TFB_DestroySoundSample(sample), BOOLEAN(0));
    HFree(data);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn SetMusicPosition() {
    let mut temp: MUSIC_POSITION = MUSIC_POSITION {
        filename_hash: 0,
        position: 0,
        last_played: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    if optMusicResume == 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        || PLRPlaying(
                ::core::ptr::with_exposed_provenance_mut::<
                    *mut TFB_SoundSample,
                >(!(0 as ::core::ffi::c_int) as usize),
            )
            .0 == 0
    {
        return;
    }
    temp.filename_hash = PLRGetFilenameHash() as DWORD;
    if temp.filename_hash == 0 {
        return;
    }
    temp.position = uqm_lua_checked!(PLRGetPos(), ());
    temp.last_played = GetTimeCounter() as DWORD;
    i = 0 as ::core::ffi::c_int;
    while i < 4096 as ::core::ffi::c_int {
        if resumeMusicArray[i as usize].filename_hash == temp.filename_hash {
            resumeMusicArray[i as usize].position = temp.position;
            resumeMusicArray[i as usize].last_played = temp.last_played;
            return;
        }
        if resumeMusicArray[i as usize].filename_hash == 0 as DWORD {
            resumeMusicArray[i as usize].filename_hash = temp.filename_hash;
            resumeMusicArray[i as usize].position = temp.position;
            resumeMusicArray[i as usize].last_played = temp.last_played;
            return;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetMusicPosition() -> DWORD {
    let mut filename_hash: DWORD = 0;
    let mut i: ::core::ffi::c_int = 0;
    if optMusicResume == 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return 0 as DWORD;
    }
    filename_hash = PLRGetFilenameHash() as DWORD;
    if filename_hash == 0 {
        return 0 as DWORD;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4096 as ::core::ffi::c_int {
        if resumeMusicArray[i as usize].filename_hash == 0 {
            return 0 as DWORD;
        }
        if resumeMusicArray[i as usize].filename_hash == filename_hash {
            return resumeMusicArray[i as usize].position;
        }
        i += 1;
    }
    return 0 as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn OkayToResume() -> BOOLEAN {
    let mut TimeIn: TimeCount = 0;
    let mut difference: TimeCount = 0;
    let mut filename_hash: DWORD = 0;
    let mut i: ::core::ffi::c_int = 0;
    if optMusicResume == 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    filename_hash = PLRGetFilenameHash() as DWORD;
    if filename_hash == 0 {
        return BOOLEAN::FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4096 as ::core::ffi::c_int {
        if resumeMusicArray[i as usize].filename_hash == filename_hash {
            break;
        }
        i += 1;
    }
    if i == 4096 as ::core::ffi::c_int {
        return BOOLEAN::FALSE;
    }
    if resumeMusicArray[i as usize].last_played == 0
        || resumeMusicArray[i as usize].position == 0
    {
        return BOOLEAN::FALSE;
    }
    TimeIn = GetTimeCounter();
    difference = TimeIn.wrapping_sub(resumeMusicArray[i as usize].last_played)
        as TimeCount;
    if optMusicResume == 2 as ::core::ffi::c_int
        || difference
            < (1000 as ::core::ffi::c_int * 300 as ::core::ffi::c_int) as TimeCount
    {
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn ResetMusicResume() {
    memset(
        &raw mut resumeMusicArray as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[MUSIC_POSITION; 4096]>() as size_t,
    );
}
