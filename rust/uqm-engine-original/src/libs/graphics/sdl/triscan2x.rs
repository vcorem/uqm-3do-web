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

// Core algorithm of the Triscan screen scaler (based on Scale2x)
//     (for scale2x please see http://scale2x.sf.net)
//  Template
//    When this file is built standalone is produces a plain C version
//    Also #included by 2xscalers_mmx.c for an MMX version

// Rust translation of src/libs/graphics/sdl/triscan2x.c.
extern "C" {
    pub type SDL_BlitMap;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut dRGB_to_dYUV: [[[sint16; 512]; 3]; 3];
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
pub type sint16 = ::core::ffi::c_short;
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
#[inline]
unsafe extern "C" fn Scale_PlatInit() {}
#[inline]
unsafe extern "C" fn Scale_PlatDone() {}
#[inline]
unsafe extern "C" fn Scale_CmpYUV(
    mut fmt: *const SDL_PixelFormat,
    mut pix1: Uint32,
    mut pix2: Uint32,
    mut toler: ::core::ffi::c_int,
) -> bool {
    let mut dr: ::core::ffi::c_int = 0;
    let mut dg: ::core::ffi::c_int = 0;
    let mut db: ::core::ffi::c_int = 0;
    let mut delta: ::core::ffi::c_int = 0;
    dr = (pix1 >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32)
        .wrapping_sub(pix2 >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32)
        .wrapping_add(255 as Uint32) as ::core::ffi::c_int;
    dg = (pix1 >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32)
        .wrapping_sub(pix2 >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32)
        .wrapping_add(255 as Uint32) as ::core::ffi::c_int;
    db = (pix1 >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32)
        .wrapping_sub(pix2 >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32)
        .wrapping_add(255 as Uint32) as ::core::ffi::c_int;
    delta = abs(
        dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_R.0 as ::core::ffi::c_int
            as usize][C2Rust_Unnamed::YUV_XFORM_Y.0 as ::core::ffi::c_int
            as usize][dr as usize] as ::core::ffi::c_int
            + dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_G.0 as ::core::ffi::c_int
                as usize][C2Rust_Unnamed::YUV_XFORM_Y.0 as ::core::ffi::c_int
                as usize][dg as usize] as ::core::ffi::c_int
            + dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_B.0 as ::core::ffi::c_int
                as usize][C2Rust_Unnamed::YUV_XFORM_Y.0 as ::core::ffi::c_int
                as usize][db as usize] as ::core::ffi::c_int,
    );
    if delta > toler {
        return false;
    }
    delta
        += abs(
            dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_R.0 as ::core::ffi::c_int
                as usize][C2Rust_Unnamed::YUV_XFORM_U.0 as ::core::ffi::c_int
                as usize][dr as usize] as ::core::ffi::c_int
                + dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_G.0 as ::core::ffi::c_int
                    as usize][C2Rust_Unnamed::YUV_XFORM_U.0 as ::core::ffi::c_int
                    as usize][dg as usize] as ::core::ffi::c_int
                + dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_B.0 as ::core::ffi::c_int
                    as usize][C2Rust_Unnamed::YUV_XFORM_U.0 as ::core::ffi::c_int
                    as usize][db as usize] as ::core::ffi::c_int,
        );
    if delta > toler {
        return false;
    }
    delta
        += abs(
            dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_R.0 as ::core::ffi::c_int
                as usize][C2Rust_Unnamed::YUV_XFORM_V.0 as ::core::ffi::c_int
                as usize][dr as usize] as ::core::ffi::c_int
                + dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_G.0 as ::core::ffi::c_int
                    as usize][C2Rust_Unnamed::YUV_XFORM_V.0 as ::core::ffi::c_int
                    as usize][dg as usize] as ::core::ffi::c_int
                + dRGB_to_dYUV[C2Rust_Unnamed::YUV_XFORM_B.0 as ::core::ffi::c_int
                    as usize][C2Rust_Unnamed::YUV_XFORM_V.0 as ::core::ffi::c_int
                    as usize][db as usize] as ::core::ffi::c_int,
        );
    return delta <= toler;
}
#[inline]
unsafe extern "C" fn Scale_Blend_11(mut pix1: Uint32, mut pix2: Uint32) -> Uint32 {
    return ((pix1 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add((pix2 & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Scale_TriScanFilter(
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
    let mut pixels: [[Uint32; 3]; 3] = [[0; 3]; 3];
    let mut src_p: *mut Uint32 = (*src).pixels as *mut Uint32;
    let mut dst_p: *mut Uint32 = (*dst).pixels as *mut Uint32;
    let mut prevline: ::core::ffi::c_int = 0;
    let mut nextline: ::core::ffi::c_int = 0;
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
        pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize] = *src_p
            .offset(0isize);
        if (*region).x > 0 as ::core::ffi::c_int {
            pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize] = *src_p.offset(-1isize);
        } else {
            pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize] = pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize];
        }
        x = (*region).x;
        while x < xend {
            pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize] = pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize];
            pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                as usize] = *src_p.offset(prevline as isize);
            pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize] = pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize];
            pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize] = *src_p.offset(nextline as isize);
            if x < w - 1 as ::core::ffi::c_int {
                pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize] = *src_p.offset(1isize);
            } else {
                pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize] = pixels[(1 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize];
            }
            if !(pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as usize][(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int) as usize]
                == pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize]
                || Scale_CmpYUV(
                    fmt,
                    pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                        as usize],
                    pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as usize],
                    100 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0)
                && !(pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize]
                    == pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize]
                    || Scale_CmpYUV(
                        fmt,
                        pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        100 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0)
            {
                if pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize]
                    == pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                        as usize]
                    || Scale_CmpYUV(
                        fmt,
                        pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int
                            + -1 as ::core::ffi::c_int) as usize],
                        100 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0
                {
                    *dst_p.offset(0isize) = Scale_Blend_11(
                        pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int
                            + -1 as ::core::ffi::c_int) as usize],
                    );
                } else {
                    *dst_p.offset(0isize) = pixels[(1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize];
                }
                if pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize]
                    == pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                        as usize]
                    || Scale_CmpYUV(
                        fmt,
                        pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int
                            + -1 as ::core::ffi::c_int) as usize],
                        100 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0
                {
                    *dst_p.offset(1isize) = Scale_Blend_11(
                        pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int
                            + -1 as ::core::ffi::c_int) as usize],
                    );
                } else {
                    *dst_p.offset(1isize) = pixels[(1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize];
                }
                if pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize]
                    == pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as usize]
                    || Scale_CmpYUV(
                        fmt,
                        pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize],
                        100 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0
                {
                    *dst_p.offset(dlen as isize) = Scale_Blend_11(
                        pixels[(1 as ::core::ffi::c_int + -1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize],
                    );
                } else {
                    *dst_p.offset(dlen as isize) = pixels[(1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize];
                }
                if pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize]
                    == pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as usize]
                    || Scale_CmpYUV(
                        fmt,
                        pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize],
                        100 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0
                {
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = Scale_Blend_11(
                        pixels[(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize],
                        pixels[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                            as usize][(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize],
                    );
                } else {
                    *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pixels[(1
                        as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                        as usize];
                }
            } else {
                *dst_p.offset(0isize) = pixels[(1 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize];
                *dst_p.offset(1isize) = pixels[(1 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize];
                *dst_p.offset(dlen as isize) = pixels[(1 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize];
                *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pixels[(1
                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize][(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                    as usize];
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
