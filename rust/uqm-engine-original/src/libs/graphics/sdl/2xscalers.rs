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

// Rust translation of src/libs/graphics/sdl/2xscalers.c.
extern "C" {
    pub type SDL_BlitMap;
    fn Scale_Nearest(src: *mut SDL_Surface, dst: *mut SDL_Surface, r: *mut SDL_Rect);
    fn Scale_BilinearFilter(
        src: *mut SDL_Surface,
        dst: *mut SDL_Surface,
        r: *mut SDL_Rect,
    );
    fn Scale_BiAdaptAdvFilter(
        src: *mut SDL_Surface,
        dst: *mut SDL_Surface,
        r: *mut SDL_Rect,
    );
    fn Scale_TriScanFilter(
        src: *mut SDL_Surface,
        dst: *mut SDL_Surface,
        r: *mut SDL_Rect,
    );
    fn Scale_HqFilter(src: *mut SDL_Surface, dst: *mut SDL_Surface, r: *mut SDL_Rect);
    fn Scale_ExpandRect(
        rect: *mut SDL_Rect,
        expansion: ::core::ffi::c_int,
        limits: *const SDL_Rect,
    );
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
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
pub type TFB_ScaleFunc = Option<
    unsafe extern "C" fn(*mut SDL_Surface, *mut SDL_Surface, *mut SDL_Rect) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Scale_FuncDef_t {
    pub flag: ::core::ffi::c_int,
    pub func: TFB_ScaleFunc,
}
#[inline]
unsafe extern "C" fn Scale_PlatInit() {}
#[inline]
unsafe extern "C" fn Scale_PlatDone() {}
#[inline]
unsafe extern "C" fn Scale_Blend_11(mut pix1: Uint32, mut pix2: Uint32) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn Scale_Blend_1111(
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut pix3: Uint32,
    mut pix4: Uint32,
) -> Uint32 {
    return ((pix1 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix4 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int);
}
#[no_mangle]
pub static mut Scale_C_Functions: [Scale_FuncDef_t; 6] = [
    Scale_FuncDef_t {
        flag: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
        func: Some(
            Scale_BilinearFilter
                as unsafe extern "C" fn(
                    *mut SDL_Surface,
                    *mut SDL_Surface,
                    *mut SDL_Rect,
                ) -> (),
        ),
    },
    Scale_FuncDef_t {
        flag: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
        func: Some(
            Scale_BiAdaptFilter
                as unsafe extern "C" fn(
                    *mut SDL_Surface,
                    *mut SDL_Surface,
                    *mut SDL_Rect,
                ) -> (),
        ),
    },
    Scale_FuncDef_t {
        flag: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
        func: Some(
            Scale_BiAdaptAdvFilter
                as unsafe extern "C" fn(
                    *mut SDL_Surface,
                    *mut SDL_Surface,
                    *mut SDL_Rect,
                ) -> (),
        ),
    },
    Scale_FuncDef_t {
        flag: (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
        func: Some(
            Scale_TriScanFilter
                as unsafe extern "C" fn(
                    *mut SDL_Surface,
                    *mut SDL_Surface,
                    *mut SDL_Rect,
                ) -> (),
        ),
    },
    Scale_FuncDef_t {
        flag: (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int,
        func: Some(
            Scale_HqFilter
                as unsafe extern "C" fn(
                    *mut SDL_Surface,
                    *mut SDL_Surface,
                    *mut SDL_Rect,
                ) -> (),
        ),
    },
    Scale_FuncDef_t {
        flag: 0 as ::core::ffi::c_int,
        func: Some(
            Scale_Nearest
                as unsafe extern "C" fn(
                    *mut SDL_Surface,
                    *mut SDL_Surface,
                    *mut SDL_Rect,
                ) -> (),
        ),
    },
];
#[no_mangle]
pub unsafe extern "C" fn Scale_BiAdaptFilter(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
    mut r: *mut SDL_Rect,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let w: ::core::ffi::c_int = (*src).w;
    let h: ::core::ffi::c_int = (*src).h;
    let mut xend: ::core::ffi::c_int = 0;
    let mut yend: ::core::ffi::c_int = 0;
    let mut dsrc: ::core::ffi::c_int = 0;
    let mut ddst: ::core::ffi::c_int = 0;
    let mut region: *mut SDL_Rect = r;
    let mut limits: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut fmt: *mut SDL_PixelFormat = (*dst).format;
    let sp: ::core::ffi::c_int = (*src).pitch;
    let dp: ::core::ffi::c_int = (*dst).pitch;
    let bpp: ::core::ffi::c_int = (*fmt).BytesPerPixel as ::core::ffi::c_int;
    let slen: ::core::ffi::c_int = sp / bpp;
    let dlen: ::core::ffi::c_int = dp / bpp;
    let mut src_p: *mut Uint32 = (*src).pixels as *mut Uint32;
    let mut dst_p: *mut Uint32 = (*dst).pixels as *mut Uint32;
    let mut pixval_tl: Uint32 = 0;
    let mut pixval_tr: Uint32 = 0;
    let mut pixval_bl: Uint32 = 0;
    let mut pixval_br: Uint32 = 0;
    Scale_PlatInit();
    limits.x = 0 as ::core::ffi::c_int;
    limits.y = 0 as ::core::ffi::c_int;
    limits.w = (*src).w;
    limits.h = (*src).h;
    Scale_ExpandRect(region, 2 as ::core::ffi::c_int, &raw mut limits);
    xend = (*region).x + (*region).w;
    yend = (*region).y + (*region).h;
    dsrc = slen - (*region).w;
    ddst = (dlen - (*region).w) * 2 as ::core::ffi::c_int;
    src_p = src_p.offset((slen * (*region).y + (*region).x) as isize);
    dst_p = dst_p
        .offset(((dlen * (*region).y + (*region).x) * 2 as ::core::ffi::c_int) as isize);
    y = (*region).y;
    while y < yend {
        x = (*region).x;
        while x < xend {
            pixval_tl = *src_p
                .offset(0 as ::core::ffi::c_int as isize)
                .offset((0 as ::core::ffi::c_int * slen) as isize);
            *dst_p = pixval_tl;
            if (y + 1 as ::core::ffi::c_int) < h {
                pixval_bl = *src_p
                    .offset(0 as ::core::ffi::c_int as isize)
                    .offset((1 as ::core::ffi::c_int * slen) as isize);
                if pixval_tl == pixval_bl {
                    *dst_p.offset(dlen as isize) = pixval_tl;
                } else {
                    *dst_p.offset(dlen as isize) = Scale_Blend_11(pixval_tl, pixval_bl);
                }
            } else {
                *dst_p.offset(dlen as isize) = pixval_tl;
                pixval_bl = pixval_tl;
            }
            dst_p = dst_p.offset(1);
            if x + 1 as ::core::ffi::c_int >= w {
                *dst_p = pixval_tl;
                if pixval_tl == pixval_bl {
                    *dst_p.offset(dlen as isize) = pixval_tl;
                } else {
                    *dst_p.offset(dlen as isize) = Scale_Blend_11(pixval_tl, pixval_bl);
                }
            } else {
                pixval_tr = *src_p
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((0 as ::core::ffi::c_int * slen) as isize);
                if pixval_tl == pixval_tr {
                    *dst_p = pixval_tr;
                } else {
                    *dst_p = Scale_Blend_11(pixval_tl, pixval_tr);
                }
                if y + 1 as ::core::ffi::c_int >= h {
                    *dst_p.offset(dlen as isize) = pixval_tl;
                } else {
                    pixval_br = *src_p
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset((1 as ::core::ffi::c_int * slen) as isize);
                    if pixval_tl == pixval_br && pixval_tr == pixval_bl {
                        let mut cl: ::core::ffi::c_int = 0;
                        let mut cr: ::core::ffi::c_int = 0;
                        let mut clr: Uint32 = 0;
                        if pixval_tl == pixval_tr {
                            *dst_p.offset(dlen as isize) = pixval_tl;
                        } else {
                            cr = 1 as ::core::ffi::c_int;
                            cl = cr;
                            if x > 0 as ::core::ffi::c_int {
                                clr = *src_p
                                    .offset(-1 as ::core::ffi::c_int as isize)
                                    .offset((0 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                                clr = *src_p
                                    .offset(-1 as ::core::ffi::c_int as isize)
                                    .offset((1 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                            }
                            if y > 0 as ::core::ffi::c_int {
                                clr = *src_p
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    .offset((-1 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                                clr = *src_p
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    .offset((-1 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                            }
                            if (x + 2 as ::core::ffi::c_int) < w {
                                clr = *src_p
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset((0 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                                clr = *src_p
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset((1 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                            }
                            if (y + 2 as ::core::ffi::c_int) < h {
                                clr = *src_p
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    .offset((2 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                                clr = *src_p
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    .offset((2 as ::core::ffi::c_int * slen) as isize);
                                if clr == pixval_tl {
                                    cl += 1;
                                } else if clr == pixval_tr {
                                    cr += 1;
                                }
                            }
                            if cl > cr {
                                *dst_p.offset(dlen as isize) = pixval_tr;
                            } else if cr > cl {
                                *dst_p.offset(dlen as isize) = pixval_tl;
                            } else {
                                *dst_p.offset(dlen as isize) = Scale_Blend_11(
                                    pixval_tl,
                                    pixval_tr,
                                );
                            }
                        }
                    } else if pixval_tl == pixval_br {
                        *dst_p.offset(dlen as isize) = pixval_tl;
                    } else if pixval_tr == pixval_bl {
                        *dst_p.offset(dlen as isize) = pixval_tr;
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_1111(
                            pixval_tl,
                            pixval_bl,
                            pixval_tr,
                            pixval_br,
                        );
                    }
                }
            }
            x += 1;
            src_p = src_p.offset(1);
            dst_p = dst_p.offset(1);
        }
        y += 1;
        dst_p = dst_p.offset(ddst as isize);
        src_p = src_p.offset(dsrc as isize);
    }
    Scale_PlatDone();
}
