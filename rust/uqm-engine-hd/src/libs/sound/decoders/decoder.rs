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

/* Sound file decoder for .wav, .mod, .ogg (to be used with OpenAL)
 * API is heavily influenced by SDL_sound.
 */

// Rust translation of src/libs/sound/decoders/decoder.c.
extern "C" {
    pub type uio_DirHandle;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn fileExists2(
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> bool;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut wava_DecoderVtbl: TFB_SoundDecoderFuncs;
    static mut duka_DecoderVtbl: TFB_SoundDecoderFuncs;
    static mut moda_DecoderVtbl: TFB_SoundDecoderFuncs;
    static mut ova_DecoderVtbl: TFB_SoundDecoderFuncs;
    static mut aifa_DecoderVtbl: TFB_SoundDecoderFuncs;
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SOUNDDECODER_OK: Self = Self(0);
    pub const SOUNDDECODER_ERROR: Self = Self(1);
    pub const SOUNDDECODER_EOF: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_RegSoundDecoder {
    pub builtin: bool,
    pub used: bool,
    pub ext: *const ::core::ffi::c_char,
    pub funcs: *const TFB_SoundDecoderFuncs,
}
pub type TFB_BufSoundDecoder = tfb_bufsounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_bufsounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub data: *mut ::core::ffi::c_void,
    pub max_pcm: uint32,
    pub cur_pcm: uint32,
}
pub type TFB_NullSoundDecoder = tfb_nullsounddecoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_nullsounddecoder {
    pub decoder: TFB_SoundDecoder,
    pub cur_pcm: uint32,
}
#[no_mangle]
pub static mut bufa_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(bufa_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        bufa_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(bufa_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(bufa_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        bufa_GetError
            as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(bufa_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(bufa_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        bufa_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(bufa_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        bufa_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        bufa_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(
        bufa_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32,
    ),
};
#[no_mangle]
pub static mut nula_DecoderVtbl: TFB_SoundDecoderFuncs = tfb_sounddecoderfunc {
    GetName: Some(nula_GetName as unsafe extern "C" fn() -> *const ::core::ffi::c_char),
    InitModule: Some(
        nula_InitModule
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const TFB_DecoderFormats,
            ) -> bool,
    ),
    TermModule: Some(nula_TermModule as unsafe extern "C" fn() -> ()),
    GetStructSize: Some(nula_GetStructSize as unsafe extern "C" fn() -> uint32),
    GetError: Some(
        nula_GetError
            as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ::core::ffi::c_int,
    ),
    Init: Some(nula_Init as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> bool),
    Term: Some(nula_Term as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Open: Some(
        nula_Open
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut uio_DirHandle,
                *const ::core::ffi::c_char,
            ) -> bool,
    ),
    Close: Some(nula_Close as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> ()),
    Decode: Some(
        nula_Decode
            as unsafe extern "C" fn(
                *mut TFB_SoundDecoder,
                *mut ::core::ffi::c_void,
                sint32,
            ) -> ::core::ffi::c_int,
    ),
    Seek: Some(
        nula_Seek as unsafe extern "C" fn(*mut TFB_SoundDecoder, uint32) -> uint32,
    ),
    GetFrame: Some(
        nula_GetFrame as unsafe extern "C" fn(*mut TFB_SoundDecoder) -> uint32,
    ),
};
static mut sd_decoders: [TFB_RegSoundDecoder; 32] = unsafe {
    [
        TFB_RegSoundDecoder {
            builtin: true,
            used: true,
            ext: b"wav\0".as_ptr() as *const ::core::ffi::c_char,
            funcs: &raw const wava_DecoderVtbl as *mut TFB_SoundDecoderFuncs,
        },
        TFB_RegSoundDecoder {
            builtin: true,
            used: true,
            ext: b"mod\0".as_ptr() as *const ::core::ffi::c_char,
            funcs: &raw const moda_DecoderVtbl as *mut TFB_SoundDecoderFuncs,
        },
        TFB_RegSoundDecoder {
            builtin: true,
            used: true,
            ext: b"ogg\0".as_ptr() as *const ::core::ffi::c_char,
            funcs: &raw const ova_DecoderVtbl as *mut TFB_SoundDecoderFuncs,
        },
        TFB_RegSoundDecoder {
            builtin: true,
            used: true,
            ext: b"duk\0".as_ptr() as *const ::core::ffi::c_char,
            funcs: &raw const duka_DecoderVtbl as *mut TFB_SoundDecoderFuncs,
        },
        TFB_RegSoundDecoder {
            builtin: true,
            used: true,
            ext: b"aif\0".as_ptr() as *const ::core::ffi::c_char,
            funcs: &raw const aifa_DecoderVtbl as *mut TFB_SoundDecoderFuncs,
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
        TFB_RegSoundDecoder {
            builtin: false,
            used: false,
            ext: ::core::ptr::null::<::core::ffi::c_char>(),
            funcs: ::core::ptr::null::<TFB_SoundDecoderFuncs>(),
        },
    ]
};
static mut decoder_formats: TFB_DecoderFormats = TFB_DecoderFormats {
    big_endian: false,
    want_big_endian: false,
    mono8: 0,
    stereo8: 0,
    mono16: 0,
    stereo16: 0,
};
static mut sd_flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_SwapWords(
    mut data: *mut uint16,
    mut size: uint32,
) {
    let mut fsize: uint32 = size & !(3 as uint32);
    size = size.wrapping_sub(fsize);
    fsize >>= 2 as ::core::ffi::c_int;
    while fsize != 0 {
        let mut v: uint32 = *(data as *mut uint32);
        *(data as *mut uint32) = ((v
            & 0xff00ff as ::core::ffi::c_int as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_int
            | (v & 0xff00ff00 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int)
            as uint32;
        fsize = fsize.wrapping_sub(1);
        data = data.offset(2 as ::core::ffi::c_int as isize);
    }
    if size != 0 {
        *data = ((*data as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | (*data as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int)
                >> 8 as ::core::ffi::c_int) as uint16;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_GetName(
    mut decoder: *mut TFB_SoundDecoder,
) -> *const ::core::ffi::c_char {
    if decoder.is_null() || (*decoder).funcs.is_null() {
        return b"(Null)\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return uqm_lua_checked!((*(*decoder).funcs).GetName.expect("non-null function pointer")(), ::core::ptr::null());
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Init(
    mut flags: ::core::ffi::c_int,
    mut formats: *mut TFB_DecoderFormats,
) -> sint32 {
    let mut info: *mut TFB_RegSoundDecoder = ::core::ptr::null_mut::<
        TFB_RegSoundDecoder,
    >();
    let mut ret: sint32 = 0 as sint32;
    if formats.is_null() {
        log_add(
            log_Level::log_Error,
            b"SoundDecoder_Init(): missing decoder formats\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 1 as sint32;
    }
    decoder_formats = *formats;
    info = &raw mut sd_decoders as *mut TFB_RegSoundDecoder;
    while !(*info).ext.is_null() {
        if !uqm_lua_checked!((*(*info).funcs)
            .InitModule
            .expect("non-null function pointer")(flags, &raw mut decoder_formats), 0)
        {
            log_add(
                log_Level::log_Error,
                b"SoundDecoder_Init(): %s audio decoder init failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                uqm_lua_checked!((*(*info).funcs).GetName.expect("non-null function pointer")(), 0),
            );
            ret = 1 as ::core::ffi::c_int as sint32;
        }
        info = info.offset(1);
    }
    sd_flags = flags;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Uninit() {
    let mut info: *mut TFB_RegSoundDecoder = ::core::ptr::null_mut::<
        TFB_RegSoundDecoder,
    >();
    info = &raw mut sd_decoders as *mut TFB_RegSoundDecoder;
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
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Register(
    mut fileext: *const ::core::ffi::c_char,
    mut decvtbl: *mut TFB_SoundDecoderFuncs,
) -> *mut TFB_RegSoundDecoder {
    let mut info: *mut TFB_RegSoundDecoder = ::core::ptr::null_mut::<
        TFB_RegSoundDecoder,
    >();
    let mut newslot: *mut TFB_RegSoundDecoder = ::core::ptr::null_mut::<
        TFB_RegSoundDecoder,
    >();
    if decvtbl.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Register(): Null decoder table\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_RegSoundDecoder>();
    }
    if fileext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Register(): Bad file type for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*decvtbl).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        return ::core::ptr::null_mut::<TFB_RegSoundDecoder>();
    }
    info = &raw mut sd_decoders as *mut TFB_RegSoundDecoder;
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
        >= (&raw mut sd_decoders as *mut TFB_RegSoundDecoder)
            .offset(31 as ::core::ffi::c_int as isize)
    {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Register(): Decoders limit reached\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_RegSoundDecoder>();
    } else if !(*info).ext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Register(): '%s' decoder already registered (%s denied)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            fileext,
            uqm_lua_checked!((*decvtbl).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        return ::core::ptr::null_mut::<TFB_RegSoundDecoder>();
    }
    if !uqm_lua_checked!((*decvtbl)
        .InitModule
        .expect("non-null function pointer")(sd_flags, &raw mut decoder_formats), ::core::ptr::null_mut())
    {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Register(): %s decoder init failed\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*decvtbl).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        return ::core::ptr::null_mut::<TFB_RegSoundDecoder>();
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
pub unsafe extern "C" fn SoundDecoder_Unregister(mut regdec: *mut TFB_RegSoundDecoder) {
    if regdec < &raw mut sd_decoders as *mut TFB_RegSoundDecoder
        || regdec
            >= (&raw mut sd_decoders as *mut TFB_RegSoundDecoder)
                .offset(31 as ::core::ffi::c_int as isize) || (*regdec).ext.is_null()
        || (*regdec).funcs.is_null()
    {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Unregister(): Invalid or expired decoder passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    uqm_lua_checked!((*(*regdec).funcs).TermModule.expect("non-null function pointer")(), ());
    (*regdec).ext = ::core::ptr::null::<::core::ffi::c_char>();
    (*regdec).funcs = ::core::ptr::null::<TFB_SoundDecoderFuncs>();
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Lookup(
    mut fileext: *const ::core::ffi::c_char,
) -> *const TFB_SoundDecoderFuncs {
    let mut info: *mut TFB_RegSoundDecoder = ::core::ptr::null_mut::<
        TFB_RegSoundDecoder,
    >();
    info = &raw mut sd_decoders as *mut TFB_RegSoundDecoder;
    while (*info).used as ::core::ffi::c_int != 0
        && ((*info).ext.is_null()
            || strcmp((*info).ext, fileext) != 0 as ::core::ffi::c_int)
    {
        info = info.offset(1);
    }
    return if !(*info).ext.is_null() {
        (*info).funcs
    } else {
        ::core::ptr::null::<TFB_SoundDecoderFuncs>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Load(
    mut dir: *mut uio_DirHandle,
    mut filename: *mut ::core::ffi::c_char,
    mut buffer_size: uint32,
    mut startTime: uint32,
    mut runTime: sint32,
) -> *mut TFB_SoundDecoder {
    let mut pext: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut info: *mut TFB_RegSoundDecoder = ::core::ptr::null_mut::<
        TFB_RegSoundDecoder,
    >();
    let mut funcs: *const TFB_SoundDecoderFuncs = ::core::ptr::null::<
        TFB_SoundDecoderFuncs,
    >();
    let mut decoder: *mut TFB_SoundDecoder = ::core::ptr::null_mut::<TFB_SoundDecoder>();
    let mut struct_size: uint32 = 0;
    pext = strrchr(filename, '.' as ::core::ffi::c_int);
    if pext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Load(): Unknown file type (%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            filename,
        );
        return ::core::ptr::null_mut::<TFB_SoundDecoder>();
    }
    pext = pext.offset(1);
    info = &raw mut sd_decoders as *mut TFB_RegSoundDecoder;
    while (*info).used as ::core::ffi::c_int != 0
        && ((*info).ext.is_null()
            || strcmp((*info).ext, pext) != 0 as ::core::ffi::c_int)
    {
        info = info.offset(1);
    }
    if (*info).ext.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Load(): Unsupported file type (%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            filename,
        );
        if runTime != 0 {
            runTime = abs(runTime) as sint32;
            startTime = 0 as uint32;
            funcs = &raw mut nula_DecoderVtbl;
        } else {
            return ::core::ptr::null_mut::<TFB_SoundDecoder>()
        }
    } else {
        funcs = (*info).funcs;
    }
    if !uqm_lua_checked!(fileExists2(dir, filename), ::core::ptr::null_mut()) {
        if runTime != 0 {
            runTime = abs(runTime) as sint32;
            startTime = 0 as uint32;
            funcs = &raw mut nula_DecoderVtbl;
        } else {
            log_add(
                log_Level::log_Warning,
                b"SoundDecoder_Load(): %s does not exist\0".as_ptr()
                    as *const ::core::ffi::c_char,
                filename,
            );
            return ::core::ptr::null_mut::<TFB_SoundDecoder>();
        }
    }
    struct_size = uqm_lua_checked!((*funcs).GetStructSize.expect("non-null function pointer")(), ::core::ptr::null_mut());
    if (struct_size as usize) < ::core::mem::size_of::<TFB_BufSoundDecoder>() {
        struct_size = ::core::mem::size_of::<TFB_BufSoundDecoder>() as uint32;
    }
    decoder = HCalloc(struct_size as size_t) as *mut TFB_SoundDecoder;
    (*decoder).funcs = funcs;
    if !uqm_lua_checked!((*(*decoder).funcs).Init.expect("non-null function pointer")(decoder), ::core::ptr::null_mut()) {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Load(): %s decoder instance failed init\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*(*decoder).funcs).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
        );
        HFree(decoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<TFB_SoundDecoder>();
    }
    if !uqm_lua_checked!((*(*decoder).funcs)
        .Open
        .expect("non-null function pointer")(decoder, dir, filename), ::core::ptr::null_mut())
    {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Load(): %s decoder could not load %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!((*(*decoder).funcs).GetName.expect("non-null function pointer")(), ::core::ptr::null_mut()),
            filename,
        );
        uqm_lua_checked!((*(*decoder).funcs).Term.expect("non-null function pointer")(decoder), ::core::ptr::null_mut());
        HFree(decoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<TFB_SoundDecoder>();
    }
    (*decoder).buffer = HMalloc(buffer_size as size_t);
    (*decoder).buffer_size = buffer_size;
    (*decoder).looping = false;
    (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_OK.0 as ::core::ffi::c_int as sint32;
    (*decoder).dir = dir;
    (*decoder).filename = HMalloc(strlen(filename).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    strcpy((*decoder).filename, filename);
    if (*decoder).is_null {
        (*decoder).length = (runTime as ::core::ffi::c_double / 1000.0f64)
            as ::core::ffi::c_float;
    }
    (*decoder).length -= startTime as ::core::ffi::c_float / 1000.0f32;
    if (*decoder).length < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*decoder).length = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    } else if runTime > 0 as ::core::ffi::c_int
        && runTime as ::core::ffi::c_double / 1000.0f64
            < (*decoder).length as ::core::ffi::c_double
    {
        (*decoder).length = (runTime as ::core::ffi::c_double / 1000.0f64)
            as ::core::ffi::c_float;
    }
    (*decoder).start_sample = (startTime as ::core::ffi::c_float / 1000.0f32
        * (*decoder).frequency as ::core::ffi::c_float) as uint32;
    (*decoder).end_sample = ((*decoder).start_sample as ::core::ffi::c_ulong)
        .wrapping_add(
            ((*decoder).length * (*decoder).frequency as ::core::ffi::c_float)
                as ::core::ffi::c_ulong,
        ) as uint32;
    if (*decoder).start_sample != 0 as uint32 {
        uqm_lua_checked!((*(*decoder).funcs)
            .Seek
            .expect("non-null function pointer")(decoder, (*decoder).start_sample), ::core::ptr::null_mut());
    }
    if (*decoder).format == decoder_formats.mono8 {
        (*decoder).bytes_per_samp = 1 as uint32;
    } else if (*decoder).format == decoder_formats.mono16 {
        (*decoder).bytes_per_samp = 2 as uint32;
    } else if (*decoder).format == decoder_formats.stereo8 {
        (*decoder).bytes_per_samp = 2 as uint32;
    } else if (*decoder).format == decoder_formats.stereo16 {
        (*decoder).bytes_per_samp = 4 as uint32;
    }
    (*decoder).pos = (*decoder).start_sample.wrapping_mul((*decoder).bytes_per_samp);
    return decoder;
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Decode(
    mut decoder: *mut TFB_SoundDecoder,
) -> uint32 {
    let mut decoded_bytes: ::core::ffi::c_long = 0;
    let mut rc: ::core::ffi::c_long = 0;
    let mut buffer_size: ::core::ffi::c_long = 0;
    let mut max_bytes: uint32 = 0xffffffff as uint32;
    let mut buffer: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    if decoder.is_null() || (*decoder).funcs.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Decode(): null or bad decoder\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as uint32;
    }
    buffer = (*decoder).buffer as *mut uint8;
    buffer_size = (*decoder).buffer_size as ::core::ffi::c_long;
    if !(*decoder).looping && (*decoder).end_sample > 0 as uint32 {
        max_bytes = (*decoder).end_sample.wrapping_mul((*decoder).bytes_per_samp);
        if max_bytes.wrapping_sub((*decoder).pos) < (*decoder).buffer_size {
            buffer_size = max_bytes.wrapping_sub((*decoder).pos) as ::core::ffi::c_long;
        }
    }
    if buffer_size == 0 as ::core::ffi::c_long {
        (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_EOF.0 as ::core::ffi::c_int
            as sint32;
        return 0 as uint32;
    }
    decoded_bytes = 0 as ::core::ffi::c_long;
    rc = 1 as ::core::ffi::c_long;
    while rc > 0 as ::core::ffi::c_long && decoded_bytes < buffer_size {
        rc = uqm_lua_checked!((*(*decoder).funcs)
            .Decode
            .expect(
                "non-null function pointer",
            )(
            decoder,
            buffer.offset(decoded_bytes as isize) as *mut ::core::ffi::c_void,
            (buffer_size - decoded_bytes) as sint32,
        ), 0) as ::core::ffi::c_long;
        if rc < 0 as ::core::ffi::c_long {
            log_add(
                log_Level::log_Warning,
                b"SoundDecoder_Decode(): error decoding %s, code %ld\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*decoder).filename,
                rc,
            );
        } else if rc == 0 as ::core::ffi::c_long {
            if (*decoder).looping {
                uqm_lua_checked!(SoundDecoder_Rewind(decoder), 0);
                if (*decoder).error != 0 {
                    log_add(
                        log_Level::log_Warning,
                        b"SoundDecoder_Decode(): tried to loop %s but couldn't rewind, error code %d\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*decoder).filename,
                        (*decoder).error,
                    );
                } else {
                    log_add(
                        log_Level::log_Info,
                        b"SoundDecoder_Decode(): looping %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*decoder).filename,
                    );
                    rc = 1 as ::core::ffi::c_long;
                }
            } else {
                log_add(
                    log_Level::log_Info,
                    b"SoundDecoder_Decode(): eof for %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*decoder).filename,
                );
            }
        } else {
            decoded_bytes += rc;
        }
    }
    (*decoder).pos = ((*decoder).pos as ::core::ffi::c_ulong)
        .wrapping_add(decoded_bytes as ::core::ffi::c_ulong) as uint32;
    if rc < 0 as ::core::ffi::c_long {
        (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_ERROR.0 as ::core::ffi::c_int
            as sint32;
    } else if rc == 0 as ::core::ffi::c_long || (*decoder).pos >= max_bytes {
        (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_EOF.0 as ::core::ffi::c_int
            as sint32;
    } else {
        (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_OK.0 as ::core::ffi::c_int
            as sint32;
    }
    if (*decoder).need_swap as ::core::ffi::c_int != 0
        && decoded_bytes > 0 as ::core::ffi::c_long
        && ((*decoder).format == decoder_formats.stereo16
            || (*decoder).format == decoder_formats.mono16)
    {
        SoundDecoder_SwapWords(
            (*decoder).buffer as *mut uint16,
            decoded_bytes as uint32,
        );
    }
    return decoded_bytes as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_DecodeAll(
    mut decoder: *mut TFB_SoundDecoder,
) -> uint32 {
    let mut decoded_bytes: uint32 = 0;
    let mut rc: ::core::ffi::c_long = 0;
    let mut reqbufsize: uint32 = 0;
    if decoder.is_null() || (*decoder).funcs.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_DecodeAll(): null or bad decoder\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as uint32;
    }
    reqbufsize = (*decoder).buffer_size;
    if (*decoder).looping {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_DecodeAll(): called for %s with looping\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*decoder).filename,
        );
        return 0 as uint32;
    }
    if reqbufsize < 4096 as uint32 {
        reqbufsize = 4096 as uint32;
    }
    decoded_bytes = 0 as uint32;
    rc = 1 as ::core::ffi::c_long;
    while rc > 0 as ::core::ffi::c_long {
        if decoded_bytes >= (*decoder).buffer_size {
            (*decoder).buffer_size = (*decoder).buffer_size.wrapping_add(reqbufsize);
            (*decoder).buffer = HRealloc(
                (*decoder).buffer,
                (*decoder).buffer_size as size_t,
            );
        }
        rc = uqm_lua_checked!((*(*decoder).funcs)
            .Decode
            .expect(
                "non-null function pointer",
            )(
            decoder,
            ((*decoder).buffer as *mut uint8).offset(decoded_bytes as isize)
                as *mut ::core::ffi::c_void,
            (*decoder).buffer_size.wrapping_sub(decoded_bytes) as sint32,
        ), 0) as ::core::ffi::c_long;
        if rc > 0 as ::core::ffi::c_long {
            decoded_bytes = (decoded_bytes as ::core::ffi::c_ulong)
                .wrapping_add(rc as ::core::ffi::c_ulong) as uint32;
        }
    }
    (*decoder).buffer_size = decoded_bytes;
    (*decoder).pos = (*decoder).pos.wrapping_add(decoded_bytes);
    (*decoder).buffer = HRealloc((*decoder).buffer, decoded_bytes as size_t);
    if (*decoder).need_swap as ::core::ffi::c_int != 0 && decoded_bytes > 0 as uint32
        && ((*decoder).format == decoder_formats.stereo16
            || (*decoder).format == decoder_formats.mono16)
    {
        SoundDecoder_SwapWords((*decoder).buffer as *mut uint16, decoded_bytes);
    }
    if rc < 0 as ::core::ffi::c_long {
        (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_ERROR.0 as ::core::ffi::c_int
            as sint32;
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_DecodeAll(): error decoding %s, code %ld\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*decoder).filename,
            rc,
        );
        return decoded_bytes;
    }
    uqm_lua_checked!((*(*decoder).funcs).Close.expect("non-null function pointer")(decoder), 0);
    uqm_lua_checked!((*(*decoder).funcs).Term.expect("non-null function pointer")(decoder), 0);
    (*decoder).funcs = &raw mut bufa_DecoderVtbl;
    uqm_lua_checked!((*(*decoder).funcs).Init.expect("non-null function pointer")(decoder), 0);
    (*decoder).pos = 0 as uint32;
    (*decoder).start_sample = 0 as uint32;
    (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_OK.0 as ::core::ffi::c_int as sint32;
    return decoded_bytes;
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Rewind(mut decoder: *mut TFB_SoundDecoder) {
    uqm_lua_checked!(SoundDecoder_Seek(decoder, 0 as uint32), ());
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Seek(
    mut decoder: *mut TFB_SoundDecoder,
    mut seekTime: uint32,
) {
    let mut pcm_pos: uint32 = 0;
    if decoder.is_null() {
        return;
    }
    if (*decoder).funcs.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Seek(): bad decoder passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if strcmp(
        uqm_lua_checked!(SoundDecoder_GetName(decoder), ()),
        b"MikMod\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        pcm_pos = (seekTime as ::core::ffi::c_float / 1000.0f32
            * (*decoder).frequency as ::core::ffi::c_float) as uint32;
        pcm_pos = uqm_lua_checked!((*(*decoder).funcs)
            .Seek
            .expect(
                "non-null function pointer",
            )(decoder, (*decoder).start_sample.wrapping_add(pcm_pos)), ());
        (*decoder).pos = pcm_pos.wrapping_mul((*decoder).bytes_per_samp);
    } else {
        uqm_lua_checked!((*(*decoder).funcs).Seek.expect("non-null function pointer")(decoder, seekTime), ());
    }
    (*decoder).error = C2Rust_Unnamed::SOUNDDECODER_OK.0 as ::core::ffi::c_int as sint32;
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_Free(mut decoder: *mut TFB_SoundDecoder) {
    if decoder.is_null() {
        return;
    }
    if (*decoder).funcs.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_Free(): bad decoder passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    uqm_lua_checked!((*(*decoder).funcs).Close.expect("non-null function pointer")(decoder), ());
    uqm_lua_checked!((*(*decoder).funcs).Term.expect("non-null function pointer")(decoder), ());
    HFree((*decoder).buffer);
    HFree((*decoder).filename as *mut ::core::ffi::c_void);
    HFree(decoder as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_GetTime(
    mut decoder: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_float {
    if decoder.is_null() {
        return 0.0f32;
    }
    if (*decoder).funcs.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_GetTime(): bad decoder passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0.0f32;
    }
    return (*decoder)
        .pos
        .wrapping_div((*decoder).bytes_per_samp)
        .wrapping_sub((*decoder).start_sample) as ::core::ffi::c_float
        / (*decoder).frequency as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SoundDecoder_GetFrame(
    mut decoder: *mut TFB_SoundDecoder,
) -> uint32 {
    if decoder.is_null() {
        return 0 as uint32;
    }
    if (*decoder).funcs.is_null() {
        log_add(
            log_Level::log_Warning,
            b"SoundDecoder_GetFrame(): bad decoder passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as uint32;
    }
    return uqm_lua_checked!((*(*decoder).funcs).GetFrame.expect("non-null function pointer")(decoder), 0);
}
unsafe extern "C" fn bufa_GetName() -> *const ::core::ffi::c_char {
    return b"Buffer\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn bufa_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    log_add(
        log_Level::log_Debug,
        b"bufa_InitModule(): dead function called\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return false;
}
unsafe extern "C" fn bufa_TermModule() {
    log_add(
        log_Level::log_Debug,
        b"bufa_TermModule(): dead function called\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn bufa_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_BufSoundDecoder>() as uint32;
}
unsafe extern "C" fn bufa_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn bufa_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    let mut bufa: *mut TFB_BufSoundDecoder = This as *mut TFB_BufSoundDecoder;
    (*This).need_swap = false;
    (*bufa).data = (*This).buffer;
    (*bufa).max_pcm = (*This).buffer_size.wrapping_div((*This).bytes_per_samp);
    (*bufa).cur_pcm = (*bufa).max_pcm;
    return true;
}
unsafe extern "C" fn bufa_Term(mut This: *mut TFB_SoundDecoder) {
    bufa_Close(This);
}
unsafe extern "C" fn bufa_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    log_add(
        log_Level::log_Debug,
        b"bufa_Open(): dead function called\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return false;
}
unsafe extern "C" fn bufa_Close(mut This: *mut TFB_SoundDecoder) {
    let mut bufa: *mut TFB_BufSoundDecoder = This as *mut TFB_BufSoundDecoder;
    if !(*bufa).data.is_null() {
        (*This).buffer = (*bufa).data;
        (*bufa).data = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*bufa).cur_pcm = 0 as uint32;
}
unsafe extern "C" fn bufa_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut bufa: *mut TFB_BufSoundDecoder = This as *mut TFB_BufSoundDecoder;
    let mut dec_pcm: uint32 = 0;
    let mut dec_bytes: uint32 = 0;
    dec_pcm = (bufsize as uint32).wrapping_div((*This).bytes_per_samp);
    if dec_pcm > (*bufa).max_pcm.wrapping_sub((*bufa).cur_pcm) {
        dec_pcm = (*bufa).max_pcm.wrapping_sub((*bufa).cur_pcm);
    }
    dec_bytes = dec_pcm.wrapping_mul((*This).bytes_per_samp);
    (*This).buffer = ((*bufa).data as *mut uint8)
        .offset((*bufa).cur_pcm.wrapping_mul((*This).bytes_per_samp) as isize)
        as *mut ::core::ffi::c_void;
    if dec_pcm > 0 as uint32 {
        (*bufa).cur_pcm = (*bufa).cur_pcm.wrapping_add(dec_pcm);
    }
    return dec_bytes as ::core::ffi::c_int;
}
unsafe extern "C" fn bufa_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut bufa: *mut TFB_BufSoundDecoder = This as *mut TFB_BufSoundDecoder;
    if pcm_pos > (*bufa).max_pcm {
        pcm_pos = (*bufa).max_pcm;
    }
    (*bufa).cur_pcm = pcm_pos;
    return pcm_pos;
}
unsafe extern "C" fn bufa_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    return 0 as uint32;
}
unsafe extern "C" fn nula_GetName() -> *const ::core::ffi::c_char {
    return b"Null\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn nula_InitModule(
    mut flags: ::core::ffi::c_int,
    mut fmts: *const TFB_DecoderFormats,
) -> bool {
    log_add(
        log_Level::log_Debug,
        b"nula_InitModule(): dead function called\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return false;
}
unsafe extern "C" fn nula_TermModule() {
    log_add(
        log_Level::log_Debug,
        b"nula_TermModule(): dead function called\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn nula_GetStructSize() -> uint32 {
    return ::core::mem::size_of::<TFB_NullSoundDecoder>() as uint32;
}
unsafe extern "C" fn nula_GetError(
    mut This: *mut TFB_SoundDecoder,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn nula_Init(mut This: *mut TFB_SoundDecoder) -> bool {
    let mut nula: *mut TFB_NullSoundDecoder = This as *mut TFB_NullSoundDecoder;
    (*This).need_swap = false;
    (*nula).cur_pcm = 0 as uint32;
    return true;
}
unsafe extern "C" fn nula_Term(mut This: *mut TFB_SoundDecoder) {
    nula_Close(This);
}
unsafe extern "C" fn nula_Open(
    mut This: *mut TFB_SoundDecoder,
    mut dir: *mut uio_DirHandle,
    mut filename: *const ::core::ffi::c_char,
) -> bool {
    (*This).frequency = 11025 as uint32;
    (*This).format = decoder_formats.mono16;
    (*This).is_null = true;
    return true;
}
unsafe extern "C" fn nula_Close(mut This: *mut TFB_SoundDecoder) {
    let mut nula: *mut TFB_NullSoundDecoder = This as *mut TFB_NullSoundDecoder;
    (*nula).cur_pcm = 0 as uint32;
}
unsafe extern "C" fn nula_Decode(
    mut This: *mut TFB_SoundDecoder,
    mut buf: *mut ::core::ffi::c_void,
    mut bufsize: sint32,
) -> ::core::ffi::c_int {
    let mut nula: *mut TFB_NullSoundDecoder = This as *mut TFB_NullSoundDecoder;
    let mut max_pcm: uint32 = 0;
    let mut dec_pcm: uint32 = 0;
    let mut dec_bytes: uint32 = 0;
    max_pcm = ((*This).length * (*This).frequency as ::core::ffi::c_float) as uint32;
    dec_pcm = (bufsize as uint32).wrapping_div((*This).bytes_per_samp);
    if dec_pcm > max_pcm.wrapping_sub((*nula).cur_pcm) {
        dec_pcm = max_pcm.wrapping_sub((*nula).cur_pcm);
    }
    dec_bytes = dec_pcm.wrapping_mul((*This).bytes_per_samp);
    if dec_pcm > 0 as uint32 {
        memset(buf, 0 as ::core::ffi::c_int, dec_bytes as size_t);
        (*nula).cur_pcm = (*nula).cur_pcm.wrapping_add(dec_pcm);
    }
    return dec_bytes as ::core::ffi::c_int;
}
unsafe extern "C" fn nula_Seek(
    mut This: *mut TFB_SoundDecoder,
    mut pcm_pos: uint32,
) -> uint32 {
    let mut nula: *mut TFB_NullSoundDecoder = This as *mut TFB_NullSoundDecoder;
    let mut max_pcm: uint32 = 0;
    max_pcm = ((*This).length * (*This).frequency as ::core::ffi::c_float) as uint32;
    if pcm_pos > max_pcm {
        pcm_pos = max_pcm;
    }
    (*nula).cur_pcm = pcm_pos;
    return pcm_pos;
}
unsafe extern "C" fn nula_GetFrame(mut This: *mut TFB_SoundDecoder) -> uint32 {
    return 0 as uint32;
}
