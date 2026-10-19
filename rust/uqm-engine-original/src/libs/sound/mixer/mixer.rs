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

/* Mixer for low-level sound output drivers
 */

// Rust translation of src/libs/sound/mixer/mixer.c.
extern "C" {
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
    fn abort() -> !;
    fn CreateRecursiveMutex_Core(
        name: *const ::core::ffi::c_char,
        syncClass: DWORD,
    ) -> RecursiveMutex;
    fn DestroyRecursiveMutex(m: RecursiveMutex);
    fn LockRecursiveMutex(m: RecursiveMutex);
    fn UnlockRecursiveMutex(m: RecursiveMutex);
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type intptr_t = isize;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct mixer_SourceState(pub ::core::ffi::c_uint);
impl mixer_SourceState {
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
pub struct mixer_BufferState(pub ::core::ffi::c_uint);
impl mixer_BufferState {
    pub const MIX_BUF_INITIAL: Self = Self(0);
    pub const MIX_BUF_FILLED: Self = Self(1);
    pub const MIX_BUF_QUEUED: Self = Self(2);
    pub const MIX_BUF_PLAYING: Self = Self(3);
    pub const MIX_BUF_PROCESSED: Self = Self(4);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mixer_Source {
    pub magic: uint32,
    pub locked: bool,
    pub state: mixer_SourceState,
    pub looping: bool,
    pub gain: ::core::ffi::c_float,
    pub cqueued: uint32,
    pub cprocessed: uint32,
    pub firstqueued: *mut mixer_Buffer,
    pub nextqueued: *mut mixer_Buffer,
    pub prevqueued: *mut mixer_Buffer,
    pub lastqueued: *mut mixer_Buffer,
    pub pos: uint32,
    pub count: uint32,
    pub samplecache: ::core::ffi::c_float,
}
pub type mixer_Buffer = _mixer_Buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mixer_Buffer {
    pub magic: uint32,
    pub locked: bool,
    pub state: mixer_BufferState,
    pub data: *mut uint8,
    pub size: uint32,
    pub sampsize: uint32,
    pub high: uint32,
    pub low: uint32,
    pub Resample: Option<
        unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
    >,
    pub orgdata: *mut ::core::ffi::c_void,
    pub orgfreq: uint32,
    pub orgsize: uint32,
    pub orgchannels: uint32,
    pub orgchansize: uint32,
    pub next: *mut _mixer_Buffer,
}
pub type mixer_Source = _mixer_Source;
pub type RecursiveMutex = *mut ::core::ffi::c_void;
pub type DWORD = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixer_Resampling {
    pub Upsample: Option<
        unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
    >,
    pub Downsample: Option<
        unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
    >,
    pub None: Option<
        unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
    >,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixer_Convertion {
    pub srcfmt: uint32,
    pub srcdata: *mut ::core::ffi::c_void,
    pub srcsize: uint32,
    pub srcbpc: uint32,
    pub srcchans: uint32,
    pub srcsamples: uint32,
    pub dstfmt: uint32,
    pub dstdata: *mut ::core::ffi::c_void,
    pub dstsize: uint32,
    pub dstbpc: uint32,
    pub dstchans: uint32,
    pub dstsamples: uint32,
    pub flags: mixer_ConvFlags,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mixer_ConvFlags(pub ::core::ffi::c_uint);
impl mixer_ConvFlags {
    pub const mixConvNone: Self = Self(0);
    pub const mixConvStereoUp: Self = Self(1);
    pub const mixConvStereoDown: Self = Self(2);
    pub const mixConvSizeUp: Self = Self(4);
    pub const mixConvSizeDown: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
static mut mixer_initialized: uint32 = 0 as uint32;
static mut mixer_format: uint32 = 0;
static mut mixer_chansize: uint32 = 0;
static mut mixer_sampsize: uint32 = 0;
static mut mixer_freq: uint32 = 0;
static mut mixer_channels: uint32 = 0;
static mut last_error: uint32 = C2Rust_Unnamed::MIX_NO_ERROR.0 as ::core::ffi::c_int
    as uint32;
static mut mixer_quality: mixer_Quality = mixer_Quality::MIX_QUALITY_LOW;
static mut mixer_resampling: mixer_Resampling = mixer_Resampling {
    Upsample: None,
    Downsample: None,
    None: None,
};
static mut mixer_flags: mixer_Flags = mixer_Flags::MIX_NOFLAGS;
static mut src_mutex: RecursiveMutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut buf_mutex: RecursiveMutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut act_mutex: RecursiveMutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut active_sources: [*mut mixer_Source; 8] = [::core::ptr::null_mut::<
    mixer_Source,
>(); 8];
unsafe extern "C" fn mixer_SetError(mut error: uint32) {
    last_error = error;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_GetError() -> uint32 {
    let mut error: uint32 = last_error;
    last_error = C2Rust_Unnamed::MIX_NO_ERROR.0 as ::core::ffi::c_int as uint32;
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Init(
    mut frequency: uint32,
    mut format: uint32,
    mut quality: mixer_Quality,
    mut flags: mixer_Flags,
) -> bool {
    if mixer_initialized != 0 {
        mixer_Uninit();
    }
    last_error = C2Rust_Unnamed::MIX_NO_ERROR.0 as ::core::ffi::c_int as uint32;
    memset(
        &raw mut active_sources as *mut *mut mixer_Source as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<*mut mixer_Source>().wrapping_mul(8 as size_t),
    );
    mixer_chansize = format & 0xff as uint32;
    mixer_channels = format >> 8 as ::core::ffi::c_int & 0xff as uint32;
    mixer_sampsize = (format & 0xff as uint32)
        .wrapping_mul(format >> 8 as ::core::ffi::c_int & 0xff as uint32);
    mixer_freq = frequency;
    mixer_quality = quality;
    mixer_format = format;
    mixer_flags = flags;
    mixer_resampling.None = Some(
        mixer_ResampleNone
            as unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
    ) as Option<unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float>;
    mixer_resampling.Downsample = Some(
        mixer_ResampleNearest
            as unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
    ) as Option<unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float>;
    if mixer_quality.0 == mixer_Quality::MIX_QUALITY_DEFAULT.0 {
        mixer_resampling.Upsample = Some(
            mixer_UpsampleLinear
                as unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
        )
            as Option<
                unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
            >;
    } else if mixer_quality.0 == mixer_Quality::MIX_QUALITY_HIGH.0 {
        mixer_resampling.Upsample = Some(
            mixer_UpsampleCubic
                as unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
        )
            as Option<
                unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
            >;
    } else {
        mixer_resampling.Upsample = Some(
            mixer_ResampleNearest
                as unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
        )
            as Option<
                unsafe extern "C" fn(*mut mixer_Source, bool) -> ::core::ffi::c_float,
            >;
    }
    src_mutex = CreateRecursiveMutex_Core(
        b"mixer_SourceMutex\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::SYNC_CLASS_AUDIO.0 as ::core::ffi::c_int as DWORD,
    );
    buf_mutex = CreateRecursiveMutex_Core(
        b"mixer_BufferMutex\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::SYNC_CLASS_AUDIO.0 as ::core::ffi::c_int as DWORD,
    );
    act_mutex = CreateRecursiveMutex_Core(
        b"mixer_ActiveMutex\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::SYNC_CLASS_AUDIO.0 as ::core::ffi::c_int as DWORD,
    );
    mixer_initialized = 1 as uint32;
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Uninit() {
    if mixer_initialized != 0 {
        DestroyRecursiveMutex(src_mutex);
        DestroyRecursiveMutex(buf_mutex);
        DestroyRecursiveMutex(act_mutex);
        mixer_initialized = 0 as uint32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mixer_MixChannels(
    mut userdata: *mut ::core::ffi::c_void,
    mut stream: *mut uint8,
    mut len: sint32,
) {
    let mut end_stream: *mut uint8 = stream.offset(len as isize);
    let mut left: bool = true;
    LockRecursiveMutex(src_mutex);
    LockRecursiveMutex(buf_mutex);
    LockRecursiveMutex(act_mutex);
    while stream < end_stream {
        let mut i: uint32 = 0;
        let mut fullsamp: ::core::ffi::c_float = 0 as ::core::ffi::c_int
            as ::core::ffi::c_float;
        i = 0 as uint32;
        while i < 8 as uint32 {
            let mut src: *mut mixer_Source = ::core::ptr::null_mut::<mixer_Source>();
            let mut samp: ::core::ffi::c_float = 0 as ::core::ffi::c_int
                as ::core::ffi::c_float;
            while i < 8 as uint32
                && {
                    src = active_sources[i as usize];
                    src.is_null() || (*src).state.0 != mixer_SourceState::MIX_PLAYING.0
                        || !mixer_SourceGetNextSample(src, &raw mut samp, left)
                }
            {
                i = i.wrapping_add(1);
            }
            if i < 8 as uint32 {
                fullsamp += samp;
            }
            i = i.wrapping_add(1);
        }
        if mixer_chansize == 2 as uint32 {
            if fullsamp > 32767 as ::core::ffi::c_int as ::core::ffi::c_float {
                fullsamp = 32767 as ::core::ffi::c_int as ::core::ffi::c_float;
            } else if fullsamp
                < (-32767 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_float
            {
                fullsamp = (-32767 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_float;
            }
        } else if fullsamp > 127 as ::core::ffi::c_int as ::core::ffi::c_float {
            fullsamp = 127 as ::core::ffi::c_int as ::core::ffi::c_float;
        } else if fullsamp < -128 as ::core::ffi::c_int as ::core::ffi::c_float {
            fullsamp = -128 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        mixer_PutSampleExt(
            stream as *mut ::core::ffi::c_void,
            mixer_chansize,
            fullsamp as sint32,
        );
        if mixer_channels == 2 as uint32 {
            left = !left;
        }
        stream = stream.offset(mixer_chansize as isize);
    }
    UnlockRecursiveMutex(act_mutex);
    UnlockRecursiveMutex(buf_mutex);
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_MixFake(
    mut userdata: *mut ::core::ffi::c_void,
    mut stream: *mut uint8,
    mut len: sint32,
) {
    let mut end_stream: *mut uint8 = stream.offset(len as isize);
    let mut left: bool = true;
    LockRecursiveMutex(src_mutex);
    LockRecursiveMutex(buf_mutex);
    LockRecursiveMutex(act_mutex);
    while stream < end_stream {
        let mut i: uint32 = 0;
        i = 0 as uint32;
        while i < 8 as uint32 {
            let mut src: *mut mixer_Source = ::core::ptr::null_mut::<mixer_Source>();
            let mut samp: ::core::ffi::c_float = 0.;
            while i < 8 as uint32
                && {
                    src = active_sources[i as usize];
                    src.is_null() || (*src).state.0 != mixer_SourceState::MIX_PLAYING.0
                        || !mixer_SourceGetFakeSample(src, &raw mut samp, left)
                }
            {
                i = i.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if mixer_channels == 2 as uint32 {
            left = !left;
        }
        stream = stream.offset(mixer_chansize as isize);
    }
    UnlockRecursiveMutex(act_mutex);
    UnlockRecursiveMutex(buf_mutex);
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_GenSources(
    mut n: uint32,
    mut psrcobj: *mut mixer_Object,
) {
    if n == 0 as uint32 {
        return;
    }
    if psrcobj.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GenSources() called with null ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    while n != 0 {
        let mut src: *mut mixer_Source = ::core::ptr::null_mut::<mixer_Source>();
        src = HMalloc(::core::mem::size_of::<mixer_Source>()) as *mut mixer_Source;
        (*src).magic = 0x5358494d as ::core::ffi::c_uint as uint32;
        (*src).locked = false;
        (*src).state = mixer_SourceState::MIX_INITIAL;
        (*src).looping = false;
        (*src).gain = 0.75f32;
        (*src).cqueued = 0 as uint32;
        (*src).cprocessed = 0 as uint32;
        (*src).firstqueued = ::core::ptr::null_mut::<mixer_Buffer>();
        (*src).nextqueued = ::core::ptr::null_mut::<mixer_Buffer>();
        (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
        (*src).lastqueued = ::core::ptr::null_mut::<mixer_Buffer>();
        (*src).pos = 0 as uint32;
        (*src).count = 0 as uint32;
        *psrcobj = src.expose_provenance() as mixer_Object;
        n = n.wrapping_sub(1);
        psrcobj = psrcobj.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mixer_DeleteSources(
    mut n: uint32,
    mut psrcobj: *mut mixer_Object,
) {
    let mut i: uint32 = 0;
    let mut pcurobj: *mut mixer_Object = ::core::ptr::null_mut::<mixer_Object>();
    if n == 0 as uint32 {
        return;
    }
    if psrcobj.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_DeleteSources() called with null ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    i = n;
    pcurobj = psrcobj;
    while i != 0 && !pcurobj.is_null() {
        let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
            mixer_Source,
        >(*pcurobj as usize);
        if !src.is_null() {
            if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
                break;
            }
        }
        i = i.wrapping_sub(1);
        pcurobj = pcurobj.offset(1);
    }
    if i != 0 {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_DeleteSources(): not a source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        while n != 0 {
            let mut src_0: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
                mixer_Source,
            >(*psrcobj as usize);
            if !src_0.is_null() {
                if (*src_0).state.0 != mixer_SourceState::MIX_INITIAL.0 {
                    mixer_SourceStop_internal(src_0);
                }
                mixer_SourceUnqueueAll(src_0);
                HFree(src_0 as *mut ::core::ffi::c_void);
                *psrcobj = 0 as mixer_Object;
            }
            n = n.wrapping_sub(1);
            psrcobj = psrcobj.offset(1);
        }
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_IsSource(mut srcobj: mixer_Object) -> bool {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    let mut ret: bool = false;
    if src.is_null() {
        return false;
    }
    LockRecursiveMutex(src_mutex);
    ret = (*src).magic == 0x5358494d as ::core::ffi::c_uint;
    UnlockRecursiveMutex(src_mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Sourcei(
    mut srcobj: mixer_Object,
    mut pname: mixer_SourceProp,
    mut value: mixer_IntVal,
) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_Sourcei() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_Sourcei(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        match pname {
            mixer_SourceProp::MIX_LOOPING => {
                (*src).looping = value != 0;
            }
            mixer_SourceProp::MIX_BUFFER => {
                let mut buf: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
                    mixer_Buffer,
                >(value as usize);
                if (*src).cqueued > 0 as uint32 {
                    mixer_SourceUnqueueAll(src);
                }
                if !(!buf.is_null()
                    && !mixer_CheckBufferState(
                        buf,
                        b"mixer_Sourcei\0".as_ptr() as *const ::core::ffi::c_char,
                    ))
                {
                    (*src).firstqueued = buf;
                    (*src).nextqueued = (*src).firstqueued;
                    (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
                    (*src).lastqueued = (*src).nextqueued;
                    if !(*src).lastqueued.is_null() {
                        (*(*src).lastqueued).next = ::core::ptr::null_mut::<
                            _mixer_Buffer,
                        >();
                    }
                    (*src).cqueued = 1 as uint32;
                }
            }
            mixer_SourceProp::MIX_SOURCE_STATE => {
                if value
                    == mixer_SourceState::MIX_INITIAL.0 as ::core::ffi::c_int
                        as mixer_IntVal
                {
                    mixer_SourceRewind_internal(src);
                } else {
                    log_add(
                        log_Level::log_Debug,
                        b"mixer_Sourcei(MIX_SOURCE_STATE): unsupported state, call ignored\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            _ => {
                mixer_SetError(
                    C2Rust_Unnamed::MIX_INVALID_ENUM.0 as ::core::ffi::c_int as uint32,
                );
                log_add(
                    log_Level::log_Debug,
                    b"mixer_Sourcei() called with unsupported property %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pname.0,
                );
            }
        }
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Sourcef(
    mut srcobj: mixer_Object,
    mut pname: mixer_SourceProp,
    mut value: ::core::ffi::c_float,
) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_Sourcef() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_Sourcef(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        match pname {
            mixer_SourceProp::MIX_GAIN => {
                (*src).gain = value * 0.75f32;
            }
            _ => {
                log_add(
                    log_Level::log_Debug,
                    b"mixer_Sourcei() called with unsupported property %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pname.0,
                );
            }
        }
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_Sourcefv(
    mut srcobj: mixer_Object,
    mut pname: mixer_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {}
#[no_mangle]
pub unsafe extern "C" fn mixer_GetSourcei(
    mut srcobj: mixer_Object,
    mut pname: mixer_SourceProp,
    mut value: *mut mixer_IntVal,
) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() || value.is_null() {
        mixer_SetError(
            (if !src.is_null() {
                C2Rust_Unnamed::MIX_INVALID_VALUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int
            }) as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetSourcei() called with null param\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetSourcei(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        match pname {
            mixer_SourceProp::MIX_LOOPING => {
                *value = (*src).looping as mixer_IntVal;
            }
            mixer_SourceProp::MIX_BUFFER => {
                *value = (*src).firstqueued.expose_provenance() as mixer_IntVal;
            }
            mixer_SourceProp::MIX_SOURCE_STATE => {
                *value = (*src).state.0 as mixer_IntVal;
            }
            mixer_SourceProp::MIX_BUFFERS_QUEUED => {
                *value = (*src).cqueued as mixer_IntVal;
            }
            mixer_SourceProp::MIX_BUFFERS_PROCESSED => {
                *value = (*src).cprocessed as mixer_IntVal;
            }
            _ => {
                mixer_SetError(
                    C2Rust_Unnamed::MIX_INVALID_ENUM.0 as ::core::ffi::c_int as uint32,
                );
                log_add(
                    log_Level::log_Debug,
                    b"mixer_GetSourcei() called with unsupported property %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pname.0,
                );
            }
        }
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_GetSourcef(
    mut srcobj: mixer_Object,
    mut pname: mixer_SourceProp,
    mut value: *mut ::core::ffi::c_float,
) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() || value.is_null() {
        mixer_SetError(
            (if !src.is_null() {
                C2Rust_Unnamed::MIX_INVALID_VALUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int
            }) as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetSourcef() called with null param\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetSourcef(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        match pname {
            mixer_SourceProp::MIX_GAIN => {
                *value = (*src).gain / 0.75f32;
            }
            _ => {
                log_add(
                    log_Level::log_Debug,
                    b"mixer_GetSourcef() called with unsupported property %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pname.0,
                );
            }
        }
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SourcePlay(mut srcobj: mixer_Object) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourcePlay() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourcePlay(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        if (*src).state.0 < mixer_SourceState::MIX_PLAYING.0 {
            if !(*src).firstqueued.is_null() && (*src).nextqueued.is_null() {
                mixer_SourceRewind_internal(src);
            }
            mixer_SourceActivate(src);
        }
        (*src).state = mixer_SourceState::MIX_PLAYING;
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SourceRewind(mut srcobj: mixer_Object) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceRewind() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourcePlay(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        mixer_SourceRewind_internal(src);
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SourcePause(mut srcobj: mixer_Object) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourcePause() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourcePause(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        if (*src).state.0 < mixer_SourceState::MIX_PLAYING.0 {
            mixer_SourceActivate(src);
        }
        (*src).state = mixer_SourceState::MIX_PAUSED;
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SourceStop(mut srcobj: mixer_Object) {
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceStop() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceStop(): not a source\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        if (*src).state.0 >= mixer_SourceState::MIX_PLAYING.0 {
            mixer_SourceDeactivate(src);
        }
        mixer_SourceStop_internal(src);
        (*src).state = mixer_SourceState::MIX_STOPPED;
    }
    UnlockRecursiveMutex(src_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SourceQueueBuffers(
    mut srcobj: mixer_Object,
    mut n: uint32,
    mut pbufobj: *mut mixer_Object,
) {
    let mut i: uint32 = 0;
    let mut pobj: *mut mixer_Object = ::core::ptr::null_mut::<mixer_Object>();
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    if src.is_null() || pbufobj.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceQueueBuffers() called with null param\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(buf_mutex);
    i = n;
    pobj = pbufobj;
    while i != 0 {
        let mut buf: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
            mixer_Buffer,
        >(*pobj as usize);
        if buf.is_null()
            || !mixer_CheckBufferState(
                buf,
                b"mixer_SourceQueueBuffers\0".as_ptr() as *const ::core::ffi::c_char,
            )
        {
            break;
        }
        i = i.wrapping_sub(1);
        pobj = pobj.offset(1);
    }
    UnlockRecursiveMutex(buf_mutex);
    if i == 0 as uint32 {
        LockRecursiveMutex(src_mutex);
        LockRecursiveMutex(buf_mutex);
        if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
            mixer_SetError(
                C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
            );
            log_add(
                log_Level::log_Debug,
                b"mixer_SourceQueueBuffers(): not a source\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            i = n;
            pobj = pbufobj;
            while i != 0 {
                let mut buf_0: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
                    mixer_Buffer,
                >(*pobj as usize);
                if !(*src).lastqueued.is_null() {
                    (*(*src).lastqueued).next = buf_0 as *mut _mixer_Buffer;
                }
                (*src).lastqueued = buf_0;
                if (*src).firstqueued.is_null() {
                    (*src).firstqueued = buf_0;
                    (*src).nextqueued = buf_0;
                    (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
                }
                (*src).cqueued = (*src).cqueued.wrapping_add(1);
                (*buf_0).state = mixer_BufferState::MIX_BUF_QUEUED;
                i = i.wrapping_sub(1);
                pobj = pobj.offset(1);
            }
        }
        UnlockRecursiveMutex(buf_mutex);
        UnlockRecursiveMutex(src_mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mixer_SourceUnqueueBuffers(
    mut srcobj: mixer_Object,
    mut n: uint32,
    mut pbufobj: *mut mixer_Object,
) {
    let mut i: uint32 = 0;
    let mut src: *mut mixer_Source = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Source,
    >(srcobj as usize);
    let mut curbuf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
    if src.is_null() || pbufobj.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceUnqueueBuffers() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(src_mutex);
    if (*src).magic != 0x5358494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceUnqueueBuffers(): not a source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if n > (*src).cqueued {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
        );
    } else {
        LockRecursiveMutex(buf_mutex);
        i = n;
        curbuf = (*src).firstqueued;
        while i != 0 && !curbuf.is_null()
            && (*curbuf).state.0 != mixer_BufferState::MIX_BUF_PLAYING.0
        {
            i = i.wrapping_sub(1);
            curbuf = (*curbuf).next as *mut mixer_Buffer;
        }
        if i != 0 {
            mixer_SetError(
                C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
            );
            log_add(
                log_Level::log_Debug,
                b"mixer_SourceUnqueueBuffers(): active buffer attempted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            i = n;
            while i != 0 {
                let mut buf: *mut mixer_Buffer = (*src).firstqueued;
                if (*src).nextqueued == buf {
                    (*src).nextqueued = (*buf).next as *mut mixer_Buffer;
                }
                if (*src).prevqueued == buf {
                    (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
                }
                if (*src).lastqueued == buf {
                    (*src).lastqueued = ::core::ptr::null_mut::<mixer_Buffer>();
                }
                (*src).firstqueued = (*buf).next as *mut mixer_Buffer;
                (*src).cqueued = (*src).cqueued.wrapping_sub(1);
                if (*buf).state.0 == mixer_BufferState::MIX_BUF_PROCESSED.0 {
                    (*src).cprocessed = (*src).cprocessed.wrapping_sub(1);
                }
                (*buf).state = mixer_BufferState::MIX_BUF_FILLED;
                (*buf).next = ::core::ptr::null_mut::<_mixer_Buffer>();
                *pbufobj = buf.expose_provenance() as mixer_Object;
                i = i.wrapping_sub(1);
                pbufobj = pbufobj.offset(1);
            }
        }
        UnlockRecursiveMutex(buf_mutex);
    }
    UnlockRecursiveMutex(src_mutex);
}
unsafe extern "C" fn mixer_SourceUnqueueAll(mut src: *mut mixer_Source) {
    let mut buf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
    let mut nextbuf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
    if src.is_null() {
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceUnqueueAll() called with null source\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(buf_mutex);
    buf = (*src).firstqueued;
    while !buf.is_null() {
        if (*buf).state.0 == mixer_BufferState::MIX_BUF_PLAYING.0 {
            log_add(
                log_Level::log_Debug,
                b"mixer_SourceUnqueueAll(): attempted on active buffer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        nextbuf = (*buf).next as *mut mixer_Buffer;
        (*buf).state = mixer_BufferState::MIX_BUF_FILLED;
        (*buf).next = ::core::ptr::null_mut::<_mixer_Buffer>();
        buf = nextbuf;
    }
    UnlockRecursiveMutex(buf_mutex);
    (*src).firstqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).nextqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).lastqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).cqueued = 0 as uint32;
    (*src).cprocessed = 0 as uint32;
    (*src).pos = 0 as uint32;
    (*src).count = 0 as uint32;
}
unsafe extern "C" fn mixer_SourceActivate(mut src: *mut mixer_Source) {
    let mut i: uint32 = 0;
    LockRecursiveMutex(act_mutex);
    i = 0 as uint32;
    while i < 8 as uint32 && active_sources[i as usize] != src {
        i = i.wrapping_add(1);
    }
    if i < 8 as uint32 {
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceActivate(): source already active in slot %u\0".as_ptr()
                as *const ::core::ffi::c_char,
            i,
        );
        UnlockRecursiveMutex(act_mutex);
        return;
    }
    i = 0 as uint32;
    while i < 8 as uint32 && !active_sources[i as usize].is_null() {
        i = i.wrapping_add(1);
    }
    if i < 8 as uint32 {
        active_sources[i as usize] = src;
    } else {
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceActivate(): no more slots available (max=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            8 as ::core::ffi::c_int,
        );
    }
    UnlockRecursiveMutex(act_mutex);
}
unsafe extern "C" fn mixer_SourceDeactivate(mut src: *mut mixer_Source) {
    let mut i: uint32 = 0;
    LockRecursiveMutex(act_mutex);
    i = 0 as uint32;
    while i < 8 as uint32 && active_sources[i as usize] != src {
        i = i.wrapping_add(1);
    }
    if i < 8 as uint32 {
        active_sources[i as usize] = ::core::ptr::null_mut::<mixer_Source>();
    } else {
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceDeactivate(): source not active\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    UnlockRecursiveMutex(act_mutex);
}
unsafe extern "C" fn mixer_SourceStop_internal(mut src: *mut mixer_Source) {
    let mut buf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
    let mut nextbuf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
    if (*src).firstqueued.is_null() {
        return;
    }
    if (*src).lastqueued.is_null() {
        log_add(
            log_Level::log_Debug,
            b"mixer_SourceStop_internal(): desynced source state\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        explode();
    }
    LockRecursiveMutex(buf_mutex);
    buf = (*src).firstqueued;
    while !buf.is_null() && !(*buf).next.is_null() && (*buf).next != (*src).nextqueued {
        buf = (*buf).next as *mut mixer_Buffer;
    }
    (*src).lastqueued = buf;
    if !buf.is_null() {
        (*buf).next = ::core::ptr::null_mut::<_mixer_Buffer>();
    }
    buf = (*src).nextqueued;
    while !buf.is_null() {
        nextbuf = (*buf).next as *mut mixer_Buffer;
        (*buf).state = mixer_BufferState::MIX_BUF_FILLED;
        (*buf).next = ::core::ptr::null_mut::<_mixer_Buffer>();
        (*src).cqueued = (*src).cqueued.wrapping_sub(1);
        buf = nextbuf;
    }
    if (*src).cqueued == 0 as uint32 {
        (*src).firstqueued = ::core::ptr::null_mut::<mixer_Buffer>();
        (*src).lastqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    }
    (*src).nextqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).pos = 0 as uint32;
    (*src).count = 0 as uint32;
    UnlockRecursiveMutex(buf_mutex);
}
unsafe extern "C" fn mixer_SourceRewind_internal(mut src: *mut mixer_Source) {
    let mut buf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
    if (*src).state.0 >= mixer_SourceState::MIX_PLAYING.0 {
        mixer_SourceDeactivate(src);
    }
    LockRecursiveMutex(buf_mutex);
    buf = (*src).firstqueued;
    while !buf.is_null() && (*buf).state.0 != mixer_BufferState::MIX_BUF_QUEUED.0 {
        (*buf).state = mixer_BufferState::MIX_BUF_QUEUED;
        buf = (*buf).next as *mut mixer_Buffer;
    }
    UnlockRecursiveMutex(buf_mutex);
    (*src).pos = 0 as uint32;
    (*src).count = 0 as uint32;
    (*src).cprocessed = 0 as uint32;
    (*src).nextqueued = (*src).firstqueued;
    (*src).prevqueued = ::core::ptr::null_mut::<mixer_Buffer>();
    (*src).state = mixer_SourceState::MIX_INITIAL;
}
#[inline]
unsafe extern "C" fn mixer_SourceGetNextSample(
    mut src: *mut mixer_Source,
    mut psamp: *mut ::core::ffi::c_float,
    mut left: bool,
) -> bool {
    if mixer_flags.0 & mixer_Flags::MIX_FAKE_DATA.0 != 0 {
        return mixer_SourceGetFakeSample(src, psamp, left);
    }
    while !(*src).nextqueued.is_null() {
        let mut buf: *mut mixer_Buffer = (*src).nextqueued;
        if (*buf).data.is_null() || (*buf).size < mixer_sampsize {
            (*buf).state = mixer_BufferState::MIX_BUF_PROCESSED;
            (*src).pos = 0 as uint32;
            (*src).nextqueued = (*(*src).nextqueued).next as *mut mixer_Buffer;
            (*src).cprocessed = (*src).cprocessed.wrapping_add(1);
        } else {
            if !left && (*buf).orgchannels == 1 as uint32 {
                *psamp = (*src).samplecache;
            } else {
                (*src).samplecache = (*buf)
                    .Resample
                    .expect("non-null function pointer")(src, left) * (*src).gain;
                *psamp = (*src).samplecache;
            }
            if (*src).pos < (*buf).size
                || left as ::core::ffi::c_int != 0 && (*buf).sampsize != mixer_sampsize
            {
                (*buf).state = mixer_BufferState::MIX_BUF_PLAYING;
            } else {
                (*buf).state = mixer_BufferState::MIX_BUF_PROCESSED;
                (*src).pos = 0 as uint32;
                (*src).prevqueued = (*src).nextqueued;
                (*src).nextqueued = (*(*src).nextqueued).next as *mut mixer_Buffer;
                (*src).cprocessed = (*src).cprocessed.wrapping_add(1);
            }
            return true;
        }
    }
    if (*src).state.0 >= mixer_SourceState::MIX_PLAYING.0 {
        mixer_SourceDeactivate(src);
    }
    (*src).state = mixer_SourceState::MIX_STOPPED;
    return false;
}
#[inline]
unsafe extern "C" fn mixer_SourceGetFakeSample(
    mut src: *mut mixer_Source,
    mut psamp: *mut ::core::ffi::c_float,
    mut left: bool,
) -> bool {
    if !(*src).nextqueued.is_null() {
        let mut buf: *mut mixer_Buffer = (*src).nextqueued;
        if left as ::core::ffi::c_int != 0 || (*buf).orgchannels != 1 as uint32 {
            if mixer_freq == (*buf).orgfreq {
                (*src).pos = (*src).pos.wrapping_add(mixer_chansize);
            } else {
                mixer_SourceAdvance(src, left);
            }
        }
        *psamp = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        if (*src).pos < (*buf).size
            || left as ::core::ffi::c_int != 0 && (*buf).sampsize != mixer_sampsize
        {
            (*buf).state = mixer_BufferState::MIX_BUF_PLAYING;
        } else {
            (*buf).state = mixer_BufferState::MIX_BUF_PROCESSED;
            (*src).pos = 0 as uint32;
            (*src).prevqueued = (*src).nextqueued;
            (*src).nextqueued = (*(*src).nextqueued).next as *mut mixer_Buffer;
            (*src).cprocessed = (*src).cprocessed.wrapping_add(1);
        }
        return true;
    }
    if (*src).state.0 >= mixer_SourceState::MIX_PLAYING.0 {
        mixer_SourceDeactivate(src);
    }
    (*src).state = mixer_SourceState::MIX_STOPPED;
    return false;
}
#[inline]
unsafe extern "C" fn mixer_SourceAdvance(
    mut src: *mut mixer_Source,
    mut left: bool,
) -> uint32 {
    let mut curr: *mut mixer_Buffer = (*src).nextqueued;
    if (*curr).orgchannels == 2 as uint32 && mixer_channels == 2 as uint32 {
        if !left {
            (*src).pos = (*src).pos.wrapping_add((*curr).high);
            (*src).count = (*src).count.wrapping_add((*curr).low);
            if (*src).count > 0xffff as uint32 {
                (*src).count = (*src).count.wrapping_sub(0xffff as uint32);
                (*src).pos = (*src).pos.wrapping_add((*curr).sampsize);
            }
            return mixer_chansize;
        }
    } else {
        (*src).pos = (*src).pos.wrapping_add((*curr).high);
        (*src).count = (*src).count.wrapping_add((*curr).low);
        if (*src).count > 0xffff as uint32 {
            (*src).count = (*src).count.wrapping_sub(0xffff as uint32);
            (*src).pos = (*src).pos.wrapping_add((*curr).sampsize);
        }
    }
    return 0 as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_GenBuffers(
    mut n: uint32,
    mut pbufobj: *mut mixer_Object,
) {
    if n == 0 as uint32 {
        return;
    }
    if pbufobj.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_VALUE.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GenBuffers() called with null ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    while n != 0 {
        let mut buf: *mut mixer_Buffer = ::core::ptr::null_mut::<mixer_Buffer>();
        buf = HMalloc(::core::mem::size_of::<mixer_Buffer>()) as *mut mixer_Buffer;
        (*buf).magic = 0x4258494d as ::core::ffi::c_uint as uint32;
        (*buf).locked = false;
        (*buf).state = mixer_BufferState::MIX_BUF_INITIAL;
        (*buf).data = ::core::ptr::null_mut::<uint8>();
        (*buf).size = 0 as uint32;
        (*buf).next = ::core::ptr::null_mut::<_mixer_Buffer>();
        (*buf).orgdata = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*buf).orgfreq = 0 as uint32;
        (*buf).orgsize = 0 as uint32;
        (*buf).orgchannels = 0 as uint32;
        (*buf).orgchansize = 0 as uint32;
        *pbufobj = buf.expose_provenance() as mixer_Object;
        n = n.wrapping_sub(1);
        pbufobj = pbufobj.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mixer_DeleteBuffers(
    mut n: uint32,
    mut pbufobj: *mut mixer_Object,
) {
    let mut i: uint32 = 0;
    let mut pcurobj: *mut mixer_Object = ::core::ptr::null_mut::<mixer_Object>();
    if n == 0 as uint32 {
        return;
    }
    if pbufobj.is_null() {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_DeleteBuffers() called with null ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(buf_mutex);
    i = n;
    pcurobj = pbufobj;
    while i != 0 && !pcurobj.is_null() {
        let mut buf: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
            mixer_Buffer,
        >(*pcurobj as usize);
        if !buf.is_null() {
            if (*buf).magic != 0x4258494d as ::core::ffi::c_uint {
                mixer_SetError(
                    C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
                );
                log_add(
                    log_Level::log_Debug,
                    b"mixer_DeleteBuffers(): not a buffer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else if (*buf).locked {
                mixer_SetError(
                    C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int
                        as uint32,
                );
                log_add(
                    log_Level::log_Debug,
                    b"mixer_DeleteBuffers(): locked buffer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else if (*buf).state.0 >= mixer_BufferState::MIX_BUF_QUEUED.0 {
                mixer_SetError(
                    C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int
                        as uint32,
                );
                log_add(
                    log_Level::log_Debug,
                    b"mixer_DeleteBuffers(): attempted on queued/active buffer\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            }
        }
        i = i.wrapping_sub(1);
        pcurobj = pcurobj.offset(1);
    }
    if i == 0 as uint32 {
        while n != 0 {
            let mut buf_0: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
                mixer_Buffer,
            >(*pbufobj as usize);
            if !buf_0.is_null() {
                if !(*buf_0).data.is_null() {
                    HFree((*buf_0).data as *mut ::core::ffi::c_void);
                }
                HFree(buf_0 as *mut ::core::ffi::c_void);
                *pbufobj = 0 as mixer_Object;
            }
            n = n.wrapping_sub(1);
            pbufobj = pbufobj.offset(1);
        }
    }
    UnlockRecursiveMutex(buf_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_IsBuffer(mut bufobj: mixer_Object) -> bool {
    let mut buf: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Buffer,
    >(bufobj as usize);
    let mut ret: bool = false;
    if buf.is_null() {
        return false;
    }
    LockRecursiveMutex(buf_mutex);
    ret = (*buf).magic == 0x4258494d as ::core::ffi::c_uint;
    UnlockRecursiveMutex(buf_mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mixer_GetBufferi(
    mut bufobj: mixer_Object,
    mut pname: mixer_BufferProp,
    mut value: *mut mixer_IntVal,
) {
    let mut buf: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Buffer,
    >(bufobj as usize);
    if buf.is_null() || value.is_null() {
        mixer_SetError(
            (if !buf.is_null() {
                C2Rust_Unnamed::MIX_INVALID_VALUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int
            }) as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetBufferi() called with null param\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(buf_mutex);
    if (*buf).locked {
        UnlockRecursiveMutex(buf_mutex);
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetBufferi() called with locked buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*buf).magic != 0x4258494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_GetBufferi(): not a buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        match pname {
            mixer_BufferProp::MIX_FREQUENCY => {
                *value = (*buf).orgfreq as mixer_IntVal;
            }
            mixer_BufferProp::MIX_BITS => {
                *value = ((*buf).orgchansize << 3 as ::core::ffi::c_int) as mixer_IntVal;
            }
            mixer_BufferProp::MIX_CHANNELS => {
                *value = (*buf).orgchannels as mixer_IntVal;
            }
            mixer_BufferProp::MIX_SIZE => {
                *value = (*buf).orgsize as mixer_IntVal;
            }
            mixer_BufferProp::MIX_DATA => {
                *value = (*buf).orgdata.expose_provenance() as mixer_IntVal;
            }
            _ => {
                mixer_SetError(
                    C2Rust_Unnamed::MIX_INVALID_ENUM.0 as ::core::ffi::c_int as uint32,
                );
                log_add(
                    log_Level::log_Debug,
                    b"mixer_GetBufferi() called with invalid property %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pname.0,
                );
            }
        }
    }
    UnlockRecursiveMutex(buf_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn mixer_BufferData(
    mut bufobj: mixer_Object,
    mut format: uint32,
    mut data: *mut ::core::ffi::c_void,
    mut size: uint32,
    mut freq: uint32,
) {
    let mut buf: *mut mixer_Buffer = ::core::ptr::with_exposed_provenance_mut::<
        mixer_Buffer,
    >(bufobj as usize);
    let mut conv: mixer_Convertion = mixer_Convertion {
        srcfmt: 0,
        srcdata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        srcsize: 0,
        srcbpc: 0,
        srcchans: 0,
        srcsamples: 0,
        dstfmt: 0,
        dstdata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dstsize: 0,
        dstbpc: 0,
        dstchans: 0,
        dstsamples: 0,
        flags: mixer_ConvFlags::mixConvNone,
    };
    let mut dstsize: uint32 = 0;
    if buf.is_null() || data.is_null() || size == 0 {
        mixer_SetError(
            (if !buf.is_null() {
                C2Rust_Unnamed::MIX_INVALID_VALUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int
            }) as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_BufferData() called with bad param\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockRecursiveMutex(buf_mutex);
    if (*buf).locked {
        UnlockRecursiveMutex(buf_mutex);
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_BufferData() called with locked buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*buf).magic != 0x4258494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_BufferData(): not a buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if (*buf).state.0 > mixer_BufferState::MIX_BUF_FILLED.0 {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"mixer_BufferData() attempted on in-use buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        if !(*buf).data.is_null() {
            HFree((*buf).data as *mut ::core::ffi::c_void);
        }
        (*buf).data = ::core::ptr::null_mut::<uint8>();
        (*buf).size = 0 as uint32;
        (*buf).orgdata = data;
        (*buf).orgfreq = freq;
        (*buf).orgsize = size;
        (*buf).orgchannels = format >> 8 as ::core::ffi::c_int & 0xff as uint32;
        (*buf).orgchansize = format & 0xff as uint32;
        conv.dstsamples = size
            .wrapping_div(
                (format & 0xff as uint32)
                    .wrapping_mul(format >> 8 as ::core::ffi::c_int & 0xff as uint32),
            );
        conv.srcsamples = conv.dstsamples;
        if conv.dstsamples
            > (0xffffffff as uint32)
                .wrapping_div(
                    (format & 0xff as uint32)
                        .wrapping_mul(format >> 8 as ::core::ffi::c_int & 0xff as uint32),
                )
        {
            mixer_SetError(
                C2Rust_Unnamed::MIX_INVALID_VALUE.0 as ::core::ffi::c_int as uint32,
            );
        } else {
            if (format >> 8 as ::core::ffi::c_int & 0xff as uint32)
                < mixer_format >> 8 as ::core::ffi::c_int & 0xff as uint32
            {
                (*buf).sampsize = (mixer_format & 0xff as uint32)
                    .wrapping_mul(format >> 8 as ::core::ffi::c_int & 0xff as uint32);
            } else {
                (*buf).sampsize = (mixer_format & 0xff as uint32)
                    .wrapping_mul(
                        mixer_format >> 8 as ::core::ffi::c_int & 0xff as uint32,
                    );
            }
            dstsize = conv.dstsamples.wrapping_mul((*buf).sampsize);
            (*buf).size = dstsize;
            if mixer_flags.0 & mixer_Flags::MIX_FAKE_DATA.0 == 0 {
                (*buf).data = HMalloc(dstsize as size_t) as *mut uint8;
                if format & 0xff as uint32 == mixer_format & 0xff as uint32
                    && format >> 8 as ::core::ffi::c_int & 0xff as uint32
                        <= mixer_format >> 8 as ::core::ffi::c_int & 0xff as uint32
                {
                    (*buf).locked = true;
                    UnlockRecursiveMutex(buf_mutex);
                    memcpy(
                        (*buf).data as *mut ::core::ffi::c_void,
                        data,
                        size as size_t,
                    );
                    if format & 0xff as uint32 == 1 as uint32 {
                        let mut dst: *mut uint8 = ::core::ptr::null_mut::<uint8>();
                        dst = (*buf).data;
                        while dstsize != 0 {
                            *dst = (*dst as ::core::ffi::c_int
                                ^ 0x80 as ::core::ffi::c_int) as uint8;
                            dstsize = dstsize.wrapping_sub(1);
                            dst = dst.offset(1);
                        }
                    }
                    LockRecursiveMutex(buf_mutex);
                    (*buf).locked = false;
                } else {
                    conv.srcfmt = format;
                    conv.srcdata = data;
                    conv.srcsize = size;
                    if (format >> 8 as ::core::ffi::c_int & 0xff as uint32)
                        < mixer_format >> 8 as ::core::ffi::c_int & 0xff as uint32
                    {
                        conv.dstfmt = 0x170000 as ::core::ffi::c_int as uint32
                            | mixer_chansize & 0xff as uint32
                            | (format >> 8 as ::core::ffi::c_int & 0xff as uint32
                                & 0xff as uint32) << 8 as ::core::ffi::c_int;
                    } else {
                        conv.dstfmt = mixer_format;
                    }
                    conv.dstdata = (*buf).data as *mut ::core::ffi::c_void;
                    conv.dstsize = dstsize;
                    (*buf).locked = true;
                    UnlockRecursiveMutex(buf_mutex);
                    mixer_ConvertBuffer_internal(&raw mut conv);
                    LockRecursiveMutex(buf_mutex);
                    (*buf).locked = false;
                }
            }
            (*buf).state = mixer_BufferState::MIX_BUF_FILLED;
            (*buf).high = (*buf)
                .orgfreq
                .wrapping_div(mixer_freq)
                .wrapping_mul((*buf).sampsize);
            (*buf).low = ((*buf).orgfreq.wrapping_rem(mixer_freq)
                << 16 as ::core::ffi::c_int)
                .wrapping_div(mixer_freq);
            if mixer_freq == (*buf).orgfreq {
                (*buf).Resample = mixer_resampling.None;
            } else if mixer_freq > (*buf).orgfreq {
                (*buf).Resample = mixer_resampling.Upsample;
            } else {
                (*buf).Resample = mixer_resampling.Downsample;
            }
        }
    }
    UnlockRecursiveMutex(buf_mutex);
}
#[inline]
unsafe extern "C" fn mixer_CheckBufferState(
    mut buf: *mut mixer_Buffer,
    mut FuncName: *const ::core::ffi::c_char,
) -> bool {
    if buf.is_null() {
        return false;
    }
    if (*buf).magic != 0x4258494d as ::core::ffi::c_uint {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_NAME.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"%s(): not a buffer\0".as_ptr() as *const ::core::ffi::c_char,
            FuncName,
        );
        return false;
    }
    if (*buf).locked {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"%s(): locked buffer attempted\0".as_ptr() as *const ::core::ffi::c_char,
            FuncName,
        );
        return false;
    }
    if (*buf).state.0 != mixer_BufferState::MIX_BUF_FILLED.0 {
        mixer_SetError(
            C2Rust_Unnamed::MIX_INVALID_OPERATION.0 as ::core::ffi::c_int as uint32,
        );
        log_add(
            log_Level::log_Debug,
            b"%s: invalid buffer attempted\0".as_ptr() as *const ::core::ffi::c_char,
            FuncName,
        );
        return false;
    }
    return true;
}
unsafe extern "C" fn mixer_ConvertBuffer_internal(mut conv: *mut mixer_Convertion) {
    (*conv).srcbpc = (*conv).srcfmt & 0xff as uint32;
    (*conv).srcchans = (*conv).srcfmt >> 8 as ::core::ffi::c_int & 0xff as uint32;
    (*conv).dstbpc = (*conv).dstfmt & 0xff as uint32;
    (*conv).dstchans = (*conv).dstfmt >> 8 as ::core::ffi::c_int & 0xff as uint32;
    (*conv).flags = mixer_ConvFlags(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    if (*conv).srcbpc > (*conv).dstbpc {
        (*conv).flags = mixer_ConvFlags(
            (*conv).flags.0 | mixer_ConvFlags::mixConvSizeDown.0,
        );
    } else if (*conv).srcbpc < (*conv).dstbpc {
        (*conv).flags = mixer_ConvFlags(
            (*conv).flags.0 | mixer_ConvFlags::mixConvSizeUp.0,
        );
    }
    if (*conv).srcchans > (*conv).dstchans {
        (*conv).flags = mixer_ConvFlags(
            (*conv).flags.0 | mixer_ConvFlags::mixConvStereoDown.0,
        );
    } else if (*conv).srcchans < (*conv).dstchans {
        (*conv).flags = mixer_ConvFlags(
            (*conv).flags.0 | mixer_ConvFlags::mixConvStereoUp.0,
        );
    }
    mixer_ResampleFlat(conv);
}
#[inline]
unsafe extern "C" fn mixer_GetSampleExt(
    mut src: *mut ::core::ffi::c_void,
    mut bpc: uint32,
) -> sint32 {
    if bpc == 2 as uint32 {
        return *(src as *mut sint16) as sint32
    } else {
        return *(src as *mut uint8) as sint32 - 128 as sint32
    };
}
#[inline]
unsafe extern "C" fn mixer_GetSampleInt(
    mut src: *mut ::core::ffi::c_void,
    mut bpc: uint32,
) -> sint32 {
    if bpc == 2 as uint32 {
        return *(src as *mut sint16) as sint32
    } else {
        return *(src as *mut sint8) as sint32
    };
}
#[inline]
unsafe extern "C" fn mixer_PutSampleExt(
    mut dst: *mut ::core::ffi::c_void,
    mut bpc: uint32,
    mut samp: sint32,
) {
    if bpc == 2 as uint32 {
        *(dst as *mut sint16) = samp as sint16;
    } else {
        *(dst as *mut uint8) = (samp ^ 0x80 as ::core::ffi::c_int) as uint8;
    };
}
#[inline]
unsafe extern "C" fn mixer_PutSampleInt(
    mut dst: *mut ::core::ffi::c_void,
    mut bpc: uint32,
    mut samp: sint32,
) {
    if bpc == 2 as uint32 {
        *(dst as *mut sint16) = samp as sint16;
    } else {
        *(dst as *mut sint8) = samp as sint8;
    };
}
unsafe extern "C" fn mixer_ResampleNone(
    mut src: *mut mixer_Source,
    mut left: bool,
) -> ::core::ffi::c_float {
    let mut d0: *mut uint8 = (*(*src).nextqueued).data.offset((*src).pos as isize);
    (*src).pos = (*src).pos.wrapping_add(mixer_chansize);
    return mixer_GetSampleInt(d0 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
}
unsafe extern "C" fn mixer_ResampleNearest(
    mut src: *mut mixer_Source,
    mut left: bool,
) -> ::core::ffi::c_float {
    let mut d0: *mut uint8 = (*(*src).nextqueued).data.offset((*src).pos as isize);
    d0 = d0.offset(mixer_SourceAdvance(src, left) as isize);
    return mixer_GetSampleInt(d0 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
}
unsafe extern "C" fn mixer_UpsampleLinear(
    mut src: *mut mixer_Source,
    mut left: bool,
) -> ::core::ffi::c_float {
    let mut curr: *mut mixer_Buffer = (*src).nextqueued;
    let mut next: *mut mixer_Buffer = (*(*src).nextqueued).next;
    let mut d0: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut d1: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut s0: ::core::ffi::c_float = 0.;
    let mut s1: ::core::ffi::c_float = 0.;
    let mut t: ::core::ffi::c_float = 0.;
    t = (*src).count as ::core::ffi::c_float / 65536.0f32;
    d0 = (*curr).data.offset((*src).pos as isize);
    d0 = d0.offset(mixer_SourceAdvance(src, left) as isize);
    if d0.offset((*curr).sampsize as isize) >= (*curr).data.offset((*curr).size as isize)
    {
        if !next.is_null() && !(*next).data.is_null() && (*next).size >= (*curr).sampsize
        {
            d1 = (*next).data;
            if !left {
                d1 = d1.offset(mixer_chansize as isize);
            }
        } else {
            d1 = d0;
        }
    } else {
        d1 = d0.offset((*curr).sampsize as isize);
    }
    s0 = mixer_GetSampleInt(d0 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
    s1 = mixer_GetSampleInt(d1 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
    return s0 + t * (s1 - s0);
}
unsafe extern "C" fn mixer_UpsampleCubic(
    mut src: *mut mixer_Source,
    mut left: bool,
) -> ::core::ffi::c_float {
    let mut prev: *mut mixer_Buffer = (*src).prevqueued;
    let mut curr: *mut mixer_Buffer = (*src).nextqueued;
    let mut next: *mut mixer_Buffer = (*(*src).nextqueued).next;
    let mut d0: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut d1: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut d2: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut d3: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut t: ::core::ffi::c_float = 0.;
    let mut t2: ::core::ffi::c_float = 0.;
    let mut a: ::core::ffi::c_float = 0.;
    let mut b: ::core::ffi::c_float = 0.;
    let mut c: ::core::ffi::c_float = 0.;
    let mut s0: ::core::ffi::c_float = 0.;
    let mut s1: ::core::ffi::c_float = 0.;
    let mut s2: ::core::ffi::c_float = 0.;
    let mut s3: ::core::ffi::c_float = 0.;
    t = (*src).count as ::core::ffi::c_float / 65536.0f32;
    t2 = t * t;
    d1 = (*curr).data.offset((*src).pos as isize);
    d1 = d1.offset(mixer_SourceAdvance(src, left) as isize);
    if d1.offset(-((*curr).sampsize as isize)) < (*curr).data {
        if !prev.is_null() && !(*prev).data.is_null() && (*prev).size >= (*curr).sampsize
        {
            d0 = (*prev)
                .data
                .offset((*prev).size as isize)
                .offset(-((*curr).sampsize as isize));
            if !left {
                d0 = d0.offset(mixer_chansize as isize);
            }
        } else {
            d0 = d1;
        }
    } else {
        d0 = d1.offset(-((*curr).sampsize as isize));
    }
    if d1.offset((*curr).sampsize as isize) >= (*curr).data.offset((*curr).size as isize)
    {
        if !next.is_null() && !(*next).data.is_null()
            && (*next).size >= (*curr).sampsize.wrapping_mul(2 as uint32)
        {
            d2 = (*next).data;
            if !left {
                d2 = d2.offset(mixer_chansize as isize);
            }
            d3 = d2.offset((*curr).sampsize as isize);
        } else {
            d3 = d1;
            d2 = d3;
        }
    } else {
        d2 = d1.offset((*curr).sampsize as isize);
        if d2.offset((*curr).sampsize as isize)
            >= (*curr).data.offset((*curr).size as isize)
        {
            if !next.is_null() && !(*next).data.is_null()
                && (*next).size >= (*curr).sampsize
            {
                d3 = (*next).data;
                if !left {
                    d3 = d3.offset(mixer_chansize as isize);
                }
            } else {
                d3 = d2;
            }
        } else {
            d3 = d2.offset((*curr).sampsize as isize);
        }
    }
    s0 = mixer_GetSampleInt(d0 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
    s1 = mixer_GetSampleInt(d1 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
    s2 = mixer_GetSampleInt(d2 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
    s3 = mixer_GetSampleInt(d3 as *mut ::core::ffi::c_void, mixer_chansize)
        as ::core::ffi::c_float;
    a = (3.0f32 * (s1 - s2) - s0 + s3) * 0.5f32;
    b = 2.0f32 * s2 + s0 - (5.0f32 * s1 + s3) * 0.5f32;
    c = (s2 - s0) * 0.5f32;
    return a * t2 * t + b * t2 + c * t + s1;
}
#[inline]
unsafe extern "C" fn mixer_GetConvSample(
    mut psrc: *mut *mut uint8,
    mut bpc: uint32,
    mut flags: uint32,
) -> sint32 {
    let mut samp: sint32 = 0;
    samp = mixer_GetSampleExt(*psrc as *mut ::core::ffi::c_void, bpc);
    *psrc = (*psrc).offset(bpc as isize);
    if flags & mixer_ConvFlags::mixConvStereoDown.0 as ::core::ffi::c_int as uint32 != 0
    {
        samp = ((samp + mixer_GetSampleExt(*psrc as *mut ::core::ffi::c_void, bpc))
            / 2 as ::core::ffi::c_int) as sint32;
        *psrc = (*psrc).offset(bpc as isize);
    }
    if flags & mixer_ConvFlags::mixConvSizeUp.0 as ::core::ffi::c_int as uint32 != 0 {
        samp <<= 8 as ::core::ffi::c_int;
    } else if flags & mixer_ConvFlags::mixConvSizeDown.0 as ::core::ffi::c_int as uint32
        != 0
    {
        samp /= 0x100 as ::core::ffi::c_int;
    }
    return samp;
}
#[inline]
unsafe extern "C" fn mixer_PutConvSample(
    mut pdst: *mut *mut uint8,
    mut bpc: uint32,
    mut flags: uint32,
    mut samp: sint32,
) {
    mixer_PutSampleInt(*pdst as *mut ::core::ffi::c_void, bpc, samp);
    *pdst = (*pdst).offset(bpc as isize);
    if flags & mixer_ConvFlags::mixConvStereoUp.0 as ::core::ffi::c_int as uint32 != 0 {
        mixer_PutSampleInt(*pdst as *mut ::core::ffi::c_void, bpc, samp);
        *pdst = (*pdst).offset(bpc as isize);
    }
}
unsafe extern "C" fn mixer_ResampleFlat(mut conv: *mut mixer_Convertion) {
    let mut flags: mixer_ConvFlags = (*conv).flags;
    let mut src: *mut uint8 = (*conv).srcdata as *mut uint8;
    let mut dst: *mut uint8 = (*conv).dstdata as *mut uint8;
    let mut srcbpc: uint32 = (*conv).srcbpc;
    let mut dstbpc: uint32 = (*conv).dstbpc;
    let mut samples: uint32 = 0;
    samples = (*conv).srcsamples;
    if (*conv).flags.0
        & (mixer_ConvFlags::mixConvStereoUp.0 as ::core::ffi::c_int
            | mixer_ConvFlags::mixConvStereoDown.0 as ::core::ffi::c_int)
            as ::core::ffi::c_uint == 0
    {
        samples = samples.wrapping_mul((*conv).srcchans);
    }
    while samples != 0 {
        let mut samp: sint32 = 0;
        samp = mixer_GetConvSample(&raw mut src, srcbpc, flags.0);
        mixer_PutConvSample(&raw mut dst, dstbpc, flags.0, samp);
        samples = samples.wrapping_sub(1);
    }
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
