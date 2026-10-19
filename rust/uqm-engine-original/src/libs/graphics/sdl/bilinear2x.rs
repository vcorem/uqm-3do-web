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

// Core algorithm of the BiLinear screen scaler
//  Template
//    When this file is built standalone is produces a plain C version
//    Also #included by 2xscalers_mmx.c for an MMX version

// Rust translation of src/libs/graphics/sdl/bilinear2x.c.
extern "C" {
    pub type SDL_BlitMap;
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
#[inline]
unsafe extern "C" fn Scale_PlatInit() {}
#[inline]
unsafe extern "C" fn Scale_PlatDone() {}
#[inline]
unsafe extern "C" fn Scale_Blend_bilinear(
    mut row0: *const Uint32,
    mut row1: *const Uint32,
    mut dst_p: *mut Uint32,
    mut dlen: Uint32,
) {
    let mut sum1111: Uint32 = 0;
    let mut sum1331: Uint32 = 0;
    let mut sum3113: Uint32 = 0;
    sum1331 = ((*row0.offset(1isize) & 0xfefefefe as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int)
        .wrapping_add(
            (*row1.offset(0isize) & 0xfefefefe as ::core::ffi::c_uint)
                >> 1 as ::core::ffi::c_int,
        ) as Uint32;
    sum3113 = ((*row0.offset(0isize) & 0xfefefefe as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int)
        .wrapping_add(
            (*row1.offset(1isize) & 0xfefefefe as ::core::ffi::c_uint)
                >> 1 as ::core::ffi::c_int,
        ) as Uint32;
    sum1111 = ((sum1331 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int)
        .wrapping_add(
            (sum3113 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
        )
        .wrapping_add(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint) as Uint32;
    sum1331 = ((sum1331 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int)
        .wrapping_add(
            (sum1111 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
        )
        .wrapping_add(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint) as Uint32;
    sum1331 = ((sum1331 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int)
        as Uint32;
    sum3113 = ((sum3113 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int)
        .wrapping_add(
            (sum1111 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
        )
        .wrapping_add(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint) as Uint32;
    sum3113 = ((sum3113 & 0xfefefefe as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int)
        as Uint32;
    *dst_p.offset(0isize) = ((*row0.offset(0isize) & 0xfefefefe as Uint32)
        >> 1 as ::core::ffi::c_int)
        .wrapping_add(sum1331);
    *dst_p.offset(1isize) = ((*row0.offset(1isize) & 0xfefefefe as Uint32)
        >> 1 as ::core::ffi::c_int)
        .wrapping_add(sum3113);
    *dst_p.offset(dlen as isize) = ((*row1.offset(0isize) & 0xfefefefe as Uint32)
        >> 1 as ::core::ffi::c_int)
        .wrapping_add(sum3113);
    *dst_p.offset(dlen.wrapping_add(1 as Uint32) as isize) = ((*row1.offset(1isize)
        & 0xfefefefe as Uint32) >> 1 as ::core::ffi::c_int)
        .wrapping_add(sum1331);
}
#[no_mangle]
pub unsafe extern "C" fn Scale_BilinearFilter(
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
    let pitch: ::core::ffi::c_int = (*src).pitch;
    let dp: ::core::ffi::c_int = (*dst).pitch;
    let bpp: ::core::ffi::c_int = (*fmt).BytesPerPixel as ::core::ffi::c_int;
    let len: ::core::ffi::c_int = pitch / bpp;
    let dlen: ::core::ffi::c_int = dp / bpp;
    let mut p: [Uint32; 4] = [0; 4];
    let mut srow0: *mut Uint32 = (*src).pixels as *mut Uint32;
    let mut dst_p: *mut Uint32 = (*dst).pixels as *mut Uint32;
    Scale_PlatInit();
    limits.x = 0 as ::core::ffi::c_int;
    limits.y = 0 as ::core::ffi::c_int;
    limits.w = w;
    limits.h = h;
    Scale_ExpandRect(region, 1 as ::core::ffi::c_int, &raw mut limits);
    xend = (*region).x + (*region).w;
    yend = (*region).y + (*region).h;
    dsrc = len - (*region).w;
    ddst = (dlen - (*region).w) * 2 as ::core::ffi::c_int;
    srow0 = srow0.offset((len * (*region).y + (*region).x) as isize);
    dst_p = dst_p
        .offset(((dlen * (*region).y + (*region).x) * 2 as ::core::ffi::c_int) as isize);
    y = (*region).y;
    while y < yend {
        let mut srow1: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
        if y < h - 1 as ::core::ffi::c_int {
            srow1 = srow0.offset(len as isize);
        } else {
            srow1 = srow0;
        }
        x = (*region).x;
        while x < xend {
            if x < w - 1 as ::core::ffi::c_int {
                Scale_Blend_bilinear(srow0, srow1, dst_p, dlen as Uint32);
            } else {
                p[0usize] = *srow0.offset(0isize);
                p[1usize] = p[0usize];
                p[2usize] = *srow1.offset(0isize);
                p[3usize] = p[2usize];
                Scale_Blend_bilinear(
                    (&raw mut p as *mut Uint32).offset(0isize),
                    (&raw mut p as *mut Uint32).offset(2isize),
                    dst_p,
                    dlen as Uint32,
                );
            }
            x += 1;
            srow0 = srow0.offset(1);
            srow1 = srow1.offset(1);
            dst_p = dst_p.offset(2 as ::core::ffi::c_int as isize);
        }
        y += 1;
        dst_p = dst_p.offset(ddst as isize);
        srow0 = srow0.offset(dsrc as isize);
    }
    Scale_PlatDone();
}
