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

// Rust translation of src/libs/graphics/sdl/scalers.c.
extern "C" {
    pub type SDL_BlitMap;
    static Scale_C_Functions: [Scale_FuncDef_t; 0];
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type sint16 = ::core::ffi::c_short;
pub type Uint8 = uint8_t;
pub type Uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: ::core::ffi::c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut SDL_PixelFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub pitch: ::core::ffi::c_int,
    pub pixels: *mut ::core::ffi::c_void,
    pub userdata: *mut ::core::ffi::c_void,
    pub locked: ::core::ffi::c_int,
    pub list_blitmap: *mut ::core::ffi::c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PLATFORM_TYPE(pub ::core::ffi::c_uint);
impl PLATFORM_TYPE {
    pub const PLATFORM_NULL: Self = Self(0);
    pub const PLATFORM_C: Self = Self(1);
    pub const PLATFORM_MMX: Self = Self(2);
    pub const PLATFORM_SSE: Self = Self(3);
    pub const PLATFORM_3DNOW: Self = Self(4);
    pub const PLATFORM_ALTIVEC: Self = Self(5);
    pub const PLATFORM_LAST: Self = Self(5);
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
pub type YUV_VECTOR = Uint32;
pub type TFB_ScaleFunc = Option<
    unsafe extern "C" fn(*mut SDL_Surface, *mut SDL_Surface, *mut SDL_Rect) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Scale_FuncDef_t {
    pub flag: ::core::ffi::c_int,
    pub func: TFB_ScaleFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Scale_PlatDef_t {
    pub platform: Scale_PlatType_t,
    pub funcdefs: *const Scale_FuncDef_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Scale_PlatType_t(pub ::core::ffi::c_uint);
impl Scale_PlatType_t {
    pub const SCALEPLAT_NULL: Self = Self(0);
    pub const SCALEPLAT_C: Self = Self(1);
    pub const SCALEPLAT_MMX: Self = Self(2);
    pub const SCALEPLAT_SSE: Self = Self(3);
    pub const SCALEPLAT_3DNOW: Self = Self(4);
    pub const SCALEPLAT_ALTIVEC: Self = Self(5);
    pub const SCALEPLAT_C_RGBA: Self = Self(6);
    pub const SCALEPLAT_C_BGRA: Self = Self(7);
    pub const SCALEPLAT_C_ARGB: Self = Self(8);
    pub const SCALEPLAT_C_ABGR: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const YUV_XFORM_R: Self = Self(0);
    pub const YUV_XFORM_G: Self = Self(1);
    pub const YUV_XFORM_B: Self = Self(2);
    pub const YUV_XFORM_Y: Self = Self(0);
    pub const YUV_XFORM_U: Self = Self(1);
    pub const YUV_XFORM_V: Self = Self(2);
}
#[no_mangle]
pub static mut YUV_matrix: [[::core::ffi::c_int; 3]; 3] = [
    [
        9794 as ::core::ffi::c_int,
        -2764 as ::core::ffi::c_int,
        8192 as ::core::ffi::c_int,
    ],
    [
        19224 as ::core::ffi::c_int,
        -5428 as ::core::ffi::c_int,
        -6853 as ::core::ffi::c_int,
    ],
    [3749 as ::core::ffi::c_int, 8192 as ::core::ffi::c_int, -1339 as ::core::ffi::c_int],
];
#[no_mangle]
pub static mut RGB_to_YUV: [[[::core::ffi::c_int; 256]; 3]; 3] = [[[0; 256]; 3]; 3];
#[no_mangle]
pub static mut dRGB_to_dYUV: [[[sint16; 512]; 3]; 3] = [[[0; 512]; 3]; 3];
#[no_mangle]
pub static mut RGB15_to_YUV: [YUV_VECTOR; 32768] = [0; 32768];
#[no_mangle]
pub static mut force_platform: PLATFORM_TYPE = PLATFORM_TYPE::PLATFORM_NULL;
#[no_mangle]
pub static mut Scale_Platform: Scale_PlatType_t = Scale_PlatType_t::SCALEPLAT_NULL;
#[no_mangle]
pub unsafe extern "C" fn Scale_Init() {
    let mut i1: ::core::ffi::c_int = 0;
    let mut i2: ::core::ffi::c_int = 0;
    let mut i3: ::core::ffi::c_int = 0;
    i1 = 0 as ::core::ffi::c_int;
    while i1 < 3 as ::core::ffi::c_int {
        i2 = 0 as ::core::ffi::c_int;
        while i2 < 3 as ::core::ffi::c_int {
            i3 = 0 as ::core::ffi::c_int;
            while i3 < 256 as ::core::ffi::c_int {
                RGB_to_YUV[i1 as usize][i2 as usize][i3 as usize] = YUV_matrix[i1
                    as usize][i2 as usize] * i3 >> 14 as ::core::ffi::c_int;
                i3 += 1;
            }
            i2 += 1;
        }
        i1 += 1;
    }
    i1 = 0 as ::core::ffi::c_int;
    while i1 < 3 as ::core::ffi::c_int {
        i2 = 0 as ::core::ffi::c_int;
        while i2 < 3 as ::core::ffi::c_int {
            i3 = -255 as ::core::ffi::c_int;
            while i3 < 256 as ::core::ffi::c_int {
                dRGB_to_dYUV[i1
                    as usize][i2 as usize][(i3 + 255 as ::core::ffi::c_int) as usize] = (YUV_matrix[i1
                    as usize][i2 as usize] * i3 >> 14 as ::core::ffi::c_int) as sint16;
                i3 += 1;
            }
            i2 += 1;
        }
        i1 += 1;
    }
    i1 = 0 as ::core::ffi::c_int;
    while i1 < 32 as ::core::ffi::c_int {
        i2 = 0 as ::core::ffi::c_int;
        while i2 < 32 as ::core::ffi::c_int {
            i3 = 0 as ::core::ffi::c_int;
            while i3 < 32 as ::core::ffi::c_int {
                let mut y: ::core::ffi::c_int = 0;
                let mut u: ::core::ffi::c_int = 0;
                let mut v: ::core::ffi::c_int = 0;
                let mut r: ::core::ffi::c_int = i1 << 3 as ::core::ffi::c_int
                    | i1 >> 3 as ::core::ffi::c_int;
                let mut g: ::core::ffi::c_int = i2 << 3 as ::core::ffi::c_int
                    | i2 >> 3 as ::core::ffi::c_int;
                let mut b: ::core::ffi::c_int = i3 << 3 as ::core::ffi::c_int
                    | i3 >> 3 as ::core::ffi::c_int;
                y = r
                    * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_R.0 as ::core::ffi::c_int
                        as usize][C2Rust_Unnamed::YUV_XFORM_Y.0 as ::core::ffi::c_int
                        as usize]
                    + g
                        * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_G.0 as ::core::ffi::c_int
                            as usize][C2Rust_Unnamed::YUV_XFORM_Y.0 as ::core::ffi::c_int
                            as usize]
                    + b
                        * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_B.0 as ::core::ffi::c_int
                            as usize][C2Rust_Unnamed::YUV_XFORM_Y.0 as ::core::ffi::c_int
                            as usize] >> 15 as ::core::ffi::c_int;
                u = 64 as ::core::ffi::c_int
                    + (r
                        * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_R.0 as ::core::ffi::c_int
                            as usize][C2Rust_Unnamed::YUV_XFORM_U.0 as ::core::ffi::c_int
                            as usize]
                        + g
                            * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_G.0
                                as ::core::ffi::c_int
                                as usize][C2Rust_Unnamed::YUV_XFORM_U.0
                                as ::core::ffi::c_int as usize]
                        + b
                            * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_B.0
                                as ::core::ffi::c_int
                                as usize][C2Rust_Unnamed::YUV_XFORM_U.0
                                as ::core::ffi::c_int as usize]
                        >> 15 as ::core::ffi::c_int);
                v = 64 as ::core::ffi::c_int
                    + (r
                        * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_R.0 as ::core::ffi::c_int
                            as usize][C2Rust_Unnamed::YUV_XFORM_V.0 as ::core::ffi::c_int
                            as usize]
                        + g
                            * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_G.0
                                as ::core::ffi::c_int
                                as usize][C2Rust_Unnamed::YUV_XFORM_V.0
                                as ::core::ffi::c_int as usize]
                        + b
                            * YUV_matrix[C2Rust_Unnamed::YUV_XFORM_B.0
                                as ::core::ffi::c_int
                                as usize][C2Rust_Unnamed::YUV_XFORM_V.0
                                as ::core::ffi::c_int as usize]
                        >> 15 as ::core::ffi::c_int);
                RGB15_to_YUV[(i1 << 10 as ::core::ffi::c_int
                    | i2 << 5 as ::core::ffi::c_int | i3) as usize] = (y
                    << 16 as ::core::ffi::c_int | u << 8 as ::core::ffi::c_int | v)
                    as YUV_VECTOR;
                i3 += 1;
            }
            i2 += 1;
        }
        i1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Scale_ExpandRect(
    mut rect: *mut SDL_Rect,
    mut expansion: ::core::ffi::c_int,
    mut limits: *const SDL_Rect,
) {
    if (*rect).x - expansion >= (*limits).x {
        (*rect).w += expansion;
        (*rect).x -= expansion;
    } else {
        (*rect).w += (*rect).x - (*limits).x;
        (*rect).x = (*limits).x;
    }
    if (*rect).y - expansion >= (*limits).y {
        (*rect).h += expansion;
        (*rect).y -= expansion;
    } else {
        (*rect).h += (*rect).y - (*limits).y;
        (*rect).y = (*limits).y;
    }
    if (*rect).x + (*rect).w + expansion <= (*limits).w {
        (*rect).w += expansion;
    } else {
        (*rect).w = (*limits).w - (*rect).x;
    }
    if (*rect).y + (*rect).h + expansion <= (*limits).h {
        (*rect).h += expansion;
    } else {
        (*rect).h = (*limits).h - (*rect).y;
    };
}
static mut Scale_PlatDefs: [Scale_PlatDef_t; 1] = unsafe {
    [
        Scale_PlatDef_t {
            platform: Scale_PlatType_t::SCALEPLAT_NULL,
            funcdefs: &raw const Scale_C_Functions as *const Scale_FuncDef_t,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn Scale_PrepPlatform(
    mut flags: ::core::ffi::c_int,
    mut fmt: *const SDL_PixelFormat,
) -> TFB_ScaleFunc {
    let mut pdef: *const Scale_PlatDef_t = ::core::ptr::null::<Scale_PlatDef_t>();
    let mut fdef: *const Scale_FuncDef_t = ::core::ptr::null::<Scale_FuncDef_t>();
    Scale_Platform = Scale_PlatType_t::SCALEPLAT_NULL;
    if Scale_Platform.0 == Scale_PlatType_t::SCALEPLAT_NULL.0 {
        if (*fmt).Rmask == 0xff000000 as ::core::ffi::c_uint
            && (*fmt).Bmask == 0xff00 as Uint32
        {
            Scale_Platform = Scale_PlatType_t::SCALEPLAT_C_RGBA;
        } else if (*fmt).Rmask == 0xff0000 as ::core::ffi::c_int as Uint32
            && (*fmt).Bmask == 0xff as Uint32
        {
            Scale_Platform = Scale_PlatType_t::SCALEPLAT_C_ARGB;
        } else if (*fmt).Rmask == 0xff00 as Uint32
            && (*fmt).Bmask == 0xff000000 as ::core::ffi::c_uint
        {
            Scale_Platform = Scale_PlatType_t::SCALEPLAT_C_BGRA;
        } else if (*fmt).Rmask == 0xff as Uint32
            && (*fmt).Bmask == 0xff0000 as ::core::ffi::c_int as Uint32
        {
            Scale_Platform = Scale_PlatType_t::SCALEPLAT_C_ABGR;
        } else {
            log_add(
                log_Level::log_Warning,
                b"Scale_PrepPlatform(): unknown masks (Red %08x, Blue %08x)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*fmt).Rmask,
                (*fmt).Bmask,
            );
            Scale_Platform = Scale_PlatType_t::SCALEPLAT_C;
        }
        if Scale_Platform.0 == Scale_PlatType_t::SCALEPLAT_C.0 {
            log_add(
                log_Level::log_Info,
                b"Screen scalers are using slow generic C code\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            log_add(
                log_Level::log_Info,
                b"Screen scalers are using optimized C code\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    pdef = &raw const Scale_PlatDefs as *const Scale_PlatDef_t;
    while (*pdef).platform.0 != Scale_Platform.0
        && (*pdef).platform.0 != Scale_PlatType_t::SCALEPLAT_NULL.0
    {
        pdef = pdef.offset(1);
    }
    fdef = (*pdef).funcdefs;
    while flags & (*fdef).flag != (*fdef).flag {
        fdef = fdef.offset(1);
    }
    return (*fdef).func;
}
