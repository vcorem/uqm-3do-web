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

// Rust translation of src/libs/video/legacyplayer.c.
extern "C" {
    pub type uio_DirHandle;
    pub type tfb_soundsample;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn LoadMusicFile(pStr: *const ::core::ffi::c_char) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn LoadVideoFile(pStr: *const ::core::ffi::c_char) -> VIDEO_REF;
    fn DestroyVideo(VidRef: VIDEO_REF) -> BOOLEAN;
    fn VidPlayEx(
        VidRef: VIDEO_REF,
        AudRef: MUSIC_REF,
        SpeechRef: MUSIC_REF,
        LoopFrame: DWORD,
    ) -> VIDEO_TYPE;
    fn VidStop();
    fn TFB_VideoPlaying(VidRef: VIDEO_REF) -> bool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct legacy_video_desc {
    pub video: *mut ::core::ffi::c_char,
    pub audio: *mut ::core::ffi::c_char,
    pub speech: *mut ::core::ffi::c_char,
    pub r#loop: uint32,
}
pub type LEGACY_VIDEO = *mut legacy_video_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct legacy_video_ref {
    pub vidref: VIDEO_REF,
    pub audref: MUSIC_REF,
    pub speechref: MUSIC_REF,
}
pub type LEGACY_VIDEO_REF = *mut legacy_video_ref;
#[no_mangle]
pub unsafe extern "C" fn PlayLegacyVideo(mut vid: LEGACY_VIDEO) -> LEGACY_VIDEO_REF {
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut audname: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut speechname: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut loopframe: uint32 = 0;
    let mut r#ref: LEGACY_VIDEO_REF = ::core::ptr::null_mut::<legacy_video_ref>();
    let mut r#type: VIDEO_TYPE = VIDEO_TYPE::NO_FMV;
    if vid.is_null() {
        return ::core::ptr::null_mut::<legacy_video_ref>();
    }
    r#ref = HCalloc(::core::mem::size_of::<legacy_video_ref>()) as LEGACY_VIDEO_REF;
    if r#ref.is_null() {
        return ::core::ptr::null_mut::<legacy_video_ref>();
    }
    name = (*vid).video;
    audname = (*vid).audio;
    speechname = (*vid).speech;
    loopframe = (*vid).r#loop;
    (*r#ref).vidref = LoadVideoFile(name);
    if (*r#ref).vidref.is_null() {
        return ::core::ptr::null_mut::<legacy_video_ref>();
    }
    if !audname.is_null() {
        (*r#ref).audref = LoadMusicFile(audname);
    }
    if !speechname.is_null() {
        (*r#ref).speechref = LoadMusicFile(speechname);
    }
    r#type = VidPlayEx((*r#ref).vidref, (*r#ref).audref, (*r#ref).speechref, loopframe);
    if r#type.0 == VIDEO_TYPE::NO_FMV.0 {
        StopLegacyVideo(r#ref);
        return ::core::ptr::null_mut::<legacy_video_ref>();
    }
    return r#ref;
}
#[no_mangle]
pub unsafe extern "C" fn StopLegacyVideo(mut r#ref: LEGACY_VIDEO_REF) {
    if r#ref.is_null() {
        return;
    }
    VidStop();
    DestroyVideo((*r#ref).vidref);
    if !(*r#ref).speechref.is_null() {
        DestroyMusic((*r#ref).speechref);
    }
    if !(*r#ref).audref.is_null() {
        DestroyMusic((*r#ref).audref);
    }
    HFree(r#ref as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn PlayingLegacyVideo(mut r#ref: LEGACY_VIDEO_REF) -> BOOLEAN {
    if r#ref.is_null() {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(TFB_VideoPlaying((*r#ref).vidref) as ::core::ffi::c_uint);
}
