// Copyright 2008 Michael Martin

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

// Rust translation of src/libs/video/vresins.c.
extern "C" {
    fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn InstallResTypeVectors(
        res_type: *const ::core::ffi::c_char,
        loadFun: Option<ResourceLoadFun>,
        freeFun: Option<ResourceFreeFun>,
        stringFun: Option<ResourceStringFun>,
    ) -> BOOLEAN;
    fn res_GetResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn res_DetachResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct legacy_video_desc {
    pub video: *mut ::core::ffi::c_char,
    pub audio: *mut ::core::ffi::c_char,
    pub speech: *mut ::core::ffi::c_char,
    pub r#loop: uint32,
}
pub type LEGACY_VIDEO = *mut legacy_video_desc;
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
pub type LEGACY_VIDEO_DESC = legacy_video_desc;
unsafe extern "C" fn FreeLegacyVideoData(mut data: *mut ::core::ffi::c_void) -> BOOLEAN {
    let mut pLV: LEGACY_VIDEO = ::core::ptr::null_mut::<legacy_video_desc>();
    if data.is_null() {
        return BOOLEAN::FALSE;
    }
    pLV = data as LEGACY_VIDEO;
    if !(*pLV).video.is_null() {
        HFree((*pLV).video as *mut ::core::ffi::c_void);
    }
    if !(*pLV).audio.is_null() {
        HFree((*pLV).audio as *mut ::core::ffi::c_void);
    }
    if !(*pLV).speech.is_null() {
        HFree((*pLV).speech as *mut ::core::ffi::c_void);
    }
    HFree(pLV as *mut ::core::ffi::c_void);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn GetLegacyVideoData(
    mut path: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut paths: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut audio_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut speech_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut loop_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut LoopFrame: uint32 = (0 as uint32).wrapping_sub(1 as uint32);
    strncpy(&raw mut paths as *mut ::core::ffi::c_char, path, 1023 as size_t);
    paths[1023usize] = '\0' as ::core::ffi::c_char;
    audio_path = strchr(
        &raw mut paths as *mut ::core::ffi::c_char,
        ':' as ::core::ffi::c_int,
    );
    if audio_path.is_null() {
        speech_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        loop_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        *audio_path = '\0' as ::core::ffi::c_char;
        audio_path = audio_path.offset(1);
        speech_path = strchr(audio_path, ':' as ::core::ffi::c_int);
        if speech_path.is_null() {
            loop_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            *speech_path = '\0' as ::core::ffi::c_char;
            speech_path = speech_path.offset(1);
            loop_str = strchr(speech_path, ':' as ::core::ffi::c_int);
            if !loop_str.is_null() {
                *loop_str = '\0' as ::core::ffi::c_char;
                loop_str = loop_str.offset(1);
            }
        }
    }
    log_add(
        log_Level::log_Info,
        b"\t'%s' -- video\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut paths as *mut ::core::ffi::c_char,
    );
    if !audio_path.is_null() {
        log_add(
            log_Level::log_Info,
            b"\t'%s' -- audio\0".as_ptr() as *const ::core::ffi::c_char,
            audio_path,
        );
    } else {
        log_add(
            log_Level::log_Info,
            b"\tNo associated audio\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !speech_path.is_null() {
        log_add(
            log_Level::log_Info,
            b"\t'%s' -- speech path\0".as_ptr() as *const ::core::ffi::c_char,
            speech_path,
        );
    } else {
        log_add(
            log_Level::log_Info,
            b"\tNo associated speech\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !loop_str.is_null() {
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        LoopFrame = strtol(loop_str, &raw mut end, 10 as ::core::ffi::c_int) as uint32;
        if *end as ::core::ffi::c_int > 32 as ::core::ffi::c_int {
            log_add(
                log_Level::log_Warning,
                b"Warning: Unparsable loop frame '%s'. Disabling loop.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                loop_str,
            );
            LoopFrame = (0 as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                as uint32;
        }
        log_add(
            log_Level::log_Info,
            b"\tLoop frame is %u\0".as_ptr() as *const ::core::ffi::c_char,
            LoopFrame,
        );
    } else {
        log_add(
            log_Level::log_Info,
            b"\tNo specified loop frame\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    result = HMalloc(::core::mem::size_of::<LEGACY_VIDEO_DESC>() as size_t);
    's_248: {
        if !result.is_null() {
            let mut pLV: LEGACY_VIDEO = result as LEGACY_VIDEO;
            let mut len: ::core::ffi::c_int = 0;
            (*pLV).video = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pLV).audio = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pLV).speech = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pLV).r#loop = LoopFrame;
            len = strlen(&raw mut paths as *mut ::core::ffi::c_char)
                .wrapping_add(1 as size_t) as ::core::ffi::c_int;
            (*pLV).video = HMalloc(len as size_t) as *mut ::core::ffi::c_char;
            '_err: {
                if (*pLV).video.is_null() {
                    log_add(
                        log_Level::log_Warning,
                        b"Warning: Couldn't allocate space for '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut paths as *mut ::core::ffi::c_char,
                    );
                } else {
                    strncpy(
                        (*pLV).video,
                        &raw mut paths as *mut ::core::ffi::c_char,
                        len as size_t,
                    );
                    if !audio_path.is_null() {
                        len = strlen(audio_path).wrapping_add(1 as size_t)
                            as ::core::ffi::c_int;
                        (*pLV).audio = HMalloc(len as size_t)
                            as *mut ::core::ffi::c_char;
                        if (*pLV).audio.is_null() {
                            log_add(
                                log_Level::log_Warning,
                                b"Warning: Couldn't allocate space for '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                audio_path,
                            );
                            break '_err;
                        } else {
                            strncpy((*pLV).audio, audio_path, len as size_t);
                        }
                    }
                    if !speech_path.is_null() {
                        len = strlen(speech_path).wrapping_add(1 as size_t)
                            as ::core::ffi::c_int;
                        (*pLV).speech = HMalloc(len as size_t)
                            as *mut ::core::ffi::c_char;
                        if (*pLV).speech.is_null() {
                            log_add(
                                log_Level::log_Warning,
                                b"Warning: Couldn't allocate space for '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                speech_path,
                            );
                            break '_err;
                        } else {
                            strncpy((*pLV).speech, speech_path, len as size_t);
                        }
                    }
                    (*resdata).ptr = result;
                    break 's_248;
                }
            }
            if !result.is_null() {
                FreeLegacyVideoData(result as LEGACY_VIDEO as *mut ::core::ffi::c_void);
            }
            (*resdata).ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn InstallVideoResType() -> BOOLEAN {
    InstallResTypeVectors(
        b"3DOVID\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetLegacyVideoData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            FreeLegacyVideoData
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadLegacyVideoInstance(mut res: RESOURCE) -> LEGACY_VIDEO {
    let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    data = res_GetResource(res);
    if !data.is_null() {
        res_DetachResource(res);
    }
    return data as LEGACY_VIDEO;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyLegacyVideo(mut vid: LEGACY_VIDEO) -> BOOLEAN {
    return FreeLegacyVideoData(vid as *mut ::core::ffi::c_void);
}
