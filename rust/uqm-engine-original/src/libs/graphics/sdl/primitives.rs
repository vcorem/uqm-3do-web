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

// Rust translation of src/libs/graphics/sdl/primitives.c.
extern "C" {
    pub type SDL_BlitMap;
    fn SDL_MapRGBA(
        format: *const SDL_PixelFormat,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> Uint32;
    fn SDL_GetRGBA(
        pixel: Uint32,
        format: *const SDL_PixelFormat,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
        a: *mut Uint8,
    );
    fn SDL_GetClipRect(surface: *mut SDL_Surface, rect: *mut SDL_Rect);
    fn TFB_GetColorKey(
        surface: *mut SDL_Surface,
        key: *mut Uint32,
    ) -> ::core::ffi::c_int;
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type Uint8 = uint8_t;
pub type Uint16 = uint16_t;
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
pub type GetPixelFn = Option<
    unsafe extern "C" fn(
        *mut SDL_Surface,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> Uint32,
>;
pub type PutPixelFn = Option<
    unsafe extern "C" fn(
        *mut SDL_Surface,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        Uint32,
    ) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RenderKind(pub ::core::ffi::c_uint);
impl RenderKind {
    pub const renderReplace: Self = Self(0);
    pub const renderAdditive: Self = Self(1);
    pub const renderAlpha: Self = Self(2);
}
pub type RenderPixelFn = Option<
    unsafe extern "C" fn(
        *mut SDL_Surface,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        Uint32,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const C_TOP: Self = Self(1);
    pub const C_BOTTOM: Self = Self(2);
    pub const C_RIGHT: Self = Self(4);
    pub const C_LEFT: Self = Self(8);
}
unsafe extern "C" fn getpixel_8(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Uint32 {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset(x as isize);
    return *p as Uint32;
}
unsafe extern "C" fn putpixel_8(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
) {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 1 as ::core::ffi::c_int) as isize);
    *p = pixel as Uint8;
}
unsafe extern "C" fn getpixel_16(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Uint32 {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 2 as ::core::ffi::c_int) as isize);
    return *(p as *mut Uint16) as Uint32;
}
unsafe extern "C" fn putpixel_16(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
) {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 2 as ::core::ffi::c_int) as isize);
    *(p as *mut Uint16) = pixel as Uint16;
}
unsafe extern "C" fn getpixel_24_be(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Uint32 {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 3 as ::core::ffi::c_int) as isize);
    return ((*p.offset(0isize) as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
        | (*p.offset(1isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | *p.offset(2isize) as ::core::ffi::c_int) as Uint32;
}
unsafe extern "C" fn putpixel_24_be(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
) {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 3 as ::core::ffi::c_int) as isize);
    *p.offset(0isize) = (pixel >> 16 as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    *p.offset(1isize) = (pixel >> 8 as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    *p.offset(2isize) = (pixel & 0xff as Uint32) as Uint8;
}
unsafe extern "C" fn getpixel_24_le(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Uint32 {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 3 as ::core::ffi::c_int) as isize);
    return (*p.offset(0isize) as ::core::ffi::c_int
        | (*p.offset(1isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (*p.offset(2isize) as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
        as Uint32;
}
unsafe extern "C" fn putpixel_24_le(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
) {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 3 as ::core::ffi::c_int) as isize);
    *p.offset(0isize) = (pixel & 0xff as Uint32) as Uint8;
    *p.offset(1isize) = (pixel >> 8 as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    *p.offset(2isize) = (pixel >> 16 as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
}
unsafe extern "C" fn getpixel_32(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Uint32 {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 4 as ::core::ffi::c_int) as isize);
    return *(p as *mut Uint32);
}
unsafe extern "C" fn putpixel_32(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
) {
    let mut p: *mut Uint8 = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 4 as ::core::ffi::c_int) as isize);
    *(p as *mut Uint32) = pixel;
}
#[no_mangle]
pub unsafe extern "C" fn getpixel_for(mut surface: *mut SDL_Surface) -> GetPixelFn {
    let mut bpp: ::core::ffi::c_int = (*(*surface).format).BytesPerPixel
        as ::core::ffi::c_int;
    match bpp {
        1 => {
            return Some(
                getpixel_8
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> Uint32,
            );
        }
        2 => {
            return Some(
                getpixel_16
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> Uint32,
            );
        }
        3 => {
            if 1234 as ::core::ffi::c_int == 4321 as ::core::ffi::c_int {
                return Some(
                    getpixel_24_be
                        as unsafe extern "C" fn(
                            *mut SDL_Surface,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> Uint32,
                )
            } else {
                return Some(
                    getpixel_24_le
                        as unsafe extern "C" fn(
                            *mut SDL_Surface,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> Uint32,
                )
            }
        }
        4 => {
            return Some(
                getpixel_32
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> Uint32,
            );
        }
        _ => {}
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn putpixel_for(mut surface: *mut SDL_Surface) -> PutPixelFn {
    let mut bpp: ::core::ffi::c_int = (*(*surface).format).BytesPerPixel
        as ::core::ffi::c_int;
    match bpp {
        1 => {
            return Some(
                putpixel_8
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        Uint32,
                    ) -> (),
            );
        }
        2 => {
            return Some(
                putpixel_16
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        Uint32,
                    ) -> (),
            );
        }
        3 => {
            if 1234 as ::core::ffi::c_int == 4321 as ::core::ffi::c_int {
                return Some(
                    putpixel_24_be
                        as unsafe extern "C" fn(
                            *mut SDL_Surface,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            Uint32,
                        ) -> (),
                )
            } else {
                return Some(
                    putpixel_24_le
                        as unsafe extern "C" fn(
                            *mut SDL_Surface,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            Uint32,
                        ) -> (),
                )
            }
        }
        4 => {
            return Some(
                putpixel_32
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        Uint32,
                    ) -> (),
            );
        }
        _ => {}
    }
    return None;
}
unsafe extern "C" fn renderpixel_replace(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
    mut factor: ::core::ffi::c_int,
) {
    putpixel_32(surface, x, y, pixel);
}
#[inline]
unsafe extern "C" fn clip_channel(mut c: ::core::ffi::c_int) -> Uint8 {
    if c < 0 as ::core::ffi::c_int {
        c = 0 as ::core::ffi::c_int;
    } else if c > 255 as ::core::ffi::c_int {
        c = 255 as ::core::ffi::c_int;
    }
    return c as Uint8;
}
#[inline]
unsafe extern "C" fn modulated_sum(
    mut dc: Uint8,
    mut sc: Uint8,
    mut factor: ::core::ffi::c_int,
) -> Uint8 {
    let mut b: ::core::ffi::c_int = dc as ::core::ffi::c_int
        + (sc as ::core::ffi::c_int * factor >> 8 as ::core::ffi::c_int);
    return clip_channel(b);
}
#[inline]
unsafe extern "C" fn alpha_blend(
    mut dc: Uint8,
    mut sc: Uint8,
    mut alpha: ::core::ffi::c_int,
) -> Uint8 {
    return (((sc as ::core::ffi::c_int - dc as ::core::ffi::c_int) * alpha
        >> 8 as ::core::ffi::c_int) + dc as ::core::ffi::c_int) as Uint8;
}
#[inline]
unsafe extern "C" fn PACK_PIXEL_32(
    mut fmt: *const SDL_PixelFormat,
    mut r: Uint8,
    mut g: Uint8,
    mut b: Uint8,
) -> Uint32 {
    return (r as Uint32) << (*fmt).Rshift as ::core::ffi::c_int
        | (g as Uint32) << (*fmt).Gshift as ::core::ffi::c_int
        | (b as Uint32) << (*fmt).Bshift as ::core::ffi::c_int;
}
unsafe extern "C" fn renderpixel_additive(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
    mut factor: ::core::ffi::c_int,
) {
    let mut fmt: *const SDL_PixelFormat = (*surface).format;
    let mut p: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut sp: Uint32 = 0;
    let mut sr: Uint8 = 0;
    let mut sg: Uint8 = 0;
    let mut sb: Uint8 = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    p = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 4 as ::core::ffi::c_int) as isize) as *mut Uint32;
    sp = *p;
    sr = (sp >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    sg = (sp >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    sb = (sp >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    r = (pixel >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32)
        as ::core::ffi::c_int;
    g = (pixel >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32)
        as ::core::ffi::c_int;
    b = (pixel >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32)
        as ::core::ffi::c_int;
    if factor == 255 as ::core::ffi::c_int {
        sr = clip_channel(sr as ::core::ffi::c_int + r);
        sg = clip_channel(sg as ::core::ffi::c_int + g);
        sb = clip_channel(sb as ::core::ffi::c_int + b);
    } else {
        sr = modulated_sum(sr, r as Uint8, factor);
        sg = modulated_sum(sg, g as Uint8, factor);
        sb = modulated_sum(sb, b as Uint8, factor);
    }
    *p = PACK_PIXEL_32(fmt, sr, sg, sb);
}
unsafe extern "C" fn renderpixel_alpha(
    mut surface: *mut SDL_Surface,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pixel: Uint32,
    mut factor: ::core::ffi::c_int,
) {
    let mut fmt: *const SDL_PixelFormat = (*surface).format;
    let mut p: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut sp: Uint32 = 0;
    let mut sr: Uint8 = 0;
    let mut sg: Uint8 = 0;
    let mut sb: Uint8 = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    if factor == 255 as ::core::ffi::c_int {
        putpixel_32(surface, x, y, pixel);
        return;
    }
    p = ((*surface).pixels as *mut Uint8)
        .offset((y * (*surface).pitch) as isize)
        .offset((x * 4 as ::core::ffi::c_int) as isize) as *mut Uint32;
    sp = *p;
    sr = (sp >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    sg = (sp >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    sb = (sp >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32) as Uint8;
    r = (pixel >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32)
        as ::core::ffi::c_int;
    g = (pixel >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32)
        as ::core::ffi::c_int;
    b = (pixel >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32)
        as ::core::ffi::c_int;
    sr = alpha_blend(sr, r as Uint8, factor);
    sg = alpha_blend(sg, g as Uint8, factor);
    sb = alpha_blend(sb, b as Uint8, factor);
    *p = PACK_PIXEL_32(fmt, sr, sg, sb);
}
#[no_mangle]
pub unsafe extern "C" fn renderpixel_for(
    mut surface: *mut SDL_Surface,
    mut kind: RenderKind,
) -> RenderPixelFn {
    let mut fmt: *const SDL_PixelFormat = (*surface).format;
    if (*fmt).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int {
        return None;
    }
    if (*fmt).Amask != 0 as Uint32 && kind.0 != RenderKind::renderReplace.0 {
        return None;
    }
    match kind {
        RenderKind::renderReplace => {
            return Some(
                renderpixel_replace
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        Uint32,
                        ::core::ffi::c_int,
                    ) -> (),
            );
        }
        RenderKind::renderAdditive => {
            return Some(
                renderpixel_additive
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        Uint32,
                        ::core::ffi::c_int,
                    ) -> (),
            );
        }
        RenderKind::renderAlpha => {
            return Some(
                renderpixel_alpha
                    as unsafe extern "C" fn(
                        *mut SDL_Surface,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        Uint32,
                        ::core::ffi::c_int,
                    ) -> (),
            );
        }
        _ => {}
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn line_prim(
    mut x1: ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut color: Uint32,
    mut plot: RenderPixelFn,
    mut factor: ::core::ffi::c_int,
    mut dst: *mut SDL_Surface,
) {
    let mut d: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut sx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut clip_r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    SDL_GetClipRect(dst, &raw mut clip_r);
    if clip_line(&raw mut x1, &raw mut y1, &raw mut x2, &raw mut y2, &raw mut clip_r)
        == 0
    {
        return;
    }
    dx = x2 - x1;
    ax = if dx < 0 as ::core::ffi::c_int { -dx } else { dx } << 1 as ::core::ffi::c_int;
    sx = if dx < 0 as ::core::ffi::c_int {
        -1 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    dy = y2 - y1;
    ay = if dy < 0 as ::core::ffi::c_int { -dy } else { dy } << 1 as ::core::ffi::c_int;
    sy = if dy < 0 as ::core::ffi::c_int {
        -1 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    x = x1;
    y = y1;
    if ax > ay {
        d = ay - (ax >> 1 as ::core::ffi::c_int);
        loop {
            Some(plot.expect("non-null function pointer"))
                .expect("non-null function pointer")(dst, x, y, color, factor);
            if x == x2 {
                return;
            }
            if d >= 0 as ::core::ffi::c_int {
                y += sy;
                d -= ax;
            }
            x += sx;
            d += ay;
        }
    } else {
        d = ax - (ay >> 1 as ::core::ffi::c_int);
        loop {
            Some(plot.expect("non-null function pointer"))
                .expect("non-null function pointer")(dst, x, y, color, factor);
            if y == y2 {
                return;
            }
            if d >= 0 as ::core::ffi::c_int {
                x += sx;
                d -= ay;
            }
            y += sy;
            d += ax;
        }
    };
}
unsafe extern "C" fn compute_code(
    mut x: ::core::ffi::c_float,
    mut y: ::core::ffi::c_float,
    mut xmin: ::core::ffi::c_float,
    mut ymin: ::core::ffi::c_float,
    mut xmax: ::core::ffi::c_float,
    mut ymax: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if y > ymax {
        c |= C2Rust_Unnamed::C_TOP.0 as ::core::ffi::c_int;
    } else if y < ymin {
        c |= C2Rust_Unnamed::C_BOTTOM.0 as ::core::ffi::c_int;
    }
    if x > xmax {
        c |= C2Rust_Unnamed::C_RIGHT.0 as ::core::ffi::c_int;
    } else if x < xmin {
        c |= C2Rust_Unnamed::C_LEFT.0 as ::core::ffi::c_int;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn clip_line(
    mut lx1: *mut ::core::ffi::c_int,
    mut ly1: *mut ::core::ffi::c_int,
    mut lx2: *mut ::core::ffi::c_int,
    mut ly2: *mut ::core::ffi::c_int,
    mut r: *const SDL_Rect,
) -> ::core::ffi::c_int {
    let mut C0: ::core::ffi::c_int = 0;
    let mut C1: ::core::ffi::c_int = 0;
    let mut C: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    let mut y: ::core::ffi::c_float = 0.;
    let mut x0: ::core::ffi::c_float = 0.;
    let mut y0: ::core::ffi::c_float = 0.;
    let mut x1: ::core::ffi::c_float = 0.;
    let mut y1: ::core::ffi::c_float = 0.;
    let mut xmin: ::core::ffi::c_float = 0.;
    let mut ymin: ::core::ffi::c_float = 0.;
    let mut xmax: ::core::ffi::c_float = 0.;
    let mut ymax: ::core::ffi::c_float = 0.;
    x0 = *lx1 as ::core::ffi::c_float;
    y0 = *ly1 as ::core::ffi::c_float;
    x1 = *lx2 as ::core::ffi::c_float;
    y1 = *ly2 as ::core::ffi::c_float;
    xmin = (*r).x as ::core::ffi::c_float;
    ymin = (*r).y as ::core::ffi::c_float;
    xmax = (*r).x as ::core::ffi::c_float + (*r).w as ::core::ffi::c_float
        - 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    ymax = (*r).y as ::core::ffi::c_float + (*r).h as ::core::ffi::c_float
        - 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    C0 = compute_code(x0, y0, xmin, ymin, xmax, ymax);
    C1 = compute_code(x1, y1, xmin, ymin, xmax, ymax);
    loop {
        if C0 | C1 == 0 as ::core::ffi::c_int {
            *lx1 = x0 as ::core::ffi::c_int;
            *ly1 = y0 as ::core::ffi::c_int;
            *lx2 = x1 as ::core::ffi::c_int;
            *ly2 = y1 as ::core::ffi::c_int;
            return 1 as ::core::ffi::c_int;
        }
        if C0 & C1 != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        C = if C0 != 0 { C0 } else { C1 };
        if C & C2Rust_Unnamed::C_TOP.0 as ::core::ffi::c_int != 0 {
            x = x0 + (x1 - x0) * (ymax - y0) / (y1 - y0);
            y = ymax;
        } else if C & C2Rust_Unnamed::C_BOTTOM.0 as ::core::ffi::c_int != 0 {
            x = x0 + (x1 - x0) * (ymin - y0) / (y1 - y0);
            y = ymin;
        } else if C & C2Rust_Unnamed::C_RIGHT.0 as ::core::ffi::c_int != 0 {
            x = xmax;
            y = y0 + (y1 - y0) * (xmax - x0) / (x1 - x0);
        } else {
            x = xmin;
            y = y0 + (y1 - y0) * (xmin - x0) / (x1 - x0);
        }
        if C == C0 {
            x0 = x;
            y0 = y;
            C0 = compute_code(x0, y0, xmin, ymin, xmax, ymax);
        } else {
            x1 = x;
            y1 = y;
            C1 = compute_code(x1, y1, xmin, ymin, xmax, ymax);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fillrect_prim(
    mut r: SDL_Rect,
    mut color: Uint32,
    mut plot: RenderPixelFn,
    mut factor: ::core::ffi::c_int,
    mut dst: *mut SDL_Surface,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut clip_r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    SDL_GetClipRect(dst, &raw mut clip_r);
    if clip_rect(&raw mut r, &raw mut clip_r) == 0 {
        return;
    }
    x1 = r.x + r.w;
    y1 = r.y + r.h;
    y = r.y;
    while y < y1 {
        x = r.x;
        while x < x1 {
            plot.expect("non-null function pointer")(dst, x, y, color, factor);
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clip_rect(
    mut r: *mut SDL_Rect,
    mut clip_r: *const SDL_Rect,
) -> ::core::ffi::c_int {
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = (*r).w;
    let mut h: ::core::ffi::c_int = (*r).h;
    dx = (*clip_r).x - (*r).x;
    if dx > 0 as ::core::ffi::c_int {
        w -= dx;
        (*r).x += dx;
    }
    dx = (*r).x + w - (*clip_r).x - (*clip_r).w;
    if dx > 0 as ::core::ffi::c_int {
        w -= dx;
    }
    dy = (*clip_r).y - (*r).y;
    if dy > 0 as ::core::ffi::c_int {
        h -= dy;
        (*r).y += dy;
    }
    dy = (*r).y + h - (*clip_r).y - (*clip_r).h;
    if dy > 0 as ::core::ffi::c_int {
        h -= dy;
    }
    if w <= 0 as ::core::ffi::c_int || h <= 0 as ::core::ffi::c_int {
        (*r).w = 0 as ::core::ffi::c_int;
        (*r).h = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    (*r).w = w;
    (*r).h = h;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blt_prim(
    mut src: *mut SDL_Surface,
    mut src_r: SDL_Rect,
    mut plot: RenderPixelFn,
    mut factor: ::core::ffi::c_int,
    mut dst: *mut SDL_Surface,
    mut dst_r: SDL_Rect,
) {
    let mut srcfmt: *mut SDL_PixelFormat = (*src).format;
    let mut srcpal: *mut SDL_Palette = (*srcfmt).palette;
    let mut dstfmt: *mut SDL_PixelFormat = (*dst).format;
    let mut mask: Uint32 = 0 as Uint32;
    let mut key: Uint32 = !(0 as ::core::ffi::c_int) as Uint32;
    let mut getpix: GetPixelFn = getpixel_for(src);
    let mut clip_r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    SDL_GetClipRect(dst, &raw mut clip_r);
    if clip_blt_rects(&raw mut src_r, &raw mut dst_r, &raw mut clip_r) == 0 {
        return;
    }
    if src_r.x >= (*src).w || src_r.y >= (*src).h {
        return;
    }
    if src_r.x + src_r.w > (*src).w {
        src_r.w = (*src).w - src_r.x;
    }
    if src_r.y + src_r.h > (*src).h {
        src_r.h = (*src).h - src_r.y;
    }
    if (*srcfmt).Amask != 0 {
        mask = (*srcfmt).Amask;
        key = 0 as Uint32;
    } else if TFB_GetColorKey(src, &raw mut key) == 0 as ::core::ffi::c_int {
        mask = !(0 as ::core::ffi::c_int) as Uint32;
    }
    y = 0 as ::core::ffi::c_int;
    while y < src_r.h {
        x = 0 as ::core::ffi::c_int;
        while x < src_r.w {
            let mut r: Uint8 = 0;
            let mut g: Uint8 = 0;
            let mut b: Uint8 = 0;
            let mut a: Uint8 = 0;
            let mut p: Uint32 = 0;
            p = getpix
                .expect("non-null function pointer")(src, src_r.x + x, src_r.y + y);
            's_90: {
                if !srcpal.is_null() {
                    if p == key {
                        break 's_90;
                    }
                } else if p & mask == key {
                    break 's_90;
                }
                SDL_GetRGBA(p, srcfmt, &raw mut r, &raw mut g, &raw mut b, &raw mut a);
                p = SDL_MapRGBA(dstfmt, r, g, b, a);
                plot
                    .expect(
                        "non-null function pointer",
                    )(dst, dst_r.x + x, dst_r.y + y, p, factor);
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clip_blt_rects(
    mut src_r: *mut SDL_Rect,
    mut dst_r: *mut SDL_Rect,
    mut clip_r: *const SDL_Rect,
) -> ::core::ffi::c_int {
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    w = (*src_r).w;
    if (*src_r).x < 0 as ::core::ffi::c_int {
        w += (*src_r).x;
        (*dst_r).x -= (*src_r).x;
        (*src_r).x = 0 as ::core::ffi::c_int;
    }
    h = (*src_r).h;
    if (*src_r).y < 0 as ::core::ffi::c_int {
        h += (*src_r).y;
        (*dst_r).y -= (*src_r).y;
        (*src_r).y = 0 as ::core::ffi::c_int;
    }
    dx = (*clip_r).x - (*dst_r).x;
    if dx > 0 as ::core::ffi::c_int {
        w -= dx;
        (*dst_r).x += dx;
        (*src_r).x += dx;
    }
    dx = (*dst_r).x + w - (*clip_r).x - (*clip_r).w;
    if dx > 0 as ::core::ffi::c_int {
        w -= dx;
    }
    dy = (*clip_r).y - (*dst_r).y;
    if dy > 0 as ::core::ffi::c_int {
        h -= dy;
        (*dst_r).y += dy;
        (*src_r).y += dy;
    }
    dy = (*dst_r).y + h - (*clip_r).y - (*clip_r).h;
    if dy > 0 as ::core::ffi::c_int {
        h -= dy;
    }
    if w <= 0 as ::core::ffi::c_int || h <= 0 as ::core::ffi::c_int {
        (*src_r).w = 0 as ::core::ffi::c_int;
        (*src_r).h = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    (*src_r).w = w;
    (*src_r).h = h;
    return 1 as ::core::ffi::c_int;
}
