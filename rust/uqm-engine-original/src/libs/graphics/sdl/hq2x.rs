//hq2x filter
//--------------------------------------------------------------------------
//Copyright (C) 2003 MaxSt ( maxst@hiend3d.com ) - Original version
//
//Portions Copyright (C) 2005 Alex Volkov ( codepro@usa.net )
//   Modified Oct-2-2005

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

// Core algorithm of the HQ screen scaler
//	adapted from hq2x -- www.hiend3d.com/hq2x.html
//  Template
//    When this file is built standalone is produces a plain C version
//    Also #included by 2xscalers_mmx.c for an MMX version

// Rust translation of src/libs/graphics/sdl/hq2x.c.
extern "C" {
    pub type SDL_BlitMap;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut RGB15_to_YUV: [YUV_VECTOR; 32768];
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
pub type sint32 = ::core::ffi::c_int;
pub type YUV_VECTOR = Uint32;
#[inline]
unsafe extern "C" fn Scale_PlatInit() {}
#[inline]
unsafe extern "C" fn Scale_PlatDone() {}
#[inline]
unsafe extern "C" fn Scale_RGBtoYUV(
    mut fmt: *const SDL_PixelFormat,
    mut pix: Uint32,
) -> YUV_VECTOR {
    return RGB15_to_YUV[((pix
        >> (*fmt).Rshift as ::core::ffi::c_int + 3 as ::core::ffi::c_int
        & 0x1f as Uint32) << 10 as ::core::ffi::c_int
        | (pix >> (*fmt).Gshift as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            & 0x1f as Uint32) << 5 as ::core::ffi::c_int
        | pix >> (*fmt).Bshift as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            & 0x1f as Uint32) as usize];
}
#[inline]
unsafe extern "C" fn Scale_DiffYUV(
    mut yuv1: Uint32,
    mut yuv2: Uint32,
) -> ::core::ffi::c_int {
    let mut delta: sint32 = 0;
    let mut ret: sint32 = 0;
    if yuv1 == yuv2 {
        return 0 as ::core::ffi::c_int;
    }
    delta = abs(
        (yuv1 & 0xff0000 as ::core::ffi::c_int as Uint32)
            .wrapping_sub(yuv2 & 0xff0000 as ::core::ffi::c_int as Uint32)
            as ::core::ffi::c_int,
    ) as sint32;
    ret = ((0x40 as sint32) << 16 as ::core::ffi::c_int) - delta;
    delta = abs(
        (yuv1 & 0xff00 as Uint32).wrapping_sub(yuv2 & 0xff00 as Uint32)
            as ::core::ffi::c_int,
    ) as sint32;
    ret |= (((0x12 as sint32) << 8 as ::core::ffi::c_int) - delta) as ::core::ffi::c_int;
    delta = abs(
        (yuv1 & 0xff as Uint32).wrapping_sub(yuv2 & 0xff as Uint32) as ::core::ffi::c_int,
    ) as sint32;
    ret |= (0xc as sint32 - delta) as ::core::ffi::c_int;
    return ret >> 31 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn Scale_Blend_31(mut pix1: Uint32, mut pix2: Uint32) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix1 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn Scale_Blend_211(
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut pix3: Uint32,
) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn Scale_Blend_521(
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut pix3: Uint32,
) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix1 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add(0x2020202 as ::core::ffi::c_int as Uint32);
}
#[inline]
unsafe extern "C" fn Scale_Blend_611(
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut pix3: Uint32,
) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix1 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add(0x2020202 as ::core::ffi::c_int as Uint32);
}
#[inline]
unsafe extern "C" fn Scale_Blend_233(
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut pix3: Uint32,
) -> Uint32 {
    return ((pix1 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add(0x2020202 as ::core::ffi::c_int as Uint32);
}
#[inline]
unsafe extern "C" fn Scale_Blend_e11(
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut pix3: Uint32,
) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix1 & 0xfcfcfcfc as Uint32) >> 2 as ::core::ffi::c_int)
        .wrapping_add((pix1 & 0xf8f8f8f8 as Uint32) >> 3 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xf0f0f0f0 as Uint32) >> 4 as ::core::ffi::c_int)
        .wrapping_add((pix3 & 0xf0f0f0f0 as Uint32) >> 4 as ::core::ffi::c_int)
        .wrapping_add(0x3030303 as ::core::ffi::c_int as Uint32);
}
#[no_mangle]
pub unsafe extern "C" fn Scale_HqFilter(
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
    let mut prevline: ::core::ffi::c_int = 0;
    let mut nextline: ::core::ffi::c_int = 0;
    let mut pix: [Uint32; 10] = [0; 10];
    let mut yuv: [Uint32; 10] = [0; 10];
    Scale_PlatInit();
    limits.x = 0 as ::core::ffi::c_int;
    limits.y = 0 as ::core::ffi::c_int;
    limits.w = (*src).w;
    limits.h = (*src).h;
    Scale_ExpandRect(region, 1 as ::core::ffi::c_int, &raw mut limits);
    xend = (*region).x + (*region).w;
    yend = (*region).y + (*region).h;
    dsrc = slen - (*region).w;
    ddst = (dlen - (*region).w) * 2 as ::core::ffi::c_int;
    src_p = src_p.offset((slen * (*region).y + (*region).x) as isize);
    dst_p = dst_p
        .offset(((dlen * (*region).y + (*region).x) * 2 as ::core::ffi::c_int) as isize);
    y = (*region).y;
    while y < yend {
        if y > 0 as ::core::ffi::c_int {
            prevline = -slen;
        } else {
            prevline = 0 as ::core::ffi::c_int;
        }
        if y < h - 1 as ::core::ffi::c_int {
            nextline = slen;
        } else {
            nextline = 0 as ::core::ffi::c_int;
        }
        pix[3usize] = *src_p.offset(prevline as isize);
        pix[6usize] = *src_p.offset(0isize);
        pix[9usize] = *src_p.offset(nextline as isize);
        yuv[3usize] = Scale_RGBtoYUV(fmt, pix[3usize]) as Uint32;
        yuv[6usize] = Scale_RGBtoYUV(fmt, pix[6usize]) as Uint32;
        yuv[9usize] = Scale_RGBtoYUV(fmt, pix[9usize]) as Uint32;
        if (*region).x > 0 as ::core::ffi::c_int {
            pix[2usize] = *src_p.offset((prevline - 1 as ::core::ffi::c_int) as isize);
            pix[5usize] = *src_p.offset(-1isize);
            pix[8usize] = *src_p.offset((nextline - 1 as ::core::ffi::c_int) as isize);
            yuv[2usize] = Scale_RGBtoYUV(fmt, pix[2usize]) as Uint32;
            yuv[5usize] = Scale_RGBtoYUV(fmt, pix[5usize]) as Uint32;
            yuv[8usize] = Scale_RGBtoYUV(fmt, pix[8usize]) as Uint32;
        } else {
            pix[2usize] = pix[3usize];
            pix[5usize] = pix[6usize];
            pix[8usize] = pix[9usize];
            yuv[2usize] = yuv[3usize];
            yuv[5usize] = yuv[6usize];
            yuv[8usize] = yuv[9usize];
        }
        x = (*region).x;
        while x < xend {
            let mut pattern: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            pix[1usize] = pix[2usize];
            pix[4usize] = pix[5usize];
            pix[7usize] = pix[8usize];
            yuv[1usize] = yuv[2usize];
            yuv[4usize] = yuv[5usize];
            yuv[7usize] = yuv[8usize];
            pix[2usize] = pix[3usize];
            pix[5usize] = pix[6usize];
            pix[8usize] = pix[9usize];
            yuv[2usize] = yuv[3usize];
            yuv[5usize] = yuv[6usize];
            yuv[8usize] = yuv[9usize];
            if x < w - 1 as ::core::ffi::c_int {
                pix[3usize] = *src_p
                    .offset((prevline + 1 as ::core::ffi::c_int) as isize);
                pix[6usize] = *src_p.offset(1isize);
                pix[9usize] = *src_p
                    .offset((nextline + 1 as ::core::ffi::c_int) as isize);
                yuv[3usize] = Scale_RGBtoYUV(fmt, pix[3usize]) as Uint32;
                yuv[6usize] = Scale_RGBtoYUV(fmt, pix[6usize]) as Uint32;
                yuv[9usize] = Scale_RGBtoYUV(fmt, pix[9usize]) as Uint32;
            } else {
                pix[3usize] = pix[2usize];
                pix[6usize] = pix[5usize];
                pix[9usize] = pix[8usize];
                yuv[3usize] = yuv[2usize];
                yuv[6usize] = yuv[5usize];
                yuv[9usize] = yuv[8usize];
            }
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[1usize]) & 0x1 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[2usize]) & 0x2 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[3usize]) & 0x4 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[4usize]) & 0x8 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[6usize]) & 0x10 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[7usize]) & 0x20 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[8usize]) & 0x40 as ::core::ffi::c_int;
            pattern
                |= Scale_DiffYUV(yuv[5usize], yuv[9usize]) & 0x80 as ::core::ffi::c_int;
            match pattern {
                0 | 1 | 4 | 32 | 128 | 5 | 132 | 160 | 33 | 129 | 36 | 133 | 164 | 161
                | 37 | 165 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                2 | 34 | 130 | 162 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                16 | 17 | 48 | 49 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                64 | 65 | 68 | 69 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                8 | 12 | 136 | 140 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                3 | 35 | 131 | 163 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                6 | 38 | 134 | 166 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                20 | 21 | 52 | 53 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                144 | 145 | 176 | 177 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                192 | 193 | 196 | 197 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                96 | 97 | 100 | 101 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                40 | 44 | 168 | 172 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                9 | 13 | 137 | 141 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                18 | 50 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                80 | 81 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                72 | 76 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                10 | 138 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                66 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                24 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                7 | 39 | 135 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                148 | 149 | 180 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                224 | 228 | 225 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                41 | 169 | 45 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                22 | 54 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                208 | 209 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                104 | 108 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                11 | 139 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                19 | 51 => {
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[2usize],
                            pix[4usize],
                        );
                        *dst_p.offset(1isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                146 | 178 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[8usize],
                        );
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                }
                84 | 85 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[6usize],
                            pix[2usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                }
                112 | 113 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                200 | 204 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[6usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[8usize],
                            pix[6usize],
                        );
                    }
                }
                73 | 77 => {
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(dlen as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                42 | 170 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[8usize],
                        );
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(dlen as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[4usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                14 | 142 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(1isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                67 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                70 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                28 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                152 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                194 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                98 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                56 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                25 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                26 | 31 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                82 | 214 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                88 | 248 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                74 | 107 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                27 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                86 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                216 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                106 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                30 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                210 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                120 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                75 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                29 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                198 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                184 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                99 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                57 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                71 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                156 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                226 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                60 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                195 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                102 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                153 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                58 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                83 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                92 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                202 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                78 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                154 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                114 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                89 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                90 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                55 | 23 => {
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[2usize],
                            pix[4usize],
                        );
                        *dst_p.offset(1isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                182 | 150 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[8usize],
                        );
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                }
                213 | 212 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[6usize],
                            pix[2usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                }
                241 | 240 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                236 | 232 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[6usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[8usize],
                            pix[6usize],
                        );
                    }
                }
                109 | 105 => {
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(dlen as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                171 | 43 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[8usize],
                        );
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(dlen as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[4usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                143 | 15 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(1isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                124 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                203 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                62 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                211 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                118 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                217 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                110 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                155 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                188 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                185 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                61 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                157 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                103 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                227 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                230 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                199 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                220 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                158 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                234 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                242 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                59 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                121 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                87 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                79 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                122 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                94 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                218 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                91 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                229 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                167 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                173 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                181 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                186 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                115 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                93 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                206 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                205 | 201 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[7usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                174 | 46 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                179 | 147 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                117 | 116 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[9usize],
                        );
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_611(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                189 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                231 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                126 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                219 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                125 => {
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(dlen as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                221 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[6usize],
                            pix[2usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                }
                207 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                        *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(1isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                238 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[6usize],
                        );
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[8usize],
                            pix[6usize],
                        );
                    }
                }
                190 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[8usize],
                        );
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                187 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[8usize],
                        );
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                        *dst_p.offset(dlen as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[4usize],
                            pix[8usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                243 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset(dlen as isize) = Scale_Blend_31(
                            pix[5usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                119 => {
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_521(
                            pix[5usize],
                            pix[2usize],
                            pix[4usize],
                        );
                        *dst_p.offset(1isize) = Scale_Blend_233(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                237 | 233 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[2usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                175 | 47 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[6usize],
                        pix[8usize],
                    );
                }
                183 | 151 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[8usize],
                        pix[4usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                245 | 244 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[4usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                250 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                123 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                95 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                222 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                252 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[2usize],
                    );
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                249 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[2usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                235 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[3usize],
                        pix[6usize],
                    );
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                111 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[6usize],
                    );
                }
                63 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[9usize],
                        pix[8usize],
                    );
                }
                159 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                215 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[7usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                246 => {
                    *dst_p.offset(0isize) = Scale_Blend_211(
                        pix[5usize],
                        pix[1usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                254 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[1usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                253 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[2usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                251 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[3usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                239 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    *dst_p.offset(1isize) = Scale_Blend_31(pix[5usize], pix[6usize]);
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[6usize],
                    );
                }
                127 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[9usize],
                    );
                }
                191 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[8usize],
                    );
                }
                223 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[7usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_211(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                247 => {
                    *dst_p.offset(0isize) = Scale_Blend_31(pix[5usize], pix[4usize]);
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    *dst_p.offset(dlen as isize) = Scale_Blend_31(
                        pix[5usize],
                        pix[4usize],
                    );
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                255 => {
                    if Scale_DiffYUV(yuv[4usize], yuv[2usize]) != 0 {
                        *dst_p.offset(0isize) = pix[5usize];
                    } else {
                        *dst_p.offset(0isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[4usize],
                            pix[2usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[2usize], yuv[6usize]) != 0 {
                        *dst_p.offset(1isize) = pix[5usize];
                    } else {
                        *dst_p.offset(1isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[2usize],
                            pix[6usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[8usize], yuv[4usize]) != 0 {
                        *dst_p.offset(dlen as isize) = pix[5usize];
                    } else {
                        *dst_p.offset(dlen as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[8usize],
                            pix[4usize],
                        );
                    }
                    if Scale_DiffYUV(yuv[6usize], yuv[8usize]) != 0 {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix[5usize];
                    } else {
                        *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_e11(
                            pix[5usize],
                            pix[6usize],
                            pix[8usize],
                        );
                    }
                }
                _ => {}
            }
            x += 1;
            src_p = src_p.offset(1);
            dst_p = dst_p.offset(2 as ::core::ffi::c_int as isize);
        }
        y += 1;
        dst_p = dst_p.offset(ddst as isize);
        src_p = src_p.offset(dsrc as isize);
    }
    Scale_PlatDone();
}
