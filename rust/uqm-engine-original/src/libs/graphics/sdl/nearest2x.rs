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

// Rust translation of src/libs/graphics/sdl/nearest2x.c.
extern "C" {
    pub type SDL_BlitMap;
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
#[no_mangle]
pub unsafe extern "C" fn Scale_Nearest(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
    mut r: *mut SDL_Rect,
) {
    let mut y: ::core::ffi::c_int = 0;
    let rw: ::core::ffi::c_int = (*r).w;
    let rh: ::core::ffi::c_int = (*r).h;
    let sp: ::core::ffi::c_int = (*src).pitch;
    let dp: ::core::ffi::c_int = (*dst).pitch;
    let bpp: ::core::ffi::c_int = (*(*dst).format).BytesPerPixel as ::core::ffi::c_int;
    let slen: ::core::ffi::c_int = sp / bpp;
    let dlen: ::core::ffi::c_int = dp / bpp;
    let dsrc: ::core::ffi::c_int = slen - rw;
    let ddst: ::core::ffi::c_int = (dlen - rw) * 2 as ::core::ffi::c_int;
    let mut src_p: *mut Uint32 = (*src).pixels as *mut Uint32;
    let mut dst_p: *mut Uint32 = (*dst).pixels as *mut Uint32;
    if rw == 0 as ::core::ffi::c_int || rh == 0 as ::core::ffi::c_int {
        return;
    }
    Scale_PlatInit();
    src_p = src_p.offset((slen * (*r).y + (*r).x) as isize);
    dst_p = dst_p.offset(((dlen * (*r).y + (*r).x) * 2 as ::core::ffi::c_int) as isize);
    y = 0 as ::core::ffi::c_int;
    while y < rh {
        let mut x: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < rw {
            let mut pix: Uint32 = *src_p;
            *dst_p.offset(0isize) = pix;
            *dst_p.offset(1isize) = pix;
            *dst_p.offset(dlen as isize) = pix;
            *dst_p.offset((dlen + 1 as ::core::ffi::c_int) as isize) = pix;
            x += 1;
            src_p = src_p.offset(1);
            dst_p = dst_p.offset(2 as ::core::ffi::c_int as isize);
        }
        dst_p = dst_p.offset(ddst as isize);
        src_p = src_p.offset(dsrc as isize);
        y += 1;
    }
    Scale_PlatDone();
}
#[inline]
unsafe extern "C" fn Scale_PlatInit() {}
#[inline]
unsafe extern "C" fn Scale_PlatDone() {}
