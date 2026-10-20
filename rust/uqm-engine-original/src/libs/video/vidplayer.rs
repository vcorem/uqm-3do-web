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

// Rust translation of src/libs/video/vidplayer.c.
extern "C" {
    pub type uio_DirHandle;
    pub type context_desc;
    pub type tfb_soundsample;
    pub type NativePalette;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn BoxIntersect(
        pr1: *mut RECT,
        pr2: *mut RECT,
        printer: *mut RECT,
    ) -> INTERSECT_CODE;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn FlushGraphics();
    fn LoadMusicFile(pStr: *const ::core::ffi::c_char) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn PLRPlaySong(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn PLRStop(MusicRef: MUSIC_REF);
    fn PLRSeek(MusicRef: MUSIC_REF, pos: DWORD);
    fn TaskSwitch();
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn TFB_DrawScreen_Image(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: *mut TFB_ColorMap,
        _: DrawMode,
        dest: SCREEN,
    );
    fn TFB_DrawScreen_DeleteImage(img: *mut TFB_Image);
    fn TFB_DrawImage_CreateForScreen(
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        withalpha: BOOLEAN,
    ) -> *mut TFB_Image;
    fn TFB_DrawCanvas_Lock(canvas: TFB_Canvas);
    fn TFB_DrawCanvas_Unlock(canvas: TFB_Canvas);
    fn TFB_DrawCanvas_GetLine(
        canvas: TFB_Canvas,
        line: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn VideoDecoder_Decode(decoder: *mut TFB_VideoDecoder) -> ::core::ffi::c_int;
    fn VideoDecoder_SeekFrame(
        decoder: *mut TFB_VideoDecoder,
        frame_pos: uint32,
    ) -> uint32;
    fn GetTimeCounter() -> TimeCount;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn TFB_SetSoundSampleData(_: *mut TFB_SoundSample, data: *mut ::core::ffi::c_void);
    fn TFB_GetSoundSampleData(_: *mut TFB_SoundSample) -> *mut ::core::ffi::c_void;
    fn TFB_SetSoundSampleCallbacks(
        _: *mut TFB_SoundSample,
        _: *const TFB_SoundCallbacks,
    );
    fn TFB_GetSoundSampleDecoder(_: *mut TFB_SoundSample) -> *mut TFB_SoundDecoder;
    fn TFB_TagBuffer(
        _: *mut TFB_SoundSample,
        buffer: audio_Object,
        data: intptr_t,
    ) -> bool;
    fn SoundDecoder_GetFrame(decoder: *mut TFB_SoundDecoder) -> uint32;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
pub type CONTEXT_DESC = context_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
pub type INTERSECT_CODE = BYTE;
pub type HOT_SPOT = POINT;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videoclip {
    pub decoder: *mut TFB_VideoDecoder,
    pub length: ::core::ffi::c_float,
    pub w: uint32,
    pub h: uint32,
    pub dst_rect: RECT,
    pub src_rect: RECT,
    pub hAudio: MUSIC_REF,
    pub frame_time: uint32,
    pub frame: *mut TFB_Image,
    pub cur_frame: uint32,
    pub playing: bool,
    pub own_audio: bool,
    pub loop_frame: uint32,
    pub loop_to: uint32,
    pub guard: Mutex,
    pub want_frame: uint32,
    pub lag_cnt: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_image {
    pub NormalImg: TFB_Canvas,
    pub ScaledImg: TFB_Canvas,
    pub MipmapImg: TFB_Canvas,
    pub FilledImg: TFB_Canvas,
    pub colormap_index: ::core::ffi::c_int,
    pub colormap_version: ::core::ffi::c_int,
    pub NormalHs: HOT_SPOT,
    pub MipmapHs: HOT_SPOT,
    pub last_scale_hs: HOT_SPOT,
    pub last_scale: ::core::ffi::c_int,
    pub last_scale_type: ::core::ffi::c_int,
    pub last_fill: Color,
    pub extent: EXTENT,
    pub mutex: Mutex,
    pub dirty: BOOLEAN,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type TFB_VideoDecoder = tfb_videodecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videodecoder {
    pub funcs: *const TFB_VideoDecoderFuncs,
    pub format: *const TFB_PixelFormat,
    pub w: uint32,
    pub h: uint32,
    pub length: ::core::ffi::c_float,
    pub frame_count: uint32,
    pub interframe_wait: uint32,
    pub audio_synced: bool,
    pub callbacks: TFB_VideoCallbacks,
    pub looping: bool,
    pub data: *mut ::core::ffi::c_void,
    pub error: sint32,
    pub pos: ::core::ffi::c_float,
    pub cur_frame: uint32,
    pub dir: *mut uio_DirHandle,
    pub filename: *mut ::core::ffi::c_char,
}
pub type TFB_VideoCallbacks = tfb_videocallbacks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videocallbacks {
    pub BeginFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub EndFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub GetCanvasLine: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> *mut ::core::ffi::c_void,
    >,
    pub GetTicks: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> uint32>,
    pub SetTimer: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> bool>,
}
pub type TFB_PixelFormat = tfb_pixelformat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_pixelformat {
    pub BitsPerPixel: ::core::ffi::c_int,
    pub BytesPerPixel: ::core::ffi::c_int,
    pub Rmask: DWORD,
    pub Gmask: DWORD,
    pub Bmask: DWORD,
    pub Amask: DWORD,
    pub Rshift: DWORD,
    pub Gshift: DWORD,
    pub Bshift: DWORD,
    pub Ashift: DWORD,
    pub Rloss: DWORD,
    pub Gloss: DWORD,
    pub Bloss: DWORD,
    pub Aloss: DWORD,
}
pub type TFB_VideoDecoderFuncs = tfb_videodecoderfunc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_videodecoderfunc {
    pub GetName: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub InitModule: Option<unsafe extern "C" fn(::core::ffi::c_int) -> bool>,
    pub TermModule: Option<unsafe extern "C" fn() -> ()>,
    pub GetStructSize: Option<unsafe extern "C" fn() -> uint32>,
    pub GetError: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_int,
    >,
    pub Init: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder, *mut TFB_PixelFormat) -> bool,
    >,
    pub Term: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub Open: Option<
        unsafe extern "C" fn(
            *mut TFB_VideoDecoder,
            *mut uio_DirHandle,
            *const ::core::ffi::c_char,
        ) -> bool,
    >,
    pub Close: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()>,
    pub DecodeNext: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_int,
    >,
    pub SeekFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> uint32>,
    pub SeekTime: Option<
        unsafe extern "C" fn(
            *mut TFB_VideoDecoder,
            ::core::ffi::c_float,
        ) -> ::core::ffi::c_float,
    >,
    pub GetFrame: Option<unsafe extern "C" fn(*mut TFB_VideoDecoder) -> uint32>,
    pub GetTime: Option<
        unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ::core::ffi::c_float,
    >,
}
pub type VIDEO_REF = *mut tfb_videoclip;
pub type TimeCount = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SCREEN(pub ::core::ffi::c_uint);
impl SCREEN {
    pub const TFB_SCREEN_MAIN: Self = Self(0);
    pub const TFB_SCREEN_EXTRA: Self = Self(1);
    pub const TFB_SCREEN_TRANSITION: Self = Self(2);
    pub const TFB_GFX_NUMSCREENS: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_colormap {
    pub index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut tfb_colormap,
    pub palette: *mut NativePalette,
}
pub type TFB_ColorMap = tfb_colormap;
pub type audio_Object = uintptr_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_SoundTag {
    pub in_use: ::core::ffi::c_int,
    pub buf_name: audio_Object,
    pub data: intptr_t,
}
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
pub type TFB_SoundCallbacks = tfb_soundcallbacks;
pub type TFB_VideoClip = tfb_videoclip;
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
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
static mut vp_DecoderCBs: TFB_VideoCallbacks = tfb_videocallbacks {
    BeginFrame: Some(vp_BeginFrame as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()),
    EndFrame: Some(vp_EndFrame as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> ()),
    GetCanvasLine: Some(
        vp_GetCanvasLine
            as unsafe extern "C" fn(
                *mut TFB_VideoDecoder,
                uint32,
            ) -> *mut ::core::ffi::c_void,
    ),
    GetTicks: Some(vp_GetTicks as unsafe extern "C" fn(*mut TFB_VideoDecoder) -> uint32),
    SetTimer: Some(
        vp_SetTimer as unsafe extern "C" fn(*mut TFB_VideoDecoder, uint32) -> bool,
    ),
};
static mut vp_AudioCBs: TFB_SoundCallbacks = tfb_soundcallbacks {
    OnStartStream: Some(
        vp_AudioStart as unsafe extern "C" fn(*mut TFB_SoundSample) -> bool,
    ),
    OnEndChunk: None,
    OnEndStream: Some(vp_AudioEnd as unsafe extern "C" fn(*mut TFB_SoundSample) -> ()),
    OnTaggedBuffer: Some(
        vp_BufferTag
            as unsafe extern "C" fn(*mut TFB_SoundSample, *mut TFB_SoundTag) -> (),
    ),
    OnQueueBuffer: Some(
        vp_QueueBuffer as unsafe extern "C" fn(*mut TFB_SoundSample, audio_Object) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn TFB_InitVideoPlayer() -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_UninitVideoPlayer() {}
#[inline]
unsafe extern "C" fn msecToTimeCount(mut msec: sint32) -> sint32 {
    return msec * 840 as sint32 / 1000 as sint32;
}
unsafe extern "C" fn processAudioSyncedFrame(mut vid: VIDEO_REF) -> bool {
    let mut ret: ::core::ffi::c_int = 0;
    let mut want_frame: uint32 = 0;
    let mut prev_want_frame: uint32 = 0;
    let mut wait_msec: sint32 = 0;
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut Now: TimeCount = GetTimeCounter();
    if !(*vid).playing {
        return false;
    }
    if Now < (*vid).frame_time {
        return true;
    }
    LockMutex((*vid).guard);
    want_frame = (*vid).want_frame;
    UnlockMutex((*vid).guard);
    if want_frame >= (*(*vid).decoder).frame_count {
        (*vid).playing = false;
        return false;
    }
    prev_want_frame = (*vid).cur_frame.wrapping_sub((*vid).lag_cnt as uint32);
    if want_frame > prev_want_frame.wrapping_sub(8 as uint32)
        && want_frame <= prev_want_frame.wrapping_add(8 as uint32)
    {
        (*vid).lag_cnt = (*vid)
            .cur_frame
            .wrapping_add(1 as uint32)
            .wrapping_sub(want_frame) as ::core::ffi::c_int;
    } else {
        (*vid).lag_cnt = 0 as ::core::ffi::c_int;
        (*vid).cur_frame = VideoDecoder_SeekFrame((*vid).decoder, want_frame);
        ret = VideoDecoder_Decode((*vid).decoder);
        if ret < 0 as ::core::ffi::c_int {
            (*vid).playing = false;
            return false;
        }
    }
    (*vid).cur_frame = (*(*vid).decoder).cur_frame;
    oldContext = SetContext(::core::ptr::null_mut::<CONTEXT_DESC>());
    TFB_DrawScreen_Image(
        (*vid).frame,
        (*vid).dst_rect.corner.x as ::core::ffi::c_int,
        (*vid).dst_rect.corner.y as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<TFB_ColorMap>(),
        MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
        SCREEN::TFB_SCREEN_MAIN,
    );
    SetContext(oldContext);
    FlushGraphics();
    wait_msec = (*(*vid).decoder)
        .interframe_wait
        .wrapping_sub(
            ((*(*vid).decoder).interframe_wait as ::core::ffi::c_int
                * 1 as ::core::ffi::c_int / 3 as ::core::ffi::c_int) as uint32,
        )
        .wrapping_add(
            ((*(*vid).decoder).interframe_wait as ::core::ffi::c_int * (*vid).lag_cnt
                / 6 as ::core::ffi::c_int) as uint32,
        ) as sint32;
    (*vid).frame_time = Now.wrapping_add(msecToTimeCount(wait_msec) as TimeCount)
        as uint32;
    ret = VideoDecoder_Decode((*vid).decoder);
    ret < 0 as ::core::ffi::c_int;
    return (*vid).playing;
}
unsafe extern "C" fn processMuteFrame(mut vid: VIDEO_REF) -> bool {
    let mut ret: ::core::ffi::c_int = 0;
    let mut Now: TimeCount = GetTimeCounter();
    if !(*vid).playing {
        return false;
    }
    if Now >= (*vid).frame_time {
        let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        (*vid).cur_frame = (*(*vid).decoder).cur_frame;
        oldContext = SetContext(::core::ptr::null_mut::<CONTEXT_DESC>());
        TFB_DrawScreen_Image(
            (*vid).frame,
            (*vid).dst_rect.corner.x as ::core::ffi::c_int,
            (*vid).dst_rect.corner.y as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<TFB_ColorMap>(),
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD),
            SCREEN::TFB_SCREEN_MAIN,
        );
        SetContext(oldContext);
        FlushGraphics();
        if (*vid).cur_frame == (*vid).loop_frame {
            VideoDecoder_SeekFrame((*vid).decoder, (*vid).loop_to);
        }
        ret = VideoDecoder_Decode((*vid).decoder);
        if ret <= 0 as ::core::ffi::c_int {
            (*vid).playing = false;
        }
    }
    return (*vid).playing;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_PlayVideo(
    mut vid: VIDEO_REF,
    mut x: uint32,
    mut y: uint32,
) -> bool {
    let mut scrn_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut clip_r: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: (*vid).w as COORD,
            height: (*vid).h as COORD,
        },
    };
    let mut vid_r: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: ScreenWidth as COORD,
            height: ScreenHeight as COORD,
        },
    };
    let mut dr: RECT = rect {
        corner: point {
            x: x as COORD,
            y: y as COORD,
        },
        extent: extent {
            width: (*vid).w as COORD,
            height: (*vid).h as COORD,
        },
    };
    let mut sr: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut loop_music: bool = false;
    let mut ret: ::core::ffi::c_int = 0;
    if vid.is_null() {
        return false;
    }
    GetContextClipRect(&raw mut scrn_r);
    if BoxIntersect(&raw mut scrn_r, &raw mut vid_r, &raw mut scrn_r) == 0 {
        return false;
    }
    sr = dr;
    sr.corner.x = -(sr.corner.x as ::core::ffi::c_int) as COORD;
    sr.corner.y = -(sr.corner.y as ::core::ffi::c_int) as COORD;
    if BoxIntersect(&raw mut clip_r, &raw mut sr, &raw mut sr) == 0 {
        return false;
    }
    dr.corner.x = (dr.corner.x as ::core::ffi::c_int
        + scrn_r.corner.x as ::core::ffi::c_int) as COORD;
    dr.corner.y = (dr.corner.y as ::core::ffi::c_int
        + scrn_r.corner.y as ::core::ffi::c_int) as COORD;
    if BoxIntersect(&raw mut scrn_r, &raw mut dr, &raw mut (*vid).dst_rect) == 0 {
        return false;
    }
    (*vid).src_rect = (*vid).dst_rect;
    (*vid).src_rect.corner.x = sr.corner.x;
    (*vid).src_rect.corner.y = sr.corner.y;
    (*(*vid).decoder).callbacks = vp_DecoderCBs;
    (*(*vid).decoder).data = vid as *mut ::core::ffi::c_void;
    (*vid).frame = TFB_DrawImage_CreateForScreen(
        (*vid).w as ::core::ffi::c_int,
        (*vid).h as ::core::ffi::c_int,
        BOOLEAN::FALSE,
    );
    (*vid).cur_frame = -1 as ::core::ffi::c_int as uint32;
    (*vid).want_frame = -1 as ::core::ffi::c_int as uint32;
    if (*vid).hAudio.is_null() {
        (*vid).hAudio = LoadMusicFile((*(*vid).decoder).filename);
        (*vid).own_audio = true;
    }
    if (*(*vid).decoder).audio_synced {
        if (*vid).hAudio.is_null() {
            log_add(
                log_Level::log_Warning,
                b"TFB_PlayVideo: Cannot load sound-track for audio-synced video\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        TFB_SetSoundSampleCallbacks(*(*vid).hAudio, &raw const vp_AudioCBs);
        TFB_SetSoundSampleData(*(*vid).hAudio, vid as *mut ::core::ffi::c_void);
    }
    ret = VideoDecoder_Decode((*vid).decoder);
    if ret < 0 as ::core::ffi::c_int {
        return false;
    }
    (*vid).playing = true;
    loop_music = !(*(*vid).decoder).audio_synced
        && (*vid).loop_frame
            != (0 as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    if !(*vid).hAudio.is_null() {
        PLRPlaySong(
            (*vid).hAudio,
            BOOLEAN(loop_music as ::core::ffi::c_uint),
            1 as BYTE,
        );
    }
    if (*(*vid).decoder).audio_synced {
        (*vid).frame_time = GetTimeCounter() as uint32;
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_StopVideo(mut vid: VIDEO_REF) {
    if vid.is_null() {
        return;
    }
    (*vid).playing = false;
    if !(*vid).hAudio.is_null() {
        PLRStop((*vid).hAudio);
        if (*vid).own_audio {
            DestroyMusic((*vid).hAudio);
            (*vid).hAudio = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
            (*vid).own_audio = false;
        }
    }
    if !(*vid).frame.is_null() {
        TFB_DrawScreen_DeleteImage((*vid).frame);
        (*vid).frame = ::core::ptr::null_mut::<TFB_Image>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_VideoPlaying(mut vid: VIDEO_REF) -> bool {
    if vid.is_null() {
        return false;
    }
    return (*vid).playing;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_ProcessVideoFrame(mut vid: VIDEO_REF) -> bool {
    if vid.is_null() {
        return false;
    }
    if (*(*vid).decoder).audio_synced {
        return processAudioSyncedFrame(vid)
    } else {
        return processMuteFrame(vid)
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetVideoPosition(mut vid: VIDEO_REF) -> uint32 {
    let mut pos: uint32 = 0;
    if !TFB_VideoPlaying(vid) {
        return 0 as uint32;
    }
    LockMutex((*vid).guard);
    pos = ((*(*vid).decoder).pos * 1000 as ::core::ffi::c_int as ::core::ffi::c_float)
        as uint32;
    UnlockMutex((*vid).guard);
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SeekVideo(mut vid: VIDEO_REF, mut pos: uint32) -> bool {
    if !TFB_VideoPlaying(vid) {
        return false;
    }
    if (*(*vid).decoder).audio_synced {
        PLRSeek((*vid).hAudio, pos);
        TaskSwitch();
        return true;
    } else {
        return false
    };
}
unsafe extern "C" fn vp_BeginFrame(mut decoder: *mut TFB_VideoDecoder) {
    let mut vid: *mut TFB_VideoClip = (*decoder).data as *mut TFB_VideoClip;
    if !vid.is_null() {
        TFB_DrawCanvas_Lock((*(*vid).frame).NormalImg);
    }
}
unsafe extern "C" fn vp_EndFrame(mut decoder: *mut TFB_VideoDecoder) {
    let mut vid: *mut TFB_VideoClip = (*decoder).data as *mut TFB_VideoClip;
    if !vid.is_null() {
        TFB_DrawCanvas_Unlock((*(*vid).frame).NormalImg);
    }
}
unsafe extern "C" fn vp_GetCanvasLine(
    mut decoder: *mut TFB_VideoDecoder,
    mut line: uint32,
) -> *mut ::core::ffi::c_void {
    let mut vid: *mut TFB_VideoClip = (*decoder).data as *mut TFB_VideoClip;
    if vid.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return TFB_DrawCanvas_GetLine((*(*vid).frame).NormalImg, line as ::core::ffi::c_int);
}
unsafe extern "C" fn vp_GetTicks(mut decoder: *mut TFB_VideoDecoder) -> uint32 {
    let mut ctr: uint32 = GetTimeCounter();
    return ctr
        .wrapping_div(840 as uint32)
        .wrapping_mul(1000 as uint32)
        .wrapping_add(
            ctr
                .wrapping_rem(840 as uint32)
                .wrapping_mul(1000 as uint32)
                .wrapping_div(840 as uint32),
        );
}
unsafe extern "C" fn vp_SetTimer(
    mut decoder: *mut TFB_VideoDecoder,
    mut msecs: uint32,
) -> bool {
    let mut vid: *mut TFB_VideoClip = (*decoder).data as *mut TFB_VideoClip;
    if vid.is_null() {
        return false;
    }
    (*vid).frame_time = GetTimeCounter()
        .wrapping_add(msecs.wrapping_mul(840 as uint32).wrapping_div(1000 as uint32));
    return true;
}
unsafe extern "C" fn vp_AudioStart(mut sample: *mut TFB_SoundSample) -> bool {
    let mut vid: *mut TFB_VideoClip = TFB_GetSoundSampleData(sample)
        as *mut TFB_VideoClip;
    let mut decoder: *mut TFB_SoundDecoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
    (::core::mem::size_of::<intptr_t>() >= ::core::mem::size_of::<*mut TFB_VideoClip>()
        || {
            __assert_fail(
                b"sizeof (intptr_t) >= sizeof (vid)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/video/vidplayer.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                434 as ::core::ffi::c_int,
                b"vp_AudioStart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!vid.is_null()
        || {
            __assert_fail(
                b"vid != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/video/vidplayer.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                435 as ::core::ffi::c_int,
                b"vp_AudioStart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    decoder = TFB_GetSoundSampleDecoder(sample);
    LockMutex((*vid).guard);
    (*vid).want_frame = SoundDecoder_GetFrame(decoder);
    UnlockMutex((*vid).guard);
    return true;
}
unsafe extern "C" fn vp_AudioEnd(mut sample: *mut TFB_SoundSample) {
    let mut vid: *mut TFB_VideoClip = TFB_GetSoundSampleData(sample)
        as *mut TFB_VideoClip;
    (!vid.is_null()
        || {
            __assert_fail(
                b"vid != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/video/vidplayer.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                451 as ::core::ffi::c_int,
                b"vp_AudioEnd\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    LockMutex((*vid).guard);
    (*vid).want_frame = (*(*vid).decoder).frame_count;
    UnlockMutex((*vid).guard);
}
unsafe extern "C" fn vp_BufferTag(
    mut sample: *mut TFB_SoundSample,
    mut tag: *mut TFB_SoundTag,
) {
    let mut vid: *mut TFB_VideoClip = TFB_GetSoundSampleData(sample)
        as *mut TFB_VideoClip;
    let mut frame: uint32 = (*tag).data as uint32;
    (::core::mem::size_of::<intptr_t>() >= ::core::mem::size_of::<uint32>()
        || {
            __assert_fail(
                b"sizeof (tag->data) >= sizeof (frame)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/video/vidplayer.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                464 as ::core::ffi::c_int,
                b"vp_BufferTag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!vid.is_null()
        || {
            __assert_fail(
                b"vid != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/video/vidplayer.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                465 as ::core::ffi::c_int,
                b"vp_BufferTag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    LockMutex((*vid).guard);
    (*vid).want_frame = frame;
    UnlockMutex((*vid).guard);
}
unsafe extern "C" fn vp_QueueBuffer(
    mut sample: *mut TFB_SoundSample,
    mut buffer: audio_Object,
) {
    let mut decoder: *mut TFB_SoundDecoder = TFB_GetSoundSampleDecoder(sample);
    TFB_TagBuffer(sample, buffer, SoundDecoder_GetFrame(decoder) as intptr_t);
}
