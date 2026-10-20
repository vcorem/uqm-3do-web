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

// Rust translation of src/libs/video/video.c.
extern "C" {
    pub type uio_DirHandle;
    pub type tfb_soundsample;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn snd_PlaySpeech(SpeechRef: MUSIC_REF);
    fn snd_StopSpeech();
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn TFB_DrawCanvas_GetScreenFormat(fmt: *mut TFB_PixelFormat);
    fn VideoDecoder_Init(
        flags: ::core::ffi::c_int,
        depth: ::core::ffi::c_int,
        Rmask: uint32,
        Gmask: uint32,
        Bmask: uint32,
        Amask: uint32,
    ) -> bool;
    fn VideoDecoder_Uninit();
    fn VideoDecoder_Load(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
    ) -> *mut TFB_VideoDecoder;
    fn VideoDecoder_Free(decoder: *mut TFB_VideoDecoder);
    static mut contentDir: *mut uio_DirHandle;
    fn TFB_InitVideoPlayer() -> bool;
    fn TFB_UninitVideoPlayer();
    fn TFB_PlayVideo(VidRef: VIDEO_REF, x: uint32, y: uint32) -> bool;
    fn TFB_StopVideo(VidRef: VIDEO_REF);
    fn TFB_VideoPlaying(VidRef: VIDEO_REF) -> bool;
    fn TFB_ProcessVideoFrame(vid: VIDEO_REF) -> bool;
    fn TFB_GetVideoPosition(VidRef: VIDEO_REF) -> uint32;
    fn TFB_SeekVideo(VidRef: VIDEO_REF, pos: uint32) -> bool;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
}
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub type HOT_SPOT = POINT;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VIDEO_TYPE(pub ::core::ffi::c_uint);
impl VIDEO_TYPE {
    pub const NO_FMV: Self = Self(0);
    pub const HARDWARE_FMV: Self = Self(1);
    pub const SOFTWARE_FMV: Self = Self(2);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
pub type TFB_VideoClip = tfb_videoclip;
static mut _cur_video: VIDEO_REF = ::core::ptr::null_mut::<tfb_videoclip>();
static mut _cur_speech: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
#[no_mangle]
pub unsafe extern "C" fn InitVideoPlayer(mut useCDROM: BOOLEAN) -> BOOLEAN {
    let mut fmt: TFB_PixelFormat = TFB_PixelFormat {
        BitsPerPixel: 0,
        BytesPerPixel: 0,
        Rmask: 0,
        Gmask: 0,
        Bmask: 0,
        Amask: 0,
        Rshift: 0,
        Gshift: 0,
        Bshift: 0,
        Ashift: 0,
        Rloss: 0,
        Gloss: 0,
        Bloss: 0,
        Aloss: 0,
    };
    TFB_DrawCanvas_GetScreenFormat(&raw mut fmt);
    if !VideoDecoder_Init(
        0 as ::core::ffi::c_int,
        fmt.BitsPerPixel,
        fmt.Rmask,
        fmt.Gmask,
        fmt.Bmask,
        0 as uint32,
    ) {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(TFB_InitVideoPlayer() as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn UninitVideoPlayer() {
    TFB_UninitVideoPlayer();
    VideoDecoder_Uninit();
}
#[no_mangle]
pub unsafe extern "C" fn VidStop() {
    if !_cur_speech.is_null() {
        snd_StopSpeech();
    }
    if !_cur_video.is_null() {
        TFB_StopVideo(_cur_video);
    }
    _cur_speech = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    _cur_video = ::core::ptr::null_mut::<tfb_videoclip>();
}
#[no_mangle]
pub unsafe extern "C" fn VidPlaying() -> VIDEO_REF {
    if _cur_video.is_null() {
        return ::core::ptr::null_mut::<tfb_videoclip>();
    }
    if TFB_VideoPlaying(_cur_video) {
        return _cur_video;
    }
    return ::core::ptr::null_mut::<tfb_videoclip>();
}
#[no_mangle]
pub unsafe extern "C" fn VidProcessFrame() -> BOOLEAN {
    if _cur_video.is_null() {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(TFB_ProcessVideoFrame(_cur_video) as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VidGetPosition() -> DWORD {
    if VidPlaying().is_null() {
        return 0 as DWORD;
    }
    return TFB_GetVideoPosition(_cur_video);
}
#[no_mangle]
pub unsafe extern "C" fn VidSeek(mut pos: DWORD) -> BOOLEAN {
    if VidPlaying().is_null() {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(TFB_SeekVideo(_cur_video, pos) as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VidPlayEx(
    mut vid: VIDEO_REF,
    mut AudRef: MUSIC_REF,
    mut SpeechRef: MUSIC_REF,
    mut LoopFrame: DWORD,
) -> VIDEO_TYPE {
    let mut ret: VIDEO_TYPE = VIDEO_TYPE::NO_FMV;
    if vid.is_null() {
        return VIDEO_TYPE::NO_FMV;
    }
    if !AudRef.is_null() {
        if !(*vid).hAudio.is_null() {
            DestroyMusic((*vid).hAudio);
        }
        (*vid).hAudio = AudRef;
        (*(*vid).decoder).audio_synced = BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0;
    }
    (*vid).loop_frame = LoopFrame as uint32;
    (*vid).loop_to = 0 as uint32;
    if !_cur_speech.is_null() {
        snd_StopSpeech();
    }
    if !_cur_video.is_null() {
        TFB_StopVideo(_cur_video);
    }
    _cur_speech = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    _cur_video = ::core::ptr::null_mut::<tfb_videoclip>();
    if TFB_PlayVideo(
        vid,
        (ScreenWidth as uint32).wrapping_sub((*vid).w).wrapping_div(2 as uint32),
        (ScreenHeight as uint32).wrapping_sub((*vid).h).wrapping_div(2 as uint32),
    ) {
        _cur_video = vid;
        ret = VIDEO_TYPE::SOFTWARE_FMV;
        if !SpeechRef.is_null() {
            snd_PlaySpeech(SpeechRef);
            _cur_speech = SpeechRef;
        }
    } else {
        ret = VIDEO_TYPE::NO_FMV;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn VidPlay(mut VidRef: VIDEO_REF) -> VIDEO_TYPE {
    return VidPlayEx(
        VidRef,
        ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        (0 as DWORD).wrapping_sub(1 as DWORD),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _init_video_file(
    mut pStr: *const ::core::ffi::c_char,
) -> VIDEO_REF {
    let mut vid: *mut TFB_VideoClip = ::core::ptr::null_mut::<TFB_VideoClip>();
    let mut dec: *mut TFB_VideoDecoder = ::core::ptr::null_mut::<TFB_VideoDecoder>();
    dec = VideoDecoder_Load(contentDir, pStr);
    if dec.is_null() {
        return ::core::ptr::null_mut::<tfb_videoclip>();
    }
    vid = HCalloc(::core::mem::size_of::<TFB_VideoClip>()) as *mut TFB_VideoClip;
    (*vid).decoder = dec;
    (*vid).length = (*dec).length;
    (*vid).w = (*(*vid).decoder).w;
    (*vid).h = (*(*vid).decoder).h;
    (*vid).guard = CreateMutex_Core(
        b"video guard\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int as DWORD,
    );
    return vid;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyVideo(mut vid: VIDEO_REF) -> BOOLEAN {
    if vid.is_null() {
        return BOOLEAN::FALSE;
    }
    TFB_StopVideo(vid);
    VideoDecoder_Free((*vid).decoder);
    DestroyMutex((*vid).guard);
    HFree(vid as *mut ::core::ffi::c_void);
    return BOOLEAN::TRUE;
}
