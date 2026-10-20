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

// Rust translation of src/libs/sound/resinst.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table;
    fn LoadResourceFromPath(
        pathname: *const ::core::ffi::c_char,
        r#fn: Option<ResourceLoadFileFun>,
    ) -> *mut ::core::ffi::c_void;
    fn InstallResTypeVectors(
        res_type: *const ::core::ffi::c_char,
        loadFun: Option<ResourceLoadFun>,
        freeFun: Option<ResourceFreeFun>,
        stringFun: Option<ResourceStringFun>,
    ) -> BOOLEAN;
    fn res_GetResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn res_DetachResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn _GetMusicData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _ReleaseMusicData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn _GetSoundBankData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _ReleaseSoundBankData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
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
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub union RESOURCE_DATA {
    pub num: DWORD,
    pub ptr: *mut ::core::ffi::c_void,
    pub str: *const ::core::ffi::c_char,
}
pub type ResourceLoadFun = unsafe extern "C" fn(
    *const ::core::ffi::c_char,
    *mut RESOURCE_DATA,
) -> ();
pub type ResourceFreeFun = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN;
pub type ResourceStringFun = unsafe extern "C" fn(
    *mut RESOURCE_DATA,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uint,
) -> ();
pub type ResourceLoadFileFun = unsafe extern "C" fn(
    *mut uio_Stream,
    DWORD,
) -> *mut ::core::ffi::c_void;
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
unsafe extern "C" fn GetSoundBankFileData(
    mut pathname: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).ptr = uqm_lua_checked!(LoadResourceFromPath(
        pathname,
        Some(
            _GetSoundBankData
                as unsafe extern "C" fn(
                    *mut uio_Stream,
                    DWORD,
                ) -> *mut ::core::ffi::c_void,
        ),
    ), ());
}
unsafe extern "C" fn GetMusicFileData(
    mut pathname: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).ptr = uqm_lua_checked!(LoadResourceFromPath(
        pathname,
        Some(
            _GetMusicData
                as unsafe extern "C" fn(
                    *mut uio_Stream,
                    DWORD,
                ) -> *mut ::core::ffi::c_void,
        ),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn InstallAudioResTypes() -> BOOLEAN {
    InstallResTypeVectors(
        b"SNDRES\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetSoundBankFileData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            _ReleaseSoundBankData
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    InstallResTypeVectors(
        b"MUSICRES\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetMusicFileData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            _ReleaseMusicData
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadSoundInstance(mut res: RESOURCE) -> SOUND_REF {
    let mut hData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hData = uqm_lua_checked!(res_GetResource(res), ::core::ptr::null_mut());
    if !hData.is_null() {
        res_DetachResource(res);
    }
    return hData as SOUND_REF;
}
#[no_mangle]
pub unsafe extern "C" fn LoadMusicInstance(mut res: RESOURCE) -> MUSIC_REF {
    let mut hData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hData = uqm_lua_checked!(res_GetResource(res), ::core::ptr::null_mut());
    if !hData.is_null() {
        res_DetachResource(res);
    }
    return hData as MUSIC_REF;
}
