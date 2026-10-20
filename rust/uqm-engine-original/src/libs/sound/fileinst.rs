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

// Rust translation of src/libs/sound/fileinst.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table;
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    static mut contentDir: *mut uio_DirHandle;
    fn _GetMusicData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _GetSoundBankData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn CheckMusicResName(filename: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type audio_Object = uintptr_t;
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
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type SOUND_REF = STRING_TABLE;
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
#[no_mangle]
pub unsafe extern "C" fn LoadSoundFile(
    mut pStr: *const ::core::ffi::c_char,
) -> SOUND_REF {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    if !_cur_resfile_name.is_null() {
        return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
    }
    fp = res_OpenResFile(
        contentDir,
        pStr,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut hData: SOUND_REF = ::core::ptr::null_mut::<STRING_TABLE_DESC>();
        _cur_resfile_name = pStr;
        hData = _GetSoundBankData(fp, LengthResFile(fp) as DWORD) as SOUND_REF;
        _cur_resfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        res_CloseResFile(fp);
        return hData;
    }
    return ::core::ptr::null_mut::<STRING_TABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn LoadMusicFile(
    mut pStr: *const ::core::ffi::c_char,
) -> MUSIC_REF {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut filename: [::core::ffi::c_char; 256] = [0; 256];
    if !_cur_resfile_name.is_null() {
        return ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    }
    strncpy(
        &raw mut filename as *mut ::core::ffi::c_char,
        pStr,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(1 as size_t),
    );
    filename[::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        .wrapping_sub(1usize)] = '\0' as ::core::ffi::c_char;
    CheckMusicResName(&raw mut filename as *mut ::core::ffi::c_char);
    fp = res_OpenResFile(
        contentDir,
        &raw mut filename as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut hData: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        _cur_resfile_name = &raw mut filename as *mut ::core::ffi::c_char;
        hData = _GetMusicData(fp, LengthResFile(fp) as DWORD) as MUSIC_REF;
        _cur_resfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        res_CloseResFile(fp);
        return hData;
    }
    return ::core::ptr::null_mut::<*mut TFB_SoundSample>();
}
