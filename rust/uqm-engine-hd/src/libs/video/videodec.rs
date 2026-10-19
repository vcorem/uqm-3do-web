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

// Rust translation of src/libs/video/videodec.c.
extern "C" {
    pub type uio_DirHandle;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    static mut dukv_DecoderVtbl: TFB_VideoDecoderFuncs;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut resolutionFactor: ::core::ffi::c_uint;
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const VIDEODECODER_OK: Self = Self(0);
    pub const VIDEODECODER_ERROR: Self = Self(1);
    pub const VIDEODECODER_EOF: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_RegVideoDecoder {
    pub builtin: bool,
    pub used: bool,
    pub ext: *const ::core::ffi::c_char,
    pub funcs: *const TFB_VideoDecoderFuncs,
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
static mut vd_inited: bool = false;
static mut vd_vidfmt: TFB_PixelFormat = TFB_PixelFormat {
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
static mut vd_flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut vd_decoders: [TFB_RegVideoDecoder; 32] = unsafe {
    [
        TFB_RegVideoDecoder {
            builtin: true,
            used: true,
            ext: b"duk\0".as_ptr() as *const ::core::ffi::c_char,
            funcs: &raw const dukv_DecoderVtbl as *mut TFB_VideoDecoderFuncs,
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
        TFB_RegVideoDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_VideoDecoderFuncs>(),
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_GetName(
    mut decoder: *mut TFB_VideoDecoder,
) -> *const ::core::ffi::c_char {
    if decoder.is_null() || (*decoder).funcs.is_null() {
        return b"(Null)\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return uqm_lua_checked!((*(*decoder).funcs).GetName.expect("non-null function pointer")(), ::core::ptr::null());
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Init(
    mut flags: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_int,
    mut Rmask: uint32,
    mut Gmask: uint32,
    mut Bmask: uint32,
    mut Amask: uint32,
) -> bool {
    let mut info: *mut TFB_RegVideoDecoder = ::core::ptr::null_mut::<
        TFB_RegVideoDecoder,
    >();
    vd_inited = false;
    if depth < 15 as ::core::ffi::c_int || depth > 32 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"VideoDecoder_Init: Unsupported video depth %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            depth,
        );
        return false;
    }
    if Rmask & Gmask != 0 || Rmask & Bmask != 0 || Rmask & Amask != 0
        || Gmask & Bmask != 0 || Gmask & Amask != 0 || Bmask & Amask != 0
    {
        log_add(
            log_Level::log_Error,
            b"VideoDecoder_Init: Invalid channel masks\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return false;
    }
    vd_vidfmt.BitsPerPixel = depth;
    vd_vidfmt.BytesPerPixel = (depth + 7 as ::core::ffi::c_int)
        / 8 as ::core::ffi::c_int;
    vd_vidfmt.Rmask = Rmask as DWORD;
    vd_vidfmt.Gmask = Gmask as DWORD;
    vd_vidfmt.Bmask = Bmask as DWORD;
    vd_vidfmt.Amask = Amask as DWORD;
    vd_computeMasks(Rmask, &raw mut vd_vidfmt.Rshift, &raw mut vd_vidfmt.Rloss);
    vd_computeMasks(Gmask, &raw mut vd_vidfmt.Gshift, &raw mut vd_vidfmt.Gloss);
    vd_computeMasks(Bmask, &raw mut vd_vidfmt.Bshift, &raw mut vd_vidfmt.Bloss);
    vd_computeMasks(Amask, &raw mut vd_vidfmt.Ashift, &raw mut vd_vidfmt.Aloss);
    info = &raw mut vd_decoders as *mut TFB_RegVideoDecoder;
    while !(*info).ext.is_null() {
        if !uqm_lua_checked!((*(*info).funcs).InitModule.expect("non-null function pointer")(flags), false) {
            log_add(
                log_Level::log_Error,
                b"VideoDecoder_Init(): %s video decoder init failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                uqm_lua_checked!((*(*info).funcs).GetName.expect("non-null function pointer")(), false),
            );
        }
        info = info.offset(1);
    }
    vd_flags = flags;
    vd_inited = true;
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Uninit() {
    let mut info: *mut TFB_RegVideoDecoder = ::core::ptr::null_mut::<
        TFB_RegVideoDecoder,
    >();
    info = &raw mut vd_decoders as *mut TFB_RegVideoDecoder;
    while (*info).used {
        if !(*info).ext.is_null() {
            uqm_lua_checked!((*(*info).funcs).TermModule.expect("non-null function pointer")(), ());
        }
        if !(*info).builtin {
            (*info).used = false;
            (*info).ext = ::core::ptr::null::<::core::ffi::c_char>();
        }
        info = info.offset(1);
    }
    vd_inited = false;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Register(
    mut fileext: *const ::core::ffi::c_char,
    mut decvtbl: *mut TFB_VideoDecoderFuncs,
) -> *mut TFB_RegVideoDecoder {
    let mut info: *mut TFB_RegVideoDecoder = ::core::ptr::null_mut::<
        TFB_RegVideoDecoder,
    >();
    let mut newslot: *mut TFB_RegVideoDecoder = ::core::ptr::null_mut::<
        TFB_RegVideoDecoder,
    >();
    if decvtbl.is_null() {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Register(): Null decoder table\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_RegVideoDecoder>();
    }
    if fileext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Register(): Bad file type for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*decvtbl).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        return ::core::ptr::null_mut::<TFB_RegVideoDecoder>();
    }
    info = &raw mut vd_decoders as *mut TFB_RegVideoDecoder;
    while (*info).used as ::core::ffi::c_int != 0
        && ((*info).ext.is_null()
            || strcmp((*info).ext, fileext) != 0 as ::core::ffi::c_int)
    {
        if newslot.is_null() && (*info).ext.is_null() {
            newslot = info;
        }
        info = info.offset(1);
    }
    if info
        >= (&raw mut vd_decoders as *mut TFB_RegVideoDecoder)
            .offset(31 as ::core::ffi::c_int as isize)
    {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Register(): Decoders limit reached\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_RegVideoDecoder>();
    } else if !(*info).ext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Register(): '%s' decoder already registered (%s denied)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            fileext,
            uqm_lua_checked!((*decvtbl).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        return ::core::ptr::null_mut::<TFB_RegVideoDecoder>();
    }
    if !uqm_lua_checked!((*decvtbl).InitModule.expect("non-null function pointer")(vd_flags), ::core::ptr::null_mut()) {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Register(): %s decoder init failed\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*decvtbl).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        return ::core::ptr::null_mut::<TFB_RegVideoDecoder>();
    }
    if newslot.is_null() {
        newslot = info;
        (*newslot).used = true;
        (*info.offset(1isize)).builtin = false;
        (*info.offset(1isize)).used = false;
        (*info.offset(1isize)).ext = ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*newslot).ext = fileext;
    (*newslot).funcs = decvtbl;
    return newslot;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Unregister(mut regdec: *mut TFB_RegVideoDecoder) {
    if regdec < &raw mut vd_decoders as *mut TFB_RegVideoDecoder
        || regdec
            >= (&raw mut vd_decoders as *mut TFB_RegVideoDecoder)
                .offset(31 as ::core::ffi::c_int as isize) || (*regdec).ext.is_null()
        || (*regdec).funcs.is_null()
    {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Unregister(): Invalid or expired decoder passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    uqm_lua_checked!((*(*regdec).funcs).TermModule.expect("non-null function pointer")(), ());
    (*regdec).ext = ::core::ptr::null::<::core::ffi::c_char>();
    (*regdec).funcs = ::core::ptr::null::<TFB_VideoDecoderFuncs>();
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Lookup(
    mut fileext: *const ::core::ffi::c_char,
) -> *const TFB_VideoDecoderFuncs {
    let mut info: *mut TFB_RegVideoDecoder = ::core::ptr::null_mut::<
        TFB_RegVideoDecoder,
    >();
    info = &raw mut vd_decoders as *mut TFB_RegVideoDecoder;
    while (*info).used as ::core::ffi::c_int != 0
        && ((*info).ext.is_null()
            || strcmp((*info).ext, fileext) != 0 as ::core::ffi::c_int)
    {
        info = info.offset(1);
    }
    return if !(*info).ext.is_null() {
        (*info).funcs
    } else {
        ::core::ptr::null::<TFB_VideoDecoderFuncs>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Load(
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> *mut TFB_VideoDecoder {
    let mut pext: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut info: *mut TFB_RegVideoDecoder = ::core::ptr::null_mut::<
        TFB_RegVideoDecoder,
    >();
    let mut decoder: *mut TFB_VideoDecoder = ::core::ptr::null_mut::<TFB_VideoDecoder>();
    if !vd_inited {
        return ::core::ptr::null_mut::<TFB_VideoDecoder>();
    }
    pext = strrchr(filename, '.' as ::core::ffi::c_int);
    if pext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Load: Unknown file type\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_VideoDecoder>();
    }
    pext = pext.offset(1);
    info = &raw mut vd_decoders as *mut TFB_RegVideoDecoder;
    while (*info).used as ::core::ffi::c_int != 0
        && ((*info).ext.is_null()
            || strcmp((*info).ext, pext) != 0 as ::core::ffi::c_int)
    {
        info = info.offset(1);
    }
    if (*info).ext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Load: Unsupported file type\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_VideoDecoder>();
    }
    decoder = HCalloc(
        uqm_lua_checked!((*(*info).funcs).GetStructSize.expect("non-null function pointer")(), ::core::ptr::null_mut()) as size_t,
    ) as *mut TFB_VideoDecoder;
    (*decoder).funcs = (*info).funcs;
    if !uqm_lua_checked!((*(*decoder).funcs)
        .Init
        .expect("non-null function pointer")(decoder, &raw mut vd_vidfmt), ::core::ptr::null_mut())
    {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Load: Cannot init '%s' decoder, code %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*(*decoder).funcs).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
            uqm_lua_checked!((*(*decoder).funcs).GetError.expect("non-null function pointer")(decoder), ::core::ptr::null_mut()),
        );
        HFree(decoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<TFB_VideoDecoder>();
    }
    (*decoder).dir = dir;
    (*decoder).filename = HMalloc(strlen(filename).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    strcpy((*decoder).filename, filename);
    (*decoder).error = C2Rust_Unnamed_0::VIDEODECODER_OK.0 as ::core::ffi::c_int
        as sint32;
    if !uqm_lua_checked!((*(*decoder).funcs)
        .Open
        .expect("non-null function pointer")(decoder, dir, filename), ::core::ptr::null_mut())
    {
        log_add(
            log_Level::log_Warning,
            b"VideoDecoder_Load: '%s' decoder did not load %s, code %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*(*decoder).funcs).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
            filename,
            uqm_lua_checked!((*(*decoder).funcs).GetError.expect("non-null function pointer")(decoder), ::core::ptr::null_mut()),
        );
        uqm_lua_checked!(VideoDecoder_Free(decoder), ::core::ptr::null_mut());
        return ::core::ptr::null_mut::<TFB_VideoDecoder>();
    }
    (*decoder).w = (*decoder).w << resolutionFactor;
    (*decoder).h = ((*decoder).h << resolutionFactor)
        .wrapping_sub(
            (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                0 as ::core::ffi::c_int
            } else {
                7 as ::core::ffi::c_int
            }) as uint32,
        );
    return decoder;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Decode(
    mut decoder: *mut TFB_VideoDecoder,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if decoder.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*decoder).cur_frame = uqm_lua_checked!((*(*decoder).funcs)
        .GetFrame
        .expect("non-null function pointer")(decoder), 0);
    (*decoder).pos = uqm_lua_checked!((*(*decoder).funcs)
        .GetTime
        .expect("non-null function pointer")(decoder), 0);
    ret = uqm_lua_checked!((*(*decoder).funcs).DecodeNext.expect("non-null function pointer")(decoder), 0);
    if ret == 0 as ::core::ffi::c_int {
        (*decoder).error = C2Rust_Unnamed_0::VIDEODECODER_EOF.0 as ::core::ffi::c_int
            as sint32;
    } else if ret < 0 as ::core::ffi::c_int {
        (*decoder).error = C2Rust_Unnamed_0::VIDEODECODER_ERROR.0 as ::core::ffi::c_int
            as sint32;
    } else {
        (*decoder).error = C2Rust_Unnamed_0::VIDEODECODER_OK.0 as ::core::ffi::c_int
            as sint32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Seek(
    mut decoder: *mut TFB_VideoDecoder,
    mut pos: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    if decoder.is_null() {
        return 0.0f32;
    }
    (*decoder).pos = uqm_lua_checked!((*(*decoder).funcs)
        .SeekTime
        .expect("non-null function pointer")(decoder, pos), 0.0);
    (*decoder).cur_frame = uqm_lua_checked!((*(*decoder).funcs)
        .GetFrame
        .expect("non-null function pointer")(decoder), 0.0);
    return (*decoder).pos;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_SeekFrame(
    mut decoder: *mut TFB_VideoDecoder,
    mut frame: uint32,
) -> uint32 {
    if decoder.is_null() {
        return 0 as uint32;
    }
    (*decoder).cur_frame = uqm_lua_checked!((*(*decoder).funcs)
        .SeekFrame
        .expect("non-null function pointer")(decoder, frame), 0);
    (*decoder).pos = uqm_lua_checked!((*(*decoder).funcs)
        .GetTime
        .expect("non-null function pointer")(decoder), 0);
    return (*decoder).cur_frame;
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Rewind(mut decoder: *mut TFB_VideoDecoder) {
    if decoder.is_null() {
        return;
    }
    uqm_lua_checked!(VideoDecoder_Seek(decoder, 0 as ::core::ffi::c_int as ::core::ffi::c_float), ());
}
#[no_mangle]
pub unsafe extern "C" fn VideoDecoder_Free(mut decoder: *mut TFB_VideoDecoder) {
    if decoder.is_null() {
        return;
    }
    uqm_lua_checked!((*(*decoder).funcs).Close.expect("non-null function pointer")(decoder), ());
    uqm_lua_checked!((*(*decoder).funcs).Term.expect("non-null function pointer")(decoder), ());
    HFree((*decoder).filename as *mut ::core::ffi::c_void);
    HFree(decoder as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn vd_computeMasks(
    mut mask: uint32,
    mut shift: *mut DWORD,
    mut loss: *mut DWORD,
) {
    *shift = 0 as DWORD;
    *loss = 8 as DWORD;
    if mask != 0 {
        while mask & 1 as uint32 == 0 {
            *shift = (*shift).wrapping_add(1);
            mask >>= 1 as ::core::ffi::c_int;
        }
        while mask & 1 as uint32 != 0 {
            *loss = (*loss).wrapping_sub(1);
            mask >>= 1 as ::core::ffi::c_int;
        }
    }
}
